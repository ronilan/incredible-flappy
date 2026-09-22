use incredible_elements::ImageData;

/// Decodes a PNG into terminal image data.
pub(crate) fn decode_png(bytes: &[u8]) -> ImageData {
    let img = image::load_from_memory(bytes).expect("asset decodes");
    let rgba = img.to_rgba8();
    let (width_px, height_px) = (rgba.width(), rgba.height());
    ImageData {
        bytes: rgba.into_raw(),
        width_px,
        height_px,
    }
}
