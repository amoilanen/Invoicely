#[derive(Debug, Clone)]
pub struct Translations {
    pub invoice: InvoiceTranslations,
    pub company_id: &'static str,
    pub vat_id: &'static str,
    pub account: AccountTranslations
}

#[derive(Debug, Clone)]
pub struct InvoiceTranslations {
    pub invoice: &'static str,
    pub number: &'static str,
    pub date: &'static str,
    pub due_date: &'static str,
    pub reference_number: &'static str,
    pub total_price_without_tax: &'static str,
    pub total_price: &'static str,
    pub vat: &'static str,
    pub line: LineTranslations
}

#[derive(Debug, Clone)]
pub struct AccountTranslations {
    pub number: &'static str,
    pub bic: &'static str
}

#[derive(Debug, Clone)]
pub struct LineTranslations {
    pub item: &'static str,
    pub quantity: &'static str,
    pub price: &'static str,
    pub price_without_tax: &'static str,
    pub vat: &'static str
}