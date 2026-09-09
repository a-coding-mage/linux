/* SPDX-License-Identifier: MIT */
/* kbdif.h -- Xen virtual keyboard/mouse */

/* Feature and parameter negotiation is described by the XenStore fields below. */

pub const XENKBD_TYPE_MOTION: u8 = 1;
pub const XENKBD_TYPE_RESERVED: u8 = 2;
pub const XENKBD_TYPE_KEY: u8 = 3;
pub const XENKBD_TYPE_POS: u8 = 4;
pub const XENKBD_TYPE_MTOUCH: u8 = 5;

/* Multi-touch event sub-codes. */
pub const XENKBD_MT_EV_DOWN: u8 = 0;
pub const XENKBD_MT_EV_UP: u8 = 1;
pub const XENKBD_MT_EV_MOTION: u8 = 2;
pub const XENKBD_MT_EV_SYN: u8 = 3;
pub const XENKBD_MT_EV_SHAPE: u8 = 4;
pub const XENKBD_MT_EV_ORIENT: u8 = 5;

pub const XENKBD_DRIVER_NAME: &str = "vkbd";
pub const XENKBD_FIELD_FEAT_DSBL_KEYBRD: &str = "feature-disable-keyboard";
pub const XENKBD_FIELD_FEAT_DSBL_POINTER: &str = "feature-disable-pointer";
pub const XENKBD_FIELD_FEAT_ABS_POINTER: &str = "feature-abs-pointer";
pub const XENKBD_FIELD_FEAT_RAW_POINTER: &str = "feature-raw-pointer";
pub const XENKBD_FIELD_FEAT_MTOUCH: &str = "feature-multi-touch";
pub const XENKBD_FIELD_REQ_ABS_POINTER: &str = "request-abs-pointer";
pub const XENKBD_FIELD_REQ_RAW_POINTER: &str = "request-raw-pointer";
pub const XENKBD_FIELD_REQ_MTOUCH: &str = "request-multi-touch";
pub const XENKBD_FIELD_RING_GREF: &str = "page-gref";
pub const XENKBD_FIELD_EVT_CHANNEL: &str = "event-channel";
pub const XENKBD_FIELD_WIDTH: &str = "width";
pub const XENKBD_FIELD_HEIGHT: &str = "height";
pub const XENKBD_FIELD_MT_WIDTH: &str = "multi-touch-width";
pub const XENKBD_FIELD_MT_HEIGHT: &str = "multi-touch-height";
pub const XENKBD_FIELD_MT_NUM_CONTACTS: &str = "multi-touch-num-contacts";
pub const XENKBD_FIELD_UNIQUE_ID: &str = "unique-id";
/* OBSOLETE, not recommended for use. */
pub const XENKBD_FIELD_RING_REF: &str = "page-ref";

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenkbd_motion { pub type_: u8, pub rel_x: i32, pub rel_y: i32, pub rel_z: i32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenkbd_key { pub type_: u8, pub pressed: u8, pub keycode: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenkbd_position { pub type_: u8, pub abs_x: i32, pub abs_y: i32, pub rel_z: i32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenkbd_mtouch_pos { pub abs_x: i32, pub abs_y: i32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenkbd_mtouch_shape { pub major: u32, pub minor: u32 }

#[repr(C)]
pub union xenkbd_mtouch_union {
    pub pos: xenkbd_mtouch_pos,
    pub shape: xenkbd_mtouch_shape,
    pub orientation: i16,
}

#[repr(C)]
pub struct xenkbd_mtouch {
    pub type_: u8,
    pub event_type: u8,
    pub contact_id: u8,
    pub reserved: [u8; 5],
    pub u: xenkbd_mtouch_union,
}

pub const XENKBD_IN_EVENT_SIZE: usize = 40;

#[repr(C)]
pub union xenkbd_in_event {
    pub type_: u8,
    pub motion: xenkbd_motion,
    pub key: xenkbd_key,
    pub pos: xenkbd_position,
    pub mtouch: xenkbd_mtouch,
    pub pad: [core::ffi::c_char; XENKBD_IN_EVENT_SIZE],
}

pub const XENKBD_OUT_EVENT_SIZE: usize = 40;

#[repr(C)]
pub union xenkbd_out_event {
    pub type_: u8,
    pub pad: [core::ffi::c_char; XENKBD_OUT_EVENT_SIZE],
}

pub const XENKBD_IN_RING_SIZE: usize = 2048;
pub const XENKBD_IN_RING_LEN: usize = XENKBD_IN_RING_SIZE / XENKBD_IN_EVENT_SIZE;
pub const XENKBD_IN_RING_OFFS: usize = 1024;

#[inline]
pub unsafe fn XENKBD_IN_RING(page: *mut core::ffi::c_char) -> *mut xenkbd_in_event {
    (page as *mut u8).add(XENKBD_IN_RING_OFFS) as *mut xenkbd_in_event
}
#[inline]
pub unsafe fn XENKBD_IN_RING_REF(page: *mut core::ffi::c_char, idx: usize) -> *mut xenkbd_in_event {
    XENKBD_IN_RING(page).add(idx % XENKBD_IN_RING_LEN)
}

pub const XENKBD_OUT_RING_SIZE: usize = 1024;
pub const XENKBD_OUT_RING_LEN: usize = XENKBD_OUT_RING_SIZE / XENKBD_OUT_EVENT_SIZE;
pub const XENKBD_OUT_RING_OFFS: usize = XENKBD_IN_RING_OFFS + XENKBD_IN_RING_SIZE;

#[inline]
pub unsafe fn XENKBD_OUT_RING(page: *mut core::ffi::c_char) -> *mut xenkbd_out_event {
    (page as *mut u8).add(XENKBD_OUT_RING_OFFS) as *mut xenkbd_out_event
}
#[inline]
pub unsafe fn XENKBD_OUT_RING_REF(page: *mut core::ffi::c_char, idx: usize) -> *mut xenkbd_out_event {
    XENKBD_OUT_RING(page).add(idx % XENKBD_OUT_RING_LEN)
}

#[repr(C)]
pub struct xenkbd_page {
    pub in_cons: u32,
    pub in_prod: u32,
    pub out_cons: u32,
    pub out_prod: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
