// SPDX-License-Identifier: GPL-2.0

// The original implementation is enabled only when CONFIG_64BIT,
// CONFIG_CPU_MIPSR6, and GCC < 10 are defined.  These build-time conditions
// are preserved here as an intentional conditional-compilation boundary.

/* multiply 64-bit values, low 64-bits returned */
#[inline]
unsafe fn dmulu(a: i64, b: i64) -> i64 {
    let res: i64;
    core::arch::asm!(
        "dmulu {res}, {a}, {b}",
        res = out(reg) res,
        a = in(reg) a,
        b = in(reg) b,
    );
    res
}

/* multiply 64-bit unsigned values, high 64-bits of 128-bit result returned */
#[inline]
unsafe fn dmuhu(a: i64, b: i64) -> i64 {
    let res: i64;
    core::arch::asm!(
        "dmuhu {res}, {a}, {b}",
        res = out(reg) res,
        a = in(reg) a,
        b = in(reg) b,
    );
    res
}

/* multiply 128-bit values, low 128-bits returned */
#[inline]
pub unsafe fn __multi3(a: ti_type, b: ti_type) -> ti_type {
    let mut res: TWunion = core::mem::MaybeUninit::zeroed().assume_init();
    let mut aa: TWunion = core::mem::MaybeUninit::zeroed().assume_init();
    let mut bb: TWunion = core::mem::MaybeUninit::zeroed().assume_init();

    aa.ti = a;
    bb.ti = b;

    /*
     * a * b =           (a.lo * b.lo)
     *         + 2^64  * (a.hi * b.lo + a.lo * b.hi)
     *        [+ 2^128 * (a.hi * b.hi)]
     */
    res.s.low = dmulu(aa.s.low, bb.s.low);
    res.s.high = dmuhu(aa.s.low, bb.s.low);
    res.s.high = res.s.high.wrapping_add(dmulu(aa.s.high, bb.s.low));
    res.s.high = res.s.high.wrapping_add(dmulu(aa.s.low, bb.s.high));

    res.ti
}

// EXPORT_SYMBOL(__multi3);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
