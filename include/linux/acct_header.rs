/* SPDX-License-Identifier: GPL-2.0 */
/*
 * BSD Process Accounting for Linux - Definitions
 *
 * Rust translation of the corresponding C header.
 */

/* Dependency supplied by the surrounding kernel translation. */

#[cfg(CONFIG_BSD_PROCESS_ACCT)]
pub struct pid_namespace;

#[cfg(CONFIG_BSD_PROCESS_ACCT)]
unsafe extern "C" {
    pub fn acct_collect(exitcode: ::core::ffi::c_long, group_dead: ::core::ffi::c_int);
    pub fn acct_process();
    pub fn acct_exit_ns(ns: *mut pid_namespace);
}

#[cfg(not(CONFIG_BSD_PROCESS_ACCT))]
#[inline]
pub unsafe fn acct_collect(_x: ::core::ffi::c_long, _y: ::core::ffi::c_int) {}

#[cfg(not(CONFIG_BSD_PROCESS_ACCT))]
#[inline]
pub unsafe fn acct_process() {}

#[cfg(not(CONFIG_BSD_PROCESS_ACCT))]
#[inline]
pub unsafe fn acct_exit_ns(_ns: *mut ::core::ffi::c_void) {}

/*
 * ACCT_VERSION numbers as yet defined:
 * 0: old format (until 2.6.7) with 16 bit uid/gid
 * 1: extended variant (binary compatible on M68K)
 * 2: extended variant (binary compatible on everything except M68K)
 * 3: new binary incompatible format (64 bytes)
 * 4: new binary incompatible format (128 bytes)
 * 5: new binary incompatible format (128 bytes, second half)
 */

#[cfg(CONFIG_BSD_PROCESS_ACCT_V3)]
pub const ACCT_VERSION: u32 = 3;
#[cfg(CONFIG_BSD_PROCESS_ACCT_V3)]
pub const AHZ: u32 = 100;
#[cfg(CONFIG_BSD_PROCESS_ACCT_V3)]
pub type acct_t = acct_v3;

#[cfg(not(CONFIG_BSD_PROCESS_ACCT_V3))]
#[cfg(CONFIG_M68K)]
pub const ACCT_VERSION: u32 = 1;
#[cfg(not(CONFIG_BSD_PROCESS_ACCT_V3))]
#[cfg(not(CONFIG_M68K))]
pub const ACCT_VERSION: u32 = 2;
#[cfg(not(CONFIG_BSD_PROCESS_ACCT_V3))]
pub const AHZ: u32 = USER_HZ;
#[cfg(not(CONFIG_BSD_PROCESS_ACCT_V3))]
pub type acct_t = acct;

/* HZ, USER_HZ, TICK_NSEC, and NSEC_PER_SEC are supplied externally. */

#[inline]
pub fn jiffies_to_AHZ(x: ::core::ffi::c_ulong) -> u32 {
    #[cfg(all(TICK_NSEC_DIVISIBLE_BY_NSEC_PER_AHZ, HZ_LT_AHZ))]
    { x.wrapping_mul(AHZ as ::core::ffi::c_ulong / HZ as ::core::ffi::c_ulong) as u32 }

    #[cfg(all(TICK_NSEC_DIVISIBLE_BY_NSEC_PER_AHZ, not(HZ_LT_AHZ)))]
    { (x / (HZ as ::core::ffi::c_ulong / AHZ as ::core::ffi::c_ulong)) as u32 }

    #[cfg(not(TICK_NSEC_DIVISIBLE_BY_NSEC_PER_AHZ))]
    {
        let tmp = (x as u64).wrapping_mul(TICK_NSEC as u64);
        (tmp / (NSEC_PER_SEC as u64 / AHZ as u64)) as u32
    }
}

#[inline]
pub fn nsec_to_AHZ(mut x: u64) -> u64 {
    #[cfg(NSEC_PER_SEC_DIVISIBLE_BY_AHZ)]
    { x /= NSEC_PER_SEC as u64 / AHZ as u64; }

    #[cfg(all(not(NSEC_PER_SEC_DIVISIBLE_BY_AHZ), AHZ_DIVISIBLE_BY_512))]
    {
        x = x.wrapping_mul(AHZ as u64 / 512);
        x /= NSEC_PER_SEC as u64 / 512;
    }

    #[cfg(all(not(NSEC_PER_SEC_DIVISIBLE_BY_AHZ), not(AHZ_DIVISIBLE_BY_512)))]
    {
        /* max relative error 5.7e-8 (1.8s per year) for AHZ <= 1024 */
        x = x.wrapping_mul(9);
        x /= (9u64.wrapping_mul(NSEC_PER_SEC as u64) + (AHZ as u64 / 2)) / AHZ as u64;
    }
    x
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
