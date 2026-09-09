/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/perf_event.h and uapi/linux/hw_breakpoint.h dependencies.

#[cfg(feature = "CONFIG_HAVE_HW_BREAKPOINT")]
#[repr(C)]
pub enum bp_type_idx {
    TYPE_INST = 0,
    #[cfg(feature = "CONFIG_HAVE_MIXED_BREAKPOINTS_REGS")]
    TYPE_DATA = 0,
    #[cfg(not(feature = "CONFIG_HAVE_MIXED_BREAKPOINTS_REGS"))]
    TYPE_DATA = 1,
    TYPE_MAX,
}

#[cfg(feature = "CONFIG_HAVE_HW_BREAKPOINT")]
extern "C" {
    pub fn init_hw_breakpoint() -> ::core::ffi::c_int;
}

#[cfg(feature = "CONFIG_HAVE_HW_BREAKPOINT")]
pub unsafe fn hw_breakpoint_init(attr: *mut perf_event_attr) {
    ::core::ptr::write_bytes(attr.cast::<u8>(), 0, ::core::mem::size_of::<perf_event_attr>());
    (*attr).type_ = PERF_TYPE_BREAKPOINT;
    (*attr).size = ::core::mem::size_of::<perf_event_attr>();
    // As it's for in-kernel or ptrace use, we want it to be pinned
    // and to call its callback every hits.
    (*attr).pinned = 1;
    (*attr).sample_period = 1;
}

#[cfg(feature = "CONFIG_HAVE_HW_BREAKPOINT")]
pub unsafe fn ptrace_breakpoint_init(attr: *mut perf_event_attr) {
    hw_breakpoint_init(attr);
    (*attr).exclude_kernel = 1;
}

#[cfg(feature = "CONFIG_HAVE_HW_BREAKPOINT")]
pub unsafe fn hw_breakpoint_addr(bp: *mut perf_event) -> ::core::ffi::c_ulong {
    (*bp).attr.bp_addr
}

#[cfg(feature = "CONFIG_HAVE_HW_BREAKPOINT")]
pub unsafe fn hw_breakpoint_type(bp: *mut perf_event) -> ::core::ffi::c_int {
    (*bp).attr.bp_type
}

#[cfg(feature = "CONFIG_HAVE_HW_BREAKPOINT")]
pub unsafe fn hw_breakpoint_len(bp: *mut perf_event) -> ::core::ffi::c_ulong {
    (*bp).attr.bp_len
}

#[cfg(feature = "CONFIG_HAVE_HW_BREAKPOINT")]
extern "C" {
    pub fn register_user_hw_breakpoint(
        attr: *mut perf_event_attr,
        triggered: perf_overflow_handler_t,
        context: *mut ::core::ffi::c_void,
        tsk: *mut task_struct,
    ) -> *mut perf_event;
    // FIXME: only change from the attr, and don't unregister
    pub fn modify_user_hw_breakpoint(bp: *mut perf_event, attr: *mut perf_event_attr) -> ::core::ffi::c_int;
    pub fn modify_user_hw_breakpoint_check(
        bp: *mut perf_event,
        attr: *mut perf_event_attr,
        check: bool,
    ) -> ::core::ffi::c_int;
    // Kernel breakpoints are not associated with any particular thread.
    pub fn register_wide_hw_breakpoint_cpu(
        attr: *mut perf_event_attr,
        triggered: perf_overflow_handler_t,
        context: *mut ::core::ffi::c_void,
        cpu: ::core::ffi::c_int,
    ) -> *mut perf_event;
    pub fn register_wide_hw_breakpoint(
        attr: *mut perf_event_attr,
        triggered: perf_overflow_handler_t,
        context: *mut ::core::ffi::c_void,
    ) -> *mut perf_event;
    pub fn register_perf_hw_breakpoint(bp: *mut perf_event) -> ::core::ffi::c_int;
    pub fn unregister_hw_breakpoint(bp: *mut perf_event);
    pub fn unregister_wide_hw_breakpoint(cpu_events: *mut *mut perf_event);
    pub fn hw_breakpoint_is_used() -> bool;
    pub fn dbg_reserve_bp_slot(bp: *mut perf_event) -> ::core::ffi::c_int;
    pub fn dbg_release_bp_slot(bp: *mut perf_event) -> ::core::ffi::c_int;
    pub fn reserve_bp_slot(bp: *mut perf_event) -> ::core::ffi::c_int;
    pub fn release_bp_slot(bp: *mut perf_event);
    pub fn flush_ptrace_hw_breakpoint(tsk: *mut task_struct);
}

#[cfg(feature = "CONFIG_HAVE_HW_BREAKPOINT")]
pub unsafe fn counter_arch_bp(bp: *mut perf_event) -> *mut arch_hw_breakpoint {
    &mut (*bp).hw.info
}

#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn init_hw_breakpoint() -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn register_user_hw_breakpoint(_: *mut perf_event_attr, _: perf_overflow_handler_t, _: *mut ::core::ffi::c_void, _: *mut task_struct) -> *mut perf_event { ::core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn modify_user_hw_breakpoint(_: *mut perf_event, _: *mut perf_event_attr) -> ::core::ffi::c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn modify_user_hw_breakpoint_check(_: *mut perf_event, _: *mut perf_event_attr, _: bool) -> ::core::ffi::c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn register_wide_hw_breakpoint_cpu(_: *mut perf_event_attr, _: perf_overflow_handler_t, _: *mut ::core::ffi::c_void, _: ::core::ffi::c_int) -> *mut perf_event { ::core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn register_wide_hw_breakpoint(_: *mut perf_event_attr, _: perf_overflow_handler_t, _: *mut ::core::ffi::c_void) -> *mut perf_event { ::core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn register_perf_hw_breakpoint(_: *mut perf_event) -> ::core::ffi::c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn unregister_hw_breakpoint(_: *mut perf_event) {}
#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn unregister_wide_hw_breakpoint(_: *mut *mut perf_event) {}
#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn hw_breakpoint_is_used() -> bool { false }
#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn reserve_bp_slot(_: *mut perf_event) -> ::core::ffi::c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn release_bp_slot(_: *mut perf_event) {}
#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn flush_ptrace_hw_breakpoint(_: *mut task_struct) {}
#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
pub unsafe fn counter_arch_bp(_: *mut perf_event) -> *mut arch_hw_breakpoint { ::core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
