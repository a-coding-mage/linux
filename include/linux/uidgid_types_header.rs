/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of: #include <linux/types.h>

#[repr(C)]
pub struct kuid_t {
	pub val: uid_t,
}

#[repr(C)]
pub struct kgid_t {
	pub val: gid_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
