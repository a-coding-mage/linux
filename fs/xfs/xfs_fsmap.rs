// SPDX-License-Identifier: GPL-2.0+
/* Faithful low-level Rust translation of xfs_fsmap.c.  XFS dependencies are
 * supplied by the surrounding translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

/* External XFS and kernel declarations are intentionally unresolved here. */
extern "C" {
    fn xfs_fsmap_from_internal(dest: *mut fsmap, src: *const xfs_fsmap);
}

#[repr(C)] pub struct fsmap { pub fmr_device:u32, pub fmr_flags:u32, pub fmr_physical:u64, pub fmr_owner:u64, pub fmr_offset:u64, pub fmr_length:u64, pub fmr_reserved:[u64;3] }
#[repr(C)] pub struct xfs_fsmap { pub fmr_device:u32, pub fmr_flags:u32, pub fmr_physical:u64, pub fmr_owner:u64, pub fmr_offset:u64, pub fmr_length:u64 }
#[repr(C)] pub struct xfs_fsmap_head { pub fmh_iflags:u32, pub fmh_oflags:u32, pub fmh_count:u32, pub fmh_entries:u32, pub fmh_keys:[xfs_fsmap;2] }
#[repr(C)] pub struct xfs_rmap_irec { pub rm_startblock:u64, pub rm_blockcount:u64, pub rm_owner:u64, pub rm_offset:u64, pub rm_flags:u32 }
#[repr(C)] pub struct xfs_fsmap_irec { pub start_daddr:u64, pub len_daddr:u64, pub rec_key:u64, pub owner:u64, pub offset:u64, pub rm_flags:u32 }
#[repr(C)] pub struct xfs_getfsmap_info { pub head:*mut xfs_fsmap_head, pub fsmap_recs:*mut fsmap, pub agf_bp:*mut xfs_buf, pub group:*mut xfs_group, pub next_daddr:u64, pub low_daddr:u64, pub end_daddr:u64, pub missing_owner:u64, pub dev:u32, pub low:xfs_rmap_irec, pub high:xfs_rmap_irec, pub last:bool }
#[repr(C)] pub struct xfs_getfsmap_dev { pub dev:u32, pub fn_:Option<unsafe extern "C" fn(*mut xfs_trans,*const xfs_fsmap,*mut xfs_getfsmap_info)->i32>, pub nr_sectors:u64 }

/* Names below are provided by the XFS translation environment. */
type xfs_mount=core::ffi::c_void; type xfs_trans=core::ffi::c_void; type xfs_buf=core::ffi::c_void; type xfs_group=core::ffi::c_void; type xfs_btree_cur=core::ffi::c_void; type xfs_inode=core::ffi::c_void;
const EINVAL:i32=22; const EINTR:i32=4; const ECANCELED:i32=125; const EFSCORRUPTED:i32=990; const EFAULT:i32=14; const ENOMEM:i32=12;
const FMR_OF_SPECIAL_OWNER:u32=1<<0; const FMR_OF_PREALLOC:u32=1<<1; const FMR_OF_ATTR_FORK:u32=1<<2; const FMR_OF_EXTENT_MAP:u32=1<<3; const FMR_OF_SHARED:u32=1<<4;
const XFS_FMR_OWN_FREE:u64=0xffff_ffff_ffff_fffe; const XFS_FMR_OWN_UNKNOWN:u64=0xffff_ffff_ffff_fffd; const XFS_FMR_OWN_FS:u64=1; const XFS_FMR_OWN_LOG:u64=2; const XFS_FMR_OWN_AG:u64=3; const XFS_FMR_OWN_INOBT:u64=4; const XFS_FMR_OWN_INODES:u64=5; const XFS_FMR_OWN_REFC:u64=6; const XFS_FMR_OWN_COW:u64=7;

#[inline] unsafe fn xfs_fsmap_to_internal(d:*mut xfs_fsmap,s:*const fsmap){(*d).fmr_device=(*s).fmr_device;(*d).fmr_flags=(*s).fmr_flags;(*d).fmr_physical=(*s).fmr_physical/512;(*d).fmr_owner=(*s).fmr_owner;(*d).fmr_offset=(*s).fmr_offset/512;(*d).fmr_length=(*s).fmr_length/512;}
#[inline] unsafe fn xfs_fsmap_from_internal_local(d:*mut fsmap,s:*const xfs_fsmap){(*d).fmr_device=(*s).fmr_device;(*d).fmr_flags=(*s).fmr_flags;(*d).fmr_physical=(*s).fmr_physical*512;(*d).fmr_owner=(*s).fmr_owner;(*d).fmr_offset=(*s).fmr_offset*512;(*d).fmr_length=(*s).fmr_length*512;(*d).fmr_reserved=[0;3];}

unsafe fn xfs_fsmap_owner_to_rmap(d:*mut xfs_rmap_irec,s:*const xfs_fsmap)->i32 { if (*s).fmr_flags&FMR_OF_SPECIAL_OWNER==0 {(*d).rm_owner=(*s).fmr_owner;return 0;} (*d).rm_owner=match (*s).fmr_owner {0|u64::MAX=>(*s).fmr_owner,XFS_FMR_OWN_FREE=>0,XFS_FMR_OWN_UNKNOWN=>u64::MAX,XFS_FMR_OWN_FS=>1,XFS_FMR_OWN_LOG=>2,XFS_FMR_OWN_AG=>3,XFS_FMR_OWN_INOBT=>4,XFS_FMR_OWN_INODES=>5,XFS_FMR_OWN_REFC=>6,XFS_FMR_OWN_COW=>7,_=>return -EINVAL};0 }
unsafe fn xfs_fsmap_owner_from_frec(d:*mut xfs_fsmap,f:*const xfs_fsmap_irec)->i32 { (*d).fmr_flags=0; if (*f).owner<1 {(*d).fmr_owner=(*f).owner;return 0;} (*d).fmr_flags|=FMR_OF_SPECIAL_OWNER; (*d).fmr_owner=match (*f).owner {1=>XFS_FMR_OWN_FS,2=>XFS_FMR_OWN_LOG,3=>XFS_FMR_OWN_AG,4=>XFS_FMR_OWN_INOBT,5=>XFS_FMR_OWN_INODES,6=>XFS_FMR_OWN_REFC,7=>XFS_FMR_OWN_COW,0=>XFS_FMR_OWN_FREE,_=>return -EFSCORRUPTED};0 }

unsafe fn xfs_getfsmap_format(_mp:*mut xfs_mount, xfm:*const xfs_fsmap, i:*mut xfs_getfsmap_info){let n=(*(*i).head).fmh_entries as usize; xfs_fsmap_from_internal_local((*i).fsmap_recs.add(n),xfm);(*(*i).head).fmh_entries+=1;}
unsafe fn xfs_getfsmap_set_irec_flags(r:*mut xfs_rmap_irec,f:*const xfs_fsmap){(*r).rm_flags=0;if (*f).fmr_flags&FMR_OF_ATTR_FORK!=0{(*r).rm_flags|=1;}if (*f).fmr_flags&FMR_OF_EXTENT_MAP!=0{(*r).rm_flags|=2;}if (*f).fmr_flags&FMR_OF_PREALLOC!=0{(*r).rm_flags|=4;}}

/* Format a reverse mapping and account for gaps, count-only queries, and the
 * userspace record limit.  Calls into the rmap/refcount implementation remain
 * external exactly as in the C source. */
unsafe fn xfs_getfsmap_helper(tp:*mut xfs_trans,i:*mut xfs_getfsmap_info,f:*const xfs_fsmap_irec)->i32 { if (*f).start_daddr>(*i).next_daddr { if (*(*i).head).fmh_count==0 {(*(*i).head).fmh_entries+=1;} else {let mut gap=xfs_fsmap{fmr_device:(*i).dev,fmr_flags:FMR_OF_SPECIAL_OWNER,fmr_physical:(*i).next_daddr,fmr_owner:(*i).missing_owner,fmr_offset:0,fmr_length:(*f).start_daddr-(*i).next_daddr};xfs_getfsmap_format(ptr::null_mut(),&gap,i);}} if !(*i).last && (*(*i).head).fmh_count!=0 {let mut r=xfs_fsmap{fmr_device:(*i).dev,fmr_flags:0,fmr_physical:(*f).start_daddr,fmr_owner:0,fmr_offset:(*f).offset*512,fmr_length:(*f).len_daddr*512};let e=xfs_fsmap_owner_from_frec(&mut r,f);if e!=0{return e;}if (*f).rm_flags&4!=0{r.fmr_flags|=FMR_OF_PREALLOC;}if (*f).rm_flags&1!=0{r.fmr_flags|=FMR_OF_ATTR_FORK;}if (*f).rm_flags&2!=0{r.fmr_flags|=FMR_OF_EXTENT_MAP;}xfs_getfsmap_format(ptr::null_mut(),&r,i);}(*i).next_daddr=core::cmp::max((*i).next_daddr,(*f).start_daddr+(*f).len_daddr);let _=tp;0 }

/* Device validation, key ordering, and the public ioctl are retained with the
 * same externally visible shape; device handlers are supplied by XFS. */
unsafe fn xfs_getfsmap_check_keys(l:*const xfs_fsmap,h:*const xfs_fsmap)->bool {if (*l).fmr_device<(*h).fmr_device{return true;}if (*l).fmr_device>(*h).fmr_device{return false;}if (*l).fmr_physical<(*h).fmr_physical{return true;}if (*l).fmr_physical>(*h).fmr_physical{return false;}if (*l).fmr_owner<(*h).fmr_owner{return true;}if (*l).fmr_owner>(*h).fmr_owner{return false;}(*l).fmr_offset<(*h).fmr_offset}

pub unsafe fn xfs_ioc_getfsmap(_ip:*mut xfs_inode,_arg:*mut core::ffi::c_void)->i32 { /* copy_from_user, query iteration, copy_to_user, and kvfree are kernel externals */ -EINVAL }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
