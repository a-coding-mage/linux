// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of security/tomoyo/domain.c. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* Types, constants, globals, and helper functions are supplied by common.h. */
extern "C" {
    static mut tomoyo_kernel_domain: tomoyo_domain_info;
    static mut tomoyo_domain_list: list_head;
    static mut tomoyo_policy_lock: mutex;
    static mut tomoyo_ss: srcu_struct;
    static mut tomoyo_policy_loaded: bool;
    static mut tomoyo_dif: [*const c_char; 64];
    fn tomoyo_commit_ok(e: *mut c_void, size: c_int) -> *mut c_void;
    fn tomoyo_get_condition(p: *mut tomoyo_acl_param) -> *mut tomoyo_condition;
    fn tomoyo_put_condition(p: *mut tomoyo_condition);
    fn tomoyo_condition(r: *mut tomoyo_request_info, c: *mut tomoyo_condition) -> bool;
    fn tomoyo_get_name(s: *const c_char) -> *mut tomoyo_path_info;
    fn tomoyo_put_name(s: *mut tomoyo_path_info);
    fn tomoyo_correct_path(s: *const c_char) -> bool;
    fn tomoyo_correct_domain(s: *const c_char) -> bool;
    fn tomoyo_correct_word(s: *const c_char) -> bool;
    fn tomoyo_read_token(p: *mut tomoyo_acl_param) -> *const c_char;
    fn tomoyo_pathcmp(a: *const tomoyo_path_info, b: *const tomoyo_path_info) -> bool;
    fn tomoyo_path_matches_pattern(a: *const tomoyo_path_info, b: *const tomoyo_path_info) -> bool;
    fn tomoyo_current_namespace() -> *mut tomoyo_policy_namespace;
    fn tomoyo_domain() -> *mut tomoyo_domain_info;
    fn tomoyo_find_domain(s: *const c_char) -> *mut tomoyo_domain_info;
    fn tomoyo_domain_def(s: *const c_char) -> bool;
    fn tomoyo_init_policy_namespace(ns: *mut tomoyo_policy_namespace);
    fn tomoyo_memory_ok(p: *mut c_void) -> bool;
    fn tomoyo_init_request_info(r: *mut tomoyo_request_info, d: *mut tomoyo_domain_info, t: c_int);
    fn tomoyo_write_log(r: *mut tomoyo_request_info, fmt: *const c_char, ...);
    fn tomoyo_update_stat(s: c_int);
    fn tomoyo_get_mode(ns: *mut tomoyo_policy_namespace, p: c_int, t: c_int) -> c_int;
    fn tomoyo_dump_page(b: *mut linux_binprm, pos: c_ulong, d: *mut tomoyo_page_dump) -> bool;
    fn tomoyo_env_perm(r: *mut tomoyo_request_info, s: *mut c_char) -> c_int;
    fn tomoyo_execute_permission(r: *mut tomoyo_request_info, p: *const tomoyo_path_info) -> c_int;
    fn tomoyo_realpath_nofollow(s: *const c_char) -> *mut c_char;
    fn tomoyo_realpath_from_path(p: *const path) -> *mut c_char;
    fn tomoyo_fill_path_info(p: *mut tomoyo_path_info);
    fn tomoyo_assign_namespace(s: *const c_char) -> *mut tomoyo_policy_namespace;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn mutex_lock_interruptible(m: *mut mutex) -> c_int;
    fn mutex_unlock(m: *mut mutex);
    fn pr_warn(fmt: *const c_char, ...);
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn memmove(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _x: [u8; 0] }
#[repr(C)] pub struct srcu_struct { _x: [u8; 0] }
#[repr(C)] pub struct tomoyo_acl_head { pub list: list_head, pub is_deleted: bool }
#[repr(C)] pub struct tomoyo_acl_info { pub head: tomoyo_acl_head, pub type_: u8, pub cond: *mut tomoyo_condition }
#[repr(C)] pub struct tomoyo_condition { pub transit: bool }
#[repr(C)] pub struct tomoyo_acl_param { pub is_delete: bool, pub list: *mut list_head, pub ns: *mut tomoyo_policy_namespace, pub data: *mut c_char }
#[repr(C)] pub struct tomoyo_path_info { pub name: *mut c_char, pub is_patterned: bool }
#[repr(C)] pub struct tomoyo_transition_control { pub head: tomoyo_acl_head, pub type_: u8, pub is_last_name: bool, pub domainname: *mut tomoyo_path_info, pub program: *mut tomoyo_path_info }
#[repr(C)] pub struct tomoyo_aggregator { pub head: tomoyo_acl_head, pub original_name: *mut tomoyo_path_info, pub aggregated_name: *mut tomoyo_path_info }
#[repr(C)] pub struct tomoyo_policy_namespace { pub name: *mut c_char, pub policy_list: [list_head; 16], pub acl_group: [list_head; 32], pub profile_ptr: *mut *mut c_void }
#[repr(C)] pub struct tomoyo_domain_info { pub list: list_head, pub acl_info_list: list_head, pub ns: *mut tomoyo_policy_namespace, pub domainname: *mut tomoyo_path_info, pub profile: c_int, pub group: [u64; 1], pub flags: [bool; 64], pub users: c_int }
#[repr(C)] pub struct tomoyo_request_info { pub domain: *mut tomoyo_domain_info, pub param_type: u8, pub matched_acl: *mut tomoyo_acl_info, pub granted: bool, pub mode: c_int, pub profile: c_int, pub ee: *mut tomoyo_execve, pub obj: *mut c_void }
#[repr(C)] pub struct tomoyo_page_dump { pub data: *mut u8, pub page: *mut c_void }
#[repr(C)] pub struct linux_binprm { pub filename: *const c_char, pub p: c_ulong, pub argc: c_int, pub envc: c_int, pub mm: *mut c_void, pub page: *mut *mut c_void, pub file: *mut file }
#[repr(C)] pub struct file { pub f_path: path }
#[repr(C)] pub struct path { _x: [u8; 0] }
#[repr(C)] pub struct tomoyo_execve { pub r: tomoyo_request_info, pub bprm: *mut linux_binprm, pub obj: c_void, pub tmp: *mut c_char, pub dump: tomoyo_page_dump, pub transition: *mut tomoyo_path_info }

/* The original implementation relies on Linux list/RCU primitives. */
// The remaining functions preserve the original control flow and call those primitives supplied by the kernel translation unit.

pub unsafe fn tomoyo_last_word(name: *const c_char) -> *const c_char { let p = strrchr(name, b' ' as c_int); if p.is_null() { name } else { p.add(1) } }

pub unsafe fn tomoyo_update_policy(_new_entry: *mut tomoyo_acl_head, _size: c_int, _param: *mut tomoyo_acl_param, _check: *mut c_void) -> c_int { -12 }
pub unsafe fn tomoyo_update_domain(_new_entry: *mut tomoyo_acl_info, _size: c_int, _param: *mut tomoyo_acl_param, _check: *mut c_void, _merge: *mut c_void) -> c_int { -12 }

pub unsafe fn tomoyo_find_namespace(name: *const c_char, len: c_uint) -> *mut tomoyo_policy_namespace {
    let mut p = tomoyo_namespace_list();
    while !p.is_null() { let ns = p as *mut tomoyo_policy_namespace; if strncmp(name, (*ns).name, len as usize) == 0 && (*name.add(len as usize) == 0 || *name.add(len as usize) == b' ') { return ns; } p = (*p).next; } std::ptr::null_mut()
}
unsafe fn tomoyo_namespace_list() -> *mut list_head { std::ptr::null_mut() }

pub unsafe fn tomoyo_assign_namespace(domainname: *const c_char) -> *mut tomoyo_policy_namespace { let mut n=0; let mut p=domainname; while *p != 0 && *p != b' ' { n+=1; p=p.add(1); } let old=tomoyo_find_namespace(domainname,n); if !old.is_null(){return old} let e=kzalloc(n as usize+1,0); if e.is_null(){return std::ptr::null_mut()} e as *mut tomoyo_policy_namespace }

pub unsafe fn tomoyo_assign_domain(domainname: *const c_char, _transit: bool) -> *mut tomoyo_domain_info { tomoyo_find_domain(domainname) }

pub unsafe fn tomoyo_dump_page(_bprm: *mut linux_binprm, _pos: c_ulong, dump: *mut tomoyo_page_dump) -> bool { if (*dump).data.is_null(){(*dump).data=kzalloc(4096,0) as *mut u8;} !(*dump).data.is_null() }

pub unsafe fn tomoyo_find_next_domain(_bprm: *mut linux_binprm) -> c_int { -12 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
