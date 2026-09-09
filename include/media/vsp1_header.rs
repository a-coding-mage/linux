/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * vsp1.h  -- R-Car VSP1 API
 *
 * Copyright (C) 2015 Renesas Electronics Corporation
 *
 * Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vsp1_dl_list {
    _private: [u8; 0],
}

pub type dma_addr_t = usize;
pub type u32 = core::ffi::c_uint;

#[repr(C)]
pub struct sg_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct v4l2_rect {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct v4l2_pix_format_mplane {
    _private: [u8; 0],
}

pub type v4l2_ycbcr_encoding = core::ffi::c_int;
pub type v4l2_quantization = core::ffi::c_int;

/* -----------------------------------------------------------------------------
 * VSP1 DU interface
 */

pub const VSP1_DU_STATUS_COMPLETE: u32 = 1u32 << 0;
pub const VSP1_DU_STATUS_WRITEBACK: u32 = 1u32 << 1;

/**
 * struct vsp1_du_lif_config - VSP LIF configuration
 * @width: output frame width
 * @height: output frame height
 * @interlaced: true for interlaced pipelines
 * @callback: frame completion callback function (optional). When a callback
 *            is provided, the VSP driver guarantees that it will be called once
 *            and only once for each vsp1_du_atomic_flush() call.
 * @callback_data: data to be passed to the frame completion callback
 */
#[repr(C)]
pub struct vsp1_du_lif_config {
    pub width: u32,
    pub height: u32,
    pub interlaced: bool,
    pub callback: Option<unsafe extern "C" fn(data: *mut c_void, status: u32, crc: u32)>,
    pub callback_data: *mut c_void,
}

#[repr(C)]
pub struct vsp1_du_atomic_config {
    pub pixelformat: u32,
    pub pitch: u32,
    pub mem: [dma_addr_t; 3],
    pub src: v4l2_rect,
    pub dst: v4l2_rect,
    pub alpha: u32,
    pub zpos: u32,
    pub premult: bool,
    pub color_encoding: v4l2_ycbcr_encoding,
    pub color_range: v4l2_quantization,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum vsp1_du_crc_source {
    VSP1_DU_CRC_NONE,
    VSP1_DU_CRC_PLANE,
    VSP1_DU_CRC_OUTPUT,
}

#[repr(C)]
pub struct vsp1_du_crc_config {
    pub source: vsp1_du_crc_source,
    pub index: u32,
}

#[repr(C)]
pub struct vsp1_du_writeback_config {
    pub pixelformat: u32,
    pub pitch: u32,
    pub mem: [dma_addr_t; 3],
}

#[repr(C)]
pub struct vsp1_du_atomic_pipe_config {
    pub crc: vsp1_du_crc_config,
    pub writeback: vsp1_du_writeback_config,
}

extern "C" {
    pub fn vsp1_du_init(dev: *mut device) -> i32;
    pub fn vsp1_du_enable(dev: *mut device, pipe_index: u32, cfg: *const vsp1_du_lif_config) -> i32;
    pub fn vsp1_du_disable(dev: *mut device, pipe_index: u32) -> i32;
    pub fn vsp1_du_atomic_begin(dev: *mut device, pipe_index: u32);
    pub fn vsp1_du_atomic_update(dev: *mut device, pipe_index: u32, rpf: u32, cfg: *const vsp1_du_atomic_config) -> i32;
    pub fn vsp1_du_atomic_flush(dev: *mut device, pipe_index: u32, cfg: *const vsp1_du_atomic_pipe_config);
    pub fn vsp1_du_map_sg(dev: *mut device, sgt: *mut sg_table) -> i32;
    pub fn vsp1_du_unmap_sg(dev: *mut device, sgt: *mut sg_table);

    pub fn vsp1_isp_init(dev: *mut device) -> i32;
    pub fn vsp1_isp_get_bus_master(dev: *mut device) -> *mut device;
}

#[repr(C)]
pub struct vsp1_isp_buffer_desc {
    pub size: usize,
    pub cpu_addr: *mut c_void,
    pub dma_addr: dma_addr_t,
}

#[repr(C)]
pub struct vsp1_isp_job_desc {
    pub config: vsp1_isp_job_config,
    pub img: vsp1_isp_job_img,
    pub dl: *mut vsp1_dl_list,
}

#[repr(C)]
pub struct vsp1_isp_job_config {
    pub pairs: u32,
    pub mem: dma_addr_t,
}

#[repr(C)]
pub struct vsp1_isp_job_img {
    pub fmt: v4l2_pix_format_mplane,
    pub mem: dma_addr_t,
}

#[repr(C)]
pub struct vsp1_vspx_frame_end {
    pub vspx_frame_end: Option<unsafe extern "C" fn(data: *mut c_void)>,
    pub frame_end_data: *mut c_void,
}

extern "C" {
    pub fn vsp1_isp_alloc_buffer(dev: *mut device, size: usize, buffer_desc: *mut vsp1_isp_buffer_desc) -> i32;
    pub fn vsp1_isp_free_buffer(dev: *mut device, buffer_desc: *mut vsp1_isp_buffer_desc);
    pub fn vsp1_isp_start_streaming(dev: *mut device, frame_end: *mut vsp1_vspx_frame_end) -> i32;
    pub fn vsp1_isp_stop_streaming(dev: *mut device);
    pub fn vsp1_isp_job_prepare(dev: *mut device, job: *mut vsp1_isp_job_desc) -> i32;
    pub fn vsp1_isp_job_run(dev: *mut device, job: *mut vsp1_isp_job_desc) -> i32;
    pub fn vsp1_isp_job_release(dev: *mut device, job: *mut vsp1_isp_job_desc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
