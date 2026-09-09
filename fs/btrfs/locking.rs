// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2008 Oracle.  All rights reserved.
 */

// Linux/Btrfs dependencies supplied by other translation units.

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
static mut BTRFS_LOCKDEP_KEYSETS: [btrfs_lockdep_keyset; 15] = [
    btrfs_lockdep_keyset::new(BTRFS_ROOT_TREE_OBJECTID, "root"),
    btrfs_lockdep_keyset::new(BTRFS_EXTENT_TREE_OBJECTID, "extent"),
    btrfs_lockdep_keyset::new(BTRFS_CHUNK_TREE_OBJECTID, "chunk"),
    btrfs_lockdep_keyset::new(BTRFS_DEV_TREE_OBJECTID, "dev"),
    btrfs_lockdep_keyset::new(BTRFS_CSUM_TREE_OBJECTID, "csum"),
    btrfs_lockdep_keyset::new(BTRFS_QUOTA_TREE_OBJECTID, "quota"),
    btrfs_lockdep_keyset::new(BTRFS_TREE_LOG_OBJECTID, "log"),
    btrfs_lockdep_keyset::new(BTRFS_TREE_RELOC_OBJECTID, "treloc"),
    btrfs_lockdep_keyset::new(BTRFS_DATA_RELOC_TREE_OBJECTID, "dreloc"),
    btrfs_lockdep_keyset::new(BTRFS_UUID_TREE_OBJECTID, "uuid"),
    btrfs_lockdep_keyset::new(BTRFS_FREE_SPACE_TREE_OBJECTID, "free-space"),
    btrfs_lockdep_keyset::new(BTRFS_BLOCK_GROUP_TREE_OBJECTID, "block-group"),
    btrfs_lockdep_keyset::new(BTRFS_RAID_STRIPE_TREE_OBJECTID, "raid-stripe"),
    btrfs_lockdep_keyset::new(BTRFS_REMAP_TREE_OBJECTID, "remap"),
    btrfs_lockdep_keyset::new(0, "tree"),
];

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
#[repr(C)]
struct btrfs_lockdep_keyset {
    id: u64,
    names: [[u8; 24]; BTRFS_MAX_LEVEL],
    keys: [lock_class_key; BTRFS_MAX_LEVEL],
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
impl btrfs_lockdep_keyset {
    const fn new(id: u64, stem: &str) -> Self {
        let mut names = [[0; 24]; BTRFS_MAX_LEVEL];
        let bytes = stem.as_bytes();
        let mut level = 0;
        while level < BTRFS_MAX_LEVEL {
            let mut i = 0;
            while i < bytes.len() && i + 8 < 24 {
                names[level][i] = bytes[i];
                i += 1;
            }
            names[level][i] = b'-';
            names[level][i + 1] = b'0' + level as u8;
            level += 1;
        }
        Self { id, names, keys: [lock_class_key::new(); BTRFS_MAX_LEVEL] }
    }
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
pub unsafe fn btrfs_set_buffer_lockdep_class(objectid: u64, eb: *mut extent_buffer, level: i32) {
    let mut ks = &BTRFS_LOCKDEP_KEYSETS[BTRFS_LOCKDEP_KEYSETS.len() - 1];
    for candidate in &BTRFS_LOCKDEP_KEYSETS {
        if candidate.id == 0 || candidate.id == objectid {
            ks = candidate;
            if candidate.id == objectid { break; }
        }
    }
    lockdep_set_class_and_name(&mut (*eb).lock, &ks.keys[level as usize], ks.names[level as usize].as_ptr());
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
pub unsafe fn btrfs_maybe_reset_lockdep_class(root: *mut btrfs_root, eb: *mut extent_buffer) {
    if test_bit(BTRFS_ROOT_RESET_LOCKDEP_CLASS, &(*root).state) != 0 {
        btrfs_set_buffer_lockdep_class(btrfs_root_id(root), eb, btrfs_header_level(eb));
    }
}

#[cfg(CONFIG_BTRFS_DEBUG)]
unsafe fn btrfs_set_eb_lock_owner(eb: *mut extent_buffer, owner: pid_t) { (*eb).lock_owner = owner; }
#[cfg(not(CONFIG_BTRFS_DEBUG))]
unsafe fn btrfs_set_eb_lock_owner(_eb: *mut extent_buffer, _owner: pid_t) {}

pub unsafe fn btrfs_tree_read_lock_nested(eb: *mut extent_buffer, nest: btrfs_lock_nesting) {
    let mut start_ns: u64 = 0;
    if trace_btrfs_tree_read_lock_enabled() { start_ns = ktime_get_ns(); }
    down_read_nested(&mut (*eb).lock, nest);
    trace_btrfs_tree_read_lock(eb, start_ns);
}

pub unsafe fn btrfs_try_tree_read_lock(eb: *mut extent_buffer) -> bool {
    if down_read_trylock(&mut (*eb).lock) != 0 {
        trace_btrfs_try_tree_read_lock(eb);
        return true;
    }
    false
}

pub unsafe fn btrfs_tree_read_unlock(eb: *mut extent_buffer) {
    trace_btrfs_tree_read_unlock(eb);
    up_read(&mut (*eb).lock);
}

pub unsafe fn btrfs_tree_lock_nested(eb: *mut extent_buffer, nest: btrfs_lock_nesting) {
    let mut start_ns: u64 = 0;
    if trace_btrfs_tree_lock_enabled() { start_ns = ktime_get_ns(); }
    down_write_nested(&mut (*eb).lock, nest);
    btrfs_set_eb_lock_owner(eb, current.pid);
    trace_btrfs_tree_lock(eb, start_ns);
}

pub unsafe fn btrfs_tree_unlock(eb: *mut extent_buffer) {
    trace_btrfs_tree_unlock(eb);
    btrfs_set_eb_lock_owner(eb, 0);
    up_write(&mut (*eb).lock);
}

pub unsafe fn btrfs_unlock_up_safe(path: *mut btrfs_path, level: i32) {
    if (*path).keep_locks { return; }
    let mut i = level;
    while i < BTRFS_MAX_LEVEL as i32 {
        if (*path).nodes[i as usize].is_null() || (*path).locks[i as usize] == 0 { i += 1; continue; }
        btrfs_tree_unlock_rw((*path).nodes[i as usize], (*path).locks[i as usize]);
        (*path).locks[i as usize] = 0;
        i += 1;
    }
}

pub unsafe fn btrfs_lock_root_node(root: *mut btrfs_root) -> *mut extent_buffer {
    loop {
        let eb = btrfs_root_node(root);
        btrfs_maybe_reset_lockdep_class(root, eb);
        btrfs_tree_lock(eb);
        if eb == (*root).node { return eb; }
        btrfs_tree_unlock(eb);
        free_extent_buffer(eb);
    }
}

pub unsafe fn btrfs_read_lock_root_node(root: *mut btrfs_root) -> *mut extent_buffer {
    loop {
        let eb = btrfs_root_node(root);
        btrfs_maybe_reset_lockdep_class(root, eb);
        btrfs_tree_read_lock(eb);
        if eb == (*root).node { return eb; }
        btrfs_tree_read_unlock(eb);
        free_extent_buffer(eb);
    }
}

pub unsafe fn btrfs_try_read_lock_root_node(root: *mut btrfs_root) -> *mut extent_buffer {
    loop {
        let eb = btrfs_root_node(root);
        if !btrfs_try_tree_read_lock(eb) {
            free_extent_buffer(eb);
            return ERR_PTR(-EAGAIN);
        }
        if eb == (*root).node { return eb; }
        btrfs_tree_read_unlock(eb);
        free_extent_buffer(eb);
    }
}

pub unsafe fn btrfs_drew_lock_init(lock: *mut btrfs_drew_lock) {
    atomic_set(&mut (*lock).readers, 0);
    atomic_set(&mut (*lock).writers, 0);
    init_waitqueue_head(&mut (*lock).pending_readers);
    init_waitqueue_head(&mut (*lock).pending_writers);
}

pub unsafe fn btrfs_drew_try_write_lock(lock: *mut btrfs_drew_lock) -> bool {
    if atomic_read(&(*lock).readers) != 0 { return false; }
    atomic_inc(&mut (*lock).writers);
    smp_mb__after_atomic();
    if atomic_read(&(*lock).readers) != 0 {
        btrfs_drew_write_unlock(lock);
        return false;
    }
    true
}

pub unsafe fn btrfs_drew_write_lock(lock: *mut btrfs_drew_lock) {
    loop {
        if btrfs_drew_try_write_lock(lock) { return; }
        wait_event((*lock).pending_writers, atomic_read(&(*lock).readers) == 0);
    }
}

pub unsafe fn btrfs_drew_write_unlock(lock: *mut btrfs_drew_lock) {
    if atomic_dec_and_test(&mut (*lock).writers) != 0 { wake_up(&mut (*lock).pending_readers); }
}

pub unsafe fn btrfs_drew_read_lock(lock: *mut btrfs_drew_lock) {
    atomic_inc(&mut (*lock).readers);
    smp_mb__after_atomic();
    wait_event((*lock).pending_readers, atomic_read(&(*lock).writers) == 0);
}

pub unsafe fn btrfs_drew_read_unlock(lock: *mut btrfs_drew_lock) {
    if atomic_dec_and_test(&mut (*lock).readers) != 0 { wake_up(&mut (*lock).pending_writers); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
