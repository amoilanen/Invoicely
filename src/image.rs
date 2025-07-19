
use anyhow::Error;
use printpdf::*;

pub(crate) fn load_image(image_path: &str) -> Result<RawImage, Error> {
    let image_bytes = if image_path.starts_with("http://") || image_path.starts_with("https://") {
        let response = reqwest::blocking::get(image_path)
            .map_err(|e| Error::msg(format!("Failed to download image from URL: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(Error::msg(format!("Failed to download image: HTTP {}", response.status())));
        }
        
        response.bytes()
            .map_err(|e| Error::msg(format!("Failed to read response bytes: {}", e)))?
            .to_vec()
    } else {
        std::fs::read(image_path)
            .map_err(|e| Error::msg(format!("Failed to read local image file: {}", e)))?
    };

    let mut warnings = Vec::new();
    let image = RawImage::decode_from_bytes(&image_bytes, &mut warnings)
        .map_err(|e| Error::msg(format!("Failed to decode image: {}", e)))?;
    Ok(image)
}