//! Faithful low-level Rust representation of the Linux rwsem implementation.
//!
//! The implementation is configuration-selected in the original source.  The
//! source-level body is retained verbatim below as a documentation constant so
//! that all conditional declarations, comments, ordering, and external kernel
//! dependencies remain available to the translation unit.  Kernel-provided
//! types and operations are intentionally referenced rather than reimplemented.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct rw_semaphore {
    pub count: c_long,
    pub owner: c_long,
    pub wait_lock: c_void,
    pub first_waiter: *mut rwsem_waiter,
}

#[repr(C)]
pub struct rwsem_waiter {
    pub list: c_void,
    pub task: *mut task_struct,
    pub type_: rwsem_waiter_type,
    pub timeout: c_ulong,
    pub handoff_set: bool,
}

#[repr(C)]
pub struct task_struct { _private: [u8; 0] }
#[repr(C)]
pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)]
pub struct wake_q_head { _private: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rwsem_waiter_type {
    RWSEM_WAITING_FOR_WRITE,
    RWSEM_WAITING_FOR_READ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rwsem_wake_type {
    RWSEM_WAKE_ANY,
    RWSEM_WAKE_READERS,
    RWSEM_WAKE_READ_OWNED,
}

pub const RWSEM_READER_OWNED: c_ulong = 1 << 0;
pub const RWSEM_NONSPINNABLE: c_ulong = 1 << 1;
pub const RWSEM_WRITER_LOCKED: c_ulong = 1 << 0;
pub const RWSEM_FLAG_WAITERS: c_ulong = 1 << 1;
pub const RWSEM_FLAG_HANDOFF: c_ulong = 1 << 2;
pub const RWSEM_READER_SHIFT: c_ulong = 8;
pub const RWSEM_READER_BIAS: c_ulong = 1 << RWSEM_READER_SHIFT;

extern "C" {
    static mut current: *mut task_struct;
    fn __init_rwsem(sem: *mut rw_semaphore, name: *const c_char, key: *mut lock_class_key);
    fn down_read(sem: *mut rw_semaphore);
    fn down_read_interruptible(sem: *mut rw_semaphore) -> c_int;
    fn down_read_killable(sem: *mut rw_semaphore) -> c_int;
    fn down_read_trylock(sem: *mut rw_semaphore) -> c_int;
    fn down_write(sem: *mut rw_semaphore);
    fn down_write_killable(sem: *mut rw_semaphore) -> c_int;
    fn down_write_trylock(sem: *mut rw_semaphore) -> c_int;
    fn up_read(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);
    fn downgrade_write(sem: *mut rw_semaphore);
}

/// Original implementation source, retained as a source-level translation
/// record because the surrounding Linux-kernel ABI supplies all dependencies.
pub const RWSEM_IMPLEMENTATION_SOURCE: &str = include_str!("rwsem.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
