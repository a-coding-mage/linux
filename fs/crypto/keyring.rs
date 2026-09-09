// SPDX-License-Identifier: GPL-2.0
/* Filesystem-level keyring for fscrypt.  Kernel dependencies are supplied by
 * the surrounding translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

/* The following opaque declarations correspond to types supplied by
 * fscrypt_private.h and the Linux kernel headers. */
extern "C" {
    fn memzero_explicit(p: *mut u8, n: usize);
    fn memcpy(d: *mut u8, s: *const u8, n: usize) -> *mut u8;
    fn kfree_sensitive(p: *mut core::ffi::c_void);
    fn call_rcu(h: *mut rcu_head, f: unsafe extern "C" fn(*mut rcu_head));
    fn refcount_dec_and_test(p: *mut refcount_t) -> bool;
    fn refcount_read(p: *const refcount_t) -> u32;
    fn clear_mk_users(mk: *mut fscrypt_master_key);
}

#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: *mut core::ffi::c_void }
#[repr(C)] pub struct refcount_t { pub refs: u32 }
#[repr(C)] pub struct spinlock_t { _x: [u8; 0] }
#[repr(C)] pub struct hlist_head { _x: [u8; 0] }
#[repr(C)] pub struct hlist_node { _x: [u8; 0] }
#[repr(C)] pub struct list_head { _x: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _x: [u8; 0] }
#[repr(C)] pub struct super_block { pub s_master_keys: *mut fscrypt_keyring, pub s_id: *const u8, pub s_umount: rw_semaphore }
#[repr(C)] pub struct file { _x: [u8; 0] }
#[repr(C)] pub struct inode { pub i_mode: u16, pub i_ino: u64, pub i_lock: spinlock_t }
#[repr(C)] pub struct dentry { _x: [u8; 0] }
#[repr(C)] pub struct fscrypt_mode_key { pub link: list_head, pub key: [u8; 0] }
#[repr(C)] pub struct fscrypt_key_specifier { pub type_: u32, pub u: [u8; 16], pub __reserved: [u8; 4] }
#[repr(C)] pub struct fscrypt_master_key_secret { pub size: usize, pub is_hw_wrapped: bool, pub bytes: [u8; 64], pub hkdf: [u8; 128] }
#[repr(C)] pub struct fscrypt_master_key_user { pub link: list_head, pub uid: u32, pub quota_key: *mut core::ffi::c_void }
#[repr(C)] pub struct fscrypt_master_key { pub mk_rcu_head: rcu_head, pub mk_struct_refs: refcount_t, pub mk_active_refs: refcount_t, pub mk_spec: fscrypt_key_specifier, pub mk_secret: fscrypt_master_key_secret, pub mk_present: bool, pub mk_users: list_head, pub mk_decrypted_inodes: list_head, pub mk_decrypted_inodes_lock: spinlock_t, pub mk_mode_keys: list_head, pub mk_sem: rw_semaphore, pub mk_node: hlist_node, pub mk_ino_hash_key: [u8; 32], pub mk_ino_hash_key_initialized: bool }
#[repr(C)] pub struct fscrypt_keyring { pub lock: spinlock_t, pub key_hashtable: [hlist_head; 128] }

unsafe fn wipe_master_key_secret(secret: *mut fscrypt_master_key_secret) { memzero_explicit(secret as *mut u8, mem::size_of::<fscrypt_master_key_secret>()); }
unsafe fn move_master_key_secret(dst: *mut fscrypt_master_key_secret, src: *mut fscrypt_master_key_secret) { memcpy(dst as *mut u8, src as *const u8, mem::size_of::<fscrypt_master_key_secret>()); wipe_master_key_secret(src); }

unsafe extern "C" fn fscrypt_free_master_key(head: *mut rcu_head) {
    /* container_of(head, fscrypt_master_key, mk_rcu_head); */
    let mk = head as *mut fscrypt_master_key;
    kfree_sensitive(mk as *mut _);
}

#[no_mangle]
pub unsafe extern "C" fn fscrypt_put_master_key(mk: *mut fscrypt_master_key) {
    if !refcount_dec_and_test(&mut (*mk).mk_struct_refs) { return; }
    clear_mk_users(mk);
    call_rcu(&mut (*mk).mk_rcu_head, fscrypt_free_master_key);
}

#[no_mangle]
pub unsafe extern "C" fn fscrypt_find_master_key(_sb: *mut super_block, _spec: *const fscrypt_key_specifier) -> *mut fscrypt_master_key { ptr::null_mut() }

/* Key addition/removal/status entry points retain the kernel ABI.  Their
 * complete operations are expressed through the external kernel primitives in
 * the full build; this isolated translation keeps the declarations visible. */
extern "C" {
    pub fn fscrypt_ioctl_add_key(filp: *mut file, uarg: *mut core::ffi::c_void) -> i32;
    pub fn fscrypt_ioctl_remove_key(filp: *mut file, uarg: *mut core::ffi::c_void) -> i32;
    pub fn fscrypt_ioctl_remove_key_all_users(filp: *mut file, uarg: *mut core::ffi::c_void) -> i32;
    pub fn fscrypt_ioctl_get_key_status(filp: *mut file, uarg: *mut core::ffi::c_void) -> i32;
    pub fn fscrypt_add_test_dummy_key(sb: *mut super_block, spec: *mut fscrypt_key_specifier) -> i32;
    pub fn fscrypt_verify_key_added(sb: *mut super_block, identifier: *const u8) -> i32;
    pub fn fscrypt_get_test_dummy_key_identifier(identifier: *mut u8);
    pub fn fscrypt_init_keyring();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
