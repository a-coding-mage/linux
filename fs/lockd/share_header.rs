/* SPDX-License-Identifier: GPL-2.0 */
/*
 * DOS share management for lockd.
 *
 * Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
 */

/* Synthetic svid for lockowner lookup during share operations */
pub const LOCKD_SHARE_SVID: u32 = u32::MAX;

/* One bit per (access, deny) pair; index = (access << 2) | deny */
#[macro_export]
macro_rules! LOCKD_FSH_BIT {
	($a:expr, $d:expr) => {
		1u32 << ((($a) << 2) | ($d))
	};
}

/*
 * DOS share for a specific file
 */
#[repr(C)]
pub struct lockd_share {
	pub s_next: *mut lockd_share, /* linked list */
	pub s_host: *mut nlm_host, /* client host */
	pub s_file: *mut nlm_file, /* shared file */
	pub s_owner: xdr_netobj, /* owner handle */
	pub s_access: u32, /* access mode */
	pub s_mode: u32, /* deny mode */
	pub s_access_deny_bmap: u16, /* held (access, deny) pairs */
}

extern "C" {
	pub fn nlmsvc_share_file(
		host: *mut nlm_host,
		file: *mut nlm_file,
		oh: *mut xdr_netobj,
		access: u32,
		mode: u32,
	) -> __be32;
	pub fn nlmsvc_unshare_file(
		host: *mut nlm_host,
		file: *mut nlm_file,
		oh: *mut xdr_netobj,
		access: u32,
		mode: u32,
	) -> __be32;
	pub fn nlmsvc_traverse_shares(
		host: *mut nlm_host,
		file: *mut nlm_file,
		match_fn: nlm_host_match_fn_t,
	);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
