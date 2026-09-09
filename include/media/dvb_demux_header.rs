/*
 * dvb_demux.h: DVB kernel demux API
 *
 * Copyright (C) 2000-2001 Marcus Metzler & Ralph Metzler
 *                         for convergence integrated media GmbH
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation; either version 2.1
 * of the License, or (at your option) any later version.
 */

// C dependencies: linux/time.h, linux/timer.h, linux/spinlock.h,
// linux/mutex.h, and media/demux.h.

#[repr(C)]
pub enum dvb_dmx_filter_type {
    DMX_TYPE_TS,
    DMX_TYPE_SEC,
}

#[repr(C)]
pub enum dvb_dmx_state {
    DMX_STATE_FREE,
    DMX_STATE_ALLOCATED,
    DMX_STATE_READY,
    DMX_STATE_GO,
}

pub const DVB_DEMUX_MASK_MAX: usize = 18;
pub const MAX_PID: u16 = 0x1fff;
pub const SPEED_PKTS_INTERVAL: u32 = 50000;

#[repr(C)]
pub struct dvb_demux_filter {
    pub filter: dmx_section_filter,
    pub maskandmode: [u8; DMX_MAX_FILTER_SIZE],
    pub maskandnotmode: [u8; DMX_MAX_FILTER_SIZE],
    pub doneq: bool,
    pub next: *mut dvb_demux_filter,
    pub feed: *mut dvb_demux_feed,
    pub index: i32,
    pub state: dvb_dmx_state,
    pub r#type: dvb_dmx_filter_type,
    pub hw_handle: u16,
}

#[repr(C)]
pub union dvb_demux_feed_feed {
    pub ts: dmx_ts_feed,
    pub sec: dmx_section_feed,
}

#[repr(C)]
pub union dvb_demux_feed_cb {
    pub ts: dmx_ts_cb,
    pub sec: dmx_section_cb,
}

#[repr(C)]
pub struct dvb_demux_feed {
    pub feed: dvb_demux_feed_feed,
    pub cb: dvb_demux_feed_cb,
    pub demux: *mut dvb_demux,
    pub r#priv: *mut core::ffi::c_void,
    pub r#type: dvb_dmx_filter_type,
    pub state: dvb_dmx_state,
    pub pid: u16,
    pub timeout: ktime_t,
    pub filter: *mut dvb_demux_filter,
    pub buffer_flags: u32,
    pub ts_type: ts_filter_type,
    pub pes_type: dmx_ts_pes,
    pub cc: i32,
    pub pusi_seen: bool,
    pub peslen: u16,
    pub list_head: list_head,
    pub index: u32,
}

#[repr(C)]
pub struct dvb_demux {
    pub dmx: dmx_demux,
    pub r#priv: *mut core::ffi::c_void,
    pub filternum: i32,
    pub feednum: i32,
    pub start_feed: Option<unsafe extern "C" fn(*mut dvb_demux_feed) -> i32>,
    pub stop_feed: Option<unsafe extern "C" fn(*mut dvb_demux_feed) -> i32>,
    pub write_to_decoder: Option<unsafe extern "C" fn(*mut dvb_demux_feed, *const u8, usize) -> i32>,
    pub check_crc32: Option<unsafe extern "C" fn(*mut dvb_demux_feed, *const u8, usize) -> u32>,
    pub memcopy: Option<unsafe extern "C" fn(*mut dvb_demux_feed, *mut u8, *const u8, usize)>,
    pub users: i32,
    pub filter: *mut dvb_demux_filter,
    pub feed: *mut dvb_demux_feed,
    pub frontend_list: list_head,
    pub pesfilter: [*mut dvb_demux_feed; DMX_PES_OTHER],
    pub pids: [u16; DMX_PES_OTHER],
    pub feed_list: list_head,
    pub tsbuf: [u8; 204],
    pub tsbufp: i32,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub cnt_storage: *mut u8,
    pub speed_last_time: ktime_t,
    pub speed_pkts_cnt: u32,
    pub playing: i32,
    pub recording: i32,
}

pub const MAX_DVB_DEMUX_USERS: usize = 10;
pub const DMX_MAX_PID: usize = 0x2000;

extern "C" {
    pub fn dvb_dmx_init(demux: *mut dvb_demux) -> i32;
    pub fn dvb_dmx_release(demux: *mut dvb_demux);
    pub fn dvb_dmx_swfilter_packets(demux: *mut dvb_demux, buf: *const u8, count: usize);
    pub fn dvb_dmx_swfilter(demux: *mut dvb_demux, buf: *const u8, count: usize);
    pub fn dvb_dmx_swfilter_204(demux: *mut dvb_demux, buf: *const u8, count: usize);
    pub fn dvb_dmx_swfilter_raw(demux: *mut dvb_demux, buf: *const u8, count: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
