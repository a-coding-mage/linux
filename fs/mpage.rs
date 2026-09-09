// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of fs/mpage.c.  Kernel types and functions are
 * supplied by the surrounding kernel translation unit. */

use core::ffi::c_void;

extern "C" {
    fn blk_status_to_errno(status: i32) -> i32;
    fn bio_put(bio: *mut bio);
    fn submit_bio(bio: *mut bio);
    fn folio_end_read(folio: *mut folio, uptodate: bool);
    fn mapping_set_error(mapping: *mut address_space, err: i32);
    fn folio_end_writeback(folio: *mut folio);
    fn bio_alloc(bdev: *mut block_device, vecs: u32, opf: u32, gfp: u32) -> *mut bio;
    fn bio_add_folio(bio: *mut bio, folio: *mut folio, len: usize, off: usize) -> bool;
    fn bio_max_segs(n: u32) -> u32;
    fn readahead_folio(rac: *mut readahead_control) -> *mut folio;
    fn readahead_count(rac: *mut readahead_control) -> u32;
    fn folio_size(folio: *mut folio) -> usize;
    fn folio_nr_pages(folio: *mut folio) -> u32;
    fn folio_pos(folio: *mut folio) -> u64;
    fn folio_shift(folio: *mut folio) -> u32;
    fn folio_buffers(folio: *mut folio) -> *mut buffer_head;
    fn folio_mark_uptodate(folio: *mut folio);
    fn folio_unlock(folio: *mut folio);
    fn folio_zero_segment(folio: *mut folio, from: usize, to: usize);
    fn folio_set_mappedtodisk(folio: *mut folio);
    fn folio_test_uptodate(folio: *mut folio) -> bool;
    fn folio_test_locked(folio: *mut folio) -> bool;
    fn folio_test_writeback(folio: *mut folio) -> bool;
    fn folio_start_writeback(folio: *mut folio);
    fn i_size_read(inode: *mut inode) -> i64;
    fn i_blocksize(inode: *mut inode) -> u32;
    fn create_empty_buffers(folio: *mut folio, size: u32, state: u32) -> *mut buffer_head;
    fn buffer_uptodate(bh: *mut buffer_head) -> bool;
    fn buffer_mapped(bh: *mut buffer_head) -> bool;
    fn buffer_boundary(bh: *mut buffer_head) -> bool;
    fn buffer_dirty(bh: *mut buffer_head) -> bool;
    fn buffer_new(bh: *mut buffer_head) -> bool;
    fn buffer_locked(bh: *mut buffer_head) -> bool;
    fn clear_buffer_mapped(bh: *mut buffer_head);
    fn clear_buffer_dirty(bh: *mut buffer_head);
    fn try_to_free_buffers(folio: *mut folio) -> bool;
    fn block_read_full_folio(folio: *mut folio, get_block: get_block_t) -> i32;
    fn block_write_full_folio(folio: *mut folio, wbc: *mut writeback_control, get_block: get_block_t) -> i32;
    fn clean_bdev_bh_alias(bh: *mut buffer_head);
    fn write_boundary_block(bdev: *mut block_device, block: u64, size: u32);
    fn wbc_to_write_flags(wbc: *mut writeback_control) -> u32;
    fn wbc_init_bio(wbc: *mut writeback_control, bio: *mut bio);
    fn wbc_account_cgroup_owner(wbc: *mut writeback_control, folio: *mut folio, size: usize);
    fn mapping_gfp_constraint(mapping: *mut address_space, gfp: u32) -> u32;
    fn prefetchw(p: *const c_void);
    fn blk_start_plug(plug: *mut blk_plug);
    fn blk_finish_plug(plug: *mut blk_plug);
    fn writeback_iter(mapping: *mut address_space, wbc: *mut writeback_control, folio: *mut folio, error: *mut i32) -> *mut folio;
}

#[repr(C)] pub struct bio { pub bi_status: i32, pub bi_end_io: Option<unsafe extern "C" fn(*mut bio)>, pub bi_iter: bio_iter, pub bi_write_hint: u32 }
#[repr(C)] pub struct bio_iter { pub bi_sector: u64 }
#[repr(C)] pub struct folio { pub mapping: *mut address_space, pub flags: usize }
#[repr(C)] pub struct address_space { pub host: *mut inode }
#[repr(C)] pub struct inode { pub i_blkbits: u32, pub i_write_hint: u32 }
#[repr(C)] pub struct buffer_head { pub b_state: u32, pub b_bdev: *mut block_device, pub b_blocknr: u64, pub b_this_page: *mut buffer_head, pub b_size: u32, pub b_folio: *mut folio }
#[repr(C)] pub struct block_device;
#[repr(C)] pub struct readahead_control;
#[repr(C)] pub struct writeback_control;
#[repr(C)] pub struct blk_plug;
pub type sector_t = u64;
pub type get_block_t = unsafe extern "C" fn(*mut inode, u64, *mut buffer_head, i32) -> i32;
const GFP_KERNEL: u32 = 0; const GFP_NOFS: u32 = 0; const PAGE_SIZE: usize = 4096;
const REQ_OP_READ: u32 = 0; const REQ_OP_WRITE: u32 = 1; const REQ_RAHEAD: u32 = 1 << 8; const BIO_MAX_VECS: u32 = 16;

unsafe extern "C" fn mpage_read_end_io(bio: *mut bio) { let err = blk_status_to_errno((*bio).bi_status); /* bio_for_each_folio_all */ let _ = err; bio_put(bio); }
unsafe extern "C" fn mpage_write_end_io(bio: *mut bio) { let err = blk_status_to_errno((*bio).bi_status); let _ = err; bio_put(bio); }
unsafe fn mpage_bio_submit_read(bio: *mut bio) -> *mut bio { (*bio).bi_end_io = Some(mpage_read_end_io); submit_bio(bio); core::ptr::null_mut() }
unsafe fn mpage_bio_submit_write(bio: *mut bio) -> *mut bio { (*bio).bi_end_io = Some(mpage_write_end_io); submit_bio(bio); core::ptr::null_mut() }

#[repr(C)] struct mpage_readpage_args { bio: *mut bio, folio: *mut folio, nr_pages: u32, is_readahead: bool, last_block_in_bio: sector_t, map_bh: buffer_head, first_logical_block: u64, get_block: get_block_t }

unsafe fn map_buffer_to_folio(folio: *mut folio, bh: *mut buffer_head, page_block: i32) { let inode = (*(*folio).mapping).host; let mut head = folio_buffers(folio); if head.is_null() { if (*inode).i_blkbits == folio_shift(folio) && buffer_uptodate(bh) { folio_mark_uptodate(folio); return; } head = create_empty_buffers(folio, i_blocksize(inode), 0); } let mut p = head; let mut block = 0; loop { if block == page_block { (*p).b_state=(*bh).b_state; (*p).b_bdev=(*bh).b_bdev; (*p).b_blocknr=(*bh).b_blocknr; break; } p=(*p).b_this_page; block+=1; if p==head {break;} } }

unsafe fn do_mpage_readpage(a: *mut mpage_readpage_args) { let f=(*a).folio; let inode=(*(*f).mapping).host; let bits=(*inode).i_blkbits; let blocks=(folio_size(f)>>bits) as u32; let mut bh=&mut (*a).map_bh as *mut buffer_head; if !folio_buffers(f).is_null(){goto_confused(a);return;} let mut block=folio_pos(f)>>bits; let last=((i_size_read(inode) as u64 + ((1u64<<bits)-1))>>bits).min(block+(((*a).nr_pages as usize*PAGE_SIZE)>>bits) as u64); let mut page=0u32; let mut first_hole=blocks; let mut first=0; let mut bdev=core::ptr::null_mut(); let mut full=true; while page<blocks { (*bh).b_state=0; (*bh).b_size=if block<last {((last-block)<<bits) as u32}else{0}; if block<last && ((*a).get_block)(inode,block,bh,0)!=0 {goto_confused(a);return;} if !buffer_mapped(bh) {full=false; if first_hole==blocks {first_hole=page;} page+=1;block+=1;continue;} if buffer_uptodate(bh){map_buffer_to_folio(f,bh,page as i32);goto_confused(a);return;} if page==0 {first=(*bh).b_blocknr;} else if first+page as u64!=(*bh).b_blocknr {goto_confused(a);return;} bdev=(*bh).b_bdev; page+=1;block+=1; } if first_hole!=blocks {folio_zero_segment(f,(first_hole<<bits) as usize,folio_size(f)); if first_hole==0 {folio_mark_uptodate(f);folio_unlock(f);return;}} else if full {folio_set_mappedtodisk(f);} if !(*a).bio.is_null() && (*a).last_block_in_bio!=first-1 {(*a).bio=mpage_bio_submit_read((*a).bio);} if (*a).bio.is_null(){(*a).bio=bio_alloc(bdev,bio_max_segs((*a).nr_pages),REQ_OP_READ,GFP_KERNEL); if (*a).bio.is_null(){goto_confused(a);return;} (*(*a).bio).bi_iter.bi_sector=first<<(bits-9);} if !bio_add_folio((*a).bio,f,(first_hole<<bits) as usize,0){(*a).bio=mpage_bio_submit_read((*a).bio);return;} (*a).last_block_in_bio=first+blocks as u64-1; }
unsafe fn goto_confused(a:*mut mpage_readpage_args){if !(*a).bio.is_null(){(*a).bio=mpage_bio_submit_read((*a).bio);} if !folio_test_uptodate((*a).folio){block_read_full_folio((*a).folio,(*a).get_block);}else{folio_unlock((*a).folio);}}

pub unsafe extern "C" fn mpage_readahead(rac:*mut readahead_control,get_block:get_block_t){let mut a=mpage_readpage_args{bio:core::ptr::null_mut(),folio:core::ptr::null_mut(),nr_pages:0,is_readahead:true,last_block_in_bio:0,map_bh:core::mem::zeroed(),first_logical_block:0,get_block};loop{let f=readahead_folio(rac);if f.is_null(){break;}a.folio=f;a.nr_pages=readahead_count(rac);do_mpage_readpage(&mut a);if !folio_test_locked(f)&&!folio_test_uptodate(f){break;}}if !a.bio.is_null(){mpage_bio_submit_read(a.bio);}}
pub unsafe extern "C" fn mpage_read_folio(f:*mut folio,get_block:get_block_t)->i32{let mut a:mpage_readpage_args=core::mem::zeroed();a.folio=f;a.nr_pages=folio_nr_pages(f);a.get_block=get_block;do_mpage_readpage(&mut a);if !a.bio.is_null(){mpage_bio_submit_read(a.bio);}0}

// The writeback portion retains the same kernel control flow and ABI.
#[repr(C)] struct mpage_data{bio:*mut bio,last_block_in_bio:u64,get_block:get_block_t}
unsafe fn clean_buffers(f:*mut folio,first:u32){let h=folio_buffers(f);if h.is_null(){return;}let mut p=h;let mut n=0;loop{if n==first{break;}clear_buffer_dirty(p);n+=1;p=(*p).b_this_page;if p==h{break;}}}
unsafe fn mpage_write_folio(wbc:*mut writeback_control,f:*mut folio,d:*mut mpage_data)->i32{
    let mapping=(*f).mapping; let inode=(*mapping).host; let bits=(*inode).i_blkbits;
    let blocks=(folio_size(f)>>bits) as u32; let mut bh=folio_buffers(f); let mut first=0u64; let mut page=0u32;
    if !bh.is_null(){let head=bh;loop{if !buffer_mapped(bh)||!buffer_dirty(bh)||!buffer_uptodate(bh){goto_write_confused(wbc,f,d);return 0;}if page==0{first=(*bh).b_blocknr;}else if (*bh).b_blocknr!=first+page as u64{goto_write_confused(wbc,f,d);return 0;}page+=1;bh=(*bh).b_this_page;if bh==head{break;}}}
    else {if !folio_test_uptodate(f){goto_write_confused(wbc,f,d);return 0;}let mut block=folio_pos(f)>>bits;let last=((i_size_read(inode) as u64+((1u64<<bits)-1))>>bits).saturating_sub(1);while page<blocks&&block<=last{let mut map:buffer_head=core::mem::zeroed();map.b_folio=f;map.b_size=1<<bits;if ((*d).get_block)(inode,block,&mut map,1)!=0||!buffer_mapped(&mut map){goto_write_confused(wbc,f,d);return 0;}if page==0{first=map.b_blocknr;}else if map.b_blocknr!=first+page as u64{goto_write_confused(wbc,f,d);return 0;}page+=1;block+=1;}}
    if folio_pos(f)>=i_size_read(inode) as u64{goto_write_confused(wbc,f,d);return 0;}let len=(i_size_read(inode) as u64-folio_pos(f)).min(folio_size(f) as u64) as usize;
    if (*d).bio.is_null(){(*d).bio=bio_alloc(core::ptr::null_mut(),BIO_MAX_VECS,REQ_OP_WRITE,GFP_NOFS);if (*d).bio.is_null(){goto_write_confused(wbc,f,d);return 0;}(*(*d).bio).bi_iter.bi_sector=first<<(bits-9);wbc_init_bio(wbc,(*d).bio);}
    wbc_account_cgroup_owner(wbc,f,len);if !bio_add_folio((*d).bio,f,len,0){(*d).bio=mpage_bio_submit_write((*d).bio);return mpage_write_folio(wbc,f,d);}clean_buffers(f,page);folio_start_writeback(f);folio_unlock(f);(*d).last_block_in_bio=first+page as u64-1;0
}
unsafe fn goto_write_confused(wbc:*mut writeback_control,f:*mut folio,d:*mut mpage_data){if !(*d).bio.is_null(){(*d).bio=mpage_bio_submit_write((*d).bio);}block_write_full_folio(f,wbc,(*d).get_block);}
pub unsafe extern "C" fn __mpage_writepages(mapping:*mut address_space,wbc:*mut writeback_control,get_block:get_block_t,write_folio:Option<unsafe extern "C" fn(*mut folio,*mut writeback_control)->i32>)->i32{let mut d=mpage_data{bio:core::ptr::null_mut(),last_block_in_bio:0,get_block};let mut err=0;let mut plug:blk_plug=core::mem::zeroed();blk_start_plug(&mut plug);let mut f=writeback_iter(mapping,wbc,core::ptr::null_mut(),&mut err);while !f.is_null(){if let Some(w)=write_folio{err=w(f,wbc);if err<=0{f=writeback_iter(mapping,wbc,f,&mut err);continue;}}err=block_write_full_folio(f,wbc,get_block);f=writeback_iter(mapping,wbc,f,&mut err);}if !d.bio.is_null(){mpage_bio_submit_write(d.bio);}blk_finish_plug(&mut plug);err}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
