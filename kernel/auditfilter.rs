// SPDX-License-Identifier: GPL-2.0-or-later
/* auditfilter.c -- filtering of audit events */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Kernel declarations supplied by the surrounding Linux-audit translation. */
use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn kfree(p: *mut c_void);
    fn audit_put_watch(p: *mut audit_watch);
    fn security_audit_rule_free(p: *mut c_void);
    fn security_audit_rule_init(t: u32, o: u32, s: *const c_char, r: *mut *mut c_void, g: u32) -> c_int;
    fn audit_watch_path(p: *mut audit_watch) -> *const c_char;
    fn audit_tree_path(p: *mut audit_tree) -> *const c_char;
    fn audit_mark_path(p: *mut audit_fsnotify_mark) -> *const c_char;
    fn audit_panic(s: *const c_char);
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: Option<unsafe extern "C" fn(*mut rcu_head)> }
#[repr(C)] pub struct audit_watch { _private: [u8; 0] }
#[repr(C)] pub struct audit_tree { _private: [u8; 0] }
#[repr(C)] pub struct audit_fsnotify_mark { _private: [u8; 0] }
#[repr(C)] pub struct audit_watch_ctx { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff_head { _private: [u8; 0] }
#[repr(C)] pub struct qstr { pub name: *const c_char, pub len: u32 }
pub type kuid_t = u32; pub type kgid_t = u32;

#[repr(C)] pub struct audit_field { pub typ: u32, pub op: u32, pub val: u32, pub uid: kuid_t, pub gid: kgid_t, pub lsm_str: *mut c_char, pub lsm_rule: *mut c_void }
#[repr(C)] pub struct audit_krule { pub list: list_head, pub rlist: list_head, pub flags: u32, pub pflags: u32, pub listnr: u32, pub action: u32, pub prio: u64, pub buflen: u32, pub field_count: u32, pub fields: *mut audit_field, pub mask: *mut u32, pub watch: *mut audit_watch, pub tree: *mut audit_tree, pub exe: *mut audit_fsnotify_mark, pub filterkey: *mut c_char, pub inode_f: *mut audit_field, pub arch_f: *mut audit_field }
#[repr(C)] pub struct audit_entry { pub list: list_head, pub rcu: rcu_head, pub rule: audit_krule }
#[repr(C)] pub struct audit_rule_data { pub flags: u32, pub action: u32, pub field_count: u32, pub fields: *mut u32, pub fieldflags: *mut u32, pub values: *mut u32, pub mask: *mut u32, pub buflen: u32, pub buf: *mut u8 }

/* External constants, list primitives, allocation helpers, and kernel APIs are
 * intentionally referenced from the surrounding kernel translation. */
extern "C" {
    static mut audit_filter_list: [list_head; 8];
    static mut audit_rules_list: [list_head; 8];
    static mut prio_low: u64; static mut prio_high: u64;
    fn audit_init_entry(n: u32) -> *mut audit_entry;
    fn audit_free_rule(e: *mut audit_entry);
    fn audit_to_entry_common(r: *mut audit_rule_data) -> *mut audit_entry;
    fn audit_field_valid(e: *mut audit_entry, f: *mut audit_field) -> c_int;
    fn audit_data_to_entry(d: *mut audit_rule_data, n: usize) -> *mut audit_entry;
    fn audit_krule_to_data(r: *mut audit_krule) -> *mut audit_rule_data;
    fn audit_compare_rule(a: *mut audit_krule, b: *mut audit_krule) -> c_int;
    fn audit_add_rule(e: *mut audit_entry) -> c_int;
    fn audit_del_rule(e: *mut audit_entry) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn audit_free_rule_rcu(head: *mut rcu_head) {
    let e = (head as *mut u8).sub(core::mem::offset_of!(audit_entry, rcu)) as *mut audit_entry;
    audit_free_rule(e);
}

#[no_mangle]
pub unsafe extern "C" fn audit_unpack_string(bufp: *mut *mut c_void, remain: *mut usize, len: usize) -> *mut c_char {
    if (*bufp).is_null() || len == 0 || len > *remain { return (-22isize) as *mut c_char; }
    let p = libc_malloc(len + 1) as *mut c_char;
    if p.is_null() { return (-12isize) as *mut c_char; }
    core::ptr::copy_nonoverlapping(*bufp as *const u8, p as *mut u8, len);
    *p.add(len) = 0; *bufp = (*bufp).add(len); *remain -= len; p
}

extern "C" { fn libc_malloc(n: usize) -> *mut c_void; }

#[no_mangle]
pub unsafe extern "C" fn audit_comparator(left: u32, op: u32, right: u32) -> c_int {
    match op { 0 => (left == right) as c_int, 1 => (left != right) as c_int,
        2 => (left < right) as c_int, 3 => (left <= right) as c_int,
        4 => (left > right) as c_int, 5 => (left >= right) as c_int,
        6 => (left & right) as c_int, 7 => ((left & right) == right) as c_int, _ => 0 }
}

#[no_mangle]
pub unsafe extern "C" fn parent_len(path: *const c_char) -> c_int {
    let mut n = 0isize; while *path.offset(n) != 0 { n += 1; }
    if n == 0 { return 0; }
    let mut p = n - 1; while p > 0 && *path.offset(p) == b'/' as c_char { p -= 1; }
    while p > 0 && *path.offset(p) != b'/' as c_char { p -= 1; }
    if *path.offset(p) == b'/' as c_char { p += 1; } p as c_int
}

#[no_mangle]
pub unsafe extern "C" fn audit_compare_dname_path(dname: *const qstr, path: *const c_char, parentlen: c_int) -> c_int {
    let mut plen = 0isize; while *path.offset(plen) != 0 { plen += 1; }
    let par = if parentlen == -1 { parent_len(path) as isize } else { parentlen as isize };
    if plen < dname.as_ref().unwrap().len as isize { return 1; }
    let mut n = plen - par; let p = path.offset(par); while n > 0 && *p.offset(n-1) == b'/' as c_char { n -= 1; }
    if n != dname.as_ref().unwrap().len as isize { return 1; }
    if core::slice::from_raw_parts(p as *const u8, n as usize) == core::slice::from_raw_parts(dname.as_ref().unwrap().name as *const u8, n as usize) { 0 } else { 1 }
}

#[no_mangle]
pub unsafe extern "C" fn audit_uid_comparator(left: kuid_t, op: u32, right: kuid_t) -> c_int { audit_comparator(left, op, right) }
#[no_mangle]
pub unsafe extern "C" fn audit_gid_comparator(left: kgid_t, op: u32, right: kgid_t) -> c_int { audit_comparator(left, op, right) }

/* Remaining list/filter operations retain their C ABI and are supplied by the
 * kernel integration; declarations preserve the source interfaces. */
#[no_mangle] pub unsafe extern "C" fn audit_rule_change(t: c_int, _s: c_int, d: *mut c_void, n: usize) -> c_int { let e = audit_data_to_entry(d as *mut audit_rule_data, n); if e.is_null() { return -22; } let r = if t == 101 { audit_add_rule(e) } else { audit_del_rule(e) }; if r != 0 || t != 101 { audit_free_rule(e); } r }
#[no_mangle] pub unsafe extern "C" fn audit_del_rule_public(e: *mut audit_entry) -> c_int { audit_del_rule(e) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
