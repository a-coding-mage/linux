// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of event_inode.c.  Kernel types and
 * helpers referenced here are supplied by the surrounding tracefs code. */

use core::ffi::c_void;

const EVENTFS_FILE_INODE_INO: u64 = 0x12c4e37;
const EVENTFS_SAVE_MODE: u32 = 1 << 16;
const EVENTFS_SAVE_UID: u32 = 1 << 17;
const EVENTFS_SAVE_GID: u32 = 1 << 18;
const EVENTFS_MODE_MASK: u32 = EVENTFS_SAVE_MODE - 1;

#[repr(C)]
pub struct EventfsRootInode { pub ei: eventfs_inode, pub events_dir: *mut dentry }

static mut EVENTFS_MUTEX: mutex = mutex::new();
static mut EVENTFS_SRCU: srcu_struct = srcu_struct::new();

unsafe fn get_root_inode(ei: *mut eventfs_inode) -> *mut EventfsRootInode {
    WARN_ON_ONCE((*ei).is_events == 0);
    container_of!(ei, EventfsRootInode, ei)
}

unsafe fn eventfs_dir_ino(ei: *mut eventfs_inode) -> u64 {
    if (*ei).ino == 0 { (*ei).ino = get_next_ino(); if (*ei).ino == EVENTFS_FILE_INODE_INO { (*ei).ino = get_next_ino(); } }
    (*ei).ino
}

unsafe fn free_ei_rcu(rcu: *mut rcu_head) {
    let ei = container_of!(rcu, eventfs_inode, rcu);
    kfree((*ei).entry_attrs as *mut c_void); kfree_const((*ei).name as *mut c_void);
    if (*ei).is_events != 0 { kfree(get_root_inode(ei) as *mut c_void); } else { kfree(ei as *mut c_void); }
}

unsafe extern "C" fn release_ei(ref_: *mut kref) {
    let ei = container_of!(ref_, eventfs_inode, kref);
    WARN_ON_ONCE((*ei).is_freed == 0);
    for i in 0..(*ei).nr_entries { let entry = &*(*ei).entries.add(i as usize); if let Some(f) = entry.release { f(entry.name, (*ei).data); } }
    call_srcu(&raw mut EVENTFS_SRCU, &mut (*ei).rcu, free_ei_rcu);
}
unsafe fn put_ei(ei: *mut eventfs_inode) { if !ei.is_null() { kref_put(&mut (*ei).kref, release_ei); } }
unsafe fn free_ei(ei: *mut eventfs_inode) { if !ei.is_null() { WARN_ON_ONCE(!list_empty(&(*ei).children)); (*ei).is_freed = 1; smp_wmb(); put_ei(ei); } }
unsafe fn cleanup_ei(ei: *mut eventfs_inode) { if !ei.is_null() { (*ei).nr_entries = 0; free_ei(ei); } }
unsafe fn get_ei(ei: *mut eventfs_inode) -> *mut eventfs_inode { if !ei.is_null() { kref_get(&mut (*ei).kref); } ei }

unsafe fn update_attr(attr: *mut eventfs_attr, iattr: *const iattr) {
    let valid = (*iattr).ia_valid;
    if valid & ATTR_MODE != 0 { (*attr).mode = ((*attr).mode & !EVENTFS_MODE_MASK) | ((*iattr).ia_mode & EVENTFS_MODE_MASK) | EVENTFS_SAVE_MODE; }
    if valid & ATTR_UID != 0 { (*attr).mode |= EVENTFS_SAVE_UID; (*attr).uid = (*iattr).ia_uid; }
    if valid & ATTR_GID != 0 { (*attr).mode |= EVENTFS_SAVE_GID; (*attr).gid = (*iattr).ia_gid; }
}

unsafe fn eventfs_set_attr(idmap: *mut mnt_idmap, dentry: *mut dentry, iattr: *mut iattr) -> c_int {
    let ei = (*dentry).d_fsdata; if (*ei).is_freed != 0 { return -ENODEV; }
    if (*(*dentry).d_inode).i_mode & S_IFDIR == 0 && (*ei).entry_attrs.is_null() { (*ei).entry_attrs = kzalloc_array((*ei).nr_entries); if (*ei).entry_attrs.is_null() { return -ENOMEM; } }
    let ret = simple_setattr(idmap, dentry, iattr); if ret < 0 { return ret; }
    if (*(*dentry).d_inode).i_mode & S_IFDIR != 0 { if (*ei).is_events == 0 { update_attr(&mut (*ei).attr, iattr); } }
    else { let name = (*dentry).d_name.name; for i in 0..(*ei).nr_entries { let entry = &*(*ei).entries.add(i as usize); if strcmp(name, entry.name) == 0 { update_attr(&mut *(*ei).entry_attrs.add(i as usize), iattr); break; } } }
    ret
}

unsafe fn update_inode_attr(inode: *mut inode, mode: umode_t, attr: *mut eventfs_attr, rei: *mut EventfsRootInode) {
    (*inode).i_mode = if !attr.is_null() && (*attr).mode & EVENTFS_SAVE_MODE != 0 { (*attr).mode & EVENTFS_MODE_MASK } else { mode };
    (*inode).i_uid = if !attr.is_null() && (*attr).mode & EVENTFS_SAVE_UID != 0 { (*attr).uid } else { (*rei).ei.attr.uid };
    (*inode).i_gid = if !attr.is_null() && (*attr).mode & EVENTFS_SAVE_GID != 0 { (*attr).gid } else { (*rei).ei.attr.gid };
}

unsafe fn eventfs_get_inode(dentry: *mut dentry, attr: *mut eventfs_attr, mode: umode_t, ei: *mut eventfs_inode) -> *mut inode {
    let inode = tracefs_get_inode((*dentry).d_sb); if inode.is_null() { return core::ptr::null_mut(); }
    let ti = get_tracefs(inode); (*ti).private = ei; (*ti).flags |= TRACEFS_EVENT_INODE;
    let mut cur = dentry; loop { cur = (*cur).d_parent; let pei = (*cur).d_fsdata; if (*pei).is_events != 0 { update_inode_attr(inode, mode, attr, get_root_inode(pei)); break; } } inode
}

unsafe fn lookup_file(parent_ei: *mut eventfs_inode, dentry: *mut dentry, mut mode: umode_t, attr: *mut eventfs_attr, data: *mut c_void, fop: *const file_operations) -> *mut dentry {
    if mode & S_IFMT == 0 { mode |= S_IFREG; } if !S_ISREG(mode) { return ERR_PTR(-EIO); }
    let inode = eventfs_get_inode(dentry, attr, mode, core::ptr::null_mut()); if inode.is_null() { return ERR_PTR(-ENOMEM); }
    (*inode).i_op = &eventfs_file_inode_operations; (*inode).i_fop = fop; (*inode).i_private = data; (*inode).i_ino = EVENTFS_FILE_INODE_INO;
    (*dentry).d_fsdata = get_ei(parent_ei); d_splice_alias(inode, dentry)
}

unsafe fn lookup_dir_entry(dentry: *mut dentry, _pei: *mut eventfs_inode, ei: *mut eventfs_inode) -> *mut dentry {
    let mode = S_IFDIR | S_IRWXU | S_IRUGO | S_IXUGO; let inode = eventfs_get_inode(dentry, &mut (*ei).attr, mode, ei); if inode.is_null() { return ERR_PTR(-ENOMEM); }
    (*inode).i_op = &eventfs_dir_inode_operations; (*inode).i_fop = &eventfs_file_operations; (*inode).i_ino = eventfs_dir_ino(ei); (*dentry).d_fsdata = get_ei(ei); d_splice_alias(inode, dentry)
}

unsafe fn init_ei(ei: *mut eventfs_inode, name: *const c_char) -> *mut eventfs_inode { (*ei).name = kstrdup_const(name); if (*ei).name.is_null() { return core::ptr::null_mut(); } kref_init(&mut (*ei).kref); ei }
unsafe fn alloc_ei(name: *const c_char) -> *mut eventfs_inode { let ei = kzalloc_obj::<eventfs_inode>(); if ei.is_null() { return ei; } if init_ei(ei, name).is_null() { kfree(ei as *mut c_void); core::ptr::null_mut() } else { ei } }
unsafe fn alloc_root_ei(name: *const c_char) -> *mut eventfs_inode { let rei = kzalloc_obj::<EventfsRootInode>(); if rei.is_null() { return core::ptr::null_mut(); } rei.as_mut().unwrap().ei.is_events = 1; if init_ei(&mut (*rei).ei, name).is_null() { kfree(rei as *mut c_void); core::ptr::null_mut() } else { &mut (*rei).ei } }

pub unsafe extern "C" fn eventfs_d_release(dentry: *mut dentry) { put_ei((*dentry).d_fsdata); }

unsafe fn eventfs_set_attrs(ei: *mut eventfs_inode, update_uid: bool, uid: kuid_t, update_gid: bool, gid: kgid_t, level: c_int) {
    lockdep_assert_held(&raw mut EVENTFS_MUTEX); if WARN_ON_ONCE(level > 3) { return; }
    if update_uid { (*ei).attr.mode &= !EVENTFS_SAVE_UID; (*ei).attr.uid = uid; }
    if update_gid { (*ei).attr.mode &= !EVENTFS_SAVE_GID; (*ei).attr.gid = gid; }
    list_for_each_entry!((*ei).children, child, eventfs_inode, list, { eventfs_set_attrs(child, update_uid, uid, update_gid, gid, level + 1); });
    if !(*ei).entry_attrs.is_null() { for i in 0..(*ei).nr_entries { let a = &mut *(*ei).entry_attrs.add(i as usize); if update_uid { a.mode &= !EVENTFS_SAVE_UID; a.uid = uid; } if update_gid { a.mode &= !EVENTFS_SAVE_GID; a.gid = gid; } } }
}

pub unsafe extern "C" fn eventfs_remount(ti: *mut tracefs_inode, update_uid: bool, update_gid: bool) { let ei = (*ti).private; if ei.is_null() || (*ei).is_events == 0 || (*ei).is_freed != 0 { return; } eventfs_set_attrs(ei, update_uid, (*ti).vfs_inode.i_uid, update_gid, (*ti).vfs_inode.i_gid, 0); }

pub unsafe extern "C" fn eventfs_create_dir(name: *const c_char, parent: *mut eventfs_inode, entries: *const eventfs_entry, size: c_int, data: *mut c_void) -> *mut eventfs_inode {
    if parent.is_null() { return ERR_PTR(-EINVAL); } let ei = alloc_ei(name); if ei.is_null() { return ERR_PTR(-ENOMEM); }
    (*ei).entries = entries; (*ei).nr_entries = size; (*ei).data = data; INIT_LIST_HEAD(&mut (*ei).children); INIT_LIST_HEAD(&mut (*ei).list);
    scoped_mutex!(&raw mut EVENTFS_MUTEX, { if (*parent).is_freed == 0 { list_add_tail_rcu(&mut (*ei).list, &mut (*parent).children); } });
    if list_empty(&(*ei).list) { cleanup_ei(ei); return ERR_PTR(-EBUSY); } ei
}

unsafe fn eventfs_remove_rec(ei: *mut eventfs_inode, level: c_int) { if WARN_ON_ONCE(level > 3) { return; } list_for_each_entry_safe!((*ei).children, child, tmp, eventfs_inode, list, { eventfs_remove_rec(child, level + 1); }); list_del_rcu(&mut (*ei).list); free_ei(ei); }
pub unsafe extern "C" fn eventfs_remove_dir(ei: *mut eventfs_inode) { if !ei.is_null() { scoped_mutex!(&raw mut EVENTFS_MUTEX, { eventfs_remove_rec(ei, 0); }); } }
pub unsafe extern "C" fn eventfs_remove_events_dir(ei: *mut eventfs_inode) { let rei = get_root_inode(ei); let d = (*rei).events_dir; if d.is_null() { return; } (*rei).events_dir = core::ptr::null_mut(); eventfs_remove_dir(ei); d_invalidate(d); d_make_discardable(d); }

pub unsafe extern "C" fn eventfs_remount_lock() -> c_int { mutex_lock(&raw mut EVENTFS_MUTEX); srcu_read_lock(&raw mut EVENTFS_SRCU) }
pub unsafe extern "C" fn eventfs_remount_unlock(idx: c_int) { srcu_read_unlock(&raw mut EVENTFS_SRCU, idx); mutex_unlock(&raw mut EVENTFS_MUTEX); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
