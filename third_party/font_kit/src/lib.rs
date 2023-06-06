use std::ffi::{c_char, CStr};
use std::ptr::null_mut;
use font_kit::source::SystemSource;

#[repr(C)]
pub struct FontBuffer {
    data: *mut u8,
    len: usize,
}

#[no_mangle]
pub extern "C" fn free_font_buffer(buffer: FontBuffer) {
    if buffer.data.is_null() {
        return;
    }
    let s = unsafe { std::slice::from_raw_parts_mut(buffer.data, buffer.len) };
    unsafe {
        let _ = Box::from_raw(s);
    }
}

#[no_mangle]
pub extern "C" fn find_system_font(font_name_raw: *const c_char) -> FontBuffer {
    let cstr = unsafe { CStr::from_ptr(font_name_raw) };

    // Get copy-on-write Cow<'_, str>, then guarantee a freshly-owned String allocation.
    let font_name = String::from_utf8_lossy(cstr.to_bytes()).to_string();

    _find_system_font(font_name)
}

fn _find_system_font(font_name: String) -> FontBuffer {
    let result = std::panic::catch_unwind(|| {
        let font;

        if font_name.is_empty() {
            let handle = SystemSource::new().all_fonts().unwrap().first().unwrap().clone();

            font = handle.load().unwrap();
        } else {
            font = SystemSource::new()
                .select_by_postscript_name(&*font_name)
                .unwrap()
                .load()
                .unwrap();
        }

        let font_data = font.copy_font_data().unwrap();
        let font_data = (*font_data).clone();

        let mut buf = font_data.into_boxed_slice();
        let data = buf.as_mut_ptr();
        let len = buf.len();

        std::mem::forget(buf);

        FontBuffer { data, len }
    });
    if result.is_err() {
        eprintln!("ERROR: Rust panicked, failed to find font: {}", font_name);
        return FontBuffer { data: null_mut(), len: 0 };
    }

    result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        _find_system_font("ArialMT".to_string());
        // assert_eq!(result, 4);
    }
}
