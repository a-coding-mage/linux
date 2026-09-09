/* SPDX-License-Identifier: MIT */
/* Copyright (C) The Asahi Linux Contributors */
/* Copyright (C) 2018-2023 Collabora Ltd. */
/* Copyright (C) 2014-2018 Broadcom */

// Translated from asahi_drm.h. Types and ioctl helpers from drm.h are external.

#[repr(i32)]
pub enum drm_asahi_ioctl_id {
    DRM_ASAHI_GET_PARAMS = 0,
    DRM_ASAHI_GET_TIME,
    DRM_ASAHI_VM_CREATE,
    DRM_ASAHI_VM_DESTROY,
    DRM_ASAHI_VM_BIND,
    DRM_ASAHI_GEM_CREATE,
    DRM_ASAHI_GEM_MMAP_OFFSET,
    DRM_ASAHI_GEM_BIND_OBJECT,
    DRM_ASAHI_QUEUE_CREATE,
    DRM_ASAHI_QUEUE_DESTROY,
    DRM_ASAHI_SUBMIT,
}

pub const DRM_ASAHI_MAX_CLUSTERS: usize = 64;

#[repr(C)]
pub struct drm_asahi_params_global {
    pub features: u64, pub gpu_generation: u32, pub gpu_variant: u32,
    pub gpu_revision: u32, pub chip_id: u32, pub num_dies: u32,
    pub num_clusters_total: u32, pub num_cores_per_cluster: u32,
    pub max_frequency_khz: u32, pub core_masks: [u64; DRM_ASAHI_MAX_CLUSTERS],
    pub vm_start: u64, pub vm_end: u64, pub vm_kernel_min_size: u64,
    pub max_commands_per_submission: u32, pub max_attachments: u32,
    pub command_timestamp_frequency_hz: u64,
}

#[repr(u64)]
pub enum drm_asahi_feature { DRM_ASAHI_FEATURE_SOFT_FAULTS = 1u64 << 0 }

#[repr(C)] pub struct drm_asahi_get_params { pub param_group: u32, pub pad: u32, pub pointer: u64, pub size: u64 }
#[repr(C)] pub struct drm_asahi_vm_create { pub kernel_start: u64, pub kernel_end: u64, pub vm_id: u32, pub pad: u32 }
#[repr(C)] pub struct drm_asahi_vm_destroy { pub vm_id: u32, pub pad: u32 }

#[repr(u32)]
pub enum drm_asahi_gem_flags { DRM_ASAHI_GEM_WRITEBACK = 1u32 << 0, DRM_ASAHI_GEM_VM_PRIVATE = 1u32 << 1 }
#[repr(C)] pub struct drm_asahi_gem_create { pub size: u64, pub flags: u32, pub vm_id: u32, pub handle: u32, pub pad: u32 }
#[repr(C)] pub struct drm_asahi_gem_mmap_offset { pub handle: u32, pub flags: u32, pub offset: u64 }

#[repr(u32)]
pub enum drm_asahi_bind_flags {
    DRM_ASAHI_BIND_UNBIND = 1u32 << 0, DRM_ASAHI_BIND_READ = 1u32 << 1,
    DRM_ASAHI_BIND_WRITE = 1u32 << 2, DRM_ASAHI_BIND_SINGLE_PAGE = 1u32 << 3,
}
#[repr(C)] pub struct drm_asahi_gem_bind_op { pub flags: u32, pub handle: u32, pub offset: u64, pub range: u64, pub addr: u64 }
#[repr(C)] pub struct drm_asahi_vm_bind { pub vm_id: u32, pub num_binds: u32, pub stride: u32, pub pad: u32, pub userptr: u64 }

#[repr(u32)] pub enum drm_asahi_bind_object_op { DRM_ASAHI_BIND_OBJECT_OP_BIND = 0, DRM_ASAHI_BIND_OBJECT_OP_UNBIND = 1 }
#[repr(u32)] pub enum drm_asahi_bind_object_flags { DRM_ASAHI_BIND_OBJECT_USAGE_TIMESTAMPS = 1u32 << 0 }
#[repr(C)] pub struct drm_asahi_gem_bind_object { pub op: u32, pub flags: u32, pub handle: u32, pub vm_id: u32, pub offset: u64, pub range: u64, pub object_handle: u32, pub pad: u32 }

#[repr(u32)] pub enum drm_asahi_cmd_type { DRM_ASAHI_CMD_RENDER = 0, DRM_ASAHI_CMD_COMPUTE = 1, DRM_ASAHI_SET_VERTEX_ATTACHMENTS = 2, DRM_ASAHI_SET_FRAGMENT_ATTACHMENTS = 3, DRM_ASAHI_SET_COMPUTE_ATTACHMENTS = 4 }
#[repr(u32)] pub enum drm_asahi_priority { DRM_ASAHI_PRIORITY_LOW = 0, DRM_ASAHI_PRIORITY_MEDIUM = 1, DRM_ASAHI_PRIORITY_HIGH = 2, DRM_ASAHI_PRIORITY_REALTIME = 3 }
#[repr(C)] pub struct drm_asahi_queue_create { pub flags: u32, pub vm_id: u32, pub priority: u32, pub queue_id: u32, pub usc_exec_base: u64 }
#[repr(C)] pub struct drm_asahi_queue_destroy { pub queue_id: u32, pub pad: u32 }
#[repr(u32)] pub enum drm_asahi_sync_type { DRM_ASAHI_SYNC_SYNCOBJ = 0, DRM_ASAHI_SYNC_TIMELINE_SYNCOBJ = 1 }
#[repr(C)] pub struct drm_asahi_sync { pub sync_type: u32, pub handle: u32, pub timeline_value: u64 }
pub const DRM_ASAHI_BARRIER_NONE: u16 = 0xFFFF;
#[repr(C)] pub struct drm_asahi_cmd_header { pub cmd_type: u16, pub size: u16, pub vdm_barrier: u16, pub cdm_barrier: u16 }
#[repr(C)] pub struct drm_asahi_submit { pub syncs: u64, pub cmdbuf: u64, pub flags: u32, pub queue_id: u32, pub in_sync_count: u32, pub out_sync_count: u32, pub cmdbuf_size: u32, pub pad: u32 }
#[repr(C)] pub struct drm_asahi_attachment { pub pointer: u64, pub size: u64, pub pad: u32, pub flags: u32 }

#[repr(u32)] pub enum drm_asahi_render_flags { DRM_ASAHI_RENDER_VERTEX_SCRATCH = 1u32 << 0, DRM_ASAHI_RENDER_PROCESS_EMPTY_TILES = 1u32 << 1, DRM_ASAHI_RENDER_NO_VERTEX_CLUSTERING = 1u32 << 2, DRM_ASAHI_RENDER_DBIAS_IS_INT = 1u32 << 18 }
#[repr(C)] pub struct drm_asahi_zls_buffer { pub base: u64, pub comp_base: u64, pub stride: u32, pub comp_stride: u32 }
#[repr(C)] pub struct drm_asahi_timestamp { pub handle: u32, pub offset: u32 }
#[repr(C)] pub struct drm_asahi_timestamps { pub start: drm_asahi_timestamp, pub end: drm_asahi_timestamp }
#[repr(C)] pub struct drm_asahi_helper_program { pub binary: u32, pub cfg: u32, pub data: u64 }
#[repr(C)] pub struct drm_asahi_bg_eot { pub usc: u32, pub rsrc_spec: u32 }

#[repr(C)]
pub struct drm_asahi_cmd_render {
    pub flags: u32, pub isp_zls_pixels: u32, pub vdm_ctrl_stream_base: u64,
    pub vertex_helper: drm_asahi_helper_program, pub fragment_helper: drm_asahi_helper_program,
    pub isp_scissor_base: u64, pub isp_dbias_base: u64, pub isp_oclqry_base: u64,
    pub depth: drm_asahi_zls_buffer, pub stencil: drm_asahi_zls_buffer,
    pub zls_ctrl: u64, pub ppp_multisamplectl: u64, pub sampler_heap: u64,
    pub ppp_ctrl: u32, pub width_px: u16, pub height_px: u16, pub layers: u16,
    pub sampler_count: u16, pub utile_width_px: u8, pub utile_height_px: u8,
    pub samples: u8, pub sample_size_B: u8, pub isp_merge_upper_x: u32,
    pub isp_merge_upper_y: u32, pub bg: drm_asahi_bg_eot, pub eot: drm_asahi_bg_eot,
    pub partial_bg: drm_asahi_bg_eot, pub partial_eot: drm_asahi_bg_eot,
    pub isp_bgobjdepth: u32, pub isp_bgobjvals: u32,
    pub ts_vtx: drm_asahi_timestamps, pub ts_frag: drm_asahi_timestamps,
}
#[repr(C)] pub struct drm_asahi_cmd_compute { pub flags: u32, pub sampler_count: u32, pub cdm_ctrl_stream_base: u64, pub cdm_ctrl_stream_end: u64, pub sampler_heap: u64, pub helper: drm_asahi_helper_program, pub ts: drm_asahi_timestamps }
#[repr(C)] pub struct drm_asahi_get_time { pub flags: u64, pub gpu_timestamp: u64 }

// DRM_IOCTL_ASAHI values depend on the external drm.h DRM_IO* definitions.
// The C header defines them as DRM_IO{W,WR}(DRM_COMMAND_BASE + id, struct).
// These declarations preserve the externally supplied ioctl-number interface;
// `drm_ioctl_asahi!` is the corresponding dependency-side mapping.
pub const DRM_IOCTL_ASAHI_GET_PARAMS: u64 = drm_ioctl_asahi!(W, GET_PARAMS, drm_asahi_get_params);
pub const DRM_IOCTL_ASAHI_GET_TIME: u64 = drm_ioctl_asahi!(WR, GET_TIME, drm_asahi_get_time);
pub const DRM_IOCTL_ASAHI_VM_CREATE: u64 = drm_ioctl_asahi!(WR, VM_CREATE, drm_asahi_vm_create);
pub const DRM_IOCTL_ASAHI_VM_DESTROY: u64 = drm_ioctl_asahi!(W, VM_DESTROY, drm_asahi_vm_destroy);
pub const DRM_IOCTL_ASAHI_VM_BIND: u64 = drm_ioctl_asahi!(W, VM_BIND, drm_asahi_vm_bind);
pub const DRM_IOCTL_ASAHI_GEM_CREATE: u64 = drm_ioctl_asahi!(WR, GEM_CREATE, drm_asahi_gem_create);
pub const DRM_IOCTL_ASAHI_GEM_MMAP_OFFSET: u64 = drm_ioctl_asahi!(WR, GEM_MMAP_OFFSET, drm_asahi_gem_mmap_offset);
pub const DRM_IOCTL_ASAHI_GEM_BIND_OBJECT: u64 = drm_ioctl_asahi!(WR, GEM_BIND_OBJECT, drm_asahi_gem_bind_object);
pub const DRM_IOCTL_ASAHI_QUEUE_CREATE: u64 = drm_ioctl_asahi!(WR, QUEUE_CREATE, drm_asahi_queue_create);
pub const DRM_IOCTL_ASAHI_QUEUE_DESTROY: u64 = drm_ioctl_asahi!(W, QUEUE_DESTROY, drm_asahi_queue_destroy);
pub const DRM_IOCTL_ASAHI_SUBMIT: u64 = drm_ioctl_asahi!(W, SUBMIT, drm_asahi_submit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
