#![allow(clippy::invisible_characters)]

use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

use base64::prelude::*;
use base64::engine::general_purpose;

use super::filter_error;

/// base64_encode
/// Returns the Base64-encoded version of a string.
/// Example:
/// {{ "lorem ipsum" | base64_encode }}
/// Returns:
/// bG9yZW0gaXBzdW0=
/// For longer strings, line feeds are added to every 60 encoded characters.
#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "base64_encode",
    description = "Returns the Base64-encoded version of a string.",
    parsed(Base64EncodeFilter)
)]
pub struct Base64Encode;

#[derive(Debug, Default, Display_filter)]
#[name = "base64_encode"]
struct Base64EncodeFilter;

impl Filter for Base64EncodeFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        if s.is_empty() {
            return Ok(Value::scalar(""));
        }
        let b64 = general_purpose::STANDARD.encode(s.as_str());
        let wrapped = textwrap::wrap(b64.as_str(), 60);
        Ok(Value::scalar(format!("{}\n", wrapped.join("\n"))))
    }
}

/// base64_strict_encode
/// Returns the Base64-encoded version of a string, with no line feeds added.
/// Example:
/// {{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec molestie gravida" | base64_strict_encode}
/// Returns:
/// TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdC4KICAgIERvbmVjIG1vbGVzdGllIGdyYXZpZGE=
#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "base64_strict_encode",
    description = "Returns the Base64-encoded version of a string with no line feeds added.",
    parsed(Base64StrictEncodeFilter)
)]
pub struct Base64StrictEncode;

#[derive(Debug, Default, Display_filter)]
#[name = "base64_strict_encode"]
struct Base64StrictEncodeFilter;

impl Filter for Base64StrictEncodeFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        let b64 = general_purpose::STANDARD.encode(s.as_str());
        Ok(Value::scalar(b64))
    }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "b64_enc",
    description = "Returns the Base64-encoded version of a string.",
    parsed(B64EncFilter)
)]
pub struct B64Enc;

#[derive(Debug, Default, Display_filter)]
#[name = "b64_enc"]
struct B64EncFilter;

impl Filter for B64EncFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        if s.is_empty() {
            return Ok(Value::scalar("".to_owned()));
        }
        let b64 = general_purpose::STANDARD.encode(s.as_str());
        let wrapped = textwrap::wrap(b64.as_str(), 60);
        Ok(Value::scalar(format!("{}\n", wrapped.join("\n"))))
    }
}


/// base64_decode
/// Returns the Base64-decoded version of a string.
/// Example:
/// {{ "lorem ipsum" | base64_decode }}
/// Returns:
/// bG9yZW0gaXBzdW0=
/// For longer strings, line feeds are added to every 60 decoded characters.
#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "base64_decode",
    description = "Returns the Base64-decoded version of a string.",
    parsed(Base64DecodeFilter)
)]
pub struct Base64Decode;

#[derive(Debug, Default, Display_filter)]
#[name = "base64_decode"]
struct Base64DecodeFilter;

impl Filter for Base64DecodeFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        let b64 = general_purpose::STANDARD.decode(s.as_str()).map_err(|err| {
            filter_error(format!("Base64 decode error: {}", err))
        })?;
        let decoded = String::from_utf8_lossy(&b64);
        Ok(Value::scalar(decoded.to_string()))
    }
}

/// base64_strict_decode
/// Returns the Base64-decoded version of a string, with no line feeds added.
/// Example:
/// {{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec molestie gravida" | base64_strict_decode}
/// Returns:
/// TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdC4KICAgIERvbmVjIG1vbGVzdGllIGdyYXZpZGE=
#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "base64_strict_decode",
    description = "Returns the Base64-decoded version of a string with no line feeds added.",
    parsed(Base64StrictDecodeFilter)
)]
pub struct Base64StrictDecode;

#[derive(Debug, Default, Display_filter)]
#[name = "base64_strict_decode"]
struct Base64StrictDecodeFilter;

impl Filter for Base64StrictDecodeFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        let b64 = general_purpose::STANDARD.decode(s.as_str()).map_err(|err| {
            filter_error(format!("Base64 decode error: {}", err))
        })?;
        let decoded = String::from_utf8_lossy(&b64);
        Ok(Value::scalar(decoded.to_string()))
    }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "b64_dec",
    description = "Returns the Base64-decoded version of a string.",
    parsed(B64DecFilter)
)]
pub struct B64dec;

#[derive(Debug, Default, Display_filter)]
#[name = "b64_dec"]
struct B64DecFilter;

impl Filter for B64DecFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        if s.is_empty() {
            return Ok(Value::scalar("".to_owned()));
        }

        let b64 = general_purpose::STANDARD.decode(s.as_str()).map_err(|err| {
            filter_error(format!("Base64 decode error: {}", err))
        })?;
        let decoded = String::from_utf8_lossy(&b64);
        Ok(Value::scalar(decoded.to_string()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_base64_encode() {
        assert_eq!(
            liquid_core::call_filter!(Base64Encode, "lorem ipsum").unwrap(),
            liquid_core::value!("bG9yZW0gaXBzdW0=\n")
        );
        let s = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec molestie gravida";
        assert_eq!(
            liquid_core::call_filter!(Base64Encode, s).unwrap(),
            liquid_core::value!("TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBp\nc2NpbmcgZWxpdC4gRG9uZWMgbW9sZXN0aWUgZ3JhdmlkYQ==\n")
        );
        assert_eq!(
            liquid_core::call_filter!(Base64Encode, format!("{}{}", s, s)).unwrap(),
            liquid_core::value!("TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBp\nc2NpbmcgZWxpdC4gRG9uZWMgbW9sZXN0aWUgZ3JhdmlkYUxvcmVtIGlwc3Vt\nIGRvbG9yIHNpdCBhbWV0LCBjb25zZWN0ZXR1ciBhZGlwaXNjaW5nIGVsaXQu\nIERvbmVjIG1vbGVzdGllIGdyYXZpZGE=\n")
        );
    }

    #[test]
    fn unit_base64_strict_encode() {
        let s = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec molestie gravida";
        assert_eq!(
            liquid_core::call_filter!(Base64StrictEncode, s).unwrap(),
            liquid_core::value!("TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdC4gRG9uZWMgbW9sZXN0aWUgZ3JhdmlkYQ==")
        );
        assert_eq!(
            liquid_core::call_filter!(Base64StrictEncode, format!("{}{}", s, s)).unwrap(),
            liquid_core::value!("TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdC4gRG9uZWMgbW9sZXN0aWUgZ3JhdmlkYUxvcmVtIGlwc3VtIGRvbG9yIHNpdCBhbWV0LCBjb25zZWN0ZXR1ciBhZGlwaXNjaW5nIGVsaXQuIERvbmVjIG1vbGVzdGllIGdyYXZpZGE=")
        );
    }


}
