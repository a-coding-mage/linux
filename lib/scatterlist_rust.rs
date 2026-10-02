// SPDX-License-Identifier: GPL-2.0-only
//! Native scatterlist owner, preserving the original exports and C ABI.
#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "scatterlist.rs"]
mod implementation;
pub use implementation::*;
ffi_export::export_symbol!(sg_nents, sg_nents, "", "");
ffi_export::export_symbol!(sg_nents_for_len, sg_nents_for_len, "", "");
ffi_export::export_symbol!(sg_nents_for_dma, sg_nents_for_dma, "", "");
ffi_export::export_symbol!(sg_last, sg_last, "", "");
ffi_export::export_symbol!(sg_init_table, sg_init_table, "", "");
ffi_export::export_symbol!(sg_init_one, sg_init_one, "", "");
ffi_export::export_symbol!(__sg_free_table, __sg_free_table, "", "");
ffi_export::export_symbol!(sg_free_append_table, sg_free_append_table, "", "");
ffi_export::export_symbol!(sg_free_table, sg_free_table, "", "");
ffi_export::export_symbol!(__sg_alloc_table, __sg_alloc_table, "", "");
ffi_export::export_symbol!(sg_alloc_table, sg_alloc_table, "", "");
ffi_export::export_symbol!(
    sg_alloc_append_table_from_pages,
    sg_alloc_append_table_from_pages,
    "",
    ""
);
ffi_export::export_symbol!(
    sg_alloc_table_from_pages_segment,
    sg_alloc_table_from_pages_segment,
    "",
    ""
);
#[cfg(CONFIG_SGL_ALLOC)]
ffi_export::export_symbol!(sgl_alloc_order, sgl_alloc_order, "", "");
#[cfg(CONFIG_SGL_ALLOC)]
ffi_export::export_symbol!(sgl_alloc, sgl_alloc, "", "");
#[cfg(CONFIG_SGL_ALLOC)]
ffi_export::export_symbol!(sgl_free_n_order, sgl_free_n_order, "", "");
#[cfg(CONFIG_SGL_ALLOC)]
ffi_export::export_symbol!(sgl_free_order, sgl_free_order, "", "");
#[cfg(CONFIG_SGL_ALLOC)]
ffi_export::export_symbol!(sgl_free, sgl_free, "", "");
ffi_export::export_symbol!(__sg_page_iter_start, __sg_page_iter_start, "", "");
ffi_export::export_symbol!(__sg_page_iter_next, __sg_page_iter_next, "", "");
ffi_export::export_symbol!(__sg_page_iter_dma_next, __sg_page_iter_dma_next, "", "");
ffi_export::export_symbol!(sg_miter_start, sg_miter_start, "", "");
ffi_export::export_symbol!(sg_miter_skip, sg_miter_skip, "", "");
ffi_export::export_symbol!(sg_miter_next, sg_miter_next, "", "");
ffi_export::export_symbol!(sg_miter_stop, sg_miter_stop, "", "");
ffi_export::export_symbol!(sg_copy_buffer, sg_copy_buffer, "", "");
ffi_export::export_symbol!(sg_copy_from_buffer, sg_copy_from_buffer, "", "");
ffi_export::export_symbol!(sg_copy_to_buffer, sg_copy_to_buffer, "", "");
ffi_export::export_symbol!(sg_pcopy_from_buffer, sg_pcopy_from_buffer, "", "");
ffi_export::export_symbol!(sg_pcopy_to_buffer, sg_pcopy_to_buffer, "", "");
ffi_export::export_symbol!(sg_zero_buffer, sg_zero_buffer, "", "");
ffi_export::export_symbol!(extract_iter_to_sg, extract_iter_to_sg, "GPL", "");
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
