/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This header provides generic wrappers for memory access instrumentation that
 * the compiler cannot emit for: KASAN, KCSAN, KMSAN.
 */

/* Depends on: linux/bug.h, linux/compiler.h, linux/kasan-checks.h,
 * linux/kcsan-checks.h, linux/kmsan-checks.h, linux/types.h */

/**
 * instrument_read - instrument regular read access
 * @v: address of access
 * @size: size of access
 *
 * Instrument a regular read access. The instrumentation should be inserted
 * before the actual read happens.
 */
#[inline(always)]
pub unsafe fn instrument_read(v: *const core::ffi::c_void, size: usize) {
    unsafe { kasan_check_read(v, size as u32) };
    kcsan_check_read!(v, size);
}

/**
 * instrument_write - instrument regular write access
 * @v: address of access
 * @size: size of access
 *
 * Instrument a regular write access. The instrumentation should be inserted
 * before the actual write happens.
 */
#[inline(always)]
pub unsafe fn instrument_write(v: *const core::ffi::c_void, size: usize) {
    unsafe { kasan_check_write(v, size as u32) };
    kcsan_check_write!(v, size);
}

/**
 * instrument_read_write - instrument regular read-write access
 * @v: address of access
 * @size: size of access
 *
 * Instrument a regular write access. The instrumentation should be inserted
 * before the actual write happens.
 */
#[inline(always)]
pub unsafe fn instrument_read_write(v: *const core::ffi::c_void, size: usize) {
    unsafe { kasan_check_write(v, size as u32) };
    kcsan_check_read_write!(v, size);
}

#[inline(always)]
pub unsafe fn instrument_atomic_check_alignment(v: *const core::ffi::c_void, size: usize) {
    #[cfg(all(not(__DISABLE_EXPORTS), CONFIG_DEBUG_ATOMIC))]
    {
        #[allow(unused_mut)]
        let mut mask = (size as kernel::ffi::c_uint).wrapping_sub(1);

        #[repr(C, align(16))]
        struct __aligned_largest {
            x: kernel::ffi::c_long,
        }
        #[cfg(CONFIG_DEBUG_ATOMIC_LARGEST_ALIGN)]
        {
            mask &= core::mem::size_of::<__aligned_largest>() as kernel::ffi::c_uint - 1;
        }
        WARN_ON_ONCE!((v as kernel::ffi::c_ulong) & mask as kernel::ffi::c_ulong != 0);
    }
    #[cfg(not(all(not(__DISABLE_EXPORTS), CONFIG_DEBUG_ATOMIC)))]
    let _ = (v, size);
}

/**
 * instrument_atomic_read - instrument atomic read access
 * @v: address of access
 * @size: size of access
 *
 * Instrument an atomic read access. The instrumentation should be inserted
 * before the actual read happens.
 */
#[inline(always)]
pub unsafe fn instrument_atomic_read(v: *const core::ffi::c_void, size: usize) {
    unsafe { kasan_check_read(v, size as u32) };
    kcsan_check_atomic_read!(v, size);
    unsafe { instrument_atomic_check_alignment(v, size) };
}

/**
 * instrument_atomic_write - instrument atomic write access
 * @v: address of access
 * @size: size of access
 *
 * Instrument an atomic write access. The instrumentation should be inserted
 * before the actual write happens.
 */
#[inline(always)]
pub unsafe fn instrument_atomic_write(v: *const core::ffi::c_void, size: usize) {
    unsafe { kasan_check_write(v, size as u32) };
    kcsan_check_atomic_write!(v, size);
    unsafe { instrument_atomic_check_alignment(v, size) };
}

/**
 * instrument_atomic_read_write - instrument atomic read-write access
 * @v: address of access
 * @size: size of access
 *
 * Instrument an atomic read-write access. The instrumentation should be
 * inserted before the actual write happens.
 */
#[inline(always)]
pub unsafe fn instrument_atomic_read_write(v: *const core::ffi::c_void, size: usize) {
    unsafe { kasan_check_write(v, size as u32) };
    kcsan_check_atomic_read_write!(v, size);
    unsafe { instrument_atomic_check_alignment(v, size) };
}

/**
 * instrument_copy_to_user - instrument reads of copy_to_user
 * @to: destination address
 * @from: source address
 * @n: number of bytes to copy
 *
 * Instrument reads from kernel memory, that are due to copy_to_user (and
 * variants). The instrumentation must be inserted before the accesses.
 */
#[inline(always)]
pub unsafe fn instrument_copy_to_user(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    n: kernel::ffi::c_ulong,
) {
    unsafe { kasan_check_read(from, n as u32) };
    kcsan_check_read!(from, n);
    kmsan_copy_to_user(to, from, n as usize, 0);
}

/**
 * instrument_copy_from_user_before - add instrumentation before copy_from_user
 * @to: destination address
 * @from: source address
 * @n: number of bytes to copy
 *
 * Instrument writes to kernel memory, that are due to copy_from_user (and
 * variants). The instrumentation should be inserted before the accesses.
 */
#[inline(always)]
pub unsafe fn instrument_copy_from_user_before(
    to: *const core::ffi::c_void,
    _from: *const core::ffi::c_void,
    n: kernel::ffi::c_ulong,
) {
    unsafe { kasan_check_write(to, n as u32) };
    kcsan_check_write!(to, n);
}

/**
 * instrument_copy_from_user_after - add instrumentation after copy_from_user
 * @to: destination address
 * @from: source address
 * @n: number of bytes to copy
 * @left: number of bytes not copied (as returned by copy_from_user)
 *
 * Instrument writes to kernel memory, that are due to copy_from_user (and
 * variants). The instrumentation should be inserted after the accesses.
 */
#[inline(always)]
pub unsafe fn instrument_copy_from_user_after(
    to: *const core::ffi::c_void,
    _from: *const core::ffi::c_void,
    n: kernel::ffi::c_ulong,
    left: kernel::ffi::c_ulong,
) {
    kmsan_unpoison_memory(to, (n - left) as usize);
}

/**
 * instrument_memcpy_before - add instrumentation before non-instrumented memcpy
 * @to: destination address
 * @from: source address
 * @n: number of bytes to copy
 *
 * Instrument memory accesses that happen in custom memcpy implementations. The
 * instrumentation should be inserted before the memcpy call.
 */
#[inline(always)]
pub unsafe fn instrument_memcpy_before(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    n: kernel::ffi::c_ulong,
) {
    unsafe {
        kasan_check_write(to, n as u32);
        kasan_check_read(from, n as u32);
    }
    kcsan_check_write!(to, n);
    kcsan_check_read!(from, n);
}

/**
 * instrument_memcpy_after - add instrumentation after non-instrumented memcpy
 * @to: destination address
 * @from: source address
 * @n: number of bytes to copy
 * @left: number of bytes not copied (if known)
 *
 * Instrument memory accesses that happen in custom memcpy implementations. The
 * instrumentation should be inserted after the memcpy call.
 */
#[inline(always)]
pub unsafe fn instrument_memcpy_after(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    n: kernel::ffi::c_ulong,
    left: kernel::ffi::c_ulong,
) {
    kmsan_memmove(to, from, (n - left) as usize);
}

/**
 * instrument_get_user() - add instrumentation to get_user()-like macros
 * @to: destination variable, may not be address-taken
 *
 * get_user() and friends are fragile, so it may depend on the implementation
 * whether the instrumentation happens before or after the data is copied from
 * the userspace.
 */
#[macro_export]
macro_rules! instrument_get_user {
    ($to:expr) => {{
        let __tmp: u64 = ($to) as u64;
        kmsan_unpoison_memory((&raw const __tmp).cast(), core::mem::size_of::<u64>());
        $to = __tmp as _;
    }};
}

/**
 * instrument_put_user() - add instrumentation to put_user()-like macros
 * @from: source address
 * @ptr: userspace pointer to copy to
 * @size: number of bytes to copy
 *
 * put_user() and friends are fragile, so it may depend on the implementation
 * whether the instrumentation happens before or after the data is copied from
 * the userspace.
 */
#[macro_export]
macro_rules! instrument_put_user {
    ($from:expr, $ptr:expr, $size:expr) => {{
        kmsan_copy_to_user(
            ($ptr).cast(),
            (&raw const $from).cast(),
            core::mem::size_of_val(&$from),
            0,
        );
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
