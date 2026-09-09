/* SPDX-License-Identifier: MIT */

/*
 * usbif.h
 *
 * USB I/O interface for Xen guest OSes.
 *
 * Copyright (C) 2009, FUJITSU LABORATORIES LTD.
 * Author: Noboru Iwamatsu <n_iwamatsu@jp.fujitsu.com>
 */

/* Detailed interface description and protocol layouts are preserved from the
 * C header above; the included ring and grant-table definitions are external
 * dependencies supplied by the surrounding Xen interface. */

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum xenusb_spec_version {
    XENUSB_VER_UNKNOWN = 0,
    XENUSB_VER_USB11,
    XENUSB_VER_USB20,
    XENUSB_VER_USB30, /* not supported yet */
}

pub const XENUSB_PIPE_PORT_MASK: u32 = 0x0000001f;
pub const XENUSB_PIPE_UNLINK: u32 = 0x00000020;
pub const XENUSB_PIPE_DIR: u32 = 0x00000080;
pub const XENUSB_PIPE_DEV_MASK: u32 = 0x0000007f;
pub const XENUSB_PIPE_DEV_SHIFT: u32 = 8;
pub const XENUSB_PIPE_EP_MASK: u32 = 0x0000000f;
pub const XENUSB_PIPE_EP_SHIFT: u32 = 15;
pub const XENUSB_PIPE_TYPE_MASK: u32 = 0x00000003;
pub const XENUSB_PIPE_TYPE_SHIFT: u32 = 30;
pub const XENUSB_PIPE_TYPE_ISOC: u32 = 0;
pub const XENUSB_PIPE_TYPE_INT: u32 = 1;
pub const XENUSB_PIPE_TYPE_CTRL: u32 = 2;
pub const XENUSB_PIPE_TYPE_BULK: u32 = 3;

#[inline]
pub const fn xenusb_pipeportnum(pipe: u32) -> u32 { pipe & XENUSB_PIPE_PORT_MASK }
#[inline]
pub const fn xenusb_setportnum_pipe(pipe: u32, portnum: u32) -> u32 { pipe | portnum }
#[inline]
pub const fn xenusb_pipeunlink(pipe: u32) -> u32 { pipe & XENUSB_PIPE_UNLINK }
#[inline]
pub const fn xenusb_pipesubmit(pipe: u32) -> bool { xenusb_pipeunlink(pipe) == 0 }
#[inline]
pub const fn xenusb_setunlink_pipe(pipe: u32) -> u32 { pipe | XENUSB_PIPE_UNLINK }
#[inline]
pub const fn xenusb_pipein(pipe: u32) -> u32 { pipe & XENUSB_PIPE_DIR }
#[inline]
pub const fn xenusb_pipeout(pipe: u32) -> bool { xenusb_pipein(pipe) == 0 }
#[inline]
pub const fn xenusb_pipedevice(pipe: u32) -> u32 {
    (pipe >> XENUSB_PIPE_DEV_SHIFT) & XENUSB_PIPE_DEV_MASK
}
#[inline]
pub const fn xenusb_pipeendpoint(pipe: u32) -> u32 {
    (pipe >> XENUSB_PIPE_EP_SHIFT) & XENUSB_PIPE_EP_MASK
}
#[inline]
pub const fn xenusb_pipetype(pipe: u32) -> u32 {
    (pipe >> XENUSB_PIPE_TYPE_SHIFT) & XENUSB_PIPE_TYPE_MASK
}
#[inline]
pub const fn xenusb_pipeisoc(pipe: u32) -> bool { xenusb_pipetype(pipe) == XENUSB_PIPE_TYPE_ISOC }
#[inline]
pub const fn xenusb_pipeint(pipe: u32) -> bool { xenusb_pipetype(pipe) == XENUSB_PIPE_TYPE_INT }
#[inline]
pub const fn xenusb_pipectrl(pipe: u32) -> bool { xenusb_pipetype(pipe) == XENUSB_PIPE_TYPE_CTRL }
#[inline]
pub const fn xenusb_pipebulk(pipe: u32) -> bool { xenusb_pipetype(pipe) == XENUSB_PIPE_TYPE_BULK }

pub const XENUSB_MAX_SEGMENTS_PER_REQUEST: usize = 16;
pub const XENUSB_MAX_PORTNR: u32 = 31;
pub const XENUSB_RING_SIZE: usize = 4096;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenusb_request_segment {
    pub gref: grant_ref_t,
    pub offset: u16,
    pub length: u16,
}

#[repr(C)]
pub union xenusb_urb_request_u {
    pub ctrl: [u8; 8],
    pub isoc: xenusb_urb_request_isoc,
    pub intr: xenusb_urb_request_intr,
    pub unlink: xenusb_urb_request_unlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenusb_urb_request_isoc {
    pub interval: u16,
    pub start_frame: u16,
    pub number_of_packets: u16,
    pub nr_frame_desc_segs: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenusb_urb_request_intr {
    pub interval: u16,
    pub pad: [u16; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenusb_urb_request_unlink {
    pub unlink_id: u16,
    pub pad: [u16; 3],
}

pub const XENUSB_SHORT_NOT_OK: u16 = 0x0001;

#[repr(C)]
pub struct xenusb_urb_request {
    pub id: u16,
    pub nr_buffer_segs: u16,
    pub pipe: u32,
    pub transfer_flags: u16,
    pub buffer_length: u16,
    pub u: xenusb_urb_request_u,
    pub seg: [xenusb_request_segment; XENUSB_MAX_SEGMENTS_PER_REQUEST],
}

#[repr(C)]
pub struct xenusb_urb_response {
    pub id: u16,
    pub start_frame: u16,
    pub status: i32,
    pub actual_length: i32,
    pub error_count: i32,
}

pub const XENUSB_STATUS_OK: i32 = 0;
pub const XENUSB_STATUS_NODEV: i32 = -19;
pub const XENUSB_STATUS_INVAL: i32 = -22;
pub const XENUSB_STATUS_STALL: i32 = -32;
pub const XENUSB_STATUS_IOERROR: i32 = -71;
pub const XENUSB_STATUS_BABBLE: i32 = -75;
pub const XENUSB_STATUS_SHUTDOWN: i32 = -108;

/* DEFINE_RING_TYPES(xenusb_urb, xenusb_urb_request, xenusb_urb_response); */
pub const XENUSB_URB_RING_SIZE: usize = __CONST_RING_SIZE_xenusb_urb(XENUSB_RING_SIZE);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenusb_conn_request { pub id: u16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenusb_conn_response {
    pub id: u16,
    pub portnum: u8,
    pub speed: u8,
}

pub const XENUSB_SPEED_NONE: u8 = 0;
pub const XENUSB_SPEED_LOW: u8 = 1;
pub const XENUSB_SPEED_FULL: u8 = 2;
pub const XENUSB_SPEED_HIGH: u8 = 3;

/* DEFINE_RING_TYPES(xenusb_conn, xenusb_conn_request, xenusb_conn_response); */
pub const XENUSB_CONN_RING_SIZE: usize = __CONST_RING_SIZE_xenusb_conn(XENUSB_RING_SIZE);

/* External ring/grant-table dependencies from ring.h and grant_table.h. */
extern "Rust" {
    fn __CONST_RING_SIZE_xenusb_urb(size: usize) -> usize;
    fn __CONST_RING_SIZE_xenusb_conn(size: usize) -> usize;
}

/* grant_ref_t is supplied by the external Xen grant-table interface. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
