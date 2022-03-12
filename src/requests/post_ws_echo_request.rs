use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct PostWsEchoRequest {
  pub message: String,
}
