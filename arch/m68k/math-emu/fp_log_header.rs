/*

  fp_log.h: floating-point math routines for the Linux-m68k
  floating point emulator.

  Copyright (c) 1998-1999 David Huggins-Daines / Roman Zippel.

  I hereby give permission, free of charge, to copy, modify, and
  redistribute this software, in source or binary form, provided that
  the above copyright notice and the following disclaimer are included
  in all such copies.

  THIS SOFTWARE IS PROVIDED "AS IS", WITH ABSOLUTELY NO WARRANTY, REAL
  OR IMPLIED.

*/

// Dependency intent: declarations from "fp_emu.h" are supplied externally.

/* floating point logarithmic instructions:

   the arguments to these are in the "internal" extended format, that
   is, an "exploded" version of the 96-bit extended fp format used by
   the 68881.

   they return a status code, which should end up in %d0, if all goes
   well.  */

#[repr(C)]
pub struct fp_ext {
    _private: [u8; 0],
}

extern "C" {
    pub fn fp_fsqrt(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fetoxm1(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fetox(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_ftwotox(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_ftentox(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_flogn(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_flognp1(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_flog10(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_flog2(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fgetexp(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
    pub fn fp_fgetman(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
