// SPDX-License-Identifier: GPL-2.0
/*
 * Ceph 'frag' type
 */

// External declarations supplied by the Linux Ceph types implementation.
extern "C" {
    fn ceph_frag_value(a: u32) -> u32;
    fn ceph_frag_bits(a: u32) -> u32;
}

pub unsafe fn ceph_frag_compare(a: u32, b: u32) -> i32 {
    let mut va: u32 = ceph_frag_value(a);
    let mut vb: u32 = ceph_frag_value(b);
    if va < vb {
        return -1;
    }
    if va > vb {
        return 1;
    }
    va = ceph_frag_bits(a);
    vb = ceph_frag_bits(b);
    if va < vb {
        return -1;
    }
    if va > vb {
        return 1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
