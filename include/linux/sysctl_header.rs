/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/sysctl.h. */

use core::ffi::{c_char, c_void};

/* Dependencies supplied by the corresponding kernel headers. */
pub type size_t = usize;
pub type loff_t = isize;
pub type ulong = usize;
pub type uint = u32;
pub type umode_t = u16;

#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct nsproxy { _private: [u8; 0] }
#[repr(C)] pub struct rb_node { _private: [u8; 0] }
#[repr(C)] pub struct rb_root { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct hlist_head { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct kuid_t { _private: [u8; 0] }
#[repr(C)] pub struct kgid_t { _private: [u8; 0] }

#[repr(C)] pub struct ctl_table;
#[repr(C)] pub struct ctl_table_root;
#[repr(C)] pub struct ctl_table_header;
#[repr(C)] pub struct ctl_dir;
#[repr(C)] pub struct ctl_table_set;

extern "C" {
    pub static sysctl_vals: [core::ffi::c_int; 12];
    pub static sysctl_long_vals: [ulong; 3];
}

#[inline] pub unsafe fn SYSCTL_ZERO() -> *mut c_void { sysctl_vals.as_ptr() as *mut c_void }
#[inline] pub unsafe fn SYSCTL_ONE() -> *mut c_void { sysctl_vals.as_ptr().add(1) as *mut c_void }
#[inline] pub unsafe fn SYSCTL_TWO() -> *mut c_void { sysctl_vals.as_ptr().add(2) as *mut c_void }
#[inline] pub unsafe fn SYSCTL_THREE() -> *mut c_void { sysctl_vals.as_ptr().add(3) as *mut c_void }
#[inline] pub unsafe fn SYSCTL_FOUR() -> *mut c_void { sysctl_vals.as_ptr().add(4) as *mut c_void }
#[inline] pub unsafe fn SYSCTL_ONE_HUNDRED() -> *mut c_void { sysctl_vals.as_ptr().add(5) as *mut c_void }
#[inline] pub unsafe fn SYSCTL_TWO_HUNDRED() -> *mut c_void { sysctl_vals.as_ptr().add(6) as *mut c_void }
#[inline] pub unsafe fn SYSCTL_ONE_THOUSAND() -> *mut c_void { sysctl_vals.as_ptr().add(7) as *mut c_void }
#[inline] pub unsafe fn SYSCTL_THREE_THOUSAND() -> *mut c_void { sysctl_vals.as_ptr().add(8) as *mut c_void }
#[inline] pub unsafe fn SYSCTL_INT_MAX() -> *mut c_void { sysctl_vals.as_ptr().add(9) as *mut c_void }
#[inline] pub unsafe fn SYSCTL_MAXOLDUID() -> *mut c_void { sysctl_vals.as_ptr().add(10) as *mut c_void }
#[inline] pub unsafe fn SYSCTL_NEG_ONE() -> *mut c_void { sysctl_vals.as_ptr().add(11) as *mut c_void }
#[inline] pub unsafe fn SYSCTL_LONG_ZERO() -> *mut c_void { sysctl_long_vals.as_ptr() as *mut c_void }
#[inline] pub unsafe fn SYSCTL_LONG_ONE() -> *mut c_void { sysctl_long_vals.as_ptr().add(1) as *mut c_void }
#[inline] pub unsafe fn SYSCTL_LONG_MAX() -> *mut c_void { sysctl_long_vals.as_ptr().add(2) as *mut c_void }

#[inline] pub const fn SYSCTL_USER_TO_KERN(dir: i32) -> bool { dir != 0 }
#[inline] pub const fn SYSCTL_KERN_TO_USER(dir: i32) -> bool { dir == 0 }

pub type proc_handler = unsafe extern "C" fn(*const ctl_table, i32, *mut c_void, *mut size_t, *mut loff_t) -> i32;

extern "C" {
    pub fn proc_dostring(*const ctl_table, i32, *mut c_void, *mut size_t, *mut loff_t) -> i32;
    pub fn proc_dobool(*const ctl_table, i32, *mut c_void, *mut size_t, *mut loff_t) -> i32;
    pub fn proc_dointvec(*const ctl_table, i32, *mut c_void, *mut size_t, *mut loff_t) -> i32;
    pub fn proc_dointvec_minmax(*const ctl_table, i32, *mut c_void, *mut size_t, *mut loff_t) -> i32;
    pub fn proc_douintvec(*const ctl_table, i32, *mut c_void, *mut size_t, *mut loff_t) -> i32;
    pub fn proc_douintvec_minmax(*const ctl_table, i32, *mut c_void, *mut size_t, *mut loff_t) -> i32;
    pub fn proc_dou8vec_minmax(*const ctl_table, i32, *mut c_void, *mut size_t, *mut loff_t) -> i32;
    pub fn proc_doulongvec_minmax(*const ctl_table, i32, *mut c_void, *mut size_t, *mut loff_t) -> i32;
    pub fn proc_do_large_bitmap(*const ctl_table, i32, *mut c_void, *mut size_t, *mut loff_t) -> i32;
    pub fn proc_do_static_key(*const ctl_table, i32, *mut c_void, *mut size_t, *mut loff_t) -> i32;
    pub fn proc_dointvec_conv(*const ctl_table, i32, *mut c_void, *mut size_t, *mut loff_t,
        conv: Option<unsafe extern "C" fn(*mut bool, *mut ulong, *mut i32, i32, *const ctl_table) -> i32>) -> i32;
    pub fn proc_douintvec_conv(*const ctl_table, i32, *mut c_void, *mut size_t, *mut loff_t,
        conv: Option<unsafe extern "C" fn(*mut bool, *mut ulong, *mut uint, i32, *const ctl_table) -> i32>) -> i32;
    pub fn proc_doulongvec_conv(*const ctl_table, i32, *mut c_void, *mut size_t, *mut loff_t,
        conv: Option<unsafe extern "C" fn(*mut bool, *mut ulong, *mut ulong, i32, *const ctl_table) -> i32>) -> i32;
}

#[repr(C)]
pub struct ctl_table_poll { pub event: atomic_t, pub wait: wait_queue_head_t }

pub unsafe fn proc_sys_poll_event(poll: *mut ctl_table_poll) -> *mut c_void { poll as *mut c_void }

#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: i32,
    pub mode: umode_t,
    pub proc_handler: Option<proc_handler>,
    pub poll: *mut ctl_table_poll,
    pub extra1: *mut c_void,
    pub extra2: *mut c_void,
}

#[repr(C)] pub struct ctl_node { pub node: rb_node, pub header: *mut ctl_table_header }

#[repr(C)]
pub union ctl_table_header_union {
    pub fields: ctl_table_header_fields,
    pub rcu: rcu_head,
}
#[repr(C)] pub struct ctl_table_header_fields { pub ctl_table: *const ctl_table, pub ctl_table_size: i32, pub used: i32, pub count: i32, pub nreg: i32 }
#[repr(C)] pub struct ctl_table_header {
    pub inner: ctl_table_header_union,
    pub unregistering: *mut completion,
    pub ctl_table_arg: *const ctl_table,
    pub root: *mut ctl_table_root,
    pub set: *mut ctl_table_set,
    pub parent: *mut ctl_dir,
    pub node: *mut ctl_node,
    pub inodes: hlist_head,
    pub type_: ctl_table_type,
}
#[repr(C)] pub enum ctl_table_type { SYSCTL_TABLE_TYPE_DEFAULT, SYSCTL_TABLE_TYPE_PERMANENTLY_EMPTY }
#[repr(C)] pub struct ctl_dir { pub header: ctl_table_header, pub root: rb_root }
#[repr(C)] pub struct ctl_table_set { pub is_seen: Option<unsafe extern "C" fn(*mut ctl_table_set) -> i32>, pub dir: ctl_dir }
#[repr(C)] pub struct ctl_table_root {
    pub default_set: ctl_table_set,
    pub lookup: Option<unsafe extern "C" fn(*mut ctl_table_root) -> *mut ctl_table_set>,
    pub set_ownership: Option<unsafe extern "C" fn(*mut ctl_table_header, *mut kuid_t, *mut kgid_t)>,
    pub permissions: Option<unsafe extern "C" fn(*mut ctl_table_header, *const ctl_table) -> i32>,
}

/* CONFIG_SYSCTL declarations; when disabled, the C inline stubs return NULL/false or do nothing. */
extern "C" {
    pub fn proc_sys_poll_notify(*mut ctl_table_poll);
    pub fn setup_sysctl_set(*mut ctl_table_set, *mut ctl_table_root, Option<unsafe extern "C" fn(*mut ctl_table_set) -> i32>);
    pub fn retire_sysctl_set(*mut ctl_table_set);
    pub fn __register_sysctl_table(*mut ctl_table_set, *const c_char, *const ctl_table, size_t) -> *mut ctl_table_header;
    pub fn register_sysctl_sz(*const c_char, *const ctl_table, size_t) -> *mut ctl_table_header;
    pub fn unregister_sysctl_table(*mut ctl_table_header);
    pub fn sysctl_init_bases() -> i32;
    pub fn __register_sysctl_init(*const c_char, *const ctl_table, *const c_char, size_t);
    pub fn register_sysctl_mount_point(*const c_char) -> *mut ctl_table_header;
    pub fn do_sysctl_args();
    pub fn sysctl_is_alias(*mut c_char) -> bool;
    pub static mut unaligned_enabled: i32;
    pub static mut no_unaligned_warning: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
