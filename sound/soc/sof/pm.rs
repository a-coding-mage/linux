// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

pub type c_int = i32;
pub type c_uint = u32;
pub type u32 = core::ffi::c_uint;
pub type bool_ = bool;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub system_suspend_target: u32,
    pub dsp_power_state: sof_dsp_power_state,
    pub fw_state: c_int,
    pub first_boot: bool,
    pub dspless_mode_selected: bool,
    pub pdata: *mut sof_platform_priv,
    pub enabled_cores_mask: u32,
    pub dfsentry_list: list_head,
    pub dsp_fw_boot_mutex: mutex,
}

#[repr(C)]
pub struct sof_dsp_power_state {
    pub state: u32,
}

#[repr(C)]
pub struct sof_platform_priv {
    pub desc: *mut sof_dev_desc,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub on_demand_dsp_boot: bool,
    pub use_acpi_target_states: bool,
}

#[repr(C)]
pub struct sof_ipc_pm_ops {
    pub ctx_save: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub ctx_restore: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
}

#[repr(C)]
pub struct sof_ipc_tplg_ops {
    pub set_up_all_pipelines: Option<unsafe extern "C" fn(*mut snd_sof_dev, bool) -> c_int>,
    pub tear_down_all_pipelines: Option<unsafe extern "C" fn(*mut snd_sof_dev, bool) -> c_int>,
}

#[repr(C)]
pub struct sof_ops_table {
    pub resume: Option<unsafe extern "C" fn()>,
    pub runtime_resume: Option<unsafe extern "C" fn()>,
    pub suspend: Option<unsafe extern "C" fn()>,
    pub runtime_suspend: Option<unsafe extern "C" fn()>,
    pub set_power_state: Option<unsafe extern "C" fn()>,
    pub remove: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct snd_sof_dfsentry {
    pub list: list_head,
    pub type_: c_int,
    pub access_type: c_int,
    pub cache_buf: *mut core::ffi::c_void,
    pub io_mem: *const core::ffi::c_void,
    pub size: usize,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pm_message_t {
    pub event: u32,
}

pub const SOF_SUSPEND_NONE: u32 = 0;
pub const SOF_SUSPEND_S0IX: u32 = 1;
pub const SOF_SUSPEND_S3: u32 = 3;
pub const SOF_SUSPEND_S4: u32 = 4;
pub const SOF_SUSPEND_S5: u32 = 5;
pub const SOF_DSP_PM_D0: u32 = 0;
pub const SOF_DSP_PM_D3: u32 = 3;
pub const SOF_FW_BOOT_NOT_STARTED: c_int = 0;
pub const SOF_FW_BOOT_PREPARE: c_int = 1;
pub const SOF_FW_BOOT_IN_PROGRESS: c_int = 2;
pub const SOF_FW_BOOT_COMPLETE: c_int = 3;
pub const SOF_FW_BOOT_FAILED: c_int = 4;
pub const SOF_FW_CRASHED: c_int = 5;
pub const SOF_DSPLESS_MODE: c_int = 6;
pub const SOF_DFSENTRY_TYPE_BUF: c_int = 0;
pub const SOF_DEBUGFS_ACCESS_D0_ONLY: c_int = 1;
pub const EBUSY: c_int = 16;
pub const EAGAIN: c_int = 11;
pub const ACPI_STATE_S0: c_int = 0;
pub const ACPI_STATE_S1: c_int = 1;
pub const ACPI_STATE_S2: c_int = 2;
pub const ACPI_STATE_S3: c_int = 3;
pub const ACPI_STATE_S4: c_int = 4;
pub const ACPI_STATE_S5: c_int = 5;

// module_param_named(on_demand_boot, override_on_demand_boot, int, 0444);
// MODULE_PARM_DESC(on_demand_boot, "Force on-demand DSP boot: 0 - disabled, 1 - enabled");
static mut override_on_demand_boot: c_int = -1;

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut snd_sof_dev;
    fn sof_ipc_get_pm_ops(sdev: *mut snd_sof_dev) -> *const sof_ipc_pm_ops;
    fn sof_ipc_get_tplg_ops(sdev: *mut snd_sof_dev) -> *const sof_ipc_tplg_ops;
    fn sof_ops(sdev: *mut snd_sof_dev) -> *const sof_ops_table;
    fn snd_sof_stream_suspend_ignored(sdev: *mut snd_sof_dev) -> bool;
    fn snd_sof_load_firmware(sdev: *mut snd_sof_dev) -> c_int;
    fn snd_sof_run_firmware(sdev: *mut snd_sof_dev) -> c_int;
    fn sof_fw_trace_resume(sdev: *mut snd_sof_dev) -> c_int;
    fn sof_fw_trace_suspend(sdev: *mut snd_sof_dev, pm_state: pm_message_t);
    fn sof_set_fw_state(sdev: *mut snd_sof_dev, state: c_int);
    fn sof_resume_clients(sdev: *mut snd_sof_dev);
    fn sof_suspend_clients(sdev: *mut snd_sof_dev, pm_state: pm_message_t);
    fn snd_sof_dsp_runtime_resume(sdev: *mut snd_sof_dev) -> c_int;
    fn snd_sof_dsp_resume(sdev: *mut snd_sof_dev) -> c_int;
    fn snd_sof_dsp_runtime_suspend(sdev: *mut snd_sof_dev) -> c_int;
    fn snd_sof_dsp_suspend(sdev: *mut snd_sof_dev, target_state: u32) -> c_int;
    fn snd_sof_dsp_hw_params_upon_resume(sdev: *mut snd_sof_dev) -> c_int;
    fn snd_sof_dsp_power_target_external(sdev: *mut snd_sof_dev) -> u32;
    fn snd_sof_dsp_runtime_idle(sdev: *mut snd_sof_dev) -> c_int;
    fn acpi_target_system_state() -> c_int;
    fn memcpy_fromio(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_warn(dev: *mut device, fmt: *const u8, ...);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
}

/*
 * Helper function to determine the target DSP state during
 * system suspend. This function only cares about the device
 * D-states. Platform-specific substates, if any, should be
 * handled by the platform-specific parts.
 */
unsafe fn snd_sof_dsp_power_target(sdev: *mut snd_sof_dev) -> u32 {
    let target_dsp_state: u32;

    match (*sdev).system_suspend_target {
        SOF_SUSPEND_S5 | SOF_SUSPEND_S4 => {
            /* DSP should be in D3 if the system is suspending to S3+ */
            target_dsp_state = SOF_DSP_PM_D3;
        }
        SOF_SUSPEND_S3 => {
            /* DSP should be in D3 if the system is suspending to S3 */
            target_dsp_state = SOF_DSP_PM_D3;
        }
        SOF_SUSPEND_S0IX => {
            /*
             * Currently, the only criterion for retaining the DSP in D0
             * is that there are streams that ignored the suspend trigger.
             * Additional criteria such Soundwire clock-stop mode and
             * device suspend latency considerations will be added later.
             */
            if snd_sof_stream_suspend_ignored(sdev) {
                target_dsp_state = SOF_DSP_PM_D0;
            } else {
                target_dsp_state = SOF_DSP_PM_D3;
            }
        }
        _ => {
            /* This case would be during runtime suspend */
            target_dsp_state = SOF_DSP_PM_D3;
        }
    }

    target_dsp_state
}

// #if IS_ENABLED(CONFIG_SND_SOC_SOF_DEBUG_ENABLE_DEBUGFS_CACHE)
unsafe fn sof_cache_debugfs(sdev: *mut snd_sof_dev) {
    let mut pos = (*sdev).dfsentry_list.next;

    while pos != &mut (*sdev).dfsentry_list {
        let dfse = pos as *mut snd_sof_dfsentry;
        pos = (*pos).next;

        /* nothing to do if debugfs buffer is not IO mem */
        if (*dfse).type_ == SOF_DFSENTRY_TYPE_BUF {
            continue;
        }

        /* cache memory that is only accessible in D0 */
        if (*dfse).access_type == SOF_DEBUGFS_ACCESS_D0_ONLY {
            memcpy_fromio((*dfse).cache_buf, (*dfse).io_mem, (*dfse).size);
        }
    }
}
// #endif

#[no_mangle]
pub unsafe extern "C" fn snd_sof_boot_dsp_firmware(sdev: *mut snd_sof_dev) -> c_int {
    let pm_ops = sof_ipc_get_pm_ops(sdev);
    let tplg_ops = sof_ipc_get_tplg_ops(sdev);
    let mut ret: c_int;

    mutex_lock(&mut (*sdev).dsp_fw_boot_mutex);

    if (*sdev).fw_state == SOF_FW_BOOT_COMPLETE {
        /* Firmware already booted, just return */
        mutex_unlock(&mut (*sdev).dsp_fw_boot_mutex);
        return 0;
    }

    dev_dbg((*sdev).dev, c"Booting DSP firmware\n".as_ptr() as *const u8);

    sof_set_fw_state(sdev, SOF_FW_BOOT_PREPARE);

    /* load the firmware */
    ret = snd_sof_load_firmware(sdev);
    if ret < 0 {
        dev_err((*sdev).dev, c"%s: failed to load DSP firmware: %d\n".as_ptr() as *const u8,
                c"snd_sof_boot_dsp_firmware".as_ptr(), ret);
        sof_set_fw_state(sdev, SOF_FW_BOOT_FAILED);
        mutex_unlock(&mut (*sdev).dsp_fw_boot_mutex);
        return ret;
    }

    sof_set_fw_state(sdev, SOF_FW_BOOT_IN_PROGRESS);

    /*
     * Boot the firmware. The FW boot status will be modified
     * in snd_sof_run_firmware() depending on the outcome.
     */
    ret = snd_sof_run_firmware(sdev);
    if ret < 0 {
        dev_err((*sdev).dev, c"%s: failed to boot DSP firmware: %d\n".as_ptr() as *const u8,
                c"snd_sof_boot_dsp_firmware".as_ptr(), ret);
        sof_set_fw_state(sdev, SOF_FW_BOOT_FAILED);
        mutex_unlock(&mut (*sdev).dsp_fw_boot_mutex);
        return ret;
    }

    /* resume DMA trace */
    ret = sof_fw_trace_resume(sdev);
    if ret < 0 {
        /* non fatal */
        dev_warn((*sdev).dev, c"%s: failed to resume trace: %d\n".as_ptr() as *const u8,
                 c"snd_sof_boot_dsp_firmware".as_ptr(), ret);
    }

    /* restore pipelines */
    if !tplg_ops.is_null() {
        if let Some(set_up_all_pipelines) = (*tplg_ops).set_up_all_pipelines {
            ret = set_up_all_pipelines(sdev, false);
            if ret < 0 {
                dev_err((*sdev).dev, c"%s: failed to restore pipeline: %d\n".as_ptr() as *const u8,
                        c"snd_sof_boot_dsp_firmware".as_ptr(), ret);
                // goto setup_fail;
                #[cfg(any())]
                {}
            }
        }
    }

    if ret >= 0 {
        /* Notify clients not managed by pm framework about core resume */
        sof_resume_clients(sdev);

        /* notify DSP of system resume */
        if !pm_ops.is_null() {
            if let Some(ctx_restore) = (*pm_ops).ctx_restore {
                ret = ctx_restore(sdev);
                if ret < 0 {
                    dev_err((*sdev).dev, c"%s: ctx_restore IPC failed: %d\n".as_ptr() as *const u8,
                            c"snd_sof_boot_dsp_firmware".as_ptr(), ret);
                }
            }
        }
    }

    // setup_fail:
    // #if IS_ENABLED(CONFIG_SND_SOC_SOF_DEBUG_ENABLE_DEBUGFS_CACHE)
    if ret < 0 {
        /*
         * Debugfs cannot be read in runtime suspend, so cache
         * the contents upon failure. This allows to capture
         * possible DSP coredump information.
         */
        sof_cache_debugfs(sdev);
    }
    // #endif

    mutex_unlock(&mut (*sdev).dsp_fw_boot_mutex);
    ret
}
// EXPORT_SYMBOL(snd_sof_boot_dsp_firmware);

unsafe fn sof_resume(dev: *mut device, runtime_resume: bool) -> c_int {
    let sdev = dev_get_drvdata(dev);
    let old_state: u32 = (*sdev).dsp_power_state.state;
    let on_demand_boot: bool;
    let mut ret: c_int;

    /* do nothing if dsp resume callbacks are not set */
    if !runtime_resume && (*sof_ops(sdev)).resume.is_none() {
        return 0;
    }

    if runtime_resume && (*sof_ops(sdev)).runtime_resume.is_none() {
        return 0;
    }

    /* DSP was never successfully started, nothing to resume */
    if (*sdev).first_boot {
        return 0;
    }

    /*
     * if the runtime_resume flag is set, call the runtime_resume routine
     * or else call the system resume routine
     */
    if runtime_resume {
        ret = snd_sof_dsp_runtime_resume(sdev);
    } else {
        ret = snd_sof_dsp_resume(sdev);
    }
    if ret < 0 {
        dev_err((*sdev).dev, c"error: failed to power up DSP after resume\n".as_ptr() as *const u8);
        return ret;
    }

    if (*sdev).dspless_mode_selected {
        sof_set_fw_state(sdev, SOF_DSPLESS_MODE);
        return 0;
    }

    /*
     * Nothing further to be done for platforms that support the low power
     * D0 substate. Resume trace and return when resuming from
     * low-power D0 substate
     */
    if !runtime_resume && (*sof_ops(sdev)).set_power_state.is_some() && old_state == SOF_DSP_PM_D0 {
        ret = sof_fw_trace_resume(sdev);
        if ret < 0 {
            /* non fatal */
            dev_warn((*sdev).dev, c"failed to enable trace after resume %d\n".as_ptr() as *const u8, ret);
        }
        return 0;
    }

    if override_on_demand_boot > -1 {
        on_demand_boot = override_on_demand_boot != 0;
    } else {
        on_demand_boot = (*(*(*sdev).pdata).desc).on_demand_dsp_boot;
    }

    if on_demand_boot {
        /* Only change the fw_state to PREPARE but skip booting */
        sof_set_fw_state(sdev, SOF_FW_BOOT_PREPARE);
        return 0;
    }

    snd_sof_boot_dsp_firmware(sdev)
}

unsafe fn sof_suspend(dev: *mut device, runtime_suspend: bool) -> c_int {
    let sdev = dev_get_drvdata(dev);
    let pm_ops = sof_ipc_get_pm_ops(sdev);
    let tplg_ops = sof_ipc_get_tplg_ops(sdev);
    let mut pm_state = pm_message_t { event: 0 };
    let target_state: u32 = snd_sof_dsp_power_target(sdev);
    let old_state: u32 = (*sdev).dsp_power_state.state;
    let mut ret: c_int = 0;

    /* do nothing if dsp suspend callback is not set */
    if !runtime_suspend && (*sof_ops(sdev)).suspend.is_none() {
        return 0;
    }

    if runtime_suspend && (*sof_ops(sdev)).runtime_suspend.is_none() {
        return 0;
    }

    /*
     * we need to tear down pipelines only if the DSP hardware is
     * active, which happens for PCI devices. if the device is
     * suspended, it is brought back to full power and then
     * suspended again
     */
    if !tplg_ops.is_null()
        && (*tplg_ops).tear_down_all_pipelines.is_some()
        && old_state == SOF_DSP_PM_D0
    {
        (*tplg_ops).tear_down_all_pipelines.unwrap()(sdev, false);
    }

    if (*sdev).fw_state == SOF_FW_BOOT_COMPLETE {
        /* prepare for streams to be resumed properly upon resume */
        if !runtime_suspend {
            ret = snd_sof_dsp_hw_params_upon_resume(sdev);
            if ret < 0 {
                dev_err((*sdev).dev,
                        c"error: setting hw_params flag during suspend %d\n".as_ptr() as *const u8,
                        ret);
                return ret;
            }
        }

        pm_state.event = target_state;

        /* suspend DMA trace */
        sof_fw_trace_suspend(sdev, pm_state);

        /* Notify clients not managed by pm framework about core suspend */
        sof_suspend_clients(sdev, pm_state);

        /* Skip to platform-specific suspend if DSP is entering D0 */
        if target_state != SOF_DSP_PM_D0 {
            // #if IS_ENABLED(CONFIG_SND_SOC_SOF_DEBUG_ENABLE_DEBUGFS_CACHE)
            /* cache debugfs contents during runtime suspend */
            if runtime_suspend {
                sof_cache_debugfs(sdev);
            }
            // #endif

            /* notify DSP of upcoming power down */
            if !pm_ops.is_null() {
                if let Some(ctx_save) = (*pm_ops).ctx_save {
                    ret = ctx_save(sdev);
                    if ret == -EBUSY || ret == -EAGAIN {
                        /*
                         * runtime PM has logic to handle -EBUSY/-EAGAIN so
                         * pass these errors up
                         */
                        dev_err((*sdev).dev, c"ctx_save IPC error during suspend: %d\n".as_ptr() as *const u8, ret);
                        return ret;
                    } else if ret < 0 {
                        /* FW in unexpected state, continue to power down */
                        dev_warn((*sdev).dev,
                                 c"ctx_save IPC error: %d, proceeding with suspend\n".as_ptr() as *const u8,
                                 ret);
                    }
                }
            }
        }
    }

    // suspend:

    /* return if the DSP was not probed successfully */
    if (*sdev).fw_state == SOF_FW_BOOT_NOT_STARTED {
        return 0;
    }

    /* platform-specific suspend */
    if runtime_suspend {
        ret = snd_sof_dsp_runtime_suspend(sdev);
    } else {
        ret = snd_sof_dsp_suspend(sdev, target_state);
    }
    if ret < 0 {
        dev_err((*sdev).dev,
                c"error: failed to power down DSP during suspend %d\n".as_ptr() as *const u8,
                ret);
    }

    /* Do not reset FW state if DSP is in D0 */
    if target_state == SOF_DSP_PM_D0 {
        return ret;
    }

    /* reset FW state */
    sof_set_fw_state(sdev, SOF_FW_BOOT_NOT_STARTED);
    (*sdev).enabled_cores_mask = 0;

    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_sof_dsp_power_down_notify(sdev: *mut snd_sof_dev) -> c_int {
    let pm_ops = sof_ipc_get_pm_ops(sdev);

    /*
     * Notify DSP of upcoming power down only if the firmware has been
     * booted up
     */
    if (*sdev).fw_state == SOF_FW_BOOT_COMPLETE
        && (*sof_ops(sdev)).remove.is_some()
        && !pm_ops.is_null()
        && (*pm_ops).ctx_save.is_some()
    {
        return (*pm_ops).ctx_save.unwrap()(sdev);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_sof_runtime_suspend(dev: *mut device) -> c_int {
    sof_suspend(dev, true)
}
// EXPORT_SYMBOL(snd_sof_runtime_suspend);

#[no_mangle]
pub unsafe extern "C" fn snd_sof_runtime_idle(dev: *mut device) -> c_int {
    let sdev = dev_get_drvdata(dev);

    snd_sof_dsp_runtime_idle(sdev)
}
// EXPORT_SYMBOL(snd_sof_runtime_idle);

#[no_mangle]
pub unsafe extern "C" fn snd_sof_runtime_resume(dev: *mut device) -> c_int {
    sof_resume(dev, true)
}
// EXPORT_SYMBOL(snd_sof_runtime_resume);

#[no_mangle]
pub unsafe extern "C" fn snd_sof_resume(dev: *mut device) -> c_int {
    sof_resume(dev, false)
}
// EXPORT_SYMBOL(snd_sof_resume);

#[no_mangle]
pub unsafe extern "C" fn snd_sof_suspend(dev: *mut device) -> c_int {
    sof_suspend(dev, false)
}
// EXPORT_SYMBOL(snd_sof_suspend);

#[no_mangle]
pub unsafe extern "C" fn snd_sof_prepare(dev: *mut device) -> c_int {
    let sdev = dev_get_drvdata(dev);
    let desc = (*(*sdev).pdata).desc;

    /* will suspend to S3 by default */
    (*sdev).system_suspend_target = SOF_SUSPEND_S3;

    /*
     * if the firmware is crashed or boot failed then we try to aim for S3
     * to reboot the firmware
     */
    if (*sdev).fw_state == SOF_FW_CRASHED || (*sdev).fw_state == SOF_FW_BOOT_FAILED {
        return 0;
    }

    if !(*desc).use_acpi_target_states {
        return 0;
    }

    // #if defined(CONFIG_ACPI)
    match acpi_target_system_state() {
        ACPI_STATE_S0 => {
            (*sdev).system_suspend_target = SOF_SUSPEND_S0IX;
        }
        ACPI_STATE_S1 | ACPI_STATE_S2 | ACPI_STATE_S3 => {
            (*sdev).system_suspend_target = SOF_SUSPEND_S3;
        }
        ACPI_STATE_S4 => {
            (*sdev).system_suspend_target = SOF_SUSPEND_S4;
        }
        ACPI_STATE_S5 => {
            (*sdev).system_suspend_target = SOF_SUSPEND_S5;
        }
        _ => {}
    }
    // #endif

    0
}
// EXPORT_SYMBOL(snd_sof_prepare);

#[no_mangle]
pub unsafe extern "C" fn snd_sof_complete(dev: *mut device) {
    let sdev = dev_get_drvdata(dev);

    (*sdev).system_suspend_target = SOF_SUSPEND_NONE;
}
// EXPORT_SYMBOL(snd_sof_complete);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
