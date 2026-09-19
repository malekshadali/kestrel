pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn draw_text(
    buffer: &mut [u32],
    font: &fontdue::Font,
    text: &str,
    x: u32,
    y: u32,
    size: f32,
    color: u32,
    screen_width: u32,
) {
    let mut cursor_x = x;
    let mut cursor_y = y;
    let line_height = size as u32 + 4;

    for ch in text.chars() {
        let (metrics, bitmap) = font.rasterize(ch, size);

        if cursor_x + metrics.advance_width as u32 > screen_width {
            cursor_x = x;
            cursor_y += line_height;
        }

        for row in 0..metrics.height {
            for col in 0..metrics.width {
                let coverage = bitmap[row * metrics.width + col];
                if coverage > 0 {
                    let pixel_y =
                        cursor_y as i32 + row as i32 - metrics.ymin as i32 - metrics.height as i32;
                    if pixel_y < 0 {
                        continue;
                    }
                    let idx = pixel_y as u32 * screen_width + (cursor_x + col as u32);
                    if idx as usize >= buffer.len() {
                        continue;
                    }
                    let alpha = coverage as u32;
                    let r = (color >> 16) & 0xff;
                    let g = (color >> 8) & 0xff;
                    let b = color & 0xff;
                    buffer[idx as usize] =
                        ((r * alpha / 255) << 16) | ((g * alpha / 255) << 8) | (b * alpha / 255);
                }
            }
        }
        cursor_x += metrics.advance_width as u32
    }
}
