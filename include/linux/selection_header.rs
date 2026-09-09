/* SPDX-License-Identifier: GPL-2.0 */
/*
 * selection.h
 *
 * Interface between console.c, tty_io.c, vt.c, vc_screen.c and selection.c
 */

// Dependency intent: definitions from <linux/tiocl.h> and
// <linux/vt_buffer.h> are supplied by other translated files.

#[repr(C)]
pub struct tty_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vc_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tiocl_selection {
    _private: [u8; 0],
}

extern "C" {
    pub fn clear_selection();
    pub fn set_selection_user(
        sel: *const tiocl_selection,
        tty: *mut tty_struct,
    ) -> ::core::ffi::c_int;
    pub fn set_selection_kernel(
        v: *mut tiocl_selection,
        tty: *mut tty_struct,
    ) -> ::core::ffi::c_int;
    pub fn paste_selection(tty: *mut tty_struct) -> ::core::ffi::c_int;
    pub fn sel_loadlut(lut: *mut u32) -> ::core::ffi::c_int;
    pub fn mouse_reporting() -> ::core::ffi::c_int;
    pub fn mouse_report(
        tty: *mut tty_struct,
        butt: ::core::ffi::c_int,
        mrx: ::core::ffi::c_int,
        mry: ::core::ffi::c_int,
    );

    pub fn vc_is_sel(vc: *const vc_data) -> bool;

    pub static mut console_blanked: ::core::ffi::c_int;

    pub static color_table: [u8; 0];
    pub static mut default_red: [u8; 0];
    pub static mut default_grn: [u8; 0];
    pub static mut default_blu: [u8; 0];

    pub fn screen_pos(
        vc: *const vc_data,
        w_offset: ::core::ffi::c_int,
        viewed: bool,
    ) -> *mut u16;
    pub fn screen_glyph(vc: *const vc_data, offset: ::core::ffi::c_int) -> u16;
    pub fn screen_glyph_unicode(vc: *const vc_data, offset: ::core::ffi::c_int) -> u32;
    pub fn complement_pos(vc: *mut vc_data, offset: ::core::ffi::c_int);
    pub fn invert_screen(
        vc: *mut vc_data,
        offset: ::core::ffi::c_int,
        count: ::core::ffi::c_int,
        viewed: bool,
    );

    pub fn getconsxy(vc: *const vc_data, xy: *mut u8);
    pub fn putconsxy(vc: *mut vc_data, xy: *mut u8);

    pub fn vcs_scr_readw(vc: *const vc_data, org: *const u16) -> u16;
    pub fn vcs_scr_writew(vc: *mut vc_data, val: u16, org: *mut u16);
    pub fn vcs_scr_updated(vc: *mut vc_data);

    pub fn vc_uniscr_check(vc: *mut vc_data) -> ::core::ffi::c_int;
    pub fn vc_uniscr_copy_line(
        vc: *const vc_data,
        dest: *mut ::core::ffi::c_void,
        viewed: bool,
        row: u32,
        col: u32,
        nr: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
