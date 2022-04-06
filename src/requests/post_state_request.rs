use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostStateRequest {
  // BOF : UNCOMMENT to use config
  // pub filter_file: String,
  // pub filter_line: String,
  // EOF : UNCOMMENT to use config
}
