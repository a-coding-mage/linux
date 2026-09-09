/* SPDX-License-Identifier: GPL-2.0 */
/*
 * fs-verity: read-only file-based authenticity protection
 *
 * This header declares the interface between the fs/verity/ support layer and
 * filesystems that support fs-verity.
 *
 * Copyright 2019 Google LLC
 */

/* Dependencies supplied by the kernel environment are intentionally external. */

/* Largest digest size among all hash algorithms supported by fs-verity. */
pub const FS_VERITY_MAX_DIGEST_SIZE: usize = SHA512_DIGEST_SIZE;

/* Arbitrary limit to bound the kmalloc() size.  Can be changed. */
pub const FS_VERITY_MAX_DESCRIPTOR_SIZE: usize = 16384;

pub struct fsverity_info {
    _private: [u8; 0],
}

/* Verity operations for filesystems */
#[repr(C)]
pub struct fsverity_operations {
    pub begin_enable_verity:
        Option<unsafe extern "C" fn(filp: *mut file) -> i32>,
    pub end_enable_verity: Option<unsafe extern "C" fn(
        filp: *mut file,
        desc: *const core::ffi::c_void,
        desc_size: usize,
        merkle_tree_size: u64,
    ) -> i32>,
    pub get_verity_descriptor: Option<unsafe extern "C" fn(
        inode: *mut inode,
        buf: *mut core::ffi::c_void,
        bufsize: usize,
    ) -> i32>,
    pub read_merkle_tree_page:
        Option<unsafe extern "C" fn(inode: *mut inode, index: pgoff_t) -> *mut page>,
    pub readahead_merkle_tree:
        Option<unsafe extern "C" fn(inode: *mut inode, index: pgoff_t, nr_pages: c_ulong)>,
    pub write_merkle_tree_block: Option<unsafe extern "C" fn(
        file: *mut file,
        buf: *const core::ffi::c_void,
        pos: u64,
        size: c_uint,
    ) -> i32>,
}

#[cfg(CONFIG_FS_VERITY)]
#[inline]
pub unsafe fn fsverity_active(inode: *const inode) -> bool {
    if IS_VERITY(inode) {
        /* This pairs with the try_cmpxchg in set_mask_bits() used for S_VERITY. */
        smp_mb();
        return true;
    }
    false
}

#[cfg(CONFIG_FS_VERITY)]
unsafe extern "C" {
    pub fn __fsverity_get_info(inode: *const inode) -> *mut fsverity_info;
}

#[cfg(CONFIG_FS_VERITY)]
#[inline]
pub unsafe fn fsverity_get_info(inode: *const inode) -> *mut fsverity_info {
    if !fsverity_active(inode) {
        return core::ptr::null_mut();
    }
    __fsverity_get_info(inode)
}

#[cfg(CONFIG_FS_VERITY)]
unsafe extern "C" {
    pub fn fsverity_ioctl_enable(filp: *mut file, arg: *const core::ffi::c_void) -> i32;
    pub fn fsverity_ioctl_measure(filp: *mut file, arg: *mut core::ffi::c_void) -> i32;
    pub fn fsverity_get_digest(
        inode: *mut inode,
        raw_digest: *mut u8,
        alg: *mut u8,
        halg: *mut hash_algo,
    ) -> i32;
    pub fn __fsverity_file_open(inode: *mut inode, filp: *mut file) -> i32;
    pub fn fsverity_ioctl_read_metadata(
        filp: *mut file,
        uarg: *const core::ffi::c_void,
    ) -> i32;
    pub fn fsverity_readahead(
        vi: *mut fsverity_info,
        index: pgoff_t,
        nr_pages: c_ulong,
    );
    pub fn fsverity_verify_blocks(
        vi: *mut fsverity_info,
        folio: *mut folio,
        len: usize,
        offset: usize,
    ) -> bool;
    pub fn fsverity_verify_bio(vi: *mut fsverity_info, bio: *mut bio);
    pub fn fsverity_enqueue_verify_work(work: *mut work_struct);
    pub fn fsverity_fill_zerohash(
        folio: *mut folio,
        offset: usize,
        len: usize,
        vi: *mut fsverity_info,
    );
}

#[cfg(not(CONFIG_FS_VERITY))]
#[inline]
pub unsafe fn fsverity_active(_inode: *const inode) -> bool { false }

#[cfg(not(CONFIG_FS_VERITY))]
#[inline]
pub unsafe fn fsverity_get_info(_inode: *const inode) -> *mut fsverity_info {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_FS_VERITY))]
#[inline]
pub unsafe fn fsverity_ioctl_enable(_filp: *mut file, _arg: *const core::ffi::c_void) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(CONFIG_FS_VERITY))]
#[inline]
pub unsafe fn fsverity_ioctl_measure(_filp: *mut file, _arg: *mut core::ffi::c_void) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(CONFIG_FS_VERITY))]
#[inline]
pub unsafe fn fsverity_get_digest(
    _inode: *mut inode,
    _raw_digest: *mut u8,
    _alg: *mut u8,
    _halg: *mut hash_algo,
) -> i32 { 0 }

#[cfg(not(CONFIG_FS_VERITY))]
#[inline]
pub unsafe fn __fsverity_file_open(_inode: *mut inode, _filp: *mut file) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(CONFIG_FS_VERITY))]
#[inline]
pub unsafe fn fsverity_ioctl_read_metadata(
    _filp: *mut file,
    _uarg: *const core::ffi::c_void,
) -> i32 { -EOPNOTSUPP }

#[cfg(not(CONFIG_FS_VERITY))]
#[inline]
pub unsafe fn fsverity_readahead(_vi: *mut fsverity_info, _index: pgoff_t, _nr_pages: c_ulong) {}

#[cfg(not(CONFIG_FS_VERITY))]
#[inline]
pub unsafe fn fsverity_verify_blocks(
    _vi: *mut fsverity_info, _folio: *mut folio, _len: usize, _offset: usize,
) -> bool {
    WARN_ON_ONCE(1);
    false
}

#[cfg(not(CONFIG_FS_VERITY))]
#[inline]
pub unsafe fn fsverity_verify_bio(_vi: *mut fsverity_info, _bio: *mut bio) { WARN_ON_ONCE(1); }

#[cfg(not(CONFIG_FS_VERITY))]
#[inline]
pub unsafe fn fsverity_enqueue_verify_work(_work: *mut work_struct) { WARN_ON_ONCE(1); }

#[cfg(not(CONFIG_FS_VERITY))]
#[inline]
pub unsafe fn fsverity_fill_zerohash(
    _folio: *mut folio, _offset: usize, _len: usize, _vi: *mut fsverity_info,
) { WARN_ON_ONCE(1); }

#[inline]
pub unsafe fn fsverity_verify_folio(vi: *mut fsverity_info, folio: *mut folio) -> bool {
    fsverity_verify_blocks(vi, folio, folio_size(folio), 0)
}

#[inline]
pub unsafe fn fsverity_file_open(inode: *mut inode, filp: *mut file) -> i32 {
    if IS_VERITY(inode) { return __fsverity_file_open(inode, filp); }
    0
}

unsafe extern "C" {
    pub fn fsverity_cleanup_inode(inode: *mut inode);
    pub fn generic_read_merkle_tree_page(inode: *mut inode, index: pgoff_t) -> *mut page;
    pub fn generic_readahead_merkle_tree(inode: *mut inode, index: pgoff_t, nr_pages: c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
