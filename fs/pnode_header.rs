/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/fs/pnode.h
 *
 * (C) Copyright IBM Corporation 2005.
 */

// Dependency intent: <linux/list.h> and "mount.h" provide the list, mount,
// mountpoint, namespace, dentry, path, and propagation flag definitions.

/* Mount flag helpers. */
macro_rules! IS_MNT_SHARED { ($m:expr) => { ($m).mnt_t_flags & T_SHARED }; }
macro_rules! IS_MNT_SLAVE { ($m:expr) => { ($m).mnt_master }; }
macro_rules! IS_MNT_NEW { ($m:expr) => { !($m).mnt_ns }; }
macro_rules! CLEAR_MNT_SHARED { ($m:expr) => { ($m).mnt_t_flags &= !T_SHARED }; }
macro_rules! IS_MNT_UNBINDABLE { ($m:expr) => { ($m).mnt_t_flags & T_UNBINDABLE }; }
macro_rules! IS_MNT_MARKED { ($m:expr) => { ($m).mnt_t_flags & T_MARKED }; }
macro_rules! SET_MNT_MARK { ($m:expr) => { ($m).mnt_t_flags |= T_MARKED }; }
macro_rules! CLEAR_MNT_MARK { ($m:expr) => { ($m).mnt_t_flags &= !T_MARKED }; }
macro_rules! IS_MNT_LOCKED { ($m:expr) => { ($m).mnt.mnt_flags & MNT_LOCKED }; }

pub const CL_EXPIRE: i32 = 0x01;
pub const CL_SLAVE: i32 = 0x02;
pub const CL_COPY_UNBINDABLE: i32 = 0x04;
pub const CL_MAKE_SHARED: i32 = 0x08;
pub const CL_PRIVATE: i32 = 0x10;
pub const CL_COPY_MNT_NS_FILE: i32 = 0x40;

/*
 * EXCL[namespace_sem]
 */
macro_rules! set_mnt_shared {
    ($mnt:expr) => {{
        ($mnt).mnt_t_flags &= !T_SHARED_MASK;
        ($mnt).mnt_t_flags |= T_SHARED;
    }};
}

macro_rules! peers {
    ($m1:expr, $m2:expr) => {
        ($m1).mnt_group_id == ($m2).mnt_group_id && ($m1).mnt_group_id != 0
    };
}

extern "C" {
    pub fn change_mnt_propagation(mnt: *mut mount, recurse: i32);
    pub fn bulk_make_private(list: *mut list_head);
    pub fn propagate_mnt(
        dest_mnt: *mut mount,
        dest_mp: *mut mountpoint,
        source_mnt: *mut mount,
        tree_list: *mut hlist_head,
    ) -> i32;
    pub fn propagate_umount(list: *mut list_head);
    pub fn propagate_mount_busy(mnt: *mut mount, refcnt: i32) -> i32;
    pub fn propagate_mount_unlock(mnt: *mut mount);
    pub fn mnt_release_group_id(mnt: *mut mount);
    pub fn get_dominating_id(mnt: *mut mount, root: *const path) -> i32;
    pub fn mnt_get_count(mnt: *mut mount) -> i32;
    pub fn mnt_set_mountpoint(
        mnt: *mut mount,
        mp: *mut mountpoint,
        child_mnt: *mut mount,
    );
    pub fn mnt_change_mountpoint(
        parent: *mut mount,
        mp: *mut mountpoint,
        mnt: *mut mount,
    );
    pub fn copy_tree(mnt: *mut mount, root: *mut dentry, flag: i32) -> *mut mount;
    pub fn is_path_reachable(
        mnt: *mut mount,
        dentry: *mut dentry,
        root: *const path,
    ) -> bool;
    pub fn count_mounts(ns: *mut mnt_namespace, mnt: *mut mount) -> i32;
    pub fn propagation_would_overmount(
        from: *const mount,
        to: *const mount,
        mp: *const mountpoint,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
