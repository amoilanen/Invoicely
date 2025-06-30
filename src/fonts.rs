use anyhow::Error;
use printpdf::*;
use font_kit::source::SystemSource;

pub struct FontManager {
    regular_font: FontId,
    bold_font: FontId,
}

impl FontManager {
    pub fn initialize(doc: &mut PdfDocument) -> Result<Self, Error> {
        let font_names: Vec<String> = vec![
            "Helvetica",
            "Arial",
            "DejaVu Sans",
            "Liberation Sans", 
            "FreeSans",
            "Ubuntu",
            "Noto Sans",
        ].iter().map(|name| name.to_string()).collect();
        let bold_font_names: Vec<String> = font_names.iter().map(|name| format!("{}-Bold", name)).collect();

        let regular_font = Self::load_system_font(doc, &font_names)?;
        let bold_font = Self::load_system_font(doc, &bold_font_names)?;
        
        Ok(FontManager {
            regular_font,
            bold_font,
        })
    }

    fn load_system_font(doc: &mut PdfDocument, font_names: &[String]) -> Result<FontId, Error> {
        let source = SystemSource::new();
        for name in font_names {
            if let Ok(font) = source.select_by_postscript_name(name) {
                if let Ok(font_data) = font.load() {
                    if let Some(font_bytes) = font_data.copy_font_data() {
                        let font_index = 0;
                        let mut warnings = Vec::new();
                        
                        if let Some(parsed_font) = ParsedFont::from_bytes(&font_bytes, font_index, &mut warnings) {
                            return Ok(doc.add_font(&parsed_font));
                        }
                    }
                } else {
                    println!("Could not load font: {}", name);
                }
            }
        }
        Err(Error::msg(format!("Could not load any appropriate system fonts. Are fonts installed? Tried: {:?}", font_names)))
    }

    pub fn regular_font(&self) -> FontId {
        self.regular_font.clone()
    }

    pub fn bold_font(&self) -> FontId {
        self.bold_font.clone()
    }
}