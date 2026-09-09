/* SPDX-License-Identifier: GPL-2.0 */
// Translated from mount.h. Types and helpers from the included kernel headers
// are supplied by other translation units.
use std::os::raw::{c_char, c_int, c_uint};

extern "C" {
    pub static mut nullfs_fs_type: file_system_type;
    pub static mut notify_list: list_head;
}

#[repr(C)]
pub struct mnt_namespace {
    pub ns: ns_common,
    pub root: *mut mount,
    pub mounts: rb_root,
    pub mnt_last_node: *mut rb_node,
    pub mnt_first_node: *mut rb_node,
    pub user_ns: *mut user_namespace,
    pub ucounts: *mut ucounts,
    pub poll: wait_queue_head_t,
    pub seq_origin: u64,
    pub event: u64,
    #[cfg(feature = "CONFIG_FSNOTIFY")]
    pub n_fsnotify_mask: u32,
    #[cfg(feature = "CONFIG_FSNOTIFY")]
    pub n_fsnotify_marks: *mut fsnotify_mark_connector,
    pub mnt_visible_mounts: hlist_head,
    pub nr_mounts: c_uint,
    pub pending_mounts: c_uint,
    pub passive: refcount_t,
    pub is_anon: bool,
}

#[repr(C)]
pub struct mnt_pcp { pub mnt_count: c_int, pub mnt_writers: c_int }

#[repr(C)]
pub struct mountpoint {
    pub m_hash: hlist_node,
    pub m_dentry: *mut dentry,
    pub m_list: hlist_head,
}

#[repr(C)]
pub union mount_node {
    pub mnt_node: rb_node,
    pub mnt_rcu: rcu_head,
    pub mnt_llist: llist_node,
}

#[repr(C)]
pub union mount_mp_list { pub mnt_mp_list: hlist_node, pub mnt_umount: hlist_node }

#[repr(C)]
pub struct mount {
    pub mnt_hash: hlist_node,
    pub mnt_parent: *mut mount,
    pub mnt_mountpoint: *mut dentry,
    pub mnt: vfsmount,
    pub node: mount_node,
    #[cfg(feature = "CONFIG_SMP")]
    pub mnt_pcp: *mut mnt_pcp,
    #[cfg(not(feature = "CONFIG_SMP"))]
    pub mnt_count: c_int,
    #[cfg(not(feature = "CONFIG_SMP"))]
    pub mnt_writers: c_int,
    pub mnt_mounts: list_head,
    pub mnt_child: list_head,
    pub mnt_next_for_sb: *mut mount,
    pub mnt_pprev_for_sb: *mut *mut mount,
    pub mnt_devname: *const c_char,
    pub mnt_list: list_head,
    pub mnt_expire: list_head,
    pub mnt_share: list_head,
    pub mnt_slave_list: hlist_head,
    pub mnt_slave: hlist_node,
    pub mnt_master: *mut mount,
    pub mnt_ns: *mut mnt_namespace,
    pub mnt_mp: *mut mountpoint,
    pub mp_list: mount_mp_list,
    #[cfg(feature = "CONFIG_FSNOTIFY")]
    pub mnt_fsnotify_marks: *mut fsnotify_mark_connector,
    #[cfg(feature = "CONFIG_FSNOTIFY")]
    pub mnt_fsnotify_mask: u32,
    #[cfg(feature = "CONFIG_FSNOTIFY")]
    pub to_notify: list_head,
    #[cfg(feature = "CONFIG_FSNOTIFY")]
    pub prev_ns: *mut mnt_namespace,
    pub mnt_t_flags: c_int,
    pub mnt_id: c_int,
    pub mnt_id_unique: u64,
    pub mnt_group_id: c_int,
    pub mnt_expiry_mark: c_int,
    pub mnt_pins: hlist_head,
    pub mnt_stuck_children: hlist_head,
    pub mnt_ns_visible: hlist_node,
    pub overmount: *mut mount,
}

pub const WRITE_HOLD: usize = 1;
pub const T_SHARED: c_int = 1;
pub const T_UNBINDABLE: c_int = 2;
pub const T_MARKED: c_int = 4;
pub const T_UMOUNT_CANDIDATE: c_int = 8;
pub const T_SHARED_MASK: c_int = T_UNBINDABLE;

pub const MNT_NS_INTERNAL: *mut mnt_namespace = (-EINVAL) as *mut mnt_namespace;

pub unsafe fn real_mount(mnt: *mut vfsmount) -> *mut mount { container_of!(mnt, mount, mnt) }
pub unsafe fn mnt_has_parent(mnt: *const mount) -> bool { (*mnt).mnt_parent != mnt as *mut mount }
pub unsafe fn is_mounted(mnt: *mut vfsmount) -> bool { !is_err_or_null!(real_mount(mnt).as_ref().unwrap().mnt_ns) }

extern "C" {
    pub fn __lookup_mnt(mnt: *mut vfsmount, dentry: *mut dentry) -> *mut mount;
    pub fn __legitimize_mnt(mnt: *mut vfsmount, seq: c_uint) -> c_int;
    pub fn __detach_mounts(dentry: *mut dentry);
    pub static mut mount_lock: seqlock_t;
    pub static mounts_op: seq_operations;
    pub fn __is_local_mountpoint(dentry: *const dentry) -> bool;
    pub fn has_locked_children(mnt: *mut mount, dentry: *mut dentry) -> bool;
    pub fn get_sequential_mnt_ns(mnt_ns: *mut mnt_namespace, previous: bool) -> *mut mnt_namespace;
    pub fn mnt_ns_from_dentry(dentry: *mut dentry) -> *mut mnt_namespace;
}

pub unsafe fn __path_is_mountpoint(path: *const path) -> bool {
    let m = __lookup_mnt((*path).mnt, (*path).dentry);
    !m.is_null() && likely!((*m).mnt.mnt_flags & MNT_SYNC_UMOUNT == 0)
}
pub unsafe fn detach_mounts(dentry: *mut dentry) { if d_mountpoint!(dentry) { __detach_mounts(dentry); } }
pub unsafe fn get_mnt_ns(ns: *mut mnt_namespace) { ns_ref_inc!(ns); }

pub unsafe fn anon_ns_root(m: *const mount) -> bool {
    let ns = std::ptr::read_volatile(&(*m).mnt_ns);
    !is_err_or_null!(ns) && is_anon_ns(ns) && m == (*ns).root
}
pub unsafe fn mnt_ns_attached(mnt: *const mount) -> bool { !rb_empty_node!(&(*mnt).node.mnt_node) }
pub unsafe fn mnt_ns_empty(ns: *const mnt_namespace) -> bool { rb_empty_root!(&(*ns).mounts) }
pub unsafe fn move_from_ns(mnt: *mut mount) {
    let ns = (*mnt).mnt_ns;
    warn_on!(!mnt_ns_attached(mnt));
    if (*ns).mnt_last_node == &mut (*mnt).node.mnt_node { (*ns).mnt_last_node = rb_prev!(&mut (*mnt).node.mnt_node); }
    if (*ns).mnt_first_node == &mut (*mnt).node.mnt_node { (*ns).mnt_first_node = rb_next!(&mut (*mnt).node.mnt_node); }
    rb_erase!(&mut (*mnt).node.mnt_node, &mut (*ns).mounts);
    rb_clear_node!(&mut (*mnt).node.mnt_node);
    if !hlist_unhashed!(&(*mnt).mnt_ns_visible) { hlist_del_init!(&mut (*mnt).mnt_ns_visible); }
}
pub unsafe fn to_mnt_ns(ns: *mut ns_common) -> *mut mnt_namespace { container_of!(ns, mnt_namespace, ns) }

#[cfg(feature = "CONFIG_FSNOTIFY")]
pub unsafe fn mnt_notify_add(m: *mut mount) {
    if (!(*m).mnt_ns.is_null() && !(*(*m).mnt_ns).n_fsnotify_marks.is_null()) ||
       (!(*m).prev_ns.is_null() && !(*(*m).prev_ns).n_fsnotify_marks.is_null()) {
        list_add_tail!(&mut (*m).to_notify, &mut notify_list);
    } else { (*m).prev_ns = (*m).mnt_ns; }
}
#[cfg(not(feature = "CONFIG_FSNOTIFY"))]
pub unsafe fn mnt_notify_add(_m: *mut mount) {}

#[repr(C)]
pub struct proc_mounts { pub ns: *mut mnt_namespace, pub root: path, pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut vfsmount) -> c_int> }

pub unsafe fn is_local_mountpoint(dentry: *const dentry) -> bool { d_mountpoint!(dentry) && __is_local_mountpoint(dentry) }
pub unsafe fn is_anon_ns(ns: *const mnt_namespace) -> bool { (*ns).is_anon }
pub unsafe fn topmost_overmount(mut m: *mut mount) -> *mut mount { while !(*m).overmount.is_null() { m = (*m).overmount; } m }
pub unsafe fn __test_write_hold(val: *mut *mut mount) -> bool { val as usize & WRITE_HOLD != 0 }
pub unsafe fn test_write_hold(m: *const mount) -> bool { __test_write_hold((*m).mnt_pprev_for_sb) }
pub unsafe fn set_write_hold(m: *mut mount) { (*m).mnt_pprev_for_sb = (((*m).mnt_pprev_for_sb as usize) | WRITE_HOLD) as *mut *mut mount; }
pub unsafe fn clear_write_hold(m: *mut mount) { (*m).mnt_pprev_for_sb = (((*m).mnt_pprev_for_sb as usize) & !WRITE_HOLD) as *mut *mut mount; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
