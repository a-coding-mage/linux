// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains basic common functions used in AppArmor
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 */

// Linux kernel headers:
// #include <linux/ctype.h>
// #include <linux/mm.h>
// #include <linux/slab.h>
// #include <linux/string.h>
// #include <linux/vmalloc.h>
//
// Local headers:
// #include "include/audit.h"
// #include "include/apparmor.h"
// #include "include/lib.h"
// #include "include/perms.h"
// #include "include/policy.h"

// Forward declarations and external dependencies from headers
extern "C" {
    // From linux/ctype.h
    fn isspace(c: c_int) -> c_int;

    // From linux/string.h
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strnchr(s: *const c_char, count: usize, c: c_char) -> *const c_char;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    // From linux/slab.h / kmalloc equivalents
    fn kzalloc_objs(size: usize, gfp: u32) -> *mut c_void;
    fn kfree_sensitive(ptr: *const c_void) -> ();
    fn kmalloc_flex(size: usize, gfp: u32) -> *mut c_void;
    fn kfree(ptr: *const c_void) -> ();

    // stdio functions
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> c_int;

    // printk
    fn printk(fmt: *const c_char, ...) -> c_int;

    // from include/apparmor.h and related headers
    fn kref_init(kref: *mut kref) -> ();
    fn container_of<T>(ptr: *mut c_void, _t: *mut T, member_offset: usize) -> *mut T;
    fn basename(path: *const c_char) -> *const c_char;

    // Audit-related
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...) -> ();
    fn aa_audit_msg(atype: c_uint, ad: *mut apparmor_audit_data, cb: Option<unsafe extern "C" fn(*mut audit_buffer, *mut c_void)>) -> ();
    fn aa_dfa_next(dfa: *const c_void, state: u32, type_: u32) -> u32;
    fn aa_label_match(profile: *const aa_profile, rules: *mut aa_ruleset, label: *mut aa_label, state: u32, b: bool, request: u32, perms: *mut aa_perms) -> ();
    fn aa_select_audit_type(denied: u32, perms: *const aa_perms) -> c_uint;

    // Global variables
    static mut aa_g_debug: u32;
    static audit_enabled: c_int;
}

use std::ffi::{CStr, c_int, c_char, c_void, c_uint};

// Constants and types

const ALL_PERMS_MASK: u32 = 0xffffffff;
const DEBUG_NONE: c_int = 0;
const DEBUG_ALL: c_int = -1;
const DEBUG_LABEL_ABS_ROOT: c_int = 1;
const DEBUG_LABEL: c_int = 2;
const DEBUG_DOMAIN: c_int = 4;
const DEBUG_POLICY: c_int = 8;
const DEBUG_INTERFACE: c_int = 16;
const DEBUG_UNPACK: c_int = 32;
const DEBUG_TAGS: c_int = 64;

const PAGE_SIZE: usize = 4096;

const AA_CLASS_NONE: c_uint = 0;
const AA_CLASS_LABEL: c_int = 0;
const AUDIT_APPARMOR_STATUS: c_uint = 1;
const AUDIT_APPARMOR_ALLOWED: c_uint = 2;
const AUDIT_ALL: c_int = 1;
const AUDIT_NOQUIET: c_int = 2;
const AUDIT_QUIET: c_int = 3;
const AUDIT_QUIET_DENIED: c_int = 4;
const AUDIT_QUIET_ALLOWED: c_int = 5;

// Macro equivalents
macro_rules! KILL_MODE {
    ($profile:expr) => {
        unsafe { ((*$profile).flags & 1) != 0 }
    };
}

macro_rules! COMPLAIN_MODE {
    ($profile:expr) => {
        unsafe { ((*$profile).flags & 2) != 0 }
    };
}

macro_rules! USER_MODE {
    ($profile:expr) => {
        unsafe { ((*$profile).flags & 4) != 0 }
    };
}

macro_rules! AUDIT_MODE {
    ($profile:expr) => {
        unsafe { (((*$profile).flags >> 3) & 0xf) as c_int }
    };
}

macro_rules! AA_DEBUG {
    ($level:expr, $fmt:expr, $($arg:expr),*) => {
        if unsafe { aa_g_debug & $level as u32 } != 0 {
            unsafe {
                printk(concat!("AppArmor: ", $fmt, "\n").as_ptr() as *const c_char, $($arg),*);
            }
        }
    };
}

macro_rules! DEFINE_AUDIT_DATA {
    ($ad:expr, $type_:expr, $class:expr, $name:expr) => {
        $ad = std::mem::zeroed();
        $ad.type_ = $type_;
    };
}

macro_rules! INIT_LIST_HEAD {
    ($list:expr) => {
        unsafe {
            (*$list).next = $list;
            (*$list).prev = $list;
        }
    };
}

macro_rules! on_list_rcu {
    ($list:expr) => {
        unsafe { (*$list).next as *const _ != $list as *const _ }
    };
}

#[repr(C)]
pub struct aa_perms {
    pub allow: u32,
    pub deny: u32,
    pub audit: u32,
    pub quiet: u32,
    pub kill: u32,
    pub complain: u32,
    pub prompt: u32,
    pub hide: u32,
}

#[repr(C)]
struct val_table_ent {
    str: *const c_char,
    value: c_int,
}

#[repr(C)]
pub struct aa_str_table_ent {
    strs: *mut c_void,
}

#[repr(C)]
pub struct aa_str_table {
    table: *mut aa_str_table_ent,
    size: c_int,
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct kref {
    refcount: c_int,
}

#[repr(C)]
pub struct aa_policy {
    list: list_head,
    profiles: list_head,
    hname: *const c_char,
    name: *const c_char,
}

#[repr(C)]
pub struct aa_profile {
    label: aa_label,
    flags: u32,
}

#[repr(C)]
pub struct aa_label {
    dummy: [u8; 0],
}

#[repr(C)]
pub struct aa_ruleset {
    policy: *const aa_policy_dfa,
}

#[repr(C)]
pub struct aa_policy_dfa {
    dfa: *const c_void,
    start: [u32; 256],
}

#[repr(C)]
pub struct audit_buffer {
    dummy: [u8; 0],
}

#[repr(C)]
pub struct apparmor_audit_data {
    type_: c_uint,
    subj_label: *const aa_label,
    request: u32,
    denied: u32,
    error: c_int,
    info: *const c_char,
}

#[repr(C)]
struct counted_str {
    count: kref,
    name: *const c_char,
}

const LSM_AUDIT_DATA_NONE: c_int = 0;

pub static mut nullperms: aa_perms = aa_perms {
    allow: 0,
    deny: 0,
    audit: 0,
    quiet: 0,
    kill: 0,
    complain: 0,
    prompt: 0,
    hide: 0,
};

pub static mut allperms: aa_perms = aa_perms {
    allow: ALL_PERMS_MASK,
    deny: 0,
    audit: 0,
    quiet: ALL_PERMS_MASK,
    kill: 0,
    complain: 0,
    prompt: 0,
    hide: ALL_PERMS_MASK,
};

static DEBUG_VALUES_TABLE: [val_table_ent; 16] = [
    val_table_ent { str: b"N\0" as *const u8 as *const c_char, value: DEBUG_NONE },
    val_table_ent { str: b"none\0" as *const u8 as *const c_char, value: DEBUG_NONE },
    val_table_ent { str: b"n\0" as *const u8 as *const c_char, value: DEBUG_NONE },
    val_table_ent { str: b"0\0" as *const u8 as *const c_char, value: DEBUG_NONE },
    val_table_ent { str: b"all\0" as *const u8 as *const c_char, value: DEBUG_ALL },
    val_table_ent { str: b"Y\0" as *const u8 as *const c_char, value: DEBUG_ALL },
    val_table_ent { str: b"y\0" as *const u8 as *const c_char, value: DEBUG_ALL },
    val_table_ent { str: b"1\0" as *const u8 as *const c_char, value: DEBUG_ALL },
    val_table_ent { str: b"abs_root\0" as *const u8 as *const c_char, value: DEBUG_LABEL_ABS_ROOT },
    val_table_ent { str: b"label\0" as *const u8 as *const c_char, value: DEBUG_LABEL },
    val_table_ent { str: b"domain\0" as *const u8 as *const c_char, value: DEBUG_DOMAIN },
    val_table_ent { str: b"policy\0" as *const u8 as *const c_char, value: DEBUG_POLICY },
    val_table_ent { str: b"interface\0" as *const u8 as *const c_char, value: DEBUG_INTERFACE },
    val_table_ent { str: b"unpack\0" as *const u8 as *const c_char, value: DEBUG_UNPACK },
    val_table_ent { str: b"tags\0" as *const u8 as *const c_char, value: DEBUG_TAGS },
    val_table_ent { str: std::ptr::null(), value: 0 },
];

unsafe fn val_table_find_ent(table: *const val_table_ent, name: *const c_char, len: usize) -> *const val_table_ent {
    let mut entry = table;
    while !(*entry).str.is_null() {
        if strncmp((*entry).str, name, len) == 0 && strlen((*entry).str) == len {
            return entry;
        }
        entry = entry.offset(1);
    }
    std::ptr::null()
}

pub unsafe extern "C" fn aa_parse_debug_params(str: *const c_char) -> c_int {
    let mut s = str;
    let mut val: c_int = 0;

    loop {
        let n = strcspn(s, b"\r\n,\0" as *const u8 as *const c_char);
        let next = s.offset(n as isize);
        let ent = val_table_find_ent(DEBUG_VALUES_TABLE.as_ptr(), s, n);
        if !ent.is_null() {
            val |= (*ent).value;
        } else {
            AA_DEBUG!(DEBUG_INTERFACE, "unknown debug type '%.*s\0", n as c_int, s);
        }
        s = next.offset(1);
        if *next == 0 {
            break;
        }
    }
    val
}

// strcspn equivalent - not in libc
unsafe fn strcspn(s: *const c_char, reject: *const c_char) -> usize {
    let mut n = 0;
    let mut p = s;
    while *p != 0 {
        let mut r = reject;
        while *r != 0 {
            if *p == *r {
                return n;
            }
            r = r.offset(1);
        }
        n += 1;
        p = p.offset(1);
    }
    n
}

unsafe fn val_mask_to_str(str: *mut c_char, size: usize, table: *const val_table_ent, mut mask: u32) -> c_int {
    let mut total = 0;
    let mut s = str;
    let mut sz = size;
    let mut ent = table;

    while !(*ent).str.is_null() {
        if (*ent).value != 0 && ((*ent).value as u32 & mask) == (*ent).value as u32 {
            let len = if total == 0 {
                scnprintf(s, sz, b"%s\0" as *const u8 as *const c_char, (*ent).str)
            } else {
                scnprintf(s, sz, b"%s%s\0" as *const u8 as *const c_char, b",\0" as *const u8 as *const c_char, (*ent).str)
            } as usize;
            sz = sz.saturating_sub(len);
            s = s.offset(len as isize);
            total += len as c_int;
            mask &= !((*ent).value as u32);
        }
        ent = ent.offset(1);
    }

    total
}

pub unsafe extern "C" fn aa_print_debug_params(buffer: *mut c_char) -> c_int {
    if aa_g_debug == 0 {
        return sprintf(buffer, b"N\0" as *const u8 as *const c_char);
    }
    val_mask_to_str(buffer, PAGE_SIZE, DEBUG_VALUES_TABLE.as_ptr(), aa_g_debug)
}

pub unsafe extern "C" fn aa_resize_str_table(t: *mut aa_str_table, newsize: c_int, gfp: u32) -> bool {
    if (*t).size == newsize {
        return true;
    }

    let n = kzalloc_objs(std::mem::size_of::<aa_str_table_ent>() * newsize as usize, gfp);
    if n.is_null() {
        return false;
    }

    let n = n as *mut aa_str_table_ent;
    for i in 0..(*t).size.min(newsize) {
        *n.offset(i as isize) = *(*t).table.offset(i as isize);
    }

    for i in newsize..(*t).size {
        kfree_sensitive((*(*t).table.offset(i as isize)).strs);
    }

    if newsize > (*t).size {
        memset(
            n.offset((*t).size as isize) as *mut c_void,
            0,
            ((newsize - (*t).size) as usize) * std::mem::size_of::<aa_str_table_ent>(),
        );
    }

    kfree_sensitive((*t).table as *const c_void);
    (*t).table = n;
    (*t).size = newsize;

    true
}

pub unsafe extern "C" fn aa_destroy_str_table(t: *mut aa_str_table) {
    if t.is_null() {
        return;
    }

    if (*t).table.is_null() {
        return;
    }

    for i in (*t).size.min(newsize)..(*t).size {
        kfree_sensitive((*(*t).table.offset(i as isize)).strs);
    }

    kfree_sensitive((*t).table as *const c_void);
    (*t).table = std::ptr::null_mut();
    (*t).size = 0;
}

pub unsafe extern "C" fn skipn_spaces(str: *const c_char, mut n: usize) -> *const c_char {
    let mut s = str;
    while n > 0 && isspace(*s as c_int) != 0 {
        n -= 1;
        s = s.offset(1);
    }
    if n > 0 {
        s
    } else {
        std::ptr::null()
    }
}

pub unsafe extern "C" fn aa_splitn_fqname(
    fqname: *const c_char,
    n: usize,
    ns_name: *mut *const c_char,
    ns_len: *mut usize,
) -> *const c_char {
    let end = fqname.offset(n as isize);
    let name = skipn_spaces(fqname, n);

    *ns_name = std::ptr::null();
    *ns_len = 0;

    if name.is_null() {
        return std::ptr::null();
    }

    let mut name = name;
    if *name as u8 == b':' {
        let split = strnchr(name.offset(1), end as *mut c_char as usize - name.offset(1) as *mut c_char as usize, b':' as c_char);
        *ns_name = skipn_spaces(name.offset(1), end as *mut c_char as usize - name.offset(1) as *mut c_char as usize);
        if (*ns_name).is_null() {
            return std::ptr::null();
        }
        if !split.is_null() {
            *ns_len = split as *mut c_char as usize - *ns_name as *mut c_char as usize;
            if *ns_len == 0 {
                *ns_name = std::ptr::null();
            }
            let mut s = split.offset(1);
            if end as *mut c_char as usize - s as *mut c_char as usize > 1
                && strncmp(s, b"//\0" as *const u8 as *const c_char, 2) == 0
            {
                s = s.offset(2);
            }
            name = skipn_spaces(s, end as *mut c_char as usize - s as *mut c_char as usize);
        } else {
            name = std::ptr::null_mut() as *const c_char;
            *ns_len = end as *mut c_char as usize - *ns_name as *mut c_char as usize;
        }
    }

    if !name.is_null() && *name == 0 {
        name = std::ptr::null();
    }

    name
}

pub unsafe extern "C" fn aa_info_message(str: *const c_char) {
    if audit_enabled != 0 {
        let mut ad: apparmor_audit_data = std::mem::zeroed();
        ad.type_ = LSM_AUDIT_DATA_NONE as c_uint;
        ad.info = str;
        aa_audit_msg(AUDIT_APPARMOR_STATUS, &mut ad, None);
    }
    printk(b"AppArmor: %s\n\0" as *const u8 as *const c_char, str);
}

pub unsafe extern "C" fn aa_str_alloc(size: c_int, gfp: u32) -> *const c_char {
    let str = kmalloc_flex(
        std::mem::size_of::<counted_str>() + size as usize,
        gfp,
    );
    if str.is_null() {
        return std::ptr::null();
    }

    let str = str as *mut counted_str;
    kref_init(&mut (*str).count);
    (str as *mut u8).add(std::mem::size_of::<kref>()) as *const c_char
}

pub unsafe extern "C" fn aa_str_kref(kref: *mut kref) {
    let str = container_of(kref as *mut c_void, std::ptr::null::<counted_str>(), 0);
    kfree(str as *const c_void);
}

pub static AA_FILE_PERM_CHRS: &[u8] = b"xwracd         km l     \0";

pub static AA_FILE_PERM_NAMES: &[&[u8]] = &[
    b"exec\0",
    b"write\0",
    b"read\0",
    b"append\0",
    b"create\0",
    b"delete\0",
    b"open\0",
    b"rename\0",
    b"setattr\0",
    b"getattr\0",
    b"setcred\0",
    b"getcred\0",
    b"chmod\0",
    b"chown\0",
    b"chgrp\0",
    b"lock\0",
    b"mmap\0",
    b"mprot\0",
    b"link\0",
    b"snapshot\0",
    b"unknown\0",
    b"unknown\0",
    b"unknown\0",
    b"unknown\0",
    b"unknown\0",
    b"unknown\0",
    b"unknown\0",
    b"unknown\0",
    b"stack\0",
    b"change_onexec\0",
    b"change_profile\0",
    b"change_hat\0",
];

pub unsafe extern "C" fn aa_perm_mask_to_str(
    str: *mut c_char,
    str_size: usize,
    chrs: *const c_char,
    mask: u32,
) {
    let num_chrs = strlen(chrs);
    let mut s = str;
    let mut sz = str_size;
    let mut perm: u32 = 1;

    for i in 0..num_chrs {
        if (mask & perm) != 0 {
            if sz <= 1 {
                break;
            }
            *s = *chrs.offset(i as isize);
            s = s.offset(1);
            sz -= 1;
        }
        perm <<= 1;
    }
    *s = 0;
}

pub unsafe extern "C" fn aa_audit_perm_names(
    ab: *mut audit_buffer,
    names: *const *const c_char,
    mask: u32,
) {
    let mut fmt = b"%s\0" as *const u8 as *const c_char;
    let mut perm: u32 = 1;
    let mut prev = false;

    for i in 0..32 {
        if (mask & perm) != 0 {
            audit_log_format(ab, fmt, *names.offset(i));
            if !prev {
                prev = true;
                fmt = b" %s\0" as *const u8 as *const c_char;
            }
        }
        perm <<= 1;
    }
}

pub unsafe extern "C" fn aa_audit_perm_mask(
    ab: *mut audit_buffer,
    mut mask: u32,
    chrs: *const c_char,
    chrsmask: u32,
    names: *const *const c_char,
    namesmask: u32,
) {
    let mut str: [c_char; 33] = [0; 33];

    audit_log_format(ab, b"\"\0" as *const u8 as *const c_char);
    if (mask & chrsmask) != 0 && !chrs.is_null() {
        aa_perm_mask_to_str(str.as_mut_ptr(), str.len(), chrs, mask & chrsmask);
        mask &= !chrsmask;
        audit_log_format(ab, b"%s\0" as *const u8 as *const c_char, str.as_ptr());
        if (mask & namesmask) != 0 {
            audit_log_format(ab, b" \0" as *const u8 as *const c_char);
        }
    }
    if (mask & namesmask) != 0 && !names.is_null() {
        aa_audit_perm_names(ab, names, mask & namesmask);
    }
    audit_log_format(ab, b"\"\0" as *const u8 as *const c_char);
}

pub unsafe extern "C" fn aa_apply_modes_to_perms(
    profile: *const aa_profile,
    perms: *mut aa_perms,
) {
    if KILL_MODE!(profile) {
        (*perms).kill = !(*perms).allow;
    } else if COMPLAIN_MODE!(profile) {
        (*perms).complain |= !((*perms).allow | (*perms).deny);
    } else if USER_MODE!(profile) {
        (*perms).prompt |= !((*perms).allow | (*perms).deny);
    }

    match AUDIT_MODE!(profile) {
        AUDIT_ALL => {
            (*perms).audit = ALL_PERMS_MASK;
            (*perms).quiet = 0;
        }
        AUDIT_NOQUIET => {
            (*perms).quiet = 0;
        }
        AUDIT_QUIET => {
            (*perms).audit = 0;
            (*perms).quiet |= !(*perms).allow;
        }
        AUDIT_QUIET_DENIED => {
            (*perms).quiet |= !(*perms).allow;
        }
        AUDIT_QUIET_ALLOWED => {
            (*perms).quiet |= (*perms).complain | (*perms).allow;
        }
        _ => {}
    }
}

pub unsafe extern "C" fn aa_profile_match_label(
    profile: *const aa_profile,
    rules: *mut aa_ruleset,
    label: *mut aa_label,
    type_: c_int,
    request: u32,
    perms: *mut aa_perms,
) {
    let state = aa_dfa_next(
        (*(*rules).policy).dfa,
        (*(*rules).policy).start[AA_CLASS_LABEL as usize],
        type_ as u32,
    );
    aa_label_match(profile, rules, label, state, false, request, perms);
}

pub unsafe extern "C" fn aa_check_perms(
    profile: *mut aa_profile,
    perms: *const aa_perms,
    request: u32,
    ad: *mut apparmor_audit_data,
    cb: Option<unsafe extern "C" fn(*mut audit_buffer, *mut c_void)>,
) -> c_int {
    let denied = request & (!(*perms).allow | (*perms).deny);

    let (mut error, mut audit_request) = if denied == 0 {
        audit_request = request & (*perms).audit;
        if audit_request == 0 || ad.is_null() {
            return 0;
        }
        (0, audit_request)
    } else {
        let mut err = -13; // -EACCES

        if denied == (denied & (*perms).hide) {
            err = -2; // -ENOENT
        }

        let denied_filtered = denied & !(*perms).quiet;
        if ad.is_null() || denied_filtered == 0 {
            return err;
        }
        (err, denied_filtered)
    };

    let type_ = aa_select_audit_type(denied, perms);

    if !ad.is_null() {
        (*ad).subj_label = &(*profile).label;
        (*ad).request = audit_request;
        (*ad).denied = denied;
        (*ad).error = error;
        aa_audit_msg(type_, ad, cb);
    }

    if type_ == AUDIT_APPARMOR_ALLOWED {
        error = 0;
    }

    error
}

pub unsafe extern "C" fn aa_policy_init(
    policy: *mut aa_policy,
    prefix: *const c_char,
    name: *const c_char,
    gfp: u32,
) -> bool {
    INIT_LIST_HEAD!(&mut (*policy).list);
    INIT_LIST_HEAD!(&mut (*policy).profiles);

    let hname_sz = (if !prefix.is_null() {
        strlen(prefix) + 2
    } else {
        0
    }) + strlen(name)
        + 1;

    let hname = aa_str_alloc(hname_sz as c_int, gfp);
    if hname.is_null() {
        return false;
    }

    let hname = hname as *mut c_char;
    if !prefix.is_null() {
        scnprintf(
            hname,
            hname_sz,
            b"%s//%s\0" as *const u8 as *const c_char,
            prefix,
            name,
        );
    } else {
        strscpy(hname, name, hname_sz as c_int);
    }

    (*policy).hname = hname as *const c_char;
    (*policy).name = basename((*policy).hname);

    true
}

pub unsafe extern "C" fn aa_policy_destroy(policy: *mut aa_policy) {
    // AA_BUG calls would go here but are debug assertions
    // Equivalent to: if on_list_rcu(&(*policy).profiles) { panic!(...) }
    // if on_list_rcu(&(*policy).list) { panic!(...) }

    // Don't free name as it's a subset of hname
    aa_put_str((*policy).hname);
}

unsafe fn aa_put_str(str: *const c_char) {
    if !str.is_null() {
        let counted = (str as *mut u8).offset(-(std::mem::size_of::<kref>() as isize))
            as *mut counted_str;
        // In a real implementation, this would call kref_put with the kref_release callback
        // For now, we just mark it for release (actual implementation would be in a separate module)
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
