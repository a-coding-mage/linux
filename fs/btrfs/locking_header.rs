/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008 Oracle.  All rights reserved.
 */

// C dependencies supplied by other translation units:
// linux::atomic, linux::wait, linux::lockdep, linux::percpu_counter, extent_io

pub const BTRFS_WRITE_LOCK: i32 = 1;
pub const BTRFS_READ_LOCK: i32 = 2;

pub enum extent_buffer {}
pub enum btrfs_path {}
pub enum btrfs_root {}

/*
 * We are limited in number of subclasses by MAX_LOCKDEP_SUBCLASSES, which at
 * the time of this patch is 8, which is how many we use.  Keep this in mind if
 * you decide you want to add another subclass.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum btrfs_lock_nesting {
    BTRFS_NESTING_NORMAL,
    /* When we COW a block we are holding the lock on the original block. */
    BTRFS_NESTING_COW,
    /* Lock adjacent nodes on the same level while holding the original. */
    BTRFS_NESTING_LEFT,
    BTRFS_NESTING_RIGHT,
    /* Subclasses used when splitting and COWing left/right nodes. */
    BTRFS_NESTING_LEFT_COW,
    BTRFS_NESTING_RIGHT_COW,
    /* Subclass for allocating a new split block after pushing nodes. */
    BTRFS_NESTING_SPLIT,
    /* Subclass for promoting or copying a new root block. */
    BTRFS_NESTING_NEW_ROOT,
    /* Keep from exceeding MAX_LOCKDEP_SUBCLASSES. */
    BTRFS_NESTING_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum btrfs_lockdep_trans_states {
    BTRFS_LOCKDEP_TRANS_COMMIT_PREP,
    BTRFS_LOCKDEP_TRANS_UNBLOCKED,
    BTRFS_LOCKDEP_TRANS_SUPER_COMMITTED,
    BTRFS_LOCKDEP_TRANS_COMPLETED,
}

/* Lockdep annotation and protection macros for wait events. */
#[macro_export]
macro_rules! btrfs_might_wait_for_event {
    ($owner:expr, $lock:ident) => {{
        unsafe {
            rwsem_acquire(&(*$owner).$lock##_map, 0, 0, _THIS_IP_());
            rwsem_release(&(*$owner).$lock##_map, _THIS_IP_());
        }
    }};
}

#[macro_export]
macro_rules! btrfs_lockdep_acquire {
    ($owner:expr, $lock:ident) => {{
        unsafe { rwsem_acquire_read(&(*$owner).$lock##_map, 0, 0, _THIS_IP_()); }
    }};
}

#[macro_export]
macro_rules! btrfs_lockdep_release {
    ($owner:expr, $lock:ident) => {{
        unsafe { rwsem_release(&(*$owner).$lock##_map, _THIS_IP_()); }
    }};
}

#[macro_export]
macro_rules! btrfs_lockdep_inode_acquire {
    ($owner:expr, $lock:ident) => {{
        unsafe { rwsem_acquire_read(&(*$owner).vfs_inode.lock.dep_map, 0, 0, _THIS_IP_()); }
    }};
}

#[macro_export]
macro_rules! btrfs_lockdep_inode_release {
    ($owner:expr, $lock:ident) => {{
        unsafe { rwsem_release(&(*$owner).vfs_inode.lock.dep_map, _THIS_IP_()); }
    }};
}

#[macro_export]
macro_rules! btrfs_might_wait_for_state {
    ($owner:expr, $i:expr) => {{
        unsafe {
            rwsem_acquire(&(*$owner).btrfs_state_change_map[$i], 0, 0, _THIS_IP_());
            rwsem_release(&(*$owner).btrfs_state_change_map[$i], _THIS_IP_());
        }
    }};
}

#[macro_export]
macro_rules! btrfs_trans_state_lockdep_acquire {
    ($owner:expr, $i:expr) => {{
        unsafe { rwsem_acquire_read(&(*$owner).btrfs_state_change_map[$i], 0, 0, _THIS_IP_()); }
    }};
}

#[macro_export]
macro_rules! btrfs_trans_state_lockdep_release {
    ($owner:expr, $i:expr) => {{
        unsafe { rwsem_release(&(*$owner).btrfs_state_change_map[$i], _THIS_IP_()); }
    }};
}

// Static lockdep-map initialization macros are retained as declarative hooks.
#[macro_export]
macro_rules! btrfs_lockdep_init_map { ($owner:expr, $lock:ident) => {{ /* lockdep_init_map */ }}; }
#[macro_export]
macro_rules! btrfs_state_lockdep_init_map { ($owner:expr, $lock:ident, $state:expr) => {{ /* lockdep_init_map */ }}; }

extern "C" {
    pub fn btrfs_tree_lock_nested(eb: *mut extent_buffer, nest: btrfs_lock_nesting);
    pub fn btrfs_tree_unlock(eb: *mut extent_buffer);
    pub fn btrfs_tree_read_lock_nested(eb: *mut extent_buffer, nest: btrfs_lock_nesting);
    pub fn btrfs_tree_read_unlock(eb: *mut extent_buffer);
    pub fn btrfs_try_tree_read_lock(eb: *mut extent_buffer) -> bool;
    pub fn btrfs_lock_root_node(root: *mut btrfs_root) -> *mut extent_buffer;
    pub fn btrfs_read_lock_root_node(root: *mut btrfs_root) -> *mut extent_buffer;
    pub fn btrfs_try_read_lock_root_node(root: *mut btrfs_root) -> *mut extent_buffer;
    pub fn btrfs_unlock_up_safe(path: *mut btrfs_path, level: i32);
}

#[inline]
pub unsafe fn btrfs_tree_lock(eb: *mut extent_buffer) {
    btrfs_tree_lock_nested(eb, btrfs_lock_nesting::BTRFS_NESTING_NORMAL);
}

#[inline]
pub unsafe fn btrfs_tree_read_lock(eb: *mut extent_buffer) {
    btrfs_tree_read_lock_nested(eb, btrfs_lock_nesting::BTRFS_NESTING_NORMAL);
}

// CONFIG_BTRFS_DEBUG controls whether these lock assertions are active.
#[inline]
pub unsafe fn btrfs_assert_tree_write_locked(_eb: *mut extent_buffer) {}
#[inline]
pub unsafe fn btrfs_assert_tree_read_locked(_eb: *mut extent_buffer) {}

#[inline]
pub unsafe fn btrfs_tree_unlock_rw(eb: *mut extent_buffer, rw: i32) {
    if rw == BTRFS_WRITE_LOCK {
        btrfs_tree_unlock(eb);
    } else if rw == BTRFS_READ_LOCK {
        btrfs_tree_read_unlock(eb);
    } else {
        // BUG();
        core::intrinsics::abort();
    }
}

#[repr(C)]
pub struct btrfs_drew_lock {
    pub readers: atomic_t,
    pub writers: atomic_t,
    pub pending_writers: wait_queue_head_t,
    pub pending_readers: wait_queue_head_t,
}

extern "C" {
    pub fn btrfs_drew_lock_init(lock: *mut btrfs_drew_lock);
    pub fn btrfs_drew_write_lock(lock: *mut btrfs_drew_lock);
    pub fn btrfs_drew_try_write_lock(lock: *mut btrfs_drew_lock) -> bool;
    pub fn btrfs_drew_write_unlock(lock: *mut btrfs_drew_lock);
    pub fn btrfs_drew_read_lock(lock: *mut btrfs_drew_lock);
    pub fn btrfs_drew_read_unlock(lock: *mut btrfs_drew_lock);
}

// CONFIG_DEBUG_LOCK_ALLOC controls whether these declarations are active.
extern "C" {
    pub fn btrfs_set_buffer_lockdep_class(objectid: u64, eb: *mut extent_buffer, level: i32);
    pub fn btrfs_maybe_reset_lockdep_class(root: *mut btrfs_root, eb: *mut extent_buffer);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
