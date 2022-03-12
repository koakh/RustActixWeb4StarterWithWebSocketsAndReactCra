use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct PostWsEchoResponse {
  pub message: String,
}
