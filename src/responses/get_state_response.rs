use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetStateResponse {
  pub counter: i32,
  pub config_file: String,
  // BOF : UNCOMMENT to use config
  // pub filter_line: String,
  // pub filter_file: String,
  // EOF : UNCOMMENT to use config
}
