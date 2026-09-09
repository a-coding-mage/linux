/* SPDX-License-Identifier: GPL-2.0 */
/* Ceph fscrypt functionality */

// Dependencies supplied by the surrounding kernel/Ceph translation are intentionally
// referenced but not implemented here. The CONFIG_FS_ENCRYPTION branches preserve
// the source build-time condition.

pub const CEPH_FSCRYPT_BLOCK_SHIFT: u32 = 12;
pub const CEPH_FSCRYPT_BLOCK_SIZE: u64 = 1u64 << CEPH_FSCRYPT_BLOCK_SHIFT;
pub const CEPH_FSCRYPT_BLOCK_MASK: u64 = !(CEPH_FSCRYPT_BLOCK_SIZE - 1);

#[repr(C)] pub struct ceph_fs_client { _private: [u8; 0] }
#[repr(C)] pub struct ceph_acl_sec_ctx { _private: [u8; 0] }
#[repr(C)] pub struct ceph_mds_request { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct ceph_sparse_extent { _private: [u8; 0] }
#[repr(C)] pub struct fscrypt_str { pub name: *mut u8, pub len: u32 }

#[repr(C)]
pub struct ceph_fname {
    pub dir: *mut inode,
    pub name: *mut i8, // b64 encoded, possibly hashed
    pub ctext: *mut u8, // binary crypttext (if any)
    pub name_len: u32, // length of name buffer
    pub ctext_len: u32, // length of crypttext
    pub no_copy: bool,
}

#[repr(C, packed)]
pub struct ceph_fscrypt_truncate_size_header {
    pub ver: u8,
    pub compat: u8,
    // Data length, change attribute, file offset, and block size are serialized
    // little-endian, matching the C __le types.
    pub data_len: u32,
    pub change_attr: u64,
    pub file_offset: u64,
    pub block_size: u32,
}

#[repr(C, packed)]
pub struct ceph_fscrypt_auth {
    pub cfa_version: u32,
    pub cfa_blob_len: u32,
    pub cfa_blob: [u8; FSCRYPT_SET_CONTEXT_MAX_SIZE],
}

pub const CEPH_FSCRYPT_AUTH_VERSION: u32 = 1;

pub unsafe fn ceph_fscrypt_auth_len(fa: *mut ceph_fscrypt_auth) -> usize {
    std::mem::offset_of!(ceph_fscrypt_auth, cfa_blob)
        + u32::from_le((*fa).cfa_blob_len) as usize
}

pub const CEPH_NOHASH_NAME_MAX: usize = 180 - SHA256_DIGEST_SIZE;

#[cfg(feature = "CONFIG_FS_ENCRYPTION")]
extern "C" {
    pub fn ceph_fscrypt_set_ops(sb: *mut super_block);
    pub fn ceph_fscrypt_free_dummy_policy(fsc: *mut ceph_fs_client);
    pub fn ceph_fscrypt_prepare_context(dir: *mut inode, inode: *mut inode, as_: *mut ceph_acl_sec_ctx) -> i32;
    pub fn ceph_fscrypt_as_ctx_to_req(req: *mut ceph_mds_request, as_: *mut ceph_acl_sec_ctx);
    pub fn ceph_encode_encrypted_dname(parent: *mut inode, buf: *mut i8, len: i32) -> i32;
    pub fn ceph_fname_to_usr(fname: *const ceph_fname, tname: *mut u8, oname: *mut fscrypt_str, is_nokey: *mut bool) -> i32;
    pub fn ceph_fscrypt_prepare_readdir(dir: *mut inode) -> i32;
    pub fn ceph_fscrypt_decrypt_block_inplace(inode: *const inode, page: *mut page, len: u32, offs: u32, lblk_num: u64) -> i32;
    pub fn ceph_fscrypt_encrypt_block_inplace(inode: *const inode, page: *mut page, len: u32, offs: u32, lblk_num: u64) -> i32;
    pub fn ceph_fscrypt_decrypt_pages(inode: *mut inode, page: *mut *mut page, off: u64, len: i32) -> i32;
    pub fn ceph_fscrypt_decrypt_extents(inode: *mut inode, page: *mut *mut page, off: u64, map: *mut ceph_sparse_extent, ext_cnt: u32) -> i32;
    pub fn ceph_fscrypt_encrypt_pages(inode: *mut inode, page: *mut *mut page, off: u64, len: i32) -> i32;
}

#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fscrypt_set_ops(_sb: *mut super_block) {}
#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fscrypt_free_dummy_policy(_fsc: *mut ceph_fs_client) {}
#[cfg(feature = "CONFIG_FS_ENCRYPTION")]
pub unsafe fn ceph_fname_alloc_buffer(parent: *mut inode, fname: *mut fscrypt_str) -> i32 {
    if !IS_ENCRYPTED(parent) { 0 } else { fscrypt_fname_alloc_buffer(NAME_MAX, fname) }
}
#[cfg(feature = "CONFIG_FS_ENCRYPTION")]
pub unsafe fn ceph_fname_free_buffer(parent: *mut inode, fname: *mut fscrypt_str) {
    if IS_ENCRYPTED(parent) { fscrypt_fname_free_buffer(fname); }
}
#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fscrypt_prepare_context(dir: *mut inode, _inode: *mut inode, _as_: *mut ceph_acl_sec_ctx) -> i32 { if IS_ENCRYPTED(dir) { -EOPNOTSUPP } else { 0 } }
#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fscrypt_as_ctx_to_req(_req: *mut ceph_mds_request, _as_ctx: *mut ceph_acl_sec_ctx) {}
#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_encode_encrypted_dname(_parent: *mut inode, _buf: *mut i8, len: i32) -> i32 { len }
#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fname_alloc_buffer(_parent: *mut inode, _fname: *mut fscrypt_str) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fname_free_buffer(_parent: *mut inode, _fname: *mut fscrypt_str) {}
#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fname_to_usr(fname: *const ceph_fname, _tname: *mut u8, oname: *mut fscrypt_str, _is_nokey: *mut bool) -> i32 { (*oname).name = (*fname).name as *mut u8; (*oname).len = (*fname).name_len; 0 }
#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fscrypt_prepare_readdir(_dir: *mut inode) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fscrypt_adjust_off_and_len(_inode: *mut inode, _off: *mut u64, _len: *mut u64) {}
#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fscrypt_decrypt_block_inplace(_inode: *const inode, _page: *mut page, _len: u32, _offs: u32, _lblk_num: u64) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fscrypt_encrypt_block_inplace(_inode: *const inode, _page: *mut page, _len: u32, _offs: u32, _lblk_num: u64) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fscrypt_decrypt_pages(_inode: *mut inode, _page: *mut *mut page, _off: u64, _len: i32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fscrypt_decrypt_extents(_inode: *mut inode, _page: *mut *mut page, _off: u64, _map: *mut ceph_sparse_extent, _ext_cnt: u32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fscrypt_encrypt_pages(_inode: *mut inode, _page: *mut *mut page, _off: u64, _len: i32) -> i32 { 0 }

pub unsafe fn ceph_fscrypt_blocks(off: u64, len: u64) -> u32 {
    ((off.wrapping_add(len).wrapping_add(CEPH_FSCRYPT_BLOCK_SIZE - 1) >> CEPH_FSCRYPT_BLOCK_SHIFT)
        - (off >> CEPH_FSCRYPT_BLOCK_SHIFT)) as u32
}

#[cfg(feature = "CONFIG_FS_ENCRYPTION")]
pub unsafe fn ceph_fscrypt_adjust_off_and_len(inode: *mut inode, off: *mut u64, len: *mut u64) {
    if IS_ENCRYPTED(inode) {
        *len = ceph_fscrypt_blocks(*off, *len) as u64 * CEPH_FSCRYPT_BLOCK_SIZE;
        *off &= CEPH_FSCRYPT_BLOCK_MASK;
    }
}

#[cfg(feature = "CONFIG_FS_ENCRYPTION")]
pub unsafe fn ceph_fscrypt_pagecache_page(p: *mut page) -> *mut page {
    if fscrypt_is_bounce_page(p) { fscrypt_pagecache_page(p) } else { p }
}

#[cfg(not(feature = "CONFIG_FS_ENCRYPTION"))]
pub unsafe fn ceph_fscrypt_pagecache_page(page: *mut page) -> *mut page { page }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
