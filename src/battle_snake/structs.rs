use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartResponse {
  color: String,
  head_type: String,
  tail_type: String,
}

impl StartResponse {
  pub fn new(color: String, head_type: String, tail_type: String) -> StartResponse {
    StartResponse {
      color,
      head_type,
      tail_type,
    }
  }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Move {
  Left,
  Right,
  Up,
  Down,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveResponse {
  r#move: Move,
}

impl MoveResponse {
  pub fn new(r#move: Move) -> MoveResponse {
    MoveResponse {
      r#move,
    }
  }
}

#[derive(Deserialize, Debug)]
pub struct Game {
  id: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Point {
  pub x: u8,
  pub y: u8,
}

#[derive(Deserialize, Debug)]
pub struct Snake {
  pub id: String,
  pub name: String,
  pub health: u8,
  pub body: Vec<Point>,
}

#[derive(Deserialize, Debug)]
pub struct Board {
  pub height: u8,
  pub width: u8,
  pub food: Vec<Point>,
  pub snakes: Vec<Snake>,
}

#[derive(Deserialize, Debug)]
pub struct GameEnvironment {
  pub game: Game,
  pub turn: u16,
  pub board: Board,
  pub you: Snake,
}

pub static POSSIBLE_MOVES: [Move; 4] = [Move::Left, Move::Right, Move::Up, Move::Down];
