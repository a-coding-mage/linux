// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 SGI.
 * All rights reserved.
 */

//! UTF-8 normalization, translated from the retained `utf8-norm.c`.
//! Configured C headers own all shared layouts, including cursor storage.

// C names and generated declarations are confined to the binding module.
#[allow(
    non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, improper_ctypes, unreachable_pub, clippy::all
)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/utf8_norm_generated.rs"));
}
#[cfg(CONFIG_UNICODE_NORMALIZATION_KUNIT_TEST = "m")]
#[path = "../../rust/ffi_export.rs"]
mod ffi_export;

use bindings::{unicode_map, utf8_normalization, utf8cursor, UTF8HANGULLEAF};
use core::ptr::{addr_of_mut, null};
use kernel::ffi::{c_char, c_int, c_short, c_uint};

const BITNUM: u8 = 0x07;
const NEXTBYTE: u8 = 0x08;
const OFFLEN: u8 = 0x30;
const OFFLEN_SHIFT: u8 = 4;
const RIGHTPATH: u8 = 0x40;
const TRIENODE: u8 = 0x80;
const RIGHTNODE: u8 = 0x40;
const LEFTNODE: u8 = 0x80;

const MINCCC: c_int = 0;
const MAXCCC: c_int = 254;
const STOPPER: c_int = 0;
const DECOMPOSE: u8 = 255;
const HANGUL: u8 = 255;

const SB: c_uint = 0xAC00;
const LB: c_uint = 0x1100;
const VB: c_uint = 0x1161;
const TB: c_uint = 0x11A7;
const VC: c_uint = 21;
const TC: c_uint = 28;
const NC: c_uint = VC * TC;

#[inline]
unsafe fn utf8clen(s: *const c_char) -> c_int {
    // SAFETY: The caller supplies the first byte of a valid UTF-8 sequence.
    let c = unsafe { *s };
    1 + c_int::from(c >= 0xC0) + c_int::from(c >= 0xE0) + c_int::from(c >= 0xF0)
}

unsafe fn utf8decode3(mut str_: *const c_char) -> c_uint {
    // SAFETY: The caller supplies a valid three-byte UTF-8 sequence.
    unsafe {
        let mut uc = c_uint::from(*str_ & 0x0F);
        str_ = str_.add(1);
        uc = (uc << 6) | c_uint::from(*str_ & 0x3F);
        str_ = str_.add(1);
        (uc << 6) | c_uint::from(*str_ & 0x3F)
    }
}

unsafe fn utf8encode3(str_: *mut c_char, mut val: c_uint) -> c_int {
    // SAFETY: The caller reserves three writable bytes for this Hangul part.
    unsafe {
        *str_.add(2) = ((val & 0x3F) | 0x80) as c_char;
        val >>= 6;
        *str_.add(1) = ((val & 0x3F) | 0x80) as c_char;
        val >>= 6;
        *str_ = (val | 0xE0) as c_char;
    }
    3
}

unsafe fn utf8hangul(str_: *const c_char, hangul: *mut u8) -> *const u8 {
    // SAFETY: Trie dispatch supplies a Hangul syllable and a writable
    // UTF8HANGULLEAF-byte buffer. L, V, optional T and NUL fit in that buffer.
    unsafe {
        let si = utf8decode3(str_).wrapping_sub(SB);
        let li = si / NC;
        let vi = (si % NC) / TC;
        let ti = si % TC;
        *hangul = 2;
        *hangul.add(1) = DECOMPOSE;
        let mut h = hangul.add(2);
        h = h.add(utf8encode3(h.cast(), li.wrapping_add(LB)) as usize);
        h = h.add(utf8encode3(h.cast(), vi.wrapping_add(VB)) as usize);
        if ti != 0 {
            h = h.add(utf8encode3(h.cast(), ti.wrapping_add(TB)) as usize);
        }
        *h = 0;
        hangul.cast_const()
    }
}

/// Report whether the selected Unicode data supports `version`.
///
/// # Safety
/// `um` and its data tables must remain valid for the call.
#[no_mangle]
pub unsafe extern "C" fn utf8version_is_supported(
    um: *const unicode_map, version: c_uint,
) -> c_int {
    // SAFETY: The caller provides the same map/table contract as the C API.
    unsafe {
        let tables = (*um).tables;
        // C uses int here, with the kernel's signed wrapping semantics.
        let mut i = (*tables).utf8agetab_size.wrapping_sub(1);
        while i >= 0 && *(*tables).utf8agetab.add(i as usize) != 0 {
            if version == *(*tables).utf8agetab.add(i as usize) {
                return 1;
            }
            i -= 1;
        }
        0
    }
}

unsafe fn utf8nlookup(
    um: *const unicode_map, n: utf8_normalization, hangul: *mut u8,
    mut s: *const c_char, mut len: usize,
) -> *const u8 {
    if len == 0 {
        return null();
    }
    // SAFETY: The map, normalization index and trie are valid; the caller
    // provides len readable input bytes (or a NUL-terminated decomposition).
    // NEXTBYTE checks the bound before moving to and reading the next byte.
    unsafe {
        let mut trie = (*(*um).tables).utf8data.add((*(*um).ntab[n as usize]).offset as usize);
        let mut node: c_int = 1;
        while node != 0 {
            let mut offlen = c_int::from((*trie & OFFLEN) >> OFFLEN_SHIFT);
            if *trie & NEXTBYTE != 0 {
                len -= 1;
                if len == 0 {
                    return null();
                }
                s = s.add(1);
            }
            let mask: c_int = 1 << (*trie & BITNUM);
            if c_int::from(*s) & mask != 0 {
                if offlen != 0 {
                    node = c_int::from(*trie & RIGHTNODE);
                    let mut offset = c_int::from(*trie.add(offlen as usize));
                    offlen -= 1;
                    while offlen != 0 {
                        offset = (offset << 8) | c_int::from(*trie.add(offlen as usize));
                        offlen -= 1;
                    }
                    trie = trie.add(offset as usize);
                } else if *trie & RIGHTPATH != 0 {
                    node = c_int::from(*trie & TRIENODE);
                    trie = trie.add(1);
                } else {
                    return null();
                }
            } else if offlen != 0 {
                node = c_int::from(*trie & LEFTNODE);
                trie = trie.add((offlen + 1) as usize);
            } else if *trie & RIGHTPATH != 0 {
                return null();
            } else {
                node = c_int::from(*trie & TRIENODE);
                trie = trie.add(1);
            }
        }
        if *trie.add(1) == DECOMPOSE && *trie.add(2) == HANGUL {
            return utf8hangul(s.sub(2), hangul);
        }
        trie
    }
}

unsafe fn utf8lookup(
    um: *const unicode_map, n: utf8_normalization, hangul: *mut u8, s: *const c_char,
) -> *const u8 {
    // SAFETY: The caller supplies a valid NUL-terminated decomposition; C
    // deliberately uses (size_t)-1 for this unbounded lookup.
    unsafe { utf8nlookup(um, n, hangul, s, usize::MAX) }
}

/// Count normalized bytes, or return -1 for invalid UTF-8.
///
/// # Safety
/// `um` and normalization `n` must be valid. `s` must be readable through the
/// first NUL or `len` bytes, whichever comes first.
#[no_mangle]
pub unsafe extern "C" fn utf8nlen(
    um: *const unicode_map, n: utf8_normalization, mut s: *const c_char, mut len: usize,
) -> isize {
    // SAFETY: The caller supplies the C API's map and bounded-string contract.
    // Each successful lookup validates a whole input sequence before advancing.
    unsafe {
        let mut ret = 0usize;
        let mut hangul = [0u8; UTF8HANGULLEAF as usize];
        while len != 0 && *s != 0 {
            let leaf = utf8nlookup(um, n, hangul.as_mut_ptr(), s, len);
            if leaf.is_null() {
                return -1;
            }
            // ret is C size_t: retain its unsigned wrapping additions.
            if *(*(*um).tables).utf8agetab.add(usize::from(*leaf))
                > (*(*um).ntab[n as usize]).maxage {
                ret = ret.wrapping_add(utf8clen(s) as usize);
            } else if *leaf.add(1) == DECOMPOSE {
                let mut p = leaf.add(2);
                while *p != 0 {
                    ret = ret.wrapping_add(1);
                    p = p.add(1);
                }
            } else {
                ret = ret.wrapping_add(utf8clen(s) as usize);
            }
            let l = utf8clen(s) as usize;
            len = len.wrapping_sub(l);
            s = s.add(l);
        }
        ret as isize
    }
}

/// Initialize a cursor, rejecting a null input, oversized length or continuation byte.
///
/// # Safety
/// `u8c` must point to aligned writable cursor storage, which may be uninitialized.
/// The map, normalization and input must satisfy `utf8nlen`'s contract and remain
/// valid while the cursor is used. The cursor must not move during iteration.
#[no_mangle]
pub unsafe extern "C" fn utf8ncursor(
    u8c: *mut utf8cursor, um: *const unicode_map, n: utf8_normalization,
    s: *const c_char, len: usize,
) -> c_int {
    if s.is_null() {
        return -1;
    }
    // SAFETY: Only raw field writes are used: no reference/value covering an
    // uninitialized cursor is formed. C leaves the Hangul buffer untouched.
    unsafe {
        addr_of_mut!((*u8c).um).write(um);
        addr_of_mut!((*u8c).n).write(n);
        addr_of_mut!((*u8c).s).write(s);
        addr_of_mut!((*u8c).p).write(null());
        addr_of_mut!((*u8c).ss).write(null());
        addr_of_mut!((*u8c).sp).write(null());
        addr_of_mut!((*u8c).len).write(len as c_uint);
        addr_of_mut!((*u8c).slen).write(0);
        addr_of_mut!((*u8c).ccc).write(STOPPER as c_short);
        addr_of_mut!((*u8c).nccc).write(STOPPER as c_short);
        // Match C's unsigned-int store followed by a size_t comparison.
        if (*u8c).len as usize != len {
            return -1;
        }
        if len > 0 && (*s & 0xC0) == 0x80 {
            return -1;
        }
        0
    }
}

/// Emit the next unsigned normalized byte, zero at end, or -1 on invalid UTF-8.
///
/// # Safety
/// `u8c` must be a successfully initialized cursor at its original address, with
/// its input and map still valid and exclusive access throughout this call.
#[no_mangle]
pub unsafe extern "C" fn utf8byte(u8c: *mut utf8cursor) -> c_int {
    // SAFETY: The initialized cursor owns the live Hangul decomposition buffer.
    // Its pointer is derived without a reference to or read of untouched bytes.
    // All remaining cursor fields were initialized individually by utf8ncursor.
    unsafe {
        let hangul = addr_of_mut!((*u8c).hangul).cast::<u8>();
        'next: loop {
            if !(*u8c).p.is_null() && *(*u8c).s == 0 {
                (*u8c).s = (*u8c).p;
                (*u8c).p = null();
            }
            // Breaking this block is C's goto ccc_mismatch. In particular, an
            // empty decomposition during a scan must not restore DECOMPOSE.
            let ccc = 'ccc_mismatch: {
                if (*u8c).p.is_null() && ((*u8c).len == 0 || *(*u8c).s == 0) {
                    if c_int::from((*u8c).ccc) == STOPPER {
                        return 0;
                    }
                    break 'ccc_mismatch STOPPER;
                } else if (*(*u8c).s & 0xC0) == 0x80 {
                    if (*u8c).p.is_null() {
                        (*u8c).len = (*u8c).len.wrapping_sub(1);
                    }
                    let v = *(*u8c).s;
                    (*u8c).s = (*u8c).s.add(1);
                    return c_int::from(v);
                }
                let mut leaf = if !(*u8c).p.is_null() {
                    utf8lookup((*u8c).um, (*u8c).n, hangul, (*u8c).s)
                } else {
                    utf8nlookup((*u8c).um, (*u8c).n, hangul, (*u8c).s, (*u8c).len as usize)
                };
                if leaf.is_null() {
                    return -1;
                }
                let mut ccc = c_int::from(*leaf.add(1));
                if *(*(*(*u8c).um).tables).utf8agetab.add(usize::from(*leaf))
                    > (*(*(*u8c).um).ntab[(*u8c).n as usize]).maxage {
                    ccc = STOPPER;
                } else if ccc == c_int::from(DECOMPOSE) {
                    let l = utf8clen((*u8c).s);
                    // C len is unsigned int, including each subtraction.
                    (*u8c).len = (*u8c).len.wrapping_sub(l as c_uint);
                    (*u8c).p = (*u8c).s.add(l as usize);
                    (*u8c).s = leaf.add(2).cast();
                    if *(*u8c).s == 0 {
                        if c_int::from((*u8c).ccc) == STOPPER {
                            continue 'next;
                        }
                        break 'ccc_mismatch STOPPER;
                    }
                    leaf = utf8lookup((*u8c).um, (*u8c).n, hangul, (*u8c).s);
                    if leaf.is_null() {
                        return -1;
                    }
                    ccc = c_int::from(*leaf.add(1));
                }
                if ccc != STOPPER && c_int::from((*u8c).ccc) < ccc
                    && ccc < c_int::from((*u8c).nccc) {
                    (*u8c).nccc = ccc as c_short;
                }
                if ccc == c_int::from((*u8c).ccc) {
                    if (*u8c).p.is_null() {
                        (*u8c).len = (*u8c).len.wrapping_sub(1);
                    }
                    let v = *(*u8c).s;
                    (*u8c).s = (*u8c).s.add(1);
                    return c_int::from(v);
                }
                ccc
            };
            if c_int::from((*u8c).nccc) == STOPPER {
                (*u8c).ccc = (MINCCC - 1) as c_short;
                (*u8c).nccc = ccc as c_short;
                (*u8c).sp = (*u8c).p;
                (*u8c).ss = (*u8c).s;
                (*u8c).slen = (*u8c).len;
                let l = utf8clen((*u8c).s);
                if (*u8c).p.is_null() {
                    (*u8c).len = (*u8c).len.wrapping_sub(l as c_uint);
                }
                (*u8c).s = (*u8c).s.add(l as usize);
            } else if ccc != STOPPER {
                let l = utf8clen((*u8c).s);
                if (*u8c).p.is_null() {
                    (*u8c).len = (*u8c).len.wrapping_sub(l as c_uint);
                }
                (*u8c).s = (*u8c).s.add(l as usize);
            } else if c_int::from((*u8c).nccc) != MAXCCC + 1 {
                (*u8c).ccc = (*u8c).nccc;
                (*u8c).nccc = (MAXCCC + 1) as c_short;
                (*u8c).s = (*u8c).ss;
                (*u8c).p = (*u8c).sp;
                (*u8c).len = (*u8c).slen;
            } else {
                (*u8c).ccc = STOPPER as c_short;
                (*u8c).nccc = STOPPER as c_short;
                (*u8c).sp = null();
                (*u8c).ss = null();
                (*u8c).slen = 0;
            }
        }
    }
}

// Exactly the C IS_MODULE condition and original GPL-only export class.
#[cfg(CONFIG_UNICODE_NORMALIZATION_KUNIT_TEST = "m")]
ffi_export::export_symbol!(utf8version_is_supported, utf8version_is_supported, "GPL", "");
#[cfg(CONFIG_UNICODE_NORMALIZATION_KUNIT_TEST = "m")]
ffi_export::export_symbol!(utf8nlen, utf8nlen, "GPL", "");
#[cfg(CONFIG_UNICODE_NORMALIZATION_KUNIT_TEST = "m")]
ffi_export::export_symbol!(utf8ncursor, utf8ncursor, "GPL", "");
#[cfg(CONFIG_UNICODE_NORMALIZATION_KUNIT_TEST = "m")]
ffi_export::export_symbol!(utf8byte, utf8byte, "GPL", "");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
