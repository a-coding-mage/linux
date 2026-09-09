/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Transport Definition
 *
 *  Copyright (C) 2005 by Latchesar Ionkov <lucho@ionkov.net>
 *  Copyright (C) 2004-2008 by Eric Van Hensbergen <ericvh@gmail.com>
 */

// Dependency declarations supplied by other translation units are intentionally
// left as external types and symbols.

pub const P9_DEF_MIN_RESVPORT: u32 = 665u32;
pub const P9_DEF_MAX_RESVPORT: u32 = 1023u32;

pub const P9_FD_PORT: i32 = 564;

pub const P9_RDMA_PORT: i32 = 5640;
pub const P9_RDMA_SQ_DEPTH: i32 = 32;
pub const P9_RDMA_RQ_DEPTH: i32 = 32;
pub const P9_RDMA_TIMEOUT: i32 = 30000; // 30 seconds

#[repr(C)]
pub struct p9_trans_module {
	pub list: list_head,
	pub name: *mut core::ffi::c_char, // name of transport
	pub maxsize: i32, // max message size of transport
	pub pooled_rbuffers: bool,
	pub def: bool, // this transport should be default
	pub supports_vmalloc: bool, // can work with vmalloc'd buffers
	pub owner: *mut module,
	pub create: Option<unsafe extern "C" fn(client: *mut p9_client, fc: *mut fs_context) -> i32>,
	pub close: Option<unsafe extern "C" fn(client: *mut p9_client)>,
	pub request: Option<unsafe extern "C" fn(client: *mut p9_client, req: *mut p9_req_t) -> i32>,
	pub cancel: Option<unsafe extern "C" fn(client: *mut p9_client, req: *mut p9_req_t) -> i32>,
	pub cancelled: Option<unsafe extern "C" fn(client: *mut p9_client, req: *mut p9_req_t) -> i32>,
	pub zc_request: Option<unsafe extern "C" fn(
		client: *mut p9_client,
		req: *mut p9_req_t,
		uidata: *mut iov_iter,
		uodata: *mut iov_iter,
		inlen: i32,
		outlen: i32,
		in_hdr_len: i32,
	) -> i32>,
	pub show_options: Option<unsafe extern "C" fn(m: *mut seq_file, client: *mut p9_client) -> i32>,
}

unsafe extern "C" {
	pub fn v9fs_register_trans(m: *mut p9_trans_module);
	pub fn v9fs_unregister_trans(m: *mut p9_trans_module);
	pub fn v9fs_get_trans_by_name(s: *const core::ffi::c_char) -> *mut p9_trans_module;
	pub fn v9fs_get_default_trans() -> *mut p9_trans_module;
	pub fn v9fs_put_trans(m: *mut p9_trans_module);
}

// MODULE_ALIAS_9P(transport) expands to MODULE_ALIAS("9p-" transport) in C.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
