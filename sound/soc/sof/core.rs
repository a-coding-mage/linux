// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

// C includes translated as external dependency intent:
// <linux/firmware.h>, <linux/module.h>, <sound/soc.h>, <sound/sof.h>,
// "sof-priv.h", "sof-of-dev.h", "ops.h", <trace/events/sof.h>
// CREATE_TRACE_POINTS

pub const TIMEOUT_DEFAULT_IPC_MS: u32 = 500;
pub const TIMEOUT_DEFAULT_BOOT_MS: u32 = 2000;

unsafe extern "C" {
    static mut CONFIG_SND_SOC_SOF_DEBUG_ENABLE_FIRMWARE_TRACE: bool;
    static mut CONFIG_SND_SOC_SOF_DEBUG: bool;
    static mut CONFIG_SND_SOC_SOF_FORCE_NOCODEC_MODE: bool;
    static mut CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT: bool;
    static mut CONFIG_SND_SOC_SOF_NOCODEC: bool;
    static mut CONFIG_SND_SOC_SOF_PROBE_WORK_QUEUE: bool;

    static mut SOF_IPC_PANIC_MEM: u32;
    static mut SOF_IPC_PANIC_WORK: u32;
    static mut SOF_IPC_PANIC_IPC: u32;
    static mut SOF_IPC_PANIC_ARCH: u32;
    static mut SOF_IPC_PANIC_PLATFORM: u32;
    static mut SOF_IPC_PANIC_TASK: u32;
    static mut SOF_IPC_PANIC_EXCEPTION: u32;
    static mut SOF_IPC_PANIC_DEADLOCK: u32;
    static mut SOF_IPC_PANIC_STACK: u32;
    static mut SOF_IPC_PANIC_IDLE: u32;
    static mut SOF_IPC_PANIC_WFI: u32;
    static mut SOF_IPC_PANIC_ASSERT: u32;
    static mut SOF_IPC_PANIC_MAGIC_MASK: u32;
    static mut SOF_IPC_PANIC_MAGIC: u32;
    static mut SOF_IPC_PANIC_CODE_MASK: u32;

    static mut SOF_DBG_FORCE_NOCODEC: c_int;
    static mut SOF_DBG_ENABLE_TRACE: c_int;
    static mut SOF_DBG_DSPLESS_MODE: c_int;
    static mut SOF_IPC_TYPE_COUNT: c_int;
    static mut SOF_DSP_PM_D0: c_int;
    static mut GFP_KERNEL: c_uint;
    static mut ENODEV: c_int;
    static mut ENOMEM: c_int;
    static mut EINVAL: c_int;
    static mut PLATFORM_DEVID_NONE: c_int;

    fn dev_printk(level: *const c_char, dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn of_machine_is_compatible(compat: *const c_char) -> bool;
    fn snd_sof_machine_select(sdev: *mut snd_sof_dev) -> *mut snd_soc_acpi_mach;
    fn snd_sof_set_mach_params(mach: *mut snd_soc_acpi_mach, sdev: *mut snd_sof_dev);
    fn sof_create_ipc_file_profile(
        sdev: *mut snd_sof_dev,
        base_profile: *mut sof_loadable_file_profile,
        out_profile: *mut sof_loadable_file_profile,
    ) -> c_int;
    fn sof_ops_init(sdev: *mut snd_sof_dev) -> c_int;
    fn sof_ops(sdev: *mut snd_sof_dev) -> *mut snd_sof_dsp_ops;
    fn sof_ops_free(sdev: *mut snd_sof_dev);
    fn snd_sof_probe(sdev: *mut snd_sof_dev) -> c_int;
    fn snd_sof_remove(sdev: *mut snd_sof_dev);
    fn snd_sof_remove_late(sdev: *mut snd_sof_dev);
    fn sof_client_fw_state_dispatcher(sdev: *mut snd_sof_dev);
    fn sof_oops(sdev: *mut snd_sof_dev, level: *const c_char, oops: *mut c_void);
    fn sof_stack(
        sdev: *mut snd_sof_dev,
        level: *const c_char,
        oops: *mut c_void,
        stack: *mut c_void,
        stack_words: usize,
    );
    fn snd_sof_new_platform_drv(sdev: *mut snd_sof_dev);
    fn snd_sof_dbg_init(sdev: *mut snd_sof_dev) -> c_int;
    fn snd_sof_ipc_init(sdev: *mut snd_sof_dev) -> *mut c_void;
    fn snd_sof_load_firmware(sdev: *mut snd_sof_dev) -> c_int;
    fn snd_sof_run_firmware(sdev: *mut snd_sof_dev) -> c_int;
    fn sof_fw_trace_init(sdev: *mut snd_sof_dev) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *mut c_void,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
    fn snd_sof_machine_register(sdev: *mut snd_sof_dev, plat_data: *mut snd_sof_pdata) -> c_int;
    fn sof_register_clients(sdev: *mut snd_sof_dev) -> c_int;
    fn pm_runtime_get_noresume(dev: *mut device);
    fn snd_sof_machine_unregister(sdev: *mut snd_sof_dev, plat_data: *mut snd_sof_pdata);
    fn sof_fw_trace_free(sdev: *mut snd_sof_dev);
    fn snd_sof_fw_unload(sdev: *mut snd_sof_dev);
    fn snd_sof_ipc_free(sdev: *mut snd_sof_dev);
    fn snd_sof_free_debug(sdev: *mut snd_sof_dev);
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(mutex: *mut mutex);
    fn snd_sof_probe_early(sdev: *mut snd_sof_dev) -> c_int;
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn schedule_work(work: *mut work_struct) -> bool;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn sof_unregister_clients(sdev: *mut snd_sof_dev);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn snd_sof_dsp_power_down_notify(sdev: *mut snd_sof_dev) -> c_int;
    fn snd_sof_shutdown(sdev: *mut snd_sof_dev) -> c_int;
    fn platform_device_register_data(
        parent: *mut device,
        name: *const c_char,
        id: c_int,
        data: *const c_void,
        size: usize,
    ) -> *mut platform_device;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn platform_device_unregister(pdev: *mut platform_device);
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_sof_dsp_power_state {
    pub state: c_int,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub pdata: *mut snd_sof_pdata,
    pub fw_state: sof_fw_state,
    pub dsp_power_state: snd_sof_dsp_power_state,
    pub first_boot: bool,
    pub dspless_mode_selected: bool,
    pub ipc: *mut c_void,
    pub fw_trace_is_supported: bool,
    pub plat_drv: c_void,
    pub probe_completed: bool,
    pub pcm_list: list_head,
    pub kcontrol_list: list_head,
    pub widget_list: list_head,
    pub pipeline_list: list_head,
    pub dai_list: list_head,
    pub dai_link_list: list_head,
    pub route_list: list_head,
    pub ipc_client_list: list_head,
    pub ipc_rx_handler_list: list_head,
    pub fw_state_handler_list: list_head,
    pub ipc_lock: spinlock_t,
    pub hw_lock: spinlock_t,
    pub power_state_access: mutex,
    pub ipc_client_mutex: mutex,
    pub client_event_handler_mutex: mutex,
    pub dsp_fw_boot_mutex: mutex,
    pub ipc_timeout: c_uint,
    pub boot_timeout: c_uint,
    pub probe_work: work_struct,
    pub d3_prevented: bool,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub desc: *const sof_dev_desc,
    pub tplg_filename: *const c_char,
    pub fw_filename: *const c_char,
    pub fw_filename_prefix: *const c_char,
    pub fw_lib_prefix: *const c_char,
    pub tplg_filename_prefix: *const c_char,
    pub ipc_file_profile_base: sof_loadable_file_profile,
    pub ipc_type: c_int,
    pub machine: *mut snd_soc_acpi_mach,
    pub of_machine: *const snd_sof_of_mach,
    pub subsystem_id_set: bool,
    pub subsystem_vendor: u16,
    pub subsystem_device: u16,
    pub disable_function_topology: bool,
    pub sof_probe_complete: Option<unsafe extern "C" fn(*mut device)>,
    pub pdev_mach: *mut platform_device,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub of_machines: *mut snd_sof_of_mach,
    pub nocodec_tplg_filename: *const c_char,
    pub ipc_default: c_int,
    pub ipc_supported_mask: c_uint,
    pub ipc_timeout: c_uint,
    pub boot_timeout: c_uint,
    pub dspless_mode_supported: bool,
}

#[repr(C)]
pub struct snd_sof_of_mach {
    pub compatible: *const c_char,
    pub sof_tplg_filename: *const c_char,
    pub fw_filename: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub drv_name: *const c_char,
    pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub subsystem_vendor: u16,
    pub subsystem_device: u16,
    pub subsystem_id_set: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_loadable_file_profile {
    pub ipc_type: c_int,
    pub fw_path: *const c_char,
    pub fw_path_postfix: *const c_char,
    pub fw_lib_path: *const c_char,
    pub fw_lib_path_postfix: *const c_char,
    pub fw_name: *const c_char,
    pub tplg_path: *const c_char,
    pub tplg_name: *const c_char,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    pub probe: *mut c_void,
    pub run: *mut c_void,
    pub block_read: *mut c_void,
    pub block_write: *mut c_void,
    pub send_msg: *mut c_void,
    pub load_firmware: *mut c_void,
    pub ipc_msg_data: *mut c_void,
    pub drv: *mut c_void,
    pub num_drv: c_int,
    pub runtime_suspend: *mut c_void,
    pub runtime_resume: *mut c_void,
}

#[repr(C)]
pub struct sof_ipc_panic_info {
    pub filename: *const c_char,
    pub linenum: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum sof_fw_state {
    SOF_FW_BOOT_NOT_STARTED = 0,
    SOF_FW_BOOT_PREPARE = 1,
    SOF_FW_BOOT_IN_PROGRESS = 2,
    SOF_FW_BOOT_COMPLETE = 3,
    SOF_FW_BOOT_FAILED = 4,
    SOF_FW_READY_FAILED = 5,
    SOF_FW_CRASHED = 6,
    SOF_DSPLESS_MODE = 7,
}

#[repr(C)]
struct sof_panic_msg {
    id: u32,
    msg: *const c_char,
}

// Module parameters for firmware, topology and IPC type override
static mut override_fw_path: *mut c_char = ptr::null_mut();
static mut override_fw_filename: *mut c_char = ptr::null_mut();
static mut override_lib_path: *mut c_char = ptr::null_mut();
static mut override_tplg_path: *mut c_char = ptr::null_mut();
static mut override_tplg_filename: *mut c_char = ptr::null_mut();
static mut override_ipc_type: c_int = -1;

/* see SOF_DBG_ flags */
static mut sof_core_debug: c_int = 0;

// #if IS_ENABLED(CONFIG_SND_SOC_SOF_DEBUG)
static mut sof_ipc_timeout_ms: c_uint = 0;
static mut sof_boot_timeout_ms: c_uint = 0;
// #endif

unsafe fn IS_ENABLED(config: bool) -> bool {
    config
}

unsafe fn BIT(nr: c_int) -> c_uint {
    1u32 << nr
}

/**
 * sof_debug_check_flag - check if a given flag(s) is set in sof_core_debug
 * @mask: Flag or combination of flags to check
 *
 * Returns true if all bits set in mask is also set in sof_core_debug, otherwise
 * false
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_debug_check_flag(mask: c_int) -> bool {
    if (sof_core_debug & mask) == mask {
        return true;
    }

    false
}

/*
 * FW Panic/fault handling.
 */

static mut panic_msg: [sof_panic_msg; 12] = [
    sof_panic_msg { id: 0, msg: c"out of memory".as_ptr() },
    sof_panic_msg { id: 0, msg: c"work subsystem init failed".as_ptr() },
    sof_panic_msg { id: 0, msg: c"IPC subsystem init failed".as_ptr() },
    sof_panic_msg { id: 0, msg: c"arch init failed".as_ptr() },
    sof_panic_msg { id: 0, msg: c"platform init failed".as_ptr() },
    sof_panic_msg { id: 0, msg: c"scheduler init failed".as_ptr() },
    sof_panic_msg { id: 0, msg: c"runtime exception".as_ptr() },
    sof_panic_msg { id: 0, msg: c"deadlock".as_ptr() },
    sof_panic_msg { id: 0, msg: c"stack overflow".as_ptr() },
    sof_panic_msg { id: 0, msg: c"can't enter idle".as_ptr() },
    sof_panic_msg { id: 0, msg: c"invalid wait state".as_ptr() },
    sof_panic_msg { id: 0, msg: c"assertion failed".as_ptr() },
];

unsafe fn init_panic_msg_ids() {
    panic_msg[0].id = SOF_IPC_PANIC_MEM;
    panic_msg[1].id = SOF_IPC_PANIC_WORK;
    panic_msg[2].id = SOF_IPC_PANIC_IPC;
    panic_msg[3].id = SOF_IPC_PANIC_ARCH;
    panic_msg[4].id = SOF_IPC_PANIC_PLATFORM;
    panic_msg[5].id = SOF_IPC_PANIC_TASK;
    panic_msg[6].id = SOF_IPC_PANIC_EXCEPTION;
    panic_msg[7].id = SOF_IPC_PANIC_DEADLOCK;
    panic_msg[8].id = SOF_IPC_PANIC_STACK;
    panic_msg[9].id = SOF_IPC_PANIC_IDLE;
    panic_msg[10].id = SOF_IPC_PANIC_WFI;
    panic_msg[11].id = SOF_IPC_PANIC_ASSERT;
}

/**
 * sof_print_oops_and_stack - Handle the printing of DSP oops and stack trace
 * @sdev: Pointer to the device's sdev
 * @level: prink log level to use for the printing
 * @panic_code: the panic code
 * @tracep_code: tracepoint code
 * @oops: Pointer to DSP specific oops data
 * @panic_info: Pointer to the received panic information message
 * @stack: Pointer to the call stack data
 * @stack_words: Number of words in the stack data
 *
 * helper to be called from .dbg_dump callbacks. No error code is
 * provided, it's left as an exercise for the caller of .dbg_dump
 * (typically IPC or loader)
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_print_oops_and_stack(
    sdev: *mut snd_sof_dev,
    level: *const c_char,
    panic_code: u32,
    tracep_code: u32,
    oops: *mut c_void,
    panic_info: *mut sof_ipc_panic_info,
    stack: *mut c_void,
    stack_words: usize,
) {
    let code: u32;

    /* is firmware dead ? */
    if (panic_code & SOF_IPC_PANIC_MAGIC_MASK) != SOF_IPC_PANIC_MAGIC {
        dev_printk(
            level,
            (*sdev).dev,
            c"unexpected fault %#010x trace %#010x\n".as_ptr(),
            panic_code,
            tracep_code,
        );
        return; /* no fault ? */
    }

    code = panic_code & (SOF_IPC_PANIC_MAGIC_MASK | SOF_IPC_PANIC_CODE_MASK);

    init_panic_msg_ids();
    let mut i = 0usize;
    while i < panic_msg.len() {
        if panic_msg[i].id == code {
            dev_printk(
                level,
                (*sdev).dev,
                c"reason: %s (%#x)\n".as_ptr(),
                panic_msg[i].msg,
                code & SOF_IPC_PANIC_CODE_MASK,
            );
            dev_printk(level, (*sdev).dev, c"trace point: %#010x\n".as_ptr(), tracep_code);
            goto_out(sdev, level, oops, panic_info, stack, stack_words);
            return;
        }
        i += 1;
    }

    /* unknown error */
    dev_printk(
        level,
        (*sdev).dev,
        c"unknown panic code: %#x\n".as_ptr(),
        code & SOF_IPC_PANIC_CODE_MASK,
    );
    dev_printk(level, (*sdev).dev, c"trace point: %#010x\n".as_ptr(), tracep_code);

    goto_out(sdev, level, oops, panic_info, stack, stack_words);
}

unsafe fn goto_out(
    sdev: *mut snd_sof_dev,
    level: *const c_char,
    oops: *mut c_void,
    panic_info: *mut sof_ipc_panic_info,
    stack: *mut c_void,
    stack_words: usize,
) {
    dev_printk(
        level,
        (*sdev).dev,
        c"panic at %s:%d\n".as_ptr(),
        (*panic_info).filename,
        (*panic_info).linenum,
    );
    sof_oops(sdev, level, oops);
    sof_stack(sdev, level, oops, stack, stack_words);
}

/* Helper to manage DSP state */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_set_fw_state(sdev: *mut snd_sof_dev, new_state: sof_fw_state) {
    if (*sdev).fw_state == new_state {
        return;
    }

    dev_dbg(
        (*sdev).dev,
        c"fw_state change: %d -> %d\n".as_ptr(),
        (*sdev).fw_state as c_int,
        new_state as c_int,
    );
    (*sdev).fw_state = new_state;

    match new_state {
        sof_fw_state::SOF_FW_BOOT_NOT_STARTED
        | sof_fw_state::SOF_FW_BOOT_COMPLETE
        | sof_fw_state::SOF_FW_CRASHED => {
            sof_client_fw_state_dispatcher(sdev);
        }
        _ => {}
    }
}

unsafe fn sof_of_machine_select(sdev: *mut snd_sof_dev) -> *mut snd_sof_of_mach {
    let sof_pdata = (*sdev).pdata;
    let desc = (*sof_pdata).desc;
    let mut mach = (*desc).of_machines;

    if mach.is_null() {
        return ptr::null_mut();
    }

    while !(*mach).compatible.is_null() {
        if of_machine_is_compatible((*mach).compatible) {
            (*sof_pdata).tplg_filename = (*mach).sof_tplg_filename;
            if !(*mach).fw_filename.is_null() {
                (*sof_pdata).fw_filename = (*mach).fw_filename;
            }

            return mach;
        }
        mach = mach.add(1);
    }

    ptr::null_mut()
}

/* SOF Driver enumeration */
unsafe fn sof_machine_check(sdev: *mut snd_sof_dev) -> c_int {
    let sof_pdata = (*sdev).pdata;
    let desc = (*sof_pdata).desc;
    let mut mach: *mut snd_soc_acpi_mach;

    if !IS_ENABLED(CONFIG_SND_SOC_SOF_FORCE_NOCODEC_MODE) {
        let of_mach: *mut snd_sof_of_mach;

        if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
            && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
        {
            return sof_machine_check_nocodec(sdev, sof_pdata, desc);
        }

        /* find machine */
        mach = snd_sof_machine_select(sdev);
        if !mach.is_null() {
            (*sof_pdata).machine = mach;

            if (*sof_pdata).subsystem_id_set {
                (*mach).mach_params.subsystem_vendor = (*sof_pdata).subsystem_vendor;
                (*mach).mach_params.subsystem_device = (*sof_pdata).subsystem_device;
                (*mach).mach_params.subsystem_id_set = true;
            }

            snd_sof_set_mach_params(mach, sdev);
            return 0;
        }

        of_mach = sof_of_machine_select(sdev);
        if !of_mach.is_null() {
            (*sof_pdata).of_machine = of_mach;
            return 0;
        }

        if !IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC) {
            dev_err(
                (*sdev).dev,
                c"error: no matching ASoC machine driver found - aborting probe\n".as_ptr(),
            );
            return -ENODEV;
        }
    } else {
        dev_warn((*sdev).dev, c"Force to use nocodec mode\n".as_ptr());
    }

    sof_machine_check_nocodec(sdev, sof_pdata, desc)
}

unsafe fn sof_machine_check_nocodec(
    sdev: *mut snd_sof_dev,
    sof_pdata: *mut snd_sof_pdata,
    desc: *const sof_dev_desc,
) -> c_int {
    /* select nocodec mode */
    dev_warn((*sdev).dev, c"Using nocodec machine driver\n".as_ptr());
    let mach = devm_kzalloc((*sdev).dev, size_of::<snd_soc_acpi_mach>(), GFP_KERNEL)
        as *mut snd_soc_acpi_mach;
    if mach.is_null() {
        return -ENOMEM;
    }

    (*mach).drv_name = c"sof-nocodec".as_ptr();
    if (*sof_pdata).tplg_filename.is_null() {
        (*sof_pdata).tplg_filename = (*desc).nocodec_tplg_filename;
    }

    (*sof_pdata).machine = mach;
    snd_sof_set_mach_params(mach, sdev);

    0
}

unsafe fn sof_select_ipc_and_paths(sdev: *mut snd_sof_dev) -> c_int {
    let plat_data = (*sdev).pdata;
    let base_profile = &mut (*plat_data).ipc_file_profile_base as *mut sof_loadable_file_profile;
    let mut out_profile: sof_loadable_file_profile = core::mem::zeroed();
    let dev = (*sdev).dev;
    let ret: c_int;

    if (*base_profile).ipc_type != (*(*plat_data).desc).ipc_default {
        dev_info(
            dev,
            c"Module parameter used, overriding default IPC %d to %d\n".as_ptr(),
            (*(*plat_data).desc).ipc_default,
            (*base_profile).ipc_type,
        );
    }

    if !(*base_profile).fw_path.is_null() {
        dev_dbg(dev, c"Module parameter used, changed fw path to %s\n".as_ptr(), (*base_profile).fw_path);
    } else if !(*base_profile).fw_path_postfix.is_null() {
        dev_dbg(dev, c"Path postfix appended to default fw path: %s\n".as_ptr(), (*base_profile).fw_path_postfix);
    }

    if !(*base_profile).fw_lib_path.is_null() {
        dev_dbg(dev, c"Module parameter used, changed fw_lib path to %s\n".as_ptr(), (*base_profile).fw_lib_path);
    } else if !(*base_profile).fw_lib_path_postfix.is_null() {
        dev_dbg(dev, c"Path postfix appended to default fw_lib path: %s\n".as_ptr(), (*base_profile).fw_lib_path_postfix);
    }

    if !(*base_profile).fw_name.is_null() {
        dev_dbg(dev, c"Module parameter used, changed fw filename to %s\n".as_ptr(), (*base_profile).fw_name);
    }

    if !(*base_profile).tplg_path.is_null() {
        dev_dbg(dev, c"Module parameter used, changed tplg path to %s\n".as_ptr(), (*base_profile).tplg_path);
    }

    if !(*base_profile).tplg_name.is_null() {
        dev_dbg(dev, c"Module parameter used, changed tplg name to %s\n".as_ptr(), (*base_profile).tplg_name);
    }

    ret = sof_create_ipc_file_profile(sdev, base_profile, &mut out_profile);
    if ret != 0 {
        return ret;
    }

    (*plat_data).ipc_type = out_profile.ipc_type;
    (*plat_data).fw_filename = out_profile.fw_name;
    (*plat_data).fw_filename_prefix = out_profile.fw_path;
    (*plat_data).fw_lib_prefix = out_profile.fw_lib_path;
    (*plat_data).tplg_filename_prefix = out_profile.tplg_path;

    0
}

unsafe fn validate_sof_ops(sdev: *mut snd_sof_dev) -> c_int {
    let ret: c_int;

    /* init ops, if necessary */
    ret = sof_ops_init(sdev);
    if ret < 0 {
        return ret;
    }

    /* check all mandatory ops */
    if sof_ops(sdev).is_null() || (*sof_ops(sdev)).probe.is_null() {
        dev_err((*sdev).dev, c"missing mandatory ops\n".as_ptr());
        sof_ops_free(sdev);
        return -EINVAL;
    }

    if !(*sdev).dspless_mode_selected
        && ((*sof_ops(sdev)).run.is_null()
            || (*sof_ops(sdev)).block_read.is_null()
            || (*sof_ops(sdev)).block_write.is_null()
            || (*sof_ops(sdev)).send_msg.is_null()
            || (*sof_ops(sdev)).load_firmware.is_null()
            || (*sof_ops(sdev)).ipc_msg_data.is_null())
    {
        dev_err((*sdev).dev, c"missing mandatory DSP ops\n".as_ptr());
        sof_ops_free(sdev);
        return -EINVAL;
    }

    0
}

unsafe fn sof_init_sof_ops(sdev: *mut snd_sof_dev) -> c_int {
    let plat_data = (*sdev).pdata;
    let base_profile = &mut (*plat_data).ipc_file_profile_base as *mut sof_loadable_file_profile;

    /* check IPC support */
    if (BIT((*base_profile).ipc_type) & (*(*plat_data).desc).ipc_supported_mask) == 0 {
        dev_err(
            (*sdev).dev,
            c"ipc_type %d is not supported on this platform, mask is %#x\n".as_ptr(),
            (*base_profile).ipc_type,
            (*(*plat_data).desc).ipc_supported_mask,
        );
        return -EINVAL;
    }

    /*
     * Save the selected IPC type and a topology name override before
     * selecting ops since platform code might need this information
     */
    (*plat_data).ipc_type = (*base_profile).ipc_type;
    (*plat_data).tplg_filename = (*base_profile).tplg_name;

    validate_sof_ops(sdev)
}

unsafe fn sof_init_environment(sdev: *mut snd_sof_dev) -> c_int {
    let plat_data = (*sdev).pdata;
    let base_profile = &mut (*plat_data).ipc_file_profile_base as *mut sof_loadable_file_profile;
    let mut ret: c_int;

    /* probe the DSP hardware */
    ret = snd_sof_probe(sdev);
    if ret < 0 {
        dev_err((*sdev).dev, c"failed to probe DSP %d\n".as_ptr(), ret);
        snd_sof_remove_late(sdev);
        sof_ops_free(sdev);
        return ret;
    }

    /* check machine info */
    ret = sof_machine_check(sdev);
    if ret < 0 {
        dev_err((*sdev).dev, c"failed to get machine info %d\n".as_ptr(), ret);
        snd_sof_remove(sdev);
        snd_sof_remove_late(sdev);
        sof_ops_free(sdev);
        return ret;
    }

    ret = sof_select_ipc_and_paths(sdev);
    if ret != 0 {
        snd_sof_remove(sdev);
        snd_sof_remove_late(sdev);
        sof_ops_free(sdev);
        return ret;
    } else if (*plat_data).ipc_type != (*base_profile).ipc_type {
        /* IPC type changed, re-initialize the ops */
        sof_ops_free(sdev);

        ret = validate_sof_ops(sdev);
        if ret < 0 {
            snd_sof_remove(sdev);
            snd_sof_remove_late(sdev);
            return ret;
        }
    }

    0
}

/*
 *			FW Boot State Transition Diagram
 *
 *    +----------------------------------------------------------------------+
 *    |									     |
 * ------------------	     ------------------				     |
 * |		    |	     |		      |				     |
 * |   BOOT_FAILED  |<-------|  READY_FAILED  |				     |
 * |		    |<--+    |	              |	   ------------------	     |
 * ------------------	|    ------------------	   |		    |	     |
 *	^		|	    ^		   |	CRASHED	    |---+    |
 *	|		|	    |		   |		    |	|    |
 * (FW Boot Timeout)	|	(FW_READY FAIL)	   ------------------	|    |
 *	|		|	    |		     ^			|    |
 *	|		|	    |		     |(DSP Panic)	|    |
 * ------------------	|	    |		   ------------------	|    |
 * |		    |	|	    |		   |		    |	|    |
 * |   IN_PROGRESS  |---------------+------------->|    COMPLETE    |	|    |
 * |		    | (FW Boot OK)   (FW_READY OK) |		    |	|    |
 * ------------------	|			   ------------------	|    |
 *	^		|				|		|    |
 *	|		|				|		|    |
 * (FW Loading OK)	|			(System Suspend/Runtime Suspend)
 *	|		|				|		|    |
 *	|	(FW Loading Fail)			|		|    |
 * ------------------	|	------------------	|		|    |
 * |		    |	|	|		 |<-----+		|    |
 * |   PREPARE	    |---+	|   NOT_STARTED  |<---------------------+    |
 * |		    |		|		 |<--------------------------+
 * ------------------		------------------
 *    |	    ^			    |	   ^
 *    |	    |			    |	   |
 *    |	    +-----------------------+	   |
 *    |		(DSP Probe OK)		   |
 *    |					   |
 *    |					   |
 *    +------------------------------------+
 *	(System Suspend/Runtime Suspend)
 */

unsafe fn sof_probe_continue(sdev: *mut snd_sof_dev) -> c_int {
    let plat_data = (*sdev).pdata;
    let mut ret: c_int;

    /* Initialize loadable file paths and check the environment validity */
    ret = sof_init_environment(sdev);
    if ret != 0 {
        return ret;
    }

    sof_set_fw_state(sdev, sof_fw_state::SOF_FW_BOOT_PREPARE);

    /* set up platform component driver */
    snd_sof_new_platform_drv(sdev);

    if (*sdev).dspless_mode_selected {
        sof_set_fw_state(sdev, sof_fw_state::SOF_DSPLESS_MODE);
        return sof_probe_continue_skip_dsp_init(sdev, plat_data);
    }

    /* register any debug/trace capabilities */
    ret = snd_sof_dbg_init(sdev);
    if ret < 0 {
        /*
         * debugfs issues are suppressed in snd_sof_dbg_init() since
         * we cannot rely on debugfs
         * here we trap errors due to memory allocation only.
         */
        dev_err((*sdev).dev, c"error: failed to init DSP trace/debug %d\n".as_ptr(), ret);
        return sof_probe_continue_error(sdev, ret, false, false, false, false);
    }

    /* init the IPC */
    (*sdev).ipc = snd_sof_ipc_init(sdev);
    if (*sdev).ipc.is_null() {
        ret = -ENOMEM;
        dev_err((*sdev).dev, c"error: failed to init DSP IPC %d\n".as_ptr(), ret);
        return sof_probe_continue_error(sdev, ret, false, false, false, true);
    }

    /* load the firmware */
    ret = snd_sof_load_firmware(sdev);
    if ret < 0 {
        dev_err((*sdev).dev, c"error: failed to load DSP firmware %d\n".as_ptr(), ret);
        sof_set_fw_state(sdev, sof_fw_state::SOF_FW_BOOT_FAILED);
        return sof_probe_continue_error(sdev, ret, false, false, true, true);
    }

    sof_set_fw_state(sdev, sof_fw_state::SOF_FW_BOOT_IN_PROGRESS);

    /*
     * Boot the firmware. The FW boot status will be modified
     * in snd_sof_run_firmware() depending on the outcome.
     */
    ret = snd_sof_run_firmware(sdev);
    if ret < 0 {
        dev_err((*sdev).dev, c"error: failed to boot DSP firmware %d\n".as_ptr(), ret);
        sof_set_fw_state(sdev, sof_fw_state::SOF_FW_BOOT_FAILED);
        return sof_probe_continue_error(sdev, ret, false, true, true, true);
    }

    if sof_debug_check_flag(SOF_DBG_ENABLE_TRACE) {
        (*sdev).fw_trace_is_supported = true;

        /* init firmware tracing */
        ret = sof_fw_trace_init(sdev);
        if ret < 0 {
            /* non fatal */
            dev_warn((*sdev).dev, c"failed to initialize firmware tracing %d\n".as_ptr(), ret);
        }
    } else {
        dev_dbg((*sdev).dev, c"SOF firmware trace disabled\n".as_ptr());
    }

    sof_probe_continue_skip_dsp_init(sdev, plat_data)
}

unsafe fn sof_probe_continue_skip_dsp_init(
    sdev: *mut snd_sof_dev,
    plat_data: *mut snd_sof_pdata,
) -> c_int {
    let mut ret: c_int;

    /* hereafter all FW boot flows are for PM reasons */
    (*sdev).first_boot = false;

    /* now register audio DSP platform driver and dai */
    ret = devm_snd_soc_register_component(
        (*sdev).dev,
        &mut (*sdev).plat_drv as *mut c_void,
        (*sof_ops(sdev)).drv,
        (*sof_ops(sdev)).num_drv,
    );
    if ret < 0 {
        dev_err((*sdev).dev, c"error: failed to register DSP DAI driver %d\n".as_ptr(), ret);
        return sof_probe_continue_error(sdev, ret, true, true, true, true);
    }

    ret = snd_sof_machine_register(sdev, plat_data);
    if ret < 0 {
        dev_err((*sdev).dev, c"error: failed to register machine driver %d\n".as_ptr(), ret);
        return sof_probe_continue_error(sdev, ret, true, true, true, true);
    }

    ret = sof_register_clients(sdev);
    if ret < 0 {
        dev_err((*sdev).dev, c"failed to register clients %d\n".as_ptr(), ret);
        snd_sof_machine_unregister(sdev, plat_data);
        return sof_probe_continue_error(sdev, ret, true, true, true, true);
    }

    /*
     * Some platforms in SOF, ex: BYT, may not have their platform PM
     * callbacks set. Increment the usage count so as to
     * prevent the device from entering runtime suspend.
     */
    if (*sof_ops(sdev)).runtime_suspend.is_null() || (*sof_ops(sdev)).runtime_resume.is_null() {
        pm_runtime_get_noresume((*sdev).dev);
    }

    if let Some(sof_probe_complete) = (*plat_data).sof_probe_complete {
        sof_probe_complete((*sdev).dev);
    }

    (*sdev).probe_completed = true;

    0
}

unsafe fn sof_probe_continue_error(
    sdev: *mut snd_sof_dev,
    ret: c_int,
    free_trace: bool,
    unload_fw: bool,
    free_ipc: bool,
    free_debug: bool,
) -> c_int {
    if free_trace {
        sof_fw_trace_free(sdev);
    }
    if unload_fw {
        snd_sof_fw_unload(sdev);
    }
    if free_ipc {
        snd_sof_ipc_free(sdev);
    }
    if free_debug {
        snd_sof_free_debug(sdev);
    }
    snd_sof_remove(sdev);
    snd_sof_remove_late(sdev);
    sof_ops_free(sdev);

    /* all resources freed, update state to match */
    sof_set_fw_state(sdev, sof_fw_state::SOF_FW_BOOT_NOT_STARTED);
    (*sdev).first_boot = true;

    ret
}

unsafe extern "C" fn sof_probe_work(work: *mut work_struct) {
    let sdev = (work as *mut u8).sub(core::mem::offset_of!(snd_sof_dev, probe_work))
        as *mut snd_sof_dev;
    let ret: c_int;

    ret = sof_probe_continue(sdev);
    if ret < 0 {
        /* errors cannot be propagated, log */
        dev_err((*sdev).dev, c"error: %s failed err: %d\n".as_ptr(), c"sof_probe_work".as_ptr(), ret);
    }
}

unsafe fn sof_apply_profile_override(
    path_override: *mut sof_loadable_file_profile,
    plat_data: *mut snd_sof_pdata,
) {
    if override_ipc_type >= 0 && override_ipc_type < SOF_IPC_TYPE_COUNT {
        (*path_override).ipc_type = override_ipc_type;
    }
    if !override_fw_path.is_null() {
        (*path_override).fw_path = override_fw_path;
    }
    if !override_fw_filename.is_null() {
        (*path_override).fw_name = override_fw_filename;
    }
    if !override_lib_path.is_null() {
        (*path_override).fw_lib_path = override_lib_path;
    }
    if !override_tplg_path.is_null() {
        (*path_override).tplg_path = override_tplg_path;
    }
    if !override_tplg_filename.is_null() {
        (*path_override).tplg_name = override_tplg_filename;
        /* User requested a specific topology file and expect it to be loaded */
        (*plat_data).disable_function_topology = true;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_device_probe(
    dev: *mut device,
    plat_data: *mut snd_sof_pdata,
) -> c_int {
    let sdev: *mut snd_sof_dev;
    let mut ret: c_int;

    sdev = devm_kzalloc(dev, size_of::<snd_sof_dev>(), GFP_KERNEL) as *mut snd_sof_dev;
    if sdev.is_null() {
        return -ENOMEM;
    }

    /* initialize sof device */
    (*sdev).dev = dev;

    /* initialize default DSP power state */
    (*sdev).dsp_power_state.state = SOF_DSP_PM_D0;

    (*sdev).pdata = plat_data;
    (*sdev).first_boot = true;
    dev_set_drvdata(dev, sdev as *mut c_void);

    if sof_core_debug != 0 {
        dev_info(dev, c"sof_debug value: %#x\n".as_ptr(), sof_core_debug);
    }

    if sof_debug_check_flag(SOF_DBG_DSPLESS_MODE) {
        if (*(*plat_data).desc).dspless_mode_supported {
            dev_info(dev, c"Switching to DSPless mode\n".as_ptr());
            (*sdev).dspless_mode_selected = true;
        } else {
            dev_info(dev, c"DSPless mode is not supported by the platform\n".as_ptr());
        }
    }

    sof_apply_profile_override(&mut (*plat_data).ipc_file_profile_base, plat_data);

    /* Initialize sof_ops based on the initial selected IPC version */
    ret = sof_init_sof_ops(sdev);
    if ret != 0 {
        return ret;
    }

    INIT_LIST_HEAD(&mut (*sdev).pcm_list);
    INIT_LIST_HEAD(&mut (*sdev).kcontrol_list);
    INIT_LIST_HEAD(&mut (*sdev).widget_list);
    INIT_LIST_HEAD(&mut (*sdev).pipeline_list);
    INIT_LIST_HEAD(&mut (*sdev).dai_list);
    INIT_LIST_HEAD(&mut (*sdev).dai_link_list);
    INIT_LIST_HEAD(&mut (*sdev).route_list);
    INIT_LIST_HEAD(&mut (*sdev).ipc_client_list);
    INIT_LIST_HEAD(&mut (*sdev).ipc_rx_handler_list);
    INIT_LIST_HEAD(&mut (*sdev).fw_state_handler_list);
    spin_lock_init(&mut (*sdev).ipc_lock);
    spin_lock_init(&mut (*sdev).hw_lock);
    mutex_init(&mut (*sdev).power_state_access);
    mutex_init(&mut (*sdev).ipc_client_mutex);
    mutex_init(&mut (*sdev).client_event_handler_mutex);
    mutex_init(&mut (*sdev).dsp_fw_boot_mutex);

    /* set default timeouts if none provided */
    if (*(*plat_data).desc).ipc_timeout == 0 {
        (*sdev).ipc_timeout = TIMEOUT_DEFAULT_IPC_MS;
    } else {
        (*sdev).ipc_timeout = (*(*plat_data).desc).ipc_timeout;
    }
    if (*(*plat_data).desc).boot_timeout == 0 {
        (*sdev).boot_timeout = TIMEOUT_DEFAULT_BOOT_MS;
    } else {
        (*sdev).boot_timeout = (*(*plat_data).desc).boot_timeout;
    }

    // #if IS_ENABLED(CONFIG_SND_SOC_SOF_DEBUG)
    /* Override the timeout values with module parameter, if set */
    if IS_ENABLED(CONFIG_SND_SOC_SOF_DEBUG) {
        if sof_ipc_timeout_ms != 0 {
            (*sdev).ipc_timeout = sof_ipc_timeout_ms;
        }

        if sof_boot_timeout_ms != 0 {
            (*sdev).boot_timeout = sof_boot_timeout_ms;
        }
    }
    // #endif

    sof_set_fw_state(sdev, sof_fw_state::SOF_FW_BOOT_NOT_STARTED);

    /*
     * first pass of probe which isn't allowed to run in a work-queue,
     * typically to rely on -EPROBE_DEFER dependencies
     */
    ret = snd_sof_probe_early(sdev);
    if ret < 0 {
        return ret;
    }

    if IS_ENABLED(CONFIG_SND_SOC_SOF_PROBE_WORK_QUEUE) {
        INIT_WORK(&mut (*sdev).probe_work, sof_probe_work);
        schedule_work(&mut (*sdev).probe_work);
        return 0;
    }

    sof_probe_continue(sdev)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_device_probe_completed(dev: *mut device) -> bool {
    let sdev = dev_get_drvdata(dev) as *mut snd_sof_dev;

    (*sdev).probe_completed
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_device_remove(dev: *mut device) -> c_int {
    let sdev = dev_get_drvdata(dev) as *mut snd_sof_dev;
    let pdata = (*sdev).pdata;
    let mut ret: c_int;
    let mut aborted = false;

    if IS_ENABLED(CONFIG_SND_SOC_SOF_PROBE_WORK_QUEUE) {
        aborted = cancel_work_sync(&mut (*sdev).probe_work);
    }

    /*
     * Unregister any registered client device first before IPC and debugfs
     * to allow client drivers to be removed cleanly
     */
    sof_unregister_clients(sdev);

    /*
     * Unregister machine driver. This will unbind the snd_card which
     * will remove the component driver and unload the topology
     * before freeing the snd_card.
     */
    snd_sof_machine_unregister(sdev, pdata);

    /*
     * Balance the runtime pm usage count in case we are faced with an
     * exception and we forcably prevented D3 power state to preserve
     * context
     */
    if (*sdev).d3_prevented {
        (*sdev).d3_prevented = false;
        pm_runtime_put_noidle((*sdev).dev);
    }

    if (*sdev).fw_state > sof_fw_state::SOF_FW_BOOT_NOT_STARTED {
        sof_fw_trace_free(sdev);
        ret = snd_sof_dsp_power_down_notify(sdev);
        if ret < 0 {
            dev_warn(
                dev,
                c"error: %d failed to prepare DSP for device removal".as_ptr(),
                ret,
            );
        }

        snd_sof_ipc_free(sdev);
        snd_sof_free_debug(sdev);
        snd_sof_remove(sdev);
        snd_sof_remove_late(sdev);
        sof_ops_free(sdev);
    } else if aborted {
        /* probe_work never ran */
        snd_sof_remove_late(sdev);
        sof_ops_free(sdev);
    }

    /* release firmware */
    snd_sof_fw_unload(sdev);

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_device_shutdown(dev: *mut device) -> c_int {
    let sdev = dev_get_drvdata(dev) as *mut snd_sof_dev;

    if IS_ENABLED(CONFIG_SND_SOC_SOF_PROBE_WORK_QUEUE) {
        cancel_work_sync(&mut (*sdev).probe_work);
    }

    if (*sdev).fw_state == sof_fw_state::SOF_FW_BOOT_COMPLETE {
        sof_fw_trace_free(sdev);
        return snd_sof_shutdown(sdev);
    }

    0
}

/* Machine driver registering and unregistering */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_machine_register(
    sdev: *mut snd_sof_dev,
    pdata: *mut c_void,
) -> c_int {
    let plat_data = pdata as *mut snd_sof_pdata;
    let drv_name: *const c_char;
    let mach: *const c_void;
    let size: c_int;

    drv_name = (*(*plat_data).machine).drv_name;
    mach = (*plat_data).machine as *const c_void;
    size = size_of::<snd_soc_acpi_mach>() as c_int;

    /* register machine driver, pass machine info as pdata */
    (*plat_data).pdev_mach = platform_device_register_data(
        (*sdev).dev,
        drv_name,
        PLATFORM_DEVID_NONE,
        mach,
        size as usize,
    );
    if IS_ERR((*plat_data).pdev_mach as *const c_void) {
        return PTR_ERR((*plat_data).pdev_mach as *const c_void);
    }

    dev_dbg(
        (*sdev).dev,
        c"created machine %s\n".as_ptr(),
        dev_name(&mut (*(*plat_data).pdev_mach).dev),
    );

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_machine_unregister(
    _sdev: *mut snd_sof_dev,
    pdata: *mut c_void,
) {
    let plat_data = pdata as *mut snd_sof_pdata;

    platform_device_unregister((*plat_data).pdev_mach);
}

// MODULE_AUTHOR("Liam Girdwood");
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("Sound Open Firmware (SOF) Core");
// MODULE_ALIAS("platform:sof-audio");
// MODULE_IMPORT_NS("SND_SOC_SOF_CLIENT");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
