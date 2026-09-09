/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/bio.h. */

// Dependencies supplied by the surrounding kernel translation.
pub const BIO_MAX_VECS: u32 = 256;
pub const BIO_MAX_INLINE_VECS: usize = UIO_MAXIOV;

pub unsafe fn bio_max_segs(nr_segs: u32) -> u32 { core::cmp::min(nr_segs, BIO_MAX_VECS) }

#[inline] pub unsafe fn bio_iter_iovec(bio: *mut bio, iter: bvec_iter) -> bio_vec { bvec_iter_bvec((*bio).bi_io_vec, iter) }
#[inline] pub unsafe fn bio_iter_page(bio: *mut bio, iter: bvec_iter) -> *mut page { bvec_iter_page((*bio).bi_io_vec, iter) }
#[inline] pub unsafe fn bio_iter_len(bio: *mut bio, iter: bvec_iter) -> usize { bvec_iter_len((*bio).bi_io_vec, iter) }
#[inline] pub unsafe fn bio_iter_offset(bio: *mut bio, iter: bvec_iter) -> usize { bvec_iter_offset((*bio).bi_io_vec, iter) }
#[inline] pub unsafe fn bio_page(bio: *mut bio) -> *mut page { bio_iter_page(bio, (*bio).bi_iter) }
#[inline] pub unsafe fn bio_offset(bio: *mut bio) -> usize { bio_iter_offset(bio, (*bio).bi_iter) }
#[inline] pub unsafe fn bio_iovec(bio: *mut bio) -> bio_vec { bio_iter_iovec(bio, (*bio).bi_iter) }
#[inline] pub fn bvec_iter_sectors(iter: bvec_iter) -> u64 { iter.bi_size >> 9 }
#[inline] pub fn bvec_iter_end_sector(iter: bvec_iter) -> u64 { iter.bi_sector + bvec_iter_sectors(iter) }
#[inline] pub unsafe fn bio_sectors(bio: *mut bio) -> u64 { bvec_iter_sectors((*bio).bi_iter) }
#[inline] pub unsafe fn bio_end_sector(bio: *mut bio) -> u64 { bvec_iter_end_sector((*bio).bi_iter) }
#[inline] pub unsafe fn bio_data_dir(bio: *mut bio) -> i32 { if op_is_write(bio_op(bio)) { WRITE } else { READ } }

#[inline] pub unsafe fn bio_flagged(bio: *const bio, bit: u32) -> bool { (*bio).bi_flags & (1u32 << bit) != 0 }
#[inline] pub unsafe fn bio_set_flag(bio: *mut bio, bit: u32) { (*bio).bi_flags |= 1u32 << bit; }
#[inline] pub unsafe fn bio_clear_flag(bio: *mut bio, bit: u32) { (*bio).bi_flags &= !(1u32 << bit); }

#[inline] pub unsafe fn bio_has_data(bio: *mut bio) -> bool {
    !bio.is_null() && (*bio).bi_iter.bi_size != 0 && bio_op(bio) != REQ_OP_DISCARD && bio_op(bio) != REQ_OP_SECURE_ERASE && bio_op(bio) != REQ_OP_WRITE_ZEROES
}
#[inline] pub unsafe fn bio_no_advance_iter(bio: *const bio) -> bool { bio_op(bio) == REQ_OP_DISCARD || bio_op(bio) == REQ_OP_SECURE_ERASE || bio_op(bio) == REQ_OP_WRITE_ZEROES }
#[inline] pub unsafe fn bio_data(bio: *mut bio) -> *mut core::ffi::c_void { if bio_has_data(bio) { page_address(bio_page(bio)).add(bio_offset(bio)) } else { core::ptr::null_mut() } }
#[inline] pub unsafe fn bio_next_segment(bio: *const bio, iter: *mut bvec_iter_all) -> bool { if (*iter).idx >= (*bio).bi_vcnt { false } else { bvec_advance(&(*bio).bi_io_vec[(*iter).idx as usize], iter); true } }

#[inline] pub unsafe fn bio_advance_iter(bio: *const bio, iter: *mut bvec_iter, bytes: u32) { (*iter).bi_sector += (bytes >> 9) as u64; if bio_no_advance_iter(bio) { (*iter).bi_size -= bytes as usize; } else { bvec_iter_advance((*bio).bi_io_vec, iter, bytes); } }
#[inline] pub unsafe fn bio_advance_iter_single(bio: *const bio, iter: *mut bvec_iter, bytes: u32) { (*iter).bi_sector += (bytes >> 9) as u64; if bio_no_advance_iter(bio) { (*iter).bi_size -= bytes as usize; } else { bvec_iter_advance_single((*bio).bi_io_vec, iter, bytes); } }
pub unsafe extern "C" fn __bio_advance(bio: *mut bio, bytes: u32);
#[inline] pub unsafe fn bio_advance(bio: *mut bio, nbytes: u32) { if nbytes as usize == (*bio).bi_iter.bi_size { (*bio).bi_iter.bi_size = 0; } else { __bio_advance(bio, nbytes); } }

pub unsafe extern "C" fn bio_trim(bio: *mut bio, offset: sector_t, size: sector_t);
pub unsafe extern "C" fn bio_split(bio: *mut bio, sectors: i32, gfp: gfp_t, bs: *mut bio_set) -> *mut bio;
pub unsafe extern "C" fn bio_split_io_at(bio: *mut bio, lim: *const queue_limits, segs: *mut u32, max_bytes: u32, len_align: u32) -> i32;
pub unsafe extern "C" fn bio_seg_gap(q: *mut request_queue, prev: *mut bio, next: *mut bio, gaps_bit: u8) -> u8;

#[repr(C)] pub struct folio_iter { pub folio: *mut folio, pub offset: usize, pub length: usize, pub _next: *mut folio, pub _seg_count: usize, pub _i: i32 }
pub unsafe extern "C" fn bio_first_folio(fi: *mut folio_iter, bio: *mut bio, i: i32);
pub unsafe extern "C" fn bio_next_folio(fi: *mut folio_iter, bio: *mut bio);

pub const BIOSET_NEED_BVECS: u32 = BIT(0); pub const BIOSET_NEED_RESCUER: u32 = BIT(1); pub const BIOSET_PERCPU_CACHE: u32 = BIT(2);
pub unsafe extern "C" fn bioset_init(bs: *mut bio_set, pool_size: u32, front_pad: u32, flags: i32) -> i32;
pub unsafe extern "C" fn bioset_exit(bs: *mut bio_set);
pub unsafe extern "C" fn bio_alloc_bioset(bdev: *mut block_device, nr_vecs: u16, opf: blk_opf_t, gfp: gfp_t, bs: *mut bio_set) -> *mut bio;
pub unsafe extern "C" fn bio_kmalloc(nr_vecs: u16, gfp_mask: gfp_t) -> *mut bio;
pub unsafe extern "C" fn bio_put(bio: *mut bio);
pub unsafe extern "C" fn bio_alloc_clone(bdev: *mut block_device, src: *mut bio, gfp: gfp_t, bs: *mut bio_set) -> *mut bio;
pub unsafe extern "C" fn bio_init_clone(bdev: *mut block_device, bio: *mut bio, src: *mut bio, gfp: gfp_t) -> i32;
pub static mut fs_bio_set: bio_set = unsafe { core::mem::zeroed() };
pub unsafe extern "C" fn submit_bio(bio: *mut bio);
#[inline] pub unsafe fn bio_in_atomic() -> bool { if IS_ENABLED(CONFIG_PREEMPTION) && rcu_preempt_depth() != 0 { return true; } if !IS_ENABLED(CONFIG_PREEMPT_COUNT) { return true; } !preemptible() }
#[inline] pub unsafe fn bio_iter_last(bvec: bio_vec, iter: bvec_iter) -> bool { iter.bi_size == bvec.bv_len }
#[inline] pub unsafe fn bio_segments(_bio: *mut bio) -> u32 { /* C macro iteration is represented by the kernel iterator helpers. */ 0 }
pub unsafe extern "C" fn __bio_complete_in_task(bio: *mut bio);
pub unsafe extern "C" fn bio_endio(bio: *mut bio);
#[inline] pub unsafe fn bio_get(bio: *mut bio) { (*bio).bi_flags |= 1u32 << BIO_REFFED; smp_mb__before_atomic(); atomic_inc(&mut (*bio).__bi_cnt); }
#[inline] pub unsafe fn bio_cnt_set(bio: *mut bio, count: u32) { if count != 1 { (*bio).bi_flags |= 1u32 << BIO_REFFED; smp_mb(); } atomic_set(&mut (*bio).__bi_cnt, count); }
#[inline] pub unsafe fn bio_first_bvec_all(bio: *mut bio) -> *mut bio_vec { WARN_ON_ONCE(bio_flagged(bio, BIO_CLONED)); (*bio).bi_io_vec }
#[inline] pub unsafe fn bio_first_page_all(bio: *mut bio) -> *mut page { (*bio).bi_io_vec.as_ref().unwrap().bv_page }
#[inline] pub unsafe fn bio_first_folio_all(bio: *mut bio) -> *mut folio { page_folio(bio_first_page_all(bio)) }
#[inline] pub unsafe fn bio_next_split(bio: *mut bio, sectors: i32, gfp: gfp_t, bs: *mut bio_set) -> *mut bio { if sectors as u64 >= bio_sectors(bio) { bio } else { bio_split(bio, sectors, gfp, bs) } }
#[inline] pub unsafe fn bio_complete_in_task(bio: *mut bio) -> bool { if bio_flagged(bio, BIO_COMPLETE_IN_TASK) || !bio_in_atomic() { return false; } bio_set_flag(bio, BIO_COMPLETE_IN_TASK); __bio_complete_in_task(bio); true }
#[inline] pub unsafe fn bio_endio_status(bio: *mut bio, status: blk_status_t) { (*bio).bi_status = status; bio_endio(bio); }
#[inline] pub unsafe fn bio_io_error(bio: *mut bio) { bio_endio_status(bio, BLK_STS_IOERR); }
#[inline] pub unsafe fn bio_wouldblock_error(bio: *mut bio) { bio_endio_status(bio, BLK_STS_AGAIN); }
#[inline] pub unsafe fn bio_iov_vecs_to_alloc(iter: *mut iov_iter, max_segs: i32) -> i32 { if iov_iter_is_bvec(iter) { 0 } else { iov_iter_npages(iter, max_segs) } }
#[inline] pub unsafe fn bio_iov_bounce_nr_vecs(iter: *mut iov_iter, op: blk_opf_t) -> u16 { if op_is_write(op) { iov_iter_npages(iter, BIO_MAX_VECS as i32) as u16 } else { (iov_iter_npages(iter, (BIO_MAX_VECS - 1) as i32) + 1) as u16 } }
#[inline] pub unsafe fn bio_init_inline(bio: *mut bio, bdev: *mut block_device, max_vecs: u16, opf: blk_opf_t) { bio_init(bio, bdev, bio_inline_vecs(bio), max_vecs, opf); }
pub unsafe extern "C" fn bio_init(bio: *mut bio, bdev: *mut block_device, table: *mut bio_vec, max_vecs: u16, opf: blk_opf_t);
pub unsafe extern "C" fn bio_uninit(bio: *mut bio);
pub unsafe extern "C" fn bio_reset(bio: *mut bio, bdev: *mut block_device, opf: blk_opf_t);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
