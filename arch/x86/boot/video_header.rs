/* SPDX-License-Identifier: GPL-2.0-only */
/* -*- linux-c -*- */

/*
 * Header file for the real-mode video probing code
 */

/* The C header included linux/types.h; these type names are supplied by the
 * surrounding low-level environment. */

/*
 * This code uses an extended set of video mode numbers. These include:
 * Aliases for standard modes
 *      NORMAL_VGA (-1)
 *      EXTENDED_VGA (-2)
 *      ASK_VGA (-3)
 * Video modes numbered by menu position -- NOT RECOMMENDED because of lack
 * of compatibility when extending the table. These are between 0x00 and 0xff.
 */
pub const VIDEO_FIRST_MENU: u16 = 0x0000;

/* Standard BIOS video modes (BIOS number + 0x0100) */
pub const VIDEO_FIRST_BIOS: u16 = 0x0100;

/* VESA BIOS video modes (VESA number + 0x0200) */
pub const VIDEO_FIRST_VESA: u16 = 0x0200;

/* Video7 special modes (BIOS number + 0x0900) */
pub const VIDEO_FIRST_V7: u16 = 0x0900;

/* Special video modes */
pub const VIDEO_FIRST_SPECIAL: u16 = 0x0f00;
pub const VIDEO_80x25: u16 = 0x0f00;
pub const VIDEO_8POINT: u16 = 0x0f01;
pub const VIDEO_80x43: u16 = 0x0f02;
pub const VIDEO_80x28: u16 = 0x0f03;
pub const VIDEO_CURRENT_MODE: u16 = 0x0f04;
pub const VIDEO_80x30: u16 = 0x0f05;
pub const VIDEO_80x34: u16 = 0x0f06;
pub const VIDEO_80x60: u16 = 0x0f07;
pub const VIDEO_GFX_HACK: u16 = 0x0f08;
pub const VIDEO_LAST_SPECIAL: u16 = 0x0f09;

/* Video modes given by resolution */
pub const VIDEO_FIRST_RESOLUTION: u16 = 0x1000;

/* The "recalculate timings" flag */
pub const VIDEO_RECALC: u16 = 0x8000;

unsafe extern "C" {
    pub fn store_screen();
    pub fn mode_defined(mode: u16) -> ::core::ffi::c_int;
    pub static mut video_cards: [card_info; 0];
    pub static mut video_cards_end: [card_info; 0];
    pub static mut adapter: ::core::ffi::c_int;
    pub static mut force_x: ::core::ffi::c_int;
    pub static mut force_y: ::core::ffi::c_int;
    pub static mut do_restore: ::core::ffi::c_int;
    pub static mut graphic_mode: ::core::ffi::c_int;
    pub fn vga_crtc() -> u16;
    pub fn outb(value: u8, port: u16);
    pub fn inb(port: u16) -> u8;
    pub fn outw(value: u16, port: u16);
}

#[inline]
pub unsafe fn do_store() {
    store_screen()
}

#[repr(C)]
pub struct mode_info {
    pub mode: u16,
    pub x: u16,
    pub y: u16,
    pub depth: u16,
}

#[repr(C)]
pub struct card_info {
    pub card_name: *const ::core::ffi::c_char,
    pub set_mode: Option<unsafe extern "C" fn(mode: *mut mode_info) -> ::core::ffi::c_int>,
    pub probe: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub modes: *mut mode_info,
    pub nmodes: ::core::ffi::c_int,
    pub unsafe_: ::core::ffi::c_int,
    pub xmode_first: u16,
    pub xmode_n: u16,
}

#[inline]
pub unsafe fn in_idx(port: u16, index: u8) -> u8 {
    outb(index, port);
    inb(port.wrapping_add(1))
}

#[inline]
pub unsafe fn out_idx(v: u8, port: u16, index: u8) {
    outw((index as u16).wrapping_add((v as u16) << 8), port);
}

/* Writes a value to an indexed port and then reads the port again */
#[inline]
pub unsafe fn tst_idx(v: u8, port: u16, index: u8) -> u8 {
    out_idx(port as u8, index as u16, v);
    in_idx(index as u16, v)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
