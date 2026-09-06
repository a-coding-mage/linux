// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor label definitions
 *
 * Copyright 2017 Canonical Ltd.
 */

// Dependencies: linux/atomic.h, linux/audit.h, linux/rbtree.h, linux/rcupdate.h, apparmor.h, lib.h

// Forward declarations
pub struct aa_ns;
pub struct aa_ruleset;

pub const LOCAL_VEC_ENTRIES: usize = 8;

// DEFINE_VEC(T, V) macro equivalent - declares a vector with local storage
// Usage: struct aa_profile *(_vec_localtmp)[LOCAL_VEC_ENTRIES + 1]; struct aa_profile **(vec)
// This is a macro pattern and should be used at call sites; Rust doesn't have direct equivalent

// vec_setup(T, V, N, GFP) macro - initializes a vector
// Allocates from local stack if N <= LOCAL_VEC_ENTRIES, otherwise uses kzalloc_objs
// Returns: 0 on success, -ENOMEM on failure

// vec_cleanup(T, V, N) macro - cleans up a vector
// Decrements reference counts and frees allocated memory if needed

// Macro helper functions - these are pattern-matching macros in C
// #define vec_last(VEC, SIZE) ((VEC)[(SIZE) - 1])
// #define vec_ns(VEC, SIZE) (vec_last((VEC), (SIZE))->ns)
// #define vec_labelset(VEC, SIZE) (&vec_ns((VEC), (SIZE))->labels)
// #define cleanup_domain_vec(V, L) cleanup_label_vec((V), (L)->size)

pub struct aa_profile;

// VEC_FLAG_TERMINATE constant
pub const VEC_FLAG_TERMINATE: i32 = 1;

// External function declarations
extern "C" {
    pub fn aa_vec_unique(vec: *mut *mut aa_profile, n: i32, flags: i32) -> i32;
    pub fn aa_vec_find_or_create_label(
        vec: *mut *mut aa_profile,
        len: i32,
        gfp: crate::gfp_t,
    ) -> *mut aa_label;
}

// Macro: aa_sort_and_merge_vec(N, V)
// Expands to: aa_sort_and_merge_profiles((N), (struct aa_profile **)(V))

// struct aa_labelset - set of labels for a namespace
//
// Labels are reference counted; aa_labelset does not contribute to label
// reference counts. Once a label's last refcount is put it is removed from
// the set.
#[repr(C)]
pub struct aa_labelset {
    pub lock: crate::rwlock_t,
    pub root: crate::rb_root,
}

// Macro: __labelset_for_each(LS, N)
// for ((N) = rb_first(&(LS)->root); (N); (N) = rb_next(N))

// Enum for label_flags
#[repr(C)]
pub enum label_flags {
    FLAG_HAT = 1,                     // profile is a hat
    FLAG_UNCONFINED = 2,              // label unconfined only if all
    FLAG_NULL = 4,                    // profile is null learning profile
    FLAG_IX_ON_NAME_ERROR = 8,        // fallback to ix on name lookup fail
    FLAG_IMMUTIBLE = 0x10,            // don't allow changes/replacement
    FLAG_USER_DEFINED = 0x20,         // user based profile - lower privs
    FLAG_NO_LIST_REF = 0x40,          // list doesn't keep profile ref
    FLAG_NS_COUNT = 0x80,             // carries NS ref count
    FLAG_IN_TREE = 0x100,             // label is in tree
    FLAG_PROFILE = 0x200,             // label is a profile
    FLAG_EXPLICIT = 0x400,            // explicit static label
    FLAG_STALE = 0x800,               // replaced/removed
    FLAG_RENAMED = 0x1000,            // label has renaming in it
    FLAG_REVOKED = 0x2000,            // label has revocation in it
    FLAG_DEBUG1 = 0x4000,
    FLAG_DEBUG2 = 0x8000,
    // These flags must correspond with PATH_flags
    // TODO: add new path flags
}

pub struct aa_label;

#[repr(C)]
pub struct aa_proxy {
    pub count: crate::aa_common_ref,
    pub label: *mut aa_label, // __rcu annotation indicates RCU-protected pointer
}

#[repr(C)]
pub struct label_it {
    pub i: i32,
    pub j: i32,
}

// struct aa_label_base - base info of label (note: actual name is aa_label)
// @count: ref count of active users
// @node: rbtree position
// @rcu: rcu callback struct
// @proxy: is set to the label that replaced this label
// @hname: text representation of the label (MAYBE_NULL)
// @flags: stale and other flags - values may change under label set lock
// @secid: secid that references this label
// @size: number of entries in @ent[]
// @mediates: bitmask for label_mediates
// @profile: label vec when embedded in a profile FLAG_PROFILE is set
// @rules: variable length rules in a profile FLAG_PROFILE is set
// @vec: vector of profiles comprising the compound label
#[repr(C)]
pub struct aa_label {
    pub count: crate::aa_common_ref,
    pub node: crate::rb_node,
    pub rcu: crate::rcu_head,
    pub proxy: *mut aa_proxy,
    pub hname: *mut crate::counted_char, // __counted char *
    pub flags: i64,
    pub secid: u32,
    pub size: i32,
    pub mediates: u64,
    // Union: either profile/rules pair or vec
    // This is a flexible-array-member union in C
    // For repr(C), we need to properly represent this
    pub profile_or_vec: aa_label_union,
}

#[repr(C)]
pub union aa_label_union {
    pub profile_rules: aa_label_profile_rules,
    pub vec: *mut *mut aa_profile, // DECLARE_FLEX_ARRAY(struct aa_profile *, vec)
}

#[repr(C)]
pub struct aa_label_profile_rules {
    // only used if the label is a profile, size of
    // rules[] is determined by the profile
    // profile[1] is poison or null as guard
    pub profile: [*mut aa_profile; 2],
    pub rules: *mut *mut aa_ruleset, // DECLARE_FLEX_ARRAY(struct aa_ruleset *, rules)
}

// Macro: last_error(E, FN)
// do { int __subE = (FN); if (__subE) (E) = __subE; } while (0)

// Macro-style inline checks
// #define label_isprofile(X) ((X)->flags & FLAG_PROFILE)
// #define label_unconfined(X) ((X)->flags & FLAG_UNCONFINED)
// #define unconfined(X) label_unconfined(X)
// #define label_is_stale(X) ((X)->flags & FLAG_STALE)
// #define __label_make_stale(X) ((X)->flags |= FLAG_STALE)
// #define labels_ns(X) (vec_ns(&((X)->vec[0]), (X)->size))
// #define labels_set(X) (&labels_ns(X)->labels)
// #define labels_view(X) labels_ns(X)
// #define labels_profile(X) ((X)->vec[(X)->size - 1])

extern "C" {
    pub fn aa_label_next_confined(l: *const aa_label, i: i32) -> i32;
}

// Macro: label_for_each(I, L, P)
// for ((I).i = 0; ((P) = (L)->vec[(I).i]); ++((I).i))

// Macro: label_for_each_cont(I, L, P)
// for (++((I).i); ((P) = (L)->vec[(I).i]); ++((I).i))

// Macro: label_for_each_confined(I, L, P)
// for ((I).i = aa_label_next_confined((L), 0); ((P) = (L)->vec[(I).i]); (I).i = aa_label_next_confined((L), (I).i + 1))

// Macro: label_for_each_in_merge(I, A, B, P)
// for ((I).i = (I).j = 0; ((P) = aa_label_next_in_merge(&(I), (A), (B))); )

// Macro: label_for_each_not_in_set(I, SET, SUB, P)
// for ((I).i = (I).j = 0; ((P) = __aa_label_next_not_in_set(&(I), (SET), (SUB))); )

// Macro: next_in_ns(i, NS, L)
// ({ typeof(i) ___i = (i); while ((L)->vec[___i] && (L)->vec[___i]->ns != (NS)) (___i)++; (___i); })

// Macro: label_for_each_in_ns(I, NS, L, P)
// for ((I).i = next_in_ns(0, (NS), (L)); ((P) = (L)->vec[(I).i]); (I).i = next_in_ns((I).i + 1, (NS), (L)))

// Macro: fn_for_each_in_ns(L, P, FN)
// ({ struct label_it __i; struct aa_ns *__ns = labels_ns(L); int __E = 0; label_for_each_in_ns(__i, __ns, (L), (P)) { last_error(__E, (FN)); } __E; })

// Macro: fn_for_each_XXX(L, P, FN, ...)
// ({ struct label_it i; int __E = 0; label_for_each ## __VA_ARGS__(i, (L), (P)) { last_error(__E, (FN)); } __E; })

// Macro: fn_for_each(L, P, FN) fn_for_each_XXX(L, P, FN)
// Macro: fn_for_each_confined(L, P, FN) fn_for_each_XXX(L, P, FN, _confined)

// Macro: fn_for_each2_XXX(L1, L2, P, FN, ...)
// ({ struct label_it i; int __E = 0; label_for_each ## __VA_ARGS__(i, (L1), (L2), (P)) { last_error(__E, (FN)); } __E; })

// Macro: fn_for_each_in_merge(L1, L2, P, FN)
// fn_for_each2_XXX((L1), (L2), P, FN, _in_merge)

// Macro: fn_for_each_not_in_set(L1, L2, P, FN)
// fn_for_each2_XXX((L1), (L2), P, FN, _not_in_set)

// Inline function: label_mediates
#[inline]
pub unsafe fn label_mediates(l: *const aa_label, c: u8) -> bool {
    ((*l).mediates & (1u64 << (c as u64))) != 0
}

// Inline function: label_mediates_safe
#[inline]
pub unsafe fn label_mediates_safe(l: *const aa_label, c: u8) -> bool {
    if c > crate::AA_CLASS_LAST as u8 {
        return false;
    }
    label_mediates(l, c)
}

extern "C" {
    pub fn aa_labelset_destroy(ls: *mut aa_labelset);
    pub fn aa_labelset_init(ls: *mut aa_labelset);
    pub fn __aa_labelset_update_subtree(ns: *mut aa_ns);

    pub fn aa_label_destroy(label: *mut aa_label);
    pub fn aa_label_free(label: *mut aa_label);
    pub fn aa_label_kref(kref: *mut crate::kref);
    pub fn aa_label_init(label: *mut aa_label, size: i32, gfp: crate::gfp_t) -> bool;
    pub fn aa_label_alloc(
        size: i32,
        proxy: *mut aa_proxy,
        gfp: crate::gfp_t,
    ) -> *mut aa_label;

    pub fn aa_label_is_subset(set: *const aa_label, sub: *const aa_label) -> bool;
    pub fn aa_label_is_unconfined_subset(
        set: *const aa_label,
        sub: *const aa_label,
    ) -> bool;
    pub fn __aa_label_next_not_in_set(
        i: *mut label_it,
        set: *const aa_label,
        sub: *const aa_label,
    ) -> *mut aa_profile;
    pub fn aa_label_remove(label: *mut aa_label) -> bool;
    pub fn aa_label_insert(ls: *mut aa_labelset, l: *mut aa_label) -> *mut aa_label;
    pub fn aa_label_replace(old: *mut aa_label, new: *mut aa_label) -> bool;
    pub fn aa_label_make_newest(
        ls: *mut aa_labelset,
        old: *mut aa_label,
        new: *mut aa_label,
    ) -> bool;

    pub fn aa_label_next_in_merge(
        i: *mut label_it,
        a: *const aa_label,
        b: *const aa_label,
    ) -> *mut aa_profile;
    pub fn aa_label_find_merge(a: *mut aa_label, b: *mut aa_label) -> *mut aa_label;
    pub fn aa_label_merge(a: *mut aa_label, b: *mut aa_label, gfp: crate::gfp_t)
        -> *mut aa_label;

    pub fn aa_update_label_name(
        ns: *mut aa_ns,
        label: *mut aa_label,
        gfp: crate::gfp_t,
    ) -> bool;
}

pub const FLAGS_NONE: i32 = 0;
pub const FLAG_SHOW_MODE: i32 = 1;
pub const FLAG_VIEW_SUBNS: i32 = 2;
pub const FLAG_HIDDEN_UNCONFINED: i32 = 4;
pub const FLAG_ABS_ROOT: i32 = 8;

extern "C" {
    pub fn aa_label_snxprint(
        str: *mut i8,
        size: usize,
        view: *mut aa_ns,
        label: *mut aa_label,
        flags: i32,
    ) -> i32;
    pub fn aa_label_asxprint(
        strp: *mut *mut i8,
        ns: *mut aa_ns,
        label: *mut aa_label,
        flags: i32,
        gfp: crate::gfp_t,
    ) -> i32;
    pub fn aa_label_acntsxprint(
        strp: *mut *mut crate::counted_char,
        ns: *mut aa_ns,
        label: *mut aa_label,
        flags: i32,
        gfp: crate::gfp_t,
    ) -> i32;
    pub fn aa_label_xaudit(
        ab: *mut crate::audit_buffer,
        ns: *mut aa_ns,
        label: *mut aa_label,
        flags: i32,
        gfp: crate::gfp_t,
    );
    pub fn aa_label_seq_xprint(
        f: *mut crate::seq_file,
        ns: *mut aa_ns,
        label: *mut aa_label,
        flags: i32,
        gfp: crate::gfp_t,
    );
    pub fn aa_label_xprintk(
        ns: *mut aa_ns,
        label: *mut aa_label,
        flags: i32,
        gfp: crate::gfp_t,
    );
    pub fn aa_label_printk(label: *mut aa_label, gfp: crate::gfp_t);

    pub fn aa_label_strn_parse(
        base: *mut aa_label,
        str: *const i8,
        n: usize,
        gfp: crate::gfp_t,
        create: bool,
        force_stack: bool,
    ) -> *mut aa_label;
    pub fn aa_label_parse(
        base: *mut aa_label,
        str: *const i8,
        gfp: crate::gfp_t,
        create: bool,
        force_stack: bool,
    ) -> *mut aa_label;
}

// Inline function: aa_label_strn_split
#[inline]
pub unsafe fn aa_label_strn_split(str: *const i8, n: i32) -> *const i8 {
    let mut pos: *const i8 = std::ptr::null();
    let state = crate::aa_dfa_matchn_until(
        crate::stacksplitdfa,
        crate::DFA_START,
        str,
        n,
        &mut pos,
    );
    if crate::ACCEPT_TABLE(crate::stacksplitdfa)[state as usize] == 0 {
        return std::ptr::null();
    }
    pos.offset(-3)
}

// Inline function: aa_label_str_split
#[inline]
pub unsafe fn aa_label_str_split(str: *const i8) -> *const i8 {
    let mut pos: *const i8 = std::ptr::null();
    let state = crate::aa_dfa_match_until(crate::stacksplitdfa, crate::DFA_START, str, &mut pos);
    if crate::ACCEPT_TABLE(crate::stacksplitdfa)[state as usize] == 0 {
        return std::ptr::null();
    }
    pos.offset(-3)
}

pub struct aa_perms;

extern "C" {
    pub fn aa_label_match(
        profile: *const aa_profile,
        rules: *mut aa_ruleset,
        label: *mut aa_label,
        state: crate::aa_state_t,
        subns: bool,
        request: u32,
        perms: *mut aa_perms,
    ) -> i32;
}

/// __aa_get_label - get a reference count to uncounted label reference
/// @l: reference to get a count on
///
/// Returns: pointer to reference OR NULL if race is lost and reference is
///          being repeated.
/// Requires: lock held, and the return code MUST be checked
#[inline]
pub unsafe fn __aa_get_label(l: *mut aa_label) -> *mut aa_label {
    if !l.is_null() && crate::kref_get_unless_zero(&mut (*l).count.count) {
        return l;
    }
    std::ptr::null_mut()
}

#[inline]
pub unsafe fn aa_get_label(l: *mut aa_label) -> *mut aa_label {
    if !l.is_null() {
        crate::kref_get(&mut (*l).count.count);
    }
    l
}

/// aa_get_label_rcu - increment refcount on a label that can be replaced
/// @l: pointer to label that can be replaced (NOT NULL)
///
/// Returns: pointer to a refcounted label.
///     else NULL if no label
#[inline]
pub unsafe fn aa_get_label_rcu(l: *mut *mut aa_label) -> *mut aa_label {
    let mut c: *mut aa_label;

    crate::rcu_read_lock();
    loop {
        c = crate::rcu_dereference(l);
        if c.is_null() || !crate::kref_get_unless_zero(&mut (*c).count.count) {
            break;
        }
    }
    crate::rcu_read_unlock();

    c
}

/// aa_get_newest_label - find the newest version of @l
/// @l: the label to check for newer versions of
///
/// Returns: refcounted newest version of @l taking into account
///          replacement, renames and removals
///          return @l.
#[inline]
pub unsafe fn aa_get_newest_label(l: *mut aa_label) -> *mut aa_label {
    if l.is_null() {
        return std::ptr::null_mut();
    }

    if crate::label_is_stale(l) {
        let tmp: *mut aa_label;

        crate::AA_BUG((*l).proxy.is_null());
        crate::AA_BUG((*(*l).proxy).label.is_null());
        // BUG: only way this can happen is @l ref count and its
        // replacement count have gone to 0 and are on their way
        // to destruction. ie. we have a refcounting error
        tmp = aa_get_label_rcu(&mut (*(*l).proxy).label);
        crate::AA_BUG(tmp.is_null());

        return tmp;
    }

    aa_get_label(l)
}

/// aa_get_newest_label_condref - find the newest version of @l
/// @l: the label to check for newer versions of
/// @needput: returns whether the reference needs put
///
/// Returns: refcounted newest version of @l taking into account
///          replacement, renames and removals
///          return @l.
#[inline]
pub unsafe fn aa_get_newest_label_condref(
    l: *mut aa_label,
    needput: *mut bool,
) -> *mut aa_label {
    if !l.is_null() && crate::unlikely(crate::label_is_stale(l)) {
        let tmp: *mut aa_label;

        crate::AA_BUG((*l).proxy.is_null());
        crate::AA_BUG((*(*l).proxy).label.is_null());
        // BUG: only way this can happen is @l ref count and its
        // replacement count have gone to 0 and are on their way
        // to destruction. ie. we have a refcounting error
        tmp = aa_get_label_rcu(&mut (*(*l).proxy).label);
        crate::AA_BUG(tmp.is_null());

        *needput = true;
        return tmp;
    }

    *needput = false;
    l
}

#[inline]
pub unsafe fn aa_put_label(l: *mut aa_label) {
    if !l.is_null() {
        crate::kref_put(&mut (*l).count.count, aa_label_kref);
    }
}

// wrapper fn to indicate semantics of the check
#[inline]
pub unsafe fn __aa_subj_label_is_cached(
    subj_label: *const aa_label,
    obj_label: *const aa_label,
) -> bool {
    aa_label_is_subset(obj_label, subj_label)
}

extern "C" {
    pub fn aa_alloc_proxy(l: *mut aa_label, gfp: crate::gfp_t) -> *mut aa_proxy;
    pub fn aa_proxy_kref(kref: *mut crate::kref);
}

#[inline]
pub unsafe fn aa_get_proxy(proxy: *mut aa_proxy) -> *mut aa_proxy {
    if !proxy.is_null() {
        crate::kref_get(&mut (*proxy).count.count);
    }
    proxy
}

#[inline]
pub unsafe fn aa_put_proxy(proxy: *mut aa_proxy) {
    if !proxy.is_null() {
        crate::kref_put(&mut (*proxy).count.count, aa_proxy_kref);
    }
}

extern "C" {
    pub fn __aa_proxy_redirect(orig: *mut aa_label, new: *mut aa_label);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
