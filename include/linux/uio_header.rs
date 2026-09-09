/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Berkeley style UIO structures - Alan Cox 1994. */

/* Dependencies supplied by the surrounding kernel translation. */

pub type iov_iter_extraction_t = u32;

#[repr(C)]
pub struct kvec {
    pub iov_base: *mut core::ffi::c_void,
    pub iov_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum iter_type {
    ITER_UBUF,
    ITER_IOVEC,
    ITER_BVEC,
    ITER_KVEC,
    ITER_FOLIOQ,
    ITER_XARRAY,
    ITER_DISCARD,
}

pub const ITER_SOURCE: u32 = 1; // == WRITE
pub const ITER_DEST: u32 = 0; // == READ

#[repr(C)]
pub struct iov_iter_state {
    pub iov_offset: usize,
    pub count: usize,
    pub nr_segs: usize,
}

#[repr(C)]
pub union iov_iter_ptrs {
    pub __iov: *const iovec,
    pub kvec: *const kvec,
    pub bvec: *const bio_vec,
    pub folioq: *const folio_queue,
    pub xarray: *mut xarray,
    pub ubuf: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct iov_iter_iovec {
    pub ptr: iov_iter_ptrs,
    pub count: usize,
}

#[repr(C)]
pub union iov_iter_primary {
    pub __ubuf_iovec: iovec,
    pub iovec: iov_iter_iovec,
}

#[repr(C)]
pub union iov_iter_tail {
    pub nr_segs: usize,
    pub folioq_slot: u8,
    pub xarray_start: loff_t,
}

#[repr(C)]
pub struct iov_iter {
    pub iter_type: u8,
    pub nofault: bool,
    pub data_source: bool,
    pub iov_offset: usize,
    pub primary: iov_iter_primary,
    pub tail: iov_iter_tail,
}

pub type uio_meta_flags_t = u16;

#[repr(C)]
pub struct uio_meta {
    pub flags: uio_meta_flags_t,
    pub app_tag: u16,
    pub seed: u64,
    pub iter: iov_iter,
}

#[inline]
pub unsafe fn iter_iov(iter: *const iov_iter) -> *const iovec {
    if (*iter).iter_type == ITER_UBUF as u8 {
        &(*iter).primary.__ubuf_iovec as *const iovec
    } else {
        (*iter).primary.iovec.ptr.__iov
    }
}

#[inline]
pub unsafe fn iter_iov_addr(iter: *const iov_iter) -> *mut core::ffi::c_void {
    ((*iter_iov(iter)).iov_base as *mut u8).add((*iter).iov_offset) as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn iter_iov_len(i: *const iov_iter) -> usize {
    if (*i).iter_type == ITER_UBUF as u8 { (*i).primary.iovec.count } else { (*iter_iov(i)).iov_len - (*i).iov_offset }
}

#[inline] pub unsafe fn iov_iter_type(i: *const iov_iter) -> iter_type { core::mem::transmute((*i).iter_type) }
#[inline] pub unsafe fn iov_iter_save_state(iter: *const iov_iter, state: *mut iov_iter_state) { (*state).iov_offset=(*iter).iov_offset; (*state).count=(*iter).primary.iovec.count; (*state).nr_segs=(*iter).tail.nr_segs; }
#[inline] pub unsafe fn iter_is_ubuf(i:*const iov_iter)->bool { (*i).iter_type==ITER_UBUF as u8 }
#[inline] pub unsafe fn iter_is_iovec(i:*const iov_iter)->bool { (*i).iter_type==ITER_IOVEC as u8 }
#[inline] pub unsafe fn iov_iter_is_kvec(i:*const iov_iter)->bool { (*i).iter_type==ITER_KVEC as u8 }
#[inline] pub unsafe fn iov_iter_is_bvec(i:*const iov_iter)->bool { (*i).iter_type==ITER_BVEC as u8 }
#[inline] pub unsafe fn iov_iter_is_discard(i:*const iov_iter)->bool { (*i).iter_type==ITER_DISCARD as u8 }
#[inline] pub unsafe fn iov_iter_is_folioq(i:*const iov_iter)->bool { (*i).iter_type==ITER_FOLIOQ as u8 }
#[inline] pub unsafe fn iov_iter_is_xarray(i:*const iov_iter)->bool { (*i).iter_type==ITER_XARRAY as u8 }
#[inline] pub unsafe fn user_backed_iter(i:*const iov_iter)->bool { iter_is_ubuf(i)||iter_is_iovec(i) }

pub unsafe fn iov_length(iov:*const iovec, nr_segs:usize)->usize { let mut ret=0; for seg in 0..nr_segs { ret += (*iov.add(seg)).iov_len; } ret }

extern "C" {
    pub fn iov_iter_advance(i:*mut iov_iter, bytes:usize); pub fn iov_iter_revert(i:*mut iov_iter, bytes:usize);
    pub fn iov_iter_count(i:*const iov_iter)->usize;
    pub fn _copy_to_iter(addr:*const core::ffi::c_void, bytes:usize, i:*mut iov_iter)->usize;
    pub fn _copy_from_iter(addr:*mut core::ffi::c_void, bytes:usize, i:*mut iov_iter)->usize;
    pub fn _copy_from_iter_nocache(addr:*mut core::ffi::c_void, bytes:usize, i:*mut iov_iter)->usize;
    pub fn iov_iter_npages(i:*const iov_iter, maxpages:i32)->i32;
    pub fn iov_iter_extract_pages(i:*mut iov_iter, pages:*mut *mut *mut page, maxsize:usize, maxpages:u32, flags:iov_iter_extraction_t, offset0:*mut usize)->isize;
    pub fn extract_iter_to_sg(iter:*mut iov_iter, len:usize, sgtable:*mut sg_table, sg_max:u32, flags:iov_iter_extraction_t)->isize;
}

pub const ITER_ALLOW_P2PDMA: iov_iter_extraction_t = 0x01;

#[inline] pub unsafe fn iov_iter_count_inline(i:*const iov_iter)->usize { (*i).primary.iovec.count }
#[inline] pub unsafe fn iov_iter_truncate(i:*mut iov_iter, count:u64) { if (*i).primary.iovec.count as u64 > count { (*i).primary.iovec.count=count as usize; } }
#[inline] pub unsafe fn iov_iter_reexpand(i:*mut iov_iter, count:usize) { (*i).primary.iovec.count=count; }
#[inline] pub unsafe fn iov_iter_extract_will_pin(iter:*const iov_iter)->bool { user_backed_iter(iter) }

/* The remaining declarations are external kernel interfaces. */
extern "C" {
    pub fn fault_in_iov_iter_readable(i:*const iov_iter, bytes:usize)->usize;
    pub fn fault_in_iov_iter_writeable(i:*const iov_iter, bytes:usize)->usize;
    pub fn iov_iter_single_seg_count(i:*const iov_iter)->usize;
    pub fn copy_page_to_iter(page:*mut page, offset:usize, bytes:usize, i:*mut iov_iter)->usize;
    pub fn copy_page_from_iter(page:*mut page, offset:usize, bytes:usize, i:*mut iov_iter)->usize;
    pub fn copy_folio_from_iter_atomic(folio:*mut folio, offset:usize, bytes:usize, i:*mut iov_iter)->usize;
    pub fn copy_page_to_iter_nofault(page:*mut page, offset:u32, bytes:usize, i:*mut iov_iter)->usize;
    pub fn iov_iter_init(i:*mut iov_iter, direction:u32, iov:*const iovec, nr_segs:usize, count:usize);
    pub fn iov_iter_kvec(i:*mut iov_iter, direction:u32, kvec:*const kvec, nr_segs:usize, count:usize);
    pub fn iov_iter_bvec(i:*mut iov_iter, direction:u32, bvec:*const bio_vec, nr_segs:usize, count:usize);
    pub fn iov_iter_discard(i:*mut iov_iter, direction:u32, count:usize);
    pub fn iov_iter_folio_queue(i:*mut iov_iter, direction:u32, folioq:*const folio_queue, first_slot:u32, offset:u32, count:usize);
    pub fn iov_iter_xarray(i:*mut iov_iter, direction:u32, xarray:*mut xarray, start:loff_t, count:usize);
    pub fn iov_iter_get_pages2(i:*mut iov_iter, pages:*mut *mut page, maxsize:usize, maxpages:u32, start:*mut usize)->isize;
    pub fn iov_iter_get_pages_alloc2(i:*mut iov_iter, pages:*mut *mut *mut page, maxsize:usize, start:*mut usize)->isize;
    pub fn iov_iter_restore(i:*mut iov_iter, state:*mut iov_iter_state);
    pub fn dup_iter(new:*mut iov_iter, old:*mut iov_iter, flags:gfp_t)->*const core::ffi::c_void;
    pub fn iovec_from_user(uvector:*const iovec, nr_segs:usize, fast_segs:usize, fast_iov:*mut iovec, compat:bool)->*mut iovec;
    pub fn import_iovec(typ:i32, uvec:*const iovec, nr_segs:u32, fast_segs:u32, iovp:*mut *mut iovec, i:*mut iov_iter)->isize;
    pub fn __import_iovec(typ:i32, uvec:*const iovec, nr_segs:u32, fast_segs:u32, iovp:*mut *mut iovec, i:*mut iov_iter, compat:bool)->isize;
    pub fn import_ubuf(typ:i32, buf:*mut core::ffi::c_void, len:usize, i:*mut iov_iter)->i32;
    pub fn iov_iter_zero(bytes:usize, i:*mut iov_iter)->usize;
    pub fn iov_iter_alignment(i:*const iov_iter)->usize; pub fn iov_iter_gap_alignment(i:*const iov_iter)->usize;
    pub fn iov_iter_npages_raw(i:*const iov_iter, maxpages:i32)->i32;
    pub fn iov_iter_extract_bvecs(iter:*mut iov_iter, bv:*mut bio_vec, max_size:usize, nr_vecs:*mut u16, max_vecs:u16, mem_align_mask:u32, flags:iov_iter_extraction_t)->isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
