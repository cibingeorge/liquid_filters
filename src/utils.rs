use liquid_core::Result;
use bigdecimal::num_traits::Pow;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use liquid_core::ValueView;
use liquid_core::model::KStringCow;
use std::borrow::Cow;
use crate::filters::invalid_input;
use crate::RenderContext;


pub(crate) fn number_with_precision(number_str: &str,
    thousands_delimiter: Option<char>, // Sets the thousands delimiter (defaults to "").
    fractional_separator: Option<char>, // Sets the separator between the fractional and integer digits (defaults to ".").
    precision: Option<i64>,
    significant: Option<u64>,
    strip_insignificant_zeros: Option<bool>,
    _format: Option<&str>,
) -> Result<String>
{
    let number = number_str.parse::<f64>().map_err(|_x| {
        invalid_input("Number expected")
    })?;

    let separator = fractional_separator.unwrap_or('.');
    let delimiter = thousands_delimiter.unwrap_or(',');
    let mut precision = precision.unwrap_or(3) as i32;
    let rounded_number: BigDecimal;
    if significant.is_some() && precision > 0 {
        let digits = if number == 0.0 {
            rounded_number = BigDecimal::default();
            1
        } else {
            let digits = (number.log10() + 1.0).floor() as i32;
            let prec_10 = 10.0.pow(digits - precision);
            let div: f64 = number/prec_10;
            let rounded = div.round() * prec_10;
            rounded_number = bigdecimal::BigDecimal::from_f64(rounded).ok_or_else(|| invalid_input("Number expected"))?;
            digits
        };
        precision = (precision - digits).abs();
    } else {
        let prec_10 = (10.pow(precision as u32)) as f64;
        let b = BigDecimal::from_f64(number * prec_10).ok_or_else(|| invalid_input("Number expected"))?;
        rounded_number = b.round(0)/ prec_10;
    }

    let formatted = format!("{:01.prec$}", rounded_number, prec = precision as usize);
    // Strip zeros before running thru number formatter. that way a . separates the decimal part
    let formatted = if strip_insignificant_zeros.unwrap_or(false) {
        fn_strip_insignificant_zeros(&formatted)
    } else {
        Cow::Owned(formatted)
    };

    let formatted_number = number_format(&formatted,
        delimiter,
        separator
    );

    Ok(formatted_number.to_string())
}


pub(crate) fn fn_strip_insignificant_zeros(num: &str) -> Cow<'_, str> {
    if !num.contains('.') {
        return Cow::Borrowed(num);
    }
    let (first_part, decimal) = num.rsplit_once('.').unwrap();
    let trimmed = decimal.trim_end_matches('0');
    if trimmed.is_empty() {
        return Cow::Borrowed(first_part);
    }

    Cow::Owned(format!("{}.{}", first_part, trimmed))
}

pub(crate) fn number_format(number: &str,
    thousands_delimiter: char, // Sets the thousands delimiter (defaults to "").
    fractional_separator: char, // Sets the separator between the fractional and integer digits (defaults to ".").
) -> String
{
    let parts: Vec<&str> = number.split('.').collect();

    let mut chars = parts[0].chars();
    let is_neg = if parts[0].starts_with('-') {
        chars.next();
        true
    } else {
        false
    };
    let mut final_vec = vec![];
    for (pos, char) in chars.rev().enumerate() {
        if pos > 0 && pos % 3 == 0 {
            final_vec.push(thousands_delimiter);
            final_vec.push(char);
        } else {
            final_vec.push(char);
        }
    }
    if is_neg {
        final_vec.push('-');
    }
    final_vec.reverse();
    // Decimal part
    if parts.len() > 1 {
        final_vec.push(fractional_separator);
        for char in parts[1].chars() {
            final_vec.push(char);
        }
    }

    String::from_iter(final_vec)
}


pub(crate) fn default_currency_type(render_context: &Option<RenderContext>) -> String {
    if let Some(cxt) = render_context {
        cxt.currency_type.to_string()
    } else {
        "USD".to_owned()
    }
}

pub(crate) fn currency_format(currency: &str) -> currency_rs::CurrencyOpts {
    let currency = currency.to_ascii_uppercase();
    let currency_hash = crate::currency_config::currency_config(&currency);

    if let Some(config) = currency_hash {
        let number_format = if config.is_symbol_prefix {
            "! #"
        } else {
            "# !"
        };
        currency_rs::CurrencyOpts::new()
            .set_symbol(if config.symbol.is_empty() { "$" } else {config.symbol} )
            .set_separator(if config.separator.is_empty() { "," } else {config.separator})
            .set_decimal(if config.delimiter.is_empty() { "." } else {config.delimiter })
            .set_pattern(number_format)
            .set_precision(if config.precision.is_none() { 2 } else {config.precision.unwrap() })
    } else {
        let number_format = "# !";
        currency_rs::CurrencyOpts::new()
            .set_symbol("$")
            .set_separator(",")
            .set_decimal(".")
            .set_pattern(number_format)
            .set_precision(2)
    }

}

pub(crate) fn format_currency(
    render_context: &Option<RenderContext>,
    value: &dyn ValueView,
    use_symbol: Option<bool>,
    use_space: Option<bool>,
    currency_type: Option<KStringCow<'_>>,
)  -> Result<String> {
    let value = if let Some(arr) = value.as_array() {
        if let Some(val) = arr.first() {
            val.to_kstr()
        } else {
            return Ok("".to_string());
        }
    } else {
        value.to_kstr()
    };

    if value.is_empty() {
        return Ok("".to_string());
    }

    let currency = match currency_rs::Currency::new_string(value.as_str(), None) {
        Ok(v) => v,
        Err(_) => {
            return Ok(value.to_string());
        },
    };
    let opt = currency_rs::CurrencyOpts::new()
        .set_symbol("")
        .set_separator(",")
        .set_precision(2)
        .set_decimal(".");

    if !use_symbol.unwrap_or(true) {
        let formatted = currency_rs::Currency::new_cur(currency, Some(opt)).format();
        return Ok(formatted);
    }

    let currency_type = currency_type.map(|x| x.to_string()).unwrap_or_else(|| {
        default_currency_type(render_context)
    });
    let currency_type = if currency_type.is_empty() {
        default_currency_type(render_context)
    } else {
        currency_type
    };
    let currency_opt = currency_format(&currency_type);
    let mut formatted_as_money = currency_rs::Currency::new_cur(currency, Some(currency_opt)).format();
    if !use_space.unwrap_or(true) {
        formatted_as_money = formatted_as_money.replace(' ', "");
    }

    Ok(formatted_as_money)
}

