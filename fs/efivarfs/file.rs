// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Red Hat, Inc.
 * Copyright (C) 2012 Jeremy Kerr <jeremy.kerr@canonical.com>
 */

// Linux EFI, delay, filesystem, slab, mount, and efivarfs-internal
// dependencies are supplied by the surrounding translation unit.

unsafe fn efivarfs_file_write(
    file: *mut file,
    userbuf: *const core::ffi::c_char,
    count: usize,
    ppos: *mut loff_t,
) -> ssize_t {
    let var = (*file).private_data as *mut efivar_entry;
    let mut data: *mut core::ffi::c_void;
    let mut attributes: u32 = 0;
    let inode = (*(*file).f_mapping).host;
    let mut datasize = count.wrapping_sub(core::mem::size_of::<u32>());
    let mut bytes: ssize_t;
    let mut set = false;

    if count < core::mem::size_of::<u32>() {
        return -EINVAL;
    }

    if copy_from_user(
        &mut attributes as *mut u32 as *mut core::ffi::c_void,
        userbuf as *const core::ffi::c_void,
        core::mem::size_of::<u32>(),
    ) != 0 {
        return -EFAULT;
    }

    if attributes & !EFI_VARIABLE_MASK != 0 {
        return -EINVAL;
    }

    data = memdup_user(
        userbuf.add(core::mem::size_of::<u32>()) as *const core::ffi::c_void,
        datasize,
    );
    if IS_ERR(data) {
        return PTR_ERR(data);
    }

    inode_lock(inode);
    if (*var).removed {
        /* file got removed; don't allow a set. */
        bytes = -EIO;
        goto_out(inode, data, bytes);
    }

    bytes = efivar_entry_set_get_size(var, attributes, &mut datasize, data, &mut set);
    if !set {
        if bytes == -ENOENT {
            bytes = -EIO;
        }
        goto_out(inode, data, bytes);
    }

    if bytes == -ENOENT {
        i_size_write(inode, 0);
    } else {
        i_size_write(inode, datasize + core::mem::size_of::<u32>());
        inode_set_mtime_to_ts(inode, inode_set_ctime_current(inode));
    }

    bytes = count as ssize_t;
    inode_unlock(inode);
    kfree(data);
    bytes
}

unsafe fn goto_out(inode: *mut inode, data: *mut core::ffi::c_void, bytes: ssize_t) -> ssize_t {
    inode_unlock(inode);
    kfree(data);
    bytes
}

unsafe fn efivarfs_file_read(
    file: *mut file,
    userbuf: *mut core::ffi::c_char,
    count: usize,
    ppos: *mut loff_t,
) -> ssize_t {
    let var = (*file).private_data as *mut efivar_entry;
    let mut datasize: usize = 0;
    let mut attributes: u32 = 0;
    let data: *mut core::ffi::c_void;
    let mut size: ssize_t = 0;
    let mut err: int;

    while !__ratelimit(&mut (*(*(*file).f_cred).user).ratelimit) {
        msleep(50);
    }

    err = efivar_entry_size(var, &mut datasize);
    if err == -ENOENT {
        return 0;
    } else if err != 0 {
        return err as ssize_t;
    }

    data = kmalloc(datasize + core::mem::size_of::<u32>(), GFP_KERNEL);
    if data.is_null() {
        return -ENOMEM;
    }

    size = efivar_entry_get(var, &mut attributes, &mut datasize, data.add(core::mem::size_of::<u32>()));
    if size != 0 {
        kfree(data);
        return size;
    }

    memcpy(data, &attributes as *const u32 as *const core::ffi::c_void, core::mem::size_of::<u32>());
    size = simple_read_from_buffer(userbuf, count, ppos, data, datasize + core::mem::size_of::<u32>());
    kfree(data);
    size
}

unsafe fn efivarfs_file_release(inode: *mut inode, file: *mut file) -> int {
    let var = (*inode).i_private as *mut efivar_entry;
    inode_lock(inode);
    (*var).open_count -= 1;
    (*var).removed = (*var).open_count == 0 && i_size_read(inode) == 0;
    inode_unlock(inode);
    if (*var).removed {
        simple_recursive_removal((*(*file).f_path).dentry, core::ptr::null_mut());
    }
    0
}

unsafe fn efivarfs_file_open(inode: *mut inode, file: *mut file) -> int {
    let entry = (*inode).i_private as *mut efivar_entry;
    (*file).private_data = entry as *mut core::ffi::c_void;
    inode_lock(inode);
    (*entry).open_count += 1;
    inode_unlock(inode);
    0
}

#[no_mangle]
pub static efivarfs_file_operations: file_operations = file_operations {
    open: Some(efivarfs_file_open),
    read: Some(efivarfs_file_read),
    write: Some(efivarfs_file_write),
    release: Some(efivarfs_file_release),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
