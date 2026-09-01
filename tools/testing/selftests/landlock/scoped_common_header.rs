/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Landlock scope test helpers
 *
 * Copyright © 2024 Tahera Fahimi <fahimitahera@gmail.com>
 */

// C source defined _GNU_SOURCE and included <sys/types.h>.

unsafe fn create_scoped_domain(_metadata: *mut __test_metadata, scope: __u16) {
	let ruleset_fd: libc::c_int;
	let ruleset_attr: landlock_ruleset_attr = landlock_ruleset_attr {
		scoped: scope,
		..unsafe { core::mem::zeroed() }
	};

	ruleset_fd = unsafe {
		landlock_create_ruleset(
			&ruleset_attr as *const landlock_ruleset_attr,
			core::mem::size_of_val(&ruleset_attr),
			0,
		)
	};
	if !(0 <= ruleset_fd) {
		TH_LOG!(
			"Failed to create a ruleset: %s",
			unsafe { strerror(errno) }
		);
	}
	unsafe { enforce_ruleset(_metadata, ruleset_fd) };
	EXPECT_EQ!(0, unsafe { close(ruleset_fd) });
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
