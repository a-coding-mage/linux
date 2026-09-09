/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 *
 * Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

use core::ffi::c_void;

#[repr(C)]
pub struct snd_sof_dsp_ops;
#[repr(C)]
pub struct snd_sof_dev;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct platform_device;
#[repr(C)]
pub struct snd_soc_acpi_mach;
#[repr(C)]
pub struct snd_sof_of_mach;
#[repr(C)]
pub struct snd_soc_pcm_runtime;

/**
 * enum sof_fw_state - DSP firmware state definitions
 * @SOF_FW_BOOT_NOT_STARTED: firmware boot is not yet started
 * @SOF_DSPLESS_MODE: DSP is not used
 * @SOF_FW_BOOT_PREPARE: preparing for boot (firmware loading for exaqmple)
 * @SOF_FW_BOOT_IN_PROGRESS: firmware boot is in progress
 * @SOF_FW_BOOT_FAILED: firmware boot failed
 * @SOF_FW_BOOT_READY_FAILED: firmware booted but fw_ready op failed
 * @SOF_FW_BOOT_READY_OK: firmware booted and fw_ready op passed
 * @SOF_FW_BOOT_COMPLETE: firmware is booted up and functional
 * @SOF_FW_CRASHED: firmware crashed after successful boot
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum sof_fw_state {
    SOF_FW_BOOT_NOT_STARTED = 0,
    SOF_DSPLESS_MODE,
    SOF_FW_BOOT_PREPARE,
    SOF_FW_BOOT_IN_PROGRESS,
    SOF_FW_BOOT_FAILED,
    SOF_FW_BOOT_READY_FAILED,
    SOF_FW_BOOT_READY_OK,
    SOF_FW_BOOT_COMPLETE,
    SOF_FW_CRASHED,
}

/* DSP power states */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum sof_dsp_power_states {
    SOF_DSP_PM_D0,
    SOF_DSP_PM_D1,
    SOF_DSP_PM_D2,
    SOF_DSP_PM_D3,
}

/* Definitions for multiple IPCs */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum sof_ipc_type {
    SOF_IPC_TYPE_3,
    SOF_IPC_TYPE_4,
    SOF_IPC_TYPE_COUNT,
}

#[repr(C)]
pub struct sof_loadable_file_profile {
    pub ipc_type: sof_ipc_type,
    pub fw_path: *const i8,
    pub fw_path_postfix: *const i8,
    pub fw_name: *const i8,
    pub fw_lib_path: *const i8,
    pub fw_lib_path_postfix: *const i8,
    pub tplg_path: *const i8,
    pub tplg_name: *const i8,
}

/* SOF Platform data. */
#[repr(C)]
pub struct snd_sof_pdata {
    pub name: *const i8,
    pub platform: *const i8,
    pub subsystem_vendor: u16,
    pub subsystem_device: u16,
    pub subsystem_id_set: bool,
    pub dev: *mut device,
    pub sof_probe_complete: Option<unsafe extern "C" fn(dev: *mut device)>,
    pub desc: *const sof_dev_desc,
    pub ipc_file_profile_base: sof_loadable_file_profile,
    pub fw_filename_prefix: *const i8,
    pub fw_filename: *const i8,
    pub tplg_filename_prefix: *const i8,
    pub tplg_filename: *const i8,
    pub disable_function_topology: bool,
    pub fw_lib_prefix: *const i8,
    pub pdev_mach: *mut platform_device,
    pub machine: *const snd_soc_acpi_mach,
    pub of_machine: *const snd_sof_of_mach,
    pub hw_pdata: *mut c_void,
    pub ipc_type: sof_ipc_type,
}

/* Descriptor used for setting up SOF platform data. */
#[repr(C)]
pub struct sof_dev_desc {
    pub machines: *mut snd_soc_acpi_mach,
    pub of_machines: *mut snd_sof_of_mach,
    pub alt_machines: *mut snd_soc_acpi_mach,
    pub use_acpi_target_states: bool,
    pub resindex_lpe_base: i32,
    pub resindex_pcicfg_base: i32,
    pub resindex_imr_base: i32,
    pub irqindex_host_ipc: i32,
    pub ipc_timeout: i32,
    pub boot_timeout: i32,
    pub chip_info: *const c_void,
    pub nocodec_tplg_filename: *const i8,
    pub ipc_supported_mask: u32,
    pub ipc_default: sof_ipc_type,
    pub dspless_mode_supported: bool,
    pub on_demand_dsp_boot: bool,
    pub default_fw_path: [*const i8; SOF_IPC_TYPE_COUNT as usize],
    pub default_lib_path: [*const i8; SOF_IPC_TYPE_COUNT as usize],
    pub default_tplg_path: [*const i8; SOF_IPC_TYPE_COUNT as usize],
    pub default_fw_filename: [*const i8; SOF_IPC_TYPE_COUNT as usize],
    pub ops: *const snd_sof_dsp_ops,
    pub ops_init: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> i32>,
    pub ops_free: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev)>,
}

unsafe extern "C" {
    pub fn sof_dai_get_mclk(rtd: *mut snd_soc_pcm_runtime) -> i32;
    pub fn sof_dai_get_bclk(rtd: *mut snd_soc_pcm_runtime) -> i32;
    pub fn sof_dai_get_tdm_slots(rtd: *mut snd_soc_pcm_runtime) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
