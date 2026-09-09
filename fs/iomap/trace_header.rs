/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2009-2021 Christoph Hellwig
 *
 * NOTE: none of these tracepoints shall be considered a stable kernel ABI
 * as they can change at any time.
 *
 * Current conventions for printing numbers measuring specific units:
 *
 * offset: byte offset into a subcomponent of a file operation
 * pos: file offset, in bytes
 * length: length of a file operation, in bytes
 * ino: inode number
 *
 * Numbers describing space allocations should be formatted in hexadecimal.
 */

// C dependency: linux/tracepoint.h and the kernel types/functions referenced below.
// The original TRACE_SYSTEM is "iomap".

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iomap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iomap_iter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kiocb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iov_iter {
    _private: [u8; 0],
}

// These arrays preserve the C tracepoint symbolic-name macro contents.
pub const IOMAP_TYPE_STRINGS: &[(&str, &str)] = &[
    ("IOMAP_HOLE", "HOLE"),
    ("IOMAP_DELALLOC", "DELALLOC"),
    ("IOMAP_MAPPED", "MAPPED"),
    ("IOMAP_UNWRITTEN", "UNWRITTEN"),
    ("IOMAP_INLINE", "INLINE"),
];

pub const IOMAP_FLAGS_STRINGS: &[(&str, &str)] = &[
    ("IOMAP_WRITE", "WRITE"),
    ("IOMAP_ZERO", "ZERO"),
    ("IOMAP_REPORT", "REPORT"),
    ("IOMAP_FAULT", "FAULT"),
    ("IOMAP_DIRECT", "DIRECT"),
    ("IOMAP_NOWAIT", "NOWAIT"),
    ("IOMAP_OVERWRITE_ONLY", "OVERWRITE_ONLY"),
    ("IOMAP_UNSHARE", "UNSHARE"),
    ("IOMAP_DAX", "DAX"),
    ("IOMAP_ATOMIC", "ATOMIC"),
    ("IOMAP_DONTCACHE", "DONTCACHE"),
];

pub const IOMAP_F_FLAGS_STRINGS: &[(&str, &str)] = &[
    ("IOMAP_F_NEW", "NEW"),
    ("IOMAP_F_DIRTY", "DIRTY"),
    ("IOMAP_F_SHARED", "SHARED"),
    ("IOMAP_F_MERGED", "MERGED"),
    ("IOMAP_F_BUFFER_HEAD", "BH"),
    ("IOMAP_F_XATTR", "XATTR"),
    ("IOMAP_F_BOUNDARY", "BOUNDARY"),
    ("IOMAP_F_ANON_WRITE", "ANON_WRITE"),
    ("IOMAP_F_ATOMIC_BIO", "ATOMIC_BIO"),
    ("IOMAP_F_PRIVATE", "PRIVATE"),
    ("IOMAP_F_SIZE_CHANGED", "SIZE_CHANGED"),
    ("IOMAP_F_STALE", "STALE"),
    ("IOMAP_F_FSVERITY", "FSVERITY"),
    ("IOMAP_F_ZERO_TAIL", "ZERO TAIL"),
];

pub const IOMAP_DIO_STRINGS: &[(&str, &str)] = &[
    ("IOMAP_DIO_FORCE_WAIT", "DIO_FORCE_WAIT"),
    ("IOMAP_DIO_OVERWRITE_ONLY", "DIO_OVERWRITE_ONLY"),
    ("IOMAP_DIO_PARTIAL", "DIO_PARTIAL"),
    ("IOMAP_DIO_FSBLOCK_ALIGNED", "DIO_FSBLOCK_ALIGNED"),
];

// C tracepoint declarations. Their implementations and registration are supplied externally.
extern "C" {
    pub fn iomap_readpage(inode: *mut inode, nr_pages: core::ffi::c_int);
    pub fn iomap_readahead(inode: *mut inode, nr_pages: core::ffi::c_int);
    pub fn iomap_writeback_folio(inode: *mut inode, off: i64, len: u64);
    pub fn iomap_release_folio(inode: *mut inode, off: i64, len: u64);
    pub fn iomap_invalidate_folio(inode: *mut inode, off: i64, len: u64);
    pub fn iomap_dio_invalidate_fail(inode: *mut inode, off: i64, len: u64);
    pub fn iomap_dio_rw_queued(inode: *mut inode, off: i64, len: u64);
    pub fn iomap_zero_iter(inode: *mut inode, off: i64, len: u64);
    pub fn iomap_iter_dstmap(inode: *mut inode, iomap: *mut iomap);
    pub fn iomap_iter_srcmap(inode: *mut inode, iomap: *mut iomap);
    pub fn iomap_add_to_ioend(
        inode: *mut inode,
        pos: u64,
        dirty_len: core::ffi::c_uint,
        iomap: *mut iomap,
    );
    pub fn iomap_iter(iter: *mut iomap_iter, ops: *const core::ffi::c_void, caller: usize);
    pub fn iomap_dio_rw_begin(
        iocb: *mut kiocb,
        iter: *mut iov_iter,
        dio_flags: core::ffi::c_uint,
        done_before: usize,
    );
    pub fn iomap_dio_complete(iocb: *mut kiocb, error: core::ffi::c_int, ret: isize);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
