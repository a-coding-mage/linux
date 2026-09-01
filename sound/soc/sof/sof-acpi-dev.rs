// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

// C includes translated as external dependencies:
// linux/acpi.h, linux/firmware.h, linux/module.h, linux/pm_runtime.h,
// sound/soc-acpi.h, sound/soc-acpi-intel-match.h, sound/sof.h,
// ../intel/common/soc-intel-quirks.h, ops.h, sof-acpi-dev.h,
// and platform specific device definitions from intel/shim.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub const GFP_KERNEL: c_uint = 0;
pub const ENOMEM: c_int = 12;
pub const ENODEV: c_int = 19;
pub const SND_SOF_SUSPEND_DELAY_MS: c_int = 0;

const fn BIT(nr: c_uint) -> c_int {
    (1_i32).wrapping_shl(nr) as c_int
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub runtime_suspend: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub runtime_idle: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub ops: *const c_void,
    pub ipc_default: c_int,
}

#[repr(C)]
pub struct ipc_file_profile_base {
    pub ipc_type: c_int,
    pub fw_path: *mut c_char,
    pub tplg_path: *mut c_char,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub desc: *const sof_dev_desc,
    pub dev: *mut device,
    pub ipc_file_profile_base: ipc_file_profile_base,
    pub sof_probe_complete: Option<unsafe extern "C" fn(dev: *mut device)>,
}

unsafe extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);

    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);

    fn snd_sof_suspend(dev: *mut device) -> c_int;
    fn snd_sof_resume(dev: *mut device) -> c_int;
    fn snd_sof_runtime_suspend(dev: *mut device) -> c_int;
    fn snd_sof_runtime_resume(dev: *mut device) -> c_int;
    fn snd_sof_runtime_idle(dev: *mut device) -> c_int;
    fn snd_sof_device_probe(dev: *mut device, pdata: *mut snd_sof_pdata) -> c_int;
    fn snd_sof_device_remove(dev: *mut device);
}

static mut fw_path: *mut c_char = ptr::null_mut();
// module_param(fw_path, charp, 0444);
// MODULE_PARM_DESC(fw_path, "deprecated - moved to snd-sof module.");

static mut tplg_path: *mut c_char = ptr::null_mut();
// module_param(tplg_path, charp, 0444);
// MODULE_PARM_DESC(tplg_path, "deprecated - moved to snd-sof module.");

static mut sof_acpi_debug: c_int = 0;
// module_param_named(sof_acpi_debug, sof_acpi_debug, int, 0444);
// MODULE_PARM_DESC(sof_acpi_debug, "SOF ACPI debug options (0x0 all off)");

const SOF_ACPI_DISABLE_PM_RUNTIME: c_int = BIT(0);

// EXPORT_NS_DEV_PM_OPS(sof_acpi_pm, SND_SOC_SOF_ACPI_DEV) = {
//     SYSTEM_SLEEP_PM_OPS(snd_sof_suspend, snd_sof_resume)
//     RUNTIME_PM_OPS(snd_sof_runtime_suspend, snd_sof_runtime_resume,
//                    snd_sof_runtime_idle)
// };
#[unsafe(no_mangle)]
pub static sof_acpi_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(snd_sof_suspend),
    resume: Some(snd_sof_resume),
    runtime_suspend: Some(snd_sof_runtime_suspend),
    runtime_resume: Some(snd_sof_runtime_resume),
    runtime_idle: Some(snd_sof_runtime_idle),
};

unsafe extern "C" fn sof_acpi_probe_complete(dev: *mut device) {
    unsafe {
        dev_dbg(dev, c"Completing SOF ACPI probe".as_ptr());

        if (sof_acpi_debug & SOF_ACPI_DISABLE_PM_RUNTIME) != 0 {
            return;
        }

        /* allow runtime_pm */
        pm_runtime_set_autosuspend_delay(dev, SND_SOF_SUSPEND_DELAY_MS);
        pm_runtime_use_autosuspend(dev);
        pm_runtime_enable(dev);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_acpi_probe(
    pdev: *mut platform_device,
    desc: *const sof_dev_desc,
) -> c_int {
    unsafe {
        let dev: *mut device = &mut (*pdev).dev;
        let sof_pdata: *mut snd_sof_pdata;

        dev_dbg(dev, c"ACPI DSP detected".as_ptr());

        sof_pdata = devm_kzalloc(
            dev,
            core::mem::size_of::<snd_sof_pdata>(),
            GFP_KERNEL,
        ) as *mut snd_sof_pdata;
        if sof_pdata.is_null() {
            return -ENOMEM;
        }

        if (*desc).ops.is_null() {
            dev_err(dev, c"error: no matching ACPI descriptor ops\n".as_ptr());
            return -ENODEV;
        }

        (*sof_pdata).desc = desc;
        (*sof_pdata).dev = &mut (*pdev).dev;

        (*sof_pdata).ipc_file_profile_base.ipc_type = (*desc).ipc_default;
        (*sof_pdata).ipc_file_profile_base.fw_path = fw_path;
        (*sof_pdata).ipc_file_profile_base.tplg_path = tplg_path;

        /* set callback to be called on successful device probe to enable runtime_pm */
        (*sof_pdata).sof_probe_complete = Some(sof_acpi_probe_complete);

        /* call sof helper for DSP hardware probe */
        snd_sof_device_probe(dev, sof_pdata)
    }
}
// EXPORT_SYMBOL_NS(sof_acpi_probe, "SND_SOC_SOF_ACPI_DEV");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_acpi_remove(pdev: *mut platform_device) {
    unsafe {
        let dev: *mut device = &mut (*pdev).dev;

        if (sof_acpi_debug & SOF_ACPI_DISABLE_PM_RUNTIME) == 0 {
            pm_runtime_disable(dev);
        }

        /* call sof helper for DSP hardware remove */
        snd_sof_device_remove(dev);
    }
}
// EXPORT_SYMBOL_NS(sof_acpi_remove, "SND_SOC_SOF_ACPI_DEV");

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF support for ACPI platforms");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
