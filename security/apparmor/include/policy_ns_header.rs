// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor policy definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2017 Canonical Ltd.

// Dependencies from external headers:
// - linux/kref.h
// - apparmor.h
// - apparmorfs.h
// - label.h
// - policy.h

// Forward declarations for types from external headers
pub struct aa_policy;
pub struct aa_profile;
pub struct mutex;
pub struct list_head;
pub struct aa_labelset;
pub struct dentry;
pub struct atomic_t;
pub struct wait_queue_head_t;
pub struct aa_label;

// Match max depth of user namespaces
pub const MAX_NS_DEPTH: i32 = 32;

// struct aa_ns_acct - accounting of profiles in namespace
// @max_size: maximum space allowed for all profiles in namespace
// @max_count: maximum number of profiles that can be in this namespace
// @size: current size of profiles
// @count: current count of profiles (includes null profiles)
#[repr(C)]
pub struct aa_ns_acct {
    pub max_size: i32,
    pub max_count: i32,
    pub size: i32,
    pub count: i32,
}

// struct aa_ns - namespace for a set of profiles
// @base: common policy
// @parent: parent of namespace
// @lock: lock for modifying the object
// @acct: accounting for the namespace
// @unconfined: special unconfined profile for the namespace
// @sub_ns: list of namespaces under the current namespace.
// @uniq_null: uniq value used for null learning profiles
// @uniq_id: a unique id count for the profiles in the namespace
// @level: level of ns within the tree hierarchy
// @dents: dentries for the namespaces file entries in apparmorfs
//
// An aa_ns defines the set profiles that are searched to determine which
// profile to attach to a task.  Profiles can not be shared between aa_ns
// and profile names within a namespace are guaranteed to be unique.  When
// profiles in separate namespaces have the same name they are NOT considered
// to be equivalent.
//
// Namespaces are hierarchical and only namespaces and profiles below the
// current namespace are visible.
//
// Namespace names must be unique and can not contain the characters :/\0
#[repr(C)]
pub struct aa_ns {
    pub base: aa_policy,
    pub parent: *mut aa_ns,
    pub lock: mutex,
    pub acct: aa_ns_acct,
    pub unconfined: *mut aa_profile,
    pub sub_ns: list_head,
    pub uniq_null: atomic_t,
    pub uniq_id: i64,
    pub level: i32,
    pub revision: i64,
    pub wait: wait_queue_head_t,
    pub labels: aa_labelset,
    pub rawdata_list: list_head,
    // AAFS_NS_SIZEOF comes from apparmorfs.h
    pub dents: [*mut dentry; AAFS_NS_SIZEOF],
}

extern "C" {
    pub static mut kernel_t: *mut aa_label;
    pub static mut root_ns: *mut aa_ns;
    pub static aa_hidden_ns_name: *const u8;
}

// ns_unconfined(NS) returns (&(NS)->unconfined->label)
#[macro_export]
macro_rules! ns_unconfined {
    ($NS:expr) => {
        unsafe { &(*(*($NS).unconfined)).label }
    };
}

extern "C" {
    pub fn aa_ns_visible(curr: *mut aa_ns, view: *mut aa_ns, subns: bool) -> bool;
    pub fn aa_ns_name(parent: *mut aa_ns, child: *mut aa_ns, subns: bool) -> *const u8;
    pub fn aa_free_ns(ns: *mut aa_ns);
    pub fn aa_alloc_root_ns() -> i32;
    pub fn aa_free_root_ns();
    pub fn __aa_lookupn_ns(view: *mut aa_ns, hname: *const u8, n: usize) -> *mut aa_ns;
    pub fn aa_lookupn_ns(view: *mut aa_ns, name: *const u8, n: usize) -> *mut aa_ns;
    pub fn __aa_find_or_create_ns(
        parent: *mut aa_ns,
        name: *const u8,
        dir: *mut dentry,
    ) -> *mut aa_ns;
    pub fn aa_prepare_ns(root: *mut aa_ns, name: *const u8) -> *mut aa_ns;
    pub fn __aa_remove_ns(ns: *mut aa_ns);
    pub fn aa_get_profile(p: *mut aa_profile);
    pub fn aa_put_profile(p: *mut aa_profile);
    pub fn __policy_strn_find(head: *mut list_head, name: *const u8, n: usize) -> *mut aa_policy;
    pub fn strlen(s: *const u8) -> usize;
    pub fn rcu_dereference_protected(ptr: *mut aa_profile, cond: bool) -> *mut aa_profile;
    pub fn mutex_is_locked(m: *const mutex) -> bool;
}

#[inline]
pub unsafe fn aa_deref_parent(p: *mut aa_profile) -> *mut aa_profile {
    rcu_dereference_protected((*p).parent, mutex_is_locked(&(*(*p).ns).lock))
}

/// aa_get_ns - increment references count on @ns
/// @ns: namespace to increment reference count of (MAYBE NULL)
///
/// Returns: pointer to @ns, if @ns is NULL returns NULL
/// Requires: @ns must be held with valid refcount when called
#[inline]
pub unsafe fn aa_get_ns(ns: *mut aa_ns) -> *mut aa_ns {
    if !ns.is_null() {
        aa_get_profile((*ns).unconfined);
    }
    ns
}

/// aa_put_ns - decrement refcount on @ns
/// @ns: namespace to put reference of
///
/// Decrement reference count of @ns and if no longer in use free it
#[inline]
pub unsafe fn aa_put_ns(ns: *mut aa_ns) {
    if !ns.is_null() {
        aa_put_profile((*ns).unconfined);
    }
}

/// __aa_findn_ns - find a namespace on a list by @name
/// @head: list to search for namespace on  (NOT NULL)
/// @name: name of namespace to look for  (NOT NULL)
/// @n: length of @name
/// Returns: unrefcounted namespace
///
/// Requires: rcu_read_lock be held
#[inline]
pub unsafe fn __aa_findn_ns(
    head: *mut list_head,
    name: *const u8,
    n: usize,
) -> *mut aa_ns {
    __policy_strn_find(head, name, n) as *mut aa_ns
}

#[inline]
pub unsafe fn __aa_find_ns(head: *mut list_head, name: *const u8) -> *mut aa_ns {
    __aa_findn_ns(head, name, strlen(name))
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
