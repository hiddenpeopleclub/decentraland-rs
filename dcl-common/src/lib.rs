use serde::Deserialize;

mod parcel;
pub use parcel::Parcel;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EthNetwork {
  Mainnet,
  Ropsten
}
