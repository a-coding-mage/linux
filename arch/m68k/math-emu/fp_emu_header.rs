/*
 * fp_emu.h
 *
 * Copyright Roman Zippel, 1997.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, and the entire permission notice in its entirety,
 *    including the disclaimer of warranties.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote
 *    products derived from this software without specific prior
 *    written permission.
 *
 * ALTERNATIVELY, this product may be distributed under the terms of
 * the GNU General Public License, in which case the provisions of the GPL are
 * required INSTEAD OF the above restrictions.
 */

// Dependency intent from the C header: <asm/asm-offsets.h> and <asm/math-emu.h>.

#[allow(non_upper_case_globals)]
extern "C" {
    pub static fp_QNaN: fp_ext;
    pub static fp_Inf: fp_ext;
    pub fn fp_conv_ext2ext(src: *mut fp_ext) -> i32;
    pub fn fp_conv_ext2long(src: *mut fp_ext) -> i32;
}

// `struct fp_ext` and `FPDATA` are supplied by the math-emulation dependencies.
// The field layout used here is the source-level layout expected by this header.
#[allow(non_camel_case_types)]
pub type fp_ext = crate::fp_ext;

#[macro_export]
macro_rules! IS_INF {
    ($a:expr) => { unsafe { (*($a)).exp == 0x7fff } };
}

#[macro_export]
macro_rules! IS_ZERO {
    ($a:expr) => { unsafe { (*($a)).mant.m64 == 0 } };
}

#[macro_export]
macro_rules! fp_set_sr {
    ($bit:expr) => {{
        unsafe { (*$crate::FPDATA).fpsr |= 1u32 << ($bit); }
    }};
}

#[macro_export]
macro_rules! fp_set_quotient {
    ($quotient:expr) => {{
        unsafe {
            (*$crate::FPDATA).fpsr &= 0xff00ffff;
            (*$crate::FPDATA).fpsr |= (($quotient) & 0xff) << 16;
        }
    }};
}

#[macro_export]
macro_rules! fp_normalize_ext {
    ($fpreg:expr) => {{
        unsafe { $crate::fp_conv_ext2ext($fpreg) }
    }};
}

#[macro_export]
macro_rules! fp_copy_ext {
    ($dest:expr, $src:expr) => {{
        unsafe { *($dest) = *($src); }
    }};
}

#[macro_export]
macro_rules! fp_monadic_check {
    ($dest:expr, $src:expr) => {{
        $crate::fp_copy_ext!($dest, $src);
        if !$crate::fp_normalize_ext!($dest) {
            return $dest;
        }
    }};
}

#[macro_export]
macro_rules! fp_dyadic_check {
    ($dest:expr, $src:expr) => {{
        if !$crate::fp_normalize_ext!($dest) {
            return $dest;
        }
        if !$crate::fp_normalize_ext!($src) {
            $crate::fp_copy_ext!($dest, $src);
            return $dest;
        }
    }};
}

#[macro_export]
macro_rules! fp_set_nan {
    ($dest:expr) => {{
        $crate::fp_set_sr!($crate::FPSR_EXC_OPERR);
        unsafe { *($dest) = $crate::fp_QNaN; }
    }};
}

/* TODO check rounding mode? */
#[macro_export]
macro_rules! fp_set_ovrflw {
    ($dest:expr) => {{
        $crate::fp_set_sr!($crate::FPSR_EXC_OVFL);
        unsafe {
            (*($dest)).exp = 0x7fff;
            (*($dest)).mant.m64 = 0;
        }
    }};
}

#[macro_export]
macro_rules! fp_conv_ext2long {
    ($src:expr) => {{
        unsafe { $crate::fp_conv_ext2long($src) }
    }};
}

#[macro_export]
macro_rules! fp_conv_long2ext {
    ($dest:expr, $src:expr) => {{
        // Preserves the source macro's assembly target and argument ordering.
        unsafe { $crate::fp_conv_ext2long($dest); }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
