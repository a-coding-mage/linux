/* SPDX-License-Identifier: MIT */
/* Rust translation of drm.h. C includes and build-time platform conditions are
   intentionally represented by primitive Rust types and external names. */

pub type __s8 = i8; pub type __u8 = u8; pub type __s16 = i16; pub type __u16 = u16;
pub type __s32 = i32; pub type __u32 = u32; pub type __s64 = i64; pub type __u64 = u64;
pub type __kernel_size_t = usize;
pub type drm_handle_t = libc::c_uint;
pub type drm_context_t = libc::c_uint; pub type drm_drawable_t = libc::c_uint; pub type drm_magic_t = libc::c_uint;

pub const DRM_NAME: &str = "drm"; pub const DRM_MIN_ORDER: u32 = 5; pub const DRM_MAX_ORDER: u32 = 22; pub const DRM_RAM_PERCENT: u32 = 10;
pub const _DRM_LOCK_HELD: u32 = 0x80000000; pub const _DRM_LOCK_CONT: u32 = 0x40000000;
#[inline] pub const fn _DRM_LOCK_IS_HELD(lock: u32) -> u32 { lock & _DRM_LOCK_HELD }
#[inline] pub const fn _DRM_LOCK_IS_CONT(lock: u32) -> u32 { lock & _DRM_LOCK_CONT }
#[inline] pub const fn _DRM_LOCKING_CONTEXT(lock: u32) -> u32 { lock & !(_DRM_LOCK_HELD | _DRM_LOCK_CONT) }

#[repr(C)] pub struct drm_clip_rect { pub x1:u16,pub y1:u16,pub x2:u16,pub y2:u16 }
#[repr(C)] pub struct drm_drawable_info { pub num_rects:u32,pub rects:*mut drm_clip_rect }
#[repr(C)] pub struct drm_tex_region { pub next:u8,pub prev:u8,pub in_use:u8,pub padding:u8,pub age:u32 }
#[repr(C)] pub struct drm_hw_lock { pub lock:u32,pub padding:[i8;60] }
#[repr(C)] pub struct drm_version { pub version_major:i32,pub version_minor:i32,pub version_patchlevel:i32,pub name_len:__kernel_size_t,pub name:*mut i8,pub date_len:__kernel_size_t,pub date:*mut i8,pub desc_len:__kernel_size_t,pub desc:*mut i8 }
#[repr(C)] pub struct drm_unique { pub unique_len:__kernel_size_t,pub unique:*mut i8 }
#[repr(C)] pub struct drm_list { pub count:i32,pub version:*mut drm_version }
#[repr(C)] pub struct drm_block { pub unused:i32 }
#[repr(C)] pub struct drm_control { pub func:i32,pub irq:i32 }

#[repr(C)] #[derive(Copy,Clone)] pub enum drm_map_type {_DRM_FRAME_BUFFER=0,_DRM_REGISTERS=1,_DRM_SHM=2,_DRM_AGP=3,_DRM_SCATTER_GATHER=4,_DRM_CONSISTENT=5}
#[repr(C)] #[derive(Copy,Clone)] pub enum drm_map_flags {_DRM_RESTRICTED=1,_DRM_READ_ONLY=2,_DRM_LOCKED=4,_DRM_KERNEL=8,_DRM_WRITE_COMBINING=16,_DRM_CONTAINS_LOCK=32,_DRM_REMOVABLE=64,_DRM_DRIVER=128}
#[repr(C)] pub struct drm_ctx_priv_map { pub ctx_id:u32,pub handle:*mut core::ffi::c_void }
#[repr(C)] pub struct drm_map { pub offset:usize,pub size:usize,pub type_:drm_map_type,pub flags:drm_map_flags,pub handle:*mut core::ffi::c_void,pub mtrr:i32 }
#[repr(C)] pub struct drm_client { pub idx:i32,pub auth:i32,pub pid:usize,pub uid:usize,pub magic:usize,pub iocs:usize }
#[repr(C)] #[derive(Copy,Clone)] pub enum drm_stat_type {_DRM_STAT_LOCK,_DRM_STAT_OPENS,_DRM_STAT_CLOSES,_DRM_STAT_IOCTLS,_DRM_STAT_LOCKS,_DRM_STAT_UNLOCKS,_DRM_STAT_VALUE,_DRM_STAT_BYTE,_DRM_STAT_COUNT,_DRM_STAT_IRQ,_DRM_STAT_PRIMARY,_DRM_STAT_SECONDARY,_DRM_STAT_DMA,_DRM_STAT_SPECIAL,_DRM_STAT_MISSED}
#[repr(C)] pub struct drm_stats_data { pub value:usize,pub type_:drm_stat_type }
#[repr(C)] pub struct drm_stats { pub count:usize,pub data:[drm_stats_data;15] }
#[repr(C)] #[derive(Copy,Clone)] pub enum drm_lock_flags {_DRM_LOCK_READY=1,_DRM_LOCK_QUIESCENT=2,_DRM_LOCK_FLUSH=4,_DRM_LOCK_FLUSH_ALL=8,_DRM_HALT_ALL_QUEUES=16,_DRM_HALT_CUR_QUEUES=32}
#[repr(C)] pub struct drm_lock { pub context:i32,pub flags:drm_lock_flags }
#[repr(C)] #[derive(Copy,Clone)] pub enum drm_dma_flags {_DRM_DMA_BLOCK=1,_DRM_DMA_WHILE_LOCKED=2,_DRM_DMA_PRIORITY=4,_DRM_DMA_WAIT=16,_DRM_DMA_SMALLER_OK=32,_DRM_DMA_LARGER_OK=64}
#[repr(C)] pub struct drm_buf_desc { pub count:i32,pub size:i32,pub low_mark:i32,pub high_mark:i32,pub flags:i32,pub agp_start:usize }
#[repr(C)] pub struct drm_buf_info { pub count:i32,pub list:*mut drm_buf_desc }
#[repr(C)] pub struct drm_buf_free { pub count:i32,pub list:*mut i32 }
#[repr(C)] pub struct drm_buf_pub { pub idx:i32,pub total:i32,pub used:i32,pub address:*mut core::ffi::c_void }
#[repr(C)] pub struct drm_buf_map { pub count:i32,pub virtual_:*mut core::ffi::c_void,pub list:*mut drm_buf_pub }
#[repr(C)] pub struct drm_dma { pub context:i32,pub send_count:i32,pub send_indices:*mut i32,pub send_sizes:*mut i32,pub flags:drm_dma_flags,pub request_count:i32,pub request_size:i32,pub request_indices:*mut i32,pub request_sizes:*mut i32,pub granted_count:i32 }
#[repr(C)] #[derive(Copy,Clone)] pub enum drm_ctx_flags {_DRM_CONTEXT_PRESERVED=1,_DRM_CONTEXT_2DONLY=2}
#[repr(C)] pub struct drm_ctx { pub handle:drm_context_t,pub flags:drm_ctx_flags }
#[repr(C)] pub struct drm_ctx_res { pub count:i32,pub contexts:*mut drm_ctx }
#[repr(C)] pub struct drm_draw { pub handle:drm_drawable_t }
pub const DRM_DRAWABLE_CLIPRECTS:u32=0;
#[repr(C)] pub struct drm_update_draw { pub handle:drm_drawable_t,pub type_:u32,pub num:u32,pub data:u64 }
#[repr(C)] pub struct drm_auth { pub magic:drm_magic_t }
#[repr(C)] pub struct drm_irq_busid { pub irq:i32,pub busnum:i32,pub devnum:i32,pub funcnum:i32 }
#[repr(C)] #[derive(Copy,Clone)] pub enum drm_vblank_seq_type {_DRM_VBLANK_ABSOLUTE=0,_DRM_VBLANK_RELATIVE=1,_DRM_VBLANK_HIGH_CRTC_MASK=0x3e,_DRM_VBLANK_EVENT=0x4000000,_DRM_VBLANK_FLIP=0x8000000,_DRM_VBLANK_NEXTONMISS=0x10000000,_DRM_VBLANK_SECONDARY=0x20000000,_DRM_VBLANK_SIGNAL=0x40000000}
pub const _DRM_VBLANK_HIGH_CRTC_SHIFT:u32=1; pub const _DRM_VBLANK_TYPES_MASK:u32=1; pub const _DRM_VBLANK_FLAGS_MASK:u32=0x70000000;
#[repr(C)] pub struct drm_wait_vblank_request { pub type_:drm_vblank_seq_type,pub sequence:u32,pub signal:usize }
#[repr(C)] pub struct drm_wait_vblank_reply { pub type_:drm_vblank_seq_type,pub sequence:u32,pub tval_sec:isize,pub tval_usec:isize }
#[repr(C)] pub union drm_wait_vblank { pub request:drm_wait_vblank_request,pub reply:drm_wait_vblank_reply }
#[repr(C)] pub struct drm_modeset_ctl { pub crtc:__u32,pub cmd:__u32 }
#[repr(C)] pub struct drm_agp_mode { pub mode:usize }
#[repr(C)] pub struct drm_agp_buffer { pub size:usize,pub handle:usize,pub type_:usize,pub physical:usize }
#[repr(C)] pub struct drm_agp_binding { pub handle:usize,pub offset:usize }
#[repr(C)] pub struct drm_agp_info { pub agp_version_major:i32,pub agp_version_minor:i32,pub mode:usize,pub aperture_base:usize,pub aperture_size:usize,pub memory_allowed:usize,pub memory_used:usize,pub id_vendor:u16,pub id_device:u16 }
#[repr(C)] pub struct drm_scatter_gather { pub size:usize,pub handle:usize }
#[repr(C)] pub struct drm_set_version { pub drm_di_major:i32,pub drm_di_minor:i32,pub drm_dd_major:i32,pub drm_dd_minor:i32 }
#[repr(C)] pub struct drm_gem_close { pub handle:__u32,pub pad:__u32 }
#[repr(C)] pub struct drm_gem_flink { pub handle:__u32,pub name:__u32 }
#[repr(C)] pub struct drm_gem_open { pub name:__u32,pub handle:__u32,pub size:__u64 }
#[repr(C)] pub struct drm_gem_change_handle { pub handle:__u32,pub new_handle:__u32 }

pub const DRM_CAP_DUMB_BUFFER:u32=1; pub const DRM_CAP_VBLANK_HIGH_CRTC:u32=2; pub const DRM_CAP_DUMB_PREFERRED_DEPTH:u32=3; pub const DRM_CAP_DUMB_PREFER_SHADOW:u32=4; pub const DRM_CAP_PRIME:u32=5; pub const DRM_PRIME_CAP_IMPORT:u32=1; pub const DRM_PRIME_CAP_EXPORT:u32=2; pub const DRM_CAP_TIMESTAMP_MONOTONIC:u32=6; pub const DRM_CAP_ASYNC_PAGE_FLIP:u32=7; pub const DRM_CAP_CURSOR_WIDTH:u32=8; pub const DRM_CAP_CURSOR_HEIGHT:u32=9; pub const DRM_CAP_ADDFB2_MODIFIERS:u32=0x10; pub const DRM_CAP_PAGE_FLIP_TARGET:u32=0x11; pub const DRM_CAP_CRTC_IN_VBLANK_EVENT:u32=0x12; pub const DRM_CAP_SYNCOBJ:u32=0x13; pub const DRM_CAP_SYNCOBJ_TIMELINE:u32=0x14; pub const DRM_CAP_ATOMIC_ASYNC_PAGE_FLIP:u32=0x15;
#[repr(C)] pub struct drm_get_cap { pub capability:__u64,pub value:__u64 }
pub const DRM_CLIENT_CAP_STEREO_3D:u32=1; pub const DRM_CLIENT_CAP_UNIVERSAL_PLANES:u32=2; pub const DRM_CLIENT_CAP_ATOMIC:u32=3; pub const DRM_CLIENT_CAP_ASPECT_RATIO:u32=4; pub const DRM_CLIENT_CAP_WRITEBACK_CONNECTORS:u32=5; pub const DRM_CLIENT_CAP_CURSOR_PLANE_HOTSPOT:u32=6; pub const DRM_CLIENT_CAP_PLANE_COLOR_PIPELINE:u32=7;
#[repr(C)] pub struct drm_set_client_cap { pub capability:__u64,pub value:__u64 }
pub const DRM_RDWR:u32=0x2; pub const DRM_CLOEXEC:u32=0x80000;
#[repr(C)] pub struct drm_prime_handle { pub handle:__u32,pub flags:__u32,pub fd:__s32 }
#[repr(C)] pub struct drm_syncobj_create { pub handle:__u32,pub flags:__u32 }
pub const DRM_SYNCOBJ_CREATE_SIGNALED:u32=1;
#[repr(C)] pub struct drm_syncobj_destroy { pub handle:__u32,pub pad:__u32 }
pub const DRM_SYNCOBJ_FD_TO_HANDLE_FLAGS_IMPORT_SYNC_FILE:u32=1; pub const DRM_SYNCOBJ_FD_TO_HANDLE_FLAGS_TIMELINE:u32=2; pub const DRM_SYNCOBJ_HANDLE_TO_FD_FLAGS_EXPORT_SYNC_FILE:u32=1; pub const DRM_SYNCOBJ_HANDLE_TO_FD_FLAGS_TIMELINE:u32=2;
#[repr(C)] pub struct drm_syncobj_handle { pub handle:__u32,pub flags:__u32,pub fd:__s32,pub pad:__u32,pub point:__u64 }
#[repr(C)] pub struct drm_syncobj_transfer { pub src_handle:__u32,pub dst_handle:__u32,pub src_point:__u64,pub dst_point:__u64,pub flags:__u32,pub pad:__u32 }
pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_ALL:u32=1; pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT:u32=2; pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_AVAILABLE:u32=4; pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_DEADLINE:u32=8;
#[repr(C)] pub struct drm_syncobj_wait { pub handles:__u64,pub timeout_nsec:__s64,pub count_handles:__u32,pub flags:__u32,pub first_signaled:__u32,pub pad:__u32,pub deadline_nsec:__u64 }
#[repr(C)] pub struct drm_syncobj_timeline_wait { pub handles:__u64,pub points:__u64,pub timeout_nsec:__s64,pub count_handles:__u32,pub flags:__u32,pub first_signaled:__u32,pub pad:__u32,pub deadline_nsec:__u64 }
#[repr(C)] pub struct drm_syncobj_eventfd { pub handle:__u32,pub flags:__u32,pub point:__u64,pub fd:__s32,pub pad:__u32 }
#[repr(C)] pub struct drm_syncobj_array { pub handles:__u64,pub count_handles:__u32,pub pad:__u32 }
pub const DRM_SYNCOBJ_QUERY_FLAGS_LAST_SUBMITTED:u32=1;
#[repr(C)] pub struct drm_syncobj_timeline_array { pub handles:__u64,pub points:__u64,pub count_handles:__u32,pub flags:__u32 }
#[repr(C)] pub struct drm_crtc_get_sequence { pub crtc_id:__u32,pub active:__u32,pub sequence:__u64,pub sequence_ns:__s64 }
pub const DRM_CRTC_SEQUENCE_RELATIVE:u32=1; pub const DRM_CRTC_SEQUENCE_NEXT_ON_MISS:u32=2;
#[repr(C)] pub struct drm_crtc_queue_sequence { pub crtc_id:__u32,pub flags:__u32,pub sequence:__u64,pub user_data:__u64 }
pub const DRM_CLIENT_NAME_MAX_LEN:u32=64;
#[repr(C)] pub struct drm_set_client_name { pub name_len:__u64,pub name:__u64 }
#[repr(C)] pub struct drm_event { pub type_:__u32,pub length:__u32 }
pub const DRM_EVENT_VBLANK:u32=1; pub const DRM_EVENT_FLIP_COMPLETE:u32=2; pub const DRM_EVENT_CRTC_SEQUENCE:u32=3;
#[repr(C)] pub struct drm_event_vblank { pub base:drm_event,pub user_data:__u64,pub tv_sec:__u32,pub tv_usec:__u32,pub sequence:__u32,pub crtc_id:__u32 }
#[repr(C)] pub struct drm_event_crtc_sequence { pub base:drm_event,pub user_data:__u64,pub time_ns:__s64,pub sequence:__u64 }
pub const DRM_COMMAND_BASE:u32=0x40; pub const DRM_COMMAND_END:u32=0xA0;
/* ioctl request-number macros and declarations depending on drm_mode.h/ioctl
   encoding are preserved as external build-time dependencies. */
pub type drm_clip_rect_t=drm_clip_rect; pub type drm_drawable_info_t=drm_drawable_info; pub type drm_tex_region_t=drm_tex_region; pub type drm_hw_lock_t=drm_hw_lock;
pub type drm_version_t=drm_version; pub type drm_unique_t=drm_unique; pub type drm_list_t=drm_list; pub type drm_block_t=drm_block; pub type drm_control_t=drm_control;
pub type drm_map_type_t=drm_map_type; pub type drm_map_flags_t=drm_map_flags; pub type drm_ctx_priv_map_t=drm_ctx_priv_map; pub type drm_map_t=drm_map; pub type drm_client_t=drm_client;
pub type drm_stat_type_t=drm_stat_type; pub type drm_stats_t=drm_stats; pub type drm_lock_flags_t=drm_lock_flags; pub type drm_lock_t=drm_lock; pub type drm_dma_flags_t=drm_dma_flags;
pub type drm_buf_desc_t=drm_buf_desc; pub type drm_buf_info_t=drm_buf_info; pub type drm_buf_free_t=drm_buf_free; pub type drm_buf_pub_t=drm_buf_pub; pub type drm_buf_map_t=drm_buf_map; pub type drm_dma_t=drm_dma;
pub type drm_wait_vblank_t=drm_wait_vblank; pub type drm_agp_mode_t=drm_agp_mode; pub type drm_ctx_flags_t=drm_ctx_flags; pub type drm_ctx_t=drm_ctx; pub type drm_ctx_res_t=drm_ctx_res; pub type drm_draw_t=drm_draw; pub type drm_update_draw_t=drm_update_draw; pub type drm_auth_t=drm_auth; pub type drm_irq_busid_t=drm_irq_busid; pub type drm_vblank_seq_type_t=drm_vblank_seq_type;
pub type drm_agp_buffer_t=drm_agp_buffer; pub type drm_agp_binding_t=drm_agp_binding; pub type drm_agp_info_t=drm_agp_info; pub type drm_scatter_gather_t=drm_scatter_gather; pub type drm_set_version_t=drm_set_version;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
