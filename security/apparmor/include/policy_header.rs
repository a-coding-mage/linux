// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor policy definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.

// Includes: linux/capability.h, linux/cred.h, linux/kref.h, linux/rhashtable.h,
// linux/sched.h, linux/slab.h, linux/socket.h
// Module includes: apparmor.h, audit.h, capability.h, domain.h, file.h, lib.h,
// label.h, perms.h, resource.h

pub struct AaNs;

extern "C" {
    pub static unprivileged_userns_apparmor_policy: i32;
    pub static aa_unprivileged_unconfined_restricted: i32;

pub static aa_profile_mode_names: *const *const i8;
}

#[inline]
pub unsafe fn PROFILE_MODE(profile: *const AaProfile, mode: ProfileMode) -> bool {
    aa_g_profile_mode == mode || (*profile).mode == mode as i64
}

#[inline]
pub unsafe fn COMPLAIN_MODE(profile: *const AaProfile) -> bool {
    PROFILE_MODE(profile, ProfileMode::AppArmorComplain)
}

#[inline]
pub unsafe fn USER_MODE(profile: *const AaProfile) -> bool {
    PROFILE_MODE(profile, ProfileMode::AppArmorUser)
}

#[inline]
pub unsafe fn KILL_MODE(profile: *const AaProfile) -> bool {
    PROFILE_MODE(profile, ProfileMode::AppArmorKill)
}

#[inline]
pub unsafe fn PROFILE_IS_HAT(profile: *const AaProfile) -> u32 {
    (*profile).label.flags & FLAG_HAT
}

#[inline]
pub unsafe fn CHECK_DEBUG1(profile: *const AaProfile) -> u32 {
    (*profile).label.flags & FLAG_DEBUG1
}

#[inline]
pub unsafe fn CHECK_DEBUG2(profile: *const AaProfile) -> u32 {
    (*profile).label.flags & FLAG_DEBUG2
}

#[inline]
pub unsafe fn profile_is_stale(profile: *const AaProfile) -> bool {
    label_is_stale(&(*profile).label)
}

extern "C" {
    pub static FLAG_HAT: u32;
    pub static FLAG_DEBUG1: u32;
    pub static FLAG_DEBUG2: u32;
    pub fn label_is_stale(label: *const AaLabel) -> bool;
}

// flags in the dfa accept2 table
#[repr(u32)]
pub enum DfaAcceptFlags {
    AcceptFlagOwner = 1,
}

#[repr(C)]
pub enum ProfileMode {
    AppArmorEnforce,      // enforce access rules
    AppArmorComplain,     // allow and log access violations
    AppArmorKill,         // kill task on access violation
    AppArmorUnconfined,   // profile set to unconfined
    AppArmorUser,         // modified complain mode to userspace
    ProfileModeNamesCount, // Must be last entry
}

#[repr(C)]
pub struct AaTagsHeader {
    pub mask: u32,  // bit mask matching permissions
    pub count: u32, // number of strings per entry
    pub size: u32,  // size of all strings covered by count
    pub tags: u32,  // index into string table
}

#[repr(C)]
pub struct AaTagsStructSets {
    pub size: u32,      // number of entries in tagsets
    pub table: *mut u32, // indexes into headers & strs
}

#[repr(C)]
pub struct AaTagsStructHdrs {
    pub size: u32,               // number of headers == num of strs
    pub table: *mut AaTagsHeader,
}

extern "C" {
    pub struct AaStrTable;
}

#[repr(C)]
pub struct AaTagsStruct {
    pub sets: AaTagsStructSets,
    pub hdrs: AaTagsStructHdrs,
    pub strs: AaStrTable,
}

extern "C" {
    pub struct AaDfa;
    pub struct AaPerms;
    pub struct Kref;

    pub type AaStateT = u32;
}

// struct aa_policydb - match engine for a policy
// @count: refcount for the pdb
// @dfa: dfa pattern match
// @perms: table of permissions
// @size: number of entries in @perms
// @trans: table of strings, index by x
// @tags: table of tags that perms->tag indexes
// @start: states to start in for each class
#[repr(C)]
pub struct AaPolicydb {
    pub count: Kref,
    pub dfa: *mut AaDfa,
    pub perms: *mut AaPerms,
    pub size: u32,
    pub trans: AaStrTable,
    pub tags: AaTagsStruct,
    pub start: [AaStateT; 21],
}

extern "C" {
    pub static mut nullpdb: *mut AaPolicydb;

    pub fn aa_destroy_tags(tags: *mut AaTagsStruct);
    pub fn aa_alloc_pdb(gfp: u32) -> *mut AaPolicydb;
    pub fn aa_pdb_free_kref(kref: *mut Kref);
}

/// aa_get_pdb - increment refcount on @pdb
/// @pdb: policydb  (MAYBE NULL)
///
/// Returns: pointer to @pdb if @pdb is NULL will return NULL
/// Requires: @pdb must be held with valid refcount when called
#[inline]
pub unsafe fn aa_get_pdb(pdb: *mut AaPolicydb) -> *mut AaPolicydb {
    if !pdb.is_null() {
        kref_get(&mut (*pdb).count);
    }
    pdb
}

/// aa_put_pdb - put a pdb refcount
/// @pdb: pdb to put refcount   (MAYBE NULL)
///
/// Requires: if @pdb != NULL that a valid refcount be held
#[inline]
pub unsafe fn aa_put_pdb(pdb: *mut AaPolicydb) {
    if !pdb.is_null() {
        kref_put(&mut (*pdb).count, aa_pdb_free_kref);
    }
}

extern "C" {
    pub fn kref_get(kref: *mut Kref);
    pub fn kref_put(kref: *mut Kref, release: unsafe extern "C" fn(*mut Kref)) -> i32;
}

extern "C" {
    pub static default_perms: AaPerms;

    pub fn ACCEPT_TABLE(dfa: *mut AaDfa) -> *mut u32;
}

/// lookup perm that doesn't have an object conditional
#[inline]
pub unsafe fn aa_lookup_perms(policy: *mut AaPolicydb, state: AaStateT) -> *mut AaPerms {
    let index = *ACCEPT_TABLE((*policy).dfa).add(state as usize) as usize;

    if (*policy).perms.is_null() {
        return &default_perms as *const _ as *mut _;
    }

    (*policy).perms.add(index)
}

/// struct aa_data - generic data structure
/// key: name for retrieving this data
/// size: size of data in bytes
/// data: binary data
/// head: reserved for rhashtable
#[repr(C)]
pub struct AaData {
    pub key: *mut i8,
    pub size: u32,
    pub data: *mut i8,
    pub head: RhashHead,
}

extern "C" {
    pub struct RhashHead;
}

extern "C" {
    pub struct AaCaps;
    pub struct AaRlimit;
    pub struct AaSecmark;
}

/// struct aa_ruleset - data covering mediation rules
/// @list: list the rule is on
/// @size: the memory consumed by this ruleset
/// @policy: general match rules governing policy
/// @file: The set of rules governing basic file access and domain transitions
/// @caps: capabilities for the profile
/// @rlimits: rlimits for the profile
/// @secmark_count: number of secmark entries
/// @secmark: secmark label match info
#[repr(C)]
pub struct AaRuleset {
    pub size: i32,

    pub policy: *mut AaPolicydb,
    pub file: *mut AaPolicydb,
    pub caps: AaCaps,

    pub rlimits: AaRlimit,

    pub secmark_count: i32,
    pub secmark: *mut AaSecmark,
}

/// struct aa_attachment - data and rules for a profiles attachment
/// @list:
/// @xmatch_str: human readable attachment string
/// @xmatch: optional extended matching for unconfined executables names
/// @xmatch_len: xmatch prefix len, used to determine xmatch priority
/// @xattr_count: number of xattrs in table
/// @xattrs: table of xattrs
#[repr(C)]
pub struct AaAttachment {
    pub xmatch_str: *const i8,
    pub xmatch: *mut AaPolicydb,
    pub xmatch_len: u32,
    pub xattr_count: i32,
    pub xattrs: *mut *mut i8,
}

extern "C" {
    pub struct AaPolicy;
    pub struct AaLabel;
    pub struct AaLoaddata;
    pub struct Dentry;
    pub struct AaProxy;
}

/// struct aa_profile - basic confinement data
/// @base - base components of the profile (name, refcount, lists, lock ...)
/// @parent: parent of profile
/// @ns: namespace the profile is in
/// @rename: optional profile name that this profile renamed
///
/// @audit: the auditing mode of the profile
/// @mode: the enforcement mode of the profile
/// @path_flags: flags controlling path generation behavior
/// @signal: the signal that should be used when kill is used
/// @disconnected: what to prepend if attach_disconnected is specified
/// @attach: attachment rules for the profile
/// @rules: rules to be enforced
///
/// learning_cache: the accesses learned in complain mode
/// raw_data: rawdata of the loaded profile policy
/// hash: cryptographic hash of the profile
/// @dents: dentries for the profiles file entries in apparmorfs
/// @dirname: name of the profile dir in apparmorfs
/// @dents: set of dentries associated with the profile
/// @data: hashtable for free-form policy aa_data
/// @label - label this profile is an extension of
/// @rules - label with the rule vec on its end
///
/// The AppArmor profile contains the basic confinement data.  Each profile
/// has a name, and exists in a namespace.  The @name and @exec_match are
/// used to determine profile attachment against unconfined tasks.  All other
/// attachments are determined by profile X transition rules.
///
/// Profiles have a hierarchy where hats and children profiles keep
/// a reference to their parent.
///
/// Profile names can not begin with a : and can not contain the \0
/// character.  If a profile name begins with / it will be considered when
/// determining profile attachment on "unconfined" tasks.
#[repr(C)]
pub struct AaProfile {
    pub base: AaPolicy,
    pub parent: *mut AaProfile,

    pub ns: *mut AaNs,
    pub rename: *const i8,

    pub audit: i32,
    pub mode: i64,
    pub path_flags: u32,
    pub signal: i32,
    pub disconnected: *const i8,

    pub attach: AaAttachment,

    pub rawdata: *mut AaLoaddata,
    pub hash: *mut u8,
    pub dirname: *mut i8,
    pub dents: [*mut Dentry; 4],
    pub data: *mut std::ffi::c_void,

    pub n_rules: i32,
    pub label: AaLabel,
}

extern "C" {
    pub static mut aa_g_profile_mode: ProfileMode;
}

pub const AA_MAY_LOAD_POLICY: u32 = crate::AA_MAY_APPEND;
pub const AA_MAY_REPLACE_POLICY: u32 = crate::AA_MAY_WRITE;
pub const AA_MAY_REMOVE_POLICY: u32 = crate::AA_MAY_DELETE;

#[inline]
pub fn profiles_ns(p: *const AaProfile) -> *mut AaNs {
    unsafe { (*p).ns }
}

#[inline]
pub fn name_is_shared(a: *const AaProfile, b: *const AaProfile) -> bool {
    unsafe {
        !(*a).rename.is_null() && (*a).rename == (*b).rename
    }
}

extern "C" {
    pub fn aa_alloc_ruleset(gfp: u32) -> *mut AaRuleset;
    pub fn aa_alloc_profile(name: *const i8, proxy: *mut AaProxy, gfp: u32) -> *mut AaProfile;
    pub fn aa_alloc_null(parent: *mut AaProfile, name: *const i8, gfp: u32) -> *mut AaProfile;
    pub fn __aa_new_learning_profile(
        parent: *mut AaProfile,
        hat: bool,
        base: *const i8,
        gfp: u32,
    ) -> *mut AaProfile;
    pub fn aa_new_learning_profile(
        parent: *mut AaProfile,
        hat: bool,
        base: *const i8,
        gfp: u32,
    ) -> *mut AaProfile;
    pub fn aa_free_profile(profile: *mut AaProfile);
    pub fn aa_find_child(parent: *mut AaProfile, name: *const i8) -> *mut AaProfile;
    pub fn aa_lookupn_profile(ns: *mut AaNs, hname: *const i8, n: usize) -> *mut AaProfile;
    pub fn aa_fqlookupn_profile(base: *mut AaLabel, fqname: *const i8, n: usize) -> *mut AaProfile;

    pub fn aa_replace_profiles(
        view: *mut AaNs,
        label: *mut AaLabel,
        mask: u32,
        udata: *mut AaLoaddata,
        compressed_profile: *mut i8,
        compressed_size: usize,
    ) -> isize;
    pub fn aa_remove_profiles(view: *mut AaNs, label: *mut AaLabel, name: *mut i8, size: usize)
        -> isize;
    pub fn __aa_profile_list_release(head: *mut ListHead);
}

extern "C" {
    pub struct ListHead;
}

#[inline]
pub fn profile_unconfined(x: *const AaProfile) -> bool {
    unsafe { (*x).mode == ProfileMode::AppArmorUnconfined as i64 }
}

/// aa_get_newest_profile - simple wrapper fn to wrap the label version
/// @p: profile (NOT NULL)
///
/// Returns refcount to newest version of the profile (maybe @p)
///
/// Requires: @p must be held with a valid refcount
#[inline]
pub unsafe fn aa_get_newest_profile(p: *mut AaProfile) -> *mut AaProfile {
    labels_profile(aa_get_newest_label(&mut (*p).label))
}

extern "C" {
    pub fn aa_get_newest_label(label: *mut AaLabel) -> *mut AaLabel;
    pub fn labels_profile(label: *mut AaLabel) -> *mut AaProfile;
}

#[inline]
pub unsafe fn RULE_MEDIATES(rules: *mut AaRuleset, class: u8) -> AaStateT {
    const AA_CLASS_LAST: u8 = 20;

    if class <= AA_CLASS_LAST {
        (*(*rules).policy).start[class as usize]
    } else {
        aa_dfa_match_len(
            (*(*rules).policy).dfa,
            (*(*rules).policy).start[0],
            &class,
            1,
        )
    }
}

#[inline]
pub unsafe fn RULE_MEDIATES_v9NET(rules: *mut AaRuleset) -> AaStateT {
    RULE_MEDIATES(rules, 5)
}

#[inline]
pub unsafe fn RULE_MEDIATES_NET(rules: *mut AaRuleset) -> AaStateT {
    let state = RULE_MEDIATES(rules, 5);

    if state == 0 {
        RULE_MEDIATES(rules, 4)
    } else {
        state
    }
}

#[inline]
pub unsafe fn RULE_MEDIATES_UNIX(rules: *mut AaRuleset) -> AaStateT {
    RULE_MEDIATES_v9NET(rules)
}

extern "C" {
    pub fn aa_dfa_match_len(
        dfa: *mut AaDfa,
        start_state: AaStateT,
        data: *const u8,
        len: usize,
    ) -> AaStateT;

    pub fn aa_compute_profile_mediates(profile: *mut AaProfile);
    pub fn label_mediates(label: *mut AaLabel, class: u8) -> bool;
    pub fn label_mediates_safe(label: *mut AaLabel, class: u8) -> bool;
}

#[inline]
pub fn profile_mediates(profile: *const AaProfile, class: u8) -> bool {
    unsafe { label_mediates(std::mem::transmute(&(*profile).label), class) }
}

#[inline]
pub fn profile_mediates_safe(profile: *const AaProfile, class: u8) -> bool {
    unsafe { label_mediates_safe(std::mem::transmute(&(*profile).label), class) }
}

/// aa_get_profile - increment refcount on profile @p
/// @p: profile  (MAYBE NULL)
///
/// Returns: pointer to @p if @p is NULL will return NULL
/// Requires: @p must be held with valid refcount when called
#[inline]
pub unsafe fn aa_get_profile(p: *mut AaProfile) -> *mut AaProfile {
    if !p.is_null() {
        kref_get(&mut (*p).label.count.count);
    }

    p
}

/// aa_get_profile_not0 - increment refcount on profile @p found via lookup
/// @p: profile  (MAYBE NULL)
///
/// Returns: pointer to @p if @p is NULL will return NULL
/// Requires: @p must be held with valid refcount when called
#[inline]
pub unsafe fn aa_get_profile_not0(p: *mut AaProfile) -> *mut AaProfile {
    if !p.is_null() && kref_get_unless_zero(&mut (*p).label.count.count) != 0 {
        p
    } else {
        std::ptr::null_mut()
    }
}

extern "C" {
    pub fn kref_get_unless_zero(kref: *mut Kref) -> i32;
}

/// aa_get_profile_rcu - increment a refcount profile that can be replaced
/// @p: pointer to profile that can be replaced (NOT NULL)
///
/// Returns: pointer to a refcounted profile.
///     else NULL if no profile
#[inline]
pub unsafe fn aa_get_profile_rcu(p: *mut *mut AaProfile) -> *mut AaProfile {
    let mut c: *mut AaProfile;

    rcu_read_lock();
    loop {
        c = rcu_dereference(*p);
        if c.is_null() || kref_get_unless_zero(&mut (*c).label.count.count) != 0 {
            break;
        }
    }
    rcu_read_unlock();

    c
}

extern "C" {
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn rcu_dereference(p: *const *mut AaProfile) -> *mut AaProfile;
}

/// aa_put_profile - decrement refcount on profile @p
/// @p: profile  (MAYBE NULL)
#[inline]
pub unsafe fn aa_put_profile(p: *mut AaProfile) {
    if !p.is_null() {
        kref_put(&mut (*p).label.count.count, aa_label_kref);
    }
}

extern "C" {
    pub fn aa_label_kref(kref: *mut Kref);
}

#[inline]
pub fn AUDIT_MODE(profile: *const AaProfile) -> i32 {
    unsafe {
        if aa_g_audit != 0 {
            aa_g_audit
        } else {
            (*profile).audit
        }
    }
}

extern "C" {
    pub static aa_g_audit: i32;
}

extern "C" {
    pub fn aa_policy_view_capable(
        subj_cred: *const std::ffi::c_void,
        label: *mut AaLabel,
        ns: *mut AaNs,
    ) -> bool;
    pub fn aa_policy_admin_capable(
        subj_cred: *const std::ffi::c_void,
        label: *mut AaLabel,
        ns: *mut AaNs,
    ) -> bool;
    pub fn aa_may_manage_policy(
        subj_cred: *const std::ffi::c_void,
        label: *mut AaLabel,
        ns: *mut AaNs,
        ocred: *const std::ffi::c_void,
        mask: u32,
    ) -> i32;
    pub fn aa_current_policy_view_capable(ns: *mut AaNs) -> bool;
    pub fn aa_current_policy_admin_capable(ns: *mut AaNs) -> bool;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
