// SPDX-License-Identifier: GPL-2.0
/*
 * nop tracer
 *
 * Copyright (C) 2008 Steven Noonan <steven@uplinklabs.net>
 *
 */

// C dependencies: <linux/module.h>, <linux/ftrace.h>, and "trace.h".
// The corresponding kernel types, constants, and functions are supplied by
// the surrounding Rust translation unit.
use crate::{printk, trace_array, trace_selftest_startup_nop, tracer, tracer_flags, tracer_opt, u32, EINVAL, KERN_DEBUG};

/* Our two options */
const TRACE_NOP_OPT_ACCEPT: u32 = 0x1;
const TRACE_NOP_OPT_REFUSE: u32 = 0x2;

/* Options for the tracer (see trace_options file) */
static mut NOP_OPTS: [tracer_opt; 3] = [
    /* Option that will be accepted by set_flag callback */
    tracer_opt {
        name: "test_nop_accept",
        bit: TRACE_NOP_OPT_ACCEPT,
    },
    /* Option that will be refused by set_flag callback */
    tracer_opt {
        name: "test_nop_refuse",
        bit: TRACE_NOP_OPT_REFUSE,
    },
    tracer_opt {
        name: "",
        bit: 0,
    }, /* Always set a last empty entry */
];

static mut NOP_FLAGS: tracer_flags = tracer_flags {
    /* You can check your flags value here when you want. */
    val: 0, /* By default: all flags disabled */
    opts: unsafe { &mut NOP_OPTS as *mut [tracer_opt; 3] as *mut tracer_opt },
};

static mut CTX_TRACE: *mut trace_array = core::ptr::null_mut();

unsafe fn start_nop_trace(_tr: *mut trace_array) {
    /* Nothing to do! */
}

unsafe fn stop_nop_trace(_tr: *mut trace_array) {
    /* Nothing to do! */
}

unsafe extern "C" fn nop_trace_init(tr: *mut trace_array) -> i32 {
    CTX_TRACE = tr;
    start_nop_trace(tr);
    0
}

unsafe extern "C" fn nop_trace_reset(tr: *mut trace_array) {
    stop_nop_trace(tr);
}

/* It only serves as a signal handler and a callback to
 * accept or refuse the setting of a flag.
 * If you don't implement it, then the flag setting will be
 * automatically accepted.
 */
unsafe extern "C" fn nop_set_flag(
    _tr: *mut trace_array,
    _old_flags: u32,
    bit: u32,
    set: i32,
) -> i32 {
    /*
     * Note that you don't need to update nop_flags.val yourself.
     * The tracing Api will do it automatically if you return 0
     */
    if bit == TRACE_NOP_OPT_ACCEPT {
        printk(
            KERN_DEBUG,
            "nop_test_accept flag set to %d: we accept. Now cat trace_options to see the result\n",
            set,
        );
        return 0;
    }

    if bit == TRACE_NOP_OPT_REFUSE {
        printk(
            KERN_DEBUG,
            "nop_test_refuse flag set to %d: we refuse. Now cat trace_options to see the result\n",
            set,
        );
        return -EINVAL;
    }

    0
}

#[no_mangle]
pub static mut nop_trace: tracer = tracer {
    name: "nop",
    init: Some(nop_trace_init),
    reset: Some(nop_trace_reset),
    #[cfg(CONFIG_FTRACE_SELFTEST)]
    selftest: Some(trace_selftest_startup_nop),
    flags: unsafe { &mut NOP_FLAGS as *mut tracer_flags },
    set_flag: Some(nop_set_flag),
    allow_instances: true,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
