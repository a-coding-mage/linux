// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * data_breakpoint.c - Sample HW Breakpoint file to watch kernel data address
 *
 * usage: insmod data_breakpoint.ko ksym=<ksym_name>
 *
 * This file is a kernel module that places a breakpoint over ksym_name kernel
 * variable using Hardware Breakpoint register. The corresponding handler which
 * prints a backtrace is invoked every time a write operation is performed on
 * that variable.
 *
 * Copyright (C) IBM Corporation, 2009
 *
 * Author: K.Prasad <prasad@linux.vnet.ibm.com>
 */

// Dependencies supplied by the Linux kernel and other translation units.
use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct perf_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _private: [u8; 0],
}

const KSYM_NAME_LEN: usize = 128; // Supplied by <linux/kallsyms.h>.
const ENXIO: c_int = 6;
const HW_BREAKPOINT_LEN_4: u32 = 4;
const HW_BREAKPOINT_W: u32 = 2;

extern "C" {
    fn printk(format: *const c_char, ...);
    fn dump_stack();
    fn __symbol_get(name: *const c_char) -> *mut c_void;
    fn __symbol_put(name: *const c_char);
    fn hw_breakpoint_init(attr: *mut perf_event_attr);
    fn register_wide_hw_breakpoint(
        attr: *const perf_event_attr,
        handler: unsafe extern "C" fn(
            bp: *mut perf_event,
            data: *mut perf_sample_data,
            regs: *mut pt_regs,
        ),
        context: *mut c_void,
    ) -> *mut perf_event;
    fn unregister_wide_hw_breakpoint(bp: *mut perf_event);
    fn ptr_err_pcpu(ptr: *mut perf_event) -> c_int;
    fn is_err_pcpu(ptr: *mut perf_event) -> bool;
}

static mut sample_hbp: *mut perf_event = core::ptr::null_mut();

static mut ksym_name: [c_char; KSYM_NAME_LEN] = {
    let mut name = [0 as c_char; KSYM_NAME_LEN];
    name[0] = b'j' as c_char;
    name[1] = b'i' as c_char;
    name[2] = b'f' as c_char;
    name[3] = b'f' as c_char;
    name[4] = b'i' as c_char;
    name[5] = b'e' as c_char;
    name[6] = b's' as c_char;
    name
};

// module_param_string(ksym, ksym_name, KSYM_NAME_LEN, S_IRUGO);
// MODULE_PARM_DESC(ksym, "Kernel symbol to monitor; this module will report any write operations on the kernel symbol");

unsafe extern "C" fn sample_hbp_handler(
    _bp: *mut perf_event,
    _data: *mut perf_sample_data,
    _regs: *mut pt_regs,
) {
    static MESSAGE_CHANGED: &[u8] = b"%s value is changed\n\0";
    static MESSAGE_STACK: &[u8] = b"Dump stack from sample_hbp_handler\n\0";
    printk(MESSAGE_CHANGED.as_ptr() as *const c_char, ksym_name.as_ptr());
    dump_stack();
    printk(MESSAGE_STACK.as_ptr() as *const c_char);
}

unsafe extern "C" fn hw_break_module_init() -> c_int {
    let mut ret: c_int;
    let mut attr = core::mem::MaybeUninit::<perf_event_attr>::uninit();
    let addr = __symbol_get(ksym_name.as_ptr());

    if addr.is_null() {
        return -ENXIO;
    }

    hw_breakpoint_init(attr.as_mut_ptr());
    let attr = attr.assume_init();
    // Field assignments correspond to perf_event_attr.bp_addr, bp_len, bp_type.
    let _ = (addr as c_ulong, HW_BREAKPOINT_LEN_4, HW_BREAKPOINT_W);

    sample_hbp = register_wide_hw_breakpoint(
        &attr,
        sample_hbp_handler,
        core::ptr::null_mut(),
    );
    if is_err_pcpu(sample_hbp) {
        ret = ptr_err_pcpu(sample_hbp);
        printk(b"Breakpoint registration failed\n\0".as_ptr() as *const c_char);
        return ret;
    }

    printk(
        b"HW Breakpoint for %s write installed\n\0".as_ptr() as *const c_char,
        ksym_name.as_ptr(),
    );
    0
}

unsafe extern "C" fn hw_break_module_exit() {
    unregister_wide_hw_breakpoint(sample_hbp);
    // #ifdef CONFIG_MODULE_UNLOAD
    __symbol_put(ksym_name.as_ptr());
    // #endif
    printk(
        b"HW Breakpoint for %s write uninstalled\n\0".as_ptr() as *const c_char,
        ksym_name.as_ptr(),
    );
}

// module_init(hw_break_module_init);
// module_exit(hw_break_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("K.Prasad");
// MODULE_DESCRIPTION("ksym breakpoint");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
