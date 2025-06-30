use printpdf::*;
use super::Component;

pub struct Label {
    pub value: String,
    pub font: IndirectFontRef,
    pub font_size: f32
}

impl Label {
    pub fn new(value: &str, font_size: f32, font: IndirectFontRef) -> Label {
        Label {
            value: value.to_owned(),
            font,
            font_size
        }
    }

    pub fn new_row(row: Vec<&str>, font_size: f32, font: &IndirectFontRef) -> Vec<Box<dyn Component>> {
        let mut label_row: Vec<Box<dyn Component>> = Vec::new();
        for value in row.into_iter() {
            label_row.push(Box::new(Label::new(value, font_size, font.clone())));
        }
        label_row
    }

    pub fn new_rows(rows: Vec<Vec<&str>>, font_size: f32, font: &IndirectFontRef) -> Vec<Vec<Box<dyn Component>>> {
        let mut labels: Vec<Vec<Box<dyn Component>>> = Vec::new();
        for row in rows.into_iter() {
            labels.push(Label::new_row(row, font_size, font));
        }
        labels
    }
}

impl Component for Label {
    fn render_at(&self, x: f32, y:f32,  layer: &PdfLayerReference) {
        layer.use_text(
            &self.value,
            self.font_size,
            Mm(x),
            Mm(y),
            &self.font,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_new() {
        let (doc, page, layer) = PdfDocument::new("test", Mm(210.0), Mm(297.0), "test");
        let current_layer = doc.get_page(page).get_layer(layer);
        let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
        
        let label = Label::new("Test Label", 12.0, font.clone());
        
        assert_eq!(label.value, "Test Label");
        assert_eq!(label.font_size, 12.0);
        label.render_at(10.0, 100.0, &current_layer);
    }

    #[test]
    fn test_label_new_row() {
        let (doc, _, _) = PdfDocument::new("test", Mm(210.0), Mm(297.0), "test");
        let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
        
        let row_data = vec!["Header1", "Header2", "Header3"];
        let labels = Label::new_row(row_data, 12.0, &font);
        
        assert_eq!(labels.len(), 3);
    }

    #[test]
    fn test_label_new_rows() {
        let (doc, _, _) = PdfDocument::new("test", Mm(210.0), Mm(297.0), "test");
        let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
        
        let rows_data = vec![
            vec!["Row1Col1", "Row1Col2"],
            vec!["Row2Col1", "Row2Col2"],
            vec!["Row3Col1", "Row3Col2"]
        ];
        let labels = Label::new_rows(rows_data, 12.0, &font);
        
        assert_eq!(labels.len(), 3);
        assert_eq!(labels[0].len(), 2);
        assert_eq!(labels[1].len(), 2);
        assert_eq!(labels[2].len(), 2);
    }
}