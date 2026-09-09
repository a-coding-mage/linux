// SPDX-License-Identifier: GPL-2.0-only
/*
 * POWER Platform specific code for non-volatile SED key access
 * Copyright (C) 2022 IBM Corporation
 *
 * Define operations for SED Opal to read/write keys
 * from POWER LPAR Platform KeyStore(PLPKS).
 *
 * Self Encrypting Drives(SED) key storage using PLPKS
 */

// Linux and architecture-specific declarations are supplied by the surrounding kernel.

static mut plpks_sed_initialized: bool = false;
static mut plpks_sed_available: bool = false;

/* structure that contains all SED data */
#[repr(C)]
struct plpks_sed_object_data {
    version: u8,
    pad1: [u8; 7],
    authority: c_ulong,
    range: c_ulong,
    key_len: c_uint,
    key: [u8; 32],
}

const PLPKS_SED_OBJECT_DATA_V0: u8 = 0;
const PLPKS_SED_MANGLED_LABEL: &str = "/default/pri";
const PLPKS_SED_COMPONENT: &str = "sed-opal";
const PLPKS_SED_KEY: &str = "opal-boot-pin";

/* authority is admin1 and range is global */
const PLPKS_SED_AUTHORITY: u64 = 0x0000000900010001;
const PLPKS_SED_RANGE: u64 = 0x0000080200000001;

extern "C" {
    fn plpks_is_available() -> bool;
    fn pr_err(fmt: *const c_char, ...);
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn plpks_read_os_var(var: *mut plpks_var) -> c_int;
    fn plpks_remove_var(component: *const c_char, os: c_uint, name: plpks_var_name) -> c_int;
    fn plpks_write_var(var: *mut plpks_var) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, value: c_int, n: usize) -> *mut c_void;
}

// Types and constants are provided by <asm/plpks.h> and related kernel headers.
#[repr(C)]
struct plpks_var {
    name: *const c_char,
    namelen: usize,
    policy: c_uint,
    os: c_uint,
    data: *mut u8,
    datalen: usize,
    component: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct plpks_var_name {
    namelen: usize,
    name: *const c_char,
}

const PLPKS_WORLDREADABLE: c_uint = 0;
const PLPKS_VAR_COMMON: c_uint = 0;
const EOPNOTSUPP: c_int = 95;

unsafe fn plpks_init_var(var: *mut plpks_var, keyname: *mut c_char) {
    if !plpks_sed_initialized {
        plpks_sed_initialized = true;
        plpks_sed_available = plpks_is_available();
        if !plpks_sed_available {
            pr_err(b"SED: plpks not available\0".as_ptr() as *const c_char);
        }
    }

    (*var).name = keyname;
    (*var).namelen = strlen(keyname);
    if strcmp(PLPKS_SED_KEY.as_ptr() as *const c_char, keyname) == 0 {
        (*var).name = PLPKS_SED_MANGLED_LABEL.as_ptr() as *const c_char;
        (*var).namelen = strlen(keyname);
    }
    (*var).policy = PLPKS_WORLDREADABLE;
    (*var).os = PLPKS_VAR_COMMON;
    (*var).data = core::ptr::null_mut();
    (*var).datalen = 0;
    (*var).component = PLPKS_SED_COMPONENT.as_ptr() as *const c_char;
}

/* Read the SED Opal key from PLPKS given the label */
unsafe fn sed_read_key(keyname: *mut c_char, key: *mut c_char, keylen: *mut c_uint) -> c_int {
    let mut var: plpks_var = core::mem::zeroed();
    let mut data: plpks_sed_object_data = core::mem::zeroed();
    plpks_init_var(&mut var, keyname);

    if !plpks_sed_available {
        return -EOPNOTSUPP;
    }

    var.data = &mut data as *mut _ as *mut u8;
    var.datalen = core::mem::size_of::<plpks_sed_object_data>();

    let ret = plpks_read_os_var(&mut var);
    if ret != 0 {
        return ret;
    }

    let len = u32::from_be(data.key_len).min(var.datalen as u32) as usize;
    memcpy(key as *mut c_void, data.key.as_ptr() as *const c_void, len);
    *key.add(len) = 0;
    *keylen = len as c_uint;
    0
}

/* Write the SED Opal key to PLPKS given the label */
unsafe fn sed_write_key(keyname: *mut c_char, key: *mut c_char, keylen: c_uint) -> c_int {
    let mut var: plpks_var = core::mem::zeroed();
    let mut data: plpks_sed_object_data = core::mem::zeroed();
    let vname: plpks_var_name;
    plpks_init_var(&mut var, keyname);

    if !plpks_sed_available {
        return -EOPNOTSUPP;
    }

    var.datalen = core::mem::size_of::<plpks_sed_object_data>();
    var.data = &mut data as *mut _ as *mut u8;

    /* initialize SED object */
    data.version = PLPKS_SED_OBJECT_DATA_V0;
    data.authority = PLPKS_SED_AUTHORITY.to_be() as c_ulong;
    data.range = PLPKS_SED_RANGE.to_be() as c_ulong;
    memset(data.pad1.as_mut_ptr() as *mut c_void, 0, data.pad1.len());
    data.key_len = keylen.to_be();
    memcpy(data.key.as_mut_ptr() as *mut c_void, key as *const c_void, keylen as usize);

    /*
     * Key update requires remove first. The return value
     * is ignored since it's okay if the key doesn't exist.
     */
    vname = plpks_var_name { namelen: var.namelen, name: var.name };
    plpks_remove_var(var.component, var.os, vname);

    plpks_write_var(&mut var)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
