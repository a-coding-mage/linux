/* SPDX-License-Identifier: MIT */
/*
 * fbif.h -- Xen virtual frame buffer device
 *
 * Copyright (C) 2005 Anthony Liguori <aliguori@us.ibm.com>
 * Copyright (C) 2006 Red Hat, Inc., Markus Armbruster <armbru@redhat.com>
 */

/* Out events (frontend -> backend). */

pub const XENFB_TYPE_UPDATE: u8 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenfb_update {
    pub type_: u8,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub const XENFB_TYPE_RESIZE: u8 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenfb_resize {
    pub type_: u8,
    pub width: i32,
    pub height: i32,
    pub stride: i32,
    pub depth: i32,
    pub offset: i32,
}

pub const XENFB_OUT_EVENT_SIZE: usize = 40;

#[repr(C)]
#[derive(Copy, Clone)]
pub union xenfb_out_event {
    pub type_: u8,
    pub update: xenfb_update,
    pub resize: xenfb_resize,
    pub pad: [core::ffi::c_char; XENFB_OUT_EVENT_SIZE],
}

/* In events (backend -> frontend). */

pub const XENFB_IN_EVENT_SIZE: usize = 40;

#[repr(C)]
#[derive(Copy, Clone)]
pub union xenfb_in_event {
    pub type_: u8,
    pub pad: [core::ffi::c_char; XENFB_IN_EVENT_SIZE],
}

/* shared page */

pub const XENFB_IN_RING_SIZE: usize = 1024;
pub const XENFB_IN_RING_LEN: usize = XENFB_IN_RING_SIZE / XENFB_IN_EVENT_SIZE;
pub const XENFB_IN_RING_OFFS: usize = 1024;

#[inline]
pub unsafe fn XENFB_IN_RING(page: *mut u8) -> *mut xenfb_in_event {
    page.add(XENFB_IN_RING_OFFS) as *mut xenfb_in_event
}

#[inline]
pub unsafe fn XENFB_IN_RING_REF(page: *mut u8, idx: usize) -> *mut xenfb_in_event {
    XENFB_IN_RING(page).add(idx % XENFB_IN_RING_LEN)
}

pub const XENFB_OUT_RING_SIZE: usize = 2048;
pub const XENFB_OUT_RING_LEN: usize = XENFB_OUT_RING_SIZE / XENFB_OUT_EVENT_SIZE;
pub const XENFB_OUT_RING_OFFS: usize = XENFB_IN_RING_OFFS + XENFB_IN_RING_SIZE;

#[inline]
pub unsafe fn XENFB_OUT_RING(page: *mut u8) -> *mut xenfb_out_event {
    page.add(XENFB_OUT_RING_OFFS) as *mut xenfb_out_event
}

#[inline]
pub unsafe fn XENFB_OUT_RING_REF(page: *mut u8, idx: usize) -> *mut xenfb_out_event {
    XENFB_OUT_RING(page).add(idx % XENFB_OUT_RING_LEN)
}

#[repr(C)]
pub struct xenfb_page {
    pub in_cons: u32,
    pub in_prod: u32,
    pub out_cons: u32,
    pub out_prod: u32,

    pub width: i32,
    pub height: i32,
    pub line_length: u32,
    pub mem_length: u32,
    pub depth: u8,

    /* Framebuffer page directory. */
    pub pd: [usize; 256],
}

/* xenkbd default resolution; exposed only to kernel builds in C. */
#[cfg(feature = "kernel")]
pub const XENFB_WIDTH: i32 = 800;
#[cfg(feature = "kernel")]
pub const XENFB_HEIGHT: i32 = 600;
#[cfg(feature = "kernel")]
pub const XENFB_DEPTH: i32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
