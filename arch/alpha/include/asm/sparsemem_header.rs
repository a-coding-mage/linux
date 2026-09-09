/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: _ASM_ALPHA_SPARSEMEM_H */

/* Equivalent of CONFIG_SPARSEMEM. */
#[cfg(CONFIG_SPARSEMEM)]
pub const SECTION_SIZE_BITS: u32 = 27;

/*
 * According to "Alpha Architecture Reference Manual" physical
 * addresses are at most 48 bits.
 * https://download.majix.org/dec/alpha_arch_ref.pdf
 */
#[cfg(CONFIG_SPARSEMEM)]
pub const MAX_PHYSMEM_BITS: u32 = 48;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
