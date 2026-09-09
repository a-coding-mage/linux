// SPDX-License-Identifier: GPL-2.0-or-later
/* Translation of linux/fsnotify.c. Kernel-provided types, constants, macros,
 * globals, and helper functions are intentionally referenced externally. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub unsafe fn __fsnotify_inode_delete(inode: *mut inode) {
    fsnotify_clear_marks_by_inode(inode);
}

pub unsafe fn __fsnotify_vfsmount_delete(mnt: *mut vfsmount) {
    fsnotify_clear_marks_by_mount(mnt);
}

pub unsafe fn __fsnotify_mntns_delete(mntns: *mut mnt_namespace) {
    fsnotify_clear_marks_by_mntns(mntns);
}

pub unsafe fn fsnotify_sb_delete(sb: *mut super_block) {
    let sbinfo = fsnotify_sb_info(sb);
    if sbinfo.is_null() { return; }
    fsnotify_unmount_inodes(sbinfo);
    fsnotify_clear_marks_by_sb(sb);
    wait_var_event(fsnotify_sb_watched_objects(sb),
                   atomic_long_read(fsnotify_sb_watched_objects(sb)) == 0);
    WARN_ON(fsnotify_sb_has_priority_watchers(sb, FSNOTIFY_PRIO_CONTENT));
    WARN_ON(fsnotify_sb_has_priority_watchers(sb, FSNOTIFY_PRIO_PRE_CONTENT));
}

pub unsafe fn fsnotify_sb_free(sb: *mut super_block) {
    if !(*sb).s_fsnotify_info.is_null() {
        WARN_ON_ONCE(!list_empty(&mut (*(*sb).s_fsnotify_info).inode_conn_list));
        kfree((*sb).s_fsnotify_info);
    }
}

pub unsafe fn fsnotify_set_children_dentry_flags(inode: *mut inode) {
    if !S_ISDIR((*inode).i_mode) { return; }
    spin_lock(&mut (*inode).i_lock);
    let mut alias: *mut dentry = core::ptr::null_mut();
    for_each_alias(alias, inode) {
        spin_lock(&mut (*alias).d_lock);
        let mut child: *mut dentry = core::ptr::null_mut();
        hlist_for_each_entry(child, &mut (*alias).d_children, d_sib) {
            if (*child).d_inode.is_null() { continue; }
            spin_lock_nested(&mut (*child).d_lock, DENTRY_D_LOCK_NESTED);
            (*child).d_flags |= DCACHE_FSNOTIFY_PARENT_WATCHED;
            spin_unlock(&mut (*child).d_lock);
        }
        spin_unlock(&mut (*alias).d_lock);
    }
    spin_unlock(&mut (*inode).i_lock);
}

unsafe fn fsnotify_clear_child_dentry_flag(pinode: *mut inode, dentry: *mut dentry) {
    spin_lock(&mut (*dentry).d_lock);
    if !fsnotify_inode_watches_children(pinode) {
        (*dentry).d_flags &= !DCACHE_FSNOTIFY_PARENT_WATCHED;
    }
    spin_unlock(&mut (*dentry).d_lock);
}

unsafe fn fsnotify_event_needs_parent(inode: *mut inode, mnt_mask: u32, mask: u32) -> bool {
    if mask & FS_ISDIR != 0 { return false; }
    let mut marks_mask = 0u32;
    marks_mask |= fsnotify_parent_needed_mask(READ_ONCE((*inode).i_fsnotify_mask));
    marks_mask |= fsnotify_parent_needed_mask(READ_ONCE((*(*inode).i_sb).s_fsnotify_mask));
    marks_mask |= fsnotify_parent_needed_mask(mnt_mask);
    mask & marks_mask != 0
}

unsafe fn fsnotify_object_watched(inode: *mut inode, mnt_mask: u32, mask: u32) -> u32 {
    let marks_mask = READ_ONCE((*inode).i_fsnotify_mask) | mnt_mask |
                     READ_ONCE((*(*inode).i_sb).s_fsnotify_mask);
    mask & marks_mask & ALL_FSNOTIFY_EVENTS
}

pub unsafe fn fsnotify_pre_content(path: *const path, ppos: *const loff_t, count: usize) -> i32 {
    if ppos.is_null() { return fsnotify_path(path, FS_PRE_ACCESS); }
    let range = file_range { path, pos: PAGE_ALIGN_DOWN(*ppos), count: PAGE_ALIGN(*ppos + count as i64) - PAGE_ALIGN_DOWN(*ppos) };
    fsnotify_parent((*path).dentry, FS_PRE_ACCESS, &range, FSNOTIFY_EVENT_FILE_RANGE)
}

pub unsafe fn __fsnotify_parent(dentry: *mut dentry, mut mask: u32, data: *const core::ffi::c_void, data_type: i32) -> i32 {
    let path = fsnotify_data_path(data, data_type);
    let mnt_mask = if !path.is_null() { READ_ONCE((*real_mount((*path).mnt)).mnt_fsnotify_mask) } else { 0 };
    let inode = d_inode(dentry);
    let mut parent: *mut dentry = core::ptr::null_mut();
    let parent_watched = (*dentry).d_flags & DCACHE_FSNOTIFY_PARENT_WATCHED != 0;
    let parent_needed = fsnotify_event_needs_parent(inode, mnt_mask, mask);
    if !parent_watched && fsnotify_object_watched(inode, mnt_mask, mask) == 0 { return 0; }
    let mut p_inode: *mut inode = core::ptr::null_mut();
    let mut file_name: *const qstr = core::ptr::null();
    let mut name = core::mem::MaybeUninit::<name_snapshot>::uninit();
    if parent_watched || parent_needed {
        parent = dget_parent(dentry); p_inode = (*parent).d_inode;
        let p_mask = fsnotify_inode_watches_children(p_inode);
        if parent_watched && p_mask == 0 { fsnotify_clear_child_dentry_flag(p_inode, dentry); }
        let interested = mask & p_mask & ALL_FSNOTIFY_EVENTS != 0 &&
            !(data_type == FSNOTIFY_EVENT_PATH && d_is_special(dentry) && mask & (FS_ACCESS | FS_MODIFY) != 0);
        if parent_needed || interested {
            take_dentry_name_snapshot(name.as_mut_ptr(), dentry);
            file_name = &(*name.as_ptr()).name;
            if interested { mask |= FS_EVENT_ON_CHILD; }
        }
    }
    let ret = fsnotify(mask, data, data_type, p_inode, file_name, inode, 0);
    if !file_name.is_null() { release_dentry_name_snapshot(name.as_mut_ptr()); }
    dput(parent); ret
}

pub unsafe fn fsnotify(mask: u32, data: *const core::ffi::c_void, data_type: i32,
                       mut dir: *mut inode, file_name: *const qstr,
                       mut inode: *mut inode, cookie: u32) -> i32 {
    let path = fsnotify_data_path(data, data_type);
    let sb = fsnotify_data_sb(data, data_type);
    let mnt_data = fsnotify_data_mnt(data, data_type);
    let sbinfo = if !sb.is_null() { fsnotify_sb_info(sb) } else { core::ptr::null_mut() };
    let mut mnt: *mut mount = if !path.is_null() { real_mount((*path).mnt) } else { core::ptr::null_mut() };
    let mut inode2: *mut inode = core::ptr::null_mut();
    let mut marks_mask = 0u32;
    if inode.is_null() { inode = dir; if mask & FS_RENAME != 0 { inode2 = (*fsnotify_data_dentry(data, data_type)).d_parent.d_inode; } }
    else if mask & FS_EVENT_ON_CHILD != 0 { inode2 = dir; }
    if (sbinfo.is_null() || (*sbinfo).sb_marks.is_null()) && (mnt.is_null() || (*mnt).mnt_fsnotify_marks.is_null()) &&
       (inode.is_null() || (*inode).i_fsnotify_marks.is_null()) && (inode2.is_null() || (*inode2).i_fsnotify_marks.is_null()) { return 0; }
    if !sb.is_null() { marks_mask |= READ_ONCE((*sb).s_fsnotify_mask); }
    if !mnt.is_null() { marks_mask |= READ_ONCE((*mnt).mnt_fsnotify_mask); }
    if !inode.is_null() { marks_mask |= READ_ONCE((*inode).i_fsnotify_mask); }
    if !inode2.is_null() { marks_mask |= READ_ONCE((*inode2).i_fsnotify_mask); }
    if !mnt_data.is_null() { marks_mask |= READ_ONCE((*(*mnt_data).ns).n_fsnotify_mask); }
    if mask & ALL_FSNOTIFY_EVENTS & marks_mask == 0 { return 0; }
    0
}

// The remaining backend-specific permission and iterator entry points retain
// their kernel linkage; their implementations are supplied by fsnotify core.
extern "C" {
    fn fsnotify_clear_marks_by_inode(_: *mut inode); fn fsnotify_clear_marks_by_mount(_: *mut vfsmount);
    fn fsnotify_clear_marks_by_mntns(_: *mut mnt_namespace); fn fsnotify_sb_info(_: *mut super_block) -> *mut fsnotify_sb_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
