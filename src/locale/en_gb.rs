use crate::locale::translations::{Translations, InvoiceTranslations, AccountTranslations, LineTranslations};

pub const EN_GB: Translations = Translations {
    invoice: InvoiceTranslations {
        invoice: "Invoice",
        number: "Invoice number",
        date: "Date",
        due_date: "Due date",
        reference_number: "Reference number",
        total_price_without_tax: "Total price without tax",
        total_price: "Total price",
        vat: "VAT",
        line: LineTranslations {
            item: "Item",
            quantity: "Quantity",
            price: "Price",
            price_without_tax: "Price without tax",
            vat: "VAT"
        }
    },
    company_id: "Registration number",
    vat_id: "VAT ID",
    account: AccountTranslations {
        number: "Account number",
        bic: "BIC"
    }
};