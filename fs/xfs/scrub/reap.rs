// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2022-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// Dependencies are supplied by the surrounding XFS Rust translation.

#[repr(C)]
pub struct xreap_state {
    pub sc: *mut xfs_scrub,
    pub oinfo: *const xfs_owner_info,
    pub resv: xfs_ag_resv_type,
    pub ip: *mut xfs_inode,
    pub whichfork: i32,
    pub nr_binval: u32,
    pub max_binval: u32,
    pub nr_deferred: u32,
    pub max_deferred: u32,
}

#[inline]
unsafe fn xreap_is_dirty(rs: *const xreap_state) -> bool { (*rs).nr_binval > 0 || (*rs).nr_deferred > 0 }
#[inline]
unsafe fn xreap_want_binval_roll(rs: *const xreap_state) -> bool { (*rs).nr_binval >= (*rs).max_binval }
#[inline]
unsafe fn xreap_binval_reset(rs: *mut xreap_state) { (*rs).nr_binval = 0; }
#[inline]
unsafe fn xreap_inc_binval(rs: *mut xreap_state) -> bool { (*rs).nr_binval += 1; (*rs).nr_binval < (*rs).max_binval }
#[inline]
unsafe fn xreap_want_defer_finish(rs: *const xreap_state) -> bool { (*rs).nr_deferred >= (*rs).max_deferred }
#[inline]
unsafe fn xreap_defer_finish_reset(rs: *mut xreap_state) { (*rs).nr_deferred = 0; (*rs).nr_binval = 0; }
#[inline]
unsafe fn xreap_inc_defer(rs: *mut xreap_state) { (*rs).nr_deferred += 1; }
#[inline]
unsafe fn xreap_force_defer_finish(rs: *mut xreap_state) { (*rs).nr_deferred = (*rs).max_deferred; }

unsafe fn xreap_put_freelist(sc: *mut xfs_scrub, agbno: xfs_agblock_t) -> i32 {
    let mut agfl_bp: *mut xfs_buf = core::ptr::null_mut();
    let mut error = xrep_fix_freelist(sc, 0); if error != 0 { return error; }
    error = xfs_rmap_alloc((*sc).tp, (*sc).sa.agf_bp, (*sc).sa.pag, agbno, 1, &XFS_RMAP_OINFO_AG); if error != 0 { return error; }
    error = xfs_alloc_read_agfl((*sc).sa.pag, (*sc).tp, &mut agfl_bp); if error != 0 { return error; }
    error = xfs_alloc_put_freelist((*sc).sa.pag, (*sc).tp, (*sc).sa.agf_bp, agfl_bp, agbno, 0); if error != 0 { return error; }
    xfs_extent_busy_insert((*sc).tp, pag_group((*sc).sa.pag), agbno, 1, XFS_EXTENT_BUSY_SKIP_DISCARD); 0
}

#[inline]
unsafe fn xrep_binval_max_fsblocks(mp: *mut xfs_mount) -> u32 { xfs_attr3_max_rmt_blocks(mp) }

pub unsafe fn xrep_bufscan_max_sectors(mp: *mut xfs_mount, fsblocks: xfs_extlen_t) -> xfs_daddr_t {
    XFS_FSB_TO_BB(mp, core::cmp::min(fsblocks, xrep_binval_max_fsblocks(mp)))
}

pub unsafe fn xrep_bufscan_advance(mp: *mut xfs_mount, scan: *mut xrep_bufscan) -> *mut xfs_buf {
    (*scan).__sector_count += (*scan).daddr_step;
    while (*scan).__sector_count <= (*scan).max_sectors {
        let mut bp: *mut xfs_buf = core::ptr::null_mut();
        let error = xfs_buf_incore((*mp).m_ddev_targp, (*scan).daddr, (*scan).__sector_count, XBF_LIVESCAN, &mut bp);
        if error == 0 { return bp; }
        (*scan).__sector_count += (*scan).daddr_step;
    }
    core::ptr::null_mut()
}

unsafe fn xreap_configure_limits(rs: *mut xreap_state, fixed: u32, variable: u32, per_intent: u32, per_binval: u32) {
    let sc = (*rs).sc; let mut res = (*(*sc).tp).t_log_res - fixed;
    if (*(*sc).tp).t_log_res < fixed + variable { xfs_force_shutdown((*sc).mp, SHUTDOWN_CORRUPT_INCORE); return; }
    (*rs).max_deferred = if per_intent != 0 { res / variable } else { 0 }; res -= (*rs).max_deferred * per_intent;
    (*rs).max_binval = if per_binval != 0 { res / per_binval } else { 0 };
}

/* The remaining routines preserve the C control flow and call surface. */
pub unsafe fn xrep_reap_agblocks(sc: *mut xfs_scrub, bitmap: *mut xagb_bitmap, oinfo: *const xfs_owner_info, typ: xfs_ag_resv_type) -> i32 {
    let mut rs = xreap_state { sc, oinfo, resv: typ, ip: core::ptr::null_mut(), whichfork: 0, nr_binval: 0, max_binval: 0, nr_deferred: 0, max_deferred: 0 };
    xreap_configure_limits(&mut rs, 0, 1, 1, 1);
    let error = xagb_bitmap_walk(bitmap, xreap_agmeta_extent, &mut rs as *mut _ as *mut core::ffi::c_void); if error != 0 { return error; }
    if xreap_is_dirty(&rs) { return xrep_defer_finish(sc); } 0
}

pub unsafe fn xrep_reap_fsblocks(sc: *mut xfs_scrub, bitmap: *mut xfsb_bitmap, oinfo: *const xfs_owner_info) -> i32 {
    let mut rs = xreap_state { sc, oinfo, resv: XFS_AG_RESV_NONE, ip: (*sc).ip, whichfork: 0, nr_binval: 0, max_binval: 0, nr_deferred: 0, max_deferred: 0 };
    xreap_configure_limits(&mut rs, 0, 1, 1, 1);
    let error = xfsb_bitmap_walk(bitmap, xreap_fsmeta_extent, &mut rs as *mut _ as *mut core::ffi::c_void); if error != 0 { return error; }
    if xreap_is_dirty(&rs) { return xrep_defer_finish(sc); } 0
}

pub unsafe fn xrep_reap_ifork(sc: *mut xfs_scrub, ip: *mut xfs_inode, whichfork: i32) -> i32 {
    let mut rs = xreap_state { sc, oinfo: core::ptr::null(), resv: XFS_AG_RESV_NONE, ip, whichfork, nr_binval: 0, max_binval: 0, nr_deferred: 0, max_deferred: 0 };
    xreap_configure_limits(&mut rs, 0, 1, 0, 1);
    let mut off: xfs_fileoff_t = 0;
    while off < XFS_MAX_FILEOFF { let mut imap = core::mem::zeroed::<xfs_bmbt_irec>(); let mut nimaps = 1; let error = xfs_bmapi_read(ip, off, XFS_MAX_FILEOFF - off, &mut imap, &mut nimaps, xfs_bmapi_aflag(whichfork)); if error != 0 { return error; } if xfs_bmap_is_real_extent(&imap) { let e = xreap_ifork_extent(&mut rs, &mut imap); if e != 0 { return e; } let e = xfs_defer_finish(&mut (*sc).tp); if e != 0 { return e; } xreap_defer_finish_reset(&mut rs); } off = imap.br_startoff + imap.br_blockcount; } 0
}

// File-local callback declarations; definitions are supplied by the translated companion units.
extern "C" { fn xreap_agmeta_extent(a: u32, l: u32, p: *mut core::ffi::c_void) -> i32; fn xreap_fsmeta_extent(a: u64, l: u64, p: *mut core::ffi::c_void) -> i32; fn xreap_ifork_extent(r: *mut xreap_state, i: *mut xfs_bmbt_irec) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
