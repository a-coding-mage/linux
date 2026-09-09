/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/kcov.h. */

/* Dependencies supplied by the surrounding kernel translation. */

#[cfg(CONFIG_KCOV)]
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum kcov_mode {
    /* Coverage collection is not enabled yet. */
    KCOV_MODE_DISABLED = 0,
    /* KCOV was initialized, but tracing mode hasn't been chosen yet. */
    KCOV_MODE_INIT = 1,
    /*
     * Tracing coverage collection mode.
     * Covered PCs are collected in a per-task buffer.
     */
    KCOV_MODE_TRACE_PC = 2,
    /* Collecting comparison operands mode. */
    KCOV_MODE_TRACE_CMP = 3,
}

#[cfg(CONFIG_KCOV)]
pub const KCOV_IN_CTXSW: u32 = 1u32 << 30;

#[cfg(CONFIG_KCOV)]
pub unsafe extern "C" {
    pub fn kcov_task_init(t: *mut task_struct);
    pub fn kcov_task_exit(t: *mut task_struct);
}

#[cfg(CONFIG_KCOV)]
macro_rules! kcov_prepare_switch {
    ($t:expr) => {{
        unsafe { (*$t).kcov_mode |= KCOV_IN_CTXSW; }
    }};
}

#[cfg(CONFIG_KCOV)]
macro_rules! kcov_finish_switch {
    ($t:expr) => {{
        unsafe { (*$t).kcov_mode &= !KCOV_IN_CTXSW; }
    }};
}

/* See Documentation/dev-tools/kcov.rst for usage details. */
#[cfg(CONFIG_KCOV)]
pub unsafe extern "C" {
    pub fn kcov_remote_start(handle: u64);
    pub fn kcov_remote_stop();
    pub fn kcov_common_handle() -> kcov_common_handle_id;
}

#[cfg(CONFIG_KCOV)]
#[inline]
pub unsafe fn kcov_remote_start_common(id: kcov_common_handle_id) {
    kcov_remote_start(kcov_remote_handle(KCOV_SUBSYSTEM_COMMON, id.val));
}

#[cfg(CONFIG_KCOV)]
#[inline]
pub unsafe fn kcov_remote_start_usb(id: u64) {
    kcov_remote_start(kcov_remote_handle(KCOV_SUBSYSTEM_USB, id));
}

/*
 * The softirq flavor of kcov_remote_*() functions is introduced as a temporary
 * work around for kcov's lack of nested remote coverage sections support in
 * task context. Adding support for nested sections is tracked in:
 * https://bugzilla.kernel.org/show_bug.cgi?id=210337
 */
#[cfg(CONFIG_KCOV)]
#[inline]
pub unsafe fn kcov_remote_start_usb_softirq(id: u64) {
    if in_serving_softirq() && !in_hardirq() {
        kcov_remote_start_usb(id);
    }
}

#[cfg(CONFIG_KCOV)]
#[inline]
pub unsafe fn kcov_remote_stop_softirq() {
    if in_serving_softirq() && !in_hardirq() {
        kcov_remote_stop();
    }
}

#[cfg(CONFIG_64BIT)]
pub type kcov_u64 = usize;
#[cfg(not(CONFIG_64BIT))]
pub type kcov_u64 = u64;

#[cfg(CONFIG_KCOV)]
pub unsafe extern "C" {
    pub fn __sanitizer_cov_trace_pc();
    pub fn __sanitizer_cov_trace_cmp1(arg1: u8, arg2: u8);
    pub fn __sanitizer_cov_trace_cmp2(arg1: u16, arg2: u16);
    pub fn __sanitizer_cov_trace_cmp4(arg1: u32, arg2: u32);
    pub fn __sanitizer_cov_trace_cmp8(arg1: kcov_u64, arg2: kcov_u64);
    pub fn __sanitizer_cov_trace_const_cmp1(arg1: u8, arg2: u8);
    pub fn __sanitizer_cov_trace_const_cmp2(arg1: u16, arg2: u16);
    pub fn __sanitizer_cov_trace_const_cmp4(arg1: u32, arg2: u32);
    pub fn __sanitizer_cov_trace_const_cmp8(arg1: kcov_u64, arg2: kcov_u64);
    pub fn __sanitizer_cov_trace_switch(val: kcov_u64, cases: *mut core::ffi::c_void);
}

#[cfg(not(CONFIG_KCOV))]
#[inline]
pub unsafe fn kcov_task_init(_t: *mut task_struct) {}
#[cfg(not(CONFIG_KCOV))]
#[inline]
pub unsafe fn kcov_task_exit(_t: *mut task_struct) {}
#[cfg(not(CONFIG_KCOV))]
#[inline]
pub unsafe fn kcov_prepare_switch(_t: *mut task_struct) {}
#[cfg(not(CONFIG_KCOV))]
#[inline]
pub unsafe fn kcov_finish_switch(_t: *mut task_struct) {}
#[cfg(not(CONFIG_KCOV))]
#[inline]
pub unsafe fn kcov_remote_start(_handle: u64) {}
#[cfg(not(CONFIG_KCOV))]
#[inline]
pub unsafe fn kcov_remote_stop() {}
#[cfg(not(CONFIG_KCOV))]
#[inline]
pub unsafe fn kcov_common_handle() -> kcov_common_handle_id {
    core::mem::zeroed()
}
#[cfg(not(CONFIG_KCOV))]
#[inline]
pub unsafe fn kcov_remote_start_common(_id: kcov_common_handle_id) {}
#[cfg(not(CONFIG_KCOV))]
#[inline]
pub unsafe fn kcov_remote_start_usb(_id: u64) {}
#[cfg(not(CONFIG_KCOV))]
#[inline]
pub unsafe fn kcov_remote_start_usb_softirq(_id: u64) {}
#[cfg(not(CONFIG_KCOV))]
#[inline]
pub unsafe fn kcov_remote_stop_softirq() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
