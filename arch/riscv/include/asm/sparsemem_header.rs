/* SPDX-License-Identifier: GPL-2.0 */

// _ASM_RISCV_SPARSEMEM header guard.

// Corresponds to CONFIG_SPARSEMEM.
#[cfg(CONFIG_SPARSEMEM)]
// Corresponds to CONFIG_64BIT.
#[cfg(CONFIG_64BIT)]
pub const MAX_PHYSMEM_BITS: usize = 56;

// Corresponds to CONFIG_SPARSEMEM and !CONFIG_64BIT.
#[cfg(all(CONFIG_SPARSEMEM, not(CONFIG_64BIT)))]
pub const MAX_PHYSMEM_BITS: usize = 32;

// Corresponds to CONFIG_SPARSEMEM.
#[cfg(CONFIG_SPARSEMEM)]
pub const SECTION_SIZE_BITS: usize = 27;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
