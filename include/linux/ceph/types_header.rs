/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

/*
 * Identify inodes by both their ino AND snapshot id (a u64).
 */
#[repr(C)]
pub struct ceph_vino {
    pub ino: u64,
    pub snap: u64,
}

/* context for the caps reservation mechanism */
#[repr(C)]
pub struct ceph_cap_reservation {
    pub count: i32,
    pub used: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
