/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: linux/list.h, linux/fsnotify.h, linux/srcu.h,
// linux/types.h, and ../mount.h are supplied by the surrounding translation.

use core::ffi::c_void;

#[repr(C)]
pub struct fsnotify_mark_connector {
    pub obj: *mut c_void,
    pub type_: fsnotify_obj_type,
}

#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
    pub i_fsnotify_marks: fsnotify_connp_t,
}

#[repr(C)]
pub struct vfsmount {
    pub mnt_sb: *mut super_block,
}

#[repr(C)]
pub struct mount {
    pub mnt: vfsmount,
    pub mnt_fsnotify_marks: fsnotify_connp_t,
}

#[repr(C)]
pub struct super_block;

#[repr(C)]
pub struct mnt_namespace {
    pub n_fsnotify_marks: fsnotify_connp_t,
}

#[repr(C)]
pub struct fsnotify_sb_info {
    pub sb_marks: fsnotify_connp_t,
}

#[repr(C)]
pub struct fsnotify_group;

#[repr(C)]
pub struct srcu_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum fsnotify_obj_type {
    FSNOTIFY_OBJ_TYPE_INODE,
    FSNOTIFY_OBJ_TYPE_VFSMOUNT,
    FSNOTIFY_OBJ_TYPE_SB,
}

/* fsnotify_connp_t is what we embed in objects which connector can be attached
 * to.
 */
pub type fsnotify_connp_t = *mut fsnotify_mark_connector;

#[inline]
pub unsafe fn fsnotify_conn_inode(
    conn: *mut fsnotify_mark_connector,
) -> *mut inode {
    (*conn).obj as *mut inode
}

#[inline]
pub unsafe fn fsnotify_conn_mount(
    conn: *mut fsnotify_mark_connector,
) -> *mut mount {
    real_mount((*conn).obj as *mut vfsmount)
}

#[inline]
pub unsafe fn fsnotify_conn_sb(
    conn: *mut fsnotify_mark_connector,
) -> *mut super_block {
    (*conn).obj as *mut super_block
}

#[inline]
pub unsafe fn fsnotify_conn_mntns(
    conn: *mut fsnotify_mark_connector,
) -> *mut mnt_namespace {
    (*conn).obj as *mut mnt_namespace
}

#[inline]
pub unsafe fn fsnotify_object_sb(
    obj: *mut c_void,
    obj_type: fsnotify_obj_type,
) -> *mut super_block {
    match obj_type {
        fsnotify_obj_type::FSNOTIFY_OBJ_TYPE_INODE => (*((obj as *mut inode))).i_sb,
        fsnotify_obj_type::FSNOTIFY_OBJ_TYPE_VFSMOUNT => (*((obj as *mut vfsmount))).mnt_sb,
        fsnotify_obj_type::FSNOTIFY_OBJ_TYPE_SB => obj as *mut super_block,
    }
}

#[inline]
pub unsafe fn fsnotify_connector_sb(
    conn: *mut fsnotify_mark_connector,
) -> *mut super_block {
    fsnotify_object_sb((*conn).obj, (*conn).type_)
}

#[inline]
pub unsafe fn fsnotify_sb_marks(sb: *mut super_block) -> *mut fsnotify_connp_t {
    let sbinfo = fsnotify_sb_info(sb);
    if !sbinfo.is_null() {
        &mut (*sbinfo).sb_marks
    } else {
        core::ptr::null_mut()
    }
}

/* destroy all events sitting in this groups notification queue */
unsafe extern "C" {
    pub fn fsnotify_flush_notify(group: *mut fsnotify_group);

    /* protects reads of inode and vfsmount marks list */
    static mut fsnotify_mark_srcu: srcu_struct;

    /* compare two groups for sorting of marks lists */
    pub fn fsnotify_compare_groups(
        a: *mut fsnotify_group,
        b: *mut fsnotify_group,
    ) -> i32;

    /* Destroy all inode marks for given superblock */
    pub fn fsnotify_unmount_inodes(sbinfo: *mut fsnotify_sb_info);

    /* Destroy all marks attached to an object via connector */
    pub fn fsnotify_destroy_marks(connp: *mut fsnotify_connp_t);

    /* update the dentry->d_flags of all of inode's children to indicate if inode cares
     * about events that happen to its children.
     */
    pub fn fsnotify_set_children_dentry_flags(inode: *mut inode);

    pub fn fsnotify_init_connector_caches();
}

/* run the list of all marks associated with inode and destroy them */
#[inline]
pub unsafe fn fsnotify_clear_marks_by_inode(inode: *mut inode) {
    fsnotify_destroy_marks(&mut (*inode).i_fsnotify_marks);
}

/* run the list of all marks associated with vfsmount and destroy them */
#[inline]
pub unsafe fn fsnotify_clear_marks_by_mount(mnt: *mut vfsmount) {
    fsnotify_destroy_marks(&mut (*real_mount(mnt)).mnt_fsnotify_marks);
}

/* run the list of all marks associated with sb and destroy them */
#[inline]
pub unsafe fn fsnotify_clear_marks_by_sb(sb: *mut super_block) {
    fsnotify_destroy_marks(fsnotify_sb_marks(sb));
}

#[inline]
pub unsafe fn fsnotify_clear_marks_by_mntns(mntns: *mut mnt_namespace) {
    fsnotify_destroy_marks(&mut (*mntns).n_fsnotify_marks);
}

// Supplied by ../mount.h.
unsafe extern "C" {
    pub fn real_mount(mnt: *mut vfsmount) -> *mut mount;
    pub fn fsnotify_sb_info(sb: *mut super_block) -> *mut fsnotify_sb_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
