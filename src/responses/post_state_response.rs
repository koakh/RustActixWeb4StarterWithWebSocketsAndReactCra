use serde::{Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostStateResponse {
  pub filter_line: String,
  pub filter_file: String,
}
