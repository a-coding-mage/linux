/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from include/linux/fsnotify.h. */

/* Dependencies supplied by the surrounding kernel translation are intentionally not implemented here. */

#[inline]
pub unsafe fn fsnotify_sb_has_priority_watchers(sb: *mut super_block, prio: i32) -> bool {
    let sbinfo = fsnotify_sb_info(sb);
    if sbinfo.is_null() {
        return false;
    }
    atomic_long_read(&mut (*sbinfo).watched_objects[prio as usize]) != 0
}

#[inline]
pub unsafe fn fsnotify_sb_has_watchers(sb: *mut super_block) -> bool {
    fsnotify_sb_has_priority_watchers(sb, 0)
}

#[inline]
pub unsafe fn fsnotify_name(
    mask: u32, data: *const core::ffi::c_void, data_type: i32,
    dir: *mut inode, name: *const qstr, cookie: u32,
) -> i32 {
    if !fsnotify_sb_has_watchers((*dir).i_sb) { return 0; }
    fsnotify(mask, data, data_type, dir, name, core::ptr::null_mut(), cookie)
}

#[inline]
pub unsafe fn fsnotify_dirent(dir: *mut inode, dentry: *mut dentry, mask: u32) {
    fsnotify_name(mask, dentry.cast(), FSNOTIFY_EVENT_DENTRY, dir, &(*dentry).d_name, 0);
}

#[inline]
pub unsafe fn fsnotify_inode(inode: *mut inode, mut mask: u32) {
    if !fsnotify_sb_has_watchers((*inode).i_sb) { return; }
    if S_ISDIR((*inode).i_mode) { mask |= FS_ISDIR; }
    fsnotify(mask, inode.cast(), FSNOTIFY_EVENT_INODE, core::ptr::null_mut(), core::ptr::null(), inode, 0);
}

#[inline]
pub unsafe fn fsnotify_parent(dentry: *mut dentry, mut mask: u32, data: *const core::ffi::c_void, data_type: i32) -> i32 {
    let inode = d_inode(dentry);
    if !fsnotify_sb_has_watchers((*inode).i_sb) { return 0; }
    if S_ISDIR((*inode).i_mode) {
        mask |= FS_ISDIR;
        if ((*dentry).d_flags & DCACHE_FSNOTIFY_PARENT_WATCHED) == 0 {
            return fsnotify(mask, data, data_type, core::ptr::null_mut(), core::ptr::null(), inode, 0);
        }
    }
    if IS_ROOT(dentry) {
        return fsnotify(mask, data, data_type, core::ptr::null_mut(), core::ptr::null(), inode, 0);
    }
    __fsnotify_parent(dentry, mask, data, data_type)
}

#[inline]
pub unsafe fn fsnotify_dentry(dentry: *mut dentry, mask: u32) { fsnotify_parent(dentry, mask, dentry.cast(), FSNOTIFY_EVENT_DENTRY); }

#[inline]
pub unsafe fn fsnotify_path(path: *const path, mask: u32) -> i32 { fsnotify_parent((*path).dentry, mask, path.cast(), FSNOTIFY_EVENT_PATH) }

#[inline]
pub unsafe fn fsnotify_file(file: *mut file, mask: u32) -> i32 {
    if FMODE_FSNOTIFY_NONE((*file).f_mode) { return 0; }
    fsnotify_path(&(*file).f_path, mask)
}

/* CONFIG_FANOTIFY_ACCESS_PERMISSIONS selects the permission-hook implementations. */
#[cfg(CONFIG_FANOTIFY_ACCESS_PERMISSIONS)]
extern "C" { pub fn fsnotify_open_perm_and_set_mode(file: *mut file) -> i32; }

#[cfg(CONFIG_FANOTIFY_ACCESS_PERMISSIONS)]
#[inline]
pub unsafe fn fsnotify_file_area_perm(file: *mut file, perm_mask: i32, ppos: *const loff_t, count: usize) -> i32 {
    lockdep_assert_once(file_write_not_started(file));
    if (perm_mask & (MAY_READ | MAY_WRITE | MAY_ACCESS)) == 0 { return 0; }
    if FMODE_FSNOTIFY_HSM((*file).f_mode) {
        let ret = fsnotify_pre_content(&(*file).f_path, ppos, count);
        if ret != 0 { return ret; }
    }
    if (perm_mask & MAY_READ) == 0 || !FMODE_FSNOTIFY_ACCESS_PERM((*file).f_mode) { return 0; }
    fsnotify_path(&(*file).f_path, FS_ACCESS_PERM)
}

#[cfg(CONFIG_FANOTIFY_ACCESS_PERMISSIONS)]
#[inline]
pub unsafe fn fsnotify_mmap_perm(file: *mut file, _prot: i32, off: loff_t, len: usize) -> i32 {
    if file.is_null() || !FMODE_FSNOTIFY_HSM((*file).f_mode) { return 0; }
    fsnotify_pre_content(&(*file).f_path, &off, len)
}

#[cfg(CONFIG_FANOTIFY_ACCESS_PERMISSIONS)]
#[inline]
pub unsafe fn fsnotify_truncate_perm(path: *const path, length: loff_t) -> i32 {
    let inode = d_inode((*path).dentry);
    if ((*(*inode).i_sb).s_iflags & SB_I_ALLOW_HSM) == 0 || !fsnotify_sb_has_priority_watchers((*inode).i_sb, FSNOTIFY_PRIO_PRE_CONTENT) { return 0; }
    fsnotify_pre_content(path, &length, 0)
}

#[cfg(CONFIG_FANOTIFY_ACCESS_PERMISSIONS)]
#[inline]
pub unsafe fn fsnotify_file_perm(file: *mut file, perm_mask: i32) -> i32 { fsnotify_file_area_perm(file, perm_mask, core::ptr::null(), 0) }

#[cfg(not(CONFIG_FANOTIFY_ACCESS_PERMISSIONS))]
#[inline] pub unsafe fn fsnotify_open_perm_and_set_mode(_file: *mut file) -> i32 { 0 }
#[cfg(not(CONFIG_FANOTIFY_ACCESS_PERMISSIONS))]
#[inline] pub unsafe fn fsnotify_file_area_perm(_file: *mut file, _perm_mask: i32, _ppos: *const loff_t, _count: usize) -> i32 { 0 }
#[cfg(not(CONFIG_FANOTIFY_ACCESS_PERMISSIONS))]
#[inline] pub unsafe fn fsnotify_mmap_perm(_file: *mut file, _prot: i32, _off: loff_t, _len: usize) -> i32 { 0 }
#[cfg(not(CONFIG_FANOTIFY_ACCESS_PERMISSIONS))]
#[inline] pub unsafe fn fsnotify_truncate_perm(_path: *const path, _length: loff_t) -> i32 { 0 }
#[cfg(not(CONFIG_FANOTIFY_ACCESS_PERMISSIONS))]
#[inline] pub unsafe fn fsnotify_file_perm(_file: *mut file, _perm_mask: i32) -> i32 { 0 }

#[inline] pub unsafe fn fsnotify_link_count(inode: *mut inode) { fsnotify_inode(inode, FS_ATTRIB); }

#[inline]
pub unsafe fn fsnotify_move(old_dir: *mut inode, new_dir: *mut inode, old_name: *const qstr, isdir: i32, target: *mut inode, moved: *mut dentry) {
    let source = (*moved).d_inode;
    let fs_cookie = fsnotify_get_cookie();
    let mut old_dir_mask = FS_MOVED_FROM; let mut new_dir_mask = FS_MOVED_TO; let mut rename_mask = FS_RENAME;
    let new_name = &(*moved).d_name;
    let rd = fsnotify_rename_data { moved, target };
    if isdir != 0 { old_dir_mask |= FS_ISDIR; new_dir_mask |= FS_ISDIR; rename_mask |= FS_ISDIR; }
    fsnotify_name(rename_mask, (&rd as *const _).cast(), FSNOTIFY_EVENT_RENAME, old_dir, old_name, 0);
    fsnotify_name(old_dir_mask, source.cast(), FSNOTIFY_EVENT_INODE, old_dir, old_name, fs_cookie);
    fsnotify_name(new_dir_mask, (&rd as *const _).cast(), FSNOTIFY_EVENT_RENAME, new_dir, new_name, fs_cookie);
    if !target.is_null() { fsnotify_link_count(target); }
    fsnotify_inode(source, FS_MOVE_SELF);
    audit_inode_child(new_dir, moved, AUDIT_TYPE_CHILD_CREATE);
}

#[inline] pub unsafe fn fsnotify_inode_delete(inode: *mut inode) { __fsnotify_inode_delete(inode); }
#[inline] pub unsafe fn fsnotify_vfsmount_delete(mnt: *mut vfsmount) { __fsnotify_vfsmount_delete(mnt); }
#[inline] pub unsafe fn fsnotify_mntns_delete(mntns: *mut mnt_namespace) { __fsnotify_mntns_delete(mntns); }
#[inline] pub unsafe fn fsnotify_inoderemove(inode: *mut inode) { fsnotify_inode(inode, FS_DELETE_SELF); __fsnotify_inode_delete(inode); }

#[inline] pub unsafe fn fsnotify_create(dir: *mut inode, dentry: *mut dentry) { audit_inode_child(dir, dentry, AUDIT_TYPE_CHILD_CREATE); fsnotify_dirent(dir, dentry, FS_CREATE); }
#[inline] pub unsafe fn fsnotify_link(dir: *mut inode, inode: *mut inode, new_dentry: *mut dentry) { fsnotify_link_count(inode); audit_inode_child(dir, new_dentry, AUDIT_TYPE_CHILD_CREATE); fsnotify_name(FS_CREATE, inode.cast(), FSNOTIFY_EVENT_INODE, dir, &(*new_dentry).d_name, 0); }

#[inline]
pub unsafe fn fsnotify_delete(dir: *mut inode, inode: *mut inode, dentry: *mut dentry) { let mut mask = FS_DELETE; if S_ISDIR((*inode).i_mode) { mask |= FS_ISDIR; } fsnotify_name(mask, inode.cast(), FSNOTIFY_EVENT_INODE, dir, &(*dentry).d_name, 0); }
#[inline] pub unsafe fn d_delete_notify(dir: *mut inode, dentry: *mut dentry) { let inode = d_inode(dentry); ihold(inode); d_delete(dentry); fsnotify_delete(dir, inode, dentry); iput(inode); }
#[inline] pub unsafe fn fsnotify_unlink(dir: *mut inode, dentry: *mut dentry) { if WARN_ON_ONCE(d_is_negative(dentry)) { return; } fsnotify_delete(dir, d_inode(dentry), dentry); }
#[inline] pub unsafe fn fsnotify_mkdir(dir: *mut inode, dentry: *mut dentry) { audit_inode_child(dir, dentry, AUDIT_TYPE_CHILD_CREATE); fsnotify_dirent(dir, dentry, FS_CREATE | FS_ISDIR); }
#[inline] pub unsafe fn fsnotify_rmdir(dir: *mut inode, dentry: *mut dentry) { if WARN_ON_ONCE(d_is_negative(dentry)) { return; } fsnotify_delete(dir, d_inode(dentry), dentry); }
#[inline] pub unsafe fn fsnotify_access(file: *mut file) { fsnotify_file(file, FS_ACCESS); }
#[inline] pub unsafe fn fsnotify_modify(file: *mut file) { fsnotify_file(file, FS_MODIFY); }
#[inline] pub unsafe fn fsnotify_open(file: *mut file) { let mut mask = FS_OPEN; if ((*file).f_flags & __FMODE_EXEC) != 0 { mask |= FS_OPEN_EXEC; } fsnotify_file(file, mask); }
#[inline] pub unsafe fn fsnotify_close(file: *mut file) { let mask = if ((*file).f_mode & FMODE_WRITE) != 0 { FS_CLOSE_WRITE } else { FS_CLOSE_NOWRITE }; fsnotify_file(file, mask); }
#[inline] pub unsafe fn fsnotify_xattr(dentry: *mut dentry) { fsnotify_dentry(dentry, FS_ATTRIB); }

#[inline]
pub unsafe fn fsnotify_change(dentry: *mut dentry, ia_valid: u32) {
    let mut mask = 0;
    if ia_valid & ATTR_UID != 0 { mask |= FS_ATTRIB; }
    if ia_valid & ATTR_GID != 0 { mask |= FS_ATTRIB; }
    if ia_valid & ATTR_SIZE != 0 { mask |= FS_MODIFY; }
    if ia_valid & (ATTR_ATIME | ATTR_MTIME) == (ATTR_ATIME | ATTR_MTIME) { mask |= FS_ATTRIB; }
    else if ia_valid & ATTR_ATIME != 0 { mask |= FS_ACCESS; }
    else if ia_valid & ATTR_MTIME != 0 { mask |= FS_MODIFY; }
    if ia_valid & ATTR_MODE != 0 { mask |= FS_ATTRIB; }
    if mask != 0 { fsnotify_dentry(dentry, mask); }
}

#[inline] pub unsafe fn fsnotify_mnt_attach(ns: *mut mnt_namespace, mnt: *mut vfsmount) { fsnotify_mnt(FS_MNT_ATTACH, ns, mnt); }
#[inline] pub unsafe fn fsnotify_mnt_detach(ns: *mut mnt_namespace, mnt: *mut vfsmount) { fsnotify_mnt(FS_MNT_DETACH, ns, mnt); }
#[inline] pub unsafe fn fsnotify_mnt_move(ns: *mut mnt_namespace, mnt: *mut vfsmount) { fsnotify_mnt(FS_MNT_MOVE, ns, mnt); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
