/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from asm-generic/div64.h. */

/* The original header includes linux/types.h and linux/compiler.h. */

/* BITS_PER_LONG == 64 */
#[cfg(target_pointer_width = "64")]
#[macro_export]
macro_rules! do_div {
    ($n:expr, $base:expr) => {{
        let __base: u32 = $base as u32;
        let __rem: u32 = (($n as u64) % __base as u64) as u32;
        $n = (($n as u64) / __base as u64) as _;
        __rem
    }};
}

/* BITS_PER_LONG == 32 */
#[cfg(target_pointer_width = "32")]
pub mod div64_32 {
    /* The original header includes linux/log2.h. */

    /*
     * Default C implementation for __arch_xprod_64().
     * Semantic: retval = ((bias ? m : 0) + m * n) >> 64.
     */
    #[inline]
    pub fn __arch_xprod_64(m: u64, n: u64, bias: bool) -> u64 {
        let m_lo = m as u32;
        let m_hi = m >> 32;
        let n_lo = n as u32;
        let n_hi = n >> 32;
        let mut x: u64;
        let mut y: u64;

        /* __builtin_constant_p(m) is a compiler/build-time property in C. */
        let no_ovf = ((m >> 32).wrapping_add(m & 0xffff_ffff) < 0x1_0000_0000);

        if no_ovf {
            x = (m_lo as u64).wrapping_mul(n_lo as u64)
                .wrapping_add(if bias { m } else { 0 });
            x >>= 32;
            x = x.wrapping_add((m_lo as u64).wrapping_mul(n_hi as u64));
            x = x.wrapping_add((m_hi as u64).wrapping_mul(n_lo as u64));
            x >>= 32;
            x = x.wrapping_add((m_hi as u64).wrapping_mul(n_hi as u64));
        } else {
            x = (m_lo as u64).wrapping_mul(n_lo as u64)
                .wrapping_add(if bias { m_lo as u64 } else { 0 });
            y = (m_lo as u64).wrapping_mul(n_hi as u64)
                .wrapping_add(x >> 32)
                .wrapping_add(if bias { m_hi } else { 0 });
            x = (m_hi as u64).wrapping_mul(n_hi as u64).wrapping_add(y >> 32);
            y = (m_hi as u64).wrapping_mul(n_lo as u64).wrapping_add(y as u32 as u64);
            x = x.wrapping_add(y >> 32);
        }
        x
    }

    pub fn __div64_const32(n: u64, b: u32) -> u64 {
        let mut res: u64;
        let mut x: u64;
        let mut t: u64;
        let mut m: u64;
        let mut p: u32;
        let mut bias = false;

        p = 1u32 << unsafe { ilog2(b) };
        m = (u64::MAX / b as u64).wrapping_mul(p as u64);
        m = m.wrapping_add(
            (((u64::MAX % b as u64 + 1).wrapping_mul(p as u64))
                .wrapping_add(b as u64 - 1)) / b as u64,
        );
        x = (u64::MAX / b as u64).wrapping_mul(b as u64).wrapping_sub(1);
        res = (m & 0xffff_ffff).wrapping_mul(x & 0xffff_ffff);
        t = (m & 0xffff_ffff).wrapping_mul(x >> 32).wrapping_add(res >> 32);
        res = (m >> 32).wrapping_mul(x >> 32).wrapping_add(t >> 32);
        t = (m >> 32).wrapping_mul(x & 0xffff_ffff).wrapping_add(t & 0xffff_ffff);
        res = res.wrapping_add(t >> 32) / p as u64;

        if res != x / b as u64 {
            bias = true;
            m = (u64::MAX / b as u64).wrapping_mul(p as u64);
            m = m.wrapping_add(((u64::MAX % b as u64 + 1).wrapping_mul(p as u64)) / b as u64);
        }

        let lowbit = m & m.wrapping_neg();
        p = (p as u64 / lowbit) as u32;
        m /= lowbit;
        __arch_xprod_64(m, n, bias) / p as u64
    }

    /* External dependency supplied by the surrounding kernel translation. */
    unsafe extern "C" {
        pub fn __div64_32(dividend: *mut u64, divisor: u32) -> u32;
        pub fn ilog2(value: u32) -> u32;
        pub fn is_power_of_2(value: u32) -> bool;
    }

    #[macro_export]
    macro_rules! do_div {
        ($n:expr, $base:expr) => {{
            let __base: u32 = $base as u32;
            let __rem: u32;
            /* C's __builtin_constant_p branches are preserved as runtime-shaped logic. */
            if __base != 0 && $crate::div64_32::is_power_of_2(__base) {
                __rem = ($n as u32) & (__base - 1);
                $n >>= $crate::div64_32::ilog2(__base);
            } else if __base != 0 {
                let __n_lo: u64 = $n;
                $n = $crate::div64_32::__div64_const32($n as u64, __base) as _;
                let __res_lo: u64 = $n;
                __rem = (__n_lo - __res_lo.wrapping_mul(__base as u64)) as u32;
            } else if (($n as u64) >> 32) == 0 {
                __rem = (($n as u64) as u32) % __base;
                $n = (($n as u64) as u32 / __base) as _;
            } else {
                __rem = unsafe { $crate::div64_32::__div64_32(&mut $n as *mut u64, __base) };
            }
            __rem
        }};
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
