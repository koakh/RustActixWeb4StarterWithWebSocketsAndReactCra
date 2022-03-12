use serde::{Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetStateResponse {
  pub filter_line: String,
  pub filter_file: String,
  pub config_file: String,
}
