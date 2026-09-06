// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2013 Politecnico di Torino, Italy
 *                    TORSEC group -- https://security.polito.it
 *
 * Author: Roberto Sassu <roberto.sassu@polito.it>
 *
 * File: ima_template_lib.c
 *      Library of supported template fields.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type bool_t = bool;

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ima_field_data {
    pub data: *mut c_void,
    pub len: u32,
}

#[repr(C)]
pub struct ima_event_data {
    pub iint: *mut ima_iint_cache,
    pub file: *mut file,
    pub filename: *const c_char,
    pub xattr_value: *mut evm_ima_xattr_data,
    pub xattr_len: u32,
    pub buf: *const c_void,
    pub buf_len: u32,
    pub violation: bool_t,
    pub modsig: *mut modsig,
}

#[repr(C)]
pub struct ima_iint_cache {
    pub ima_hash: *mut ima_digest_data,
    pub flags: c_ulong,
}

#[repr(C)]
pub struct ima_digest_data_hdr {
    pub algo: u8,
    pub length: u8,
}

#[repr(C)]
pub struct ima_digest_data {
    pub hdr: ima_digest_data_hdr,
    pub digest: [u8; 0],
}

#[repr(C)]
pub struct ima_max_digest_data {
    pub hdr: ima_digest_data_hdr,
    pub digest: [u8; IMA_MAX_DIGEST_SIZE as usize],
}

#[repr(C)]
pub struct file {
    pub f_path: path,
}

#[repr(C)]
pub struct path {
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    pub i_mode: u16,
}

#[repr(C)]
pub struct qstr {
    pub name: *const c_char,
}

#[repr(C)]
pub struct name_snapshot {
    pub name: qstr,
}

#[repr(C)]
pub struct evm_ima_xattr_data {
    pub type_: u8,
}

#[repr(C)]
pub struct modsig {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ima_show_type {
    IMA_SHOW_ASCII,
    IMA_SHOW_BINARY,
    IMA_SHOW_BINARY_NO_FIELD_LEN,
    IMA_SHOW_BINARY_OLD_STRING_FMT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum data_formats {
    DATA_FMT_DIGEST = 0,
    DATA_FMT_DIGEST_WITH_ALGO,
    DATA_FMT_DIGEST_WITH_TYPE_AND_ALGO,
    DATA_FMT_STRING,
    DATA_FMT_HEX,
    DATA_FMT_UINT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum digest_type {
    DIGEST_TYPE_IMA,
    DIGEST_TYPE_VERITY,
    DIGEST_TYPE__LAST,
}

const DIGEST_TYPE_NAME_LEN_MAX: usize = 7; /* including NUL */
const DATA_FMT_DIGEST: data_formats = data_formats::DATA_FMT_DIGEST;
const DATA_FMT_DIGEST_WITH_ALGO: data_formats = data_formats::DATA_FMT_DIGEST_WITH_ALGO;
const DATA_FMT_DIGEST_WITH_TYPE_AND_ALGO: data_formats =
    data_formats::DATA_FMT_DIGEST_WITH_TYPE_AND_ALGO;
const DATA_FMT_STRING: data_formats = data_formats::DATA_FMT_STRING;
const DATA_FMT_HEX: data_formats = data_formats::DATA_FMT_HEX;
const DATA_FMT_UINT: data_formats = data_formats::DATA_FMT_UINT;
const DIGEST_TYPE_IMA: u8 = digest_type::DIGEST_TYPE_IMA as u8;
const DIGEST_TYPE_VERITY: u8 = digest_type::DIGEST_TYPE_VERITY as u8;
const DIGEST_TYPE__LAST: u8 = digest_type::DIGEST_TYPE__LAST as u8;

extern "C" {
    static digest_type_name: [*const c_char; digest_type::DIGEST_TYPE__LAST as usize];
    static hash_algo_name: [*const c_char; HASH_ALGO__LAST as usize];
    static hash_digest_size: [u32; HASH_ALGO__LAST as usize];
    static mut ima_canonical_fmt: bool_t;
    static mut ima_hash_algo: u8;
    static mut ima_tpm_chip: *mut c_void;
    static boot_aggregate_name: *const c_char;
    static boot_aggregate_late_name: *const c_char;
    static nop_mnt_idmap: c_void;

    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn sprintf(s: *mut u8, format: *const c_char, ...) -> c_int;
    fn seq_printf(m: *mut seq_file, format: *const c_char, ...);
    fn ima_print_digest(m: *mut seq_file, digest: *const u8, size: u32);
    fn ima_putc(m: *mut seq_file, data: *const c_void, datalen: usize);
    fn test_bit(nr: c_int, addr: *const c_ulong) -> bool_t;
    fn pr_err(format: *const c_char, ...);
    fn ima_calc_boot_aggregate(hash: *mut ima_digest_data) -> c_int;
    fn ima_calc_file_hash(file: *mut file, hash: *mut ima_digest_data) -> c_int;
    fn integrity_audit_msg(
        audit_msgno: c_int,
        inode: *mut inode,
        name: *const c_char,
        op: *const c_char,
        cause: *const c_char,
        result: c_int,
        info: c_int,
    );
    fn file_inode(file: *mut file) -> *mut inode;
    fn file_dentry(file: *mut file) -> *mut dentry;
    fn ima_get_modsig_digest(
        modsig: *mut modsig,
        algo: *mut u8,
        digest: *mut *const u8,
        digestsize: *mut u32,
    ) -> c_int;
    fn take_dentry_name_snapshot(name: *mut name_snapshot, dentry: *mut dentry);
    fn release_dentry_name_snapshot(name: *mut name_snapshot);
    fn ima_get_raw_modsig(
        modsig: *mut modsig,
        data: *mut *const c_void,
        data_len: *mut u32,
    ) -> c_int;
    fn vfs_getxattr_alloc(
        idmap: *const c_void,
        dentry: *mut dentry,
        name: *const c_char,
        value: *mut *mut c_char,
        size: usize,
        flags: c_uint,
    ) -> c_int;
    fn i_uid_read(inode: *mut inode) -> c_uint;
    fn i_gid_read(inode: *mut inode) -> c_uint;
    fn evm_read_protected_xattrs(
        dentry: *mut dentry,
        buffer: *mut u8,
        buffer_size: c_int,
        type_: c_char,
        canonical_fmt: bool_t,
    ) -> c_int;
}

const GFP_KERNEL: c_uint = 0;
const GFP_NOFS: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const HASH_ALGO_SHA1: u8 = 0;
const HASH_ALGO_MD5: u8 = 1;
const HASH_ALGO__LAST: usize = 255;
const CRYPTO_MAX_ALG_NAME: usize = 64;
const IMA_MAX_DIGEST_SIZE: usize = 64;
const IMA_DIGEST_SIZE: u32 = 20;
const IMA_EVENT_NAME_LEN_MAX: u32 = 255;
const IMA_VERITY_REQUIRED: c_ulong = 0;
const EVM_IMA_XATTR_DIGSIG: u8 = 0;
const IMA_VERITY_DIGSIG: u8 = 0;
const EVM_XATTR_PORTABLE_DIGSIG: u8 = 0;
const AUDIT_INTEGRITY_DATA: c_int = 0;
const ENFORCE_FIELDS: c_int = 1;
const ENFORCE_BUFEND: c_int = 2;
const XATTR_NAME_EVM: *const c_char = b"security.evm\0".as_ptr() as *const c_char;

unsafe fn le16_to_cpu(x: u16) -> u16 {
    u16::from_le(x)
}

unsafe fn le32_to_cpu(x: u32) -> u32 {
    u32::from_le(x)
}

unsafe fn le64_to_cpu(x: u64) -> u64 {
    u64::from_le(x)
}

unsafe fn cpu_to_le16(x: u16) -> u16 {
    x.to_le()
}

unsafe fn cpu_to_le32(x: u32) -> u32 {
    x.to_le()
}

unsafe fn BUG_ON(condition: bool_t) {
    if condition {
        core::intrinsics::abort();
    }
}

unsafe fn ima_template_hash_algo_allowed(algo: u8) -> bool_t {
    if algo == HASH_ALGO_SHA1 || algo == HASH_ALGO_MD5 {
        return true;
    }

    false
}

unsafe fn ima_write_template_field_data(
    data: *const c_void,
    datalen: u32,
    datafmt: data_formats,
    field_data: *mut ima_field_data,
) -> c_int {
    let mut buf: *mut u8;
    let mut buf_ptr: *mut u8;
    let mut buflen: u32 = datalen;

    if datafmt == DATA_FMT_STRING {
        buflen = datalen.wrapping_add(1);
    }

    buf = kzalloc(buflen as usize, GFP_KERNEL) as *mut u8;
    if buf.is_null() {
        return -ENOMEM;
    }

    memcpy(buf as *mut c_void, data, datalen as usize);

    /*
     * Replace all space characters with underscore for event names and
     * strings. This avoid that, during the parsing of a measurements list,
     * filenames with spaces or that end with the suffix ' (deleted)' are
     * split into multiple template fields (the space is the delimitator
     * character for measurements lists in ASCII format).
     */
    if datafmt == DATA_FMT_STRING {
        buf_ptr = buf;
        while buf_ptr.offset_from(buf) < datalen as isize {
            if *buf_ptr == b' ' {
                *buf_ptr = b'_';
            }
            buf_ptr = buf_ptr.add(1);
        }
    }

    (*field_data).data = buf as *mut c_void;
    (*field_data).len = buflen;
    0
}

unsafe fn ima_show_template_data_ascii(
    m: *mut seq_file,
    _show: ima_show_type,
    datafmt: data_formats,
    field_data: *mut ima_field_data,
) {
    let mut buf_ptr: *mut u8 = (*field_data).data as *mut u8;
    let mut buflen: u32 = (*field_data).len;

    match datafmt {
        data_formats::DATA_FMT_DIGEST_WITH_TYPE_AND_ALGO
        | data_formats::DATA_FMT_DIGEST_WITH_ALGO => {
            buf_ptr = strrchr((*field_data).data as *const c_char, b':' as c_int) as *mut u8;
            if buf_ptr != (*field_data).data as *mut u8 {
                seq_printf(m, b"%s\0".as_ptr() as *const c_char, (*field_data).data);
            }

            /* skip ':' and '\0' */
            buf_ptr = buf_ptr.add(2);
            buflen = buflen.wrapping_sub(buf_ptr.offset_from((*field_data).data as *mut u8) as u32);
            if buflen != 0 {
                ima_print_digest(m, buf_ptr, buflen);
            }
        }
        data_formats::DATA_FMT_DIGEST | data_formats::DATA_FMT_HEX => {
            if buflen != 0 {
                ima_print_digest(m, buf_ptr, buflen);
            }
        }
        data_formats::DATA_FMT_STRING => {
            seq_printf(m, b"%s\0".as_ptr() as *const c_char, buf_ptr);
        }
        data_formats::DATA_FMT_UINT => {
            match (*field_data).len as usize {
                x if x == size_of::<u8>() => {
                    seq_printf(m, b"%u\0".as_ptr() as *const c_char, *(buf_ptr as *mut u8) as c_uint);
                }
                x if x == size_of::<u16>() => {
                    if ima_canonical_fmt {
                        seq_printf(
                            m,
                            b"%u\0".as_ptr() as *const c_char,
                            le16_to_cpu(*(buf_ptr as *mut u16)) as c_uint,
                        );
                    } else {
                        seq_printf(m, b"%u\0".as_ptr() as *const c_char, *(buf_ptr as *mut u16) as c_uint);
                    }
                }
                x if x == size_of::<u32>() => {
                    if ima_canonical_fmt {
                        seq_printf(
                            m,
                            b"%u\0".as_ptr() as *const c_char,
                            le32_to_cpu(*(buf_ptr as *mut u32)),
                        );
                    } else {
                        seq_printf(m, b"%u\0".as_ptr() as *const c_char, *(buf_ptr as *mut u32));
                    }
                }
                x if x == size_of::<u64>() => {
                    if ima_canonical_fmt {
                        seq_printf(
                            m,
                            b"%llu\0".as_ptr() as *const c_char,
                            le64_to_cpu(*(buf_ptr as *mut u64)),
                        );
                    } else {
                        seq_printf(m, b"%llu\0".as_ptr() as *const c_char, *(buf_ptr as *mut u64));
                    }
                }
                _ => {}
            }
        }
    }
}

unsafe fn ima_show_template_data_binary(
    m: *mut seq_file,
    show: ima_show_type,
    _datafmt: data_formats,
    field_data: *mut ima_field_data,
) {
    let len: u32 = if show == ima_show_type::IMA_SHOW_BINARY_OLD_STRING_FMT {
        strlen((*field_data).data as *const c_char) as u32
    } else {
        (*field_data).len
    };

    if show != ima_show_type::IMA_SHOW_BINARY_NO_FIELD_LEN {
        let field_len: u32 = if !ima_canonical_fmt {
            len
        } else {
            cpu_to_le32(len)
        };

        ima_putc(
            m,
            &field_len as *const u32 as *const c_void,
            size_of::<u32>(),
        );
    }

    if len == 0 {
        return;
    }

    ima_putc(m, (*field_data).data, len as usize);
}

unsafe fn ima_show_template_field_data(
    m: *mut seq_file,
    show: ima_show_type,
    datafmt: data_formats,
    field_data: *mut ima_field_data,
) {
    match show {
        ima_show_type::IMA_SHOW_ASCII => {
            ima_show_template_data_ascii(m, show, datafmt, field_data);
        }
        ima_show_type::IMA_SHOW_BINARY
        | ima_show_type::IMA_SHOW_BINARY_NO_FIELD_LEN
        | ima_show_type::IMA_SHOW_BINARY_OLD_STRING_FMT => {
            ima_show_template_data_binary(m, show, datafmt, field_data);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ima_show_template_digest(
    m: *mut seq_file,
    show: ima_show_type,
    field_data: *mut ima_field_data,
) {
    ima_show_template_field_data(m, show, DATA_FMT_DIGEST, field_data);
}

#[no_mangle]
pub unsafe extern "C" fn ima_show_template_digest_ng(
    m: *mut seq_file,
    show: ima_show_type,
    field_data: *mut ima_field_data,
) {
    ima_show_template_field_data(m, show, DATA_FMT_DIGEST_WITH_ALGO, field_data);
}

#[no_mangle]
pub unsafe extern "C" fn ima_show_template_digest_ngv2(
    m: *mut seq_file,
    show: ima_show_type,
    field_data: *mut ima_field_data,
) {
    ima_show_template_field_data(m, show, DATA_FMT_DIGEST_WITH_TYPE_AND_ALGO, field_data);
}

#[no_mangle]
pub unsafe extern "C" fn ima_show_template_string(
    m: *mut seq_file,
    show: ima_show_type,
    field_data: *mut ima_field_data,
) {
    ima_show_template_field_data(m, show, DATA_FMT_STRING, field_data);
}

#[no_mangle]
pub unsafe extern "C" fn ima_show_template_sig(
    m: *mut seq_file,
    show: ima_show_type,
    field_data: *mut ima_field_data,
) {
    ima_show_template_field_data(m, show, DATA_FMT_HEX, field_data);
}

#[no_mangle]
pub unsafe extern "C" fn ima_show_template_buf(
    m: *mut seq_file,
    show: ima_show_type,
    field_data: *mut ima_field_data,
) {
    ima_show_template_field_data(m, show, DATA_FMT_HEX, field_data);
}

#[no_mangle]
pub unsafe extern "C" fn ima_show_template_uint(
    m: *mut seq_file,
    show: ima_show_type,
    field_data: *mut ima_field_data,
) {
    ima_show_template_field_data(m, show, DATA_FMT_UINT, field_data);
}

/**
 * ima_parse_buf() - Parses lengths and data from an input buffer
 * @bufstartp:       Buffer start address.
 * @bufendp:         Buffer end address.
 * @bufcurp:         Pointer to remaining (non-parsed) data.
 * @maxfields:       Length of fields array.
 * @fields:          Array containing lengths and pointers of parsed data.
 * @curfields:       Number of array items containing parsed data.
 * @len_mask:        Bitmap (if bit is set, data length should not be parsed).
 * @enforce_mask:    Check if curfields == maxfields and/or bufcurp == bufendp.
 * @bufname:         String identifier of the input buffer.
 *
 * Return: 0 on success, -EINVAL on error.
 */
#[no_mangle]
pub unsafe extern "C" fn ima_parse_buf(
    bufstartp: *mut c_void,
    bufendp: *mut c_void,
    bufcurp: *mut *mut c_void,
    maxfields: c_int,
    fields: *mut ima_field_data,
    curfields: *mut c_int,
    len_mask: *mut c_ulong,
    enforce_mask: c_int,
    bufname: *mut c_char,
) -> c_int {
    let mut bufp: *mut u8 = bufstartp as *mut u8;
    let mut i: c_int = 0;

    while i < maxfields {
        if len_mask.is_null() || !test_bit(i, len_mask) {
            if bufp > (bufendp as *mut u8).sub(size_of::<u32>()) {
                break;
            }

            if ima_canonical_fmt {
                (*fields.add(i as usize)).len = le32_to_cpu(*(bufp as *mut u32));
            } else {
                (*fields.add(i as usize)).len = *(bufp as *mut u32);
            }

            bufp = bufp.add(size_of::<u32>());
        }

        if bufp > (bufendp as *mut u8).sub((*fields.add(i as usize)).len as usize) {
            break;
        }

        (*fields.add(i as usize)).data = bufp as *mut c_void;
        bufp = bufp.add((*fields.add(i as usize)).len as usize);
        i += 1;
    }

    if (enforce_mask & ENFORCE_FIELDS) != 0 && i != maxfields {
        pr_err(
            b"%s: nr of fields mismatch: expected: %d, current: %d\n\0".as_ptr() as *const c_char,
            bufname,
            maxfields,
            i,
        );
        return -EINVAL;
    }

    if (enforce_mask & ENFORCE_BUFEND) != 0 && bufp != bufendp as *mut u8 {
        pr_err(
            b"%s: buf end mismatch: expected: %p, current: %p\n\0".as_ptr() as *const c_char,
            bufname,
            bufendp,
            bufp,
        );
        return -EINVAL;
    }

    if !curfields.is_null() {
        *curfields = i;
    }

    if !bufcurp.is_null() {
        *bufcurp = bufp as *mut c_void;
    }

    0
}

unsafe fn ima_eventdigest_init_common(
    digest: *const u8,
    digestsize: u32,
    digest_type: u8,
    hash_algo: u8,
    field_data: *mut ima_field_data,
) -> c_int {
    /*
     * digest formats:
     *  - DATA_FMT_DIGEST: digest
     *  - DATA_FMT_DIGEST_WITH_ALGO: <hash algo> + ':' + '\0' + digest,
     *  - DATA_FMT_DIGEST_WITH_TYPE_AND_ALGO:
     *	<digest type> + ':' + <hash algo> + ':' + '\0' + digest,
     *
     *    where 'DATA_FMT_DIGEST' is the original digest format ('d')
     *      with a hash size limitation of 20 bytes,
     *    where <digest type> is either "ima" or "verity",
     *    where <hash algo> is the hash_algo_name[] string.
     */
    let mut buffer: [u8; DIGEST_TYPE_NAME_LEN_MAX + CRYPTO_MAX_ALG_NAME + 2 + IMA_MAX_DIGEST_SIZE] =
        [0; DIGEST_TYPE_NAME_LEN_MAX + CRYPTO_MAX_ALG_NAME + 2 + IMA_MAX_DIGEST_SIZE];
    let mut fmt: data_formats = DATA_FMT_DIGEST;
    let mut offset: u32 = 0;

    if digest_type < DIGEST_TYPE__LAST && (hash_algo as usize) < HASH_ALGO__LAST {
        fmt = DATA_FMT_DIGEST_WITH_TYPE_AND_ALGO;
        offset = offset.wrapping_add(
            1 + sprintf(
                buffer.as_mut_ptr(),
                b"%s:%s:\0".as_ptr() as *const c_char,
                digest_type_name[digest_type as usize],
                hash_algo_name[hash_algo as usize],
            ) as u32,
        );
    } else if (hash_algo as usize) < HASH_ALGO__LAST {
        fmt = DATA_FMT_DIGEST_WITH_ALGO;
        offset = offset.wrapping_add(
            1 + sprintf(
                buffer.as_mut_ptr(),
                b"%s:\0".as_ptr() as *const c_char,
                hash_algo_name[hash_algo as usize],
            ) as u32,
        );
    }

    if !digest.is_null() {
        memcpy(
            buffer.as_mut_ptr().add(offset as usize) as *mut c_void,
            digest as *const c_void,
            digestsize as usize,
        );
    } else {
        /*
         * If digest is NULL, the event being recorded is a violation.
         * Make room for the digest by increasing the offset by the
         * hash algorithm digest size. If the hash algorithm is not
         * specified increase the offset by IMA_DIGEST_SIZE which
         * fits SHA1 or MD5
         */
        if (hash_algo as usize) < HASH_ALGO__LAST {
            offset = offset.wrapping_add(hash_digest_size[hash_algo as usize]);
        } else {
            offset = offset.wrapping_add(IMA_DIGEST_SIZE);
        }
    }

    ima_write_template_field_data(
        buffer.as_ptr() as *const c_void,
        offset.wrapping_add(digestsize),
        fmt,
        field_data,
    )
}

/*
 * This function writes the digest of an event (with size limit).
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventdigest_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    let mut hash: ima_max_digest_data = core::mem::zeroed();
    let hash_hdr: *mut ima_digest_data = &mut hash as *mut ima_max_digest_data as *mut ima_digest_data;
    let mut cur_digest: *mut u8 = ptr::null_mut();
    let mut cur_digestsize: u32 = 0;
    let mut inode: *mut inode;
    let mut result: c_int;

    memset(
        &mut hash as *mut ima_max_digest_data as *mut c_void,
        0,
        size_of::<ima_max_digest_data>(),
    );

    'out: loop {
        if (*event_data).violation {
            /* recording a violation. */
            break 'out;
        }

        if ima_template_hash_algo_allowed((*(*(*event_data).iint).ima_hash).hdr.algo) {
            cur_digest = (*(*(*event_data).iint).ima_hash).digest.as_mut_ptr();
            cur_digestsize = (*(*(*event_data).iint).ima_hash).hdr.length as u32;
            break 'out;
        }

        if (*event_data).filename == boot_aggregate_name
            || (*event_data).filename == boot_aggregate_late_name
        {
            if !ima_tpm_chip.is_null() {
                hash.hdr.algo = HASH_ALGO_SHA1;
                result = ima_calc_boot_aggregate(hash_hdr);

                /* algo can change depending on available PCR banks */
                if result == 0 && hash.hdr.algo != HASH_ALGO_SHA1 {
                    result = -EINVAL;
                }

                if result < 0 {
                    memset(
                        &mut hash as *mut ima_max_digest_data as *mut c_void,
                        0,
                        size_of::<ima_max_digest_data>(),
                    );
                }
            }

            cur_digest = (*hash_hdr).digest.as_mut_ptr();
            cur_digestsize = hash_digest_size[HASH_ALGO_SHA1 as usize];
            break 'out;
        }

        if (*event_data).file.is_null() {
            /* missing info to re-calculate the digest */
            return -EINVAL;
        }

        inode = file_inode((*event_data).file);
        hash.hdr.algo = if ima_template_hash_algo_allowed(ima_hash_algo) {
            ima_hash_algo
        } else {
            HASH_ALGO_SHA1
        };
        result = ima_calc_file_hash((*event_data).file, hash_hdr);
        if result != 0 {
            integrity_audit_msg(
                AUDIT_INTEGRITY_DATA,
                inode,
                (*event_data).filename,
                b"collect_data\0".as_ptr() as *const c_char,
                b"failed\0".as_ptr() as *const c_char,
                result,
                0,
            );
            return result;
        }
        cur_digest = (*hash_hdr).digest.as_mut_ptr();
        cur_digestsize = hash.hdr.length as u32;
        break 'out;
    }
    ima_eventdigest_init_common(
        cur_digest,
        cur_digestsize,
        DIGEST_TYPE__LAST,
        HASH_ALGO__LAST as u8,
        field_data,
    )
}

/*
 * This function writes the digest of an event (without size limit).
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventdigest_ng_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    let mut cur_digest: *mut u8 = ptr::null_mut();
    let mut hash_algo: u8 = ima_hash_algo;
    let mut cur_digestsize: u32 = 0;

    'out: loop {
        if (*event_data).violation {
            /* recording a violation. */
            break 'out;
        }

        cur_digest = (*(*(*event_data).iint).ima_hash).digest.as_mut_ptr();
        cur_digestsize = (*(*(*event_data).iint).ima_hash).hdr.length as u32;

        hash_algo = (*(*(*event_data).iint).ima_hash).hdr.algo;
        break 'out;
    }
    ima_eventdigest_init_common(cur_digest, cur_digestsize, DIGEST_TYPE__LAST, hash_algo, field_data)
}

/*
 * This function writes the digest of an event (without size limit),
 * prefixed with both the digest type and hash algorithm.
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventdigest_ngv2_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    let mut cur_digest: *mut u8 = ptr::null_mut();
    let mut hash_algo: u8 = ima_hash_algo;
    let mut cur_digestsize: u32 = 0;
    let mut digest_type: u8 = DIGEST_TYPE_IMA;

    'out: loop {
        if (*event_data).violation {
            /* recording a violation. */
            break 'out;
        }

        cur_digest = (*(*(*event_data).iint).ima_hash).digest.as_mut_ptr();
        cur_digestsize = (*(*(*event_data).iint).ima_hash).hdr.length as u32;

        hash_algo = (*(*(*event_data).iint).ima_hash).hdr.algo;
        if ((*(*event_data).iint).flags & IMA_VERITY_REQUIRED) != 0 {
            digest_type = DIGEST_TYPE_VERITY;
        }
        break 'out;
    }
    ima_eventdigest_init_common(cur_digest, cur_digestsize, digest_type, hash_algo, field_data)
}

/*
 * This function writes the digest of the file which is expected to match the
 * digest contained in the file's appended signature.
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventdigest_modsig_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    let mut hash_algo: u8;
    let mut cur_digest: *const u8;
    let mut cur_digestsize: u32;

    if (*event_data).modsig.is_null() {
        return 0;
    }

    if (*event_data).violation {
        /* Recording a violation. */
        hash_algo = HASH_ALGO_SHA1;
        cur_digest = ptr::null();
        cur_digestsize = 0;
    } else {
        let rc: c_int;

        rc = ima_get_modsig_digest(
            (*event_data).modsig,
            &mut hash_algo,
            &mut cur_digest,
            &mut cur_digestsize,
        );
        if rc != 0 {
            return rc;
        } else if (hash_algo as usize) == HASH_ALGO__LAST || cur_digestsize == 0 {
            /* There was some error collecting the digest. */
            return -EINVAL;
        }
    }

    ima_eventdigest_init_common(
        cur_digest,
        cur_digestsize,
        DIGEST_TYPE__LAST,
        hash_algo,
        field_data,
    )
}

unsafe fn ima_eventname_init_common(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
    size_limit: bool_t,
) -> c_int {
    let mut cur_filename: *const c_char = ptr::null();
    let mut filename: name_snapshot = core::mem::zeroed();
    let mut cur_filename_len: u32 = 0;
    let mut snapshot: bool_t = false;
    let ret: c_int;

    BUG_ON((*event_data).filename.is_null() && (*event_data).file.is_null());

    'out: loop {
        if !(*event_data).filename.is_null() {
            cur_filename = (*event_data).filename;
            cur_filename_len = strlen((*event_data).filename) as u32;

            if !size_limit || cur_filename_len <= IMA_EVENT_NAME_LEN_MAX {
                break 'out;
            }
        }

        if !(*event_data).file.is_null() {
            take_dentry_name_snapshot(&mut filename, (*(*event_data).file).f_path.dentry);
            snapshot = true;
            cur_filename = filename.name.name;
            cur_filename_len = strlen(cur_filename) as u32;
        } else {
            /*
             * Truncate filename if the latter is too long and
             * the file descriptor is not available.
             */
            cur_filename_len = IMA_EVENT_NAME_LEN_MAX;
        }
        break 'out;
    }
    ret = ima_write_template_field_data(
        cur_filename as *const c_void,
        cur_filename_len,
        DATA_FMT_STRING,
        field_data,
    );

    if snapshot {
        release_dentry_name_snapshot(&mut filename);
    }

    ret
}

/*
 * This function writes the name of an event (with size limit).
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventname_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    ima_eventname_init_common(event_data, field_data, true)
}

/*
 * This function writes the name of an event (without size limit).
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventname_ng_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    ima_eventname_init_common(event_data, field_data, false)
}

/*
 *  ima_eventsig_init - include the file signature as part of the template data
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventsig_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    let xattr_value: *mut evm_ima_xattr_data = (*event_data).xattr_value;

    if xattr_value.is_null()
        || ((*xattr_value).type_ != EVM_IMA_XATTR_DIGSIG
            && (*xattr_value).type_ != IMA_VERITY_DIGSIG)
    {
        return ima_eventevmsig_init(event_data, field_data);
    }

    ima_write_template_field_data(
        xattr_value as *const c_void,
        (*event_data).xattr_len,
        DATA_FMT_HEX,
        field_data,
    )
}

/*
 *  ima_eventbuf_init - include the buffer(kexec-cmldine) as part of the
 *  template data.
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventbuf_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    if (*event_data).buf.is_null() || (*event_data).buf_len == 0 {
        return 0;
    }

    ima_write_template_field_data(
        (*event_data).buf,
        (*event_data).buf_len,
        DATA_FMT_HEX,
        field_data,
    )
}

/*
 *  ima_eventmodsig_init - include the appended file signature as part of the
 *  template data
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventmodsig_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    let mut data: *const c_void = ptr::null();
    let mut data_len: u32 = 0;
    let mut rc: c_int;

    if (*event_data).modsig.is_null() {
        return 0;
    }

    /*
     * modsig is a runtime structure containing pointers. Get its raw data
     * instead.
     */
    rc = ima_get_raw_modsig((*event_data).modsig, &mut data, &mut data_len);
    if rc != 0 {
        return rc;
    }

    ima_write_template_field_data(data, data_len, DATA_FMT_HEX, field_data)
}

/*
 *  ima_eventevmsig_init - include the EVM portable signature as part of the
 *  template data
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventevmsig_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    let mut xattr_data: *mut evm_ima_xattr_data = ptr::null_mut();
    let mut rc: c_int = 0;

    if (*event_data).file.is_null() {
        return 0;
    }

    rc = vfs_getxattr_alloc(
        &nop_mnt_idmap as *const c_void,
        file_dentry((*event_data).file),
        XATTR_NAME_EVM,
        &mut xattr_data as *mut *mut evm_ima_xattr_data as *mut *mut c_char,
        0,
        GFP_NOFS,
    );
    if rc <= 0 || (*xattr_data).type_ != EVM_XATTR_PORTABLE_DIGSIG {
        rc = 0;
    } else {
        rc = ima_write_template_field_data(
            xattr_data as *const c_char as *const c_void,
            rc as u32,
            DATA_FMT_HEX,
            field_data,
        );
    }

    kfree(xattr_data as *mut c_void);
    rc
}

unsafe fn ima_eventinodedac_init_common(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
    get_uid: bool_t,
) -> c_int {
    let mut id: c_uint;

    if (*event_data).file.is_null() {
        return 0;
    }

    if get_uid {
        id = i_uid_read(file_inode((*event_data).file));
    } else {
        id = i_gid_read(file_inode((*event_data).file));
    }

    if ima_canonical_fmt {
        if size_of::<c_uint>() == size_of::<u16>() {
            id = cpu_to_le16(id as u16) as c_uint;
        } else {
            id = cpu_to_le32(id as u32) as c_uint;
        }
    }

    ima_write_template_field_data(
        &id as *const c_uint as *const c_void,
        size_of::<c_uint>() as u32,
        DATA_FMT_UINT,
        field_data,
    )
}

/*
 *  ima_eventinodeuid_init - include the inode UID as part of the template
 *  data
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventinodeuid_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    ima_eventinodedac_init_common(event_data, field_data, true)
}

/*
 *  ima_eventinodegid_init - include the inode GID as part of the template
 *  data
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventinodegid_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    ima_eventinodedac_init_common(event_data, field_data, false)
}

/*
 *  ima_eventinodemode_init - include the inode mode as part of the template
 *  data
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventinodemode_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    let inode: *mut inode;
    let mut mode: u16;

    if (*event_data).file.is_null() {
        return 0;
    }

    inode = file_inode((*event_data).file);
    mode = (*inode).i_mode;
    if ima_canonical_fmt {
        mode = cpu_to_le16(mode);
    }

    ima_write_template_field_data(
        &mode as *const u16 as *const c_void,
        size_of::<u16>() as u32,
        DATA_FMT_UINT,
        field_data,
    )
}

unsafe fn ima_eventinodexattrs_init_common(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
    type_: c_char,
) -> c_int {
    let mut buffer: *mut u8 = ptr::null_mut();
    let mut rc: c_int;

    if (*event_data).file.is_null() {
        return 0;
    }

    rc = evm_read_protected_xattrs(
        file_dentry((*event_data).file),
        ptr::null_mut(),
        0,
        type_,
        ima_canonical_fmt,
    );
    if rc < 0 {
        return 0;
    }

    buffer = kmalloc(rc as usize, GFP_KERNEL) as *mut u8;
    if buffer.is_null() {
        return 0;
    }

    rc = evm_read_protected_xattrs(
        file_dentry((*event_data).file),
        buffer,
        rc,
        type_,
        ima_canonical_fmt,
    );
    if rc < 0 {
        rc = 0;
    } else {
        rc = ima_write_template_field_data(
            buffer as *const c_char as *const c_void,
            rc as u32,
            DATA_FMT_HEX,
            field_data,
        );
    }
    kfree(buffer as *mut c_void);
    rc
}

/*
 *  ima_eventinodexattrnames_init - include a list of xattr names as part of the
 *  template data
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventinodexattrnames_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    ima_eventinodexattrs_init_common(event_data, field_data, b'n' as c_char)
}

/*
 *  ima_eventinodexattrlengths_init - include a list of xattr lengths as part of
 *  the template data
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventinodexattrlengths_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    ima_eventinodexattrs_init_common(event_data, field_data, b'l' as c_char)
}

/*
 *  ima_eventinodexattrvalues_init - include a list of xattr values as part of
 *  the template data
 */
#[no_mangle]
pub unsafe extern "C" fn ima_eventinodexattrvalues_init(
    event_data: *mut ima_event_data,
    field_data: *mut ima_field_data,
) -> c_int {
    ima_eventinodexattrs_init_common(event_data, field_data, b'v' as c_char)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
