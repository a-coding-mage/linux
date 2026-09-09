// SPDX-License-Identifier: GPL-2.0-only

// Secure variable implementation using the PowerVM LPAR Platform KeyStore (PLPKS)
//
// Copyright 2022, 2023 IBM Corporation
// Authors: Russell Currey
//          Andrew Donnellan
//          Nayna Jain

// Kernel and architecture dependencies supplied by the surrounding translation.

use core::ffi::{c_char, c_int, c_void};

type U8 = u8;
type U32 = u32;
type U64 = u64;
type SsizeT = isize;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const EIO: c_int = 5;
const ENOENT: c_int = 2;
const EPERM: c_int = 1;
const PLPKS_WORLDREADABLE: U32 = 0;
const PLPKS_SIGNEDUPDATE: U32 = 0;
const PLPKS_VAR_LINUX: U32 = 0;
const UTF16_LITTLE_ENDIAN: c_int = 0;
const GFP_KERNEL: U32 = 0;

#[repr(C)]
pub struct PlpksVar {
    pub component: *const c_char,
    pub name: *mut c_void,
    pub namelen: U64,
    pub data: *mut U8,
    pub datalen: U64,
    pub os: U32,
    pub policy: U32,
}

#[repr(C)]
pub struct SecvarOperations {
    pub get: Option<unsafe extern "C" fn(*const c_char, U64, *mut U8, *mut U64) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*const c_char, U64, *mut U8, U64) -> c_int>,
    pub format: Option<unsafe extern "C" fn(*mut c_char, usize) -> SsizeT>,
    pub max_size: Option<unsafe extern "C" fn(*mut U64) -> c_int>,
    pub var_names: *const *const c_char,
}

extern "C" {
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn kcalloc(n: usize, size: usize, flags: U32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn utf8s_to_utf16s(src: *const c_char, len: usize, endian: c_int, dst: *mut c_void, maxlen: usize) -> c_int;
    fn plpks_read_os_var(var: *mut PlpksVar) -> c_int;
    fn plpks_signed_update_var(var: *mut PlpksVar, flags: U64) -> c_int;
    fn plpks_read_fw_var(var: *mut PlpksVar) -> SsizeT;
    fn plpks_get_maxobjectsize() -> U64;
    fn plpks_is_available() -> bool;
    fn set_secvar_ops(ops: *const SecvarOperations) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
}

unsafe fn get_policy(name: *const c_char) -> U32 {
    if strcmp(name, b"db\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"dbx\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"grubdb\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"grubdbx\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"sbat\0".as_ptr() as *const c_char) == 0
    {
        PLPKS_WORLDREADABLE | PLPKS_SIGNEDUPDATE
    } else {
        PLPKS_SIGNEDUPDATE
    }
}

static PLPKS_VAR_NAMES_STATIC: [*const c_char; 4] = [
    b"PK\0".as_ptr() as *const c_char,
    b"moduledb\0".as_ptr() as *const c_char,
    b"trustedcadb\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

static PLPKS_VAR_NAMES_DYNAMIC: [*const c_char; 10] = [
    b"PK\0".as_ptr() as *const c_char,
    b"KEK\0".as_ptr() as *const c_char,
    b"db\0".as_ptr() as *const c_char,
    b"dbx\0".as_ptr() as *const c_char,
    b"grubdb\0".as_ptr() as *const c_char,
    b"grubdbx\0".as_ptr() as *const c_char,
    b"sbat\0".as_ptr() as *const c_char,
    b"moduledb\0".as_ptr() as *const c_char,
    b"trustedcadb\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

unsafe extern "C" fn plpks_get_variable(key: *const c_char, key_len: U64, data: *mut U8, data_size: *mut U64) -> c_int {
    let mut var: PlpksVar = core::mem::zeroed();
    let mut rc: c_int = 0;

    // We subtract 1 from key_len because we don't need to include the null terminator at the end of the string
    var.name = kcalloc((key_len - 1) as usize, core::mem::size_of::<u16>(), GFP_KERNEL);
    if var.name.is_null() {
        return -ENOMEM;
    }
    rc = utf8s_to_utf16s(key, (key_len - 1) as usize, UTF16_LITTLE_ENDIAN, var.name, (key_len - 1) as usize);
    if rc < 0 { kfree(var.name); return rc; }
    var.namelen = (rc * 2) as U64;
    var.os = PLPKS_VAR_LINUX;
    if !data.is_null() { var.data = data; var.datalen = *data_size; }
    rc = plpks_read_os_var(&mut var);
    if rc == 0 { *data_size = var.datalen; }
    kfree(var.name);
    if rc != 0 && rc != -ENOENT { pr_err(b"Failed to read variable '%s': %d\0".as_ptr() as *const c_char, key, rc); rc = -EIO; }
    rc
}

unsafe extern "C" fn plpks_set_variable(key: *const c_char, key_len: U64, data: *mut U8, data_size: U64) -> c_int {
    let mut var: PlpksVar = core::mem::zeroed();
    let mut rc: c_int = 0;
    let flags: U64;
    if data_size <= core::mem::size_of::<U64>() as U64 { return -EINVAL; }
    var.name = kcalloc((key_len - 1) as usize, core::mem::size_of::<u16>(), GFP_KERNEL);
    if var.name.is_null() { return -ENOMEM; }
    rc = utf8s_to_utf16s(key, (key_len - 1) as usize, UTF16_LITTLE_ENDIAN, var.name, (key_len - 1) as usize);
    if rc < 0 { kfree(var.name); return rc; }
    var.namelen = (rc * 2) as U64;
    // Flags are contained in the first 8 bytes of the buffer, and are always big-endian
    flags = U64::from_be(core::ptr::read_unaligned(data as *const U64));
    var.datalen = data_size - core::mem::size_of::<U64>() as U64;
    var.data = data.add(core::mem::size_of::<U64>());
    var.os = PLPKS_VAR_LINUX;
    var.policy = get_policy(key);
    rc = plpks_signed_update_var(&mut var, flags);
    kfree(var.name);
    rc
}

unsafe extern "C" fn plpks_get_sb_keymgmt_mode() -> U8 {
    let mut mode: U8 = 0;
    let mut var: PlpksVar = core::mem::zeroed();
    var.name = b"SB_VERSION\0".as_ptr() as *mut c_void;
    var.namelen = 10;
    var.datalen = 1;
    var.data = &mut mode;
    let rc = plpks_read_fw_var(&mut var);
    if rc != 0 {
        if rc != -(ENOENT as isize) && rc != -(EPERM as isize) { pr_info(b"Error %ld reading SB_VERSION from firmware\n\0".as_ptr() as *const c_char, rc); }
        mode = 0;
    }
    mode
}

unsafe extern "C" fn plpks_secvar_format(buf: *mut c_char, bufsize: usize) -> SsizeT {
    snprintf(buf, bufsize, b"ibm,plpks-sb-v%hhu\0".as_ptr() as *const c_char, plpks_get_sb_keymgmt_mode()) as SsizeT
}

unsafe extern "C" fn plpks_max_size(max_size: *mut U64) -> c_int {
    *max_size = plpks_get_maxobjectsize().wrapping_add(core::mem::size_of::<U64>() as U64);
    0
}

static PLPKS_SECVAR_OPS_STATIC: SecvarOperations = SecvarOperations { get: Some(plpks_get_variable), set: Some(plpks_set_variable), format: Some(plpks_secvar_format), max_size: Some(plpks_max_size), var_names: PLPKS_VAR_NAMES_STATIC.as_ptr() };
static PLPKS_SECVAR_OPS_DYNAMIC: SecvarOperations = SecvarOperations { get: Some(plpks_get_variable), set: Some(plpks_set_variable), format: Some(plpks_secvar_format), max_size: Some(plpks_max_size), var_names: PLPKS_VAR_NAMES_DYNAMIC.as_ptr() };

unsafe extern "C" fn plpks_secvar_init() -> c_int {
    if !plpks_is_available() { return -ENODEV; }
    if plpks_get_sb_keymgmt_mode() != 0 { set_secvar_ops(&PLPKS_SECVAR_OPS_DYNAMIC) } else { set_secvar_ops(&PLPKS_SECVAR_OPS_STATIC) }
}

// machine_device_initcall(pseries, plpks_secvar_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
