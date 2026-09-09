/* SPDX-License-Identifier: GPL-2.0 */

/* TRACE_SYSTEM context_tracking */
/*
 * The C header guard and TRACE_HEADER_MULTI_READ condition protect the
 * tracepoint declarations during preprocessing.  Rust items in this file
 * are emitted once by the module containing this translation.
 */

/// Payload shared by the `user_enter` and `user_exit` trace events.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ContextTrackingUser {
    pub dummy: ::core::ffi::c_int,
}

impl ContextTrackingUser {
    #[inline]
    pub const unsafe fn from_dummy(dummy: ::core::ffi::c_int) -> Self {
        Self { dummy }
    }
}

/*
 * DECLARE_EVENT_CLASS(context_tracking_user, TP_PROTO(int dummy),
 * TP_ARGS(dummy), __field(int, dummy), __entry->dummy = dummy,
 * TP_printk("%s", ""));
 */

/**
 * user_enter - called when the kernel resumes to userspace
 * @dummy: dummy arg to make trace event macro happy
 *
 * This event occurs when the kernel resumes to userspace after an exception
 * or a syscall.
 */
#[repr(C)]
pub struct UserEnterEvent {
    pub dummy: ::core::ffi::c_int,
}

/**
 * user_exit - called when userspace enters the kernel
 * @dummy: dummy arg to make trace event macro happy
 *
 * This event occurs when userspace enters the kernel through an exception or
 * a syscall.
 */
#[repr(C)]
pub struct UserExitEvent {
    pub dummy: ::core::ffi::c_int,
}

/* DEFINE_EVENT(context_tracking_user, user_enter, TP_PROTO(int dummy),
 *              TP_ARGS(dummy));
 * DEFINE_EVENT(context_tracking_user, user_exit, TP_PROTO(int dummy),
 *              TP_ARGS(dummy));
 *
 * <trace/define_trace.h> supplies the tracepoint definitions in C and is an
 * external dependency of this translation.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
