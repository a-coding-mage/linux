// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

use core::ffi::{c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

extern "C" {
    static code_loader: c_void;

    static AVS_ADSP_HIPCCTL_DONE: u32;
    static AVS_ADSP_HIPCCTL_BUSY: u32;
    static SKL_ADSP_REG_HIPCT: u32;
    static SKL_ADSP_REG_HIPCTE: u32;
    static SKL_ADSP_HIPCT_BUSY: u32;
    static AVS_ADSP_REG_ADSPIS: u32;
    static AVS_ADSP_ADSPIS_CLDMA: u32;
    static AVS_ADSP_ADSPIS_IPC: u32;
    static UINT_MAX: u32;
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_int;
    static AVS_FW_REGS_WINDOW: u32;
    static AVS_FW_REGS_SIZE: usize;

    fn snd_hdac_adsp_updatel(adev: *mut avs_dev, offset: u32, mask: u32, value: u32);
    fn snd_hdac_adsp_readl(adev: *mut avs_dev, offset: u32) -> u32;
    fn complete(completion: *mut completion);
    fn avs_dsp_process_response(adev: *mut avs_dev, msg: u64);
    fn hda_cldma_interrupt(loader: *const c_void);
    fn fls_long(x: c_ulong) -> c_int;
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn avs_ipc_set_enable_logs(adev: *mut avs_dev, data: *mut u8, size: u32) -> c_int;
    fn avs_log_buffer_size(adev: *mut avs_dev) -> c_int;
    fn avs_logging_fw(adev: *mut avs_dev) -> bool;
    fn avs_sram_addr(adev: *mut avs_dev, window: u32) -> *mut c_void;
    fn readl(addr: *mut c_void) -> u32;
    fn avs_log_buffer_addr(adev: *mut avs_dev, core: u32) -> *mut c_void;
    fn avs_dump_fw_log_wakeup(adev: *mut avs_dev, buf: *mut c_void, size: u16);
    fn vzalloc(size: usize) -> *mut u8;
    fn memcpy_fromio(dst: *mut u8, src: *mut c_void, size: usize);
    fn dev_coredumpv(dev: *mut device, data: *mut u8, size: usize, flags: c_int);

    fn avs_dsp_core_power(adev: *mut avs_dev, core_mask: u32, power: bool) -> c_int;
    fn avs_dsp_core_reset(adev: *mut avs_dev, core_mask: u32, reset: bool) -> c_int;
    fn avs_dsp_core_stall(adev: *mut avs_dev, core_mask: u32, stall: bool) -> c_int;
    fn avs_dsp_interrupt_control(adev: *mut avs_dev, enable: bool);
    fn avs_cldma_load_basefw(adev: *mut avs_dev, fw: *const c_void) -> c_int;
    fn avs_cldma_load_library(adev: *mut avs_dev, lib: *const c_void, id: u32) -> c_int;
    fn avs_cldma_transfer_modules(adev: *mut avs_dev, modules: *const c_void, num_modules: u32) -> c_int;
}

#[repr(C)]
pub struct avs_dev {
    pub spec: *const avs_spec,
    pub ipc: *mut avs_ipc,
    pub hw_cfg: avs_hw_cfg,
    pub dev: *mut device,
}

#[repr(C)]
pub struct avs_spec {
    pub hipc: *const avs_hipc_spec,
}

#[repr(C)]
pub struct avs_hipc_spec {
    pub ctl_offset: u32,
    pub ack_offset: u32,
    pub rsp_offset: u32,
    pub ack_done_mask: u32,
    pub rsp_busy_mask: u32,
}

#[repr(C)]
pub struct avs_ipc {
    pub done_completion: completion,
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_hw_cfg {
    pub dsp_cores: u32,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub type irqreturn_t = c_int;
pub type avs_log_enable = c_int;

#[repr(C)]
pub struct avs_ipc_msg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_skl_log_state_info {
    pub core_mask: c_ulong,
    pub logs_core: [avs_skl_log_state_core; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_skl_log_state_core {
    pub enable: avs_log_enable,
    pub min_priority: u32,
}

#[repr(C)]
pub union avs_reply_msg {
    pub val: u64,
    pub primary: u32,
    pub ext: avs_reply_msg_ext,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_reply_msg_ext {
    pub val: u32,
}

#[repr(C)]
pub union avs_notify_msg {
    pub log: avs_notify_msg_log,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_notify_msg_log {
    pub core: u32,
}

#[repr(C)]
pub struct avs_dsp_ops {
    pub power: Option<unsafe extern "C" fn(*mut avs_dev, u32, bool) -> c_int>,
    pub reset: Option<unsafe extern "C" fn(*mut avs_dev, u32, bool) -> c_int>,
    pub stall: Option<unsafe extern "C" fn(*mut avs_dev, u32, bool) -> c_int>,
    pub dsp_interrupt: Option<unsafe extern "C" fn(*mut avs_dev) -> irqreturn_t>,
    pub int_control: Option<unsafe extern "C" fn(*mut avs_dev, bool)>,
    pub load_basefw: Option<unsafe extern "C" fn(*mut avs_dev, *const c_void) -> c_int>,
    pub load_lib: Option<unsafe extern "C" fn(*mut avs_dev, *const c_void, u32) -> c_int>,
    pub transfer_mods: Option<unsafe extern "C" fn(*mut avs_dev, *const c_void, u32) -> c_int>,
    pub log_buffer_offset: Option<unsafe extern "C" fn(*mut avs_dev, u32) -> c_int>,
    pub log_buffer_status: Option<unsafe extern "C" fn(*mut avs_dev, *mut avs_notify_msg) -> c_int>,
    pub coredump: Option<unsafe extern "C" fn(*mut avs_dev, *mut avs_notify_msg) -> c_int>,
    pub d0ix_toggle: Option<unsafe extern "C" fn(*mut avs_dev, *mut avs_ipc_msg, bool) -> bool>,
    pub set_d0ix: Option<unsafe extern "C" fn(*mut avs_dev, bool) -> c_int>,
    /* AVS_SET_ENABLE_LOGS_OP(skl) */
    pub enable_logs:
        Option<unsafe extern "C" fn(*mut avs_dev, avs_log_enable, u32, u32, c_ulong, *mut u32) -> c_int>,
}

const fn FW_REGS_DBG_LOG_WP(core: u32) -> usize {
    0x30 + 0x4 * core as usize
}

const fn AVS_IPC_RET(ret: c_int) -> c_int {
    ret
}

unsafe fn logs_core_ptr(info: *mut avs_skl_log_state_info, index: u32) -> *mut avs_skl_log_state_core {
    (info as *mut u8)
        .add(size_of::<avs_skl_log_state_info>() + index as usize * size_of::<avs_skl_log_state_core>())
        as *mut avs_skl_log_state_core
}

#[no_mangle]
pub unsafe extern "C" fn avs_skl_ipc_interrupt(adev: *mut avs_dev) {
    let spec = (*adev).spec;
    let mut hipc_ack: u32;
    let mut hipc_rsp: u32;

    snd_hdac_adsp_updatel(
        adev,
        (*(*spec).hipc).ctl_offset,
        AVS_ADSP_HIPCCTL_DONE | AVS_ADSP_HIPCCTL_BUSY,
        0,
    );

    hipc_ack = snd_hdac_adsp_readl(adev, (*(*spec).hipc).ack_offset);
    hipc_rsp = snd_hdac_adsp_readl(adev, (*(*spec).hipc).rsp_offset);

    /* DSP acked host's request. */
    if hipc_ack & (*(*spec).hipc).ack_done_mask != 0 {
        complete(&mut (*(*adev).ipc).done_completion);

        /* Tell DSP it has our attention. */
        snd_hdac_adsp_updatel(
            adev,
            (*(*spec).hipc).ack_offset,
            (*(*spec).hipc).ack_done_mask,
            (*(*spec).hipc).ack_done_mask,
        );
    }

    /* DSP sent new response to process */
    if hipc_rsp & (*(*spec).hipc).rsp_busy_mask != 0 {
        let mut msg = avs_reply_msg { val: 0 };

        msg.primary = snd_hdac_adsp_readl(adev, SKL_ADSP_REG_HIPCT);
        msg.ext.val = snd_hdac_adsp_readl(adev, SKL_ADSP_REG_HIPCTE);

        avs_dsp_process_response(adev, msg.val);

        /* Tell DSP we accepted its message. */
        snd_hdac_adsp_updatel(
            adev,
            SKL_ADSP_REG_HIPCT,
            SKL_ADSP_HIPCT_BUSY,
            SKL_ADSP_HIPCT_BUSY,
        );
    }

    snd_hdac_adsp_updatel(
        adev,
        (*(*spec).hipc).ctl_offset,
        AVS_ADSP_HIPCCTL_DONE | AVS_ADSP_HIPCCTL_BUSY,
        AVS_ADSP_HIPCCTL_DONE | AVS_ADSP_HIPCCTL_BUSY,
    );
}

unsafe extern "C" fn avs_skl_dsp_interrupt(adev: *mut avs_dev) -> irqreturn_t {
    let adspis = snd_hdac_adsp_readl(adev, AVS_ADSP_REG_ADSPIS);
    let mut ret = IRQ_NONE;

    if adspis == UINT_MAX {
        return ret;
    }

    if adspis & AVS_ADSP_ADSPIS_CLDMA != 0 {
        hda_cldma_interrupt(&code_loader);
        ret = IRQ_HANDLED;
    }

    if adspis & AVS_ADSP_ADSPIS_IPC != 0 {
        avs_skl_ipc_interrupt(adev);
        ret = IRQ_HANDLED;
    }

    ret
}

unsafe extern "C" fn avs_skl_enable_logs(
    adev: *mut avs_dev,
    enable: avs_log_enable,
    _aging_period: u32,
    _fifo_full_period: u32,
    resource_mask: c_ulong,
    mut priorities: *mut u32,
) -> c_int {
    let mut info: *mut avs_skl_log_state_info;
    let num_cores = (*adev).hw_cfg.dsp_cores;
    let size: u32;
    let mut ret: c_int;
    let mut i: u32;

    if fls_long(resource_mask) as u32 > num_cores {
        return -EINVAL;
    }
    size = (size_of::<avs_skl_log_state_info>()
        + num_cores as usize * size_of::<avs_skl_log_state_core>()) as u32;
    info = kzalloc(size as usize, GFP_KERNEL) as *mut avs_skl_log_state_info;
    if info.is_null() {
        return -ENOMEM;
    }

    (*info).core_mask = resource_mask;
    if enable != 0 {
        i = 0;
        while i < num_cores {
            if resource_mask & (1 as c_ulong).wrapping_shl(i) != 0 {
                (*logs_core_ptr(info, i)).enable = enable;
                (*logs_core_ptr(info, i)).min_priority = *priorities;
                priorities = priorities.add(1);
            }
            i += 1;
        }
    } else {
        i = 0;
        while i < num_cores {
            if resource_mask & (1 as c_ulong).wrapping_shl(i) != 0 {
                (*logs_core_ptr(info, i)).enable = enable;
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

#[no_mangle]
pub unsafe extern "C" fn avs_skl_log_buffer_offset(adev: *mut avs_dev, core: u32) -> c_int {
    (core as c_int) * avs_log_buffer_size(adev)
}

/* fw DbgLogWp registers */

unsafe extern "C" fn avs_skl_log_buffer_status(
    adev: *mut avs_dev,
    msg: *mut avs_notify_msg,
) -> c_int {
    let buf: *mut c_void;
    let size: u16;
    let write: u16;
    let offset: u16;

    if !avs_logging_fw(adev) {
        return 0;
    }

    size = (avs_log_buffer_size(adev) / 2) as u16;
    write = readl(
        (avs_sram_addr(adev, AVS_FW_REGS_WINDOW) as *mut u8)
            .add(FW_REGS_DBG_LOG_WP((*msg).log.core)) as *mut c_void,
    ) as u16;
    /* determine buffer half */
    offset = if write < size { size } else { 0 };

    /* Address is guaranteed to exist in SRAM2. */
    buf = (avs_log_buffer_addr(adev, (*msg).log.core) as *mut u8).add(offset as usize) as *mut c_void;
    avs_dump_fw_log_wakeup(adev, buf, size);

    0
}

unsafe extern "C" fn avs_skl_coredump(adev: *mut avs_dev, _msg: *mut avs_notify_msg) -> c_int {
    let dump: *mut u8;

    dump = vzalloc(AVS_FW_REGS_SIZE);
    if dump.is_null() {
        return -ENOMEM;
    }

    memcpy_fromio(dump, avs_sram_addr(adev, AVS_FW_REGS_WINDOW), AVS_FW_REGS_SIZE);
    dev_coredumpv((*adev).dev, dump, AVS_FW_REGS_SIZE, GFP_KERNEL);

    0
}

unsafe extern "C" fn avs_skl_d0ix_toggle(
    _adev: *mut avs_dev,
    _tx: *mut avs_ipc_msg,
    _wake: bool,
) -> bool {
    /* unsupported on cAVS 1.5 hw */
    false
}

unsafe extern "C" fn avs_skl_set_d0ix(_adev: *mut avs_dev, _enable: bool) -> c_int {
    /* unsupported on cAVS 1.5 hw */
    0
}

#[no_mangle]
pub static avs_skl_dsp_ops: avs_dsp_ops = avs_dsp_ops {
    power: Some(avs_dsp_core_power),
    reset: Some(avs_dsp_core_reset),
    stall: Some(avs_dsp_core_stall),
    dsp_interrupt: Some(avs_skl_dsp_interrupt),
    int_control: Some(avs_dsp_interrupt_control),
    load_basefw: Some(avs_cldma_load_basefw),
    load_lib: Some(avs_cldma_load_library),
    transfer_mods: Some(avs_cldma_transfer_modules),
    log_buffer_offset: Some(avs_skl_log_buffer_offset),
    log_buffer_status: Some(avs_skl_log_buffer_status),
    coredump: Some(avs_skl_coredump),
    d0ix_toggle: Some(avs_skl_d0ix_toggle),
    set_d0ix: Some(avs_skl_set_d0ix),
    enable_logs: Some(avs_skl_enable_logs),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
