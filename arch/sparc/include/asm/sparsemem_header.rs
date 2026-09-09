/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: _SPARC64_SPARSEMEM_H */

/* The declarations below are available only when building the kernel. */

/// Number of bits in a sparse-memory section size.
pub const SECTION_SIZE_BITS: usize = 30;

/// Maximum physical memory address width, supplied by the page header.
pub const MAX_PHYSMEM_BITS: usize = MAX_PHYS_ADDRESS_BITS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
