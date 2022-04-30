use enum_from_str::ParseEnumVariantError;
use enum_from_str_derive::FromStr;

use crate::crypto::hash;

pub(crate) const LEN_BYTES: usize = 9;
pub(crate) const CHECKSUM_BYTES: usize = 4;

#[derive(FromStr, Debug)]
pub enum TYPES {
    AddTxn = 0,
}

pub struct P2pMessage {
    pub message: String,
    pub message_type: TYPES,
}

impl P2pMessage {
    pub fn to_string(&self) -> String {
        let raw = hex::encode(format!("{}:{:?}", self.message, self.message_type));
        let checksum: String = String::from_utf8(
            hash(raw.as_bytes().into()).as_bytes().to_vec()[0..CHECKSUM_BYTES].to_vec(),
        )
        .unwrap();
        let mut len = raw.len().to_string();
        while len.len() != LEN_BYTES as usize {
            len += "0"
        }
        format!("{}{}{}", len, raw, checksum)
    }
    pub fn from_string(string: String) -> Result<P2pMessage, Box<dyn std::error::Error>> {
        let decoded_string = String::from_utf8(hex::decode(string)?)?;
        let split: Vec<&str> = decoded_string.split(":").collect();
        Ok(P2pMessage {
            message: split[0].to_owned(),
            message_type: split[1].parse()?,
        })
    }
}
