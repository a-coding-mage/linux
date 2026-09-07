// SPDX-License-Identifier: GPL-2.0
/*
 * security/tomoyo/mount.c
 *
 * Copyright (C) 2005-2011  NTT DATA CORPORATION
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

/* Dependencies supplied by the surrounding kernel/TOMOYO translation. */
/* #include <linux/slab.h> */
/* #include <uapi/linux/mount.h> */
/* #include "common.h" */

/* String table for special mount operations. */
static tomoyo_mounts: [*const c_char; TOMOYO_MAX_SPECIAL_MOUNT] = [
    /* TOMOYO_MOUNT_BIND */            b"--bind\0".as_ptr() as *const c_char,
    /* TOMOYO_MOUNT_MOVE */            b"--move\0".as_ptr() as *const c_char,
    /* TOMOYO_MOUNT_REMOUNT */         b"--remount\0".as_ptr() as *const c_char,
    /* TOMOYO_MOUNT_MAKE_UNBINDABLE */ b"--make-unbindable\0".as_ptr() as *const c_char,
    /* TOMOYO_MOUNT_MAKE_PRIVATE */    b"--make-private\0".as_ptr() as *const c_char,
    /* TOMOYO_MOUNT_MAKE_SLAVE */      b"--make-slave\0".as_ptr() as *const c_char,
    /* TOMOYO_MOUNT_MAKE_SHARED */     b"--make-shared\0".as_ptr() as *const c_char,
];

/**
 * tomoyo_audit_mount_log - Audit mount log.
 *
 * @r: Pointer to "struct tomoyo_request_info".
 *
 * Returns 0 on success, negative value otherwise.
 */
unsafe fn tomoyo_audit_mount_log(r: *mut tomoyo_request_info) -> c_int {
    tomoyo_supervisor(
        r,
        b"file mount %s %s %s 0x%lX\n\0".as_ptr() as *const c_char,
        (*(*r).param.mount.dev).name,
        (*(*r).param.mount.dir).name,
        (*(*r).param.mount.type_).name,
        (*r).param.mount.flags,
    )
}

/**
 * tomoyo_check_mount_acl - Check permission for path path path number operation.
 *
 * @r:   Pointer to "struct tomoyo_request_info".
 * @ptr: Pointer to "struct tomoyo_acl_info".
 *
 * Returns true if granted, false otherwise.
 */
unsafe fn tomoyo_check_mount_acl(
    r: *mut tomoyo_request_info,
    ptr: *const tomoyo_acl_info,
) -> bool {
    let acl = container_of_mount_acl(ptr);

    tomoyo_compare_number_union((*r).param.mount.flags, &(*acl).flags)
        && tomoyo_compare_name_union((*r).param.mount.type_, &(*acl).fs_type)
        && tomoyo_compare_name_union((*r).param.mount.dir, &(*acl).dir_name)
        && (!(*r).param.mount.need_dev
            || tomoyo_compare_name_union((*r).param.mount.dev, &(*acl).dev_name))
}

/**
 * tomoyo_mount_acl - Check permission for mount() operation.
 *
 * @r:        Pointer to "struct tomoyo_request_info".
 * @dev_name: Name of device file. Maybe NULL.
 * @dir:      Pointer to "struct path".
 * @type:     Name of filesystem type.
 * @flags:    Mount options.
 *
 * Returns 0 on success, negative value otherwise.
 *
 * Caller holds tomoyo_read_lock().
 */
unsafe fn tomoyo_mount_acl(
    r: *mut tomoyo_request_info,
    mut dev_name: *const c_char,
    dir: *const path,
    type_: *const c_char,
    mut flags: c_ulong,
) -> c_int {
    let mut obj: tomoyo_obj_info = core::mem::zeroed();
    let mut path = core::mem::zeroed::<path>();
    let mut fstype: *mut file_system_type = core::ptr::null_mut();
    let mut requested_type: *const c_char = core::ptr::null();
    let mut requested_dir_name: *const c_char = core::ptr::null();
    let mut requested_dev_name: *const c_char = core::ptr::null();
    let mut rtype: tomoyo_path_info = core::mem::zeroed();
    let mut rdev: tomoyo_path_info = core::mem::zeroed();
    let mut rdir: tomoyo_path_info = core::mem::zeroed();
    let mut need_dev: c_int = 0;
    let mut error: c_int = -ENOMEM;

    (*r).obj = &mut obj;

    /* Get fstype. */
    requested_type = tomoyo_encode(type_);
    if requested_type.is_null() {
        goto_out!();
    }
    rtype.name = requested_type;
    tomoyo_fill_path_info(&mut rtype);

    /* Get mount point. */
    obj.path2 = *dir;
    requested_dir_name = tomoyo_realpath_from_path(dir);
    if requested_dir_name.is_null() {
        error = -ENOMEM;
        goto_out!();
    }
    rdir.name = requested_dir_name;
    tomoyo_fill_path_info(&mut rdir);

    /* Compare fs name. */
    if type_ == tomoyo_mounts[TOMOYO_MOUNT_REMOUNT] {
        /* dev_name is ignored. */
    } else if type_ == tomoyo_mounts[TOMOYO_MOUNT_MAKE_UNBINDABLE]
        || type_ == tomoyo_mounts[TOMOYO_MOUNT_MAKE_PRIVATE]
        || type_ == tomoyo_mounts[TOMOYO_MOUNT_MAKE_SLAVE]
        || type_ == tomoyo_mounts[TOMOYO_MOUNT_MAKE_SHARED]
    {
        /* dev_name is ignored. */
    } else if type_ == tomoyo_mounts[TOMOYO_MOUNT_BIND]
        || type_ == tomoyo_mounts[TOMOYO_MOUNT_MOVE]
    {
        need_dev = -1; /* dev_name is a directory */
    } else {
        fstype = get_fs_type(type_);
        if fstype.is_null() {
            error = -ENODEV;
            goto_out!();
        }
        if (*fstype).fs_flags & FS_REQUIRES_DEV != 0 {
            /* dev_name is a block device file. */
            need_dev = 1;
        }
    }
    if need_dev != 0 {
        /* Get mount point or device file. */
        if dev_name.is_null() || kern_path(dev_name, LOOKUP_FOLLOW, &mut path) != 0 {
            error = -ENOENT;
            goto_out!();
        }
        obj.path1 = path;
        requested_dev_name = tomoyo_realpath_from_path(&path);
        if requested_dev_name.is_null() {
            error = -ENOENT;
            goto_out!();
        }
    } else {
        /* Map dev_name to "<NULL>" if no dev_name given. */
        if dev_name.is_null() {
            dev_name = b"<NULL>\0".as_ptr() as *const c_char;
        }
        requested_dev_name = tomoyo_encode(dev_name);
        if requested_dev_name.is_null() {
            error = -ENOMEM;
            goto_out!();
        }
    }
    rdev.name = requested_dev_name;
    tomoyo_fill_path_info(&mut rdev);
    (*r).param_type = TOMOYO_TYPE_MOUNT_ACL;
    (*r).param.mount.need_dev = need_dev;
    (*r).param.mount.dev = &mut rdev;
    (*r).param.mount.dir = &mut rdir;
    (*r).param.mount.type_ = &mut rtype;
    (*r).param.mount.flags = flags;
    loop {
        tomoyo_check_acl(r, tomoyo_check_mount_acl);
        error = tomoyo_audit_mount_log(r);
        if error != TOMOYO_RETRY_REQUEST {
            break;
        }
    }

    goto_out!();

    macro_rules! goto_out {
        () => {
            kfree(requested_dev_name as *mut c_void);
            kfree(requested_dir_name as *mut c_void);
            if !fstype.is_null() {
                put_filesystem(fstype);
            }
            kfree(requested_type as *mut c_void);
            /* Drop refcount obtained by kern_path(). */
            if !obj.path1.dentry.is_null() {
                path_put(&mut obj.path1);
            }
            return error;
        };
    }
}

/**
 * tomoyo_mount_permission - Check permission for mount() operation.
 *
 * @dev_name:  Name of device file. Maybe NULL.
 * @path:      Pointer to "struct path".
 * @type:      Name of filesystem type. Maybe NULL.
 * @flags:     Mount options.
 * @data_page: Optional data. Maybe NULL.
 *
 * Returns 0 on success, negative value otherwise.
 */
unsafe fn tomoyo_mount_permission(
    dev_name: *const c_char,
    path: *const path,
    mut type_: *const c_char,
    mut flags: c_ulong,
    _data_page: *mut c_void,
) -> c_int {
    let mut r: tomoyo_request_info = core::mem::zeroed();
    let error: c_int;
    let idx: c_int;

    if tomoyo_init_request_info(&mut r, core::ptr::null_mut(), TOMOYO_MAC_FILE_MOUNT)
        == TOMOYO_CONFIG_DISABLED
    {
        return 0;
    }
    if (flags & MS_MGC_MSK) == MS_MGC_VAL {
        flags &= !MS_MGC_MSK;
    }
    if flags & MS_REMOUNT != 0 {
        type_ = tomoyo_mounts[TOMOYO_MOUNT_REMOUNT];
        flags &= !MS_REMOUNT;
    } else if flags & MS_BIND != 0 {
        type_ = tomoyo_mounts[TOMOYO_MOUNT_BIND];
        flags &= !MS_BIND;
    } else if flags & MS_SHARED != 0 {
        if flags & (MS_PRIVATE | MS_SLAVE | MS_UNBINDABLE) != 0 {
            return -EINVAL;
        }
        type_ = tomoyo_mounts[TOMOYO_MOUNT_MAKE_SHARED];
        flags &= !MS_SHARED;
    } else if flags & MS_PRIVATE != 0 {
        if flags & (MS_SHARED | MS_SLAVE | MS_UNBINDABLE) != 0 {
            return -EINVAL;
        }
        type_ = tomoyo_mounts[TOMOYO_MOUNT_MAKE_PRIVATE];
        flags &= !MS_PRIVATE;
    } else if flags & MS_SLAVE != 0 {
        if flags & (MS_SHARED | MS_PRIVATE | MS_UNBINDABLE) != 0 {
            return -EINVAL;
        }
        type_ = tomoyo_mounts[TOMOYO_MOUNT_MAKE_SLAVE];
        flags &= !MS_SLAVE;
    } else if flags & MS_UNBINDABLE != 0 {
        if flags & (MS_SHARED | MS_PRIVATE | MS_SLAVE) != 0 {
            return -EINVAL;
        }
        type_ = tomoyo_mounts[TOMOYO_MOUNT_MAKE_UNBINDABLE];
        flags &= !MS_UNBINDABLE;
    } else if flags & MS_MOVE != 0 {
        type_ = tomoyo_mounts[TOMOYO_MOUNT_MOVE];
        flags &= !MS_MOVE;
    }
    if type_.is_null() {
        type_ = b"<NULL>\0".as_ptr() as *const c_char;
    }
    idx = tomoyo_read_lock();
    error = tomoyo_mount_acl(&mut r, dev_name, path, type_, flags);
    tomoyo_read_unlock(idx);
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
