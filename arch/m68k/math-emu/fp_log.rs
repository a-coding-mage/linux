/*

  fp_log.c: floating-point math routines for the Linux-m68k
  floating point emulator.

  Copyright (c) 1998-1999 David Huggins-Daines / Roman Zippel.

  I hereby give permission, free of charge, to copy, modify, and
  redistribute this software, in source or binary form, provided that
  the above copyright notice and the following disclaimer are included
  in all such copies.

  THIS SOFTWARE IS PROVIDED "AS IS", WITH ABSOLUTELY NO WARRANTY, REAL
  OR IMPLIED.

*/

// Dependencies supplied by fp_arith.h, fp_emu.h, and fp_log.h are external.

#[repr(C)]
pub struct fp_ext {
    pub exp: i32,
    pub sign: i32,
}

static FP_ONE: fp_ext = fp_ext { exp: 0x3fff, sign: 0 };

extern "C" {
    fn dprint(level: i32, message: *const u8);
    fn uprint(message: *const u8);
    fn fp_monadic_check(dest: *mut fp_ext, src: *mut fp_ext);
    fn fp_copy_ext(dest: *mut fp_ext, src: *mut fp_ext);
    fn fp_fadd(dest: *mut fp_ext, src: *mut fp_ext);
    fn fp_fdiv(dest: *mut fp_ext, src: *mut fp_ext);
    fn fp_set_nan(dest: *mut fp_ext);
    fn fp_conv_long2ext(dest: *mut fp_ext, value: i32);
    fn fp_normalize_ext(dest: *mut fp_ext);
    fn IS_ZERO(value: *mut fp_ext) -> bool;
    fn IS_INF(value: *mut fp_ext) -> bool;
}

pub unsafe fn fp_fsqrt(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    let mut tmp: fp_ext = core::mem::zeroed();
    let mut src2: fp_ext = core::mem::zeroed();
    let mut i: i32;
    let exp: i32;

    dprint(0, b"fsqrt\n".as_ptr());

    fp_monadic_check(dest, src);

    if IS_ZERO(dest) {
        return dest;
    }

    if (*dest).sign != 0 {
        fp_set_nan(dest);
        return dest;
    }
    if IS_INF(dest) {
        return dest;
    }

    /*
     * sqrt(m) * 2^(p), if e = 2*p
     * sqrt(m*2^e) = sqrt(2*m) * 2^(p), if e = 2*p + 1
     */
    exp = (*dest).exp;
    (*dest).exp = 0x3fff;
    if (exp & 1) == 0 {
        (*dest).exp += 1;
    }
    fp_copy_ext(&mut src2, dest);

    /* sqrt(x) = 1 + 1/2*(x-1) = 1/2*(1+x) */
    fp_fadd(dest, &FP_ONE as *const fp_ext as *mut fp_ext);
    (*dest).exp -= 1;

    /* Newton iteration: x' = (x + r/x) / 2. */
    i = 0;
    while i < 9 {
        fp_copy_ext(&mut tmp, &mut src2);
        fp_fdiv(&mut tmp, dest);
        fp_fadd(dest, &mut tmp);
        (*dest).exp -= 1;
        i += 1;
    }

    (*dest).exp += (exp - 0x3fff) / 2;

    dest
}

pub unsafe fn fp_fetoxm1(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(b"fetoxm1\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe fn fp_fetox(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(b"fetox\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe fn fp_ftwotox(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(b"ftwotox\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe fn fp_ftentox(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(b"ftentox\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe fn fp_flogn(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(b"flogn\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe fn fp_flognp1(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(b"flognp1\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe fn fp_flog10(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(b"flog10\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe fn fp_flog2(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    uprint(b"flog2\n".as_ptr());
    fp_monadic_check(dest, src);
    dest
}

pub unsafe fn fp_fgetexp(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    dprint(0, b"fgetexp\n".as_ptr());
    fp_monadic_check(dest, src);
    if IS_INF(dest) {
        fp_set_nan(dest);
        return dest;
    }
    if IS_ZERO(dest) {
        return dest;
    }
    fp_conv_long2ext(dest, (*dest).exp - 0x3fff);
    fp_normalize_ext(dest);
    dest
}

pub unsafe fn fp_fgetman(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    dprint(0, b"fgetman\n".as_ptr());
    fp_monadic_check(dest, src);
    if IS_ZERO(dest) {
        return dest;
    }
    if IS_INF(dest) {
        return dest;
    }
    (*dest).exp = 0x3fff;
    dest
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
