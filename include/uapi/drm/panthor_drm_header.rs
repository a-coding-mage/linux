/* SPDX-License-Identifier: MIT */
/* Copyright (C) 2023 Collabora ltd. */
// Rust translation of panthor_drm.h. Types such as __u32/__u64 are represented
// by their fixed-width Rust equivalents; ioctl encoding is supplied by drm.h.

pub const DRM_PANTHOR_USER_MMIO_OFFSET_32BIT: u64 = 1u64 << 43;
pub const DRM_PANTHOR_USER_MMIO_OFFSET_64BIT: u64 = 1u64 << 56;
// C selects this from sizeof(unsigned long); this is the 64-bit translation.
pub const DRM_PANTHOR_USER_MMIO_OFFSET: u64 = DRM_PANTHOR_USER_MMIO_OFFSET_64BIT;
pub const DRM_PANTHOR_USER_FLUSH_ID_MMIO_OFFSET: u64 = DRM_PANTHOR_USER_MMIO_OFFSET | 0;

#[repr(u32)]
pub enum drm_panthor_ioctl_id {
    DRM_PANTHOR_DEV_QUERY = 0,
    DRM_PANTHOR_VM_CREATE,
    DRM_PANTHOR_VM_DESTROY,
    DRM_PANTHOR_VM_BIND,
    DRM_PANTHOR_VM_GET_STATE,
    DRM_PANTHOR_BO_CREATE,
    DRM_PANTHOR_BO_MMAP_OFFSET,
    DRM_PANTHOR_GROUP_CREATE,
    DRM_PANTHOR_GROUP_DESTROY,
    DRM_PANTHOR_GROUP_SUBMIT,
    DRM_PANTHOR_GROUP_GET_STATE,
    DRM_PANTHOR_TILER_HEAP_CREATE,
    DRM_PANTHOR_TILER_HEAP_DESTROY,
    DRM_PANTHOR_BO_SET_LABEL,
    DRM_PANTHOR_SET_USER_MMIO_OFFSET,
    DRM_PANTHOR_BO_SYNC,
    DRM_PANTHOR_BO_QUERY_INFO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_obj_array { pub stride: u32, pub count: u32, pub array: u64 }

#[macro_export]
macro_rules! DRM_PANTHOR_OBJ_ARRAY { ($cnt:expr, $ptr:expr) => { $crate::drm_panthor_obj_array { stride: core::mem::size_of_val(&$ptr[0]) as u32, count: $cnt, array: ($ptr as *const _ as usize) as u64 } }; }

pub const DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_MASK: u32 = 0xff;
pub const DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_SYNCOBJ: u32 = 0;
pub const DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_TIMELINE_SYNCOBJ: u32 = 1;
pub const DRM_PANTHOR_SYNC_OP_WAIT: u32 = 0;
pub const DRM_PANTHOR_SYNC_OP_SIGNAL: u32 = 1u32 << 31;
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_sync_op { pub flags: u32, pub handle: u32, pub timeline_value: u64 }

#[repr(u32)] pub enum drm_panthor_dev_query_type { DRM_PANTHOR_DEV_QUERY_GPU_INFO=0, DRM_PANTHOR_DEV_QUERY_CSIF_INFO, DRM_PANTHOR_DEV_QUERY_TIMESTAMP_INFO, DRM_PANTHOR_DEV_QUERY_GROUP_PRIORITIES_INFO, DRM_PANTHOR_DEV_QUERY_MMU_INFO }
#[repr(u32)] pub enum drm_panthor_gpu_coherency { DRM_PANTHOR_GPU_COHERENCY_ACE_LITE=0, DRM_PANTHOR_GPU_COHERENCY_ACE=1, DRM_PANTHOR_GPU_COHERENCY_NONE=31 }

pub const DRM_PANTHOR_ARCH_MAJOR: fn(u32)->u32 = |x| x >> 28;
pub const DRM_PANTHOR_ARCH_MINOR: fn(u32)->u32 = |x| (x >> 24) & 0xf;
pub const DRM_PANTHOR_ARCH_REV: fn(u32)->u32 = |x| (x >> 20) & 0xf;
pub const DRM_PANTHOR_PRODUCT_MAJOR: fn(u32)->u32 = |x| (x >> 16) & 0xf;
pub const DRM_PANTHOR_VERSION_MAJOR: fn(u32)->u32 = |x| (x >> 12) & 0xf;
pub const DRM_PANTHOR_VERSION_MINOR: fn(u32)->u32 = |x| (x >> 4) & 0xff;
pub const DRM_PANTHOR_VERSION_STATUS: fn(u32)->u32 = |x| x & 0xf;
pub const DRM_PANTHOR_CSHW_MAJOR: fn(u32)->u32 = |x| (x >> 26) & 0x3f;
pub const DRM_PANTHOR_CSHW_MINOR: fn(u32)->u32 = |x| (x >> 20) & 0x3f;
pub const DRM_PANTHOR_CSHW_REV: fn(u32)->u32 = |x| (x >> 16) & 0xf;
pub const DRM_PANTHOR_MCU_MAJOR: fn(u32)->u32 = |x| (x >> 10) & 0x3f;
pub const DRM_PANTHOR_MCU_MINOR: fn(u32)->u32 = |x| (x >> 4) & 0x3f;
pub const DRM_PANTHOR_MCU_REV: fn(u32)->u32 = |x| x & 0xf;
pub const DRM_PANTHOR_MMU_VA_BITS: fn(u32)->u32 = |x| x & 0xff;

#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_gpu_info { pub gpu_id:u32,pub gpu_rev:u32,pub csf_id:u32,pub l2_features:u32,pub tiler_features:u32,pub mem_features:u32,pub mmu_features:u32,pub thread_features:u32,pub max_threads:u32,pub thread_max_workgroup_size:u32,pub thread_max_barrier_size:u32,pub coherency_features:u32,pub texture_features:[u32;4],pub as_present:u32,pub selected_coherency:u32,pub shader_present:u64,pub l2_present:u64,pub tiler_present:u64,pub core_features:u32,pub pad:u32,pub gpu_features:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_csif_info { pub csg_slot_count:u32,pub cs_slot_count:u32,pub cs_reg_count:u32,pub scoreboard_slot_count:u32,pub unpreserved_cs_reg_count:u32,pub pad:u32 }
pub const DRM_PANTHOR_TIMESTAMP_GPU:u32=1<<0; pub const DRM_PANTHOR_TIMESTAMP_CPU_NONE:u32=0; pub const DRM_PANTHOR_TIMESTAMP_CPU_MONOTONIC:u32=1<<1; pub const DRM_PANTHOR_TIMESTAMP_CPU_MONOTONIC_RAW:u32=2<<1; pub const DRM_PANTHOR_TIMESTAMP_CPU_TYPE_MASK:u32=7<<1; pub const DRM_PANTHOR_TIMESTAMP_GPU_OFFSET:u32=1<<4; pub const DRM_PANTHOR_TIMESTAMP_GPU_CYCLE_COUNT:u32=1<<5; pub const DRM_PANTHOR_TIMESTAMP_FREQ:u32=1<<6; pub const DRM_PANTHOR_TIMESTAMP_DURATION:u32=1<<7;
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_timestamp_info { pub timestamp_frequency:u64,pub current_timestamp:u64,pub timestamp_offset:u64,pub flags:u32,pub duration_nsec:u32,pub cycle_count:u64,pub cpu_timestamp_sec:u64,pub cpu_timestamp_nsec:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_mmu_info { pub page_size_bitmap:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_group_priorities_info { pub allowed_mask:u8,pub pad:[u8;3] }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_dev_query { pub r#type:u32,pub size:u32,pub pointer:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_vm_create { pub flags:u32,pub id:u32,pub user_va_range:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_vm_destroy { pub id:u32,pub pad:u32 }

pub const DRM_PANTHOR_VM_BIND_OP_MAP_READONLY:u32=1<<0; pub const DRM_PANTHOR_VM_BIND_OP_MAP_NOEXEC:u32=1<<1; pub const DRM_PANTHOR_VM_BIND_OP_MAP_UNCACHED:u32=1<<2; pub const DRM_PANTHOR_VM_BIND_OP_MAP_SPARSE:u32=1<<3; pub const DRM_PANTHOR_VM_BIND_OP_TYPE_MASK:u32=0xf<<28; pub const DRM_PANTHOR_VM_BIND_OP_TYPE_MAP:u32=0; pub const DRM_PANTHOR_VM_BIND_OP_TYPE_UNMAP:u32=1<<28; pub const DRM_PANTHOR_VM_BIND_OP_TYPE_SYNC_ONLY:u32=2<<28;
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_vm_bind_op { pub flags:u32,pub bo_handle:u32,pub bo_offset:u64,pub va:u64,pub size:u64,pub syncs:drm_panthor_obj_array }
pub const DRM_PANTHOR_VM_BIND_ASYNC:u32=1<<0;
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_vm_bind { pub vm_id:u32,pub flags:u32,pub ops:drm_panthor_obj_array }
#[repr(u32)] pub enum drm_panthor_vm_state { DRM_PANTHOR_VM_STATE_USABLE=0, DRM_PANTHOR_VM_STATE_UNUSABLE }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_vm_get_state { pub vm_id:u32,pub state:u32 }
pub const DRM_PANTHOR_BO_NO_MMAP:u32=1<<0; pub const DRM_PANTHOR_BO_WB_MMAP:u32=1<<1;
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_bo_create { pub size:u64,pub flags:u32,pub exclusive_vm_id:u32,pub handle:u32,pub pad:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_bo_mmap_offset { pub handle:u32,pub pad:u32,pub offset:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_queue_create { pub priority:u8,pub pad:[u8;3],pub ringbuf_size:u32 }
#[repr(u32)] pub enum drm_panthor_group_priority { PANTHOR_GROUP_PRIORITY_LOW=0, PANTHOR_GROUP_PRIORITY_MEDIUM, PANTHOR_GROUP_PRIORITY_HIGH, PANTHOR_GROUP_PRIORITY_REALTIME }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_group_create { pub queues:drm_panthor_obj_array,pub max_compute_cores:u8,pub max_fragment_cores:u8,pub max_tiler_cores:u8,pub priority:u8,pub pad:u32,pub compute_core_mask:u64,pub fragment_core_mask:u64,pub tiler_core_mask:u64,pub vm_id:u32,pub group_handle:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_group_destroy { pub group_handle:u32,pub pad:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_queue_submit { pub queue_index:u32,pub stream_size:u32,pub stream_addr:u64,pub latest_flush:u32,pub pad:u32,pub syncs:drm_panthor_obj_array }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_group_submit { pub group_handle:u32,pub pad:u32,pub queue_submits:drm_panthor_obj_array }
pub const DRM_PANTHOR_GROUP_STATE_TIMEDOUT:u32=1<<0; pub const DRM_PANTHOR_GROUP_STATE_FATAL_FAULT:u32=1<<1; pub const DRM_PANTHOR_GROUP_STATE_INNOCENT:u32=1<<2;
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_group_get_state { pub group_handle:u32,pub state:u32,pub fatal_queues:u32,pub pad:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_tiler_heap_create { pub vm_id:u32,pub initial_chunk_count:u32,pub chunk_size:u32,pub max_chunks:u32,pub target_in_flight:u32,pub handle:u32,pub tiler_heap_ctx_gpu_va:u64,pub first_heap_chunk_gpu_va:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_tiler_heap_destroy { pub handle:u32,pub pad:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_bo_set_label { pub handle:u32,pub pad:u32,pub label:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_set_user_mmio_offset { pub offset:u64 }
#[repr(u32)] pub enum drm_panthor_bo_sync_op_type { DRM_PANTHOR_BO_SYNC_CPU_CACHE_FLUSH=0, DRM_PANTHOR_BO_SYNC_CPU_CACHE_FLUSH_AND_INVALIDATE=1 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_bo_sync_op { pub handle:u32,pub r#type:u32,pub offset:u64,pub size:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_bo_sync { pub ops:drm_panthor_obj_array }
pub const DRM_PANTHOR_BO_IS_IMPORTED:u32=1<<0;
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_panthor_bo_query_info { pub handle:u32,pub extra_flags:u32,pub create_flags:u32,pub pad:u32 }

// DRM_IOCTL_PANTHOR and the following ioctl constants depend on the platform
// DRM_IO/DRM_COMMAND_BASE definitions supplied by the translated drm.h.
// They are intentionally retained as dependency-bound declarations here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
