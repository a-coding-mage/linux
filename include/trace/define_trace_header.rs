/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of the tracepoint-generation preprocessor header.
 *
 * The C header is active only when CREATE_TRACE_POINTS is defined.  Its
 * include and macro-redefinition operations are retained here as comments;
 * the trace-definition macros are expressed as Rust forwarding macros.
 */

/* CREATE_TRACE_POINTS: prevent recursion by undefining it while processing. */

/* The following names are supplied by the tracepoint implementation. */

#[macro_export]
macro_rules! TRACE_EVENT {
    ($name:ident, $proto:tt, $args:tt, $tstruct:tt, $assign:tt, $print:tt) => {
        DEFINE_TRACE!($name, $proto, $args);
    };
}

#[macro_export]
macro_rules! TRACE_EVENT_CONDITION {
    ($name:ident, $proto:tt, $args:tt, $cond:tt, $tstruct:tt, $assign:tt, $print:tt) => {
        TRACE_EVENT!($name, $proto, $args, $tstruct, $assign, $print);
    };
}

#[macro_export]
macro_rules! TRACE_EVENT_FN {
    ($name:ident, $proto:tt, $args:tt, $tstruct:tt, $assign:tt, $print:tt, $reg:tt, $unreg:tt) => {
        DEFINE_TRACE_FN!($name, $reg, $unreg, $proto, $args);
    };
}

#[macro_export]
macro_rules! TRACE_EVENT_FN_COND {
    ($name:ident, $proto:tt, $args:tt, $cond:tt, $tstruct:tt, $assign:tt, $print:tt, $reg:tt, $unreg:tt) => {
        DEFINE_TRACE_FN!($name, $reg, $unreg, $proto, $args);
    };
}

#[macro_export]
macro_rules! TRACE_EVENT_SYSCALL {
    ($name:ident, $proto:tt, $args:tt, $struct_:tt, $assign:tt, $print:tt, $reg:tt, $unreg:tt) => {
        DEFINE_TRACE_SYSCALL!($name, $reg, $unreg, $proto, $args);
    };
}

#[macro_export]
macro_rules! TRACE_EVENT_NOP {
    ($name:ident, $proto:tt, $args:tt, $struct_:tt, $assign:tt, $print:tt) => {};
}

#[macro_export]
macro_rules! DEFINE_EVENT_NOP {
    ($template:ident, $name:ident, $proto:tt, $args:tt) => {};
}

#[macro_export]
macro_rules! DEFINE_EVENT {
    ($template:ident, $name:ident, $proto:tt, $args:tt) => {
        DEFINE_TRACE!($name, $proto, $args);
    };
}

#[macro_export]
macro_rules! DEFINE_EVENT_FN {
    ($template:ident, $name:ident, $proto:tt, $args:tt, $reg:tt, $unreg:tt) => {
        DEFINE_TRACE_FN!($name, $reg, $unreg, $proto, $args);
    };
}

#[macro_export]
macro_rules! DEFINE_EVENT_PRINT {
    ($template:ident, $name:ident, $proto:tt, $args:tt, $print:tt) => {
        DEFINE_TRACE!($name, $proto, $args);
    };
}

#[macro_export]
macro_rules! DEFINE_EVENT_CONDITION {
    ($template:ident, $name:ident, $proto:tt, $args:tt, $cond:tt) => {
        DEFINE_EVENT!($template, $name, $proto, $args);
    };
}

#[macro_export]
macro_rules! DECLARE_TRACE {
    ($name:ident, $proto:tt, $args:tt) => {
        DEFINE_TRACE!($name##_tp, $proto, $args);
    };
}

#[macro_export]
macro_rules! DECLARE_TRACE_CONDITION {
    ($name:ident, $proto:tt, $args:tt, $cond:tt) => {
        DEFINE_TRACE!($name##_tp, $proto, $args);
    };
}

#[macro_export]
macro_rules! DECLARE_TRACE_EVENT {
    ($name:ident, $proto:tt, $args:tt) => {
        DEFINE_TRACE!($name, $proto, $args);
    };
}

#[macro_export]
macro_rules! DECLARE_TRACE_EVENT_CONDITION {
    ($name:ident, $proto:tt, $args:tt, $cond:tt) => {
        DEFINE_TRACE!($name, $proto, $args);
    };
}

/* CREATE_RUST_TRACE_POINTS: define the Rust trace helper when requested. */
#[macro_export]
macro_rules! DEFINE_RUST_DO_TRACE {
    ($name:ident, $proto:tt, $args:tt) => {
        __DEFINE_RUST_DO_TRACE!($name, $proto, $args);
    };
}

/* TRACE_INCLUDE_FILE defaults to TRACE_SYSTEM; TRACE_INCLUDE_PATH controls
 * the generated trace/events include path.  The corresponding C includes
 * are intentionally not executable Rust dependencies. */
/* #include TRACE_INCLUDE(TRACE_INCLUDE_FILE) */
/* TRACE_HEADER_MULTI_READ */
/* TRACEPOINTS_ENABLED: include trace/trace_events.h, trace/perf.h,
 * and trace/bpf_probe.h. */

/* The C header undefines all temporary macros here, then restores
 * CREATE_TRACE_POINTS for subsequent files. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
