// SPDX-License-Identifier: GPL-2.0-only

// `uint64x2_t` and the NEON lane accessor are supplied by the corresponding
// architecture dependencies.

#[inline]
pub unsafe fn pmull64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    let l: u64 = vgetq_lane_u64(a, 0);
    let m: u64 = vgetq_lane_u64(b, 0);
    let mut result: uint64x2_t;

    // C inline assembly: asm("vmull.p64 %q0, %P1, %P2" ...).
    // The exact operand constraints are compiler/architecture-specific.
    core::arch::asm!(
        "vmull.p64 {result:v.2d}, {l:d}, {m:d}",
        result = out(vreg) result,
        l = in(reg) l,
        m = in(reg) m,
    );

    result
}

#[inline]
pub unsafe fn pmull64_high(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    let l: u64 = vgetq_lane_u64(a, 1);
    let m: u64 = vgetq_lane_u64(b, 1);
    let mut result: uint64x2_t;

    // C inline assembly: asm("vmull.p64 %q0, %P1, %P2" ...).
    core::arch::asm!(
        "vmull.p64 {result:v.2d}, {l:d}, {m:d}",
        result = out(vreg) result,
        l = in(reg) l,
        m = in(reg) m,
    );

    result
}

#[inline]
pub unsafe fn pmull64_hi_lo(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    let l: u64 = vgetq_lane_u64(a, 1);
    let m: u64 = vgetq_lane_u64(b, 0);
    let mut result: uint64x2_t;

    // C inline assembly: asm("vmull.p64 %q0, %P1, %P2" ...).
    core::arch::asm!(
        "vmull.p64 {result:v.2d}, {l:d}, {m:d}",
        result = out(vreg) result,
        l = in(reg) l,
        m = in(reg) m,
    );

    result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
