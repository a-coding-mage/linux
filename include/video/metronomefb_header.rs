/*
 * metronomefb.h - definitions for the metronome framebuffer driver
 *
 * Copyright (C) 2008 by Jaya Kumar
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file COPYING in the main directory of this archive for
 * more details.
 *
 */

// C header guard: _LINUX_METRONOMEFB_H_

/* command structure used by metronome controller */
#[repr(C)]
pub struct metromem_cmd {
    pub opcode: u16,
    pub args: [u16; (64 - 2) / 2],
    pub csum: u16,
}

/* struct used by metronome. board specific stuff comes from *board */
#[repr(C)]
pub struct metronomefb_par {
    pub metromem_cmd: *mut metromem_cmd,
    pub metromem_wfm: *mut u8,
    pub metromem_img: *mut u8,
    pub metromem_img_csum: *mut u16,
    pub csum_table: *mut u16,
    pub metromem_dma: dma_addr_t,
    pub info: *mut fb_info,
    pub board: *mut metronome_board,
    pub waitq: wait_queue_head_t,
    pub frame_count: u8,
    pub extra_size: ::core::ffi::c_int,
    pub dt: ::core::ffi::c_int,
}

/* board specific routines and data */
#[repr(C)]
pub struct metronome_board {
    pub owner: *mut module, /* the platform device */
    pub set_rst: Option<unsafe extern "C" fn(*mut metronomefb_par, ::core::ffi::c_int)>,
    pub set_stdby: Option<unsafe extern "C" fn(*mut metronomefb_par, ::core::ffi::c_int)>,
    pub cleanup: Option<unsafe extern "C" fn(*mut metronomefb_par)>,
    pub met_wait_event:
        Option<unsafe extern "C" fn(*mut metronomefb_par) -> ::core::ffi::c_int>,
    pub met_wait_event_intr:
        Option<unsafe extern "C" fn(*mut metronomefb_par) -> ::core::ffi::c_int>,
    pub setup_irq: Option<unsafe extern "C" fn(*mut fb_info) -> ::core::ffi::c_int>,
    pub setup_fb: Option<unsafe extern "C" fn(*mut metronomefb_par) -> ::core::ffi::c_int>,
    pub setup_io: Option<unsafe extern "C" fn(*mut metronomefb_par) -> ::core::ffi::c_int>,
    pub get_panel_type: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub metromem: *mut u8,
    pub fw: ::core::ffi::c_int,
    pub fh: ::core::ffi::c_int,
    pub wfm_size: ::core::ffi::c_int,
    pub host_fbinfo: *mut fb_info, /* the host LCD controller's fbi */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
