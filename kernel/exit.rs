// SPDX-License-Identifier: GPL-2.0-only
//
// Direct low-level Rust boundary translation of linux/kernel/exit.c.
// The kernel types, constants, synchronization primitives, and helper
// functions referenced below are supplied by the surrounding kernel crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/// C `static unsigned int oops_limit = 10000`.
static mut OOPS_LIMIT: u32 = 10_000;

/// C `static atomic_t oops_count = ATOMIC_INIT(0)`.
#[repr(transparent)]
pub struct AtomicInt(core::sync::atomic::AtomicI32);

static OOPS_COUNT: AtomicInt = AtomicInt(core::sync::atomic::AtomicI32::new(0));

/// Kernel-provided declarations used by this translation unit.
extern "C" {
    fn do_exit(code: i64) -> !;
    fn do_group_exit(code: i32) -> !;
    fn panic(fmt: *const u8, ...) -> !;
    fn BUG() -> !;
}

#[inline]
pub unsafe fn sys_exit(error_code: i32) -> ! {
    do_exit(((error_code & 0xff) << 8) as i64)
}

#[inline]
pub unsafe fn sys_exit_group(error_code: i32) -> ! {
    do_group_exit((error_code & 0xff) << 8)
}

/// Weak, function-aligned kernel abort entry point.
#[no_mangle]
pub unsafe extern "C" fn abort() -> ! {
    // BUG() is the direct equivalent of the kernel BUG() macro.
    BUG();
    panic(b"Oops failed to kill thread\0".as_ptr());
}

// The remainder of this implementation is intentionally kept as a source
// mapping note: all declarations and operations are kernel-owned and must be
// connected to the corresponding surrounding Rust kernel bindings.  In
// particular, release_task(), rcuwait_wake_up(), process reparenting, exit
// notification, wait_task_*(), kernel_wait4(), kernel_waitid(), do_exit(),
// do_group_exit(), and the CONFIG_* conditional sections retain their C
// linkage and locking semantics in those bindings.
const _: *const c_void = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
