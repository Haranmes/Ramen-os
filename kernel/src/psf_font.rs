use heapless::Vec;

// A struct to represent a PSF1 font.
// - `charsize` is the height (in bytes) of each glyph.
// - `glyphs` is a slice of raw glyph data (each character bitmap).
pub struct PSF1Font<'a> {
    pub charsize: usize,
    glyphs: &'a [u8],
}

/// Attempts to load a PSF1 font from raw binary data.
// Returns `Some(PSF1Font)` if valid, otherwise `None`.
pub fn load_psf1_font(data: &[u8]) -> Option<PSF1Font> {
    // Check for minimum size and PSF1 magic bytes (0x36, 0x04)
    if data.len() < 4 || data[0] != 0x36 || data[1] != 0x04 {
        return None;
    }

    // Get the size (height) of each character from the 4th byte
    let charsize = data[3] as usize;

    // The glyphs follow the 4-byte header
    let glyphs = &data[4..];

    // Return a new PSF1Font instance
    Some(PSF1Font { charsize, glyphs })
}


// Draws a single character onto the framebuffer using a PSF1 font
pub fn draw_char(
    framebuffer: &limine::framebuffer::Framebuffer, // target framebuffer
    font: &PSF1Font,                                // loaded font
    ch: u8,                                         // character to draw (ASCII)
    x: usize, y: usize,                             // top-left pixel position
    fg_color: u32,                                  // foreground (text) color
    bg_color: u32,                                  // background color
) {
    // Pitch = number of bytes per line in framebuffer
    let pitch = framebuffer.pitch() as usize;

    // Framebuffer dimensions
    let fb_width = framebuffer.width() as usize;
    let fb_height = framebuffer.height() as usize;

    // Raw framebuffer pointer
    let fb_ptr = framebuffer.addr() as *mut u8;

    // Offset into glyph array for the character we want to draw
    let char_offset = ch as usize * font.charsize;

    // Safety check: make sure the character doesn't exceed glyph array size
    if char_offset + font.charsize > font.glyphs.len() {
        return; // Character is not valid, do nothing
    }

    // Get glyph slice (bitmap for the character)
    let glyph = &font.glyphs[char_offset..char_offset + font.charsize];

    // Loop through each byte of the glyph (one byte per row)
    for (row, byte) in glyph.iter().enumerate() {
        // Loop through each bit (i.e., pixel) of the byte
        for col in 0..8 {
            let mask = 1 << (7 - col);               // Bitmask for left-most bit
            let is_set = byte & mask != 0;           // Check if pixel is "on"
            let color = if is_set { fg_color } else { bg_color };

            // Calculate absolute pixel position on screen
            let px = x + col;
            let py = y + row;

            // Check bounds
            if px < fb_width && py < fb_height {
                // Calculate memory offset in framebuffer
                let offset = py * pitch + px * 4;

                // Write the pixel color (RGBA) to the framebuffer
                unsafe {
                    (fb_ptr.add(offset) as *mut u32).write_volatile(color);
                }
            }
        }
    }
}



// Draws a string of text to the screen using the PSF1 font.
pub fn draw_text(
    framebuffer: &limine::framebuffer::Framebuffer,
    font: &PSF1Font,
    text: &str,               // The text string to render
    mut x: usize,             // Starting x position
    mut y: usize,             // Starting y position
    fg: u32,                  // Foreground color
    bg: u32,                  // Background color
    highlight_color: Option<u32>    
) {
    

    let char_height = font.charsize;
    let mut in_brackets = false;

    for ch in text.bytes() {
        match ch {
            b'\n' => {
                x = 0;
                y += char_height;
            }
            b'[' => {
                in_brackets = true;
                let color = highlight_color.unwrap_or(fg);
                draw_char(framebuffer, font, ch, x, y, color, bg);
                x += 8;
            }
            b']' => {
                let color = highlight_color.unwrap_or(fg);
                draw_char(framebuffer, font, ch, x, y, color, bg);
                x += 8;
                in_brackets = false;
            }
            _ => {
                let color = if in_brackets {
                    highlight_color.unwrap_or(fg)
                } else {
                    fg
                };
                draw_char(framebuffer, font, ch, x, y, color, bg);
                x += 8;
            }
        }
    }

}



