use serde::{Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStateResponse {
  pub server_id: usize,
  pub counter: i32,
  pub request_count: usize,
  pub filter_line: String,
  pub filter_file: String,
}
