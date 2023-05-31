use core::borrow;

use chass_web_server::chess::chess::Game;
use chass_web_server::server::server::Server;

use pleco::core::GenTypes;
use pleco::{Board, Player, SQ};

// GenTypes::Evasions -> 체크인 경우, 피할 수 있는 위치
// let fev = "rnbqkbnr/ppp1pppp/8/1B1p4/4P3/8/PPPP1PPP/RNBQK1NR b KQkq - 1 2";
// // let mut game = Game::init(3);
// // println!("{}", game.apply_turn("e2e4"));
// // let obj = Server{};
// // obj.run();
// let board = Board::from_fen(fev).unwrap();
// let moves = board.generate_moves();
// let sq = SQ(33);
// let moves = board.attacks_from(pleco::PieceType::B, sq, Player::White);
// let d = board.piece_at_sq(sq);
// println!("{:?}", d.player());

// // board.print_debug_info();
// // board.fancy_print();

// // println!("{:?}", moves.to_sq());

// // let mut board = Board::start_pos();
// // let capturing_moves = board.generate_moves_of_type(GenTypes::QuietChecks);

// // println!("{}", capturing_moves.len());

// // for item in capturing_moves{
// //     board.apply_move(item);
// //     board.fancy_print();
// //     board.in_check();
// // }

fn test<T>(data : &T){
    match std::any::type_name::<T>() {
        "&str" => println!("asd"),
        _ => println!("d"),
    }
}

fn main() {
    let d = "Asd";
    test(&d);
}
