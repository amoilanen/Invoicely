use anyhow::Error;
use printpdf::*;
use crate::invoice::Invoice;
use crate::locale::{translations::Translations};
use crate::fonts::FontManager;

pub(crate) struct RenderingContext {
    pub(crate) translations: &'static Translations,
    pub(crate) currency: String,
    pub(crate) locale: String,
    pub(crate) regular_font_id: FontId,
    pub(crate) bold_font_id: FontId
}

pub(crate) fn init_rendering_context(doc: &mut PdfDocument, invoice: &Invoice, translations: &'static Translations, locale: &str) -> Result<RenderingContext, Error> {
    let currency = if &invoice.currency == "EUR" {
        "€"
    } else {
        &invoice.currency
    };

    let font_manager = FontManager::initialize(doc)?;
    let regular_font_id = font_manager.regular_font();
    let bold_font_id = font_manager.bold_font();    

    Ok(RenderingContext {
        translations,
        currency: currency.to_string(),
        locale: locale.to_string(),
        regular_font_id,
        bold_font_id
    })
}