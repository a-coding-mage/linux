// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - Credential hooks
 *
 * Copyright © 2017-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2018-2020 ANSSI
 * Copyright © 2024-2025 Microsoft Corporation
 */

/* C dependencies:
 * #include <linux/binfmts.h>
 * #include <linux/cred.h>
 * #include <linux/lsm_hooks.h>
 *
 * #include "common.h"
 * #include "cred.h"
 * #include "ruleset.h"
 * #include "setup.h"
 */

use core::ffi::{c_int, c_uint};

pub type gfp_t = c_uint;

#[repr(C)]
pub struct cred {
	_private: [u8; 0],
}

#[repr(C)]
pub struct linux_binprm {
	pub cred: *mut cred,
}

#[repr(C)]
pub struct landlock_domain {
	_private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_cred_security {
	pub domain: *mut landlock_domain,
	/* Only used when CONFIG_SECURITY_LANDLOCK_LOG is enabled. */
	pub domain_exec: u64,
}

#[repr(C)]
pub struct security_hook_list {
	_private: [u8; 0],
}

unsafe extern "C" {
	fn landlock_cred(cred: *const cred) -> *mut landlock_cred_security;
	fn landlock_get_domain(domain: *mut landlock_domain);
	fn landlock_put_domain_deferred(domain: *mut landlock_domain);
	fn security_add_hooks(
		hooks: *mut security_hook_list,
		count: usize,
		lsmid: *const core::ffi::c_void,
	);

	static landlock_lsmid: core::ffi::c_void;
}

unsafe extern "C" fn hook_cred_transfer(new: *mut cred, old: *const cred) {
	let old_llcred: *const landlock_cred_security = unsafe { landlock_cred(old) };

	unsafe {
		landlock_get_domain((*old_llcred).domain);
		*landlock_cred(new) = *old_llcred;
	}
}

unsafe extern "C" fn hook_cred_prepare(
	new: *mut cred,
	old: *const cred,
	_gfp: gfp_t,
) -> c_int {
	unsafe {
		hook_cred_transfer(new, old);
	}
	0
}

unsafe extern "C" fn hook_cred_free(cred: *mut cred) {
	let dom: *mut landlock_domain = unsafe { (*landlock_cred(cred)).domain };

	if !dom.is_null() {
		unsafe {
			landlock_put_domain_deferred(dom);
		}
	}
}

/* CONFIG_SECURITY_LANDLOCK_LOG */
unsafe extern "C" fn hook_bprm_creds_for_exec(bprm: *mut linux_binprm) -> c_int {
	/* Resets for each execution. */
	unsafe {
		(*landlock_cred((*bprm).cred)).domain_exec = 0;
	}
	0
}

/* __ro_after_init */
#[cfg(not(CONFIG_SECURITY_LANDLOCK_LOG))]
static mut landlock_hooks: [security_hook_list; 3] = [
	LSM_HOOK_INIT!(cred_prepare, hook_cred_prepare),
	LSM_HOOK_INIT!(cred_transfer, hook_cred_transfer),
	LSM_HOOK_INIT!(cred_free, hook_cred_free),
];

/* __ro_after_init */
#[cfg(CONFIG_SECURITY_LANDLOCK_LOG)]
static mut landlock_hooks: [security_hook_list; 4] = [
	LSM_HOOK_INIT!(cred_prepare, hook_cred_prepare),
	LSM_HOOK_INIT!(cred_transfer, hook_cred_transfer),
	LSM_HOOK_INIT!(cred_free, hook_cred_free),
	LSM_HOOK_INIT!(bprm_creds_for_exec, hook_bprm_creds_for_exec),
];

/* __init */
pub unsafe extern "C" fn landlock_add_cred_hooks() {
	unsafe {
		security_add_hooks(
			core::ptr::addr_of_mut!(landlock_hooks).cast::<security_hook_list>(),
			landlock_hooks.len(),
			core::ptr::addr_of!(landlock_lsmid),
		);
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
