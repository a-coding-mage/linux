// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

use core::ffi::{c_char, c_int, c_void};

pub type u8 = u8;
pub type u16 = u16;
pub type u32 = u32;

const AVS_ADSPCS_DELAY_US: u32 = 1000;
const EINVAL: c_int = 22;
const GFP_KERNEL: u32 = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ida {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_hw_cfg {
    pub dsp_cores: u32,
}

#[repr(C)]
pub struct avs_fw_cfg {
    pub max_ppl_count: u32,
}

#[repr(C)]
pub struct avs_dev {
    pub dev: *mut device,
    pub hw_cfg: avs_hw_cfg,
    pub fw_cfg: avs_fw_cfg,
    pub core_refs: *mut u32,
    pub ppl_ida: ida,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_module_type {
    pub load_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_module_entry {
    pub r#type: avs_module_type,
}

unsafe extern "C" {
    static AVS_ADSP_REG_ADSPCS: u32;
    static AVS_ADSPCS_INTERVAL_US: u32;
    static AVS_ADSPCS_TIMEOUT_US: u32;
    static AVS_MAIN_CORE_MASK: u32;
    static AVS_EIPC: c_int;
    static AVS_MODULE_LOAD_TYPE_LOADABLE: u32;
    static INVALID_PIPELINE_ID: u8;

    fn AVS_ADSPCS_SPA_MASK(core_mask: u32) -> u32;
    fn AVS_ADSPCS_CPA_MASK(core_mask: u32) -> u32;
    fn AVS_ADSPCS_CRST_MASK(core_mask: u32) -> u32;
    fn AVS_ADSPCS_CSTALL_MASK(core_mask: u32) -> u32;
    fn AVS_IPC_RET(ret: c_int) -> c_int;
    fn BIT_MASK(nr: u32) -> u32;

    fn snd_hdac_adsp_readl(adev: *mut avs_dev, reg: u32) -> u32;
    fn snd_hdac_adsp_updatel(adev: *mut avs_dev, reg: u32, mask: u32, value: u32);
    fn snd_hdac_adsp_readl_poll(
        adev: *mut avs_dev,
        addr: u32,
        reg: *mut u32,
        mask: u32,
        value: u32,
        interval_us: u32,
        timeout_us: u32,
    ) -> c_int;
    fn trace_avs_dsp_core_op(value: u32, core_mask: u32, op: *const c_char, flag: bool);
    fn usleep_range(min: u32, max: u32);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn str_on_off(v: bool) -> *const c_char;

    fn avs_ipc_set_dx(adev: *mut avs_dev, core_mask: u32, power: bool) -> c_int;
    fn avs_dsp_disable_d0ix(adev: *mut avs_dev) -> c_int;
    fn avs_dsp_enable_d0ix(adev: *mut avs_dev) -> c_int;
    fn avs_module_id_alloc(adev: *mut avs_dev, module_id: u16) -> c_int;
    fn avs_module_id_free(adev: *mut avs_dev, module_id: u16, instance_id: c_int);
    fn avs_get_module_id_entry(
        adev: *mut avs_dev,
        module_id: u16,
        mentry: *mut avs_module_entry,
    ) -> c_int;
    fn avs_module_entry_is_loaded(mentry: *const avs_module_entry) -> bool;
    fn avs_ipc_init_instance(
        adev: *mut avs_dev,
        module_id: u16,
        instance_id: c_int,
        ppl_instance_id: u8,
        core_id: u8,
        domain: u8,
        param: *mut c_void,
        param_size: u32,
    ) -> c_int;
    fn avs_ipc_delete_instance(adev: *mut avs_dev, module_id: u16, instance_id: u8) -> c_int;
    fn avs_is_module_ida_empty(adev: *mut avs_dev, module_id: u16) -> bool;
    fn ida_alloc_max(ida: *mut ida, max: u32, gfp: u32) -> c_int;
    fn ida_free(ida: *mut ida, id: c_int);
    fn avs_ipc_create_pipeline(
        adev: *mut avs_dev,
        req_size: u16,
        priority: u8,
        instance_id: c_int,
        lp: bool,
        attributes: u16,
    ) -> c_int;
    fn avs_ipc_delete_pipeline(adev: *mut avs_dev, instance_id: u8) -> c_int;
}

unsafe fn avs_dsp_op_power(adev: *mut avs_dev, core_mask: u32, power: bool) -> c_int {
    avs_dsp_core_power(adev, core_mask, power)
}

unsafe fn avs_dsp_op_reset(adev: *mut avs_dev, core_mask: u32, reset: bool) -> c_int {
    avs_dsp_core_reset(adev, core_mask, reset)
}

unsafe fn avs_dsp_op_stall(adev: *mut avs_dev, core_mask: u32, stall: bool) -> c_int {
    avs_dsp_core_stall(adev, core_mask, stall)
}

unsafe extern "C" {
    fn avs_dsp_op_transfer_mods(
        adev: *mut avs_dev,
        load: bool,
        mentry: *mut avs_module_entry,
        count: c_int,
    ) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_dsp_core_power(
    adev: *mut avs_dev,
    core_mask: u32,
    power: bool,
) -> c_int {
    let mut value: u32;
    let mut mask: u32;
    let mut reg: u32 = 0;
    let ret: c_int;

    value = unsafe { snd_hdac_adsp_readl(adev, AVS_ADSP_REG_ADSPCS) };
    unsafe { trace_avs_dsp_core_op(value, core_mask, c"power".as_ptr(), power) };

    mask = unsafe { AVS_ADSPCS_SPA_MASK(core_mask) };
    value = if power { mask } else { 0 };

    unsafe { snd_hdac_adsp_updatel(adev, AVS_ADSP_REG_ADSPCS, mask, value) };
    /* Delay the polling to avoid false positives. */
    unsafe { usleep_range(AVS_ADSPCS_DELAY_US, 2 * AVS_ADSPCS_DELAY_US) };

    mask = unsafe { AVS_ADSPCS_CPA_MASK(core_mask) };
    value = if power { mask } else { 0 };

    ret = unsafe {
        snd_hdac_adsp_readl_poll(
            adev,
            AVS_ADSP_REG_ADSPCS,
            &mut reg,
            mask,
            value,
            AVS_ADSPCS_INTERVAL_US,
            AVS_ADSPCS_TIMEOUT_US,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                (*adev).dev,
                c"core_mask %d power %s failed: %d\n".as_ptr(),
                core_mask,
                str_on_off(power),
                ret,
            )
        };
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_dsp_core_reset(
    adev: *mut avs_dev,
    core_mask: u32,
    reset: bool,
) -> c_int {
    let mut value: u32;
    let mask: u32;
    let mut reg: u32 = 0;
    let ret: c_int;

    value = unsafe { snd_hdac_adsp_readl(adev, AVS_ADSP_REG_ADSPCS) };
    unsafe { trace_avs_dsp_core_op(value, core_mask, c"reset".as_ptr(), reset) };

    mask = unsafe { AVS_ADSPCS_CRST_MASK(core_mask) };
    value = if reset { mask } else { 0 };

    unsafe { snd_hdac_adsp_updatel(adev, AVS_ADSP_REG_ADSPCS, mask, value) };

    ret = unsafe {
        snd_hdac_adsp_readl_poll(
            adev,
            AVS_ADSP_REG_ADSPCS,
            &mut reg,
            mask,
            value,
            AVS_ADSPCS_INTERVAL_US,
            AVS_ADSPCS_TIMEOUT_US,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                (*adev).dev,
                c"core_mask %d %s reset failed: %d\n".as_ptr(),
                core_mask,
                if reset { c"enter".as_ptr() } else { c"exit".as_ptr() },
                ret,
            )
        };
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_dsp_core_stall(
    adev: *mut avs_dev,
    core_mask: u32,
    stall: bool,
) -> c_int {
    let mut value: u32;
    let mask: u32;
    let mut reg: u32 = 0;
    let ret: c_int;

    value = unsafe { snd_hdac_adsp_readl(adev, AVS_ADSP_REG_ADSPCS) };
    unsafe { trace_avs_dsp_core_op(value, core_mask, c"stall".as_ptr(), stall) };

    mask = unsafe { AVS_ADSPCS_CSTALL_MASK(core_mask) };
    value = if stall { mask } else { 0 };

    unsafe { snd_hdac_adsp_updatel(adev, AVS_ADSP_REG_ADSPCS, mask, value) };

    ret = unsafe {
        snd_hdac_adsp_readl_poll(
            adev,
            AVS_ADSP_REG_ADSPCS,
            &mut reg,
            mask,
            value,
            AVS_ADSPCS_INTERVAL_US,
            AVS_ADSPCS_TIMEOUT_US,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                (*adev).dev,
                c"core_mask %d %sstall failed: %d\n".as_ptr(),
                core_mask,
                if stall { c"".as_ptr() } else { c"un".as_ptr() },
                ret,
            )
        };
        return ret;
    }

    /* Give HW time to propagate the change. */
    unsafe { usleep_range(AVS_ADSPCS_DELAY_US, 2 * AVS_ADSPCS_DELAY_US) };
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_dsp_core_enable(adev: *mut avs_dev, core_mask: u32) -> c_int {
    let mut ret: c_int;

    ret = unsafe { avs_dsp_op_power(adev, core_mask, true) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { avs_dsp_op_reset(adev, core_mask, false) };
    if ret != 0 {
        return ret;
    }

    unsafe { avs_dsp_op_stall(adev, core_mask, false) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_dsp_core_disable(adev: *mut avs_dev, core_mask: u32) -> c_int {
    /* No error checks to allow for complete DSP shutdown. */
    unsafe { avs_dsp_op_stall(adev, core_mask, true) };
    unsafe { avs_dsp_op_reset(adev, core_mask, true) };

    unsafe { avs_dsp_op_power(adev, core_mask, false) }
}

unsafe fn avs_dsp_enable(adev: *mut avs_dev, core_mask: u32) -> c_int {
    let mask: u32;
    let mut ret: c_int;

    ret = unsafe { avs_dsp_core_enable(adev, core_mask) };
    if ret < 0 {
        return ret;
    }

    mask = core_mask & !unsafe { AVS_MAIN_CORE_MASK };
    if mask == 0 {
        /*
         * without main core, fw is dead anyway
         * so setting D0 for it is futile.
         */
        return 0;
    }

    ret = unsafe { avs_ipc_set_dx(adev, mask, true) };
    unsafe { AVS_IPC_RET(ret) }
}

unsafe fn avs_dsp_disable(adev: *mut avs_dev, core_mask: u32) -> c_int {
    let mut ret: c_int;

    ret = unsafe { avs_ipc_set_dx(adev, core_mask, false) };
    if ret != 0 {
        return unsafe { AVS_IPC_RET(ret) };
    }

    unsafe { avs_dsp_core_disable(adev, core_mask) }
}

unsafe fn avs_dsp_get_core(adev: *mut avs_dev, core_id: u32) -> c_int {
    let mask: u32;
    let mut ret: c_int;

    mask = unsafe { BIT_MASK(core_id) };
    if mask == unsafe { AVS_MAIN_CORE_MASK } {
        /* nothing to do for main core */
        return 0;
    }
    if core_id >= unsafe { (*adev).hw_cfg.dsp_cores } {
        ret = -EINVAL;
        unsafe {
            dev_err(
                (*adev).dev,
                c"get core %d failed: %d\n".as_ptr(),
                core_id,
                ret,
            )
        };
        return ret;
    }

    unsafe {
        let refs = (*adev).core_refs.add(core_id as usize);
        *refs = (*refs).wrapping_add(1);
        if *refs == 1 {
            /*
             * No cores other than main-core can be running for DSP
             * to achieve d0ix. Conscious SET_D0IX IPC failure is permitted,
             * simply d0ix power state will no longer be attempted.
             */
            ret = avs_dsp_disable_d0ix(adev);
            if ret != 0 && ret != -AVS_EIPC {
                *refs = (*refs).wrapping_sub(1);
                dev_err((*adev).dev, c"get core %d failed: %d\n".as_ptr(), core_id, ret);
                return ret;
            }

            ret = avs_dsp_enable(adev, mask);
            if ret != 0 {
                avs_dsp_enable_d0ix(adev);
                *refs = (*refs).wrapping_sub(1);
                dev_err((*adev).dev, c"get core %d failed: %d\n".as_ptr(), core_id, ret);
                return ret;
            }
        }
    }

    0
}

unsafe fn avs_dsp_put_core(adev: *mut avs_dev, core_id: u32) -> c_int {
    let mask: u32;
    let mut ret: c_int;

    mask = unsafe { BIT_MASK(core_id) };
    if mask == unsafe { AVS_MAIN_CORE_MASK } {
        /* nothing to do for main core */
        return 0;
    }
    if core_id >= unsafe { (*adev).hw_cfg.dsp_cores } {
        ret = -EINVAL;
        unsafe {
            dev_err(
                (*adev).dev,
                c"put core %d failed: %d\n".as_ptr(),
                core_id,
                ret,
            )
        };
        return ret;
    }

    unsafe {
        let refs = (*adev).core_refs.add(core_id as usize);
        *refs = (*refs).wrapping_sub(1);
        if *refs == 0 {
            ret = avs_dsp_disable(adev, mask);
            if ret != 0 {
                dev_err((*adev).dev, c"put core %d failed: %d\n".as_ptr(), core_id, ret);
                return ret;
            }

            /* Match disable_d0ix in avs_dsp_get_core(). */
            avs_dsp_enable_d0ix(adev);
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_dsp_init_module(
    adev: *mut avs_dev,
    module_id: u16,
    ppl_instance_id: u8,
    core_id: u8,
    domain: u8,
    param: *mut c_void,
    param_size: u32,
    instance_id: *mut u8,
) -> c_int {
    let mut mentry: avs_module_entry = core::mem::zeroed();
    let mut was_loaded: bool = false;
    let mut ret: c_int;
    let id: c_int;

    id = unsafe { avs_module_id_alloc(adev, module_id) };
    if id < 0 {
        return id;
    }

    ret = unsafe { avs_get_module_id_entry(adev, module_id, &mut mentry) };
    if ret != 0 {
        unsafe { avs_module_id_free(adev, module_id, id) };
        return ret;
    }

    ret = unsafe { avs_dsp_get_core(adev, core_id as u32) };
    if ret != 0 {
        unsafe { avs_module_id_free(adev, module_id, id) };
        return ret;
    }

    /* Load code into memory if this is the first instance. */
    if id == 0 && !unsafe { avs_module_entry_is_loaded(&mentry) } {
        ret = unsafe { avs_dsp_op_transfer_mods(adev, true, &mut mentry, 1) };
        if ret != 0 {
            unsafe {
                dev_err((*adev).dev, c"load modules failed: %d\n".as_ptr(), ret);
                avs_module_id_free(adev, module_id, id);
            }
            return ret;
        }
        was_loaded = true;
    }

    ret = unsafe {
        avs_ipc_init_instance(
            adev,
            module_id,
            id,
            ppl_instance_id,
            core_id,
            domain,
            param,
            param_size,
        )
    };
    if ret != 0 {
        ret = unsafe { AVS_IPC_RET(ret) };
        if was_loaded {
            unsafe { avs_dsp_op_transfer_mods(adev, false, &mut mentry, 1) };
        }
        unsafe {
            avs_dsp_put_core(adev, core_id as u32);
            avs_module_id_free(adev, module_id, id);
        }
        return ret;
    }

    unsafe { *instance_id = id as u8 };
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_dsp_delete_module(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
    ppl_instance_id: u8,
    core_id: u8,
) {
    let mut mentry: avs_module_entry = core::mem::zeroed();
    let mut ret: c_int;

    /* Modules not owned by any pipeline need to be freed explicitly. */
    if ppl_instance_id == unsafe { INVALID_PIPELINE_ID } {
        unsafe { avs_ipc_delete_instance(adev, module_id, instance_id) };
    }

    unsafe { avs_module_id_free(adev, module_id, instance_id as c_int) };

    ret = unsafe { avs_get_module_id_entry(adev, module_id, &mut mentry) };
    /* Unload occupied memory if this was the last instance. */
    if ret == 0 && mentry.r#type.load_type == unsafe { AVS_MODULE_LOAD_TYPE_LOADABLE } {
        if unsafe { avs_is_module_ida_empty(adev, module_id) } {
            ret = unsafe { avs_dsp_op_transfer_mods(adev, false, &mut mentry, 1) };
            if ret != 0 {
                unsafe { dev_err((*adev).dev, c"unload modules failed: %d\n".as_ptr(), ret) };
            }
        }
    }

    unsafe { avs_dsp_put_core(adev, core_id as u32) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_dsp_create_pipeline(
    adev: *mut avs_dev,
    req_size: u16,
    priority: u8,
    lp: bool,
    attributes: u16,
    instance_id: *mut u8,
) -> c_int {
    let fw_cfg: *mut avs_fw_cfg = unsafe { &mut (*adev).fw_cfg };
    let ret: c_int;
    let id: c_int;

    id = unsafe { ida_alloc_max(&mut (*adev).ppl_ida, (*fw_cfg).max_ppl_count - 1, GFP_KERNEL) };
    if id < 0 {
        return id;
    }

    ret = unsafe { avs_ipc_create_pipeline(adev, req_size, priority, id, lp, attributes) };
    if ret != 0 {
        unsafe { ida_free(&mut (*adev).ppl_ida, id) };
        return unsafe { AVS_IPC_RET(ret) };
    }

    unsafe { *instance_id = id as u8 };
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_dsp_delete_pipeline(adev: *mut avs_dev, instance_id: u8) -> c_int {
    let mut ret: c_int;

    ret = unsafe { avs_ipc_delete_pipeline(adev, instance_id) };
    if ret != 0 {
        ret = unsafe { AVS_IPC_RET(ret) };
    }

    unsafe { ida_free(&mut (*adev).ppl_ida, instance_id as c_int) };
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
