use bigdecimal::BigDecimal;

pub fn format_vat(value: &f32) -> String {
    format!("{:.1}", value).replace(".", ",")
}

pub fn format_price(value: &BigDecimal, currency: &str, locale: &str) -> String {
    let formatted = format!("{:.2} {}", value, currency);
    if locale != "en-GB" {
        formatted.replace(".", ",")
    } else {
        formatted
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::FromPrimitive;

    use super::*;

    #[test]
    fn test_format_price() {
        assert_eq!(format_price(&BigDecimal::from_f32(1.212423).unwrap(), "EUR", "fi-FI"), "1,21 EUR");
        assert_eq!(format_price(&BigDecimal::from_f32(2.3450).unwrap(), "USD", "fi-FI"), "2,35 USD");
        assert_eq!(format_price(&BigDecimal::from_f32(2.30).unwrap(), "€", "fi-FI"), "2,30 €");
        assert_eq!(format_price(&BigDecimal::from_f32(2.00).unwrap(), "EUR", "fi-FI"), "2,00 EUR");
        assert_eq!(format_price(&BigDecimal::from_f32(100.00).unwrap(), "EUR", "fi-FI"), "100,00 EUR");
        assert_eq!(format_price(&BigDecimal::from_f32(5400.12).unwrap(), "EUR", "fi-FI"), "5400,12 EUR");
        assert_eq!(format_price(&BigDecimal::from_f32(1.212423).unwrap(), "EUR", "en-GB"), "1.21 EUR");
    }

    #[test]
    fn test_format_vat() {
        assert_eq!(format_vat(&25.5), "25,5");
        assert_eq!(format_vat(&14.0), "14,0");
    }
}