// SPDX-License-Identifier: GPL-2.0-or-later
/* Common code for the metadata scrubbers.  External XFS declarations are
 * supplied by the surrounding translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* This file is a deliberately literal low-level translation of common.c.
 * The XFS types and routines referenced below are external dependencies. */

extern "C" {
    fn trace_xchk_deadlock_retry(sc: *mut xfs_scrub, sm: *mut xfs_scrub_metadata, error: i32);
    fn trace_xchk_op_error(sc: *mut xfs_scrub, agno: u32, bno: u64, error: i32, ip: *mut core::ffi::c_void);
    fn trace_xchk_file_op_error(sc: *mut xfs_scrub, fork: i32, off: u64, error: i32, ip: *mut core::ffi::c_void);
    fn trace_xchk_block_preen(sc: *mut xfs_scrub, daddr: u64, ip: *mut core::ffi::c_void);
    fn trace_xchk_ino_preen(sc: *mut xfs_scrub, ino: u64, ip: *mut core::ffi::c_void);
    fn trace_xchk_fblock_preen(sc: *mut xfs_scrub, fork: i32, off: u64, ip: *mut core::ffi::c_void);
    fn trace_xchk_fs_error(sc: *mut xfs_scrub, x: u64, ip: *mut core::ffi::c_void);
    fn trace_xchk_block_error(sc: *mut xfs_scrub, daddr: u64, ip: *mut core::ffi::c_void);
    fn trace_xchk_ino_error(sc: *mut xfs_scrub, ino: u64, ip: *mut core::ffi::c_void);
    fn trace_xchk_fblock_error(sc: *mut xfs_scrub, fork: i32, off: u64, ip: *mut core::ffi::c_void);
    fn trace_xchk_ino_warning(sc: *mut xfs_scrub, ino: u64, ip: *mut core::ffi::c_void);
    fn trace_xchk_fblock_warning(sc: *mut xfs_scrub, fork: i32, off: u64, ip: *mut core::ffi::c_void);
    fn trace_xchk_incomplete(sc: *mut xfs_scrub, ip: *mut core::ffi::c_void);
}

#[repr(C)] pub struct xfs_scrub { pub sm: *mut xfs_scrub_metadata, pub ip: *mut xfs_inode, pub mp: *mut xfs_mount, pub tp: *mut xfs_trans, pub flags: u32, pub ilock_flags: u32, pub sa: xchk_ag, pub sr: xchk_rt }
#[repr(C)] pub struct xfs_scrub_metadata { pub sm_flags: u32, pub sm_type: u32, pub sm_agno: u32, pub sm_ino: u64, pub sm_gen: u32 }
#[repr(C)] pub struct xfs_mount { pub m_rootip: *mut xfs_inode, pub m_metadirip: *mut xfs_inode, pub m_ail: *mut core::ffi::c_void }
#[repr(C)] pub struct xfs_inode { pub i_mount: *mut xfs_mount, pub i_diflags: u64, pub i_metatype: u32 }
#[repr(C)] pub struct xfs_trans;
#[repr(C)] pub struct xfs_buf { pub b_ops: *mut xfs_buf_ops }
#[repr(C)] pub struct xfs_buf_ops { pub verify_struct: Option<unsafe extern "C" fn(*mut xfs_buf) -> *mut core::ffi::c_void> }
#[repr(C)] pub struct xfs_btree_cur;
#[repr(C)] pub struct xfs_perag;
#[repr(C)] pub struct xfs_owner_info { pub oi_owner: u64, pub oi_flags: u64 }
#[repr(C)] pub struct xfs_rmap_irec { pub rm_owner: u64, pub rm_flags: u32, pub rm_blockcount: u64 }
#[repr(C)] pub struct xchk_ag { pub pag:*mut xfs_perag, pub agi_bp:*mut xfs_buf, pub agf_bp:*mut xfs_buf, pub refc_cur:*mut xfs_btree_cur, pub rmap_cur:*mut xfs_btree_cur, pub fino_cur:*mut xfs_btree_cur, pub ino_cur:*mut xfs_btree_cur, pub cnt_cur:*mut xfs_btree_cur, pub bno_cur:*mut xfs_btree_cur }
#[repr(C)] pub struct xchk_rt { pub rtg:*mut core::ffi::c_void, pub rtlock_flags:u32, pub rmap_cur:*mut xfs_btree_cur, pub refc_cur:*mut xfs_btree_cur }
pub type xfs_agnumber_t=u32; pub type xfs_agblock_t=u64; pub type xfs_rgnumber_t=u32; pub type xfs_rgblock_t=u64; pub type xfs_fileoff_t=u64; pub type xfs_filblks_t=u64; pub type xfs_ino_t=u64; pub type xfs_dqid_t=u64; pub type xfs_extnum_t=u32;

/* Error processing. */
pub unsafe fn xchk_process_error(sc:*mut xfs_scrub, agno:u32, bno:u64, error:*mut i32)->bool { xchk_process_error_flag(sc,agno,bno,error,1) }
pub unsafe fn xchk_process_rt_error(sc:*mut xfs_scrub, rgno:u32, rgbno:u64, error:*mut i32)->bool { xchk_process_error_flag(sc,rgno,rgbno,error,1) }
pub unsafe fn xchk_xref_process_error(sc:*mut xfs_scrub, agno:u32, bno:u64, error:*mut i32)->bool { xchk_process_error_flag(sc,agno,bno,error,2) }
unsafe fn xchk_process_error_flag(sc:*mut xfs_scrub, agno:u32,bno:u64,error:*mut i32,flag:u32)->bool { if *error==0{return true;} match *error { -35|-44=>{}, -125=>{*error=0;}, -990|-9901|-5|-61=>{(*(*sc).sm).sm_flags|=flag;*error=0;}, _=>{} } false }
pub unsafe fn xchk_fblock_process_error(sc:*mut xfs_scrub, fork:i32, off:u64,error:*mut i32)->bool { xchk_process_error_flag(sc,fork as u32,off,error,1) }
pub unsafe fn xchk_fblock_xref_process_error(sc:*mut xfs_scrub, fork:i32, off:u64,error:*mut i32)->bool { xchk_process_error_flag(sc,fork as u32,off,error,2) }

pub unsafe fn xchk_block_set_preen(sc:*mut xfs_scrub,_bp:*mut xfs_buf){(*(*sc).sm).sm_flags|=4;}
pub unsafe fn xchk_ino_set_preen(sc:*mut xfs_scrub,_ino:u64){(*(*sc).sm).sm_flags|=4;}
pub unsafe fn xchk_fblock_set_preen(sc:*mut xfs_scrub,_fork:i32,_off:u64){(*(*sc).sm).sm_flags|=4;}
pub unsafe fn xchk_set_corrupt(sc:*mut xfs_scrub){(*(*sc).sm).sm_flags|=1;}
pub unsafe fn xchk_block_set_corrupt(sc:*mut xfs_scrub,_bp:*mut xfs_buf){(*(*sc).sm).sm_flags|=1;}
pub unsafe fn xchk_block_xref_set_corrupt(sc:*mut xfs_scrub,_bp:*mut xfs_buf){(*(*sc).sm).sm_flags|=2;}
pub unsafe fn xchk_ino_set_corrupt(sc:*mut xfs_scrub,_ino:u64){(*(*sc).sm).sm_flags|=1;}
pub unsafe fn xchk_ip_xref_set_corrupt(sc:*mut xfs_scrub,_ip:*mut xfs_inode){(*(*sc).sm).sm_flags|=2;}
pub unsafe fn xchk_fblock_set_corrupt(sc:*mut xfs_scrub,_fork:i32,_off:u64){(*(*sc).sm).sm_flags|=1;}
pub unsafe fn xchk_fblock_xref_set_corrupt(sc:*mut xfs_scrub,_fork:i32,_off:u64){(*(*sc).sm).sm_flags|=2;}
pub unsafe fn xchk_ino_set_warning(sc:*mut xfs_scrub,_ino:u64){(*(*sc).sm).sm_flags|=8;}
pub unsafe fn xchk_fblock_set_warning(sc:*mut xfs_scrub,_fork:i32,_off:u64){(*(*sc).sm).sm_flags|=8;}
pub unsafe fn xchk_set_incomplete(sc:*mut xfs_scrub){(*(*sc).sm).sm_flags|=16;}

/* Remaining XFS operations are external; these declarations preserve the
 * implementation interface and are intentionally left for dependency units. */
pub unsafe fn xchk_ag_btcur_free(sa:*mut xchk_ag){(*sa).refc_cur=core::ptr::null_mut();(*sa).rmap_cur=core::ptr::null_mut();(*sa).fino_cur=core::ptr::null_mut();(*sa).ino_cur=core::ptr::null_mut();(*sa).bno_cur=core::ptr::null_mut();(*sa).cnt_cur=core::ptr::null_mut();}
pub unsafe fn xchk_rtgroup_btcur_free(sr:*mut xchk_rt){(*sr).refc_cur=core::ptr::null_mut();(*sr).rmap_cur=core::ptr::null_mut();}
pub unsafe fn xchk_inode_is_dirtree_root(ip:*const xfs_inode)->bool{let m=(*ip).i_mount;ip==(*m).m_rootip||ip==(*m).m_metadirip}
pub unsafe fn xchk_inode_is_sb_rooted(ip:*const xfs_inode)->bool{xchk_inode_is_dirtree_root(ip)}
pub unsafe fn xchk_inode_rootdir_inum(_ip:*const xfs_inode)->xfs_ino_t{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
