/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019-2022 Red Hat, Inc. Daniel Bristot de Oliveira <bristot@kernel.org>
 *
 * Helper functions to facilitate the instrumentation of auto-generated
 * RV monitors create by dot2k.
 *
 * The dot2k tool is available at tools/verification/dot2/
 */

// Dependency supplied by the Linux tracing subsystem:
// #include <linux/ftrace.h>

/*
 * rv_attach_trace_probe - check and attach a handler function to a tracepoint
 *
 * Rust has no stable identifier-token pasting equivalent for the C spelling
 * check_trace_callback_type_##tp and register_trace_##tp.  The generated
 * tracepoint symbols are therefore passed as path arguments while retaining
 * the same ordering and side effects.
 */
#[macro_export]
macro_rules! rv_attach_trace_probe {
    ($monitor:ident, $tp:ident, $rv_handler:expr, $check_trace_callback_type:path, $register_trace:path) => {{
        $check_trace_callback_type($rv_handler);
        if $register_trace($rv_handler, core::ptr::null_mut()) != 0 {
            // Equivalent to WARN_ONCE(register_trace_##tp(...), ...).
            // The kernel WARN_ONCE implementation is supplied externally.
            $crate::WARN_ONCE(
                true,
                concat!("fail attaching ", stringify!($monitor), " ", stringify!($tp), "handler"),
            );
        }
    }};
}

/*
 * rv_detach_trace_probe - detach a handler function to a tracepoint
 */
#[macro_export]
macro_rules! rv_detach_trace_probe {
    ($monitor:ident, $tp:ident, $rv_handler:expr, $unregister_trace:path) => {{
        $unregister_trace($rv_handler, core::ptr::null_mut());
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
