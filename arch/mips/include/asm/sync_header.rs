/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * sync types are defined by the MIPS64 Instruction Set documentation in Volume
 * II-A of the MIPS Architecture Reference Manual.
 *
 * Completion barriers ensure that memory operations have completed; ordering
 * barriers ensure that affected memory operations are not reordered.
 */

/* No sync instruction at all. */
pub const __SYNC_none: i32 = -1;

/* A full completion barrier. */
pub const __SYNC_full: i32 = 0x00;

/* Lightweight ordering barriers currently use the full completion barrier. */
pub const __SYNC_aq: i32 = __SYNC_full;
pub const __SYNC_rl: i32 = __SYNC_full;
pub const __SYNC_mb: i32 = __SYNC_full;

/*
 * Cavium Octeon uses the wmb ordering barrier and omits rmb barriers because
 * its CPUs do not perform speculative reads.  These configuration-dependent
 * values correspond to CONFIG_CPU_CAVIUM_OCTEON.
 */
#[cfg(CONFIG_CPU_CAVIUM_OCTEON)]
pub const __SYNC_rmb: i32 = __SYNC_none;
#[cfg(CONFIG_CPU_CAVIUM_OCTEON)]
pub const __SYNC_wmb: i32 = 0x04;
#[cfg(not(CONFIG_CPU_CAVIUM_OCTEON))]
pub const __SYNC_rmb: i32 = __SYNC_full;
#[cfg(not(CONFIG_CPU_CAVIUM_OCTEON))]
pub const __SYNC_wmb: i32 = __SYNC_full;

/* GINV synchronizes a ginvi or ginvt global invalidation operation. */
pub const __SYNC_ginv: i32 = 0x14;

/* Reasons for emitting a sync instruction. */
pub const __SYNC_always: u32 = 1 << 0;

#[cfg(CONFIG_WEAK_ORDERING)]
pub const __SYNC_weak_ordering: u32 = 1 << 1;
#[cfg(not(CONFIG_WEAK_ORDERING))]
pub const __SYNC_weak_ordering: u32 = 0;

#[cfg(CONFIG_WEAK_REORDERING_BEYOND_LLSC)]
pub const __SYNC_weak_llsc: u32 = 1 << 2;
#[cfg(not(CONFIG_WEAK_REORDERING_BEYOND_LLSC))]
pub const __SYNC_weak_llsc: u32 = 0;

/* Loongson 3 LL/SC atomicity workaround. */
#[cfg(CONFIG_CPU_LOONGSON3_WORKAROUNDS)]
pub const __SYNC_loongson3_war: u32 = 1 << 31;
#[cfg(not(CONFIG_CPU_LOONGSON3_WORKAROUNDS))]
pub const __SYNC_loongson3_war: u32 = 0;

/*
 * Cavium Octeon requires two consecutive wmb barriers.  The original
 * expression is evaluated by the assembler, where equality yields 0 or -1.
 */
#[cfg(CONFIG_CPU_CAVIUM_OCTEON)]
#[inline]
pub const fn __SYNC_rpt(sync_type: i32) -> i32 {
    1 - if sync_type == __SYNC_wmb { 1 } else { 0 }
}
#[cfg(not(CONFIG_CPU_CAVIUM_OCTEON))]
#[inline]
pub const fn __SYNC_rpt(_sync_type: i32) -> i32 {
    1
}

/*
 * The C header's ____SYNC/___SYNC/__SYNC/__SYNC_ELSE macros emit MIPS
 * assembler when CONFIG_CPU_HAS_SYNC is enabled.  Rust has no direct
 * file-local equivalent for those preprocessor/assembler stringification
 * macros; preserve their interface and conditional intent here.
 */
#[cfg(CONFIG_CPU_HAS_SYNC)]
#[inline]
pub unsafe fn ____SYNC(_sync_type: i32, _reason: u32, _else: &str) {
    /* Emits: sync _sync_type, repeated __SYNC_rpt times, when reason != 0. */
}

#[cfg(not(CONFIG_CPU_HAS_SYNC))]
#[inline]
pub unsafe fn ____SYNC(_sync_type: i32, _reason: u32, _else: &str) {}

#[inline]
pub unsafe fn ___SYNC(sync_type: i32, reason: u32, else_code: &str) {
    ____SYNC(sync_type, reason, else_code);
}

#[inline]
pub unsafe fn __SYNC(type_: i32, reason: u32) {
    ___SYNC(type_, reason, "");
}

#[inline]
pub unsafe fn __SYNC_ELSE(type_: i32, reason: u32, else_code: &str) {
    ___SYNC(type_, reason, else_code);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
