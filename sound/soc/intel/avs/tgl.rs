// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2024 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct avs_base {
    pub pci: *mut pci_dev,
}

#[repr(C)]
pub struct avs_dev {
    pub base: avs_base,
}

#[repr(C)]
pub struct pci_dev {
    pub device: u32,
    pub subsystem_vendor: u32,
    pub subsystem_device: u32,
    pub revision: u32,
}

#[repr(C)]
pub struct cpuinfo_x86 {
    pub cpuid_level: c_int,
}

#[repr(C)]
pub struct avs_bus_hwid {
    pub device: u32,
    pub subsystem: u32,
    pub revision: u32,
}

pub type AvsDspCorePowerFn =
    unsafe extern "C" fn(adev: *mut avs_dev, core_mask: u32, power: bool) -> c_int;
pub type AvsDspCoreResetFn =
    unsafe extern "C" fn(adev: *mut avs_dev, core_mask: u32, reset: bool) -> c_int;
pub type AvsDspCoreStallFn =
    unsafe extern "C" fn(adev: *mut avs_dev, core_mask: u32, stall: bool) -> c_int;

#[repr(C)]
pub struct avs_dsp_ops {
    pub power: Option<AvsDspCorePowerFn>,
    pub reset: Option<AvsDspCoreResetFn>,
    pub stall: Option<AvsDspCoreStallFn>,
    pub dsp_interrupt: *const c_void,
    pub int_control: *const c_void,
    pub load_basefw: *const c_void,
    pub load_lib: *const c_void,
    pub transfer_mods: *const c_void,
    pub config_basefw: Option<unsafe extern "C" fn(adev: *mut avs_dev) -> c_int>,
    pub log_buffer_offset: *const c_void,
    pub log_buffer_status: *const c_void,
    pub coredump: *const c_void,
    pub d0ix_toggle: *const c_void,
    pub set_d0ix: *const c_void,
    /* AVS_SET_ENABLE_LOGS_OP(icl) */
}

unsafe extern "C" {
    static AVS_MAIN_CORE_MASK: u32;
    static CPUID_LEAF_TSC: c_int;
    static boot_cpu_data: cpuinfo_x86;
    static AVS_FW_CFG_XTAL_FREQ_HZ: u32;
    static AVS_FW_CFG_BUS_HARDWARE_ID: u32;

    fn avs_dsp_core_power(adev: *mut avs_dev, core_mask: u32, power: bool) -> c_int;
    fn avs_dsp_core_reset(adev: *mut avs_dev, core_mask: u32, reset: bool) -> c_int;
    fn avs_dsp_core_stall(adev: *mut avs_dev, core_mask: u32, stall: bool) -> c_int;
    fn cpuid_ecx(op: c_uint) -> c_uint;
    fn avs_ipc_set_fw_config(
        adev: *mut avs_dev,
        instance_id: u32,
        param_id: u32,
        data_size: usize,
        data: *const c_void,
    ) -> c_int;
    fn AVS_IPC_RET(ret: c_int) -> c_int;

    static avs_cnl_dsp_interrupt: c_void;
    static avs_dsp_interrupt_control: c_void;
    static avs_icl_load_basefw: c_void;
    static avs_hda_load_library: c_void;
    static avs_hda_transfer_modules: c_void;
    static avs_icl_log_buffer_offset: c_void;
    static avs_apl_log_buffer_status: c_void;
    static avs_apl_coredump: c_void;
    static avs_icl_d0ix_toggle: c_void;
    static avs_icl_set_d0ix: c_void;
}

unsafe extern "C" fn avs_tgl_dsp_core_power(
    adev: *mut avs_dev,
    mut core_mask: u32,
    power: bool,
) -> c_int {
    core_mask &= AVS_MAIN_CORE_MASK;

    if core_mask == 0 {
        return 0;
    }
    avs_dsp_core_power(adev, core_mask, power)
}

unsafe extern "C" fn avs_tgl_dsp_core_reset(
    adev: *mut avs_dev,
    mut core_mask: u32,
    reset: bool,
) -> c_int {
    core_mask &= AVS_MAIN_CORE_MASK;

    if core_mask == 0 {
        return 0;
    }
    avs_dsp_core_reset(adev, core_mask, reset)
}

unsafe extern "C" fn avs_tgl_dsp_core_stall(
    adev: *mut avs_dev,
    mut core_mask: u32,
    stall: bool,
) -> c_int {
    core_mask &= AVS_MAIN_CORE_MASK;

    if core_mask == 0 {
        return 0;
    }
    avs_dsp_core_stall(adev, core_mask, stall)
}

/*
 * Succeed if CPUID(0x15) is not available, or if the nominal core crystal clock
 * frequency cannot be enumerated from it.  There is nothing to do in both cases.
 */
unsafe extern "C" fn avs_tgl_set_xtal_freq(adev: *mut avs_dev) -> c_int {
    let freq: c_uint;
    let ret: c_int;

    if boot_cpu_data.cpuid_level < CPUID_LEAF_TSC {
        return 0;
    }

    freq = cpuid_ecx(CPUID_LEAF_TSC as c_uint);
    if freq != 0 {
        ret = avs_ipc_set_fw_config(
            adev,
            1,
            AVS_FW_CFG_XTAL_FREQ_HZ,
            core::mem::size_of_val(&freq),
            (&freq as *const c_uint).cast::<c_void>(),
        );
        if ret != 0 {
            return AVS_IPC_RET(ret);
        }
    }

    0
}

unsafe extern "C" fn avs_tgl_config_basefw(adev: *mut avs_dev) -> c_int {
    let pci: *mut pci_dev = (*adev).base.pci;
    let mut hwid: avs_bus_hwid = core::mem::zeroed();
    let mut ret: c_int;

    ret = avs_tgl_set_xtal_freq(adev);
    if ret != 0 {
        return ret;
    }

    hwid.device = (*pci).device;
    hwid.subsystem = (*pci).subsystem_vendor | ((*pci).subsystem_device << 16);
    hwid.revision = (*pci).revision;

    ret = avs_ipc_set_fw_config(
        adev,
        1,
        AVS_FW_CFG_BUS_HARDWARE_ID,
        core::mem::size_of_val(&hwid),
        (&hwid as *const avs_bus_hwid).cast::<c_void>(),
    );
    if ret != 0 {
        return AVS_IPC_RET(ret);
    }

    0
}

#[unsafe(no_mangle)]
pub static avs_tgl_dsp_ops: avs_dsp_ops = avs_dsp_ops {
    power: Some(avs_tgl_dsp_core_power),
    reset: Some(avs_tgl_dsp_core_reset),
    stall: Some(avs_tgl_dsp_core_stall),
    dsp_interrupt: unsafe { (&avs_cnl_dsp_interrupt as *const c_void).cast::<c_void>() },
    int_control: unsafe { (&avs_dsp_interrupt_control as *const c_void).cast::<c_void>() },
    load_basefw: unsafe { (&avs_icl_load_basefw as *const c_void).cast::<c_void>() },
    load_lib: unsafe { (&avs_hda_load_library as *const c_void).cast::<c_void>() },
    transfer_mods: unsafe { (&avs_hda_transfer_modules as *const c_void).cast::<c_void>() },
    config_basefw: Some(avs_tgl_config_basefw),
    log_buffer_offset: unsafe { (&avs_icl_log_buffer_offset as *const c_void).cast::<c_void>() },
    log_buffer_status: unsafe { (&avs_apl_log_buffer_status as *const c_void).cast::<c_void>() },
    coredump: unsafe { (&avs_apl_coredump as *const c_void).cast::<c_void>() },
    d0ix_toggle: unsafe { (&avs_icl_d0ix_toggle as *const c_void).cast::<c_void>() },
    set_d0ix: unsafe { (&avs_icl_set_d0ix as *const c_void).cast::<c_void>() },
    /* AVS_SET_ENABLE_LOGS_OP(icl) */
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
