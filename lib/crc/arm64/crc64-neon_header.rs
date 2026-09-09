// SPDX-License-Identifier: GPL-2.0-only

// External ARM NEON types and intrinsics are supplied by the surrounding
// translation unit.

#[inline]
unsafe fn pmull64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    vreinterpretq_u64_p128(vmull_p64(
        vgetq_lane_u64(a, 0),
        vgetq_lane_u64(b, 0),
    ))
}

#[inline]
unsafe fn pmull64_high(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    let l: poly64x2_t = vreinterpretq_p64_u64(a);
    let m: poly64x2_t = vreinterpretq_p64_u64(b);

    vreinterpretq_u64_p128(vmull_high_p64(l, m))
}

#[inline]
unsafe fn pmull64_hi_lo(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    vreinterpretq_u64_p128(vmull_p64(
        vgetq_lane_u64(a, 1),
        vgetq_lane_u64(b, 0),
    ))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
