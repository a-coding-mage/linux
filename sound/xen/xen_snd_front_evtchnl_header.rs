// SPDX-License-Identifier: GPL-2.0 OR MIT

// Xen para-virtual sound device
//
// Copyright (C) 2016-2018 EPAM Systems Inc.
//
// Author: Oleksandr Andrushchenko <oleksandr_andrushchenko@epam.com>

// Requires: xen/interface/io/sndif.h

pub struct XenSndFrontInfo;

// Timeout in ms to wait for backend to respond.
pub const VSND_WAIT_BACK_MS: i32 = 3000;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum XenSndFrontEvtchnlState {
    EvtchnlStateDisconnected = 0,
    EvtchnlStateConnected = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum XenSndFrontEvtchnlType {
    EvtchnlTypeReq = 0,
    EvtchnlTypeEvt = 1,
}

#[repr(C)]
pub struct XenSndFrontEvtchnl {
    pub front_info: *mut XenSndFrontInfo,
    pub gref: i32,
    pub port: i32,
    pub irq: i32,
    pub index: i32,
    // State of the event channel.
    pub state: XenSndFrontEvtchnlState,
    // C field name: type (Rust keyword, renamed with trailing underscore)
    pub type_: XenSndFrontEvtchnlType,
    // Current response id or next expected incoming event id.
    pub evt_id: u16,
    // Next request id.
    pub evt_next_id: u16,
    // Shared ring access lock.
    pub ring_io_lock: Mutex,
    pub u: XenSndFrontEvtchnlU,
}

#[repr(C)]
pub union XenSndFrontEvtchnlU {
    pub req: XenSndFrontEvtchnlReq,
    pub evt: XenSndFrontEvtchnlEvt,
}

#[repr(C)]
pub struct XenSndFrontEvtchnlReq {
    pub ring: XenSndifFrontRing,
    pub completion: Completion,
    // Serializer for backend IO: request/response.
    pub req_io_lock: Mutex,
    // Latest response status.
    pub resp_status: i32,
    pub resp: XenSndFrontEvtchnlReqResp,
}

#[repr(C)]
pub union XenSndFrontEvtchnlReqResp {
    pub hw_param: XensndQueryHwParam,
}

#[repr(C)]
pub struct XenSndFrontEvtchnlEvt {
    pub page: *mut XensndEventPage,
    // This is needed to handle XENSND_EVT_CUR_POS event.
    pub substream: *mut SndPcmSubstream,
}

#[repr(C)]
pub struct XenSndFrontEvtchnlPair {
    pub req: XenSndFrontEvtchnl,
    pub evt: XenSndFrontEvtchnl,
}

// Opaque types from external headers (xen/interface/io/sndif.h and Linux kernel headers)
pub struct Mutex;
pub struct Completion;
pub struct XenSndifFrontRing;
pub struct XensndQueryHwParam;
pub struct XensndEventPage;
pub struct SndPcmSubstream;

extern "C" {
    pub fn xen_snd_front_evtchnl_create_all(
        front_info: *mut XenSndFrontInfo,
        num_streams: i32,
    ) -> i32;

    pub fn xen_snd_front_evtchnl_free_all(front_info: *mut XenSndFrontInfo);

    pub fn xen_snd_front_evtchnl_publish_all(front_info: *mut XenSndFrontInfo) -> i32;

    pub fn xen_snd_front_evtchnl_flush(evtchnl: *mut XenSndFrontEvtchnl);

    pub fn xen_snd_front_evtchnl_set_connected(
        channel: *mut XenSndFrontEvtchnl,
        is_connected: bool,
    );

    pub fn xen_snd_front_evtchnl_pair_set_connected(
        evt_pair: *mut XenSndFrontEvtchnlPair,
        is_connected: bool,
    );

    pub fn xen_snd_front_evtchnl_pair_clear(evt_pair: *mut XenSndFrontEvtchnlPair);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
