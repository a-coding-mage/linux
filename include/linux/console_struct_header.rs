/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of console_struct.h.
 * Included C dependencies are supplied by other translation units.
 */

pub const NPAR: usize = 16;
pub const VC_TABSTOPS_COUNT: u32 = 256;

pub struct uni_pagedict;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum vc_intensity {
    VCI_HALF_BRIGHT,
    VCI_NORMAL,
    VCI_BOLD,
    VCI_MASK = 0x3,
}

#[repr(C)]
pub struct vc_state {
    pub x: ::core::ffi::c_uint,
    pub y: ::core::ffi::c_uint,
    pub color: ::core::ffi::c_uchar,
    pub Gx_charset: [::core::ffi::c_uchar; 2],
    pub charset: ::core::ffi::c_uint,
    pub intensity: vc_intensity,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub reverse: bool,
}

#[repr(C)]
pub struct vc_font {
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub charcount: ::core::ffi::c_uint,
    pub data: *const ::core::ffi::c_uchar,
}

unsafe extern "C" {
    pub fn vc_font_pitch(font: *const vc_font) -> ::core::ffi::c_uint;
    pub fn vc_font_size(font: *const vc_font) -> ::core::ffi::c_uint;
}

#[repr(C)]
pub struct vc_data {
    pub port: tty_port,
    pub state: vc_state,
    pub saved_state: vc_state,
    pub vc_num: ::core::ffi::c_ushort,
    pub vc_cols: ::core::ffi::c_uint,
    pub vc_rows: ::core::ffi::c_uint,
    pub vc_size_row: ::core::ffi::c_uint,
    pub vc_scan_lines: ::core::ffi::c_uint,
    pub vc_cell_height: ::core::ffi::c_uint,
    pub vc_origin: ::core::ffi::c_ulong,
    pub vc_scr_end: ::core::ffi::c_ulong,
    pub vc_visible_origin: ::core::ffi::c_ulong,
    pub vc_top: ::core::ffi::c_uint,
    pub vc_bottom: ::core::ffi::c_uint,
    pub vc_sw: *const consw,
    pub vc_screenbuf: *mut ::core::ffi::c_ushort,
    pub vc_screenbuf_size: ::core::ffi::c_uint,
    pub vc_mode: ::core::ffi::c_uchar,
    pub vc_attr: ::core::ffi::c_uchar,
    pub vc_def_color: ::core::ffi::c_uchar,
    pub vc_ulcolor: ::core::ffi::c_uchar,
    pub vc_itcolor: ::core::ffi::c_uchar,
    pub vc_halfcolor: ::core::ffi::c_uchar,
    pub vc_cursor_type: ::core::ffi::c_uint,
    pub vc_complement_mask: ::core::ffi::c_ushort,
    pub vc_s_complement_mask: ::core::ffi::c_ushort,
    pub vc_pos: ::core::ffi::c_ulong,
    pub vc_hi_font_mask: ::core::ffi::c_ushort,
    pub vc_font: vc_font,
    pub vc_video_erase_char: ::core::ffi::c_ushort,
    pub vc_state: ::core::ffi::c_uint,
    pub vc_npar: ::core::ffi::c_uint,
    pub vc_par: [::core::ffi::c_uint; NPAR],
    pub vt_mode: vt_mode,
    pub vt_pid: *mut pid,
    pub vt_newvt: ::core::ffi::c_int,
    pub paste_wait: wait_queue_head_t,
    pub vc_disp_ctrl: ::core::ffi::c_uint,
    pub vc_toggle_meta: ::core::ffi::c_uint,
    pub vc_decscnm: ::core::ffi::c_uint,
    pub vc_decom: ::core::ffi::c_uint,
    pub vc_decawm: ::core::ffi::c_uint,
    pub vc_deccm: ::core::ffi::c_uint,
    pub vc_decim: ::core::ffi::c_uint,
    pub vc_priv: ::core::ffi::c_uint,
    pub vc_need_wrap: ::core::ffi::c_uint,
    pub vc_can_do_color: ::core::ffi::c_uint,
    pub vc_report_mouse: ::core::ffi::c_uint,
    pub vc_bracketed_paste: ::core::ffi::c_uint,
    pub vc_utf: ::core::ffi::c_uchar,
    pub vc_utf_count: ::core::ffi::c_uchar,
    pub vc_utf_char: ::core::ffi::c_int,
    pub vc_tab_stop: [::core::ffi::c_ulong; 4],
    pub vc_palette: [::core::ffi::c_uchar; 16 * 3],
    pub vc_translate: *mut ::core::ffi::c_ushort,
    pub vc_bell_pitch: ::core::ffi::c_uint,
    pub vc_bell_duration: ::core::ffi::c_uint,
    pub vc_cur_blink_ms: ::core::ffi::c_ushort,
    pub vc_display_fg: *mut *mut vc_data,
    pub uni_pagedict: *mut uni_pagedict,
    pub uni_pagedict_loc: *mut *mut uni_pagedict,
    pub vc_uni_lines: *mut *mut u32,
    pub vc_saved_screen: *mut u16,
    pub vc_saved_uni_lines: *mut *mut u32,
    pub vc_saved_cols: ::core::ffi::c_uint,
    pub vc_saved_rows: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct vc {
    pub d: *mut vc_data,
    pub SAK_work: work_struct,
}

unsafe extern "C" {
    pub static mut vc_cons: [vc; MAX_NR_CONSOLES as usize];
    pub fn vc_SAK(work: *mut work_struct);
    pub fn con_is_visible(vc: *const vc_data) -> bool;
}

#[inline]
pub const fn CUR_MAKE(size: u32, change: u32, set: u32) -> u32 {
    size | (change << 8) | (set << 16)
}
#[inline]
pub const fn CUR_SIZE(c: u32) -> u32 { c & 0x00000f }
pub const CUR_DEF: u32 = 0;
pub const CUR_NONE: u32 = 1;
pub const CUR_UNDERLINE: u32 = 2;
pub const CUR_LOWER_THIRD: u32 = 3;
pub const CUR_LOWER_HALF: u32 = 4;
pub const CUR_TWO_THIRDS: u32 = 5;
pub const CUR_BLOCK: u32 = 6;
pub const CUR_SW: u32 = 0x000010;
pub const CUR_ALWAYS_BG: u32 = 0x000020;
pub const CUR_INVERT_FG_BG: u32 = 0x000040;
pub const CUR_FG: u32 = 0x000700;
pub const CUR_BG: u32 = 0x007000;
#[inline]
pub const fn CUR_CHANGE(c: u32) -> u32 { c & 0x00ff00 }
#[inline]
pub const fn CUR_SET(c: u32) -> u32 { (c & 0xff0000) >> 8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
