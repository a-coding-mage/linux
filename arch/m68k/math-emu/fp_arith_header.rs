/* SPDX-License-Identifier: GPL-2.0-or-later */
/*

   fp_arith.h: floating-point math routines for the Linux-m68k
   floating point emulator.

   Copyright (c) 1998 David Huggins-Daines.

   Somewhat based on the AlphaLinux floating point emulator, by David
   Mosberger-Tang.


 */

// C header guard: _FP_ARITH_H

// Forward declaration supplied by the surrounding translation unit.
pub struct fp_ext;

/* easy ones */
unsafe extern "C" {
    pub fn fp_fabs(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fneg(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;

    /* straightforward arithmetic */
    pub fn fp_fadd(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fsub(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fcmp(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_ftst(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fmul(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fdiv(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fsglmul(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fsgldiv(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;

    /* ones that do rounding and integer conversions */
    pub fn fp_fmod(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_frem(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fint(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fintrz(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fscale(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
