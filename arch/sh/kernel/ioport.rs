// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/ioport.c
 *
 * Copyright (C) 2000  Niibe Yutaka
 * Copyright (C) 2005 - 2007 Paul Mundt
 */

// C dependencies: linux/module.h, linux/io.h, asm/io_trapped.h

unsafe extern "C" {
    fn __ioport_map_trapped(port: ::core::ffi::c_ulong, nr: ::core::ffi::c_uint)
        -> *mut ::core::ffi::c_void;
}

// __read_mostly
#[no_mangle]
pub static mut sh_io_port_base: ::core::ffi::c_ulong = !0;

// EXPORT_SYMBOL(sh_io_port_base);

#[no_mangle]
pub unsafe extern "C" fn ioport_map(
    port: ::core::ffi::c_ulong,
    nr: ::core::ffi::c_uint,
) -> *mut ::core::ffi::c_void {
    let ret = unsafe { __ioport_map_trapped(port, nr) };
    if !ret.is_null() {
        return ret;
    }

    (port.wrapping_add(unsafe { sh_io_port_base })) as *mut ::core::ffi::c_void
}

// EXPORT_SYMBOL(ioport_map);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
