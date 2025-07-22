use printpdf::*;
use super::Component;

pub struct Label {
    pub value: String,
    pub font_size: f32,
    pub font_id: FontId
}

impl Label {
    pub fn new(value: &str, font_size: f32, font_id: &FontId) -> Label {
        Label {
            value: value.to_owned(),
            font_size,
            font_id: font_id.clone()
        }
    }

    pub fn new_row(row: Vec<&str>, font_size: f32, font_id: &FontId) -> Vec<Box<dyn Component>> {
        let mut label_row: Vec<Box<dyn Component>> = Vec::new();
        for value in row.into_iter() {
            label_row.push(Box::new(Label::new(value, font_size, font_id)));
        }
        label_row
    }

    pub fn new_rows(rows: Vec<Vec<&str>>, font_size: f32, font_id: &FontId) -> Vec<Vec<Box<dyn Component>>> {
        let mut labels: Vec<Vec<Box<dyn Component>>> = Vec::new();
        for row in rows.into_iter() {
            labels.push(Label::new_row(row, font_size, font_id));
        }
        labels
    }
}

impl Component for Label {
    fn render_at(&self, x: f32, y: f32) -> Vec<Op> {
        let mut ops: Vec<Op> = Vec::new();
        let lines: Vec<&str> = self.value.split('\n').collect();
        let line_height = self.font_size / 2.0; // Approximate line height
        for (index, line) in lines.iter().enumerate() {
            let current_y = y - (index as f32 * line_height);
            ops.push(Op::SetFontSize { size: Pt(self.font_size), font: self.font_id.clone() });
            ops.push(Op::StartTextSection);
            ops.push(Op::SetTextCursor {
                pos: Point { x: Mm(x).into(), y: Mm(current_y).into() }
            });
            ops.push(Op::WriteText {
                items: vec![TextItem::Text(line.to_string())],
                font: self.font_id.clone(),
            });
            ops.push(Op::EndTextSection);
        }
        ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use printpdf::FontId;

    #[test]
    fn test_label_new() {
        let label = Label::new("Test Label", 12.0, &FontId::new());
        assert_eq!(label.value, "Test Label");
        assert_eq!(label.font_size, 12.0);
    }

    #[test]
    fn test_label_new_row() {
        let row_data = vec!["Header1", "Header2", "Header3"];
        let labels = Label::new_row(row_data, 12.0, &FontId::new());
        assert_eq!(labels.len(), 3);
    }

    #[test]
    fn test_label_new_rows() {
        let rows_data = vec![
            vec!["Row1Col1", "Row1Col2"],
            vec!["Row2Col1", "Row2Col2"],
            vec!["Row3Col1", "Row3Col2"]
        ];
        let labels = Label::new_rows(rows_data, 12.0, &FontId::new());
        assert_eq!(labels.len(), 3);
        assert_eq!(labels[0].len(), 2);
        assert_eq!(labels[1].len(), 2);
        assert_eq!(labels[2].len(), 2);
    }

    #[test]
    fn test_multi_line_label() {
        let multi_line_text = "Line 1\nLine 2\nLine 3";
        let label = Label::new(multi_line_text, 12.0, &FontId::new());
        assert_eq!(label.value, multi_line_text);
        
        let ops = label.render_at(10.0, 100.0);
        // Should have 5 operations per line: SetFontSize, StartTextSection, SetTextCursor, WriteText, EndTextSection
        // For 3 lines, that's 15 operations total
        assert_eq!(ops.len(), 15);
    }
}