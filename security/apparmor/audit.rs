// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor auditing functions
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 */

// Linux audit and socket headers are provided by external dependencies
// #include <linux/audit.h>
// #include <linux/socket.h>
// #include "include/apparmor.h"
// #include "include/audit.h"
// #include "include/policy.h"
// #include "include/policy_ns.h"
// #include "include/secid.h"

pub const AUDIT_MODE_NAMES: &[&str] = &[
	"normal",
	"quiet_denied",
	"quiet.allowed",
	"quiet",
	"noquiet",
	"all"
];

static AA_AUDIT_TYPE: &[&str] = &[
	"AUDIT",
	"ALLOWED",
	"DENIED",
	"HINT",
	"STATUS",
	"ERROR",
	"KILLED",
	"AUTO"
];

static AA_CLASS_NAMES: &[&str] = &[
	"none",
	"unknown",
	"file",
	"cap",
	"net",
	"rlimits",
	"domain",
	"mount",
	"unknown",
	"ptrace",
	"signal",
	"xmatch",
	"unknown",
	"unknown",
	"net",
	"netv9",
	"label",
	"posix_mqueue",
	"io_uring",
	"module",
	"lsm",
	"namespace",
	"io_uring",
	"unknown",
	"unknown",
	"unknown",
	"unknown",
	"unknown",
	"unknown",
	"unknown",
	"netv9_packet",
	"X",
	"dbus",
];

/*
 * Currently AppArmor auditing is fed straight into the audit framework.
 *
 * TODO:
 * netlink interface for complain mode
 * user auditing, - send user auditing to netlink interface
 * system control of whether user audit messages go to system log
 */

// External types and functions from other modules
extern "C" {
	pub struct audit_buffer;
	pub struct apparmor_audit_data;
	pub struct aa_label;
	pub struct aa_profile;
	pub struct label_it;
	pub struct audit_krule;
	pub struct audit_field;
	pub struct lsm_prop;

	pub static aa_g_audit_header: u32;
	pub static root_ns: *mut core::ffi::c_void;

	fn aad_of_va(va: *mut core::ffi::c_void) -> *mut apparmor_audit_data;
	fn audit_log_format(ab: *mut audit_buffer, fmt: *const u8, ...);
	fn audit_log_untrustedstring(ab: *mut audit_buffer, s: *const u8);
	fn label_isprofile(label: *mut aa_label) -> i32;
	fn labels_profile(label: *mut aa_label) -> *mut aa_profile;
	fn aa_label_xaudit(ab: *mut audit_buffer, ns: *mut core::ffi::c_void,
	                    label: *mut aa_label, flags: u32, gfp: u32) -> i32;
	fn common_lsm_audit(common: *mut core::ffi::c_void,
	                     audit_pre: extern "C" fn(*mut audit_buffer, *mut core::ffi::c_void),
	                     cb: Option<extern "C" fn(*mut audit_buffer, *mut core::ffi::c_void)>);
	fn send_sig_info(sig: i32, info: *mut core::ffi::c_void, t: *mut core::ffi::c_void) -> i32;
	fn aa_put_label(label: *mut aa_label);
	fn kfree(ptr: *mut core::ffi::c_void);
	fn kzalloc_obj(size: usize, gfp: u32) -> *mut core::ffi::c_void;
	fn aa_label_parse(label: *mut aa_label, rulestr: *const u8, gfp: u32,
	                   b1: bool, b2: bool) -> *mut aa_label;
	fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
	fn PTR_ERR(ptr: *const core::ffi::c_void) -> i32;
	fn aa_label_is_subset(label: *mut aa_label, subset: *mut aa_label) -> i32;
	fn complain_error(error: i32) -> i32;
}

// Macros that reference external constants/functions
// These would typically come from included headers
extern "C" {
	fn AUDIT_MODE(profile: *mut aa_profile) -> u32;
	fn COMPLAIN_MODE(profile: *mut aa_profile) -> bool;
	fn KILL_MODE(profile: *mut aa_profile) -> bool;
	fn likely(x: u32) -> u32;
}

// Constants from external headers
const AUDIT_APPARMOR_AUDIT: i32 = 0;
const AUDIT_APPARMOR_KILL: i32 = 1;
const AUDIT_APPARMOR_ALLOWED: i32 = 2;
const AUDIT_APPARMOR_DENIED: i32 = 3;
const AUDIT_APPARMOR_AUTO: i32 = 4;

const AA_CLASS_LAST: u32 = 32;
const FLAG_VIEW_SUBNS: u32 = 1;
const GFP_ATOMIC: u32 = 0;
const LSM_AUDIT_DATA_TASK: u32 = 1;
const SEND_SIG_NOINFO: i32 = 0;
const AUDIT_SUBJ_ROLE: u32 = 1;
const Audit_equal: u32 = 0;
const Audit_not_equal: u32 = 1;
const EINVAL: i32 = -22;
const ENOMEM: i32 = -12;
const ENOENT: i32 = -2;

pub struct aa_perms {
	pub kill: u32,
	pub complain: u32,
}

pub static NULLPERMS: aa_perms = aa_perms { kill: 0, complain: 0 };

#[repr(C)]
pub struct aa_audit_rule {
	pub label: *mut aa_label,
}

/**
 * audit_pre() - core AppArmor function.
 * @ab: audit buffer to fill (NOT NULL)
 * @va: audit structure containing data to audit (NOT NULL)
 *
 * Record common AppArmor audit data from @va
 */
unsafe extern "C" fn audit_pre(ab: *mut audit_buffer, va: *mut core::ffi::c_void) {
	let ad = aad_of_va(va);

	if aa_g_audit_header != 0 {
		audit_log_format(ab, b"apparmor=\"%s\"\0".as_ptr(),
		                 AA_AUDIT_TYPE[(*ad).type as usize].as_ptr());
	}

	if !(*ad).op.is_null() {
		audit_log_format(ab, b" operation=\"%s\"\0".as_ptr(), (*ad).op);
	}

	if (*ad).class != 0 {
		let class_name = if (*ad).class <= AA_CLASS_LAST {
			AA_CLASS_NAMES[(*ad).class as usize].as_ptr()
		} else {
			b"unknown\0".as_ptr()
		};
		audit_log_format(ab, b" class=\"%s\"\0".as_ptr(), class_name);
	}

	if !(*ad).info.is_null() {
		audit_log_format(ab, b" info=\"%s\"\0".as_ptr(), (*ad).info);
		if (*ad).error != 0 {
			audit_log_format(ab, b" error=%d\0".as_ptr(), (*ad).error);
		}
	}

	if !(*ad).subj_label.is_null() {
		let label = (*ad).subj_label;

		if label_isprofile(label) != 0 {
			let profile = labels_profile(label);

			if (*profile).ns != root_ns {
				audit_log_format(ab, b" namespace=\0".as_ptr());
				audit_log_untrustedstring(ab,
				                         (*((*profile).ns as *mut core::ffi::c_void) as *mut u8));
			}
			audit_log_format(ab, b" profile=\0".as_ptr());
			audit_log_untrustedstring(ab, ((*profile).base.hname as *const u8));
		} else {
			audit_log_format(ab, b" label=\0".as_ptr());
			aa_label_xaudit(ab, root_ns, label, FLAG_VIEW_SUBNS, GFP_ATOMIC);
		}
	}

	if !(*ad).name.is_null() {
		audit_log_format(ab, b" name=\0".as_ptr());
		audit_log_untrustedstring(ab, (*ad).name);
	}
}

pub extern "C" fn aa_select_audit_type(denied: u32, perms: *const aa_perms) -> i32 {
	if likely(denied == 0) != 0 {
		return AUDIT_APPARMOR_AUDIT;
	} else if denied & unsafe { (*perms).kill } != 0 {
		return AUDIT_APPARMOR_KILL;
	} else if denied == (denied & unsafe { (*perms).complain }) {
		return AUDIT_APPARMOR_ALLOWED;
	}
	AUDIT_APPARMOR_DENIED
}

/**
 * aa_audit_msg - Log a message to the audit subsystem
 * @type: audit type for the message
 * @ad: audit event structure (NOT NULL)
 * @cb: optional callback fn for type specific fields (MAYBE NULL)
 */
pub unsafe extern "C" fn aa_audit_msg(
	type_: i32,
	ad: *mut apparmor_audit_data,
	cb: Option<extern "C" fn(*mut audit_buffer, *mut core::ffi::c_void)>,
) {
	(*ad).type_ = type_;
	// Assuming common_lsm_audit signature takes a reference to common field
	// and the audit_pre function pointer and optional callback
	common_lsm_audit((ad as *mut core::ffi::c_void), audit_pre, cb);
}

pub unsafe extern "C" fn aa_audit_perm_error(
	label: *mut aa_label,
	request: u32,
	error: i32,
	ad: *mut apparmor_audit_data,
	cb: Option<extern "C" fn(*mut audit_buffer, *mut core::ffi::c_void)>,
) -> i32 {
	let type_ = aa_select_audit_type(request, &NULLPERMS);

	if !ad.is_null() {
		let mut i: label_it = core::mem::zeroed();

		(*ad).request = request;
		(*ad).denied = request;
		(*ad).error = error;

		label_for_each_confined(&mut i, label, |profile: *mut aa_profile| {
			(*ad).subj_label = &(*profile).label;
			aa_audit_msg(type_, ad, cb);
		});
	}

	error
}

/**
 * aa_audit - Log a profile based audit event to the audit subsystem
 * @type: audit type for the message
 * @profile: profile to check against (NOT NULL)
 * @ad: audit event (NOT NULL)
 * @cb: optional callback fn for type specific fields (MAYBE NULL)
 *
 * Handle default message switching based off of audit mode flags
 *
 * Returns: error on failure
 */
pub unsafe extern "C" fn aa_audit(
	type_: i32,
	profile: *mut aa_profile,
	ad: *mut apparmor_audit_data,
	cb: Option<extern "C" fn(*mut audit_buffer, *mut core::ffi::c_void)>,
) -> i32 {
	// AA_BUG(!profile);
	debug_assert!(!profile.is_null());

	let mut audit_type = type_;

	if audit_type == AUDIT_APPARMOR_AUTO {
		if likely((*ad).error == 0) != 0 {
			if AUDIT_MODE(profile) != 0 {
				return 0;
			}
			audit_type = AUDIT_APPARMOR_AUDIT;
		} else if COMPLAIN_MODE(profile) {
			audit_type = AUDIT_APPARMOR_ALLOWED;
		} else {
			audit_type = AUDIT_APPARMOR_DENIED;
		}
	}

	if AUDIT_MODE(profile) == 0 ||
	   (audit_type == AUDIT_APPARMOR_DENIED && AUDIT_MODE(profile) == 0) {
		return (*ad).error;
	}

	if KILL_MODE(profile) && audit_type == AUDIT_APPARMOR_DENIED {
		audit_type = AUDIT_APPARMOR_KILL;
	}

	(*ad).subj_label = &(*profile).label;

	aa_audit_msg(audit_type, ad, cb);

	if (*ad).type_ == AUDIT_APPARMOR_KILL {
		let tsk = if (*ad).common.type_ == LSM_AUDIT_DATA_TASK && !(*ad).common.u.tsk.is_null() {
			(*ad).common.u.tsk
		} else {
			core::ptr::null_mut() // current in kernel
		};
		send_sig_info((*profile).signal, core::ptr::null_mut(), tsk);
	}

	if (*ad).type_ == AUDIT_APPARMOR_ALLOWED {
		return complain_error((*ad).error);
	}

	(*ad).error
}

pub unsafe extern "C" fn aa_audit_rule_free(vrule: *mut core::ffi::c_void) {
	let rule = vrule as *mut aa_audit_rule;

	if !rule.is_null() {
		if !IS_ERR((*rule).label as *const core::ffi::c_void) {
			aa_put_label((*rule).label);
		}
		kfree(rule as *mut core::ffi::c_void);
	}
}

pub unsafe extern "C" fn aa_audit_rule_init(
	field: u32,
	op: u32,
	rulestr: *mut u8,
	vrule: *mut *mut core::ffi::c_void,
	gfp: u32,
) -> i32 {
	match field {
		AUDIT_SUBJ_ROLE => {
			if op != Audit_equal && op != Audit_not_equal {
				return EINVAL;
			}
		}
		_ => {
			return EINVAL;
		}
	}

	let rule = kzalloc_obj(core::mem::size_of::<aa_audit_rule>(), gfp) as *mut aa_audit_rule;

	if rule.is_null() {
		return ENOMEM;
	}

	// Currently rules are treated as coming from the root ns
	(*rule).label = aa_label_parse(
		&(*(root_ns as *mut core::ffi::c_void)) as *const _ as *mut aa_label,
		rulestr,
		gfp,
		true,
		false,
	);

	if IS_ERR((*rule).label as *const core::ffi::c_void) {
		let err = PTR_ERR((*rule).label as *const core::ffi::c_void);
		aa_audit_rule_free(rule as *mut core::ffi::c_void);
		return err;
	}

	*vrule = rule as *mut core::ffi::c_void;
	0
}

pub unsafe extern "C" fn aa_audit_rule_known(rule: *mut audit_krule) -> i32 {
	for i in 0..(*rule).field_count {
		let f = &(*rule).fields[i as usize];

		match (*f).type_ {
			AUDIT_SUBJ_ROLE => {
				return 1;
			}
			_ => {}
		}
	}

	0
}

pub unsafe extern "C" fn aa_audit_rule_match(
	prop: *mut lsm_prop,
	field: u32,
	op: u32,
	vrule: *mut core::ffi::c_void,
) -> i32 {
	let rule = vrule as *mut aa_audit_rule;
	let label = (*prop).apparmor.label;
	let mut found = 0;

	if label.is_null() {
		return ENOENT;
	}

	if aa_label_is_subset(label, (*rule).label) != 0 {
		found = 1;
	}

	match field {
		AUDIT_SUBJ_ROLE => {
			match op {
				Audit_equal => {
					return found;
				}
				Audit_not_equal => {
					return if found != 0 { 0 } else { 1 };
				}
				_ => {}
			}
		}
		_ => {}
	}
	0
}

// Helper macro for label iteration (would be defined in external headers)
// For now, a stub that would need to be implemented externally
#[inline]
unsafe fn label_for_each_confined<F>(i: &mut label_it, label: *mut aa_label, mut callback: F)
where
	F: FnMut(*mut aa_profile),
{
	// This would be implemented in the external label iteration code
	// Placeholder for external macro behavior
	let _ = (i, label, callback);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
