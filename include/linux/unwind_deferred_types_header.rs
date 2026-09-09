/* SPDX-License-Identifier: GPL-2.0 */

/* Required declarations from <linux/types.h> and <linux/atomic.h>. */

#[repr(C)]
pub struct unwind_cache {
    pub unwind_completed: ::core::ffi::c_ulong,
    pub nr_entries: ::core::ffi::c_uint,
    pub entries: [::core::ffi::c_ulong; 0],
}

/*
 * The unwind_task_id is a unique identifier that maps to a user space
 * stacktrace. It is generated the first time a deferred user space
 * stacktrace is requested after a task has entered the kerenl and
 * is cleared to zero when it exits. The mapped id will be a non-zero
 * number.
 *
 * To simplify the generation of the 64 bit number, 32 bits will
 * be the CPU it was generated on, and the other 32 bits will be a per
 * cpu counter that gets incremented by two every time a new identifier
 * is generated. The LSB will always be set to keep the value
 * from being zero.
 */
#[repr(C)]
pub struct unwind_task_id_fields {
    pub cpu: u32,
    pub cnt: u32,
}

#[repr(C)]
pub union unwind_task_id {
    pub fields: unwind_task_id_fields,
    pub id: u64,
}

#[repr(C)]
pub struct unwind_task_info {
    pub unwind_mask: atomic_long_t,
    pub cache: *mut unwind_cache,
    pub work: callback_head,
    pub id: unwind_task_id,
}

pub struct unwind_work;
pub struct unwind_stacktrace;

pub type unwind_callback_t = Option<
    unsafe extern "C" fn(
        work: *mut unwind_work,
        trace: *mut unwind_stacktrace,
        cookie: u64,
    ),
>;

#[repr(C)]
pub struct unwind_work {
    pub list: list_head,
    pub func: unwind_callback_t,
    pub bit: ::core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
