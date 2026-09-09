// SPDX-License-Identifier: GPL-2.0-only
/*
 * Font rotation
 *
 *    Copyright (C) 2005 Antonino Daplas <adaplas @ pol.net>
 */

use core::ffi::c_void;

#[repr(C)]
pub struct font_data_t {
    _private: [u8; 0],
}

extern "C" {
    fn font_glyph_size(width: u32, height: u32) -> usize;
    fn font_data_buf(fd: *mut font_data_t) -> *const u8;
    fn kmalloc_array(n: usize, size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
}

const GFP_KERNEL: u32 = 0;

unsafe fn font_glyph_bit_pitch(width: u32) -> u32 {
    (width + 7) & !7
}

unsafe fn __font_glyph_pos(x: u32, y: u32, bit_pitch: u32, bit: *mut u32) -> u32 {
    let off = y * bit_pitch + x;
    let bit_shift = off % 8;
    *bit = 0x80 >> bit_shift;
    off / 8
}

unsafe fn font_glyph_test_bit(glyph: *const u8, x: u32, y: u32, bit_pitch: u32) -> bool {
    let mut bit = 0;
    let i = __font_glyph_pos(x, y, bit_pitch, &mut bit);
    *glyph.add(i as usize) & bit as u8 != 0
}

unsafe fn font_glyph_set_bit(glyph: *mut u8, x: u32, y: u32, bit_pitch: u32) {
    let mut bit = 0;
    let i = __font_glyph_pos(x, y, bit_pitch, &mut bit);
    *glyph.add(i as usize) |= bit as u8;
}

unsafe fn __font_glyph_rotate_90(glyph: *const u8, width: u32, height: u32, out: *mut u8) {
    let shift = (8 - (height % 8)) & 7;
    let bit_pitch = font_glyph_bit_pitch(width);
    let out_bit_pitch = font_glyph_bit_pitch(height);
    for y in 0..height {
        for x in 0..width {
            if font_glyph_test_bit(glyph, x, y, bit_pitch) {
                font_glyph_set_bit(out, out_bit_pitch - 1 - y - shift, x, out_bit_pitch);
            }
        }
    }
}

pub unsafe fn font_glyph_rotate_90(glyph: *const u8, width: u32, height: u32, out: *mut u8) {
    core::ptr::write_bytes(out, 0, font_glyph_size(height, width));
    __font_glyph_rotate_90(glyph, width, height, out);
}

unsafe fn __font_glyph_rotate_180(glyph: *const u8, width: u32, height: u32, out: *mut u8) {
    let shift = (8 - (width % 8)) & 7;
    let bit_pitch = font_glyph_bit_pitch(width);
    for y in 0..height {
        for x in 0..width {
            if font_glyph_test_bit(glyph, x, y, bit_pitch) {
                font_glyph_set_bit(out, bit_pitch - 1 - x - shift, height - 1 - y, bit_pitch);
            }
        }
    }
}

pub unsafe fn font_glyph_rotate_180(glyph: *const u8, width: u32, height: u32, out: *mut u8) {
    core::ptr::write_bytes(out, 0, font_glyph_size(width, height));
    __font_glyph_rotate_180(glyph, width, height, out);
}

unsafe fn __font_glyph_rotate_270(glyph: *const u8, width: u32, height: u32, out: *mut u8) {
    let shift = (8 - (width % 8)) & 7;
    let bit_pitch = font_glyph_bit_pitch(width);
    let out_bit_pitch = font_glyph_bit_pitch(height);
    for y in 0..height {
        for x in 0..width {
            if font_glyph_test_bit(glyph, x, y, bit_pitch) {
                font_glyph_set_bit(out, y, bit_pitch - 1 - x - shift, out_bit_pitch);
            }
        }
    }
}

pub unsafe fn font_glyph_rotate_270(glyph: *const u8, width: u32, height: u32, out: *mut u8) {
    core::ptr::write_bytes(out, 0, font_glyph_size(height, width));
    __font_glyph_rotate_270(glyph, width, height, out);
}

pub unsafe fn font_data_rotate(
    fd: *mut font_data_t, width: u32, height: u32, charcount: u32, mut steps: u32,
    mut buf: *mut u8, bufsize: *mut usize,
) -> *mut u8 {
    let mut src = font_data_buf(fd);
    let s_cellsize = font_glyph_size(width, height);
    let d_cellsize;
    let size;
    steps %= 4;
    match steps {
        0 | 2 => d_cellsize = s_cellsize,
        1 | 3 => d_cellsize = font_glyph_size(height, width),
        _ => unreachable!(),
    }
    if charcount as usize > usize::MAX / d_cellsize {
        return (-22isize) as *mut u8;
    }
    size = charcount as usize * d_cellsize;
    let dst: *mut u8;
    if buf.is_null() || bufsize.is_null() || size > *bufsize {
        dst = kmalloc_array(charcount as usize, d_cellsize, GFP_KERNEL);
        if dst.is_null() { return (-12isize) as *mut u8; }
        kfree(buf);
        buf = dst;
        if !bufsize.is_null() { *bufsize = size; }
    } else {
        dst = buf;
    }
    match steps {
        0 => core::ptr::copy_nonoverlapping(src, dst, size),
        1 | 2 | 3 => {
            core::ptr::write_bytes(dst, 0, size);
            let mut d = dst;
            for _ in 0..charcount {
                match steps {
                    1 => __font_glyph_rotate_90(src, width, height, d),
                    2 => __font_glyph_rotate_180(src, width, height, d),
                    _ => __font_glyph_rotate_270(src, width, height, d),
                }
                src = src.add(s_cellsize);
                d = d.add(d_cellsize);
            }
        }
        _ => unreachable!(),
    }
    buf
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
