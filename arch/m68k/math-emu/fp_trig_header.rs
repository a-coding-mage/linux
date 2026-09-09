/*

  fp_trig.h: floating-point math routines for the Linux-m68k
  floating point emulator.

  Copyright (c) 1998 David Huggins-Daines.

  I hereby give permission, free of charge, to copy, modify, and
  redistribute this software, in source or binary form, provided that
  the above copyright notice and the following disclaimer are included
  in all such copies.

  THIS SOFTWARE IS PROVIDED "AS IS", WITH ABSOLUTELY NO WARRANTY, REAL
  OR IMPLIED.

*/

// Dependency supplied by the translated fp_emu interface.
pub struct fp_ext;

/* floating point trigonometric instructions:

   the arguments to these are in the "internal" extended format, that
   is, an "exploded" version of the 96-bit extended fp format used by
   the 68881.

   they return a status code, which should end up in %d0, if all goes
   well.  */

unsafe extern "C" {
    pub fn fp_fsin(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fcos(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_ftan(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fasin(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_facos(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fatan(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fsinh(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fcosh(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_ftanh(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fatanh(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fsincos0(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fsincos1(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fsincos2(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fsincos3(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fsincos4(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fsincos5(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fsincos6(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fsincos7(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
