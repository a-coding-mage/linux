/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding build: <linux/linkage.h>.

/* L1 cache line size */
pub const L1_CACHE_SHIFT: usize = CONFIG_X86_L1_CACHE_SHIFT;
pub const L1_CACHE_BYTES: usize = 1usize << L1_CACHE_SHIFT;

// C attribute macro: __section(".data..read_mostly")
// Rust items requiring this placement should preserve the corresponding
// linker-section attribute at their declaration site.

pub const INTERNODE_CACHE_SHIFT: usize = CONFIG_X86_INTERNODE_CACHE_SHIFT;
pub const INTERNODE_CACHE_BYTES: usize = 1usize << INTERNODE_CACHE_SHIFT;

// When CONFIG_X86_VSMP and CONFIG_SMP are enabled, the C macro
// __cacheline_aligned_in_smp applies alignment of INTERNODE_CACHE_BYTES and
// page-aligned data. Preserve those attributes at each affected declaration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
