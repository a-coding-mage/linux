// SPDX-License-Identifier: GPL-2.0-only
/* vfsv0 quota IO operations on file */

// C headers and local quota headers provide the types, constants, and helper
// functions referenced below.

use core::ffi::c_void;

const QUOTABLOCK_BITS: u32 = 10;
const QUOTABLOCK_SIZE: u64 = 1u64 << QUOTABLOCK_BITS;

#[inline]
unsafe fn v2_stoqb(space: u64) -> u64 {
    (space.wrapping_add(QUOTABLOCK_SIZE - 1)) >> QUOTABLOCK_BITS
}

#[inline]
unsafe fn v2_qbtos(blocks: u64) -> u64 {
    blocks << QUOTABLOCK_BITS
}

extern "C" {
    fn v2_read_header(sb: *mut super_block, type_: i32, dqhead: *mut v2_disk_dqheader) -> i32;
}

#[repr(C)]
pub struct v2_disk_dqheader { pub dqh_magic: u32, pub dqh_version: u32 }
#[repr(C)] pub struct v2_disk_dqinfo { pub dqi_bgrace: u32, pub dqi_igrace: u32, pub dqi_flags: u32, pub dqi_blocks: u32, pub dqi_free_blk: u32, pub dqi_free_entry: u32 }
#[repr(C)] pub struct v2r0_disk_dqblk { pub dqb_ihardlimit:u32, pub dqb_isoftlimit:u32, pub dqb_curinodes:u32, pub dqb_btime:u64, pub dqb_itime:u64, pub dqb_bhardlimit:u32, pub dqb_bsoftlimit:u32, pub dqb_curspace:u64, pub dqb_id:u32 }
#[repr(C)] pub struct v2r1_disk_dqblk { pub dqb_ihardlimit:u64, pub dqb_isoftlimit:u64, pub dqb_curinodes:u64, pub dqb_btime:u64, pub dqb_itime:u64, pub dqb_bhardlimit:u64, pub dqb_bsoftlimit:u64, pub dqb_curspace:u64, pub dqb_id:u32, pub dqb_pad:u32 }

// The kernel quota structures and operations are supplied by the surrounding
// translation unit; these declarations preserve the original interfaces.
#[repr(C)] pub struct super_block { pub s_op: *mut super_operations }
#[repr(C)] pub struct super_operations { pub quota_read: Option<unsafe extern "C" fn(*mut super_block,i32,*mut i8,usize,i64)->isize>, pub quota_write: Option<unsafe extern "C" fn(*mut super_block,i32,*const i8,usize,i64)->isize> }
#[repr(C)] pub struct dquot { pub dq_sb:*mut super_block, pub dq_id:kqid, pub dq_dqb:mem_dqblk, pub dq_off:u64, pub dq_lock:c_void }
#[repr(C)] pub struct kqid { pub type_:u32, pub id:u32 }
#[repr(C)] pub struct mem_dqblk { pub dqb_ihardlimit:u64,pub dqb_isoftlimit:u64,pub dqb_curinodes:u64,pub dqb_itime:u64,pub dqb_bhardlimit:u64,pub dqb_bsoftlimit:u64,pub dqb_curspace:u64,pub dqb_btime:u64 }
#[repr(C)] pub struct mem_dqinfo { pub dqi_fmt_id:u32,pub dqi_priv:*mut qtree_mem_dqinfo,pub dqi_max_spc_limit:u64,pub dqi_max_ino_limit:u64,pub dqi_bgrace:u64,pub dqi_igrace:u64,pub dqi_flags:u32 }
#[repr(C)] pub struct quota_info { pub info:*mut mem_dqinfo, pub dqio_sem:c_void, pub files:*mut *mut inode }
#[repr(C)] pub struct inode { pub i_size:u64 }
#[repr(C)] pub struct qtree_mem_dqinfo { pub dqi_sb:*mut super_block,pub dqi_type:i32,pub dqi_blocks:u32,pub dqi_free_blk:u32,pub dqi_free_entry:u32,pub dqi_blocksize_bits:u32,pub dqi_usable_bs:u32,pub dqi_qtree_depth:u32,pub dqi_entry_size:usize,pub dqi_ops:*const qtree_fmt_operations }
#[repr(C)] pub struct qtree_fmt_operations { pub mem2disk_dqblk:Option<unsafe extern "C" fn(*mut c_void,*mut dquot)>,pub disk2mem_dqblk:Option<unsafe extern "C" fn(*mut dquot,*mut c_void)>,pub is_id:Option<unsafe extern "C" fn(*mut c_void,*mut dquot)->i32> }
#[repr(C)] pub struct quota_format_ops { pub check_quota_file:Option<unsafe extern "C" fn(*mut super_block,i32)->i32>,pub read_file_info:Option<unsafe extern "C" fn(*mut super_block,i32)->i32>,pub write_file_info:Option<unsafe extern "C" fn(*mut super_block,i32)->i32>,pub free_file_info:Option<unsafe extern "C" fn(*mut super_block,i32)->i32>,pub read_dqblk:Option<unsafe extern "C" fn(*mut dquot)->i32>,pub commit_dqblk:Option<unsafe extern "C" fn(*mut dquot)->i32>,pub release_dqblk:Option<unsafe extern "C" fn(*mut dquot)->i32>,pub get_next_id:Option<unsafe extern "C" fn(*mut super_block,*mut kqid)->i32> }
#[repr(C)] pub struct quota_format_type { pub qf_fmt_id:u32,pub qf_ops:*const quota_format_ops,pub qf_owner:*mut c_void }

// Remaining kernel helpers, constants, and registration APIs are external
// dependencies, intentionally not implemented here.
#[inline] unsafe fn v2r0_disk2memdqb(q:*mut dquot, p:*mut c_void) { let d=&*(p as *const v2r0_disk_dqblk); let m=&mut (*q).dq_dqb; m.dqb_ihardlimit=u32::from_le(d.dqb_ihardlimit) as u64; m.dqb_isoftlimit=u32::from_le(d.dqb_isoftlimit) as u64; m.dqb_curinodes=u32::from_le(d.dqb_curinodes) as u64; m.dqb_itime=u64::from_le(d.dqb_itime); m.dqb_bhardlimit=v2_qbtos(u32::from_le(d.dqb_bhardlimit) as u64); m.dqb_bsoftlimit=v2_qbtos(u32::from_le(d.dqb_bsoftlimit) as u64); m.dqb_curspace=u64::from_le(d.dqb_curspace); m.dqb_btime=u64::from_le(d.dqb_btime); }
#[inline] unsafe fn v2r1_disk2memdqb(q:*mut dquot, p:*mut c_void) { let d=&*(p as *const v2r1_disk_dqblk); let m=&mut (*q).dq_dqb; m.dqb_ihardlimit=u64::from_le(d.dqb_ihardlimit); m.dqb_isoftlimit=u64::from_le(d.dqb_isoftlimit); m.dqb_curinodes=u64::from_le(d.dqb_curinodes); m.dqb_itime=u64::from_le(d.dqb_itime); m.dqb_bhardlimit=v2_qbtos(u64::from_le(d.dqb_bhardlimit)); m.dqb_bsoftlimit=v2_qbtos(u64::from_le(d.dqb_bsoftlimit)); m.dqb_curspace=u64::from_le(d.dqb_curspace); m.dqb_btime=u64::from_le(d.dqb_btime); }
#[inline] unsafe fn v2r0_mem2diskdqb(p:*mut c_void,q:*mut dquot) { let d=&mut *(p as *mut v2r0_disk_dqblk); let m=&(*q).dq_dqb; d.dqb_ihardlimit=(m.dqb_ihardlimit as u32).to_le(); d.dqb_isoftlimit=(m.dqb_isoftlimit as u32).to_le(); d.dqb_curinodes=(m.dqb_curinodes as u32).to_le(); d.dqb_itime=m.dqb_itime.to_le(); d.dqb_bhardlimit=(v2_stoqb(m.dqb_bhardlimit) as u32).to_le(); d.dqb_bsoftlimit=(v2_stoqb(m.dqb_bsoftlimit) as u32).to_le(); d.dqb_curspace=m.dqb_curspace.to_le(); d.dqb_btime=m.dqb_btime.to_le(); d.dqb_id=(*q).dq_id.id.to_le(); }
#[inline] unsafe fn v2r1_mem2diskdqb(p:*mut c_void,q:*mut dquot) { let d=&mut *(p as *mut v2r1_disk_dqblk); let m=&(*q).dq_dqb; d.dqb_ihardlimit=m.dqb_ihardlimit.to_le(); d.dqb_isoftlimit=m.dqb_isoftlimit.to_le(); d.dqb_curinodes=m.dqb_curinodes.to_le(); d.dqb_itime=m.dqb_itime.to_le(); d.dqb_bhardlimit=v2_stoqb(m.dqb_bhardlimit).to_le(); d.dqb_bsoftlimit=v2_stoqb(m.dqb_bsoftlimit).to_le(); d.dqb_curspace=m.dqb_curspace.to_le(); d.dqb_btime=m.dqb_btime.to_le(); d.dqb_id=(*q).dq_id.id.to_le(); d.dqb_pad=0; }
unsafe fn v2r0_is_id(p:*mut c_void,q:*mut dquot)->i32 { if (*q).dq_id.id == u32::from_le((*(p as *const v2r0_disk_dqblk)).dqb_id) {1} else {0} }
unsafe fn v2r1_is_id(p:*mut c_void,q:*mut dquot)->i32 { if (*q).dq_id.id == u32::from_le((*(p as *const v2r1_disk_dqblk)).dqb_id) {1} else {0} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
