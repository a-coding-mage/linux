/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <linux/bitops.h>; bitmap layout helpers supplied by
// that dependency are represented locally only where required by this type.

// Build-time CONFIG_NODES_SHIFT, when supplied by the surrounding build, should
// determine this value.  No such configuration value is available in this
// isolated translation, so the source's fallback is retained here.
pub const NODES_SHIFT: usize = 0;

pub const MAX_NUMNODES: usize = 1usize << NODES_SHIFT;

pub const NUMA_NO_NODE: i32 = -1;

#[repr(C)]
pub struct nodemask_t {
    pub bits: [usize; (MAX_NUMNODES + usize::BITS as usize - 1) / usize::BITS as usize],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
