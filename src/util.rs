use bigdecimal::BigDecimal;
use chrono::prelude::*;

pub fn string_to_timestamp(timestamp_str: &str) -> Option<i64> {
    if let Ok(parsed_time) = NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S %Z") {
        Some(parsed_time.timestamp())
    } else {
        None
    }
}

pub fn bigdecimal_fractional_count(big_decimal: BigDecimal) -> u32 {
    if big_decimal.fractional_digit_count() <= 0 {
        return 0;
    }

    let big_decimal_str = big_decimal.to_string().trim_end_matches('0').to_string();
    (if let Some(decimal_pos) = big_decimal_str.find('.') {
        big_decimal_str.len() - decimal_pos - 1
    } else {
        0
    }) as u32
}
