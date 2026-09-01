// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2024 Marcos Paulo de Souza <mpdesouza@suse.com>
// Copyright (C) 2024 Michael Vetter <mvetter@suse.com>

// Dependencies from C includes:
// linux/kernel.h
// linux/module.h
// linux/kprobes.h

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_ulong};

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

pub type kprobe_opcode_t = core::ffi::c_void;

#[repr(C)]
pub struct kprobe {
    pub addr: *mut kprobe_opcode_t,
    pub symbol_name: *const c_char,
    pub offset: c_ulong,
    pub pre_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs) -> core::ffi::c_int>,
    pub post_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs, c_ulong)>,
    pub fault_handler:
        Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs, core::ffi::c_int) -> core::ffi::c_int>,
    pub breakpoint_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs) -> core::ffi::c_int>,
    pub opcode: kprobe_opcode_t,
}

unsafe extern "C" {
    fn register_kprobe(p: *mut kprobe) -> core::ffi::c_int;
    fn unregister_kprobe(p: *mut kprobe);
}

static mut has_post_handler: bool = true;
// module_param(has_post_handler, bool, 0444);

unsafe extern "C" fn post_handler(_p: *mut kprobe, _regs: *mut pt_regs, _flags: c_ulong) {}

static CMDLINE_PROC_SHOW: &[u8] = b"cmdline_proc_show\0";

static mut kp: kprobe = kprobe {
    symbol_name: CMDLINE_PROC_SHOW.as_ptr().cast::<c_char>(),
    addr: core::ptr::null_mut(),
    offset: 0,
    pre_handler: None,
    post_handler: None,
    fault_handler: None,
    breakpoint_handler: None,
    opcode: (),
};

unsafe extern "C" fn kprobe_init() -> core::ffi::c_int {
    if unsafe { has_post_handler } {
        unsafe {
            kp.post_handler = Some(post_handler);
        }
    }

    unsafe { register_kprobe(core::ptr::addr_of_mut!(kp)) }
}

unsafe extern "C" fn kprobe_exit() {
    unsafe {
        unregister_kprobe(core::ptr::addr_of_mut!(kp));
    }
}

// module_init(kprobe_init)
// module_exit(kprobe_exit)
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Michael Vetter <mvetter@suse.com>");
// MODULE_DESCRIPTION("Livepatch test: kprobe function");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
