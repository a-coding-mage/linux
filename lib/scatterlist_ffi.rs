// SPDX-License-Identifier: GPL-2.0-only
//! Header-only primitives used by the scatterlist translation.
//! Layouts come from the kernel's generated bindings, never Rust replicas.
#![allow(non_camel_case_types, non_snake_case, dead_code, improper_ctypes)]
pub(crate) use bindings::{
    folio, folio_queue, gfp_t, iov_iter, iov_iter_extraction_t, page, scatterlist, sg_append_table,
    sg_dma_page_iter, sg_mapping_iter, sg_page_iter, sg_table, xa_state, xarray,
};
pub(crate) use bindings::{iov_iter_advance, iov_iter_extract_pages, unpin_user_page};
pub(crate) const PAGE_SIZE: usize = bindings::PAGE_SIZE;
pub(crate) const PAGE_SHIFT: u32 = bindings::PAGE_SHIFT;
pub(crate) const SG_MAX_SINGLE_ALLOC: u32 =
    (PAGE_SIZE / core::mem::size_of::<scatterlist>()) as u32;
pub(crate) const SG_MITER_ATOMIC: u32 = bindings::SG_MITER_ATOMIC;
pub(crate) const SG_MITER_LOCAL: u32 = bindings::SG_MITER_LOCAL;
pub(crate) const SG_MITER_TO_SG: u32 = bindings::SG_MITER_TO_SG;
pub(crate) const SG_MITER_FROM_SG: u32 = bindings::SG_MITER_FROM_SG;
pub(crate) const EINVAL: i32 = bindings::EINVAL as i32;
pub(crate) const ENOMEM: i32 = bindings::ENOMEM as i32;
pub(crate) const EOPNOTSUPP: i32 = bindings::EOPNOTSUPP as i32;
pub(crate) const EIO: i32 = bindings::EIO as i32;
pub(crate) type sg_alloc_fn = bindings::sg_alloc_fn;
pub(crate) type sg_free_fn = bindings::sg_free_fn;
extern "C" {
    #[link_name = "rust_sg_next"]
    pub(crate) fn sg_next(sg: *mut scatterlist) -> *mut scatterlist;
    #[link_name = "rust_sg_is_last"]
    pub(crate) fn sg_is_last(sg: *mut scatterlist) -> bool;
    #[link_name = "rust_sg_chain_ptr"]
    pub(crate) fn sg_chain_ptr(sg: *mut scatterlist) -> *mut scatterlist;
    #[link_name = "rust_sg_chain"]
    pub(crate) fn sg_chain(sg: *mut scatterlist, n: u32, next: *mut scatterlist);
    #[link_name = "rust_sg_chain_entry"]
    pub(crate) fn sg_chain_entry(sg: *mut scatterlist, next: *mut scatterlist);
    #[link_name = "rust_sg_mark_end"]
    pub(crate) fn sg_mark_end(sg: *mut scatterlist);
    #[link_name = "rust_sg_init_marker"]
    pub(crate) fn sg_init_marker(sg: *mut scatterlist, n: u32);
    #[link_name = "rust_sg_set_page"]
    pub(crate) fn sg_set_page(sg: *mut scatterlist, p: *mut page, len: u32, off: u32);
    #[link_name = "rust_sg_set_buf"]
    pub(crate) fn sg_set_buf(sg: *mut scatterlist, buf: *const core::ffi::c_void, len: u32);
    #[link_name = "rust_sg_page"]
    pub(crate) fn sg_page(sg: *mut scatterlist) -> *mut page;
    #[link_name = "rust_sg_phys"]
    pub(crate) fn sg_phys(sg: *mut scatterlist) -> u64;
    #[link_name = "rust_sg_dma_len"]
    pub(crate) fn sg_dma_len(sg: *mut scatterlist) -> u32;
    #[link_name = "rust_sg_page_to_pfn"]
    pub(crate) fn page_to_pfn(p: *mut page) -> usize;
    #[link_name = "rust_sg_pfn_to_page"]
    pub(crate) fn pfn_to_page(pfn: usize) -> *mut page;
    #[link_name = "rust_sg_same_pgmap"]
    pub(crate) fn same_pgmap(a: *mut page, b: *mut page) -> bool;
    #[link_name = "rust_sg_alloc_page"]
    pub(crate) fn alloc_page(gfp: gfp_t) -> *mut scatterlist;
    #[link_name = "rust_sg_free_page"]
    pub(crate) fn free_page(sg: *mut scatterlist);
    #[link_name = "rust_sg_kmemleak_alloc"]
    pub(crate) fn kmemleak_alloc_page(sg: *mut scatterlist, gfp: gfp_t);
    #[link_name = "rust_sg_kmemleak_free"]
    pub(crate) fn kmemleak_free_page(sg: *mut scatterlist);
    #[link_name = "rust_sg_kmalloc"]
    pub(crate) fn kmalloc_sg(n: u32, gfp: gfp_t) -> *mut scatterlist;
    #[link_name = "rust_sg_kfree"]
    pub(crate) fn kfree_sg(sg: *mut scatterlist);
    #[link_name = "rust_sg_alloc_pages"]
    pub(crate) fn alloc_pages(gfp: gfp_t, order: u32) -> *mut page;
    #[link_name = "rust_sg_free_pages"]
    pub(crate) fn free_pages(page: *mut page, order: u32);
    #[link_name = "rust_sg_without_dma"]
    pub(crate) fn without_dma(gfp: gfp_t) -> gfp_t;
    #[link_name = "rust_sg_bug_on"]
    pub(crate) fn bug_on(condition: bool);
    #[link_name = "rust_sg_warn_on"]
    pub(crate) fn warn_on(condition: bool) -> bool;
    #[link_name = "rust_sg_warn_no_chain"]
    pub(crate) fn warn_no_chain(condition: bool) -> bool;
    #[link_name = "rust_sg_warn_folioq_next"]
    pub(crate) fn warn_folioq_next(condition: bool) -> bool;
    #[link_name = "rust_sg_warn_folioq_count"]
    pub(crate) fn warn_folioq_count(condition: bool) -> bool;
    #[link_name = "rust_sg_kmap"]
    pub(crate) fn kmap(page: *mut page) -> *mut u8;
    #[link_name = "rust_sg_kmap_atomic"]
    pub(crate) fn kmap_atomic(page: *mut page) -> *mut u8;
    #[link_name = "rust_sg_kmap_local_page"]
    pub(crate) fn kmap_local_page(page: *mut page) -> *mut u8;
    #[link_name = "rust_sg_kunmap"]
    pub(crate) fn kunmap(page: *mut page);
    #[link_name = "rust_sg_kunmap_atomic"]
    pub(crate) fn kunmap_atomic(addr: *mut core::ffi::c_void);
    #[link_name = "rust_sg_kunmap_local"]
    pub(crate) fn kunmap_local(addr: *mut core::ffi::c_void);
    #[link_name = "rust_sg_flush_dcache_page"]
    pub(crate) fn flush_dcache_page(page: *mut page);
    #[link_name = "rust_sg_pagefault_disabled"]
    pub(crate) fn pagefault_disabled() -> bool;
    #[link_name = "rust_sg_warn_atomic_pagefault"]
    pub(crate) fn warn_atomic_pagefault(condition: bool) -> bool;
    #[link_name = "rust_sg_iter_page"]
    pub(crate) fn iter_page(p: *mut sg_page_iter) -> *mut page;
    #[link_name = "rust_sg_folioq_next"]
    pub(crate) fn folioq_next(q: *const folio_queue) -> *const folio_queue;
    #[link_name = "rust_sg_folioq_slots"]
    pub(crate) fn folioq_slots(q: *const folio_queue) -> u32;
    #[link_name = "rust_sg_folioq_folio"]
    pub(crate) fn folioq_folio(q: *const folio_queue, slot: u32) -> *mut folio;
    #[link_name = "rust_sg_folioq_size"]
    pub(crate) fn folioq_size(q: *const folio_queue, slot: u32) -> usize;
    #[link_name = "rust_sg_folio_page"]
    pub(crate) fn folio_page(folio: *mut folio) -> *mut page;
    #[link_name = "rust_sg_folio_size"]
    pub(crate) fn folio_size(folio: *mut folio) -> usize;
    #[link_name = "rust_sg_offset_in_folio"]
    pub(crate) fn offset_in_folio(folio: *mut folio, start: i64) -> usize;
    #[link_name = "rust_sg_is_hugetlb"]
    pub(crate) fn is_hugetlb(folio: *mut folio) -> bool;
    #[link_name = "rust_sg_is_vmalloc_addr"]
    pub(crate) fn is_vmalloc_addr(addr: usize) -> bool;
    #[link_name = "rust_sg_vmalloc_to_page"]
    pub(crate) fn vmalloc_to_page(addr: usize) -> *mut page;
    #[link_name = "rust_sg_virt_to_page"]
    pub(crate) fn virt_to_page(addr: usize) -> *mut page;
    #[link_name = "rust_sg_xas_init"]
    pub(crate) fn xas_init(xas: *mut xa_state, xa: *mut xarray, index: usize);
    #[link_name = "rust_sg_xas_find"]
    pub(crate) fn xas_find(xas: *mut xa_state) -> *mut folio;
    #[link_name = "rust_sg_xas_next"]
    pub(crate) fn xas_next(xas: *mut xa_state) -> *mut folio;
    #[link_name = "rust_sg_xas_retry"]
    pub(crate) fn xas_retry(xas: *mut xa_state, folio: *mut folio) -> bool;
    #[link_name = "rust_sg_xa_is_value"]
    pub(crate) fn xa_is_value(folio: *mut folio) -> bool;
    #[link_name = "rust_sg_rcu_lock"]
    pub(crate) fn rcu_lock();
    #[link_name = "rust_sg_rcu_unlock"]
    pub(crate) fn rcu_unlock();
    #[link_name = "rust_sg_unsupported"]
    pub(crate) fn unsupported(iter_type: u8);
}
