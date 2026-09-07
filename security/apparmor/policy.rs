// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor policy manipulation functions
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 *
 * AppArmor policy is based around profiles, which contain the rules a
 * task is confined by.  Every task in the system has a profile attached
 * to it determined either by matching "unconfined" tasks against the
 * visible set of profiles or by following a profiles attachment rules.
 *
 * Each profile exists in a profile namespace which is a container of
 * visible profiles.  Each namespace contains a special "unconfined" profile,
 * which doesn't enforce any confinement on a task beyond DAC.
 *
 * Namespace and profile names can be written together in either
 * of two syntaxes.
 *      :namespace:profile - used by kernel interfaces for easy detection
 *      namespace://profile - used by policy
 *
 * Profile names can not start with : or @ or ^ and may not contain \0
 *
 * Reserved profile names
 *      unconfined - special automatically generated unconfined profile
 *      inherit - special name to indicate profile inheritance
 *      null-XXXX-YYYY - special automatically generated learning profiles
 *
 * Namespace names may not start with / or @ and may not contain \0 or :
 * Reserved namespace names
 *      user-XXXX - user defined profiles
 *
 * a // in a profile or namespace name indicates a hierarchical name with the
 * name before the // being the parent and the name after the child.
 *
 * Profile and namespace hierarchies serve two different but similar purposes.
 * The namespace contains the set of visible profiles that are considered
 * for attachment.  The hierarchy of namespaces allows for virtualizing
 * the namespace so that for example a chroot can have its own set of profiles
 * which may define some local user namespaces.
 * The profile hierarchy severs two distinct purposes,
 * -  it allows for sub profiles or hats, which allows an application to run
 *    subprograms under its own profile with different restriction than it
 *    self, and not have it use the system profile.
 *    eg. if a mail program starts an editor, the policy might make the
 *        restrictions tighter on the editor tighter than the mail program,
 *        and definitely different than general editor restrictions
 * - it allows for binary hierarchy of profiles, so that execution history
 *   is preserved.  This feature isn't exploited by AppArmor reference policy
 *   but is allowed.  NOTE: this is currently suboptimal because profile
 *   aliasing is not currently implemented so that a profile for each
 *   level must be defined.
 *   eg. /bin/bash///bin/ls as a name would indicate /bin/ls was started
 *       from /bin/bash
 *
 *   A profile or namespace name that can contain one or more // separators
 *   is referred to as an hname (hierarchical).
 *   eg.  /bin/bash//bin/ls
 *
 *   An fqname is a name that may contain both namespace and profile hnames.
 *   eg. :ns:/bin/bash//bin/ls
 *
 * NOTES:
 *   - locking of profile lists is currently fairly coarse.  All profile
 *     lists within a namespace use the namespace lock.
 * FIXME: move profile lists to using rcu_lists
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr;

pub type bool_ = bool;
pub type gfp_t = u32;
pub type ssize_t = isize;
pub type size_t = usize;
pub type u32_ = u32;
pub type u64_ = u64;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct kref {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rhashtable {
    _private: [u8; 0],
}

#[repr(C)]
pub struct audit_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_namespace {
    pub level: c_int,
}

#[repr(C)]
pub struct cred {
    pub user_ns: *mut user_namespace,
    pub cap_inheritable: kernel_cap_t,
    pub cap_permitted: kernel_cap_t,
    pub cap_effective: kernel_cap_t,
    pub cap_bset: kernel_cap_t,
    pub cap_ambient: kernel_cap_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_cap_t {
    pub cap: [u32; 2],
}

#[repr(C)]
pub struct aa_str_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aa_table {
    pub table: *mut c_void,
}

#[repr(C)]
pub struct aa_tags_struct {
    pub hdrs: aa_table,
    pub sets: aa_table,
    pub strs: aa_str_table,
}

#[repr(C)]
pub struct aa_policydb {
    pub count: kref,
    pub dfa: *mut c_void,
    pub perms: *mut c_void,
    pub trans: aa_str_table,
    pub tags: aa_tags_struct,
}

#[repr(C)]
pub struct aa_policy {
    pub list: list_head,
    pub profiles: list_head,
    pub hname: *const c_char,
    pub name: *const c_char,
}

#[repr(C)]
pub struct aa_proxy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aa_attachment {
    pub xattr_count: c_int,
    pub xattrs: *mut *mut c_char,
    pub xmatch: *mut aa_policydb,
}

#[repr(C)]
pub struct aa_cap_rules {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aa_rlimit_rules {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aa_secmark {
    pub label: *mut c_char,
}

#[repr(C)]
pub struct aa_ruleset {
    pub file: *mut aa_policydb,
    pub policy: *mut aa_policydb,
    pub caps: aa_cap_rules,
    pub rlimits: aa_rlimit_rules,
    pub secmark_count: c_int,
    pub secmark: *mut aa_secmark,
}

#[repr(C)]
pub struct aa_label {
    pub proxy: *mut aa_proxy,
    pub hname: *const c_char,
    pub flags: u64,
    pub vec: [*mut aa_profile; 1],
    pub mediates: u64,
    pub rules: [*mut aa_ruleset; 1],
}

#[repr(C)]
pub struct aa_ns {
    pub base: aa_policy,
    pub labels: c_void,
    pub lock: mutex,
    pub level: c_int,
    pub unconfined: *mut aa_profile,
    pub uniq_null: c_int,
    pub rawdata_list: list_head,
    pub revision: u64,
    pub parent: *mut aa_ns,
}

#[repr(C)]
pub struct aa_profile {
    pub base: aa_policy,
    pub ns: *mut aa_ns,
    pub label: aa_label,
    pub parent: *mut aa_profile,
    pub rawdata: *mut aa_loaddata,
    pub rename: *mut c_char,
    pub disconnected: *mut c_char,
    pub attach: aa_attachment,
    pub n_rules: c_int,
    pub dirname: *mut c_char,
    pub data: *mut rhashtable,
    pub hash: *mut c_void,
    pub signal: c_int,
    pub path_flags: u32,
    pub mode: c_int,
    pub dents: [*mut dentry; 1],
}

#[repr(C)]
pub struct aa_data {
    pub data: *mut c_void,
    pub size: size_t,
    pub key: *mut c_char,
}

#[repr(C)]
pub struct aa_load_ent {
    pub list: list_head,
    pub new: *mut aa_profile,
    pub old: *mut aa_profile,
    pub rename: *mut aa_profile,
    pub ns_name: *mut c_char,
}

#[repr(C)]
pub struct aa_loaddata {
    pub list: list_head,
    pub size: ssize_t,
    pub dents: [*mut dentry; 1],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct common_audit_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct apparmor_audit_iface {
    pub ns: *const c_char,
}

#[repr(C)]
pub struct apparmor_audit_data {
    pub iface: apparmor_audit_iface,
    pub name: *const c_char,
    pub info: *const c_char,
    pub error: c_int,
    pub subj_label: *mut aa_label,
}

pub const SIGKILL: c_int = 9;
pub const APPARMOR_COMPLAIN: c_int = 1;
pub const FLAG_PROFILE: u64 = 1 << 0;
pub const FLAG_NULL: u64 = 1 << 1;
pub const FLAG_HAT: u64 = 1 << 2;
pub const FLAG_IMMUTIBLE: u64 = 1 << 3;
pub const AA_CLASS_NONE: c_int = 0;
pub const AA_CLASS_NS: c_int = 1;
pub const AA_CLASS_IO_URING: c_int = 2;
pub const AA_CLASS_LAST: c_int = 63;
pub const AA_MAY_REMOVE_POLICY: u32 = 1 << 0;
pub const AA_MAY_REPLACE_POLICY: u32 = 1 << 1;
pub const CAP_MAC_ADMIN: c_int = 33;
pub const CAP_OPT_NONE: c_int = 0;
pub const AUDIT_APPARMOR_STATUS: c_int = 1400;
pub const LSM_AUDIT_DATA_NONE: c_int = 0;
pub const AAFS_LOADDATA_DIR: usize = 0;
pub const GFP_KERNEL: gfp_t = 0;
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const EACCES: c_int = 13;
pub const EEXIST: c_int = 17;

static OP_PROF_RM: &[u8] = b"profile_remove\0";
static OP_PROF_REPL: &[u8] = b"profile_replace\0";
static OP_PROF_LOAD: &[u8] = b"profile_load\0";

#[no_mangle]
pub static mut unprivileged_userns_apparmor_policy: c_int = 1;
#[no_mangle]
pub static mut aa_unprivileged_unconfined_restricted: c_int = 0;

#[no_mangle]
pub static aa_profile_mode_names: [*const c_char; 5] = [
    b"enforce\0".as_ptr() as *const c_char,
    b"complain\0".as_ptr() as *const c_char,
    b"kill\0".as_ptr() as *const c_char,
    b"unconfined\0".as_ptr() as *const c_char,
    b"user\0".as_ptr() as *const c_char,
];

extern "C" {
    static mut init_user_ns: user_namespace;
    static mut aa_g_lock_policy: bool;
    static mut aa_g_export_binary: bool;
    static mut nullpdb: *mut aa_policydb;

    fn kfree_sensitive(ptr: *mut c_void);
    fn kvfree(ptr: *mut c_void);
    fn kvfree_sensitive(ptr: *mut c_void, size: size_t);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strnstr(haystack: *const c_char, needle: *const c_char, len: size_t) -> *mut c_char;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn kmalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kstrndup(s: *const c_char, max: size_t, flags: gfp_t) -> *mut c_char;
    fn basename(path: *const c_char) -> *const c_char;

    fn kref_init(kref: *mut kref);
    fn kzalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn aa_destroy_str_table(table: *mut aa_str_table);
    fn aa_put_dfa(dfa: *mut c_void);
    fn aa_get_profile(profile: *mut aa_profile) -> *mut aa_profile;
    fn aa_get_profile_not0(profile: *mut aa_profile) -> bool;
    fn aa_put_profile(profile: *mut aa_profile);
    fn aa_policy_init(policy: *mut aa_policy, parent: *mut aa_policy, hname: *const c_char, gfp: gfp_t) -> bool;
    fn aa_policy_destroy(policy: *mut aa_policy);
    fn aa_label_init(label: *mut aa_label, size: c_int, gfp: gfp_t) -> bool;
    fn aa_label_destroy(label: *mut aa_label);
    fn aa_label_insert(set: *mut c_void, label: *mut aa_label) -> *mut aa_label;
    fn aa_label_remove(label: *mut aa_label);
    fn aa_label_replace(old: *mut aa_label, new: *mut aa_label);
    fn aa_put_label(label: *mut aa_label);
    fn aa_alloc_proxy(label: *mut aa_label, gfp: gfp_t) -> *mut aa_proxy;
    fn aa_get_proxy(proxy: *mut aa_proxy) -> *mut aa_proxy;
    fn aa_put_proxy(proxy: *mut aa_proxy);
    fn aa_put_ns(ns: *mut aa_ns);
    fn aa_get_ns(ns: *mut aa_ns) -> *mut aa_ns;
    fn aa_prepare_ns(ns: *mut aa_ns, name: *const c_char) -> *mut aa_ns;
    fn aa_lookupn_ns(ns: *mut aa_ns, name: *const c_char, n: size_t) -> *mut aa_ns;
    fn aa_ns_visible(view: *mut aa_ns, ns: *mut aa_ns, subns: bool) -> bool;
    fn labels_ns(label: *mut aa_label) -> *mut aa_ns;
    fn labels_view(label: *mut aa_label) -> *mut aa_ns;
    fn aa_get_newest_profile(profile: *mut aa_profile) -> *mut aa_profile;
    fn aa_deref_parent(profile: *mut aa_profile) -> *mut aa_profile;
    fn aa_get_pdb(pdb: *mut aa_policydb) -> *mut aa_policydb;
    fn aa_put_pdb(pdb: *mut aa_policydb);
    fn aa_free_cap_rules(rules: *mut aa_cap_rules);
    fn aa_free_rlimit_rules(rules: *mut aa_rlimit_rules);
    fn rhashtable_free_and_destroy(rht: *mut rhashtable, free_fn: unsafe extern "C" fn(*mut c_void, *mut c_void), arg: *mut c_void);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn mutex_is_locked(lock: *mut mutex) -> bool;
    fn mutex_lock_nested(lock: *mut mutex, subclass: c_int);
    fn mutex_unlock(lock: *mut mutex);
    fn list_add_rcu(new: *mut list_head, head: *mut list_head);
    fn list_del_rcu(entry: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn list_replace_rcu(old: *mut list_head, new: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn list_is_last(list: *const list_head, head: *const list_head) -> bool;
    fn list_splice_init_rcu(list: *mut list_head, head: *mut list_head, sync: unsafe extern "C" fn());
    fn synchronize_rcu();
    fn __policy_strn_find(head: *mut list_head, name: *const c_char, len: c_int) -> *mut c_void;
    fn __aafs_profile_rmdir(profile: *mut aa_profile);
    fn __aafs_profile_migrate_dents(old: *mut aa_profile, new: *mut aa_profile);
    fn __aafs_profile_mkdir(profile: *mut aa_profile, parent: *mut dentry) -> ssize_t;
    fn prof_child_dir(profile: *mut aa_profile) -> *mut dentry;
    fn ns_subprofs_dir(ns: *mut aa_ns) -> *mut dentry;
    fn aa_put_profile_loaddata(data: *mut aa_loaddata);
    fn aa_get_profile_loaddata(data: *mut aa_loaddata) -> *mut aa_loaddata;
    fn aa_get_profile_loaddata_not0(data: *mut aa_loaddata) -> *mut aa_loaddata;
    fn aa_rawdata_eq(a: *mut aa_loaddata, b: *mut aa_loaddata) -> bool;
    fn __aa_fs_create_rawdata(ns: *mut aa_ns, data: *mut aa_loaddata) -> ssize_t;
    fn __aa_loaddata_update(data: *mut aa_loaddata, revision: u64);
    fn __aa_remove_rawdata_symlink_dents(profile: *mut aa_profile);
    fn __aa_create_rawdata_symlink_dents(profile: *mut aa_profile);
    fn aa_load_ent_free(ent: *mut aa_load_ent);
    fn aa_unpack(data: *mut aa_loaddata, lh: *mut list_head, ns_name: *mut *const c_char, compressed: *mut c_char, compressed_size: size_t) -> ssize_t;
    fn __aa_bump_ns_revision(ns: *mut aa_ns);
    fn __aa_labelset_update_subtree(ns: *mut aa_ns);
    fn __aa_remove_ns(ns: *mut aa_ns);
    fn aa_splitn_fqname(fqname: *const c_char, n: size_t, ns_name: *mut *const c_char, ns_len: *mut size_t) -> *const c_char;
    fn profile_unconfined(profile: *mut aa_profile) -> bool;
    fn RULE_MEDIATES(rules: *mut aa_ruleset, class: u8) -> bool;
    fn current_euid() -> kuid_t;
    fn current_cred() -> *const cred;
    fn make_kuid(ns: *mut user_namespace, uid: u32) -> kuid_t;
    fn make_kgid(ns: *mut user_namespace, gid: u32) -> kgid_t;
    fn uid_eq(a: kuid_t, b: kuid_t) -> bool;
    fn in_egroup_p(gid: kgid_t) -> bool;
    fn cap_capable(cred: *const cred, ns: *mut user_namespace, cap: c_int, opts: c_int) -> c_int;
    fn aa_capable(cred: *const cred, label: *mut aa_label, cap: c_int, opts: c_int) -> c_int;
    fn aa_label_is_subset(label: *mut aa_label, other: *mut aa_label) -> bool;
    fn cred_label(cred: *const cred) -> *mut aa_label;
    fn cap_issubset(a: kernel_cap_t, b: kernel_cap_t) -> bool;
    fn __begin_current_label_crit_section(needput: *mut bool) -> *mut aa_label;
    fn __end_current_label_crit_section(label: *mut aa_label, needput: bool);
    fn aad(sa: *mut common_audit_data) -> *mut apparmor_audit_data;
    fn aa_audit_msg(typ: c_int, ad: *mut apparmor_audit_data, cb: unsafe extern "C" fn(*mut audit_buffer, *mut c_void));
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn audit_log_untrustedstring(ab: *mut audit_buffer, s: *const c_char);
    fn aa_put_str(s: *const c_char);
    fn aa_get_str(s: *const c_char) -> *const c_char;
    fn atomic_inc_return(v: *mut c_int) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kuid_t {
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kgid_t {
    pub val: u32,
}

unsafe fn AA_BUG(cond: bool) {
    if cond {
        core::hint::unreachable_unchecked();
    }
}

unsafe fn kzalloc_obj<T>(gfp: gfp_t) -> *mut T {
    kzalloc(size_of::<T>(), gfp) as *mut T
}

unsafe fn kzalloc_flex_profile(_rules: usize, gfp: gfp_t) -> *mut aa_profile {
    kzalloc(size_of::<aa_profile>(), gfp) as *mut aa_profile
}

unsafe fn ptr_offset<T>(a: *const T, b: *const T) -> isize {
    (a as isize - b as isize) / size_of::<T>() as isize
}

#[no_mangle]
pub unsafe extern "C" fn aa_destroy_tags(tags: *mut aa_tags_struct) {
    kfree_sensitive((*tags).hdrs.table);
    kfree_sensitive((*tags).sets.table);
    aa_destroy_str_table(&mut (*tags).strs);
    memset(tags as *mut c_void, 0, size_of::<aa_tags_struct>());
}

unsafe fn aa_free_pdb(pdb: *mut aa_policydb) {
    if !pdb.is_null() {
        aa_put_dfa((*pdb).dfa);
        kvfree((*pdb).perms);
        aa_destroy_str_table(&mut (*pdb).trans);
        aa_destroy_tags(&mut (*pdb).tags);
        kfree_sensitive(pdb as *mut c_void);
    }
}

/**
 * aa_pdb_free_kref - free aa_policydb by kref (called by aa_put_pdb)
 * @kref: kref callback for freeing of a dfa  (NOT NULL)
 */
#[no_mangle]
pub unsafe extern "C" fn aa_pdb_free_kref(kref: *mut kref) {
    let pdb = kref as *mut aa_policydb;
    aa_free_pdb(pdb);
}

#[no_mangle]
pub unsafe extern "C" fn aa_alloc_pdb(gfp: gfp_t) -> *mut aa_policydb {
    let pdb = kzalloc_obj::<aa_policydb>(gfp);
    if pdb.is_null() {
        return ptr::null_mut();
    }
    kref_init(&mut (*pdb).count);
    pdb
}

/**
 * __add_profile - add a profiles to list and label tree
 * @list: list to add it to  (NOT NULL)
 * @profile: the profile to add  (NOT NULL)
 *
 * refcount @profile, should be put by __list_remove_profile
 *
 * Requires: namespace lock be held, or list not be shared
 */
unsafe fn __add_profile(list: *mut list_head, profile: *mut aa_profile) {
    let l: *mut aa_label;
    AA_BUG(list.is_null());
    AA_BUG(profile.is_null());
    AA_BUG((*profile).ns.is_null());
    AA_BUG(!mutex_is_locked(&mut (*(*profile).ns).lock));

    list_add_rcu(&mut (*profile).base.list, list);
    aa_get_profile(profile);
    l = aa_label_insert(&mut (*(*profile).ns).labels, &mut (*profile).label);
    AA_BUG(l != &mut (*profile).label);
    aa_put_label(l);
}

/**
 * __list_remove_profile - remove a profile from the list it is on
 * @profile: the profile to remove  (NOT NULL)
 *
 * remove a profile from the list, warning generally removal should
 * be done with __replace_profile as most profile removals are
 * replacements to the unconfined profile.
 *
 * put @profile list refcount
 *
 * Requires: namespace lock be held, or list not have been live
 */
unsafe fn __list_remove_profile(profile: *mut aa_profile) {
    AA_BUG(profile.is_null());
    AA_BUG((*profile).ns.is_null());
    AA_BUG(!mutex_is_locked(&mut (*(*profile).ns).lock));

    list_del_rcu(&mut (*profile).base.list);
    aa_put_profile(profile);
}

unsafe fn first_profile(head: *mut list_head) -> *mut aa_profile {
    (*head).next as *mut aa_profile
}

unsafe fn next_profile(profile: *mut aa_profile) -> *mut aa_profile {
    (*profile).base.list.next as *mut aa_profile
}

unsafe fn first_load_ent(head: *mut list_head) -> *mut aa_load_ent {
    (*head).next as *mut aa_load_ent
}

unsafe fn next_load_ent(ent: *mut aa_load_ent) -> *mut aa_load_ent {
    (*ent).list.next as *mut aa_load_ent
}

/**
 * __remove_profile - remove profile, and children
 * @profile: profile to be removed  (NOT NULL)
 *
 * Requires: namespace list lock be held, or list not be shared
 */
unsafe fn __remove_profile(profile: *mut aa_profile) {
    let mut curr: *mut aa_profile;
    let mut to_remove: *mut aa_profile;

    AA_BUG(profile.is_null());
    AA_BUG((*profile).ns.is_null());
    AA_BUG(!mutex_is_locked(&mut (*(*profile).ns).lock));

    if !list_empty(&(*profile).base.profiles) {
        curr = first_profile(&mut (*profile).base.profiles);
        while curr != profile {
            while !list_empty(&(*curr).base.profiles) {
                curr = first_profile(&mut (*curr).base.profiles);
            }
            to_remove = curr;
            if !list_is_last(&(*to_remove).base.list, &(*aa_deref_parent(curr)).base.profiles) {
                curr = next_profile(to_remove);
            } else {
                curr = aa_deref_parent(curr);
            }
            aa_label_remove(&mut (*to_remove).label);
            __aafs_profile_rmdir(to_remove);
            __list_remove_profile(to_remove);
        }
    }

    aa_label_remove(&mut (*profile).label);
    __aafs_profile_rmdir(profile);
    __list_remove_profile(profile);
    /*
     * rawdata is only ever referenced by fs lookup, that is no
     * longer possible here, so put the reference to it. This will
     * enable the rawdata to be freed if for some reason the profile
     * is pinned and going to live for a while.
     */
    aa_put_profile_loaddata((*profile).rawdata);
    (*profile).rawdata = ptr::null_mut();
}

/**
 * __aa_profile_list_release - remove all profiles on the list and put refs
 * @head: list of profiles  (NOT NULL)
 *
 * Requires: namespace lock be held
 */
#[no_mangle]
pub unsafe extern "C" fn __aa_profile_list_release(head: *mut list_head) {
    let mut profile = first_profile(head);
    while &mut (*profile).base.list as *mut list_head != head {
        let tmp = next_profile(profile);
        __remove_profile(profile);
        profile = tmp;
    }
}

/**
 * aa_free_data - free a data blob
 * @ptr: data to free
 * @arg: unused
 */
unsafe extern "C" fn aa_free_data(ptr: *mut c_void, _arg: *mut c_void) {
    let data = ptr as *mut aa_data;
    if ptr.is_null() {
        return;
    }
    kvfree_sensitive((*data).data, (*data).size);
    kfree_sensitive((*data).key as *mut c_void);
    kfree_sensitive(data as *mut c_void);
}

unsafe fn free_attachment(attach: *mut aa_attachment) {
    if attach.is_null() {
        return;
    }
    for i in 0..(*attach).xattr_count {
        kfree_sensitive(*(*attach).xattrs.add(i as usize) as *mut c_void);
    }
    kfree_sensitive((*attach).xattrs as *mut c_void);
    aa_put_pdb((*attach).xmatch);
}

unsafe fn free_ruleset(rules: *mut aa_ruleset) {
    if rules.is_null() {
        return;
    }
    aa_put_pdb((*rules).file);
    aa_put_pdb((*rules).policy);
    aa_free_cap_rules(&mut (*rules).caps);
    aa_free_rlimit_rules(&mut (*rules).rlimits);

    for i in 0..(*rules).secmark_count {
        kfree_sensitive((*(*rules).secmark.add(i as usize)).label as *mut c_void);
    }
    kfree_sensitive((*rules).secmark as *mut c_void);
    kfree_sensitive(rules as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn aa_alloc_ruleset(gfp: gfp_t) -> *mut aa_ruleset {
    kzalloc_obj::<aa_ruleset>(gfp)
}

/**
 * aa_free_profile - free a profile
 * @profile: the profile to free  (MAYBE NULL)
 *
 * Free a profile, its hats and null_profile. All references to the profile,
 * its hats and null_profile must have been put.
 *
 * If the profile was referenced from a task context, free_profile() will
 * be called from an rcu callback routine, so we must not sleep here.
 */
#[no_mangle]
pub unsafe extern "C" fn aa_free_profile(profile: *mut aa_profile) {
    if profile.is_null() {
        return;
    }

    aa_policy_destroy(&mut (*profile).base);
    aa_put_profile((*profile).parent);

    aa_put_ns((*profile).ns);
    kfree_sensitive((*profile).rename as *mut c_void);
    kfree_sensitive((*profile).disconnected as *mut c_void);

    free_attachment(&mut (*profile).attach);

    for i in 0..(*profile).n_rules {
        free_ruleset((*profile).label.rules[i as usize]);
    }

    kfree_sensitive((*profile).dirname as *mut c_void);

    if !(*profile).data.is_null() {
        let rht = (*profile).data;
        (*profile).data = ptr::null_mut();
        rhashtable_free_and_destroy(rht, aa_free_data, ptr::null_mut());
        kfree_sensitive(rht as *mut c_void);
    }

    kfree_sensitive((*profile).hash);
    aa_put_profile_loaddata((*profile).rawdata);
    aa_label_destroy(&mut (*profile).label);

    kfree_sensitive(profile as *mut c_void);
}

/**
 * aa_alloc_profile - allocate, initialize and return a new profile
 * @hname: name of the profile  (NOT NULL)
 * @proxy: proxy to use OR null if to allocate a new one
 * @gfp: allocation type
 *
 * Returns: refcount profile or NULL on failure
 */
#[no_mangle]
pub unsafe extern "C" fn aa_alloc_profile(hname: *const c_char, mut proxy: *mut aa_proxy, gfp: gfp_t) -> *mut aa_profile {
    let profile = kzalloc_flex_profile(1, gfp);
    if profile.is_null() {
        return ptr::null_mut();
    }

    if !aa_policy_init(&mut (*profile).base, ptr::null_mut(), hname, gfp) {
        goto_fail(profile);
        return ptr::null_mut();
    }
    if !aa_label_init(&mut (*profile).label, 1, gfp) {
        goto_fail(profile);
        return ptr::null_mut();
    }

    (*profile).label.rules[0] = aa_alloc_ruleset(gfp);
    if (*profile).label.rules[0].is_null() {
        goto_fail(profile);
        return ptr::null_mut();
    }
    (*profile).n_rules = 1;

    if proxy.is_null() {
        proxy = aa_alloc_proxy(&mut (*profile).label, gfp);
        if proxy.is_null() {
            goto_fail(profile);
            return ptr::null_mut();
        }
    } else {
        aa_get_proxy(proxy);
    }
    (*profile).label.proxy = proxy;

    (*profile).label.hname = (*profile).base.hname;
    (*profile).label.flags |= FLAG_PROFILE;
    (*profile).label.vec[0] = profile;

    (*profile).signal = SIGKILL;
    profile
}

unsafe fn goto_fail(profile: *mut aa_profile) {
    aa_free_profile(profile);
}

unsafe fn ANY_RULE_MEDIATES(profile: *mut aa_profile, class: u8) -> bool {
    for i in 0..(*profile).n_rules {
        if RULE_MEDIATES((*profile).label.rules[i as usize], class) {
            return true;
        }
    }
    false
}

static mut unconfined_mediates: [c_int; 3] = [AA_CLASS_NS, AA_CLASS_IO_URING, 0];

#[no_mangle]
pub unsafe extern "C" fn aa_compute_profile_mediates(profile: *mut aa_profile) {
    if profile_unconfined(profile) {
        let mut pos = unconfined_mediates.as_ptr();
        while *pos != 0 {
            if ANY_RULE_MEDIATES(profile, *pos as u8) {
                (*profile).label.mediates |= (1u64) << AA_CLASS_NS;
            }
            pos = pos.add(1);
        }
        return;
    }
    for c in 0..=AA_CLASS_LAST {
        if ANY_RULE_MEDIATES(profile, c as u8) {
            (*profile).label.mediates |= (1u64) << c;
        }
    }
}

/* TODO: profile accounting - setup in remove */

/**
 * __strn_find_child - find a profile on @head list using substring of @name
 */
unsafe fn __strn_find_child(head: *mut list_head, name: *const c_char, len: c_int) -> *mut aa_profile {
    __policy_strn_find(head, name, len) as *mut aa_profile
}

unsafe fn __find_child(head: *mut list_head, name: *const c_char) -> *mut aa_profile {
    __strn_find_child(head, name, strlen(name) as c_int)
}

#[no_mangle]
pub unsafe extern "C" fn aa_find_child(parent: *mut aa_profile, name: *const c_char) -> *mut aa_profile {
    let mut profile: *mut aa_profile;
    rcu_read_lock();
    loop {
        profile = __find_child(&mut (*parent).base.profiles, name);
        if profile.is_null() || aa_get_profile_not0(profile) {
            break;
        }
    }
    rcu_read_unlock();
    profile
}

unsafe fn __lookup_parent(ns: *mut aa_ns, mut hname: *const c_char) -> *mut aa_policy {
    let mut policy = &mut (*ns).base as *mut aa_policy;
    let mut profile: *mut aa_profile = ptr::null_mut();
    let mut split = strstr(hname, b"//\0".as_ptr() as *const c_char);

    while !split.is_null() {
        profile = __strn_find_child(&mut (*policy).profiles, hname, (split as isize - hname as isize) as c_int);
        if profile.is_null() {
            return ptr::null_mut();
        }
        policy = &mut (*profile).base;
        hname = split.add(2);
        split = strstr(hname, b"//\0".as_ptr() as *const c_char);
    }
    if profile.is_null() {
        &mut (*ns).base
    } else {
        &mut (*profile).base
    }
}

unsafe fn __create_missing_ancestors(ns: *mut aa_ns, mut hname: *const c_char, gfp: gfp_t) -> *mut aa_policy {
    let mut policy = &mut (*ns).base as *mut aa_policy;
    let mut parent: *mut aa_profile;
    let mut profile: *mut aa_profile = ptr::null_mut();
    let mut split = strstr(hname, b"//\0".as_ptr() as *const c_char);

    AA_BUG(ns.is_null());
    AA_BUG(hname.is_null());

    while !split.is_null() {
        parent = profile;
        profile = __strn_find_child(&mut (*policy).profiles, hname, (split as isize - hname as isize) as c_int);
        if profile.is_null() {
            let name = kstrndup(hname, (split as isize - hname as isize) as size_t, gfp);
            if name.is_null() {
                return ptr::null_mut();
            }
            profile = aa_alloc_null(parent, name, gfp);
            kfree_sensitive(name as *mut c_void);
            if profile.is_null() {
                return ptr::null_mut();
            }
            if parent.is_null() {
                (*profile).ns = aa_get_ns(ns);
            }
        }
        policy = &mut (*profile).base;
        hname = split.add(2);
        split = strstr(hname, b"//\0".as_ptr() as *const c_char);
    }
    if profile.is_null() {
        &mut (*ns).base
    } else {
        &mut (*profile).base
    }
}

unsafe fn __lookupn_profile(mut base: *mut aa_policy, mut hname: *const c_char, mut n: size_t) -> *mut aa_profile {
    let mut profile: *mut aa_profile;
    let mut split = strnstr(hname, b"//\0".as_ptr() as *const c_char, n);
    while !split.is_null() {
        profile = __strn_find_child(&mut (*base).profiles, hname, (split as isize - hname as isize) as c_int);
        if profile.is_null() {
            return ptr::null_mut();
        }
        base = &mut (*profile).base;
        n -= (split.add(2) as usize) - (hname as usize);
        hname = split.add(2);
        split = strnstr(hname, b"//\0".as_ptr() as *const c_char, n);
    }

    if n != 0 {
        return __strn_find_child(&mut (*base).profiles, hname, n as c_int);
    }
    ptr::null_mut()
}

unsafe fn __lookup_profile(base: *mut aa_policy, hname: *const c_char) -> *mut aa_profile {
    __lookupn_profile(base, hname, strlen(hname))
}

#[no_mangle]
pub unsafe extern "C" fn aa_lookupn_profile(ns: *mut aa_ns, hname: *const c_char, n: size_t) -> *mut aa_profile {
    let mut profile: *mut aa_profile;
    rcu_read_lock();
    loop {
        profile = __lookupn_profile(&mut (*ns).base, hname, n);
        if profile.is_null() || aa_get_profile_not0(profile) {
            break;
        }
    }
    rcu_read_unlock();

    if profile.is_null() && strncmp(hname, b"unconfined\0".as_ptr() as *const c_char, n) == 0 {
        profile = aa_get_newest_profile((*ns).unconfined);
    }
    profile
}

#[no_mangle]
pub unsafe extern "C" fn aa_fqlookupn_profile(base: *mut aa_label, fqname: *const c_char, n: size_t) -> *mut aa_profile {
    let mut ns: *mut aa_ns;
    let mut ns_name: *const c_char = ptr::null();
    let mut ns_len: size_t = 0;
    let name = aa_splitn_fqname(fqname, n, &mut ns_name, &mut ns_len);

    if !ns_name.is_null() {
        ns = aa_lookupn_ns(labels_ns(base), ns_name, ns_len);
        if ns.is_null() {
            return ptr::null_mut();
        }
    } else {
        ns = aa_get_ns(labels_ns(base));
    }

    let profile = if !name.is_null() {
        aa_lookupn_profile(ns, name, n - (name as usize - fqname as usize))
    } else if !ns.is_null() {
        aa_get_newest_profile((*ns).unconfined)
    } else {
        ptr::null_mut()
    };
    aa_put_ns(ns);
    profile
}

#[no_mangle]
pub unsafe extern "C" fn aa_alloc_null(parent: *mut aa_profile, name: *const c_char, gfp: gfp_t) -> *mut aa_profile {
    let profile = aa_alloc_profile(name, ptr::null_mut(), gfp);
    if profile.is_null() {
        return ptr::null_mut();
    }

    (*profile).label.flags |= FLAG_NULL;
    (*profile).attach.xmatch = aa_get_pdb(nullpdb);
    let rules = (*profile).label.rules[0];
    (*rules).file = aa_get_pdb(nullpdb);
    (*rules).policy = aa_get_pdb(nullpdb);
    aa_compute_profile_mediates(profile);

    if !parent.is_null() {
        (*profile).path_flags = (*parent).path_flags;
        (*profile).label.mediates = (*parent).label.mediates;
        (*profile).parent = aa_get_profile(parent);
        (*profile).ns = aa_get_ns((*parent).ns);
    }

    profile
}

#[no_mangle]
pub unsafe extern "C" fn __aa_new_learning_profile(parent: *mut aa_profile, hat: bool, base: *const c_char, gfp: gfp_t) -> *mut aa_profile {
    let mut profile: *mut aa_profile;
    let mut name: *mut c_char = ptr::null_mut();
    let mut name_sz: size_t;

    AA_BUG(parent.is_null());
    AA_BUG(!mutex_is_locked(&mut (*(*parent).ns).lock));

    if !base.is_null() {
        name_sz = strlen((*parent).base.hname) + 8 + strlen(base);
        name = kmalloc(name_sz, gfp) as *mut c_char;
        if !name.is_null() {
            snprintf(name, name_sz, b"%s//null-%s\0".as_ptr() as *const c_char, (*parent).base.hname, base);
        }
    }

    if name.is_null() {
        name_sz = strlen((*parent).base.hname) + 2 + 7 + 8;
        name = kmalloc(name_sz, gfp) as *mut c_char;
        if name.is_null() {
            return ptr::null_mut();
        }
        snprintf(
            name,
            name_sz,
            b"%s//null-%x\0".as_ptr() as *const c_char,
            (*parent).base.hname,
            atomic_inc_return(&mut (*(*parent).ns).uniq_null),
        );
    }

    let bname = basename(name);
    profile = aa_find_child(parent, bname);
    if !profile.is_null() {
        kfree_sensitive(name as *mut c_void);
        return profile;
    }

    profile = aa_alloc_null(parent, name, gfp);
    if profile.is_null() {
        kfree_sensitive(name as *mut c_void);
        return ptr::null_mut();
    }
    (*profile).mode = APPARMOR_COMPLAIN;
    if hat {
        (*profile).label.flags |= FLAG_HAT;
    }

    let p = __find_child(&mut (*parent).base.profiles, bname);
    if !p.is_null() {
        aa_free_profile(profile);
        profile = aa_get_profile(p);
    } else {
        __add_profile(&mut (*parent).base.profiles, profile);
    }

    kfree_sensitive(name as *mut c_void);
    profile
}

#[no_mangle]
pub unsafe extern "C" fn aa_new_learning_profile(parent: *mut aa_profile, hat: bool, base: *const c_char, gfp: gfp_t) -> *mut aa_profile {
    mutex_lock_nested(&mut (*(*parent).ns).lock, (*(*parent).ns).level);
    let profile = __aa_new_learning_profile(parent, hat, base, gfp);
    mutex_unlock(&mut (*(*parent).ns).lock);
    profile
}

unsafe fn replacement_allowed(profile: *mut aa_profile, noreplace: c_int, info: *mut *const c_char) -> c_int {
    if !profile.is_null() {
        if ((*profile).label.flags & FLAG_IMMUTIBLE) != 0 {
            *info = b"cannot replace immutable profile\0".as_ptr() as *const c_char;
            return -EPERM;
        } else if noreplace != 0 {
            *info = b"profile already exists\0".as_ptr() as *const c_char;
            return -EEXIST;
        }
    }
    0
}

unsafe extern "C" fn audit_cb(ab: *mut audit_buffer, va: *mut c_void) {
    let sa = va as *mut common_audit_data;
    let ad = aad(sa);
    if !(*ad).iface.ns.is_null() {
        audit_log_format(ab, b" ns=\0".as_ptr() as *const c_char);
        audit_log_untrustedstring(ab, (*ad).iface.ns);
    }
}

unsafe fn audit_policy(subj_label: *mut aa_label, op: *const c_char, ns_name: *const c_char, name: *const c_char, info: *const c_char, error: c_int) -> c_int {
    let mut ad = apparmor_audit_data {
        iface: apparmor_audit_iface { ns: ns_name },
        name,
        info,
        error,
        subj_label,
    };
    aa_audit_msg(AUDIT_APPARMOR_STATUS, &mut ad, audit_cb);
    error
}

unsafe fn policy_ns_capable(subj_cred: *const cred, label: *mut aa_label, userns: *mut user_namespace, cap: c_int) -> c_int {
    let mut err = cap_capable(subj_cred, userns, cap, CAP_OPT_NONE);
    if err == 0 {
        err = aa_capable(subj_cred, label, cap, CAP_OPT_NONE);
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn aa_policy_view_capable(subj_cred: *const cred, label: *mut aa_label, mut ns: *mut aa_ns) -> bool {
    let user_ns = (*subj_cred).user_ns;
    let view_ns = labels_view(label);
    let root_in_user_ns = uid_eq(current_euid(), make_kuid(user_ns, 0)) || in_egroup_p(make_kgid(user_ns, 0));
    let mut response = false;
    if ns.is_null() {
        ns = view_ns;
    }

    if root_in_user_ns
        && aa_ns_visible(view_ns, ns, true)
        && (user_ns == &mut init_user_ns
            || (unprivileged_userns_apparmor_policy != 0 && (*user_ns).level == (*view_ns).level))
    {
        response = true;
    }

    response
}

#[no_mangle]
pub unsafe extern "C" fn aa_policy_admin_capable(subj_cred: *const cred, label: *mut aa_label, ns: *mut aa_ns) -> bool {
    let capable = policy_ns_capable(subj_cred, label, (*subj_cred).user_ns, CAP_MAC_ADMIN) == 0;
    aa_policy_view_capable(subj_cred, label, ns) && capable && !aa_g_lock_policy
}

#[no_mangle]
pub unsafe extern "C" fn aa_current_policy_view_capable(ns: *mut aa_ns) -> bool {
    let mut needput = false;
    let label = __begin_current_label_crit_section(&mut needput);
    let res = aa_policy_view_capable(current_cred(), label, ns);
    __end_current_label_crit_section(label, needput);
    res
}

#[no_mangle]
pub unsafe extern "C" fn aa_current_policy_admin_capable(ns: *mut aa_ns) -> bool {
    let mut needput = false;
    let label = __begin_current_label_crit_section(&mut needput);
    let res = aa_policy_admin_capable(current_cred(), label, ns);
    __end_current_label_crit_section(label, needput);
    res
}

unsafe fn is_subset_of_obj_privilege(cred_: *const cred, label: *mut aa_label, ocred: *const cred) -> bool {
    if cred_ == ocred {
        return true;
    }
    if !aa_label_is_subset(label, cred_label(ocred)) {
        return false;
    }
    if (*cred_).user_ns != (*ocred).user_ns {
        return false;
    }
    if !cap_issubset((*cred_).cap_inheritable, (*ocred).cap_inheritable) {
        return false;
    }
    if !cap_issubset((*cred_).cap_permitted, (*ocred).cap_permitted) {
        return false;
    }
    if !cap_issubset((*cred_).cap_effective, (*ocred).cap_effective) {
        return false;
    }
    if !cap_issubset((*cred_).cap_bset, (*ocred).cap_bset) {
        return false;
    }
    if !cap_issubset((*cred_).cap_ambient, (*ocred).cap_ambient) {
        return false;
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn aa_may_manage_policy(subj_cred: *const cred, label: *mut aa_label, ns: *mut aa_ns, ocred: *const cred, mask: u32) -> c_int {
    let op = if (mask & AA_MAY_REMOVE_POLICY) != 0 {
        OP_PROF_RM.as_ptr() as *const c_char
    } else if (mask & AA_MAY_REPLACE_POLICY) != 0 {
        OP_PROF_REPL.as_ptr() as *const c_char
    } else {
        OP_PROF_LOAD.as_ptr() as *const c_char
    };

    if aa_g_lock_policy {
        return audit_policy(label, op, ptr::null(), ptr::null(), b"policy_locked\0".as_ptr() as *const c_char, -EACCES);
    }

    if !ocred.is_null() && !is_subset_of_obj_privilege(subj_cred, label, ocred) {
        return audit_policy(label, op, ptr::null(), ptr::null(), b"not privileged for target profile\0".as_ptr() as *const c_char, -EACCES);
    }

    if !aa_policy_admin_capable(subj_cred, label, ns) {
        return audit_policy(label, op, ptr::null(), ptr::null(), b"not policy admin\0".as_ptr() as *const c_char, -EACCES);
    }

    0
}

unsafe fn __list_lookup_parent(lh: *mut list_head, profile: *mut aa_profile) -> *mut aa_profile {
    let base = basename((*profile).base.hname);
    let mut len = base as c_long - (*profile).base.hname as c_long;
    if len <= 2 {
        return ptr::null_mut();
    }
    len -= 2;

    let mut ent = first_load_ent(lh);
    while &mut (*ent).list as *mut list_head != lh {
        if (*ent).new != profile
            && strncmp((*(*ent).new).base.hname, (*profile).base.hname, len as size_t) == 0
            && *(*(*ent).new).base.hname.add(len as usize) == 0
        {
            return (*ent).new;
        }
        ent = next_load_ent(ent);
    }
    ptr::null_mut()
}

unsafe fn __replace_profile(old: *mut aa_profile, new: *mut aa_profile) {
    if !list_empty(&(*old).base.profiles) {
        let mut lh = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
        list_splice_init_rcu(&mut (*old).base.profiles, &mut lh, synchronize_rcu);

        let mut child = first_profile(&mut lh);
        while &mut (*child).base.list as *mut list_head != &mut lh {
            let tmp = next_profile(child);
            list_del_init(&mut (*child).base.list);
            let p = __find_child(&mut (*new).base.profiles, (*child).base.name);
            if !p.is_null() {
                __replace_profile(child, p);
                child = tmp;
                continue;
            }

            let p_parent = aa_deref_parent(child);
            (*child).parent = aa_get_profile(new);
            list_add_rcu(&mut (*child).base.list, &mut (*new).base.profiles);
            aa_put_profile(p_parent);
            child = tmp;
        }
    }

    if (*new).parent.is_null() {
        let parent = aa_deref_parent(old);
        (*new).parent = aa_get_profile(parent);
    }
    aa_label_replace(&mut (*old).label, &mut (*new).label);
    __aafs_profile_migrate_dents(old, new);

    if list_empty(&(*new).base.list) {
        list_replace_rcu(&mut (*old).base.list, &mut (*new).base.list);
        aa_get_profile(new);
        aa_put_profile(old);
    } else {
        __list_remove_profile(old);
    }
}

unsafe fn __lookup_replace(ns: *mut aa_ns, hname: *const c_char, noreplace: bool, p: *mut *mut aa_profile, info: *mut *const c_char) -> c_int {
    *p = aa_get_profile(__lookup_profile(&mut (*ns).base, hname));
    if !(*p).is_null() {
        let error = replacement_allowed(*p, noreplace as c_int, info);
        if error != 0 {
            *info = b"profile can not be replaced\0".as_ptr() as *const c_char;
            return error;
        }
    }
    0
}

unsafe fn share_name(old: *mut aa_profile, new: *mut aa_profile) {
    aa_put_str((*new).base.hname);
    aa_get_str((*old).base.hname);
    (*new).base.hname = (*old).base.hname;
    (*new).base.name = (*old).base.name;
    (*new).label.hname = (*old).label.hname;
}

unsafe fn update_to_newest_parent(new: *mut aa_profile) -> *mut aa_profile {
    let parent = (*new).parent;
    let newest = aa_get_newest_profile(parent);
    if newest != parent {
        aa_put_profile(parent);
        (*new).parent = newest;
    } else {
        aa_put_profile(newest);
    }
    newest
}

#[no_mangle]
pub unsafe extern "C" fn aa_replace_profiles(
    policy_ns: *mut aa_ns,
    label: *mut aa_label,
    mask: u32,
    mut udata: *mut aa_loaddata,
    compressed_profile: *mut c_char,
    compressed_size: size_t,
) -> ssize_t {
    let mut ns_name: *const c_char = ptr::null();
    let mut info: *const c_char = ptr::null();
    let mut ns: *mut aa_ns = ptr::null_mut();
    let mut ent: *mut aa_load_ent = ptr::null_mut();
    let mut tmp: *mut aa_load_ent;
    let mut op = if (mask & AA_MAY_REPLACE_POLICY) != 0 { OP_PROF_REPL.as_ptr() as *const c_char } else { OP_PROF_LOAD.as_ptr() as *const c_char };
    let mut count: ssize_t;
    let mut error: ssize_t;
    let mut lh = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };

    aa_get_profile_loaddata(udata);
    error = aa_unpack(udata, &mut lh, &mut ns_name, compressed_profile, compressed_size);
    if error != 0 {
        return replace_out(ns, ns_name, udata, error);
    }

    count = 0;
    ent = first_load_ent(&mut lh);
    while &mut (*ent).list as *mut list_head != &mut lh {
        if !ns_name.is_null() {
            if !(*ent).ns_name.is_null() && strcmp((*ent).ns_name, ns_name) != 0 {
                info = b"policy load has mixed namespaces\0".as_ptr() as *const c_char;
                error = -EACCES as ssize_t;
                return replace_fail(label, op, ns_name, ent, info, error, &mut lh, ns, udata);
            }
        } else if !(*ent).ns_name.is_null() {
            if count != 0 {
                info = b"policy load has mixed namespaces\0".as_ptr() as *const c_char;
                error = -EACCES as ssize_t;
                return replace_fail(label, op, ns_name, ent, info, error, &mut lh, ns, udata);
            }
            ns_name = (*ent).ns_name;
            (*ent).ns_name = ptr::null_mut();
        } else {
            count += 1;
        }
        ent = next_load_ent(ent);
    }

    if !ns_name.is_null() {
        ns = aa_prepare_ns(if !policy_ns.is_null() { policy_ns } else { labels_ns(label) }, ns_name);
        if ns.is_null() {
            op = OP_PROF_LOAD.as_ptr() as *const c_char;
            info = b"failed to prepare namespace\0".as_ptr() as *const c_char;
            error = -EACCES as ssize_t;
            ent = ptr::null_mut();
            return replace_fail(label, op, ns_name, ent, info, error, &mut lh, ns, udata);
        }
    } else {
        ns = aa_get_ns(if !policy_ns.is_null() { policy_ns } else { labels_ns(label) });
    }

    mutex_lock_nested(&mut (*ns).lock, (*ns).level);

    if !list_empty(&(*ns).rawdata_list) {
        let mut rawdata_ent = first_load_ent(&mut (*ns).rawdata_list) as *mut aa_loaddata;
        while &mut (*rawdata_ent).list as *mut list_head != &mut (*ns).rawdata_list {
            if aa_rawdata_eq(rawdata_ent, udata) {
                let live = aa_get_profile_loaddata_not0(rawdata_ent);
                if !live.is_null() {
                    aa_put_profile_loaddata(udata);
                    udata = live;
                    break;
                }
            }
            rawdata_ent = (*rawdata_ent).list.next as *mut aa_loaddata;
        }
    }

    ent = first_load_ent(&mut lh);
    while &mut (*ent).list as *mut list_head != &mut lh {
        if aa_g_export_binary {
            (*(*ent).new).rawdata = aa_get_profile_loaddata(udata);
        }
        error = __lookup_replace(ns, (*(*ent).new).base.hname, (mask & AA_MAY_REPLACE_POLICY) == 0, &mut (*ent).old, &mut info) as ssize_t;
        if error != 0 {
            return replace_fail_lock(label, op, ns_name, ent, info, error, &mut lh, ns, udata);
        }
        if !(*(*ent).new).rename.is_null() {
            error = __lookup_replace(ns, (*(*ent).new).rename, (mask & AA_MAY_REPLACE_POLICY) == 0, &mut (*ent).rename, &mut info) as ssize_t;
            if error != 0 {
                return replace_fail_lock(label, op, ns_name, ent, info, error, &mut lh, ns, udata);
            }
        }
        (*(*ent).new).ns = aa_get_ns(ns);
        if !(*ent).old.is_null() || !(*ent).rename.is_null() {
            ent = next_load_ent(ent);
            continue;
        }

        let mut p: *mut aa_profile = ptr::null_mut();
        let mut policy = __lookup_parent(ns, (*(*ent).new).base.hname);
        if policy.is_null() {
            p = __list_lookup_parent(&mut lh, (*ent).new);
            if p.is_null() {
                policy = __create_missing_ancestors(ns, (*(*ent).new).base.hname, GFP_KERNEL);
                if policy.is_null() {
                    error = -ENOENT as ssize_t;
                    info = b"parent does not exist\0".as_ptr() as *const c_char;
                    return replace_fail_lock(label, op, ns_name, ent, info, error, &mut lh, ns, udata);
                }
            }
        }
        if p.is_null() && policy != &mut (*ns).base {
            p = policy as *mut aa_profile;
        }
        (*(*ent).new).parent = aa_get_profile(p);
        ent = next_load_ent(ent);
    }

    if (*udata).dents[AAFS_LOADDATA_DIR].is_null() && aa_g_export_binary {
        error = __aa_fs_create_rawdata(ns, udata);
        if error != 0 {
            info = b"failed to create raw_data dir and files\0".as_ptr() as *const c_char;
            ent = ptr::null_mut();
            return replace_fail_lock(label, op, ns_name, ent, info, error, &mut lh, ns, udata);
        }
    }

    ent = first_load_ent(&mut lh);
    while &mut (*ent).list as *mut list_head != &mut lh {
        if (*ent).old.is_null() {
            let parent = if !(*(*ent).new).parent.is_null() {
                prof_child_dir(aa_deref_parent((*ent).new))
            } else {
                ns_subprofs_dir((*(*ent).new).ns)
            };
            error = __aafs_profile_mkdir((*ent).new, parent);
        }
        if error != 0 {
            info = b"failed to create\0".as_ptr() as *const c_char;
            return replace_fail_lock(label, op, ns_name, ent, info, error, &mut lh, ns, udata);
        }
        ent = next_load_ent(ent);
    }

    __aa_bump_ns_revision(ns);
    if aa_g_export_binary {
        __aa_loaddata_update(udata, (*ns).revision);
    }

    ent = first_load_ent(&mut lh);
    while &mut (*ent).list as *mut list_head != &mut lh {
        tmp = next_load_ent(ent);
        list_del_init(&mut (*ent).list);
        op = if (*ent).old.is_null() && (*ent).rename.is_null() { OP_PROF_LOAD.as_ptr() as *const c_char } else { OP_PROF_REPL.as_ptr() as *const c_char };

        if !(*ent).old.is_null() && (*(*ent).old).rawdata == (*(*ent).new).rawdata && !(*(*ent).new).rawdata.is_null() {
            audit_policy(label, op, ns_name, (*(*ent).new).base.hname, b"same as current profile, skipping\0".as_ptr() as *const c_char, error as c_int);
            aa_put_proxy((*(*ent).new).label.proxy);
            (*(*ent).new).label.proxy = ptr::null_mut();
        } else {
            if !aa_g_export_binary {
                if !(*ent).old.is_null() && !(*(*ent).old).rawdata.is_null() && !(*(*ent).old).dents[AAFS_LOADDATA_DIR].is_null() {
                    __aa_remove_rawdata_symlink_dents((*ent).old);
                }
            }
            audit_policy(label, op, ns_name, (*(*ent).new).base.hname, ptr::null(), error as c_int);
            if !(*ent).old.is_null() {
                share_name((*ent).old, (*ent).new);
                __replace_profile((*ent).old, (*ent).new);
                if aa_g_export_binary && (*(*ent).old).rawdata.is_null() {
                    __aa_create_rawdata_symlink_dents((*ent).new);
                }
            } else {
                let lh_target = if !(*(*ent).new).parent.is_null() {
                    let parent = update_to_newest_parent((*ent).new);
                    &mut (*parent).base.profiles
                } else {
                    &mut (*ns).base.profiles
                };
                __add_profile(lh_target, (*ent).new);
            }
        }
        aa_load_ent_free(ent);
        ent = tmp;
    }

    __aa_labelset_update_subtree(ns);
    mutex_unlock(&mut (*ns).lock);
    replace_out(ns, ns_name, udata, error)
}

unsafe fn replace_out(ns: *mut aa_ns, ns_name: *const c_char, udata: *mut aa_loaddata, error: ssize_t) -> ssize_t {
    aa_put_ns(ns);
    let udata_sz = (*udata).size;
    aa_put_profile_loaddata(udata);
    kfree_sensitive(ns_name as *mut c_void);
    if error != 0 {
        error
    } else {
        udata_sz
    }
}

unsafe fn replace_fail_lock(label: *mut aa_label, op: *const c_char, ns_name: *const c_char, ent: *mut aa_load_ent, info: *const c_char, error: ssize_t, lh: *mut list_head, ns: *mut aa_ns, udata: *mut aa_loaddata) -> ssize_t {
    mutex_unlock(&mut (*ns).lock);
    let op2 = if !ent.is_null() && (*ent).old.is_null() { OP_PROF_LOAD.as_ptr() as *const c_char } else { OP_PROF_REPL.as_ptr() as *const c_char };
    replace_fail(label, op2, ns_name, ent, info, error, lh, ns, udata)
}

unsafe fn replace_fail(label: *mut aa_label, _op: *const c_char, ns_name: *const c_char, ent: *mut aa_load_ent, mut info: *const c_char, error: ssize_t, lh: *mut list_head, ns: *mut aa_ns, udata: *mut aa_loaddata) -> ssize_t {
    let first_name = if !ent.is_null() { (*(*ent).new).base.hname } else { ptr::null() };
    let first_op = if !ent.is_null() && (*ent).old.is_null() { OP_PROF_LOAD.as_ptr() as *const c_char } else { OP_PROF_REPL.as_ptr() as *const c_char };
    audit_policy(label, first_op, ns_name, first_name, info, error as c_int);

    info = b"valid profile in failed atomic policy load\0".as_ptr() as *const c_char;
    let mut tmp = first_load_ent(lh);
    while &mut (*tmp).list as *mut list_head != lh {
        if tmp == ent {
            info = b"unchecked profile in failed atomic policy load\0".as_ptr() as *const c_char;
            tmp = next_load_ent(tmp);
            continue;
        }
        let op = if (*tmp).old.is_null() { OP_PROF_LOAD.as_ptr() as *const c_char } else { OP_PROF_REPL.as_ptr() as *const c_char };
        audit_policy(label, op, ns_name, (*(*tmp).new).base.hname, info, error as c_int);
        tmp = next_load_ent(tmp);
    }

    let mut e = first_load_ent(lh);
    while &mut (*e).list as *mut list_head != lh {
        let next = next_load_ent(e);
        list_del_init(&mut (*e).list);
        aa_load_ent_free(e);
        e = next;
    }

    replace_out(ns, ns_name, udata, error)
}

#[no_mangle]
pub unsafe extern "C" fn aa_remove_profiles(policy_ns: *mut aa_ns, subj: *mut aa_label, fqname: *mut c_char, size: size_t) -> ssize_t {
    let mut ns: *mut aa_ns = ptr::null_mut();
    let mut profile: *mut aa_profile = ptr::null_mut();
    let mut name: *const c_char = fqname;
    let mut info: *const c_char = ptr::null();
    let mut ns_name: *const c_char = ptr::null();
    let mut error: ssize_t = 0;

    if *fqname == 0 {
        info = b"no profile specified\0".as_ptr() as *const c_char;
        error = -ENOENT as ssize_t;
        audit_policy(subj, OP_PROF_RM.as_ptr() as *const c_char, ns_name, name, info, error as c_int);
        return error;
    }

    if *fqname == b':' as c_char {
        let mut ns_len: size_t = 0;
        name = aa_splitn_fqname(fqname, size, &mut ns_name, &mut ns_len);
        ns = aa_lookupn_ns(if !policy_ns.is_null() { policy_ns } else { labels_ns(subj) }, ns_name, ns_len);
        if ns.is_null() {
            info = b"namespace does not exist\0".as_ptr() as *const c_char;
            error = -ENOENT as ssize_t;
            audit_policy(subj, OP_PROF_RM.as_ptr() as *const c_char, ns_name, name, info, error as c_int);
            return error;
        }
    } else {
        ns = aa_get_ns(if !policy_ns.is_null() { policy_ns } else { labels_ns(subj) });
    }

    if name.is_null() {
        mutex_lock_nested(&mut (*(*ns).parent).lock, (*(*ns).parent).level);
        __aa_bump_ns_revision(ns);
        __aa_remove_ns(ns);
        mutex_unlock(&mut (*(*ns).parent).lock);
    } else {
        mutex_lock_nested(&mut (*ns).lock, (*ns).level);
        profile = aa_get_profile(__lookup_profile(&mut (*ns).base, name));
        if profile.is_null() {
            error = -ENOENT as ssize_t;
            info = b"profile does not exist\0".as_ptr() as *const c_char;
            mutex_unlock(&mut (*ns).lock);
            aa_put_ns(ns);
            audit_policy(subj, OP_PROF_RM.as_ptr() as *const c_char, ns_name, name, info, error as c_int);
            return error;
        }
        name = (*profile).base.hname;
        __aa_bump_ns_revision(ns);
        __remove_profile(profile);
        __aa_labelset_update_subtree(ns);
        mutex_unlock(&mut (*ns).lock);
    }

    audit_policy(subj, OP_PROF_RM.as_ptr() as *const c_char, ns_name, name, info, error as c_int);
    aa_put_ns(ns);
    aa_put_profile(profile);
    size as ssize_t
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
