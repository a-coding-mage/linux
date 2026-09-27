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
    pub x: ::kernel::ffi::c_uint,
    pub y: ::kernel::ffi::c_uint,
    pub color: ::kernel::ffi::c_uchar,
    pub Gx_charset: [::kernel::ffi::c_uchar; 2],
    pub charset: ::kernel::ffi::c_uint,
    pub intensity: vc_intensity,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub reverse: bool,
}

#[repr(C)]
pub struct vc_font {
    pub width: ::kernel::ffi::c_uint,
    pub height: ::kernel::ffi::c_uint,
    pub charcount: ::kernel::ffi::c_uint,
    pub data: *const ::kernel::ffi::c_uchar,
}

unsafe extern "C" {
    pub fn vc_font_pitch(font: *const vc_font) -> ::kernel::ffi::c_uint;
    pub fn vc_font_size(font: *const vc_font) -> ::kernel::ffi::c_uint;
}

#[repr(C)]
pub struct vc_data {
    pub port: tty_port,
    pub state: vc_state,
    pub saved_state: vc_state,
    pub vc_num: ::kernel::ffi::c_ushort,
    pub vc_cols: ::kernel::ffi::c_uint,
    pub vc_rows: ::kernel::ffi::c_uint,
    pub vc_size_row: ::kernel::ffi::c_uint,
    pub vc_scan_lines: ::kernel::ffi::c_uint,
    pub vc_cell_height: ::kernel::ffi::c_uint,
    pub vc_origin: ::kernel::ffi::c_ulong,
    pub vc_scr_end: ::kernel::ffi::c_ulong,
    pub vc_visible_origin: ::kernel::ffi::c_ulong,
    pub vc_top: ::kernel::ffi::c_uint,
    pub vc_bottom: ::kernel::ffi::c_uint,
    pub vc_sw: *const consw,
    pub vc_screenbuf: *mut ::kernel::ffi::c_ushort,
    pub vc_screenbuf_size: ::kernel::ffi::c_uint,
    pub vc_mode: ::kernel::ffi::c_uchar,
    pub vc_attr: ::kernel::ffi::c_uchar,
    pub vc_def_color: ::kernel::ffi::c_uchar,
    pub vc_ulcolor: ::kernel::ffi::c_uchar,
    pub vc_itcolor: ::kernel::ffi::c_uchar,
    pub vc_halfcolor: ::kernel::ffi::c_uchar,
    pub vc_cursor_type: ::kernel::ffi::c_uint,
    pub vc_complement_mask: ::kernel::ffi::c_ushort,
    pub vc_s_complement_mask: ::kernel::ffi::c_ushort,
    pub vc_pos: ::kernel::ffi::c_ulong,
    pub vc_hi_font_mask: ::kernel::ffi::c_ushort,
    pub vc_font: vc_font,
    pub vc_video_erase_char: ::kernel::ffi::c_ushort,
    pub vc_state: ::kernel::ffi::c_uint,
    pub vc_npar: ::kernel::ffi::c_uint,
    pub vc_par: [::kernel::ffi::c_uint; NPAR],
    pub vt_mode: vt_mode,
    pub vt_pid: *mut pid,
    pub vt_newvt: ::kernel::ffi::c_int,
    pub paste_wait: wait_queue_head_t,
    pub vc_disp_ctrl: ::kernel::ffi::c_uint,
    pub vc_toggle_meta: ::kernel::ffi::c_uint,
    pub vc_decscnm: ::kernel::ffi::c_uint,
    pub vc_decom: ::kernel::ffi::c_uint,
    pub vc_decawm: ::kernel::ffi::c_uint,
    pub vc_deccm: ::kernel::ffi::c_uint,
    pub vc_decim: ::kernel::ffi::c_uint,
    pub vc_priv: ::kernel::ffi::c_uint,
    pub vc_need_wrap: ::kernel::ffi::c_uint,
    pub vc_can_do_color: ::kernel::ffi::c_uint,
    pub vc_report_mouse: ::kernel::ffi::c_uint,
    pub vc_bracketed_paste: ::kernel::ffi::c_uint,
    pub vc_utf: ::kernel::ffi::c_uchar,
    pub vc_utf_count: ::kernel::ffi::c_uchar,
    pub vc_utf_char: ::kernel::ffi::c_int,
    pub vc_tab_stop: [::kernel::ffi::c_ulong; 4],
    pub vc_palette: [::kernel::ffi::c_uchar; 16 * 3],
    pub vc_translate: *mut ::kernel::ffi::c_ushort,
    pub vc_bell_pitch: ::kernel::ffi::c_uint,
    pub vc_bell_duration: ::kernel::ffi::c_uint,
    pub vc_cur_blink_ms: ::kernel::ffi::c_ushort,
    pub vc_display_fg: *mut *mut vc_data,
    pub uni_pagedict: *mut uni_pagedict,
    pub uni_pagedict_loc: *mut *mut uni_pagedict,
    pub vc_uni_lines: *mut *mut u32,
    pub vc_saved_screen: *mut u16,
    pub vc_saved_uni_lines: *mut *mut u32,
    pub vc_saved_cols: ::kernel::ffi::c_uint,
    pub vc_saved_rows: ::kernel::ffi::c_uint,
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
