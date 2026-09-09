/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_X86_32 selects the 32-bit implementation in the original header.
#[cfg(target_pointer_width = "32")]
#[macro_export]
macro_rules! do_div {
    ($n:expr, $base:expr) => {{
        let mut __upper: usize;
        let mut __low: usize;
        let mut __high: usize;
        let mut __mod: usize;
        let __base: usize = $base;
        if __base.is_power_of_two() {
            __mod = $n & (__base - 1);
            $n >>= __base.trailing_zeros();
        } else {
            unsafe {
                core::arch::asm!("", out("eax") __low, out("edx") __high, inout("eax") $n => _, options(nostack));
            }
            __upper = __high;
            if __high != 0 {
                __upper = __high % __base;
                __high /= __base;
            }
            unsafe {
                core::arch::asm!(
                    "divl {base}",
                    base = in(reg) __base,
                    inout("eax") __low,
                    lateout("edx") __mod,
                    in("edx") __upper,
                );
                core::arch::asm!("", inout("eax") __low, inout("edx") __high);
            }
            $n = ((__high as u64) << 32) | (__low as u64);
        }
        __mod
    }};
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
union Div64Parts {
    v64: u64,
    v32: [u32; 2],
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn div_u64_rem(dividend: u64, divisor: u32, remainder: *mut u32) -> u64 {
    let mut d = Div64Parts { v64: dividend };
    let mut upper: u32;

    upper = unsafe { d.v32[1] };
    unsafe { d.v32[1] = 0 };
    if upper >= divisor {
        unsafe { d.v32[1] = upper / divisor };
        upper %= divisor;
    }
    unsafe {
        core::arch::asm!(
            "divl {divisor}",
            divisor = in(reg) divisor,
            inout("eax") d.v32[0],
            lateout("edx") *remainder,
            in("edx") upper,
        );
        d.v64
    }
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn mul_u32_u32(a: u32, b: u32) -> u64 {
    let high: u32;
    let low: u32;

    unsafe {
        core::arch::asm!("mull {b}", b = in(reg) b, out("eax") low, out("edx") high, in("eax") a);
    }
    (low as u64) | ((high as u64) << 32)
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn add_u64_u32(a: u64, b: u32) -> u64 {
    let mut high = (a >> 32) as u32;
    let mut low = a as u32;

    unsafe {
        core::arch::asm!(
            "add {b}, {low}; adc $0, {high}",
            b = in(reg) b,
            low = inout(reg) low,
            high = inout(reg) high,
        );
    }
    (low as u64) | ((high as u64) << 32)
}

// __div64_32() is never called on x86, so prevent the generic definition
// from getting built.

#[cfg(not(target_pointer_width = "32"))]
#[inline]
pub unsafe fn mul_u64_add_u64_div_u64(mut rax: u64, mul: u64, add: u64, div: u64) -> u64 {
    let rdx: u64;

    unsafe {
        core::arch::asm!("mul {mul}", mul = in(reg) mul, inout("rax") rax, lateout("rdx") rdx);
    }

    if add != 0 {
        unsafe {
            core::arch::asm!(
                "add {add}, {lo}; adc $0, {hi}",
                add = in(reg) add,
                lo = inout(reg) rax,
                hi = inout(reg) rdx,
            );
        }
    }

    unsafe {
        core::arch::asm!("div {div}", div = in(reg) div, inout("rax") rax, inout("rdx") rdx);
    }
    rax
}

#[cfg(not(target_pointer_width = "32"))]
#[inline]
pub unsafe fn mul_u64_u32_div(a: u64, mul: u32, div: u32) -> u64 {
    unsafe { mul_u64_add_u64_div_u64(a, mul as u64, 0, div as u64) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
