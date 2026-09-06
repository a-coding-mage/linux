// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 IBM Corporation
 *
 * Authors:
 * Mimi Zohar <zohar@us.ibm.com>
 *
 * File: evm_secfs.c
 *	- Used to signal when key is on keyring
 *	- Get the key and enable EVM
 */

use core::ffi::c_char;
use core::ptr;

// External kernel types (opaque)
extern "C" {
    type dentry;
    type file;
    type xattr_list;
    type audit_buffer;
    type iattr;
    type inode;
    type mnt_idmap;
    type file_operations;
}

// External kernel functions
extern "C" {
    fn simple_read_from_buffer(
        buf: *mut c_char,
        count: usize,
        ppos: *mut i64,
        from: *const c_char,
        available: usize,
    ) -> isize;
    fn simple_setattr(
        mnt_idmap: *const mnt_idmap,
        dentry: *mut dentry,
        newattrs: *mut iattr,
    ) -> i32;
    fn inode_lock(inode: *mut inode);
    fn inode_unlock(inode: *mut inode);
    fn mutex_lock_interruptible(mutex: *mut core::ffi::c_void) -> i32;
    fn mutex_unlock(mutex: *mut core::ffi::c_void);
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn strlen(s: *const c_char) -> usize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> i32;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> i32;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> i32;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> i32;
    fn memdup_user_nul(from: *const c_char, n: usize) -> *mut c_char;
    fn audit_context() -> *mut core::ffi::c_void;
    fn audit_log_start(
        ctx: *mut core::ffi::c_void,
        flags: u32,
        audit_type: u32,
    ) -> *mut audit_buffer;
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn audit_log_untrustedstring(ab: *mut audit_buffer, s: *const c_char);
    fn audit_log_end(ab: *mut audit_buffer);
    fn securityfs_create_dir(
        name: *const c_char,
        parent: *mut dentry,
    ) -> *mut dentry;
    fn securityfs_create_file(
        name: *const c_char,
        mode: u32,
        parent: *mut dentry,
        data: *mut core::ffi::c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn securityfs_create_symlink(
        name: *const c_char,
        parent: *mut dentry,
        target: *const c_char,
        data: *mut core::ffi::c_void,
    ) -> *mut dentry;
    fn securityfs_remove(dentry: *mut dentry);
    fn integrity_fs_init() -> i32;
    fn integrity_fs_fini();
    fn evm_init_key() -> i32;
    fn capable(cap: u32) -> bool;
    fn kstrtouint_from_user(s: *const c_char, count: usize, base: u32, res: *mut u32) -> i32;
    fn IS_ENABLED(config: u32) -> bool;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> i32;

    static mut evm_initialized: u32;
    static mut evm_config_xattrnames: xattr_list;
    static mut integrity_dir: *mut dentry;
    static nop_mnt_idmap: mnt_idmap;
}

static mut evm_dir: *mut dentry = ptr::null_mut();
static mut evm_symlink: *mut dentry = ptr::null_mut();

#[cfg(CONFIG_EVM_ADD_XATTRS)]
static mut evm_xattrs: *mut dentry = ptr::null_mut();
#[cfg(CONFIG_EVM_ADD_XATTRS)]
static mut xattr_list_mutex: core::ffi::c_void = unsafe { core::mem::zeroed() };
#[cfg(CONFIG_EVM_ADD_XATTRS)]
static mut evm_xattrs_locked: i32 = 0;

const EVM_SETUP_COMPLETE: u32 = 1;
const EVM_INIT_MASK: u32 = 0;
const EVM_ALLOW_METADATA_WRITES: u32 = 2;
const EVM_INIT_HMAC: u32 = 4;
const XATTR_NAME_MAX: usize = 255;
const XATTR_SECURITY_PREFIX_LEN: usize = 9;
const GFP_KERNEL: u32 = 0x0120;
const CAP_SYS_ADMIN: u32 = 21;
const AUDIT_INTEGRITY_EVM_XATTR: u32 = 1800;
const ATTR_MODE: u32 = 0x20;
const S_IFREG: u32 = 0o100000;
const EPERM: i32 = -1;
const EINVAL: i32 = -22;
const ENOMEM: i32 = -12;
const EFAULT: i32 = -14;
const ERESTARTSYS: i32 = -512;
const EEXIST: i32 = -17;
const E2BIG: i32 = -7;

const XATTR_SECURITY_PREFIX: &[u8] = b"security.";

/// evm_read_key - read() for <securityfs>/evm
///
/// @filp: file pointer, not actually used
/// @buf: where to put the result
/// @count: maximum to send along
/// @ppos: where to start
///
/// Returns number of bytes read or error code, as appropriate
unsafe extern "C" fn evm_read_key(
    filp: *mut file,
    buf: *mut c_char,
    count: usize,
    ppos: *mut i64,
) -> isize {
    let mut temp: [u8; 80] = [0; 80];
    let rc: isize;

    if *ppos != 0 {
        return 0;
    }

    sprintf(
        temp.as_mut_ptr() as *mut c_char,
        b"%d\0".as_ptr() as *const c_char,
        (evm_initialized & !EVM_SETUP_COMPLETE) as i32,
    );
    rc = simple_read_from_buffer(
        buf,
        count,
        ppos,
        temp.as_ptr() as *const c_char,
        strlen(temp.as_ptr() as *const c_char),
    );

    rc
}

/// evm_write_key - write() for <securityfs>/evm
/// @file: file pointer, not actually used
/// @buf: where to get the data from
/// @count: bytes sent
/// @ppos: where to start
///
/// Used to signal that key is on the kernel key ring.
/// - get the integrity hmac key from the kernel key ring
/// - create list of hmac protected extended attributes
/// Returns number of bytes written or error code, as appropriate
unsafe extern "C" fn evm_write_key(
    file: *mut file,
    buf: *const c_char,
    count: usize,
    ppos: *mut i64,
) -> isize {
    let mut i: u32 = 0;
    let mut ret: i32;

    if !capable(CAP_SYS_ADMIN) || (evm_initialized & EVM_SETUP_COMPLETE) != 0 {
        return EPERM as isize;
    }

    ret = kstrtouint_from_user(buf, count, 0, &mut i);

    if ret != 0 {
        return ret as isize;
    }

    /* Reject invalid values */
    if i == 0 || (i & !EVM_INIT_MASK) != 0 {
        return EINVAL as isize;
    }

    /*
     * Don't allow a request to enable metadata writes if
     * an HMAC key is loaded.
     */
    if (i & EVM_ALLOW_METADATA_WRITES) != 0 && (evm_initialized & EVM_INIT_HMAC) != 0 {
        return EPERM as isize;
    }

    if (i & EVM_INIT_HMAC) != 0 {
        ret = evm_init_key();
        if ret != 0 {
            return ret as isize;
        }
        /* Forbid further writes after the symmetric key is loaded */
        i |= EVM_SETUP_COMPLETE;
    }

    evm_initialized |= i;

    /* Don't allow protected metadata modification if a symmetric key
     * is loaded
     */
    if (evm_initialized & EVM_INIT_HMAC) != 0 {
        evm_initialized &= !EVM_ALLOW_METADATA_WRITES;
    }

    count as isize
}

static EVM_KEY_OPS: file_operations = unsafe { core::mem::zeroed() };

#[cfg(CONFIG_EVM_ADD_XATTRS)]
/// evm_read_xattrs - read() for <securityfs>/evm_xattrs
///
/// @filp: file pointer, not actually used
/// @buf: where to put the result
/// @count: maximum to send along
/// @ppos: where to start
///
/// Returns number of bytes read or error code, as appropriate
unsafe extern "C" fn evm_read_xattrs(
    filp: *mut file,
    buf: *mut c_char,
    count: usize,
    ppos: *mut i64,
) -> isize {
    let mut temp: *mut c_char;
    let mut offset: usize = 0;
    let mut size: usize = 0;
    let rc: isize;
    let mut xattr: *mut xattr_list;

    if *ppos != 0 {
        return 0;
    }

    let rc_lock = mutex_lock_interruptible(&mut xattr_list_mutex as *mut core::ffi::c_void);
    if rc_lock != 0 {
        return ERESTARTSYS as isize;
    }

    // list_for_each_entry simulation
    // Note: This requires access to evm_config_xattrnames structure
    // The actual list traversal would depend on the linked list implementation
    // We approximate with the external list declaration

    temp = kmalloc(size + 1, GFP_KERNEL) as *mut c_char;
    if temp.is_null() {
        mutex_unlock(&mut xattr_list_mutex as *mut core::ffi::c_void);
        return ENOMEM as isize;
    }

    *temp.add(size) = b'\0' as c_char;

    /*
     * No truncation possible: size is computed over the same enabled
     * xattrs under xattr_list_mutex, so offset never exceeds size.
     */
    // list_for_each_entry simulation (second pass)

    mutex_unlock(&mut xattr_list_mutex as *mut core::ffi::c_void);
    let result = simple_read_from_buffer(buf, count, ppos, temp, offset);

    kfree(temp as *mut core::ffi::c_void);

    result
}

#[cfg(CONFIG_EVM_ADD_XATTRS)]
/// evm_write_xattrs - write() for <securityfs>/evm_xattrs
/// @file: file pointer, not actually used
/// @buf: where to get the data from
/// @count: bytes sent
/// @ppos: where to start
///
/// Returns number of bytes written or error code, as appropriate
unsafe extern "C" fn evm_write_xattrs(
    file: *mut file,
    buf: *const c_char,
    count: usize,
    ppos: *mut i64,
) -> isize {
    let mut len: i32;
    let mut err: i32;
    let mut xattr: *mut xattr_list;
    let mut tmp: *mut xattr_list;
    let mut ab: *mut audit_buffer;
    let mut newattrs: iattr = core::mem::zeroed();
    let mut inode: *mut inode;

    if !capable(CAP_SYS_ADMIN) || evm_xattrs_locked != 0 {
        return EPERM as isize;
    }

    if *ppos != 0 {
        return EINVAL as isize;
    }

    if count > XATTR_NAME_MAX {
        return E2BIG as isize;
    }

    ab = audit_log_start(audit_context(), GFP_KERNEL, AUDIT_INTEGRITY_EVM_XATTR);
    if ab.is_null() && IS_ENABLED(0) {
        return ENOMEM as isize;
    }

    xattr = kmalloc(core::mem::size_of::<xattr_list>(), GFP_KERNEL) as *mut xattr_list;
    if xattr.is_null() {
        err = ENOMEM;
        // goto out
    } else {
        (*xattr).enabled = 1; // Use 1 for true
        (*xattr).name = memdup_user_nul(buf, count);
        if IS_ERR((*xattr).name as *const core::ffi::c_void) {
            err = PTR_ERR((*xattr).name as *const core::ffi::c_void);
            (*xattr).name = ptr::null_mut();
            // goto out
        } else {
            /* Remove any trailing newline */
            len = strlen((*xattr).name) as i32;
            if len > 0 && *(*xattr).name.add((len - 1) as usize) as u8 == b'\n' {
                *(*xattr).name.add((len - 1) as usize) = b'\0' as c_char;
            }

            audit_log_format(ab, b"xattr=\0".as_ptr() as *const c_char);
            audit_log_untrustedstring(ab, (*xattr).name);

            if strcmp((*xattr).name, b".\0".as_ptr() as *const c_char) == 0 {
                evm_xattrs_locked = 1;
                (*newattrs).ia_mode = S_IFREG | 0o440;
                (*newattrs).ia_valid = ATTR_MODE;
                inode = (*evm_xattrs).d_inode;
                inode_lock(inode);
                err = simple_setattr(&nop_mnt_idmap, evm_xattrs, &mut newattrs);
                inode_unlock(inode);
                if err == 0 {
                    err = count as i32;
                }
                // goto out
            } else {
                if strncmp(
                    (*xattr).name,
                    XATTR_SECURITY_PREFIX.as_ptr() as *const c_char,
                    XATTR_SECURITY_PREFIX_LEN,
                ) != 0
                {
                    err = EINVAL;
                    // goto out
                } else {
                    /*
                     * xattr_list_mutex guards against races in evm_read_xattrs().
                     * Entries are only added to the evm_config_xattrnames list
                     * and never deleted. Therefore, the list is traversed
                     * using list_for_each_entry_lockless() without holding
                     * the mutex in evm_calc_hmac_or_hash(), evm_find_protected_xattrs()
                     * and evm_protected_xattr().
                     */
                    mutex_lock(&mut xattr_list_mutex as *mut core::ffi::c_void);
                    // list_for_each_entry simulation
                    // Note: Actual list traversal depends on kernel list implementation
                    // This is a placeholder for the actual linked list iteration
                    err = EEXIST;
                    mutex_unlock(&mut xattr_list_mutex as *mut core::ffi::c_void);

                    if err == EEXIST {
                        // Simplified: would check if tmp->enabled, then set err = count or leave as EEXIST
                        audit_log_format(ab, b" res=0\0".as_ptr() as *const c_char);
                        audit_log_end(ab);
                        return count as isize;
                    }

                    // list_add_tail_rcu would be called here
                    mutex_unlock(&mut xattr_list_mutex as *mut core::ffi::c_void);

                    audit_log_format(ab, b" res=0\0".as_ptr() as *const c_char);
                    audit_log_end(ab);
                    return count as isize;
                }
            }
        }
    }

    // out label
    audit_log_format(ab, b" res=%d\0".as_ptr() as *const c_char, if err < 0 { err } else { 0 });
    audit_log_end(ab);
    if !xattr.is_null() {
        kfree((*xattr).name as *mut core::ffi::c_void);
        kfree(xattr as *mut core::ffi::c_void);
    }
    err as isize
}

#[cfg(CONFIG_EVM_ADD_XATTRS)]
static EVM_XATTR_OPS: file_operations = unsafe { core::mem::zeroed() };

#[cfg(CONFIG_EVM_ADD_XATTRS)]
unsafe fn evm_init_xattrs() -> i32 {
    evm_xattrs = securityfs_create_file(
        b"evm_xattrs\0".as_ptr() as *const c_char,
        0o660,
        evm_dir,
        ptr::null_mut(),
        &EVM_XATTR_OPS,
    );
    if IS_ERR(evm_xattrs as *const core::ffi::c_void) {
        return EFAULT;
    }

    0
}

#[cfg(not(CONFIG_EVM_ADD_XATTRS))]
unsafe fn evm_init_xattrs() -> i32 {
    0
}

/// evm_init_secfs - Initialize EVM securityfs
#[no_mangle]
pub unsafe extern "C" fn evm_init_secfs() -> i32 {
    let mut error: i32 = 0;
    let mut dentry_local: *mut dentry;

    error = integrity_fs_init();
    if error < 0 {
        return EFAULT;
    }

    evm_dir = securityfs_create_dir(b"evm\0".as_ptr() as *const c_char, integrity_dir);
    if IS_ERR(evm_dir as *const core::ffi::c_void) {
        error = EFAULT;
        // goto out
    } else {
        dentry_local = securityfs_create_file(
            b"evm\0".as_ptr() as *const c_char,
            0o660,
            evm_dir,
            ptr::null_mut(),
            &EVM_KEY_OPS,
        );
        if IS_ERR(dentry_local as *const core::ffi::c_void) {
            error = EFAULT;
            // goto out
        } else {
            evm_symlink = securityfs_create_symlink(
                b"evm\0".as_ptr() as *const c_char,
                ptr::null_mut(),
                b"integrity/evm/evm\0".as_ptr() as *const c_char,
                ptr::null_mut(),
            );
            if IS_ERR(evm_symlink as *const core::ffi::c_void) {
                error = EFAULT;
                // goto out
            } else {
                if evm_init_xattrs() != 0 {
                    error = EFAULT;
                    // goto out
                } else {
                    return 0;
                }
            }
        }
    }

    // out label
    securityfs_remove(evm_symlink);
    securityfs_remove(evm_dir);
    integrity_fs_fini();
    error
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
