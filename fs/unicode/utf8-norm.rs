// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 SGI.
 * All rights reserved.
 */

const BITNUM: u8 = 0x07;
const NEXTBYTE: u8 = 0x08;
const OFFLEN: u8 = 0x30;
const OFFLEN_SHIFT: u8 = 4;
const RIGHTPATH: u8 = 0x40;
const TRIENODE: u8 = 0x80;
const RIGHTNODE: u8 = 0x40;
const LEFTNODE: u8 = 0x80;

const MINCCC: i32 = 0;
const MAXCCC: i32 = 254;
const STOPPER: i32 = 0;
const DECOMPOSE: u8 = 255;
const HANGUL: u8 = 255;
const UTF8HANGULLEAF: usize = 12;

const SB: u32 = 0xAC00;
const LB: u32 = 0x1100;
const VB: u32 = 0x1161;
const TB: u32 = 0x11A7;
const LC: u32 = 19;
const VC: u32 = 21;
const TC: u32 = 28;
const NC: u32 = VC * TC;
const SC: u32 = LC * NC;

#[inline]
unsafe fn utf8clen(s: *const core::ffi::c_char) -> usize {
    let c = *s as u8;
    1 + (c >= 0xC0) as usize + (c >= 0xE0) as usize + (c >= 0xF0) as usize
}

unsafe fn utf8decode3(mut str_: *const core::ffi::c_char) -> u32 {
    let mut uc = (*str_ as u8 & 0x0F) as u32;
    str_ = str_.add(1); uc = (uc << 6) | ((*str_ as u8 & 0x3F) as u32);
    str_ = str_.add(1); (uc << 6) | ((*str_ as u8 & 0x3F) as u32)
}

unsafe fn utf8encode3(str_: *mut core::ffi::c_char, mut val: u32) -> usize {
    *str_.add(2) = ((val & 0x3F) | 0x80) as i8 as core::ffi::c_char;
    val >>= 6; *str_.add(1) = ((val & 0x3F) | 0x80) as i8 as core::ffi::c_char;
    val >>= 6; *str_ = (val | 0xE0) as i8 as core::ffi::c_char; 3
}

unsafe fn utf8hangul(str_: *const core::ffi::c_char, hangul: *mut u8) -> *mut u8 {
    let si = utf8decode3(str_) - SB; let li = si / NC; let vi = (si % NC) / TC; let ti = si % TC;
    *hangul = 2; *hangul.add(1) = DECOMPOSE; let mut h = hangul.add(2);
    h = h.add(utf8encode3(h as *mut core::ffi::c_char, li + LB));
    h = h.add(utf8encode3(h as *mut core::ffi::c_char, vi + VB));
    if ti != 0 { h = h.add(utf8encode3(h as *mut core::ffi::c_char, ti + TB)); }
    *h = 0; hangul
}

// The structures `unicode_map`, `utf8_normalization`, and `utf8cursor` are
// supplied by the corresponding translated header/dependencies.

pub unsafe fn utf8version_is_supported(um: *const unicode_map, version: u32) -> i32 {
    let mut i = (*(*um).tables).utf8agetab_size - 1;
    while i >= 0 && *(*um).tables.utf8agetab.add(i as usize) != 0 {
        if version == *(*um).tables.utf8agetab.add(i as usize) { return 1; }
        i -= 1;
    } 0
}

unsafe fn utf8nlookup(um: *const unicode_map, n: utf8_normalization, hangul: *mut u8,
    mut s: *const core::ffi::c_char, mut len: usize) -> *mut u8 {
    let mut trie = (*(*um).tables).utf8data.add((*(*um).ntab.add(n as usize)).offset as usize);
    let mut node: i32 = 1;
    while node != 0 {
        let offlen = ((*trie & OFFLEN) >> OFFLEN_SHIFT) as usize;
        if *trie & NEXTBYTE != 0 { len -= 1; if len == 0 { return core::ptr::null_mut(); } s = s.add(1); }
        let mask = 1u8 << (*trie & BITNUM);
        if (*(s as *const u8)) & mask != 0 {
            if offlen != 0 { node = (*trie & RIGHTNODE) as i32; let mut offset = *trie.add(offlen) as usize; let mut j = offlen; while j > 1 { j -= 1; offset = (offset << 8) | *trie.add(j) as usize; } trie = trie.add(offset); }
            else if *trie & RIGHTPATH != 0 { node = (*trie & TRIENODE) as i32; trie = trie.add(1); } else { return core::ptr::null_mut(); }
        } else if offlen != 0 { node = (*trie & LEFTNODE) as i32; trie = trie.add(offlen + 1); }
        else if *trie & RIGHTPATH != 0 { return core::ptr::null_mut(); }
        else { node = (*trie & TRIENODE) as i32; trie = trie.add(1); }
    }
    if *trie.add(1) == DECOMPOSE && *trie.add(2) == HANGUL { return utf8hangul(s.sub(2), hangul); }
    trie as *mut u8
}

unsafe fn utf8lookup(um: *const unicode_map, n: utf8_normalization, hangul: *mut u8, s: *const core::ffi::c_char) -> *mut u8 {
    utf8nlookup(um, n, hangul, s, usize::MAX)
}

pub unsafe fn utf8nlen(um: *const unicode_map, n: utf8_normalization, mut s: *const core::ffi::c_char, mut len: usize) -> isize {
    let mut ret = 0usize; let mut hangul = [0u8; UTF8HANGULLEAF];
    while len != 0 && *s != 0 {
        let leaf = utf8nlookup(um, n, hangul.as_mut_ptr(), s, len); if leaf.is_null() { return -1; }
        if *(*um).tables.utf8agetab.add(*leaf as usize) > (*(*um).ntab.add(n as usize)).maxage { ret += utf8clen(s); }
        else if *leaf.add(1) == DECOMPOSE { let mut p = leaf.add(2); while *p != 0 { ret += 1; p = p.add(1); } } else { ret += utf8clen(s); }
        let l = utf8clen(s); len -= l; s = s.add(l);
    } ret as isize
}

pub unsafe fn utf8ncursor(u8c: *mut utf8cursor, um: *const unicode_map, n: utf8_normalization,
    s: *const core::ffi::c_char, len: usize) -> i32 {
    if s.is_null() { return -1; }
    (*u8c).um = um; (*u8c).n = n; (*u8c).s = s; (*u8c).p = core::ptr::null();
    (*u8c).ss = core::ptr::null(); (*u8c).sp = core::ptr::null(); (*u8c).len = len;
    (*u8c).slen = 0; (*u8c).ccc = STOPPER; (*u8c).nccc = STOPPER;
    if (*u8c).len != len { return -1; }
    if len > 0 && (*s as u8 & 0xC0) == 0x80 { return -1; }
    0
}

pub unsafe fn utf8byte(u8c: *mut utf8cursor) -> i32 {
    let mut hangul = [0u8; UTF8HANGULLEAF];
    loop {
        if !(*u8c).p.is_null() && *(*u8c).s == 0 { (*u8c).s = (*u8c).p; (*u8c).p = core::ptr::null(); }
        let ccc: i32;
        if (*u8c).p.is_null() && ((*u8c).len == 0 || *(*u8c).s == 0) {
            if (*u8c).ccc == STOPPER { return 0; }
            ccc = STOPPER;
        } else if *(*u8c).s as u8 & 0xC0 == 0x80 {
            if (*u8c).p.is_null() { (*u8c).len -= 1; }
            let v = *(*u8c).s as u8; (*u8c).s = (*u8c).s.add(1); return v as i32;
        } else {
            let mut leaf = if !(*u8c).p.is_null() { utf8lookup((*u8c).um, (*u8c).n, hangul.as_mut_ptr(), (*u8c).s) }
                else { utf8nlookup((*u8c).um, (*u8c).n, hangul.as_mut_ptr(), (*u8c).s, (*u8c).len) };
            if leaf.is_null() { return -1; }
            let mut class = *leaf.add(1) as i32;
            if *(*(*u8c).um).tables.utf8agetab.add(*leaf as usize) > (*(*(*u8c).um).ntab.add((*u8c).n as usize)).maxage { class = STOPPER; }
            else if class as u8 == DECOMPOSE {
                let l = utf8clen((*u8c).s); (*u8c).len -= l; (*u8c).p = (*u8c).s.add(l); (*u8c).s = leaf.add(2) as *const core::ffi::c_char;
                if *(*u8c).s == 0 { if (*u8c).ccc == STOPPER { continue; } ccc = STOPPER; } else { leaf = utf8lookup((*u8c).um, (*u8c).n, hangul.as_mut_ptr(), (*u8c).s); if leaf.is_null() { return -1; } class = *leaf.add(1) as i32; }
            }
            ccc = class;
        }
        if ccc != STOPPER && (*u8c).ccc < ccc && ccc < (*u8c).nccc { (*u8c).nccc = ccc; }
        if ccc == (*u8c).ccc { if (*u8c).p.is_null() { (*u8c).len -= 1; } let v = *(*u8c).s as u8; (*u8c).s = (*u8c).s.add(1); return v as i32; }
        if (*u8c).nccc == STOPPER {
            (*u8c).ccc = MINCCC - 1; (*u8c).nccc = ccc; (*u8c).sp = (*u8c).p; (*u8c).ss = (*u8c).s; (*u8c).slen = (*u8c).len;
            if (*u8c).p.is_null() { (*u8c).len -= utf8clen((*u8c).s); } (*u8c).s = (*u8c).s.add(utf8clen((*u8c).s));
        } else if ccc != STOPPER { if (*u8c).p.is_null() { (*u8c).len -= utf8clen((*u8c).s); } (*u8c).s = (*u8c).s.add(utf8clen((*u8c).s));
        } else if (*u8c).nccc != MAXCCC + 1 { (*u8c).ccc = (*u8c).nccc; (*u8c).nccc = MAXCCC + 1; (*u8c).s = (*u8c).ss; (*u8c).p = (*u8c).sp; (*u8c).len = (*u8c).slen;
        } else { (*u8c).ccc = STOPPER; (*u8c).nccc = STOPPER; (*u8c).sp = core::ptr::null(); (*u8c).ss = core::ptr::null(); (*u8c).slen = 0; }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
