/*
 * dmxdev.h
 *
 * Copyright (C) 2000 Ralph Metzler & Marcus Metzler
 *                    for convergence integrated media GmbH
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation; either version 2.1
 * of the License, or (at your option) any later version.
 */

// Dependencies supplied by the surrounding kernel/media bindings.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum dmxdev_type {
    DMXDEV_TYPE_NONE,
    DMXDEV_TYPE_SEC,
    DMXDEV_TYPE_PES,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum dmxdev_state {
    DMXDEV_STATE_FREE,
    DMXDEV_STATE_ALLOCATED,
    DMXDEV_STATE_SET,
    DMXDEV_STATE_GO,
    DMXDEV_STATE_DONE,
    DMXDEV_STATE_TIMEDOUT,
}

#[repr(C)]
pub struct dmxdev_feed {
    pub pid: u16,
    pub ts: *mut dmx_ts_feed,
    pub next: list_head,
}

#[repr(C)]
pub union dmxdev_filter_filter {
    pub sec: *mut dmx_section_filter,
}

#[repr(C)]
pub union dmxdev_filter_feed {
    pub ts: list_head,
    pub sec: *mut dmx_section_feed,
}

#[repr(C)]
pub union dmxdev_filter_params {
    pub sec: dmx_sct_filter_params,
    pub pes: dmx_pes_filter_params,
}

#[repr(C)]
pub struct dmxdev_filter {
    pub filter: dmxdev_filter_filter,
    pub feed: dmxdev_filter_feed,
    pub params: dmxdev_filter_params,
    pub type_: dmxdev_type,
    pub state: dmxdev_state,
    pub dev: *mut dmxdev,
    pub buffer: dvb_ringbuffer,
    pub vb2_ctx: dvb_vb2_ctx,
    pub mutex: mutex,
    pub timer: timer_list,
    pub todo: i32,
    pub secheader: [u8; 3],
}

pub const DMXDEV_CAP_DUPLEX: i32 = 1;
pub const DVR_BUFFER_SIZE: usize = 10 * 188 * 1024;

#[repr(C)]
pub struct dmxdev {
    pub dvbdev: *mut dvb_device,
    pub dvr_dvbdev: *mut dvb_device,
    pub filter: *mut dmxdev_filter,
    pub demux: *mut dmx_demux,
    pub filternum: i32,
    pub capabilities: i32,
    // C bit-fields: may_do_mmap:1 and exit:1, represented in their storage unit.
    pub flags: u32,
    pub dvr_orig_fe: *mut dmx_frontend,
    pub dvr_buffer: dvb_ringbuffer,
    pub dvr_vb2_ctx: dvb_vb2_ctx,
    pub mutex: mutex,
    pub lock: spinlock_t,
}

extern "C" {
    pub fn dvb_dmxdev_init(dmxdev: *mut dmxdev, adap: *mut dvb_adapter) -> i32;
    pub fn dvb_dmxdev_release(dmxdev: *mut dmxdev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
