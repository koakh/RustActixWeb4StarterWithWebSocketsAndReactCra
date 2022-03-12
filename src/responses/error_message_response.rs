use serde::{Serialize};

#[derive(Serialize)]
pub struct ErrorMessageResponse {
  pub message: String,
}
