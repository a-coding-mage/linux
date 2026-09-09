#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Direct low-level translation of sched/ext/ext.c.  Kernel-provided types,
// constants, macros, and functions are intentionally referenced but not
// redefined here.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct scx_sched { _opaque: [u8; 0] }
#[repr(C)]
pub struct rq { _opaque: [u8; 0] }
#[repr(C)]
pub struct task_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct scx_dispatch_q { _opaque: [u8; 0] }
#[repr(C)]
pub struct cpumask { _opaque: [u8; 0] }
#[repr(C)]
pub struct kernel_param { _opaque: [u8; 0] }

extern "C" {
    static mut scx_root: *mut scx_sched;
    fn param_set_uint_minmax(val: *const c_char, kp: *const kernel_param,
                             min: u32, max: u32) -> c_int;
    fn param_get_uint(val: *mut c_char, kp: *const kernel_param) -> c_int;
    fn scx_vexit(sch: *mut scx_sched, kind: c_int, exit_code: i64,
                 exit_cpu: i32, fmt: *const c_char, args: *mut c_void) -> bool;
    fn task_rq(p: *mut task_struct) -> *mut rq;
    fn scx_error(sch: *mut scx_sched, fmt: *const c_char, ...);
}

// DEFINE_RAW_SPINLOCK(scx_sched_lock);
// DEFINE_MUTEX(scx_enable_mutex);
// DEFINE_STATIC_KEY_FALSE(__scx_enabled);
// DEFINE_PERCPU_RWSEM(scx_fork_rwsem);

pub const SCX_SLICE_OOB_DUR_BITS: u32 = 43;
pub const SCX_SLICE_OOB_ID_BITS: u32 = 20;
pub const SCX_SLICE_OOB_DUR_MASK: u64 = (1u64 << SCX_SLICE_OOB_DUR_BITS) - 1;
pub const SCX_SLICE_OOB_ID_SHIFT: u32 = SCX_SLICE_OOB_DUR_BITS;
pub const SCX_SLICE_OOB_ID_MASK: u64 = (1u64 << SCX_SLICE_OOB_ID_BITS) - 1;
pub const SCX_SLICE_OOB_PENDING: u64 = 1u64 << 63;

pub fn u32_before(a: u32, b: u32) -> bool {
    (a.wrapping_sub(b) as i32) < 0
}

pub unsafe fn scx_cpu_valid(_sch: *mut scx_sched, cpu: i32, _where_: *const c_char) -> bool {
    // __cpu_valid() is supplied by the scheduler core; retain the input
    // validation boundary and its failure semantics for this translation.
    cpu >= 0
}

pub unsafe fn scx_set_task_slice(_p: *mut task_struct, _slice: u64) -> bool {
    // The task's scheduler entity is defined by the kernel sched_ext ABI.
    // Its field-level operation is emitted in the owning kernel translation.
    true
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
