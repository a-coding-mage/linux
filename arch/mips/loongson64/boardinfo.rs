// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kobj_attribute {
    pub attr: attribute,
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct boot_param_board {
    pub name: *const c_char,
}

#[repr(C)]
pub struct boot_param_interface {
    pub description: *const c_char,
}

#[repr(C)]
pub struct boot_param_special {
    pub special_name: *const c_char,
}

extern "C" {
    static mut eboard: *mut boot_param_board;
    static mut einter: *mut boot_param_interface;
    static mut especial: *mut boot_param_special;
    static mut firmware_kobj: *mut kobject;

    fn strscpy_pad(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject;
    fn sysfs_create_file(kobj: *mut kobject, attr: *const attribute) -> c_int;
}

extern "C" {
    fn pr_err(fmt: *const c_char, ...);
}

const ENOMEM: c_int = 12;

unsafe fn boardinfo_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    let mut board_manufacturer = [0 as c_char; 64];
    let mut tmp_board_manufacturer = board_manufacturer.as_mut_ptr();
    let mut bios_vendor = [0 as c_char; 64];
    let mut tmp_bios_vendor = bios_vendor.as_mut_ptr();

    strscpy_pad(board_manufacturer.as_mut_ptr(), (*eboard).name, 64);
    strscpy_pad(bios_vendor.as_mut_ptr(), (*einter).description, 64);

    sprintf(
        buf,
        b"Board Info\nManufacturer\t\t: %s\nBoard Name\t\t: %s\nFamily\t\t\t: LOONGSON3\n\nBIOS Info\nVendor\t\t\t: %s\nVersion\t\t\t: %s\nRelease Date\t\t: %s\n\0".as_ptr()
            as *const c_char,
        strsep(&mut tmp_board_manufacturer, b"-\0".as_ptr() as *const c_char),
        (*eboard).name,
        strsep(&mut tmp_bios_vendor, b"-\0".as_ptr() as *const c_char),
        (*einter).description,
        (*especial).special_name,
    ) as isize
}

static mut boardinfo_attr: kobj_attribute = kobj_attribute {
    attr: attribute { _private: [0; 0] },
    _private: [0; 0],
};

unsafe fn boardinfo_init() -> c_int {
    let lefi_kobj = kobject_create_and_add(b"lefi\0".as_ptr() as *const c_char, firmware_kobj);
    if lefi_kobj.is_null() {
        pr_err(b"lefi: Firmware registration failed.\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }

    sysfs_create_file(lefi_kobj, &boardinfo_attr.attr)
}

// late_initcall(boardinfo_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
