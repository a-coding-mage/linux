// SPDX-License-Identifier: GPL-2.0

use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use std::mem;
use std::ptr;

// C dependencies: <stdio.h>, <stdlib.h>, <string.h>, <helpers/bitmask.h>

#[repr(C)]
pub struct bitmask {
    pub size: c_uint,
    pub maskp: *mut c_ulong,
}

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
}

/* How many bits in an unsigned long */
const fn bitsperlong() -> usize {
    8 * mem::size_of::<c_ulong>()
}

/* howmany(a,b) : how many elements of size b needed to hold all of a */
const fn howmany(x: usize, y: usize) -> usize {
    (x + (y - 1)) / y
}

/* How many longs in mask of n bits */
const fn longsperbits(n: c_uint) -> usize {
    howmany(n as usize, bitsperlong())
}

fn max(a: c_int, b: c_int) -> c_int {
    if a > b { a } else { b }
}

/*
 * Allocate and free `struct bitmask *`
 */

/* Allocate a new `struct bitmask` with a size of n bits */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bitmask_alloc(n: c_uint) -> *mut bitmask {
    let bmp: *mut bitmask;

    bmp = unsafe { malloc(mem::size_of::<bitmask>()) as *mut bitmask };
    if bmp.is_null() {
        return ptr::null_mut();
    }
    unsafe {
        (*bmp).size = n;
        (*bmp).maskp = calloc(longsperbits(n), mem::size_of::<c_ulong>()) as *mut c_ulong;
    }
    if unsafe { (*bmp).maskp.is_null() } {
        unsafe { free(bmp as *mut c_void) };
        return ptr::null_mut();
    }
    bmp
}

/* Free `struct bitmask` */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bitmask_free(bmp: *mut bitmask) {
    if bmp.is_null() {
        return;
    }
    unsafe {
        free((*bmp).maskp as *mut c_void);
        (*bmp).maskp = 0xdeadcdefusize as *mut c_ulong; /* double free tripwire */
        free(bmp as *mut c_void);
    }
}

/*
 * The routines _getbit() and _setbit() are the only
 * routines that actually understand the layout of bmp->maskp[].
 *
 * On little endian architectures, this could simply be an array of
 * bytes.  But the kernel layout of bitmasks _is_ visible to userspace
 * via the sched_(set/get)affinity calls in Linux 2.6, and on big
 * endian architectures, it is painfully obvious that this is an
 * array of unsigned longs.
 */

/* Return the value (0 or 1) of bit n in bitmask bmp */
unsafe fn _getbit(bmp: *const bitmask, n: c_uint) -> c_uint {
    if n < unsafe { (*bmp).size } {
        unsafe {
            ((*(*bmp).maskp.add(n as usize / bitsperlong()) >> (n as usize % bitsperlong())) & 1)
                as c_uint
        }
    } else {
        0
    }
}

/* Set bit n in bitmask bmp to value v (0 or 1) */
unsafe fn _setbit(bmp: *mut bitmask, n: c_uint, v: c_uint) {
    if n < unsafe { (*bmp).size } {
        unsafe {
            let p = (*bmp).maskp.add(n as usize / bitsperlong());
            if v != 0 {
                *p |= 1 as c_ulong << (n as usize % bitsperlong());
            } else {
                *p &= !(1 as c_ulong << (n as usize % bitsperlong()));
            }
        }
    }
}

/*
 * When parsing bitmask lists, only allow numbers, separated by one
 * of the allowed next characters.
 *
 * The parameter 'sret' is the return from a sscanf "%u%c".  It is
 * -1 if the sscanf input string was empty.  It is 0 if the first
 * character in the sscanf input string was not a decimal number.
 * It is 1 if the unsigned number matching the "%u" was the end of the
 * input string.  It is 2 if one or more additional characters followed
 * the matched unsigned number.  If it is 2, then 'nextc' is the first
 * character following the number.  The parameter 'ok_next_chars'
 * is the nul-terminated list of allowed next characters.
 *
 * The mask term just scanned was ok if and only if either the numbers
 * matching the %u were all of the input or if the next character in
 * the input past the numbers was one of the allowed next characters.
 */
unsafe fn scan_was_ok(sret: c_int, nextc: c_char, ok_next_chars: *const c_char) -> c_int {
    (sret == 1 || (sret == 2 && unsafe { !strchr(ok_next_chars, nextc as c_int).is_null() }))
        as c_int
}

unsafe fn nexttoken(mut q: *const c_char, sep: c_int) -> *const c_char {
    if !q.is_null() {
        q = unsafe { strchr(q, sep) as *const c_char };
    }
    if !q.is_null() {
        q = unsafe { q.add(1) };
    }
    q
}

/* Set a single bit i in bitmask */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bitmask_setbit(bmp: *mut bitmask, i: c_uint) -> *mut bitmask {
    unsafe { _setbit(bmp, i, 1) };
    bmp
}

/* Set all bits in bitmask: bmp = ~0 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bitmask_setall(bmp: *mut bitmask) -> *mut bitmask {
    let mut i: c_uint;
    i = 0;
    while i < unsafe { (*bmp).size } {
        unsafe { _setbit(bmp, i, 1) };
        i += 1;
    }
    bmp
}

/* Clear all bits in bitmask: bmp = 0 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bitmask_clearall(bmp: *mut bitmask) -> *mut bitmask {
    let mut i: c_uint;
    i = 0;
    while i < unsafe { (*bmp).size } {
        unsafe { _setbit(bmp, i, 0) };
        i += 1;
    }
    bmp
}

/* True if all bits are clear */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bitmask_isallclear(bmp: *const bitmask) -> c_int {
    let mut i: c_uint;
    i = 0;
    while i < unsafe { (*bmp).size } {
        if unsafe { _getbit(bmp, i) } != 0 {
            return 0;
        }
        i += 1;
    }
    1
}

/* True if specified bit i is set */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bitmask_isbitset(bmp: *const bitmask, i: c_uint) -> c_int {
    unsafe { _getbit(bmp, i) as c_int }
}

/* Number of lowest set bit (min) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bitmask_first(bmp: *const bitmask) -> c_uint {
    unsafe { bitmask_next(bmp, 0) }
}

/* Number of highest set bit (max) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bitmask_last(bmp: *const bitmask) -> c_uint {
    let mut i: c_uint;
    let mut m: c_uint = unsafe { (*bmp).size };
    i = 0;
    while i < unsafe { (*bmp).size } {
        if unsafe { _getbit(bmp, i) } != 0 {
            m = i;
        }
        i += 1;
    }
    m
}

/* Number of next set bit at or above given bit i */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bitmask_next(bmp: *const bitmask, i: c_uint) -> c_uint {
    let mut n: c_uint;
    n = i;
    while n < unsafe { (*bmp).size } {
        if unsafe { _getbit(bmp, n) } != 0 {
            break;
        }
        n += 1;
    }
    n
}

/*
 * Parses a comma-separated list of numbers and ranges of numbers,
 * with optional ':%u' strides modifying ranges, into provided bitmask.
 * Some examples of input lists and their equivalent simple list:
 *	Input		Equivalent to
 *	0-3		0,1,2,3
 *	0-7:2		0,2,4,6
 *	1,3,5-7		1,3,5,6,7
 *	0-3:2,8-15:4	0,2,8,12
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bitmask_parselist(buf: *const c_char, bmp: *mut bitmask) -> c_int {
    let mut p: *const c_char;
    let mut q: *const c_char;

    unsafe { bitmask_clearall(bmp) };

    q = buf;
    loop {
        p = q;
        q = unsafe { nexttoken(q, b',' as c_int) };
        if p.is_null() {
            break;
        }
        let mut a: c_uint = 0; /* begin of range */
        let mut b: c_uint; /* end of range */
        let mut s: c_uint; /* stride */
        let mut c1: *const c_char; /* next tokens after '-' or ',' */
        let c2: *const c_char;
        let mut nextc: c_char = 0; /* char after sscanf %u match */
        let mut sret: c_int; /* sscanf return (number of matches) */

        sret = unsafe { sscanf(p, c"%u%c".as_ptr(), &mut a, &mut nextc) };
        if unsafe { scan_was_ok(sret, nextc, c",-".as_ptr()) } == 0 {
            unsafe { bitmask_clearall(bmp) };
            return -1;
        }
        b = a;
        s = 1;
        c1 = unsafe { nexttoken(p, b'-' as c_int) };
        c2 = unsafe { nexttoken(p, b',' as c_int) };
        if !c1.is_null() && (c2.is_null() || c1 < c2) {
            sret = unsafe { sscanf(c1, c"%u%c".as_ptr(), &mut b, &mut nextc) };
            if unsafe { scan_was_ok(sret, nextc, c",:".as_ptr()) } == 0 {
                unsafe { bitmask_clearall(bmp) };
                return -1;
            }
            c1 = unsafe { nexttoken(c1, b':' as c_int) };
            if !c1.is_null() && (c2.is_null() || c1 < c2) {
                sret = unsafe { sscanf(c1, c"%u%c".as_ptr(), &mut s, &mut nextc) };
                if unsafe { scan_was_ok(sret, nextc, c",".as_ptr()) } == 0 {
                    unsafe { bitmask_clearall(bmp) };
                    return -1;
                }
            }
        }
        if !(a <= b) {
            unsafe { bitmask_clearall(bmp) };
            return -1;
        }
        if b >= unsafe { (*bmp).size } {
            unsafe { bitmask_clearall(bmp) };
            return -1;
        }
        while a <= b {
            unsafe { _setbit(bmp, a, 1) };
            a = a.wrapping_add(s);
        }
    }
    0
}

/*
 * emit(buf, buflen, rbot, rtop, len)
 *
 * Helper routine for bitmask_displaylist().  Write decimal number
 * or range to buf+len, suppressing output past buf+buflen, with optional
 * comma-prefix.  Return len of what would be written to buf, if it
 * all fit.
 */

unsafe fn emit(buf: *mut c_char, buflen: c_int, rbot: c_int, rtop: c_int, mut len: c_int) -> c_int {
    if len > 0 {
        len += unsafe {
            snprintf(
                buf.add(len as usize),
                max(buflen - len, 0) as usize,
                c",".as_ptr(),
            )
        };
    }
    if rbot == rtop {
        len += unsafe {
            snprintf(
                buf.add(len as usize),
                max(buflen - len, 0) as usize,
                c"%d".as_ptr(),
                rbot,
            )
        };
    } else {
        len += unsafe {
            snprintf(
                buf.add(len as usize),
                max(buflen - len, 0) as usize,
                c"%d-%d".as_ptr(),
                rbot,
                rtop,
            )
        };
    }
    len
}

/*
 * Write decimal list representation of bmp to buf.
 *
 * Output format is a comma-separated list of decimal numbers and
 * ranges.  Consecutively set bits are shown as two hyphen-separated
 * decimal numbers, the smallest and largest bit numbers set in
 * the range.  Output format is compatible with the format
 * accepted as input by bitmap_parselist().
 *
 * The return value is the number of characters which would be
 * generated for the given input, excluding the trailing '\0', as
 * per ISO C99.
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bitmask_displaylist(
    buf: *mut c_char,
    buflen: c_int,
    bmp: *const bitmask,
) -> c_int {
    let mut len: c_int = 0;
    /* current bit is 'cur', most recently seen range is [rbot, rtop] */
    let mut cur: c_uint;
    let mut rbot: c_uint;
    let mut rtop: c_uint;

    if buflen > 0 {
        unsafe { *buf = 0 };
    }
    cur = unsafe { bitmask_first(bmp) };
    rbot = cur;
    while cur < unsafe { (*bmp).size } {
        rtop = cur;
        cur = unsafe { bitmask_next(bmp, cur.wrapping_add(1)) };
        if cur >= unsafe { (*bmp).size } || cur > rtop.wrapping_add(1) {
            len = unsafe { emit(buf, buflen, rbot as c_int, rtop as c_int, len) };
            rbot = cur;
        }
    }
    len
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
