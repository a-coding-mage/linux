/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright (C) 2015 Etnaviv Project */

/* Rust translation of etnaviv_drm.h.  DRM_IOW/DRM_IOWR and DRM_COMMAND_BASE
 * are supplied by the corresponding DRM bindings. */

#[repr(C)]
pub struct drm_etnaviv_timespec { pub tv_sec: i64, pub tv_nsec: i64 }

pub const ETNAVIV_PARAM_GPU_MODEL: u32 = 0x01;
pub const ETNAVIV_PARAM_GPU_REVISION: u32 = 0x02;
pub const ETNAVIV_PARAM_GPU_FEATURES_0: u32 = 0x03;
pub const ETNAVIV_PARAM_GPU_FEATURES_1: u32 = 0x04;
pub const ETNAVIV_PARAM_GPU_FEATURES_2: u32 = 0x05;
pub const ETNAVIV_PARAM_GPU_FEATURES_3: u32 = 0x06;
pub const ETNAVIV_PARAM_GPU_FEATURES_4: u32 = 0x07;
pub const ETNAVIV_PARAM_GPU_FEATURES_5: u32 = 0x08;
pub const ETNAVIV_PARAM_GPU_FEATURES_6: u32 = 0x09;
pub const ETNAVIV_PARAM_GPU_FEATURES_7: u32 = 0x0a;
pub const ETNAVIV_PARAM_GPU_FEATURES_8: u32 = 0x0b;
pub const ETNAVIV_PARAM_GPU_FEATURES_9: u32 = 0x0c;
pub const ETNAVIV_PARAM_GPU_FEATURES_10: u32 = 0x0d;
pub const ETNAVIV_PARAM_GPU_FEATURES_11: u32 = 0x0e;
pub const ETNAVIV_PARAM_GPU_FEATURES_12: u32 = 0x0f;
pub const ETNAVIV_PARAM_GPU_STREAM_COUNT: u32 = 0x10;
pub const ETNAVIV_PARAM_GPU_REGISTER_MAX: u32 = 0x11;
pub const ETNAVIV_PARAM_GPU_THREAD_COUNT: u32 = 0x12;
pub const ETNAVIV_PARAM_GPU_VERTEX_CACHE_SIZE: u32 = 0x13;
pub const ETNAVIV_PARAM_GPU_SHADER_CORE_COUNT: u32 = 0x14;
pub const ETNAVIV_PARAM_GPU_PIXEL_PIPES: u32 = 0x15;
pub const ETNAVIV_PARAM_GPU_VERTEX_OUTPUT_BUFFER_SIZE: u32 = 0x16;
pub const ETNAVIV_PARAM_GPU_BUFFER_SIZE: u32 = 0x17;
pub const ETNAVIV_PARAM_GPU_INSTRUCTION_COUNT: u32 = 0x18;
pub const ETNAVIV_PARAM_GPU_NUM_CONSTANTS: u32 = 0x19;
pub const ETNAVIV_PARAM_GPU_NUM_VARYINGS: u32 = 0x1a;
pub const ETNAVIV_PARAM_SOFTPIN_START_ADDR: u32 = 0x1b;
pub const ETNAVIV_PARAM_GPU_PRODUCT_ID: u32 = 0x1c;
pub const ETNAVIV_PARAM_GPU_CUSTOMER_ID: u32 = 0x1d;
pub const ETNAVIV_PARAM_GPU_ECO_ID: u32 = 0x1e;
pub const ETNA_MAX_PIPES: u32 = 4;

#[repr(C)] pub struct drm_etnaviv_param { pub pipe: u32, pub param: u32, pub value: u64 }
pub const ETNA_BO_CACHE_MASK: u32 = 0x000f0000;
pub const ETNA_BO_CACHED: u32 = 0x00010000;
pub const ETNA_BO_WC: u32 = 0x00020000;
pub const ETNA_BO_UNCACHED: u32 = 0x00040000;
pub const ETNA_BO_FORCE_MMU: u32 = 0x00100000;
#[repr(C)] pub struct drm_etnaviv_gem_new { pub size: u64, pub flags: u32, pub handle: u32 }
#[repr(C)] pub struct drm_etnaviv_gem_info { pub handle: u32, pub pad: u32, pub offset: u64 }
pub const ETNA_PREP_READ: u32 = 0x01; pub const ETNA_PREP_WRITE: u32 = 0x02; pub const ETNA_PREP_NOSYNC: u32 = 0x04;
#[repr(C)] pub struct drm_etnaviv_gem_cpu_prep { pub handle: u32, pub op: u32, pub timeout: drm_etnaviv_timespec }
#[repr(C)] pub struct drm_etnaviv_gem_cpu_fini { pub handle: u32, pub flags: u32 }

#[repr(C)] pub struct drm_etnaviv_gem_submit_reloc { pub submit_offset: u32, pub reloc_idx: u32, pub reloc_offset: u64, pub flags: u32 }
pub const ETNA_SUBMIT_BO_READ: u32 = 0x0001; pub const ETNA_SUBMIT_BO_WRITE: u32 = 0x0002;
#[repr(C)] pub struct drm_etnaviv_gem_submit_bo { pub flags: u32, pub handle: u32, pub presumed: u64 }
pub const ETNA_PM_PROCESS_PRE: u32 = 0x0001; pub const ETNA_PM_PROCESS_POST: u32 = 0x0002;
#[repr(C)] pub struct drm_etnaviv_gem_submit_pmr { pub flags: u32, pub domain: u8, pub pad: u8, pub signal: u16, pub sequence: u32, pub read_offset: u32, pub read_idx: u32 }
pub const ETNA_SUBMIT_NO_IMPLICIT: u32 = 0x0001; pub const ETNA_SUBMIT_FENCE_FD_IN: u32 = 0x0002; pub const ETNA_SUBMIT_FENCE_FD_OUT: u32 = 0x0004; pub const ETNA_SUBMIT_SOFTPIN: u32 = 0x0008;
pub const ETNA_SUBMIT_FLAGS: u32 = ETNA_SUBMIT_NO_IMPLICIT | ETNA_SUBMIT_FENCE_FD_IN | ETNA_SUBMIT_FENCE_FD_OUT | ETNA_SUBMIT_SOFTPIN;
pub const ETNA_PIPE_3D: u32 = 0; pub const ETNA_PIPE_2D: u32 = 1; pub const ETNA_PIPE_VG: u32 = 2;
#[repr(C)] pub struct drm_etnaviv_gem_submit { pub fence: u32, pub pipe: u32, pub exec_state: u32, pub nr_bos: u32, pub nr_relocs: u32, pub stream_size: u32, pub bos: u64, pub relocs: u64, pub stream: u64, pub flags: u32, pub fence_fd: i32, pub pmrs: u64, pub nr_pmrs: u32, pub pad: u32 }
pub const ETNA_WAIT_NONBLOCK: u32 = 1;
#[repr(C)] pub struct drm_etnaviv_wait_fence { pub pipe: u32, pub fence: u32, pub flags: u32, pub pad: u32, pub timeout: drm_etnaviv_timespec }
pub const ETNA_USERPTR_READ: u32 = 1; pub const ETNA_USERPTR_WRITE: u32 = 2;
#[repr(C)] pub struct drm_etnaviv_gem_userptr { pub user_ptr: u64, pub user_size: u64, pub flags: u32, pub handle: u32 }
#[repr(C)] pub struct drm_etnaviv_gem_wait { pub pipe: u32, pub handle: u32, pub flags: u32, pub pad: u32, pub timeout: drm_etnaviv_timespec }
#[repr(C)] pub struct drm_etnaviv_pm_domain { pub pipe: u32, pub iter: u8, pub id: u8, pub nr_signals: u16, pub name: [std::os::raw::c_char; 64] }
#[repr(C)] pub struct drm_etnaviv_pm_signal { pub pipe: u32, pub domain: u8, pub pad: u8, pub iter: u16, pub id: u16, pub name: [std::os::raw::c_char; 64] }

pub const DRM_ETNAVIV_GET_PARAM: u32 = 0x00; pub const DRM_ETNAVIV_GEM_NEW: u32 = 0x02; pub const DRM_ETNAVIV_GEM_INFO: u32 = 0x03; pub const DRM_ETNAVIV_GEM_CPU_PREP: u32 = 0x04; pub const DRM_ETNAVIV_GEM_CPU_FINI: u32 = 0x05; pub const DRM_ETNAVIV_GEM_SUBMIT: u32 = 0x06; pub const DRM_ETNAVIV_WAIT_FENCE: u32 = 0x07; pub const DRM_ETNAVIV_GEM_USERPTR: u32 = 0x08; pub const DRM_ETNAVIV_GEM_WAIT: u32 = 0x09; pub const DRM_ETNAVIV_PM_QUERY_DOM: u32 = 0x0a; pub const DRM_ETNAVIV_PM_QUERY_SIG: u32 = 0x0b; pub const DRM_ETNAVIV_NUM_IOCTLS: u32 = 0x0c;

/* ioctl encodings depend on the external DRM_IOW/DRM_IOWR macros and are
 * intentionally preserved as source-level declarations. */
pub const DRM_IOCTL_ETNAVIV_GET_PARAM: u32 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_ETNAVIV_GET_PARAM, drm_etnaviv_param);
pub const DRM_IOCTL_ETNAVIV_GEM_NEW: u32 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_ETNAVIV_GEM_NEW, drm_etnaviv_gem_new);
pub const DRM_IOCTL_ETNAVIV_GEM_INFO: u32 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_ETNAVIV_GEM_INFO, drm_etnaviv_gem_info);
pub const DRM_IOCTL_ETNAVIV_GEM_CPU_PREP: u32 = DRM_IOW!(DRM_COMMAND_BASE + DRM_ETNAVIV_GEM_CPU_PREP, drm_etnaviv_gem_cpu_prep);
pub const DRM_IOCTL_ETNAVIV_GEM_CPU_FINI: u32 = DRM_IOW!(DRM_COMMAND_BASE + DRM_ETNAVIV_GEM_CPU_FINI, drm_etnaviv_gem_cpu_fini);
pub const DRM_IOCTL_ETNAVIV_GEM_SUBMIT: u32 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_ETNAVIV_GEM_SUBMIT, drm_etnaviv_gem_submit);
pub const DRM_IOCTL_ETNAVIV_WAIT_FENCE: u32 = DRM_IOW!(DRM_COMMAND_BASE + DRM_ETNAVIV_WAIT_FENCE, drm_etnaviv_wait_fence);
pub const DRM_IOCTL_ETNAVIV_GEM_USERPTR: u32 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_ETNAVIV_GEM_USERPTR, drm_etnaviv_gem_userptr);
pub const DRM_IOCTL_ETNAVIV_GEM_WAIT: u32 = DRM_IOW!(DRM_COMMAND_BASE + DRM_ETNAVIV_GEM_WAIT, drm_etnaviv_gem_wait);
pub const DRM_IOCTL_ETNAVIV_PM_QUERY_DOM: u32 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_ETNAVIV_PM_QUERY_DOM, drm_etnaviv_pm_domain);
pub const DRM_IOCTL_ETNAVIV_PM_QUERY_SIG: u32 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_ETNAVIV_PM_QUERY_SIG, drm_etnaviv_pm_signal);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
