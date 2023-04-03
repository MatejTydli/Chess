#![allow(dead_code)]

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
    _type: PieceType,
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

// type special_req = dyn ;

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
            PieceType::King,
            PieceType::Queen,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        for i in 0..8 {
            for j in 0..8 {
                match i {
                    0 | 7 => {
                        board.pos[i][j].piece = Some(Piece {
                            _type: army[j],
                            color: if i == 7 { true } else { false },
                        });
                    }
                    1 | 6 => {
                        board.pos[i][j].piece = Some(Piece {
                            _type: PieceType::Pawn,
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
                        let mut p_let = format!("{:?}", p._type).as_bytes()[0] as char;

                        if p._type == PieceType::Knight {
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

    fn get_moves(&self, i: usize, j: usize) -> Vec<Move> {
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
            Some(piece) if piece.color == self.turn => match piece._type {
                PieceType::Pawn => {
                    fn handle_pawn_push(
                        board: &Board,
                        i: usize,
                        j: usize,
                        moves: &mut Vec<Move>,
                        consts: ((i8, i8), (i8, i8)),
                        mul: i8,
                    ) {
                        match board.create_move(
                            board.pos[i][j],
                            if board.turn { consts.0 } else { consts.1 },
                            mul,
                            None,
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
                        let special_req = |board: &mut Board, pos: &Square, des: &Square| {
                            board.pos[(des.pos.0 as i8
                                + if pos.piece.unwrap().color { 1 } else { -1 })
                                as usize][des.pos.1]
                                .piece = None;
                        };
                        match board.create_move(
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
                            None,
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
                                        if p._type == PieceType::Pawn && p.color != board.turn {
                                            if let Some(p_h) =
                                                board.history[board.history.len() - 2].pos
                                                    [if p.color { 6 } else { 1 }]
                                                    [(j as i8 + offset) as usize]
                                                    .piece
                                            {
                                                if p_h.color == !board.turn {
                                                    m.special_req = Some(special_req);
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
                        match self.create_move(
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
                            match self.create_move(self.pos[i][j], c, k, None) {
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
                    todo!()
                }
                PieceType::Queen => {
                    todo!()
                }
                PieceType::King => {
                    todo!()
                }
            },
            _ => {}
        }

        moves
    }

    fn do_move(&mut self, _move: Move) {
        self.pos[_move.des.pos.0][_move.des.pos.1].piece =
            self.pos[_move.pos.pos.0][_move.pos.pos.1].piece;
        self.pos[_move.pos.pos.0][_move.pos.pos.1].piece = None;
        self.turn = !self.turn;

        match _move.special_req {
            Some(special_req) => {
                special_req(self, &_move.pos, &_move.des);
            }
            None => {}
        }

        let mut self_clone = self.clone();
        self_clone.history = Vec::new();
        self.history.push(self_clone);
    }

    fn create_move(
        &self,
        sq: Square,
        move_const: (i8, i8),
        mul: i8,
        special_req: Option<fn(&mut Board, &Square, &Square)>,
    ) -> Option<Move> {
        let x = sq.pos.0 as i8 + move_const.0 * mul;
        let y = sq.pos.1 as i8 + move_const.1 * mul;
        if x >= 0 && x <= 8 && y >= 0 && y <= 8 {
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
    let mut my_board = Board::new();

    my_board.print();
    println!();
    std::thread::sleep(std::time::Duration::from_secs(1));

    dm(&mut my_board, 6, 0, 1);
    dm(&mut my_board, 1, 1, 1);
    dm(&mut my_board, 4, 0, 1);
    dm(&mut my_board, 1, 0, 1);
    dm(&mut my_board, 3, 1, 1);
    dm(&mut my_board, 0, 1, 0);
}

fn dm(board: &mut Board, i: usize, j: usize, num: usize) {
    let debug = board.get_moves(i, j);
    board.do_move(debug[num]);
    board.print();
    println!();
    std::thread::sleep(std::time::Duration::from_secs(1));
}
