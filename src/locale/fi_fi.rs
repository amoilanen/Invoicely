use crate::locale::translations::{Translations, InvoiceTranslations, AccountTranslations, LineTranslations};

pub const FI_FI: Translations = Translations {
    invoice: InvoiceTranslations {
        invoice: "Lasku",
        number: "Laskunumero",
        date: "Päiväys",
        due_date: "Eräpäivä",
        reference_number: "Viitenumero",
        total_price_without_tax: "Veroton hinta yhteensä",
        total_price: "Summa yhteensä",
        vat: "Alv",
        line: LineTranslations {
            item: "Tuote",
            quantity: "Määrä",
            price: "Hinta(sis. ALV)",
            price_without_tax: "Veroton hinta",
            vat: "ALV"
        }
    },
    company_id: "Yritystunnus",
    vat_id: "ALV tunnus",
    account: AccountTranslations {
        number: "Tilinumero",
        bic: "BIC-koodi"
    }
};