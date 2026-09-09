// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024 Allied Telesis
 */

// Declarations supplied by the surrounding kernel sources.
use core::ffi::{c_char, c_void};

extern "C" {
    fn fdt_path_offset(fdt: *const c_void, path: *const c_char) -> i32;
    fn fw_getenvl(name: *const c_char) -> u32;
    fn fdt_setprop_u32(
        fdt: *mut c_void,
        node: i32,
        name: *const c_char,
        value: u32,
    ) -> i32;
    fn fdt_check_header(fdt: *const c_void) -> i32;
    fn fw_init_cmdline();
    fn apply_mips_fdt_fixups(
        buf: *mut c_void,
        buf_size: usize,
        fdt: *const c_void,
        fixups: *const MipsFdtFixup,
    ) -> i32;
    fn panic(message: *const c_char, ...);
}

#[repr(C)]
struct MipsFdtFixup {
    fixup: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    name: *const c_char,
}

#[repr(C)]
struct OfDeviceId {
    compatible: *const c_char,
}

unsafe extern "C" fn realtek_add_initrd(fdt: *mut c_void) -> i32 {
    let mut node: i32;
    let mut err: i32;
    let mut start: u32;
    let mut size: u32;

    node = fdt_path_offset(fdt, b"/chosen\0".as_ptr() as *const c_char);
    if node < 0 {
        pr_err(b"/chosen node not found\n\0".as_ptr() as *const c_char);
        return -2; // -ENOENT
    }

    start = fw_getenvl(b"initrd_start\0".as_ptr() as *const c_char);
    size = fw_getenvl(b"initrd_size\0".as_ptr() as *const c_char);

    if start == 0 && size == 0 {
        return 0;
    }

    pr_info(b"Adding initrd info from environment\n\0".as_ptr() as *const c_char);

    err = fdt_setprop_u32(
        fdt,
        node,
        b"linux,initrd-start\0".as_ptr() as *const c_char,
        start,
    );
    if err != 0 {
        pr_err(b"unable to set initrd-start: %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    err = fdt_setprop_u32(
        fdt,
        node,
        b"linux,initrd-end\0".as_ptr() as *const c_char,
        start.wrapping_add(size),
    );
    if err != 0 {
        pr_err(b"unable to set initrd-end: %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    0
}

extern "C" {
    fn pr_err(format: *const c_char, ...);
    fn pr_info(format: *const c_char, ...);
}

static REALTEK_FDT_FIXUPS: [MipsFdtFixup; 2] = [
    MipsFdtFixup {
        fixup: Some(realtek_add_initrd),
        name: b"add initrd\0".as_ptr() as *const c_char,
    },
    MipsFdtFixup {
        fixup: None,
        name: core::ptr::null(),
    },
];

unsafe extern "C" fn realtek_fixup_fdt(
    fdt: *const c_void,
    _match_data: *const c_void,
) -> *const c_void {
    static mut FDT_BUF: [u8; 16 << 10] = [0; 16 << 10];
    let err: i32;

    if fdt_check_header(fdt) != 0 {
        panic(b"Corrupt DT\0".as_ptr() as *const c_char);
    }

    fw_init_cmdline();

    err = apply_mips_fdt_fixups(
        FDT_BUF.as_mut_ptr() as *mut c_void,
        core::mem::size_of::<[u8; 16 << 10]>(),
        fdt,
        REALTEK_FDT_FIXUPS.as_ptr(),
    );
    if err != 0 {
        panic(b"Unable to fixup FDT: %d\0".as_ptr() as *const c_char, err);
    }

    FDT_BUF.as_ptr() as *const c_void
}

static REALTEK_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"realtek,rtl9302-soc\0".as_ptr() as *const c_char,
    },
    OfDeviceId {
        compatible: core::ptr::null(),
    },
];

// MIPS_MACHINE(realtek) = {
//     .matches = realtek_of_match,
//     .fixup_fdt = realtek_fixup_fdt,
// };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
