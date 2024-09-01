use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());

    let currencies = include_str!("config/currency.json");
    let value = serde_json::from_str::<serde_json::Value>(currencies).expect("invalid currency.json");

    let mut map = phf_codegen::Map::new();
    if let Some(arr) = value["currency_types"].as_array() {
        for val in arr {
            map.entry(val["value"].as_str().unwrap(), &format!("CurrencyConfig{{
                value: \"{}\",
                name: \"{}\",
                symbol: \"{}\",
                is_symbol_prefix: {},
                separator: \"{}\",
                delimiter: \"{}\",
                precision: Some({}),
            }}",
                val["value"].as_str().unwrap(),
                val["delimiter"].as_str().unwrap(),
                val["symbol"].as_str().unwrap(),
                val["is_symbol_prefix"].as_bool().unwrap(),
                val["separator"].as_str().unwrap(),
                val["delimiter"].as_str().unwrap(),
                val["precision"].as_i64().unwrap_or(2),
            ));
        }
    }

    write!(
        &mut file,
        "static CURRENCY_CONFIG: phf::Map<&'static str, CurrencyConfig> = {}", map.build()
    )
    .unwrap();
    writeln!(&mut file, ";").unwrap();
}