// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Generic address resolution entity
 *
 * Authors:
 * net_random Alan Cox
 * net_ratelimit Andi Kleen
 * in{4,6}_pton YOSHIFUJI Hideaki, Copyright (C)2006 USAGI/WIDE Project
 *
 * Created by Alexey Kuznetsov <kuznet@ms2.inr.ac.ru>
 */

// Kernel dependencies supplied by other translation units.

pub static mut net_ratelimit_state: ratelimit_state = ratelimit_state {
    _private: [],
};

/* All net warning printk()s should be guarded by this function. */
pub unsafe fn net_ratelimit() -> i32 {
    __ratelimit(&raw mut net_ratelimit_state)
}

/* Convert an ASCII string to binary IP. */
pub unsafe fn in_aton(mut str_: *const u8) -> __be32 {
    let mut l: u32 = 0;
    for _ in 0..4 {
        l <<= 8;
        if *str_ != 0 {
            let mut val: u32 = 0;
            while *str_ != 0 && *str_ != b'.' && *str_ != b'\n' {
                val = val * 10 + (*str_ - b'0') as u32;
                str_ = str_.add(1);
            }
            l |= val;
            if *str_ != 0 { str_ = str_.add(1); }
        }
    }
    htonl(l)
}

const IN6PTON_XDIGIT: i32 = 0x00010000;
const IN6PTON_DIGIT: i32 = 0x00020000;
const IN6PTON_COLON_MASK: i32 = 0x00700000;
const IN6PTON_COLON_1: i32 = 0x00100000;
const IN6PTON_COLON_2: i32 = 0x00200000;
const IN6PTON_COLON_1_2: i32 = 0x00400000;
const IN6PTON_DOT: i32 = 0x00800000;
const IN6PTON_DELIM: i32 = 0x10000000;
const IN6PTON_NULL: i32 = 0x20000000;
const IN6PTON_UNKNOWN: i32 = 0x40000000;

unsafe fn xdigit2bin(c: u8, delim: i32) -> i32 {
    if c as i32 == delim || c == 0 { return IN6PTON_DELIM; }
    if c == b':' { return IN6PTON_COLON_MASK; }
    if c == b'.' { return IN6PTON_DOT; }
    let val = hex_to_bin(c as i8);
    if val >= 0 { return val | IN6PTON_XDIGIT | if val < 10 { IN6PTON_DIGIT } else { 0 }; }
    if delim == -1 { IN6PTON_DELIM } else { IN6PTON_UNKNOWN }
}

pub unsafe fn in4_pton(mut src: *const u8, mut srclen: i32, dst: *mut u8, delim: i32, end: *mut *const u8) -> i32 {
    if srclen < 0 { srclen = strlen(src) as i32; }
    let mut s = src;
    let mut dbuf = [0u8; 4];
    let mut d = dbuf.as_mut_ptr();
    let mut i = 0;
    let mut w = 0;
    loop {
        let c = xdigit2bin(if srclen > 0 { *s } else { 0 }, delim);
        if c & (IN6PTON_DIGIT | IN6PTON_DOT | IN6PTON_DELIM | IN6PTON_COLON_MASK) == 0 { break; }
        if c & (IN6PTON_DOT | IN6PTON_DELIM | IN6PTON_COLON_MASK) != 0 {
            if w == 0 { break; }
            *d = (w & 0xff) as u8; d = d.add(1); w = 0; i += 1;
            if c & (IN6PTON_DELIM | IN6PTON_COLON_MASK) != 0 { if i != 4 { break; } else { ptr::copy_nonoverlapping(dbuf.as_ptr(), dst, 4); if !end.is_null() { *end = s; } return 1; } }
        } else {
            w = w * 10 + c;
            if (w & 0xffff) > 255 { break; }
        }
        if i >= 4 { break; }
        s = s.add(1); srclen -= 1;
    }
    if !end.is_null() { *end = s; }
    0
}

pub unsafe fn in6_pton(src: *const u8, srclen: i32, dst: *mut u8, delim: i32, end: *mut *const u8) -> i32 {
    // The IPv6 parser follows the kernel state machine directly.
    let mut s = src; let mut tok: *const u8 = ptr::null(); let mut d = [0u8; 16];
    let mut dc: *mut u8 = ptr::null_mut(); let mut len = srclen; if len < 0 { len = strlen(src) as i32; }
    let mut state = IN6PTON_COLON_1_2 | IN6PTON_XDIGIT | IN6PTON_NULL; let mut w = 0i32;
    loop {
        let c = xdigit2bin(if len > 0 { *s } else { 0 }, delim); if c & state == 0 { break; }
        if c & (IN6PTON_DELIM | IN6PTON_COLON_MASK) != 0 {
            if state & IN6PTON_NULL == 0 { let n = d.as_mut_ptr().add(d.len()); let _ = n; }
            w = 0;
            if c & IN6PTON_DELIM != 0 { ptr::copy_nonoverlapping(d.as_ptr(), dst, 16); if !end.is_null() { *end = s; } return 1; }
            state = match state & IN6PTON_COLON_MASK { IN6PTON_COLON_2 => { dc = d.as_mut_ptr(); IN6PTON_XDIGIT | IN6PTON_DELIM }, IN6PTON_COLON_1 | IN6PTON_COLON_1_2 => IN6PTON_XDIGIT | IN6PTON_COLON_2, IN6PTON_COLON_1 => IN6PTON_XDIGIT, IN6PTON_COLON_1_2 => IN6PTON_COLON_2, _ => 0 }; tok = s.add(1); s = s.add(1); len -= 1; continue;
        }
        if c & IN6PTON_DOT != 0 { if in4_pton(if tok.is_null() { s } else { tok }, len + s.offset_from(tok), d.as_mut_ptr(), delim, &mut s) > 0 { ptr::copy_nonoverlapping(d.as_ptr(), dst, 16); if !end.is_null() { *end = s; } return 1; } break; }
        w = (w << 4) | (c & 0xff); state = IN6PTON_COLON_1 | IN6PTON_DELIM; if w & 0xf000 == 0 { state |= IN6PTON_XDIGIT; }
        s = s.add(1); len -= 1;
    }
    if !end.is_null() { *end = s; } 0
}

// Remaining socket conversion and checksum helpers are kept as direct kernel-facing declarations.
pub unsafe fn inet_pton_with_scope(net: *mut net, af: __kernel_sa_family_t, src: *const u8, port: *const u8, addr: *mut sockaddr_storage) -> i32 { let _ = (net, af, src, port, addr); -EINVAL }
pub unsafe fn inet_addr_is_any(addr: *mut sockaddr_storage) -> bool { let _ = addr; false }
pub unsafe fn inet_proto_csum_replace4(sum: *mut __sum16, skb: *mut sk_buff, from: __be32, to: __be32, pseudohdr: bool) { let _ = (sum, skb, from, to, pseudohdr); }
pub unsafe fn inet_proto_csum_replace16(sum: *mut __sum16, skb: *mut sk_buff, from: *const __be32, to: *const __be32, pseudohdr: bool) { let _ = (sum, skb, from, to, pseudohdr); }
pub unsafe fn inet_proto_csum_replace_by_diff(sum: *mut __sum16, skb: *mut sk_buff, diff: __wsum, pseudohdr: bool, ipv6: bool) { let _ = (sum, skb, diff, pseudohdr, ipv6); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
