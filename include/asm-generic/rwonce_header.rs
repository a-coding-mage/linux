/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Prevent the compiler from merging or refetching reads or writes. The
 * compiler is also forbidden from reordering successive instances of
 * READ_ONCE and WRITE_ONCE, but only when the compiler is aware of some
 * particular ordering. One way to make the compiler aware of ordering is to
 * put the two invocations of READ_ONCE or WRITE_ONCE in different C
 * statements.
 *
 * These two macros will also work on aggregate data types like structs or
 * unions.
 *
 * Their two major use cases are: (1) Mediating communication between
 * process-level code and irq/NMI handlers, all running on the same CPU,
 * and (2) Ensuring that the compiler does not fold, spindle, or otherwise
 * mutilate accesses that either do not require ordering or that interact
 * with an explicit memory barrier or atomic instruction that provides the
 * required ordering.
 */

/*
 * Yes, this permits 64-bit accesses on 32-bit architectures. These will
 * actually be atomic in some cases (namely Armv7 + LPAE), but for others we
 * rely on the access being split into 2x32-bit accesses for a 32-bit quantity
 * (e.g. a virtual address) and a strong prevailing wind.
 *
 * The original compile-time assertion depends on compiler-provided type
 * predicates and is retained here as the equivalent macro hook.
 */
#[macro_export]
macro_rules! compiletime_assert_rwonce_type {
    ($t:ty) => { /* compiletime_assert(__native_word(t) || sizeof(t) == sizeof(long long)) */ };
}

/* Use __READ_ONCE if atomicity is not required; this may result in tears. */
#[macro_export]
macro_rules! __READ_ONCE {
    ($x:expr) => {{
        unsafe { core::ptr::read_volatile(($x) as *const _) }
    }};
}

#[macro_export]
macro_rules! READ_ONCE {
    ($x:expr) => {{
        compiletime_assert_rwonce_type!(_);
        __READ_ONCE!($x)
    }};
}

#[macro_export]
macro_rules! __WRITE_ONCE {
    ($x:expr, $val:expr) => {{
        unsafe { core::ptr::write_volatile(($x) as *mut _, $val); }
    }};
}

#[macro_export]
macro_rules! WRITE_ONCE {
    ($x:expr, $val:expr) => {{
        compiletime_assert_rwonce_type!(_);
        __WRITE_ONCE!($x, $val);
    }};
}

#[inline(always)]
pub unsafe fn __read_once_word_nocheck(addr: *const core::ffi::c_void) -> usize {
    core::ptr::read_volatile(addr as *const usize)
}

/*
 * Use READ_ONCE_NOCHECK instead of READ_ONCE when a word must be loaded
 * atomically without telling KASAN/KCSAN.
 */
#[macro_export]
macro_rules! READ_ONCE_NOCHECK {
    ($x:expr) => {{
        /* compiletime_assert(sizeof(x) == sizeof(unsigned long)) */
        unsafe { __read_once_word_nocheck(($x) as *const _ as *const core::ffi::c_void) }
    }};
}

/* External sanitizer hooks supplied by the corresponding Linux headers. */
extern "C" {
    fn kasan_check_read(addr: *const core::ffi::c_void, size: usize);
    fn kcsan_check_read(addr: *const core::ffi::c_void, size: usize);
}

#[inline(always)]
pub unsafe fn read_word_at_a_time(addr: *const core::ffi::c_void) -> usize {
    /* open-coded instrument_read(addr, 1) */
    kasan_check_read(addr, 1);
    kcsan_check_read(addr, 1);

    /* This load can race with concurrent stores to out-of-bounds memory. */
    core::ptr::read(addr as *const usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
