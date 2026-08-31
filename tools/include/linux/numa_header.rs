/* SPDX-License-Identifier: GPL-2.0 */

/* From <linux/numa.h>; C header guard omitted in Rust translation. */

/*
 * C preprocessor intent:
 *   #ifdef CONFIG_NODES_SHIFT
 *   #define NODES_SHIFT CONFIG_NODES_SHIFT
 *   #else
 *   #define NODES_SHIFT 0
 *   #endif
 *
 * If CONFIG_NODES_SHIFT is supplied by the surrounding build, this constant
 * should track it. File-locally, the fallback value is 0.
 */
pub const NODES_SHIFT: i32 = 0;

pub const MAX_NUMNODES: i32 = 1_i32 << NODES_SHIFT;

pub const NUMA_NO_NODE: i32 = -1;

#[inline]
pub fn numa_valid_node(nid: i32) -> bool {
    nid >= 0 && nid < MAX_NUMNODES
}
