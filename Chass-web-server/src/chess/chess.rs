use pleco::tools::eval::Eval;
use pleco::Board;
use pleco::{
    core::{
        piece_move::ScoringMove,
        score::{DRAW, MATE},
    },
    SQ,
};
use std::str;

const MATE_V: i16 = MATE as i16;
const DRAW_V: i16 = DRAW as i16;
const ALPHA: i16 = -32001;
const BETA: i16 = -ALPHA;

fn eval_board(board: &Board) -> ScoringMove {
    ScoringMove::blank(Eval::eval_low(board) as i16)
}

pub struct ChessTools {}

// trait DummyU8{}

// trait DummyString{}

// impl<T> DummyString for Tile<T>{}
// impl<T> DummyU8 for Tile<T>{}

pub struct Tile<T>
where T : std::convert::From<u8> + std::convert::From<String>
{
    data: T,
}

impl<T> Tile<T> 
where T : std::convert::From<u8> + std::convert::From<String> 
{
    pub fn new(data: T) -> Tile<T> {
        const STRING: &str = "string";
        const STR: &str = "str";
        const U8: &str = "u8";

        let byte_calc = |data: &str| {
            if let Ok(num) = data[..1].parse::<u8>() {
                data.bytes().nth(0).unwrap() * num
            } else {
                panic!("parsing error");
            }
        };

        match std::any::type_name::<T>() {
            STRING => Tile {
                data: "asd".to_string(),
            },
            // STR => Tile { data: () },
            // U8 => Tile { data: () },
            _ => panic!("Tile new error"),
        }
    }

    
}

impl ChessTools {
    pub fn make_it_sq(location: usize) -> SQ {
        SQ(location as u8 + 1)
    }

    pub fn alpha_beta_search(
        mut board: &mut Board,
        mut alpha: i16,
        beta: i16,
        depth: u16,
    ) -> ScoringMove {
        if depth == 0 {
            return eval_board(&mut board);
        }

        let mut moves = board.generate_scoring_moves();

        if moves.is_empty() {
            if board.in_check() {
                return ScoringMove::blank(-MATE_V);
            } else {
                return ScoringMove::blank(DRAW_V);
            }
        }

        let mut best_move = ScoringMove::blank(alpha);
        for mov in moves.iter_mut() {
            board.apply_move(mov.bit_move);
            mov.score = -ChessTools::alpha_beta_search(&mut board, -beta, -alpha, depth - 1).score;
            board.undo_move();
            if mov.score > alpha {
                alpha = mov.score;
                if alpha >= beta {
                    return *mov;
                }
                best_move = *mov;
            }
        }

        best_move
    }

    fn king_attacks(position: usize) -> Vec<usize> {
        let x = position % 8;
        let y = position / 8;
        let mut attacks = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < 8 && ny >= 0 && ny < 8 {
                    attacks.push((ny * 8 + nx) as usize);
                }
            }
        }

        attacks
    }
}

pub struct Game {
    pub board: Board,
    bitborad: [&'static str; 64],
    depth: u16,
}

impl Game {
    pub fn init(depth: u16) -> Game {
        Game {
            board: Board::start_pos(),
            bitborad: [
                "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2",
                "g2", "h2", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4",
                "e4", "f4", "g4", "h4", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6",
                "c6", "d6", "e6", "f6", "g6", "h6", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
                "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
            ],
            depth: depth,
        }
    }

    pub fn apply_turn(&mut self, uci: &str) -> String {
        self.board.apply_uci_move(uci);
        // does function check Checkmate?

        let v = ChessTools::alpha_beta_search(&mut self.board, ALPHA, BETA, self.depth).bitmove();
        self.board.apply_move(v);

        match self.board.last_move() {
            Some(v) => String::from(v.stringify()),
            None => String::from(""),
        }
    }
}
