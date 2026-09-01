// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Load firmware files from Analog Devices SigmaStudio
 *
 * Copyright 2009-2011 Analog Devices Inc.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Dependencies supplied by the original C includes:
// <linux/device.h>, <linux/regmap.h>, <linux/list.h>, and <sound/pcm.h>.
use crate::{
    device, i2c_client, list_head, mutex, regmap, snd_pcm_hw_constraint_list,
    snd_pcm_substream, snd_soc_component,
};

#[repr(C)]
pub struct sigmadsp_ops {
    pub safeload: Option<
        unsafe extern "C" fn(
            sigmadsp: *mut sigmadsp,
            addr: c_uint,
            data: *const u8,
            len: usize,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct sigmadsp {
    pub ops: *const sigmadsp_ops,

    pub ctrl_list: list_head,
    pub data_list: list_head,

    pub rate_constraints: snd_pcm_hw_constraint_list,

    pub current_samplerate: c_uint,
    pub component: *mut snd_soc_component,
    pub dev: *mut device,

    pub lock: mutex,

    pub control_data: *mut c_void,
    pub write: Option<
        unsafe extern "C" fn(
            arg1: *mut c_void,
            arg2: c_uint,
            arg3: *const u8,
            arg4: usize,
        ) -> c_int,
    >,
    pub read: Option<
        unsafe extern "C" fn(
            arg1: *mut c_void,
            arg2: c_uint,
            arg3: *mut u8,
            arg4: usize,
        ) -> c_int,
    >,
}

unsafe extern "C" {
    pub fn devm_sigmadsp_init(
        dev: *mut device,
        ops: *const sigmadsp_ops,
        firmware_name: *const c_char,
    ) -> *mut sigmadsp;

    pub fn sigmadsp_restrict_params(
        sigmadsp: *mut sigmadsp,
        substream: *mut snd_pcm_substream,
    ) -> c_int;

    pub fn devm_sigmadsp_init_regmap(
        dev: *mut device,
        regmap: *mut regmap,
        ops: *const sigmadsp_ops,
        firmware_name: *const c_char,
    ) -> *mut sigmadsp;

    pub fn devm_sigmadsp_init_i2c(
        client: *mut i2c_client,
        ops: *const sigmadsp_ops,
        firmware_name: *const c_char,
    ) -> *mut sigmadsp;

    pub fn sigmadsp_attach(
        sigmadsp: *mut sigmadsp,
        component: *mut snd_soc_component,
    ) -> c_int;

    pub fn sigmadsp_setup(sigmadsp: *mut sigmadsp, samplerate: c_uint) -> c_int;

    pub fn sigmadsp_reset(sigmadsp: *mut sigmadsp);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
