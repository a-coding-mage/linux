// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//
// Generic firmware loader.
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

pub type ssize_t = isize;

pub const GFP_KERNEL: c_uint = 0;
pub const ENOMEM: c_int = 12;
pub const EIO: c_int = 5;

pub const SOF_DBG_DUMP_REGS: c_uint = 1 << 0;
pub const SOF_DBG_DUMP_MBOX: c_uint = 1 << 1;
pub const SOF_DBG_DUMP_TEXT: c_uint = 1 << 2;
pub const SOF_DBG_DUMP_PCI: c_uint = 1 << 3;

pub const SOF_FW_BOOT_IN_PROGRESS: c_int = 0;
pub const SOF_FW_BOOT_READY_FAILED: c_int = 1;
pub const SOF_FW_BOOT_COMPLETE: c_int = 2;

#[repr(C)]
pub struct firmware {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub fw_filename_prefix: *const c_char,
    pub fw_filename: *const c_char,
}

#[repr(C)]
pub struct snd_sof_fw {
    pub fw: *const firmware,
    pub payload_offset: ssize_t,
}

#[repr(C)]
pub struct snd_sof_fw_loader_ops {
    pub parse_ext_manifest: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> ssize_t>,
    pub validate: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub load_fw_to_dsp: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
}

#[repr(C)]
pub struct snd_sof_ipc_ops {
    pub fw_loader: *mut snd_sof_fw_loader_ops,
    pub post_fw_boot: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
}

#[repr(C)]
pub struct snd_sof_ipc {
    pub ops: *mut snd_sof_ipc_ops,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub pdata: *mut snd_sof_pdata,
    pub basefw: snd_sof_fw,
    pub dev: *mut device,
    pub ipc: *mut snd_sof_ipc,
    pub boot_wait: wait_queue_head_t,
    pub dbg_dump_printed: bool,
    pub ipc_dump_printed: bool,
    pub first_boot: bool,
    pub fw_version: c_uint,
    pub fw_state: c_int,
    pub boot_timeout: c_uint,
}

unsafe extern "C" {
    pub fn kasprintf(gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    pub fn kfree(ptr: *const c_void);

    pub fn request_firmware(
        firmware_p: *mut *const firmware,
        name: *const c_char,
        device: *mut device,
    ) -> c_int;
    pub fn release_firmware(fw: *const firmware);

    pub fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    pub fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);

    pub fn snd_sof_dsp_reset(sdev: *mut snd_sof_dev) -> c_int;
    pub fn snd_sof_debugfs_buf_item(
        sdev: *mut snd_sof_dev,
        buf: *mut c_void,
        size: usize,
        name: *const c_char,
        mode: c_uint,
    ) -> c_int;
    pub fn snd_sof_dsp_pre_fw_run(sdev: *mut snd_sof_dev) -> c_int;
    pub fn snd_sof_dsp_run(sdev: *mut snd_sof_dev) -> c_int;
    pub fn snd_sof_dsp_dbg_dump(sdev: *mut snd_sof_dev, msg: *const c_char, flags: c_uint);
    pub fn snd_sof_dsp_post_fw_run(sdev: *mut snd_sof_dev) -> c_int;
    pub fn sof_set_fw_state(sdev: *mut snd_sof_dev, state: c_int);

    pub fn init_waitqueue_head(wq_head: *mut wait_queue_head_t);
    pub fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    /*
     * wait_event_timeout is a C macro.  The declaration below preserves this
     * file-local call shape for the translated code; the actual Rust binding
     * should map to the kernel macro or equivalent helper supplied externally.
     */
    pub fn wait_event_timeout(
        wq_head: *mut wait_queue_head_t,
        condition: bool,
        timeout: c_ulong,
    ) -> c_long;
}

pub type c_long = isize;

// EXPORT_SYMBOL(snd_sof_load_firmware_raw);
#[no_mangle]
pub unsafe extern "C" fn snd_sof_load_firmware_raw(sdev: *mut snd_sof_dev) -> c_int {
    let plat_data: *mut snd_sof_pdata = (*sdev).pdata;
    let fw_filename: *const c_char;
    let ext_man_size: ssize_t;
    let mut ret: c_int;

    /* Don't request firmware again if firmware is already requested */
    if !(*sdev).basefw.fw.is_null() {
        return 0;
    }

    fw_filename = kasprintf(
        GFP_KERNEL,
        b"%s/%s\0".as_ptr() as *const c_char,
        (*plat_data).fw_filename_prefix,
        (*plat_data).fw_filename,
    );
    if fw_filename.is_null() {
        return -ENOMEM;
    }

    ret = request_firmware(
        &mut (*sdev).basefw.fw as *mut *const firmware,
        fw_filename,
        (*sdev).dev,
    );

    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"error: sof firmware file is missing, you might need to\n\0".as_ptr()
                as *const c_char,
        );
        dev_err(
            (*sdev).dev,
            b"       download it from https://github.com/thesofproject/sof-bin/\n\0".as_ptr()
                as *const c_char,
        );
        kfree(fw_filename as *const c_void);
        return ret;
    } else {
        dev_dbg(
            (*sdev).dev,
            b"request_firmware %s successful\n\0".as_ptr() as *const c_char,
            fw_filename,
        );
    }

    /* check for extended manifest */
    ext_man_size =
        ((*(*(*sdev).ipc).ops).fw_loader).as_mut().unwrap().parse_ext_manifest.unwrap())(sdev);
    if ext_man_size > 0 {
        /* when no error occurred, drop extended manifest */
        (*sdev).basefw.payload_offset = ext_man_size;
    } else if ext_man_size == 0 {
        /* No extended manifest, so nothing to skip during FW load */
        dev_dbg(
            (*sdev).dev,
            b"firmware doesn't contain extended manifest\n\0".as_ptr() as *const c_char,
        );
    } else {
        ret = ext_man_size as c_int;
        dev_err(
            (*sdev).dev,
            b"error: firmware %s contains unsupported or invalid extended manifest: %d\n\0"
                .as_ptr() as *const c_char,
            fw_filename,
            ret,
        );
    }

    kfree(fw_filename as *const c_void);

    ret
}

// EXPORT_SYMBOL(snd_sof_load_firmware_memcpy);
#[no_mangle]
pub unsafe extern "C" fn snd_sof_load_firmware_memcpy(sdev: *mut snd_sof_dev) -> c_int {
    let mut ret: c_int;

    ret = snd_sof_load_firmware_raw(sdev);
    if ret < 0 {
        return ret;
    }

    /* make sure the FW header and file is valid */
    ret = ((*(*(*sdev).ipc).ops).fw_loader).as_mut().unwrap().validate.unwrap())(sdev);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"error: invalid FW header\n\0".as_ptr() as *const c_char,
        );
        release_firmware((*sdev).basefw.fw);
        (*sdev).basefw.fw = ptr::null();
        return ret;
    }

    /* prepare the DSP for FW loading */
    ret = snd_sof_dsp_reset(sdev);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"error: failed to reset DSP\n\0".as_ptr() as *const c_char,
        );
        release_firmware((*sdev).basefw.fw);
        (*sdev).basefw.fw = ptr::null();
        return ret;
    }

    /* parse and load firmware modules to DSP */
    if (*((*(*sdev).ipc).ops).fw_loader)
        .load_fw_to_dsp
        .is_some()
    {
        ret = (*((*(*sdev).ipc).ops).fw_loader)
            .load_fw_to_dsp
            .unwrap()(sdev);
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                b"Firmware loading failed\n\0".as_ptr() as *const c_char,
            );
            release_firmware((*sdev).basefw.fw);
            (*sdev).basefw.fw = ptr::null();
            return ret;
        }
    }

    0
}

// EXPORT_SYMBOL(snd_sof_run_firmware);
#[no_mangle]
pub unsafe extern "C" fn snd_sof_run_firmware(sdev: *mut snd_sof_dev) -> c_int {
    let mut ret: c_int;

    init_waitqueue_head(&mut (*sdev).boot_wait);

    /* (re-)enable dsp dump */
    (*sdev).dbg_dump_printed = false;
    (*sdev).ipc_dump_printed = false;

    /* create read-only fw_version debugfs to store boot version info */
    if (*sdev).first_boot {
        ret = snd_sof_debugfs_buf_item(
            sdev,
            &mut (*sdev).fw_version as *mut c_uint as *mut c_void,
            size_of::<c_uint>(),
            b"fw_version\0".as_ptr() as *const c_char,
            0o444,
        );
        /* errors are only due to memory allocation, not debugfs */
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                b"snd_sof_debugfs_buf_item failed\n\0".as_ptr() as *const c_char,
            );
            return ret;
        }
    }

    /* perform pre fw run operations */
    ret = snd_sof_dsp_pre_fw_run(sdev);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"failed pre fw run op\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    dev_dbg(
        (*sdev).dev,
        b"booting DSP firmware\n\0".as_ptr() as *const c_char,
    );

    /* boot the firmware on the DSP */
    ret = snd_sof_dsp_run(sdev);
    if ret < 0 {
        snd_sof_dsp_dbg_dump(
            sdev,
            b"Failed to start DSP\0".as_ptr() as *const c_char,
            SOF_DBG_DUMP_MBOX | SOF_DBG_DUMP_PCI,
        );
        return ret;
    }

    /*
     * now wait for the DSP to boot. There are 3 possible outcomes:
     * 1. Boot wait times out indicating FW boot failure.
     * 2. FW boots successfully and fw_ready op succeeds.
     * 3. FW boots but fw_ready op fails.
     */
    ret = wait_event_timeout(
        &mut (*sdev).boot_wait,
        (*sdev).fw_state > SOF_FW_BOOT_IN_PROGRESS,
        msecs_to_jiffies((*sdev).boot_timeout),
    ) as c_int;
    if ret == 0 {
        snd_sof_dsp_dbg_dump(
            sdev,
            b"Firmware boot failure due to timeout\0".as_ptr() as *const c_char,
            SOF_DBG_DUMP_REGS | SOF_DBG_DUMP_MBOX | SOF_DBG_DUMP_TEXT | SOF_DBG_DUMP_PCI,
        );
        return -EIO;
    }

    if (*sdev).fw_state == SOF_FW_BOOT_READY_FAILED {
        return -EIO; /* FW boots but fw_ready op failed */
    }

    dev_dbg(
        (*sdev).dev,
        b"firmware boot complete\n\0".as_ptr() as *const c_char,
    );
    sof_set_fw_state(sdev, SOF_FW_BOOT_COMPLETE);

    /* perform post fw run operations */
    ret = snd_sof_dsp_post_fw_run(sdev);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"error: failed post fw run op\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    if (*(*(*sdev).ipc).ops).post_fw_boot.is_some() {
        return (*(*(*sdev).ipc).ops).post_fw_boot.unwrap()(sdev);
    }

    0
}

// EXPORT_SYMBOL(snd_sof_fw_unload);
#[no_mangle]
pub unsafe extern "C" fn snd_sof_fw_unload(sdev: *mut snd_sof_dev) {
    /* TODO: support module unloading at runtime */
    release_firmware((*sdev).basefw.fw);
    (*sdev).basefw.fw = ptr::null();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
