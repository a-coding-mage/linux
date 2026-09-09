/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the C header. Include guards and includes are omitted.

extern "C" {
    pub fn __register_ftrace_function(ops: *mut crate::ftrace_ops) -> ::core::ffi::c_int;
    pub fn __unregister_ftrace_function(ops: *mut crate::ftrace_ops) -> ::core::ffi::c_int;
}

// CONFIG_FUNCTION_TRACER
extern "C" {
    pub static mut ftrace_lock: crate::mutex;
    pub static mut global_ops: crate::ftrace_ops;
}

// CONFIG_DYNAMIC_FTRACE
#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
extern "C" {
    pub fn ftrace_startup(
        ops: *mut crate::ftrace_ops,
        command: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn ftrace_shutdown(
        ops: *mut crate::ftrace_ops,
        command: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn ftrace_ops_test(
        ops: *mut crate::ftrace_ops,
        ip: ::core::ffi::c_ulong,
        regs: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn ftrace_startup_subops(
        ops: *mut crate::ftrace_ops,
        subops: *mut crate::ftrace_ops,
        command: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn ftrace_shutdown_subops(
        ops: *mut crate::ftrace_ops,
        subops: *mut crate::ftrace_ops,
        command: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// !CONFIG_DYNAMIC_FTRACE: the C macros are represented as inline Rust functions.
#[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE"))]
pub unsafe fn ftrace_startup(
    ops: *mut crate::ftrace_ops,
    _command: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let ret = unsafe { __register_ftrace_function(ops) };
    if ret == 0 {
        unsafe { (*ops).flags |= crate::FTRACE_OPS_FL_ENABLED };
    }
    ret
}

#[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE"))]
pub unsafe fn ftrace_shutdown(
    ops: *mut crate::ftrace_ops,
    _command: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let ret = unsafe { __unregister_ftrace_function(ops) };
    if ret == 0 {
        unsafe { (*ops).flags &= !crate::FTRACE_OPS_FL_ENABLED };
    }
    ret
}

#[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE"))]
pub unsafe fn ftrace_ops_test(
    _ops: *mut crate::ftrace_ops,
    _ip: ::core::ffi::c_ulong,
    _regs: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    1
}

#[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE"))]
pub unsafe fn ftrace_startup_subops(
    _ops: *mut crate::ftrace_ops,
    _subops: *mut crate::ftrace_ops,
    _command: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    -crate::EINVAL
}

#[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE"))]
pub unsafe fn ftrace_shutdown_subops(
    _ops: *mut crate::ftrace_ops,
    _subops: *mut crate::ftrace_ops,
    _command: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    -crate::EINVAL
}

// CONFIG_FUNCTION_GRAPH_TRACER
#[cfg(feature = "CONFIG_FUNCTION_GRAPH_TRACER")]
extern "C" {
    pub static mut ftrace_graph_active: ::core::ffi::c_int;
}

#[cfg(all(
    feature = "CONFIG_FUNCTION_GRAPH_TRACER",
    feature = "CONFIG_DYNAMIC_FTRACE"
))]
extern "C" {
    pub fn fgraph_update_pid_func();
}

#[cfg(any(
    not(feature = "CONFIG_FUNCTION_GRAPH_TRACER"),
    not(feature = "CONFIG_DYNAMIC_FTRACE")
))]
pub unsafe fn fgraph_update_pid_func() {}

#[cfg(not(feature = "CONFIG_FUNCTION_GRAPH_TRACER"))]
pub const ftrace_graph_active: ::core::ffi::c_int = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
