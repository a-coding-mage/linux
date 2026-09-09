/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Squashfs
 *
 * Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * squashfs_fs_sb.h
 *
 * The declarations below depend on types supplied by squashfs_fs.h and the
 * surrounding kernel environment.
 */

/*
 * Waiters for a cache entry sleep on wait_queue as exclusive waiters, so
 * freeing one entry wakes one task.  See squashfs_cache_get().
 *
 * num_waiters is only a hint used to skip pointless wakeups: it is
 * incremented before a task queues itself and decremented after it is woken,
 * so it can transiently exceed the number of queued tasks.  It never
 * undercounts them, which is what the wakeup paths rely on.
 */
#[repr(C)]
pub struct squashfs_cache {
    pub name: *mut i8,
    pub entries: i32,
    pub curr_blk: i32,
    pub next_blk: i32,
    pub num_waiters: i32,
    pub unused: i32,
    pub block_size: i32,
    pub pages: i32,
    pub lock: spinlock_t,
    pub wait_queue: wait_queue_head_t,
    pub entry: *mut squashfs_cache_entry,
}

#[repr(C)]
pub struct squashfs_cache_entry {
    pub block: u64,
    pub length: i32,
    pub refcount: i32,
    pub next_index: u64,
    pub pending: i32,
    pub error: i32,
    pub num_waiters: i32,
    pub wait_queue: wait_queue_head_t,
    pub cache: *mut squashfs_cache,
    pub data: *mut *mut core::ffi::c_void,
    pub actor: *mut squashfs_page_actor,
}

#[repr(C)]
pub struct squashfs_sb_info {
    pub decompressor: *const squashfs_decompressor,
    pub devblksize: i32,
    pub devblksize_log2: i32,
    pub block_cache: *mut squashfs_cache,
    pub fragment_cache: *mut squashfs_cache,
    pub read_page: *mut squashfs_cache,
    pub cache_mapping: *mut address_space,
    pub next_meta_index: i32,
    pub id_table: *mut __le64,
    pub fragment_index: *mut __le64,
    pub xattr_id_table: *mut __le64,
    pub meta_index_mutex: mutex,
    pub meta_index: *mut meta_index,
    pub stream: *mut core::ffi::c_void,
    pub inode_lookup_table: *mut __le64,
    pub inode_table: u64,
    pub directory_table: u64,
    pub xattr_table: u64,
    pub block_size: u32,
    pub block_log: u16,
    pub bytes_used: i64,
    pub inodes: u32,
    pub fragments: u32,
    pub xattr_ids: u32,
    pub ids: u32,
    pub panic_on_errors: bool,
    pub thread_ops: *const squashfs_decompressor_thread_ops,
    pub max_thread_num: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
