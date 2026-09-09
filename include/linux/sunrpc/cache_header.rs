/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Translated from include/linux/sunrpc/cache.h.
 * C-only include directives and header guards are omitted; referenced kernel
 * types and functions are supplied by the surrounding translation unit.
 */

#[repr(C)]
pub struct cache_head {
    pub cache_list: hlist_node,
    pub expiry_time: time64_t,
    pub last_refresh: time64_t,
    pub ref_: kref,
    pub flags: c_ulong,
}

pub const CACHE_VALID: u32 = 0;
pub const CACHE_NEGATIVE: u32 = 1;
pub const CACHE_PENDING: u32 = 2;
pub const CACHE_CLEANED: u32 = 3;

pub const CACHE_NEW_EXPIRY: i32 = 120;

#[repr(C)]
pub struct cache_detail {
    pub owner: *mut module,
    pub hash_size: c_int,
    pub hash_table: *mut hlist_head,
    pub hash_lock: spinlock_t,
    pub name: *mut c_char,
    pub cache_put: Option<unsafe extern "C" fn(*mut kref)>,
    pub cache_upcall: Option<unsafe extern "C" fn(*mut cache_detail, *mut cache_head) -> c_int>,
    pub cache_notify: Option<unsafe extern "C" fn(*mut cache_detail, *mut cache_head) -> c_int>,
    pub cache_request: Option<unsafe extern "C" fn(*mut cache_detail, *mut cache_head, *mut *mut c_char, *mut c_int)>,
    pub cache_parse: Option<unsafe extern "C" fn(*mut cache_detail, *mut c_char, c_int) -> c_int>,
    pub cache_show: Option<unsafe extern "C" fn(*mut seq_file, *mut cache_detail, *mut cache_head) -> c_int>,
    pub warn_no_listener: Option<unsafe extern "C" fn(*mut cache_detail, c_int)>,
    pub alloc: Option<unsafe extern "C" fn() -> *mut cache_head>,
    pub flush: Option<unsafe extern "C" fn()>,
    pub r#match: Option<unsafe extern "C" fn(*mut cache_head, *mut cache_head) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut cache_head, *mut cache_head)>,
    pub update: Option<unsafe extern "C" fn(*mut cache_head, *mut cache_head)>,
    pub flush_time: time64_t,
    pub others: list_head,
    pub nextcheck: time64_t,
    pub entries: c_int,
    pub requests: list_head,
    pub readers: list_head,
    pub queue_lock: spinlock_t,
    pub queue_wait: wait_queue_head_t,
    pub next_seqno: u64,
    pub writers: atomic_t,
    pub last_close: time64_t,
    pub last_warn: time64_t,
    pub fs: cache_detail_fs,
    pub net: *mut net,
}

#[repr(C)]
pub union cache_detail_fs {
    pub procfs: *mut proc_dir_entry,
    pub pipefs: *mut dentry,
}

#[repr(C)]
pub struct cache_req {
    pub defer: Option<unsafe extern "C" fn(*mut cache_req) -> *mut cache_deferred_req>,
    pub thread_wait: c_ulong,
}

#[repr(C)]
pub struct cache_deferred_req {
    pub hash: hlist_node,
    pub recent: list_head,
    pub item: *mut cache_head,
    pub owner: *mut c_void,
    pub revisit: Option<unsafe extern "C" fn(*mut cache_deferred_req, c_int)>,
}

pub unsafe fn seconds_since_boot() -> time64_t {
    let mut boot: timespec64 = core::mem::zeroed();
    getboottime64(&mut boot);
    ktime_get_real_seconds() - boot.tv_sec
}

pub unsafe fn convert_to_wallclock(sinceboot: time64_t) -> time64_t {
    let mut boot: timespec64 = core::mem::zeroed();
    getboottime64(&mut boot);
    boot.tv_sec + sinceboot
}

extern "C" {
    pub static cache_file_operations_pipefs: file_operations;
    pub static content_file_operations_pipefs: file_operations;
    pub static cache_flush_operations_pipefs: file_operations;

    pub fn sunrpc_cache_lookup_rcu(detail: *mut cache_detail, key: *mut cache_head, hash: c_int) -> *mut cache_head;
    pub fn sunrpc_cache_update(detail: *mut cache_detail, new_: *mut cache_head, old: *mut cache_head, hash: c_int) -> *mut cache_head;
    pub fn sunrpc_cache_upcall(detail: *mut cache_detail, h: *mut cache_head) -> c_int;
    pub fn sunrpc_cache_upcall_warn(detail: *mut cache_detail, h: *mut cache_head) -> c_int;
    pub fn cache_clean_deferred(owner: *mut c_void);
    pub fn cache_check_rcu(detail: *mut cache_detail, h: *mut cache_head, rqstp: *mut cache_req) -> c_int;
    pub fn cache_check(detail: *mut cache_detail, h: *mut cache_head, rqstp: *mut cache_req) -> c_int;
    pub fn cache_flush();
    pub fn cache_purge(detail: *mut cache_detail);
    pub fn cache_initialize();
}

pub unsafe fn cache_get(h: *mut cache_head) -> *mut cache_head {
    kref_get(&mut (*h).ref_);
    h
}

pub unsafe fn cache_get_rcu(h: *mut cache_head) -> *mut cache_head {
    if kref_get_unless_zero(&mut (*h).ref_) { h } else { core::ptr::null_mut() }
}

pub unsafe fn cache_put(h: *mut cache_head, cd: *mut cache_detail) {
    if kref_read(&(*h).ref_) <= 2 && (*h).expiry_time < (*cd).nextcheck {
        (*cd).nextcheck = (*h).expiry_time;
    }
    kref_put(&mut (*h).ref_, (*cd).cache_put);
}

pub unsafe fn cache_is_expired(detail: *mut cache_detail, h: *mut cache_head) -> bool {
    if (*h).expiry_time < seconds_since_boot() { return true; }
    if !test_bit(CACHE_VALID as c_ulong, &(*h).flags) { return false; }
    (*detail).flush_time >= (*h).last_refresh
}

pub const NEVER: i32 = 0x7FFFFFFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
