/* SPDX-License-Identifier: GPL-2.0-only */
/* Translation of arch/arm/mach-rpc/include/mach/uncompress.h. */

// SCREEN_START, IO_START, PAGE_SIZE, ATAG_CORE, ATAG_VIDEOTEXT,
// ATAG_VIDEOLFB, ATAG_MEM, error, tag_next, and the tag type are supplied by
// the corresponding platform/decompression dependencies.

pub const VIDMEM: *mut u8 = SCREEN_START as *mut u8;

pub static mut video_size_row: i32 = 0;
pub static mut bytes_per_char_h: u8 = 0;

extern "C" {
    pub static mut con_charconvtable: [u64; 256];
}

#[repr(C)]
pub struct param_struct {
    pub page_size: u64,
    pub nr_pages: u64,
    pub ramdisk_size: u64,
    pub mountrootrdonly: u64,
    pub rootdev: u64,
    pub video_num_cols: u64,
    pub video_num_rows: u64,
    pub video_x: u64,
    pub video_y: u64,
    pub memc_control_reg: u64,
    pub sounddefault: u8,
    pub adfsdrives: u8,
    pub bytes_per_char_h: u8,
    pub bytes_per_char_v: u8,
    pub unused: [u64; 256 / 4 - 11],
}

static palette_4: [u64; 16] = [
    0x00000000, 0x000000cc, 0x0000cc00, 0x0000cccc,
    0x00cc0000, 0x00cc00cc, 0x00cccc00, 0x00cccccc,
    0x00000000, 0x000000ff, 0x0000ff00, 0x0000ffff,
    0x00ff0000, 0x00ff00ff, 0x00ffff00, 0x00ffffff,
];

#[inline]
unsafe fn palette_setpixel(p: u64) {
    core::ptr::write_volatile((IO_START + 0x00400000) as *mut u64, 0x10000000 | (p & 255));
}

#[inline]
unsafe fn palette_write(v: u64) {
    core::ptr::write_volatile((IO_START + 0x00400000) as *mut u64, v & 0x00ffffff);
}

extern "C" {
    pub fn params() -> *mut param_struct;
}

#[cfg(not(STANDALONE_DEBUG))]
pub mod non_standalone_debug {
    use super::*;

    pub static mut video_num_cols: u64 = 0;
    pub static mut video_num_rows: u64 = 0;
    pub static mut video_x: u64 = 0;
    pub static mut video_y: u64 = 0;
    pub static mut bytes_per_char_v: u8 = 0;
    pub static mut white: i32 = 0;

    extern "C" {
        fn ll_write_char(ptr: *mut u8, c: i8, white: i32);
        fn error(message: *const i8) -> !;
    }

    #[inline]
    pub unsafe fn putc(c: i32) {
        let mut x = video_x;
        let mut y = video_y;
        if c == b'\n' as i32 {
            y = y.wrapping_add(1);
            if y >= video_num_rows { y = y.wrapping_sub(1); }
        } else if c == b'\r' as i32 {
            x = 0;
        } else {
            let offset = ((y * video_num_cols * bytes_per_char_v as u64 + x)
                * bytes_per_char_h as u64) as usize;
            ll_write_char(VIDMEM.add(offset), c as i8, white);
            x = x.wrapping_add(1);
            if x >= video_num_cols {
                x = 0;
                y = y.wrapping_add(1);
                if y >= video_num_rows { y = y.wrapping_sub(1); }
            }
        }
        video_x = x;
        video_y = y;
    }

    #[inline]
    pub unsafe fn flush() {}

    // The tag layout and tag_next are supplied by asm/setup.h.  The accesses
    // below intentionally retain the source header's external tag interface.
    pub unsafe fn arch_decomp_setup() {
        todo!("translate accesses to the externally supplied struct tag layout");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
