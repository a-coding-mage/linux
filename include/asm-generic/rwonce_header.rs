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

/* Depends on: linux/compiler_types.h, linux/kasan-checks.h,
 * linux/kcsan-checks.h */

/*
 * Yes, this permits 64-bit accesses on 32-bit architectures. These will
 * actually be atomic in some cases (namely Armv7 + LPAE), but for others we
 * rely on the access being split into 2x32-bit accesses for a 32-bit quantity
 * (e.g. a virtual address) and a strong prevailing wind.
 */
#[inline(always)]
pub const fn __compiletime_assert_rwonce_type<T>(_x: *const T) {
    const {
        assert!(
            matches!(core::mem::size_of::<T>(), 1 | 2 | 4 | 8),
            "Unsupported access size for {{READ,WRITE}}_ONCE()."
        )
    }
}

#[macro_export]
macro_rules! compiletime_assert_rwonce_type {
    ($t:expr) => {
        __compiletime_assert_rwonce_type(&raw const $t)
    };
}

/* READ_ONCE()/WRITE_ONCE() take the lvalue's address once and reuse it for
 * the size check, so the lvalue expression is evaluated exactly once. */

/*
 * Use __READ_ONCE() instead of READ_ONCE() if you do not require any
 * atomicity. Note that this may result in tears!
 *
 * Like the C macros these take an lvalue and access it through its address
 * (`&raw const/mut`), the volatile access being the only unsafe operation.
 */
#[cfg(not(any(all(CONFIG_ALPHA, CONFIG_SMP), all(CONFIG_ARM64, CONFIG_LTO, not(BUILD_VDSO)))))]
#[macro_export]
macro_rules! __READ_ONCE {
    ($x:expr) => {{
        let __p = &raw const $x;
        #[allow(unused_unsafe)]
        let __v = unsafe { core::ptr::read_volatile(__p) };
        __v
    }};
}

#[macro_export]
macro_rules! READ_ONCE {
    ($x:expr) => {{
        let __x = &raw const $x;
        __compiletime_assert_rwonce_type(__x);
        __READ_ONCE!(*__x)
    }};
}

#[macro_export]
macro_rules! __WRITE_ONCE {
    ($x:expr, $val:expr) => {{
        let __v = $val;
        let __p = &raw mut $x;
        #[allow(unused_unsafe)]
        let () = unsafe { core::ptr::write_volatile(__p, __v) };
    }};
}

#[macro_export]
macro_rules! WRITE_ONCE {
    ($x:expr, $val:expr) => {{
        let __x = &raw mut $x;
        let __val = $val;
        __compiletime_assert_rwonce_type(__x);
        __WRITE_ONCE!(*__x, __val)
    }};
}

#[inline(always)]
pub unsafe fn __read_once_word_nocheck(addr: *const kernel::ffi::c_void) -> kernel::ffi::c_ulong {
    __READ_ONCE!(*addr.cast::<kernel::ffi::c_ulong>())
}

/*
 * Use READ_ONCE_NOCHECK() instead of READ_ONCE() if you need to load a
 * word from memory atomically but without telling KASAN/KCSAN. This is
 * usually used by unwinding code when walking the stack of a running process.
 */
#[macro_export]
macro_rules! READ_ONCE_NOCHECK {
    ($x:expr) => {{
        let __p = &raw const $x;
        const fn __assert_word<T>(_: *const T) {
            const {
                assert!(
                    core::mem::size_of::<T>() == core::mem::size_of::<kernel::ffi::c_ulong>(),
                    "Unsupported access size for READ_ONCE_NOCHECK()."
                )
            }
        }
        __assert_word(__p);
        #[allow(unused_unsafe)]
        let __v = unsafe { core::mem::transmute_copy(&__read_once_word_nocheck(__p.cast())) };
        __v
    }};
}

#[inline(always)]
pub unsafe fn read_word_at_a_time(addr: *const kernel::ffi::c_void) -> kernel::ffi::c_ulong {
    /* open-coded instrument_read(addr, 1) */
    unsafe { kasan_check_read(addr, 1) };
    kcsan_check_read!(addr, 1);

    /*
     * This load can race with concurrent stores to out-of-bounds memory,
     * but READ_ONCE() can't be used because it requires higher alignment
     * than plain loads in arm64 builds with LTO.
     */
    unsafe { *addr.cast::<kernel::ffi::c_ulong>() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
