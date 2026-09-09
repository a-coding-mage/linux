/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// x86-specific signal information definitions.
//
// The C header includes <asm-generic/siginfo.h>; its declarations are supplied
// by the corresponding Rust translation and are intentionally not duplicated
// here.

#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
pub type __kernel_si_clock_t = i64;

#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
pub type __ARCH_SI_CLOCK_T = __kernel_si_clock_t;

// In C, __ARCH_SI_ATTRIBUTES expands to __attribute__((aligned(8))) for the
// x32 configuration. Rust has no direct equivalent for an attribute macro;
// apply #[repr(align(8))] at the containing declaration where this macro is
// consumed.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
