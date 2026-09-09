// SPDX-License-Identifier: GPL-2.0

// Corresponds to <linux/errno.h>.
const EINVAL: i32 = 22;

pub unsafe fn ceph_armor(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, end: *const core::ffi::c_char) -> i32 {
    let mut olen: i32 = 0;
    let mut line: i32 = 0;
    let mut dst = dst;
    let mut src = src;

    while src < end {
        let a: u8;
        let b: u8;
        let c: u8;

        a = *(src as *const u8);
        src = src.add(1);
        *dst = encode_bits(a >> 2) as core::ffi::c_char;
        dst = dst.add(1);
        if src < end {
            b = *(src as *const u8);
            src = src.add(1);
            *dst = encode_bits(((a & 3) << 4 | (b >> 4)) as usize) as core::ffi::c_char;
            dst = dst.add(1);
            if src < end {
                c = *(src as *const u8);
                src = src.add(1);
                *dst = encode_bits((((b & 15) << 2) | (c >> 6)) as usize) as core::ffi::c_char;
                dst = dst.add(1);
                *dst = encode_bits((c & 63) as usize) as core::ffi::c_char;
                dst = dst.add(1);
            } else {
                *dst = encode_bits(((b & 15) << 2) as usize) as core::ffi::c_char;
                dst = dst.add(1);
                *dst = b'=' as core::ffi::c_char;
                dst = dst.add(1);
            }
        } else {
            *dst = encode_bits(((a & 3) << 4) as usize) as core::ffi::c_char;
            dst = dst.add(1);
            *dst = b'=' as core::ffi::c_char;
            dst = dst.add(1);
            *dst = b'=' as core::ffi::c_char;
            dst = dst.add(1);
        }
        olen += 4;
        line += 4;
        if line == 64 {
            line = 0;
            *dst = b'\n' as core::ffi::c_char;
            dst = dst.add(1);
            olen += 1;
        }
    }
    olen
}

pub unsafe fn ceph_unarmor(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, end: *const core::ffi::c_char) -> i32 {
    let mut olen: i32 = 0;
    let mut dst = dst;
    let mut src = src;

    while src < end {
        let a: i32;
        let b: i32;
        let c: i32;
        let d: i32;

        if *(src as *const u8) == b'\n' {
            src = src.add(1);
            continue;
        }
        if src.add(4) > end {
            return -EINVAL;
        }
        a = decode_bits(*(src as *const u8) as core::ffi::c_char);
        b = decode_bits(*src.add(1) as core::ffi::c_char);
        c = decode_bits(*src.add(2) as core::ffi::c_char);
        d = decode_bits(*src.add(3) as core::ffi::c_char);
        if a < 0 || b < 0 || c < 0 || d < 0 {
            return -EINVAL;
        }

        *dst = ((a << 2) | (b >> 4)) as core::ffi::c_char;
        dst = dst.add(1);
        if *src.add(2) as u8 == b'=' {
            return olen + 1;
        }
        *dst = (((b & 15) << 4) | (c >> 2)) as core::ffi::c_char;
        dst = dst.add(1);
        if *src.add(3) as u8 == b'=' {
            return olen + 2;
        }
        *dst = (((c & 3) << 6) | d) as core::ffi::c_char;
        dst = dst.add(1);
        olen += 3;
        src = src.add(4);
    }
    olen
}

/* base64 encode/decode. */

static PEM_KEY: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn encode_bits(c: usize) -> i32 {
    PEM_KEY[c] as i32
}

fn decode_bits(c: core::ffi::c_char) -> i32 {
    let c = c as u8;
    if c >= b'A' && c <= b'Z' {
        return (c - b'A') as i32;
    }
    if c >= b'a' && c <= b'z' {
        return (c - b'a' + 26) as i32;
    }
    if c >= b'0' && c <= b'9' {
        return (c - b'0' + 52) as i32;
    }
    if c == b'+' {
        return 62;
    }
    if c == b'/' {
        return 63;
    }
    if c == b'=' {
        return 0; /* just non-negative, please */
    }
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
