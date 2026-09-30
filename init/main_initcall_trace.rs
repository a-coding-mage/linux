// SPDX-License-Identifier: GPL-2.0-only
//! Initcall diagnostic callbacks, registration and trace dispatch.
//!
//! Shared tracing infrastructure is owned by the staged C macro expansion in
//! main_tracepoints.c. Its original SRCU/static-call/LOCKDEP behavior is retained
//! while initcall ordering and probe callbacks are owned here in Rust.

use super::main_globals;
use super::{bindings, main_initcall_types::InitcallFn, main_printk::main_printk};
use kernel::ffi::{c_char, c_int, c_ulonglong, c_void};

/// Register all three original diagnostic callbacks, retaining the first
/// callback registrations even if a later registration fails.
///
/// # Safety
/// The tracepoint owners and shared timer have kernel lifetime. Boot initcall
/// registration is serialized, before the init section can be discarded.
#[cfg(all(CONFIG_TRACEPOINTS, not(NOTRACE)))]
#[link_section = ".init.text"]
pub(super) unsafe fn initcall_debug_enable() {
    // SAFETY: the callback prototypes exactly match each tracepoint's data
    // prototype. The generic registration ABI takes an opaque function address.
    unsafe {
        let calltime = core::ptr::addr_of_mut!(main_globals::initcall_calltime).cast();
        let mut result = bindings::tracepoint_probe_register(
            core::ptr::addr_of_mut!(bindings::__tracepoint_initcall_start),
            trace_initcall_start_cb as *mut c_void,
            calltime,
        );
        result |= bindings::tracepoint_probe_register(
            core::ptr::addr_of_mut!(bindings::__tracepoint_initcall_finish),
            trace_initcall_finish_cb as *mut c_void,
            calltime,
        );
        result |= bindings::tracepoint_probe_register(
            core::ptr::addr_of_mut!(bindings::__tracepoint_initcall_level),
            trace_initcall_level_cb as *mut c_void,
            core::ptr::null_mut(),
        );
        super::main_warn::main_warn!(result != 0, b"Failed to register initcall tracepoints\n\0");
    }
}

/// The original no-tracepoint configuration requires no registration.
#[cfg(any(not(CONFIG_TRACEPOINTS), NOTRACE))]
pub(super) unsafe fn initcall_debug_enable() {}

/// Enter the actual shared tracing infrastructure, including its RCU check even
/// when the tracepoint's static key is disabled.
///
/// # Safety
/// The tracepoint owner has been linked and tracing's RCU context is available.
#[cfg(all(CONFIG_TRACEPOINTS, not(NOTRACE)))]
#[cfg_attr(not(CONFIG_MODULES), link_section = ".init.text")]
pub(super) unsafe fn do_trace_initcall_start(function: InitcallFn) {
    // SAFETY: Some transports the original nonnull initcall through the genuine
    // bindgen prototype; the C owner supplies the original inline expansion.
    unsafe {
        bindings::rust_main_trace_initcall_start(Some(function));
    }
}

/// Dispatch the finish event through the original shared trace implementation.
///
/// # Safety
/// The tracepoint owner and its RCU context are available during this call.
#[cfg(all(CONFIG_TRACEPOINTS, not(NOTRACE)))]
#[cfg_attr(not(CONFIG_MODULES), link_section = ".init.text")]
pub(super) unsafe fn do_trace_initcall_finish(function: InitcallFn, result: c_int) {
    // SAFETY: the bridge's real C prototype carries this function and result.
    unsafe {
        bindings::rust_main_trace_initcall_finish(Some(function), result);
    }
}

/// Dispatch the level event through the original shared trace implementation.
///
/// # Safety
/// `level` is terminated and live; the tracepoint owner and RCU are available.
#[cfg(all(CONFIG_TRACEPOINTS, not(NOTRACE)))]
#[link_section = ".init.text"]
pub(super) unsafe fn do_trace_initcall_level(level: *const c_char) {
    // SAFETY: this preserves the original trace invocation unconditionally.
    unsafe {
        bindings::rust_main_trace_initcall_level(level);
    }
}

/// Log the initcall and save the timestamp after that diagnostic completes.
///
/// # Safety
/// `data` names writable, aligned ktime_t storage owned by the caller. The
/// current task and referenced callback symbol remain live during this call.
#[cfg_attr(not(CONFIG_MODULES), link_section = ".init.text")]
pub(super) unsafe extern "C" fn trace_initcall_start_cb(data: *mut c_void, function: InitcallFn) {
    // SAFETY: current is live in this context; pid is immutable after task
    // initialization. Only raw pointers cross printk and the timer service.
    unsafe {
        let current = kernel::bindings::get_current().cast::<bindings::task_struct>();
        main_printk!(
            "trace_initcall_start_cb",
            b"\x017calling  %pS @ %i\n\0",
            function as *const c_void,
            core::ptr::addr_of!((*current).pid).read()
        );
        data.cast::<bindings::ktime_t>()
            .write(bindings::ktime_get());
    }
}

/// Read the finish time before reading the shared start time and logging.
///
/// # Safety
/// `data` names initialized, aligned ktime_t storage owned by the caller.
#[cfg_attr(not(CONFIG_MODULES), link_section = ".init.text")]
pub(super) unsafe extern "C" fn trace_initcall_finish_cb(
    data: *mut c_void,
    function: InitcallFn,
    result: c_int,
) {
    // SAFETY: the caller supplies the shared start time. C is built with
    // wrapping signed arithmetic; division by positive NSEC_PER_USEC truncates
    // toward zero. The original printk passes unsigned long long to %lld.
    unsafe {
        let end = bindings::ktime_get();
        let microseconds = end.wrapping_sub(data.cast::<bindings::ktime_t>().read())
            / bindings::NSEC_PER_USEC as bindings::ktime_t;
        main_printk!(
            "trace_initcall_finish_cb",
            b"\x017initcall %pS returned %d after %lld usecs\n\0",
            function as *const c_void,
            result,
            microseconds as c_ulonglong
        );
    }
}

/// Log entry into an initcall level.
///
/// # Safety
/// `level` points to a live terminated C string for the duration of the call.
#[cfg_attr(not(CONFIG_MODULES), link_section = ".init.text")]
pub(super) unsafe extern "C" fn trace_initcall_level_cb(_data: *mut c_void, level: *const c_char) {
    // SAFETY: the format matches the caller's live level string.
    #[cfg_attr(not(CONFIG_PRINTK), allow(unused_unsafe))]
    unsafe {
        main_printk!(
            "trace_initcall_level_cb",
            b"\x017entering initcall level: %s\n\0",
            level
        );
    }
}

/// Invoke the original debug fallback when this TU has no tracepoints.
///
/// # Safety
/// The caller serializes boot/module initcall diagnostic state.
#[cfg(any(not(CONFIG_TRACEPOINTS), NOTRACE))]
#[cfg_attr(not(CONFIG_MODULES), link_section = ".init.text")]
pub(super) unsafe fn do_trace_initcall_start(function: InitcallFn) {
    // SAFETY: the sole shared timer owner and callback obey the caller contract.
    unsafe {
        if main_globals::initcall_debug {
            trace_initcall_start_cb(
                core::ptr::addr_of_mut!(main_globals::initcall_calltime).cast(),
                function,
            );
        }
    }
}

/// Invoke the original finish fallback when this TU has no tracepoints.
///
/// # Safety
/// The caller serializes boot/module initcall diagnostic state.
#[cfg(any(not(CONFIG_TRACEPOINTS), NOTRACE))]
#[cfg_attr(not(CONFIG_MODULES), link_section = ".init.text")]
pub(super) unsafe fn do_trace_initcall_finish(function: InitcallFn, result: c_int) {
    // SAFETY: the sole shared timer owner and callback obey the caller contract.
    unsafe {
        if main_globals::initcall_debug {
            trace_initcall_finish_cb(
                core::ptr::addr_of_mut!(main_globals::initcall_calltime).cast(),
                function,
                result,
            );
        }
    }
}

/// Invoke the original level fallback when this TU has no tracepoints.
///
/// # Safety
/// The caller serializes diagnostic state and supplies a live level string.
#[cfg(any(not(CONFIG_TRACEPOINTS), NOTRACE))]
#[link_section = ".init.text"]
pub(super) unsafe fn do_trace_initcall_level(level: *const c_char) {
    // SAFETY: the level callback does not use its null data argument.
    unsafe {
        if main_globals::initcall_debug {
            trace_initcall_level_cb(core::ptr::null_mut(), level);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
