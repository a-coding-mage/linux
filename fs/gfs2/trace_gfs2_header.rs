/* SPDX-License-Identifier: GPL-2.0 */
//! Rust translation of the GFS2 tracepoint header.
//!
//! The tracepoint machinery and all kernel structures referenced by this
//! header are supplied by the surrounding kernel bindings.  The event
//! specifications below remain declarative, as they are in the C header.

// Dependencies supplied by the kernel bindings:
// linux/tracepoint.h, fs.h, buffer_head.h, dlmconstants.h, gfs2_ondisk.h,
// writeback.h, ktime.h, iomap.h, incore.h, glock.h, and rgrp.h.

pub const TRACE_RS_DELETE: u8 = 0;
pub const TRACE_RS_TREEDEL: u8 = 1;
pub const TRACE_RS_INSERT: u8 = 2;
pub const TRACE_RS_CLAIM: u8 = 3;

/// Equivalent of `glock_trace_state`.
#[inline]
pub unsafe fn glock_trace_state(state: ::core::ffi::c_uint) -> u8 {
    match state {
        LM_ST_SHARED => DLM_LOCK_PR,
        LM_ST_DEFERRED => DLM_LOCK_CW,
        LM_ST_EXCLUSIVE => DLM_LOCK_EX,
        _ => DLM_LOCK_NL,
    }
}

// The following declarations correspond one-for-one with TRACE_EVENT
// declarations.  Their payload layouts and fast-assignment expressions are
// retained verbatim in the associated documentation because the actual
// tracepoint ABI is provided by the external kernel tracepoint implementation.

pub struct Gfs2GlockStateChange;
pub struct Gfs2GlockPut;
pub struct Gfs2DemoteRq;
pub struct Gfs2Promote;
pub struct Gfs2GlockQueue;
pub struct Gfs2GlockLockTime;
pub struct Gfs2Pin;
pub struct Gfs2LogFlush;
pub struct Gfs2LogBlocks;
pub struct Gfs2AilFlush;
pub struct Gfs2Bmap;
pub struct Gfs2IomapStart;
pub struct Gfs2IomapEnd;
pub struct Gfs2BlockAlloc;
pub struct Gfs2Rs;

// Local symbolic-printing macro equivalents.  Kernel formatting is performed
// by the external tracepoint implementation.
#[macro_export]
macro_rules! dlm_state_name {
    ($nn:ident) => { (DLM_LOCK_$nn, stringify!($nn)) };
}

#[macro_export]
macro_rules! glock_trace_name {
    ($x:expr) => { $x };
}

#[macro_export]
macro_rules! block_state_name {
    ($x:expr) => { $x };
}

#[macro_export]
macro_rules! rs_func_name {
    ($x:expr) => { $x };
}

#[macro_export]
macro_rules! show_glock_flags {
    ($flags:expr) => { $flags };
}

/*
 * Event specifications translated from the source:
 *
 * gfs2_glock_state_change(gl, new_state): dev, glnum, gltype, cur_state,
 * new_state, dmt_state, tgt_state, flags; captures the glock device and
 * states, then prints the state transition and flags.
 *
 * gfs2_glock_put(gl): dev, glnum, gltype, cur_state, flags; captures the
 * deallocation transition to DLM_LOCK_IV.
 *
 * gfs2_demote_rq(gl, remote): dev, glnum, gltype, cur_state, dmt_state,
 * flags, remote; captures local or remote demotion requests.
 *
 * gfs2_promote(gh): dev, glnum, gltype, state; captures promotion/grant.
 * gfs2_glock_queue(gh, queue): dev, glnum, gltype, queue, state; captures
 * queue/dequeue operations.
 *
 * gfs2_glock_lock_time(gl, tdiff): dev, glnum, gltype, status, flags, tdiff,
 * srtt, srttvar, srttb, srttvarb, sirt, sirtvar, dcount, qcount.
 *
 * gfs2_pin(bd, pin): dev, pin, len, block, ino; captures log pin/unpin.
 * gfs2_log_flush(sdp, start, flags): dev, start, log_seq, flags.
 * gfs2_log_blocks(sdp, blocks): dev, blocks, blks_free.
 * gfs2_ail_flush(sdp, wbc, start): dev, start, sync_mode, nr_to_write.
 *
 * gfs2_bmap(ip, bh, lblock, create, errno): dev, lblock, pblock, inum,
 * state, len, create, errno.
 * gfs2_iomap_start(ip, pos, length, flags): dev, inum, pos, length, flags.
 * gfs2_iomap_end(ip, iomap, ret): dev, inum, offset, length, pblock, flags,
 * type, ret.
 * gfs2_block_alloc(ip, rgd, block, len, block_state): dev, start, inum, len,
 * block_state, rd_addr, rd_free_clone, rd_requested, rd_reserved.
 * gfs2_rs(rs, func): dev, rd_addr, rd_free_clone, rd_requested, rd_reserved,
 * inum, start, requested, reserved, func.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
