// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright(c) 2021-2025 Intel Corporation
 *
 * Authors: Cezary Rojewski <cezary.rojewski@intel.com>
 *          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 */

// C dependencies:
// #include <sound/hdaudio_ext.h>
// #include "avs.h"
// #include "debug.h"
// #include "registers.h"

use core::ffi::{c_int, c_void};

pub type u32 = core::ffi::c_uint;

extern "C" {
    static AZX_REG_ML_LCTL: u32;
    static AZX_ML_LCTL_OFLEN: u32;

    fn avs_mtl_core_stall(adev: *mut avs_dev, core_mask: u32, stall: bool) -> c_int;
    fn snd_hdac_updatel(addr: *mut c_void, reg: u32, mask: u32, value: u32);
}

#[repr(C)]
pub struct avs_dev {
    pub base: avs_base,
}

#[repr(C)]
pub struct avs_base {
    pub core: hdac_bus,
}

#[repr(C)]
pub struct hdac_bus {
    pub hlink_list: list_head,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hdac_ext_link {
    pub ml_addr: *mut c_void,
    pub list: list_head,
}

extern "Rust" {
    fn list_for_each_entry_hdac_ext_link(
        head: *mut list_head,
        callback: unsafe extern "C" fn(*mut hdac_ext_link, *mut c_void),
        data: *mut c_void,
    );
}

unsafe extern "C" fn avs_lnl_core_stall_update_hlink(
    hlink: *mut hdac_ext_link,
    _data: *mut c_void,
) {
    snd_hdac_updatel(
        (*hlink).ml_addr,
        AZX_REG_ML_LCTL,
        AZX_ML_LCTL_OFLEN,
        AZX_ML_LCTL_OFLEN,
    );
}

#[no_mangle]
pub unsafe extern "C" fn avs_lnl_core_stall(
    adev: *mut avs_dev,
    core_mask: u32,
    stall: bool,
) -> c_int {
    let bus: *mut hdac_bus = &mut (*adev).base.core;
    let ret: c_int;

    ret = avs_mtl_core_stall(adev, core_mask, stall);

    /* On unstall, route interrupts from the links to the DSP firmware. */
    if ret == 0 && !stall {
        list_for_each_entry_hdac_ext_link(
            &mut (*bus).hlink_list,
            avs_lnl_core_stall_update_hlink,
            core::ptr::null_mut(),
        );
    }

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
