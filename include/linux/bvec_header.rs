/* SPDX-License-Identifier: GPL-2.0 */
/* bvec iterator */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/highmem.h, linux/bug.h, linux/errno.h, linux/limits.h,
// linux/minmax.h, and linux/types.h.

#[repr(C)]
pub struct bio_vec {
    pub bv_page: *mut page,
    pub bv_len: ::core::ffi::c_uint,
    pub bv_offset: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct bvec_iter {
    pub bi_sector: sector_t,
    pub bi_size: ::core::ffi::c_uint,
    pub bi_idx: ::core::ffi::c_uint,
    pub bi_offset: ::core::ffi::c_uint,
}

#[repr(C, packed(4))]
pub struct bvec_iter_all {
    pub bv: bio_vec,
    pub idx: ::core::ffi::c_int,
    pub done: ::core::ffi::c_uint,
}

pub unsafe fn bvec_set_page(
    bv: *mut bio_vec,
    page_: *mut page,
    len: ::core::ffi::c_uint,
    offset: ::core::ffi::c_uint,
) {
    (*bv).bv_page = page_;
    (*bv).bv_len = len;
    (*bv).bv_offset = offset;
}

pub unsafe fn bvec_set_folio(
    bv: *mut bio_vec,
    folio: *mut folio,
    len: usize,
    offset: usize,
) {
    let nr: ::core::ffi::c_ulong = offset / PAGE_SIZE;
    WARN_ON_ONCE(len > UINT_MAX);
    bvec_set_page(
        bv,
        folio_page(folio, nr),
        len as ::core::ffi::c_uint,
        (offset % PAGE_SIZE) as ::core::ffi::c_uint,
    );
}

pub unsafe fn bvec_set_virt(
    bv: *mut bio_vec,
    vaddr: *mut ::core::ffi::c_void,
    len: ::core::ffi::c_uint,
) {
    bvec_set_page(bv, virt_to_page(vaddr), len, offset_in_page(vaddr));
}

pub unsafe fn bvec_folio(bv: *const bio_vec) -> *mut folio {
    page_folio((*bv).bv_page)
}

pub unsafe fn __bvec_iter_bvec(
    bvecs: *const bio_vec,
    iter: bvec_iter,
) -> *const bio_vec {
    bvecs.add(iter.bi_idx as usize)
}

pub unsafe fn mp_bvec_iter_page(bvecs: *const bio_vec, iter: bvec_iter) -> *mut page {
    (*__bvec_iter_bvec(bvecs, iter)).bv_page
}

pub unsafe fn mp_bvec_iter_len(bvecs: *const bio_vec, iter: bvec_iter) -> ::core::ffi::c_uint {
    min((*__bvec_iter_bvec(bvecs, iter)).bv_len - iter.bi_offset, iter.bi_size)
}

pub unsafe fn mp_bvec_iter_offset(bvecs: *const bio_vec, iter: bvec_iter) -> ::core::ffi::c_uint {
    (*__bvec_iter_bvec(bvecs, iter)).bv_offset + iter.bi_offset
}

pub unsafe fn mp_bvec_iter_page_idx(bvecs: *const bio_vec, iter: bvec_iter) -> ::core::ffi::c_uint {
    mp_bvec_iter_offset(bvecs, iter) / PAGE_SIZE as ::core::ffi::c_uint
}

pub unsafe fn mp_bvec_iter_bvec(bvecs: *const bio_vec, iter: bvec_iter) -> bio_vec {
    bio_vec {
        bv_page: mp_bvec_iter_page(bvecs, iter),
        bv_len: mp_bvec_iter_len(bvecs, iter),
        bv_offset: mp_bvec_iter_offset(bvecs, iter),
    }
}

pub unsafe fn bvec_iter_offset(bvecs: *const bio_vec, iter: bvec_iter) -> ::core::ffi::c_uint {
    mp_bvec_iter_offset(bvecs, iter) % PAGE_SIZE as ::core::ffi::c_uint
}

pub unsafe fn bvec_iter_len(bvecs: *const bio_vec, iter: bvec_iter) -> ::core::ffi::c_uint {
    min(mp_bvec_iter_len(bvecs, iter), PAGE_SIZE as ::core::ffi::c_uint - bvec_iter_offset(bvecs, iter))
}

pub unsafe fn bvec_iter_page(bvecs: *const bio_vec, iter: bvec_iter) -> *mut page {
    mp_bvec_iter_page(bvecs, iter).add(mp_bvec_iter_page_idx(bvecs, iter) as usize)
}

pub unsafe fn bvec_iter_bvec(bvecs: *const bio_vec, iter: bvec_iter) -> bio_vec {
    bio_vec {
        bv_page: bvec_iter_page(bvecs, iter),
        bv_len: bvec_iter_len(bvecs, iter),
        bv_offset: bvec_iter_offset(bvecs, iter),
    }
}

pub unsafe fn bvec_iter_advance(
    bv: *const bio_vec,
    iter: *mut bvec_iter,
    mut bytes: ::core::ffi::c_uint,
) -> bool {
    let mut idx = (*iter).bi_idx;
    if WARN_ONCE(bytes > (*iter).bi_size, "Attempted to advance past end of bvec iter\n") {
        (*iter).bi_size = 0;
        return false;
    }
    (*iter).bi_size -= bytes;
    bytes += (*iter).bi_offset;
    while bytes != 0 && bytes >= (*bv.add(idx as usize)).bv_len {
        bytes -= (*bv.add(idx as usize)).bv_len;
        idx += 1;
    }
    (*iter).bi_idx = idx;
    (*iter).bi_offset = bytes;
    true
}

pub unsafe fn bvec_iter_advance_single(
    bv: *const bio_vec,
    iter: *mut bvec_iter,
    bytes: ::core::ffi::c_uint,
) {
    let mut done = (*iter).bi_offset + bytes;
    if done == (*bv.add((*iter).bi_idx as usize)).bv_len {
        done = 0;
        (*iter).bi_idx += 1;
    }
    (*iter).bi_offset = done;
    (*iter).bi_size -= bytes;
}

#[macro_export]
macro_rules! for_each_bvec {
    ($bvl:ident, $bio_vec:expr, $iter:ident, $start:expr) => {
        for $iter = $start; $iter.bi_size != 0; {
            $bvl = unsafe { $crate::bvec_iter_bvec($bio_vec, $iter) };
            unsafe { $crate::bvec_iter_advance_single($bio_vec, &mut $iter, $bvl.bv_len) };
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
