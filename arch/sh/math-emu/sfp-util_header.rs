/* SPDX-License-Identifier: GPL-2.0 */
/*
 * These are copied from glibc/stdlib/longlong.h
 */

macro_rules! add_ssaaaa {
    ($sh:expr, $sl:expr, $ah:expr, $al:expr, $bh:expr, $bl:expr) => {{
        let __x = ($al).wrapping_add($bl);
        ($sh) = ($ah).wrapping_add($bh).wrapping_add((__x < ($al)) as _);
        ($sl) = __x;
    }};
}

macro_rules! sub_ddmmss {
    ($sh:expr, $sl:expr, $ah:expr, $al:expr, $bh:expr, $bl:expr) => {{
        let __x = ($al).wrapping_sub($bl);
        ($sh) = ($ah).wrapping_sub($bh).wrapping_sub((__x > ($al)) as _);
        ($sl) = __x;
    }};
}

/* The original is a SH dmulu.l inline-assembly macro. */
macro_rules! umul_ppmm {
    ($w1:expr, $w0:expr, $u:expr, $v:expr) => {{
        unsafe {
            core::arch::asm!(
                "dmulu.l {u},{v}\n\tsts macl,{w0}\n\tsts mach,{w1}",
                u = in(reg) ($u as u32),
                v = in(reg) ($v as u32),
                w0 = out(reg) ($w0 as u32),
                w1 = out(reg) ($w1 as u32),
                clobber_abi("C"),
            );
        }
    }};
}

macro_rules! __ll_B {
    () => { ((1 as UWtype) << (W_TYPE_SIZE / 2)) };
}

macro_rules! __ll_lowpart {
    ($t:expr) => { (($t as UWtype) & (__ll_B!() - 1)) };
}

macro_rules! __ll_highpart {
    ($t:expr) => { (($t as UWtype) >> (W_TYPE_SIZE / 2)) };
}

macro_rules! udiv_qrnnd {
    ($q:expr, $r:expr, $n1:expr, $n0:expr, $d:expr) => {{
        let __d1: UWtype = __ll_highpart!($d);
        let __d0: UWtype = __ll_lowpart!($d);

        let mut __r1: UWtype = ($n1) % __d1;
        let mut __q1: UWtype = ($n1) / __d1;
        let mut __m: UWtype = __q1.wrapping_mul(__d0);
        __r1 = __r1.wrapping_mul(__ll_B!()) | __ll_highpart!($n0);
        if __r1 < __m {
            __q1 = __q1.wrapping_sub(1);
            __r1 = __r1.wrapping_add($d);
            if __r1 >= ($d) {
                /* i.e. we didn't get carry when adding to __r1 */
                if __r1 < __m {
                    __q1 = __q1.wrapping_sub(1);
                    __r1 = __r1.wrapping_add($d);
                }
            }
        }
        __r1 = __r1.wrapping_sub(__m);

        let mut __r0: UWtype = __r1 % __d1;
        let __q0: UWtype = __r1 / __d1;
        __m = __q0.wrapping_mul(__d0);
        __r0 = __r0.wrapping_mul(__ll_B!()) | __ll_lowpart!($n0);
        if __r0 < __m {
            __r0 = __r0.wrapping_sub(1);
            __r0 = __r0.wrapping_add($d);
            if __r0 >= ($d) {
                if __r0 < __m {
                    __r0 = __r0.wrapping_sub(1);
                    __r0 = __r0.wrapping_add($d);
                }
            }
        }
        __r0 = __r0.wrapping_sub(__m);

        ($q) = __q1.wrapping_mul(__ll_B!()) | __q0;
        ($r) = __r0;
    }};
}

macro_rules! abort {
    () => { return 0 };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
