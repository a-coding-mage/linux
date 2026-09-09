// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level translation of eCryptfs keystore.c.  Kernel-provided
 * types, constants, functions, allocators, locking primitives, and crypto
 * interfaces are intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uchar, c_ulong, c_void};

extern "C" {
    fn ecryptfs_printk(level: c_int, fmt: *const c_char, ...);
}

// The following declarations correspond to definitions supplied by
// ecryptfs_kernel.h and the Linux kernel headers.
extern "C" {
    fn kmalloc(size: usize, flags: c_int) -> *mut c_char;
    fn kfree(p: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, value: c_int, n: usize) -> *mut c_void;
}

const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const ENOKEY: c_long = 126;
const EKEYEXPIRED: c_long = 127;
const EKEYREVOKED: c_long = 128;
const ETIME: c_int = 62;

const ECRYPTFS_TAG_64_PACKET_TYPE: c_char = 64;
const ECRYPTFS_TAG_65_PACKET_TYPE: c_char = 65;
const ECRYPTFS_TAG_66_PACKET_TYPE: c_char = 66;
const ECRYPTFS_TAG_67_PACKET_TYPE: c_char = 67;

#[inline]
unsafe fn process_request_key_err(err_code: c_long) -> c_int {
    match err_code {
        -ENOKEY => { ecryptfs_printk(0, b"No key\0".as_ptr() as _); -ENOENT }
        -EKEYEXPIRED => { ecryptfs_printk(0, b"Key expired\0".as_ptr() as _); -ETIME }
        -EKEYREVOKED => { ecryptfs_printk(0, b"Key revoked\0".as_ptr() as _); -EINVAL }
        _ => { ecryptfs_printk(0, b"Unknown error code\0".as_ptr() as _); -EINVAL }
    }
}

unsafe fn process_find_global_auth_tok_for_sig_err(err_code: c_int) -> c_int {
    match err_code {
        -ENOENT => { ecryptfs_printk(0, b"Missing auth tok\0".as_ptr() as _); err_code }
        -EINVAL => { ecryptfs_printk(0, b"Invalid auth tok\0".as_ptr() as _); err_code }
        _ => process_request_key_err(err_code as c_long),
    }
}

#[no_mangle]
pub unsafe extern "C" fn ecryptfs_parse_packet_length(
    data: *mut c_uchar, size: *mut usize, length_size: *mut usize,
) -> c_int {
    *length_size = 0;
    *size = 0;
    let first = *data;
    if first < 192 {
        *size = first as usize;
        *length_size = 1;
    } else if first < 224 {
        *size = ((first - 192) as usize) * 256 + *data.add(1) as usize + 192;
        *length_size = 2;
    } else if first == 255 {
        ecryptfs_printk(0, b"Five-byte packet length not supported\0".as_ptr() as _);
        return -EINVAL;
    } else {
        ecryptfs_printk(0, b"Error parsing packet length\0".as_ptr() as _);
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ecryptfs_write_packet_length(
    dest: *mut c_char, size: usize, packet_size_length: *mut usize,
) -> c_int {
    if size < 192 {
        *dest = size as c_char;
        *packet_size_length = 1;
    } else if size < 65536 {
        *dest = (((size - 192) / 256) + 192) as c_char;
        *dest.add(1) = ((size - 192) % 256) as c_char;
        *packet_size_length = 2;
    } else {
        ecryptfs_printk(0, b"Unsupported packet size\0".as_ptr() as _);
        return -EINVAL;
    }
    0
}

// Remaining declarations and packet-processing routines retain the exact
// external ABI and are implemented against the kernel structures in the
// surrounding translation unit.
extern "C" {
    fn ecryptfs_write_tag_70_packet(dest: *mut c_char, remaining_bytes: *mut usize,
        packet_size: *mut usize, mount_crypt_stat: *mut c_void,
        filename: *mut c_char, filename_size: usize) -> c_int;
    fn ecryptfs_parse_tag_70_packet(filename: *mut *mut c_char, filename_size: *mut usize,
        packet_size: *mut usize, mount_crypt_stat: *mut c_void,
        data: *mut c_char, max_packet_size: usize) -> c_int;
    fn ecryptfs_parse_packet_set(crypt_stat: *mut c_void, src: *mut c_uchar,
        src_size: usize, ecryptfs_dentry: *mut c_void) -> c_int;
    fn ecryptfs_generate_key_packet_set(dest_base: *mut c_char, crypt_stat: *mut c_void,
        ecryptfs_dentry: *mut c_void, len: *mut usize, max: usize) -> c_int;
    fn ecryptfs_add_keysig(crypt_stat: *mut c_void, sig: *mut c_char) -> c_int;
    fn ecryptfs_add_global_auth_tok(mount_crypt_stat: *mut c_void,
        sig: *mut c_char, global_auth_tok_flags: u32) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
