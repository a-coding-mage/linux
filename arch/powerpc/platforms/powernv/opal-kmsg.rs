// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * kmsg dumper that ensures the OPAL console fully flushes panic messages
 *
 * Author: Russell Currey <ruscur@russell.cc>
 *
 * Copyright 2015 IBM Corporation.
 */

// Dependencies supplied by the kernel headers:
// linux/kmsg_dump.h, asm/opal.h, and asm/opal-api.h.

use core::ffi::c_char;

#[repr(C)]
pub struct kmsg_dumper {
    pub dump: Option<unsafe extern "C" fn(
        dumper: *mut kmsg_dumper,
        detail: *mut kmsg_dump_detail,
    )>,
}

#[repr(C)]
pub struct kmsg_dump_detail {
    pub reason: i32,
}

unsafe extern "C" {
    fn opal_flush_console(cpu: u32);
    fn kmsg_dump_register(dumper: *mut kmsg_dumper) -> i32;
    fn pr_err(fmt: *const c_char, ...);
}

pub const KMSG_DUMP_PANIC: i32 = 1;

/*
 * Console output is controlled by OPAL firmware.  The kernel regularly calls
 * OPAL_POLL_EVENTS, which flushes some console output.  In a panic state,
 * however, the kernel no longer calls OPAL_POLL_EVENTS and the panic message
 * may not be completely printed.  This function does not actually dump the
 * message, it just ensures that OPAL completely flushes the console buffer.
 */
unsafe extern "C" fn kmsg_dump_opal_console_flush(
    _dumper: *mut kmsg_dumper,
    detail: *mut kmsg_dump_detail,
) {
    /*
     * Outside of a panic context the pollers will continue to run,
     * so we don't need to do any special flushing.
     */
    if (*detail).reason != KMSG_DUMP_PANIC {
        return;
    }

    opal_flush_console(0);
}

static mut opal_kmsg_dumper: kmsg_dumper = kmsg_dumper {
    dump: Some(kmsg_dump_opal_console_flush),
};

pub unsafe extern "C" fn opal_kmsg_init() {
    let rc: i32;

    /* Add our dumper to the list */
    rc = kmsg_dump_register(&raw mut opal_kmsg_dumper);
    if rc != 0 {
        pr_err(
            b"opal: kmsg_dump_register failed; returned %d\n\0".as_ptr() as *const c_char,
            rc,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
