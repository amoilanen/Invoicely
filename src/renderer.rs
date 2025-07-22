use anyhow::Error;
use printpdf::*;
use crate::components::Component;
use crate::invoice::Invoice;
use bigdecimal::{BigDecimal, FromPrimitive};
use crate::format::{format_price, format_vat};
use crate::components::table::Table;
use crate::components::label::Label;
use crate::locale::get_translations;
use crate::image::load_image;
use crate::renderer::rendering_context::{init_rendering_context, RenderingContext};

mod rendering_context;

pub fn render(invoice: &Invoice) -> Result<PdfDocument, Error> {
    let translations = get_translations(&invoice.locale)?;
    let mut doc: PdfDocument = PdfDocument::new(&format!("{} {}", translations.invoice.invoice, invoice.invoice_number));
    let rendering_context= init_rendering_context(&mut doc, invoice, translations, &invoice.locale)?;

    let invoice_parts = vec![
        if let Some(logo_url) = &invoice.billed_by.logo {
            let image = load_image(logo_url)?;
            logo(&mut doc, &image)?
        } else {
            Vec::new()
        },
        Label::new(rendering_context.translations.invoice.invoice, 22.0, &rendering_context.bold_font_id).render_at(110.0, 270.0),
        invoice_info(invoice, &rendering_context).render_at(110.0, 260.0),
        billed_to(invoice, &rendering_context).render_at(15.0, 260.0),
        invoice_lines(invoice, &rendering_context).render_at(15.0, 200.0),
        summary(invoice, &rendering_context).render_at(125.0, 164.0),
        if let Some(note) = invoice.note.as_ref() {
            Label::new(note, 10.0, &rendering_context.regular_font_id).render_at(15.0, 140.0)
        } else {
            Vec::new()
        },
        if let Some(description) = invoice.invoice_description.as_ref() {
            Label::new(description, 10.0, &rendering_context.regular_font_id).render_at(15.0, 130.0)
        } else {
            Vec::new()
        },
        vec![
            Op::SetOutlineThickness { pt: Pt(0.8) },
            Op::DrawLine { 
                line: Line {
                    points: vec![
                        LinePoint { p: Point::new(Mm(15.0), Mm(25.0)), bezier: false },
                        LinePoint { p: Point::new(Mm(200.0), Mm(25.0)), bezier: false },
                    ],
                    is_closed: false
                }
            }
        ],
        billed_by(invoice, &rendering_context).render_at(25.0, 20.0)
    ];
    let mut page_contents = Vec::new();
    for ops in invoice_parts {
        page_contents.extend(ops);
    }
    let page = PdfPage::new(Mm(210.0), Mm(297.0), page_contents);
    doc.with_pages(vec![page]);
    Ok(doc)
}

fn logo(doc: &mut PdfDocument, logo_image: &RawImage) -> Result<Vec<Op>, Error> {
    let image_xobject_id = doc.add_image(&logo_image);
    
    let transform = XObjectTransform {
        translate_x: Some(Pt(45.0)),
        translate_y: Some(Pt(770.0)),
        rotate: None,
        scale_x: Some(0.5),
        scale_y: Some(0.5),
        dpi: Some(300.0),
    };
    
    Ok(vec![Op::UseXobject { 
        id: image_xobject_id.clone(), 
        transform
    }])
}

fn invoice_info(invoice: &Invoice, rendering_context: &RenderingContext) -> Table {
    let translations = rendering_context.translations;
    let regular_font_id = &rendering_context.regular_font_id;
    Table {
        column_widths: vec![40.0, 30.0],
        row_height: 5.0,
        header: None,
        rows: Label::new_rows(
            vec![
                vec![&format!("{}:", translations.invoice.number), invoice.invoice_number.as_str()],
                vec![&format!("{}:", translations.invoice.date), invoice.billed_at.as_str()],
                vec![&format!("{}:", translations.invoice.due_date), invoice.due_date.as_str()],
                vec![&format!("{}:", translations.invoice.reference_number), invoice.reference_id.as_ref().map(|s| s.as_str()).unwrap_or("")],
                vec![&format!("{}:", translations.account.number), invoice.bank_details.account_number.as_str()],
                vec![&format!("{}:", translations.account.bic), invoice.bank_details.bic_code.as_str()]
            ],
            11.0,
            regular_font_id
        )
    }
}

fn billed_to(invoice: &Invoice, rendering_context: &RenderingContext) -> Table {
    let translations = rendering_context.translations;
    let regular_font_id = &rendering_context.regular_font_id;
    let mut billed_to_lines = vec![
        vec![invoice.billed_to.name.as_str()],
        vec![invoice.billed_to.address_line_1.as_str()]
    ];
    if let Some(address_line_2) = invoice.billed_to.address_line_2.as_ref() {
        billed_to_lines.push(vec![address_line_2.as_str()]);
    }
    if let Some(address_line_3) = invoice.billed_to.address_line_3.as_ref() {
        billed_to_lines.push(vec![address_line_3.as_str()]);
    }
    let company_id_line = if let Some(company_id) = invoice.billed_to.company_id.as_ref() {
        format!("{}: {}", translations.company_id, company_id)
    } else {
        "".to_owned()
    };
    if !company_id_line.is_empty() {
        billed_to_lines.push(vec![company_id_line.as_str()]);
    }
    let vat_id_line = if let Some(vat_id) = invoice.billed_to.vat_id.as_ref() {
        format!("{}: {}", translations.vat_id, vat_id)
    } else {
        "".to_owned()
    };
    if !vat_id_line.is_empty() {
        billed_to_lines.push(vec![vat_id_line.as_str()]);
    }
    let billed_to = Table {
        column_widths: vec![30.0],
        row_height: 5.0,
        header: None,
        rows: Label::new_rows(billed_to_lines, 11.0, &regular_font_id)
    };
    billed_to
}

fn invoice_lines(invoice: &Invoice, rendering_context: &RenderingContext) -> Table {
    let translations = rendering_context.translations;
    let regular_font_id = &rendering_context.regular_font_id;
    let bold_font_id = &rendering_context.bold_font_id;
    let currency = &rendering_context.currency;
    let mut invoice_lines: Vec<Vec<String>> = Vec::new();
    for invoice_line in invoice.invoice_lines.iter() {
        let price_without_vat = &invoice_line.price / BigDecimal::from_f32(1.0 + invoice.vat_percent / 100.0).unwrap();
        invoice_lines.push(vec![
            format!("{}", invoice_line.name),
            format!("{}", invoice_line.count),
            format_price(&invoice_line.price, currency, &rendering_context.locale),
            format_price(&price_without_vat, currency, &rendering_context.locale),
            format_vat(&invoice.vat_percent)
        ]);
    }
    Table {
        column_widths: vec![80.0, 20.0, 30.0, 40.0, 15.0],
        row_height: 5.0,
        header: Some(Label::new_row(vec![
            translations.invoice.line.item, translations.invoice.line.quantity, translations.invoice.line.price,
            translations.invoice.line.price_without_tax, &format!("{} %", translations.invoice.line.vat)
        ], 10.0, &bold_font_id)),
        rows: Label::new_rows(
            invoice_lines.iter().map(|x| x.iter().map(|s| s.as_str()).collect()).collect(),
            10.0,
            &regular_font_id
        )
    }
}

fn summary(invoice: &Invoice, rendering_context: &RenderingContext) -> Table {
    let translations = rendering_context.translations;
    let regular_font_id = &rendering_context.regular_font_id;
    let bold_font_id = &rendering_context.bold_font_id;
    let currency = &rendering_context.currency;

    let total_price: BigDecimal = invoice.invoice_lines.iter().map(|line| &line.price).sum();
    let total_vat: BigDecimal = BigDecimal::from_f32(invoice.vat_percent / 100.0).unwrap() * &total_price;
    let total_price_without_vat = &total_price / BigDecimal::from_f32(1.0 + invoice.vat_percent / 100.0).unwrap();

    Table {
        column_widths: vec![45.0, 30.0],
        row_height: 5.0,
        header: None,
        rows: vec![
            Label::new_row(
                vec![&format!("{}:", translations.invoice.total_price_without_tax), &format_price(&total_price_without_vat, currency, &rendering_context.locale)],
                10.0,
                &regular_font_id
            ),
            Label::new_row(
                vec![&format!("{} {} %:", translations.invoice.vat, &format_vat(&invoice.vat_percent)), &format_price(&total_vat, currency, &rendering_context.locale)],
                10.0,
                &regular_font_id
            ),
            Label::new_row(
                vec![&format!("{}:", translations.invoice.total_price), &format_price(&total_price, currency, &rendering_context.locale)],
                10.0,
                &bold_font_id
            )
        ]
    }
}

fn billed_by(invoice: &Invoice, rendering_context: &RenderingContext) -> Table {
    let translations = rendering_context.translations;
    let regular_font_id = &rendering_context.regular_font_id;
    Table {
        column_widths: vec![60.0, 60.0, 65.0],
        row_height: 3.0,
        header: None,
        rows: Label::new_rows(
            vec![
                vec![invoice.billed_by.name.as_str(), invoice.billed_by.company_id.as_ref().map(|s| format!("{}: {}", translations.company_id, s)).unwrap_or_else(|| "".to_owned()).as_str(), invoice.billed_by.email.as_ref().map(|s| s.as_str()).unwrap_or("")],
                vec![invoice.billed_by.address_line_1.as_str(), invoice.billed_by.vat_id.as_ref().map(|s| format!("{}: {}", translations.vat_id, s)).unwrap_or_else(|| "".to_owned()).as_str(), invoice.billed_by.phone_number.as_ref().map(|s| s.as_str()).unwrap_or("")],
                vec![invoice.billed_by.address_line_2.as_ref().map(|s| s.as_str()).unwrap_or(""), invoice.bank_details.account_number.as_str(), ""],
                vec![invoice.billed_by.address_line_3.as_ref().map(|s| s.as_str()).unwrap_or(""), invoice.bank_details.bic_code.as_str(), ""],
                vec![invoice.billed_by.detail.as_ref().map(|s| s.as_str()).unwrap_or(""), "", ""]
            ],
            7.0,
            &regular_font_id
        )
    }
}