/*

  fp_trig.c: floating-point math routines for the Linux-m68k
  floating point emulator.

  Copyright (c) 1998-1999 David Huggins-Daines / Roman Zippel.

  I hereby give permission, free of charge, to copy, modify, and
  redistribute this software, in source or binary form, provided that
  the above copyright notice and the following disclaimer are included
  in all such copies.

  THIS SOFTWARE IS PROVIDED "AS IS", WITH ABSOLUTELY NO WARRANTY, REAL
  OR IMPLIED.

*/

use core::ffi::c_char;

// Declarations supplied by the surrounding emulator sources.
#[repr(C)]
pub struct fp_ext {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn uprint(fmt: *const c_char);
    fn fp_monadic_check(dest: *mut fp_ext, src: *mut fp_ext);
}

pub unsafe extern "C" fn fp_fsin(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fsin\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe extern "C" fn fp_fcos(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fcos\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe extern "C" fn fp_ftan(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"ftan\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe extern "C" fn fp_fasin(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fasin\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe extern "C" fn fp_facos(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"facos\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe extern "C" fn fp_fatan(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fatan\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe extern "C" fn fp_fsinh(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fsinh\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe extern "C" fn fp_fcosh(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fcosh\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe extern "C" fn fp_ftanh(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"ftanh\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe extern "C" fn fp_fatanh(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fatanh\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe extern "C" fn fp_fsincos0(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fsincos0\n".as_ptr());
    dest
}

pub unsafe extern "C" fn fp_fsincos1(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fsincos1\n".as_ptr());
    dest
}

pub unsafe extern "C" fn fp_fsincos2(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fsincos2\n".as_ptr());
    dest
}

pub unsafe extern "C" fn fp_fsincos3(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fsincos3\n".as_ptr());
    dest
}

pub unsafe extern "C" fn fp_fsincos4(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fsincos4\n".as_ptr());
    dest
}

pub unsafe extern "C" fn fp_fsincos5(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fsincos5\n".as_ptr());
    dest
}

pub unsafe extern "C" fn fp_fsincos6(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fsincos6\n".as_ptr());
    dest
}

pub unsafe extern "C" fn fp_fsincos7(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(c"fsincos7\n".as_ptr());
    dest
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
