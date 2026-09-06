// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

// Translated from linux/dcache.h, linux/security.h, ipe.h, fs.h, eval.h,
// policy.h, and audit.h dependencies by reference only.

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

pub type ssize_t = isize;
pub type size_t = usize;
pub type loff_t = i64;
pub type umode_t = u16;

pub const EPERM: c_int = 1;
pub const EOPNOTSUPP: c_int = 95;
pub const CAP_MAC_ADMIN: c_int = 33;

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ipe_policy {
    _private: [u8; 0],
}

pub type WriteFn = unsafe extern "C" fn(
    f: *mut file,
    data: *const c_char,
    len: size_t,
    offset: *mut loff_t,
) -> ssize_t;

pub type ReadFn = unsafe extern "C" fn(
    f: *mut file,
    data: *mut c_char,
    len: size_t,
    offset: *mut loff_t,
) -> ssize_t;

#[repr(C)]
pub struct file_operations {
    pub write: Option<WriteFn>,
    pub read: Option<ReadFn>,
}

unsafe extern "C" {
    static init_user_ns: user_namespace;
    static mut success_audit: bool;
    static mut enforce: bool;
    static ipe_enabled: bool;
    static mut ipe_active_policy: *mut ipe_policy;

    fn file_ns_capable(f: *mut file, ns: *const user_namespace, cap: c_int) -> bool;
    fn kstrtobool_from_user(data: *const c_char, len: size_t, value: *mut bool) -> c_int;
    fn simple_read_from_buffer(
        to: *mut c_char,
        count: size_t,
        ppos: *mut loff_t,
        from: *const c_void,
        available: size_t,
    ) -> ssize_t;
    fn memdup_user_nul(src: *const c_char, len: size_t) -> *mut c_char;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn ERR_PTR(error: c_int) -> *mut ipe_policy;
    fn kfree(ptr: *const c_void);
    fn ipe_new_policy(
        pkcs7: *const c_void,
        pkcs7len: size_t,
        text: *mut c_char,
        textlen: size_t,
    ) -> *mut ipe_policy;
    fn ipe_new_policyfs_node(p: *mut ipe_policy) -> c_int;
    fn ipe_free_policy(p: *mut ipe_policy);
    fn ipe_audit_policy_load(p: *mut ipe_policy);
    fn ipe_audit_enforce(new_value: bool, old_value: bool);
    fn securityfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn securityfs_create_file(
        name: *const c_char,
        mode: umode_t,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn securityfs_remove(dentry: *mut dentry);
}

static mut root: *mut dentry = ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut policy_root: *mut dentry = ptr::null_mut();

/**
 * setaudit() - Write handler for the securityfs node, "ipe/success_audit"
 * @f: Supplies a file structure representing the securityfs node.
 * @data: Supplies a buffer passed to the write syscall.
 * @len: Supplies the length of @data.
 * @offset: unused.
 *
 * Return:
 * * Length of buffer written	- Success
 * * %-EPERM			- Insufficient permission
 */
unsafe extern "C" fn setaudit(
    f: *mut file,
    data: *const c_char,
    len: size_t,
    _offset: *mut loff_t,
) -> ssize_t {
    let mut rc: c_int;
    let mut value = false;

    if !unsafe { file_ns_capable(f, &raw const init_user_ns, CAP_MAC_ADMIN) } {
        return -(EPERM as ssize_t);
    }

    rc = unsafe { kstrtobool_from_user(data, len, &mut value) };
    if rc != 0 {
        return rc as ssize_t;
    }

    unsafe {
        core::ptr::write_volatile(&raw mut success_audit, value);
    }

    len as ssize_t
}

/**
 * getaudit() - Read handler for the securityfs node, "ipe/success_audit"
 * @f: Supplies a file structure representing the securityfs node.
 * @data: Supplies a buffer passed to the read syscall.
 * @len: Supplies the length of @data.
 * @offset: unused.
 *
 * Return: Length of buffer written
 */
unsafe extern "C" fn getaudit(
    _f: *mut file,
    data: *mut c_char,
    len: size_t,
    offset: *mut loff_t,
) -> ssize_t {
    let result: *const c_char;

    result = if unsafe { core::ptr::read_volatile(&raw const success_audit) } {
        c"1".as_ptr()
    } else {
        c"0".as_ptr()
    };

    unsafe { simple_read_from_buffer(data, len, offset, result as *const c_void, 1) }
}

/**
 * setenforce() - Write handler for the securityfs node, "ipe/enforce"
 * @f: Supplies a file structure representing the securityfs node.
 * @data: Supplies a buffer passed to the write syscall.
 * @len: Supplies the length of @data.
 * @offset: unused.
 *
 * Return:
 * * Length of buffer written	- Success
 * * %-EPERM			- Insufficient permission
 */
unsafe extern "C" fn setenforce(
    f: *mut file,
    data: *const c_char,
    len: size_t,
    _offset: *mut loff_t,
) -> ssize_t {
    let mut rc: c_int;
    let mut new_value = false;
    let old_value: bool;

    if !unsafe { file_ns_capable(f, &raw const init_user_ns, CAP_MAC_ADMIN) } {
        return -(EPERM as ssize_t);
    }

    old_value = unsafe { core::ptr::read_volatile(&raw const enforce) };
    rc = unsafe { kstrtobool_from_user(data, len, &mut new_value) };
    if rc != 0 {
        return rc as ssize_t;
    }

    if new_value != old_value {
        unsafe {
            ipe_audit_enforce(new_value, old_value);
            core::ptr::write_volatile(&raw mut enforce, new_value);
        }
    }

    len as ssize_t
}

/**
 * getenforce() - Read handler for the securityfs node, "ipe/enforce"
 * @f: Supplies a file structure representing the securityfs node.
 * @data: Supplies a buffer passed to the read syscall.
 * @len: Supplies the length of @data.
 * @offset: unused.
 *
 * Return: Length of buffer written
 */
unsafe extern "C" fn getenforce(
    _f: *mut file,
    data: *mut c_char,
    len: size_t,
    offset: *mut loff_t,
) -> ssize_t {
    let result: *const c_char;

    result = if unsafe { core::ptr::read_volatile(&raw const enforce) } {
        c"1".as_ptr()
    } else {
        c"0".as_ptr()
    };

    unsafe { simple_read_from_buffer(data, len, offset, result as *const c_void, 1) }
}

/**
 * new_policy() - Write handler for the securityfs node, "ipe/new_policy".
 * @f: Supplies a file structure representing the securityfs node.
 * @data: Supplies a buffer passed to the write syscall.
 * @len: Supplies the length of @data.
 * @offset: unused.
 *
 * Return:
 * * Length of buffer written	- Success
 * * %-EPERM			- Insufficient permission
 * * %-ENOMEM			- Out of memory (OOM)
 * * %-EBADMSG			- Policy is invalid
 * * %-ERANGE			- Policy version number overflow
 * * %-EINVAL			- Policy version parsing error
 * * %-EEXIST			- Same name policy already deployed
 * * %-ENOKEY			- Policy signing key not found
 * * %-EKEYREJECTED		- Policy signature verification failed
 */
unsafe extern "C" fn new_policy(
    f: *mut file,
    data: *const c_char,
    len: size_t,
    _offset: *mut loff_t,
) -> ssize_t {
    let mut p: *mut ipe_policy = ptr::null_mut();
    let mut copy: *mut c_char = ptr::null_mut();
    let mut rc: c_int = 0;

    if !unsafe { file_ns_capable(f, &raw const init_user_ns, CAP_MAC_ADMIN) } {
        rc = -EPERM;
    } else {
        copy = unsafe { memdup_user_nul(data, len) };
        if unsafe { IS_ERR(copy as *const c_void) } {
            rc = unsafe { PTR_ERR(copy as *const c_void) };
            copy = ptr::null_mut();
        } else {
            p = unsafe { ipe_new_policy(ptr::null(), 0, copy, len) };
            if unsafe { IS_ERR(p as *const c_void) } {
                rc = unsafe { PTR_ERR(p as *const c_void) };
            } else {
                rc = unsafe { ipe_new_policyfs_node(p) };
            }
        }
    }

    unsafe {
        kfree(copy as *const c_void);
        if rc < 0 {
            ipe_free_policy(p);
            ipe_audit_policy_load(ERR_PTR(rc));
        } else {
            ipe_audit_policy_load(p);
        }
    }

    if rc < 0 {
        rc as ssize_t
    } else {
        len as ssize_t
    }
}

static np_fops: file_operations = file_operations {
    write: Some(new_policy),
    read: None,
};

static audit_fops: file_operations = file_operations {
    write: Some(setaudit),
    read: Some(getaudit),
};

static enforce_fops: file_operations = file_operations {
    write: Some(setenforce),
    read: Some(getenforce),
};

/**
 * ipe_init_securityfs() - Initialize IPE's securityfs tree at fsinit.
 *
 * Return: %0 on success. If an error occurs, the function will return
 * the -errno.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipe_init_securityfs() -> c_int {
    let mut rc: c_int;
    let ap: *mut ipe_policy;
    let mut dentry: *mut dentry;

    if !unsafe { ipe_enabled } {
        return -EOPNOTSUPP;
    }

    unsafe {
        root = securityfs_create_dir(c"ipe".as_ptr(), ptr::null_mut());
    }
    if unsafe { IS_ERR(root as *const c_void) } {
        return unsafe { PTR_ERR(root as *const c_void) };
    }

    dentry = unsafe {
        securityfs_create_file(
            c"success_audit".as_ptr(),
            0o600,
            root,
            ptr::null_mut(),
            &audit_fops,
        )
    };
    if unsafe { IS_ERR(dentry as *const c_void) } {
        rc = unsafe { PTR_ERR(dentry as *const c_void) };
        unsafe {
            securityfs_remove(root);
        }
        return rc;
    }

    dentry = unsafe {
        securityfs_create_file(
            c"enforce".as_ptr(),
            0o600,
            root,
            ptr::null_mut(),
            &enforce_fops,
        )
    };
    if unsafe { IS_ERR(dentry as *const c_void) } {
        rc = unsafe { PTR_ERR(dentry as *const c_void) };
        unsafe {
            securityfs_remove(root);
        }
        return rc;
    }

    unsafe {
        policy_root = securityfs_create_dir(c"policies".as_ptr(), root);
    }
    if unsafe { IS_ERR(policy_root as *const c_void) } {
        rc = unsafe { PTR_ERR(policy_root as *const c_void) };
        unsafe {
            securityfs_remove(root);
        }
        return rc;
    }

    ap = unsafe { core::ptr::read_volatile(&raw const ipe_active_policy) };
    if !ap.is_null() {
        rc = unsafe { ipe_new_policyfs_node(ap) };
        if rc != 0 {
            unsafe {
                securityfs_remove(root);
            }
            return rc;
        }
    }

    dentry = unsafe {
        securityfs_create_file(
            c"new_policy".as_ptr(),
            0o200,
            root,
            ptr::null_mut(),
            &np_fops,
        )
    };
    if unsafe { IS_ERR(dentry as *const c_void) } {
        rc = unsafe { PTR_ERR(dentry as *const c_void) };
        unsafe {
            securityfs_remove(root);
        }
        return rc;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
