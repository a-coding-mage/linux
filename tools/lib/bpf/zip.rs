// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/*
 * Routines for dealing with .zip archives.
 *
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr;

type __u16 = u16;
type __u32 = u32;
type off_t = c_long;
type size_t = usize;

const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const ENOTSUP: c_int = 95;

const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2000000;
const SEEK_END: c_int = 2;
const PROT_READ: c_int = 0x1;
const MAP_PRIVATE: c_int = 0x02;
const UINT32_MAX: off_t = u32::MAX as off_t;

unsafe extern "C" {
    static mut errno: c_int;

    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> size_t;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;

    fn ERR_PTR(error: c_long) -> *mut c_void;
}

/* Specification of ZIP file format can be found here:
 * https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT
 * For a high level overview of the structure of a ZIP file see
 * sections 4.3.1 - 4.3.6.
 *
 * Data structures appearing in ZIP files do not contain any
 * padding and they might be misaligned. To allow us to safely
 * operate on pointers to such structures and their members, we
 * declare the types as packed.
 */

const END_OF_CD_RECORD_MAGIC: __u32 = 0x06054b50;

/* See section 4.3.16 of the spec. */
#[repr(C, packed)]
struct end_of_cd_record {
    /* Magic value equal to END_OF_CD_RECORD_MAGIC */
    magic: __u32,

    /* Number of the file containing this structure or 0xFFFF if ZIP64 archive.
     * Zip archive might span multiple files (disks).
     */
    this_disk: __u16,

    /* Number of the file containing the beginning of the central directory or
     * 0xFFFF if ZIP64 archive.
     */
    cd_disk: __u16,

    /* Number of central directory records on this disk or 0xFFFF if ZIP64
     * archive.
     */
    cd_records: __u16,

    /* Number of central directory records on all disks or 0xFFFF if ZIP64
     * archive.
     */
    cd_records_total: __u16,

    /* Size of the central directory record or 0xFFFFFFFF if ZIP64 archive. */
    cd_size: __u32,

    /* Offset of the central directory from the beginning of the archive or
     * 0xFFFFFFFF if ZIP64 archive.
     */
    cd_offset: __u32,

    /* Length of comment data following end of central directory record. */
    comment_length: __u16,

    /* Up to 64k of arbitrary bytes. */
    /* uint8_t comment[comment_length] */
}

const CD_FILE_HEADER_MAGIC: __u32 = 0x02014b50;
const FLAG_ENCRYPTED: __u16 = 1 << 0;
const FLAG_HAS_DATA_DESCRIPTOR: __u16 = 1 << 3;

/* See section 4.3.12 of the spec. */
#[repr(C, packed)]
struct cd_file_header {
    /* Magic value equal to CD_FILE_HEADER_MAGIC. */
    magic: __u32,
    version: __u16,
    /* Minimum zip version needed to extract the file. */
    min_version: __u16,
    flags: __u16,
    compression: __u16,
    last_modified_time: __u16,
    last_modified_date: __u16,
    crc: __u32,
    compressed_size: __u32,
    uncompressed_size: __u32,
    file_name_length: __u16,
    extra_field_length: __u16,
    file_comment_length: __u16,
    /* Number of the disk where the file starts or 0xFFFF if ZIP64 archive. */
    disk: __u16,
    internal_attributes: __u16,
    external_attributes: __u32,
    /* Offset from the start of the disk containing the local file header to the
     * start of the local file header.
     */
    offset: __u32,
}

const LOCAL_FILE_HEADER_MAGIC: __u32 = 0x04034b50;

/* See section 4.3.7 of the spec. */
#[repr(C, packed)]
struct local_file_header {
    /* Magic value equal to LOCAL_FILE_HEADER_MAGIC. */
    magic: __u32,
    /* Minimum zip version needed to extract the file. */
    min_version: __u16,
    flags: __u16,
    compression: __u16,
    last_modified_time: __u16,
    last_modified_date: __u16,
    crc: __u32,
    compressed_size: __u32,
    uncompressed_size: __u32,
    file_name_length: __u16,
    extra_field_length: __u16,
}

#[repr(C)]
pub struct zip_archive {
    data: *mut c_void,
    size: __u32,
    cd_offset: __u32,
    cd_records: __u32,
}

#[repr(C)]
pub struct zip_entry {
    pub name: *const c_char,
    pub name_length: __u32,
    pub data: *mut c_void,
    pub data_length: __u32,
    pub data_offset: __u32,
    pub compression: __u16,
}

unsafe fn check_access(archive: *mut zip_archive, offset: __u32, size: __u32) -> *mut c_void {
    if offset.wrapping_add(size) > unsafe { (*archive).size } || offset > offset.wrapping_add(size)
    {
        return ptr::null_mut();
    }

    unsafe { ((*archive).data as *mut u8).add(offset as usize) as *mut c_void }
}

/* Returns 0 on success, -EINVAL on error and -ENOTSUP if the eocd indicates the
 * archive uses features which are not supported.
 */
unsafe fn try_parse_end_of_cd(archive: *mut zip_archive, offset: __u32) -> c_int {
    let comment_length: __u16;
    let cd_records: __u16;
    let eocd: *mut end_of_cd_record;
    let cd_offset: __u32;
    let cd_size: __u32;

    eocd = unsafe {
        check_access(
            archive,
            offset,
            size_of::<end_of_cd_record>() as __u32,
        ) as *mut end_of_cd_record
    };
    if eocd.is_null() || unsafe { (*eocd).magic } != END_OF_CD_RECORD_MAGIC {
        return -EINVAL;
    }

    comment_length = unsafe { (*eocd).comment_length };
    if offset
        .wrapping_add(size_of::<end_of_cd_record>() as __u32)
        .wrapping_add(comment_length as __u32)
        != unsafe { (*archive).size }
    {
        return -EINVAL;
    }

    cd_records = unsafe { (*eocd).cd_records };
    if unsafe { (*eocd).this_disk } != 0
        || unsafe { (*eocd).cd_disk } != 0
        || unsafe { (*eocd).cd_records_total } != cd_records
    {
        /* This is a valid eocd, but we only support single-file non-ZIP64 archives. */
        return -ENOTSUP;
    }

    cd_offset = unsafe { (*eocd).cd_offset };
    cd_size = unsafe { (*eocd).cd_size };
    if unsafe { check_access(archive, cd_offset, cd_size) }.is_null() {
        return -EINVAL;
    }

    unsafe {
        (*archive).cd_offset = cd_offset;
        (*archive).cd_records = cd_records as __u32;
    }
    0
}

unsafe fn find_cd(archive: *mut zip_archive) -> c_int {
    let limit: i64;
    let mut offset: i64;
    let mut rc: c_int = -EINVAL;

    if unsafe { (*archive).size } <= size_of::<end_of_cd_record>() as __u32 {
        return -EINVAL;
    }

    /* Because the end of central directory ends with a variable length array of
     * up to 0xFFFF bytes we can't know exactly where it starts and need to
     * search for it at the end of the file, scanning the (limit, offset] range.
     */
    offset = (unsafe { (*archive).size } - size_of::<end_of_cd_record>() as __u32) as i64;
    limit = offset - (1 << 16);

    while offset >= 0 && offset > limit && rc != 0 {
        rc = unsafe { try_parse_end_of_cd(archive, offset as __u32) };
        if rc == -ENOTSUP {
            break;
        }
        offset -= 1;
    }
    rc
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zip_archive_open(path: *const c_char) -> *mut zip_archive {
    let archive: *mut zip_archive;
    let mut err: c_int;
    let fd: c_int;
    let size: off_t;
    let data: *mut c_void;

    fd = unsafe { open(path, O_RDONLY | O_CLOEXEC) };
    if fd < 0 {
        return unsafe { ERR_PTR(-(errno as c_long)) as *mut zip_archive };
    }

    size = unsafe { lseek(fd, 0, SEEK_END) };
    if size == -1 as off_t || size > UINT32_MAX {
        unsafe { close(fd) };
        return unsafe { ERR_PTR(-(EINVAL as c_long)) as *mut zip_archive };
    }

    data = unsafe { mmap(ptr::null_mut(), size as size_t, PROT_READ, MAP_PRIVATE, fd, 0) };
    err = unsafe { -errno };
    unsafe { close(fd) };

    if data == (-1isize) as *mut c_void {
        return unsafe { ERR_PTR(err as c_long) as *mut zip_archive };
    }

    archive = unsafe { malloc(size_of::<zip_archive>()) as *mut zip_archive };
    if archive.is_null() {
        unsafe { munmap(data, size as size_t) };
        return unsafe { ERR_PTR(-(ENOMEM as c_long)) as *mut zip_archive };
    }

    unsafe {
        (*archive).data = data;
        (*archive).size = size as __u32;
    }

    err = unsafe { find_cd(archive) };
    if err != 0 {
        unsafe {
            munmap(data, size as size_t);
            free(archive as *mut c_void);
        }
        return unsafe { ERR_PTR(err as c_long) as *mut zip_archive };
    }

    archive
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zip_archive_close(archive: *mut zip_archive) {
    unsafe {
        munmap((*archive).data, (*archive).size as size_t);
        free(archive as *mut c_void);
    }
}

unsafe fn local_file_header_at_offset(
    archive: *mut zip_archive,
    offset: __u32,
) -> *mut local_file_header {
    let lfh: *mut local_file_header;

    lfh = unsafe {
        check_access(
            archive,
            offset,
            size_of::<local_file_header>() as __u32,
        ) as *mut local_file_header
    };
    if lfh.is_null() || unsafe { (*lfh).magic } != LOCAL_FILE_HEADER_MAGIC {
        return ptr::null_mut();
    }

    lfh
}

unsafe fn get_entry_at_offset(
    archive: *mut zip_archive,
    mut offset: __u32,
    out: *mut zip_entry,
) -> c_int {
    let lfh: *mut local_file_header;
    let compressed_size: __u32;
    let name: *const c_char;
    let data: *mut c_void;

    lfh = unsafe { local_file_header_at_offset(archive, offset) };
    if lfh.is_null() {
        return -EINVAL;
    }

    offset = offset.wrapping_add(size_of::<local_file_header>() as __u32);
    if (unsafe { (*lfh).flags } & FLAG_ENCRYPTED) != 0
        || (unsafe { (*lfh).flags } & FLAG_HAS_DATA_DESCRIPTOR) != 0
    {
        return -EINVAL;
    }

    name = unsafe { check_access(archive, offset, (*lfh).file_name_length as __u32) as *const c_char };
    if name.is_null() {
        return -EINVAL;
    }

    offset = offset.wrapping_add(unsafe { (*lfh).file_name_length } as __u32);
    if unsafe { check_access(archive, offset, (*lfh).extra_field_length as __u32) }.is_null() {
        return -EINVAL;
    }

    offset = offset.wrapping_add(unsafe { (*lfh).extra_field_length } as __u32);
    compressed_size = unsafe { (*lfh).compressed_size };
    data = unsafe { check_access(archive, offset, compressed_size) };
    if data.is_null() {
        return -EINVAL;
    }

    unsafe {
        (*out).compression = (*lfh).compression;
        (*out).name_length = (*lfh).file_name_length as __u32;
        (*out).name = name;
        (*out).data = data;
        (*out).data_length = compressed_size;
        (*out).data_offset = offset;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zip_archive_find_entry(
    archive: *mut zip_archive,
    file_name: *const c_char,
    out: *mut zip_entry,
) -> c_int {
    let file_name_length: size_t = unsafe { strlen(file_name) };
    let mut i: __u32;
    let mut offset: __u32 = unsafe { (*archive).cd_offset };

    i = 0;
    while i < unsafe { (*archive).cd_records } {
        let cdfh_name_length: __u16;
        let cdfh_flags: __u16;
        let cdfh: *mut cd_file_header;
        let cdfh_name: *const c_char;

        cdfh = unsafe {
            check_access(
                archive,
                offset,
                size_of::<cd_file_header>() as __u32,
            ) as *mut cd_file_header
        };
        if cdfh.is_null() || unsafe { (*cdfh).magic } != CD_FILE_HEADER_MAGIC {
            return -EINVAL;
        }

        offset = offset.wrapping_add(size_of::<cd_file_header>() as __u32);
        cdfh_name_length = unsafe { (*cdfh).file_name_length };
        cdfh_name =
            unsafe { check_access(archive, offset, cdfh_name_length as __u32) as *const c_char };
        if cdfh_name.is_null() {
            return -EINVAL;
        }

        cdfh_flags = unsafe { (*cdfh).flags };
        if (cdfh_flags & FLAG_ENCRYPTED) == 0
            && (cdfh_flags & FLAG_HAS_DATA_DESCRIPTOR) == 0
            && file_name_length == cdfh_name_length as size_t
            && unsafe {
                memcmp(
                    file_name as *const c_void,
                    (unsafe { (*archive).data } as *mut u8).add(offset as usize) as *const c_void,
                    file_name_length,
                )
            } == 0
        {
            return unsafe { get_entry_at_offset(archive, (*cdfh).offset, out) };
        }

        offset = offset.wrapping_add(cdfh_name_length as __u32);
        offset = offset.wrapping_add(unsafe { (*cdfh).extra_field_length } as __u32);
        offset = offset.wrapping_add(unsafe { (*cdfh).file_comment_length } as __u32);
        i = i.wrapping_add(1);
    }

    -ENOENT
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
