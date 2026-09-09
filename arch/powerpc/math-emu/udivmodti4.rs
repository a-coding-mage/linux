// SPDX-License-Identifier: GPL-2.0
/* This has so very few changes over libgcc2's __udivmoddi4 it isn't funny. */

// Dependency supplied by the surrounding math-emu implementation.
// The C source includes <math-emu/soft-fp.h>.

extern "C" {
    fn udiv_qrnnd(q: *mut _FP_W_TYPE, r: *mut _FP_W_TYPE,
                  n1: _FP_W_TYPE, n0: _FP_W_TYPE, d: _FP_W_TYPE);
    fn sub_ddmmss(sh: *mut _FP_W_TYPE, sl: *mut _FP_W_TYPE,
                  ah: _FP_W_TYPE, al: _FP_W_TYPE,
                  bh: _FP_W_TYPE, bl: _FP_W_TYPE);
    fn umul_ppmm(w1: *mut _FP_W_TYPE, w0: *mut _FP_W_TYPE,
                 u: _FP_W_TYPE, v: _FP_W_TYPE);
}

#[inline]
unsafe fn count_leading_zeros(x: _FP_W_TYPE) -> _FP_I_TYPE {
    __FP_CLZ(x)
}

pub unsafe fn _fp_udivmodti4(
    q: *mut _FP_W_TYPE,
    r: *mut _FP_W_TYPE,
    mut n1: _FP_W_TYPE,
    mut n0: _FP_W_TYPE,
    mut d1: _FP_W_TYPE,
    mut d0: _FP_W_TYPE,
) {
    let (mut q0, mut q1, mut r0, mut r1):
        (_FP_W_TYPE, _FP_W_TYPE, _FP_W_TYPE, _FP_W_TYPE);
    let (mut b, mut bm): (_FP_I_TYPE, _FP_I_TYPE);

    if d1 == 0 {
        #[cfg(not(feature = "udiv_needs_normalization"))]
        {
            if d0 > n1 {
                /* 0q = nn / 0D */
                udiv_qrnnd(&mut q0, &mut n0, n1, n0, d0);
                q1 = 0;
                /* Remainder in n0. */
            } else {
                /* qq = NN / 0d */
                if d0 == 0 { d0 = 1 / d0; }
                udiv_qrnnd(&mut q1, &mut n1, 0, n1, d0);
                udiv_qrnnd(&mut q0, &mut n0, n1, n0, d0);
                /* Remainder in n0. */
            }
            r0 = n0;
            r1 = 0;
        }

        #[cfg(feature = "udiv_needs_normalization")]
        {
            if d0 > n1 {
                /* 0q = nn / 0D */
                bm = count_leading_zeros(d0);
                if bm != 0 {
                    d0 = d0 << bm;
                    n1 = (n1 << bm) | (n0 >> (_FP_W_TYPE_SIZE - bm));
                    n0 = n0 << bm;
                }
                udiv_qrnnd(&mut q0, &mut n0, n1, n0, d0);
                q1 = 0;
            } else {
                /* qq = NN / 0d */
                if d0 == 0 { d0 = 1 / d0; }
                bm = count_leading_zeros(d0);
                if bm == 0 {
                    n1 -= d0;
                    q1 = 1;
                } else {
                    let mut n2: _FP_W_TYPE;
                    b = _FP_W_TYPE_SIZE - bm;
                    d0 = d0 << bm;
                    n2 = n1 >> b;
                    n1 = (n1 << bm) | (n0 >> b);
                    n0 = n0 << bm;
                    udiv_qrnnd(&mut q1, &mut n1, n2, n1, d0);
                }
                udiv_qrnnd(&mut q0, &mut n0, n1, n0, d0);
            }
            r0 = n0 >> bm;
            r1 = 0;
        }
    } else if d1 > n1 {
        q0 = 0;
        q1 = 0;
        r0 = n0;
        r1 = n1;
    } else {
        bm = count_leading_zeros(d1);
        if bm == 0 {
            if n1 > d1 || n0 >= d0 {
                q0 = 1;
                sub_ddmmss(&mut n1, &mut n0, n1, n0, d1, d0);
            } else { q0 = 0; }
            q1 = 0;
            r0 = n0;
            r1 = n1;
        } else {
            let (mut m1, mut m0, mut n2): (_FP_W_TYPE, _FP_W_TYPE, _FP_W_TYPE);
            b = _FP_W_TYPE_SIZE - bm;
            d1 = (d1 << bm) | (d0 >> b);
            d0 = d0 << bm;
            n2 = n1 >> b;
            n1 = (n1 << bm) | (n0 >> b);
            n0 = n0 << bm;
            udiv_qrnnd(&mut q0, &mut n1, n2, n1, d1);
            umul_ppmm(&mut m1, &mut m0, q0, d0);
            if m1 > n1 || (m1 == n1 && m0 > n0) {
                q0 -= 1;
                sub_ddmmss(&mut m1, &mut m0, m1, m0, d1, d0);
            }
            q1 = 0;
            sub_ddmmss(&mut n1, &mut n0, n1, n0, m1, m0);
            r0 = (n1 << b) | (n0 >> bm);
            r1 = n1 >> bm;
        }
    }

    *q.add(0) = q0;
    *q.add(1) = q1;
    *r.add(0) = r0;
    *r.add(1) = r1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
