/* SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT */
/* Copyright 2017-2018 Qiang Yu <yuq825@gmail.com> */

// Translated from lima_drm.h. DRM ioctl helpers and integer aliases are
// supplied by the surrounding DRM bindings.

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum drm_lima_param_gpu_id {
    DRM_LIMA_PARAM_GPU_ID_UNKNOWN,
    DRM_LIMA_PARAM_GPU_ID_MALI400,
    DRM_LIMA_PARAM_GPU_ID_MALI450,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum drm_lima_param {
    DRM_LIMA_PARAM_GPU_ID,
    DRM_LIMA_PARAM_NUM_PP,
    DRM_LIMA_PARAM_GP_VERSION,
    DRM_LIMA_PARAM_PP_VERSION,
}

/// get various information of the GPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_get_param {
    pub param: u32, // in, value in enum drm_lima_param
    pub pad: u32,   // pad, must be zero
    pub value: u64, // out, parameter value
}

pub const LIMA_BO_FLAG_HEAP: u32 = 1 << 0;

/// create a buffer for used by GPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gem_create {
    pub size: u32,
    pub flags: u32,
    pub handle: u32,
    pub pad: u32,
}

/// get information of a buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gem_info {
    pub handle: u32,
    pub va: u32,
    pub offset: u64,
}

pub const LIMA_SUBMIT_BO_READ: u32 = 0x01;
pub const LIMA_SUBMIT_BO_WRITE: u32 = 0x02;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gem_submit_bo {
    pub handle: u32,
    pub flags: u32,
}

pub const LIMA_GP_FRAME_REG_NUM: usize = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gp_frame {
    pub frame: [u32; LIMA_GP_FRAME_REG_NUM],
}

pub const LIMA_PP_FRAME_REG_NUM: usize = 23;
pub const LIMA_PP_WB_REG_NUM: usize = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_m400_pp_frame {
    pub frame: [u32; LIMA_PP_FRAME_REG_NUM],
    pub num_pp: u32,
    pub wb: [u32; 3 * LIMA_PP_WB_REG_NUM],
    pub plbu_array_address: [u32; 4],
    pub fragment_stack_address: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_lima_m450_pp_frame__bindgen_ty_1 {
    pub plbu_array_address: [u32; 8],
    pub dlbu_regs: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_m450_pp_frame {
    pub frame: [u32; LIMA_PP_FRAME_REG_NUM],
    pub num_pp: u32,
    pub wb: [u32; 3 * LIMA_PP_WB_REG_NUM],
    pub use_dlbu: u32,
    pub _pad: u32,
    pub __bindgen_anon_1: drm_lima_m450_pp_frame__bindgen_ty_1,
    pub fragment_stack_address: [u32; 8],
}

pub const LIMA_PIPE_GP: u32 = 0x00;
pub const LIMA_PIPE_PP: u32 = 0x01;
pub const LIMA_SUBMIT_FLAG_EXPLICIT_FENCE: u32 = 1 << 0;

/// submit a task to GPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gem_submit {
    pub ctx: u32,
    pub pipe: u32,
    pub nr_bos: u32,
    pub frame_size: u32,
    pub bos: u64,
    pub frame: u64,
    pub flags: u32,
    pub out_sync: u32,
    pub in_sync: [u32; 2],
}

pub const LIMA_GEM_WAIT_READ: u32 = 0x01;
pub const LIMA_GEM_WAIT_WRITE: u32 = 0x02;

/// wait pending GPU task finish of a buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gem_wait {
    pub handle: u32,
    pub op: u32,
    pub timeout_ns: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_ctx_create { pub id: u32, pub _pad: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_ctx_free { pub id: u32, pub _pad: u32 }

pub const DRM_LIMA_GET_PARAM: u32 = 0x00;
pub const DRM_LIMA_GEM_CREATE: u32 = 0x01;
pub const DRM_LIMA_GEM_INFO: u32 = 0x02;
pub const DRM_LIMA_GEM_SUBMIT: u32 = 0x03;
pub const DRM_LIMA_GEM_WAIT: u32 = 0x04;
pub const DRM_LIMA_CTX_CREATE: u32 = 0x05;
pub const DRM_LIMA_CTX_FREE: u32 = 0x06;

// These ioctl values depend on DRM_IOWR/DRM_IOW/DRM_IOR and DRM_COMMAND_BASE.
// Preserve the source-level declarations for the surrounding DRM bindings.
pub const DRM_IOCTL_LIMA_GET_PARAM = DRM_IOWR!(DRM_COMMAND_BASE + DRM_LIMA_GET_PARAM, drm_lima_get_param);
pub const DRM_IOCTL_LIMA_GEM_CREATE = DRM_IOWR!(DRM_COMMAND_BASE + DRM_LIMA_GEM_CREATE, drm_lima_gem_create);
pub const DRM_IOCTL_LIMA_GEM_INFO = DRM_IOWR!(DRM_COMMAND_BASE + DRM_LIMA_GEM_INFO, drm_lima_gem_info);
pub const DRM_IOCTL_LIMA_GEM_SUBMIT = DRM_IOW!(DRM_COMMAND_BASE + DRM_LIMA_GEM_SUBMIT, drm_lima_gem_submit);
pub const DRM_IOCTL_LIMA_GEM_WAIT = DRM_IOW!(DRM_COMMAND_BASE + DRM_LIMA_GEM_WAIT, drm_lima_gem_wait);
pub const DRM_IOCTL_LIMA_CTX_CREATE = DRM_IOR!(DRM_COMMAND_BASE + DRM_LIMA_CTX_CREATE, drm_lima_ctx_create);
pub const DRM_IOCTL_LIMA_CTX_FREE = DRM_IOW!(DRM_COMMAND_BASE + DRM_LIMA_CTX_FREE, drm_lima_ctx_free);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
