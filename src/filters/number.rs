#![allow(clippy::invisible_characters)]

use liquid_core::Expression;
use liquid_core::FilterParameters;
use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, FromFilterParameters, ParseFilter};
use liquid_core::{Value, ValueView};

use crate::utils::number_format;
use crate::utils::number_with_precision;

use super::invalid_input;



#[derive(Debug, FilterParameters)]
struct NumberWithPrecisionArgs {
    #[parameter(description = "Sets the thousands precision (defaults to “,”).", arg_type = "str")]
    thousands_delimiter: Option<Expression>,

    #[parameter(description = " Sets the separator between the fractional and integer digits (defaults to “.”).", arg_type = "str")]
    fractional_separator: Option<Expression>,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "number_with_precision",
    description = "Formats a number.",
    parameters(NumberWithPrecisionArgs),
    parsed(NumberWithPrecisionFilter)
)]
pub struct NumberWithPrecision;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "number_with_precision"]
struct NumberWithPrecisionFilter {
    #[parameters]
    args: NumberWithPrecisionArgs,
}

impl Filter for NumberWithPrecisionFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;

        let value = if let Some(scalar) = &input.as_scalar() {
            if scalar.to_float().is_none() {
                return Ok(Value::scalar(input.to_kstr().to_string()));
            }
            input.to_kstr()
        } else {
            return Ok(Value::scalar(input.to_kstr().to_string()));
        };

        let mut separator = args.fractional_separator.map(|x| x.to_string()).unwrap_or_default();
        if separator.is_empty() {
            separator = ".".to_owned();
        }

        let mut precision = args.thousands_delimiter.map(|x| x.to_string()).unwrap_or_default();
        if precision.is_empty() {
            precision = ",".to_owned();
        }

        let value = value.as_str();
        let formatted = number_format(
            value,
            precision.chars().next().unwrap(),
            separator.chars().next().unwrap(),
        );
        Ok(Value::scalar(formatted))
    }
}



#[derive(Debug, FilterParameters)]
struct NumberWithDelimiterArgs {
    #[parameter(description = "Sets the thousands delimiter (defaults to “,”).", arg_type = "str")]
    thousands_delimiter: Option<Expression>,

    #[parameter(description = " Sets the separator between the fractional and integer digits (defaults to “.”).", arg_type = "str")]
    fractional_separator: Option<Expression>,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "number_with_delimiter",
    description = "Formats a number.",
    parameters(NumberWithDelimiterArgs),
    parsed(NumberWithDelimiterFilter)
)]
pub struct NumberWithDelimiter;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "number_with_delimiter"]
struct NumberWithDelimiterFilter {
    #[parameters]
    args: NumberWithDelimiterArgs,
}

impl Filter for NumberWithDelimiterFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;

        let value = if let Some(scalar) = &input.as_scalar() {
            if scalar.to_float().is_none() {
                return Ok(Value::scalar(input.to_kstr().to_string()));
            }
            input.to_kstr()
        } else {
            return Ok(Value::scalar(input.to_kstr().to_string()));
        };

        let mut separator = args.fractional_separator.map(|x| x.to_string()).unwrap_or_default();
        if separator.is_empty() {
            separator = ".".to_owned();
        }

        let mut delimiter = args.thousands_delimiter.map(|x| x.to_string()).unwrap_or_default();
        if delimiter.is_empty() {
            delimiter = ",".to_owned();
        }

        let value = value.as_str();
        let formatted = number_format(
            value,
            delimiter.chars().next().unwrap(),
            separator.chars().next().unwrap(),
        );
        Ok(Value::scalar(formatted))
    }
}

#[derive(Debug, FilterParameters)]
struct NumberToPercentageArgs {
    #[parameter(description = "Sets the thousands delimiter (defaults to “,”).", arg_type = "str")]
    thousands_delimiter: Option<Expression>,

    #[parameter(description = " Sets the separator between the fractional and integer digits (defaults to “.”).", arg_type = "str")]
    fractional_separator: Option<Expression>,

    #[parameter(description = "precision", arg_type = "integer", mode = "keyword")]
    precision: Option<Expression>,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "number_to_percentage",
    description = "Number to percentage",
    parameters(NumberToPercentageArgs),
    parsed(NumberToPercentageFilter)
)]
pub struct NumberToPercentage;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "number_to_percentage"]
struct NumberToPercentageFilter {
    #[parameters]
    args: NumberToPercentageArgs,
}

impl Filter for NumberToPercentageFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;
        let number = input.to_kstr().to_string();

        let mut fractional_separator = args.fractional_separator.map(|x| x.to_string()).unwrap_or_default();
        if fractional_separator.is_empty() {
            fractional_separator = ".".to_owned();
        }

        let mut thousands_delimiter = args.thousands_delimiter.map(|x| x.to_string()).unwrap_or_default();
        if thousands_delimiter.is_empty() {
            thousands_delimiter = ",".to_owned();
        }

        let  precision = args.precision;
        let formatted = number_with_precision(
            &number,
            thousands_delimiter.chars().next(),
            fractional_separator.chars().next(),
            precision,
            None,
            None,
            None,
        )?;

        Ok(Value::scalar(format!("{}%", formatted)))
    }
}



#[derive(Debug, FilterParameters)]
struct NumberToCurrencyArgs {
    #[parameter(description = "Sets the thousands delimiter (defaults to “,”).", arg_type = "str", mode = "keyword")]
    delimiter: Option<Expression>,

    #[parameter(description = " Sets the separator between the fractional and integer digits (defaults to “.”).", arg_type = "str", mode = "keyword")]
    separator: Option<Expression>,

    #[parameter(description = "Sets the denomination of the currency (defaults to '$').", arg_type = "str", mode = "keyword")]
    unit: Option<Expression>,

    #[parameter(description = "Sets the level of precision (defaults to 2)", arg_type = "integer", mode = "keyword")]
    precision: Option<Expression>,

    #[parameter(description = "Sets the format of the output string (defaults to \"%u%n\"). The field types are:\n\t%u  The currency unit\n\t%n  The number", arg_type = "str", mode = "keyword")]
    format: Option<Expression>,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "number_to_currency",
    description = "Number to currency",
    parameters(NumberToCurrencyArgs),
    parsed(NumberToCurrencyFilter)
)]
pub struct NumberToCurrency;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "number_to_currency"]
struct NumberToCurrencyFilter {
    #[parameters]
    args: NumberToCurrencyArgs,
}

impl Filter for NumberToCurrencyFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;
        let number = input.to_kstr().to_string();
        if number.is_empty() {
            return Ok(Value::scalar(""));
        }

        let fractional_separator = args.separator.map(|x| x.to_string()).unwrap_or(".".to_owned());
        let thousands_delimiter = args.delimiter.map(|x| x.to_string()).unwrap_or(",".to_owned());
        let  precision = args.precision.unwrap_or(2);

        let value = number_with_precision(
            &number,
            thousands_delimiter.chars().next(),
            fractional_separator.chars().next(),
            Some(precision),
            None,
            None,
            None,
        );

        let unit = args.unit.map(|x| x.to_string()).unwrap_or("$".to_owned());
        let fmt = args.format.map(|x| x.to_string()).unwrap_or("%u%n".to_owned());

        let formatted = if let Ok(value) = value {
            fmt.replace("%u", &unit).replace("%n", &value)
        } else {
            fmt.replace("%u", &unit).replace("%n", &number)
        };

        Ok(Value::scalar(formatted))
    }
}



#[derive(Debug, FilterParameters)]
struct NumberBetweenArgs {
    #[parameter(description = "The low value.", arg_type = "integer")]
    low: Option<Expression>,

    #[parameter(description = "The high value).", arg_type = "integer")]
    high: Option<Expression>,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "between",
    description = "Checks whether a given number is between 2 numbers.",
    parameters(NumberBetweenArgs),
    parsed(NumberBetweenFilter)
)]
pub struct NumberBetween;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "between"]
struct NumberBetweenFilter {
    #[parameters]
    args: NumberBetweenArgs,
}

impl Filter for NumberBetweenFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;

        let low = args.low.ok_or_else(|| {
            super::invalid_argument("low", "required argument low is missing")
        })?;

        let high = args.high.ok_or_else(|| {
            super::invalid_argument("high", "required argument high is missing")
        })?;


        if let Some(scalar) = &input.as_scalar() {
            if let Some(flt) = scalar.to_float() {
                return Ok(Value::scalar((flt > low as f64) && (flt < high as f64)));
            } else if let Some(int) = scalar.to_integer() {
                return Ok(Value::scalar((int > low) && (int < high)));
            }
        }
        Err(invalid_input("Number expected"))
    }
}


#[derive(Debug, FilterParameters)]
struct NumberMoreThanArgs {
    #[parameter(description = "The reference value.", arg_type = "integer")]
    reference: Option<Expression>,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "more_than",
    description = "Checks whether a given number is more than the reference number.",
    parameters(NumberMoreThanArgs),
    parsed(NumberMoreThanFilter)
)]
pub struct NumberMoreThan;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "more_than"]
struct NumberMoreThanFilter {
    #[parameters]
    args: NumberMoreThanArgs,
}

impl Filter for NumberMoreThanFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;

        let reference = args.reference.ok_or_else(|| {
            super::invalid_argument("reference", "required argument is missing")
        })?;

        if let Some(scalar) = &input.as_scalar() {
            if let Some(flt) = scalar.to_float() {
                return Ok(Value::scalar(flt > reference as f64));
            } else if let Some(int) = scalar.to_integer() {
                return Ok(Value::scalar(int > reference));
            }
        }
        Err(invalid_input("Number expected"))
    }
}


#[derive(Debug, FilterParameters)]
struct NumberLessThanArgs {
    #[parameter(description = "The reference value.", arg_type = "integer")]
    reference: Option<Expression>,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "less_than",
    description = "Checks whether a given number is less than the reference number.",
    parameters(NumberLessThanArgs),
    parsed(NumberLessThanFilter)
)]
pub struct NumberLessThan;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "less_than"]
struct NumberLessThanFilter {
    #[parameters]
    args: NumberLessThanArgs,
}

impl Filter for NumberLessThanFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;

        let reference = args.reference.ok_or_else(|| {
            super::invalid_argument("reference", "required argument is missing")
        })?;

        if let Some(scalar) = &input.as_scalar() {
            if let Some(flt) = scalar.to_float() {
                return Ok(Value::scalar(flt < reference as f64));
            } else if let Some(int) = scalar.to_integer() {
                return Ok(Value::scalar(int < reference));
            }
        }
        Err(invalid_input("Number expected"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_number_format() {
        assert_eq!(number_format("-12345678.00", ',', '.'), "-12,345,678.00");
        assert_eq!(number_format("-123456789.00", ',', '.'), "-123,456,789.00");
        assert_eq!(number_format("8", ',', '.'), "8");
        assert_eq!(number_format("-8", ',', '.'), "-8");
        assert_eq!(number_format("678", ',', '.'), "678");
        assert_eq!(number_format("-678", ',', '.'), "-678");
    }

    #[test]
    fn unit_number_with_delimiter() {
        assert_eq!(
            liquid_core::call_filter!(NumberWithDelimiter, "-12345678.00", ",", ".").unwrap(),
            liquid_core::value!("-12,345,678.00")
        );
        assert_eq!(
            liquid_core::call_filter!(NumberWithDelimiter, "-123456789.00", ",", ".").unwrap(),
            liquid_core::value!("-123,456,789.00")
        );
        assert_eq!(
            liquid_core::call_filter!(NumberWithDelimiter, "8", ",", ".").unwrap(),
            liquid_core::value!("8")
        );
        assert_eq!(
            liquid_core::call_filter!(NumberWithDelimiter, "-8", ",", ".").unwrap(),
            liquid_core::value!("-8")
        );
        assert_eq!(
            liquid_core::call_filter!(NumberWithDelimiter, "678", ",", ".").unwrap(),
            liquid_core::value!("678")
        );
        assert_eq!(
            liquid_core::call_filter!(NumberWithDelimiter, "-678", ",", ".").unwrap(),
            liquid_core::value!("-678")
        );

    }

    #[test]
    fn unit_number_with_precision() {
        assert_eq!(
            number_with_precision("5.10", Some(','), Some('.'), None, None, None, None).unwrap(),
            "5.100"
        );
        assert_eq!(
            number_with_precision("0.1", Some(','), Some('.'), None, None, None, None).unwrap(),
            "0.100"
        );
        assert_eq!(
            number_with_precision("0.1", Some(','), Some('.'), None, None, Some(true), None).unwrap(),
            "0.1"
        );

        assert_eq!(
            number_with_precision("6.1", Some(','), Some('.'), None, None, None, None).unwrap(),
            "6.100"
        );
        assert_eq!(
            number_with_precision("-12345678.1236", Some(','), Some('.'), None, None, None, None).unwrap(),
            "-12,345,678.124"
        );
        assert_eq!(
            number_with_precision("-12345678.1236", Some('.'), Some(','), None, None, None, None).unwrap(),
            "-12.345.678,124"
        );

        assert_eq!(
            number_with_precision("100", None, None, None, None, None, None).unwrap(),
            "100.000"
        );
        assert_eq!(
            number_with_precision("100", None, None, Some(0), None, None, None).unwrap(),
            "100"
        );
        assert_eq!(
            number_with_precision("1000", Some('.'), Some(','), None, None, None, None).unwrap(),
            "1.000,000"
        );
        assert_eq!(
            number_with_precision("302.24398923423", Some(','), Some('.'), Some(5), None, None, None).unwrap(),
            "302.24399"
        );

        assert_eq!(
            number_with_precision("2.79336291208791", None, Some('.'), Some(2), None, None, None).unwrap(),
            "2.79"
        );


    }
}