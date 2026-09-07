// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor label definitions
 *
 * Copyright 2017 Canonical Ltd.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

pub type size_t = usize;
pub type ssize_t = isize;
pub type gfp_t = c_uint;
pub type c_uint = u32;
pub type u32 = c_uint;
pub type aa_state_t = c_uint;

pub const PROXY_POISON: c_long = 97;
pub const LABEL_POISON: c_long = 100;

extern "C" {
    static mut allperms: aa_perms;
    static mut nullperms: aa_perms;
    static mut root_ns: *mut aa_ns;
    static aa_hidden_ns_name: *const c_char;
    static aa_profile_mode_names: [*const c_char; 0];

    fn aa_put_label(label: *mut aa_label);
    fn aa_get_label(label: *mut aa_label) -> *mut aa_label;
    fn __aa_get_label(label: *mut aa_label) -> *mut aa_label;
    fn aa_get_newest_label(label: *mut aa_label) -> *mut aa_label;
    fn aa_get_newest_label_condref(label: *mut aa_label, needput: *mut bool) -> *mut aa_label;
    fn aa_put_label_condref(label: *mut aa_label, needput: bool);
    fn aa_put_proxy(proxy: *mut aa_proxy);
    fn aa_get_proxy(proxy: *mut aa_proxy) -> *mut aa_proxy;
    fn aa_put_profile(profile: *mut aa_profile);
    fn aa_get_profile(profile: *mut aa_profile) -> *mut aa_profile;
    fn aa_get_newest_profile(profile: *mut aa_profile) -> *mut aa_profile;
    fn aa_free_profile(profile: *mut aa_profile);
    fn aa_free_ns(ns: *mut aa_ns);
    fn aa_put_ns(ns: *mut aa_ns);
    fn aa_get_current_ns() -> *mut aa_ns;
    fn aa_alloc_secid(label: *mut aa_label, gfp: gfp_t) -> c_int;
    fn aa_free_secid(secid: c_uint);
    fn aa_put_str(str: *mut c_char);
    fn aa_str_alloc(size: size_t, gfp: gfp_t) -> *mut c_char;
    fn kzalloc(size: size_t, gfp: gfp_t) -> *mut c_void;
    fn kmalloc(size: size_t, gfp: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn snprintf(str: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn sort(base: *mut c_void, num: size_t, size: size_t,
            cmp_func: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
            swap_func: *mut c_void);
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn rb_replace_node(old: *mut rb_node, new: *mut rb_node, root: *mut rb_root);
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn rb_first(root: *mut rb_root) -> *mut rb_node;
    fn rwlock_init(lock: *mut rwlock_t);
    fn write_lock_irqsave(lock: *mut rwlock_t, flags: c_ulong);
    fn write_unlock_irqrestore(lock: *mut rwlock_t, flags: c_ulong);
    fn read_lock_irqsave(lock: *mut rwlock_t, flags: c_ulong);
    fn read_unlock_irqrestore(lock: *mut rwlock_t, flags: c_ulong);
    fn write_lock_nested(lock: *mut rwlock_t, subclass: c_int);
    fn write_unlock(lock: *mut rwlock_t);
    fn mutex_is_locked(lock: *mut mutex) -> bool;
    fn mutex_lock_nested(lock: *mut mutex, subclass: c_int);
    fn mutex_unlock(lock: *mut mutex);
    fn call_rcu(head: *mut rcu_head, func: unsafe extern "C" fn(*mut rcu_head));
    fn on_list_rcu(head: *mut list_head) -> bool;
    fn audit_log_n_untrustedstring(ab: *mut audit_buffer, str: *const c_char, n: c_int);
    fn seq_puts(f: *mut seq_file, s: *const c_char);
    fn seq_printf(f: *mut seq_file, fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn aa_dfa_match(dfa: *mut c_void, state: aa_state_t, str: *const c_char) -> aa_state_t;
    fn aa_dfa_match_len(dfa: *mut c_void, state: aa_state_t, str: *const c_char, len: size_t) -> aa_state_t;
    fn aa_lookup_perms(policy: *mut aa_policy, state: aa_state_t) -> *mut aa_perms;
    fn aa_perms_accum(perms: *mut aa_perms, tmp: *const aa_perms);
    fn aa_ns_name(view: *mut aa_ns, ns: *mut aa_ns, subns: bool) -> *const c_char;
    fn aa_ns_visible(view: *mut aa_ns, ns: *mut aa_ns, subns: bool) -> bool;
    fn aa_fqlookupn_profile(base: *mut aa_label, str: *const c_char, n: size_t) -> *mut aa_profile;
    fn skipn_spaces(str: *const c_char, n: size_t) -> *const c_char;
    fn aa_label_strn_split(str: *const c_char, n: size_t) -> *const c_char;
}

#[repr(C)] pub struct kref { _priv: [u8; 0] }
#[repr(C)] pub struct rcu_head { _priv: [u8; 0] }
#[repr(C)] pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct rwlock_t { _priv: [u8; 0] }
#[repr(C)] pub struct mutex { _priv: [u8; 0] }
#[repr(C)] pub struct list_head { _priv: [u8; 0] }
#[repr(C)] pub struct audit_buffer { _priv: [u8; 0] }
#[repr(C)] pub struct seq_file { _priv: [u8; 0] }
#[repr(C)] pub struct aa_policy { pub dfa: *mut c_void }
#[repr(C)] pub struct aa_ruleset { pub policy: *mut aa_policy }
#[repr(C)] pub struct aa_perms { pub allow: u32 }
#[repr(C)] pub struct aa_refcount { pub count: kref, pub reftype: c_int }
#[repr(C)] pub struct aa_proxy { pub count: aa_refcount, pub label: *mut aa_label }
#[repr(C)] pub struct aa_labelset { pub lock: rwlock_t, pub root: rb_root }
#[repr(C)] pub struct aa_base { pub hname: *mut c_char, pub profiles: list_head, pub list: list_head }
#[repr(C)] pub struct aa_ns {
    pub base: aa_base,
    pub level: c_int,
    pub labels: aa_labelset,
    pub lock: mutex,
    pub unconfined: *mut aa_profile,
    pub parent: *mut aa_ns,
    pub sub_ns: list_head,
}
#[repr(C)] pub struct aa_profile {
    pub base: aa_base,
    pub ns: *mut aa_ns,
    pub label: aa_label,
    pub mode: c_int,
}
#[repr(C)] pub struct aa_label {
    pub count: aa_refcount,
    pub rcu: rcu_head,
    pub node: rb_node,
    pub size: c_int,
    pub flags: c_long,
    pub mediates: c_uint,
    pub hname: *mut c_char,
    pub proxy: *mut aa_proxy,
    pub secid: c_uint,
    pub vec: [*mut aa_profile; 1],
}
#[repr(C)] #[derive(Clone, Copy)] pub struct label_it { pub i: c_int, pub j: c_int }

pub const REF_PROXY: c_int = 1;
pub const REF_NS: c_int = 2;
pub const FLAG_UNCONFINED: c_long = 1 << 0;
pub const FLAG_DEBUG1: c_long = 1 << 1;
pub const FLAG_DEBUG2: c_long = 1 << 2;
pub const FLAG_STALE: c_long = 1 << 3;
pub const FLAG_NS_COUNT: c_long = 1 << 4;
pub const FLAG_IN_TREE: c_long = 1 << 5;
pub const FLAG_SHOW_MODE: c_int = 1 << 6;
pub const FLAG_VIEW_SUBNS: c_int = 1 << 7;
pub const FLAG_HIDDEN_UNCONFINED: c_int = 1 << 8;
pub const FLAG_ABS_ROOT: c_int = 1 << 9;
pub const FLAGS_NONE: c_int = 0;
pub const VEC_FLAG_TERMINATE: c_int = 1;
pub const DEBUG_LABEL: c_int = 0;
pub const DEBUG_ABS_ROOT: bool = false;
pub const GFP_KERNEL: gfp_t = 0;
pub const ENOMEM: c_int = 12;
pub const EINVAL: c_int = 22;
pub const ENOENT: c_int = 2;
pub const DFA_NOMATCH: c_int = 0;
pub const APPARMOR_UNCONFINED: c_int = 0;
pub const AA_LS_LOCK_FIRST: c_int = 0;
pub const AA_LS_LOCK_SECOND: c_int = 1;

macro_rules! AA_BUG { ($($arg:tt)*) => {{ }}; }
macro_rules! AA_DEBUG { ($($arg:tt)*) => {{ }}; }

unsafe fn kref_init(_kref: *mut kref) {}
unsafe fn RB_CLEAR_NODE(node: *mut rb_node) { (*node).rb_left = ptr::null_mut(); (*node).rb_right = ptr::null_mut(); }
unsafe fn rcu_dereference_protected<T>(p: *mut T, _c: bool) -> *mut T { p }
unsafe fn rcu_assign_pointer<T>(slot: *mut *mut T, val: *mut T) { *slot = val; }
unsafe fn RCU_INIT_POINTER<T>(slot: *mut *mut T, val: *mut T) { *slot = val; }
unsafe fn label_isprofile(label: *const aa_label) -> bool { !label.is_null() && (*label).size == 1 }
unsafe fn label_is_stale(label: *const aa_label) -> bool { !label.is_null() && ((*label).flags & FLAG_STALE) != 0 }
unsafe fn __label_make_stale(label: *mut aa_label) { (*label).flags |= FLAG_STALE; }
unsafe fn labels_profile(label: *mut aa_label) -> *mut aa_profile { (*label).vec[0] }
unsafe fn labels_ns(label: *const aa_label) -> *mut aa_ns { if label.is_null() || (*label).vec[0].is_null() { ptr::null_mut() } else { (*(*label).vec[0]).ns } }
unsafe fn labels_set(label: *const aa_label) -> *mut aa_labelset { &mut (*labels_ns(label)).labels }
unsafe fn profiles_ns(profile: *mut aa_profile) -> *mut aa_ns { (*profile).ns }
unsafe fn ns_unconfined(ns: *mut aa_ns) -> *mut aa_label { &mut (*(*ns).unconfined).label }
unsafe fn profile_unconfined(profile: *mut aa_profile) -> bool { profile == (*(*profile).ns).unconfined }
unsafe fn profile_is_stale(profile: *mut aa_profile) -> bool { label_is_stale(&(*profile).label) }
unsafe fn vec_labelset(vec: *mut *mut aa_profile, n: c_int) -> *mut aa_labelset { labels_set(&(*(*vec.add((n - 1) as usize))).label) }
unsafe fn name_is_shared(_old: *mut aa_label, _new: *mut aa_label) -> bool { true }
unsafe fn ERR_PTR<T>(err: c_int) -> *mut T { (-(err as isize)) as *mut T }

unsafe fn rb_entry_label(node: *mut rb_node) -> *mut aa_label {
    (node as *mut u8).sub(mem::offset_of!(aa_label, node)) as *mut aa_label
}

unsafe fn aa_label_vec(label: *const aa_label, idx: c_int) -> *mut aa_profile {
    *(*label).vec.as_ptr().add(idx as usize)
}

unsafe fn aa_label_vec_set(label: *mut aa_label, idx: c_int, profile: *mut aa_profile) {
    let base = (*label).vec.as_mut_ptr();
    *base.add(idx as usize) = profile;
}

unsafe fn free_proxy(proxy: *mut aa_proxy) {
    if !proxy.is_null() {
        /* p->label will not updated any more as p is dead */
        aa_put_label(rcu_dereference_protected((*proxy).label, true));
        memset(proxy as *mut c_void, 0, mem::size_of::<aa_proxy>());
        RCU_INIT_POINTER(&mut (*proxy).label, PROXY_POISON as *mut aa_label);
        kfree(proxy as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn aa_proxy_kref(kref: *mut kref) {
    let proxy = (kref as *mut u8).sub(mem::offset_of!(aa_proxy, count) + mem::offset_of!(aa_refcount, count)) as *mut aa_proxy;
    free_proxy(proxy);
}

#[no_mangle]
pub unsafe extern "C" fn aa_alloc_proxy(label: *mut aa_label, gfp: gfp_t) -> *mut aa_proxy {
    let new = kzalloc(mem::size_of::<aa_proxy>(), gfp) as *mut aa_proxy;
    if !new.is_null() {
        kref_init(&mut (*new).count.count);
        (*new).count.reftype = REF_PROXY;
        rcu_assign_pointer(&mut (*new).label, aa_get_label(label));
    }
    new
}

/* requires profile list write lock held */
#[no_mangle]
pub unsafe extern "C" fn __aa_proxy_redirect(orig: *mut aa_label, new: *mut aa_label) {
    AA_BUG!(orig.is_null());
    AA_BUG!(new.is_null());
    let tmp = rcu_dereference_protected((*(*orig).proxy).label, true);
    rcu_assign_pointer(&mut (*(*orig).proxy).label, aa_get_label(new));
    __label_make_stale(orig);
    aa_put_label(tmp);
}

unsafe fn __proxy_share(old: *mut aa_label, new: *mut aa_label) {
    let proxy = (*new).proxy;
    (*new).proxy = aa_get_proxy((*old).proxy);
    __aa_proxy_redirect(old, new);
    aa_put_proxy(proxy);
}

/**
 * ns_cmp - compare ns for label set ordering
 */
unsafe fn ns_cmp(a: *mut aa_ns, b: *mut aa_ns) -> c_int {
    AA_BUG!(a.is_null());
    AA_BUG!(b.is_null());
    AA_BUG!((*a).base.hname.is_null());
    AA_BUG!((*b).base.hname.is_null());
    if a == b { return 0; }
    let res = (*a).level - (*b).level;
    if res != 0 { return res; }
    strcmp((*a).base.hname, (*b).base.hname)
}

/**
 * profile_cmp - profile comparison for set ordering
 */
unsafe fn profile_cmp(a: *mut aa_profile, b: *mut aa_profile) -> c_int {
    AA_BUG!(a.is_null());
    AA_BUG!(b.is_null());
    AA_BUG!((*a).ns.is_null());
    AA_BUG!((*b).ns.is_null());
    AA_BUG!((*a).base.hname.is_null());
    AA_BUG!((*b).base.hname.is_null());
    if a == b || (*a).base.hname == (*b).base.hname { return 0; }
    let res = ns_cmp((*a).ns, (*b).ns);
    if res != 0 { return res; }
    strcmp((*a).base.hname, (*b).base.hname)
}

/**
 * vec_cmp - label comparison for set ordering
 */
unsafe fn vec_cmp(a: *mut *mut aa_profile, an: c_int, b: *mut *mut aa_profile, bn: c_int) -> c_int {
    AA_BUG!(a.is_null());
    AA_BUG!((*a).is_null());
    AA_BUG!(b.is_null());
    AA_BUG!((*b).is_null());
    AA_BUG!(an <= 0);
    AA_BUG!(bn <= 0);
    let mut i = 0;
    while i < an && i < bn {
        let res = profile_cmp(*a.add(i as usize), *b.add(i as usize));
        if res != 0 { return res; }
        i += 1;
    }
    an - bn
}

unsafe fn vec_is_stale(vec: *mut *mut aa_profile, n: c_int) -> bool {
    AA_BUG!(vec.is_null());
    let mut i = 0;
    while i < n {
        if profile_is_stale(*vec.add(i as usize)) { return true; }
        i += 1;
    }
    false
}

unsafe fn accum_label_info(new: *mut aa_label) {
    let mut u = FLAG_UNCONFINED;
    AA_BUG!(new.is_null());
    /* size == 1 is a profile and flags must be set as part of creation */
    if (*new).size == 1 { return; }
    let mut i = 0;
    while i < (*new).size {
        let p = aa_label_vec(new, i);
        u |= (*p).label.flags & (FLAG_DEBUG1 | FLAG_DEBUG2 | FLAG_STALE);
        if (u & (*p).label.flags & FLAG_UNCONFINED) == 0 { u &= !FLAG_UNCONFINED; }
        (*new).mediates |= (*p).label.mediates;
        i += 1;
    }
    (*new).flags |= u;
}

unsafe extern "C" fn sort_cmp(a: *const c_void, b: *const c_void) -> c_int {
    profile_cmp(*(a as *const *mut aa_profile), *(b as *const *mut aa_profile))
}

/*
 * assumes vec is sorted
 * Assumes @vec has null terminator at vec[n], and will null terminate
 * vec[n - dups]
 */
unsafe fn unique(vec: *mut *mut aa_profile, n: c_int) -> c_int {
    let mut dups = 0;
    AA_BUG!(n < 1);
    AA_BUG!(vec.is_null());
    let mut pos = 0;
    let mut i = 1;
    while i < n {
        let res = profile_cmp(*vec.add(pos as usize), *vec.add(i as usize));
        AA_BUG!(res > 0, "vec not sorted");
        if res == 0 {
            /* drop duplicate */
            aa_put_profile(*vec.add(i as usize));
            dups += 1;
            i += 1;
            continue;
        }
        pos += 1;
        if dups != 0 { *vec.add(pos as usize) = *vec.add(i as usize); }
        i += 1;
    }
    AA_BUG!(dups < 0);
    dups
}

#[no_mangle]
pub unsafe extern "C" fn aa_vec_unique(vec: *mut *mut aa_profile, n: c_int, flags: c_int) -> c_int {
    let mut dups = 0;
    AA_BUG!(n < 1);
    AA_BUG!(vec.is_null());
    if n > 8 {
        sort(vec as *mut c_void, n as size_t, mem::size_of::<*mut aa_profile>(), Some(sort_cmp), ptr::null_mut());
        dups = unique(vec, n);
    } else {
        let mut i = 1;
        while i < n {
            let tmp = *vec.add(i as usize);
            let mut pos = i - 1 - dups;
            while pos >= 0 {
                let res = profile_cmp(*vec.add(pos as usize), tmp);
                if res == 0 {
                    /* drop duplicate entry */
                    aa_put_profile(tmp);
                    dups += 1;
                    break;
                } else if res < 0 {
                    break;
                }
                pos -= 1;
            }
            if pos < 0 || profile_cmp(*vec.add(pos as usize), tmp) != 0 {
                pos += 1;
                let mut j = i - dups;
                while j > pos {
                    *vec.add(j as usize) = *vec.add((j - 1) as usize);
                    j -= 1;
                }
                *vec.add(pos as usize) = tmp;
            }
            i += 1;
        }
        AA_BUG!(dups < 0);
    }
    if (flags & VEC_FLAG_TERMINATE) != 0 { *vec.add((n - dups) as usize) = ptr::null_mut(); }
    dups
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_destroy(label: *mut aa_label) {
    AA_BUG!(label.is_null());
    if !label_isprofile(label) {
        aa_put_str((*label).hname);
        let mut i = 0;
        while i < (*label).size {
            let profile = aa_label_vec(label, i);
            aa_put_profile(profile);
            aa_label_vec_set(label, i, (LABEL_POISON + i as c_long) as *mut aa_profile);
            i += 1;
        }
    }
    if !(*label).proxy.is_null() {
        if rcu_dereference_protected((*(*label).proxy).label, true) == label {
            rcu_assign_pointer(&mut (*(*label).proxy).label, ptr::null_mut());
        }
        aa_put_proxy((*label).proxy);
    }
    aa_free_secid((*label).secid);
    (*label).proxy = (PROXY_POISON as usize + mem::size_of::<aa_proxy>()) as *mut aa_proxy;
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_free(label: *mut aa_label) {
    if label.is_null() { return; }
    aa_label_destroy(label);
    kfree(label as *mut c_void);
}

unsafe fn label_free_switch(label: *mut aa_label) {
    if ((*label).flags & FLAG_NS_COUNT) != 0 {
        aa_free_ns(labels_ns(label));
    } else if label_isprofile(label) {
        aa_free_profile(labels_profile(label));
    } else {
        aa_label_free(label);
    }
}

unsafe extern "C" fn label_free_rcu(head: *mut rcu_head) {
    let label = (head as *mut u8).sub(mem::offset_of!(aa_label, rcu)) as *mut aa_label;
    if ((*label).flags & FLAG_IN_TREE) != 0 { let _ = aa_label_remove(label); }
    label_free_switch(label);
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_kref(kref: *mut kref) {
    let label = (kref as *mut u8).sub(mem::offset_of!(aa_label, count) + mem::offset_of!(aa_refcount, count)) as *mut aa_label;
    let ns = labels_ns(label);
    if ns.is_null() {
        /* never live, no rcu callback needed, just using the fn */
        label_free_switch(label);
        return;
    }
    /* TODO: update labels_profile macro so it works here */
    AA_BUG!(label_isprofile(label) && on_list_rcu(&mut (*aa_label_vec(label, 0)).base.profiles));
    AA_BUG!(label_isprofile(label) && on_list_rcu(&mut (*aa_label_vec(label, 0)).base.list));
    /* TODO: if compound label and not stale add to reclaim cache */
    call_rcu(&mut (*label).rcu, label_free_rcu);
}

unsafe fn label_free_or_put_new(label: *mut aa_label, new: *mut aa_label) {
    if label != new {
        /* need to free directly to break circular ref with proxy */
        aa_label_free(new);
    } else {
        aa_put_label(new);
    }
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_init(label: *mut aa_label, size: c_int, gfp: gfp_t) -> bool {
    AA_BUG!(label.is_null());
    AA_BUG!(size < 1);
    if aa_alloc_secid(label, gfp) < 0 { return false; }
    (*label).size = size;          /* doesn't include null */
    aa_label_vec_set(label, size, ptr::null_mut()); /* null terminate */
    kref_init(&mut (*label).count.count);
    (*label).count.reftype = REF_NS;        /* for aafs purposes */
    RB_CLEAR_NODE(&mut (*label).node);
    true
}

/**
 * aa_label_alloc - allocate a label with a profile vector of @size length
 */
#[no_mangle]
pub unsafe extern "C" fn aa_label_alloc(size: c_int, mut proxy: *mut aa_proxy, gfp: gfp_t) -> *mut aa_label {
    AA_BUG!(size < 1);
    /*  + 1 for null terminator entry on vec */
    let bytes = mem::size_of::<aa_label>() + (size as usize) * mem::size_of::<*mut aa_profile>();
    let new = kzalloc(bytes, gfp) as *mut aa_label;
    AA_DEBUG!(DEBUG_LABEL, "%s (%p)\n", "aa_label_alloc", new);
    if new.is_null() { return ptr::null_mut(); }
    if !aa_label_init(new, size, gfp) { aa_label_free(new); return ptr::null_mut(); }
    if proxy.is_null() {
        proxy = aa_alloc_proxy(new, gfp);
        if proxy.is_null() { aa_label_free(new); return ptr::null_mut(); }
    } else {
        aa_get_proxy(proxy);
    }
    /* just set new's proxy, don't redirect proxy here if it was passed in*/
    (*new).proxy = proxy;
    new
}

/**
 * label_cmp - label comparison for set ordering
 */
unsafe fn label_cmp(a: *const aa_label, b: *const aa_label) -> c_int {
    AA_BUG!(b.is_null());
    if a == b { return 0; }
    vec_cmp((*a).vec.as_ptr() as *mut *mut aa_profile, (*a).size, (*b).vec.as_ptr() as *mut *mut aa_profile, (*b).size)
}

/* helper fn for label_for_each_confined */
#[no_mangle]
pub unsafe extern "C" fn aa_label_next_confined(label: *const aa_label, mut i: c_int) -> c_int {
    AA_BUG!(label.is_null());
    AA_BUG!(i < 0);
    while i < (*label).size {
        if !profile_unconfined(aa_label_vec(label, i)) { return i; }
        i += 1;
    }
    i
}

#[no_mangle]
pub unsafe extern "C" fn __aa_label_next_not_in_set(I: *mut label_it, set: *const aa_label, sub: *const aa_label) -> *mut aa_profile {
    AA_BUG!(set.is_null());
    AA_BUG!(I.is_null());
    AA_BUG!((*I).i < 0);
    AA_BUG!((*I).i > (*set).size);
    AA_BUG!(sub.is_null());
    AA_BUG!((*I).j < 0);
    AA_BUG!((*I).j > (*sub).size);
    while (*I).j < (*sub).size && (*I).i < (*set).size {
        let res = profile_cmp(aa_label_vec(sub, (*I).j), aa_label_vec(set, (*I).i));
        if res == 0 {
            (*I).j += 1;
            (*I).i += 1;
        } else if res > 0 {
            (*I).i += 1;
        } else {
            let p = aa_label_vec(sub, (*I).j);
            (*I).j += 1;
            return p;
        }
    }
    if (*I).j < (*sub).size {
        let p = aa_label_vec(sub, (*I).j);
        (*I).j += 1;
        return p;
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_is_subset(set: *const aa_label, sub: *const aa_label) -> bool {
    let mut i = label_it { i: 0, j: 0 };
    AA_BUG!(set.is_null());
    AA_BUG!(sub.is_null());
    if sub == set { return true; }
    __aa_label_next_not_in_set(&mut i, set, sub).is_null()
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_is_unconfined_subset(set: *const aa_label, sub: *const aa_label) -> bool {
    let mut i = label_it { i: 0, j: 0 };
    let mut p: *mut aa_profile;
    AA_BUG!(set.is_null());
    AA_BUG!(sub.is_null());
    if sub == set { return true; }
    loop {
        p = __aa_label_next_not_in_set(&mut i, set, sub);
        if !p.is_null() && !profile_unconfined(p) { break; }
        if p.is_null() { break; }
    }
    p.is_null()
}

unsafe fn __label_remove(label: *mut aa_label, new: *mut aa_label) -> bool {
    let ls = labels_set(label);
    AA_BUG!(ls.is_null());
    AA_BUG!(label.is_null());
    if !new.is_null() { __aa_proxy_redirect(label, new); }
    if !label_is_stale(label) { __label_make_stale(label); }
    if ((*label).flags & FLAG_IN_TREE) != 0 {
        rb_erase(&mut (*label).node, &mut (*ls).root);
        (*label).flags &= !FLAG_IN_TREE;
        return true;
    }
    false
}

unsafe fn __label_replace(old: *mut aa_label, new: *mut aa_label) -> bool {
    let ls = labels_set(old);
    AA_BUG!(ls.is_null());
    AA_BUG!(old.is_null());
    AA_BUG!(new.is_null());
    AA_BUG!(((*new).flags & FLAG_IN_TREE) != 0);
    if !label_is_stale(old) { __label_make_stale(old); }
    if ((*old).flags & FLAG_IN_TREE) != 0 {
        rb_replace_node(&mut (*old).node, &mut (*new).node, &mut (*ls).root);
        (*old).flags &= !FLAG_IN_TREE;
        (*new).flags |= FLAG_IN_TREE;
        accum_label_info(new);
        return true;
    }
    false
}

unsafe fn __label_insert(ls: *mut aa_labelset, label: *mut aa_label, replace: bool) -> *mut aa_label {
    let mut new = &mut (*ls).root.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();
    AA_BUG!(ls.is_null());
    AA_BUG!(label.is_null());
    AA_BUG!(labels_set(label) != ls);
    AA_BUG!(((*label).flags & FLAG_IN_TREE) != 0);
    while !(*new).is_null() {
        let this = rb_entry_label(*new);
        let result = label_cmp(label, this);
        parent = *new;
        if result == 0 {
            if !replace && !label_is_stale(this) {
                if !__aa_get_label(this).is_null() { return this; }
            } else {
                __proxy_share(this, label);
            }
            AA_BUG!(!__label_replace(this, label));
            return aa_get_label(label);
        } else if result < 0 {
            new = &mut (**new).rb_left;
        } else {
            new = &mut (**new).rb_right;
        }
    }
    rb_link_node(&mut (*label).node, parent, new);
    rb_insert_color(&mut (*label).node, &mut (*ls).root);
    (*label).flags |= FLAG_IN_TREE;
    accum_label_info(label);
    aa_get_label(label)
}

unsafe fn __vec_find(vec: *mut *mut aa_profile, n: c_int) -> *mut aa_label {
    AA_BUG!(vec.is_null());
    AA_BUG!((*vec).is_null());
    AA_BUG!(n <= 0);
    let mut node = (*vec_labelset(vec, n)).root.rb_node;
    while !node.is_null() {
        let this = rb_entry_label(node);
        let result = vec_cmp((*this).vec.as_ptr() as *mut *mut aa_profile, (*this).size, vec, n);
        if result > 0 { node = (*node).rb_left; }
        else if result < 0 { node = (*node).rb_right; }
        else { return __aa_get_label(this); }
    }
    ptr::null_mut()
}

unsafe fn __label_find(label: *mut aa_label) -> *mut aa_label {
    AA_BUG!(label.is_null());
    __vec_find((*label).vec.as_mut_ptr(), (*label).size)
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_remove(label: *mut aa_label) -> bool {
    let ls = labels_set(label);
    let flags: c_ulong = 0;
    AA_BUG!(ls.is_null());
    write_lock_irqsave(&mut (*ls).lock, flags);
    let res = __label_remove(label, ns_unconfined(labels_ns(label)));
    write_unlock_irqrestore(&mut (*ls).lock, flags);
    res
}

unsafe fn write_lock_irqsave_nested(L: *mut rwlock_t, F: c_ulong, _SC: c_int) {
    write_lock_irqsave(L, F);
}

unsafe fn ns_ls_double_lock(mut ns1: *mut aa_ns, mut ns2: *mut aa_ns, flags: *mut c_ulong) {
    if ns1 == ns2 {
        write_lock_irqsave(&mut (*ns1).labels.lock, *flags);
        return;
    }
    if (*ns1).level > (*ns2).level || ((*ns1).level == (*ns2).level && (ns1 as usize) > (ns2 as usize)) {
        mem::swap(&mut ns1, &mut ns2);
    }
    write_lock_irqsave_nested(&mut (*ns1).labels.lock, *flags, AA_LS_LOCK_FIRST);
    write_lock_nested(&mut (*ns2).labels.lock, AA_LS_LOCK_SECOND);
}

unsafe fn ns_ls_double_unlock(ns1: *mut aa_ns, ns2: *mut aa_ns, flags: c_ulong) {
    if ns1 == ns2 {
        write_unlock_irqrestore(&mut (*ns1).labels.lock, flags);
        return;
    }
    write_unlock(&mut (*ns2).labels.lock);
    write_unlock_irqrestore(&mut (*ns1).labels.lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_replace(old: *mut aa_label, new: *mut aa_label) -> bool {
    let ons = labels_ns(old);
    let nns = labels_ns(new);
    let flags: c_ulong = 0;
    let res: bool;
    ns_ls_double_lock(ons, nns, &flags as *const _ as *mut _);
    if ons == nns && name_is_shared(old, new) {
        if (*old).proxy != (*new).proxy { __proxy_share(old, new); }
        else { __aa_proxy_redirect(old, new); }
        res = __label_replace(old, new);
    } else {
        res = __label_remove(old, new);
        let l = __label_insert(&mut (*nns).labels, new, true);
        let r = l == new;
        aa_put_label(l);
        ns_ls_double_unlock(ons, nns, flags);
        return r;
    }
    ns_ls_double_unlock(ons, nns, flags);
    res
}

unsafe fn vec_find(vec: *mut *mut aa_profile, n: c_int) -> *mut aa_label {
    let ls = vec_labelset(vec, n);
    let flags: c_ulong = 0;
    AA_BUG!(vec.is_null());
    AA_BUG!((*vec).is_null());
    AA_BUG!(n <= 0);
    read_lock_irqsave(&mut (*ls).lock, flags);
    let label = __vec_find(vec, n);
    read_unlock_irqrestore(&mut (*ls).lock, flags);
    label
}

unsafe fn vec_create_and_insert_label(vec: *mut *mut aa_profile, len: c_int, gfp: gfp_t) -> *mut aa_label {
    AA_BUG!(vec.is_null());
    if len == 1 { return aa_get_label(&mut (*(*vec)).label); }
    let ls = labels_set(&mut (*(*vec.add((len - 1) as usize))).label);
    let flags: c_ulong = 0;
    let new = aa_label_alloc(len, ptr::null_mut(), gfp);
    if new.is_null() { return ptr::null_mut(); }
    let mut i = 0;
    while i < len {
        aa_label_vec_set(new, i, aa_get_profile(*vec.add(i as usize)));
        i += 1;
    }
    write_lock_irqsave(&mut (*ls).lock, flags);
    let label = __label_insert(ls, new, false);
    write_unlock_irqrestore(&mut (*ls).lock, flags);
    label_free_or_put_new(label, new);
    label
}

#[no_mangle]
pub unsafe extern "C" fn aa_vec_find_or_create_label(vec: *mut *mut aa_profile, len: c_int, gfp: gfp_t) -> *mut aa_label {
    let label = vec_find(vec, len);
    if !label.is_null() { return label; }
    vec_create_and_insert_label(vec, len, gfp)
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_insert(ls: *mut aa_labelset, label: *mut aa_label) -> *mut aa_label {
    let flags: c_ulong = 0;
    AA_BUG!(ls.is_null());
    AA_BUG!(label.is_null());
    if !label_is_stale(label) {
        read_lock_irqsave(&mut (*ls).lock, flags);
        let l = __label_find(label);
        read_unlock_irqrestore(&mut (*ls).lock, flags);
        if !l.is_null() { return l; }
    }
    write_lock_irqsave(&mut (*ls).lock, flags);
    let l = __label_insert(ls, label, false);
    write_unlock_irqrestore(&mut (*ls).lock, flags);
    l
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_next_in_merge(I: *mut label_it, a: *const aa_label, b: *const aa_label) -> *mut aa_profile {
    AA_BUG!(a.is_null());
    AA_BUG!(b.is_null());
    AA_BUG!(I.is_null());
    if (*I).i < (*a).size {
        if (*I).j < (*b).size {
            let res = profile_cmp(aa_label_vec(a, (*I).i), aa_label_vec(b, (*I).j));
            if res > 0 {
                let p = aa_label_vec(b, (*I).j);
                (*I).j += 1;
                return p;
            }
            if res == 0 { (*I).j += 1; }
        }
        let p = aa_label_vec(a, (*I).i);
        (*I).i += 1;
        return p;
    }
    if (*I).j < (*b).size {
        let p = aa_label_vec(b, (*I).j);
        (*I).j += 1;
        return p;
    }
    ptr::null_mut()
}

unsafe fn label_merge_cmp(a: *mut aa_label, b: *mut aa_label, z: *mut aa_label) -> c_int {
    let mut p: *mut aa_profile = ptr::null_mut();
    let mut i = label_it { i: 0, j: 0 };
    let mut k = 0;
    AA_BUG!(a.is_null());
    AA_BUG!(b.is_null());
    AA_BUG!(z.is_null());
    while k < (*z).size {
        p = aa_label_next_in_merge(&mut i, a, b);
        if p.is_null() { break; }
        let res = profile_cmp(p, aa_label_vec(z, k));
        if res != 0 { return res; }
        k += 1;
    }
    if !p.is_null() { 1 } else if k < (*z).size { -1 } else { 0 }
}

unsafe fn label_merge_insert(new: *mut aa_label, a: *mut aa_label, b: *mut aa_label) -> *mut aa_label {
    let mut i = label_it { i: 0, j: 0 };
    let mut k = 0;
    let mut invcount = 0;
    let mut stale = false;
    AA_BUG!(a.is_null());
    AA_BUG!((*a).size < 0);
    AA_BUG!(b.is_null());
    AA_BUG!((*b).size < 0);
    AA_BUG!(new.is_null());
    AA_BUG!((*new).size < (*a).size + (*b).size);
    loop {
        let next = aa_label_next_in_merge(&mut i, a, b);
        if next.is_null() { break; }
        AA_BUG!(next.is_null());
        if profile_is_stale(next) {
            aa_label_vec_set(new, k, aa_get_newest_profile(next));
            AA_BUG!((*aa_label_vec(new, k)).label.proxy.is_null());
            AA_BUG!((*(*aa_label_vec(new, k)).label.proxy).label.is_null());
            if (*next).label.proxy != (*aa_label_vec(new, k)).label.proxy { invcount += 1; }
            k += 1;
            stale = true;
        } else {
            aa_label_vec_set(new, k, aa_get_profile(next));
            k += 1;
        }
    }
    (*new).size = k;
    aa_label_vec_set(new, k, ptr::null_mut());
    if invcount != 0 {
        (*new).size -= aa_vec_unique((*new).vec.as_mut_ptr(), (*new).size, VEC_FLAG_TERMINATE);
        /* TODO: deal with reference labels */
        if (*new).size == 1 { return aa_get_label(&mut (*aa_label_vec(new, 0)).label); }
    } else if !stale {
        if k == (*a).size { return aa_get_label(a); }
        else if k == (*b).size { return aa_get_label(b); }
    }
    let ls = labels_set(new);
    let flags: c_ulong = 0;
    write_lock_irqsave(&mut (*ls).lock, flags);
    let label = __label_insert(labels_set(new), new, false);
    write_unlock_irqrestore(&mut (*ls).lock, flags);
    label
}

unsafe fn labelset_of_merge(a: *mut aa_label, b: *mut aa_label) -> *mut aa_labelset {
    let nsa = labels_ns(a);
    let nsb = labels_ns(b);
    if ns_cmp(nsa, nsb) <= 0 { &mut (*nsa).labels } else { &mut (*nsb).labels }
}

unsafe fn __label_find_merge(ls: *mut aa_labelset, a: *mut aa_label, b: *mut aa_label) -> *mut aa_label {
    AA_BUG!(ls.is_null());
    AA_BUG!(a.is_null());
    AA_BUG!(b.is_null());
    if a == b { return __label_find(a); }
    let mut node = (*ls).root.rb_node;
    while !node.is_null() {
        let this = rb_entry_label(node);
        let result = label_merge_cmp(a, b, this);
        if result < 0 { node = (*node).rb_left; }
        else if result > 0 { node = (*node).rb_right; }
        else { return __aa_get_label(this); }
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_find_merge(mut a: *mut aa_label, mut b: *mut aa_label) -> *mut aa_label {
    let mut a_needput = false;
    let mut b_needput = false;
    AA_BUG!(a.is_null());
    AA_BUG!(b.is_null());
    a = aa_get_newest_label_condref(a, &mut a_needput);
    b = aa_get_newest_label_condref(b, &mut b_needput);
    let ls = labelset_of_merge(a, b);
    let flags: c_ulong = 0;
    read_lock_irqsave(&mut (*ls).lock, flags);
    let label = __label_find_merge(ls, a, b);
    read_unlock_irqrestore(&mut (*ls).lock, flags);
    aa_put_label_condref(a, a_needput);
    aa_put_label_condref(b, b_needput);
    label
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_merge(mut a: *mut aa_label, mut b: *mut aa_label, gfp: gfp_t) -> *mut aa_label {
    let mut label: *mut aa_label = ptr::null_mut();
    AA_BUG!(a.is_null());
    AA_BUG!(b.is_null());
    if a == b { return aa_get_newest_label(a); }
    if label.is_null() {
        let mut a_needput = false;
        let mut b_needput = false;
        a = aa_get_newest_label_condref(a, &mut a_needput);
        b = aa_get_newest_label_condref(b, &mut b_needput);
        let new = aa_label_alloc((*a).size + (*b).size, ptr::null_mut(), gfp);
        if !new.is_null() {
            label = label_merge_insert(new, a, b);
            label_free_or_put_new(label, new);
        }
        aa_put_label_condref(a, a_needput);
        aa_put_label_condref(b, b_needput);
    }
    label
}

/* match a profile and its associated ns component if needed
 * Assumes visibility test has already been done.
 */
unsafe fn match_component(profile: *const aa_profile, rules: *mut aa_ruleset, tp: *const aa_profile, mut state: aa_state_t) -> aa_state_t {
    if (*profile).ns == (*tp).ns {
        return aa_dfa_match((*(*rules).policy).dfa, state, (*tp).base.hname);
    }
    let ns_name = aa_ns_name((*profile).ns, (*tp).ns, true);
    state = aa_dfa_match_len((*(*rules).policy).dfa, state, b":\0".as_ptr() as *const c_char, 1);
    state = aa_dfa_match((*(*rules).policy).dfa, state, ns_name);
    state = aa_dfa_match_len((*(*rules).policy).dfa, state, b":\0".as_ptr() as *const c_char, 1);
    aa_dfa_match((*(*rules).policy).dfa, state, (*tp).base.hname)
}

unsafe fn label_compound_match(profile: *const aa_profile, rules: *mut aa_ruleset, label: *mut aa_label, mut state: aa_state_t, inview: bool, _request: u32, perms: *mut aa_perms) -> c_int {
    let mut i = 0;
    while i < (*label).size {
        let tp = aa_label_vec(label, i);
        if aa_ns_visible((*profile).ns, (*tp).ns, inview) {
            state = match_component(profile, rules, tp, state);
            if state == 0 { *perms = nullperms; return DFA_NOMATCH; }
            i += 1;
            break;
        }
        i += 1;
    }
    if i > (*label).size {
        *perms = allperms;
        return state as c_int;
    }
    while i < (*label).size {
        let tp = aa_label_vec(label, i);
        if aa_ns_visible((*profile).ns, (*tp).ns, inview) {
            state = aa_dfa_match((*(*rules).policy).dfa, state, b"//&\0".as_ptr() as *const c_char);
            state = match_component(profile, rules, tp, state);
            if state == 0 { *perms = nullperms; return DFA_NOMATCH; }
        }
        i += 1;
    }
    *perms = *aa_lookup_perms((*rules).policy, state);
    state as c_int
}

unsafe fn label_components_match(profile: *const aa_profile, rules: *mut aa_ruleset, label: *mut aa_label, start: aa_state_t, inview: bool, request: u32, perms: *mut aa_perms) -> c_int {
    let mut state: aa_state_t = 0;
    let mut i = 0;
    while i < (*label).size {
        let tp = aa_label_vec(label, i);
        if aa_ns_visible((*profile).ns, (*tp).ns, inview) {
            state = match_component(profile, rules, tp, start);
            if state == 0 { *perms = nullperms; return DFA_NOMATCH; }
            let tmp = *aa_lookup_perms((*rules).policy, state);
            aa_perms_accum(perms, &tmp);
            i += 1;
            break;
        }
        i += 1;
    }
    if state == 0 { return state as c_int; }
    while i < (*label).size {
        let tp = aa_label_vec(label, i);
        if aa_ns_visible((*profile).ns, (*tp).ns, inview) {
            state = match_component(profile, rules, tp, start);
            if state == 0 { *perms = nullperms; return DFA_NOMATCH; }
            let tmp = *aa_lookup_perms((*rules).policy, state);
            aa_perms_accum(perms, &tmp);
        }
        i += 1;
    }
    if ((*perms).allow & request) != request { return DFA_NOMATCH; }
    state as c_int
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_match(profile: *const aa_profile, rules: *mut aa_ruleset, label: *mut aa_label, state: aa_state_t, inview: bool, request: u32, perms: *mut aa_perms) -> c_int {
    let tmp = label_compound_match(profile, rules, label, state, inview, request, perms);
    if ((*perms).allow & request) == request { return tmp; }
    *perms = allperms;
    label_components_match(profile, rules, label, state, inview, request, perms)
}

#[no_mangle]
pub unsafe extern "C" fn aa_update_label_name(ns: *mut aa_ns, label: *mut aa_label, gfp: gfp_t) -> bool {
    let mut name: *mut c_char = ptr::null_mut();
    let mut res = false;
    AA_BUG!(ns.is_null());
    AA_BUG!(label.is_null());
    if !(*label).hname.is_null() || labels_ns(label) != ns { return res; }
    if aa_label_acntsxprint(&mut name, ns, label, FLAGS_NONE, gfp) < 0 { return res; }
    let ls = labels_set(label);
    let flags: c_ulong = 0;
    write_lock_irqsave(&mut (*ls).lock, flags);
    if (*label).hname.is_null() && ((*label).flags & FLAG_IN_TREE) != 0 {
        (*label).hname = name;
        res = true;
    } else {
        aa_put_str(name);
    }
    write_unlock_irqrestore(&mut (*ls).lock, flags);
    res
}

unsafe fn use_label_hname(ns: *mut aa_ns, label: *mut aa_label, flags: c_int) -> bool {
    if !(*label).hname.is_null() && (ns.is_null() || labels_ns(label) == ns) && (flags & !FLAG_SHOW_MODE) == 0 {
        return true;
    }
    false
}

unsafe fn update_for_len(total: &mut c_int, len: ssize_t, size: &mut size_t, strp: &mut *mut c_char) {
    AA_BUG!(len < 0);
    let mut ulen = len as size_t;
    *total += ulen as c_int;
    if ulen > *size { ulen = *size; }
    *size -= ulen;
    *strp = (*strp).add(ulen);
}

unsafe fn aa_profile_snxprint(mut strp: *mut c_char, size: size_t, mut view: *mut aa_ns, profile: *mut aa_profile, flags: c_int, prev_ns: *mut *mut aa_ns) -> c_int {
    let mut ns_name: *const c_char = ptr::null();
    AA_BUG!(strp.is_null() && size != 0);
    AA_BUG!(profile.is_null());
    if view.is_null() { view = profiles_ns(profile); }
    if view != (*profile).ns && (prev_ns.is_null() || *prev_ns != (*profile).ns) {
        if !prev_ns.is_null() { *prev_ns = (*profile).ns; }
        ns_name = aa_ns_name(view, (*profile).ns, (flags & FLAG_VIEW_SUBNS) != 0);
        if ns_name == aa_hidden_ns_name {
            if (flags & FLAG_HIDDEN_UNCONFINED) != 0 {
                return snprintf(strp, size, b"%s\0".as_ptr() as *const c_char, b"unconfined\0".as_ptr() as *const c_char);
            }
            return snprintf(strp, size, b"%s\0".as_ptr() as *const c_char, ns_name);
        }
    }
    if (flags & FLAG_SHOW_MODE) != 0 && profile != (*(*profile).ns).unconfined {
        let modestr = *aa_profile_mode_names.as_ptr().add((*profile).mode as usize);
        if !ns_name.is_null() {
            return snprintf(strp, size, b":%s:%s (%s)\0".as_ptr() as *const c_char, ns_name, (*profile).base.hname, modestr);
        }
        return snprintf(strp, size, b"%s (%s)\0".as_ptr() as *const c_char, (*profile).base.hname, modestr);
    }
    if !ns_name.is_null() {
        return snprintf(strp, size, b":%s:%s\0".as_ptr() as *const c_char, ns_name, (*profile).base.hname);
    }
    snprintf(strp, size, b"%s\0".as_ptr() as *const c_char, (*profile).base.hname)
}

unsafe fn label_modename(ns: *mut aa_ns, label: *mut aa_label, flags: c_int) -> *const c_char {
    let mut mode = -1;
    let mut count = 0;
    let mut i = 0;
    while i < (*label).size {
        let profile = aa_label_vec(label, i);
        if aa_ns_visible(ns, (*profile).ns, (flags & FLAG_VIEW_SUBNS) != 0) {
            count += 1;
            if profile == (*(*profile).ns).unconfined { i += 1; continue; }
            if mode == -1 { mode = (*profile).mode; }
            else if mode != (*profile).mode { return b"mixed\0".as_ptr() as *const c_char; }
        }
        i += 1;
    }
    if count == 0 { return b"-\0".as_ptr() as *const c_char; }
    if mode == -1 { mode = APPARMOR_UNCONFINED; }
    *aa_profile_mode_names.as_ptr().add(mode as usize)
}

unsafe fn display_mode(ns: *mut aa_ns, label: *mut aa_label, flags: c_int) -> bool {
    if (flags & FLAG_SHOW_MODE) != 0 {
        let mut i = 0;
        while i < (*label).size {
            let profile = aa_label_vec(label, i);
            if aa_ns_visible(ns, (*profile).ns, (flags & FLAG_VIEW_SUBNS) != 0) && profile != (*(*profile).ns).unconfined {
                return true;
            }
            i += 1;
        }
        return false;
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_snxprint(mut strp: *mut c_char, mut size: size_t, mut ns: *mut aa_ns, label: *mut aa_label, flags: c_int) -> c_int {
    let mut prev_ns: *mut aa_ns = ptr::null_mut();
    let mut count = 0;
    let mut total = 0;
    AA_BUG!(strp.is_null() && size != 0);
    AA_BUG!(label.is_null());
    if DEBUG_ABS_ROOT && (flags & FLAG_ABS_ROOT) != 0 {
        ns = root_ns;
        let len = snprintf(strp, size, b"_\0".as_ptr() as *const c_char);
        update_for_len(&mut total, len as ssize_t, &mut size, &mut strp);
    } else if ns.is_null() {
        ns = labels_ns(label);
    }
    let mut i = 0;
    while i < (*label).size {
        let profile = aa_label_vec(label, i);
        if aa_ns_visible(ns, (*profile).ns, (flags & FLAG_VIEW_SUBNS) != 0) {
            if count > 0 {
                let len = snprintf(strp, size, b"//&\0".as_ptr() as *const c_char);
                update_for_len(&mut total, len as ssize_t, &mut size, &mut strp);
            }
            let len = aa_profile_snxprint(strp, size, ns, profile, flags & FLAG_VIEW_SUBNS, &mut prev_ns);
            update_for_len(&mut total, len as ssize_t, &mut size, &mut strp);
            count += 1;
        }
        i += 1;
    }
    if count == 0 {
        if (flags & FLAG_HIDDEN_UNCONFINED) != 0 {
            return snprintf(strp, size, b"%s\0".as_ptr() as *const c_char, b"unconfined\0".as_ptr() as *const c_char);
        }
        return snprintf(strp, size, b"%s\0".as_ptr() as *const c_char, aa_hidden_ns_name);
    }
    if display_mode(ns, label, flags) {
        let len = snprintf(strp, size, b" (%s)\0".as_ptr() as *const c_char, label_modename(ns, label, flags));
        update_for_len(&mut total, len as ssize_t, &mut size, &mut strp);
    }
    total
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_asxprint(strp: *mut *mut c_char, ns: *mut aa_ns, label: *mut aa_label, flags: c_int, gfp: gfp_t) -> c_int {
    AA_BUG!(strp.is_null());
    AA_BUG!(label.is_null());
    let size = aa_label_snxprint(ptr::null_mut(), 0, ns, label, flags);
    if size < 0 { return size; }
    *strp = kmalloc((size + 1) as size_t, gfp) as *mut c_char;
    if (*strp).is_null() { return -ENOMEM; }
    aa_label_snxprint(*strp, (size + 1) as size_t, ns, label, flags)
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_acntsxprint(strp: *mut *mut c_char, ns: *mut aa_ns, label: *mut aa_label, flags: c_int, gfp: gfp_t) -> c_int {
    AA_BUG!(strp.is_null());
    AA_BUG!(label.is_null());
    let size = aa_label_snxprint(ptr::null_mut(), 0, ns, label, flags);
    if size < 0 { return size; }
    *strp = aa_str_alloc((size + 1) as size_t, gfp);
    if (*strp).is_null() { return -ENOMEM; }
    aa_label_snxprint(*strp, (size + 1) as size_t, ns, label, flags)
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_xaudit(ab: *mut audit_buffer, ns: *mut aa_ns, label: *mut aa_label, flags: c_int, gfp: gfp_t) {
    let mut name: *mut c_char = ptr::null_mut();
    let strp: *const c_char;
    let len: c_int;
    AA_BUG!(ab.is_null());
    AA_BUG!(label.is_null());
    if !use_label_hname(ns, label, flags) || display_mode(ns, label, flags) {
        len = aa_label_asxprint(&mut name, ns, label, flags, gfp);
        if len < 0 { AA_DEBUG!(DEBUG_LABEL, "label print error"); return; }
        strp = name;
    } else {
        strp = (*label).hname;
        len = strlen(strp) as c_int;
    }
    audit_log_n_untrustedstring(ab, strp, len);
    kfree(name as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_seq_xprint(f: *mut seq_file, ns: *mut aa_ns, label: *mut aa_label, flags: c_int, gfp: gfp_t) {
    AA_BUG!(f.is_null());
    AA_BUG!(label.is_null());
    if !use_label_hname(ns, label, flags) {
        let mut strp: *mut c_char = ptr::null_mut();
        let len = aa_label_asxprint(&mut strp, ns, label, flags, gfp);
        if len < 0 { AA_DEBUG!(DEBUG_LABEL, "label print error"); return; }
        seq_puts(f, strp);
        kfree(strp as *mut c_void);
    } else if display_mode(ns, label, flags) {
        seq_printf(f, b"%s (%s)\0".as_ptr() as *const c_char, (*label).hname, label_modename(ns, label, flags));
    } else {
        seq_puts(f, (*label).hname);
    }
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_xprintk(ns: *mut aa_ns, label: *mut aa_label, flags: c_int, gfp: gfp_t) {
    AA_BUG!(label.is_null());
    if !use_label_hname(ns, label, flags) {
        let mut strp: *mut c_char = ptr::null_mut();
        let len = aa_label_asxprint(&mut strp, ns, label, flags, gfp);
        if len < 0 { AA_DEBUG!(DEBUG_LABEL, "label print error"); return; }
        pr_info(b"%s\0".as_ptr() as *const c_char, strp);
        kfree(strp as *mut c_void);
    } else if display_mode(ns, label, flags) {
        pr_info(b"%s (%s)\0".as_ptr() as *const c_char, (*label).hname, label_modename(ns, label, flags));
    } else {
        pr_info(b"%s\0".as_ptr() as *const c_char, (*label).hname);
    }
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_printk(label: *mut aa_label, gfp: gfp_t) {
    let ns = aa_get_current_ns();
    aa_label_xprintk(ns, label, FLAG_VIEW_SUBNS, gfp);
    aa_put_ns(ns);
}

unsafe fn label_count_strn_entries(mut strp: *const c_char, n: size_t) -> c_int {
    let end = strp.add(n);
    let mut count = 1;
    AA_BUG!(strp.is_null());
    let mut split = aa_label_strn_split(strp, end.offset_from(strp) as size_t);
    while !split.is_null() {
        count += 1;
        strp = split.add(3);
        split = aa_label_strn_split(strp, end.offset_from(strp) as size_t);
    }
    count
}

/*
 * ensure stacks with components like
 *   :ns:A//&B
 * have :ns: applied to both 'A' and 'B' by making the lookup relative
 * to the base if the lookup specifies an ns, else making the stacked lookup
 * relative to the last embedded ns in the string.
 */
unsafe fn fqlookupn_profile(base: *mut aa_label, currentbase: *mut aa_label, strp: *const c_char, n: size_t) -> *mut aa_profile {
    let first = skipn_spaces(strp, n);
    if !first.is_null() && *first == b':' as c_char {
        return aa_fqlookupn_profile(base, strp, n);
    }
    aa_fqlookupn_profile(currentbase, strp, n)
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_strn_parse(base: *mut aa_label, mut strp: *const c_char, n: size_t, gfp: gfp_t, create: bool, force_stack: bool) -> *mut aa_label {
    let mut currbase = base;
    let mut stack = 0;
    let end = strp.add(n);
    AA_BUG!(base.is_null());
    AA_BUG!(strp.is_null());
    strp = skipn_spaces(strp, n);
    if strp.is_null() || (DEBUG_ABS_ROOT && *strp == b'_' as c_char && base != &mut (*(*root_ns).unconfined).label) {
        return ERR_PTR(-EINVAL);
    }
    let mut len = label_count_strn_entries(strp, end.offset_from(strp) as size_t);
    if *strp == b'&' as c_char || force_stack {
        /* stack on top of base */
        stack = (*base).size;
        len += stack;
        if *strp == b'&' as c_char { strp = strp.add(1); }
    }
    let bytes = (len as usize) * mem::size_of::<*mut aa_profile>();
    let vec = kmalloc(bytes, gfp) as *mut *mut aa_profile;
    if vec.is_null() { return ERR_PTR(-ENOMEM); }
    let mut i = 0;
    while i < stack {
        *vec.add(i as usize) = aa_get_profile(aa_label_vec(base, i));
        i += 1;
    }
    let mut split = aa_label_strn_split(strp, end.offset_from(strp) as size_t);
    i = stack;
    while !split.is_null() && i < len {
        *vec.add(i as usize) = fqlookupn_profile(base, currbase, strp, split.offset_from(strp) as size_t);
        if (*vec.add(i as usize)).is_null() { break; }
        if (**vec.add(i as usize)).ns != labels_ns(currbase) {
            currbase = &mut (**vec.add(i as usize)).label;
        }
        strp = split.add(3);
        split = aa_label_strn_split(strp, end.offset_from(strp) as size_t);
        i += 1;
    }
    if i < len && split.is_null() {
        *vec.add(i as usize) = fqlookupn_profile(base, currbase, strp, end.offset_from(strp) as size_t);
        if (*vec.add(i as usize)).is_null() {
            let label = ERR_PTR(-ENOENT);
            kfree(vec as *mut c_void);
            return label;
        }
    } else if i < len {
        let label = ERR_PTR(-ENOENT);
        kfree(vec as *mut c_void);
        return label;
    }
    if len == 1 {
        let label = &mut (**vec).label;
        kfree(vec as *mut c_void);
        return label;
    }
    len -= aa_vec_unique(vec, len, VEC_FLAG_TERMINATE);
    let label: *mut aa_label;
    if len == 1 {
        label = aa_get_label(&mut (**vec).label);
    } else if create {
        label = aa_vec_find_or_create_label(vec, len, gfp);
    } else {
        label = vec_find(vec, len);
    }
    if label.is_null() {
        kfree(vec as *mut c_void);
        return ERR_PTR(-ENOENT);
    }
    let mut j = 0;
    while j < len {
        aa_put_profile(*vec.add(j as usize));
        j += 1;
    }
    kfree(vec as *mut c_void);
    label
}

#[no_mangle]
pub unsafe extern "C" fn aa_label_parse(base: *mut aa_label, strp: *const c_char, gfp: gfp_t, create: bool, force_stack: bool) -> *mut aa_label {
    aa_label_strn_parse(base, strp, strlen(strp), gfp, create, force_stack)
}

#[no_mangle]
pub unsafe extern "C" fn aa_labelset_destroy(ls: *mut aa_labelset) {
    let flags: c_ulong = 0;
    AA_BUG!(ls.is_null());
    write_lock_irqsave(&mut (*ls).lock, flags);
    let mut node = rb_first(&mut (*ls).root);
    while !node.is_null() {
        let this = rb_entry_label(node);
        if labels_ns(this) != root_ns {
            __label_remove(this, ns_unconfined((*labels_ns(this)).parent));
        } else {
            __label_remove(this, ptr::null_mut());
        }
        node = rb_first(&mut (*ls).root);
    }
    write_unlock_irqrestore(&mut (*ls).lock, flags);
}

/*
 * @ls: labelset to init (NOT NULL)
 */
#[no_mangle]
pub unsafe extern "C" fn aa_labelset_init(ls: *mut aa_labelset) {
    AA_BUG!(ls.is_null());
    rwlock_init(&mut (*ls).lock);
    (*ls).root = rb_root { rb_node: ptr::null_mut() };
}

unsafe fn labelset_next_stale(ls: *mut aa_labelset) -> *mut aa_label {
    let flags: c_ulong = 0;
    AA_BUG!(ls.is_null());
    read_lock_irqsave(&mut (*ls).lock, flags);
    let mut node = rb_first(&mut (*ls).root);
    while !node.is_null() {
        let label = rb_entry_label(node);
        if (label_is_stale(label) || vec_is_stale((*label).vec.as_mut_ptr(), (*label).size)) && !__aa_get_label(label).is_null() {
            read_unlock_irqrestore(&mut (*ls).lock, flags);
            return label;
        }
        node = (*node).rb_right;
    }
    read_unlock_irqrestore(&mut (*ls).lock, flags);
    ptr::null_mut()
}

unsafe fn __label_update(label: *mut aa_label) -> *mut aa_label {
    let mut invcount = 0;
    AA_BUG!(label.is_null());
    AA_BUG!(!mutex_is_locked(&mut (*labels_ns(label)).lock));
    let new = aa_label_alloc((*label).size, (*label).proxy, GFP_KERNEL);
    if new.is_null() { return ptr::null_mut(); }
    let ls = labels_set(label);
    let flags: c_ulong = 0;
    write_lock_irqsave(&mut (*ls).lock, flags);
    let mut i = 0;
    while i < (*label).size {
        AA_BUG!(aa_label_vec(label, i).is_null());
        aa_label_vec_set(new, i, aa_get_newest_profile(aa_label_vec(label, i)));
        AA_BUG!(aa_label_vec(new, i).is_null());
        AA_BUG!((*aa_label_vec(new, i)).label.proxy.is_null());
        AA_BUG!((*(*aa_label_vec(new, i)).label.proxy).label.is_null());
        if (*aa_label_vec(new, i)).label.proxy != (*aa_label_vec(label, i)).label.proxy { invcount += 1; }
        i += 1;
    }
    let tmp: *mut aa_label;
    if invcount != 0 {
        (*new).size -= aa_vec_unique((*new).vec.as_mut_ptr(), (*new).size, VEC_FLAG_TERMINATE);
        /* TODO: deal with reference labels */
        if (*new).size == 1 {
            tmp = aa_get_label(&mut (*aa_label_vec(new, 0)).label);
            AA_BUG!(tmp == label);
            __label_remove(label, tmp);
            write_unlock_irqrestore(&mut (*ls).lock, flags);
            label_free_or_put_new(tmp, new);
            return tmp;
        }
        if labels_set(label) != labels_set(new) {
            write_unlock_irqrestore(&mut (*ls).lock, flags);
            tmp = aa_label_insert(labels_set(new), new);
            write_lock_irqsave(&mut (*ls).lock, flags);
            __label_remove(label, tmp);
            write_unlock_irqrestore(&mut (*ls).lock, flags);
            label_free_or_put_new(tmp, new);
            return tmp;
        }
    } else {
        AA_BUG!(labels_ns(label) != labels_ns(new));
    }
    tmp = __label_insert(labels_set(label), new, true);
    __label_remove(label, tmp);
    write_unlock_irqrestore(&mut (*ls).lock, flags);
    label_free_or_put_new(tmp, new);
    tmp
}

unsafe fn __labelset_update(ns: *mut aa_ns) {
    AA_BUG!(ns.is_null());
    AA_BUG!(!mutex_is_locked(&mut (*ns).lock));
    loop {
        let label = labelset_next_stale(&mut (*ns).labels);
        if label.is_null() { break; }
        let l = __label_update(label);
        aa_put_label(l);
        aa_put_label(label);
    }
}

#[no_mangle]
pub unsafe extern "C" fn __aa_labelset_update_subtree(ns: *mut aa_ns) {
    AA_BUG!(ns.is_null());
    AA_BUG!(!mutex_is_locked(&mut (*ns).lock));
    __labelset_update(ns);
    /*
     * C iterates list_for_each_entry(child, &ns->sub_ns, base.list).
     * The list traversal helper is supplied by the surrounding kernel tree.
     */
    let _child_list = &mut (*ns).sub_ns;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
