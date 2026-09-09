/* SPDX-License-Identifier: GPL-2.0 */
/* Translation of linux/kernel.h. Included C headers and build-time conditions
 * are dependencies of the surrounding translation unit. */

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user {
    _private: [u8; 0],
}

extern "C" {
    pub fn __cond_resched() -> ::core::ffi::c_int;
    pub fn dynamic_might_resched() -> ::core::ffi::c_int;

    pub fn __might_resched(file: *const ::core::ffi::c_char, line: ::core::ffi::c_int,
                           offsets: ::core::ffi::c_uint);
    pub fn __might_sleep(file: *const ::core::ffi::c_char, line: ::core::ffi::c_int);
    pub fn __cant_sleep(file: *const ::core::ffi::c_char, line: ::core::ffi::c_int);
    pub fn __cant_migrate(file: *const ::core::ffi::c_char, line: ::core::ffi::c_int);
    pub fn __might_fault(file: *const ::core::ffi::c_char, line: ::core::ffi::c_int);

    pub fn do_exit(error_code: ::core::ffi::c_long) -> !;
    pub fn core_kernel_text(addr: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn __kernel_text_address(addr: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn kernel_text_address(addr: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn func_ptr_is_kernel_text(ptr: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn bust_spinlocks(yes: ::core::ffi::c_int);

    pub static mut root_mountflags: ::core::ffi::c_int;
    pub static mut early_boot_irqs_disabled: bool;
    pub static mut system_state: system_states;
}

/* CONFIG_PREEMPT_* selects the implementation of might_resched(). */
#[inline(always)]
pub unsafe fn might_resched() {
    // CONFIG_PREEMPT_VOLUNTARY_BUILD / CONFIG_PREEMPT_DYNAMIC variants.
    let _ = __cond_resched();
}

/* CONFIG_DEBUG_ATOMIC_SLEEP annotations. */
#[inline(always)]
pub unsafe fn might_sleep() {
    might_resched();
}

#[inline(always)]
pub unsafe fn cant_sleep() {}

#[inline(always)]
pub unsafe fn cant_migrate() {}

#[inline(always)]
pub unsafe fn sched_annotate_sleep() {}

#[inline(always)]
pub unsafe fn non_block_start() {}

#[inline(always)]
pub unsafe fn non_block_end() {}

#[inline(always)]
pub unsafe fn might_sleep_if(cond: bool) {
    if cond {
        might_sleep();
    }
}

/* CONFIG_MMU and lock-debugging configuration select __might_fault(). */
#[inline(always)]
pub unsafe fn might_fault() {}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum system_states {
    SYSTEM_BOOTING,
    SYSTEM_SCHEDULING,
    SYSTEM_FREEING_INITMEM,
    SYSTEM_RUNNING,
    SYSTEM_HALT,
    SYSTEM_POWER_OFF,
    SYSTEM_RESTART,
    SYSTEM_SUSPEND,
}

/* Rebuild everything on CONFIG_DYNAMIC_FTRACE. */
#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
pub const REBUILD_DUE_TO_DYNAMIC_FTRACE: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
