/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies from the original header:
 * stdarg.h, stdlib.h, stdio.h, linux/compiler.h
 */

use core::ffi::{c_char, c_int, c_void, VaList, VaListImpl};

unsafe extern "C" {
    static mut stderr: *mut c_void;

    fn vsnprintf(
        s: *mut c_char,
        n: usize,
        format: *const c_char,
        arg: VaList,
    ) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
}

#[inline]
pub unsafe fn report(prefix: *const c_char, err: *const c_char, params: VaList) {
    let mut msg: [c_char; 1024] = [0; 1024];

    unsafe {
        vsnprintf(msg.as_mut_ptr(), core::mem::size_of_val(&msg), err, params);
        fprintf(
            stderr,
            c" %s%s\n".as_ptr(),
            prefix,
            msg.as_mut_ptr() as *const c_char,
        );
    }
}

#[inline]
pub unsafe extern "C" fn die(err: *const c_char, mut args: ...) -> ! {
    unsafe {
        let params: VaListImpl = args.clone();

        report(c" Fatal: ".as_ptr(), err, params.as_va_list());
        exit(128);
    }
}

macro_rules! zfree {
    ($ptr:expr) => {{
        unsafe {
            free(*$ptr as *mut c_void);
            *$ptr = core::ptr::null_mut();
        }
    }};
}

pub(crate) use zfree;

macro_rules! alloc_nr {
    ($x:expr) => {
        ((($x) + 16) * 3 / 2)
    };
}

pub(crate) use alloc_nr;

/*
 * Realloc the buffer pointed at by variable 'x' so that it can hold
 * at least 'nr' entries; the number of entries currently allocated
 * is 'alloc', using the standard growing factor alloc_nr() macro.
 *
 * DO NOT USE any expression with side-effect for 'x' or 'alloc'.
 */
macro_rules! ALLOC_GROW {
    ($x:expr, $nr:expr, $alloc:expr) => {{
        if ($nr) > $alloc {
            if alloc_nr!($alloc) < ($nr) {
                $alloc = ($nr);
            } else {
                $alloc = alloc_nr!($alloc);
            }
            $x = unsafe {
                xrealloc(
                    ($x) as *mut c_void,
                    ($alloc) * core::mem::size_of_val(&*($x)),
                ) as _
            };
        }
    }};
}

pub(crate) use ALLOC_GROW;

#[inline]
pub unsafe fn xrealloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    let ret: *mut c_void = unsafe { realloc(ptr, size) };

    if ret.is_null() {
        unsafe {
            die(c"Out of memory, realloc failed".as_ptr());
        }
    }

    ret
}

macro_rules! astrcatf {
    ($out:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            let tmp: *mut c_char = *$out;
            if asprintf(
                $out,
                concat!("%s", $fmt).as_ptr() as *const c_char,
                if tmp.is_null() { c"".as_ptr() } else { tmp as *const c_char },
                $($arg,)*
            ) == -1
            {
                die(c"asprintf failed".as_ptr());
            }
            free(tmp as *mut c_void);
        }
    }};
}

pub(crate) use astrcatf;

#[inline]
pub unsafe fn astrcat(out: *mut *mut c_char, add: *const c_char) {
    let tmp: *mut c_char = unsafe { *out };

    unsafe {
        if asprintf(
            out,
            c"%s%s".as_ptr(),
            if tmp.is_null() { c"".as_ptr() } else { tmp as *const c_char },
            add,
        ) == -1
        {
            die(c"asprintf failed".as_ptr());
        }

        free(tmp as *mut c_void);
    }
}
