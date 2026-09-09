/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: _TRACEFS_INTERNAL_H */

/* BIT(n) from the surrounding kernel headers. */
pub const TRACEFS_EVENT_INODE: u32 = 1u32 << 1;
pub const TRACEFS_GID_PERM_SET: u32 = 1u32 << 2;
pub const TRACEFS_UID_PERM_SET: u32 = 1u32 << 3;
pub const TRACEFS_INSTANCE_INODE: u32 = 1u32 << 4;

#[repr(C)]
pub struct tracefs_inode {
    pub vfs_inode: inode,
    /* The below gets initialized with memset_after(ti, 0, vfs_inode) */
    pub list: list_head,
    pub flags: c_ulong,
    pub private: *mut c_void,
}

/*
 * struct eventfs_attr - cache the mode and ownership of a eventfs entry
 * @mode: saved mode plus flags of what is saved
 * @uid: saved uid if changed
 * @gid: saved gid if changed
 */
#[repr(C)]
pub struct eventfs_attr {
    pub mode: c_int,
    pub uid: kuid_t,
    pub gid: kgid_t,
}

/*
 * struct eventfs_inode - hold the properties of the eventfs directories.
 * @list: link list into the parent directory
 * @rcu: Union with @list for freeing
 * @children: link list into the child eventfs_inode
 * @entries: the array of entries representing the files in the directory
 * @name: the name of the directory to create
 * @entry_attrs: Saved mode and ownership of the @d_children
 * @data: The private data to pass to the callbacks
 * @attr: Saved mode and ownership of eventfs_inode itself
 * @is_freed: Flag set if the eventfs is on its way to be freed
 *            Note if is_freed is set, then dentry is corrupted.
 * @is_events: Flag set for only the top level "events" directory
 * @nr_entries: The number of items in @entries
 * @ino: The saved inode number
 */
#[repr(C)]
pub struct eventfs_inode {
    pub list: list_head,
    pub children_or_rcu: eventfs_inode_children_or_rcu,
    pub entries: *const eventfs_entry,
    pub name: *const c_char,
    pub entry_attrs: *mut eventfs_attr,
    pub data: *mut c_void,
    pub attr: eventfs_attr,
    pub kref: kref,
    /* C bit-fields: is_freed:1, is_events:1, nr_entries:30. */
    pub is_freed: u32,
    pub is_events: u32,
    pub nr_entries: u32,
    pub ino: c_uint,
}

#[repr(C)]
pub union eventfs_inode_children_or_rcu {
    pub children: list_head,
    pub rcu: rcu_head,
}

#[inline]
pub unsafe fn get_tracefs(inode: *const inode) -> *mut tracefs_inode {
    (inode as *const u8).sub(core::mem::offset_of!(tracefs_inode, vfs_inode))
        as *mut tracefs_inode
}

unsafe extern "C" {
    pub fn tracefs_start_creating(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    pub fn tracefs_end_creating(dentry: *mut dentry) -> *mut dentry;
    pub fn tracefs_failed_creating(dentry: *mut dentry) -> *mut dentry;
    pub fn tracefs_get_inode(sb: *mut super_block) -> *mut inode;

    pub fn eventfs_remount(ti: *mut tracefs_inode, update_uid: bool, update_gid: bool);
    pub fn eventfs_d_release(dentry: *mut dentry);

    pub fn eventfs_remount_lock() -> c_int;
    pub fn eventfs_remount_unlock(srcu_idx: c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
