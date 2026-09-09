/*
 * Copyright 2013 Red Hat
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

// Dependency symbols from drm.h are intentionally left external.

pub const DRM_VIRTGPU_MAP: u32 = 0x01;
pub const DRM_VIRTGPU_EXECBUFFER: u32 = 0x02;
pub const DRM_VIRTGPU_GETPARAM: u32 = 0x03;
pub const DRM_VIRTGPU_RESOURCE_CREATE: u32 = 0x04;
pub const DRM_VIRTGPU_RESOURCE_INFO: u32 = 0x05;
pub const DRM_VIRTGPU_TRANSFER_FROM_HOST: u32 = 0x06;
pub const DRM_VIRTGPU_TRANSFER_TO_HOST: u32 = 0x07;
pub const DRM_VIRTGPU_WAIT: u32 = 0x08;
pub const DRM_VIRTGPU_GET_CAPS: u32 = 0x09;
pub const DRM_VIRTGPU_RESOURCE_CREATE_BLOB: u32 = 0x0a;
pub const DRM_VIRTGPU_CONTEXT_INIT: u32 = 0x0b;

pub const VIRTGPU_EXECBUF_FENCE_FD_IN: u32 = 0x01;
pub const VIRTGPU_EXECBUF_FENCE_FD_OUT: u32 = 0x02;
pub const VIRTGPU_EXECBUF_RING_IDX: u32 = 0x04;
pub const VIRTGPU_EXECBUF_FLAGS: u32 = VIRTGPU_EXECBUF_FENCE_FD_IN | VIRTGPU_EXECBUF_FENCE_FD_OUT | VIRTGPU_EXECBUF_RING_IDX | 0;

#[repr(C)]
pub struct drm_virtgpu_map { pub offset: u64, pub handle: u32, pub pad: u32 }

pub const VIRTGPU_EXECBUF_SYNCOBJ_RESET: u32 = 0x01;
pub const VIRTGPU_EXECBUF_SYNCOBJ_FLAGS: u32 = VIRTGPU_EXECBUF_SYNCOBJ_RESET | 0;
#[repr(C)]
pub struct drm_virtgpu_execbuffer_syncobj { pub handle: u32, pub flags: u32, pub point: u64 }

// fence_fd is modified on success if VIRTGPU_EXECBUF_FENCE_FD_OUT flag is set.
#[repr(C)]
pub struct drm_virtgpu_execbuffer {
    pub flags: u32, pub size: u32, pub command: u64, pub bo_handles: u64,
    pub num_bo_handles: u32, pub fence_fd: i32, pub ring_idx: u32,
    pub syncobj_stride: u32, pub num_in_syncobjs: u32, pub num_out_syncobjs: u32,
    pub in_syncobjs: u64, pub out_syncobjs: u64,
}

pub const VIRTGPU_PARAM_3D_FEATURES: u32 = 1;
pub const VIRTGPU_PARAM_CAPSET_QUERY_FIX: u32 = 2;
pub const VIRTGPU_PARAM_RESOURCE_BLOB: u32 = 3;
pub const VIRTGPU_PARAM_HOST_VISIBLE: u32 = 4;
pub const VIRTGPU_PARAM_CROSS_DEVICE: u32 = 5;
pub const VIRTGPU_PARAM_CONTEXT_INIT: u32 = 6;
pub const VIRTGPU_PARAM_SUPPORTED_CAPSET_IDs: u32 = 7;
pub const VIRTGPU_PARAM_EXPLICIT_DEBUG_NAME: u32 = 8;
pub const VIRTGPU_PARAM_BLOB_ALIGNMENT: u32 = 9;
#[repr(C)] pub struct drm_virtgpu_getparam { pub param: u64, pub value: u64 }

#[repr(C)]
pub struct drm_virtgpu_resource_create {
    pub target: u32, pub format: u32, pub bind: u32, pub width: u32, pub height: u32,
    pub depth: u32, pub array_size: u32, pub last_level: u32, pub nr_samples: u32,
    pub flags: u32, pub bo_handle: u32, pub res_handle: u32, pub size: u32, pub stride: u32,
}
#[repr(C)] pub struct drm_virtgpu_resource_info { pub bo_handle: u32, pub res_handle: u32, pub size: u32, pub blob_mem: u32 }
#[repr(C)] pub struct drm_virtgpu_3d_box { pub x: u32, pub y: u32, pub z: u32, pub w: u32, pub h: u32, pub d: u32 }
#[repr(C)] pub struct drm_virtgpu_3d_transfer_to_host { pub bo_handle: u32, pub box_: drm_virtgpu_3d_box, pub level: u32, pub offset: u32, pub stride: u32, pub layer_stride: u32 }
#[repr(C)] pub struct drm_virtgpu_3d_transfer_from_host { pub bo_handle: u32, pub box_: drm_virtgpu_3d_box, pub level: u32, pub offset: u32, pub stride: u32, pub layer_stride: u32 }

pub const VIRTGPU_WAIT_NOWAIT: u32 = 1;
#[repr(C)] pub struct drm_virtgpu_3d_wait { pub handle: u32, pub flags: u32 }
pub const VIRTGPU_DRM_CAPSET_VIRGL: u32 = 1;
pub const VIRTGPU_DRM_CAPSET_VIRGL2: u32 = 2;
pub const VIRTGPU_DRM_CAPSET_GFXSTREAM_VULKAN: u32 = 3;
pub const VIRTGPU_DRM_CAPSET_VENUS: u32 = 4;
pub const VIRTGPU_DRM_CAPSET_CROSS_DOMAIN: u32 = 5;
pub const VIRTGPU_DRM_CAPSET_DRM: u32 = 6;
#[repr(C)] pub struct drm_virtgpu_get_caps { pub cap_set_id: u32, pub cap_set_ver: u32, pub addr: u64, pub size: u32, pub pad: u32 }

pub const VIRTGPU_BLOB_MEM_GUEST: u32 = 0x0001;
pub const VIRTGPU_BLOB_MEM_HOST3D: u32 = 0x0002;
pub const VIRTGPU_BLOB_MEM_HOST3D_GUEST: u32 = 0x0003;
pub const VIRTGPU_BLOB_FLAG_USE_MAPPABLE: u32 = 0x0001;
pub const VIRTGPU_BLOB_FLAG_USE_SHAREABLE: u32 = 0x0002;
pub const VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE: u32 = 0x0004;
pub const DRM_VIRTGPU_BLOB_FLAG_HINT_DEFER_MAPPING: u32 = 0x0001;
#[repr(C)]
pub struct drm_virtgpu_resource_create_blob {
    pub blob_mem: u32, pub blob_flags: u32, pub bo_handle: u32, pub res_handle: u32,
    pub size: u64, pub pad: u32, pub cmd_size: u32, pub cmd: u64, pub blob_id: u64,
    pub blob_hints: u32, pub pad2: u32,
}

pub const VIRTGPU_CONTEXT_PARAM_CAPSET_ID: u32 = 0x0001;
pub const VIRTGPU_CONTEXT_PARAM_NUM_RINGS: u32 = 0x0002;
pub const VIRTGPU_CONTEXT_PARAM_POLL_RINGS_MASK: u32 = 0x0003;
pub const VIRTGPU_CONTEXT_PARAM_DEBUG_NAME: u32 = 0x0004;
#[repr(C)] pub struct drm_virtgpu_context_set_param { pub param: u64, pub value: u64 }
#[repr(C)] pub struct drm_virtgpu_context_init { pub num_params: u32, pub pad: u32, pub ctx_set_params: u64 }
pub const VIRTGPU_EVENT_FENCE_SIGNALED: u32 = 0x90000000;

// ioctl encodings depend on DRM_IOWR and DRM_COMMAND_BASE supplied by drm.h.
pub const DRM_IOCTL_VIRTGPU_MAP: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_VIRTGPU_MAP, drm_virtgpu_map);
pub const DRM_IOCTL_VIRTGPU_EXECBUFFER: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_VIRTGPU_EXECBUFFER, drm_virtgpu_execbuffer);
pub const DRM_IOCTL_VIRTGPU_GETPARAM: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_VIRTGPU_GETPARAM, drm_virtgpu_getparam);
pub const DRM_IOCTL_VIRTGPU_RESOURCE_CREATE: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_VIRTGPU_RESOURCE_CREATE, drm_virtgpu_resource_create);
pub const DRM_IOCTL_VIRTGPU_RESOURCE_INFO: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_VIRTGPU_RESOURCE_INFO, drm_virtgpu_resource_info);
pub const DRM_IOCTL_VIRTGPU_TRANSFER_FROM_HOST: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_VIRTGPU_TRANSFER_FROM_HOST, drm_virtgpu_3d_transfer_from_host);
pub const DRM_IOCTL_VIRTGPU_TRANSFER_TO_HOST: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_VIRTGPU_TRANSFER_TO_HOST, drm_virtgpu_3d_transfer_to_host);
pub const DRM_IOCTL_VIRTGPU_WAIT: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_VIRTGPU_WAIT, drm_virtgpu_3d_wait);
pub const DRM_IOCTL_VIRTGPU_GET_CAPS: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_VIRTGPU_GET_CAPS, drm_virtgpu_get_caps);
pub const DRM_IOCTL_VIRTGPU_RESOURCE_CREATE_BLOB: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_VIRTGPU_RESOURCE_CREATE_BLOB, drm_virtgpu_resource_create_blob);
pub const DRM_IOCTL_VIRTGPU_CONTEXT_INIT: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_VIRTGPU_CONTEXT_INIT, drm_virtgpu_context_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
