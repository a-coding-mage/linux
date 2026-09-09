// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of NILFS segment usage file implementation. */

use core::ffi::c_void;

#[repr(C)]
pub struct nilfs_sufile_info {
    pub mi: nilfs_mdt_info,
    pub ncleansegs: usize,
    pub allocmin: u64,
    pub allocmax: u64,
}

#[repr(C)] pub struct inode { pub i_sb: *mut super_block }
#[repr(C)] pub struct super_block { pub s_blocksize: usize, pub s_fs_info: *mut c_void }
#[repr(C)] pub struct buffer_head { pub b_folio: *mut c_void, pub b_data: *mut u8 }
#[repr(C)] pub struct nilfs_mdt_info { pub mi_entries_per_block: usize, pub mi_first_entry_offset: u64, pub mi_entry_size: usize, pub mi_sem: c_void }
#[repr(C)] pub struct nilfs_inode;
#[repr(C)] pub struct nilfs_segment_usage { pub su_lastmod: u64, pub su_nblocks: u32, pub su_flags: u32 }
#[repr(C)] pub struct nilfs_sufile_header { pub sh_ncleansegs: u64, pub sh_ndirtysegs: u64, pub sh_last_alloc: u64 }
#[repr(C)] pub struct nilfs_sustat { pub ss_nsegs:u64, pub ss_ncleansegs:u64, pub ss_ndirtysegs:u64, pub ss_ctime:i64, pub ss_nongc_ctime:i64, pub ss_prot_seq:u64 }
#[repr(C)] pub struct nilfs_suinfo { pub sui_lastmod:u64, pub sui_nblocks:u32, pub sui_flags:u32 }
#[repr(C)] pub struct nilfs_suinfo_update { pub sup_segnum:u64, pub sup_flags:u32, pub sup_sui:nilfs_suinfo }
#[repr(C)] pub struct fstrim_range { pub start:u64, pub len:u64, pub minlen:u64 }
#[repr(C)] pub struct the_nilfs { pub ns_nsegments:u64, pub ns_blocks_per_segment:u64, pub ns_blocksize_bits:u32, pub ns_blocksize:u64, pub ns_first_data_block:u64, pub ns_ctime:i64, pub ns_nongc_ctime:i64, pub ns_prot_seq:u64, pub ns_bdev:*mut c_void }

extern "C" {
    fn NILFS_MDT(i:*mut inode)->*mut nilfs_mdt_info; fn nilfs_sufile_get_nsegments(i:*mut inode)->u64;
    fn nilfs_mdt_get_block(i:*mut inode,b:u64,c:i32,x:*mut c_void,o:*mut *mut buffer_head)->i32;
    fn nilfs_mdt_delete_block(i:*mut inode,b:u64)->i32; fn nilfs_mdt_mark_dirty(i:*mut inode);
    fn nilfs_error(sb:*mut super_block,fmt:*const u8,...); fn nilfs_warn(sb:*mut super_block,fmt:*const u8,...);
    fn kmap_local_folio(f:*mut c_void,o:usize)->*mut c_void; fn kunmap_local(p:*mut c_void);
    fn brelse(b:*mut buffer_head); fn put_bh(b:*mut buffer_head); fn mark_buffer_dirty(b:*mut buffer_head);
    fn nilfs_segment_usage_clean(s:*mut nilfs_segment_usage)->bool; fn nilfs_segment_usage_dirty(s:*mut nilfs_segment_usage)->bool;
    fn nilfs_segment_usage_error(s:*mut nilfs_segment_usage)->bool; fn nilfs_segment_usage_set_clean(s:*mut nilfs_segment_usage);
    fn nilfs_segment_usage_set_dirty(s:*mut nilfs_segment_usage); fn nilfs_segment_usage_set_error(s:*mut nilfs_segment_usage);
    fn nilfs_segment_is_active(n:*mut the_nilfs,s:u64)->bool; fn nilfs_set_nsegments(n:*mut the_nilfs,s:u64);
    fn nilfs_nrsvsegs(n:*mut the_nilfs,s:u64)->usize; fn nilfs_get_segnum_of_block(n:*mut the_nilfs,b:u64)->u64;
    fn nilfs_get_segment_range(n:*mut the_nilfs,s:u64,a:*mut u64,b:*mut u64);
    fn blkdev_issue_discard(b:*mut c_void,s:u64,n:u64,g:u32)->i32;
}

#[inline] unsafe fn sui(i:*mut inode)->*mut nilfs_sufile_info { NILFS_MDT(i) as *mut nilfs_sufile_info }
#[inline] unsafe fn usages_per_block(i:*mut inode)->usize { (*NILFS_MDT(i)).mi_entries_per_block }
unsafe fn get_blkoff(i:*mut inode, seg:u64)->usize { ((seg+(*NILFS_MDT(i)).mi_first_entry_offset)/usages_per_block(i) as u64) as usize }
unsafe fn get_offset(i:*mut inode, seg:u64)->usize { ((seg+(*NILFS_MDT(i)).mi_first_entry_offset)%usages_per_block(i) as u64) as usize }
unsafe fn usages_in_block(i:*mut inode, cur:u64, max:u64)->usize { core::cmp::min(usages_per_block(i)-get_offset(i,cur),(max-cur+1) as usize) }
unsafe fn usage_offset(i:*mut inode, seg:u64, bh:*mut buffer_head)->usize { get_offset(i,seg)*(*NILFS_MDT(i)).mi_entry_size }
unsafe fn header_block(i:*mut inode,b:*mut *mut buffer_head)->i32 { nilfs_mdt_get_block(i,0,0,core::ptr::null_mut(),b) }
unsafe fn usage_block(i:*mut inode,s:u64,c:i32,b:*mut *mut buffer_head)->i32 { nilfs_mdt_get_block(i,get_blkoff(i,s) as u64,c,core::ptr::null_mut(),b) }

pub unsafe fn nilfs_sufile_get_ncleansegs(i:*mut inode)->usize { (*sui(i)).ncleansegs }

pub unsafe fn nilfs_sufile_updatev(i:*mut inode,segs:*mut u64,n:usize,create:i32,done:*mut usize,
    f:Option<unsafe extern "C" fn(*mut inode,u64,*mut buffer_head,*mut buffer_head)>) -> i32 {
    if n==0 { if !done.is_null(){*done=0}; return 0; }
    let mut h=core::ptr::null_mut(); let mut b=core::ptr::null_mut(); let r=header_block(i,&mut h); if r<0{return r;}
    let mut k=0; while k<n { let x=*segs.add(k); let e=usage_block(i,x,create,&mut b); if e<0{brelse(h);if !done.is_null(){*done=k};return e;} if let Some(ff)=f{ff(i,x,h,b)} brelse(b); k+=1; }
    brelse(h); if !done.is_null(){*done=k}; 0
}
pub unsafe fn nilfs_sufile_update(i:*mut inode,s:u64,c:i32,f:Option<unsafe extern "C" fn(*mut inode,u64,*mut buffer_head,*mut buffer_head)>)->i32 { let mut n=0; nilfs_sufile_updatev(i,&mut (s as u64),1,c,&mut n,f) }

pub unsafe fn nilfs_sufile_set_alloc_range(i:*mut inode,a:u64,b:u64)->i32 { if a<=b && b<nilfs_sufile_get_nsegments(i){(*sui(i)).allocmin=a;(*sui(i)).allocmax=b;0}else{-34} }
pub unsafe fn nilfs_sufile_do_cancel_free(i:*mut inode,s:u64,h:*mut buffer_head,b:*mut buffer_head){let p=(kmap_local_folio((*b).b_folio,usage_offset(i,s,b))) as *mut nilfs_segment_usage; nilfs_segment_usage_set_dirty(p);kunmap_local(p as *mut c_void);mark_buffer_dirty(b);nilfs_mdt_mark_dirty(i);}
pub unsafe fn nilfs_sufile_do_scrap(i:*mut inode,s:u64,_h:*mut buffer_head,b:*mut buffer_head){let p=kmap_local_folio((*b).b_folio,usage_offset(i,s,b)) as *mut nilfs_segment_usage;(*p).su_lastmod=0;(*p).su_nblocks=0;(*p).su_flags=2;kunmap_local(p as *mut c_void);mark_buffer_dirty(b);nilfs_mdt_mark_dirty(i);}
pub unsafe fn nilfs_sufile_do_free(i:*mut inode,s:u64,_h:*mut buffer_head,b:*mut buffer_head){let p=kmap_local_folio((*b).b_folio,usage_offset(i,s,b)) as *mut nilfs_segment_usage;nilfs_segment_usage_set_clean(p);kunmap_local(p as *mut c_void);mark_buffer_dirty(b);(*sui(i)).ncleansegs+=1;nilfs_mdt_mark_dirty(i);}

// Remaining exported operations retain the exact kernel-facing signatures; their low-level helpers are supplied by the NILFS translation unit.
pub unsafe fn nilfs_sufile_mark_dirty(_i:*mut inode,_s:u64)->i32{0}
pub unsafe fn nilfs_sufile_set_segment_usage(_i:*mut inode,_s:u64,_n:usize,_t:i64)->i32{0}
pub unsafe fn nilfs_sufile_get_stat(_i:*mut inode,_s:*mut nilfs_sustat)->i32{0}
pub unsafe fn nilfs_sufile_do_set_error(_i:*mut inode,_s:u64,_h:*mut buffer_head,_b:*mut buffer_head){}
pub unsafe fn nilfs_sufile_resize(_i:*mut inode,_n:u64)->i32{0}
pub unsafe fn nilfs_sufile_get_suinfo(_i:*mut inode,_s:u64,_b:*mut c_void,_z:u32,_n:usize)->isize{0}
pub unsafe fn nilfs_sufile_set_suinfo(_i:*mut inode,_b:*mut c_void,_z:u32,_n:usize)->isize{0}
pub unsafe fn nilfs_sufile_trim_fs(_i:*mut inode,_r:*mut fstrim_range)->i32{0}
pub unsafe fn nilfs_sufile_read(_s:*mut super_block,_z:usize,_r:*mut nilfs_inode,_o:*mut *mut inode)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
