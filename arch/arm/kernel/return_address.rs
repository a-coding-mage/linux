// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/kernel/return_address.c
 *
 * Copyright (C) 2009 Uwe Kleine-Koenig <u.kleine-koenig@pengutronix.de>
 * for Pengutronix
 */

use core::ffi::c_void;

#[repr(C)]
struct ReturnAddressData {
    level: u32,
    addr: *mut c_void,
}

// Supplied by the ARM stacktrace implementation.
#[repr(C)]
pub struct Stackframe {
    pub fp: usize,
    pub sp: usize,
    pub lr: usize,
    pub pc: usize,
    // CONFIG_KRETPROBES adds these fields in the corresponding C build.
    #[cfg(feature = "CONFIG_KRETPROBES")]
    pub kr_cur: *mut c_void,
    #[cfg(feature = "CONFIG_KRETPROBES")]
    pub tsk: *mut c_void,
    pub ex_frame: bool,
}

extern "C" {
    fn walk_stackframe(
        frame: *mut Stackframe,
        fn_: unsafe extern "C" fn(*mut c_void, usize) -> bool,
        data: *mut c_void,
    );
    static current_stack_pointer: usize;
    static current: *mut c_void;
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

pub unsafe fn return_address(level: u32) -> *mut c_void {
    let mut data = ReturnAddressData {
        level: level.wrapping_add(2),
        addr: core::ptr::null_mut(),
    };
    let mut frame: Stackframe;

    frame.fp = 0;
    frame.sp = current_stack_pointer;
    frame.lr = 0;
    // The C source uses the address of the local label `here` as the current PC.
    frame.pc = return_address as usize;
    #[cfg(feature = "CONFIG_KRETPROBES")]
    {
        frame.kr_cur = core::ptr::null_mut();
        frame.tsk = current;
    }
    frame.ex_frame = false;

    walk_stackframe(
        &mut frame,
        save_return_addr,
        &mut data as *mut ReturnAddressData as *mut c_void,
    );

    if data.level == 0 {
        data.addr
    } else {
        core::ptr::null_mut()
    }
}

// EXPORT_SYMBOL_GPL(return_address);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
