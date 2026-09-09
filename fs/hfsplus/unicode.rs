// SPDX-License-Identifier: GPL-2.0
/* Handler routines for unicode strings. */

/* Kernel types, tables, structures, constants, and helpers are supplied by
 * the surrounding HFS+ implementation. */

#[inline]
unsafe fn case_fold(mut c: u16) -> u16 {
    let mut tmp = hfsplus_case_fold_table[(c >> 8) as usize];
    if tmp != 0 {
        tmp = hfsplus_case_fold_table[(tmp as usize) + (c & 0xff) as usize];
    } else {
        tmp = c;
    }
    tmp
}

pub unsafe fn hfsplus_strcasecmp(s1: *const hfsplus_unistr, s2: *const hfsplus_unistr) -> i32 {
    let mut len1 = be16_to_cpu((*s1).length);
    let mut len2 = be16_to_cpu((*s2).length);
    let mut p1 = (*s1).unicode;
    let mut p2 = (*s2).unicode;
    if len1 > HFSPLUS_MAX_STRLEN as u16 { len1 = HFSPLUS_MAX_STRLEN as u16; }
    if len2 > HFSPLUS_MAX_STRLEN as u16 { len2 = HFSPLUS_MAX_STRLEN as u16; }
    loop {
        let (mut c1, mut c2) = (0u16, 0u16);
        while len1 != 0 && c1 == 0 { c1 = case_fold(be16_to_cpu(*p1)); p1 = p1.add(1); len1 -= 1; }
        while len2 != 0 && c2 == 0 { c2 = case_fold(be16_to_cpu(*p2)); p2 = p2.add(1); len2 -= 1; }
        if c1 != c2 { return if c1 < c2 { -1 } else { 1 }; }
        if c1 == 0 { return 0; }
    }
}

pub unsafe fn hfsplus_strcmp(s1: *const hfsplus_unistr, s2: *const hfsplus_unistr) -> i32 {
    let mut len1 = be16_to_cpu((*s1).length);
    let mut len2 = be16_to_cpu((*s2).length);
    let mut p1 = (*s1).unicode;
    let mut p2 = (*s2).unicode;
    if len1 > HFSPLUS_MAX_STRLEN as u16 { len1 = HFSPLUS_MAX_STRLEN as u16; }
    if len2 > HFSPLUS_MAX_STRLEN as u16 { len2 = HFSPLUS_MAX_STRLEN as u16; }
    let mut len = core::cmp::min(len1, len2);
    while len > 0 {
        let c1 = be16_to_cpu(*p1); let c2 = be16_to_cpu(*p2);
        if c1 != c2 { return if c1 < c2 { -1 } else { 1 }; }
        p1 = p1.add(1); p2 = p2.add(1); len -= 1;
    }
    if len1 < len2 { -1 } else if len1 > len2 { 1 } else { 0 }
}

const HANGUL_SBASE: i32 = 0xac00;
const HANGUL_LBASE: i32 = 0x1100;
const HANGUL_VBASE: i32 = 0x1161;
const HANGUL_TBASE: i32 = 0x11a7;
const HANGUL_SCOUNT: i32 = 11172;
const HANGUL_VCOUNT: i32 = 21;
const HANGUL_TCOUNT: i32 = 28;
const HANGUL_NCOUNT: i32 = HANGUL_VCOUNT * HANGUL_TCOUNT;

unsafe fn hfsplus_compose_lookup(p: *mut u16, cc: u16) -> *mut u16 {
    let (mut s, mut e) = (1i32, *p.add(1) as i32);
    if e == 0 || cc < *p.add((s * 2) as usize) || cc > *p.add((e * 2) as usize) { return core::ptr::null_mut(); }
    while s <= e { let i = (s + e) / 2; let v = *p.add((i * 2) as usize); if cc > v { s = i + 1; } else if cc < v { e = i - 1; } else { return hfsplus_compose_table.add(*p.add((i * 2 + 1) as usize) as usize); } }
    core::ptr::null_mut()
}

#[inline] unsafe fn hfsplus_mac2linux_compatibility_check(symbol: u16, conversion: *mut u16, name_type: i32) {
    *conversion = symbol; if name_type == HFS_XATTR_NAME { return; }
    if symbol == 0 { *conversion = 0x2400; } else if symbol == b'/' as u16 { *conversion = b':' as u16; }
}

unsafe fn hfsplus_uni2asc(sb: *mut super_block, ustr: *const hfsplus_unistr, max_len: i32, astr: *mut i8, len_p: *mut i32, name_type: i32) -> i32 {
    let nls = HFSPLUS_SB(sb).nls; let mut ip = (*ustr).unicode; let mut ustrlen = be16_to_cpu((*ustr).length) as i32;
    if ustrlen > max_len { ustrlen = max_len; }
    let mut op = astr as *mut u8; let mut len = *len_p; let compose = !test_bit(HFSPLUS_SB_NODECOMPOSE, &HFSPLUS_SB(sb).flags);
    while ustrlen > 0 { let c0 = be16_to_cpu(*ip); ip = ip.add(1); ustrlen -= 1; let mut cc = 0u16; let ce1 = if compose { hfsplus_compose_lookup(hfsplus_compose_table, c0) } else { core::ptr::null_mut() }; if !ce1.is_null() { cc = *ce1; }
        if cc == 0xffff && ustrlen > 0 { let c1 = be16_to_cpu(*ip) as i32 - 0x1161; if c1 >= 0 && c1 < HANGUL_VCOUNT { cc = (((c0 as i32 - HANGUL_LBASE) * HANGUL_VCOUNT + c1) * HANGUL_TCOUNT + HANGUL_SBASE) as u16; ip = ip.add(1); ustrlen -= 1; if ustrlen > 0 { let t = be16_to_cpu(*ip) as i32 - HANGUL_TBASE; if t > 0 && t < HANGUL_TCOUNT { cc = cc.wrapping_add(t as u16); ip = ip.add(1); ustrlen -= 1; } } } }
        if cc == 0 { cc = c0; } hfsplus_mac2linux_compatibility_check(cc, &mut cc, name_type); let r = (*nls).uni2char(cc, op, len); if r < 0 { if r == -ENAMETOOLONG { *len_p = op.offset_from(astr as *mut u8) as i32; return r; } *op = b'?'; } let n = if r < 0 { 1 } else { r }; op = op.add(n as usize); len -= n;
    }
    *len_p = op.offset_from(astr as *mut u8) as i32; 0
}

pub unsafe fn hfsplus_uni2asc_str(sb: *mut super_block, u: *const hfsplus_unistr, a: *mut i8, l: *mut i32) -> i32 { hfsplus_uni2asc(sb, u, HFSPLUS_MAX_STRLEN, a, l, HFS_REGULAR_NAME) }
pub unsafe fn hfsplus_uni2asc_xattr_str(sb: *mut super_block, u: *const hfsplus_attr_unistr, a: *mut i8, l: *mut i32) -> i32 { hfsplus_uni2asc(sb, u as *const hfsplus_unistr, HFSPLUS_ATTR_MAX_STRLEN, a, l, HFS_XATTR_NAME) }

#[inline] unsafe fn hfsplus_linux2mac_compatibility_check(uc: *mut wchar_t, name_type: i32) { if name_type == HFS_XATTR_NAME { return; } if *uc == 0x2400 { *uc = 0; } else if *uc == b':' as wchar_t { *uc = b'/' as wchar_t; } }
#[inline] unsafe fn asc2unichar(sb: *mut super_block, astr: *const i8, len: i32, uc: *mut wchar_t, name_type: i32) -> i32 { let mut size = HFSPLUS_SB(sb).nls.char2uni(astr, len, uc); if size <= 0 { *uc = b'?' as wchar_t; size = 1; } hfsplus_linux2mac_compatibility_check(uc, name_type); size }

unsafe fn hfsplus_decompose_nonhangul(uc: wchar_t, size: *mut i32) -> *mut u16 { let mut off = hfsplus_decompose_table[((uc >> 12) & 0xf) as usize]; if off == 0 || off == 0xffff { return core::ptr::null_mut(); } off = hfsplus_decompose_table[(off + ((uc >> 8) & 0xf)) as usize]; if off == 0 { return core::ptr::null_mut(); } off = hfsplus_decompose_table[(off + ((uc >> 4) & 0xf)) as usize]; if off == 0 { return core::ptr::null_mut(); } off = hfsplus_decompose_table[(off + (uc & 0xf)) as usize]; *size = (off & 3) as i32; if *size == 0 { return core::ptr::null_mut(); } hfsplus_decompose_table.add((off / 4) as usize) }

unsafe fn decompose_unichar(uc: wchar_t, size: *mut i32, buf: *mut u16) -> *mut u16 { let index = uc as i32 - HANGUL_SBASE; if index >= 0 && index < HANGUL_SCOUNT { let l = HANGUL_LBASE + index / HANGUL_NCOUNT; let v = HANGUL_VBASE + (index % HANGUL_NCOUNT) / HANGUL_TCOUNT; let t = HANGUL_TBASE + index % HANGUL_TCOUNT; *buf = l as u16; *buf.add(1) = v as u16; if t != HANGUL_TBASE { *buf.add(2) = t as u16; *size = 3; } else { *size = 2; } buf } else { hfsplus_decompose_nonhangul(uc, size) } }

pub unsafe fn hfsplus_asc2uni(sb: *mut super_block, ustr: *mut hfsplus_unistr, max: i32, mut astr: *const i8, mut len: i32, nt: i32) -> i32 { let dec = !test_bit(HFSPLUS_SB_NODECOMPOSE, &HFSPLUS_SB(sb).flags); let mut out = 0u16; let mut buf = [0u16; 3]; while (out as i32) < max && len > 0 { let mut c = 0 as wchar_t; let n = asc2unichar(sb, astr, len, &mut c, nt); let mut ds = 0; let d = if dec { decompose_unichar(c, &mut ds, buf.as_mut_ptr()) } else { core::ptr::null_mut() }; if !d.is_null() { if (out as i32) + ds > max { break; } for i in 0..ds { (*ustr).unicode.add(out as usize + i as usize).write(cpu_to_be16(*d.add(i as usize))); } out += ds as u16; } else { (*ustr).unicode.add(out as usize).write(cpu_to_be16(c as u16)); out += 1; } astr = astr.add(n as usize); len -= n; } (*ustr).length = cpu_to_be16(out); if len > 0 { -ENAMETOOLONG } else { 0 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
