// SPDX-License-Identifier: GPL-2.0-only
/*
 * vpd_decode.c
 *
 * Google VPD decoding routines.
 *
 * Copyright 2017 Google Inc.
 */

// Dependency declarations and build-time definitions are supplied by the
// corresponding VPD header and other translation units.

unsafe fn vpd_decode_len(
    max_len: u32,
    input: *const u8,
    length: *mut u32,
    decoded_len: *mut u32,
) -> i32 {
    let mut more: u8;
    let mut i: u32 = 0;

    if length.is_null() || decoded_len.is_null() {
        return VPD_FAIL;
    }

    *length = 0;
    loop {
        if i >= max_len {
            return VPD_FAIL;
        }

        more = *input.add(i as usize) & 0x80;
        *length <<= 7;
        *length |= (*input.add(i as usize) & 0x7f) as u32;
        i += 1;
        if more == 0 {
            break;
        }
    }

    *decoded_len = i;
    VPD_OK
}

unsafe fn vpd_decode_entry(
    max_len: u32,
    input_buf: *const u8,
    consumed: *mut u32,
    entry: *mut *const u8,
    entry_len: *mut u32,
) -> i32 {
    let mut decoded_len: u32 = 0;
    let mut current_consumed = *consumed;

    if vpd_decode_len(
        max_len - current_consumed,
        input_buf.add(current_consumed as usize),
        entry_len,
        &mut decoded_len,
    ) != VPD_OK
    {
        return VPD_FAIL;
    }
    if max_len - current_consumed < decoded_len {
        return VPD_FAIL;
    }

    current_consumed += decoded_len;
    *entry = input_buf.add(current_consumed as usize);

    /* entry_len is untrusted data and must be checked again. */
    if max_len - current_consumed < *entry_len {
        return VPD_FAIL;
    }

    current_consumed += *entry_len;
    *consumed = current_consumed;
    VPD_OK
}

pub unsafe fn vpd_decode_string(
    max_len: u32,
    input_buf: *const u8,
    consumed: *mut u32,
    callback: vpd_decode_callback,
    callback_arg: *mut core::ffi::c_void,
) -> i32 {
    let mut type_: i32;
    let mut key_len: u32 = 0;
    let mut value_len: u32 = 0;
    let mut key: *const u8 = core::ptr::null();
    let mut value: *const u8 = core::ptr::null();

    /* type */
    if *consumed >= max_len {
        return VPD_FAIL;
    }

    type_ = *input_buf.add(*consumed as usize) as i32;

    match type_ {
        VPD_TYPE_INFO | VPD_TYPE_STRING => {
            *consumed += 1;

            if vpd_decode_entry(max_len, input_buf, consumed, &mut key, &mut key_len)
                != VPD_OK
            {
                return VPD_FAIL;
            }

            if vpd_decode_entry(max_len, input_buf, consumed, &mut value, &mut value_len)
                != VPD_OK
            {
                return VPD_FAIL;
            }

            if type_ == VPD_TYPE_STRING {
                return callback(key, key_len, value, value_len, callback_arg);
            }
        }
        _ => return VPD_FAIL,
    }

    VPD_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
