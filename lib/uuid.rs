// SPDX-License-Identifier: GPL-2.0-only
/*
 * Unified UUID/GUID definition
 *
 * Copyright (C) 2009, 2016 Intel Corp.
 *	Huang Ying <ying.huang@intel.com>
 */

use core::ffi::{c_char, c_int, c_void};

// Types and helper functions are supplied by the corresponding kernel headers.
#[repr(C)]
pub struct guid_t {
    pub b: [u8; 16],
}

#[repr(C)]
pub struct uuid_t {
    pub b: [u8; 16],
}

unsafe extern "C" {
    fn get_random_bytes(buf: *mut c_void, nbytes: usize);
    fn isxdigit(c: c_int) -> c_int;
    fn hex_to_bin(c: c_int) -> c_int;
}

pub const guid_null: guid_t = guid_t { b: [0; 16] };
pub const uuid_null: uuid_t = uuid_t { b: [0; 16] };

pub const guid_index: [u8; 16] = [3, 2, 1, 0, 5, 4, 7, 6, 8, 9, 10, 11, 12, 13, 14, 15];
pub const uuid_index: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

pub const UUID_STRING_LEN: usize = 36;

/// generate_random_uuid - generate a random UUID
/// @uuid: where to put the generated UUID
///
/// Random UUID interface
///
/// Used to create a Boot ID or a filesystem UUID/GUID, but can be
/// useful for other kernel drivers.
pub unsafe fn generate_random_uuid(uuid: *mut u8) {
    get_random_bytes(uuid.cast::<c_void>(), 16);
    /* Set UUID version to 4 --- truly random generation */
    *uuid.add(6) = (*uuid.add(6) & 0x0F) | 0x40;
    /* Set the UUID variant to DCE */
    *uuid.add(8) = (*uuid.add(8) & 0x3F) | 0x80;
}

pub unsafe fn generate_random_guid(guid: *mut u8) {
    get_random_bytes(guid.cast::<c_void>(), 16);
    /* Set GUID version to 4 --- truly random generation */
    *guid.add(7) = (*guid.add(7) & 0x0F) | 0x40;
    /* Set the GUID variant to DCE */
    *guid.add(8) = (*guid.add(8) & 0x3F) | 0x80;
}

unsafe fn __uuid_gen_common(b: *mut u8) {
    get_random_bytes(b.cast::<c_void>(), 16);
    /* revision 0b10 */
    *b.add(8) = (*b.add(8) & 0x3F) | 0x80;
}

pub unsafe fn guid_gen(lu: *mut guid_t) {
    __uuid_gen_common((*lu).b.as_mut_ptr());
    /* version 4 : random generation */
    (*lu).b[7] = ((*lu).b[7] & 0x0F) | 0x40;
}

pub unsafe fn uuid_gen(bu: *mut uuid_t) {
    __uuid_gen_common((*bu).b.as_mut_ptr());
    /* version 4 : random generation */
    (*bu).b[6] = ((*bu).b[6] & 0x0F) | 0x40;
}

/// uuid_is_valid - checks if a UUID string is valid
/// @uuid: UUID string to check
///
/// Description:
/// It checks if the UUID string is following the format:
///     xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
///
/// where x is a hex digit.
///
/// Return: true if input is valid UUID string.
pub unsafe fn uuid_is_valid(uuid: *const c_char) -> bool {
    for i in 0..UUID_STRING_LEN {
        if i == 8 || i == 13 || i == 18 || i == 23 {
            if *uuid.add(i) as u8 != b'-' {
                return false;
            }
        } else if isxdigit(*uuid.add(i) as c_int) == 0 {
            return false;
        }
    }

    true
}

unsafe fn __uuid_parse(uuid: *const c_char, b: *mut u8, ei: *const u8) -> c_int {
    const SI: [u8; 16] = [0, 2, 4, 6, 9, 11, 14, 16, 19, 21, 24, 26, 28, 30, 32, 34];

    if !uuid_is_valid(uuid) {
        return -22;
    }

    for i in 0..16 {
        let hi = hex_to_bin(*uuid.add(SI[i] as usize) as c_int);
        let lo = hex_to_bin(*uuid.add((SI[i] + 1) as usize) as c_int);
        *b.add(*ei.add(i) as usize) = ((hi << 4) | lo) as u8;
    }

    0
}

pub unsafe fn guid_parse(uuid: *const c_char, u: *mut guid_t) -> c_int {
    __uuid_parse(uuid, (*u).b.as_mut_ptr(), guid_index.as_ptr())
}

pub unsafe fn uuid_parse(uuid: *const c_char, u: *mut uuid_t) -> c_int {
    __uuid_parse(uuid, (*u).b.as_mut_ptr(), uuid_index.as_ptr())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
