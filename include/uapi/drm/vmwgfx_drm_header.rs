/* SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT */
/* Faithful Rust translation of vmwgfx_drm.h. */

pub const DRM_VMW_MAX_SURFACE_FACES: usize = 6;
pub const DRM_VMW_MAX_MIP_LEVELS: usize = 24;

pub const DRM_VMW_GET_PARAM: u32 = 0;
pub const DRM_VMW_ALLOC_DMABUF: u32 = 1;
pub const DRM_VMW_ALLOC_BO: u32 = 1;
pub const DRM_VMW_UNREF_DMABUF: u32 = 2;
pub const DRM_VMW_HANDLE_CLOSE: u32 = 2;
pub const DRM_VMW_CURSOR_BYPASS: u32 = 3;
pub const DRM_VMW_CONTROL_STREAM: u32 = 4;
pub const DRM_VMW_CLAIM_STREAM: u32 = 5;
pub const DRM_VMW_UNREF_STREAM: u32 = 6;
pub const DRM_VMW_CREATE_CONTEXT: u32 = 7;
pub const DRM_VMW_UNREF_CONTEXT: u32 = 8;
pub const DRM_VMW_CREATE_SURFACE: u32 = 9;
pub const DRM_VMW_UNREF_SURFACE: u32 = 10;
pub const DRM_VMW_REF_SURFACE: u32 = 11;
pub const DRM_VMW_EXECBUF: u32 = 12;
pub const DRM_VMW_GET_3D_CAP: u32 = 13;
pub const DRM_VMW_FENCE_WAIT: u32 = 14;
pub const DRM_VMW_FENCE_SIGNALED: u32 = 15;
pub const DRM_VMW_FENCE_UNREF: u32 = 16;
pub const DRM_VMW_FENCE_EVENT: u32 = 17;
pub const DRM_VMW_PRESENT: u32 = 18;
pub const DRM_VMW_PRESENT_READBACK: u32 = 19;
pub const DRM_VMW_UPDATE_LAYOUT: u32 = 20;
pub const DRM_VMW_CREATE_SHADER: u32 = 21;
pub const DRM_VMW_UNREF_SHADER: u32 = 22;
pub const DRM_VMW_GB_SURFACE_CREATE: u32 = 23;
pub const DRM_VMW_GB_SURFACE_REF: u32 = 24;
pub const DRM_VMW_SYNCCPU: u32 = 25;
pub const DRM_VMW_CREATE_EXTENDED_CONTEXT: u32 = 26;
pub const DRM_VMW_GB_SURFACE_CREATE_EXT: u32 = 27;
pub const DRM_VMW_GB_SURFACE_REF_EXT: u32 = 28;
pub const DRM_VMW_MSG: u32 = 29;
pub const DRM_VMW_MKSSTAT_RESET: u32 = 30;
pub const DRM_VMW_MKSSTAT_ADD: u32 = 31;
pub const DRM_VMW_MKSSTAT_REMOVE: u32 = 32;

pub const DRM_VMW_PARAM_NUM_STREAMS: u32 = 0;
pub const DRM_VMW_PARAM_NUM_FREE_STREAMS: u32 = 1;
pub const DRM_VMW_PARAM_3D: u32 = 2;
pub const DRM_VMW_PARAM_HW_CAPS: u32 = 3;
pub const DRM_VMW_PARAM_FIFO_CAPS: u32 = 4;
pub const DRM_VMW_PARAM_MAX_FB_SIZE: u32 = 5;
pub const DRM_VMW_PARAM_FIFO_HW_VERSION: u32 = 6;
pub const DRM_VMW_PARAM_MAX_SURF_MEMORY: u32 = 7;
pub const DRM_VMW_PARAM_3D_CAPS_SIZE: u32 = 8;
pub const DRM_VMW_PARAM_MAX_MOB_MEMORY: u32 = 9;
pub const DRM_VMW_PARAM_MAX_MOB_SIZE: u32 = 10;
pub const DRM_VMW_PARAM_SCREEN_TARGET: u32 = 11;
pub const DRM_VMW_PARAM_DX: u32 = 12;
pub const DRM_VMW_PARAM_HW_CAPS2: u32 = 13;
pub const DRM_VMW_PARAM_SM4_1: u32 = 14;
pub const DRM_VMW_PARAM_SM5: u32 = 15;
pub const DRM_VMW_PARAM_GL43: u32 = 16;
pub const DRM_VMW_PARAM_DEVICE_ID: u32 = 17;

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum drm_vmw_handle_type { DRM_VMW_HANDLE_LEGACY = 0, DRM_VMW_HANDLE_PRIME = 1 }

#[repr(C)] pub struct drm_vmw_getparam_arg { pub value: u64, pub param: u32, pub pad64: u32 }
#[repr(C)] pub struct drm_vmw_context_arg { pub cid: i32, pub pad64: u32 }
#[repr(C)] pub struct drm_vmw_surface_create_req { pub flags: u32, pub format: u32, pub mip_levels: [u32; DRM_VMW_MAX_SURFACE_FACES], pub size_addr: u64, pub shareable: i32, pub scanout: i32 }
#[repr(C)] pub struct drm_vmw_surface_arg { pub sid: i32, pub handle_type: drm_vmw_handle_type }
#[repr(C)] pub struct drm_vmw_size { pub width: u32, pub height: u32, pub depth: u32, pub pad64: u32 }
#[repr(C)] pub union drm_vmw_surface_create_arg { pub rep: drm_vmw_surface_arg, pub req: drm_vmw_surface_create_req }
#[repr(C)] pub union drm_vmw_surface_reference_arg { pub rep: drm_vmw_surface_create_req, pub req: drm_vmw_surface_arg }

pub const DRM_VMW_EXECBUF_VERSION: u32 = 2;
pub const DRM_VMW_EXECBUF_FLAG_IMPORT_FENCE_FD: u32 = 1 << 0;
pub const DRM_VMW_EXECBUF_FLAG_EXPORT_FENCE_FD: u32 = 1 << 1;
#[repr(C)] pub struct drm_vmw_execbuf_arg { pub commands: u64, pub command_size: u32, pub throttle_us: u32, pub fence_rep: u64, pub version: u32, pub flags: u32, pub context_handle: u32, pub imported_fence_fd: i32 }
#[repr(C)] pub struct drm_vmw_fence_rep { pub handle: u32, pub mask: u32, pub seqno: u32, pub passed_seqno: u32, pub fd: i32, pub error: i32 }
#[repr(C)] pub struct drm_vmw_alloc_bo_req { pub size: u32, pub pad64: u32 }
pub type drm_vmw_alloc_dmabuf_req = drm_vmw_alloc_bo_req;
#[repr(C)] pub struct drm_vmw_bo_rep { pub map_handle: u64, pub handle: u32, pub cur_gmr_id: u32, pub cur_gmr_offset: u32, pub pad64: u32 }
pub type drm_vmw_dmabuf_rep = drm_vmw_bo_rep;
#[repr(C)] pub union drm_vmw_alloc_bo_arg { pub req: drm_vmw_alloc_bo_req, pub rep: drm_vmw_bo_rep }
pub type drm_vmw_alloc_dmabuf_arg = drm_vmw_alloc_bo_arg;

#[repr(C)] pub struct drm_vmw_rect { pub x: i32, pub y: i32, pub w: u32, pub h: u32 }
#[repr(C)] pub struct drm_vmw_control_stream_arg { pub stream_id: u32, pub enabled: u32, pub flags: u32, pub color_key: u32, pub handle: u32, pub offset: u32, pub format: i32, pub size: u32, pub width: u32, pub height: u32, pub pitch: [u32; 3], pub pad64: u32, pub src: drm_vmw_rect, pub dst: drm_vmw_rect }
pub const DRM_VMW_CURSOR_BYPASS_ALL: u32 = 1 << 0;
pub const DRM_VMW_CURSOR_BYPASS_FLAGS: u32 = 1;
#[repr(C)] pub struct drm_vmw_cursor_bypass_arg { pub flags: u32, pub crtc_id: u32, pub xpos: i32, pub ypos: i32, pub xhot: i32, pub yhot: i32 }
#[repr(C)] pub struct drm_vmw_stream_arg { pub stream_id: u32, pub pad64: u32 }
#[repr(C)] pub struct drm_vmw_get_3d_cap_arg { pub buffer: u64, pub max_size: u32, pub pad64: u32 }
pub const DRM_VMW_FENCE_FLAG_EXEC: u32 = 1 << 0;
pub const DRM_VMW_FENCE_FLAG_QUERY: u32 = 1 << 1;
pub const DRM_VMW_WAIT_OPTION_UNREF: u32 = 1 << 0;
#[repr(C)] pub struct drm_vmw_fence_wait_arg { pub handle: u32, pub cookie_valid: i32, pub kernel_cookie: u64, pub timeout_us: u64, pub lazy: i32, pub flags: i32, pub wait_options: i32, pub pad64: i32 }
#[repr(C)] pub struct drm_vmw_fence_signaled_arg { pub handle: u32, pub flags: u32, pub signaled: i32, pub passed_seqno: u32, pub signaled_flags: u32, pub pad64: u32 }
#[repr(C)] pub struct drm_vmw_fence_arg { pub handle: u32, pub pad64: u32 }
pub const DRM_VMW_EVENT_FENCE_SIGNALED: u32 = 0x80000000;
#[repr(C)] pub struct drm_vmw_event_fence { pub base: drm_event, pub user_data: u64, pub tv_sec: u32, pub tv_usec: u32 }
pub const DRM_VMW_FE_FLAG_REQ_TIME: u32 = 1 << 0;
#[repr(C)] pub struct drm_vmw_fence_event_arg { pub fence_rep: u64, pub user_data: u64, pub handle: u32, pub flags: u32 }
#[repr(C)] pub struct drm_vmw_present_arg { pub fb_id: u32, pub sid: u32, pub dest_x: i32, pub dest_y: i32, pub clips_ptr: u64, pub num_clips: u32, pub pad64: u32 }
#[repr(C)] pub struct drm_vmw_present_readback_arg { pub fb_id: u32, pub num_clips: u32, pub clips_ptr: u64, pub fence_rep: u64 }
#[repr(C)] pub struct drm_vmw_update_layout_arg { pub num_outputs: u32, pub pad64: u32, pub rects: u64 }

#[repr(i32)] #[derive(Copy, Clone)] pub enum drm_vmw_shader_type { drm_vmw_shader_type_vs = 0, drm_vmw_shader_type_ps = 1 }
#[repr(C)] pub struct drm_vmw_shader_create_arg { pub shader_type: drm_vmw_shader_type, pub size: u32, pub buffer_handle: u32, pub shader_handle: u32, pub offset: u64 }
#[repr(C)] pub struct drm_vmw_shader_arg { pub handle: u32, pub pad64: u32 }
#[repr(i32)] #[derive(Copy, Clone)] pub enum drm_vmw_surface_flags { drm_vmw_surface_flag_shareable = 1 << 0, drm_vmw_surface_flag_scanout = 1 << 1, drm_vmw_surface_flag_create_buffer = 1 << 2, drm_vmw_surface_flag_coherent = 1 << 3 }
#[repr(C)] pub struct drm_vmw_gb_surface_create_req { pub svga3d_flags: u32, pub format: u32, pub mip_levels: u32, pub drm_surface_flags: drm_vmw_surface_flags, pub multisample_count: u32, pub autogen_filter: u32, pub buffer_handle: u32, pub array_size: u32, pub base_size: drm_vmw_size }
#[repr(C)] pub struct drm_vmw_gb_surface_create_rep { pub handle: u32, pub backup_size: u32, pub buffer_handle: u32, pub buffer_size: u32, pub buffer_map_handle: u64 }
#[repr(C)] pub union drm_vmw_gb_surface_create_arg { pub rep: drm_vmw_gb_surface_create_rep, pub req: drm_vmw_gb_surface_create_req }
#[repr(C)] pub struct drm_vmw_gb_surface_ref_rep { pub creq: drm_vmw_gb_surface_create_req, pub crep: drm_vmw_gb_surface_create_rep }
#[repr(C)] pub union drm_vmw_gb_surface_reference_arg { pub rep: drm_vmw_gb_surface_ref_rep, pub req: drm_vmw_surface_arg }
#[repr(i32)] #[derive(Copy, Clone)] pub enum drm_vmw_synccpu_flags { drm_vmw_synccpu_read = 1 << 0, drm_vmw_synccpu_write = 1 << 1, drm_vmw_synccpu_dontblock = 1 << 2, drm_vmw_synccpu_allow_cs = 1 << 3 }
#[repr(i32)] #[derive(Copy, Clone)] pub enum drm_vmw_synccpu_op { drm_vmw_synccpu_grab = 0, drm_vmw_synccpu_release = 1 }
#[repr(C)] pub struct drm_vmw_synccpu_arg { pub op: drm_vmw_synccpu_op, pub flags: drm_vmw_synccpu_flags, pub handle: u32, pub pad64: u32 }
#[repr(i32)] #[derive(Copy, Clone)] pub enum drm_vmw_extended_context { drm_vmw_context_legacy = 0, drm_vmw_context_dx = 1 }
#[repr(C)] pub union drm_vmw_extended_context_arg { pub req: drm_vmw_extended_context, pub rep: drm_vmw_context_arg }
#[repr(C)] pub struct drm_vmw_handle_close_arg { pub handle: u32, pub pad64: u32 }
pub type drm_vmw_unref_dmabuf_arg = drm_vmw_handle_close_arg;
#[repr(i32)] #[derive(Copy, Clone)] pub enum drm_vmw_surface_version { drm_vmw_gb_surface_v1 = 0 }
#[repr(C)] pub struct drm_vmw_gb_surface_create_ext_req { pub base: drm_vmw_gb_surface_create_req, pub version: drm_vmw_surface_version, pub svga3d_flags_upper_32_bits: u32, pub multisample_pattern: u32, pub quality_level: u32, pub buffer_byte_stride: u32, pub must_be_zero: u32 }
#[repr(C)] pub union drm_vmw_gb_surface_create_ext_arg { pub rep: drm_vmw_gb_surface_create_rep, pub req: drm_vmw_gb_surface_create_ext_req }
#[repr(C)] pub struct drm_vmw_gb_surface_ref_ext_rep { pub creq: drm_vmw_gb_surface_create_ext_req, pub crep: drm_vmw_gb_surface_create_rep }
#[repr(C)] pub union drm_vmw_gb_surface_reference_ext_arg { pub rep: drm_vmw_gb_surface_ref_ext_rep, pub req: drm_vmw_surface_arg }
#[repr(C)] pub struct drm_vmw_msg_arg { pub send: u64, pub receive: u64, pub send_only: i32, pub receive_len: u32 }
#[repr(C)] pub struct drm_vmw_mksstat_add_arg { pub stat: u64, pub info: u64, pub strs: u64, pub stat_len: u64, pub info_len: u64, pub strs_len: u64, pub description: u64, pub id: u64 }
#[repr(C)] pub struct drm_vmw_mksstat_remove_arg { pub id: u64 }

/* Supplied by drm.h. */
#[allow(non_camel_case_types)] pub type drm_event = crate::drm_event;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
