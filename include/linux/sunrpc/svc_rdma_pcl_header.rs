/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2020, Oracle and/or its affiliates
 */

/* Dependency supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct svc_rdma_segment {
    pub rs_handle: u32,
    pub rs_length: u32,
    pub rs_offset: u64,
}

#[repr(C)]
pub struct svc_rdma_chunk {
    pub ch_list: list_head,
    pub ch_position: u32,
    pub ch_length: u32,
    pub ch_payload_length: u32,
    pub ch_segcount: u32,
    pub ch_segments: [svc_rdma_segment; 0],
}

#[repr(C)]
pub struct svc_rdma_pcl {
    pub cl_count: ::core::ffi::c_uint,
    pub cl_chunks: list_head,
}

pub unsafe fn pcl_init(pcl: *mut svc_rdma_pcl) {
    INIT_LIST_HEAD(&mut (*pcl).cl_chunks);
}

pub unsafe fn pcl_is_empty(pcl: *const svc_rdma_pcl) -> bool {
    list_empty(&(*pcl).cl_chunks)
}

pub unsafe fn pcl_first_chunk(pcl: *const svc_rdma_pcl) -> *mut svc_rdma_chunk {
    if pcl_is_empty(pcl) {
        return core::ptr::null_mut();
    }
    list_first_entry(&(*pcl).cl_chunks, svc_rdma_chunk, ch_list)
}

pub unsafe fn pcl_next_chunk(
    pcl: *const svc_rdma_pcl,
    chunk: *mut svc_rdma_chunk,
) -> *mut svc_rdma_chunk {
    if list_is_last(&(*chunk).ch_list, &(*pcl).cl_chunks) {
        return core::ptr::null_mut();
    }
    list_next_entry(chunk, ch_list)
}

#[macro_export]
macro_rules! pcl_for_each_chunk {
    ($pos:ident, $pcl:expr) => {
        for $pos = list_first_entry(
            &(*$pcl).cl_chunks,
            svc_rdma_chunk,
            ch_list,
        );
        &(*$pos).ch_list != &(*$pcl).cl_chunks;
        $pos = list_next_entry($pos, ch_list)
    };
}

#[macro_export]
macro_rules! pcl_for_each_segment {
    ($pos:ident, $chunk:expr) => {
        for $pos = &mut (*$chunk).ch_segments[0];
        $pos < &mut (*$chunk).ch_segments[(*$chunk).ch_segcount as usize];
        $pos = $pos.add(1)
    };
}

pub unsafe fn pcl_chunk_end_offset(chunk: *const svc_rdma_chunk) -> ::core::ffi::c_uint {
    xdr_align_size((*chunk).ch_position + (*chunk).ch_payload_length)
}

#[repr(C)]
pub struct svc_rdma_recv_ctxt {
    _private: [u8; 0],
}

extern "C" {
    pub fn pcl_free(pcl: *mut svc_rdma_pcl);
    pub fn pcl_alloc_call(rctxt: *mut svc_rdma_recv_ctxt, p: *mut __be32) -> bool;
    pub fn pcl_alloc_read(rctxt: *mut svc_rdma_recv_ctxt, p: *mut __be32) -> bool;
    pub fn pcl_alloc_write(
        rctxt: *mut svc_rdma_recv_ctxt,
        pcl: *mut svc_rdma_pcl,
        p: *mut __be32,
    ) -> bool;
    pub fn pcl_check_read_chunk_positions(
        rctxt: *mut svc_rdma_recv_ctxt,
        inline_len: ::core::ffi::c_uint,
    ) -> bool;
    pub fn pcl_process_nonpayloads(
        pcl: *const svc_rdma_pcl,
        xdr: *const xdr_buf,
        actor: Option<unsafe extern "C" fn(*const xdr_buf, *mut core::ffi::c_void) -> ::core::ffi::c_int>,
        data: *mut core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
