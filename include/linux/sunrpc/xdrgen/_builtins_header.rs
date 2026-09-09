/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2024 Oracle and/or its affiliates.
 *
 * This header defines XDR data type primitives specified in
 * Section 4 of RFC 4506, used by RPC programs implemented
 * in the Linux kernel.
 */

// Dependency supplied by the Linux SUNRPC XDR implementation:
// linux/sunrpc/xdr.h

pub unsafe fn xdrgen_decode_void(_xdr: *mut xdr_stream) -> bool {
    true
}

pub unsafe fn xdrgen_encode_void(_xdr: *mut xdr_stream) -> bool {
    true
}

pub unsafe fn xdrgen_decode_bool(xdr: *mut xdr_stream, ptr: *mut bool) -> bool {
    let p: *mut __be32 = xdr_inline_decode(xdr, XDR_UNIT);

    if p.is_null() {
        return false;
    }
    *ptr = *p != xdr_zero;
    true
}

pub unsafe fn xdrgen_encode_bool(xdr: *mut xdr_stream, val: bool) -> bool {
    let p: *mut __be32 = xdr_reserve_space(xdr, XDR_UNIT);

    if p.is_null() {
        return false;
    }
    *p = if val { xdr_one } else { xdr_zero };
    true
}

/*
 * De facto (non-standard but commonly implemented) signed short type:
 *  - Wire sends sign-extended 32-bit value (e.g., 0xFFFFFFFF)
 *  - be32_to_cpup() returns u32 (0xFFFFFFFF)
 *  - Explicit (s16) cast truncates to 16 bits (0xFFFF = -1)
 */
pub unsafe fn xdrgen_decode_short(xdr: *mut xdr_stream, ptr: *mut s16) -> bool {
    let p: *mut __be32 = xdr_inline_decode(xdr, XDR_UNIT);

    if p.is_null() {
        return false;
    }
    *ptr = be32_to_cpup(p) as s16;
    true
}

/*
 * De facto (non-standard but commonly implemented) signed short type:
 *  - C integer promotion sign-extends s16 val to int before passing to
 *    cpu_to_be32()
 *  - This is well-defined: -1 as s16 -1 as int 0xFFFFFFFF on wire
 */
pub unsafe fn xdrgen_encode_short(xdr: *mut xdr_stream, val: s16) -> bool {
    let p: *mut __be32 = xdr_reserve_space(xdr, XDR_UNIT);

    if p.is_null() {
        return false;
    }
    *p = cpu_to_be32(val as s32);
    true
}

/*
 * De facto (non-standard but commonly implemented) unsigned short type:
 * 16-bit integer zero-extended to fill one XDR_UNIT.
 */
pub unsafe fn xdrgen_decode_unsigned_short(xdr: *mut xdr_stream, ptr: *mut u16) -> bool {
    let p: *mut __be32 = xdr_inline_decode(xdr, XDR_UNIT);

    if p.is_null() {
        return false;
    }
    *ptr = be32_to_cpup(p) as u16;
    true
}

pub unsafe fn xdrgen_encode_unsigned_short(xdr: *mut xdr_stream, val: u16) -> bool {
    let p: *mut __be32 = xdr_reserve_space(xdr, XDR_UNIT);

    if p.is_null() {
        return false;
    }
    *p = cpu_to_be32(val as u32);
    true
}

pub unsafe fn xdrgen_decode_int(xdr: *mut xdr_stream, ptr: *mut s32) -> bool {
    let p: *mut __be32 = xdr_inline_decode(xdr, XDR_UNIT);

    if p.is_null() {
        return false;
    }
    *ptr = be32_to_cpup(p) as s32;
    true
}

pub unsafe fn xdrgen_encode_int(xdr: *mut xdr_stream, val: s32) -> bool {
    let p: *mut __be32 = xdr_reserve_space(xdr, XDR_UNIT);

    if p.is_null() {
        return false;
    }
    *p = cpu_to_be32(val);
    true
}

pub unsafe fn xdrgen_decode_unsigned_int(xdr: *mut xdr_stream, ptr: *mut u32) -> bool {
    let p: *mut __be32 = xdr_inline_decode(xdr, XDR_UNIT);

    if p.is_null() {
        return false;
    }
    *ptr = be32_to_cpup(p);
    true
}

pub unsafe fn xdrgen_encode_unsigned_int(xdr: *mut xdr_stream, val: u32) -> bool {
    let p: *mut __be32 = xdr_reserve_space(xdr, XDR_UNIT);

    if p.is_null() {
        return false;
    }
    *p = cpu_to_be32(val);
    true
}

pub unsafe fn xdrgen_decode_long(xdr: *mut xdr_stream, ptr: *mut s32) -> bool {
    let p: *mut __be32 = xdr_inline_decode(xdr, XDR_UNIT);

    if p.is_null() {
        return false;
    }
    *ptr = be32_to_cpup(p) as s32;
    true
}

pub unsafe fn xdrgen_encode_long(xdr: *mut xdr_stream, val: s32) -> bool {
    let p: *mut __be32 = xdr_reserve_space(xdr, XDR_UNIT);

    if p.is_null() {
        return false;
    }
    *p = cpu_to_be32(val);
    true
}

pub unsafe fn xdrgen_decode_unsigned_long(xdr: *mut xdr_stream, ptr: *mut u32) -> bool {
    let p: *mut __be32 = xdr_inline_decode(xdr, XDR_UNIT);

    if p.is_null() {
        return false;
    }
    *ptr = be32_to_cpup(p);
    true
}

pub unsafe fn xdrgen_encode_unsigned_long(xdr: *mut xdr_stream, val: u32) -> bool {
    let p: *mut __be32 = xdr_reserve_space(xdr, XDR_UNIT);

    if p.is_null() {
        return false;
    }
    *p = cpu_to_be32(val);
    true
}

pub unsafe fn xdrgen_decode_hyper(xdr: *mut xdr_stream, ptr: *mut s64) -> bool {
    let p: *mut __be32 = xdr_inline_decode(xdr, XDR_UNIT * 2);

    if p.is_null() {
        return false;
    }
    *ptr = get_unaligned_be64(p) as s64;
    true
}

pub unsafe fn xdrgen_encode_hyper(xdr: *mut xdr_stream, val: s64) -> bool {
    let p: *mut __be32 = xdr_reserve_space(xdr, XDR_UNIT * 2);

    if p.is_null() {
        return false;
    }
    put_unaligned_be64(val as u64, p);
    true
}

pub unsafe fn xdrgen_decode_unsigned_hyper(xdr: *mut xdr_stream, ptr: *mut u64) -> bool {
    let p: *mut __be32 = xdr_inline_decode(xdr, XDR_UNIT * 2);

    if p.is_null() {
        return false;
    }
    *ptr = get_unaligned_be64(p);
    true
}

pub unsafe fn xdrgen_encode_unsigned_hyper(xdr: *mut xdr_stream, val: u64) -> bool {
    let p: *mut __be32 = xdr_reserve_space(xdr, XDR_UNIT * 2);

    if p.is_null() {
        return false;
    }
    put_unaligned_be64(val, p);
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
