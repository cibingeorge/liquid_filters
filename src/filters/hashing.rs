#![allow(clippy::invisible_characters)]

use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};
use md5::Digest;


#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "sha1",
    description = "computes sha-1 of a string",
    parsed(Sha1Filter)
)]
pub struct Sha1;

#[derive(Debug, Default, Display_filter)]
#[name = "sha1"]
struct Sha1Filter;

impl Filter for Sha1Filter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        if s.as_str().is_empty() {
            return Ok(Value::scalar(String::new()));
        }
        let digest = ring::digest::digest(&ring::digest::SHA1_FOR_LEGACY_USE_ONLY, s.as_bytes());
        let encoded = data_encoding::HEXLOWER.encode(digest.as_ref());
        Ok(Value::scalar(encoded))
    }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "sha256",
    description = "computes sha-256 of a string",
    parsed(Sha256Filter)
)]
pub struct Sha256;

#[derive(Debug, Default, Display_filter)]
#[name = "sha256"]
struct Sha256Filter;

impl Filter for Sha256Filter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        if s.as_str().is_empty() {
            return Ok(Value::scalar(String::new()));
        }
        let digest = ring::digest::digest(&ring::digest::SHA256, s.as_bytes());
        let encoded = data_encoding::HEXLOWER.encode(digest.as_ref());
        Ok(Value::scalar(encoded))
    }
}


#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "md5",
    description = "computes md5 of a string",
    parsed(Md5Filter)
)]
pub struct Md5;

#[derive(Debug, Default, Display_filter)]
#[name = "md5"]
struct Md5Filter;

impl Filter for Md5Filter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        if s.as_str().is_empty() {
            return Ok(Value::scalar(String::new()));
        }
        let mut hasher = md5::Md5::new();
        hasher.update(s.as_bytes());
        let result = hasher.finalize();
        let encoded = data_encoding::HEXLOWER.encode(result.as_ref());
        Ok(Value::scalar(encoded))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_sha256() {
        assert_eq!(
            liquid_core::call_filter!(Sha256, "Abc").unwrap(),
            liquid_core::value!("06d90109c8cce34ec0c776950465421e176f08b831a938b3c6e76cb7bee8790b")
        );
        assert_eq!(
            liquid_core::call_filter!(Sha256, "Hello World").unwrap(),
            liquid_core::value!("a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e")
        );
    }

    #[test]
    fn unit_md5() {
        assert_eq!(
            liquid_core::call_filter!(Md5, "testuser@getblueshift.com").unwrap(),
            liquid_core::value!("e169b640b9ceda26ce4c3d8a919eb42c")
        );
    }
}
