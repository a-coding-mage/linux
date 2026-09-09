/* SPDX-License-Identifier: GPL-2.0 */
//! Rust translation of Linux `pagemap.h`.
//!
//! The Linux headers included by the original are external dependencies.  Their
//! types and functions are therefore referenced here but not implemented.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::c_void;

/* External Linux types supplied by the surrounding kernel translation. */
#[repr(C)] pub struct address_space { pub i_pages: xarray, pub wb_err: errseq_t, pub host: *mut inode, pub flags: u64, pub gfp_mask: gfp_t, pub nrpages: usize }
#[repr(C)] pub struct inode { pub i_mode: u16, pub i_mapping: *mut address_space, pub i_sb: *mut super_block, pub i_size: i64, pub i_blkbits: u32 }
#[repr(C)] pub struct super_block { pub s_wb_err: errseq_t }
#[repr(C)] pub struct file { pub f_path: path }
#[repr(C)] pub struct path { pub dentry: *mut dentry }
#[repr(C)] pub struct dentry { pub d_sb: *mut super_block }
#[repr(C)] pub struct folio { pub mapping: *mut address_space, pub index: pgoff_t, pub private: *mut c_void, pub page: page }
#[repr(C)] pub struct page;
#[repr(C)] pub struct kiocb { pub ki_flags: u32 }
#[repr(C)] pub struct vm_area_struct { pub vm_start: usize, pub vm_file: *mut file }
#[repr(C)] pub struct vm_fault;
#[repr(C)] pub struct bdi_writeback;
#[repr(C)] pub struct mempolicy;
#[repr(C)] pub struct file_ra_state;
#[repr(C)] pub struct folio_batch;
#[repr(C)] pub struct xarray { pub xa_head: *mut c_void }
#[repr(C)] pub struct wait_queue_entry_t;

pub type pgoff_t = usize;
pub type loff_t = i64;
pub type gfp_t = u32;
pub type errseq_t = u32;
pub type xa_mark_t = u32;
pub type vm_fault_t = u32;
pub type ssize_t = isize;
pub type filler_t = unsafe extern "C" fn(*mut file, *mut folio) -> i32;
pub type fgf_t = u32;

pub const LLONG_MAX: loff_t = i64::MAX;
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const PMD_ORDER: u32 = 9;
pub const XA_CHUNK_SHIFT: u32 = 6;
pub const FOLIO_MAPPING_ANON: usize = 1;
pub const IOCB_DONTCACHE: u32 = 1 << 16;
pub const __GFP_NORETRY: gfp_t = 1 << 0;
pub const __GFP_NOWARN: gfp_t = 1 << 1;
pub const __GFP_FS: gfp_t = 1 << 2;
pub const ENOSPC: i32 = 28;
pub const EFAULT: isize = 14;
pub const EINTR: i32 = 4;
pub const ENOENT: i32 = 2;
pub const ENOMEM: i32 = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mapping_flags { AS_EIO=0, AS_ENOSPC=1, AS_MM_ALL_LOCKS=2, AS_UNEVICTABLE=3, AS_EXITING=4, AS_NO_WRITEBACK_TAGS=5, AS_RELEASE_ALWAYS=6, AS_STABLE_WRITES=7, AS_INACCESSIBLE=8, AS_WRITEBACK_MAY_DEADLOCK_ON_RECLAIM=9, AS_KERNEL_FILE=10, AS_FOLIO_ORDER_BITS=5, AS_FOLIO_ORDER_MIN=16, AS_FOLIO_ORDER_MAX=21 }
pub const AS_FOLIO_ORDER_BITS_MASK: u32 = (1u32 << 5) - 1;
pub const AS_FOLIO_ORDER_MIN_MASK: u64 = (AS_FOLIO_ORDER_BITS_MASK as u64) << 16;
pub const AS_FOLIO_ORDER_MAX_MASK: u64 = (AS_FOLIO_ORDER_BITS_MASK as u64) << 21;
pub const AS_FOLIO_ORDER_MASK: u64 = AS_FOLIO_ORDER_MIN_MASK | AS_FOLIO_ORDER_MAX_MASK;

pub const FGP_ACCESSED: fgf_t=0x1; pub const FGP_LOCK: fgf_t=0x2; pub const FGP_CREAT: fgf_t=0x4;
pub const FGP_WRITE: fgf_t=0x8; pub const FGP_NOFS: fgf_t=0x10; pub const FGP_NOWAIT: fgf_t=0x20;
pub const FGP_FOR_MMAP: fgf_t=0x40; pub const FGP_STABLE: fgf_t=0x80; pub const FGP_DONTCACHE: fgf_t=0x100;
pub const FGP_WRITEBEGIN: fgf_t = FGP_LOCK | FGP_WRITE | FGP_CREAT | FGP_STABLE;
#[inline] pub const fn FGF_GET_ORDER(f: fgf_t) -> u32 { f >> 26 }

pub const PREFERRED_MAX_PAGECACHE_ORDER: u32 = 8;
pub const MAX_XAS_ORDER: u32 = XA_CHUNK_SHIFT * 2 - 1;
pub const MAX_PAGECACHE_ORDER: u32 = if MAX_XAS_ORDER < PREFERRED_MAX_PAGECACHE_ORDER { MAX_XAS_ORDER } else { PREFERRED_MAX_PAGECACHE_ORDER };

extern "C" {
    pub fn invalidate_mapping_pages(*mut address_space, pgoff_t, pgoff_t) -> usize;
    pub fn invalidate_inode_pages2(*mut address_space) -> i32;
    pub fn invalidate_inode_pages2_range(*mut address_space, pgoff_t, pgoff_t) -> i32;
    pub fn kiocb_invalidate_pages(*mut kiocb, usize) -> i32;
    pub fn kiocb_invalidate_post_direct_write(*mut kiocb, usize);
    pub fn filemap_invalidate_pages(*mut address_space, loff_t, loff_t, bool) -> i32;
    pub fn filemap_fdatawait_range(*mut address_space, loff_t, loff_t) -> i32;
    pub fn filemap_write_and_wait_range(*mut address_space, loff_t, loff_t) -> i32;
    pub fn __filemap_set_wb_err(*mut address_space, i32);
    pub fn folio_mapping(*const folio) -> *mut address_space;
    pub fn __filemap_get_folio_mpol(*mut address_space, pgoff_t, fgf_t, gfp_t, *mut mempolicy) -> *mut folio;
    pub fn pagecache_get_page(*mut address_space, pgoff_t, fgf_t, gfp_t) -> *mut page;
    pub fn filemap_get_entry(*mut address_space, pgoff_t) -> *mut c_void;
    pub fn page_cache_next_miss(*mut address_space, pgoff_t, usize) -> pgoff_t;
    pub fn page_cache_prev_miss(*mut address_space, pgoff_t, usize) -> pgoff_t;
    pub fn __folio_lock(*mut folio); pub fn __folio_lock_killable(*mut folio) -> i32;
    pub fn __folio_lock_or_retry(*mut folio, *mut vm_fault) -> vm_fault_t;
    pub fn folio_unlock(*mut folio); pub fn folio_wait_bit(*mut folio, i32); pub fn folio_wait_bit_killable(*mut folio,i32)->i32;
    pub fn folio_end_read(*mut folio,bool); pub fn folio_wait_writeback(*mut folio); pub fn folio_end_writeback(*mut folio);
    pub fn filemap_add_folio(*mut address_space,*mut folio,pgoff_t,gfp_t)->i32; pub fn filemap_remove_folio(*mut folio);
    pub fn filemap_range_has_writeback(*mut address_space,loff_t,loff_t)->bool;
    pub fn read_cache_folio(*mut address_space,pgoff_t,Option<filler_t>,*mut file)->*mut folio;
    pub fn read_cache_page(*mut address_space,pgoff_t,Option<filler_t>,*mut file)->*mut page;
    pub fn mapping_tagged(*const address_space,u32)->bool;
}

#[inline] pub unsafe fn filemap_fdatawait(m:*mut address_space)->i32 { filemap_fdatawait_range(m,0,LLONG_MAX) }
#[inline] pub unsafe fn filemap_write_and_wait(m:*mut address_space)->i32 { filemap_write_and_wait_range(m,0,LLONG_MAX) }
#[inline] pub unsafe fn filemap_set_wb_err(m:*mut address_space,e:i32) { if e != 0 { __filemap_set_wb_err(m,e); } }
#[inline] pub unsafe fn mapping_gfp_mask(m:*const address_space)->gfp_t { (*m).gfp_mask }
#[inline] pub unsafe fn mapping_gfp_constraint(m:*const address_space,g:gfp_t)->gfp_t { mapping_gfp_mask(m)&g }
#[inline] pub unsafe fn mapping_set_gfp_mask(m:*mut address_space,g:gfp_t) { (*m).gfp_mask=g; }
#[inline] pub unsafe fn readahead_gfp_mask(m:*mut address_space)->gfp_t { mapping_gfp_mask(m)|__GFP_NORETRY|__GFP_NOWARN }
#[inline] pub const fn filemap_get_order(size:usize)->u32 { if size <= PAGE_SIZE {0} else {(usize::BITS - size.leading_zeros() - 1) - PAGE_SHIFT as u32} }
#[inline] pub fn fgf_set_order(size:usize)->fgf_t { let o=filemap_get_order(size); if o==0 {0} else {o<<26} }
#[inline] pub unsafe fn __filemap_get_folio(m:*mut address_space,i:pgoff_t,f:fgf_t,g:gfp_t)->*mut folio { __filemap_get_folio_mpol(m,i,f,g,core::ptr::null_mut()) }
#[inline] pub unsafe fn filemap_get_folio(m:*mut address_space,i:pgoff_t)->*mut folio { __filemap_get_folio(m,i,0,0) }
#[inline] pub unsafe fn filemap_lock_folio(m:*mut address_space,i:pgoff_t)->*mut folio { __filemap_get_folio(m,i,FGP_LOCK,0) }
#[inline] pub unsafe fn find_get_page(m:*mut address_space,i:pgoff_t)->*mut page { pagecache_get_page(m,i,0,0) }
#[inline] pub unsafe fn find_lock_page(m:*mut address_space,i:pgoff_t)->*mut page { pagecache_get_page(m,i,FGP_LOCK,0) }
#[inline] pub unsafe fn folio_next_index(f:*const folio)->pgoff_t { (*f).index + PAGE_SIZE/PAGE_SIZE }
#[inline] pub unsafe fn folio_pos(f:*const folio)->loff_t { (*f).index as loff_t * PAGE_SIZE as loff_t }
#[inline] pub unsafe fn page_pgoff(f:*const folio,_p:*const page)->pgoff_t { (*f).index }
#[inline] pub unsafe fn linear_page_delta(v:*const vm_area_struct,a:usize)->pgoff_t { (a-(*v).vm_start)>>PAGE_SHIFT }
#[inline] pub unsafe fn linear_page_index(v:*const vm_area_struct,a:usize)->pgoff_t { linear_page_delta(v,a) }

#[repr(C)] pub struct wait_page_key { pub folio:*mut folio, pub bit_nr:i32, pub page_match:i32 }
#[repr(C)] pub struct wait_page_queue { pub folio:*mut folio, pub bit_nr:i32, pub wait:wait_queue_entry_t }
#[repr(C)] pub struct readahead_control { pub file:*mut file, pub mapping:*mut address_space, pub ra:*mut file_ra_state, pub _index:pgoff_t, pub _nr_pages:u32, pub _batch_count:u32, pub dropbehind:bool, pub _workingset:bool, pub _pflags:usize }

#[inline] pub unsafe fn readahead_pos(r:*const readahead_control)->loff_t { (*r)._index as loff_t * PAGE_SIZE as loff_t }
#[inline] pub unsafe fn readahead_length(r:*const readahead_control)->usize { (*r)._nr_pages as usize * PAGE_SIZE }
#[inline] pub unsafe fn readahead_index(r:*const readahead_control)->pgoff_t { (*r)._index }
#[inline] pub unsafe fn readahead_count(r:*const readahead_control)->u32 { (*r)._nr_pages }
#[inline] pub unsafe fn readahead_batch_length(r:*const readahead_control)->usize { (*r)._batch_count as usize * PAGE_SIZE }
#[inline] pub unsafe fn dir_pages(i:*const inode)->usize { ((*i).i_size as usize + PAGE_SIZE - 1)>>PAGE_SHIFT }
#[inline] pub unsafe fn i_blocks_per_folio(i:*const inode, _f:*const folio)->u32 { (PAGE_SIZE as u32) >> (*i).i_blkbits }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
