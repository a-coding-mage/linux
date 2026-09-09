/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by <asm/cache.h> in the C source.
// The Rust translation expects `L1_CACHE_BYTES` to be supplied by the
// surrounding target-specific bindings.

// #ifndef SMP_CACHE_BYTES
// #define SMP_CACHE_BYTES L1_CACHE_BYTES
#[allow(non_upper_case_globals)]
pub const SMP_CACHE_BYTES: usize = L1_CACHE_BYTES;

// #ifndef ____cacheline_aligned
// #define ____cacheline_aligned __attribute__((__aligned__(SMP_CACHE_BYTES)))
// Rust declarations requiring this C attribute should use
// `#[repr(align(SMP_CACHE_BYTES))]` when the target provides a literal
// alignment value.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
