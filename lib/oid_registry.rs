// SPDX-License-Identifier: GPL-2.0-or-later
/* ASN.1 Object identifier (OID) registry
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel dependencies supplied by other translation units.

pub type OID = u32;

unsafe extern "C" {
    static oid_search_table: [OidSearchTable; OID__NR as usize];
    static oid_index: [u32; (OID__NR as usize) + 1];
    static oid_data: [u8; 0];
    fn snprintf(buffer: *mut core::ffi::c_char, size: usize, format: *const core::ffi::c_char, ...) -> i32;
}

#[repr(C)]
pub struct OidSearchTable {
    pub hash: u8,
    pub oid: OID,
}

unsafe extern "C" {
    pub static OID__NR: u32;
}

pub const ASN1_OID: u8 = 0x06;
pub const EBADMSG: i32 = 74;
pub const ENOBUFS: i32 = 105;

/**
 * look_up_OID - Find an OID registration for the specified data
 * @data: Binary representation of the OID
 * @datasize: Size of the binary representation
 */
#[no_mangle]
pub unsafe extern "C" fn look_up_OID(data: *const core::ffi::c_void, datasize: usize) -> OID {
    let octets = data as *const u8;
    let mut xhash: u8;
    let mut oid: OID;
    let mut i: u32;
    let mut j: u32;
    let mut k: u32;
    let mut hash: u32;
    let mut len: usize;

    hash = (datasize as u32).wrapping_sub(1);
    for idx in 0..datasize {
        hash = hash.wrapping_add((*octets.add(idx) as u32).wrapping_mul(33));
    }
    hash = (hash >> 24) ^ (hash >> 16) ^ (hash >> 8) ^ hash;
    hash &= 0xff;

    i = 0;
    k = OID__NR;
    'search: while i < k {
        j = (i + k) / 2;

        xhash = oid_search_table[j as usize].hash;
        if (xhash as u32) > hash {
            k = j;
            continue;
        }
        if (xhash as u32) < hash {
            i = j + 1;
            continue;
        }

        oid = oid_search_table[j as usize].oid;
        len = (oid_index[(oid + 1) as usize] - oid_index[oid as usize]) as usize;
        if len > datasize {
            k = j;
            continue;
        }
        if len < datasize {
            i = j + 1;
            continue;
        }

        while len > 0 {
            len -= 1;
            let a = *oid_data.as_ptr().add(oid_index[oid as usize] as usize + len);
            let b = *octets.add(len);
            if a > b {
                k = j;
                continue 'search;
            }
            if a < b {
                i = j + 1;
                continue 'search;
            }
        }
        return oid;
    }

    OID__NR
}

/**
 * parse_OID - Parse an OID from a bytestream
 * @data: Binary representation of the header + OID
 * @datasize: Size of the binary representation
 * @oid: Pointer to oid to return result
 */
#[no_mangle]
pub unsafe extern "C" fn parse_OID(data: *const core::ffi::c_void, datasize: usize, oid: *mut OID) -> i32 {
    let v = data as *const u8;
    if datasize < 3 || *v != ASN1_OID || *v.add(1) as usize != datasize - 2 {
        return -EBADMSG;
    }

    *oid = look_up_OID(data.add(2), datasize - 2);
    0
}

/**
 * sprint_oid - Print an Object Identifier into a buffer
 */
#[no_mangle]
pub unsafe extern "C" fn sprint_oid(
    data: *const core::ffi::c_void,
    datasize: usize,
    mut buffer: *mut core::ffi::c_char,
    mut bufsize: usize,
) -> i32 {
    let mut v = data as *const u8;
    let end = v.add(datasize);
    let mut num: u64;
    let mut n: u8;
    let mut ret: usize;
    let mut count: i32;

    if v >= end {
        snprintf(buffer, bufsize, b"(bad)\0".as_ptr() as *const _);
        return -EBADMSG;
    }

    n = *v;
    v = v.add(1);
    count = snprintf(buffer, bufsize, b"%u.%u\0".as_ptr() as *const _, n / 40, n % 40);
    ret = count as usize;
    if count >= bufsize as i32 {
        return -ENOBUFS;
    }
    buffer = buffer.add(count as usize);
    bufsize -= count as usize;

    while v < end {
        n = *v;
        v = v.add(1);
        if n & 0x80 == 0 {
            num = n as u64;
        } else {
            num = (n & 0x7f) as u64;
            loop {
                if v >= end {
                    snprintf(buffer, bufsize, b"(bad)\0".as_ptr() as *const _);
                    return -EBADMSG;
                }
                n = *v;
                v = v.add(1);
                num = (num << 7) | (n & 0x7f) as u64;
                if n & 0x80 == 0 {
                    break;
                }
            }
        }
        count = snprintf(buffer, bufsize, b".%lu\0".as_ptr() as *const _, num);
        ret += count as usize;
        if count >= bufsize as i32 {
            return -ENOBUFS;
        }
        buffer = buffer.add(count as usize);
        bufsize -= count as usize;
    }

    ret as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
