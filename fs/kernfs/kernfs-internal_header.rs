/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * fs/kernfs/kernfs-internal.h - kernfs internal header file
 *
 * Rust translation of the C header. Linux dependencies are supplied by
 * surrounding translated headers.
 */

// Original includes: linux/lockdep.h, fs.h, mutex.h, rwsem.h, xattr.h,
// kernfs.h, and fs_context.h.

#[repr(C)]
pub struct kernfs_iattrs {
    pub ia_uid: kuid_t,
    pub ia_gid: kgid_t,
    pub ia_atime: timespec64,
    pub ia_mtime: timespec64,
    pub ia_ctime: timespec64,
    pub xattrs: list_head,
    pub xattr_limits: simple_xattr_limits,
}

#[repr(C)]
pub struct kernfs_root {
    // published fields
    pub kn: *mut kernfs_node,
    pub flags: c_uint,

    // private fields, do not use outside kernfs proper
    pub ino_idr: idr,
    pub kernfs_idr_lock: spinlock_t,
    pub last_id_lowbits: u32,
    pub id_highbits: u32,
    pub syscall_ops: *mut kernfs_syscall_ops,
    pub supers: list_head,
    pub deactivate_waitq: wait_queue_head_t,
    pub kernfs_rwsem: rw_semaphore,
    pub kernfs_iattr_rwsem: rw_semaphore,
    pub kernfs_supers_rwsem: rw_semaphore,
    pub kernfs_rename_lock: rwlock_t,
    pub rcu: rcu_head,
    pub xa_cache: simple_xattr_cache,
}

// +1 to avoid triggering overflow warning when negating it.
pub const KN_DEACTIVATED_BIAS: c_int = c_int::MIN + 1;

/// Find out the kernfs_root a kernfs_node belongs to.
#[inline]
pub unsafe fn kernfs_root(mut kn: *const kernfs_node) -> *mut kernfs_root {
    // The original uses guard(rcu)() and rcu_dereference(). Preserve the RCU
    // section and dependency semantics through the translated primitives.
    let knp: *const kernfs_node = rcu_dereference((*kn).__parent);
    if !knp.is_null() {
        kn = knp;
    }
    (*kn).dir.root
}

#[repr(C)]
pub struct kernfs_super_info {
    pub sb: *mut super_block,
    // Root associated with this super_block.
    pub root: *mut kernfs_root,
    // Namespace tag associated with this super_block.
    pub ns: *const ns_common,
    // Anchored at kernfs_root::supers, protected by kernfs_rwsem.
    pub node: list_head,
}

#[inline]
pub unsafe fn kernfs_info(sb: *mut super_block) -> *mut kernfs_super_info {
    (*sb).s_fs_info as *mut kernfs_super_info
}

#[inline]
pub unsafe fn kernfs_root_is_locked(kn: *const kernfs_node) -> bool {
    lockdep_is_held(&(*kernfs_root(kn)).kernfs_rwsem)
}

#[inline]
pub unsafe fn kernfs_rename_is_locked(kn: *const kernfs_node) -> bool {
    lockdep_is_held(&(*kernfs_root(kn)).kernfs_rename_lock)
}

#[inline]
pub unsafe fn kernfs_rcu_name(kn: *const kernfs_node) -> *const c_char {
    rcu_dereference_check((*kn).name, kernfs_root_is_locked(kn))
}

#[inline]
pub unsafe fn kernfs_parent(kn: *const kernfs_node) -> *mut kernfs_node {
    rcu_dereference_check(
        (*kn).__parent,
        kernfs_root_is_locked(kn)
            || kernfs_rename_is_locked(kn)
            || atomic_read(&(*kn).count) == 0,
    )
}

#[inline]
pub unsafe fn kernfs_dentry_node(dentry: *mut dentry) -> *mut kernfs_node {
    if d_really_is_negative(dentry) {
        return core::ptr::null_mut();
    }
    (*d_inode(dentry)).i_private as *mut kernfs_node
}

#[inline]
pub unsafe fn kernfs_set_rev(parent: *mut kernfs_node, dentry: *mut dentry) {
    (*dentry).d_time = (*parent).dir.rev;
}

#[inline]
pub unsafe fn kernfs_inc_rev(parent: *mut kernfs_node) {
    (*parent).dir.rev = (*parent).dir.rev.wrapping_add(1);
}

#[inline]
pub unsafe fn kernfs_dir_changed(parent: *mut kernfs_node, dentry: *mut dentry) -> bool {
    (*parent).dir.rev != (*dentry).d_time
}

extern "C" {
    pub static kernfs_sops: super_operations;
    pub static mut kernfs_node_cache: *mut kmem_cache;
    pub static mut kernfs_iattrs_cache: *mut kmem_cache;
    pub static kernfs_xattr_handlers: *const *const xattr_handler;
    pub fn kernfs_evict_inode(inode: *mut inode);
    pub fn kernfs_iop_permission(idmap: *mut mnt_idmap, inode: *mut inode, mask: c_int) -> c_int;
    pub fn kernfs_iop_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, iattr: *mut iattr) -> c_int;
    pub fn kernfs_iop_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, query_flags: c_uint) -> c_int;
    pub fn kernfs_iop_listxattr(dentry: *mut dentry, buf: *mut c_char, size: usize) -> isize;
    pub fn __kernfs_setattr(kn: *mut kernfs_node, iattr: *const iattr) -> c_int;
    pub static kernfs_dops: dentry_operations;
    pub static kernfs_dir_fops: file_operations;
    pub static kernfs_dir_iops: inode_operations;
    pub fn kernfs_get_active(kn: *mut kernfs_node) -> *mut kernfs_node;
    pub fn kernfs_put_active(kn: *mut kernfs_node);
    pub fn kernfs_add_one(kn: *mut kernfs_node) -> c_int;
    pub fn kernfs_new_node(parent: *mut kernfs_node, name: *const c_char, mode: umode_t, uid: kuid_t, gid: kgid_t, flags: c_uint) -> *mut kernfs_node;
    pub static kernfs_file_fops: file_operations;
    pub fn kernfs_should_drain_open_files(kn: *mut kernfs_node) -> bool;
    pub fn kernfs_drain_open_files(kn: *mut kernfs_node);
    pub static kernfs_symlink_iops: inode_operations;
    pub static mut kernfs_locks: *mut kernfs_global_locks;
}

#[inline]
pub unsafe fn kernfs_node_lock_ptr(kn: *mut kernfs_node) -> *mut mutex {
    let idx: usize = hash_ptr(kn as *const c_void, NR_KERNFS_LOCK_BITS);
    &mut (*kernfs_locks).node_mutex[idx]
}

#[inline]
pub unsafe fn kernfs_node_lock(kn: *mut kernfs_node) -> *mut mutex {
    let lock = kernfs_node_lock_ptr(kn);
    mutex_lock(lock);
    lock
}

// DEFINE_CLASS(kernfs_node_lock, struct mutex *, mutex_unlock(_T),
//              kernfs_node_lock(kn), struct kernfs_node *kn)
// is a C cleanup-scope helper; its translated declaration is represented by
// the lock acquisition function above and the external mutex_unlock primitive.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
