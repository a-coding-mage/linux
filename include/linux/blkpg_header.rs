/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Partition table and disk geometry handling
 */

/* Dependencies supplied by the corresponding Linux compatibility and UAPI headers. */

#[cfg(CONFIG_COMPAT)]
/* For 32-bit/64-bit compatibility of struct blkpg_ioctl_arg */
#[repr(C)]
pub struct blkpg_compat_ioctl_arg {
	pub op: compat_int_t,
	pub flags: compat_int_t,
	pub datalen: compat_int_t,
	pub data: compat_uptr_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
