/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/fncpy.h - helper macros for function body copying
 *
 * Rust translation of the C header.  The copied function body must be
 * self-contained and position-independent.
 */

/// Minimum alignment requirement for source and destination addresses.
pub const FNCPY_ALIGN: usize = 8;

extern "C" {
    fn flush_icache_range(start: libc::c_ulong, end: libc::c_ulong);
}

/// Copy a low-level function body and return a callable pointer preserving
/// the ARM Thumb bit of the original function pointer.
#[macro_export]
macro_rules! fncpy {
    ($dest_buf:expr, $funcp:expr, $size:expr) => {{
        let __funcp_address: usize = $funcp as usize;
        let __dest_buf = $dest_buf;
        let __size = $size;

        // BUG_ON((uintptr_t)(dest_buf) & (FNCPY_ALIGN - 1) ||
        //     (__funcp_address & !(uintptr_t)1 & (FNCPY_ALIGN - 1)));
        assert!(
            (__dest_buf as usize) & ($crate::FNCPY_ALIGN - 1) == 0
                && (__funcp_address & !1usize & ($crate::FNCPY_ALIGN - 1)) == 0
        );

        unsafe {
            core::ptr::copy_nonoverlapping(
                (__funcp_address & !1usize) as *const u8,
                __dest_buf as *mut u8,
                __size,
            );
            $crate::flush_icache_range(
                __dest_buf as libc::c_ulong,
                (__dest_buf as usize + __size) as libc::c_ulong,
            );
            core::mem::transmute::<usize, _>(
                (__dest_buf as usize) | (__funcp_address & 1usize),
            )
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
