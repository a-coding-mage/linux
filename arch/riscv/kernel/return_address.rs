// SPDX-License-Identifier: GPL-2.0-only
/*
 * This code come from arch/arm64/kernel/return_address.c
 *
 * Copyright (C) 2023 SiFive.
 */

use core::ffi::c_void;

#[repr(C)]
struct ReturnAddressData {
    level: u32,
    addr: *mut c_void,
}

extern "C" {
    static mut current: *mut c_void;
    fn arch_stack_walk(
        consume_entry: unsafe extern "C" fn(*mut c_void, c_ulong) -> bool,
        cookie: *mut c_void,
        task: *mut c_void,
        stack: *mut c_void,
    );
}

type c_ulong = usize;

unsafe extern "C" fn save_return_addr(d: *mut c_void, pc: c_ulong) -> bool {
    let data = &mut *(d as *mut ReturnAddressData);

    if data.level == 0 {
        data.addr = pc as *mut c_void;
        return false;
    }

    data.level -= 1;

    true
}

#[inline(never)]
pub unsafe extern "C" fn return_address(level: u32) -> *mut c_void {
    let mut data = ReturnAddressData {
        level: level + 3,
        addr: core::ptr::null_mut(),
    };

    arch_stack_walk(
        save_return_addr,
        &mut data as *mut ReturnAddressData as *mut c_void,
        current,
        core::ptr::null_mut(),
    );

    if data.level == 0 {
        data.addr
    } else {
        core::ptr::null_mut()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
