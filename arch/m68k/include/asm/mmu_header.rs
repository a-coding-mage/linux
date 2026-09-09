/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_MMU */
#[cfg(feature = "CONFIG_MMU")]
pub type mm_context_t = core::ffi::c_ulong;

/*
 * When CONFIG_MMU is not enabled, the C header includes
 * <asm-generic/mmu.h>.  The corresponding Rust dependency is supplied by
 * the surrounding translation.
 */
#[cfg(not(feature = "CONFIG_MMU"))]
pub type mm_context_t = crate::asm_generic::mmu::mm_context_t;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
