#![allow(clippy::invisible_characters)]

use liquid_core::Expression;
use liquid_core::FilterParameters;
use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter, FromFilterParameters};
use liquid_core::{Value, ValueView};

use super::invalid_argument;
use super::invalid_input;
#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "camelcase",
    description = "Makes each character in a string camelcase.",
    parsed(CamelcaseFilter)
)]
pub struct Camelcase;

#[derive(Debug, Default, Display_filter)]
#[name = "camelcase"]
struct CamelcaseFilter;

impl Filter for CamelcaseFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        let collected: Vec<String> = s.as_str().split(' ').filter(|x| !x.is_empty()).map(ruby_camelize).collect();
        //let collected: String = cruet::to_title_case(s.as_str());
        Ok(Value::scalar(collected.join(" ")))
    }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "escape_newlines",
    description = "Replaces \\n and \\r with the equivalent character.",
    parsed(EscapeNewlineFilter)
)]
pub struct EscapeNewline;

#[derive(Debug, Default, Display_filter)]
#[name = "escape_newlines"]
struct EscapeNewlineFilter;

impl Filter for EscapeNewlineFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let s = input.to_kstr();
        let replaced = s.as_str().replace('\n', "\\n").replace('\r', "\\r");
        Ok(Value::scalar(replaced))
    }
}

#[derive(Debug, FilterParameters)]
struct AnyContainsArgs {
    #[parameter(description = "Format ISO 8601 compliant datetime into a given timezone.", arg_type = "str")]
    search_str: Option<Expression>,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "any_contains",
    description = "convert time to given timezone",
    parameters(AnyContainsArgs),
    parsed(AnyContainsFilter)
)]
pub struct AnyContains;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "any_contains"]
struct AnyContainsFilter {
    #[parameters]
    args: AnyContainsArgs,
}

impl Filter for AnyContainsFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;
        let search_str = if let Some(s) = args.search_str.map(|x| x.to_string()) {
            s
        } else {
            return Err(invalid_argument("search_string", "is_required"));
        };

        if search_str.is_empty() {
            return Err(invalid_argument("search_string", "is empty"));
        }

        if let Some(scalar) = input.as_scalar() {
            if let Some(scalar_str) = scalar.as_str() {
                return Ok(Value::scalar(scalar_str.contains(&search_str)));
            }
            return Ok(Value::scalar(false));
        }

        let array = input
            .as_array()
            .ok_or_else(|| invalid_input("Array or string expected"))?;

        for x in array.values() {
            if let Some(scalar) = x.as_scalar() {
                if let Some(scalar_str) = scalar.as_str() {
                    if scalar_str.contains(&search_str) {
                        return Ok(Value::scalar(true));
                    }
                }
            }
        }

        Ok(Value::scalar(false))
    }
}




// uppercase_first_letter is always true
pub fn ruby_camelize(input: &str) -> String {
    let upcased = uppercase_first_letter(input);

    enum ChIs {
        UpcaseNext,
        Others,
    }
    let mut flag = ChIs::Others;

    let mut result = String::with_capacity(input.len());

    for ch in upcased.chars() {
        match flag {
            ChIs::UpcaseNext => {
                if ch == '_' {
                } else if ch == '/' {
                    result.push(':');
                    result.push(':');
                } else {
                    result.push(ch.to_ascii_uppercase());
                    flag = ChIs::Others;
                }
            },
            ChIs::Others => {
                if ch == '_' {
                    // do nothing
                    flag = ChIs::UpcaseNext;
                } else if ch == '/' {
                    result.push(':');
                    result.push(':');
                    flag = ChIs::UpcaseNext;
                } else {
                    result.push(ch);
                }
            },
        }
    }

    result
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[allow(dead_code)]
pub fn capitalize(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    // .len returns byte count but ok in this case!

    for (idx,ch) in input.chars().enumerate() {
        if idx == 0 {
            if ch.is_ascii_lowercase() {
                result.push(ch.to_ascii_uppercase());
            } else {
                result.push(ch);
            }
        } else if ch.is_ascii_uppercase() {
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_capitalize() {
        assert_eq!(capitalize("hello"), String::from("Hello"));
        assert_eq!(capitalize("HELLO"), String::from("Hello"));
        assert_eq!(capitalize("123ABC"), String::from("123abc"));
        assert_eq!(capitalize("123abc"), String::from("123abc"));
        assert_eq!(capitalize("123abC"), String::from("123abc"));
        assert_eq!(capitalize("123 abc"), String::from("123 abc"));
        assert_eq!(capitalize("123 abC"), String::from("123 abc"));
        assert_eq!(capitalize("abc-abc"), String::from("Abc-abc"));
    }

    #[test]
    fn unit_camelcase() {
        assert_eq!(
            liquid_core::call_filter!(Camelcase, "").unwrap(),
            liquid_core::value!("")
        );
        assert_eq!(
            liquid_core::call_filter!(Camelcase, "foo_bar_baz").unwrap(),
            liquid_core::value!("FooBarBaz")
        );

        assert_eq!(
            liquid_core::call_filter!(Camelcase, "foo/bar/baz").unwrap(),
            liquid_core::value!("Foo::Bar::Baz")
        );

        assert_eq!(
            liquid_core::call_filter!(Camelcase, "pink polka-dot polka%dot poster").unwrap(),
            liquid_core::value!("Pink Polka-dot Polka%dot Poster")
        );

    }
}
