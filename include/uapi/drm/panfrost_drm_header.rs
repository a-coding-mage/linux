/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2014-2018 Broadcom
 * Copyright © 2019 Collabora ltd.
 */

// Dependency: the C header includes "drm.h" for DRM_COMMAND_BASE and ioctl
// encoding helpers. Those names are intentionally left as external dependencies.

pub const DRM_PANFROST_SUBMIT: u32 = 0x00;
pub const DRM_PANFROST_WAIT_BO: u32 = 0x01;
pub const DRM_PANFROST_CREATE_BO: u32 = 0x02;
pub const DRM_PANFROST_MMAP_BO: u32 = 0x03;
pub const DRM_PANFROST_GET_PARAM: u32 = 0x04;
pub const DRM_PANFROST_GET_BO_OFFSET: u32 = 0x05;
pub const DRM_PANFROST_PERFCNT_ENABLE: u32 = 0x06;
pub const DRM_PANFROST_PERFCNT_DUMP: u32 = 0x07;
pub const DRM_PANFROST_MADVISE: u32 = 0x08;
pub const DRM_PANFROST_SET_LABEL_BO: u32 = 0x09;
pub const DRM_PANFROST_JM_CTX_CREATE: u32 = 0x0a;
pub const DRM_PANFROST_JM_CTX_DESTROY: u32 = 0x0b;
pub const DRM_PANFROST_SYNC_BO: u32 = 0x0c;
pub const DRM_PANFROST_QUERY_BO_INFO: u32 = 0x0d;

/* Unstable ioctl(s): only exposed when unsafe unstable_ioctls is enabled. */

pub const PANFROST_JD_REQ_FS: u32 = 1 << 0;
pub const PANFROST_JD_REQ_CYCLE_COUNT: u32 = 1 << 1;

#[repr(C)]
pub struct drm_panfrost_submit {
    pub jc: u64,
    pub in_syncs: u64,
    pub in_sync_count: u32,
    pub out_sync: u32,
    pub bo_handles: u64,
    pub bo_handle_count: u32,
    pub requirements: u32,
    pub jm_ctx_handle: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct drm_panfrost_wait_bo { pub handle: u32, pub pad: u32, pub timeout_ns: i64 }

pub const PANFROST_BO_NOEXEC: u32 = 1;
pub const PANFROST_BO_HEAP: u32 = 2;
pub const PANFROST_BO_WB_MMAP: u32 = 4;

#[repr(C)]
pub struct drm_panfrost_create_bo { pub size: u32, pub flags: u32, pub handle: u32, pub pad: u32, pub offset: u64 }

#[repr(C)]
pub struct drm_panfrost_mmap_bo { pub handle: u32, pub flags: u32, pub offset: u64 }

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum drm_panfrost_param {
    DRM_PANFROST_PARAM_GPU_PROD_ID,
    DRM_PANFROST_PARAM_GPU_REVISION,
    DRM_PANFROST_PARAM_SHADER_PRESENT,
    DRM_PANFROST_PARAM_TILER_PRESENT,
    DRM_PANFROST_PARAM_L2_PRESENT,
    DRM_PANFROST_PARAM_STACK_PRESENT,
    DRM_PANFROST_PARAM_AS_PRESENT,
    DRM_PANFROST_PARAM_JS_PRESENT,
    DRM_PANFROST_PARAM_L2_FEATURES,
    DRM_PANFROST_PARAM_CORE_FEATURES,
    DRM_PANFROST_PARAM_TILER_FEATURES,
    DRM_PANFROST_PARAM_MEM_FEATURES,
    DRM_PANFROST_PARAM_MMU_FEATURES,
    DRM_PANFROST_PARAM_THREAD_FEATURES,
    DRM_PANFROST_PARAM_MAX_THREADS,
    DRM_PANFROST_PARAM_THREAD_MAX_WORKGROUP_SZ,
    DRM_PANFROST_PARAM_THREAD_MAX_BARRIER_SZ,
    DRM_PANFROST_PARAM_COHERENCY_FEATURES,
    DRM_PANFROST_PARAM_TEXTURE_FEATURES0,
    DRM_PANFROST_PARAM_TEXTURE_FEATURES1,
    DRM_PANFROST_PARAM_TEXTURE_FEATURES2,
    DRM_PANFROST_PARAM_TEXTURE_FEATURES3,
    DRM_PANFROST_PARAM_JS_FEATURES0,
    DRM_PANFROST_PARAM_JS_FEATURES1,
    DRM_PANFROST_PARAM_JS_FEATURES2,
    DRM_PANFROST_PARAM_JS_FEATURES3,
    DRM_PANFROST_PARAM_JS_FEATURES4,
    DRM_PANFROST_PARAM_JS_FEATURES5,
    DRM_PANFROST_PARAM_JS_FEATURES6,
    DRM_PANFROST_PARAM_JS_FEATURES7,
    DRM_PANFROST_PARAM_JS_FEATURES8,
    DRM_PANFROST_PARAM_JS_FEATURES9,
    DRM_PANFROST_PARAM_JS_FEATURES10,
    DRM_PANFROST_PARAM_JS_FEATURES11,
    DRM_PANFROST_PARAM_JS_FEATURES12,
    DRM_PANFROST_PARAM_JS_FEATURES13,
    DRM_PANFROST_PARAM_JS_FEATURES14,
    DRM_PANFROST_PARAM_JS_FEATURES15,
    DRM_PANFROST_PARAM_NR_CORE_GROUPS,
    DRM_PANFROST_PARAM_THREAD_TLS_ALLOC,
    DRM_PANFROST_PARAM_AFBC_FEATURES,
    DRM_PANFROST_PARAM_SYSTEM_TIMESTAMP,
    DRM_PANFROST_PARAM_SYSTEM_TIMESTAMP_FREQUENCY,
    DRM_PANFROST_PARAM_ALLOWED_JM_CTX_PRIORITIES,
    DRM_PANFROST_PARAM_SELECTED_COHERENCY,
}

#[repr(u32)]
pub enum drm_panfrost_gpu_coherency { DRM_PANFROST_GPU_COHERENCY_ACE_LITE = 0, DRM_PANFROST_GPU_COHERENCY_ACE = 1, DRM_PANFROST_GPU_COHERENCY_NONE = 31 }

#[repr(C)] pub struct drm_panfrost_get_param { pub param: u32, pub pad: u32, pub value: u64 }
#[repr(C)] pub struct drm_panfrost_get_bo_offset { pub handle: u32, pub pad: u32, pub offset: u64 }
#[repr(C)] pub struct drm_panfrost_perfcnt_enable { pub enable: u32, pub counterset: u32 }
#[repr(C)] pub struct drm_panfrost_perfcnt_dump { pub buf_ptr: u64 }

pub const PANFROST_MADV_WILLNEED: u32 = 0;
pub const PANFROST_MADV_DONTNEED: u32 = 1;
#[repr(C)] pub struct drm_panfrost_madvise { pub handle: u32, pub madv: u32, pub retained: u32 }
#[repr(C)] pub struct drm_panfrost_set_label_bo { pub handle: u32, pub pad: u32, pub label: u64 }
pub const PANFROST_BO_SYNC_CPU_CACHE_FLUSH: u32 = 0;
pub const PANFROST_BO_SYNC_CPU_CACHE_FLUSH_AND_INVALIDATE: u32 = 1;
#[repr(C)] pub struct drm_panfrost_bo_sync_op { pub handle: u32, pub r#type: u32, pub offset: u32, pub size: u32 }
#[repr(C)] pub struct drm_panfrost_sync_bo { pub ops: u64, pub op_count: u32, pub pad: u32 }
pub const DRM_PANFROST_BO_IS_IMPORTED: u32 = 1 << 0;
#[repr(C)] pub struct drm_panfrost_query_bo_info { pub handle: u32, pub extra_flags: u32, pub create_flags: u32, pub pad: u32 }

pub const PANFROSTDUMP_MAJOR: u32 = 1;
pub const PANFROSTDUMP_MINOR: u32 = 0;
pub const PANFROSTDUMP_MAGIC: u32 = 0x464E4150;
pub const PANFROSTDUMP_BUF_REG: u32 = 0;
pub const PANFROSTDUMP_BUF_BOMAP: u32 = PANFROSTDUMP_BUF_REG + 1;
pub const PANFROSTDUMP_BUF_BO: u32 = PANFROSTDUMP_BUF_BOMAP + 1;
pub const PANFROSTDUMP_BUF_TRAILER: u32 = PANFROSTDUMP_BUF_BO + 1;

#[repr(C)]
pub union panfrost_dump_object_header__bindgen_ty_1 {
    pub reghdr: panfrost_dump_object_header__bindgen_ty_1__bindgen_ty_1,
    pub bomap: panfrost_dump_object_header__bindgen_ty_1__bindgen_ty_2,
    pub sizer: [u32; 496],
}
#[repr(C)] pub struct panfrost_dump_object_header__bindgen_ty_1__bindgen_ty_1 { pub jc: u64, pub gpu_id: u32, pub major: u32, pub minor: u32, pub nbos: u64 }
#[repr(C)] pub struct panfrost_dump_object_header__bindgen_ty_1__bindgen_ty_2 { pub valid: u32, pub iova: u64, pub data: [u32; 2] }
#[repr(C)] pub struct panfrost_dump_object_header { pub magic: u32, pub r#type: u32, pub file_size: u32, pub file_offset: u32, pub __bindgen_anon_1: panfrost_dump_object_header__bindgen_ty_1 }
#[repr(C)] pub struct panfrost_dump_registers { pub reg: u32, pub value: u32 }

#[repr(u32)]
pub enum drm_panfrost_jm_ctx_priority { PANFROST_JM_CTX_PRIORITY_LOW = 0, PANFROST_JM_CTX_PRIORITY_MEDIUM, PANFROST_JM_CTX_PRIORITY_HIGH }
#[repr(C)] pub struct drm_panfrost_jm_ctx_create { pub handle: u32, pub priority: u32 }
#[repr(C)] pub struct drm_panfrost_jm_ctx_destroy { pub handle: u32, pub pad: u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
