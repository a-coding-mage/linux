// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022 Intel Corporation

// Rust translation of implementation source: soc/sof/ipc4-loader.c
// Dependencies originally provided by:
// <linux/firmware.h>, <sound/sof/ext_manifest4.h>,
// <sound/sof/ipc4/header.h>, <trace/events/sof.h>,
// "ipc4-priv.h", "sof-audio.h", "sof-priv.h", "ops.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type ssize_t = isize;
type size_t = usize;
type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type bool_ = bool;

const SOF_IPC4_MOD_LIB_ID_SHIFT: u32 = 12;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EOPNOTSUPP: c_int = 95;
const GFP_KERNEL: c_uint = 0;

extern "C" {
    static SOF_EXT_MAN4_MAGIC_NUMBER: u32;
    static SOF_IPC4_MODULE_MSG: u32;
    static SOF_IPC4_MSG_REQUEST: u32;
    static SOF_IPC4_MOD_INIT_BASEFW_MOD_ID: u32;
    static SOF_IPC4_MOD_INIT_BASEFW_INSTANCE_ID: u32;
    static SOF_IPC4_FW_PARAM_FW_CONFIG: u32;
    static SOF_IPC4_FW_PARAM_HW_CONFIG_GET: u32;
    static SOF_IPC4_FW_CFG_FW_VERSION: u32;
    static SOF_IPC4_FW_CFG_DL_MAILBOX_BYTES: u32;
    static SOF_IPC4_FW_CFG_UL_MAILBOX_BYTES: u32;
    static SOF_IPC4_FW_CFG_TRACE_LOG_BYTES: u32;
    static SOF_IPC4_FW_CFG_MAX_LIBS_COUNT: u32;
    static SOF_IPC4_FW_CFG_MAX_PPL_COUNT: u32;
    static SOF_IPC4_FW_CONTEXT_SAVE: u32;
    static SOF_IPC4_HW_CFG_INTEL_MIC_PRIVACY_CAPS: u32;
}

extern "C" {
    fn SOF_IPC4_MSG_TARGET(x: u32) -> u32;
    fn SOF_IPC4_MSG_DIR(x: u32) -> u32;
    fn SOF_IPC4_MOD_ID(x: u32) -> u32;
    fn SOF_IPC4_MOD_INSTANCE(x: u32) -> u32;
    fn SOF_IPC4_MOD_EXT_MSG_PARAM_ID(x: u32) -> u32;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);

    fn devm_kmalloc_array(dev: *mut device, n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kfree(dev: *mut device, p: *mut c_void);
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(p: *const c_void);
    fn kasprintf(flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: size_t) -> ssize_t;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;

    fn firmware_request_nowarn(fw: *mut *const firmware, name: *const c_char, device: *mut device) -> c_int;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, device: *mut device) -> c_int;
    fn release_firmware(fw: *const firmware);

    fn xa_insert(xa: *mut xarray, index: c_ulong, entry: *mut c_void, flags: c_uint) -> c_int;
    fn ida_init(ida: *mut ida);
    fn guid_is_null(guid: *const guid_t) -> bool_;
    fn guid_equal(a: *const guid_t, b: *const guid_t) -> bool_;
    fn trace_sof_ipc4_fw_config(sdev: *mut snd_sof_dev, name: *const c_char, value: u32);
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ida {
    _private: [u8; 0],
}

#[repr(C)]
pub struct guid_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct firmware {
    pub size: size_t,
    pub data: *const u8,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub fw_filename: *const c_char,
    pub fw_filename_prefix: *const c_char,
    pub fw_lib_prefix: *const c_char,
}

#[repr(C)]
pub struct sof_ipc_ops {
    pub set_get_data:
        Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut sof_ipc4_msg, size_t, bool_) -> c_int>,
}

#[repr(C)]
pub struct snd_sof_ipc {
    pub ops: *const sof_ipc_ops,
    pub max_payload_size: size_t,
}

#[repr(C)]
pub struct sof_fw {
    pub fw: *const firmware,
    pub payload_offset: ssize_t,
}

#[repr(C)]
pub struct sof_ipc4_fw_version {
    pub major: u32,
    pub minor: u32,
    pub hotfix: u32,
    pub build: u32,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub private: *mut c_void,
    pub basefw: sof_fw,
    pub fw_version: sof_ipc4_fw_version,
    pub pdata: *mut snd_sof_pdata,
    pub ipc: *mut snd_sof_ipc,
}

#[repr(C)]
pub struct sof_ext_manifest4_hdr {
    pub id: u32,
    pub len: u32,
}

#[repr(C)]
pub struct sof_man4_fw_binary_header {
    pub len: u32,
    pub name: *const c_char,
    pub major_version: u32,
    pub minor_version: u32,
    pub hotfix_version: u32,
    pub build_version: u32,
    pub num_module_entries: u32,
}

#[repr(C)]
pub struct sof_man4_module {
    pub name: *const c_char,
    pub uuid: guid_t,
    pub id: u32,
    pub cfg_count: u32,
    pub cfg_offset: u32,
}

#[repr(C)]
pub struct sof_man4_module_config {
    pub is_bytes: u32,
    pub obs: u32,
    pub ibs: u32,
    pub cpc: u32,
}

#[repr(C)]
pub struct sof_ipc4_fw_module {
    pub man4_module_entry: sof_man4_module,
    pub fw_mod_cfg: *const sof_man4_module_config,
    pub m_ida: ida,
    pub private: *mut c_void,
}

#[repr(C)]
pub struct sof_ipc4_fw_library {
    pub sof_fw: sof_fw,
    pub modules: *mut sof_ipc4_fw_module,
    pub name: *const c_char,
    pub num_modules: u32,
    pub id: c_ulong,
}

#[repr(C)]
pub struct sof_ipc4_fw_data {
    pub manifest_fw_hdr_offset: u32,
    pub fw_lib_xa: xarray,
    pub load_library:
        Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut sof_ipc4_fw_library, bool_) -> c_int>,
    pub max_libs_count: u32,
    pub mtrace_log_bytes: u32,
    pub max_num_pipelines: c_int,
    pub fw_context_save: u32,
    pub libraries_restored: u32,
    pub intel_configure_mic_privacy:
        Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut sof_ipc4_intel_mic_privacy_cap)>,
}

#[repr(C)]
pub struct sof_ipc4_tuple {
    pub type_: u32,
    pub size: u32,
    pub value: [u32; 0],
}

#[repr(C)]
pub struct sof_ipc4_msg {
    pub primary: u32,
    pub extension: u32,
    pub data_size: size_t,
    pub data_ptr: *mut c_void,
}

#[repr(C)]
pub struct sof_ipc4_intel_mic_privacy_cap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc4_base_module_cfg {
    pub obs: u32,
    pub ibs: u32,
    pub cpc: u32,
}

#[repr(C)]
pub struct sof_ipc_fw_loader_ops {
    pub validate: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub parse_ext_manifest: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> size_t>,
}

unsafe fn sof_ipc4_fw_parse_ext_man(
    sdev: *mut snd_sof_dev,
    fw_lib: *mut sof_ipc4_fw_library,
) -> ssize_t {
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let fw = (*fw_lib).sof_fw.fw;
    let mut fw_header: *mut sof_man4_fw_binary_header;
    let ext_man_hdr: *mut sof_ext_manifest4_hdr;
    let mut fm_config: *mut sof_man4_module_config;
    let mut fw_module: *mut sof_ipc4_fw_module;
    let mut fm_entry: *mut sof_man4_module;
    let mut remaining: ssize_t;
    let fw_hdr_offset: u32;
    let mut i: c_int;

    if ipc4_data.is_null() {
        dev_err((*sdev).dev, c"%s: ipc4_data is not available\n".as_ptr(), c"sof_ipc4_fw_parse_ext_man".as_ptr());
        return -(EINVAL as ssize_t);
    }

    remaining = (*fw).size as ssize_t;
    if remaining <= size_of::<sof_ext_manifest4_hdr>() as ssize_t {
        dev_err((*sdev).dev, c"Firmware size is too small: %zu\n".as_ptr(), remaining as size_t);
        return -(EINVAL as ssize_t);
    }

    ext_man_hdr = (*fw).data as *mut sof_ext_manifest4_hdr;

    /*
     * At the start of the firmware image we must have an extended manifest.
     * Verify that the magic number is correct.
     */
    if (*ext_man_hdr).id != SOF_EXT_MAN4_MAGIC_NUMBER {
        dev_err(
            (*sdev).dev,
            c"Unexpected extended manifest magic number: %#x\n".as_ptr(),
            (*ext_man_hdr).id,
        );
        return -(EINVAL as ssize_t);
    }

    fw_hdr_offset = (*ipc4_data).manifest_fw_hdr_offset;
    if fw_hdr_offset == 0 {
        return -(EINVAL as ssize_t);
    }

    if remaining
        <= ((*ext_man_hdr).len as ssize_t
            + fw_hdr_offset as ssize_t
            + size_of::<sof_man4_fw_binary_header>() as ssize_t)
    {
        dev_err(
            (*sdev).dev,
            c"Invalid firmware size %zu, should be at least %zu\n".as_ptr(),
            remaining as size_t,
            ((*ext_man_hdr).len as size_t + fw_hdr_offset as size_t + size_of::<sof_man4_fw_binary_header>()),
        );
        return -(EINVAL as ssize_t);
    }

    fw_header = (*fw)
        .data
        .add((*ext_man_hdr).len as usize + fw_hdr_offset as usize)
        as *mut sof_man4_fw_binary_header;
    remaining -= (*ext_man_hdr).len as ssize_t + fw_hdr_offset as ssize_t;

    if remaining <= (*fw_header).len as ssize_t {
        dev_err((*sdev).dev, c"Invalid fw_header->len %u\n".as_ptr(), (*fw_header).len);
        return -(EINVAL as ssize_t);
    }

    dev_info(
        (*sdev).dev,
        c"Loaded firmware library: %s, version: %u.%u.%u.%u\n".as_ptr(),
        (*fw_header).name,
        (*fw_header).major_version,
        (*fw_header).minor_version,
        (*fw_header).hotfix_version,
        (*fw_header).build_version,
    );
    dev_dbg(
        (*sdev).dev,
        c"Header length: %u, module count: %u\n".as_ptr(),
        (*fw_header).len,
        (*fw_header).num_module_entries,
    );

    /* copy the fw_version of basefw into debugfs at first boot */
    if fw == (*sdev).basefw.fw {
        (*sdev).fw_version.major = (*fw_header).major_version;
        (*sdev).fw_version.minor = (*fw_header).minor_version;
        (*sdev).fw_version.hotfix = (*fw_header).hotfix_version;
        (*sdev).fw_version.build = (*fw_header).build_version;
    }

    (*fw_lib).modules = devm_kmalloc_array(
        (*sdev).dev,
        (*fw_header).num_module_entries as size_t,
        size_of::<sof_ipc4_fw_module>(),
        GFP_KERNEL,
    ) as *mut sof_ipc4_fw_module;
    if (*fw_lib).modules.is_null() {
        return -(ENOMEM as ssize_t);
    }

    (*fw_lib).name = (*fw_header).name;
    (*fw_lib).num_modules = (*fw_header).num_module_entries;
    fw_module = (*fw_lib).modules;

    fm_entry = (fw_header as *mut u8).add((*fw_header).len as usize) as *mut sof_man4_module;
    remaining -= (*fw_header).len as ssize_t;

    if remaining
        < ((*fw_header).num_module_entries as ssize_t * size_of::<sof_man4_module>() as ssize_t)
    {
        dev_err(
            (*sdev).dev,
            c"Invalid num_module_entries %u\n".as_ptr(),
            (*fw_header).num_module_entries,
        );
        return -(EINVAL as ssize_t);
    }

    fm_config = fm_entry.add((*fw_header).num_module_entries as usize) as *mut sof_man4_module_config;
    remaining -= (*fw_header).num_module_entries as ssize_t * size_of::<sof_man4_module>() as ssize_t;
    i = 0;
    while i < (*fw_header).num_module_entries as c_int {
        memcpy(
            &mut (*fw_module).man4_module_entry as *mut _ as *mut c_void,
            fm_entry as *const c_void,
            size_of::<sof_man4_module>(),
        );

        if (*fm_entry).cfg_count != 0 {
            if remaining
                < (((*fm_entry).cfg_offset + (*fm_entry).cfg_count) as ssize_t
                    * size_of::<sof_man4_module_config>() as ssize_t)
            {
                dev_err(
                    (*sdev).dev,
                    c"Invalid module cfg_offset %u\n".as_ptr(),
                    (*fm_entry).cfg_offset,
                );
                return -(EINVAL as ssize_t);
            }

            (*fw_module).fw_mod_cfg = fm_config.add((*fm_entry).cfg_offset as usize);

            dev_dbg(
                (*sdev).dev,
                c"module %s: UUID %pUL cfg_count: %u, bss_size: %#x\n".as_ptr(),
                (*fm_entry).name,
                &(*fm_entry).uuid as *const guid_t,
                (*fm_entry).cfg_count,
                (*fm_config.add((*fm_entry).cfg_offset as usize)).is_bytes,
            );
        } else {
            dev_dbg(
                (*sdev).dev,
                c"module %s: UUID %pUL\n".as_ptr(),
                (*fm_entry).name,
                &(*fm_entry).uuid as *const guid_t,
            );
        }

        (*fw_module).man4_module_entry.id = i as u32;
        ida_init(&mut (*fw_module).m_ida);
        (*fw_module).private = ptr::null_mut();

        fw_module = fw_module.add(1);
        fm_entry = fm_entry.add(1);
        i += 1;
    }

    (*ext_man_hdr).len as ssize_t
}

unsafe fn sof_ipc4_fw_parse_basefw_ext_man(sdev: *mut snd_sof_dev) -> size_t {
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let fw_lib: *mut sof_ipc4_fw_library;
    let payload_offset: ssize_t;
    let mut ret: c_int;

    fw_lib = devm_kzalloc((*sdev).dev, size_of::<sof_ipc4_fw_library>(), GFP_KERNEL)
        as *mut sof_ipc4_fw_library;
    if fw_lib.is_null() {
        return (-(ENOMEM as ssize_t)) as size_t;
    }

    (*fw_lib).sof_fw.fw = (*sdev).basefw.fw;

    payload_offset = sof_ipc4_fw_parse_ext_man(sdev, fw_lib);
    if payload_offset > 0 {
        (*fw_lib).sof_fw.payload_offset = payload_offset;

        /* basefw ID is 0 */
        (*fw_lib).id = 0;
        ret = xa_insert(&mut (*ipc4_data).fw_lib_xa, 0, fw_lib as *mut c_void, GFP_KERNEL);
        if ret != 0 {
            return ret as size_t;
        }
    }

    payload_offset as size_t
}

unsafe fn sof_ipc4_load_library(
    sdev: *mut snd_sof_dev,
    lib_id: c_ulong,
    lib_filename: *const c_char,
    optional: bool_,
) -> c_int {
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let fw_lib: *mut sof_ipc4_fw_library;
    let payload_offset: ssize_t;
    let mut ret: c_int;
    let mut i: c_int;

    if (*ipc4_data).load_library.is_none() {
        dev_err((*sdev).dev, c"Library loading is not supported on this platform\n".as_ptr());
        return -EOPNOTSUPP;
    }

    fw_lib = devm_kzalloc((*sdev).dev, size_of::<sof_ipc4_fw_library>(), GFP_KERNEL)
        as *mut sof_ipc4_fw_library;
    if fw_lib.is_null() {
        return -ENOMEM;
    }

    if optional {
        ret = firmware_request_nowarn(&mut (*fw_lib).sof_fw.fw, lib_filename, (*sdev).dev);
        if ret < 0 {
            /* optional library, override the error */
            ret = 0;
            devm_kfree((*sdev).dev, fw_lib as *mut c_void);
            return ret;
        }
    } else {
        ret = request_firmware(&mut (*fw_lib).sof_fw.fw, lib_filename, (*sdev).dev);
        if ret < 0 {
            dev_err((*sdev).dev, c"Library file '%s' is missing\n".as_ptr(), lib_filename);
            devm_kfree((*sdev).dev, fw_lib as *mut c_void);
            return ret;
        }
    }

    dev_dbg((*sdev).dev, c"Library file '%s' loaded\n".as_ptr(), lib_filename);

    payload_offset = sof_ipc4_fw_parse_ext_man(sdev, fw_lib);
    if payload_offset <= 0 {
        if payload_offset == 0 {
            ret = -EINVAL;
        } else {
            ret = payload_offset as c_int;
        }

        release_firmware((*fw_lib).sof_fw.fw);
        /* Allocated within sof_ipc4_fw_parse_ext_man() */
        devm_kfree((*sdev).dev, (*fw_lib).modules as *mut c_void);
        devm_kfree((*sdev).dev, fw_lib as *mut c_void);
        return ret;
    }

    (*fw_lib).sof_fw.payload_offset = payload_offset;
    (*fw_lib).id = lib_id;

    /* Fix up the module ID numbers within the library */
    i = 0;
    while i < (*fw_lib).num_modules as c_int {
        (*(*fw_lib).modules.add(i as usize)).man4_module_entry.id |=
            ((lib_id as u32) << SOF_IPC4_MOD_LIB_ID_SHIFT) as u32;
        i += 1;
    }

    ret = (*ipc4_data).load_library.unwrap()(sdev, fw_lib, false);
    if ret != 0 {
        release_firmware((*fw_lib).sof_fw.fw);
        /* Allocated within sof_ipc4_fw_parse_ext_man() */
        devm_kfree((*sdev).dev, (*fw_lib).modules as *mut c_void);
        devm_kfree((*sdev).dev, fw_lib as *mut c_void);
        return ret;
    }

    ret = xa_insert(&mut (*ipc4_data).fw_lib_xa, lib_id, fw_lib as *mut c_void, GFP_KERNEL);
    if ret != 0 {
        release_firmware((*fw_lib).sof_fw.fw);
        /* Allocated within sof_ipc4_fw_parse_ext_man() */
        devm_kfree((*sdev).dev, (*fw_lib).modules as *mut c_void);
        devm_kfree((*sdev).dev, fw_lib as *mut c_void);
        return ret;
    }

    0
}

/**
 * sof_ipc4_complete_split_release - loads the library parts of a split firmware
 * @sdev: SOF device
 *
 * With IPC4 the firmware can be a single binary or a split release.
 * - single binary: only the basefw
 * - split release: basefw and two libraries (openmodules, debug)
 *
 * With split firmware release it is also allowed that for example only the
 * debug library is present (the openmodules content is built in the basefw).
 *
 * To handle the permutations try to load the openmodules then the debug
 * libraries as optional ones after the basefw boot.
 *
 * The libraries for the split release are stored alongside the basefw on the
 * filesystem.
 */
#[no_mangle]
pub unsafe extern "C" fn sof_ipc4_complete_split_release(sdev: *mut snd_sof_dev) -> c_int {
    static lib_bundle: [*const c_char; 2] = [c"openmodules".as_ptr(), c"debug".as_ptr()];
    let fw_filename = (*(*sdev).pdata).fw_filename;
    let mut lib_filename: *mut c_char;
    let p: *mut c_char;
    let lib_name_base_size: size_t;
    let mut lib_id: c_ulong = 1;
    let lib_name_base: *mut c_char;
    let mut i: c_int;

    p = strstr(fw_filename, c".ri".as_ptr());
    if p.is_null() || strlen(p) != 3 {
        dev_info(
            (*sdev).dev,
            c"%s: Firmware name '%s' is missing .ri extension\n".as_ptr(),
            c"sof_ipc4_complete_split_release".as_ptr(),
            fw_filename,
        );
        return 0;
    }

    /* Space for the firmware basename + '\0', without the extension */
    lib_name_base_size = strlen(fw_filename) - 2;
    lib_name_base = kzalloc(lib_name_base_size, GFP_KERNEL) as *mut c_char;
    if lib_name_base.is_null() {
        return -ENOMEM;
    }

    /*
     * strscpy will 0 terminate the copied string, removing the '.ri' from
     * the end of the fw_filename, for example:
     * fw_filename:		"sof-ptl.ri\0"
     * lib_name_base:	"sof-ptl\0"
     */
    strscpy(lib_name_base, fw_filename, lib_name_base_size);

    i = 0;
    while i < lib_bundle.len() as c_int {
        let ret: c_int;

        lib_filename = kasprintf(
            GFP_KERNEL,
            c"%s/%s-%s.ri".as_ptr(),
            (*(*sdev).pdata).fw_filename_prefix,
            lib_name_base,
            lib_bundle[i as usize],
        );
        if lib_filename.is_null() {
            kfree(lib_name_base as *const c_void);
            return -ENOMEM;
        }

        ret = sof_ipc4_load_library(sdev, lib_id, lib_filename, true);
        if ret != 0 {
            dev_warn(
                (*sdev).dev,
                c"%s: Failed to load %s: %d\n".as_ptr(),
                c"sof_ipc4_complete_split_release".as_ptr(),
                lib_filename,
                ret,
            );
        } else {
            lib_id += 1;
        }

        kfree(lib_filename as *const c_void);
        i += 1;
    }

    kfree(lib_name_base as *const c_void);

    0
}

unsafe fn sof_ipc4_load_library_by_uuid(
    sdev: *mut snd_sof_dev,
    lib_id: c_ulong,
    uuid: *const guid_t,
) -> c_int {
    let lib_filename: *mut c_char;
    let ret: c_int;

    if (*(*sdev).pdata).fw_lib_prefix.is_null() {
        dev_err(
            (*sdev).dev,
            c"Library loading is not supported due to not set library path\n".as_ptr(),
        );
        return -EINVAL;
    }

    lib_filename = kasprintf(GFP_KERNEL, c"%s/%pUL.bin".as_ptr(), (*(*sdev).pdata).fw_lib_prefix, uuid);
    if lib_filename.is_null() {
        return -ENOMEM;
    }

    ret = sof_ipc4_load_library(sdev, lib_id, lib_filename, false);

    kfree(lib_filename as *const c_void);

    ret
}

// xa_for_each/xa_for_each_start are C iteration macros. Their Rust form is kept
// as external iterator helpers supplied by the translated xarray dependency.
extern "C" {
    fn xa_for_each_next(
        xa: *mut xarray,
        index: *mut c_ulong,
        entry: *mut *mut sof_ipc4_fw_library,
        start: c_ulong,
    ) -> bool_;
}

#[no_mangle]
pub unsafe extern "C" fn sof_ipc4_find_module_by_uuid(
    sdev: *mut snd_sof_dev,
    uuid: *const guid_t,
) -> *mut sof_ipc4_fw_module {
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let mut fw_lib: *mut sof_ipc4_fw_library = ptr::null_mut();
    let mut lib_id: c_ulong = 0;
    let mut i: c_int;
    let mut ret: c_int;

    if guid_is_null(uuid) {
        return ptr::null_mut();
    }

    while xa_for_each_next(&mut (*ipc4_data).fw_lib_xa, &mut lib_id, &mut fw_lib, 0) {
        i = 0;
        while i < (*fw_lib).num_modules as c_int {
            if guid_equal(
                uuid,
                &(*(*fw_lib).modules.add(i as usize)).man4_module_entry.uuid as *const guid_t,
            ) {
                return (*fw_lib).modules.add(i as usize);
            }
            i += 1;
        }
    }

    /*
     * Do not attempt to load external library in case the maximum number of
     * firmware libraries have been already loaded
     */
    if (lib_id + 1) == (*ipc4_data).max_libs_count as c_ulong {
        dev_err(
            (*sdev).dev,
            c"%s: Maximum allowed number of libraries reached (%u)\n".as_ptr(),
            c"sof_ipc4_find_module_by_uuid".as_ptr(),
            (*ipc4_data).max_libs_count,
        );
        return ptr::null_mut();
    }

    /* The module cannot be found, try to load it as a library */
    ret = sof_ipc4_load_library_by_uuid(sdev, lib_id + 1, uuid);
    if ret != 0 {
        return ptr::null_mut();
    }

    /* Look for the module in the newly loaded library, it should be available now */
    while xa_for_each_next(&mut (*ipc4_data).fw_lib_xa, &mut lib_id, &mut fw_lib, lib_id) {
        i = 0;
        while i < (*fw_lib).num_modules as c_int {
            if guid_equal(
                uuid,
                &(*(*fw_lib).modules.add(i as usize)).man4_module_entry.uuid as *const guid_t,
            ) {
                return (*fw_lib).modules.add(i as usize);
            }
            i += 1;
        }
    }

    ptr::null_mut()
}

unsafe extern "C" fn sof_ipc4_validate_firmware(sdev: *mut snd_sof_dev) -> c_int {
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let fw_hdr_offset: u32 = (*ipc4_data).manifest_fw_hdr_offset;
    let fw_header: *mut sof_man4_fw_binary_header;
    let fw = (*sdev).basefw.fw;
    let ext_man_hdr: *mut sof_ext_manifest4_hdr;

    ext_man_hdr = (*fw).data as *mut sof_ext_manifest4_hdr;
    fw_header = (*fw)
        .data
        .add((*ext_man_hdr).len as usize + fw_hdr_offset as usize)
        as *mut sof_man4_fw_binary_header;

    /* TODO: Add firmware verification code here */

    dev_dbg(
        (*sdev).dev,
        c"Validated firmware version: %u.%u.%u.%u\n".as_ptr(),
        (*fw_header).major_version,
        (*fw_header).minor_version,
        (*fw_header).hotfix_version,
        (*fw_header).build_version,
    );

    0
}

#[no_mangle]
pub unsafe extern "C" fn sof_ipc4_query_fw_configuration(sdev: *mut snd_sof_dev) -> c_int {
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let iops = (*(*sdev).ipc).ops;
    let mut fw_ver: *mut sof_ipc4_fw_version;
    let mut tuple: *mut sof_ipc4_tuple;
    let mut msg: sof_ipc4_msg = core::mem::zeroed();
    let mut offset: size_t = 0;
    let mut ret: c_int;

    /* Get the firmware configuration */
    msg.primary = SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG);
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MOD_ID(SOF_IPC4_MOD_INIT_BASEFW_MOD_ID);
    msg.primary |= SOF_IPC4_MOD_INSTANCE(SOF_IPC4_MOD_INIT_BASEFW_INSTANCE_ID);
    msg.extension = SOF_IPC4_MOD_EXT_MSG_PARAM_ID(SOF_IPC4_FW_PARAM_FW_CONFIG);

    msg.data_size = (*(*sdev).ipc).max_payload_size;
    msg.data_ptr = kzalloc(msg.data_size, GFP_KERNEL);
    if msg.data_ptr.is_null() {
        return -ENOMEM;
    }

    ret = (*iops).set_get_data.unwrap()(sdev, &mut msg, msg.data_size, false);
    if ret != 0 {
        kfree(msg.data_ptr);
        return ret;
    }

    while offset < msg.data_size {
        tuple = (msg.data_ptr as *mut u8).add(offset) as *mut sof_ipc4_tuple;

        if (*tuple).type_ == SOF_IPC4_FW_CFG_FW_VERSION {
            fw_ver = (*tuple).value.as_ptr() as *mut sof_ipc4_fw_version;

            dev_info(
                (*sdev).dev,
                c"Booted firmware version: %u.%u.%u.%u\n".as_ptr(),
                (*fw_ver).major,
                (*fw_ver).minor,
                (*fw_ver).hotfix,
                (*fw_ver).build,
            );
        } else if (*tuple).type_ == SOF_IPC4_FW_CFG_DL_MAILBOX_BYTES {
            trace_sof_ipc4_fw_config(sdev, c"DL mailbox size".as_ptr(), *(*tuple).value.as_ptr());
        } else if (*tuple).type_ == SOF_IPC4_FW_CFG_UL_MAILBOX_BYTES {
            trace_sof_ipc4_fw_config(sdev, c"UL mailbox size".as_ptr(), *(*tuple).value.as_ptr());
        } else if (*tuple).type_ == SOF_IPC4_FW_CFG_TRACE_LOG_BYTES {
            trace_sof_ipc4_fw_config(sdev, c"Trace log size".as_ptr(), *(*tuple).value.as_ptr());
            (*ipc4_data).mtrace_log_bytes = *(*tuple).value.as_ptr();
        } else if (*tuple).type_ == SOF_IPC4_FW_CFG_MAX_LIBS_COUNT {
            trace_sof_ipc4_fw_config(
                sdev,
                c"maximum number of libraries".as_ptr(),
                *(*tuple).value.as_ptr(),
            );
            (*ipc4_data).max_libs_count = *(*tuple).value.as_ptr();
            if (*ipc4_data).max_libs_count == 0 {
                (*ipc4_data).max_libs_count = 1;
            }
        } else if (*tuple).type_ == SOF_IPC4_FW_CFG_MAX_PPL_COUNT {
            (*ipc4_data).max_num_pipelines = *(*tuple).value.as_ptr() as c_int;
            trace_sof_ipc4_fw_config(
                sdev,
                c"Max PPL count %d".as_ptr(),
                (*ipc4_data).max_num_pipelines as u32,
            );
            if (*ipc4_data).max_num_pipelines <= 0 {
                dev_err(
                    (*sdev).dev,
                    c"Invalid max_num_pipelines %d".as_ptr(),
                    (*ipc4_data).max_num_pipelines,
                );
                ret = -EINVAL;
                kfree(msg.data_ptr);
                return ret;
            }
        } else if (*tuple).type_ == SOF_IPC4_FW_CONTEXT_SAVE {
            (*ipc4_data).fw_context_save = *(*tuple).value.as_ptr();
            /*
             * Set the default libraries_restored value - if full
             * context save is supported then it means that
             * libraries are restored
             */
            (*ipc4_data).libraries_restored = (*ipc4_data).fw_context_save;
        }

        offset += size_of::<sof_ipc4_tuple>() + (*tuple).size as size_t;
    }

    /* Get the hardware configuration */
    msg.primary = SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG);
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MOD_ID(SOF_IPC4_MOD_INIT_BASEFW_MOD_ID);
    msg.primary |= SOF_IPC4_MOD_INSTANCE(SOF_IPC4_MOD_INIT_BASEFW_INSTANCE_ID);
    msg.extension = SOF_IPC4_MOD_EXT_MSG_PARAM_ID(SOF_IPC4_FW_PARAM_HW_CONFIG_GET);

    msg.data_size = (*(*sdev).ipc).max_payload_size;

    ret = (*iops).set_get_data.unwrap()(sdev, &mut msg, msg.data_size, false);
    if ret != 0 {
        kfree(msg.data_ptr);
        return ret;
    }

    offset = 0;
    while offset < msg.data_size {
        tuple = (msg.data_ptr as *mut u8).add(offset) as *mut sof_ipc4_tuple;

        if (*tuple).type_ == SOF_IPC4_HW_CFG_INTEL_MIC_PRIVACY_CAPS {
            if let Some(intel_configure_mic_privacy) = (*ipc4_data).intel_configure_mic_privacy {
                let caps: *mut sof_ipc4_intel_mic_privacy_cap =
                    (*tuple).value.as_ptr() as *mut sof_ipc4_intel_mic_privacy_cap;

                intel_configure_mic_privacy(sdev, caps);
            }
        }

        offset += size_of::<sof_ipc4_tuple>() + (*tuple).size as size_t;
    }

    kfree(msg.data_ptr);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn sof_ipc4_reload_fw_libraries(sdev: *mut snd_sof_dev) -> c_int {
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let mut fw_lib: *mut sof_ipc4_fw_library = ptr::null_mut();
    let mut lib_id: c_ulong = 0;
    let mut ret: c_int = 0;

    while xa_for_each_next(&mut (*ipc4_data).fw_lib_xa, &mut lib_id, &mut fw_lib, 1) {
        ret = (*ipc4_data).load_library.unwrap()(sdev, fw_lib, true);
        if ret != 0 {
            dev_err(
                (*sdev).dev,
                c"%s: Failed to reload library: %s, %d\n".as_ptr(),
                c"sof_ipc4_reload_fw_libraries".as_ptr(),
                (*fw_lib).name,
                ret,
            );
            break;
        }
    }

    ret
}

/**
 * sof_ipc4_update_cpc_from_manifest - Update the cpc in base config from manifest
 * @sdev: SOF device
 * @fw_module: pointer struct sof_ipc4_fw_module to parse
 * @basecfg: Pointer to the base_config to update
 */
#[no_mangle]
pub unsafe extern "C" fn sof_ipc4_update_cpc_from_manifest(
    sdev: *mut snd_sof_dev,
    fw_module: *mut sof_ipc4_fw_module,
    basecfg: *mut sof_ipc4_base_module_cfg,
) {
    let mut fw_mod_cfg: *const sof_man4_module_config;
    let mut cpc_pick: u32 = 0;
    let mut max_cpc: u32 = 0;
    let msg: *const c_char;
    let mut i: c_int;

    if (*fw_module).fw_mod_cfg.is_null() {
        msg = c"No mod_cfg available for CPC lookup in the firmware file's manifest".as_ptr();
        dev_dbg(
            (*sdev).dev,
            c"%s (UUID: %pUL): %s (ibs/obs: %u/%u)\n".as_ptr(),
            (*fw_module).man4_module_entry.name,
            &(*fw_module).man4_module_entry.uuid as *const guid_t,
            msg,
            (*basecfg).ibs,
            (*basecfg).obs,
        );
        return;
    }

    /*
     * Find the best matching (highest) CPC value based on the module's
     * IBS/OBS configuration inferred from the audio format selection.
     *
     * The CPC value in each module config entry has been measured and
     * recorded as a IBS/OBS/CPC triplet and stored in the firmware file's
     * manifest
     */
    fw_mod_cfg = (*fw_module).fw_mod_cfg;
    i = 0;
    while i < (*fw_module).man4_module_entry.cfg_count as c_int {
        if (*basecfg).obs == (*fw_mod_cfg.add(i as usize)).obs
            && (*basecfg).ibs == (*fw_mod_cfg.add(i as usize)).ibs
            && cpc_pick < (*fw_mod_cfg.add(i as usize)).cpc
        {
            cpc_pick = (*fw_mod_cfg.add(i as usize)).cpc;
        }

        if max_cpc < (*fw_mod_cfg.add(i as usize)).cpc {
            max_cpc = (*fw_mod_cfg.add(i as usize)).cpc;
        }
        i += 1;
    }

    (*basecfg).cpc = cpc_pick;

    /* We have a matching configuration for CPC */
    if (*basecfg).cpc != 0 {
        return;
    }

    /*
     * No matching IBS/OBS found, the firmware manifest is missing
     * information in the module's module configuration table.
     */
    if max_cpc == 0 {
        msg = c"No CPC value available in the firmware file's manifest".as_ptr();
    } else if cpc_pick == 0 {
        msg = c"No CPC match in the firmware file's manifest".as_ptr();
    } else {
        msg = ptr::null();
    }

    dev_dbg(
        (*sdev).dev,
        c"%s (UUID: %pUL): %s (ibs/obs: %u/%u)\n".as_ptr(),
        (*fw_module).man4_module_entry.name,
        &(*fw_module).man4_module_entry.uuid as *const guid_t,
        msg,
        (*basecfg).ibs,
        (*basecfg).obs,
    );
}

#[no_mangle]
pub static ipc4_loader_ops: sof_ipc_fw_loader_ops = sof_ipc_fw_loader_ops {
    validate: Some(sof_ipc4_validate_firmware),
    parse_ext_manifest: Some(sof_ipc4_fw_parse_basefw_ext_man),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
