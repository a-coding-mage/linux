/* SPDX-License-Identifier: GPL-2.0 */

/*
 * When CONFIG_CPU_HAS_NO_MULDIV64 is enabled, this header includes the
 * generic implementation.  That dependency is supplied by the surrounding
 * build and is intentionally not reproduced here.
 */

#[cfg(not(CONFIG_CPU_HAS_NO_MULDIV64))]
#[repr(C)]
union DoDivValue {
    n32: [usize; 2],
    n64: u64,
}

/* n = n / base; return rem; */
#[cfg(not(CONFIG_CPU_HAS_NO_MULDIV64))]
#[macro_export]
macro_rules! do_div {
    ($n:expr, $base:expr) => {{
        let mut __n = $crate::DoDivValue { n64: $n };
        let mut __rem: usize;
        let mut __upper: usize;
        let __base: usize = $base;

        unsafe {
            __upper = __n.n32[0];
            if __upper != 0 {
                core::arch::asm!(
                    "divul.l {base},{upper}:{low}",
                    base = in(reg) __base,
                    upper = inout(reg) __upper,
                    low = inout(reg) __n.n32[0],
                    options(nostack)
                );
            }
            core::arch::asm!(
                "divu.l {base},{rem}:{low}",
                base = in(reg) __base,
                rem = lateout(reg) __rem,
                upper = in(reg) __upper,
                low = inout(reg) __n.n32[1],
                options(nostack)
            );
            $n = __n.n64;
        }
        __rem
    }};
}

/* Defining this stops the unused helper function from being built. */
#[cfg(not(CONFIG_CPU_HAS_NO_MULDIV64))]
#[macro_export]
macro_rules! __div64_32 {
    ($($tokens:tt)*) => { $crate::__div64_32!($($tokens)*) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
