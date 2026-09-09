// SPDX-License-Identifier: GPL-2.0-only
/*
 * memconsole.c
 *
 * Architecture-independent parts of the memory based BIOS console.
 *
 * Copyright 2017 Google Inc.
 */

// Translated dependencies supplied by the Linux kernel and memconsole.h.
use core::ffi::{c_char, c_int, c_void};

pub type SsizeT = isize;
pub type LoffT = i64;

#[repr(C)]
pub struct File {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Kobject {
    _private: [u8; 0],
}

pub type MemconsoleReadFunc = unsafe extern "C" fn(*mut c_char, LoffT, usize) -> SsizeT;

#[repr(C)]
pub struct Attribute {
    pub name: *const c_char,
    pub mode: u16,
}

#[repr(C)]
pub struct BinAttribute {
    pub attr: Attribute,
    pub read: Option<unsafe extern "C" fn(
        *mut File,
        *mut Kobject,
        *const BinAttribute,
        *mut c_char,
        LooffT,
        usize,
    ) -> SsizeT>,
    pub private: *mut c_void,
}

extern "C" {
    static mut firmware_kobj: *mut Kobject;
    fn sysfs_create_bin_file(kobj: *mut Kobject, attr: *mut BinAttribute) -> c_int;
    fn sysfs_remove_bin_file(kobj: *mut Kobject, attr: *mut BinAttribute);
}

static LOG_NAME: &[u8] = b"log\0";

unsafe extern "C" fn memconsole_read(
    _filp: *mut File,
    _kobp: *mut Kobject,
    bin_attr: *const BinAttribute,
    buf: *mut c_char,
    pos: LooffT,
    count: usize,
) -> SsizeT {
    let memconsole_read_func = (*bin_attr).private;
    if memconsole_read_func.is_null() {
        // WARN_ON_ONCE(!memconsole_read_func)
        return -5; // -EIO
    }

    let read_func: MemconsoleReadFunc = core::mem::transmute(memconsole_read_func);
    read_func(buf, pos, count)
}

static mut memconsole_bin_attr: BinAttribute = BinAttribute {
    attr: Attribute {
        name: LOG_NAME.as_ptr() as *const c_char,
        mode: 0o444,
    },
    read: Some(memconsole_read),
    private: core::ptr::null_mut(),
};

#[no_mangle]
pub unsafe extern "C" fn memconsole_setup(read_func: Option<MemconsoleReadFunc>) {
    memconsole_bin_attr.private = read_func
        .map(|func| func as *mut c_void)
        .unwrap_or(core::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn memconsole_sysfs_init() -> c_int {
    sysfs_create_bin_file(firmware_kobj, &mut memconsole_bin_attr)
}

#[no_mangle]
pub unsafe extern "C" fn memconsole_exit() {
    sysfs_remove_bin_file(firmware_kobj, &mut memconsole_bin_attr);
}

// EXPORT_SYMBOL(memconsole_setup);
// EXPORT_SYMBOL(memconsole_sysfs_init);
// EXPORT_SYMBOL(memconsole_exit);
// MODULE_AUTHOR("Google, Inc.");
// MODULE_DESCRIPTION("Architecture-independent parts of the memory based BIOS console");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
