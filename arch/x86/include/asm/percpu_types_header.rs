/* SPDX-License-Identifier: GPL-2.0 */

// C preprocessor condition:
// #if defined(CONFIG_SMP) && defined(CONFIG_CC_HAS_NAMED_AS)
// #define __percpu_seg_override CONCATENATE(__seg_, __percpu_seg)
// #else /* !CONFIG_CC_HAS_NAMED_AS: */
// #define __percpu_seg_override
// #endif
//
// The source macro expands to a compiler-specific named address-space
// qualifier when both configuration symbols are enabled. Rust has no direct
// file-local equivalent for that qualifier; preserve the conditional intent
// for consumers supplying the corresponding configuration.

#[cfg(all(CONFIG_SMP, CONFIG_CC_HAS_NAMED_AS))]
#[allow(unused_macros)]
macro_rules! __percpu_seg_override {
    () => { __seg_ __percpu_seg };
}

#[cfg(not(all(CONFIG_SMP, CONFIG_CC_HAS_NAMED_AS)))]
#[allow(unused_macros)]
macro_rules! __percpu_seg_override {
    () => {};
}

// C preprocessor condition:
// #if defined(CONFIG_USE_X86_SEG_SUPPORT) && defined(USE_TYPEOF_UNQUAL)
// #define __percpu_qual __percpu_seg_override
// #endif
#[cfg(all(CONFIG_USE_X86_SEG_SUPPORT, USE_TYPEOF_UNQUAL))]
#[allow(unused_macros)]
macro_rules! __percpu_qual {
    () => { __percpu_seg_override!() };
}

// Dependency supplied by asm-generic/percpu_types.h in the original header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
