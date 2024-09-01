#![allow(clippy::invisible_characters)]

use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

use phf::phf_map;
use phf::phf_set;


static ALLOWED_CHARS: phf::Set<char> = phf_set! {
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
    '1','2','3','4','5','6','7','8','9','0',
    '_','.','-','~',
};


static PERCENT_ENCODING_MAP: phf::Map<char, &'static str> = phf_map! {
    ':' => "%3A",
    '/' => "%2F",
    '?' => "%3F",
    '#' => "%23",
    '[' => "%5B",
    ']' => "%5D",
    '@' => "%40",
    '!' => "%21",
    '$' => "%24",
    '&' => "%26",
    '\'' => "%27",
    '(' => "%28",
    ')' => "%29",
    '*' => "%2A",
    '+' => "%2B",
    ',' => "%2C",
    ';' => "%3B",
    '=' => "%3D",
    '%' => "%25",
    ' ' => "+",
};


#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "url_encode",
    description = "url_encode a string.",
    parsed(UrlEncodeFilter)
)]
pub struct UrlEncode;

#[derive(Debug, Default, Display_filter)]
#[name = "url_encode"]
struct UrlEncodeFilter;

impl Filter for UrlEncodeFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        let mut chars = String::with_capacity(s.len());
        for ch in s.as_str().chars() {
            if ALLOWED_CHARS.contains(&ch) {
                chars.push(ch);
            } else if let Some(tr) = PERCENT_ENCODING_MAP.get(&ch) {
                for tr_ch in tr.chars() {
                    chars.push(tr_ch);
                }
            } else {
                let mut b = [0; 4];
                ch.encode_utf8(&mut b);
                for i in &b[..ch.len_utf8()] {
                    chars.push_str(&format!("%{:02X}", i));
                }
            }
        }

        Ok(Value::scalar(chars))
    }
}



#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "escape_url",
    description = "url escape a string.",
    parsed(EscapeUrlFilter)
)]
pub struct EscapeUrl;

#[derive(Debug, Default, Display_filter)]
#[name = "escape_url"]
struct EscapeUrlFilter;

impl Filter for EscapeUrlFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();

        let mut chars = String::with_capacity(s.len());
        for ch in s.as_str().chars() {
            if let Some(tr) = PERCENT_ENCODING_MAP.get(&ch) {
                for tr_ch in tr.chars() {
                    chars.push(tr_ch);
                }
            } else {
                chars.push(ch);
            }
        }


        Ok(Value::scalar(chars))
    }
}

