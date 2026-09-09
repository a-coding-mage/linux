// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  linux/arch/arm/kernel/isa.c
 *
 *  Copyright (C) 1999 Phil Blundell
 *
 *  ISA shared memory and I/O port support, and is required to support
 *  iopl, inb, outb and friends in userspace via glibc emulation.
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;

// Types and symbols supplied by the corresponding kernel dependencies.
use crate::ctl_table;

extern "C" {
    fn proc_dointvec(
        table: *mut ctl_table,
        write: *mut c_void,
        buffer: *mut c_void,
        length: *mut usize,
        ppos: *mut u64,
    ) -> c_int;

    fn register_sysctl(path: *const c_char, table: *const ctl_table) -> *mut c_void;
}

static mut isa_membase: u32 = 0;
static mut isa_portbase: u32 = 0;
static mut isa_portshift: u32 = 0;

static ctl_isa_vars: [ctl_table; 3] = [
    ctl_table {
        procname: b"membase\0".as_ptr() as *const c_char,
        data: unsafe { &isa_membase as *const u32 as *mut c_void },
        maxlen: size_of::<u32>(),
        mode: 0o444,
        proc_handler: Some(proc_dointvec),
    },
    ctl_table {
        procname: b"portbase\0".as_ptr() as *const c_char,
        data: unsafe { &isa_portbase as *const u32 as *mut c_void },
        maxlen: size_of::<u32>(),
        mode: 0o444,
        proc_handler: Some(proc_dointvec),
    },
    ctl_table {
        procname: b"portshift\0".as_ptr() as *const c_char,
        data: unsafe { &isa_portshift as *const u32 as *mut c_void },
        maxlen: size_of::<u32>(),
        mode: 0o444,
        proc_handler: Some(proc_dointvec),
    },
];

static mut isa_sysctl_header: *mut c_void = core::ptr::null_mut();

// __init
pub unsafe extern "C" fn register_isa_ports(
    membase: u32,
    portbase: u32,
    portshift: u32,
) {
    isa_membase = membase;
    isa_portbase = portbase;
    isa_portshift = portshift;
    isa_sysctl_header = register_sysctl(b"bus/isa\0".as_ptr() as *const c_char, ctl_isa_vars.as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
