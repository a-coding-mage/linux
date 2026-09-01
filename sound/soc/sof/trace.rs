// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2022 Intel Corporation

// C dependency: #include "sof-priv.h"

use core::ffi::{c_char, c_int, c_void};

pub type pm_message_t = c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub fw_trace_is_supported: bool,
    pub ipc: *mut snd_sof_ipc,
}

#[repr(C)]
pub struct snd_sof_ipc {
    pub ops: *mut sof_ipc_ops,
}

#[repr(C)]
pub struct sof_ipc_ops {
    pub fw_tracing: *const sof_ipc_fw_tracing_ops,
}

#[repr(C)]
pub struct sof_ipc_fw_tracing_ops {
    pub init: unsafe extern "C" fn(*mut snd_sof_dev) -> c_int,
    pub r#free: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub fw_crashed: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub suspend: unsafe extern "C" fn(*mut snd_sof_dev, pm_message_t),
    pub resume: unsafe extern "C" fn(*mut snd_sof_dev) -> c_int,
}

unsafe extern "C" {
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);

    // C source uses: sof_ipc_get_ops(sdev, fw_tracing)
    fn sof_ipc_get_ops_fw_tracing(sdev: *mut snd_sof_dev) -> *const sof_ipc_fw_tracing_ops;
}

#[no_mangle]
pub unsafe extern "C" fn sof_fw_trace_init(sdev: *mut snd_sof_dev) -> c_int {
    let fw_tracing = unsafe { sof_ipc_get_ops_fw_tracing(sdev) };

    if fw_tracing.is_null() {
        unsafe {
            dev_info(
                (*sdev).dev,
                c"Firmware tracing is not available\n".as_ptr(),
            );
            (*sdev).fw_trace_is_supported = false;
        }

        return 0;
    }

    unsafe { ((*fw_tracing).init)(sdev) }
}

#[no_mangle]
pub unsafe extern "C" fn sof_fw_trace_free(sdev: *mut snd_sof_dev) {
    if unsafe { !(*sdev).fw_trace_is_supported } {
        return;
    }

    let free = unsafe { (*(*(*sdev).ipc).ops).fw_tracing.as_ref().unwrap().r#free };
    if let Some(free) = free {
        unsafe { free(sdev) };
    }
}

#[no_mangle]
pub unsafe extern "C" fn sof_fw_trace_fw_crashed(sdev: *mut snd_sof_dev) {
    if unsafe { !(*sdev).fw_trace_is_supported } {
        return;
    }

    let fw_crashed = unsafe { (*(*(*sdev).ipc).ops).fw_tracing.as_ref().unwrap().fw_crashed };
    if let Some(fw_crashed) = fw_crashed {
        unsafe { fw_crashed(sdev) };
    }
}

#[no_mangle]
pub unsafe extern "C" fn sof_fw_trace_suspend(sdev: *mut snd_sof_dev, pm_state: pm_message_t) {
    if unsafe { !(*sdev).fw_trace_is_supported } {
        return;
    }

    unsafe { ((*(*(*(*sdev).ipc).ops).fw_tracing).suspend)(sdev, pm_state) };
}

#[no_mangle]
pub unsafe extern "C" fn sof_fw_trace_resume(sdev: *mut snd_sof_dev) -> c_int {
    if unsafe { !(*sdev).fw_trace_is_supported } {
        return 0;
    }

    unsafe { ((*(*(*(*sdev).ipc).ops).fw_tracing).resume)(sdev) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
