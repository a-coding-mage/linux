// SPDX-License-Identifier: GPL-2.0-only
/*
 * Authors: Jackie Liu <liuyun01@kylinos.cn>
 * Copyright (C) 2018,Tianjin KYLIN Information Technology Co., Ltd.
 */

// Dependencies supplied by xor_impl.h, xor-neon.h, and asm/neon-intrinsics.h

unsafe fn __xor_neon_2(
    bytes: c_ulong,
    p1: *mut c_ulong,
    p2: *const c_ulong,
) {
    let mut dp1 = p1 as *mut u64;
    let mut dp2 = p2 as *const u64;

    let mut v0: uint64x2_t;
    let mut v1: uint64x2_t;
    let mut v2: uint64x2_t;
    let mut v3: uint64x2_t;
    let mut lines = (bytes / (core::mem::size_of::<uint64x2_t>() as c_ulong * 4)) as c_long;

    loop {
        // p1 ^= p2
        v0 = veorq_u64(vld1q_u64(dp1.add(0)), vld1q_u64(dp2.add(0)));
        v1 = veorq_u64(vld1q_u64(dp1.add(2)), vld1q_u64(dp2.add(2)));
        v2 = veorq_u64(vld1q_u64(dp1.add(4)), vld1q_u64(dp2.add(4)));
        v3 = veorq_u64(vld1q_u64(dp1.add(6)), vld1q_u64(dp2.add(6)));

        // store
        vst1q_u64(dp1.add(0), v0);
        vst1q_u64(dp1.add(2), v1);
        vst1q_u64(dp1.add(4), v2);
        vst1q_u64(dp1.add(6), v3);

        dp1 = dp1.add(8);
        dp2 = dp2.add(8);
        lines -= 1;
        if lines <= 0 { break; }
    }
}

unsafe fn __xor_neon_3(bytes: c_ulong, p1: *mut c_ulong, p2: *const c_ulong, p3: *const c_ulong) {
    let mut dp1 = p1 as *mut u64;
    let mut dp2 = p2 as *const u64;
    let mut dp3 = p3 as *const u64;
    let mut lines = (bytes / (core::mem::size_of::<uint64x2_t>() as c_ulong * 4)) as c_long;
    loop {
        let mut v0 = veorq_u64(vld1q_u64(dp1.add(0)), vld1q_u64(dp2.add(0)));
        let mut v1 = veorq_u64(vld1q_u64(dp1.add(2)), vld1q_u64(dp2.add(2)));
        let mut v2 = veorq_u64(vld1q_u64(dp1.add(4)), vld1q_u64(dp2.add(4)));
        let mut v3 = veorq_u64(vld1q_u64(dp1.add(6)), vld1q_u64(dp2.add(6)));
        v0 = veorq_u64(v0, vld1q_u64(dp3.add(0))); v1 = veorq_u64(v1, vld1q_u64(dp3.add(2)));
        v2 = veorq_u64(v2, vld1q_u64(dp3.add(4))); v3 = veorq_u64(v3, vld1q_u64(dp3.add(6)));
        vst1q_u64(dp1.add(0), v0); vst1q_u64(dp1.add(2), v1);
        vst1q_u64(dp1.add(4), v2); vst1q_u64(dp1.add(6), v3);
        dp1 = dp1.add(8); dp2 = dp2.add(8); dp3 = dp3.add(8);
        lines -= 1; if lines <= 0 { break; }
    }
}

unsafe fn __xor_neon_4(bytes: c_ulong, p1: *mut c_ulong, p2: *const c_ulong, p3: *const c_ulong, p4: *const c_ulong) {
    let mut dp1 = p1 as *mut u64; let mut dp2 = p2 as *const u64;
    let mut dp3 = p3 as *const u64; let mut dp4 = p4 as *const u64;
    let mut lines = (bytes / (core::mem::size_of::<uint64x2_t>() as c_ulong * 4)) as c_long;
    loop {
        let mut v0 = veorq_u64(vld1q_u64(dp1.add(0)), vld1q_u64(dp2.add(0))); let mut v1 = veorq_u64(vld1q_u64(dp1.add(2)), vld1q_u64(dp2.add(2)));
        let mut v2 = veorq_u64(vld1q_u64(dp1.add(4)), vld1q_u64(dp2.add(4))); let mut v3 = veorq_u64(vld1q_u64(dp1.add(6)), vld1q_u64(dp2.add(6)));
        v0 = veorq_u64(veorq_u64(v0, vld1q_u64(dp3.add(0))), vld1q_u64(dp4.add(0))); v1 = veorq_u64(veorq_u64(v1, vld1q_u64(dp3.add(2))), vld1q_u64(dp4.add(2)));
        v2 = veorq_u64(veorq_u64(v2, vld1q_u64(dp3.add(4))), vld1q_u64(dp4.add(4))); v3 = veorq_u64(veorq_u64(v3, vld1q_u64(dp3.add(6))), vld1q_u64(dp4.add(6)));
        vst1q_u64(dp1.add(0), v0); vst1q_u64(dp1.add(2), v1); vst1q_u64(dp1.add(4), v2); vst1q_u64(dp1.add(6), v3);
        dp1 = dp1.add(8); dp2 = dp2.add(8); dp3 = dp3.add(8); dp4 = dp4.add(8); lines -= 1; if lines <= 0 { break; }
    }
}

unsafe fn __xor_neon_5(bytes: c_ulong, p1: *mut c_ulong, p2: *const c_ulong, p3: *const c_ulong, p4: *const c_ulong, p5: *const c_ulong) {
    let mut dp1 = p1 as *mut u64; let mut dp2 = p2 as *const u64; let mut dp3 = p3 as *const u64; let mut dp4 = p4 as *const u64; let mut dp5 = p5 as *const u64;
    let mut lines = (bytes / (core::mem::size_of::<uint64x2_t>() as c_ulong * 4)) as c_long;
    loop {
        let mut v0 = veorq_u64(vld1q_u64(dp1.add(0)), vld1q_u64(dp2.add(0))); let mut v1 = veorq_u64(vld1q_u64(dp1.add(2)), vld1q_u64(dp2.add(2)));
        let mut v2 = veorq_u64(vld1q_u64(dp1.add(4)), vld1q_u64(dp2.add(4))); let mut v3 = veorq_u64(vld1q_u64(dp1.add(6)), vld1q_u64(dp2.add(6)));
        v0 = veorq_u64(veorq_u64(veorq_u64(v0, vld1q_u64(dp3.add(0))), vld1q_u64(dp4.add(0))), vld1q_u64(dp5.add(0)));
        v1 = veorq_u64(veorq_u64(veorq_u64(v1, vld1q_u64(dp3.add(2))), vld1q_u64(dp4.add(2))), vld1q_u64(dp5.add(2)));
        v2 = veorq_u64(veorq_u64(veorq_u64(v2, vld1q_u64(dp3.add(4))), vld1q_u64(dp4.add(4))), vld1q_u64(dp5.add(4)));
        v3 = veorq_u64(veorq_u64(veorq_u64(v3, vld1q_u64(dp3.add(6))), vld1q_u64(dp4.add(6))), vld1q_u64(dp5.add(6)));
        vst1q_u64(dp1.add(0), v0); vst1q_u64(dp1.add(2), v1); vst1q_u64(dp1.add(4), v2); vst1q_u64(dp1.add(6), v3);
        dp1 = dp1.add(8); dp2 = dp2.add(8); dp3 = dp3.add(8); dp4 = dp4.add(8); dp5 = dp5.add(8); lines -= 1; if lines <= 0 { break; }
    }
}

// __DO_XOR_BLOCKS(neon_inner, __xor_neon_2, __xor_neon_3, __xor_neon_4, __xor_neon_5);

// Under CONFIG_ARM64, __xor_eor3_2 is an alias of __xor_neon_2.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
