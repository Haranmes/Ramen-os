
pub struct PSF1Font<'a> {
    charsize: usize,
    glyphs: &'a [u8],
}

pub fn load_psf1_font(data: &[u8]) -> Option<PSF1Font> {
    if data.len() < 4 || data[0] != 0x36 || data[1] != 0x04 {
        return None;
    }

    let charsize = data[3] as usize;
    let glyphs = &data[4..];

    Some(PSF1Font { charsize, glyphs })
}

pub fn draw_char(
    framebuffer: &limine::framebuffer::Framebuffer,
    font: &PSF1Font,
    ch: u8,
    x: usize,
    y: usize,
    fg_color: u32,
    bg_color: u32,
) {
    let pitch = framebuffer.pitch() as usize;
    let fb_width = framebuffer.width() as usize;
    let fb_height = framebuffer.height() as usize;
    let fb_ptr = framebuffer.addr() as *mut u8;

    let char_offset = ch as usize * font.charsize;
    if char_offset + font.charsize > font.glyphs.len() {
        return; // invalid character
    }

    let glyph = &font.glyphs[char_offset..char_offset + font.charsize];

    for (row, byte) in glyph.iter().enumerate() {
        for col in 0..8 {
            let mask = 1 << (7 - col);
            let is_set = byte & mask != 0;
            let color = if is_set { fg_color } else { bg_color };

            let px = x + col;
            let py = y + row;

            if px < fb_width && py < fb_height {
                let offset = py * pitch + px * 4;
                unsafe {
                    (fb_ptr.add(offset) as *mut u32).write_volatile(color);
                }
            }
        }
    }
}


pub fn draw_text(
    framebuffer: &limine::framebuffer::Framebuffer,
    font: &PSF1Font,
    text: &str,
    mut x: usize,
    mut y: usize,
    fg: u32,
    bg: u32,
) {
    let char_height = font.charsize;

    for ch in text.bytes() {
        match ch {
            b'\n' => {
                x = 0;
                y += char_height;
            }
            _ => {
                draw_char(framebuffer, font, ch, x, y, fg, bg);
                x += 8;
            }
        }
    }
}


