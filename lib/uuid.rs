// SPDX-License-Identifier: GPL-2.0-only
/*
 * Unified UUID/GUID definition
 *
 * Copyright (C) 2009, 2016 Intel Corp.
 *	Huang Ying <ying.huang@intel.com>
 */

//! UUID/GUID helpers with bytewise C-compatible memory access.

use kernel::bindings::{self, guid_t, uuid_t};
use kernel::ffi::{c_char, c_int, c_uchar};

/// All-zero GUID.
#[no_mangle]
pub static guid_null: guid_t = guid_t { b: [0; 16] };
/// All-zero UUID.
#[no_mangle]
pub static uuid_null: uuid_t = uuid_t { b: [0; 16] };

/// Text-pair to GUID byte mapping (global, deliberately not exported).
#[no_mangle]
pub static guid_index: [u8; 16] = [3, 2, 1, 0, 5, 4, 7, 6, 8, 9, 10, 11, 12, 13, 14, 15];
/// Text-pair to UUID byte mapping (global, deliberately not exported).
#[no_mangle]
pub static uuid_index: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

fn hex(byte: u8) -> c_int {
    match byte {
        b'0'..=b'9' => (byte - b'0') as c_int,
        b'a'..=b'f' => (byte - b'a' + 10) as c_int,
        b'A'..=b'F' => (byte - b'A' + 10) as c_int,
        _ => -1,
    }
}

fn valid(mut read: impl FnMut(usize) -> u8) -> bool {
    for i in 0..36 {
        let byte = read(i);
        if matches!(i, 8 | 13 | 18 | 23) {
            if byte != b'-' {
                return false;
            }
        } else if hex(byte) < 0 {
            return false;
        }
    }
    true
}

fn parse(
    mut read: impl FnMut(usize) -> u8,
    mut write: impl FnMut(usize, u8),
    index: &[u8; 16],
) -> c_int {
    const SI: [usize; 16] = [0, 2, 4, 6, 9, 11, 14, 16, 19, 21, 24, 26, 28, 30, 32, 34];
    if !valid(&mut read) {
        return -22;
    }
    for i in 0..16 {
        // Reread each pair after preceding writes: input and output may alias.
        // Overlap can replace validated characters with nonhex bytes. Preserve
        // hex_to_bin's -1 result and the original low-byte bit operation.
        let hi = hex(read(SI[i]));
        let lo = hex(read(SI[i] + 1));
        write(index[i] as usize, ((hi << 4) | lo) as u8);
    }
    0
}

fn version(byte: u8) -> u8 {
    (byte & 0x0f) | 0x40
}

fn variant(byte: u8) -> u8 {
    (byte & 0x3f) | 0x80
}

unsafe fn generate(bytes: *mut u8, version_index: usize, variant_first: bool) {
    // SAFETY: Every entry point requires 16 writable bytes. The genuine binding
    // carries the kernel RNG signature; no references to caller memory exist.
    unsafe {
        bindings::get_random_bytes(bytes.cast(), 16);
        if variant_first {
            bytes.add(8).write(variant(bytes.add(8).read()));
        }
        bytes
            .add(version_index)
            .write(version(bytes.add(version_index).read()));
        if !variant_first {
            bytes.add(8).write(variant(bytes.add(8).read()));
        }
    }
}

/// Fills a UUID using the kernel cryptographic RNG.
///
/// # Safety
/// `uuid` must address 16 writable bytes.
#[no_mangle]
pub unsafe extern "C" fn generate_random_uuid(uuid: *mut c_uchar) {
    // SAFETY: Forward the caller's 16-byte writable region.
    unsafe { generate(uuid, 6, false) }
}

/// Fills a GUID using the kernel cryptographic RNG.
///
/// # Safety
/// `guid` must address 16 writable bytes.
#[no_mangle]
pub unsafe extern "C" fn generate_random_guid(guid: *mut c_uchar) {
    // SAFETY: Forward the caller's 16-byte writable region.
    unsafe { generate(guid, 7, false) }
}

/// Generates a GUID, setting the variant before the version as in C.
///
/// # Safety
/// `u` must address a writable GUID.
#[no_mangle]
pub unsafe extern "C" fn guid_gen(u: *mut guid_t) {
    // SAFETY: Raw field addressing introduces no overlapping reference.
    unsafe { generate(core::ptr::addr_of_mut!((*u).b).cast(), 7, true) }
}

/// Generates a UUID, setting the variant before the version as in C.
///
/// # Safety
/// `u` must address a writable UUID.
#[no_mangle]
pub unsafe extern "C" fn uuid_gen(u: *mut uuid_t) {
    // SAFETY: Raw field addressing introduces no overlapping reference.
    unsafe { generate(core::ptr::addr_of_mut!((*u).b).cast(), 6, true) }
}

/// Validates exactly 36 positions, stopping at the first invalid byte.
///
/// # Safety
/// `uuid` must be readable through the first invalid position, or for 36 bytes
/// if valid. No trailing NUL or readable bytes beyond that prefix are required.
#[no_mangle]
pub unsafe extern "C" fn uuid_is_valid(uuid: *const c_char) -> bool {
    // SAFETY: valid requests bytes in order and stops at the first invalid one.
    valid(|i| unsafe { uuid.add(i).read() })
}

unsafe fn parse_raw(uuid: *const c_char, bytes: *mut u8, index: &[u8; 16]) -> c_int {
    // SAFETY: The caller guarantees the accessed input prefix and, on success,
    // 16 output bytes. Raw accesses allow overlap without creating references.
    parse(
        |i| unsafe { uuid.add(i).read() },
        |i, value| unsafe { bytes.add(i).write(value) },
        index,
    )
}

/// Parses GUID text with the original validation-before-write behavior.
///
/// # Safety
/// Input must satisfy uuid_is_valid's prefix contract. On valid input `u`
/// must address a writable GUID; overlap with input is permitted.
#[no_mangle]
pub unsafe extern "C" fn guid_parse(uuid: *const c_char, u: *mut guid_t) -> c_int {
    // SAFETY: Use a raw cast so invalid input never requires dereferencing u.
    unsafe { parse_raw(uuid, u.cast(), &guid_index) }
}

/// Parses UUID text with the original validation-before-write behavior.
///
/// # Safety
/// Input must satisfy uuid_is_valid's prefix contract. On valid input `u`
/// must address a writable UUID; overlap with input is permitted.
#[no_mangle]
pub unsafe extern "C" fn uuid_parse(uuid: *const c_char, u: *mut uuid_t) -> c_int {
    // SAFETY: The binding's sole byte array starts at offset zero.
    unsafe { parse_raw(uuid, u.cast(), &uuid_index) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
