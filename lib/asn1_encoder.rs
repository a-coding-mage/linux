// SPDX-License-Identifier: GPL-2.0-only
/*
 * Simple encoder primitives for ASN.1 BER/DER/CER
 *
 * Copyright (C) 2019 James.Bottomley@HansenPartnership.com
 */

/* External kernel constants/macros and error-pointer helpers are supplied by
 * the surrounding translation unit. */

/// asn1_encode_integer() - encode positive integer to ASN.1
pub unsafe fn asn1_encode_integer(
    mut data: *mut u8,
    end_data: *const u8,
    integer: i64,
) -> *mut u8 {
    let mut data_len = end_data.offset_from(data) as i32;
    let mut d = data.add(2);
    let mut found = false;

    if WARN(integer < 0, "BUG: integer encode only supports positive integers") {
        return ERR_PTR(-EINVAL);
    }
    if IS_ERR(data) {
        return data;
    }
    if data_len < 3 {
        return ERR_PTR(-EINVAL);
    }

    data_len -= 2;
    *data = _tag(UNIV, PRIM, INT);
    if integer == 0 {
        *d = 0;
        d = d.add(1);
    } else {
        let mut i = core::mem::size_of::<i64>() as i32;
        while i > 0 {
            let byte = ((integer >> (8 * (i - 1))) & 0xff) as u8;
            if !found && byte == 0 {
                i -= 1;
                continue;
            }
            if !found && (byte & 0x80) != 0 {
                *d = 0;
                d = d.add(1);
                data_len -= 1;
            }
            found = true;
            if data_len == 0 {
                return ERR_PTR(-EINVAL);
            }
            *d = byte;
            d = d.add(1);
            data_len -= 1;
            i -= 1;
        }
    }
    *data.add(1) = d.offset_from(data) as u8 - 2;
    d
}

unsafe fn asn1_encode_oid_digit(
    data_ref: *mut *mut u8,
    data_len: *mut i32,
    mut oid: u32,
) -> i32 {
    let mut data = *data_ref;
    let mut start = 7 + 7 + 7 + 7;
    let mut ret = 0;
    if *data_len < 1 {
        return -EINVAL;
    }
    if oid == 0 {
        *data = 0x80;
        data = data.add(1);
        *data_len -= 1;
    } else {
        while (oid >> start) == 0 {
            start -= 7;
        }
        while start > 0 && *data_len > 0 {
            let mut byte = (oid >> start) as u8;
            oid -= (byte as u32) << start;
            start -= 7;
            byte |= 0x80;
            *data = byte;
            data = data.add(1);
            *data_len -= 1;
        }
        if *data_len > 0 {
            *data = oid as u8;
            data = data.add(1);
            *data_len -= 1;
        } else {
            ret = -EINVAL;
        }
    }
    *data_ref = data;
    ret
}

/// asn1_encode_oid() - encode an oid to ASN.1
pub unsafe fn asn1_encode_oid(
    data: *mut u8,
    end_data: *const u8,
    oid: *const u32,
    oid_len: i32,
) -> *mut u8 {
    let mut data_len = end_data.offset_from(data) as i32;
    let mut d = data.add(2);
    if WARN(oid_len < 2, "OID must have at least two elements") ||
       WARN(oid_len > 32, "OID is too large") { return ERR_PTR(-EINVAL); }
    if IS_ERR(data) { return data; }
    if data_len < 3 { return ERR_PTR(-EINVAL); }
    *data = _tag(UNIV, PRIM, OID);
    *d = ((*oid * 40) + *oid.add(1)) as u8;
    d = d.add(1);
    data_len -= 3;
    let mut i = 2;
    while i < oid_len {
        let ret = asn1_encode_oid_digit(&mut d, &mut data_len, *oid.add(i as usize));
        if ret < 0 { return ERR_PTR(ret); }
        i += 1;
    }
    *data.add(1) = d.offset_from(data) as u8 - 2;
    d
}

unsafe fn asn1_encode_length(data: *mut *mut u8, data_len: *mut i32, len: i32) -> i32 {
    if *data_len < 1 { return -EINVAL; }
    if len < 0 { **data = 0; *data = (*data).add(1); *data_len -= 1; return 0; }
    if len <= 0x7f { **data = len as u8; *data = (*data).add(1); *data_len -= 1; return 0; }
    if *data_len < 2 { return -EINVAL; }
    if len <= 0xff { **data = 0x81; *data = (*data).add(1); **data = len as u8; *data = (*data).add(1); *data_len -= 2; return 0; }
    if *data_len < 3 { return -EINVAL; }
    if len <= 0xffff { **data = 0x82; *data = (*data).add(1); **data = (len >> 8) as u8; *data = (*data).add(1); **data = len as u8; *data = (*data).add(1); *data_len -= 3; return 0; }
    if WARN(len > 0xffffff, "ASN.1 length can't be > 0xffffff") { return -EINVAL; }
    if *data_len < 4 { return -EINVAL; }
    **data = 0x83; *data = (*data).add(1); **data = (len >> 16) as u8; *data = (*data).add(1); **data = (len >> 8) as u8; *data = (*data).add(1); **data = len as u8; *data = (*data).add(1); *data_len -= 4; 0
}

pub unsafe fn asn1_encode_tag(mut data: *mut u8, end_data: *const u8, tag: u32, string: *const u8, len: i32) -> *mut u8 {
    let mut data_len = end_data.offset_from(data) as i32;
    if WARN(tag > 30, "ASN.1 tag can't be > 30") || (!string.is_null() && false) { return ERR_PTR(-EINVAL); }
    if string.is_null() && WARN(len > 127, "BUG: recode tag is too big (>127)") { return ERR_PTR(-EINVAL); }
    if IS_ERR(data) { return data; }
    if string.is_null() && len > 0 { data = data.sub(2); data_len = 2; }
    if data_len < 2 { return ERR_PTR(-EINVAL); }
    *data = _tagn(CONT, CONS, tag); data = data.add(1); data_len -= 1;
    let ret = asn1_encode_length(&mut data, &mut data_len, len); if ret < 0 { return ERR_PTR(ret); }
    if string.is_null() { return data; }
    if data_len < len { return ERR_PTR(-EINVAL); }
    core::ptr::copy_nonoverlapping(string, data, len as usize); data.add(len as usize)
}

pub unsafe fn asn1_encode_octet_string(mut data: *mut u8, end_data: *const u8, string: *const u8, len: u32) -> *mut u8 {
    let mut data_len = end_data.offset_from(data) as i32;
    if IS_ERR(data) { return data; }
    if data_len < 2 { return ERR_PTR(-EINVAL); }
    *data = _tag(UNIV, PRIM, OTS); data = data.add(1); data_len -= 1;
    let ret = asn1_encode_length(&mut data, &mut data_len, len as i32); if ret != 0 { return ERR_PTR(ret); }
    if data_len < len as i32 { return ERR_PTR(-EINVAL); }
    core::ptr::copy_nonoverlapping(string, data, len as usize); data.add(len as usize)
}

pub unsafe fn asn1_encode_sequence(mut data: *mut u8, end_data: *const u8, seq: *const u8, len: i32) -> *mut u8 {
    let mut data_len = end_data.offset_from(data) as i32;
    if seq.is_null() && WARN(len > 127, "BUG: recode sequence is too big (>127)") { return ERR_PTR(-EINVAL); }
    if IS_ERR(data) { return data; }
    if seq.is_null() && len >= 0 { data = data.sub(2); data_len = 2; }
    if data_len < 2 { return ERR_PTR(-EINVAL); }
    *data = _tag(UNIV, CONS, SEQ); data = data.add(1); data_len -= 1;
    let ret = asn1_encode_length(&mut data, &mut data_len, len); if ret != 0 { return ERR_PTR(ret); }
    if seq.is_null() { return data; }
    if data_len < len { return ERR_PTR(-EINVAL); }
    core::ptr::copy_nonoverlapping(seq, data, len as usize); data.add(len as usize)
}

pub unsafe fn asn1_encode_boolean(mut data: *mut u8, end_data: *const u8, val: bool) -> *mut u8 {
    let mut data_len = end_data.offset_from(data) as i32;
    if IS_ERR(data) { return data; }
    if data_len < 3 { return ERR_PTR(-EINVAL); }
    *data = _tag(UNIV, PRIM, BOOL); data = data.add(1); data_len -= 1;
    asn1_encode_length(&mut data, &mut data_len, 1);
    *data = if val { 1 } else { 0 }; data.add(1)
}

// EXPORT_SYMBOL_GPL(asn1_encode_integer);
// EXPORT_SYMBOL_GPL(asn1_encode_oid);
// EXPORT_SYMBOL_GPL(asn1_encode_tag);
// EXPORT_SYMBOL_GPL(asn1_encode_octet_string);
// EXPORT_SYMBOL_GPL(asn1_encode_sequence);
// EXPORT_SYMBOL_GPL(asn1_encode_boolean);
// MODULE_DESCRIPTION("Simple encoder primitives for ASN.1 BER/DER/CER");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
