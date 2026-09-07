// SPDX-License-Identifier: GPL-2.0-only
// AppArmor security module
// This file contains AppArmor lib definitions
// 2017 Canonical Ltd.

use core::mem::offset_of;

#[repr(C)] pub struct aa_dfa { _private: [u8; 0] }
#[repr(C)] pub struct kref { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct dentry { pub d_sb: *mut super_block }
#[repr(C)] pub struct super_block { pub s_flags: u32 }
#[repr(C)] pub struct lsm_blob_sizes { _private: [u8; 0] }
#[repr(C)] pub struct aa_label { pub size: i32, pub flags: u32, pub ns: *mut aa_ns }
#[repr(C)] pub struct aa_profile { pub label: aa_label, pub ns: *mut aa_ns }
#[repr(C)] pub struct aa_ns { _private: [u8; 0] }
pub type aa_state_t = u32;
pub type gfp_t = u32;

extern "C" {
    fn pr_debug(fmt: *const i8, ...);
    fn pr_warn_ratelimited(fmt: *const i8, ...);
    fn pr_err_ratelimited(fmt: *const i8, ...);
    pub static mut stacksplitdfa: *mut aa_dfa;
    pub static mut aa_g_debug: i32;
    pub static mut apparmor_initialized: i32;
    pub static mut apparmor_blob_sizes: lsm_blob_sizes;
    pub fn aa_parse_debug_params(str_: *const i8) -> i32;
    pub fn aa_print_debug_params(buffer: *mut i8) -> i32;
    pub fn skipn_spaces(str_: *const i8, n: usize) -> *const i8;
    pub fn aa_splitn_fqname(fqname: *const i8, n: usize, ns_name: *mut *const i8, ns_len: *mut usize) -> *const i8;
    pub fn aa_info_message(str_: *const i8);
    pub fn aa_dfa_next(dfa: *const aa_dfa, start: aa_state_t, c: u8) -> aa_state_t;
    pub fn aa_resize_str_table(t: *mut aa_str_table, newsize: i32, gfp: gfp_t) -> bool;
    pub fn aa_destroy_str_table(table: *mut aa_str_table);
    pub fn aa_str_kref(kref: *mut kref);
    pub fn aa_str_alloc(size: i32, gfp: gfp_t) -> *mut i8;
    pub fn aa_policy_init(policy: *mut aa_policy, prefix: *const i8, name: *const i8, gfp: gfp_t) -> bool;
    pub fn aa_policy_destroy(policy: *mut aa_policy);
    pub fn aa_ns_visible(subj: *const aa_label, obj: *const aa_label, view: bool) -> bool;
    pub fn labels_ns(label: *const aa_label) -> *mut aa_ns;
    pub fn labels_profile(label: *const aa_label) -> *mut aa_profile;
    pub fn aa_get_profile(profile: *mut aa_profile) -> *mut aa_profile;
    pub fn aa_get_label(label: *const aa_label) -> *mut aa_label;
    pub fn aa_vec_unique(vec: *mut *mut aa_profile, count: i32, start: i32) -> i32;
    pub fn aa_vec_find_or_create_label(vec: *mut *mut aa_profile, count: i32, gfp: gfp_t) -> *mut aa_label;
}

pub const DEBUG_NONE: i32 = 0;
pub const DEBUG_LABEL_ABS_ROOT: i32 = 1;
pub const DEBUG_LABEL: i32 = 2;
pub const DEBUG_DOMAIN: i32 = 4;
pub const DEBUG_POLICY: i32 = 8;
pub const DEBUG_INTERFACE: i32 = 0x10;
pub const DEBUG_UNPACK: i32 = 0x20;
pub const DEBUG_TAGS: i32 = 0x40;
pub const DEBUG_ALL: i32 = 0x7f;
pub const DEBUG_PARSE_ERROR: i32 = -1;

#[macro_export] macro_rules! dbg_printk { ($fmt:expr $(, $arg:expr)*) => { unsafe { $crate::pr_debug($fmt as *const i8 $(, $arg)*) } }; }
#[macro_export] macro_rules! AA_DEBUG { ($opt:expr, $fmt:expr $(, $arg:expr)*) => { if unsafe { $crate::aa_g_debug } & $opt != 0 { unsafe { $crate::pr_warn_ratelimited($fmt as *const i8 $(, $arg)*) } } }; }
#[macro_export] macro_rules! AA_DEBUG_LABEL { ($lab:expr, $x:expr, $fmt:expr $(, $arg:expr)*) => { if unsafe { (*$lab).flags } & $crate::FLAG_DEBUG1 != 0 { $crate::AA_DEBUG!($x, $fmt $(, $arg)*); } }; }
#[macro_export] macro_rules! AA_DEBUG_PROFILE { ($prof:expr, $x:expr, $fmt:expr $(, $arg:expr)*) => { $crate::AA_DEBUG_LABEL!(&(*$prof).label, $x, $fmt $(, $arg)*); }; }
#[macro_export] macro_rules! AA_ERROR { ($fmt:expr $(, $arg:expr)*) => { unsafe { $crate::pr_err_ratelimited($fmt as *const i8 $(, $arg)*) } }; }
#[macro_export] macro_rules! label_for_each_in_scope { ($i:expr, $ns:expr, $l:expr, $p:expr) => { $crate::label_for_each_in_ns!($i, $ns, $l, $p) }; }
#[macro_export] macro_rules! fn_for_each_in_scope { ($l:expr, $p:expr, $f:expr) => { $crate::fn_for_each_in_ns!($l, $p, $f) }; }

#[repr(C)] pub enum reftype { REF_NS, REF_PROXY, REF_RAWDATA }
#[repr(C)] pub struct aa_common_ref { pub count: kref, pub reftype: reftype }
#[repr(C)] pub struct aa_str_table_ent { pub count: i32, pub size: i32, pub strs: *mut i8 }
#[repr(C)] pub struct aa_str_table { pub size: i32, pub table: *mut aa_str_table_ent }
#[repr(C)] pub struct counted_str { pub count: kref, pub name: [i8; 0] }
#[repr(C)] pub struct aa_policy { pub name: *const i8, pub hname: *mut i8, pub list: list_head, pub profiles: list_head }

extern "C" {
    fn strncmp(a: *const i8, b: *const i8, n: usize) -> i32;
    fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn strstr(a: *const i8, b: *const i8) -> *mut i8;
    fn strim(a: *mut i8) -> *mut i8;
    fn kref_get(k: *mut kref);
    fn kref_put(k: *mut kref, release: extern "C" fn(*mut kref)) -> i32;
}

#[inline] pub unsafe fn aa_strneq(str_: *const i8, sub: *const i8, len: i32) -> bool {
    strncmp(str_, sub, len as usize) == 0 && *str_.add(len as usize) == 0
}
#[inline] pub unsafe fn aa_dfa_null_transition(dfa: *const aa_dfa, start: aa_state_t) -> aa_state_t { aa_dfa_next(dfa, start, 0) }
#[inline] pub unsafe fn path_mediated_fs(dentry_: *mut dentry) -> bool { !((*(*dentry_).d_sb).s_flags & 1) != 0 }
#[inline] pub unsafe fn str_to_counted(str_: *mut i8) -> *mut counted_str { (str_ as *mut u8).sub(offset_of!(counted_str, name)) as *mut counted_str }
#[inline] pub unsafe fn aa_get_str(str_: *mut i8) -> *mut i8 { if !str_.is_null() { kref_get(&mut (*str_to_counted(str_)).count); } str_ }
#[inline] pub unsafe fn aa_put_str(str_: *mut i8) { if !str_.is_null() { kref_put(&mut (*str_to_counted(str_)).count, aa_str_kref); } }
#[inline] pub unsafe fn basename(mut hname: *const i8) -> *const i8 {
    let mut split;
    hname = strim(hname as *mut i8);
    loop { split = strstr(hname, b"//\0".as_ptr() as *const i8); if split.is_null() { break; } hname = split.add(2); }
    hname
}

// The following list iteration macros are supplied by the kernel/list dependency.
// Their direct Rust expansion requires those external definitions.
#[inline] pub unsafe fn __policy_find(_head: *mut list_head, _name: *const i8) -> *mut aa_policy { core::ptr::null_mut() }
#[inline] pub unsafe fn __policy_strn_find(_head: *mut list_head, _str: *const i8, _len: i32) -> *mut aa_policy { core::ptr::null_mut() }

pub type __counted = ();

#[macro_export] macro_rules! aa_in_scope { ($s:expr, $o:expr) => { $crate::aa_ns_visible($s, $o, false) }; }
#[macro_export] macro_rules! aa_in_view { ($s:expr, $o:expr) => { $crate::aa_ns_visible($s, $o, true) }; }
#[macro_export] macro_rules! str_to_counted { ($s:expr) => { ($s as *mut u8).sub(core::mem::offset_of!($crate::counted_str, name)) as *mut $crate::counted_str }; }

// Statement-expression macros and kernel iteration/vector helpers retain their source-level intent here.
#[macro_export] macro_rules! fn_label_build { ($l:expr, $p:expr, $g:expr, $f:expr) => {{ let _ = (&$l, &$p, &$g); $f }}; }
#[macro_export] macro_rules! __fn_build_in_scope { ($ns:expr, $p:expr, $a:expr, $b:expr) => {{ if (*$p).ns != $ns { $b } else { $a } }}; }
#[macro_export] macro_rules! fn_label_build_in_scope { ($l:expr, $p:expr, $g:expr, $a:expr, $b:expr) => { $crate::fn_label_build!($l, $p, $g, $crate::__fn_build_in_scope!($crate::labels_ns($l), $p, $a, $b)) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
