pub static mut CURSOR_X: usize = 0;
pub static mut CURSOR_Y: usize = 0;

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        static FONT: &[u8] = include_bytes!("../font.psf");
        if let Some(response) = FRAMEBUFFER_REQUEST.get_response() {
            if let Some(framebuffer) = response.framebuffers().next() {
                if let Some(font) = crate::psf_font::load_psf1_font(FONT) {
                    let mut buffer = [0u8; 512];
                    let mut writer = crate::utils::BufferWriter::new(&mut buffer);

                    core::fmt::write(&mut writer, format_args!($($arg)*)).unwrap();

                    unsafe {
                        let x = crate::makros::CURSOR_X;
                        let y = crate::makros::CURSOR_Y;

                        crate::psf_font::draw_text(
                            &framebuffer,
                            &font,
                            writer.as_str(),
                            x,
                            y,
                            0xFFFFFF,
                            0x000000,
                        );

                        crate::makros::CURSOR_X = 0;
                        crate::makros::CURSOR_Y += font.charsize;
                    }
                } else {
                    panic!("Bad font!");
                }
            }
        }
    }};
}


#[macro_export]
macro_rules! error {
       ($($arg:tt)*) => {{
        static FONT: &[u8] = include_bytes!("../font.psf");
        if let Some(response) = FRAMEBUFFER_REQUEST.get_response() {
            if let Some(framebuffer) = response.framebuffers().next() {
                if let Some(font) = crate::psf_font::load_psf1_font(FONT) {
                    let mut buffer = [0u8; 512];
                    let mut writer = crate::utils::BufferWriter::new(&mut buffer);

                    core::fmt::write(&mut writer, format_args!($($arg)*)).unwrap();

                    unsafe {
                        let x = crate::makros::CURSOR_X;
                        let y = crate::makros::CURSOR_Y;

                        crate::psf_font::draw_text(
                            &framebuffer,
                            &font,
                            writer.as_str(),
                            x,
                            y,
                            0xEE4B2B,
                            0x000000,
                        );

                        crate::makros::CURSOR_X = 0;
                        crate::makros::CURSOR_Y += font.charsize;
                    }
                } else {
                    panic!("Bad font!");
                }
            }
        }
    }};
}
