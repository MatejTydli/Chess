// #![allow(dead_code)]
use std::io::Read;

#[derive(Clone, Copy, Debug, PartialEq)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Piece {
    type_: PieceType,
    color: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Square {
    piece: Option<Piece>,
    pos: (usize, usize),
}

#[derive(Clone, Debug)]
struct Board {
    pos: [[Square; 8]; 8],
    turn: bool,
    history: Vec<Board>,
}

#[derive(Clone, Copy)]
struct Move {
    pos: Square,
    des: Square,
    special_req: Option<fn(&mut Board, &Square, &Square)>,
}

impl Board {
    fn new() -> Board {
        let mut board = Board::empty();

        let army = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        for i in 0..8 {
            for j in 0..8 {
                match i {
                    0 | 7 => {
                        board.pos[i][j].piece = Some(Piece {
                            type_: army[j],
                            color: if i == 7 { true } else { false },
                        });
                    }
                    1 | 6 => {
                        board.pos[i][j].piece = Some(Piece {
                            type_: PieceType::Pawn,
                            color: if i == 6 { true } else { false },
                        });
                    }
                    _ => {}
                }
            }
        }

        board.history.push(board.clone());

        board
    }

    fn empty() -> Board {
        let mut board = Board {
            pos: [[Square {
                piece: None,
                pos: (0, 0),
            }; 8]; 8],
            turn: true,
            history: Vec::new(),
        };

        for i in 0..8 {
            for j in 0..8 {
                board.pos[i][j].pos = (i, j);
            }
        }

        board
    }

    fn print(&self) {
        for row in self.pos.iter() {
            println!("========================");
            for sq in row {
                match sq.piece {
                    Some(p) => {
                        let mut p_let = format!("{:?}", p.type_).as_bytes()[0] as char;

                        if p.type_ == PieceType::Knight {
                            p_let = 'n';
                        }

                        if p.color {
                            print!("|{}|", p_let.to_ascii_uppercase());
                        } else {
                            print!("|{}|", p_let.to_ascii_lowercase());
                        }
                    }
                    None => print!("| |"),
                }
            }
            println!();
        }
    }

    fn getmove_s(&self, i: usize, j: usize) -> Vec<Move> {
        const DOWN: (i8, i8) = (1, 0);
        const UP: (i8, i8) = (-1, 0);
        const RIGHT: (i8, i8) = (0, 1);
        const LEFT: (i8, i8) = (0, -1);
        const DOWN_RIGHT: (i8, i8) = (1, 1);
        const DOWN_LEFT: (i8, i8) = (1, -1);
        const UP_RIGHT: (i8, i8) = (-1, 1);
        const UP_LEFT: (i8, i8) = (-1, -1);

        let mut moves = Vec::new();

        match self.pos[i][j].piece {
            Some(piece) if piece.color == self.turn => match piece.type_ {
                PieceType::Pawn => {
                    fn promote(board: &mut Board, pos: &Square, des: &Square) {
                        println!("What do you want to promote the pawn into?: u");

                        let mut buf = [0; 1024];
                        std::io::stdin().read(&mut buf).unwrap();

                        let buf_string_result = String::from_utf8(buf.to_vec()).unwrap();
                        let buf_string = buf_string_result.trim().to_owned();

                        board.pos[des.pos.0][des.pos.1].piece = Some(Piece {
                            type_: match buf_string.to_ascii_lowercase().chars().next() {
                                Some('q') => PieceType::Queen,
                                Some('r') => PieceType::Rook,
                                Some('b') => PieceType::Bishop,
                                Some('n') => PieceType::Knight,
                                _ => panic!("invalid promotion"),
                            },
                            color: pos.piece.unwrap().color,
                        });
                    }

                    fn en_passant(board: &mut Board, pos: &Square, des: &Square) {
                        board.pos[(des.pos.0 as i8
                            + if pos.piece.unwrap().color { 1 } else { -1 })
                            as usize][des.pos.1]
                            .piece = None;
                    }

                    fn handle_pawn_push(
                        board: &Board,
                        i: usize,
                        j: usize,
                        moves: &mut Vec<Move>,
                        consts: ((i8, i8), (i8, i8)),
                        mul: i8,
                    ) {
                        match board.createmove_(
                            board.pos[i][j],
                            if board.turn { consts.0 } else { consts.1 },
                            mul,
                            if i == if board.turn { 1 } else { 6 } {
                                Some(promote)
                            } else {
                                None
                            },
                        ) {
                            Some(m) => {
                                if m.des.piece == None {
                                    moves.push(m);
                                }
                            }
                            None => {}
                        }
                    }

                    handle_pawn_push(self, i, j, &mut moves, (UP, DOWN), 1);
                    if i == if self.turn { 6 } else { 1 } {
                        handle_pawn_push(self, i, j, &mut moves, (UP, DOWN), 2);
                    }

                    fn handle_pawn_take(
                        board: &Board,
                        i: usize,
                        j: usize,
                        moves: &mut Vec<Move>,
                        direction: bool,
                    ) {
                        match board.createmove_(
                            board.pos[i][j],
                            if direction {
                                if board.turn {
                                    UP_RIGHT
                                } else {
                                    DOWN_LEFT
                                }
                            } else {
                                if board.turn {
                                    UP_LEFT
                                } else {
                                    DOWN_RIGHT
                                }
                            },
                            1,
                            if i == if board.turn { 1 } else { 6 } {
                                Some(promote)
                            } else {
                                None
                            },
                        ) {
                            Some(mut m) => {
                                if let Some(p) = m.des.piece {
                                    if p.color != board.turn {
                                        moves.push(m);
                                    }
                                } else if i == if board.turn { 3 } else { 4 } {
                                    let offset: i8 = if direction { 1 } else { -1 };
                                    if let Some(p) = board.pos[i][(j as i8 + offset) as usize].piece
                                    {
                                        if p.type_ == PieceType::Pawn && p.color != board.turn {
                                            if let Some(p_h) =
                                                board.history[board.history.len() - 2].pos
                                                    [if p.color { 6 } else { 1 }]
                                                    [(j as i8 + offset) as usize]
                                                    .piece
                                            {
                                                if p_h.color == !board.turn {
                                                    m.special_req = Some(en_passant);
                                                    moves.push(m);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            None => {}
                        }
                    }

                    handle_pawn_take(self, i, j, &mut moves, true);
                    handle_pawn_take(self, i, j, &mut moves, false);
                }
                PieceType::Knight => {
                    for c in [
                        (UP, UP_RIGHT),
                        (UP, UP_LEFT),
                        (DOWN, DOWN_RIGHT),
                        (DOWN, DOWN_LEFT),
                        (RIGHT, UP_LEFT),
                        (RIGHT, DOWN_LEFT),
                        (LEFT, UP_RIGHT),
                        (LEFT, DOWN_RIGHT),
                    ] {
                        match self.createmove_(
                            self.pos[i][j],
                            (c.0 .0 + c.1 .0, c.0 .1 + c.1 .1),
                            1,
                            None,
                        ) {
                            Some(m) => match m.des.piece {
                                Some(p) => {
                                    if p.color != self.turn {
                                        moves.push(m);
                                    }
                                }
                                None => moves.push(m),
                            },
                            None => {}
                        }
                    }
                }
                PieceType::Bishop => {
                    for c in [DOWN_RIGHT, DOWN_LEFT, UP_RIGHT, UP_LEFT] {
                        for k in 1..7 {
                            match self.createmove_(self.pos[i][j], c, k, None) {
                                Some(m) => {
                                    if let Some(p) = m.des.piece {
                                        if p.color != self.turn {
                                            moves.push(m);
                                            break;
                                        } else {
                                            break;
                                        }
                                    } else {
                                        moves.push(m);
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                }
                PieceType::Rook => {
                    for c in [UP, DOWN, RIGHT, LEFT] {
                        for k in 1..7 {
                            match self.createmove_(self.pos[i][j], c, k, None) {
                                Some(m) => {
                                    if let Some(p) = m.des.piece {
                                        if p.color != self.turn {
                                            moves.push(m);
                                            break;
                                        } else {
                                            break;
                                        }
                                    } else {
                                        moves.push(m);
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                }
                PieceType::Queen => {
                    for c in [
                        UP, DOWN, RIGHT, LEFT, DOWN_RIGHT, DOWN_LEFT, UP_RIGHT, UP_LEFT,
                    ] {
                        for k in 1..7 {
                            match self.createmove_(self.pos[i][j], c, k, None) {
                                Some(m) => {
                                    if let Some(p) = m.des.piece {
                                        if p.color != self.turn {
                                            moves.push(m);
                                            break;
                                        } else {
                                            break;
                                        }
                                    } else {
                                        moves.push(m);
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                }
                PieceType::King => {
                    for c in [
                        UP, DOWN, RIGHT, LEFT, DOWN_RIGHT, DOWN_LEFT, UP_RIGHT, UP_LEFT,
                    ] {
                        match self.createmove_(self.pos[i][j], c, 1, None) {
                            Some(m) => {
                                if let Some(p) = m.des.piece {
                                    if p.color != self.turn {
                                        moves.push(m);
                                    }
                                } else {
                                    moves.push(m);
                                }
                            }
                            None => {}
                        }
                    }
                }
            },
            _ => {}
        }

        if self.is_givinig_check(self.turn) {
            let mut in_check_possible_moves = Vec::new();
            for m in moves {
                if false {
                    in_check_possible_moves.push(m);
                }
            }

            todo!()
        } else {
            moves
        }
    }

    fn is_givinig_check(&self, color: bool) -> bool {
        let mut clone = self.clone();
        clone.turn = color;

        for i in 0..8 {
            for j in 0..8 {
                for m in clone.getmove_s(i, j) {
                    if let Some(p) = m.des.piece {
                        if p.type_ == PieceType::King && p.color != color {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn domove_(&mut self, move_: &Move) {
        self.pos[move_.des.pos.0][move_.des.pos.1].piece =
            self.pos[move_.pos.pos.0][move_.pos.pos.1].piece;
        self.pos[move_.pos.pos.0][move_.pos.pos.1].piece = None;
        self.turn = !self.turn;

        match move_.special_req {
            Some(special_req) => {
                special_req(self, &move_.pos, &move_.des);
            }
            None => {}
        }

        let mut self_clone = self.clone();
        self_clone.history = Vec::new();
        self.history.push(self_clone);
    }

    fn createmove_(
        &self,
        sq: Square,
        move_const: (i8, i8),
        mul: i8,
        special_req: Option<fn(&mut Board, &Square, &Square)>,
    ) -> Option<Move> {
        let x = sq.pos.0 as i8 + move_const.0 * mul;
        let y = sq.pos.1 as i8 + move_const.1 * mul;
        if x >= 0 && x < 8 && y >= 0 && y < 8 {
            return Some(Move {
                pos: sq,
                des: self.pos[x as usize][y as usize],
                special_req,
            });
        }

        None
    }
}

fn main() {
    let mut br = Board::new();

    br.print();
    println!();

    // TEST GAME:
    dm(&mut br, 6, 0, 1); //
    dm(&mut br, 1, 1, 1); //
    dm(&mut br, 4, 0, 1); //
    dm(&mut br, 1, 0, 1); //
    dm(&mut br, 3, 1, 1); //
    dm(&mut br, 1, 7, 0); //
    dm(&mut br, 2, 0, 0); //
    dm(&mut br, 1, 6, 0); //
    dm(&mut br, 1, 0, 0); //
}

fn dm(board: &mut Board, i: usize, j: usize, num: usize) {
    let debug = board.getmove_s(i, j);
    board.domove_(&debug[num]);
    board.print();
    println!("white {:?}", board.is_givinig_check(true));
    println!("black {:?}", board.is_givinig_check(false));
    println!();
    std::thread::sleep(std::time::Duration::from_millis(300));
}
