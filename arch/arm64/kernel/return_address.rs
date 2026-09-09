// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm64/kernel/return_address.c
 *
 * Copyright (C) 2013 Linaro Limited
 * Author: AKASHI Takahiro <takahiro.akashi@linaro.org>
 */

use core::ffi::c_void;

#[repr(C)]
struct ReturnAddressData {
    level: u32,
    addr: *mut c_void,
}

unsafe extern "C" {
    fn arch_stack_walk(
        callback: unsafe extern "C" fn(*mut c_void, usize) -> bool,
        data: *mut c_void,
        task: *mut c_void,
        bp: *mut c_void,
    );

    static mut current: *mut c_void;
}

unsafe extern "C" fn save_return_addr(d: *mut c_void, pc: usize) -> bool {
    let data = &mut *(d as *mut ReturnAddressData);

    if data.level == 0 {
        data.addr = pc as *mut c_void;
        false
    } else {
        data.level -= 1;
        true
    }
}
// NOKPROBE_SYMBOL(save_return_addr);

pub unsafe extern "C" fn return_address(level: u32) -> *mut c_void {
    let mut data = ReturnAddressData {
        level: level.wrapping_add(2),
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
// EXPORT_SYMBOL_GPL(return_address);
// NOKPROBE_SYMBOL(return_address);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
