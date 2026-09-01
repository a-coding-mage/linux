// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2024 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

// C dependencies:
// #include <linux/slab.h>
// #include <sound/hdaudio.h>
// #include <sound/hdaudio_ext.h>
// #include "avs.h"
// #include "debug.h"
// #include "messages.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::{offset_of, size_of};

type u8 = ::core::ffi::c_uchar;
type u32 = ::core::ffi::c_uint;
type c_int = ::core::ffi::c_int;
type c_ulong = ::core::ffi::c_ulong;
type bool_ = bool;

const ICL_VS_LTRP_GB_ICCMAX: u8 = 95;
const AVS_ICL_MEMWND2_SLOTS_COUNT: usize = 15;

const GFP_KERNEL: u32 = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const EBUSY: c_int = 16;
const SZ_4K: usize = 4096;
const AVS_DEBUG_WINDOW: u32 = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const HDAC_EXT_STREAM_TYPE_HOST: c_int = 0;
const VS_LTRP: u32 = 0;
const AZX_REG_VS_LTRP_GB_MASK: u8 = 0xff;
const AVS_PPL_STATE_RUNNING: u32 = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
}

#[repr(C)]
pub struct hdac_bus {
    pub reg_lock: spinlock_t,
}

#[repr(C)]
pub struct hdac_ext_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_dma_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_fw_cfg {
    pub max_libs_count: u32,
}

#[repr(C)]
pub struct avs_base {
    pub core: hdac_bus,
}

#[repr(C)]
pub struct avs_dev {
    pub fw_cfg: avs_fw_cfg,
    pub dev: *mut device,
    pub base: avs_base,
}

#[repr(C)]
pub struct avs_glb_set_ppl_state {
    pub state: u32,
}

#[repr(C)]
pub struct avs_glb {
    pub set_ppl_state: avs_glb_set_ppl_state,
}

#[repr(C)]
pub struct avs_ipc_msg {
    pub glb: avs_glb,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum avs_log_enable {
    AVS_LOG_DISABLE = 0,
    AVS_LOG_ENABLE = 1,
}

#[repr(C, packed)]
pub struct avs_icl_log_state_info {
    pub aging_timer_period: u32,
    pub fifo_full_timer_period: u32,
    pub enable: avs_log_enable,
    pub logs_priorities_mask: [u32; 0],
}

#[repr(C)]
pub struct avs_dsp_ops {
    pub power: Option<unsafe extern "C" fn(*mut avs_dev, u32, bool_) -> c_int>,
    pub reset: Option<unsafe extern "C" fn(*mut avs_dev, u32, bool_) -> c_int>,
    pub stall: Option<unsafe extern "C" fn(*mut avs_dev, u32, bool_) -> c_int>,
    pub dsp_interrupt: Option<unsafe extern "C" fn(*mut avs_dev)>,
    pub int_control: Option<unsafe extern "C" fn(*mut avs_dev, bool_)>,
    pub load_basefw: Option<unsafe extern "C" fn(*mut avs_dev, *mut firmware) -> c_int>,
    pub load_lib: Option<unsafe extern "C" fn(*mut avs_dev, *mut firmware, u32) -> c_int>,
    pub transfer_mods: Option<unsafe extern "C" fn(*mut avs_dev, *mut firmware) -> c_int>,
    pub log_buffer_offset: Option<unsafe extern "C" fn(*mut avs_dev, u32) -> c_int>,
    pub log_buffer_status: Option<unsafe extern "C" fn(*mut avs_dev, u32) -> c_int>,
    pub coredump: Option<unsafe extern "C" fn(*mut avs_dev, *mut c_void, usize)>,
    pub d0ix_toggle: Option<unsafe extern "C" fn(*mut avs_dev, *mut avs_ipc_msg, bool_) -> bool_>,
    pub set_d0ix: Option<unsafe extern "C" fn(*mut avs_dev, bool_) -> c_int>,
    #[cfg(CONFIG_DEBUG_FS)]
    pub enable_logs: Option<
        unsafe extern "C" fn(*mut avs_dev, avs_log_enable, u32, u32, c_ulong, *mut u32) -> c_int,
    >,
}

unsafe extern "C" {
    fn fls_long(x: c_ulong) -> c_int;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn avs_ipc_set_enable_logs(adev: *mut avs_dev, data: *mut u8, size: u32) -> c_int;
    fn avs_ipc_set_d0ix(adev: *mut avs_dev, enable: bool_, wake: bool_) -> c_int;
    fn AVS_IPC_RET(ret: c_int) -> c_int;
    fn memcpy_fromio(to: *mut c_void, from: *const c_void, count: usize);
    fn avs_sram_addr(adev: *mut avs_dev, window: u32) -> *const c_void;
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn snd_hdac_ext_stream_assign(
        bus: *mut hdac_bus,
        substream: *mut snd_pcm_substream,
        stream_type: c_int,
    ) -> *mut hdac_ext_stream;
    fn snd_hdac_chip_readb(bus: *mut hdac_bus, reg: u32) -> u8;
    fn snd_hdac_stream_format(channels: c_int, bits: c_int, rate: c_int) -> u32;
    fn snd_hdac_dsp_prepare(
        stream: *mut hdac_stream,
        format: u32,
        size: usize,
        dmab: *mut snd_dma_buffer,
    ) -> c_int;
    fn snd_hdac_chip_updateb(bus: *mut hdac_bus, reg: u32, mask: u8, val: u8);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn hdac_stream(stream: *mut hdac_ext_stream) -> *mut hdac_stream;
    fn snd_hdac_stream_start(stream: *mut hdac_stream);
    fn snd_hdac_stream_stop(stream: *mut hdac_stream);
    fn avs_hda_load_basefw(adev: *mut avs_dev, fw: *mut firmware) -> c_int;
    fn snd_hdac_dsp_cleanup(stream: *mut hdac_stream, dmab: *mut snd_dma_buffer);
    fn snd_hdac_ext_stream_release(stream: *mut hdac_ext_stream, stream_type: c_int);

    fn avs_dsp_core_power(adev: *mut avs_dev, core: u32, enable: bool_) -> c_int;
    fn avs_dsp_core_reset(adev: *mut avs_dev, core: u32, reset: bool_) -> c_int;
    fn avs_dsp_core_stall(adev: *mut avs_dev, core: u32, stall: bool_) -> c_int;
    fn avs_cnl_dsp_interrupt(adev: *mut avs_dev);
    fn avs_dsp_interrupt_control(adev: *mut avs_dev, enable: bool_);
    fn avs_hda_load_library(adev: *mut avs_dev, fw: *mut firmware, id: u32) -> c_int;
    fn avs_hda_transfer_modules(adev: *mut avs_dev, fw: *mut firmware) -> c_int;
    fn avs_apl_log_buffer_status(adev: *mut avs_dev, core: u32) -> c_int;
    fn avs_apl_coredump(adev: *mut avs_dev, data: *mut c_void, size: usize);
}

#[cfg(CONFIG_DEBUG_FS)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_icl_enable_logs(
    adev: *mut avs_dev,
    enable: avs_log_enable,
    aging_period: u32,
    fifo_full_period: u32,
    resource_mask: c_ulong,
    mut priorities: *mut u32,
) -> c_int {
    let mut info: *mut avs_icl_log_state_info;
    let size: u32;
    let num_libs: u32 = (*adev).fw_cfg.max_libs_count;
    let mut i: u32;
    let ret: c_int;

    if fls_long(resource_mask) as u32 > num_libs {
        return -EINVAL;
    }

    size = (size_of::<avs_icl_log_state_info>() + size_of::<u32>() * num_libs as usize) as u32;
    info = kzalloc(size as usize, GFP_KERNEL) as *mut avs_icl_log_state_info;
    if info.is_null() {
        return -ENOMEM;
    }

    (*info).aging_timer_period = aging_period;
    (*info).fifo_full_timer_period = fifo_full_period;
    (*info).enable = enable;
    if enable as u32 != 0 {
        i = 0;
        while i < num_libs {
            if ((resource_mask >> i) & 1) != 0 {
                let logs = (*info).logs_priorities_mask.as_mut_ptr();
                *logs.add(i as usize) = *priorities;
                priorities = priorities.add(1);
            }
            i += 1;
        }
    }

    ret = avs_ipc_set_enable_logs(adev, info as *mut u8, size);
    kfree(info as *mut c_void);
    if ret != 0 {
        return AVS_IPC_RET(ret);
    }

    0
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct avs_icl_memwnd2_slot_type {
    pub val: u32,
}

impl avs_icl_memwnd2_slot_type {
    pub unsafe fn set_resource_id(&mut self, resource_id: u32) {
        self.val = (self.val & !0xff) | (resource_id & 0xff);
    }

    pub unsafe fn resource_id(&self) -> u32 {
        self.val & 0xff
    }

    pub unsafe fn type_(&self) -> u32 {
        self.val >> 8
    }
}

const _: [(); 4] = [(); size_of::<avs_icl_memwnd2_slot_type>()];

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct avs_icl_memwnd2_desc {
    pub resource_id: u32,
    pub slot_id: avs_icl_memwnd2_slot_type,
    pub vma: u32,
}

const _: [(); 12] = [(); size_of::<avs_icl_memwnd2_desc>()];

#[repr(C)]
pub union avs_icl_memwnd2_desc_area {
    pub slot_desc: [avs_icl_memwnd2_desc; AVS_ICL_MEMWND2_SLOTS_COUNT],
    pub rsvd: [u8; SZ_4K],
}

#[repr(C, packed)]
pub struct avs_icl_memwnd2 {
    pub desc_area: avs_icl_memwnd2_desc_area,
    pub slot_array: [[u8; SZ_4K]; AVS_ICL_MEMWND2_SLOTS_COUNT],
}

const _: [(); 65536] = [(); size_of::<avs_icl_memwnd2>()];

const AVS_ICL_SLOT_UNUSED: avs_icl_memwnd2_slot_type = avs_icl_memwnd2_slot_type { val: 0x00000000 };
const AVS_ICL_SLOT_CRITICAL_LOG: avs_icl_memwnd2_slot_type =
    avs_icl_memwnd2_slot_type { val: 0x54524300 };
const AVS_ICL_SLOT_DEBUG_LOG: avs_icl_memwnd2_slot_type =
    avs_icl_memwnd2_slot_type { val: 0x474f4c00 };
const AVS_ICL_SLOT_GDB_STUB: avs_icl_memwnd2_slot_type =
    avs_icl_memwnd2_slot_type { val: 0x42444700 };
const AVS_ICL_SLOT_BROKEN: avs_icl_memwnd2_slot_type =
    avs_icl_memwnd2_slot_type { val: 0x44414544 };

unsafe extern "C" fn avs_icl_slot_offset(
    adev: *mut avs_dev,
    slot_type: avs_icl_memwnd2_slot_type,
) -> c_int {
    let mut desc: [avs_icl_memwnd2_desc; AVS_ICL_MEMWND2_SLOTS_COUNT] =
        [avs_icl_memwnd2_desc {
            resource_id: 0,
            slot_id: avs_icl_memwnd2_slot_type { val: 0 },
            vma: 0,
        }; AVS_ICL_MEMWND2_SLOTS_COUNT];
    let mut i: c_int;

    memcpy_fromio(
        desc.as_mut_ptr() as *mut c_void,
        avs_sram_addr(adev, AVS_DEBUG_WINDOW),
        size_of::<[avs_icl_memwnd2_desc; AVS_ICL_MEMWND2_SLOTS_COUNT]>(),
    );

    i = 0;
    while i < AVS_ICL_MEMWND2_SLOTS_COUNT as c_int {
        if desc[i as usize].slot_id.val == slot_type.val {
            return (offset_of!(avs_icl_memwnd2, slot_array) + i as usize * SZ_4K) as c_int;
        }
        i += 1;
    }

    -ENXIO
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_icl_log_buffer_offset(adev: *mut avs_dev, core: u32) -> c_int {
    let mut slot_type: avs_icl_memwnd2_slot_type = AVS_ICL_SLOT_DEBUG_LOG;
    let ret: c_int;

    slot_type.set_resource_id(core);
    ret = avs_icl_slot_offset(adev, slot_type);
    if ret < 0 {
        dev_dbg(
            (*adev).dev,
            c"No slot offset found for: %x\n".as_ptr() as *const u8,
            slot_type.val,
        );
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_icl_d0ix_toggle(
    _adev: *mut avs_dev,
    tx: *mut avs_ipc_msg,
    _wake: bool_,
) -> bool_ {
    /* Full-power when starting DMA engines. */
    if (*tx).glb.set_ppl_state.state == AVS_PPL_STATE_RUNNING {
        return true;
    }

    /* Payload-less IPCs do not take part in d0ix toggling. */
    (*tx).size != 0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_icl_set_d0ix(adev: *mut avs_dev, enable: bool_) -> c_int {
    let ret: c_int;

    ret = avs_ipc_set_d0ix(adev, enable, false);
    AVS_IPC_RET(ret)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_icl_load_basefw(adev: *mut avs_dev, fw: *mut firmware) -> c_int {
    let bus: *mut hdac_bus = &mut (*adev).base.core;
    let host_stream: *mut hdac_ext_stream;
    let mut substream: snd_pcm_substream = core::mem::zeroed();
    let mut dmab: snd_dma_buffer = core::mem::zeroed();
    let sd_fmt: u32;
    let ltrp_gb: u8;
    let mut ret: c_int;

    /*
     * ICCMAX:
     *
     * For ICL+ platforms, as per HW recommendation LTRP_GB is set to 95us
     * during FW load. Its original value shall be restored once load completes.
     *
     * To avoid DMI/OPIO L1 entry during the load procedure, additional CAPTURE
     * stream is allocated and set to run.
     */

    memset(
        &mut substream as *mut snd_pcm_substream as *mut c_void,
        0,
        size_of::<snd_pcm_substream>(),
    );
    substream.stream = SNDRV_PCM_STREAM_CAPTURE;

    host_stream = snd_hdac_ext_stream_assign(bus, &mut substream, HDAC_EXT_STREAM_TYPE_HOST);
    if host_stream.is_null() {
        return -EBUSY;
    }

    ltrp_gb = snd_hdac_chip_readb(bus, VS_LTRP) & AZX_REG_VS_LTRP_GB_MASK;
    /* Carries no real data, use default format. */
    sd_fmt = snd_hdac_stream_format(1, 32, 48000);

    ret = snd_hdac_dsp_prepare(hdac_stream(host_stream), sd_fmt, (*fw).size, &mut dmab);
    if ret < 0 {
        snd_hdac_ext_stream_release(host_stream, HDAC_EXT_STREAM_TYPE_HOST);
        snd_hdac_chip_updateb(bus, VS_LTRP, AZX_REG_VS_LTRP_GB_MASK, ltrp_gb);
        return ret;
    }

    snd_hdac_chip_updateb(
        bus,
        VS_LTRP,
        AZX_REG_VS_LTRP_GB_MASK,
        ICL_VS_LTRP_GB_ICCMAX,
    );

    spin_lock(&mut (*bus).reg_lock);
    snd_hdac_stream_start(hdac_stream(host_stream));
    spin_unlock(&mut (*bus).reg_lock);

    ret = avs_hda_load_basefw(adev, fw);

    spin_lock(&mut (*bus).reg_lock);
    snd_hdac_stream_stop(hdac_stream(host_stream));
    spin_unlock(&mut (*bus).reg_lock);

    snd_hdac_dsp_cleanup(hdac_stream(host_stream), &mut dmab);

    snd_hdac_ext_stream_release(host_stream, HDAC_EXT_STREAM_TYPE_HOST);
    snd_hdac_chip_updateb(bus, VS_LTRP, AZX_REG_VS_LTRP_GB_MASK, ltrp_gb);

    ret
}

#[unsafe(no_mangle)]
pub static avs_icl_dsp_ops: avs_dsp_ops = avs_dsp_ops {
    power: Some(avs_dsp_core_power),
    reset: Some(avs_dsp_core_reset),
    stall: Some(avs_dsp_core_stall),
    dsp_interrupt: Some(avs_cnl_dsp_interrupt),
    int_control: Some(avs_dsp_interrupt_control),
    load_basefw: Some(avs_icl_load_basefw),
    load_lib: Some(avs_hda_load_library),
    transfer_mods: Some(avs_hda_transfer_modules),
    log_buffer_offset: Some(avs_icl_log_buffer_offset),
    log_buffer_status: Some(avs_apl_log_buffer_status),
    coredump: Some(avs_apl_coredump),
    d0ix_toggle: Some(avs_icl_d0ix_toggle),
    set_d0ix: Some(avs_icl_set_d0ix),
    // AVS_SET_ENABLE_LOGS_OP(icl)
    #[cfg(CONFIG_DEBUG_FS)]
    enable_logs: Some(avs_icl_enable_logs),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
