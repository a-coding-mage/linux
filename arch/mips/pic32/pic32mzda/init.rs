// SPDX-License-Identifier: GPL-2.0-only
/*
 * Joshua Henderson, joshua.henderson@microchip.com
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */

// Declarations supplied by the kernel and platform headers are referenced here
// as external dependencies. Build-time configuration conditions from the C
// source are preserved in comments where they cannot be resolved locally.

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn get_fdt() -> *mut c_void;
    fn __dt_setup_arch(dtb: *mut c_void);
    fn fw_init_early_console();
    fn pic32_config_init();
    fn pic32_set_sdhci_adma_fifo_threshold();
    fn of_find_node_by_path(path: *const c_char) -> *mut device_node;
    fn of_find_compatible_node(
        from: *mut device_node,
        typ: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn of_address_to_resource(
        node: *mut device_node,
        index: c_int,
        resource: *mut resource,
    ) -> c_int;
    fn of_have_populated_dt() -> c_int;
    fn of_platform_default_populate(
        root: *mut device_node,
        lookup: *mut of_dev_auxdata,
        parent: *mut c_void,
    ) -> c_int;
    fn panic(message: *const c_char) -> !;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;

    static mut boot_command_line: *const c_char;
    static mut arcs_cmdline: [c_char; COMMAND_LINE_SIZE];
    static __dtb_start: c_void;
    static fw_arg0: c_ulong;
    static fw_arg1: c_ulong;
}

type c_ulong = usize;
const COMMAND_LINE_SIZE: usize = 4096;

#[repr(C)]
struct device_node {
    name: *const c_char,
}

#[repr(C)]
struct resource {
    start: usize,
}

#[repr(C)]
struct pic32_sdhci_platform_data {
    setup_dma: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
struct of_dev_auxdata {
    compatible: *const c_char,
    phys_addr: usize,
    name: *mut c_char,
    platform_data: *mut c_void,
}

pub unsafe extern "C" fn get_system_type() -> *const c_char {
    b"PIC32MZDA\0".as_ptr() as *const c_char
}

pub unsafe extern "C" fn plat_mem_setup() {
    let dtb: *mut c_void;

    dtb = get_fdt();
    if dtb.is_null() {
        pr_err(b"pic32: no DTB found.\n\0".as_ptr() as *const c_char);
        return;
    }

    /*
     * Load the builtin device tree. This causes the chosen node to be
     * parsed resulting in our memory appearing.
     */
    __dt_setup_arch(dtb);

    pr_info(b"Found following command lines\n\0".as_ptr() as *const c_char);
    pr_info(b" boot_command_line: %s\n\0".as_ptr() as *const c_char, boot_command_line);
    pr_info(b" arcs_cmdline     : %s\n\0".as_ptr() as *const c_char, arcs_cmdline.as_ptr());
    // #ifdef CONFIG_CMDLINE_BOOL
    // pr_info(" builtin_cmdline  : %s\n", CONFIG_CMDLINE);
    // #endif
    if dtb != (&__dtb_start as *const c_void as *mut c_void) {
        strscpy(arcs_cmdline.as_mut_ptr(), boot_command_line, COMMAND_LINE_SIZE);
    }

    // #ifdef CONFIG_EARLY_PRINTK
    fw_init_early_console();
    // #endif
    pic32_config_init();
}

unsafe fn pic32_init_cmdline(argc: c_int, argv: *mut *mut c_char) {
    let mut count: c_uint = (COMMAND_LINE_SIZE - 1) as c_uint;
    let mut i: c_int;
    let mut dst = arcs_cmdline.as_mut_ptr();
    let mut src: *mut c_char;

    i = 1;
    while i < argc && count != 0 {
        src = *argv.offset(i as isize);
        while *src != 0 && count != 0 {
            *dst = *src;
            dst = dst.add(1);
            src = src.add(1);
            count -= 1;
        }
        *dst = b' ' as c_char;
        dst = dst.add(1);
        i += 1;
    }
    if i > 1 {
        dst = dst.sub(1);
    }

    *dst = 0;
}

pub unsafe extern "C" fn prom_init() {
    pic32_init_cmdline(fw_arg0 as c_int, fw_arg1 as *mut *mut c_char);
}

static mut sdhci_data: pic32_sdhci_platform_data = pic32_sdhci_platform_data {
    setup_dma: Some(pic32_set_sdhci_adma_fifo_threshold),
};

static mut pic32_auxdata_lookup: [of_dev_auxdata; 2] = [
    of_dev_auxdata {
        compatible: b"microchip,pic32mzda-sdhci\0".as_ptr() as *const c_char,
        phys_addr: 0,
        name: b"sdhci\0".as_ptr() as *mut c_char,
        platform_data: unsafe { &mut sdhci_data as *mut _ as *mut c_void },
    },
    of_dev_auxdata {
        compatible: core::ptr::null(),
        phys_addr: 0,
        name: core::ptr::null_mut(),
        platform_data: core::ptr::null_mut(),
    },
];

unsafe fn pic32_of_prepare_platform_data(lookup: *mut of_dev_auxdata) -> c_int {
    let root = of_find_node_by_path(b"/\0".as_ptr() as *const c_char);
    let mut entry = lookup;
    let mut res = resource { start: 0 };

    while !(*entry).compatible.is_null() {
        let np = of_find_compatible_node(
            core::ptr::null_mut(),
            core::ptr::null(),
            (*entry).compatible,
        );
        if !np.is_null() {
            (*entry).name = (*np).name as *mut c_char;
            if (*entry).phys_addr != 0 {
                of_node_put(np);
                entry = entry.add(1);
                continue;
            }
            if of_address_to_resource(np, 0, &mut res) == 0 {
                (*entry).phys_addr = res.start;
            }
            of_node_put(np);
        }
        entry = entry.add(1);
    }

    of_node_put(root);
    0
}

unsafe fn plat_of_setup() -> c_int {
    if of_have_populated_dt() == 0 {
        panic(b"Device tree not present\0".as_ptr() as *const c_char);
    }

    pic32_of_prepare_platform_data(pic32_auxdata_lookup.as_mut_ptr());
    if of_platform_default_populate(
        core::ptr::null_mut(),
        pic32_auxdata_lookup.as_mut_ptr(),
        core::ptr::null_mut(),
    ) != 0 {
        panic(b"Failed to populate DT\0".as_ptr() as *const c_char);
    }

    0
}

// arch_initcall(plat_of_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
