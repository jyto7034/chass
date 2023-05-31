use pleco::{BitMove, Board, MoveList};
use std::borrow::Borrow;
use std::borrow::Cow;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

/*
8 | 56 57 58 59 60 61 62 63
7 | 48 49 50 51 52 53 54 55
6 | 40 41 42 43 44 45 46 47
5 | 32 33 34 35 36 37 38 39
4 | 24 25 26 27 28 29 30 31
3 | 16 17 18 19 20 21 22 23
2 | 8  9  10 11 12 13 14 15
1 | 0  1  2  3  4  5  6  7
  -------------------------
     a  b  c  d  e  f  g  h
*/

/// Arduino uno 와 통신하는 Server 구조체.
/// 체스판의 모든 타일엔 센서가 존재하고, 각 센서는 고유한 ID 를 가짐. 만약 기물이 체스판위에 있다면 On 상태이고, 그렇지 않다면 Off 상태임.
/// 체스판에서 기물을 옮기기 위해 손으로 잡아 들었을 때, 기물이 체스판으로부터 떨어진 것을 센서가 감지함.
/// 이때 Off 상태가 된 타일의 정보를 Arduino Uno 에서 server 로 전송.
///
/// 보드에서 발생 가능한 모든 이벤트
///  - Picked up
///    -> Put down ( Move none )
///    -> Move
///
/// piece 가 들어지는 순간, Arduino 는 Server 로 info picked 의 준말인 ip 와 piece 가 들어진 위치 정보를 합쳐 보냄.
/// eg. ipe4
///
/// 위치 정보를 받은 Server 는 해당 위치의 기물 정보를 파악 후, move 의 초성인 m 과 이동 가능한 위치들을 합친 정보를 Arduino 에게 전송함.
/// eg. me6e7
struct AnalyzeTool {}

enum InfoType {
    /// Just picked up a piece
    PickedUp(BitMove),
    /// Just do nothing, put down
    MoveNone,
    /// Move a piece
    Move(BitMove),
}

impl AnalyzeTool {
    ///
    ///
    fn analyze_info(board: &Board, data: &Cow<str>) -> InfoType {
        let loc = match data {
            Cow::Borrowed(v) => String::from(*v),
            Cow::Owned(v) => v.into(),
        };

        // let pieceType = board.piece_at_sq(sq);
        InfoType::PickedUp(BitMove::new(33))
    }

    fn analyze_move(board: &Board, data: &Cow<str>) -> InfoType {
        InfoType::PickedUp(BitMove::new(33))
    }

    pub fn analyze(board: &Board, data: &Cow<str>) -> InfoType {
        const INFO: char = 'i';
        const MOVE: char = 'm';

        match data.chars().nth(0).unwrap() {
            INFO => AnalyzeTool::analyze_info(board, data),
            MOVE => AnalyzeTool::analyze_move(board, data),
            _ => InfoType::MoveNone,
        }
    }
}

pub struct Server {
    board: Board,
}

impl Server {
    pub fn new(board: Board) -> Server {
        Server { board }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.handle_connection(stream);
        }
    }

    pub fn send_uci(&self) {}

    pub fn receive_uci(&self) {}

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();
        let data = String::from_utf8_lossy(&buffer[..]);
        AnalyzeTool::analyze(&self.board, &data);
    }
}
