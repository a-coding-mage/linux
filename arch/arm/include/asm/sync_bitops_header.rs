/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the corresponding bit operations implementation:
// `asm/bitops.h`.

/* sync_bitops functions are equivalent to the SMP implementation of the
 * original functions, independently from CONFIG_SMP being defined.
 *
 * We need them because _set_bit etc are not SMP safe if !CONFIG_SMP. But
 * under Xen you might be communicating with a completely external entity
 * who might be on another CPU (e.g. two uniprocessor guests communicating
 * via event channels and grant tables). So we need a variant of the bit
 * ops which are SMP safe even on a UP kernel.
 */

/*
 * Unordered
 */

macro_rules! sync_set_bit {
    ($nr:expr, $p:expr) => {
        _set_bit!($nr, $p)
    };
}

macro_rules! sync_clear_bit {
    ($nr:expr, $p:expr) => {
        _clear_bit!($nr, $p)
    };
}

macro_rules! sync_change_bit {
    ($nr:expr, $p:expr) => {
        _change_bit!($nr, $p)
    };
}

macro_rules! sync_test_bit {
    ($nr:expr, $addr:expr) => {
        test_bit!($nr, $addr)
    };
}

/*
 * Fully ordered
 */

unsafe extern "C" {
    pub fn _sync_test_and_set_bit(nr: ::core::ffi::c_int, p: *mut ::core::ffi::c_ulong)
        -> ::core::ffi::c_int;
    pub fn _sync_test_and_clear_bit(
        nr: ::core::ffi::c_int,
        p: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn _sync_test_and_change_bit(
        nr: ::core::ffi::c_int,
        p: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
}

macro_rules! sync_test_and_set_bit {
    ($nr:expr, $p:expr) => {
        unsafe { _sync_test_and_set_bit($nr, $p) }
    };
}

macro_rules! sync_test_and_clear_bit {
    ($nr:expr, $p:expr) => {
        unsafe { _sync_test_and_clear_bit($nr, $p) }
    };
}

macro_rules! sync_test_and_change_bit {
    ($nr:expr, $p:expr) => {
        unsafe { _sync_test_and_change_bit($nr, $p) }
    };
}

macro_rules! arch_sync_cmpxchg {
    ($ptr:expr, $old:expr, $new:expr) => {{
        __smp_mb__before_atomic!();
        let __ret = arch_cmpxchg_relaxed!($ptr, $old, $new);
        __smp_mb__after_atomic!();
        __ret
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
