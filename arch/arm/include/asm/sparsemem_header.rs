/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: the C header includes <asm/page.h>.

/*
 * Two definitions are required for sparsemem:
 *
 * MAX_PHYSMEM_BITS: The number of physical address bits required
 *   to address the last byte of memory.
 *
 * SECTION_SIZE_BITS: The number of physical address bits to cover
 *   the maximum amount of memory in a section.
 *
 * Eg, if you have 2 banks of up to 64MB at 0x80000000, 0x84000000,
 * then MAX_PHYSMEM_BITS is 32, SECTION_SIZE_BITS is 26.
 *
 * These can be overridden in your mach/memory.h.
 */
// C preprocessor intent: when either MAX_PHYSMEM_BITS or SECTION_SIZE_BITS
// is not defined by the build configuration, both values default to these.
pub const MAX_PHYSMEM_BITS: usize = 36;
pub const SECTION_SIZE_BITS: usize = 28;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
