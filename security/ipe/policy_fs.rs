// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

use core::ffi::{c_char, c_int, c_void};

type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type umode_t = u16;
type bool_ = bool;

const ENOENT: c_int = 2;
const EPERM: c_int = 1;
const EINVAL: c_int = 22;
const CAP_MAC_ADMIN: c_int = 33;
const MAX_VERSION_SIZE: usize = "65535.65535.65535".len() + 1;

#[repr(C)]
pub struct file {
    pub f_path: path,
}

#[repr(C)]
pub struct path {
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct dentry {
    pub d_parent: *mut dentry,
}

#[repr(C)]
pub struct inode {
    pub i_private: *mut c_void,
}

#[repr(C)]
pub struct file_operations {
    pub read: Option<
        unsafe extern "C" fn(
            f: *mut file,
            data: *mut c_char,
            len: size_t,
            offset: *mut loff_t,
        ) -> ssize_t,
    >,
    pub write: Option<
        unsafe extern "C" fn(
            f: *mut file,
            data: *const c_char,
            len: size_t,
            offset: *mut loff_t,
        ) -> ssize_t,
    >,
}

/**
 * struct ipefs_file - defines a file in securityfs.
 *
 * @name: file name inside the policy subdirectory
 * @access: file permissions
 * @fops: &file_operations specific to this file
 */
#[repr(C)]
struct ipefs_file {
    name: *const c_char,
    access: umode_t,
    fops: *const file_operations,
}

#[repr(C)]
pub struct ipe_policy {
    pub pkcs7: *const c_void,
    pub pkcs7len: size_t,
    pub text: *const c_void,
    pub textlen: size_t,
    pub parsed: *mut ipe_parsed_policy,
    pub policyfs: *mut dentry,
}

#[repr(C)]
pub struct ipe_parsed_policy {
    pub name: *const c_char,
    pub version: ipe_policy_version,
}

#[repr(C)]
pub struct ipe_policy_version {
    pub major: u16,
    pub minor: u16,
    pub rev: u16,
}

unsafe extern "C" {
    static init_user_ns: c_void;
    static mut ipe_active_policy: *mut ipe_policy;
    static mut ipe_policy_lock: c_void;
    static mut policy_root: *mut dentry;

    fn d_inode(dentry: *mut dentry) -> *mut inode;
    fn inode_lock_shared(inode: *mut inode);
    fn inode_unlock_shared(inode: *mut inode);
    fn inode_lock(inode: *mut inode);
    fn inode_unlock(inode: *mut inode);
    fn simple_read_from_buffer(
        to: *mut c_char,
        count: size_t,
        ppos: *mut loff_t,
        from: *const c_void,
        available: size_t,
    ) -> ssize_t;
    fn strlen(s: *const c_char) -> size_t;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn file_ns_capable(file: *mut file, ns: *const c_void, cap: c_int) -> bool_;
    fn kstrtobool_from_user(s: *const c_char, count: size_t, res: *mut bool_) -> c_int;
    fn ipe_set_active_pol(p: *const ipe_policy) -> c_int;
    fn memdup_user(src: *const c_char, len: size_t) -> *mut c_char;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn kfree(ptr: *const c_void);
    fn ipe_update_policy(
        root: *mut inode,
        pkcs7: *const c_void,
        pkcs7len: size_t,
        text: *mut c_char,
        textlen: size_t,
    ) -> c_int;
    fn ERR_PTR(error: c_int) -> *mut ipe_policy;
    fn ipe_audit_policy_load(p: *mut ipe_policy);
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn lockdep_is_held(lock: *const c_void) -> c_int;
    fn rcu_dereference_protected(p: *mut ipe_policy, c: c_int) -> *mut ipe_policy;
    fn synchronize_rcu();
    fn ipe_free_policy(p: *mut ipe_policy);
    fn securityfs_remove(dentry: *mut dentry);
    fn securityfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn securityfs_create_file(
        name: *const c_char,
        mode: umode_t,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
}

unsafe fn rcu_access_pointer(p: *mut ipe_policy) -> *mut ipe_policy {
    p
}

/**
 * read_pkcs7() - Read handler for "ipe/policies/$name/pkcs7".
 * @f: Supplies a file structure representing the securityfs node.
 * @data: Supplies a buffer passed to the write syscall.
 * @len: Supplies the length of @data.
 * @offset: unused.
 *
 * @data will be populated with the pkcs7 blob representing the policy
 * on success. If the policy is unsigned (like the boot policy), this
 * will return -ENOENT.
 *
 * Return:
 * * Length of buffer written	- Success
 * * %-ENOENT			- Policy initializing/deleted or is unsigned
 */
unsafe extern "C" fn read_pkcs7(
    f: *mut file,
    data: *mut c_char,
    len: size_t,
    offset: *mut loff_t,
) -> ssize_t {
    let mut p: *const ipe_policy = core::ptr::null();
    let root: *mut inode;
    let mut rc: c_int = 0;

    root = d_inode((*(*(*f).f_path.dentry).d_parent));

    inode_lock_shared(root);
    p = (*root).i_private as *mut ipe_policy;
    if p.is_null() {
        rc = -ENOENT;
    } else if (*p).pkcs7.is_null() {
        rc = -ENOENT;
    } else {
        rc = simple_read_from_buffer(data, len, offset, (*p).pkcs7, (*p).pkcs7len) as c_int;
    }

    inode_unlock_shared(root);

    rc as ssize_t
}

/**
 * read_policy() - Read handler for "ipe/policies/$name/policy".
 * @f: Supplies a file structure representing the securityfs node.
 * @data: Supplies a buffer passed to the write syscall.
 * @len: Supplies the length of @data.
 * @offset: unused.
 *
 * @data will be populated with the plain-text version of the policy
 * on success.
 *
 * Return:
 * * Length of buffer written	- Success
 * * %-ENOENT			- Policy initializing/deleted
 */
unsafe extern "C" fn read_policy(
    f: *mut file,
    data: *mut c_char,
    len: size_t,
    offset: *mut loff_t,
) -> ssize_t {
    let mut p: *const ipe_policy = core::ptr::null();
    let root: *mut inode;
    let mut rc: c_int = 0;

    root = d_inode((*(*(*f).f_path.dentry).d_parent));

    inode_lock_shared(root);
    p = (*root).i_private as *mut ipe_policy;
    if p.is_null() {
        rc = -ENOENT;
    } else {
        rc = simple_read_from_buffer(data, len, offset, (*p).text, (*p).textlen) as c_int;
    }

    inode_unlock_shared(root);

    rc as ssize_t
}

/**
 * read_name() - Read handler for "ipe/policies/$name/name".
 * @f: Supplies a file structure representing the securityfs node.
 * @data: Supplies a buffer passed to the write syscall.
 * @len: Supplies the length of @data.
 * @offset: unused.
 *
 * @data will be populated with the policy_name attribute on success.
 *
 * Return:
 * * Length of buffer written	- Success
 * * %-ENOENT			- Policy initializing/deleted
 */
unsafe extern "C" fn read_name(
    f: *mut file,
    data: *mut c_char,
    len: size_t,
    offset: *mut loff_t,
) -> ssize_t {
    let mut p: *const ipe_policy = core::ptr::null();
    let root: *mut inode;
    let mut rc: c_int = 0;

    root = d_inode((*(*(*f).f_path.dentry).d_parent));

    inode_lock_shared(root);
    p = (*root).i_private as *mut ipe_policy;
    if p.is_null() {
        rc = -ENOENT;
    } else {
        rc = simple_read_from_buffer(
            data,
            len,
            offset,
            (*(*p).parsed).name as *const c_void,
            strlen((*(*p).parsed).name),
        ) as c_int;
    }

    inode_unlock_shared(root);

    rc as ssize_t
}

/**
 * read_version() - Read handler for "ipe/policies/$name/version".
 * @f: Supplies a file structure representing the securityfs node.
 * @data: Supplies a buffer passed to the write syscall.
 * @len: Supplies the length of @data.
 * @offset: unused.
 *
 * @data will be populated with the version string on success.
 *
 * Return:
 * * Length of buffer written	- Success
 * * %-ENOENT			- Policy initializing/deleted
 */
unsafe extern "C" fn read_version(
    f: *mut file,
    data: *mut c_char,
    len: size_t,
    offset: *mut loff_t,
) -> ssize_t {
    let mut buffer: [c_char; MAX_VERSION_SIZE] = [0; MAX_VERSION_SIZE];
    let mut p: *const ipe_policy = core::ptr::null();
    let root: *mut inode;
    let mut strsize: size_t = 0;
    let mut rc: ssize_t = 0;

    root = d_inode((*(*(*f).f_path.dentry).d_parent));

    inode_lock_shared(root);
    p = (*root).i_private as *mut ipe_policy;
    if p.is_null() {
        rc = -ENOENT as ssize_t;
    } else {
        strsize = scnprintf(
            buffer.as_mut_ptr(),
            buffer.len(),
            c"%hu.%hu.%hu".as_ptr(),
            (*(*p).parsed).version.major as c_int,
            (*(*p).parsed).version.minor as c_int,
            (*(*p).parsed).version.rev as c_int,
        ) as size_t;

        rc = simple_read_from_buffer(data, len, offset, buffer.as_ptr() as *const c_void, strsize);
    }

    inode_unlock_shared(root);

    rc
}

/**
 * setactive() - Write handler for "ipe/policies/$name/active".
 * @f: Supplies a file structure representing the securityfs node.
 * @data: Supplies a buffer passed to the write syscall.
 * @len: Supplies the length of @data.
 * @offset: unused.
 *
 * Return:
 * * Length of buffer written	- Success
 * * %-EPERM			- Insufficient permission
 * * %-EINVAL			- Invalid input
 * * %-ENOENT			- Policy initializing/deleted
 */
unsafe extern "C" fn setactive(
    f: *mut file,
    data: *const c_char,
    len: size_t,
    _offset: *mut loff_t,
) -> ssize_t {
    let mut p: *const ipe_policy = core::ptr::null();
    let root: *mut inode;
    let mut value: bool_ = false;
    let mut rc: c_int = 0;

    if !file_ns_capable(f, &init_user_ns, CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }

    rc = kstrtobool_from_user(data, len, &mut value);
    if rc != 0 {
        return rc as ssize_t;
    }

    if !value {
        return -EINVAL as ssize_t;
    }

    root = d_inode((*(*(*f).f_path.dentry).d_parent));
    inode_lock(root);

    p = (*root).i_private as *mut ipe_policy;
    if p.is_null() {
        rc = -ENOENT;
    } else {
        rc = ipe_set_active_pol(p);
    }

    inode_unlock(root);
    if rc < 0 {
        rc as ssize_t
    } else {
        len as ssize_t
    }
}

/**
 * getactive() - Read handler for "ipe/policies/$name/active".
 * @f: Supplies a file structure representing the securityfs node.
 * @data: Supplies a buffer passed to the write syscall.
 * @len: Supplies the length of @data.
 * @offset: unused.
 *
 * @data will be populated with the 1 or 0 depending on if the
 * corresponding policy is active.
 *
 * Return:
 * * Length of buffer written	- Success
 * * %-ENOENT			- Policy initializing/deleted
 */
unsafe extern "C" fn getactive(
    f: *mut file,
    data: *mut c_char,
    len: size_t,
    offset: *mut loff_t,
) -> ssize_t {
    let mut p: *const ipe_policy = core::ptr::null();
    let root: *mut inode;
    let str_: *const c_char;
    let rc: c_int;

    root = d_inode((*(*(*f).f_path.dentry).d_parent));

    inode_lock_shared(root);
    p = (*root).i_private as *mut ipe_policy;
    if p.is_null() {
        inode_unlock_shared(root);
        return -ENOENT as ssize_t;
    }
    inode_unlock_shared(root);

    str_ = if p == rcu_access_pointer(ipe_active_policy) {
        c"1".as_ptr()
    } else {
        c"0".as_ptr()
    };
    rc = simple_read_from_buffer(data, len, offset, str_ as *const c_void, 1) as c_int;

    rc as ssize_t
}

/**
 * update_policy() - Write handler for "ipe/policies/$name/update".
 * @f: Supplies a file structure representing the securityfs node.
 * @data: Supplies a buffer passed to the write syscall.
 * @len: Supplies the length of @data.
 * @offset: unused.
 *
 * On success this updates the policy represented by $name,
 * in-place.
 *
 * Return:
 * * Length of buffer written		- Success
 * * %-EPERM				- Insufficient permission
 * * %-ENOMEM				- Out of memory (OOM)
 * * %-ENOENT				- Policy was deleted while updating
 * * %-EINVAL				- Policy name mismatch
 * * %-ESTALE				- Policy version too old
 */
unsafe extern "C" fn update_policy(
    f: *mut file,
    data: *const c_char,
    len: size_t,
    _offset: *mut loff_t,
) -> ssize_t {
    let mut root: *mut inode = core::ptr::null_mut();
    let mut copy: *mut c_char = core::ptr::null_mut();
    let mut rc: c_int = 0;

    if !file_ns_capable(f, &init_user_ns, CAP_MAC_ADMIN) {
        rc = -EPERM;
    } else {
        copy = memdup_user(data, len);
        if IS_ERR(copy as *const c_void) {
            rc = PTR_ERR(copy as *const c_void);
            copy = core::ptr::null_mut();
        } else {
            root = d_inode((*(*(*f).f_path.dentry).d_parent));
            inode_lock(root);
            rc = ipe_update_policy(root, core::ptr::null(), 0, copy, len);
            inode_unlock(root);
        }
    }

    kfree(copy as *const c_void);
    if rc != 0 {
        ipe_audit_policy_load(ERR_PTR(rc));
        return rc as ssize_t;
    }

    len as ssize_t
}

/**
 * delete_policy() - write handler for  "ipe/policies/$name/delete".
 * @f: Supplies a file structure representing the securityfs node.
 * @data: Supplies a buffer passed to the write syscall.
 * @len: Supplies the length of @data.
 * @offset: unused.
 *
 * On success this deletes the policy represented by $name.
 *
 * Return:
 * * Length of buffer written	- Success
 * * %-EPERM			- Insufficient permission/deleting active policy
 * * %-EINVAL			- Invalid input
 * * %-ENOENT			- Policy initializing/deleted
 */
unsafe extern "C" fn delete_policy(
    f: *mut file,
    data: *const c_char,
    len: size_t,
    _offset: *mut loff_t,
) -> ssize_t {
    let mut ap: *mut ipe_policy;
    let mut p: *mut ipe_policy;
    let root: *mut inode;
    let mut value: bool_ = false;
    let mut rc: c_int = 0;

    if !file_ns_capable(f, &init_user_ns, CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }

    rc = kstrtobool_from_user(data, len, &mut value);
    if rc != 0 {
        return rc as ssize_t;
    }

    if !value {
        return -EINVAL as ssize_t;
    }

    root = d_inode((*(*(*f).f_path.dentry).d_parent));
    inode_lock(root);
    p = (*root).i_private as *mut ipe_policy;
    if p.is_null() {
        inode_unlock(root);
        return -ENOENT as ssize_t;
    }

    mutex_lock(&mut ipe_policy_lock);
    ap = rcu_dereference_protected(ipe_active_policy, lockdep_is_held(&ipe_policy_lock));
    if p == ap {
        mutex_unlock(&mut ipe_policy_lock);
        inode_unlock(root);
        return -EPERM as ssize_t;
    }
    mutex_unlock(&mut ipe_policy_lock);

    (*root).i_private = core::ptr::null_mut();
    inode_unlock(root);

    synchronize_rcu();
    ipe_free_policy(p);

    len as ssize_t
}

static content_fops: file_operations = file_operations {
    read: Some(read_policy),
    write: None,
};

static pkcs7_fops: file_operations = file_operations {
    read: Some(read_pkcs7),
    write: None,
};

static name_fops: file_operations = file_operations {
    read: Some(read_name),
    write: None,
};

static ver_fops: file_operations = file_operations {
    read: Some(read_version),
    write: None,
};

static active_fops: file_operations = file_operations {
    write: Some(setactive),
    read: Some(getactive),
};

static update_fops: file_operations = file_operations {
    write: Some(update_policy),
    read: None,
};

static delete_fops: file_operations = file_operations {
    write: Some(delete_policy),
    read: None,
};

/*
 * policy_subdir - files under a policy subdirectory
 */
static policy_subdir: [ipefs_file; 7] = [
    ipefs_file {
        name: c"pkcs7".as_ptr(),
        access: 0o444,
        fops: &pkcs7_fops,
    },
    ipefs_file {
        name: c"policy".as_ptr(),
        access: 0o444,
        fops: &content_fops,
    },
    ipefs_file {
        name: c"name".as_ptr(),
        access: 0o444,
        fops: &name_fops,
    },
    ipefs_file {
        name: c"version".as_ptr(),
        access: 0o444,
        fops: &ver_fops,
    },
    ipefs_file {
        name: c"active".as_ptr(),
        access: 0o600,
        fops: &active_fops,
    },
    ipefs_file {
        name: c"update".as_ptr(),
        access: 0o200,
        fops: &update_fops,
    },
    ipefs_file {
        name: c"delete".as_ptr(),
        access: 0o200,
        fops: &delete_fops,
    },
];

/**
 * ipe_del_policyfs_node() - Delete a securityfs entry for @p.
 * @p: Supplies a pointer to the policy to delete a securityfs entry for.
 */
#[no_mangle]
pub unsafe extern "C" fn ipe_del_policyfs_node(p: *mut ipe_policy) {
    securityfs_remove((*p).policyfs);
    (*p).policyfs = core::ptr::null_mut();
}

/**
 * ipe_new_policyfs_node() - Create a securityfs entry for @p.
 * @p: Supplies a pointer to the policy to create a securityfs entry for.
 *
 * Return: %0 on success. If an error occurs, the function will return
 * the -errno.
 */
#[no_mangle]
pub unsafe extern "C" fn ipe_new_policyfs_node(p: *mut ipe_policy) -> c_int {
    let mut f: *const ipefs_file;
    let mut policyfs: *mut dentry;
    let root: *mut inode;
    let mut d: *mut dentry;
    let mut i: size_t = 0;
    let mut rc: c_int = 0;

    if !(*p).policyfs.is_null() {
        return 0;
    }

    policyfs = securityfs_create_dir((*(*p).parsed).name, policy_root);
    if IS_ERR(policyfs as *const c_void) {
        return PTR_ERR(policyfs as *const c_void);
    }

    root = d_inode(policyfs);

    while i < policy_subdir.len() {
        f = &policy_subdir[i];

        d = securityfs_create_file(
            (*f).name,
            (*f).access,
            policyfs,
            core::ptr::null_mut(),
            (*f).fops,
        );
        if IS_ERR(d as *const c_void) {
            rc = PTR_ERR(d as *const c_void);
            securityfs_remove(policyfs);
            return rc;
        }

        i += 1;
    }

    inode_lock(root);
    (*p).policyfs = policyfs;
    (*root).i_private = p as *mut c_void;
    inode_unlock(root);

    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
