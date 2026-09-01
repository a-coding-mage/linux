// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Authors: Keyon Jie <yang.jie@linux.intel.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_int, c_void};
use core::mem::size_of;
use core::ptr;

// C includes translated as external dependencies:
// <linux/io.h>
// <sound/hdaudio.h>
// <sound/hda_i915.h>
// <sound/hda_codec.h>
// <sound/hda_register.h>
// "../sof-priv.h"
// "hda.h"

type bool_ = bool;

const WAKEEN: c_int = 0;
const STATESTS_INT_MASK: u32 = 0;
const HDA_IDISP_ADDR: c_uint = 0;
const HDA_CODEC_IDX_CONTROLLER: c_int = 0;
const SOF_INTEL_ACE_2_0: c_uint = 0;

type c_uint = u32;
type c_ulong = usize;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct spinlock_t {
    _data: [u8; 0],
}

#[repr(C)]
pub struct device {
    _data: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_pdata {
    _data: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    pub pdata: *mut snd_sof_pdata,
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub hw_ip_version: c_uint,
}

#[repr(C)]
pub struct hdac_bus {
    pub dev: *mut device,
    pub stream_list: list_head,
    pub irq: c_int,
    pub idx: c_int,
    pub reg_lock: spinlock_t,
    pub codec_powered: c_ulong,
    pub use_pio_for_commands: bool_,
}

#[repr(C)]
pub struct hdac_device {
    pub bus: *mut hdac_bus,
    pub addr: c_uint,
}

#[repr(C)]
pub struct hdac_bus_ops {
    pub command: Option<unsafe extern "C" fn(*mut hdac_bus, c_uint) -> c_int>,
    pub get_response: Option<unsafe extern "C" fn(*mut hdac_bus, c_uint, *mut c_uint) -> c_int>,
    pub link_power: Option<unsafe extern "C" fn(*mut hdac_device, bool_)>,
}

unsafe extern "C" {
    fn snd_hdac_chip_readw(bus: *mut hdac_bus, reg: c_int) -> c_uint;
    fn snd_hdac_chip_updatew(bus: *mut hdac_bus, reg: c_int, mask: c_uint, val: c_uint);
    fn snd_hdac_ext_bus_link_power(codec: *mut hdac_device, enable: bool_);
    fn snd_hdac_display_power(bus: *mut hdac_bus, idx: c_int, enable: bool_);
    fn snd_hdac_bus_send_cmd(bus: *mut hdac_bus, val: c_uint) -> c_int;
    fn snd_hdac_bus_get_response(bus: *mut hdac_bus, addr: c_uint, res: *mut c_uint) -> c_int;
    fn snd_soc_hdac_hda_get_ops() -> *const c_void;
    fn sof_to_bus(sdev: *mut snd_sof_dev) -> *mut hdac_bus;
    fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_intel_dsp_desc;
    fn snd_hdac_ext_bus_init(
        bus: *mut hdac_bus,
        dev: *mut device,
        ops: *const hdac_bus_ops,
        ext_ops: *const c_void,
    );
    fn snd_hdac_ext_bus_exit(bus: *mut hdac_bus);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn spin_lock_init(lock: *mut spinlock_t);
}

#[inline]
const fn BIT(nr: c_uint) -> c_uint {
    1u32.wrapping_shl(nr)
}

#[inline]
unsafe fn test_bit(nr: c_uint, addr: *const c_ulong) -> bool_ {
    ((*addr) & ((1usize).wrapping_shl(nr))) != 0
}

// Original C condition:
// #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_AUDIO_CODEC)
// #include "../../codecs/hdac_hda.h"
// #define sof_hda_ext_ops snd_soc_hdac_hda_get_ops()
unsafe fn sof_hda_ext_ops() -> *const c_void {
    snd_soc_hdac_hda_get_ops()
}

unsafe fn update_codec_wake_enable(bus: *mut hdac_bus, addr: c_uint, link_power: bool_) {
    let mut mask: c_uint = snd_hdac_chip_readw(bus, WAKEEN);

    if link_power {
        mask &= !BIT(addr);
    } else {
        mask |= BIT(addr);
    }

    snd_hdac_chip_updatew(bus, WAKEEN, STATESTS_INT_MASK, mask);
}

unsafe extern "C" fn sof_hda_bus_link_power(codec: *mut hdac_device, enable: bool_) {
    let bus: *mut hdac_bus = (*codec).bus;
    let oldstate: bool_ = test_bit((*codec).addr, ptr::addr_of!((*bus).codec_powered));

    snd_hdac_ext_bus_link_power(codec, enable);

    if enable == oldstate {
        return;
    }

    /*
     * Both codec driver and controller can hold references to
     * display power. To avoid unnecessary power-up/down cycles,
     * controller doesn't immediately release its reference.
     *
     * If the codec driver powers down the link, release
     * the controller reference as well.
     */
    if (*codec).addr == HDA_IDISP_ADDR && !enable {
        snd_hdac_display_power(bus, HDA_CODEC_IDX_CONTROLLER, false);
    }

    /* WAKEEN needs to be set for disabled links */
    update_codec_wake_enable(bus, (*codec).addr, enable);
}

static bus_core_ops: hdac_bus_ops = hdac_bus_ops {
    command: Some(snd_hdac_bus_send_cmd),
    get_response: Some(snd_hdac_bus_get_response),
    link_power: Some(sof_hda_bus_link_power),
};
// #endif

/*
 * This can be used for both with/without hda link support.
 */
#[no_mangle]
pub unsafe extern "C" fn sof_hda_bus_init(sdev: *mut snd_sof_dev, dev: *mut device) {
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    // Original C condition:
    // #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_LINK)
    // #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_AUDIO_CODEC)
    {
        let chip: *const sof_intel_dsp_desc = get_chip_info((*sdev).pdata);

        snd_hdac_ext_bus_init(bus, dev, &bus_core_ops, sof_hda_ext_ops());

        if !chip.is_null() && (*chip).hw_ip_version >= SOF_INTEL_ACE_2_0 {
            (*bus).use_pio_for_commands = true;
        }
    }
    // #else
    // snd_hdac_ext_bus_init(bus, dev, NULL, NULL);
    // #endif
    // #else
    {
        memset(
            bus as *mut c_void,
            0,
            size_of::<hdac_bus>(),
        );
        (*bus).dev = dev;

        INIT_LIST_HEAD(ptr::addr_of_mut!((*bus).stream_list));

        (*bus).irq = -1;

        /*
         * There is only one HDA bus atm. keep the index as 0.
         * Need to fix when there are more than one HDA bus.
         */
        (*bus).idx = 0;

        spin_lock_init(ptr::addr_of_mut!((*bus).reg_lock));
    }
    // #endif /* CONFIG_SND_SOC_SOF_HDA_LINK */
}
// EXPORT_SYMBOL_NS(sof_hda_bus_init, "SND_SOC_SOF_INTEL_HDA_COMMON");

#[no_mangle]
pub unsafe extern "C" fn sof_hda_bus_exit(sdev: *mut snd_sof_dev) {
    // Original C condition:
    // #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_LINK)
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    snd_hdac_ext_bus_exit(bus);
    // #endif
}
// EXPORT_SYMBOL_NS(sof_hda_bus_exit, "SND_SOC_SOF_INTEL_HDA_COMMON");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
