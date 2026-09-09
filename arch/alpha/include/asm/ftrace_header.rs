/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_FRAME_POINTER is a build-time condition preserved from the C header. */
#[cfg(CONFIG_FRAME_POINTER)]
#[inline(never)]
unsafe fn alpha_ftrace_return_address0() -> *mut core::ffi::c_void {
    /* __builtin_return_address(0) has no direct stable Rust equivalent. */
    unsafe { __builtin_return_address(0) }
}

#[cfg(CONFIG_FRAME_POINTER)]
extern "C" {
    fn __builtin_return_address(level: i32) -> *mut core::ffi::c_void;
}

#[cfg(CONFIG_FRAME_POINTER)]
macro_rules! ftrace_return_address0 {
    () => {
        unsafe { alpha_ftrace_return_address0() }
    };
}

/*
 * __builtin_return_address() requires a constant integer argument.
 * Keep this as a macro so the value is seen at the callsite.
 */
#[cfg(CONFIG_FRAME_POINTER)]
macro_rules! ftrace_return_address {
    ($n:expr) => {
        unsafe { __builtin_return_address($n) }
    };
}

/* !CONFIG_FRAME_POINTER */
#[cfg(not(CONFIG_FRAME_POINTER))]
macro_rules! ftrace_return_address0 {
    () => {
        0usize as *mut core::ffi::c_void
    };
}

#[cfg(not(CONFIG_FRAME_POINTER))]
macro_rules! ftrace_return_address {
    ($n:expr) => {{
        let _ = $n;
        0usize as *mut core::ffi::c_void
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
