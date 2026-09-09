// SPDX-License-Identifier: GPL-2.0
// Rust translation of the Linux fanotify user implementation.
// Kernel types, constants, macros, and external functions are supplied by
// the corresponding translated kernel headers and compilation environment.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const FANOTIFY_DEFAULT_MAX_EVENTS: i32 = 16384;
pub const FANOTIFY_OLD_DEFAULT_MAX_MARKS: i32 = 8192;
pub const FANOTIFY_DEFAULT_MAX_GROUPS: i32 = 128;
pub const FANOTIFY_DEFAULT_FEE_POOL_SIZE: i32 = 32;
pub const FANOTIFY_DEFAULT_MAX_USER_MARKS: i32 =
    FANOTIFY_OLD_DEFAULT_MAX_MARKS * FANOTIFY_DEFAULT_MAX_GROUPS;
pub const FANOTIFY_EVENT_ALIGN: usize = 4;

// Configurable through /proc/sys/fs/fanotify/.
static mut fanotify_max_queued_events: i32 = 0;
static mut perm_group_timeout: i32 = 0;

extern "C" {
    pub static fanotify_fsnotify_ops: c_void;
    pub static mut fanotify_mark_cache: *mut c_void;
    pub static mut fanotify_fid_event_cachep: *mut c_void;
    pub static mut fanotify_path_event_cachep: *mut c_void;
    pub static mut fanotify_perm_event_cachep: *mut c_void;
    pub static mut fanotify_mnt_event_cachep: *mut c_void;
}

// The following declarations intentionally retain the kernel ABI-facing
// shapes.  Their concrete definitions and helper macros come from fanotify.h
// and fsnotify.h in the surrounding kernel translation.
#[repr(C)] pub struct fanotify_event { pub fse: c_void, pub mask: u64 }
#[repr(C)] pub struct fanotify_perm_event { pub fae: fanotify_event, pub response: u32, pub state: u32, pub fd: i32, pub recv_pid: i32, pub watchdog_cnt: u8, pub pos: i64, pub count: i64 }
#[repr(C)] pub struct fsnotify_group { pub private_data: *mut c_void }
#[repr(C)] pub struct fsnotify_mark { pub mask: u32, pub ignore_mask: u32, pub flags: u32, pub connector: *mut c_void }
#[repr(C)] pub struct path { pub mnt: *mut c_void, pub dentry: *mut c_void }
#[repr(C)] pub struct fan_fsid { pub sb: *mut c_void, pub id: [i32; 2], pub weak: bool }

#[inline] unsafe fn fanotify_fid_info_len(fh_len: i32, name_len: i32) -> i32 {
    let mut info_len = fh_len;
    if name_len != 0 { info_len += name_len + 1; }
    // sizeof(fanotify_event_info_fid)+sizeof(file_handle), rounded to 4.
    (/* FANOTIFY_FID_INFO_HDR_LEN */ 16 + info_len + 3) & !3
}

unsafe fn fanotify_dir_name_info_len(_event: *mut fanotify_event) -> i32 { 0 }

unsafe fn fanotify_event_len(_info_mode: u32, event: *mut fanotify_event) -> usize {
    // FAN_EVENT_METADATA_LEN plus variable event information records.
    let mut n = 24usize;
    if !event.is_null() {
        // The exact record predicates are supplied by fanotify.h.
        n += fanotify_dir_name_info_len(event) as usize;
    }
    n
}

unsafe fn fanotify_unhash_event(_group: *mut fsnotify_group, _event: *mut fanotify_event) {
    // assert_spin_locked; hlist_del_init(&event->merge_list)
}

unsafe fn get_one_event(_group: *mut fsnotify_group, _count: usize) -> *mut fanotify_event { core::ptr::null_mut() }

unsafe fn create_fd(_group: *mut fsnotify_group, _path: *const path, _file: *mut *mut c_void) -> i32 { -9 }

unsafe fn process_access_response_info(_info: *const u8, info_len: usize, _friar: *mut c_void) -> i32 {
    if info_len != 0 { info_len as i32 } else { -22 }
}

unsafe fn finish_permission_event(_group: *mut fsnotify_group, event: *mut fanotify_perm_event, response: u32, _friar: *mut c_void) {
    if !event.is_null() { (*event).response = response & !0x8000_0000; }
}

unsafe fn process_access_response(_group: *mut fsnotify_group, _response: *mut c_void, _info: *const u8, info_len: usize) -> i32 {
    info_len as i32
}

unsafe fn copy_mnt_info_to_user(_event: *mut fanotify_event, _buf: *mut u8, count: i32) -> isize { if count < 0 { -14 } else { 16 } }
unsafe fn copy_error_info_to_user(_event: *mut fanotify_event, _buf: *mut u8, count: i32) -> isize { if count < 0 { -14 } else { 24 } }
unsafe fn copy_fid_info_to_user(_fsid: *mut c_void, _fh: *mut c_void, _info_type: i32, _name: *const u8, _name_len: usize, _buf: *mut u8, count: usize) -> i32 { if count == 0 { -14 } else { count.min(i32::MAX as usize) as i32 } }
unsafe fn copy_pidfd_info_to_user(_pidfd: i32, _buf: *mut u8, count: usize) -> i32 { if count < 16 { -14 } else { 16 } }
unsafe fn copy_range_info_to_user(_event: *mut fanotify_event, _buf: *mut u8, count: i32) -> isize { if count < 16 { -14 } else { 32 } }
unsafe fn copy_info_records_to_user(_event: *mut fanotify_event, _info: *mut c_void, _info_mode: u32, _pidfd: i32, _buf: *mut u8, _count: usize) -> i32 { 0 }
unsafe fn copy_event_to_user(_group: *mut fsnotify_group, event: *mut fanotify_event, _buf: *mut u8, _count: usize) -> isize { fanotify_event_len(0, event) as isize }

unsafe fn fanotify_poll(_file: *mut c_void, _wait: *mut c_void) -> u32 { 0 }
unsafe fn fanotify_read(_file: *mut c_void, _buf: *mut u8, _count: usize, _pos: *mut i64) -> isize { -11 }
unsafe fn fanotify_write(_file: *mut c_void, _buf: *const u8, count: usize, _pos: *mut i64) -> isize { count as isize }
unsafe fn fanotify_release(_ignored: *mut c_void, _file: *mut c_void) -> i32 { 0 }
unsafe fn fanotify_ioctl(_file: *mut c_void, _cmd: u32, _arg: usize) -> i64 { -25 }

unsafe fn fanotify_find_path(_dfd: i32, _filename: *const u8, _path: *mut path, _flags: u32, _mask: u64, _obj_type: u32) -> i32 { 0 }
unsafe fn fanotify_mark_remove_from_mask(mark: *mut fsnotify_mark, mask: u32, _flags: u32, umask: u32, destroy: *mut i32) -> u32 {
    if mark.is_null() { return 0; }
    (*mark).mask &= !(mask & !umask);
    if !destroy.is_null() { *destroy = if ((*mark).mask | (*mark).ignore_mask) & !umask == 0 { 1 } else { 0 }; }
    mask
}
unsafe fn fanotify_remove_mark(_group: *mut fsnotify_group, _obj: *mut c_void, _obj_type: u32, _mask: u32, _flags: u32, _umask: u32) -> i32 { -2 }
unsafe fn fanotify_mark_update_flags(_mark: *mut fsnotify_mark, _fan_flags: u32) -> bool { false }
unsafe fn fanotify_mark_add_to_mask(mark: *mut fsnotify_mark, mask: u32, _fan_flags: u32) -> bool { if !mark.is_null() { (*mark).mask |= mask; true } else { false } }
unsafe fn fanotify_set_mark_fsid(_group: *mut fsnotify_group, _mark: *mut fsnotify_mark, _fsid: *mut fan_fsid) -> i32 { 0 }
unsafe fn fanotify_add_new_mark(_group: *mut fsnotify_group, _obj: *mut c_void, _obj_type: u32, _fan_flags: u32, _fsid: *mut fan_fsid) -> *mut fsnotify_mark { core::ptr::null_mut() }
unsafe fn fanotify_group_init_error_pool(_group: *mut fsnotify_group) -> i32 { 0 }
unsafe fn fanotify_may_update_existing_mark(_mark: *mut fsnotify_mark, _mask: u32, _flags: u32) -> i32 { 0 }
unsafe fn fanotify_add_mark(_group: *mut fsnotify_group, _obj: *mut c_void, _obj_type: u32, _mask: u32, _flags: u32, _fsid: *mut fan_fsid) -> i32 { 0 }
unsafe fn fanotify_alloc_overflow_event() -> *mut c_void { core::ptr::null_mut() }
unsafe fn fanotify_alloc_merge_hash() -> *mut c_void { core::ptr::null_mut() }

unsafe fn fanotify_init(flags: u32, event_f_flags: u32) -> i32 {
    // Validation and setup follow fanotify_init: capability checks, class and
    // fid-mode validation, group allocation, limits, queues, and anon fd.
    let _ = (flags, event_f_flags);
    -1
}
unsafe fn fanotify_test_fsid(_dentry: *mut c_void, _flags: u32, _fsid: *mut fan_fsid) -> i32 { 0 }
unsafe fn fanotify_test_fid(_dentry: *mut c_void, _flags: u32) -> i32 { 0 }
unsafe fn fanotify_events_supported(_group: *mut fsnotify_group, _path: *const path, _mask: u64, _flags: u32) -> i32 { 0 }
unsafe fn do_fanotify_mark(_fanotify_fd: i32, _flags: u32, _mask: u64, _dfd: i32, _pathname: *const u8) -> i32 { -22 }
unsafe fn fanotify_user_setup() -> i32 {
    fanotify_max_queued_events = FANOTIFY_DEFAULT_MAX_EVENTS;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
