/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the corresponding UAPI header:
// #include <uapi/video/uvesafb.h>

/* VBE CRTC Info Block */
#[repr(C, packed)]
pub struct vbe_crtc_ib {
    pub horiz_total: u16,
    pub horiz_start: u16,
    pub horiz_end: u16,
    pub vert_total: u16,
    pub vert_start: u16,
    pub vert_end: u16,
    pub flags: u8,
    pub pixel_clock: u32,
    pub refresh_rate: u16,
    pub reserved: [u8; 40],
}

pub const VBE_MODE_VGACOMPAT: u32 = 0x20;
pub const VBE_MODE_COLOR: u32 = 0x08;
pub const VBE_MODE_SUPPORTEDHW: u32 = 0x01;
pub const VBE_MODE_GRAPHICS: u32 = 0x10;
pub const VBE_MODE_LFB: u32 = 0x80;

pub const VBE_MODE_MASK: u32 =
    VBE_MODE_COLOR | VBE_MODE_SUPPORTEDHW | VBE_MODE_GRAPHICS | VBE_MODE_LFB;

/* VBE Mode Info Block */
#[repr(C, packed)]
pub struct vbe_mode_ib {
    /* for all VBE revisions */
    pub mode_attr: u16,
    pub winA_attr: u8,
    pub winB_attr: u8,
    pub win_granularity: u16,
    pub win_size: u16,
    pub winA_seg: u16,
    pub winB_seg: u16,
    pub win_func_ptr: u32,
    pub bytes_per_scan_line: u16,

    /* for VBE 1.2+ */
    pub x_res: u16,
    pub y_res: u16,
    pub x_char_size: u8,
    pub y_char_size: u8,
    pub planes: u8,
    pub bits_per_pixel: u8,
    pub banks: u8,
    pub memory_model: u8,
    pub bank_size: u8,
    pub image_pages: u8,
    pub reserved1: u8,

    /* Direct color fields for direct/6 and YUV/7 memory models. */
    /* Offsets are bit positions of lsb in the mask. */
    pub red_len: u8,
    pub red_off: u8,
    pub green_len: u8,
    pub green_off: u8,
    pub blue_len: u8,
    pub blue_off: u8,
    pub rsvd_len: u8,
    pub rsvd_off: u8,
    pub direct_color_info: u8, /* direct color mode attributes */

    /* for VBE 2.0+ */
    pub phys_base_ptr: u32,
    pub reserved2: [u8; 6],

    /* for VBE 3.0+ */
    pub lin_bytes_per_scan_line: u16,
    pub bnk_image_pages: u8,
    pub lin_image_pages: u8,
    pub lin_red_len: u8,
    pub lin_red_off: u8,
    pub lin_green_len: u8,
    pub lin_green_off: u8,
    pub lin_blue_len: u8,
    pub lin_blue_off: u8,
    pub lin_rsvd_len: u8,
    pub lin_rsvd_off: u8,
    pub max_pixel_clock: u32,
    pub mode_id: u16,
    pub depth: u8,
}

pub const UVESAFB_DEFAULT_MODE: &str = "640x480-16";

/* How long to wait for a reply from userspace [ms] */
pub const UVESAFB_TIMEOUT: u32 = 5000;

/* Max number of concurrent tasks */
pub const UVESAFB_TASKS_MAX: u32 = 16;

pub const dac_reg: u32 = 0x3c8;
pub const dac_val: u32 = 0x3c9;

#[repr(C, packed)]
pub struct uvesafb_pal_entry {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub pad: u8,
}

#[repr(C)]
pub struct uvesafb_ktask {
    pub t: uvesafb_task,
    pub buf: *mut core::ffi::c_void,
    pub done: *mut completion,
    pub ack: u32,
}

pub const UVESAFB_EXACT_RES: u32 = 1;
pub const UVESAFB_EXACT_DEPTH: u32 = 2;

#[repr(C)]
pub struct uvesafb_par {
    pub vbe_ib: vbe_ib, /* VBE Info Block */
    pub vbe_modes: *mut vbe_mode_ib, /* list of supported VBE modes */
    pub vbe_modes_cnt: i32,

    pub nocrtc: u8,
    pub ypan: u8, /* 0 - nothing, 1 - ypan, 2 - ywrap */
    pub pmi_setpal: u8, /* PMI for palette changes */
    pub pmi_base: *mut u16, /* protected mode interface location */
    pub pmi_start: *mut core::ffi::c_void,
    pub pmi_pal: *mut core::ffi::c_void,
    pub vbe_state_orig: *mut u8, /* original hardware state, before the driver was loaded */
    pub vbe_state_saved: *mut u8, /* state saved by fb_save_state */
    pub vbe_state_size: i32,
    pub ref_count: atomic_t,

    pub mode_idx: i32,
    pub crtc: vbe_crtc_ib,
    pub mtrr_handle: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
