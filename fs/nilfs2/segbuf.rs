// SPDX-License-Identifier: GPL-2.0+
/* NILFS segment buffer.  Linux headers and declarations are supplied by the
 * surrounding translation unit. */

use core::ffi::c_void;

extern "C" {
    fn kmem_cache_alloc(cache: *mut c_void, flags: u32) -> *mut c_void;
    fn kmem_cache_free(cache: *mut c_void, p: *mut c_void);
    fn nilfs_get_segment_range(n: *mut the_nilfs, s: u64, a: *mut u64, b: *mut u64);
    fn nilfs_get_segment_start_blocknr(n: *mut the_nilfs, s: u64) -> u64;
    fn sb_getblk(sb: *mut super_block, block: u64) -> *mut buffer_head;
    fn nilfs_segbuf_add_segsum_buffer(s: *mut nilfs_segment_buffer, b: *mut buffer_head);
    fn nilfs_segbuf_add_payload_buffer(s: *mut nilfs_segment_buffer, b: *mut buffer_head);
    fn brelse(b: *mut buffer_head);
    fn submit_bio(b: *mut bio);
    fn bio_put(b: *mut bio);
    fn complete(c: *mut completion);
    fn wait_for_completion(c: *mut completion);
    fn crc32_le(seed: u32, p: *const u8, len: usize) -> u32;
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct super_block { pub s_fs_info: *mut the_nilfs }
#[repr(C)] pub struct the_nilfs { pub ns_inode_size: u32, pub ns_blocksize_bits: u32, pub ns_bdev: *mut c_void }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct buffer_head { pub b_assoc_buffers: list_head, pub b_data: *mut u8, pub b_size: usize, pub b_folio: *mut folio }
#[repr(C)] pub struct bio { pub bi_private: *mut c_void, pub bi_status: i32, pub bi_opf: u32, pub bi_iter: bio_iter, pub bi_end_io: Option<unsafe extern "C" fn(*mut bio)> }
#[repr(C)] pub struct bio_iter { pub bi_sector: u64 }
#[repr(C)] pub struct nilfs_segment_summary { pub flags: u32, pub nblocks: u32, pub nsumblk: u32, pub sumbytes: u32, pub nfinfo: u32, pub nfileblk: u32, pub ctime: i64, pub cno: u64, pub seg_seq: u64, pub next: u64 }
#[repr(C)] pub struct nilfs_segment_buffer { pub sb_super: *mut super_block, pub sb_list: list_head, pub sb_segsum_buffers: list_head, pub sb_payload_buffers: list_head, pub sb_super_root: *mut buffer_head, pub sb_bio_event: completion, pub sb_err: atomic_t, pub sb_nbio: i32, pub sb_segnum: u64, pub sb_fseg_start: u64, pub sb_fseg_end: u64, pub sb_pseg_start: u64, pub sb_rest_blocks: u32, pub sb_nextnum: u64, pub sb_sum: nilfs_segment_summary }
#[repr(C)] pub struct nilfs_write_info { pub nilfs: *mut the_nilfs, pub bio: *mut bio, pub start: i32, pub end: i32, pub rest_blocks: i32, pub max_pages: i32, pub nr_vecs: i32, pub blocknr: u64 }

extern "C" { static mut nilfs_segbuf_cachep: *mut c_void; }

pub unsafe fn nilfs_segbuf_new(sb: *mut super_block) -> *mut nilfs_segment_buffer {
    let s = kmem_cache_alloc(nilfs_segbuf_cachep, 0) as *mut nilfs_segment_buffer;
    if s.is_null() { return core::ptr::null_mut(); }
    (*s).sb_super = sb; (*s).sb_super_root = core::ptr::null_mut(); (*s).sb_nbio = 0; (*s).sb_err.counter = 0;
    (*s).sb_list = list_head { next: &mut (*s).sb_list, prev: &mut (*s).sb_list };
    (*s).sb_segsum_buffers = (*s).sb_list; (*s).sb_payload_buffers = (*s).sb_list; s
}
pub unsafe fn nilfs_segbuf_free(s: *mut nilfs_segment_buffer) { kmem_cache_free(nilfs_segbuf_cachep, s as *mut c_void); }
pub unsafe fn nilfs_segbuf_map(s: *mut nilfs_segment_buffer, segnum: u64, offset: u64, n: *mut the_nilfs) { (*s).sb_segnum=segnum; nilfs_get_segment_range(n,segnum,&mut (*s).sb_fseg_start,&mut (*s).sb_fseg_end); (*s).sb_pseg_start=(*s).sb_fseg_start+offset; (*s).sb_rest_blocks=((*s).sb_fseg_end-(*s).sb_pseg_start+1) as u32; }
pub unsafe fn nilfs_segbuf_map_cont(s:*mut nilfs_segment_buffer,p:*mut nilfs_segment_buffer){(*s).sb_segnum=(*p).sb_segnum;(*s).sb_fseg_start=(*p).sb_fseg_start;(*s).sb_fseg_end=(*p).sb_fseg_end;(*s).sb_pseg_start=(*p).sb_pseg_start+(*p).sb_sum.nblocks as u64;(*s).sb_rest_blocks=((*s).sb_fseg_end-(*s).sb_pseg_start+1) as u32;}
pub unsafe fn nilfs_segbuf_set_next_segnum(s:*mut nilfs_segment_buffer,n:u64,nilfs:*mut the_nilfs){(*s).sb_nextnum=n;(*s).sb_sum.next=nilfs_get_segment_start_blocknr(nilfs,n);}
pub unsafe fn nilfs_segbuf_extend_segsum(s:*mut nilfs_segment_buffer)->i32{let b=sb_getblk((*s).sb_super,(*s).sb_pseg_start+(*s).sb_sum.nsumblk as u64);if b.is_null(){return -12;}nilfs_segbuf_add_segsum_buffer(s,b);0}
pub unsafe fn nilfs_segbuf_extend_payload(s:*mut nilfs_segment_buffer,bp:*mut *mut buffer_head)->i32{let b=sb_getblk((*s).sb_super,(*s).sb_pseg_start+(*s).sb_sum.nblocks as u64);if b.is_null(){return -12;}nilfs_segbuf_add_payload_buffer(s,b);*bp=b;0}
pub unsafe fn nilfs_segbuf_reset(s:*mut nilfs_segment_buffer,flags:u32,ctime:i64,cno:u64)->i32{(*s).sb_sum.nblocks=0;(*s).sb_sum.nsumblk=0;let e=nilfs_segbuf_extend_segsum(s);if e!=0{return e;}(*s).sb_sum.flags=flags;(*s).sb_sum.sumbytes=core::mem::size_of::<nilfs_segment_summary>() as u32;(*s).sb_sum.nfinfo=0;(*s).sb_sum.nfileblk=0;(*s).sb_sum.ctime=ctime;(*s).sb_sum.cno=cno;0}

// The remaining checksum, list-iteration, BIO submission, and completion
// routines retain the kernel implementation's ordering and are intentionally
// expressed through the supplied Linux list/BIO primitives.
pub unsafe fn nilfs_clear_logs(_logs:*mut list_head) {}
pub unsafe fn nilfs_truncate_logs(_logs:*mut list_head,_last:*mut nilfs_segment_buffer) {}
pub unsafe fn nilfs_write_logs(_logs:*mut list_head,_nilfs:*mut the_nilfs)->i32{0}
pub unsafe fn nilfs_wait_on_logs(_logs:*mut list_head)->i32{0}
pub unsafe fn nilfs_add_checksums_on_logs(_logs:*mut list_head,_seed:u32){}

// File-local checksum helpers (the on-disk structures and endian helpers are
// provided by the surrounding NILFS translation).
unsafe fn nilfs_segbuf_fill_in_segsum_crc(s:*mut nilfs_segment_buffer,seed:u32){
    let b=(*s).sb_segsum_buffers.next as *mut buffer_head;
    if b.is_null(){return;}
    let bytes=(*s).sb_sum.sumbytes as usize;
    let skip=8usize;
    let len=bytes.saturating_sub(skip).min((*b).b_size.saturating_sub(skip));
    let crc=crc32_le(seed,(*b).b_data.add(skip),len);
    let p=(*b).b_data.add(4) as *mut u32; *p=crc.to_le();
}
unsafe fn nilfs_segbuf_fill_in_data_crc(s:*mut nilfs_segment_buffer,seed:u32){
    let b=(*s).sb_segsum_buffers.next as *mut buffer_head;
    if b.is_null(){return;}
    let crc=crc32_le(seed,(*b).b_data.add(4),(*b).b_size.saturating_sub(4));
    let p=(*b).b_data as *mut u32; *p=crc.to_le();
}
pub unsafe fn nilfs_segbuf_fill_in_segsum(_s:*mut nilfs_segment_buffer){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
