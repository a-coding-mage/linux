/*
 * videobuf2-dma-contig.h - DMA contig memory allocator for videobuf2
 *
 * Copyright (C) 2010 Samsung Electronics
 *
 * Author: Pawel Osciak <pawel@osciak.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 */

// Dependencies supplied by the surrounding translation unit:
// <media/videobuf2-v4l2.h>
// <linux/dma-mapping.h>

pub unsafe fn vb2_dma_contig_plane_dma_addr(
    vb: *mut vb2_buffer,
    plane_no: ::core::ffi::c_uint,
) -> dma_addr_t {
    let addr: *mut dma_addr_t = vb2_plane_cookie(vb, plane_no);

    *addr
}

extern "C" {
    pub fn vb2_dma_contig_set_max_seg_size(
        dev: *mut device,
        size: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub static vb2_dma_contig_memops: vb2_mem_ops;
}

pub unsafe fn vb2_dma_contig_clear_max_seg_size(_dev: *mut device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
