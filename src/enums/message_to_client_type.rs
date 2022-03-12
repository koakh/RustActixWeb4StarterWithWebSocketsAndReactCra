use std::fmt::Display;

pub enum MessageToClientType {
  Echo,
}

impl Display for MessageToClientType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      Self::Echo => write!(f, "echo"),
    }
  }
}
