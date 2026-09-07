// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor contexts used to associate "labels" to objects.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.

// TODO: Depends on external symbols from linux/cred, linux/slab, linux/sched
// TODO: Depends on external modules: label, policy_ns, task

#[repr(C)]
pub struct AaLabel {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct Cred {
    pub security: *mut u8,
}

#[repr(C)]
pub struct AaNs {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct AppArmorBlobSizes {
    pub lbs_cred: usize,
}

extern "C" {
    pub static apparmor_blob_sizes: AppArmorBlobSizes;
    pub fn current_cred() -> *const Cred;
    pub fn aa_get_newest_label(label: *mut AaLabel) -> *mut AaLabel;
    pub fn label_is_stale(label: *const AaLabel) -> bool;
    pub fn aa_put_label(label: *mut AaLabel);
    pub fn aa_get_label(label: *const AaLabel) -> *mut AaLabel;
    pub fn labels_ns(label: *const AaLabel) -> *mut AaNs;
    pub fn aa_get_ns(ns: *mut AaNs) -> *mut AaNs;
    pub fn aa_schedule_stale_label_replacement();
}

#[inline]
pub unsafe fn cred_label(cred: *const Cred) -> *mut AaLabel {
    let blob = ((*cred).security.add(apparmor_blob_sizes.lbs_cred)) as *mut *mut AaLabel;

    debug_assert!(!blob.is_null(), "blob is null");
    *blob
}

#[inline]
pub unsafe fn set_cred_label(cred: *const Cred, label: *mut AaLabel) {
    let blob = ((*cred).security.add(apparmor_blob_sizes.lbs_cred)) as *mut *mut AaLabel;

    debug_assert!(!blob.is_null(), "blob is null");
    *blob = label;
}

/// aa_get_newest_cred_label - obtain the newest label on a cred
/// @cred: cred to obtain label from (NOT NULL)
///
/// Returns: newest version of confining label
#[inline]
pub unsafe fn aa_get_newest_cred_label(cred: *const Cred) -> *mut AaLabel {
    aa_get_newest_label(cred_label(cred))
}

#[inline]
pub unsafe fn aa_get_newest_cred_label_condref(cred: *const Cred, needput: *mut bool) -> *mut AaLabel {
    let l = cred_label(cred);

    if label_is_stale(l) {
        *needput = true;
        return aa_get_newest_label(l);
    }

    *needput = false;
    l
}

#[inline]
pub unsafe fn aa_put_label_condref(l: *mut AaLabel, needput: bool) {
    if needput {
        aa_put_label(l);
    }
}

/// aa_current_raw_label - find the current tasks confining label
///
/// Returns: up to date confining label or the ns unconfined label (NOT NULL)
///
/// This fn will not update the tasks cred to the most up to date version
/// of the label so it is safe to call when inside of locks.
#[inline]
pub unsafe fn aa_current_raw_label() -> *mut AaLabel {
    cred_label(current_cred())
}

/// aa_get_current_label - get the newest version of the current tasks label
///
/// Returns: newest version of confining label (NOT NULL)
///
/// This fn will not update the tasks cred, so it is safe inside of locks
///
/// The returned reference must be put with aa_put_label()
#[inline]
pub unsafe fn aa_get_current_label() -> *mut AaLabel {
    let l = aa_current_raw_label();

    if label_is_stale(l) {
        return aa_get_newest_label(l);
    }
    aa_get_label(l)
}

/// __end_cred_crit_section - end crit section begun with __begin_...
/// @label: label obtained from __begin_cred_crit_section
/// @needput: output: bool set by __begin_cred_crit_section
///
/// While the cred passed to __begin is guaranteed to not change
/// and the cred and label could be passed here instead of needput
/// using needput with a local var makes it easier for the compiler
/// and processor to optimize and speculatively execute the comparison
/// than chasing a pointer in the cred struct.
#[inline]
pub unsafe fn __end_cred_crit_section(label: *mut AaLabel, needput: bool) {
    if needput {
        aa_put_label(label);
    }
}

/// __begin_cred_crit_section - @cred's confining label
/// @cred: current's cred to start a crit section on its label
/// @needput: store whether the label needs to be put when ending crit section
///
/// Returns: up to date confining label or the ns unconfined label (NOT NULL)
///
/// safe to call inside locks
///
/// The returned reference must be put with __end_cred_crit_section()
/// This must NOT be used if the task cred could be updated within the
/// critical section between
///   __begin_cred_crit_section() ..  __end_cred_crit_section()
///
/// The crit section is an optimization to avoid having to get and put
/// the newest version of the label. While the cred won't change and
/// hence the label it contains won't change, the newest version of the
/// label can. During the crit section the newest versions of the label
/// will be used until the end of the crit section.
///
/// If the label has not been updated at the start of the crit section
/// no refcount is taken, the cred's refcount is enough to hold the
/// label for the duration of the crit section.
///
/// If the label has been updated then a refcount will be taken and the
/// newest version of the label will be returned. While the cred label
/// and the returned label could be compared at the end of the crit
/// section, needput is used because it allows better optimization by
/// the compiler and the processor's speculative execution.
#[inline]
pub unsafe fn __begin_cred_crit_section(cred: *const Cred, needput: *mut bool) -> *mut AaLabel {
    let label = cred_label(cred);

    if label_is_stale(label) {
        *needput = true;
        return aa_get_newest_label(label);
    }

    *needput = false;
    label
}

/// __end_current_label_crit_section - end crit section begun with __begin_...
/// @label: label obtained from __begin_current_label_crit_section
/// @needput: output: bool set by __begin_current_label_crit_section
///
/// wrapper around __end_cred_crit_section() to pair nicely with
/// __begin_current_label_crit_section()
#[inline]
pub unsafe fn __end_current_label_crit_section(label: *mut AaLabel, needput: bool) {
    __end_cred_crit_section(label, needput);
}

/// end_current_label_crit_section - put a reference found with begin_current_label..
/// @label: label reference to put
/// @needput: output: bool set by __begin_current_label_crit_section
///
/// Should only be used with a reference obtained with
/// begin_current_label_crit_section and never used in situations where the
/// task cred may be updated
#[inline]
pub unsafe fn end_current_label_crit_section(label: *mut AaLabel, needput: bool) {
    if label != aa_current_raw_label() {
        aa_put_label(label);
    }
}

/// __begin_current_label_crit_section - current's confining label
/// @needput: store whether the label needs to be put when ending crit section
///
/// Returns: up to date confining label or the ns unconfined label (NOT NULL)
///
/// safe to call inside locks
///
/// The returned reference must be put with __end_current_label_crit_section()
/// This must NOT be used if the task cred could be updated within the
/// critical section between __begin_current_label_crit_section() ..
/// __end_current_label_crit_section()
#[inline]
pub unsafe fn __begin_current_label_crit_section(needput: *mut bool) -> *mut AaLabel {
    __begin_cred_crit_section(current_cred(), needput)
}

/// begin_current_label_crit_section - current's confining label and update it
/// @needput: store whether the label needs to be put when ending crit section
///
/// Returns: up to date confining label or the ns unconfined label (NOT NULL)
///
/// The returned reference must be put with end_current_label_crit_section()
/// This should NOT be used if the task cred could be updated within the
/// critical section between begin_current_label_crit_section() ..
/// end_current_label_crit_section()
#[inline]
pub unsafe fn begin_current_label_crit_section(needput: *mut bool) -> *mut AaLabel {
    let label = aa_current_raw_label();

    let label = __begin_current_label_crit_section(needput);
    if *needput {
        aa_schedule_stale_label_replacement();
    }

    label
}

#[inline]
pub unsafe fn aa_get_current_ns() -> *mut AaNs {
    let mut needput: bool = false;

    let label = __begin_current_label_crit_section(&mut needput);
    let ns = aa_get_ns(labels_ns(label));
    __end_current_label_crit_section(label, needput);

    ns
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
