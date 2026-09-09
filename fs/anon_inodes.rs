// SPDX-License-Identifier: GPL-2.0-only
/*
 *  fs/anon_inodes.c
 *
 *  Copyright (C) 2007  Davide Libenzi <davidel@xmailserver.org>
 *
 *  Thanks to Arnd Bergmann for code review and suggestions.
 *  More changes for Thomas Gleixner suggestions.
 */

// Kernel dependencies supplied by other translation units.

static mut anon_inode_mnt: *mut vfsmount = core::ptr::null_mut();
static mut anon_inode_inode: *mut inode = core::ptr::null_mut();

/*
 * User space expects anonymous inodes to have no file type in st_mode.
 *
 * Rather than mess with our internal sane inode data, fix it up in getattr()
 * by masking off the format bits.
 */
pub unsafe extern "C" fn anon_inode_getattr(
    _idmap: *mut mnt_idmap,
    path: *const path,
    stat: *mut kstat,
    request_mask: u32,
    _query_flags: u32,
) -> i32 {
    let inode = d_inode((*path).dentry);
    generic_fillattr(&nop_mnt_idmap, request_mask, inode, stat);
    (*stat).mode &= !S_IFMT;
    0
}

pub unsafe extern "C" fn anon_inode_setattr(
    _idmap: *mut mnt_idmap,
    _dentry: *mut dentry,
    _attr: *mut iattr,
) -> i32 {
    -EOPNOTSUPP
}

static anon_inode_operations: inode_operations = inode_operations {
    getattr: Some(anon_inode_getattr),
    setattr: Some(anon_inode_setattr),
};

/* anon_inodefs_dname() is called from d_path(). */
unsafe extern "C" fn anon_inodefs_dname(
    dentry: *mut dentry,
    buffer: *mut core::ffi::c_char,
    buflen: i32,
) -> *mut core::ffi::c_char {
    dynamic_dname(buffer, buflen, c"anon_inode:%s".as_ptr(), (*dentry).d_name.name)
}

static anon_inodefs_dentry_operations: dentry_operations = dentry_operations {
    d_dname: Some(anon_inodefs_dname),
};

unsafe extern "C" fn anon_inodefs_init_fs_context(fc: *mut fs_context) -> i32 {
    let ctx = init_pseudo(fc, ANON_INODE_FS_MAGIC);
    if ctx.is_null() {
        return -ENOMEM;
    }
    (*ctx).dops = &anon_inodefs_dentry_operations;
    0
}

static mut anon_inode_fs_type: file_system_type = file_system_type {
    name: c"anon_inodefs".as_ptr(),
    init_fs_context: Some(anon_inodefs_init_fs_context),
    kill_sb: Some(kill_anon_super),
};

/// Allocate an anonymous inode with security context.
pub unsafe extern "C" fn anon_inode_make_secure_inode(
    sb: *mut super_block,
    name: *const core::ffi::c_char,
    context_inode: *const inode,
) -> *mut inode {
    let inode = alloc_anon_inode(sb);
    if IS_ERR(inode) {
        return inode;
    }
    (*inode).i_flags &= !S_PRIVATE;
    (*inode).i_op = &anon_inode_operations;
    let error = security_inode_init_security_anon(inode, &QSTR(name), context_inode);
    if error != 0 {
        iput(inode);
        return ERR_PTR(error);
    }
    inode
}

// EXPORT_SYMBOL_FOR_MODULES(anon_inode_make_secure_inode, "kvm");

unsafe fn __anon_inode_getfile(
    name: *const core::ffi::c_char,
    fops: *const file_operations,
    priv_: *mut core::ffi::c_void,
    flags: i32,
    context_inode: *const inode,
    make_inode: bool,
) -> *mut file {
    let mut inode: *mut inode;
    let file: *mut file;

    if !(*fops).owner.is_null() && !try_module_get((*fops).owner) {
        return ERR_PTR(-ENOENT);
    }

    if make_inode {
        inode = anon_inode_make_secure_inode((*anon_inode_mnt).mnt_sb, name, context_inode);
        if IS_ERR(inode) {
            file = ERR_CAST(inode);
            goto_err(fops, file);
            return file;
        }
    } else {
        inode = anon_inode_inode;
        if IS_ERR(inode) {
            file = ERR_PTR(-ENODEV);
            goto_err(fops, file);
            return file;
        }
        ihold(inode);
    }

    file = alloc_file_pseudo(
        inode,
        anon_inode_mnt,
        name,
        flags & (O_ACCMODE | O_NONBLOCK),
        fops,
    );
    if IS_ERR(file) {
        iput(inode);
        goto_err(fops, file);
        return file;
    }
    (*file).f_mapping = (*inode).i_mapping;
    (*file).private_data = priv_;
    file
}

unsafe fn goto_err(fops: *const file_operations, file: *mut file) {
    let _ = file;
    module_put((*fops).owner);
}

pub unsafe extern "C" fn anon_inode_getfile(
    name: *const core::ffi::c_char,
    fops: *const file_operations,
    priv_: *mut core::ffi::c_void,
    flags: i32,
) -> *mut file {
    __anon_inode_getfile(name, fops, priv_, flags, core::ptr::null(), false)
}

// EXPORT_SYMBOL_GPL(anon_inode_getfile);

pub unsafe extern "C" fn anon_inode_getfile_fmode(
    name: *const core::ffi::c_char,
    fops: *const file_operations,
    priv_: *mut core::ffi::c_void,
    flags: i32,
    f_mode: fmode_t,
) -> *mut file {
    let file = __anon_inode_getfile(name, fops, priv_, flags, core::ptr::null(), false);
    if !IS_ERR(file) {
        (*file).f_mode |= f_mode;
    }
    file
}

// EXPORT_SYMBOL_GPL(anon_inode_getfile_fmode);

pub unsafe extern "C" fn anon_inode_create_getfile(
    name: *const core::ffi::c_char,
    fops: *const file_operations,
    priv_: *mut core::ffi::c_void,
    flags: i32,
    context_inode: *const inode,
) -> *mut file {
    __anon_inode_getfile(name, fops, priv_, flags, context_inode, true)
}

// EXPORT_SYMBOL_GPL(anon_inode_create_getfile);

unsafe fn __anon_inode_getfd(
    name: *const core::ffi::c_char,
    fops: *const file_operations,
    priv_: *mut core::ffi::c_void,
    flags: i32,
    context_inode: *const inode,
    make_inode: bool,
) -> i32 {
    FD_ADD(
        flags,
        __anon_inode_getfile(name, fops, priv_, flags, context_inode, make_inode),
    )
}

pub unsafe extern "C" fn anon_inode_getfd(
    name: *const core::ffi::c_char,
    fops: *const file_operations,
    priv_: *mut core::ffi::c_void,
    flags: i32,
) -> i32 {
    __anon_inode_getfd(name, fops, priv_, flags, core::ptr::null(), false)
}

// EXPORT_SYMBOL_GPL(anon_inode_getfd);

pub unsafe extern "C" fn anon_inode_create_getfd(
    name: *const core::ffi::c_char,
    fops: *const file_operations,
    priv_: *mut core::ffi::c_void,
    flags: i32,
    context_inode: *const inode,
) -> i32 {
    __anon_inode_getfd(name, fops, priv_, flags, context_inode, true)
}

unsafe fn anon_inode_init() -> i32 {
    anon_inode_mnt = kern_mount(&anon_inode_fs_type);
    if IS_ERR(anon_inode_mnt) {
        panic!("anon_inode_init() kernel mount failed (%ld)\n", PTR_ERR(anon_inode_mnt));
    }

    anon_inode_inode = alloc_anon_inode((*anon_inode_mnt).mnt_sb);
    if IS_ERR(anon_inode_inode) {
        panic!("anon_inode_init() inode allocation failed (%ld)\n", PTR_ERR(anon_inode_inode));
    }
    (*anon_inode_inode).i_op = &anon_inode_operations;
    0
}

// fs_initcall(anon_inode_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
