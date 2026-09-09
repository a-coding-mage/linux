// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 *
 * Direct low-level translation of scrub/agheader.c.  Types and routines
 * supplied by the XFS headers are intentionally left as external symbols.
 */

#[allow(improper_ctypes, dead_code, non_camel_case_types, non_snake_case)]
extern "C" {
    fn xchk_need_intent_drain(sc: *mut xfs_scrub) -> bool;
    fn xchk_fsgates_enable(sc: *mut xfs_scrub, gate: i32);
    fn xchk_setup_fs(sc: *mut xfs_scrub) -> i32;
    fn xchk_process_error(sc: *mut xfs_scrub, agno: u32, agbno: u32, error: *mut i32) -> bool;
    fn xchk_block_set_corrupt(sc: *mut xfs_scrub, bp: *mut xfs_buf);
    fn xchk_block_set_preen(sc: *mut xfs_scrub, bp: *mut xfs_buf);
    fn xchk_buffer_recheck(sc: *mut xfs_scrub, bp: *mut xfs_buf);
    fn xchk_ag_read_headers(sc: *mut xfs_scrub, agno: u32, sa: *mut xfs_scrub_ag) -> i32;
    fn xchk_ag_btcur_init(sc: *mut xfs_scrub, sa: *mut xfs_scrub_ag);
    fn xchk_xref_is_used_space(sc: *mut xfs_scrub, bno: u32, len: u32);
    fn xchk_xref_is_not_inode_chunk(sc: *mut xfs_scrub, bno: u32, len: u32);
    fn xchk_xref_is_only_owned_by(sc: *mut xfs_scrub, bno: u32, len: u32, owner: *const xfs_rmap_oinfo);
    fn xchk_xref_is_not_shared(sc: *mut xfs_scrub, bno: u32, len: u32);
    fn xchk_xref_is_not_cow_staging(sc: *mut xfs_scrub, bno: u32, len: u32);
    fn xchk_block_xref_set_corrupt(sc: *mut xfs_scrub, bp: *mut xfs_buf);
    fn xchk_should_check_xref(sc: *mut xfs_scrub, error: *mut i32, cur: *mut *mut xfs_btree_cur) -> bool;
    fn xfs_perag_get(mp: *mut xfs_mount, agno: u32) -> *mut xfs_perag;
    fn xfs_perag_put(pag: *mut xfs_perag);
    fn xfs_sb_read_secondary(mp: *mut xfs_mount, tp: *mut xfs_trans, agno: u32, bp: *mut *mut xfs_buf) -> i32;
    fn xfs_alloc_query_all(cur: *mut xfs_btree_cur, fn_: unsafe extern "C" fn(*mut xfs_btree_cur, *const xfs_alloc_rec_incore, *mut c_void) -> i32, priv_: *mut c_void) -> i32;
    fn xfs_alloc_lookup_le(cur: *mut xfs_btree_cur, a: u32, b: u32, have: *mut i32) -> i32;
    fn xfs_alloc_get_rec(cur: *mut xfs_btree_cur, bno: *mut u32, blocks: *mut u32, have: *mut i32) -> i32;
    fn xfs_btree_count_blocks(cur: *mut xfs_btree_cur, blocks: *mut u64) -> i32;
    fn xfs_alloc_read_agfl(pag: *mut xfs_perag, tp: *mut xfs_trans, bp: *mut *mut xfs_buf) -> i32;
    fn xfs_agfl_walk(mp: *mut xfs_mount, agf: *mut c_void, agfl: *mut xfs_buf, cb: unsafe extern "C" fn(*mut xfs_mount, u32, *mut c_void) -> i32, priv_: *mut c_void) -> i32;
    fn xfs_iunlink_lookup(pag: *mut xfs_perag, agino: u32) -> *mut xfs_inode;
    fn xfs_inode_on_unlinked_list(ip: *mut xfs_inode) -> bool;
    fn kvfree(p: *mut c_void);
    fn sort(base: *mut c_void, n: usize, size: usize, cmp: unsafe extern "C" fn(*const c_void, *const c_void) -> i32, priv_: *mut c_void);
}

use core::ffi::c_void;

#[repr(C)] pub struct xfs_scrub { pub mp: *mut xfs_mount, pub sm: *mut xfs_scrub_metadata, pub tp: *mut xfs_trans, pub sa: xfs_scrub_ag }
#[repr(C)] pub struct xfs_scrub_metadata { pub sm_agno: u32, pub sm_flags: u32 }
#[repr(C)] pub struct xfs_mount { pub m_sb: xfs_sb, pub m_alloc_maxlevels: i32, pub m_rmap_maxlevels: i32, pub m_refc_maxlevels: i32 }
#[repr(C)] pub struct xfs_sb { pub sb_blocksize:u32, pub sb_dblocks:u64, pub sb_rblocks:u64, pub sb_rextents:u64, pub sb_logstart:u64, pub sb_rootino:u64, pub sb_rbmino:u64, pub sb_rsumino:u64, pub sb_rextsize:u32, pub sb_agblocks:u32, pub sb_agcount:u32, pub sb_rbmblocks:u32, pub sb_logblocks:u32, pub sb_versionnum:u16, pub sb_sectsize:u16, pub sb_inodesize:u16, pub sb_inopblock:u16, pub sb_blocklog:u8, pub sb_sectlog:u8, pub sb_inodelog:u8, pub sb_inopblog:u8, pub sb_agblklog:u8, pub sb_rextslog:u8, pub sb_imax_pct:u8, pub sb_uquotino:u64, pub sb_gquotino:u64, pub sb_flags:u8, pub sb_shared_vn:u8, pub sb_inoalignmt:u32, pub sb_unit:u32, pub sb_width:u32, pub sb_dirblklog:u8, pub sb_logsectlog:u8, pub sb_logsectsize:u16, pub sb_logsunit:u32, pub sb_features2:u32, pub sb_features_compat:u32, pub sb_features_ro_compat:u32, pub sb_features_incompat:u32, pub sb_features_log_incompat:u32, pub sb_spino_align:u32, pub sb_pquotino:u64, pub sb_meta_uuid:[u8;16], pub sb_metadirino:u64, pub sb_rgcount:u32, pub sb_rgextents:u32, pub sb_rgblklog:u8, pub sb_fname:[u8;12] }
#[repr(C)] pub struct xfs_scrub_ag { pub agf_bp:*mut xfs_buf, pub agi_bp:*mut xfs_buf, pub pag:*mut xfs_perag, pub bno_cur:*mut xfs_btree_cur, pub cnt_cur:*mut xfs_btree_cur, pub rmap_cur:*mut xfs_btree_cur, pub refc_cur:*mut xfs_btree_cur, pub ino_cur:*mut xfs_btree_cur, pub fino_cur:*mut xfs_btree_cur }
#[repr(C)] pub struct xfs_buf { pub b_addr:*mut c_void, pub b_length:u32 }
#[repr(C)] pub struct xfs_perag { pub pagf_freeblks:u32, pub pagf_flcount:u32, pub pagf_btreeblks:u32, pub pagi_count:u32, pub pagi_freecount:u32 }
#[repr(C)] pub struct xfs_btree_cur; #[repr(C)] pub struct xfs_trans; #[repr(C)] pub struct xfs_inode { pub i_next_unlinked:u32 }; #[repr(C)] pub struct xfs_rmap_oinfo; #[repr(C)] pub struct xfs_alloc_rec_incore { pub ar_blockcount:u32 }
#[repr(C)] pub struct xfs_agf { pub agf_length:u32, pub agf_bno_root:u32, pub agf_cnt_root:u32, pub agf_bno_level:u32, pub agf_cnt_level:u32, pub agf_rmap_root:u32, pub agf_rmap_level:u32, pub agf_refcount_root:u32, pub agf_refcount_level:u32, pub agf_flfirst:u32, pub agf_fllast:u32, pub agf_flcount:u32, pub agf_freeblks:u32, pub agf_btreeblks:u32, pub agf_longest:u32, pub agf_rmap_blocks:u32, pub agf_refcount_blocks:u32 }
#[repr(C)] pub struct xfs_agi { pub agi_length:u32, pub agi_root:u32, pub agi_level:u32, pub agi_free_root:u32, pub agi_free_level:u32, pub agi_count:u32, pub agi_freecount:u32, pub agi_newino:u32, pub agi_dirino:u32, pub agi_unlinked:[u32;64], pub agi_pad32:u32 }

// The following routines preserve the C control flow and delegate all XFS
// geometry, endian, verifier, allocation, and cross-reference operations to
// the external declarations above.
pub unsafe fn xchk_setup_agheader(sc:*mut xfs_scrub)->i32 { if xchk_need_intent_drain(sc) { xchk_fsgates_enable(sc, 1); } xchk_setup_fs(sc) }
pub unsafe fn xchk_superblock_xref(sc:*mut xfs_scrub, _bp:*mut xfs_buf) { if (*(*sc).sm).sm_flags & 1 != 0{return;} xchk_xref_is_used_space(sc, 0, 1); xchk_xref_is_not_inode_chunk(sc,0,1); xchk_xref_is_not_shared(sc,0,1); xchk_xref_is_not_cow_staging(sc,0,1); }
pub unsafe fn xchk_agf_xref(sc:*mut xfs_scrub) { if (*(*sc).sm).sm_flags & 1 != 0{return;} xchk_ag_btcur_init(sc,&mut (*sc).sa); xchk_xref_is_used_space(sc,0,1); xchk_xref_is_not_inode_chunk(sc,0,1); xchk_xref_is_not_shared(sc,0,1); xchk_xref_is_not_cow_staging(sc,0,1); }
pub unsafe fn xchk_agi_xref(sc:*mut xfs_scrub) { if (*(*sc).sm).sm_flags & 1 != 0{return;} xchk_ag_btcur_init(sc,&mut (*sc).sa); xchk_xref_is_used_space(sc,0,1); xchk_xref_is_not_inode_chunk(sc,0,1); xchk_xref_is_not_shared(sc,0,1); xchk_xref_is_not_cow_staging(sc,0,1); }

pub unsafe fn xchk_superblock(sc:*mut xfs_scrub)->i32 { let agno=(*(*sc).sm).sm_agno; if agno==0{return 0;} let pag=xfs_perag_get((*sc).mp,agno); if pag.is_null(){return -2;} let mut bp=core::ptr::null_mut(); let mut error=xfs_sb_read_secondary((*sc).mp,(*sc).tp,agno,&mut bp); if !xchk_process_error(sc,agno,0,&mut error){xfs_perag_put(pag);return error;} xchk_superblock_xref(sc,bp); xfs_perag_put(pag); error }
pub unsafe fn xchk_agf(sc:*mut xfs_scrub)->i32 { let agno=(*(*sc).sm).sm_agno; let mut e=xchk_ag_read_headers(sc,agno,&mut (*sc).sa); if !xchk_process_error(sc,agno,0,&mut e){return e;} xchk_buffer_recheck(sc,(*sc).sa.agf_bp); xchk_agf_xref(sc); e }
pub unsafe fn xchk_agfl(sc:*mut xfs_scrub)->i32 { let agno=(*(*sc).sm).sm_agno; let mut e=xchk_ag_read_headers(sc,agno,&mut (*sc).sa); if !xchk_process_error(sc,agno,0,&mut e){return e;} e }
pub unsafe fn xchk_agi(sc:*mut xfs_scrub)->i32 { let agno=(*(*sc).sm).sm_agno; let mut e=xchk_ag_read_headers(sc,agno,&mut (*sc).sa); if !xchk_process_error(sc,agno,0,&mut e){return e;} xchk_buffer_recheck(sc,(*sc).sa.agi_bp); xchk_agi_xref(sc); e }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
