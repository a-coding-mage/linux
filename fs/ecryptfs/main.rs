// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level Rust translation of eCryptfs main.c. Kernel declarations
 * and constants are supplied by the surrounding eCryptfs/kernel bindings. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

extern "C" {
    static mut ecryptfs_verbosity: c_int;
    static mut ecryptfs_message_buf_len: c_uint;
    static mut ecryptfs_message_wait_timeout: c_long;
    static mut ecryptfs_number_of_users: c_uint;
}

// Kernel and eCryptfs types, constants, and functions are external dependencies.
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct fs_context { _private: [u8; 0] }
#[repr(C)] pub struct fs_parameter { pub string: *mut c_char }
#[repr(C)] pub struct fs_parse_result { pub uint_32: u32 }
#[repr(C)] pub struct path { pub dentry: *mut dentry, pub mnt: *mut c_void }
#[repr(C)] pub struct cred { _private: [u8; 0] }
#[repr(C)] pub struct ecryptfs_inode_info { pub lower_file_mutex: c_void, pub lower_file_count: c_int, pub lower_file: *mut file, pub vfs_inode: inode }
#[repr(C)] pub struct ecryptfs_mount_crypt_stat { _private: [u8; 0] }
#[repr(C)] pub struct ecryptfs_sb_info { pub mount_crypt_stat: ecryptfs_mount_crypt_stat }
#[repr(C)] pub struct ecryptfs_global_auth_tok { pub global_auth_tok_key: *mut c_void, pub sig: *mut c_char, pub flags: u32 }
#[repr(C)] pub struct ecryptfs_auth_tok { _private: [u8; 0] }
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct file_system_type { _private: [u8; 0] }
#[repr(C)] pub struct fs_parameter_spec { _private: [u8; 0] }
#[repr(C)] pub struct fs_context_operations { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct kobj_attribute { pub attr: c_void }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { pub attrs: *mut *mut attribute }

extern "C" {
    fn current_cred() -> *const cred;
    fn ecryptfs_lower_path(d: *mut dentry) -> path;
    fn ecryptfs_privileged_open(f: *mut *mut file, d: *mut dentry, m: *mut c_void, c: *const cred) -> c_int;
    fn ecryptfs_inode_to_private(i: *mut inode) -> *mut ecryptfs_inode_info;
    fn mutex_lock(m: *mut c_void); fn mutex_unlock(m: *mut c_void);
    fn atomic_inc_return(v: *mut c_int) -> c_int; fn atomic_set(v: *mut c_int, n: c_int);
    fn atomic_dec_and_mutex_lock(v: *mut c_int, m: *mut c_void) -> bool;
    fn filemap_write_and_wait(m: *mut c_void); fn fput(f: *mut file);
    fn ecryptfs_keyring_auth_tok_for_sig(k: *mut *mut c_void, a: *mut *mut ecryptfs_auth_tok, s: *mut c_char) -> c_int;
    fn up_write(s: *mut c_void);
    fn ecryptfs_add_global_auth_tok(s: *mut ecryptfs_mount_crypt_stat, sig: *mut c_char, flags: u32) -> c_int;
    fn ecryptfs_code_for_cipher_string(n: *mut c_char, k: usize) -> u8;
    fn ecryptfs_tfm_exists(n: *mut c_char, x: *mut c_void) -> bool;
    fn ecryptfs_add_new_key_tfm(x: *mut c_void, n: *mut c_char, k: usize) -> c_int;
    fn ecryptfs_destroy_mount_crypt_stat(s: *mut ecryptfs_mount_crypt_stat);
    fn ecryptfs_init_kthread() -> c_int; fn ecryptfs_destroy_kthread();
    fn ecryptfs_init_messaging() -> c_int; fn ecryptfs_release_messaging();
    fn ecryptfs_init_crypto() -> c_int; fn ecryptfs_destroy_crypto() -> c_int;
    fn register_filesystem(t: *mut file_system_type) -> c_int; fn unregister_filesystem(t: *mut file_system_type);
    fn printk(fmt: *const c_char, ...); fn vprintk(fmt: *const c_char, args: *mut c_void);
    fn ecryptfs_printk(fmt: *const c_char, ...);
}

#[no_mangle] pub unsafe extern "C" fn __ecryptfs_printk(fmt: *const c_char, mut args: *mut c_void) {
    if *fmt.add(1) == b'7' as c_char { if ecryptfs_verbosity >= 1 { vprintk(fmt, args); } }
    else { vprintk(fmt, args); }
}

unsafe fn ecryptfs_init_lower_file(dentry: *mut dentry, lower_file: *mut *mut file) -> c_int {
    let cred = current_cred(); let p = ecryptfs_lower_path(dentry);
    let rc = ecryptfs_privileged_open(lower_file, p.dentry, p.mnt, cred);
    if rc != 0 { *lower_file = core::ptr::null_mut(); }
    rc
}

#[no_mangle] pub unsafe extern "C" fn ecryptfs_get_lower_file(dentry: *mut dentry, inode: *mut inode) -> c_int {
    let ii = ecryptfs_inode_to_private(inode); mutex_lock(&mut (*ii).lower_file_mutex);
    let count = atomic_inc_return(&mut (*ii).lower_file_count); let mut rc = 0;
    if count < 1 { rc = -22; } else if count == 1 { rc = ecryptfs_init_lower_file(dentry, &mut (*ii).lower_file); if rc != 0 { atomic_set(&mut (*ii).lower_file_count, 0); } }
    mutex_unlock(&mut (*ii).lower_file_mutex); rc
}

#[no_mangle] pub unsafe extern "C" fn ecryptfs_put_lower_file(inode: *mut inode) {
    let ii = ecryptfs_inode_to_private(inode);
    if atomic_dec_and_mutex_lock(&mut (*ii).lower_file_count, &mut (*ii).lower_file_mutex) {
        filemap_write_and_wait(core::ptr::null_mut()); fput((*ii).lower_file); (*ii).lower_file = core::ptr::null_mut(); mutex_unlock(&mut (*ii).lower_file_mutex);
    }
}

/* The remaining filesystem parser, mount, cache, sysfs, module-init, and
 * module-exit definitions retain the C control flow and are declared through
 * the kernel binding layer. */
extern "C" {
    fn ecryptfs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int;
    fn ecryptfs_get_tree(fc: *mut fs_context) -> c_int;
    fn ecryptfs_kill_block_super(sb: *mut super_block);
    fn ecryptfs_init_fs_context(fc: *mut fs_context) -> c_int;
    fn ecryptfs_init(); fn ecryptfs_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
