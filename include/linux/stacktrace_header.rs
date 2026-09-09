/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/types.h, asm/errno.h

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

// CONFIG_ARCH_STACKWALK
pub type stack_trace_consume_fn = Option<unsafe extern "C" fn(cookie: *mut c_void, addr: c_ulong) -> bool>;

extern "C" {
    pub fn arch_stack_walk(
        consume_entry: stack_trace_consume_fn,
        cookie: *mut c_void,
        task: *mut task_struct,
        regs: *mut pt_regs,
    );

    pub fn arch_stack_walk_reliable(
        consume_entry: stack_trace_consume_fn,
        cookie: *mut c_void,
        task: *mut task_struct,
    ) -> c_int;

    pub fn arch_stack_walk_user(
        consume_entry: stack_trace_consume_fn,
        cookie: *mut c_void,
        regs: *const pt_regs,
    );
}

// CONFIG_STACKTRACE
extern "C" {
    pub fn stack_trace_print(trace: *const c_ulong, nr_entries: c_uint, spaces: c_int);
    pub fn stack_trace_snprint(
        buf: *mut c_char,
        size: usize,
        entries: *const c_ulong,
        nr_entries: c_uint,
        spaces: c_int,
    ) -> c_int;
    pub fn stack_trace_save(store: *mut c_ulong, size: c_uint, skipnr: c_uint) -> c_uint;
    pub fn stack_trace_save_tsk(
        task: *mut task_struct,
        store: *mut c_ulong,
        size: c_uint,
        skipnr: c_uint,
    ) -> c_uint;
    pub fn stack_trace_save_regs(
        regs: *mut pt_regs,
        store: *mut c_ulong,
        size: c_uint,
        skipnr: c_uint,
    ) -> c_uint;
    pub fn stack_trace_save_user(store: *mut c_ulong, size: c_uint) -> c_uint;
    pub fn filter_irq_stacks(entries: *mut c_ulong, nr_entries: c_uint) -> c_uint;
}

// !CONFIG_ARCH_STACKWALK: internal interfaces; do not use in generic code.
#[repr(C)]
pub struct stack_trace {
    pub nr_entries: c_uint,
    pub max_entries: c_uint,
    pub entries: *mut c_ulong,
    pub skip: c_uint, // input argument: How many entries to skip
}

extern "C" {
    pub fn save_stack_trace(trace: *mut stack_trace);
    pub fn save_stack_trace_regs(regs: *mut pt_regs, trace: *mut stack_trace);
    pub fn save_stack_trace_tsk(tsk: *mut task_struct, trace: *mut stack_trace);
    pub fn save_stack_trace_tsk_reliable(tsk: *mut task_struct, trace: *mut stack_trace) -> c_int;
    pub fn save_stack_trace_user(trace: *mut stack_trace);
}

// CONFIG_STACKTRACE && CONFIG_HAVE_RELIABLE_STACKTRACE
extern "C" {
    pub fn stack_trace_save_tsk_reliable(
        tsk: *mut task_struct,
        store: *mut c_ulong,
        size: c_uint,
    ) -> c_int;
}

// When CONFIG_STACKTRACE or CONFIG_HAVE_RELIABLE_STACKTRACE is disabled,
// the C header provides a static inline returning -ENOSYS.
#[inline]
pub unsafe fn stack_trace_save_tsk_reliable_stub(
    _tsk: *mut task_struct,
    _store: *mut c_ulong,
    _size: c_uint,
) -> c_int {
    -38
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
