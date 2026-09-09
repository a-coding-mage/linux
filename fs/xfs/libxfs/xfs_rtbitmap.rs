// SPDX-License-Identifier: GPL-2.0
/* Realtime allocator bitmap functions shared with userspace. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* Types, constants, structures, and helper functions are supplied by the XFS
 * translation unit headers.  They are intentionally referenced, not defined,
 * here. */

extern "C" {
    fn xfs_verify_magic(bp: *mut xfs_buf, magic: u32) -> bool;
    fn xfs_has_rtgroups(mp: *mut xfs_mount) -> bool;
    fn xfs_has_crc(mp: *mut xfs_mount) -> bool;
    fn uuid_equal(a: *const c_void, b: *const c_void) -> bool;
    fn xfs_buf_daddr(bp: *mut xfs_buf) -> u64;
    fn xfs_log_check_lsn(mp: *mut xfs_mount, lsn: u64) -> bool;
    fn xfs_buf_verify_cksum(bp: *mut xfs_buf, off: u32) -> bool;
    fn xfs_buf_update_cksum(bp: *mut xfs_buf, off: u32);
    fn xfs_verifier_error(bp: *mut xfs_buf, err: i32, fa: xfs_failaddr_t);
    fn xfs_trans_brelse(tp: *mut xfs_trans, bp: *mut xfs_buf);
    fn xfs_bmapi_read(ip: *mut xfs_inode, b: u64, n: u64, map: *mut xfs_bmbt_irec, nm: *mut i32, flags: u32) -> i32;
    fn xfs_bmap_is_written_extent(map: *const xfs_bmbt_irec) -> bool;
    fn xfs_rtginode_mark_sick(rtg: *mut xfs_rtgroup, ty: xfs_rtg_inodes);
    fn xfs_trans_read_buf(mp: *mut xfs_mount, tp: *mut xfs_trans, targ: *mut c_void, daddr: u64, len: u32, flags: u32, bp: *mut *mut xfs_buf, ops: *const xfs_buf_ops) -> i32;
    fn xfs_rtblock_ops(mp: *mut xfs_mount, ty: xfs_rtg_inodes) -> *const xfs_buf_ops;
    fn xfs_trans_buf_set_type(tp: *mut xfs_trans, bp: *mut xfs_buf, ty: xfs_blft);
    fn xfs_buf_mark_corrupt(bp: *mut xfs_buf);
    fn xfs_rtx_to_rbmblock(mp: *mut xfs_mount, r: u64) -> u64;
    fn xfs_rtx_to_rbmword(mp: *mut xfs_mount, r: u64) -> u32;
    fn xfs_rtbitmap_read_buf(a: *mut xfs_rtalloc_args, b: u64) -> i32;
    fn xfs_rtbitmap_getword(a: *mut xfs_rtalloc_args, w: u32) -> u32;
    fn xfs_rtbitmap_setword(a: *mut xfs_rtalloc_args, w: u32, v: u32);
    fn xfs_highbit32(v: u32) -> u32;
    fn xfs_lowbit32(v: u32) -> u32;
    fn xfs_rtmodify_summary(a: *mut xfs_rtalloc_args, log: i32, bb: u64, d: i32) -> i32;
    fn xfs_rtmodify_range(a: *mut xfs_rtalloc_args, s: u64, l: u64, v: i32) -> i32;
    fn xfs_trans_log_buf(tp: *mut xfs_trans, bp: *mut xfs_buf, first: usize, last: usize);
}

/* Header declarations represented as opaque FFI types. */
#[repr(C)] pub struct xfs_mount { pub m_sb: xfs_sb, pub m_blockwsize: u32, pub m_bsize: u32, pub m_ddev_targp: *mut c_void, pub m_rsumblocks: u64 }
#[repr(C)] pub struct xfs_sb { pub sb_rbmblocks: u64, pub sb_blocksize: u32, pub sb_rextents: u64, pub sb_rgextents: u64 }
#[repr(C)] pub struct xfs_buf { pub b_mount: *mut xfs_mount, pub b_addr: *mut u8, pub b_log_item: *mut xfs_buf_log_item, pub b_ops: *const xfs_buf_ops }
#[repr(C)] pub struct xfs_buf_log_item { pub bli_item: xfs_log_item }
#[repr(C)] pub struct xfs_log_item { pub li_lsn: u64 }
#[repr(C)] pub struct xfs_buf_ops { pub name: *const u8, pub magic: [u32;2], pub verify_read: Option<unsafe extern "C" fn(*mut xfs_buf)>, pub verify_write: Option<unsafe extern "C" fn(*mut xfs_buf)>, pub verify_struct: Option<unsafe extern "C" fn(*mut xfs_buf)->xfs_failaddr_t> }
#[repr(C)] pub struct xfs_inode { pub i_mount: *mut xfs_mount, pub i_disk_size: u64, pub i_diflags: u64 }
#[repr(C)] pub struct xfs_rtgroup { pub rtg_inodes: [*mut xfs_inode; 2], pub rtg_extents: u64 }
#[repr(C)] pub struct xfs_trans { pub t_mountp: *mut xfs_mount, pub t_frextents_delta: i64 }
#[repr(C)] pub struct xfs_bmbt_irec { pub br_startblock: u64, pub br_blockcount: u64, pub br_startoff: u64 }
#[repr(C)] pub struct xfs_rtalloc_args { pub mp:*mut xfs_mount, pub tp:*mut xfs_trans, pub rtg:*mut xfs_rtgroup, pub rbmbp:*mut xfs_buf, pub sumbp:*mut xfs_buf, pub rbmoff:u64, pub sumoff:u64 }
pub type xfs_failaddr_t = *const c_void; pub type xfs_rtg_inodes = u32; pub type xfs_blft = u32;
pub type xfs_rtxnum_t=u64; pub type xfs_rtxlen_t=u64; pub type xfs_fileoff_t=u64; pub type xfs_filblks_t=u64; pub type xfs_fsblock_t=u64; pub type xfs_rtbxlen_t=u64; pub type xfs_suminfo_t=i32; pub type xfs_rtalloc_query_range_fn=unsafe extern "C" fn(*mut xfs_rtgroup,*mut xfs_trans,*mut c_void,*mut c_void)->i32;
const XFS_RTGI_SUMMARY:xfs_rtg_inodes=0; const XFS_RTGI_BITMAP:xfs_rtg_inodes=1; const XFS_NBWORD:u64=32; const NULLFILEOFF:u64=!0;

#[inline] unsafe fn rtbuf_verify(bp:*mut xfs_buf)->xfs_failaddr_t { let mp=(*bp).b_mount; if !xfs_verify_magic(bp,0) || !xfs_has_rtgroups(mp) || !xfs_has_crc(mp) { return rtbuf_verify as *const c_void; } core::ptr::null() }
unsafe extern "C" fn xfs_rtbuf_verify_read(bp:*mut xfs_buf) { let mp=(*bp).b_mount; if !xfs_has_rtgroups(mp){return;} if !xfs_log_check_lsn(mp,0)||!xfs_buf_verify_cksum(bp,0){xfs_verifier_error(bp,-117,rtbuf_verify as *const c_void);return;} let fa=rtbuf_verify(bp); if !fa.is_null(){xfs_verifier_error(bp,-117,fa);} }
unsafe extern "C" fn xfs_rtbuf_verify_write(bp:*mut xfs_buf) { if !xfs_has_rtgroups((*bp).b_mount){return;} let fa=rtbuf_verify(bp); if !fa.is_null(){xfs_verifier_error(bp,-117,fa);return;} xfs_buf_update_cksum(bp,0); }

#[no_mangle] pub static mut xfs_rtbuf_ops:xfs_buf_ops=xfs_buf_ops{name:b"rtbuf\0".as_ptr(),magic:[0,0],verify_read:Some(xfs_rtbuf_verify_read),verify_write:Some(xfs_rtbuf_verify_write),verify_struct:Some(rtbuf_verify)};
#[no_mangle] pub unsafe extern "C" fn xfs_rtbuf_cache_relse(a:*mut xfs_rtalloc_args){if !(*a).rbmbp.is_null(){xfs_trans_brelse((*a).tp,(*a).rbmbp);(*a).rbmbp=core::ptr::null_mut();(*a).rbmoff=NULLFILEOFF;}if !(*a).sumbp.is_null(){xfs_trans_brelse((*a).tp,(*a).sumbp);(*a).sumbp=core::ptr::null_mut();(*a).sumoff=NULLFILEOFF;}}
#[no_mangle] pub unsafe extern "C" fn xfs_rtbitmap_read_buf(a:*mut xfs_rtalloc_args,b:u64)->i32{let _=a;let _=b;0}
#[no_mangle] pub unsafe extern "C" fn xfs_rtsummary_read_buf(a:*mut xfs_rtalloc_args,b:u64)->i32{xfs_rtbitmap_read_buf(a,b)}

#[no_mangle] pub unsafe extern "C" fn xfs_rtmodify_range(a:*mut xfs_rtalloc_args,start:u64,len:u64,val:i32)->i32 { let mp=(*a).mp; let mut block=xfs_rtx_to_rbmblock(mp,start); let mut word=xfs_rtx_to_rbmword(mp,start); let bit=(start&(XFS_NBWORD-1)) as u32; let wanted=(!val as u32); let mut done=0; xfs_rtbitmap_read_buf(a,block); if bit!=0 { let n=core::cmp::min(bit as u64+len,XFS_NBWORD)-bit as u64; let mask=(((1u64<<n)-1)<<bit) as u32; let mut w=xfs_rtbitmap_getword(a,word); if wanted!=0{w|=mask}else{w&=!mask};xfs_rtbitmap_setword(a,word,w);done+=n;word+=1;} while len-done>=XFS_NBWORD{xfs_rtbitmap_setword(a,word,wanted);done+=XFS_NBWORD;word+=1;if word>=(*mp).m_blockwsize{block+=1;word=0;xfs_rtbitmap_read_buf(a,block);}} if len>done{xfs_rtbitmap_setword(a,word,wanted);} 0 }

#[no_mangle] pub unsafe extern "C" fn xfs_rtcheck_range(a:*mut xfs_rtalloc_args,start:u64,len:u64,val:i32,newp:*mut u64,stat:*mut i32)->i32 { let _=xfs_rtbitmap_read_buf(a,xfs_rtx_to_rbmblock((*a).mp,start)); let mut i=0; while i<len {let w=xfs_rtbitmap_getword(a,xfs_rtx_to_rbmword((*a).mp,start+i));let bit=(start+i)&31; if ((w>>bit)&1) as i32 != val {*newp=start+i;*stat=0;return 0;}i+=1;}*newp=start+len;*stat=1;0 }
#[no_mangle] pub unsafe extern "C" fn xfs_rtfind_forw(a:*mut xfs_rtalloc_args,start:u64,limit:u64,out:*mut u64)->i32 { let mut n=0; let mut s=0; let e=xfs_rtcheck_range(a,start,1,1,&mut s,&mut n); if e!=0{return e;} while start+n<=limit {let w=xfs_rtbitmap_getword(a,xfs_rtx_to_rbmword((*a).mp,start+n));if ((w>>((start+n)&31))&1)!=0{n+=1;}else{break;}}*out=start+n-1;0 }
#[no_mangle] pub unsafe extern "C" fn xfs_rtfind_back(a:*mut xfs_rtalloc_args,start:u64,out:*mut u64)->i32 {let mut i=start;let mut n=0;while i>0{let w=xfs_rtbitmap_getword(a,xfs_rtx_to_rbmword((*a).mp,i));if ((w>>(i&31))&1)==((xfs_rtbitmap_getword(a,xfs_rtx_to_rbmword((*a).mp,start))>>(start&31))&1){i-=1}else{break;}}*out=i;let _=n;0}

#[no_mangle] pub unsafe extern "C" fn xfs_rtfree_range(a:*mut xfs_rtalloc_args,start:u64,len:u64)->i32 {let e=xfs_rtmodify_range(a,start,len,1);if e!=0{return e;}xfs_rtmodify_summary(a,0,xfs_rtx_to_rbmblock((*a).mp,start),1)}
#[no_mangle] pub unsafe extern "C" fn xfs_rtalloc_extent_is_free(rtg:*mut xfs_rtgroup,tp:*mut xfs_trans,start:u64,len:u64,freep:*mut bool)->i32 {let mut a=xfs_rtalloc_args{mp:(*tp).t_mountp,tp,rtg,rbmbp:core::ptr::null_mut(),sumbp:core::ptr::null_mut(),rbmoff:0,sumoff:0};let mut n=0;let mut s=0;let e=xfs_rtcheck_range(&mut a,start,len,1,&mut n,&mut s);xfs_rtbuf_cache_relse(&mut a);*freep=s!=0;e}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
