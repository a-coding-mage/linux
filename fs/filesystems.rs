// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/filesystems.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  table of configured filesystems
 */

// Kernel dependencies supplied by the surrounding translation unit.

static mut FILE_SYSTEMS: HListHead = HListHead::new();
static mut FILE_SYSTEMS_LOCK: SpinLock = SpinLock::new();

#[cfg(CONFIG_PROC_FS)]
struct FileSystemsString {
    rcu: RcuHead,
    gen: c_ulong,
    len: usize,
    string: [c_char; 0],
}

#[cfg(CONFIG_PROC_FS)]
static mut FILE_SYSTEMS_GEN: c_ulong = 0;
#[cfg(CONFIG_PROC_FS)]
static mut FILE_SYSTEMS_STRING: *mut FileSystemsString = core::ptr::null_mut();

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
unsafe fn invalidate_filesystems_string() {}

/* WARNING: This can be used only if we _already_ own a reference */
unsafe fn get_filesystem(fs: *mut FileSystemType) -> *mut FileSystemType {
    __module_get((*fs).owner);
    fs
}

unsafe fn put_filesystem(fs: *mut FileSystemType) {
    module_put((*fs).owner);
}

unsafe fn find_filesystem(name: *const c_char, len: c_uint) -> *mut FileSystemType {
    let mut fs: *mut FileSystemType;
    hlist_for_each_entry_rcu!(fs, &mut FILE_SYSTEMS, list,
        lockdep_is_held(&FILE_SYSTEMS_LOCK), {
            if strncmp((*fs).name, name, len) == 0 && *(*fs).name.add(len as usize) == 0 {
                return fs;
            }
        });
    core::ptr::null_mut()
}

pub unsafe fn register_filesystem(fs: *mut FileSystemType) -> c_int {
    if !(*fs).parameters.is_null() && !fs_validate_description((*fs).name, (*fs).parameters) {
        return -EINVAL;
    }

    BUG_ON!(strchr((*fs).name, b'.' as c_int).is_null());
    if !hlist_unhashed_lockless(&mut (*fs).list) {
        return -EBUSY;
    }

    let _guard = GuardSpinLock::new(&mut FILE_SYSTEMS_LOCK);
    if !find_filesystem((*fs).name, strlen((*fs).name) as c_uint).is_null() {
        return -EBUSY;
    }
    hlist_add_tail_rcu(&mut (*fs).list, &mut FILE_SYSTEMS);
    invalidate_filesystems_string();
    0
}

pub unsafe fn unregister_filesystem(fs: *mut FileSystemType) -> c_int {
    let _guard = ScopedGuardSpinLock::new(&mut FILE_SYSTEMS_LOCK);
    if hlist_unhashed(&mut (*fs).list) {
        return -EINVAL;
    }
    hlist_del_init_rcu(&mut (*fs).list);
    invalidate_filesystems_string();
    synchronize_rcu();
    0
}

#[cfg(CONFIG_SYSFS_SYSCALL)]
unsafe fn fs_index(name: *const c_char) -> c_int {
    let mut p: *mut FileSystemType;
    let name = strndup_user(name, PATH_MAX);
    if IS_ERR(name) {
        return PTR_ERR(name) as c_int;
    }
    let _guard = RcuGuard::new();
    let mut index = 0;
    hlist_for_each_entry_rcu!(p, &mut FILE_SYSTEMS, list, {
        if strcmp((*p).name, name) == 0 { return index; }
        index += 1;
    });
    -EINVAL
}

#[cfg(CONFIG_SYSFS_SYSCALL)]
unsafe fn fs_name(mut index: c_uint, buf: *mut c_char) -> c_int {
    let mut p: *mut FileSystemType;
    let mut found: *mut FileSystemType = core::ptr::null_mut();
    let _guard = RcuGuard::new();
    hlist_for_each_entry_rcu!(p, &mut FILE_SYSTEMS, list, {
        if index != 0 { index -= 1; continue; }
        if try_module_get((*p).owner) { found = p; }
        break;
    });
    if found.is_null() { return -EINVAL; }
    let len = strlen((*found).name) + 1;
    let res = if copy_to_user(buf, (*found).name, len) != 0 { -EFAULT } else { 0 };
    put_filesystem(found);
    res
}

#[cfg(CONFIG_SYSFS_SYSCALL)]
unsafe fn fs_maxindex() -> c_int {
    let mut p: *mut FileSystemType;
    let _guard = RcuGuard::new();
    let mut index = 0;
    hlist_for_each_entry_rcu!(p, &mut FILE_SYSTEMS, list, { index += 1; });
    index
}

#[cfg(CONFIG_SYSFS_SYSCALL)]
pub unsafe fn sysfs(option: c_int, arg1: c_ulong, arg2: c_ulong) -> c_int {
    let mut retval = -EINVAL;
    match option {
        1 => retval = fs_index(arg1 as *const c_char),
        2 => retval = fs_name(arg1 as c_uint, arg2 as *mut c_char),
        3 => retval = fs_maxindex(),
        _ => {}
    }
    retval
}

pub unsafe fn list_bdev_fs_names(mut buf: *mut c_char, mut size: usize) -> c_int {
    let mut p: *mut FileSystemType;
    let _guard = RcuGuard::new();
    let mut count = 0;
    hlist_for_each_entry_rcu!(p, &mut FILE_SYSTEMS, list, {
        if (*p).fs_flags & FS_REQUIRES_DEV == 0 { continue; }
        let len = strlen((*p).name) + 1;
        if len > size {
            pr_warn!("%s: truncating file system list\n", __func__);
            break;
        }
        memcpy(buf, (*p).name, len);
        buf = buf.add(len);
        size -= len;
        count += 1;
    });
    count
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn invalidate_filesystems_string() {
    lockdep_assert_held_write(&FILE_SYSTEMS_LOCK);
    FILE_SYSTEMS_GEN += 1;
    let old = rcu_replace_pointer(&mut FILE_SYSTEMS_STRING, core::ptr::null_mut(),
                                   lockdep_is_held(&FILE_SYSTEMS_LOCK));
    if !old.is_null() { kfree_rcu(old, rcu); }
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn regen_filesystems_string() -> c_int {
    loop {
        let mut newlen = 0usize;
        spin_lock(&mut FILE_SYSTEMS_LOCK);
        let gen = FILE_SYSTEMS_GEN;
        let mut p: *mut FileSystemType;
        hlist_for_each_entry_rcu!(p, &mut FILE_SYSTEMS, list, {
            if (*p).fs_flags & FS_REQUIRES_DEV == 0 { newlen += strlen(b"nodev\0" as *const u8 as *const c_char); }
            newlen += strlen(b"\t\0" as *const u8 as *const c_char) + strlen((*p).name) + strlen(b"\n\0" as *const u8 as *const c_char);
        });
        spin_unlock(&mut FILE_SYSTEMS_LOCK);
        let new = kmalloc(core::mem::offset_of!(FileSystemsString, string) + newlen + 1, GFP_KERNEL) as *mut FileSystemsString;
        if new.is_null() { return -ENOMEM; }
        (*new).gen = gen; (*new).len = newlen;
        spin_lock(&mut FILE_SYSTEMS_LOCK);
        let old = FILE_SYSTEMS_STRING;
        if !old.is_null() && (*old).gen == FILE_SYSTEMS_GEN { spin_unlock(&mut FILE_SYSTEMS_LOCK); kfree(new); return 0; }
        if gen != FILE_SYSTEMS_GEN { spin_unlock(&mut FILE_SYSTEMS_LOCK); kfree(new); continue; }
        let mut usedlen = 0;
        hlist_for_each_entry_rcu!(p, &mut FILE_SYSTEMS, list, {
            usedlen += sprintf((*new).string.as_mut_ptr().add(usedlen), b"%s\t%s\n\0".as_ptr() as *const c_char,
                if (*p).fs_flags & FS_REQUIRES_DEV != 0 { b"\0".as_ptr() } else { b"nodev\0".as_ptr() }, (*p).name);
        });
        if WARN_ON_ONCE!((*new).len != strlen((*new).string.as_ptr())) { spin_unlock(&mut FILE_SYSTEMS_LOCK); kfree(new); return -EINVAL; }
        rcu_assign_pointer(&mut FILE_SYSTEMS_STRING, new);
        spin_unlock(&mut FILE_SYSTEMS_LOCK);
        if !old.is_null() { kfree_rcu(old, rcu); }
        return 0;
    }
}

unsafe fn __get_fs_type(name: *const c_char, len: c_int) -> *mut FileSystemType {
    let _guard = RcuGuard::new();
    let mut fs = find_filesystem(name, len as c_uint);
    if !fs.is_null() && !try_module_get((*fs).owner) { fs = core::ptr::null_mut(); }
    fs
}

pub unsafe fn get_fs_type(name: *const c_char) -> *mut FileSystemType {
    let dot = strchr(name, b'.' as c_int);
    let len = if dot.is_null() { strlen(name) as c_int } else { dot.offset_from(name) as c_int };
    let mut fs = __get_fs_type(name, len);
    if fs.is_null() && request_module(b"fs-%.*s\0".as_ptr() as *const c_char, len, name) == 0 {
        fs = __get_fs_type(name, len);
        if fs.is_null() { pr_warn_once!("request_module fs-%.*s succeeded, but still no fs?\n", len, name); }
    }
    if !dot.is_null() && !fs.is_null() && (*fs).fs_flags & FS_HAS_SUBTYPE == 0 {
        put_filesystem(fs); fs = core::ptr::null_mut();
    }
    fs
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
