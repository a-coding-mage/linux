/*
 * Virtio GPU Device
 *
 * Copyright Red Hat, Inc. 2013-2014
 *
 * Authors:
 *     Dave Airlie <airlied@redhat.com>
 *     Gerd Hoffmann <kraxel@redhat.com>
 *
 * This header is BSD licensed so anyone can use the definitions
 * to implement compatible drivers/servers:
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of IBM nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL IBM OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF
 * USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT
 * OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

// #include <linux/types.h>

pub const VIRTIO_GPU_F_VIRGL: u32 = 0;
pub const VIRTIO_GPU_F_EDID: u32 = 1;
pub const VIRTIO_GPU_F_RESOURCE_UUID: u32 = 2;
pub const VIRTIO_GPU_F_RESOURCE_BLOB: u32 = 3;
pub const VIRTIO_GPU_F_CONTEXT_INIT: u32 = 4;
pub const VIRTIO_GPU_F_BLOB_ALIGNMENT: u32 = 5;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum virtio_gpu_ctrl_type {
    VIRTIO_GPU_UNDEFINED = 0,
    VIRTIO_GPU_CMD_GET_DISPLAY_INFO = 0x0100,
    VIRTIO_GPU_CMD_RESOURCE_CREATE_2D,
    VIRTIO_GPU_CMD_RESOURCE_UNREF,
    VIRTIO_GPU_CMD_SET_SCANOUT,
    VIRTIO_GPU_CMD_RESOURCE_FLUSH,
    VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D,
    VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING,
    VIRTIO_GPU_CMD_RESOURCE_DETACH_BACKING,
    VIRTIO_GPU_CMD_GET_CAPSET_INFO,
    VIRTIO_GPU_CMD_GET_CAPSET,
    VIRTIO_GPU_CMD_GET_EDID,
    VIRTIO_GPU_CMD_RESOURCE_ASSIGN_UUID,
    VIRTIO_GPU_CMD_RESOURCE_CREATE_BLOB,
    VIRTIO_GPU_CMD_SET_SCANOUT_BLOB,
    VIRTIO_GPU_CMD_CTX_CREATE = 0x0200,
    VIRTIO_GPU_CMD_CTX_DESTROY,
    VIRTIO_GPU_CMD_CTX_ATTACH_RESOURCE,
    VIRTIO_GPU_CMD_CTX_DETACH_RESOURCE,
    VIRTIO_GPU_CMD_RESOURCE_CREATE_3D,
    VIRTIO_GPU_CMD_TRANSFER_TO_HOST_3D,
    VIRTIO_GPU_CMD_TRANSFER_FROM_HOST_3D,
    VIRTIO_GPU_CMD_SUBMIT_3D,
    VIRTIO_GPU_CMD_RESOURCE_MAP_BLOB,
    VIRTIO_GPU_CMD_RESOURCE_UNMAP_BLOB,
    VIRTIO_GPU_CMD_UPDATE_CURSOR = 0x0300,
    VIRTIO_GPU_CMD_MOVE_CURSOR,
    VIRTIO_GPU_RESP_OK_NODATA = 0x1100,
    VIRTIO_GPU_RESP_OK_DISPLAY_INFO,
    VIRTIO_GPU_RESP_OK_CAPSET_INFO,
    VIRTIO_GPU_RESP_OK_CAPSET,
    VIRTIO_GPU_RESP_OK_EDID,
    VIRTIO_GPU_RESP_OK_RESOURCE_UUID,
    VIRTIO_GPU_RESP_OK_MAP_INFO,
    VIRTIO_GPU_RESP_ERR_UNSPEC = 0x1200,
    VIRTIO_GPU_RESP_ERR_OUT_OF_MEMORY,
    VIRTIO_GPU_RESP_ERR_INVALID_SCANOUT_ID,
    VIRTIO_GPU_RESP_ERR_INVALID_RESOURCE_ID,
    VIRTIO_GPU_RESP_ERR_INVALID_CONTEXT_ID,
    VIRTIO_GPU_RESP_ERR_INVALID_PARAMETER,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum virtio_gpu_shm_id {
    VIRTIO_GPU_SHM_ID_UNDEFINED = 0,
    VIRTIO_GPU_SHM_ID_HOST_VISIBLE = 1,
}

pub const VIRTIO_GPU_FLAG_FENCE: u32 = 1 << 0;
pub const VIRTIO_GPU_FLAG_INFO_RING_IDX: u32 = 1 << 1;

#[repr(C)]
pub struct virtio_gpu_ctrl_hdr { pub r#type: __le32, pub flags: __le32, pub fence_id: __le64, pub ctx_id: __le32, pub ring_idx: __u8, pub padding: [__u8; 3] }

#[repr(C)]
pub struct virtio_gpu_cursor_pos { pub scanout_id: __le32, pub x: __le32, pub y: __le32, pub padding: __le32 }

#[repr(C)]
pub struct virtio_gpu_update_cursor { pub hdr: virtio_gpu_ctrl_hdr, pub pos: virtio_gpu_cursor_pos, pub resource_id: __le32, pub hot_x: __le32, pub hot_y: __le32, pub padding: __le32 }

#[repr(C)]
pub struct virtio_gpu_rect { pub x: __le32, pub y: __le32, pub width: __le32, pub height: __le32 }

#[repr(C)]
pub struct virtio_gpu_resource_unref { pub hdr: virtio_gpu_ctrl_hdr, pub resource_id: __le32, pub padding: __le32 }
#[repr(C)]
pub struct virtio_gpu_resource_create_2d { pub hdr: virtio_gpu_ctrl_hdr, pub resource_id: __le32, pub format: __le32, pub width: __le32, pub height: __le32 }
#[repr(C)]
pub struct virtio_gpu_set_scanout { pub hdr: virtio_gpu_ctrl_hdr, pub r: virtio_gpu_rect, pub scanout_id: __le32, pub resource_id: __le32 }
#[repr(C)]
pub struct virtio_gpu_resource_flush { pub hdr: virtio_gpu_ctrl_hdr, pub r: virtio_gpu_rect, pub resource_id: __le32, pub padding: __le32 }
#[repr(C)]
pub struct virtio_gpu_transfer_to_host_2d { pub hdr: virtio_gpu_ctrl_hdr, pub r: virtio_gpu_rect, pub offset: __le64, pub resource_id: __le32, pub padding: __le32 }
#[repr(C)]
pub struct virtio_gpu_mem_entry { pub addr: __le64, pub length: __le32, pub padding: __le32 }
#[repr(C)]
pub struct virtio_gpu_resource_attach_backing { pub hdr: virtio_gpu_ctrl_hdr, pub resource_id: __le32, pub nr_entries: __le32 }
#[repr(C)]
pub struct virtio_gpu_resource_detach_backing { pub hdr: virtio_gpu_ctrl_hdr, pub resource_id: __le32, pub padding: __le32 }

pub const VIRTIO_GPU_MAX_SCANOUTS: usize = 16;
#[repr(C)]
pub struct virtio_gpu_display_one { pub r: virtio_gpu_rect, pub enabled: __le32, pub flags: __le32 }
#[repr(C)]
pub struct virtio_gpu_resp_display_info { pub hdr: virtio_gpu_ctrl_hdr, pub pmodes: [virtio_gpu_display_one; VIRTIO_GPU_MAX_SCANOUTS] }

#[repr(C)]
pub struct virtio_gpu_box { pub x: __le32, pub y: __le32, pub z: __le32, pub w: __le32, pub h: __le32, pub d: __le32 }
#[repr(C)]
pub struct virtio_gpu_transfer_host_3d { pub hdr: virtio_gpu_ctrl_hdr, pub box_: virtio_gpu_box, pub offset: __le64, pub resource_id: __le32, pub level: __le32, pub stride: __le32, pub layer_stride: __le32 }
pub const VIRTIO_GPU_RESOURCE_FLAG_Y_0_TOP: u32 = 1 << 0;
#[repr(C)]
pub struct virtio_gpu_resource_create_3d { pub hdr: virtio_gpu_ctrl_hdr, pub resource_id: __le32, pub target: __le32, pub format: __le32, pub bind: __le32, pub width: __le32, pub height: __le32, pub depth: __le32, pub array_size: __le32, pub last_level: __le32, pub nr_samples: __le32, pub flags: __le32, pub padding: __le32 }
pub const VIRTIO_GPU_CONTEXT_INIT_CAPSET_ID_MASK: u32 = 0x000000ff;
#[repr(C)]
pub struct virtio_gpu_ctx_create { pub hdr: virtio_gpu_ctrl_hdr, pub nlen: __le32, pub context_init: __le32, pub debug_name: [::std::ffi::c_char; 64] }
#[repr(C)]
pub struct virtio_gpu_ctx_destroy { pub hdr: virtio_gpu_ctrl_hdr }
#[repr(C)]
pub struct virtio_gpu_ctx_resource { pub hdr: virtio_gpu_ctrl_hdr, pub resource_id: __le32, pub padding: __le32 }
#[repr(C)]
pub struct virtio_gpu_cmd_submit { pub hdr: virtio_gpu_ctrl_hdr, pub size: __le32, pub padding: __le32 }

pub const VIRTIO_GPU_CAPSET_VIRGL: u32 = 1;
pub const VIRTIO_GPU_CAPSET_VIRGL2: u32 = 2;
pub const VIRTIO_GPU_CAPSET_GFXSTREAM_VULKAN: u32 = 3;
pub const VIRTIO_GPU_CAPSET_VENUS: u32 = 4;
pub const VIRTIO_GPU_CAPSET_CROSS_DOMAIN: u32 = 5;
pub const VIRTIO_GPU_CAPSET_DRM: u32 = 6;
#[repr(C)]
pub struct virtio_gpu_get_capset_info { pub hdr: virtio_gpu_ctrl_hdr, pub capset_index: __le32, pub padding: __le32 }
#[repr(C)]
pub struct virtio_gpu_resp_capset_info { pub hdr: virtio_gpu_ctrl_hdr, pub capset_id: __le32, pub capset_max_version: __le32, pub capset_max_size: __le32, pub padding: __le32 }
#[repr(C)]
pub struct virtio_gpu_get_capset { pub hdr: virtio_gpu_ctrl_hdr, pub capset_id: __le32, pub capset_version: __le32 }
#[repr(C)]
pub struct virtio_gpu_resp_capset { pub hdr: virtio_gpu_ctrl_hdr, pub capset_data: [__u8; 0] }
#[repr(C)]
pub struct virtio_gpu_cmd_get_edid { pub hdr: virtio_gpu_ctrl_hdr, pub scanout: __le32, pub padding: __le32 }
#[repr(C)]
pub struct virtio_gpu_resp_edid { pub hdr: virtio_gpu_ctrl_hdr, pub size: __le32, pub padding: __le32, pub edid: [__u8; 1024] }
pub const VIRTIO_GPU_EVENT_DISPLAY: u32 = 1 << 0;
#[repr(C)]
pub struct virtio_gpu_config { pub events_read: __le32, pub events_clear: __le32, pub num_scanouts: __le32, pub num_capsets: __le32, pub blob_alignment: __le32 }

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum virtio_gpu_formats {
    VIRTIO_GPU_FORMAT_B8G8R8A8_UNORM = 1,
    VIRTIO_GPU_FORMAT_B8G8R8X8_UNORM = 2,
    VIRTIO_GPU_FORMAT_A8R8G8B8_UNORM = 3,
    VIRTIO_GPU_FORMAT_X8R8G8B8_UNORM = 4,
    VIRTIO_GPU_FORMAT_R8G8B8A8_UNORM = 67,
    VIRTIO_GPU_FORMAT_X8B8G8R8_UNORM = 68,
    VIRTIO_GPU_FORMAT_A8B8G8R8_UNORM = 121,
    VIRTIO_GPU_FORMAT_R8G8B8X8_UNORM = 134,
}

#[repr(C)]
pub struct virtio_gpu_resource_assign_uuid { pub hdr: virtio_gpu_ctrl_hdr, pub resource_id: __le32, pub padding: __le32 }
#[repr(C)]
pub struct virtio_gpu_resp_resource_uuid { pub hdr: virtio_gpu_ctrl_hdr, pub uuid: [__u8; 16] }
#[repr(C)]
pub struct virtio_gpu_resource_create_blob { pub hdr: virtio_gpu_ctrl_hdr, pub resource_id: __le32, pub blob_mem: __le32, pub blob_flags: __le32, pub nr_entries: __le32, pub blob_id: __le64, pub size: __le64 }
pub const VIRTIO_GPU_BLOB_MEM_GUEST: u32 = 0x0001;
pub const VIRTIO_GPU_BLOB_MEM_HOST3D: u32 = 0x0002;
pub const VIRTIO_GPU_BLOB_MEM_HOST3D_GUEST: u32 = 0x0003;
pub const VIRTIO_GPU_BLOB_FLAG_USE_MAPPABLE: u32 = 0x0001;
pub const VIRTIO_GPU_BLOB_FLAG_USE_SHAREABLE: u32 = 0x0002;
pub const VIRTIO_GPU_BLOB_FLAG_USE_CROSS_DEVICE: u32 = 0x0004;
#[repr(C)]
pub struct virtio_gpu_set_scanout_blob { pub hdr: virtio_gpu_ctrl_hdr, pub r: virtio_gpu_rect, pub scanout_id: __le32, pub resource_id: __le32, pub width: __le32, pub height: __le32, pub format: __le32, pub padding: __le32, pub strides: [__le32; 4], pub offsets: [__le32; 4] }
#[repr(C)]
pub struct virtio_gpu_resource_map_blob { pub hdr: virtio_gpu_ctrl_hdr, pub resource_id: __le32, pub padding: __le32, pub offset: __le64 }
pub const VIRTIO_GPU_MAP_CACHE_MASK: u32 = 0x0f;
pub const VIRTIO_GPU_MAP_CACHE_NONE: u32 = 0x00;
pub const VIRTIO_GPU_MAP_CACHE_CACHED: u32 = 0x01;
pub const VIRTIO_GPU_MAP_CACHE_UNCACHED: u32 = 0x02;
pub const VIRTIO_GPU_MAP_CACHE_WC: u32 = 0x03;
#[repr(C)]
pub struct virtio_gpu_resp_map_info { pub hdr: virtio_gpu_ctrl_hdr, pub map_info: __u32, pub padding: __u32 }
#[repr(C)]
pub struct virtio_gpu_resource_unmap_blob { pub hdr: virtio_gpu_ctrl_hdr, pub resource_id: __le32, pub padding: __le32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
