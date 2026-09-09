/* SPDX-License-Identifier: GPL-2.0-or-later */
/* General netfs cache on cache files internal defs */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

// C header dependencies are supplied by the surrounding kernel translation.

pub const CACHEFILES_DIO_BLOCK_SIZE: usize = 4096;

#[repr(i32)]
pub enum cachefiles_content {
    CACHEFILES_CONTENT_NO_DATA = 0,
    CACHEFILES_CONTENT_SINGLE = 1,
    CACHEFILES_CONTENT_ALL = 2,
    CACHEFILES_CONTENT_BACKFS_MAP = 3,
    CACHEFILES_CONTENT_DIRTY = 4,
    nr__cachefiles_content,
}

#[repr(C)]
pub struct cachefiles_volume {
    pub cache: *mut cachefiles_cache,
    pub cache_link: list_head,
    pub vcookie: *mut fscache_volume,
    pub dentry: *mut dentry,
    pub fanout: [*mut dentry; 256],
}

#[repr(C)]
pub struct cachefiles_object {
    pub cookie: *mut fscache_cookie,
    pub volume: *mut cachefiles_volume,
    pub cache_link: list_head,
    pub file: *mut file,
    pub d_name: *mut c_char,
    pub debug_id: c_int,
    pub lock: spinlock_t,
    pub ref_: refcount_t,
    pub content_info: cachefiles_content,
    pub flags: c_ulong,
}
pub const CACHEFILES_OBJECT_USING_TMPFILE: u32 = 0;

#[repr(C)]
pub struct cachefiles_cache {
    pub cache: *mut fscache_cache,
    pub mnt: *mut vfsmount,
    pub store: *mut dentry,
    pub graveyard: *mut dentry,
    pub cachefilesd: *mut file,
    pub volumes: list_head,
    pub object_list: list_head,
    pub object_list_lock: spinlock_t,
    pub cache_cred: *const cred,
    pub daemon_mutex: mutex,
    pub daemon_pollwq: wait_queue_head_t,
    pub gravecounter: atomic_t,
    pub f_released: atomic_t,
    pub b_released: atomic_long_t,
    pub b_writing: atomic_long_t,
    pub frun_percent: c_uint,
    pub fcull_percent: c_uint,
    pub fstop_percent: c_uint,
    pub brun_percent: c_uint,
    pub bcull_percent: c_uint,
    pub bstop_percent: c_uint,
    pub bsize: c_uint,
    pub bshift: c_uint,
    pub frun: u64,
    pub fcull: u64,
    pub fstop: u64,
    pub brun: sector_t,
    pub bcull: sector_t,
    pub bstop: sector_t,
    pub flags: c_ulong,
    pub rootdirname: *mut c_char,
    pub tag: *mut c_char,
    pub secid: u32,
    pub have_secid: bool,
}
pub const CACHEFILES_READY: u32 = 0;
pub const CACHEFILES_DEAD: u32 = 1;
pub const CACHEFILES_CULLING: u32 = 2;
pub const CACHEFILES_STATE_CHANGED: u32 = 3;

pub unsafe fn cachefiles_cres_file(cres: *mut netfs_cache_resources) -> *mut file {
    (*cres).cache_priv2
}
pub unsafe fn cachefiles_cres_object(cres: *mut netfs_cache_resources) -> *mut cachefiles_object {
    fscache_cres_cookie(cres).cache_priv
}
pub unsafe fn cachefiles_state_changed(cache: *mut cachefiles_cache) {
    set_bit(CACHEFILES_STATE_CHANGED, &mut (*cache).flags);
    wake_up_all(&mut (*cache).daemon_pollwq);
}

pub unsafe fn cachefiles_begin_secure(cache: *mut cachefiles_cache, saved_cred: *mut *const cred) {
    *saved_cred = override_creds((*cache).cache_cred);
}
pub unsafe fn cachefiles_end_secure(_cache: *mut cachefiles_cache, saved_cred: *const cred) {
    revert_creds(saved_cred);
}

pub enum cachefiles_has_space_for {
    cachefiles_has_space_check,
    cachefiles_has_space_for_write,
    cachefiles_has_space_for_create,
}

extern "C" {
    pub fn cachefiles_add_cache(cache: *mut cachefiles_cache) -> c_int;
    pub fn cachefiles_withdraw_cache(cache: *mut cachefiles_cache);
    pub fn cachefiles_has_space(cache: *mut cachefiles_cache, fnr: c_uint, bnr: c_uint, reason: cachefiles_has_space_for) -> c_int;
    pub static cachefiles_daemon_fops: file_operations;
    pub static cachefiles_cache_ops: fscache_cache_ops;
    pub fn cachefiles_see_object(object: *mut cachefiles_object, why: enum_cachefiles_obj_ref_trace);
    pub fn cachefiles_grab_object(object: *mut cachefiles_object, why: enum_cachefiles_obj_ref_trace) -> *mut cachefiles_object;
    pub fn cachefiles_put_object(object: *mut cachefiles_object, why: enum_cachefiles_obj_ref_trace);
    pub fn cachefiles_begin_operation(cres: *mut netfs_cache_resources, want_state: fscache_want_state) -> bool;
    pub fn __cachefiles_prepare_write(object: *mut cachefiles_object, file: *mut file, start: *mut loff_t, len: *mut size_t, upper_len: size_t, no_space_allocated_yet: bool) -> c_int;
    pub fn __cachefiles_write(object: *mut cachefiles_object, file: *mut file, start_pos: loff_t, iter: *mut iov_iter, term_func: netfs_io_terminated_t, term_func_priv: *mut c_void) -> c_int;
    pub fn cachefiles_cook_key(object: *mut cachefiles_object) -> bool;
    pub static mut cachefiles_object_jar: *mut kmem_cache;
    pub fn cachefiles_unmark_inode_in_use(object: *mut cachefiles_object, file: *mut file);
    pub fn cachefiles_bury_object(cache: *mut cachefiles_cache, object: *mut cachefiles_object, dir: *mut dentry, rep: *mut dentry, why: fscache_why_object_killed) -> c_int;
    pub fn cachefiles_delete_object(object: *mut cachefiles_object, why: fscache_why_object_killed) -> c_int;
    pub fn cachefiles_look_up_object(object: *mut cachefiles_object) -> bool;
    pub fn cachefiles_get_directory(cache: *mut cachefiles_cache, dir: *mut dentry, name: *const c_char, is_new: *mut bool) -> *mut dentry;
    pub fn cachefiles_put_directory(dir: *mut dentry);
    pub fn cachefiles_cull(cache: *mut cachefiles_cache, dir: *mut dentry, filename: *mut c_char) -> c_int;
    pub fn cachefiles_check_in_use(cache: *mut cachefiles_cache, dir: *mut dentry, filename: *mut c_char) -> c_int;
    pub fn cachefiles_create_tmpfile(object: *mut cachefiles_object) -> *mut file;
    pub fn cachefiles_commit_tmpfile(cache: *mut cachefiles_cache, object: *mut cachefiles_object) -> bool;
    pub fn cachefiles_get_security_ID(cache: *mut cachefiles_cache) -> c_int;
    pub fn cachefiles_determine_cache_security(cache: *mut cachefiles_cache, root: *mut dentry, saved_cred: *mut *const cred) -> c_int;
    pub fn cachefiles_acquire_volume(volume: *mut fscache_volume);
    pub fn cachefiles_free_volume(volume: *mut fscache_volume);
    pub fn cachefiles_withdraw_volume(volume: *mut cachefiles_volume);
    pub fn cachefiles_set_object_xattr(object: *mut cachefiles_object) -> c_int;
    pub fn cachefiles_check_auxdata(object: *mut cachefiles_object, file: *mut file) -> c_int;
    pub fn cachefiles_remove_object_xattr(cache: *mut cachefiles_cache, object: *mut cachefiles_object, dentry: *mut dentry) -> c_int;
    pub fn cachefiles_prepare_to_write(cookie: *mut fscache_cookie);
    pub fn cachefiles_set_volume_xattr(volume: *mut cachefiles_volume) -> bool;
    pub fn cachefiles_check_volume_xattr(volume: *mut cachefiles_volume) -> c_int;
}

pub const CACHEFILES_DEBUG_KENTER: u32 = 1;
pub const CACHEFILES_DEBUG_KLEAVE: u32 = 2;
pub const CACHEFILES_DEBUG_KDEBUG: u32 = 4;

pub unsafe fn cachefiles_inject_read_error() -> c_int { if cachefiles_error_injection_state & 2 != 0 { -EIO } else { 0 } }
pub unsafe fn cachefiles_inject_write_error() -> c_int { if cachefiles_error_injection_state & 2 != 0 { -EIO } else if cachefiles_error_injection_state & 1 != 0 { -ENOSPC } else { 0 } }
pub unsafe fn cachefiles_inject_remove_error() -> c_int { if cachefiles_error_injection_state & 2 != 0 { -EIO } else { 0 } }

#[cfg(CONFIG_CACHEFILES_ERROR_INJECTION)]
extern "C" { pub static mut cachefiles_error_injection_state: c_uint; pub fn cachefiles_register_error_injection() -> c_int; pub fn cachefiles_unregister_error_injection(); }
#[cfg(not(CONFIG_CACHEFILES_ERROR_INJECTION))]
pub static mut cachefiles_error_injection_state: c_uint = 0;
#[cfg(not(CONFIG_CACHEFILES_ERROR_INJECTION))]
pub unsafe fn cachefiles_register_error_injection() -> c_int { 0 }
#[cfg(not(CONFIG_CACHEFILES_ERROR_INJECTION))]
pub unsafe fn cachefiles_unregister_error_injection() {}

// C variadic logging/debugging macros (_enter, _leave, _debug, ASSERT, ASSERTCMP,
// ASSERTIF, and ASSERTIFCMP) retain their conditional intent in the declarations
// and call sites that consume this header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
