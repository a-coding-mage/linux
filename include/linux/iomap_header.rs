/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

/* External kernel types supplied by other headers. */
#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct fiemap_extent_info { _private: [u8; 0] }
#[repr(C)] pub struct inode { pub i_sb: *mut super_block }
#[repr(C)] pub struct iomap_iter { pub inode: *mut inode, pub pos: loff_t, pub len: u64, pub iter_start_pos: loff_t, pub status: i32, pub flags: u32, pub iomap: iomap, pub srcmap: iomap, pub fbatch: *mut folio_batch, pub private: *mut c_void }
#[repr(C)] pub struct iomap_dio { _private: [u8; 0] }
#[repr(C)] pub struct iomap_writepage_ctx { pub iomap: iomap, pub inode: *mut inode, pub wbc: *mut writeback_control, pub ops: *const iomap_writeback_ops, pub nr_folios: u32, pub wb_ctx: *mut c_void }
#[repr(C)] pub struct iomap_read_folio_ctx { pub ops: *const iomap_read_ops, pub cur_folio: *mut folio, pub rac: *mut readahead_control, pub read_ctx: *mut c_void, pub read_ctx_file_offset: loff_t, pub vi: *mut fsverity_info }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct kiocb { pub ki_pos: loff_t, pub ki_filp: *mut file, pub ki_flags: u32 }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct vm_fault { _private: [u8; 0] }
#[repr(C)] pub struct block_device { _private: [u8; 0] }
#[repr(C)] pub struct dax_device { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct folio_batch { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct swap_info_struct { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct bio { _private: [u8; 0] }
#[repr(C)] pub struct bio_set { _private: [u8; 0] }
#[repr(C)] pub struct readahead_control { _private: [u8; 0] }
#[repr(C)] pub struct fsverity_info { _private: [u8; 0] }
#[repr(C)] pub struct writeback_control { _private: [u8; 0] }
#[repr(C)] pub struct super_block { pub s_blocksize: u32 }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
pub type loff_t = i64; pub type u64_t = u64; pub type sector_t = u64; pub type gfp_t = u32; pub type vm_fault_t = u32; pub type ssize_t = isize; pub type bio_end_io_t = unsafe extern "C" fn(*mut bio);

pub const IOMAP_HOLE: u16 = 0; pub const IOMAP_DELALLOC: u16 = 1; pub const IOMAP_MAPPED: u16 = 2; pub const IOMAP_UNWRITTEN: u16 = 3; pub const IOMAP_INLINE: u16 = 4;
pub const IOMAP_F_NEW: u32 = 1 << 0; pub const IOMAP_F_DIRTY: u32 = 1 << 1; pub const IOMAP_F_SHARED: u32 = 1 << 2; pub const IOMAP_F_MERGED: u32 = 1 << 3;
/* CONFIG_BUFFER_HEAD: IOMAP_F_BUFFER_HEAD is 1 << 4, otherwise 0. */ pub const IOMAP_F_BUFFER_HEAD: u32 = 0;
pub const IOMAP_F_XATTR: u32 = 1 << 5; pub const IOMAP_F_BOUNDARY: u32 = 1 << 6; pub const IOMAP_F_ANON_WRITE: u32 = 1 << 7; pub const IOMAP_F_ATOMIC_BIO: u32 = 1 << 8;
/* CONFIG_BLK_DEV_INTEGRITY: IOMAP_F_INTEGRITY is 1 << 9, otherwise 0. */ pub const IOMAP_F_INTEGRITY: u32 = 0;
pub const IOMAP_F_ZERO_TAIL: u32 = 1 << 10; pub const IOMAP_F_FSVERITY: u32 = 1 << 11; pub const IOMAP_F_PRIVATE: u32 = 1 << 12; pub const IOMAP_F_FOLIO_BATCH: u32 = 1 << 13; pub const IOMAP_F_SIZE_CHANGED: u32 = 1 << 14; pub const IOMAP_F_STALE: u32 = 1 << 15;
pub const IOMAP_NULL_ADDR: u64 = u64::MAX;

#[repr(C)] pub struct iomap { pub addr: u64, pub offset: loff_t, pub length: u64, pub r#type: u16, pub flags: u16, pub bdev: *mut block_device, pub dax_dev: *mut dax_device, pub inline_data: *mut c_void, pub private: *mut c_void, pub validity_cookie: u64 }
pub const SECTOR_SHIFT: u32 = 9; pub const U64_MAX: u64 = u64::MAX;
#[inline] pub unsafe fn iomap_sector(i: *const iomap, pos: loff_t) -> sector_t { if ((*i).flags as u32 & IOMAP_F_ANON_WRITE) != 0 { U64_MAX } else { ((*i).addr.wrapping_add(pos as u64).wrapping_sub((*i).offset as u64)) >> SECTOR_SHIFT } }
#[inline] pub unsafe fn iomap_inline_data(i: *const iomap, pos: loff_t) -> *mut c_void { ((*i).inline_data as *mut u8).offset(pos.wrapping_sub((*i).offset) as isize) as *mut c_void }

pub type get_folio_fn = unsafe extern "C" fn(*mut iomap_iter, loff_t, u32) -> *mut folio;
pub type put_folio_fn = unsafe extern "C" fn(*mut inode, loff_t, u32, *mut folio);
pub type iomap_valid_fn = unsafe extern "C" fn(*mut inode, *const iomap) -> bool;
pub type read_folio_range_fn = unsafe extern "C" fn(*const iomap_iter, *mut folio, loff_t, usize) -> i32;
#[repr(C)] pub struct iomap_write_ops { pub get_folio: Option<get_folio_fn>, pub put_folio: Option<put_folio_fn>, pub iomap_valid: Option<iomap_valid_fn>, pub read_folio_range: Option<read_folio_range_fn> }

pub const IOMAP_WRITE:u32=1<<0; pub const IOMAP_ZERO:u32=1<<1; pub const IOMAP_REPORT:u32=1<<2; pub const IOMAP_FAULT:u32=1<<3; pub const IOMAP_DIRECT:u32=1<<4; pub const IOMAP_NOWAIT:u32=1<<5; pub const IOMAP_OVERWRITE_ONLY:u32=1<<6; pub const IOMAP_UNSHARE:u32=1<<7; /* CONFIG_FS_DAX: IOMAP_DAX is 1<<8, otherwise 0. */ pub const IOMAP_DAX:u32=0; pub const IOMAP_ATOMIC:u32=1<<9; pub const IOMAP_DONTCACHE:u32=1<<10;
pub type iomap_iter_begin_fn = unsafe extern "C" fn(*mut inode,loff_t,loff_t,u32,*mut iomap,*mut iomap)->i32; pub type iomap_iter_end_fn = unsafe extern "C" fn(*mut inode,loff_t,loff_t,ssize_t,u32,*mut iomap)->i32; pub type iomap_iter_next_fn = unsafe extern "C" fn(*const iomap_iter,*mut iomap,*mut iomap)->i32;
#[repr(C)] pub struct iomap_ops { pub iomap_begin: Option<iomap_iter_begin_fn>, pub iomap_end: Option<iomap_iter_end_fn>, pub iomap_next: Option<iomap_iter_next_fn> }

extern "C" { pub fn iomap_iter(iter:*mut iomap_iter,ops:*const iomap_ops)->i32; pub fn iomap_iter_advance(iter:*mut iomap_iter,count:u64)->i32; pub fn iomap_iter_continue(iter:*const iomap_iter,iomap:*mut iomap,srcmap:*mut iomap,ret:i32)->i32; }
#[inline] pub unsafe fn iomap_length_trim(iter:*const iomap_iter,pos:loff_t,len:u64)->u64 { let mut end=(*iter).iomap.offset+(*iter).iomap.length as i64; if (*iter).srcmap.r#type!=IOMAP_HOLE { end=core::cmp::min(end,(*iter).srcmap.offset+(*iter).srcmap.length as i64); } core::cmp::min(len,(end-pos) as u64) }
#[inline] pub unsafe fn iomap_length(iter:*const iomap_iter)->u64 { iomap_length_trim(iter,(*iter).pos,(*iter).len) }
#[inline] pub unsafe fn iomap_iter_advance_full(iter:*mut iomap_iter)->i32 { iomap_iter_advance(iter,iomap_length(iter)) }
#[inline] pub unsafe fn iomap_iter_srcmap(i:*const iomap_iter)->*const iomap { if (*i).srcmap.r#type!=IOMAP_HOLE { &(*i).srcmap } else { &(*i).iomap } }

#[inline] pub unsafe fn iomap_last_written_block(_inode:*mut inode,pos:loff_t,written:ssize_t)->loff_t { let b=4096i64; if written==0 { pos & !(b-1) } else { (pos+written as i64+b-1)&!(b-1) } }
#[inline] pub unsafe fn iomap_want_unshare_iter(iter:*const iomap_iter)->bool { ((*iter).iomap.flags as u32&IOMAP_F_SHARED)!=0 && (*iter).srcmap.r#type==IOMAP_MAPPED }

extern "C" {
 pub fn iomap_file_buffered_write(*mut kiocb,*mut iov_iter,*const iomap_ops,*const iomap_write_ops,*mut c_void)->ssize_t; pub fn iomap_fsverity_write(*mut file,loff_t,usize,*const c_void,*const iomap_ops,*const iomap_write_ops)->i32; pub fn iomap_read_folio(*const iomap_ops,*mut iomap_read_folio_ctx,*mut c_void); pub fn iomap_readahead(*const iomap_ops,*mut iomap_read_folio_ctx,*mut c_void); pub fn iomap_is_partially_uptodate(*mut folio,usize,usize)->bool; pub fn iomap_get_folio(*mut iomap_iter,loff_t,usize)->*mut folio; pub fn iomap_release_folio(*mut folio,gfp_t)->bool; pub fn iomap_invalidate_folio(*mut folio,usize,usize); pub fn iomap_dirty_folio(*mut address_space,*mut folio)->bool; pub fn iomap_folio_mark_uptodate(*mut folio); pub fn iomap_file_unshare(*mut inode,loff_t,loff_t,*const iomap_ops,*const iomap_write_ops)->i32; pub fn iomap_fill_dirty_folios(*mut iomap_iter,*mut loff_t,loff_t,*mut u32)->u32; pub fn iomap_zero_range(*mut inode,loff_t,loff_t,*mut bool,*const iomap_ops,*const iomap_write_ops,*mut c_void)->i32; pub fn iomap_truncate_page(*mut inode,loff_t,*mut bool,*const iomap_ops,*const iomap_write_ops,*mut c_void)->i32; pub fn iomap_page_mkwrite(*mut vm_fault,*const iomap_ops,*mut c_void)->vm_fault_t;
 pub fn iomap_fiemap(*mut inode,*mut fiemap_extent_info,u64,u64,*const iomap_ops)->i32; pub fn iomap_seek_hole(*mut inode,loff_t,*const iomap_ops)->loff_t; pub fn iomap_seek_data(*mut inode,loff_t,*const iomap_ops)->loff_t; pub fn iomap_bmap(*mut address_space,sector_t,*const iomap_ops)->sector_t;
}

pub const IOMAP_IOEND_SHARED:u16=1<<0; pub const IOMAP_IOEND_UNWRITTEN:u16=1<<1; pub const IOMAP_IOEND_BOUNDARY:u16=1<<2; pub const IOMAP_IOEND_DIRECT:u16=1<<3; pub const IOMAP_IOEND_NOMERGE_FLAGS:u16=IOMAP_IOEND_SHARED|IOMAP_IOEND_UNWRITTEN|IOMAP_IOEND_DIRECT;
#[repr(C)] pub struct iomap_ioend { pub io_list:list_head, pub io_flags:u16, pub io_inode:*mut inode, pub io_size:usize, pub io_remaining:atomic_t, pub io_error:i32, pub io_parent:*mut iomap_ioend, pub io_offset:loff_t, pub io_sector:sector_t, pub io_private:*mut c_void, pub io_vi:*mut fsverity_info, pub io_bio:bio }
#[repr(C)] pub struct iomap_writeback_ops { pub writeback_range: Option<unsafe extern "C" fn(*mut iomap_writepage_ctx,*mut folio,u64,u32,u64)->ssize_t>, pub writeback_submit: Option<unsafe extern "C" fn(*mut iomap_writepage_ctx,i32)->i32> }
extern "C" { pub fn iomap_init_ioend(*mut inode,*mut bio,loff_t,u16)->*mut iomap_ioend; pub fn iomap_split_ioend(*mut iomap_ioend,u32,bool)->*mut iomap_ioend; pub fn iomap_finish_ioends(*mut iomap_ioend,i32); pub fn iomap_ioend_try_merge(*mut iomap_ioend,*mut list_head); pub fn iomap_sort_ioends(*mut list_head); pub fn iomap_add_to_ioend(*mut iomap_writepage_ctx,*mut folio,loff_t,loff_t,u32)->ssize_t; pub fn iomap_ioend_writeback_submit(*mut iomap_writepage_ctx,i32)->i32; pub fn iomap_finish_folio_read(*mut folio,usize,usize,i32); pub fn iomap_finish_folio_write(*mut inode,*mut folio,usize); pub fn iomap_writeback_folio(*mut iomap_writepage_ctx,*mut folio)->i32; pub fn iomap_writepages(*mut iomap_writepage_ctx)->i32; }

#[repr(C)] pub struct iomap_read_ops { pub read_folio_range: Option<unsafe extern "C" fn(*const iomap_iter,*mut iomap_read_folio_ctx,usize)->i32>, pub submit_read: Option<unsafe extern "C" fn(*const iomap_iter,*mut iomap_read_folio_ctx)>, pub bio_set:*mut bio_set }
pub const IOMAP_DIO_UNWRITTEN:u32=1; pub const IOMAP_DIO_COW:u32=2; #[repr(C)] pub struct iomap_dio_ops { pub end_io:Option<unsafe extern "C" fn(*mut kiocb,ssize_t,i32,u32)->i32>, pub submit_io:Option<unsafe extern "C" fn(*const iomap_iter,*mut bio,loff_t)>, pub bio_set:*mut bio_set }
pub const IOMAP_DIO_FORCE_WAIT:u32=1; pub const IOMAP_DIO_OVERWRITE_ONLY:u32=2; pub const IOMAP_DIO_PARTIAL:u32=4; pub const IOMAP_DIO_FSBLOCK_ALIGNED:u32=8; pub const IOMAP_DIO_BOUNCE:u32=16;
extern "C" { pub fn iomap_dio_rw(*mut kiocb,*mut iov_iter,*const iomap_ops,*const iomap_dio_ops,u32,*mut c_void,usize)->ssize_t; pub fn __iomap_dio_rw(*mut kiocb,*mut iov_iter,*const iomap_ops,*const iomap_dio_ops,u32,*mut c_void,usize)->*mut iomap_dio; pub fn iomap_dio_complete(*mut iomap_dio)->ssize_t; pub fn iomap_dio_bio_end_io(*mut bio); pub fn __iomap_dio_read_simple(*mut kiocb,*mut iov_iter,*mut iomap_iter)->ssize_t; }

pub type iomap_punch_t = unsafe extern "C" fn(*mut inode,loff_t,loff_t,*mut iomap);
extern "C" { pub fn iomap_write_delalloc_release(*mut inode,loff_t,loff_t,u32,*mut iomap,Option<iomap_punch_t>); }
/* CONFIG_SWAP and CONFIG_BLOCK declarations are available only when enabled by the build. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
