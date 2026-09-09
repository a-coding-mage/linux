// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * eCryptfs: Linux filesystem encryption layer
 *
 * Rust source-level translation of crypto.c. Kernel types and helper symbols
 * are supplied by the surrounding eCryptfs kernel bindings.
 */

const DECRYPT: i32 = 0;
const ENCRYPT: i32 = 1;

// External kernel/eCryptfs declarations are intentionally left unresolved;
// they are provided by the translated headers and other compilation units.
extern "C" {
    fn kasprintf(flags: u32, fmt: *const i8, ...) -> *mut i8;
    fn kfree(p: *mut core::ffi::c_void);
    fn simple_strtol(s: *const i8, end: *mut *mut i8, base: u32) -> i64;
    fn md5(src: *const i8, len: usize, dst: *mut i8);
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn memset(dst: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
}

#[no_mangle]
pub unsafe extern "C" fn ecryptfs_from_hex(dst: *mut i8, src: *mut i8, dst_size: i32) {
    let mut x = 0;
    let mut tmp = [0i8; 3];
    while x < dst_size {
        tmp[0] = *src.add((x * 2) as usize);
        tmp[1] = *src.add((x * 2 + 1) as usize);
        *dst.add(x as usize) = simple_strtol(tmp.as_ptr(), core::ptr::null_mut(), 16) as u8 as i8;
        x += 1;
    }
}

unsafe fn ecryptfs_crypto_api_algify_cipher_name(
    algified_name: *mut *mut i8, cipher_name: *const i8, chaining_modifier: *const i8,
) -> i32 {
    // kasprintf(GFP_KERNEL, "%s(%s)", chaining_modifier, cipher_name)
    *algified_name = kasprintf(0, b"%s(%s)\0".as_ptr() as *const i8, chaining_modifier, cipher_name);
    if (*algified_name).is_null() { return -12; }
    0
}

// The following declarations preserve the externally visible implementation
// interface; structure layouts and kernel operations come from ecryptfs_kernel.h.
#[repr(C)] pub struct ecryptfs_crypt_stat { _private: [u8; 0] }
#[repr(C)] pub struct ecryptfs_mount_crypt_stat { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct ecryptfs_filename { _private: [u8; 0] }
#[repr(C)] pub struct ecryptfs_key_tfm { _private: [u8; 0] }

extern "C" {
    fn ecryptfs_derive_iv(iv: *mut i8, crypt_stat: *mut ecryptfs_crypt_stat, offset: i64);
    fn ecryptfs_init_crypt_stat(crypt_stat: *mut ecryptfs_crypt_stat);
    fn ecryptfs_destroy_crypt_stat(crypt_stat: *mut ecryptfs_crypt_stat);
    fn ecryptfs_destroy_mount_crypt_stat(crypt_stat: *mut ecryptfs_mount_crypt_stat);
    fn ecryptfs_encrypt_page(folio: *mut folio) -> i32;
    fn ecryptfs_decrypt_page(folio: *mut folio) -> i32;
    fn ecryptfs_init_crypt_ctx(crypt_stat: *mut ecryptfs_crypt_stat) -> i32;
    fn ecryptfs_set_default_sizes(crypt_stat: *mut ecryptfs_crypt_stat);
    fn ecryptfs_compute_root_iv(crypt_stat: *mut ecryptfs_crypt_stat) -> i32;
    fn ecryptfs_new_file_context(inode: *mut inode) -> i32;
    fn ecryptfs_code_for_cipher_string(cipher_name: *mut i8, key_bytes: usize) -> u8;
    fn ecryptfs_cipher_code_to_string(str_: *mut i8, size: usize, cipher_code: u8) -> i32;
    fn ecryptfs_read_and_validate_header_region(inode: *mut inode) -> i32;
    fn ecryptfs_write_metadata(dentry: *mut dentry, inode: *mut inode) -> i32;
    fn ecryptfs_read_xattr_region(page_virt: *mut i8, inode: *mut inode) -> i32;
    fn ecryptfs_read_and_validate_xattr_region(dentry: *mut dentry, inode: *mut inode) -> i32;
    fn ecryptfs_read_metadata(dentry: *mut dentry) -> i32;
    fn ecryptfs_init_crypto() -> i32;
    fn ecryptfs_destroy_crypto() -> i32;
    fn ecryptfs_add_new_key_tfm(key_tfm: *mut *mut ecryptfs_key_tfm, cipher_name: *mut i8, key_size: usize) -> i32;
    fn ecryptfs_tfm_exists(cipher_name: *mut i8, key_tfm: *mut *mut ecryptfs_key_tfm) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
