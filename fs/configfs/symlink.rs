// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * symlink.c - operations for configfs symlinks.
 *
 * Based on sysfs:
 *     sysfs is Copyright (C) 2001, 2002, 2003 Patrick Mochel
 *
 * configfs Copyright (C) 2005 Oracle.  All rights reserved.
 */

// Dependencies supplied by the surrounding kernel/configfs translation.

pub static mut configfs_symlink_mutex: mutex = mutex::default();

unsafe fn item_depth(mut item: *mut config_item) -> i32 {
    let mut p = item;
    let mut depth = 0;
    loop {
        depth += 1;
        p = (*p).ci_parent;
        if p.is_null() || configfs_is_root(p) {
            break;
        }
    }
    depth
}

unsafe fn item_path_length(mut item: *mut config_item) -> i32 {
    let mut p = item;
    let mut length = 1;
    loop {
        length += strlen(config_item_name(p)) as i32 + 1;
        p = (*p).ci_parent;
        if p.is_null() || configfs_is_root(p) {
            break;
        }
    }
    length
}

unsafe fn fill_item_path(item: *mut config_item, mut buffer: *mut c_char, mut length: i32) {
    let mut p = item;
    length -= 1;
    while !p.is_null() && !configfs_is_root(p) {
        let cur = strlen(config_item_name(p)) as i32;
        length -= cur;
        memcpy(buffer.add(length as usize), config_item_name(p), cur as usize);
        length -= 1;
        *buffer.add(length as usize) = b'/' as c_char;
        p = (*p).ci_parent;
    }
}

unsafe fn configfs_get_target_path(
    item: *mut config_item,
    target: *mut config_item,
    path: *mut c_char,
) -> i32 {
    let depth = item_depth(item);
    let size = item_path_length(target) + depth * 3 - 1;
    if size > PATH_MAX {
        return -ENAMETOOLONG;
    }

    pr_debug!("%s: depth = %d, size = %d\n", "configfs_get_target_path", depth, size);
    let mut s = path;
    let mut remaining = depth;
    while remaining != 0 {
        strcpy(s, b"../\0".as_ptr() as *const c_char);
        s = s.add(3);
        remaining -= 1;
    }
    fill_item_path(target, path, size);
    pr_debug!("%s: path = '%s'\n", "configfs_get_target_path", path);
    0
}

unsafe fn create_link(
    parent_item: *mut config_item,
    item: *mut config_item,
    dentry: *mut dentry,
) -> i32 {
    let target_sd = (*(*item).ci_dentry).d_fsdata as *mut configfs_dirent;
    let body = kzalloc(PAGE_SIZE, GFP_KERNEL) as *mut c_char;
    if body.is_null() {
        return -ENOMEM;
    }

    configfs_get(target_sd);
    spin_lock(&raw mut configfs_dirent_lock);
    if (*target_sd).s_type & CONFIGFS_USET_DROPPING != 0 {
        spin_unlock(&raw mut configfs_dirent_lock);
        configfs_put(target_sd);
        kfree(body as *mut c_void);
        return -ENOENT;
    }
    (*target_sd).s_links += 1;
    spin_unlock(&raw mut configfs_dirent_lock);
    let mut ret = configfs_get_target_path(parent_item, item, body);
    if ret == 0 {
        ret = configfs_create_link(target_sd, (*parent_item).ci_dentry, dentry, body);
    }
    if ret != 0 {
        spin_lock(&raw mut configfs_dirent_lock);
        (*target_sd).s_links -= 1;
        spin_unlock(&raw mut configfs_dirent_lock);
        configfs_put(target_sd);
        kfree(body as *mut c_void);
    }
    ret
}

unsafe fn get_target(
    symname: *const c_char,
    target: *mut *mut config_item,
    sb: *mut super_block,
) -> i32 {
    let mut path: path = core::mem::zeroed();
    let ret = kern_path(symname, LOOKUP_FOLLOW | LOOKUP_DIRECTORY, &mut path);
    if ret != 0 {
        return ret;
    }
    if (*path.dentry).d_sb != sb {
        return -EPERM;
    }
    *target = configfs_get_config_item(path.dentry);
    if (*target).is_null() {
        return -ENOENT;
    }
    0
}

pub unsafe extern "C" fn configfs_symlink(
    _idmap: *mut mnt_idmap,
    dir: *mut inode,
    dentry: *mut dentry,
    symname: *const c_char,
) -> i32 {
    let sd = (*(*dentry).d_parent).d_fsdata as *mut configfs_dirent;
    if !configfs_dirent_is_ready(sd) {
        return -ENOENT;
    }
    let parent_item = configfs_get_config_item((*dentry).d_parent);
    let item_type = (*parent_item).ci_type;
    let mut ret = -EPERM;
    if item_type.is_null() || (*item_type).ct_item_ops.is_null()
        || (*(*item_type).ct_item_ops).allow_link.is_none() {
        config_item_put(parent_item);
        return ret;
    }

    inode_unlock(dir);
    let mut target_item: *mut config_item = core::ptr::null_mut();
    ret = get_target(symname, &mut target_item, (*dentry).d_sb);
    inode_lock(dir);
    if ret == 0 {
        if !(*dentry).d_inode.is_null() || d_unhashed(dentry) {
            ret = -EEXIST;
        } else {
            ret = inode_permission(&nop_mnt_idmap, dir, MAY_WRITE | MAY_EXEC);
        }
        if ret == 0 {
            ret = ((*(*item_type).ct_item_ops).allow_link.unwrap())(parent_item, target_item);
        }
        if ret == 0 {
            mutex_lock(&raw mut configfs_symlink_mutex);
            ret = create_link(parent_item, target_item, dentry);
            mutex_unlock(&raw mut configfs_symlink_mutex);
            if ret != 0 && (*(*item_type).ct_item_ops).drop_link.is_some() {
                ((*(*item_type).ct_item_ops).drop_link.unwrap())(parent_item, target_item);
            }
        }
        config_item_put(target_item);
    }
    config_item_put(parent_item);
    ret
}

pub unsafe extern "C" fn configfs_unlink(dir: *mut inode, dentry: *mut dentry) -> i32 {
    let sd = (*dentry).d_fsdata as *mut configfs_dirent;
    let mut ret = -EPERM;
    if (*sd).s_type & CONFIGFS_ITEM_LINK == 0 {
        return ret;
    }
    let target_sd = (*sd).s_element;
    let parent_item = configfs_get_config_item((*dentry).d_parent);
    let item_type = (*parent_item).ci_type;
    spin_lock(&raw mut configfs_dirent_lock);
    list_del_init(&mut (*sd).s_sibling);
    spin_unlock(&raw mut configfs_dirent_lock);
    configfs_put(sd);
    simple_unlink(dir, dentry);
    if !item_type.is_null() && (*item_type).ct_item_ops != core::ptr::null_mut()
        && (*(*item_type).ct_item_ops).drop_link.is_some() {
        ((*(*item_type).ct_item_ops).drop_link.unwrap())(parent_item, (*target_sd).s_element);
    }
    spin_lock(&raw mut configfs_dirent_lock);
    (*target_sd).s_links -= 1;
    spin_unlock(&raw mut configfs_dirent_lock);
    configfs_put(target_sd);
    config_item_put(parent_item);
    ret = 0;
    ret
}

pub static configfs_symlink_inode_operations: inode_operations = inode_operations {
    get_link: Some(simple_get_link),
    setattr: Some(configfs_setattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
