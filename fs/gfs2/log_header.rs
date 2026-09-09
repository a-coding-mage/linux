/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Dependencies supplied by the corresponding translated kernel and GFS2 headers.

/*
 * The minimum amount of log space required for a log flush is one block for
 * revokes and one block for the log header.  Log flushes other than
 * GFS2_LOG_HEAD_FLUSH_NORMAL may write one or two more log headers.
 */
pub const GFS2_LOG_FLUSH_MIN_BLOCKS: u32 = 4;

pub unsafe fn gfs2_ordered_add_inode(ip: *mut gfs2_inode) {
	let sdp: *mut gfs2_sbd = GFS2_SB(&mut (*ip).i_inode);

	if gfs2_is_jdata(ip) || !gfs2_is_ordered(sdp) {
		return;
	}

	if list_empty(&(*ip).i_ordered) {
		spin_lock(&mut (*sdp).sd_ordered_lock);
		if list_empty(&(*ip).i_ordered) {
			list_add(&mut (*ip).i_ordered, &mut (*sdp).sd_log_ordered);
		}
		spin_unlock(&mut (*sdp).sd_ordered_lock);
	}
}

extern "C" {
	pub fn gfs2_ordered_del_inode(ip: *mut gfs2_inode);
	pub fn gfs2_struct2blk(sdp: *mut gfs2_sbd, nstruct: u32) -> u32;
	pub fn gfs2_log_is_empty(sdp: *mut gfs2_sbd) -> bool;
	pub fn gfs2_log_release_revokes(sdp: *mut gfs2_sbd, revokes: u32);
	pub fn gfs2_log_release(sdp: *mut gfs2_sbd, blks: u32);
	pub fn gfs2_log_try_reserve(
		sdp: *mut gfs2_sbd,
		tr: *mut gfs2_trans,
		extra_revokes: *mut u32,
	) -> bool;
	pub fn gfs2_log_reserve(
		sdp: *mut gfs2_sbd,
		tr: *mut gfs2_trans,
		extra_revokes: *mut u32,
	);
	pub fn gfs2_write_log_header(
		sdp: *mut gfs2_sbd,
		jd: *mut gfs2_jdesc,
		seq: u64,
		tail: u32,
		lblock: u32,
		flags: u32,
		op_flags: blk_opf_t,
	);
	pub fn gfs2_remove_from_journal(bh: *mut buffer_head, meta: i32);
	pub fn gfs2_log_flush(sdp: *mut gfs2_sbd, gl: *mut gfs2_glock, r#type: u32);
	pub fn gfs2_log_commit(sdp: *mut gfs2_sbd, trans: *mut gfs2_trans);
	pub fn gfs2_ail1_flush(sdp: *mut gfs2_sbd, wbc: *mut writeback_control);
	pub fn log_flush_wait(sdp: *mut gfs2_sbd);

	pub fn gfs2_logd(data: *mut core::ffi::c_void) -> i32;
	pub fn gfs2_add_revoke(sdp: *mut gfs2_sbd, bd: *mut gfs2_bufdata);
	pub fn gfs2_glock_remove_revoke(gl: *mut gfs2_glock);
	pub fn gfs2_flush_revokes(sdp: *mut gfs2_sbd);
	pub fn gfs2_ail_drain(sdp: *mut gfs2_sbd);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
