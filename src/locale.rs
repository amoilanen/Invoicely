use anyhow::Error;

pub mod en_gb;
pub mod fi_fi;
pub mod translations;

use en_gb::EN_GB;
use fi_fi::FI_FI;
use translations::Translations;

pub fn get_translations(locale: &str) -> Result<&'static Translations, Error> {
    match locale {
        "en-GB" => Ok(&EN_GB),
        "fi-FI" => Ok(&FI_FI),
        _ => Err(Error::msg(format!("Unsupported locale: {}", locale)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_finnish_translations() {
        let translations = get_translations("fi-FI").unwrap();
        assert_eq!(translations.company_id, "Yritystunnus");
        assert_eq!(translations.invoice.line.price_without_tax, "Veroton hinta");
    }

    #[test]
    fn should_get_english_translations() {
        let translations = get_translations("en-GB").unwrap();
        assert_eq!(translations.company_id, "Registration number");
        assert_eq!(translations.invoice.line.price_without_tax, "Price without tax");
    }

    #[test]
    fn should_return_error_for_unsupported_locale() {
        let result = get_translations("fr-FR");
        assert!(result.is_err());
    }
}