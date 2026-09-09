// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

extern "C" {
    pub fn xchk_trans_alloc(sc: *mut xfs_scrub, resblks: uint) -> c_int;
    pub fn xchk_trans_alloc_empty(sc: *mut xfs_scrub);
    pub fn xchk_trans_cancel(sc: *mut xfs_scrub);

    pub fn xchk_process_error(sc: *mut xfs_scrub, agno: xfs_agnumber_t, bno: xfs_agblock_t, error: *mut c_int) -> bool;
    pub fn xchk_process_rt_error(sc: *mut xfs_scrub, rgno: xfs_rgnumber_t, rgbno: xfs_rgblock_t, error: *mut c_int) -> bool;
    pub fn xchk_fblock_process_error(sc: *mut xfs_scrub, whichfork: c_int, offset: xfs_fileoff_t, error: *mut c_int) -> bool;

    pub fn xchk_xref_process_error(sc: *mut xfs_scrub, agno: xfs_agnumber_t, bno: xfs_agblock_t, error: *mut c_int) -> bool;
    pub fn xchk_fblock_xref_process_error(sc: *mut xfs_scrub, whichfork: c_int, offset: xfs_fileoff_t, error: *mut c_int) -> bool;

    pub fn xchk_block_set_preen(sc: *mut xfs_scrub, bp: *mut xfs_buf);
    pub fn xchk_ino_set_preen(sc: *mut xfs_scrub, ino: xfs_ino_t);
    pub fn xchk_fblock_set_preen(sc: *mut xfs_scrub, whichfork: c_int, offset: xfs_fileoff_t);
    pub fn xchk_set_corrupt(sc: *mut xfs_scrub);
    pub fn xchk_block_set_corrupt(sc: *mut xfs_scrub, bp: *mut xfs_buf);
    pub fn xchk_ino_set_corrupt(sc: *mut xfs_scrub, ino: xfs_ino_t);
    pub fn xchk_fblock_set_corrupt(sc: *mut xfs_scrub, whichfork: c_int, offset: xfs_fileoff_t);
    pub fn xchk_block_xref_set_corrupt(sc: *mut xfs_scrub, bp: *mut xfs_buf);
    pub fn xchk_ip_xref_set_corrupt(sc: *mut xfs_scrub, ip: *mut xfs_inode);
    pub fn xchk_fblock_xref_set_corrupt(sc: *mut xfs_scrub, whichfork: c_int, offset: xfs_fileoff_t);
    pub fn xchk_ino_set_warning(sc: *mut xfs_scrub, ino: xfs_ino_t);
    pub fn xchk_fblock_set_warning(sc: *mut xfs_scrub, whichfork: c_int, offset: xfs_fileoff_t);
    pub fn xchk_set_incomplete(sc: *mut xfs_scrub);
    pub fn xchk_checkpoint_log(mp: *mut xfs_mount) -> c_int;
    pub fn xchk_should_check_xref(sc: *mut xfs_scrub, error: *mut c_int, curpp: *mut *mut xfs_btree_cur) -> bool;

    pub fn xchk_setup_agheader(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_fs(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_rt(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_ag_allocbt(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_ag_iallocbt(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_ag_rmapbt(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_ag_refcountbt(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_inode(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_inode_bmap(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_inode_bmap_data(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_directory(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_xattr(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_symlink(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_parent(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_dirtree(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_metapath(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_rtbitmap(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_rtsummary(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_rgsuperblock(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_rtrmapbt(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_rtrefcountbt(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_ino_dqattach(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_quota(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_quotacheck(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_fscounters(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_nlinks(sc: *mut xfs_scrub) -> c_int;

    pub fn xchk_ag_free(sc: *mut xfs_scrub, sa: *mut xchk_ag);
    pub fn xchk_ag_init(sc: *mut xfs_scrub, agno: xfs_agnumber_t, sa: *mut xchk_ag) -> c_int;
    pub fn xchk_perag_drain_and_lock(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_rtgroup_init(sc: *mut xfs_scrub, rgno: xfs_rgnumber_t, sr: *mut xchk_rt) -> c_int;
    pub fn xchk_rtgroup_lock(sc: *mut xfs_scrub, sr: *mut xchk_rt, rtglock_flags: c_uint) -> c_int;
    pub fn xchk_rtgroup_unlock(sr: *mut xchk_rt);
    pub fn xchk_rtgroup_btcur_free(sr: *mut xchk_rt);
    pub fn xchk_rtgroup_free(sc: *mut xfs_scrub, sr: *mut xchk_rt);
    pub fn xchk_ag_read_headers(sc: *mut xfs_scrub, agno: xfs_agnumber_t, sa: *mut xchk_ag) -> c_int;
    pub fn xchk_ag_btcur_free(sa: *mut xchk_ag);
    pub fn xchk_ag_btcur_init(sc: *mut xfs_scrub, sa: *mut xchk_ag);
    pub fn xchk_count_rmap_ownedby_ag(sc: *mut xfs_scrub, cur: *mut xfs_btree_cur, oinfo: *const xfs_owner_info, blocks: *mut xfs_filblks_t) -> c_int;
    pub fn xchk_setup_ag_btree(sc: *mut xfs_scrub, force_log: bool) -> c_int;
    pub fn xchk_iget_for_scrubbing(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_setup_inode_contents(sc: *mut xfs_scrub, resblks: c_uint) -> c_int;
    pub fn xchk_install_live_inode(sc: *mut xfs_scrub, ip: *mut xfs_inode) -> c_int;
    pub fn xchk_ilock(sc: *mut xfs_scrub, ilock_flags: c_uint);
    pub fn xchk_ilock_nowait(sc: *mut xfs_scrub, ilock_flags: c_uint) -> bool;
    pub fn xchk_iunlock(sc: *mut xfs_scrub, ilock_flags: c_uint);
    pub fn xchk_buffer_recheck(sc: *mut xfs_scrub, bp: *mut xfs_buf);
    pub fn xchk_iget(sc: *mut xfs_scrub, inum: xfs_ino_t, ipp: *mut *mut xfs_inode) -> c_int;
    pub fn xchk_iget_agi(sc: *mut xfs_scrub, inum: xfs_ino_t, agi_bpp: *mut *mut xfs_buf, ipp: *mut *mut xfs_inode) -> c_int;
    pub fn xchk_irele(sc: *mut xfs_scrub, ip: *mut xfs_inode);
    pub fn xchk_install_handle_inode(sc: *mut xfs_scrub, ip: *mut xfs_inode) -> c_int;
    pub fn xchk_dir_looks_zapped(dp: *mut xfs_inode) -> bool;
    pub fn xchk_pptr_looks_zapped(ip: *mut xfs_inode) -> bool;
    pub fn xchk_metadata_inode_forks(sc: *mut xfs_scrub) -> c_int;
    pub fn xchk_fsgates_enable(sc: *mut xfs_scrub, scrub_fshooks: c_uint);
    pub fn xchk_inode_is_allocated(sc: *mut xfs_scrub, agino: xfs_agino_t, inuse: *mut bool) -> c_int;
    pub fn xchk_inode_count_blocks(sc: *mut xfs_scrub, whichfork: c_int, nextents: *mut xfs_extnum_t, count: *mut xfs_filblks_t) -> c_int;
    pub fn xchk_inode_is_dirtree_root(ip: *const xfs_inode) -> bool;
    pub fn xchk_inode_is_sb_rooted(ip: *const xfs_inode) -> bool;
    pub fn xchk_inode_rootdir_inum(ip: *const xfs_inode) -> xfs_ino_t;
}

pub unsafe fn xchk_setup_nothing(_sc: *mut xfs_scrub) -> c_int { -ENOENT }

pub unsafe fn xchk_ip_set_corrupt(sc: *mut xfs_scrub, ip: *mut xfs_inode) {
    xchk_ino_set_corrupt(sc, I_INO(ip));
}

pub unsafe fn xchk_ag_init_existing(sc: *mut xfs_scrub, agno: xfs_agnumber_t, sa: *mut xchk_ag) -> c_int {
    let error = xchk_ag_init(sc, agno, sa);
    if error == -ENOENT { -EFSCORRUPTED } else { error }
}

pub unsafe fn xchk_rtgroup_init_existing(sc: *mut xfs_scrub, rgno: xfs_rgnumber_t, sr: *mut xchk_rt) -> c_int {
    let error = xchk_rtgroup_init(sc, rgno, sr);
    if error == -ENOENT { -EFSCORRUPTED } else { error }
}

pub unsafe fn xchk_iget_safe(sc: *mut xfs_scrub, inum: xfs_ino_t, ipp: *mut *mut xfs_inode) -> c_int {
    ASSERT((*sc).tp.is_null());
    let error = xchk_trans_alloc(sc, 0);
    if error != 0 { return error; }
    let error = xchk_iget(sc, inum, ipp);
    xchk_trans_cancel(sc);
    error
}

pub unsafe fn xchk_skip_xref(sm: *mut xfs_scrub_metadata) -> bool {
    (*sm).sm_flags & (XFS_SCRUB_OFLAG_CORRUPT | XFS_SCRUB_OFLAG_XCORRUPT) != 0
}

pub unsafe fn xchk_needs_repair(sm: *const xfs_scrub_metadata) -> bool {
    (*sm).sm_flags & (XFS_SCRUB_OFLAG_CORRUPT | XFS_SCRUB_OFLAG_XCORRUPT | XFS_SCRUB_OFLAG_PREEN) != 0
}

pub unsafe fn xchk_could_repair(sc: *const xfs_scrub) -> bool {
    ((*(*sc).sm).sm_flags & XFS_SCRUB_IFLAG_REPAIR != 0) && ((*sc).flags & XREP_ALREADY_FIXED == 0)
}

pub unsafe fn xchk_need_intent_drain(sc: *mut xfs_scrub) -> bool {
    (*sc).flags & XCHK_NEED_DRAIN != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
