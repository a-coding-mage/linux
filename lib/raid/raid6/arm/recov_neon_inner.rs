// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Intel Corporation
 * Copyright (C) 2017 Linaro Ltd. <ard.biesheuvel@linaro.org>
 */

// C dependencies: <arm_neon.h> and "arm/neon.h" provide the NEON types and
// intrinsics referenced below.

#[cfg(CONFIG_ARM)]
/*
 * AArch32 does not provide this intrinsic natively because it does not
 * implement the underlying instruction. AArch32 only provides a 64-bit
 * wide vtbl.8 instruction, so use that instead.
 */
unsafe fn vqtbl1q_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    #[repr(C)]
    union NeonPair {
        val: uint8x16_t,
        pair: uint8x8x2_t,
    }

    let a = NeonPair { val: a };
    vcombine_u8(
        vtbl2_u8(a.pair, vget_low_u8(b)),
        vtbl2_u8(a.pair, vget_high_u8(b)),
    )
}

pub unsafe fn __raid6_2data_recov_neon(
    mut bytes: i32,
    mut p: *mut u8,
    mut q: *mut u8,
    mut dp: *mut u8,
    mut dq: *mut u8,
    pbmul: *const u8,
    qmul: *const u8,
) {
    let pm0 = vld1q_u8(pbmul);
    let pm1 = vld1q_u8(pbmul.add(16));
    let qm0 = vld1q_u8(qmul);
    let qm1 = vld1q_u8(qmul.add(16));
    let x0f = vdupq_n_u8(0x0f);

    /*
     * while ( bytes-- ) {
     *     uint8_t px, qx, db;
     *
     *     px    = *p ^ *dp;
     *     qx    = qmul[*q ^ *dq];
     *     *dq++  = db = pbmul[px] ^ qx;
     *     *dp++  = db ^ px;
     *     p++; q++;
     * }
     */

    while bytes != 0 {
        let vx;
        let vy;
        let px;
        let qx;
        let db;

        px = veorq_u8(vld1q_u8(p), vld1q_u8(dp));
        vx = veorq_u8(vld1q_u8(q), vld1q_u8(dq));

        vy = vshrq_n_u8(vx, 4);
        vx = vqtbl1q_u8(qm0, vandq_u8(vx, x0f));
        vy = vqtbl1q_u8(qm1, vy);
        qx = veorq_u8(vx, vy);

        vy = vshrq_n_u8(px, 4);
        vx = vqtbl1q_u8(pm0, vandq_u8(px, x0f));
        vy = vqtbl1q_u8(pm1, vy);
        vx = veorq_u8(vx, vy);
        db = veorq_u8(vx, qx);

        vst1q_u8(dq, db);
        vst1q_u8(dp, veorq_u8(db, px));

        bytes -= 16;
        p = p.add(16);
        q = q.add(16);
        dp = dp.add(16);
        dq = dq.add(16);
    }
}

pub unsafe fn __raid6_datap_recov_neon(
    mut bytes: i32,
    mut p: *mut u8,
    mut q: *mut u8,
    mut dq: *mut u8,
    qmul: *const u8,
) {
    let qm0 = vld1q_u8(qmul);
    let qm1 = vld1q_u8(qmul.add(16));
    let x0f = vdupq_n_u8(0x0f);

    /*
     * while (bytes--) {
     *     *p++ ^= *dq = qmul[*q ^ *dq];
     *     q++; dq++;
     * }
     */

    while bytes != 0 {
        let vx;
        let vy;

        vx = veorq_u8(vld1q_u8(q), vld1q_u8(dq));

        vy = vshrq_n_u8(vx, 4);
        vx = vqtbl1q_u8(qm0, vandq_u8(vx, x0f));
        vy = vqtbl1q_u8(qm1, vy);
        vx = veorq_u8(vx, vy);
        vy = veorq_u8(vx, vld1q_u8(p));

        vst1q_u8(dq, vx);
        vst1q_u8(p, vy);

        bytes -= 16;
        p = p.add(16);
        q = q.add(16);
        dq = dq.add(16);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
