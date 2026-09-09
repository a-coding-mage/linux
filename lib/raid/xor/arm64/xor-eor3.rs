// SPDX-License-Identifier: GPL-2.0-only

// ARM64 NEON/SHAS3 implementation dependencies are supplied by the surrounding
// kernel build.  The C source uses ARM64_ASM_PREAMBLE and xor block helpers.

#[repr(C)]
#[derive(Copy, Clone)]
struct uint64x2_t([u64; 2]);

extern "C" {
    fn __xor_eor3_2(bytes: usize, p1: *mut c_ulong, p2: *const c_ulong);
}

type c_ulong = usize;

#[inline]
unsafe fn vld1q_u64(p: *const u64) -> uint64x2_t {
    uint64x2_t([*p, *p.add(1)])
}

#[inline]
unsafe fn vst1q_u64(p: *mut u64, v: uint64x2_t) {
    *p = v.0[0];
    *p.add(1) = v.0[1];
}

#[inline]
unsafe fn veorq_u64(p: uint64x2_t, q: uint64x2_t) -> uint64x2_t {
    uint64x2_t([p.0[0] ^ q.0[0], p.0[1] ^ q.0[1]])
}

#[inline]
unsafe fn eor3(p: uint64x2_t, q: uint64x2_t, r: uint64x2_t) -> uint64x2_t {
    // C implementation uses the ARM64 SHA3 EOR3 instruction.
    uint64x2_t([p.0[0] ^ q.0[0] ^ r.0[0], p.0[1] ^ q.0[1] ^ r.0[1]])
}

unsafe fn __xor_eor3_3(
    bytes: usize,
    p1: *mut c_ulong,
    p2: *const c_ulong,
    p3: *const c_ulong,
) {
    let mut dp1 = p1 as *mut u64;
    let mut dp2 = p2 as *const u64;
    let mut dp3 = p3 as *const u64;
    let mut lines = (bytes / (core::mem::size_of::<uint64x2_t>() * 4)) as isize;

    loop {
        // p1 ^= p2 ^ p3
        let v0 = eor3(vld1q_u64(dp1.add(0)), vld1q_u64(dp2.add(0)), vld1q_u64(dp3.add(0)));
        let v1 = eor3(vld1q_u64(dp1.add(2)), vld1q_u64(dp2.add(2)), vld1q_u64(dp3.add(2)));
        let v2 = eor3(vld1q_u64(dp1.add(4)), vld1q_u64(dp2.add(4)), vld1q_u64(dp3.add(4)));
        let v3 = eor3(vld1q_u64(dp1.add(6)), vld1q_u64(dp2.add(6)), vld1q_u64(dp3.add(6)));

        // store
        vst1q_u64(dp1.add(0), v0);
        vst1q_u64(dp1.add(2), v1);
        vst1q_u64(dp1.add(4), v2);
        vst1q_u64(dp1.add(6), v3);
        dp1 = dp1.add(8); dp2 = dp2.add(8); dp3 = dp3.add(8);
        lines -= 1;
        if lines <= 0 { break; }
    }
}

unsafe fn __xor_eor3_4(
    bytes: usize, p1: *mut c_ulong, p2: *const c_ulong, p3: *const c_ulong,
    p4: *const c_ulong,
) {
    let (mut dp1, mut dp2, mut dp3, mut dp4) =
        (p1 as *mut u64, p2 as *const u64, p3 as *const u64, p4 as *const u64);
    let mut lines = (bytes / (core::mem::size_of::<uint64x2_t>() * 4)) as isize;
    loop {
        // p1 ^= p2 ^ p3
        let mut v0 = eor3(vld1q_u64(dp1), vld1q_u64(dp2), vld1q_u64(dp3));
        let mut v1 = eor3(vld1q_u64(dp1.add(2)), vld1q_u64(dp2.add(2)), vld1q_u64(dp3.add(2)));
        let mut v2 = eor3(vld1q_u64(dp1.add(4)), vld1q_u64(dp2.add(4)), vld1q_u64(dp3.add(4)));
        let mut v3 = eor3(vld1q_u64(dp1.add(6)), vld1q_u64(dp2.add(6)), vld1q_u64(dp3.add(6)));
        // p1 ^= p4
        v0 = veorq_u64(v0, vld1q_u64(dp4)); v1 = veorq_u64(v1, vld1q_u64(dp4.add(2)));
        v2 = veorq_u64(v2, vld1q_u64(dp4.add(4))); v3 = veorq_u64(v3, vld1q_u64(dp4.add(6)));
        // store
        vst1q_u64(dp1, v0); vst1q_u64(dp1.add(2), v1); vst1q_u64(dp1.add(4), v2); vst1q_u64(dp1.add(6), v3);
        dp1=dp1.add(8); dp2=dp2.add(8); dp3=dp3.add(8); dp4=dp4.add(8); lines-=1; if lines<=0 {break;}
    }
}

unsafe fn __xor_eor3_5(
    bytes: usize, p1: *mut c_ulong, p2: *const c_ulong, p3: *const c_ulong,
    p4: *const c_ulong, p5: *const c_ulong,
) {
    let (mut a, mut b, mut c, mut d, mut e) = (p1 as *mut u64, p2 as *const u64, p3 as *const u64, p4 as *const u64, p5 as *const u64);
    let mut lines = (bytes / (core::mem::size_of::<uint64x2_t>() * 4)) as isize;
    loop {
        // p1 ^= p2 ^ p3
        let mut v = [
            eor3(vld1q_u64(a), vld1q_u64(b), vld1q_u64(c)),
            eor3(vld1q_u64(a.add(2)), vld1q_u64(b.add(2)), vld1q_u64(c.add(2))),
            eor3(vld1q_u64(a.add(4)), vld1q_u64(b.add(4)), vld1q_u64(c.add(4))),
            eor3(vld1q_u64(a.add(6)), vld1q_u64(b.add(6)), vld1q_u64(c.add(6))),
        ];
        // p1 ^= p4 ^ p5
        for (j, i) in [0usize,2,4,6].iter().enumerate() { v[j] = eor3(v[j], vld1q_u64(d.add(*i)), vld1q_u64(e.add(*i))); vst1q_u64(a.add(*i), v[j]); }
        a=a.add(8); b=b.add(8); c=c.add(8); d=d.add(8); e=e.add(8); lines-=1; if lines<=0 {break;}
    }
}

// __DO_XOR_BLOCKS(eor3_inner, __xor_eor3_2, __xor_eor3_3, __xor_eor3_4, __xor_eor3_5)
// is supplied by xor_impl.h and generates the architecture-specific dispatcher.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
