/* SPDX-License-Identifier: GPL-2.0 */
/*
 * parport.h: platform-specific PC-style parport initialisation
 *
 * Copyright (C) 1999, 2000  Tim Waugh <tim@cyberelk.demon.co.uk>
 *
 * This file should only be included by drivers/parport/parport_pc.c.
 */

// The original header guard is omitted from executable Rust.
// The contents are intended for the kernel build (__KERNEL__).
// Dependency: <linux/of_irq.h>.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    fn of_get_property(
        np: *mut device_node,
        name: *const c_char,
        propsize: *mut i32,
    ) -> *const u32;
    fn irq_of_parse_and_map(np: *mut device_node, index: i32) -> i32;
    fn parport_pc_probe_port(
        io1: u32,
        io2: u32,
        irq: i32,
        dma: i32,
        irq_func: *mut c_void,
        flags: i32,
    ) -> *mut c_void;
}

// Dependency: the kernel's for_each_compatible_node macro.
extern "Rust" {
    // This declaration documents the macro dependency; the kernel supplies
    // the iteration construct at the inclusion site.
}

pub unsafe fn parport_pc_find_nonpci_ports(autoirq: i32, autodma: i32) -> i32 {
    let mut np: *mut device_node = core::ptr::null_mut();
    let mut count: i32 = 0;

    for_each_compatible_node!(np, "parallel", "pnpPNP,400") {
        let mut propsize: i32 = 0;
        let prop: *const u32 = of_get_property(
            np,
            b"reg\0".as_ptr() as *const c_char,
            &mut propsize,
        );
        if prop.is_null() || propsize > 6 * core::mem::size_of::<u32>() as i32 {
            continue;
        }

        let io1: u32 = *prop.add(1);
        let io2: u32 = *prop.add(2);

        let virq: i32 = irq_of_parse_and_map(np, 0);
        if virq == 0 {
            continue;
        }

        if !parport_pc_probe_port(io1, io2, virq, autodma, core::ptr::null_mut(), 0)
            .is_null()
        {
            count += 1;
        }
    }
    count
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
