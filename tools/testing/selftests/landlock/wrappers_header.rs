/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Syscall wrappers
 *
 * Copyright © 2017-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2019-2020 ANSSI
 * Copyright © 2021-2025 Microsoft Corporation
 */

// C header context:
// #define _GNU_SOURCE
// #include <linux/landlock.h>
// #include <sys/syscall.h>
// #include <sys/types.h>
// #include <unistd.h>

use std::os::raw::{c_int, c_long, c_void};

unsafe extern "C" {
	fn syscall(num: c_long, ...) -> c_long;
}

// Original C condition: #ifndef landlock_create_ruleset
pub unsafe fn landlock_create_ruleset(
	attr: *const landlock_ruleset_attr,
	size: usize,
	flags: u32,
) -> c_int {
	unsafe {
		syscall(
			__NR_landlock_create_ruleset as c_long,
			attr,
			size,
			flags,
		) as c_int
	}
}

// Original C condition: #ifndef landlock_add_rule
pub unsafe fn landlock_add_rule(
	ruleset_fd: c_int,
	rule_type: landlock_rule_type,
	rule_attr: *const c_void,
	flags: u32,
) -> c_int {
	unsafe {
		syscall(
			__NR_landlock_add_rule as c_long,
			ruleset_fd,
			rule_type,
			rule_attr,
			flags,
		) as c_int
	}
}

// Original C condition: #ifndef landlock_restrict_self
pub unsafe fn landlock_restrict_self(ruleset_fd: c_int, flags: u32) -> c_int {
	unsafe {
		syscall(
			__NR_landlock_restrict_self as c_long,
			ruleset_fd,
			flags,
		) as c_int
	}
}

pub unsafe fn sys_gettid() -> pid_t {
	unsafe { syscall(__NR_gettid as c_long) as pid_t }
}
