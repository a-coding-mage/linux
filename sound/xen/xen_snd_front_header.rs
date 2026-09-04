// SPDX-License-Identifier: GPL-2.0 OR MIT

// Xen para-virtual sound device
//
// Copyright (C) 2016-2018 EPAM Systems Inc.
//
// Author: Oleksandr Andrushchenko <oleksandr_andrushchenko@epam.com>

// Depends on: xen_snd_front_cfg.h

use std::os::raw::{c_int, c_uint, c_ulong};

// Forward declarations from external sources
pub enum xenbus_device {}
pub struct xen_snd_front_card_info;
pub struct xen_snd_front_evtchnl;
pub struct xen_snd_front_evtchnl_pair;
pub struct xen_front_pgdir_shbuf;
pub struct xensnd_query_hw_param;
pub struct xen_front_cfg_card;

#[repr(C)]
pub struct xen_snd_front_info {
    pub xb_dev: *mut xenbus_device,
    pub card_info: *mut xen_snd_front_card_info,
    pub num_evt_pairs: c_int,
    pub evt_pairs: *mut xen_snd_front_evtchnl_pair,
    pub cfg: xen_front_cfg_card,
}

extern "C" {
    pub fn xen_snd_front_stream_query_hw_param(
        evtchnl: *mut xen_snd_front_evtchnl,
        hw_param_req: *mut xensnd_query_hw_param,
        hw_param_resp: *mut xensnd_query_hw_param,
    ) -> c_int;

    pub fn xen_snd_front_stream_prepare(
        evtchnl: *mut xen_snd_front_evtchnl,
        shbuf: *mut xen_front_pgdir_shbuf,
        format: u8,
        channels: c_uint,
        rate: c_uint,
        buffer_sz: u32,
        period_sz: u32,
    ) -> c_int;

    pub fn xen_snd_front_stream_close(
        evtchnl: *mut xen_snd_front_evtchnl,
    ) -> c_int;

    pub fn xen_snd_front_stream_write(
        evtchnl: *mut xen_snd_front_evtchnl,
        pos: c_ulong,
        count: c_ulong,
    ) -> c_int;

    pub fn xen_snd_front_stream_read(
        evtchnl: *mut xen_snd_front_evtchnl,
        pos: c_ulong,
        count: c_ulong,
    ) -> c_int;

    pub fn xen_snd_front_stream_trigger(
        evtchnl: *mut xen_snd_front_evtchnl,
        type_: c_int,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
