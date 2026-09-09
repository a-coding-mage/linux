/* SPDX-License-Identifier: GPL-2.0 */
/* S/390 debug facility; translated from the C header. */

use core::ffi::{c_char, c_int, c_void};

pub const DEBUG_MAX_LEVEL: c_int = 6;
pub const DEBUG_OFF_LEVEL: c_int = -1;
pub const DEBUG_FLUSH_ALL: c_int = -1;
pub const DEBUG_MAX_VIEWS: usize = 10;
pub const DEBUG_MAX_NAME_LEN: usize = 64;
pub const DEBUG_DEFAULT_LEVEL: c_int = 3;
pub const DEBUG_DIR_ROOT: &[u8] = b"s390dbf\0";
pub const __DEBUG_FEATURE_VERSION: c_int = 3;
pub const DEBUG_SPRINTF_MAX_ARGS: c_int = 10;
pub const EARLY_PAGES: usize = 8;
pub const EARLY_AREAS: usize = 1;

pub type size_t = usize;
pub type ssize_t = isize;
pub type umode_t = u16;
pub type uid_t = u32;
pub type gid_t = u32;
pub type loff_t = i64;

#[repr(C)]
pub struct refcount_t;
#[repr(C)]
pub struct raw_spinlock_t;
#[repr(C)]
pub struct dentry;
#[repr(C)]
pub struct file;

#[repr(C, packed)]
pub struct __debug_entry {
    /* C bitfields: clock:60, exception:1, level:3, represented in one word. */
    pub clock_exception_level: u64,
    pub caller: *mut c_void,
    pub cpu: u16,
}
pub type debug_entry_t = __debug_entry;

#[repr(C)]
pub struct debug_view;

#[repr(C)]
pub struct debug_info {
    pub next: *mut debug_info,
    pub prev: *mut debug_info,
    pub ref_count: refcount_t,
    pub lock: raw_spinlock_t,
    pub level: c_int,
    pub nr_areas: c_int,
    pub pages_per_area: c_int,
    pub buf_size: c_int,
    pub entry_size: c_int,
    pub areas: *mut *mut *mut debug_entry_t,
    pub active_area: c_int,
    pub active_pages: *mut c_int,
    pub active_entries: *mut c_int,
    pub debugfs_root_entry: *mut dentry,
    pub debugfs_entries: [*mut dentry; DEBUG_MAX_VIEWS],
    pub views: [*mut debug_view; DEBUG_MAX_VIEWS],
    pub name: [c_char; DEBUG_MAX_NAME_LEN],
    pub mode: umode_t,
}
pub type debug_info_t = debug_info;

pub type debug_header_proc_t = unsafe extern "C" fn(*mut debug_info_t, *mut debug_view, c_int, *mut debug_entry_t, *mut c_char, size_t) -> c_int;
pub type debug_format_proc_t = unsafe extern "C" fn(*mut debug_info_t, *mut debug_view, *mut c_char, size_t, *const c_char) -> c_int;
pub type debug_prolog_proc_t = unsafe extern "C" fn(*mut debug_info_t, *mut debug_view, *mut c_char, size_t) -> c_int;
pub type debug_input_proc_t = unsafe extern "C" fn(*mut debug_info_t, *mut debug_view, *mut file, *const c_char, size_t, *mut loff_t) -> c_int;

#[repr(C)]
pub struct debug_view {
    pub name: [c_char; DEBUG_MAX_NAME_LEN],
    pub prolog_proc: Option<debug_prolog_proc_t>,
    pub header_proc: Option<debug_header_proc_t>,
    pub format_proc: Option<debug_format_proc_t>,
    pub input_proc: Option<debug_input_proc_t>,
    pub private_data: *mut c_void,
}

unsafe extern "C" {
    pub static mut debug_hex_ascii_view: debug_view;
    pub static mut debug_sprintf_view: debug_view;
    pub fn debug_dflt_header_fn(id: *mut debug_info_t, view: *mut debug_view, area: c_int, entry: *mut debug_entry_t, out_buf: *mut c_char, out_buf_size: size_t) -> c_int;
    pub fn debug_sprintf_format_fn(id: *mut debug_info_t, view: *mut debug_view, out_buf: *mut c_char, out_buf_size: size_t, inbuf: *const c_char) -> c_int;
    pub fn debug_event_common(id: *mut debug_info_t, level: c_int, data: *const c_void, length: c_int) -> *mut debug_entry_t;
    pub fn debug_exception_common(id: *mut debug_info_t, level: c_int, data: *const c_void, length: c_int) -> *mut debug_entry_t;
    pub fn debug_register(name: *const c_char, pages: c_int, nr_areas: c_int, buf_size: c_int) -> *mut debug_info_t;
    pub fn debug_register_mode(name: *const c_char, pages: c_int, nr_areas: c_int, buf_size: c_int, mode: umode_t, uid: uid_t, gid: gid_t) -> *mut debug_info_t;
    pub fn debug_dump(id: *mut debug_info_t, view: *mut debug_view, buf: *mut c_char, buf_size: size_t, reverse: bool) -> ssize_t;
    pub fn debug_unregister(id: *mut debug_info_t);
    pub fn debug_set_level(id: *mut debug_info_t, new_level: c_int);
    pub fn debug_set_critical();
    pub fn debug_stop_all();
    pub fn __debug_sprintf_event(id: *mut debug_info_t, level: c_int, string: *mut c_char, ...) -> *mut debug_entry_t;
    pub fn __debug_sprintf_exception(id: *mut debug_info_t, level: c_int, string: *mut c_char, ...) -> *mut debug_entry_t;
    pub fn debug_register_view(id: *mut debug_info_t, view: *mut debug_view) -> c_int;
    pub fn debug_unregister_view(id: *mut debug_info_t, view: *mut debug_view) -> c_int;
    pub fn debug_register_static(id: *mut debug_info_t, pages_per_area: c_int, nr_areas: c_int);
}

#[inline]
pub unsafe fn debug_level_enabled(id: *mut debug_info_t, level: c_int) -> bool { !id.is_null() && level <= (*id).level }

#[inline]
pub unsafe fn debug_event(id: *mut debug_info_t, level: c_int, data: *mut c_void, length: c_int) -> *mut debug_entry_t {
    if id.is_null() || level > (*id).level || (*id).pages_per_area == 0 { core::ptr::null_mut() } else { debug_event_common(id, level, data, length) }
}

#[inline]
pub unsafe fn debug_exception(id: *mut debug_info_t, level: c_int, data: *mut c_void, length: c_int) -> *mut debug_entry_t {
    if id.is_null() || level > (*id).level || (*id).pages_per_area == 0 { core::ptr::null_mut() } else { debug_exception_common(id, level, data, length) }
}

/* The remaining typed event helpers and variadic C macros are represented by
 * direct Rust functions where Rust's type system permits the same operation. */
#[inline] pub unsafe fn debug_int_event(id: *mut debug_info_t, level: c_int, tag: u32) -> *mut debug_entry_t { debug_event(id, level, &tag as *const u32 as *mut c_void, core::mem::size_of::<u32>() as c_int) }
#[inline] pub unsafe fn debug_long_event(id: *mut debug_info_t, level: c_int, tag: u64) -> *mut debug_entry_t { debug_event(id, level, &tag as *const u64 as *mut c_void, core::mem::size_of::<u64>() as c_int) }
#[inline] pub unsafe fn debug_exception_int(id: *mut debug_info_t, level: c_int, tag: u32) -> *mut debug_entry_t { debug_exception(id, level, &tag as *const u32 as *mut c_void, core::mem::size_of::<u32>() as c_int) }
#[inline] pub unsafe fn debug_exception_long(id: *mut debug_info_t, level: c_int, tag: u64) -> *mut debug_entry_t { debug_exception(id, level, &tag as *const u64 as *mut c_void, core::mem::size_of::<u64>() as c_int) }

#[inline]
pub unsafe fn debug_text_event(id: *mut debug_info_t, level: c_int, txt: *const c_char, length: size_t) -> *mut debug_entry_t { debug_event(id, level, txt as *mut c_void, length as c_int) }
#[inline]
pub unsafe fn debug_text_exception(id: *mut debug_info_t, level: c_int, txt: *const c_char, length: size_t) -> *mut debug_entry_t { debug_exception(id, level, txt as *mut c_void, length as c_int) }

#[inline]
pub unsafe fn debug_sprintf_event(id: *mut debug_info_t, level: c_int, fmt: *mut c_char) -> *mut debug_entry_t { if id.is_null() || level > (*id).level { core::ptr::null_mut() } else { __debug_sprintf_event(id, level, fmt) } }
#[inline]
pub unsafe fn debug_sprintf_exception(id: *mut debug_info_t, level: c_int, fmt: *mut c_char) -> *mut debug_entry_t { if id.is_null() || level > (*id).level { core::ptr::null_mut() } else { __debug_sprintf_exception(id, level, fmt) } }

#[inline]
pub unsafe fn debug_data(entry: *mut debug_entry_t) -> *mut c_char { entry.add(1) as *mut c_char }

/* MODULE-only static-area macros retained as declarative placeholders because
 * PAGE_SIZE, initdata, section, and arch_initcall are build-time kernel items. */
#[macro_export]
macro_rules! __DEFINE_STATIC_AREA { ($var:ident) => {}; }
#[macro_export]
macro_rules! __DEBUG_INFO_INIT { ($var:ident, $name:expr, $buf_size:expr) => { debug_info { next: core::ptr::null_mut(), prev: core::ptr::null_mut(), ref_count: unsafe { core::mem::zeroed() }, lock: unsafe { core::mem::zeroed() }, level: DEBUG_DEFAULT_LEVEL, nr_areas: EARLY_AREAS as c_int, pages_per_area: EARLY_PAGES as c_int, buf_size: $buf_size, entry_size: core::mem::size_of::<debug_entry_t>() as c_int + $buf_size, areas: core::ptr::null_mut(), active_area: 0, active_pages: core::ptr::null_mut(), active_entries: core::ptr::null_mut(), debugfs_root_entry: core::ptr::null_mut(), debugfs_entries: [core::ptr::null_mut(); DEBUG_MAX_VIEWS], views: [core::ptr::null_mut(); DEBUG_MAX_VIEWS], name: $name, mode: 0o600 } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
