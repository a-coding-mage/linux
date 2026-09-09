/* SPDX-License-Identifier: GPL-2.0 */
/*
 * NFS protocol definitions
 *
 * This file contains constants mostly for Version 2 of the protocol,
 * but also has a couple of NFSv3 bits in (notably the error codes).
 */

/* Dependencies supplied by the corresponding kernel headers. */
extern "C" {
    fn memcmp(lhs: *const core::ffi::c_void, rhs: *const core::ffi::c_void, count: usize) -> i32;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, count: usize) -> *mut core::ffi::c_void;
    fn crc32_le(crc: u32, buf: *const u8, len: usize) -> u32;
}

/* The LOCALIO program is entirely private to Linux and is
 * NOT part of the uapi.
 */
pub const NFS_LOCALIO_PROGRAM: i32 = 400122;
pub const LOCALIOPROC_NULL: i32 = 0;
pub const LOCALIOPROC_UUID_IS_LOCAL: i32 = 1;

/*
 * This is the kernel NFS client file handle representation
 */
pub const NFS_MAXFHSIZE: usize = 128;
#[repr(C)]
pub struct nfs_fh {
    pub size: u16,
    pub data: [u8; NFS_MAXFHSIZE],
}

/*
 * Returns a zero iff the size and data fields match.
 * Checks only "size" bytes in the data field.
 */
#[inline]
pub unsafe fn nfs_compare_fh(a: *const nfs_fh, b: *const nfs_fh) -> i32 {
    ((*a).size != (*b).size
        || memcmp(
            (*a).data.as_ptr() as *const core::ffi::c_void,
            (*b).data.as_ptr() as *const core::ffi::c_void,
            (*a).size as usize,
        ) != 0) as i32
}

#[inline]
pub unsafe fn nfs_copy_fh(target: *mut nfs_fh, source: *const nfs_fh) {
    (*target).size = (*source).size;
    memcpy(
        (*target).data.as_mut_ptr() as *mut core::ffi::c_void,
        (*source).data.as_ptr() as *const core::ffi::c_void,
        (*source).size as usize,
    );
}

#[repr(i32)]
pub enum nfs3_stable_how {
    NFS_UNSTABLE = 0,
    NFS_DATA_SYNC = 1,
    NFS_FILE_SYNC = 2,

    /* used by direct.c to mark verf as invalid */
    NFS_INVALID_STABLE_HOW = -1,
}

/**
 * nfs_fhandle_hash - calculate the crc32 hash for the filehandle
 * @fh - pointer to filehandle
 *
 * returns a crc32 hash for the filehandle that is compatible with
 * the one displayed by "wireshark".
 */
#[inline]
pub unsafe fn nfs_fhandle_hash(fh: *const nfs_fh) -> u32 {
    !crc32_le(0xFFFFFFFF, (*fh).data.as_ptr(), (*fh).size as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
