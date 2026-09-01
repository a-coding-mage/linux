// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright 2019 NXP
//
// Author: Daniel Baluta <daniel.baluta@nxp.com>
//

// C dependencies: <linux/firmware.h>, <linux/module.h>,
// <linux/moduleparam.h>, <linux/pm_runtime.h>, <sound/sof.h>,
// "sof-of-dev.h", "ops.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

extern "C" {
    fn snd_sof_prepare(dev: *mut device) -> c_int;
    fn snd_sof_complete(dev: *mut device);
    fn snd_sof_suspend(dev: *mut device) -> c_int;
    fn snd_sof_resume(dev: *mut device) -> c_int;
    fn snd_sof_runtime_suspend(dev: *mut device) -> c_int;
    fn snd_sof_runtime_resume(dev: *mut device) -> c_int;

    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);

    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut c_void;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);

    fn snd_sof_device_probe(dev: *mut device, pdata: *mut snd_sof_pdata) -> c_int;
    fn snd_sof_device_remove(dev: *mut device);
    fn snd_sof_device_shutdown(dev: *mut device);
}

type gfp_t = u32;

const GFP_KERNEL: gfp_t = 0;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

extern "C" {
    static SND_SOF_SUSPEND_DELAY_MS: c_int;
}

static mut fw_path: *mut c_char = ptr::null_mut();
static mut fw_filename: *mut c_char = ptr::null_mut();
static mut tplg_path: *mut c_char = ptr::null_mut();
static mut tplg_filename: *mut c_char = ptr::null_mut();

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub ops: *const c_void,
    pub ipc_default: c_int,
}

#[repr(C)]
pub struct ipc_file_profile {
    pub ipc_type: c_int,
    pub fw_path: *mut c_char,
    pub tplg_path: *mut c_char,
    pub fw_name: *mut c_char,
    pub tplg_name: *mut c_char,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub desc: *const sof_dev_desc,
    pub dev: *mut device,
    pub ipc_file_profile_base: ipc_file_profile,
    pub sof_probe_complete: Option<unsafe extern "C" fn(dev: *mut device)>,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub prepare: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub complete: Option<unsafe extern "C" fn(dev: *mut device)>,
    pub suspend: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub runtime_suspend: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub runtime_idle: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
}

// module_param(...), MODULE_PARM_DESC(...), EXPORT_SYMBOL(...),
// MODULE_LICENSE(...), and MODULE_DESCRIPTION(...) are Linux module metadata
// declarations in C.

// EXPORT_DEV_PM_OPS(sof_of_pm) = { ... };
#[no_mangle]
pub static sof_of_pm: dev_pm_ops = dev_pm_ops {
    prepare: Some(snd_sof_prepare),
    complete: Some(snd_sof_complete),
    // SYSTEM_SLEEP_PM_OPS(snd_sof_suspend, snd_sof_resume)
    suspend: Some(snd_sof_suspend),
    resume: Some(snd_sof_resume),
    // RUNTIME_PM_OPS(snd_sof_runtime_suspend, snd_sof_runtime_resume, NULL)
    runtime_suspend: Some(snd_sof_runtime_suspend),
    runtime_resume: Some(snd_sof_runtime_resume),
    runtime_idle: None,
};

unsafe extern "C" fn sof_of_probe_complete(dev: *mut device) {
    /* allow runtime_pm */
    pm_runtime_set_autosuspend_delay(dev, SND_SOF_SUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_mark_last_busy(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
}

#[no_mangle]
pub unsafe extern "C" fn sof_of_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let mut desc: *const sof_dev_desc;
    let sof_pdata: *mut snd_sof_pdata;

    dev_info(&mut (*pdev).dev, b"DT DSP detected\0".as_ptr() as *const c_char);

    sof_pdata = devm_kzalloc(dev, core::mem::size_of::<snd_sof_pdata>(), GFP_KERNEL)
        as *mut snd_sof_pdata;
    if sof_pdata.is_null() {
        return -ENOMEM;
    }

    desc = device_get_match_data(dev) as *const sof_dev_desc;
    if desc.is_null() {
        return -ENODEV;
    }

    if (*desc).ops.is_null() {
        dev_err(
            dev,
            b"error: no matching DT descriptor ops\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }

    (*sof_pdata).desc = desc;
    (*sof_pdata).dev = &mut (*pdev).dev;

    (*sof_pdata).ipc_file_profile_base.ipc_type = (*desc).ipc_default;
    (*sof_pdata).ipc_file_profile_base.fw_path = fw_path;
    (*sof_pdata).ipc_file_profile_base.tplg_path = tplg_path;
    (*sof_pdata).ipc_file_profile_base.fw_name = fw_filename;
    (*sof_pdata).ipc_file_profile_base.tplg_name = tplg_filename;

    /* set callback to be called on successful device probe to enable runtime_pm */
    (*sof_pdata).sof_probe_complete = Some(sof_of_probe_complete);

    /* call sof helper for DSP hardware probe */
    snd_sof_device_probe(dev, sof_pdata)
}

#[no_mangle]
pub unsafe extern "C" fn sof_of_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);

    /* call sof helper for DSP hardware remove */
    snd_sof_device_remove(&mut (*pdev).dev);
}

#[no_mangle]
pub unsafe extern "C" fn sof_of_shutdown(pdev: *mut platform_device) {
    snd_sof_device_shutdown(&mut (*pdev).dev);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
