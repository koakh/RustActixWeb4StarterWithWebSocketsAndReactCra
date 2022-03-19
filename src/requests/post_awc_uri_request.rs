use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostAwcUriRequest {
  pub uri: String,
}
