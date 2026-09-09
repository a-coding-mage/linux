/*
 * videobuf2-dma-sg.h - DMA scatter/gather memory allocator for videobuf2
 *
 * Copyright (C) 2010 Samsung Electronics
 *
 * Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 */

// Dependency supplied by <media/videobuf2-v4l2.h>.
use core::ffi::c_void;

#[repr(C)]
pub struct sg_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vb2_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vb2_mem_ops {
    _private: [u8; 0],
}

extern "C" {
    pub fn vb2_plane_cookie(vb: *mut vb2_buffer, plane_no: u32) -> *mut c_void;

    pub static vb2_dma_sg_memops: vb2_mem_ops;
}

#[inline]
pub unsafe fn vb2_dma_sg_plane_desc(
    vb: *mut vb2_buffer,
    plane_no: u32,
) -> *mut sg_table {
    vb2_plane_cookie(vb, plane_no) as *mut sg_table
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
