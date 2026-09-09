/* SPDX-License-Identifier: MIT */
/*
 * Rust translation of xen/interface/io/displif.h.
 * The ring and grant-table definitions are supplied by other headers.
 */

// Dependencies supplied by the Xen ring and grant-table headers:
// `grant_ref_t`, `XEN_PAGE_SIZE`, and the `DEFINE_RING_TYPES` declarations.

pub const XENDISPL_PROTOCOL_VERSION: &str = "2";
pub const XENDISPL_PROTOCOL_VERSION_INT: u32 = 2;

pub const XENDISPL_OP_DBUF_CREATE: u8 = 0x10;
pub const XENDISPL_OP_DBUF_DESTROY: u8 = 0x11;
pub const XENDISPL_OP_FB_ATTACH: u8 = 0x12;
pub const XENDISPL_OP_FB_DETACH: u8 = 0x13;
pub const XENDISPL_OP_SET_CONFIG: u8 = 0x14;
pub const XENDISPL_OP_PG_FLIP: u8 = 0x15;
/* The below command is available in protocol version 2 and above. */
pub const XENDISPL_OP_GET_EDID: u8 = 0x16;

pub const XENDISPL_EVT_PG_FLIP: u8 = 0x00;

pub const XENDISPL_DRIVER_NAME: &str = "vdispl";
pub const XENDISPL_LIST_SEPARATOR: &str = ",";
pub const XENDISPL_RESOLUTION_SEPARATOR: &str = "x";

pub const XENDISPL_FIELD_BE_VERSIONS: &str = "versions";
pub const XENDISPL_FIELD_FE_VERSION: &str = "version";
pub const XENDISPL_FIELD_REQ_RING_REF: &str = "req-ring-ref";
pub const XENDISPL_FIELD_REQ_CHANNEL: &str = "req-event-channel";
pub const XENDISPL_FIELD_EVT_RING_REF: &str = "evt-ring-ref";
pub const XENDISPL_FIELD_EVT_CHANNEL: &str = "evt-event-channel";
pub const XENDISPL_FIELD_RESOLUTION: &str = "resolution";
pub const XENDISPL_FIELD_BE_ALLOC: &str = "be-alloc";
pub const XENDISPL_FIELD_UNIQUE_ID: &str = "unique-id";

pub const XENDISPL_EDID_BLOCK_SIZE: u32 = 128;
pub const XENDISPL_EDID_BLOCK_COUNT: u32 = 256;
pub const XENDISPL_EDID_MAX_SIZE: u32 =
    XENDISPL_EDID_BLOCK_SIZE * XENDISPL_EDID_BLOCK_COUNT;

pub const XENDISPL_DBUF_FLG_REQ_ALLOC: u32 = 1 << 0;

#[repr(C)]
pub struct xendispl_dbuf_create_req {
    pub dbuf_cookie: u64,
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub buffer_sz: u32,
    pub flags: u32,
    pub gref_directory: grant_ref_t,
    pub data_ofs: u32,
}

#[repr(C)]
pub struct xendispl_page_directory {
    pub gref_dir_next_page: grant_ref_t,
    pub gref: [grant_ref_t; 0],
}

#[repr(C)]
pub struct xendispl_dbuf_destroy_req {
    pub dbuf_cookie: u64,
}

#[repr(C)]
pub struct xendispl_fb_attach_req {
    pub dbuf_cookie: u64,
    pub fb_cookie: u64,
    pub width: u32,
    pub height: u32,
    pub pixel_format: u32,
}

#[repr(C)]
pub struct xendispl_fb_detach_req {
    pub fb_cookie: u64,
}

#[repr(C)]
pub struct xendispl_set_config_req {
    pub fb_cookie: u64,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
}

#[repr(C)]
pub struct xendispl_page_flip_req {
    pub fb_cookie: u64,
}

#[repr(C)]
pub struct xendispl_get_edid_req {
    pub buffer_sz: u32,
    pub gref_directory: grant_ref_t,
}

#[repr(C)]
pub struct xendispl_get_edid_resp {
    pub edid_sz: u32,
}

#[repr(C)]
pub struct xendispl_pg_flip_evt {
    pub fb_cookie: u64,
}

#[repr(C)]
pub union xendispl_req_op {
    pub dbuf_create: xendispl_dbuf_create_req,
    pub dbuf_destroy: xendispl_dbuf_destroy_req,
    pub fb_attach: xendispl_fb_attach_req,
    pub fb_detach: xendispl_fb_detach_req,
    pub set_config: xendispl_set_config_req,
    pub pg_flip: xendispl_page_flip_req,
    pub get_edid: xendispl_get_edid_req,
    pub reserved: [u8; 56],
}

#[repr(C)]
pub struct xendispl_req {
    pub id: u16,
    pub operation: u8,
    pub reserved: [u8; 5],
    pub op: xendispl_req_op,
}

#[repr(C)]
pub union xendispl_resp_op {
    pub get_edid: xendispl_get_edid_resp,
    pub reserved1: [u8; 56],
}

#[repr(C)]
pub struct xendispl_resp {
    pub id: u16,
    pub operation: u8,
    pub reserved: u8,
    pub status: i32,
    pub op: xendispl_resp_op,
}

#[repr(C)]
pub union xendispl_evt_op {
    pub pg_flip: xendispl_pg_flip_evt,
    pub reserved: [u8; 56],
}

#[repr(C)]
pub struct xendispl_evt {
    pub id: u16,
    pub r#type: u8,
    pub reserved: [u8; 5],
    pub op: xendispl_evt_op,
}

/* DEFINE_RING_TYPES(xen_displif, struct xendispl_req, struct xendispl_resp); */

#[repr(C)]
pub struct xendispl_event_page {
    pub in_cons: u32,
    pub in_prod: u32,
    pub reserved: [u8; 56],
}

pub const XENDISPL_EVENT_PAGE_SIZE: usize = XEN_PAGE_SIZE as usize;
pub const XENDISPL_IN_RING_OFFS: usize = core::mem::size_of::<xendispl_event_page>();
pub const XENDISPL_IN_RING_SIZE: usize =
    XENDISPL_EVENT_PAGE_SIZE - XENDISPL_IN_RING_OFFS;
pub const XENDISPL_IN_RING_LEN: usize =
    XENDISPL_IN_RING_SIZE / core::mem::size_of::<xendispl_evt>();

#[inline]
pub unsafe fn XENDISPL_IN_RING(page: *mut core::ffi::c_void) -> *mut xendispl_evt {
    (page as *mut u8).add(XENDISPL_IN_RING_OFFS) as *mut xendispl_evt
}

#[inline]
pub unsafe fn XENDISPL_IN_RING_REF(
    page: *mut core::ffi::c_void,
    idx: usize,
) -> *mut xendispl_evt {
    XENDISPL_IN_RING(page).add(idx % XENDISPL_IN_RING_LEN)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
