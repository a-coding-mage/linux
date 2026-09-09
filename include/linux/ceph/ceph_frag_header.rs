/* SPDX-License-Identifier: GPL-2.0 */

/*
 * "Frags" are a way to describe a subset of a 32-bit number space,
 * using a mask and a value to match against that mask.  Any given frag
 * (subset of the number space) can be partitioned into 2^n sub-frags.
 *
 * Frags are encoded into a 32-bit word:
 *   8 upper bits = "bits"
 *   24 lower bits = "value"
 * (We could go to 5+27 bits, but who cares.)
 *
 * We use the _most_ significant bits of the 24 bit value.  This makes
 * values logically sort.
 *
 * Unfortunately, because the "bits" field is still in the high bits, we
 * can't sort encoded frags numerically.  However, it does allow you
 * to feed encoded frags as values into frag_contains_value.
 */
pub fn ceph_frag_make(b: __u32, v: __u32) -> __u32 {
    b.wrapping_shl(24)
        | (v & (0xffffffu32.wrapping_shl(24u32.wrapping_sub(b))) & 0xffffff)
}

pub fn ceph_frag_bits(f: __u32) -> __u32 {
    f >> 24
}

pub fn ceph_frag_value(f: __u32) -> __u32 {
    f & 0xffffff
}

pub fn ceph_frag_mask(f: __u32) -> __u32 {
    0xffffffu32.wrapping_shl(24u32.wrapping_sub(ceph_frag_bits(f))) & 0xffffff
}

pub fn ceph_frag_mask_shift(f: __u32) -> __u32 {
    24u32.wrapping_sub(ceph_frag_bits(f))
}

pub fn ceph_frag_contains_value(f: __u32, v: __u32) -> bool {
    (v & ceph_frag_mask(f)) == ceph_frag_value(f)
}

pub fn ceph_frag_make_child(f: __u32, by: i32, i: i32) -> __u32 {
    let newbits = ceph_frag_bits(f).wrapping_add(by as __u32);
    ceph_frag_make(
        newbits,
        ceph_frag_value(f) | ((i as __u32).wrapping_shl(24u32.wrapping_sub(newbits))),
    )
}

pub fn ceph_frag_is_leftmost(f: __u32) -> bool {
    ceph_frag_value(f) == 0
}

pub fn ceph_frag_is_rightmost(f: __u32) -> bool {
    ceph_frag_value(f) == ceph_frag_mask(f)
}

pub fn ceph_frag_next(f: __u32) -> __u32 {
    ceph_frag_make(
        ceph_frag_bits(f),
        ceph_frag_value(f)
            .wrapping_add(0x1000000u32.wrapping_shr(ceph_frag_bits(f))),
    )
}

/*
 * comparator to sort frags logically, as when traversing the
 * number space in ascending order...
 */
unsafe extern "C" {
    pub fn ceph_frag_compare(a: __u32, b: __u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
