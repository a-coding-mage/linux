/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm-alpha/cache.h
 */

/* Bytes per L1 (data) cache line. */
/* CONFIG_ALPHA_GENERIC or CONFIG_ALPHA_EV6 selects the 64-byte cache line. */
#[cfg(any(CONFIG_ALPHA_GENERIC, CONFIG_ALPHA_EV6))]
pub const L1_CACHE_BYTES: usize = 64;
#[cfg(any(CONFIG_ALPHA_GENERIC, CONFIG_ALPHA_EV6))]
pub const L1_CACHE_SHIFT: usize = 6;

/* Both EV4 and EV5 are write-through, read-allocate,
 * direct-mapped, physical.
 */
#[cfg(not(any(CONFIG_ALPHA_GENERIC, CONFIG_ALPHA_EV6)))]
pub const L1_CACHE_BYTES: usize = 32;
#[cfg(not(any(CONFIG_ALPHA_GENERIC, CONFIG_ALPHA_EV6)))]
pub const L1_CACHE_SHIFT: usize = 5;

pub const SMP_CACHE_BYTES: usize = L1_CACHE_BYTES;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
