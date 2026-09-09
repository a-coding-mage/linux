/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/scatterlist.h.  External kernel types and functions
 * are intentionally left as dependencies of this header translation. */

#[repr(C)]
pub struct scatterlist {
    pub page_link: ::std::ffi::c_ulong,
    pub offset: u32,
    pub length: u32,
    pub dma_address: dma_addr_t,
    #[cfg(feature = "CONFIG_NEED_SG_DMA_LENGTH")]
    pub dma_length: u32,
    #[cfg(feature = "CONFIG_NEED_SG_DMA_FLAGS")]
    pub dma_flags: u32,
}

pub type dma_addr_t = u64;

#[inline] pub unsafe fn sg_dma_address(sg: *mut scatterlist) -> dma_addr_t { (*sg).dma_address }
#[inline]
pub unsafe fn sg_dma_len(sg: *mut scatterlist) -> u32 {
    #[cfg(feature = "CONFIG_NEED_SG_DMA_LENGTH")] { (*sg).dma_length }
    #[cfg(not(feature = "CONFIG_NEED_SG_DMA_LENGTH"))] { (*sg).length }
}

#[repr(C)] pub struct sg_table { pub sgl: *mut scatterlist, pub nents: u32, pub orig_nents: u32 }
#[repr(C)] pub struct sg_append_table { pub sgt: sg_table, pub prv: *mut scatterlist, pub total_nents: u32 }

pub const SG_CHAIN: ::std::ffi::c_ulong = 0x01;
pub const SG_END: ::std::ffi::c_ulong = 0x02;
pub const SG_PAGE_LINK_MASK: ::std::ffi::c_ulong = SG_CHAIN | SG_END;

#[inline] pub unsafe fn __sg_flags(sg: *mut scatterlist) -> ::std::ffi::c_uint { ((*sg).page_link & SG_PAGE_LINK_MASK) as _ }
#[inline] pub unsafe fn sg_chain_ptr(sg: *mut scatterlist) -> *mut scatterlist { ((*sg).page_link & !SG_PAGE_LINK_MASK) as *mut scatterlist }
#[inline] pub unsafe fn sg_is_chain(sg: *mut scatterlist) -> bool { __sg_flags(sg) & SG_CHAIN as _ != 0 }
#[inline] pub unsafe fn sg_is_last(sg: *mut scatterlist) -> bool { __sg_flags(sg) & SG_END as _ != 0 }

#[inline] pub unsafe fn sg_next(mut sg: *mut scatterlist) -> *mut scatterlist {
    if sg_is_last(sg) { return core::ptr::null_mut(); }
    sg = sg.add(1);
    if sg_is_chain(sg) { sg = sg_chain_ptr(sg); }
    sg
}

extern "C" {
    pub fn bug_on(condition: bool);
    pub fn vm_warn_on_once(condition: bool) -> bool;
    pub fn page_range_contiguous(page: *mut page, pages: usize) -> bool;
    pub fn page_to_phys(page: *mut page) -> dma_addr_t;
    pub fn page_address(page: *mut page) -> *mut ::std::ffi::c_void;
    pub fn virt_to_page(addr: *const ::std::ffi::c_void) -> *mut page;
    pub fn offset_in_page(addr: *const ::std::ffi::c_void) -> usize;
    pub fn sg_alloc_table_from_pages_segment(sgt: *mut sg_table, pages: *mut *mut page, n_pages: u32, offset: u32, size: u64, max_segment: u32, gfp_mask: gfp_t) -> i32;
}
#[repr(C)] pub struct page { pub _opaque: [u8; 0] }
#[repr(C)] pub struct folio { pub page: page }
pub type gfp_t = usize;
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SHIFT: usize = 12;

#[inline] pub unsafe fn sg_assign_page(sg: *mut scatterlist, page: *mut page) {
    let link = (*sg).page_link & (SG_CHAIN | SG_END);
    bug_on(page as usize & SG_PAGE_LINK_MASK as usize != 0);
    (*sg).page_link = link | page as usize as ::std::ffi::c_ulong;
}
#[inline] pub unsafe fn sg_set_page(sg: *mut scatterlist, page: *mut page, len: u32, offset: u32) {
    let _ = vm_warn_on_once(!page_range_contiguous(page, (align_up((len as usize) + offset as usize, PAGE_SIZE)) / PAGE_SIZE));
    sg_assign_page(sg, page); (*sg).offset = offset; (*sg).length = len;
}
#[inline] pub unsafe fn sg_set_folio(sg: *mut scatterlist, folio: *mut folio, len: usize, offset: usize) {
    sg_assign_page(sg, &mut (*folio).page); (*sg).offset = offset as u32; (*sg).length = len as u32;
}
#[inline] pub unsafe fn sg_page(sg: *mut scatterlist) -> *mut page { ((*sg).page_link & !SG_PAGE_LINK_MASK) as *mut page }
#[inline] pub unsafe fn sg_set_buf(sg: *mut scatterlist, buf: *const ::std::ffi::c_void, buflen: u32) { sg_set_page(sg, virt_to_page(buf), buflen, offset_in_page(buf) as u32); }
#[inline] pub unsafe fn align_up(v: usize, a: usize) -> usize { (v + a - 1) & !(a - 1) }

#[inline] pub unsafe fn __sg_chain(chain_sg: *mut scatterlist, sgl: *mut scatterlist) {
    (*chain_sg).offset = 0; (*chain_sg).length = 0;
    (*chain_sg).page_link = ((sgl as usize as ::std::ffi::c_ulong) | SG_CHAIN) & !SG_END;
}
#[inline] pub unsafe fn sg_chain(prv: *mut scatterlist, prv_nents: u32, sgl: *mut scatterlist) { __sg_chain(prv.add((prv_nents - 1) as usize), sgl); }
#[inline] pub unsafe fn sg_mark_end(sg: *mut scatterlist) { (*sg).page_link |= SG_END; (*sg).page_link &= !SG_CHAIN; }
#[inline] pub unsafe fn sg_unmark_end(sg: *mut scatterlist) { (*sg).page_link &= !SG_END; }

#[cfg(feature = "CONFIG_NEED_SG_DMA_FLAGS")]
pub const SG_DMA_BUS_ADDRESS: u32 = 1 << 0;
#[cfg(feature = "CONFIG_NEED_SG_DMA_FLAGS")] pub const SG_DMA_SWIOTLB: u32 = 1 << 1;
#[inline] pub unsafe fn sg_dma_is_bus_address(sg: *mut scatterlist) -> bool { cfg!(feature = "CONFIG_NEED_SG_DMA_FLAGS") && ((*sg).dma_flags & SG_DMA_BUS_ADDRESS) != 0 }
#[inline] pub unsafe fn sg_dma_mark_bus_address(sg: *mut scatterlist) { #[cfg(feature = "CONFIG_NEED_SG_DMA_FLAGS")] { (*sg).dma_flags |= SG_DMA_BUS_ADDRESS; } }
#[inline] pub unsafe fn sg_dma_unmark_bus_address(sg: *mut scatterlist) { #[cfg(feature = "CONFIG_NEED_SG_DMA_FLAGS")] { (*sg).dma_flags &= !SG_DMA_BUS_ADDRESS; } }
#[inline] pub unsafe fn sg_dma_is_swiotlb(sg: *mut scatterlist) -> bool { cfg!(feature = "CONFIG_NEED_SG_DMA_FLAGS") && ((*sg).dma_flags & SG_DMA_SWIOTLB) != 0 }
#[inline] pub unsafe fn sg_dma_mark_swiotlb(sg: *mut scatterlist) { #[cfg(feature = "CONFIG_NEED_SG_DMA_FLAGS")] { (*sg).dma_flags |= SG_DMA_SWIOTLB; } }

#[inline] pub unsafe fn sg_phys(sg: *mut scatterlist) -> dma_addr_t { page_to_phys(sg_page(sg)) + (*sg).offset as dma_addr_t }
#[inline] pub unsafe fn sg_virt(sg: *mut scatterlist) -> *mut ::std::ffi::c_void { page_address(sg_page(sg)).add((*sg).offset as usize) }
#[inline] pub unsafe fn sg_init_marker(sgl: *mut scatterlist, nents: u32) { sg_mark_end(sgl.add((nents - 1) as usize)); }

extern "C" {
    pub fn sg_nents(sg: *mut scatterlist) -> i32;
    pub fn sg_nents_for_len(sg: *mut scatterlist, len: u64) -> i32;
    pub fn sg_nents_for_dma(sgl: *mut scatterlist, sglen: u32, len: usize) -> i32;
    pub fn sg_last(s: *mut scatterlist, nents: u32) -> *mut scatterlist;
    pub fn sg_init_table(sg: *mut scatterlist, nents: u32);
    pub fn sg_init_one(sg: *mut scatterlist, buf: *const ::std::ffi::c_void, buflen: u32);
    pub fn sg_split(input: *mut scatterlist, in_mapped_nents: i32, skip: isize, nb_splits: i32, split_sizes: *const usize, output: *mut *mut scatterlist, out_mapped_nents: *mut i32, gfp_mask: gfp_t) -> i32;
    pub fn __sg_free_table(table: *mut sg_table, max_ents: u32, nents_first_chunk: u32, free_fn: Option<unsafe extern "C" fn(*mut scatterlist, u32)>, order: u32);
    pub fn sg_free_table(table: *mut sg_table);
    pub fn sg_free_append_table(table: *mut sg_append_table);
    pub fn __sg_alloc_table(table: *mut sg_table, max_ents: u32, max_chunk: u32, first_chunk: *mut scatterlist, nents_first_chunk: u32, gfp_mask: gfp_t, alloc_fn: Option<unsafe extern "C" fn(u32, gfp_t) -> *mut scatterlist>) -> i32;
    pub fn sg_alloc_table(table: *mut sg_table, nents: u32, gfp_mask: gfp_t) -> i32;
    pub fn sg_alloc_table_from_pages_segment(table: *mut sg_table, pages: *mut *mut page, n_pages: u32, offset: u32, size: u64, max_segment: u32, gfp_mask: gfp_t) -> i32;
}
pub type sg_alloc_fn = unsafe extern "C" fn(u32, gfp_t) -> *mut scatterlist;
pub type sg_free_fn = unsafe extern "C" fn(*mut scatterlist, u32);
#[inline] pub unsafe fn sg_alloc_table_from_pages(sgt: *mut sg_table, pages: *mut *mut page, n_pages: u32, offset: u32, size: u64, gfp_mask: gfp_t) -> i32 { sg_alloc_table_from_pages_segment(sgt, pages, n_pages, offset, size, u32::MAX, gfp_mask) }

pub const SG_CHUNK_SIZE: usize = 128;
pub const SG_MAX_SINGLE_ALLOC: usize = PAGE_SIZE / core::mem::size_of::<scatterlist>();
#[cfg(feature = "CONFIG_ARCH_NO_SG_CHAIN")] pub const SG_MAX_SEGMENTS: usize = SG_CHUNK_SIZE;
#[cfg(not(feature = "CONFIG_ARCH_NO_SG_CHAIN"))] pub const SG_MAX_SEGMENTS: usize = 2048;
pub const SG_MITER_ATOMIC: u32 = 1 << 0;
pub const SG_MITER_TO_SG: u32 = 1 << 1;
pub const SG_MITER_FROM_SG: u32 = 1 << 2;
pub const SG_MITER_LOCAL: u32 = 1 << 3;

#[repr(C)] pub struct sg_page_iter { pub sg: *mut scatterlist, pub sg_pgoffset: u32, pub __nents: u32, pub __pg_advance: i32 }
#[repr(C)] pub struct sg_dma_page_iter { pub base: sg_page_iter }
extern "C" { pub fn __sg_page_iter_next(piter: *mut sg_page_iter) -> bool; pub fn __sg_page_iter_dma_next(iter: *mut sg_dma_page_iter) -> bool; pub fn __sg_page_iter_start(piter: *mut sg_page_iter, sglist: *mut scatterlist, nents: u32, pgoffset: usize); }
#[inline] pub unsafe fn sg_page_iter_page(piter: *mut sg_page_iter) -> *mut page { sg_page((*piter).sg).add((*piter).sg_pgoffset as usize) }
#[inline] pub unsafe fn sg_page_iter_dma_address(iter: *mut sg_dma_page_iter) -> dma_addr_t { sg_dma_address((*iter).base.sg) + (((*iter).base.sg_pgoffset as usize) << PAGE_SHIFT) as dma_addr_t }

#[repr(C)] pub struct sg_mapping_iter { pub page: *mut page, pub addr: *mut ::std::ffi::c_void, pub length: usize, pub consumed: usize, pub piter: sg_page_iter, pub __offset: u32, pub __remaining: u32, pub __flags: u32 }
extern "C" { pub fn sg_miter_start(miter: *mut sg_mapping_iter, sgl: *mut scatterlist, nents: u32, flags: u32); pub fn sg_miter_skip(miter: *mut sg_mapping_iter, offset: isize) -> bool; pub fn sg_miter_next(miter: *mut sg_mapping_iter) -> bool; pub fn sg_miter_stop(miter: *mut sg_mapping_iter); }

#[cfg(feature = "CONFIG_SGL_ALLOC")]
extern "C" {
    pub fn sgl_alloc_order(length: u64, order: u32, chainable: bool, gfp: gfp_t, nent_p: *mut u32) -> *mut scatterlist;
    pub fn sgl_alloc(length: u64, gfp: gfp_t, nent_p: *mut u32) -> *mut scatterlist;
    pub fn sgl_free_n_order(sgl: *mut scatterlist, nents: i32, order: i32);
    pub fn sgl_free_order(sgl: *mut scatterlist, order: i32);
    pub fn sgl_free(sgl: *mut scatterlist);
}
extern "C" {
    pub fn sg_copy_buffer(sgl: *mut scatterlist, nents: u32, buf: *mut ::std::ffi::c_void, buflen: usize, skip: isize, to_buffer: bool) -> usize;
    pub fn sg_copy_from_buffer(sgl: *mut scatterlist, nents: u32, buf: *const ::std::ffi::c_void, buflen: usize) -> usize;
    pub fn sg_copy_to_buffer(sgl: *mut scatterlist, nents: u32, buf: *mut ::std::ffi::c_void, buflen: usize) -> usize;
    pub fn sg_pcopy_from_buffer(sgl: *mut scatterlist, nents: u32, buf: *const ::std::ffi::c_void, buflen: usize, skip: isize) -> usize;
    pub fn sg_pcopy_to_buffer(sgl: *mut scatterlist, nents: u32, buf: *mut ::std::ffi::c_void, buflen: usize, skip: isize) -> usize;
    pub fn sg_zero_buffer(sgl: *mut scatterlist, nents: u32, buflen: usize, skip: isize) -> usize;
}
#[cfg(feature = "CONFIG_SG_POOL")]
extern "C" {
    pub fn sg_free_table_chained(table: *mut sg_table, nents_first_chunk: u32);
    pub fn sg_alloc_table_chained(table: *mut sg_table, nents: i32, first_chunk: *mut scatterlist, nents_first_chunk: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
