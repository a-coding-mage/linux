/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/sparc64/math-emu/sfp-util.h
 *
 * Copyright (C) 1999 Jakub Jelinek (jj@ultra.linux.cz)
 * Copyright (C) 1999 David S. Miller (davem@redhat.com)
 */

// Dependencies supplied by the surrounding translation unit:
// linux/kernel.h, linux/sched.h, linux/types.h, and asm/byteorder.h.

/* The original implementations use SPARC inline assembly. */
macro_rules! add_ssaaaa {
    ($sh:expr, $sl:expr, $ah:expr, $al:expr, $bh:expr, $bl:expr) => {{
        let (__hi, __lo) = (($ah as UDItype).overflowing_add($bh as UDItype));
        let (__sum, __carry) = (($al as UDItype).overflowing_add($bl as UDItype));
        $sl = __sum;
        $sh = __hi.wrapping_add(__carry as UDItype);
    }};
}

macro_rules! sub_ddmmss {
    ($sh:expr, $sl:expr, $ah:expr, $al:expr, $bh:expr, $bl:expr) => {{
        let (__diff, __borrow) = (($al as UDItype).overflowing_sub($bl as UDItype));
        $sl = __diff;
        $sh = ($ah as UDItype).wrapping_sub($bh as UDItype)
            .wrapping_sub(__borrow as UDItype);
    }};
}

macro_rules! umul_ppmm {
    ($wh:expr, $wl:expr, $u:expr, $v:expr) => {{
        let __product = ($u as u128).wrapping_mul($v as u128);
        $wl = __product as UDItype;
        $wh = (__product >> 64) as UDItype;
    }};
}

macro_rules! udiv_qrnnd {
    ($q:expr, $r:expr, $n1:expr, $n0:expr, $d:expr) => {{
        let __numerator = (($n1 as u128) << 64) | ($n0 as u128);
        let __divisor = $d as u128;
        $q = (__numerator / __divisor) as UWtype;
        $r = (__numerator % __divisor) as UWtype;
    }};
}

const UDIV_NEEDS_NORMALIZATION: i32 = 1;

macro_rules! abort {
    () => { return 0 };
}

#[cfg(target_endian = "big")]
const __BYTE_ORDER: i32 = __BIG_ENDIAN;
#[cfg(not(target_endian = "big"))]
const __BYTE_ORDER: i32 = __LITTLE_ENDIAN;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
