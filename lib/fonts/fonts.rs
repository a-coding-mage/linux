/* `Soft' font definitions -- translated from fonts.c. */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

/* Types and helpers supplied by the surrounding kernel translation. */
extern "C" {
    static font_vga_8x8: FontDesc;
    static font_vga_8x16: FontDesc;
    static font_vga_6x11: FontDesc;
    static font_7x14: FontDesc;
    static font_sun_8x16: FontDesc;
    static font_sun_12x22: FontDesc;
    static font_10x18: FontDesc;
    static font_acorn_8x8: FontDesc;
    static font_pearl_8x8: FontDesc;
    static font_mini_4x6: FontDesc;
    static font_6x10: FontDesc;
    static font_ter_10x18: FontDesc;
    static font_ter_16x32: FontDesc;
    static font_6x8: FontDesc;
}

#[repr(C)]
pub struct FontDesc {
    pub name: *const c_char,
    pub width: u32,
    pub height: u32,
    pub pref: c_int,
    pub idx: c_int,
}

pub type FontData = u8;

#[inline]
unsafe fn font_glyph_pitch(width: u32) -> u32 { (width + 7) / 8 }
#[inline]
unsafe fn font_glyph_size(width: u32, height: u32) -> usize {
    (font_glyph_pitch(width) * height) as usize
}
#[inline]
unsafe fn font_data_buf(fd: *mut FontData) -> *mut u8 { fd }

#[repr(C)]
struct FontDataStruct {
    extra: [i32; 4],
    data: [FontData; 0],
}

#[inline]
unsafe fn refcount(fd: *mut FontData) -> *mut i32 {
    (fd as *mut i32).offset(-1)
}
#[inline]
unsafe fn fntsize(fd: *mut FontData) -> *mut i32 {
    (fd as *mut i32).offset(-2)
}
#[inline]
unsafe fn fntsum(fd: *mut FontData) -> *mut i32 {
    (fd as *mut i32).offset(-4)
}

unsafe fn to_font_data_struct(fd: *mut FontData) -> *mut FontDataStruct {
    (fd as *mut u8).sub(core::mem::size_of::<[i32; 4]>()) as *mut FontDataStruct
}

unsafe fn font_data_is_internal(fd: *mut FontData) -> bool { *refcount(fd) == 0 }

unsafe fn font_data_free(fd: *mut FontData) {
    libc::free(to_font_data_struct(fd) as *mut c_void);
}

pub unsafe fn font_data_import(
    font: *const ConsoleFont, vpitch: u32,
    calc_csum: Option<unsafe extern "C" fn(u32, *const c_void, usize) -> u32>,
) -> *mut FontData {
    let pitch = font_glyph_pitch((*font).width);
    let h = (*font).height;
    let charcount = (*font).charcount;
    let data = (*font).data;
    let size = (h as usize).wrapping_mul(pitch as usize).wrapping_mul(charcount as usize);
    let alloc_size = core::mem::size_of::<FontDataStruct>().wrapping_add(size);
    let font_data = libc::malloc(alloc_size) as *mut FontDataStruct;
    if font_data.is_null() { return (-12isize) as *mut FontData; }
    (*font_data).extra = [0; 4];
    for i in 0..charcount as usize {
        ptr::copy_nonoverlapping(data.add(i * vpitch as usize * pitch as usize),
            (*font_data).data.as_mut_ptr().add(i * h as usize * pitch as usize),
            h as usize * pitch as usize);
    }
    let csum = calc_csum.map_or(0, |f| f(0, (*font_data).data.as_ptr() as *const c_void, size));
    let fd = (*font_data).data.as_mut_ptr();
    *refcount(fd) = 1;
    *fntsize(fd) = size as i32;
    *fntsum(fd) = csum as i32;
    fd
}

pub unsafe fn font_data_get(fd: *mut FontData) {
    if font_data_is_internal(fd) { return; }
    if *refcount(fd) == 0 { return; }
    *refcount(fd) += 1;
}

pub unsafe fn font_data_put(fd: *mut FontData) -> bool {
    if font_data_is_internal(fd) { return false; }
    if *refcount(fd) == 0 { return false; }
    *refcount(fd) -= 1;
    let zero = *refcount(fd) == 0;
    if zero { font_data_free(fd); }
    zero
}

pub unsafe fn font_data_size(fd: *mut FontData) -> u32 { *fntsize(fd) as u32 }

unsafe fn font_data_num_glyphs(fd: *mut FontData, width: u32, height: u32) -> u32 {
    font_data_size(fd) / font_glyph_size(width, height) as u32
}

pub unsafe fn font_data_glyph_buf(fd: *mut FontData, width: u32, vpitch: u32, c: u32) -> *const u8 {
    if c >= font_data_num_glyphs(fd, width, vpitch) { return ptr::null(); }
    font_data_buf(fd).add(font_glyph_size(width, vpitch) * c as usize)
}

pub unsafe fn font_data_is_equal(lhs: *mut FontData, rhs: *mut FontData) -> bool {
    if font_data_is_internal(lhs) != font_data_is_internal(rhs) { return false; }
    if font_data_size(lhs) != font_data_size(rhs) { return false; }
    if *fntsum(lhs) != 0 && *fntsum(rhs) != 0 && *fntsum(lhs) != *fntsum(rhs) { return false; }
    ptr::eq(lhs, rhs) || (0..font_data_size(lhs) as usize).all(|i| *lhs.add(i) == *rhs.add(i))
}

pub unsafe fn font_data_export(fd: *mut FontData, font: *mut ConsoleFont, vpitch: u32) -> c_int {
    let mut font_data = font_data_buf(fd) as *const u8;
    let mut data = (*font).data;
    if (*font).width == 0 || (*font).height == 0 || (*font).charcount == 0 || data.is_null() { return 0; }
    let pitch = font_glyph_pitch((*font).width) as usize;
    let glyphsize = (*font).height as usize * pitch;
    if (*font).charcount as usize * glyphsize > font_data_size(fd) as usize { return -22; }
    for _ in 0..(*font).charcount {
        ptr::copy_nonoverlapping(font_data, data, glyphsize);
        ptr::write_bytes(data.add(glyphsize), 0, pitch * vpitch as usize - glyphsize);
        data = data.add(pitch * vpitch as usize);
        font_data = font_data.add(glyphsize);
    }
    0
}

#[repr(C)]
pub struct ConsoleFont { pub width: u32, pub height: u32, pub charcount: u32, pub data: *mut u8 }

#[cfg(feature = "CONFIG_FONT_8x8")] static FONTS_8X8: *const FontDesc = unsafe { &font_vga_8x8 };
static FONTS: &[*const FontDesc] = &[
    #[cfg(feature = "CONFIG_FONT_8x8")] FONTS_8X8,
];

pub unsafe fn find_font(name: *const c_char) -> *const FontDesc {
    for &f in FONTS { if libc::strcmp((*f).name, name) == 0 { return f; } }
    ptr::null()
}

pub unsafe fn get_default_font(xres: c_int, yres: c_int, font_w: *mut usize, font_h: *mut usize) -> *const FontDesc {
    let mut g = ptr::null(); let mut cc = -10000;
    for &f in FONTS {
        let mut c = (*f).pref;
        if (yres < 400) == ((*f).height <= 8) { c += 1000; }
        let res = (xres / (*f).width as c_int) * (yres / (*f).height as c_int) / 1000;
        if res > 20 { c += 20 - res; }
        if (font_w.is_null() || *font_w.add((*f).width as usize - 1) != 0) &&
           (font_h.is_null() || *font_h.add((*f).height as usize - 1) != 0) { c += 1000; }
        if c > cc { cc = c; g = f; }
    }
    g
}

extern "C" { fn malloc(size: usize) -> *mut c_void; fn free(p: *mut c_void); fn strcmp(a: *const c_char, b: *const c_char) -> c_int; }
mod libc { pub use super::{free, malloc, strcmp}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
