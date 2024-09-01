use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::FilterParameters;
use liquid_core::FromFilterParameters;
use liquid_core::Expression;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};


use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit, KeyInit};
use super::{invalid_argument, invalid_input};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

type Aes256EcbEnc = ecb::Encryptor<aes::Aes256>;
type Aes256EcbDec = ecb::Decryptor<aes::Aes256>;


//   def aes256_encrypt_v2(data, key, iv=nil, cipher_name=nil)
//     c = cipher(cipher_name).encrypt
//     c.iv = hex_to_binary_string(iv) if iv.present?
//     c.key = hex_to_binary_string(key)
//     output = c.update(data.to_s) + c.final
//     binary_string_to_hex(output)
//   end

fn aes256_encrypt(data: &str, key_hex: &str, iv_hex: Option<&str>, cipher_name: Option<&str>, truncate_iv_v1: bool) -> Result<String> {
    let key = hex::decode(key_hex).map_err(|_err|{
        invalid_argument(
            key_hex.to_owned(),
            "Hex decoding key error".to_owned(),
        )
    })?;
    if let Some("aes-256-ecb") = cipher_name {
        let res = Aes256EcbEnc::new_from_slice(key.as_slice()).unwrap()
            .encrypt_padded_vec_mut::<Pkcs7>(data.as_bytes());
        let encoded = hex::encode(res);
        return Ok(encoded);
    }

    // aes-256-cbc
    if iv_hex.is_none() {
        return Err(liquid_core::Error::with_msg("Missing required argument iv"));
    }
    let iv_hex = iv_hex.unwrap();
    let mut iv_str = iv_hex.to_owned();
    let decoded: Vec<u8>;
    let iv = if truncate_iv_v1 {
        let upto = iv_hex.char_indices().map(|(i, _)| i).nth(16).unwrap_or(iv_hex.len());
        iv_str.truncate(upto);
        iv_str.as_bytes()
    } else {
        decoded = hex::decode(iv_hex).map_err(|_err|{
            invalid_argument(
                iv_hex.to_owned(),
                "Hex decoding iv error".to_owned(),
            )
        })?;
        decoded.as_slice()
    };
    let res = Aes256CbcEnc::new_from_slices(key.as_slice(), iv).unwrap()
        .encrypt_padded_vec_mut::<Pkcs7>(data.as_bytes());

    let encoded = hex::encode(res);
    Ok(encoded)

}


fn aes256_decrypt(data: &str, key_hex: &str, iv_hex: Option<&str>, cipher_name: Option<&str>, truncate_iv_v1: bool) -> Result<String> {
    let data = hex::decode(data).map_err(|_err|{
        invalid_argument(
            data.to_owned(),
            "Hex decoding data error".to_owned(),
        )
    })?;
    let key = hex::decode(key_hex).map_err(|_err|{
        invalid_argument(
            key_hex.to_owned(),
            "Hex decoding key error".to_owned(),
        )
    })?;

    if let Some("aes-256-ecb") = cipher_name {
        let res = Aes256EcbDec::new_from_slice(key.as_slice()).unwrap()
            .decrypt_padded_vec_mut::<Pkcs7>(&data).map_err(|err|{
                invalid_argument(
                    key_hex.to_owned(),
                    format!("decryption error: {}", err),
                )
            })?;

        let decoded = String::from_utf8(res).map_err(|err| {
            invalid_input(
                format!("Decrypted data is not utf8. error: {}", err),
            )
        })?;
        return Ok(decoded);
    }

    // aes-256-cbc
    if iv_hex.is_none() {
        return Err(liquid_core::Error::with_msg("Missing required argument iv"));
    }

    let iv_hex = iv_hex.unwrap();
    let decoded: Vec<u8>;
    let mut iv_str = iv_hex.to_owned();
    let iv = if truncate_iv_v1 {
        let upto = iv_hex.char_indices().map(|(i, _)| i).nth(16).unwrap_or(iv_hex.len());
        iv_str.truncate(upto);
        iv_str.as_bytes()
    } else {
        decoded = hex::decode(iv_hex).map_err(|_err|{
            invalid_argument(
                iv_hex.to_owned(),
                "Hex decoding iv error".to_owned(),
            )
        })?;
        decoded.as_slice()
    };

    let res = Aes256CbcDec::new_from_slices(key.as_slice(), iv).unwrap()
        .decrypt_padded_vec_mut::<Pkcs7>(&data).map_err(|err|{
            invalid_argument(
                key_hex.to_owned(),
                format!("decryption error: {}", err),
            )
        })?;

    let decoded = String::from_utf8(res).map_err(|err| {
        invalid_input(
            format!("Decrypted data is not utf8. error: {}", err),
        )
    })?;
    Ok(decoded)

}


#[derive(Debug, FilterParameters)]
struct Aes256Args {
    #[parameter(description = "Hex encoded key.", arg_type = "str")]
    key_hex: Expression,

    #[parameter(description = "Hex encoded initialization vector.", arg_type = "str")]
    iv_hex: Option<Expression>,

    #[parameter(description = "Cipher name. aes-256-cbc or aes-256-ecb. Default is aes-256-cbc", arg_type = "str")]
    cipher_name: Option<Expression>,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "aes256_encrypt_v2",
    description = "This is an AES 256 Encrypt/Decrypt filter. This filter returns a hex encoded AES 256 ciphertext of the input based on a given key, initialization vector and cipher name.",
    parameters(Aes256Args),
    parsed(Aes256EncryptV2Filter),
)]
pub struct Aes256EncryptV2;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "aes256_encrypt_v2"]
struct Aes256EncryptV2Filter {
    #[parameters]
    args: Aes256Args,
}

impl Filter for Aes256EncryptV2Filter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        if s.as_str().is_empty() {
            return Ok(Value::scalar(String::new()));
        }
        let args = self.args.evaluate(runtime)?;

        let key_hex = args.key_hex.as_str();
        let iv_hex = args.iv_hex.map(|x| x.to_string());
        let cipher_name = args.cipher_name.map(|x| x.to_string());

        let encoded = aes256_encrypt(s.as_str(), key_hex, iv_hex.as_deref(), cipher_name.as_deref(), false)?;
        Ok(Value::scalar(encoded))
    }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "aes256_encrypt",
    description = "Deprecated: This is an AES 256 Encrypt/Decrypt filter v1. This filter returns a hex encoded AES 256 ciphertext of the input based on a given key, initialization vector and cipher name.",
    parameters(Aes256Args),
    parsed(Aes256EncryptV1Filter),
)]
pub struct Aes256EncryptV1Deprecated;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "aes256_encrypt"]
struct Aes256EncryptV1Filter {
    #[parameters]
    args: Aes256Args,
}

impl Filter for Aes256EncryptV1Filter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        if s.as_str().is_empty() {
            return Ok(Value::scalar(String::new()));
        }
        let args = self.args.evaluate(runtime)?;

        let key_hex = args.key_hex.as_str();
        let iv_hex = args.iv_hex.map(|x| x.to_string());
        let cipher_name = args.cipher_name.map(|x| x.to_string());

        let encoded = aes256_encrypt(s.as_str(), key_hex, iv_hex.as_deref(), cipher_name.as_deref(), true)?;
        Ok(Value::scalar(encoded))
    }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "aes256_decrypt_v2",
    description = "This is an AES 256 Encrypt/Decrypt filter. This filter returns a hex encoded AES 256 ciphertext of the input based on a given key, initialization vector and cipher name.",
    parameters(Aes256Args),
    parsed(Aes256DecryptV2Filter),
)]
pub struct Aes256DecryptV2;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "aes256_decrypt_v2"]
struct Aes256DecryptV2Filter {
    #[parameters]
    args: Aes256Args,
}

impl Filter for Aes256DecryptV2Filter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        if s.as_str().is_empty() {
            return Ok(Value::scalar(String::new()));
        }
        let args = self.args.evaluate(runtime)?;

        let key_hex = args.key_hex.as_str();
        let iv_hex = args.iv_hex.map(|x| x.to_string());
        let cipher_name = args.cipher_name.map(|x| x.to_string());

        let encoded = aes256_decrypt(s.as_str(), key_hex, iv_hex.as_deref(), cipher_name.as_deref(), false)?;
        Ok(Value::scalar(encoded))
    }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "aes256_decrypt",
    description = "Deprecated: This is an AES 256 Encrypt/Decrypt filter v1. This filter returns a hex encoded AES 256 ciphertext of the input based on a given key, initialization vector and cipher name.",
    parameters(Aes256Args),
    parsed(Aes256DecryptV1Filter),
)]
pub struct Aes256DecryptV1Deprecated;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "aes256_decrypt"]
struct Aes256DecryptV1Filter {
    #[parameters]
    args: Aes256Args,
}

impl Filter for Aes256DecryptV1Filter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        if s.as_str().is_empty() {
            return Ok(Value::scalar(String::new()));
        }
        let args = self.args.evaluate(runtime)?;

        let key_hex = args.key_hex.as_str();
        let iv_hex = args.iv_hex.map(|x| x.to_string());
        let cipher_name = args.cipher_name.map(|x| x.to_string());

        let encoded = aes256_decrypt(s.as_str(), key_hex, iv_hex.as_deref(), cipher_name.as_deref(), true)?;
        Ok(Value::scalar(encoded))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_aes256_encrypt_v2() {
        assert_eq!(
            liquid_core::call_filter!(Aes256EncryptV2,
                "testuser@getblueshift.com",
                "9cc25c7879fc94d5a19eeb8e47573b8423becb608a9a4e9d3c25c20aa7e04357",
                "", // Doesnt matter what this is
                "aes-256-ecb"
            ).unwrap(),
            liquid_core::value!("0690337ed5120d439952decd9ca2f2382d8914d304885306cf4c4a4d606797e6")
        );

        assert_eq!(
            liquid_core::call_filter!(Aes256EncryptV2,
                "testuser@getblueshift.com",
                "9cc25c7879fc94d5a19eeb8e47573b8423becb608a9a4e9d3c25c20aa7e04357",
                "7bdc922b354cc8fa8d3f2910ba7cc411"
            ).unwrap(),
            liquid_core::value!("9be4086dd0f2592273dbe6e0000377ef94ab6aa3573f9344b4abbbbcf47088b7")
        );

        assert_eq!(
            liquid_core::call_filter!(Aes256EncryptV2,
                "testuser@getblueshift.com",
                "9cc25c7879fc94d5a19eeb8e47573b8423becb608a9a4e9d3c25c20aa7e04357",
                "7bdc922b354cc8fa8d3f2910ba7cc411",
                "aes-256-cbc"
            ).unwrap(),
            liquid_core::value!("9be4086dd0f2592273dbe6e0000377ef94ab6aa3573f9344b4abbbbcf47088b7")
        );

    }


    #[test]
    fn unit_aes256_decrypt_v2() {
        assert_eq!(
            liquid_core::call_filter!(Aes256DecryptV2,
                "0690337ed5120d439952decd9ca2f2382d8914d304885306cf4c4a4d606797e6",
                "9cc25c7879fc94d5a19eeb8e47573b8423becb608a9a4e9d3c25c20aa7e04357",
                "", // Doesnt matter what this is
                "aes-256-ecb"
            ).unwrap(),
            liquid_core::value!("testuser@getblueshift.com")
        );

        assert_eq!(
            liquid_core::call_filter!(Aes256DecryptV2,
                "9be4086dd0f2592273dbe6e0000377ef94ab6aa3573f9344b4abbbbcf47088b7",
                "9cc25c7879fc94d5a19eeb8e47573b8423becb608a9a4e9d3c25c20aa7e04357",
                "7bdc922b354cc8fa8d3f2910ba7cc411"
            ).unwrap(),
            liquid_core::value!("testuser@getblueshift.com")
        );

        assert_eq!(
            liquid_core::call_filter!(Aes256DecryptV2,
                "9be4086dd0f2592273dbe6e0000377ef94ab6aa3573f9344b4abbbbcf47088b7",
                "9cc25c7879fc94d5a19eeb8e47573b8423becb608a9a4e9d3c25c20aa7e04357",
                "7bdc922b354cc8fa8d3f2910ba7cc411",
                "aes-256-cbc"
            ).unwrap(),
            liquid_core::value!("testuser@getblueshift.com")
        );

    }

    #[test]
    fn unit_aes256_encrypt_v1() {
        assert_eq!(
            liquid_core::call_filter!(Aes256EncryptV1Deprecated,
                "testuser@getblueshift.com",
                "9cc25c7879fc94d5a19eeb8e47573b8423becb608a9a4e9d3c25c20aa7e04357",
                "7bdc922b354cc8fa8d3f2910ba7cc411"
            ).unwrap(),
            liquid_core::value!("e8420a7c166353e5a5c5b0aa21b4360b60e5ea357e8c0f36fac272c45cb20c6b")
        );

        assert_eq!(
            liquid_core::call_filter!(Aes256EncryptV1Deprecated,
                "testuser@getblueshift.com",
                "9cc25c7879fc94d5a19eeb8e47573b8423becb608a9a4e9d3c25c20aa7e04357",
                "7bdc922b354cc8fa8d3f2910ba7cc411",
                "aes-256-cbc"
            ).unwrap(),
            liquid_core::value!("e8420a7c166353e5a5c5b0aa21b4360b60e5ea357e8c0f36fac272c45cb20c6b")
        );

        assert_eq!(
            liquid_core::call_filter!(Aes256EncryptV1Deprecated,
                "testuser@getblueshift.com",
                "9cc25c7879fc94d5a19eeb8e47573b8423becb608a9a4e9d3c25c20aa7e04357",
                "", // Doesnt matter what this is
                "aes-256-ecb"
            ).unwrap(),
            liquid_core::value!("0690337ed5120d439952decd9ca2f2382d8914d304885306cf4c4a4d606797e6")
        );
    }



    #[test]
    fn unit_aes256_decrypt_v1() {
        assert_eq!(
            liquid_core::call_filter!(Aes256DecryptV1Deprecated,
                "e8420a7c166353e5a5c5b0aa21b4360b60e5ea357e8c0f36fac272c45cb20c6b",
                "9cc25c7879fc94d5a19eeb8e47573b8423becb608a9a4e9d3c25c20aa7e04357",
                "7bdc922b354cc8fa8d3f2910ba7cc411"
            ).unwrap(),
            liquid_core::value!("testuser@getblueshift.com")
        );

        assert_eq!(
            liquid_core::call_filter!(Aes256DecryptV1Deprecated,
                "e8420a7c166353e5a5c5b0aa21b4360b60e5ea357e8c0f36fac272c45cb20c6b",
                "9cc25c7879fc94d5a19eeb8e47573b8423becb608a9a4e9d3c25c20aa7e04357",
                "7bdc922b354cc8fa8d3f2910ba7cc411",
                "aes-256-cbc"
            ).unwrap(),
            liquid_core::value!("testuser@getblueshift.com")
        );

        assert_eq!(
            liquid_core::call_filter!(Aes256DecryptV1Deprecated,
                "0690337ed5120d439952decd9ca2f2382d8914d304885306cf4c4a4d606797e6",
                "9cc25c7879fc94d5a19eeb8e47573b8423becb608a9a4e9d3c25c20aa7e04357",
                "", // Doesnt matter what this is
                "aes-256-ecb"
            ).unwrap(),
            liquid_core::value!("testuser@getblueshift.com")
        );
    }


}

