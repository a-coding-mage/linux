/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Packet network namespace
 */

// Dependencies corresponding to <linux/rculist.h> and <linux/mutex.h> are
// supplied by other translation units.

#[repr(C)]
pub struct netns_packet {
    pub sklist_lock: mutex,
    pub sklist: hlist_head,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
