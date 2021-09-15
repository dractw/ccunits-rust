extern crate rust_decimal;
extern crate wasm_bindgen;

mod util;

use rust_decimal::prelude::*;
use wasm_bindgen::prelude::*;


fn format_currency_str (amount_str: Box<str>, decimal: &usize) -> String {
    let formatted = Decimal::from_str(&*amount_str).unwrap();

    format!("{:.1$}", formatted, decimal)
}

fn format_minimal_str (amount_str: Box<str>, decimal: &usize) -> String {
    let str = String::from(amount_str);
    // let formatted: String = str.chars().take(*decimal).collect();

    str.to_string()
}

#[wasm_bindgen]
pub fn from_minimal (amount: &str, decimal: usize) -> String {
    let formatted_amount = format_minimal_str(Box::from(amount), &decimal);
    let satoshis  = 10_i64.pow(decimal.to_u32().unwrap());

    (formatted_amount.parse::<f64>().unwrap() / satoshis as f64).to_string()
}

#[wasm_bindgen]
pub fn to_minimal (amount: &str, decimal: usize) -> String {
    let formatted_amount = format_currency_str(Box::from(amount), &decimal);
    let satoshis  = 10_i64.pow(decimal.to_u32().unwrap());

    (formatted_amount.parse::<f64>().unwrap() * satoshis as f64).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_test() {
        assert_eq!(to_minimal("134.677452125", 18), "134677452125000000000");
    }

    #[test]
    fn cur_test() {
        assert_eq!(from_minimal("134677452125000000000", 18), "134.677452125");
    }
}