use printpdf::*;
use super::Component;

pub struct Table {
    pub column_widths: Vec<f32>,
    pub row_height: f32,
    pub header: Option<Vec<Box <dyn Component>>>,
    pub rows: Vec<Vec<Box<dyn Component>>>
}

impl Table {
    fn render_row_at(&self, values: &Vec<Box<dyn Component>>, x: f32, y:f32, layer: &PdfLayerReference) {
        let mut current_x_offset = x;
        for (column_value, column_width) in values.iter().zip(self.column_widths.iter()) {
            column_value.render_at(current_x_offset, y, layer);
            current_x_offset = current_x_offset + column_width;
        }
    }

    fn render_bottom_border_at(&self, x: f32, y:f32, layer: &PdfLayerReference) {
        let total_width: f32 = self.column_widths.iter().sum();
        layer.set_outline_thickness(0.4);
        layer.add_line(
            Line {
                points: vec![
                    (Point::new(Mm(x), Mm(y)), false),
                    (Point::new(Mm(x + total_width), Mm(y)), false),
                ],
                is_closed: false
            }
        );
    }
}

impl Component for Table {

    fn render_at(&self, x: f32, y:f32, layer: &PdfLayerReference) {
        let mut current_y_offset = y;
        if let Some(header_values) = &self.header {
            self.render_row_at(header_values, x, current_y_offset, layer);
            self.render_bottom_border_at(x, current_y_offset - self.row_height / 2.0 , layer);
            current_y_offset = current_y_offset - self.row_height - self.row_height / 2.0;
        }
        for row in self.rows.iter() {
            self.render_row_at(row, x, current_y_offset, layer);
            current_y_offset = current_y_offset - self.row_height;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockComponent {
    }

    impl MockComponent {
        fn new(_value: &str) -> Self {
            MockComponent {
            }
        }
    }

    impl Component for MockComponent {
        fn render_at(&self, _x: f32, _y: f32, _layer: &PdfLayerReference) {
        }
    }

    #[test]
    fn test_empty_table() {
        let column_widths = vec![50.0, 100.0, 75.0];
        let row_height = 20.0;
        let header = None;
        let rows = vec![];

        let table = Table {
            column_widths: column_widths.clone(),
            row_height,
            header,
            rows,
        };

        assert_eq!(table.column_widths, column_widths);
        assert_eq!(table.row_height, row_height);
        assert!(table.header.is_none());
        assert!(table.rows.is_empty());

        let (doc, page, layer) = PdfDocument::new("test", Mm(210.0), Mm(297.0), "test");
        let current_layer = doc.get_page(page).get_layer(layer);

        table.render_at(10.0, 100.0, &current_layer);
    }

    #[test]
    fn test_table_with_header_and_rows() -> Result<(), Box<dyn std::error::Error>> {
        let column_widths = vec![40.0, 80.0];
        let row_height = 15.0;
        
        let header = Some(vec![
            Box::new(MockComponent::new("Header1")) as Box<dyn Component>,
            Box::new(MockComponent::new("Header2")) as Box<dyn Component>
        ]);
        
        let rows = vec![
            vec![Box::new(MockComponent::new("Cell1")) as Box<dyn Component>, Box::new(MockComponent::new("Cell2")) as Box<dyn Component>],
            vec![Box::new(MockComponent::new("Cell3")) as Box<dyn Component>, Box::new(MockComponent::new("Cell4")) as Box<dyn Component>]
        ];
        
        let table = Table {
            column_widths,
            row_height,
            header,
            rows,
        };

        // Test table with both header and rows
        assert!(table.header.is_some());
        assert_eq!(table.header.as_ref().ok_or("Header is None")?.len(), 2);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0].len(), 2);
        assert_eq!(table.rows[1].len(), 2);

        let (doc, page, layer) = PdfDocument::new("test", Mm(210.0), Mm(297.0), "test");
        let current_layer = doc.get_page(page).get_layer(layer);

        table.render_at(10.0, 100.0, &current_layer);
        Ok(())
    }
}