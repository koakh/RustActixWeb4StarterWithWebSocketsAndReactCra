use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostStateRequest {
  pub filter_file: String,
  pub filter_line: String,
}
