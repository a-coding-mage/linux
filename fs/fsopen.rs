// SPDX-License-Identifier: GPL-2.0-or-later
/* Filesystem access-by-fd.
 *
 * Copyright (C) 2017 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel headers and symbols are supplied by the surrounding kernel translation.

#[inline]
unsafe fn fetch_message_locked(log: *mut fs_log, len: usize, need_free: *mut bool) -> *const core::ffi::c_char {
    let mut index: usize;
    let p: *const core::ffi::c_char;

    if unlikely((*log).head == (*log).tail) {
        return ERR_PTR(-ENODATA);
    }

    index = ((*log).tail as usize) & (ARRAY_SIZE((*log).buffer) - 1);
    p = (*log).buffer[index];
    if unlikely(strlen(p) > len) {
        return ERR_PTR(-EMSGSIZE);
    }

    (*log).buffer[index] = core::ptr::null();
    *need_free = ((*log).need_free & (1 << index)) != 0;
    (*log).need_free &= !(1 << index);
    (*log).tail += 1;

    p
}

/*
 * Allow the user to read back any error, warning or informational messages.
 * Only one message is returned for each read(2) call.
 */
unsafe fn fscontext_read(file: *mut file, buf: *mut core::ffi::c_void, len: usize, _pos: *mut loff_t) -> isize {
    let fc = (*file).private_data as *mut fs_context;
    let err: isize;
    let message: *const core::ffi::c_char;
    let mut need_free = false;
    let mut p: *const core::ffi::c_char = core::ptr::null();
    let n: usize;

    err = mutex_lock_interruptible(&mut (*fc).uapi_mutex);
    if err < 0 {
        return err;
    }
    message = fetch_message_locked((*fc).log.log, len, &mut need_free);
    mutex_unlock(&mut (*fc).uapi_mutex);
    if IS_ERR(message) {
        return PTR_ERR(message);
    }

    if need_free {
        p = message;
    }

    n = strlen(message);
    if copy_to_user(buf, message as *const core::ffi::c_void, n) != 0 {
        return -EFAULT;
    }
    n as isize
}

unsafe fn fscontext_release(_inode: *mut inode, file: *mut file) -> i32 {
    let fc = (*file).private_data as *mut fs_context;

    if !fc.is_null() {
        (*file).private_data = core::ptr::null_mut();
        put_fs_context(fc);
    }
    0
}

#[no_mangle]
pub static fscontext_fops: file_operations = file_operations {
    read: Some(fscontext_read),
    release: Some(fscontext_release),
};

/*
 * Attach a filesystem context to a file and an fd.
 */
unsafe fn fscontext_create_fd(fc: *mut fs_context, o_flags: u32) -> i32 {
    let fd = anon_inode_getfd(b"[fscontext]\0".as_ptr() as *const _, &fscontext_fops, fc, O_RDWR | o_flags);
    if fd < 0 {
        put_fs_context(fc);
    }
    fd
}

unsafe fn fscontext_alloc_log(fc: *mut fs_context) -> i32 {
    (*fc).log.log = kzalloc_obj();
    if (*fc).log.log.is_null() {
        return -ENOMEM;
    }
    refcount_set(&mut (*(*fc).log.log).usage, 1);
    (*(*fc).log.log).owner = (*fc).fs_type.owner;
    0
}

/*
 * Open a filesystem by name so that it can be configured for mounting.
 *
 * We are allowed to specify a container in which the filesystem will be
 * opened, thereby indicating which namespaces will be used (notably, which
 * network namespace will be used for network filesystems).
 */
#[no_mangle]
pub unsafe extern "C" fn fsopen(_fs_name: *const core::ffi::c_char, flags: u32) -> i32 {
    let fs_type: *mut file_system_type;
    let fc: *mut fs_context;
    let fs_name: *mut core::ffi::c_char;
    let ret: i32;

    if !may_mount() { return -EPERM; }
    if flags & !FSOPEN_CLOEXEC != 0 { return -EINVAL; }

    fs_name = strndup_user(_fs_name, PAGE_SIZE);
    if IS_ERR(fs_name) { return PTR_ERR(fs_name); }
    fs_type = get_fs_type(fs_name);
    kfree(fs_name as *mut core::ffi::c_void);
    if fs_type.is_null() { return -ENODEV; }

    fc = fs_context_for_mount(fs_type, 0);
    put_filesystem(fs_type);
    if IS_ERR(fc) { return PTR_ERR(fc); }
    (*fc).phase = FS_CONTEXT_CREATE_PARAMS;

    ret = fscontext_alloc_log(fc);
    if ret < 0 { put_fs_context(fc); return ret; }
    fscontext_create_fd(fc, if flags & FSOPEN_CLOEXEC != 0 { O_CLOEXEC } else { 0 })
}

/*
 * Pick a superblock into a context for reconfiguration.
 */
#[no_mangle]
pub unsafe extern "C" fn fspick(dfd: i32, path: *const core::ffi::c_char, flags: u32) -> i32 {
    let mut fc: *mut fs_context;
    let mut target = core::mem::MaybeUninit::<path>::uninit();
    let mut lookup_flags: u32;
    let mut ret: i32;

    if !may_mount() { return -EPERM; }
    if flags & !(FSPICK_CLOEXEC | FSPICK_SYMLINK_NOFOLLOW | FSPICK_NO_AUTOMOUNT | FSPICK_EMPTY_PATH) != 0 { return -EINVAL; }
    lookup_flags = LOOKUP_FOLLOW | LOOKUP_AUTOMOUNT;
    if flags & FSPICK_SYMLINK_NOFOLLOW != 0 { lookup_flags &= !LOOKUP_FOLLOW; }
    if flags & FSPICK_NO_AUTOMOUNT != 0 { lookup_flags &= !LOOKUP_AUTOMOUNT; }
    let filename = filename_flags(path, if flags & FSPICK_EMPTY_PATH != 0 { LOOKUP_EMPTY } else { 0 });
    ret = filename_lookup(dfd, filename, lookup_flags, target.as_mut_ptr(), core::ptr::null_mut());
    if ret < 0 { return ret; }
    ret = -EINVAL;
    if (*target.as_ptr()).mnt.mnt_root != (*target.as_ptr()).dentry { path_put(target.as_mut_ptr()); return ret; }
    fc = fs_context_for_reconfigure((*target.as_ptr()).dentry, 0, 0);
    if IS_ERR(fc) { ret = PTR_ERR(fc); path_put(target.as_mut_ptr()); return ret; }
    (*fc).phase = FS_CONTEXT_RECONF_PARAMS;
    ret = fscontext_alloc_log(fc);
    if ret < 0 { put_fs_context(fc); path_put(target.as_mut_ptr()); return ret; }
    path_put(target.as_mut_ptr());
    fscontext_create_fd(fc, if flags & FSPICK_CLOEXEC != 0 { O_CLOEXEC } else { 0 })
}

unsafe fn vfs_cmd_create(fc: *mut fs_context, exclusive: bool) -> i32 {
    let sb: *mut super_block;
    let ret: i32;
    if (*fc).phase != FS_CONTEXT_CREATE_PARAMS { return -EBUSY; }
    if !mount_capable(fc) { return -EPERM; }
    (*fc).phase = FS_CONTEXT_CREATING;
    (*fc).exclusive = exclusive;
    ret = vfs_get_tree(fc);
    if ret != 0 { (*fc).phase = FS_CONTEXT_FAILED; return ret; }
    sb = (*(*fc).root).d_sb;
    ret = security_sb_kern_mount(sb);
    if ret != 0 { fc_drop_locked(fc); (*fc).phase = FS_CONTEXT_FAILED; return ret; }
    up_write(&mut (*sb).s_umount);
    (*fc).phase = FS_CONTEXT_AWAITING_MOUNT;
    0
}

unsafe fn vfs_cmd_reconfigure(fc: *mut fs_context) -> i32 {
    let sb: *mut super_block;
    let ret: i32;
    if (*fc).phase != FS_CONTEXT_RECONF_PARAMS { return -EBUSY; }
    (*fc).phase = FS_CONTEXT_RECONFIGURING;
    sb = (*(*fc).root).d_sb;
    if !ns_capable((*sb).s_user_ns, CAP_SYS_ADMIN) { (*fc).phase = FS_CONTEXT_FAILED; return -EPERM; }
    down_write(&mut (*sb).s_umount);
    ret = reconfigure_super(fc);
    up_write(&mut (*sb).s_umount);
    if ret != 0 { (*fc).phase = FS_CONTEXT_FAILED; return ret; }
    vfs_clean_context(fc);
    0
}

unsafe fn vfs_fsconfig_locked(fc: *mut fs_context, cmd: i32, param: *mut fs_parameter) -> i32 {
    let ret = finish_clean_context(fc);
    if ret != 0 { return ret; }
    match cmd {
        FSCONFIG_CMD_CREATE => vfs_cmd_create(fc, false),
        FSCONFIG_CMD_CREATE_EXCL => vfs_cmd_create(fc, true),
        FSCONFIG_CMD_RECONFIGURE => vfs_cmd_reconfigure(fc),
        _ => {
            if (*fc).phase != FS_CONTEXT_CREATE_PARAMS && (*fc).phase != FS_CONTEXT_RECONF_PARAMS { return -EBUSY; }
            vfs_parse_fs_param(fc, param)
        }
    }
}

// sys_fsconfig - Set parameters and trigger actions on a context.
#[no_mangle]
pub unsafe extern "C" fn fsconfig(fd: i32, cmd: u32, key: *const core::ffi::c_char, value: *const core::ffi::c_void, aux: i32) -> i32 {
    // The full syscall parameter validation and cleanup follows the C implementation.
    if fd < 0 { return -EINVAL; }
    match cmd {
        FSCONFIG_SET_FLAG if key.is_null() || !value.is_null() || aux != 0 => return -EINVAL,
        FSCONFIG_SET_STRING if key.is_null() || value.is_null() || aux != 0 => return -EINVAL,
        FSCONFIG_SET_BINARY if key.is_null() || value.is_null() || aux <= 0 || aux > 1024 * 1024 => return -EINVAL,
        FSCONFIG_SET_PATH | FSCONFIG_SET_PATH_EMPTY if key.is_null() || value.is_null() || (aux != AT_FDCWD && aux < 0) => return -EINVAL,
        FSCONFIG_SET_FD if key.is_null() || !value.is_null() || aux < 0 => return -EINVAL,
        FSCONFIG_CMD_CREATE | FSCONFIG_CMD_CREATE_EXCL | FSCONFIG_CMD_RECONFIGURE if !key.is_null() || !value.is_null() || aux != 0 => return -EINVAL,
        FSCONFIG_SET_FLAG | FSCONFIG_SET_STRING | FSCONFIG_SET_BINARY | FSCONFIG_SET_PATH | FSCONFIG_SET_PATH_EMPTY | FSCONFIG_SET_FD | FSCONFIG_CMD_CREATE | FSCONFIG_CMD_CREATE_EXCL | FSCONFIG_CMD_RECONFIGURE => {},
        _ => return -EOPNOTSUPP,
    }
    let mut param = fs_parameter { type_: fs_value_is_undefined, ..core::mem::zeroed() };
    let f = fdget(fd);
    if fd_empty(&f) { return -EBADF; }
    if (*fd_file(&f)).f_op != &fscontext_fops { return -EINVAL; }
    let fc = (*fd_file(&f)).private_data as *mut fs_context;
    if !key.is_null() {
        param.key = strndup_user(key, 256);
        if IS_ERR(param.key) { return PTR_ERR(param.key); }
    }
    let mut ret: i32 = 0;
    match cmd {
        FSCONFIG_SET_FLAG => param.type_ = fs_value_is_flag,
        FSCONFIG_SET_STRING => { param.type_ = fs_value_is_string; param.string = strndup_user(value as *const _, 256); if IS_ERR(param.string) { ret = PTR_ERR(param.string); } else { param.size = strlen(param.string); } },
        FSCONFIG_SET_BINARY => { param.type_ = fs_value_is_blob; param.size = aux as usize; param.blob = memdup_user_nul(value, aux as usize); if IS_ERR(param.blob) { ret = PTR_ERR(param.blob); } },
        FSCONFIG_SET_PATH_EMPTY => { param.type_ = fs_value_is_filename; param.name = getname_flags(value as *const _, LOOKUP_EMPTY); if IS_ERR(param.name) { ret = PTR_ERR(param.name); } else { param.dirfd = aux; param.size = strlen((*param.name).name); } },
        FSCONFIG_SET_PATH => { param.type_ = fs_value_is_filename; param.name = getname_flags(value as *const _, 0); if IS_ERR(param.name) { ret = PTR_ERR(param.name); } else { param.dirfd = aux; param.size = strlen((*param.name).name); } },
        FSCONFIG_SET_FD => { param.type_ = fs_value_is_file; param.file = fget_raw(aux); if param.file.is_null() { ret = -EBADF; } else { param.dirfd = aux; } },
        _ => {}
    }
    if ret == 0 {
        ret = mutex_lock_interruptible(&mut (*fc).uapi_mutex);
        if ret == 0 { ret = vfs_fsconfig_locked(fc, cmd as i32, &mut param); mutex_unlock(&mut (*fc).uapi_mutex); }
    }
    match cmd {
        FSCONFIG_SET_STRING | FSCONFIG_SET_BINARY => kfree(param.string as *mut _),
        FSCONFIG_SET_PATH | FSCONFIG_SET_PATH_EMPTY => if !param.name.is_null() { putname(param.name); },
        FSCONFIG_SET_FD => if !param.file.is_null() { fput(param.file); },
        _ => {}
    }
    kfree(param.key as *mut _);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
