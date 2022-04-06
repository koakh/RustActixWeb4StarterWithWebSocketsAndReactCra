use serde::{Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStateResponse {
  pub server_id: usize,
  pub counter: i32,
  pub request_count: usize,
  // BOF : UNCOMMENT to use config
  // pub filter_line: String,
  // pub filter_file: String,
  // EOF : UNCOMMENT to use config
}
