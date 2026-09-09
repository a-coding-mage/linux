/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Rust translation of exynos_drm.h. */

// Dependency supplied by the DRM header: DRM_COMMAND_BASE, DRM_IOWR, and
// struct drm_event are intentionally referenced but not redefined here.

#[repr(C)]
pub struct drm_exynos_gem_create { pub size: u64, pub flags: u32, pub handle: u32 }
#[repr(C)]
pub struct drm_exynos_gem_map { pub handle: u32, pub reserved: u32, pub offset: u64 }
#[repr(C)]
pub struct drm_exynos_gem_info { pub handle: u32, pub flags: u32, pub size: u64 }
#[repr(C)]
pub struct drm_exynos_vidi_connection { pub connection: u32, pub extensions: u32, pub edid: u64 }

pub const EXYNOS_BO_CONTIG: u32 = 0 << 0;
pub const EXYNOS_BO_NONCONTIG: u32 = 1 << 0;
pub const EXYNOS_BO_NONCACHABLE: u32 = 0 << 1;
pub const EXYNOS_BO_CACHABLE: u32 = 1 << 1;
pub const EXYNOS_BO_WC: u32 = 1 << 2;
pub const EXYNOS_BO_MASK: u32 = EXYNOS_BO_NONCONTIG | EXYNOS_BO_CACHABLE | EXYNOS_BO_WC;

#[repr(C)] pub struct drm_exynos_g2d_get_ver { pub major: u32, pub minor: u32 }
#[repr(C)] pub struct drm_exynos_g2d_cmd { pub offset: u32, pub data: u32 }
pub const G2D_BUF_USERPTR: u32 = 1 << 31;
pub const G2D_EVENT_NOT: u32 = 0;
pub const G2D_EVENT_NONSTOP: u32 = 1;
pub const G2D_EVENT_STOP: u32 = 2; // not yet
#[repr(C)] pub struct drm_exynos_g2d_userptr { pub userptr: ::core::ffi::c_ulong, pub size: ::core::ffi::c_ulong }
#[repr(C)] pub struct drm_exynos_g2d_set_cmdlist { pub cmd: u64, pub cmd_buf: u64, pub cmd_nr: u32, pub cmd_buf_nr: u32, pub event_type: u64, pub user_data: u64 }
#[repr(C)] pub struct drm_exynos_g2d_exec { pub async_: u64 }

#[repr(C)] pub struct drm_exynos_ioctl_ipp_get_res { pub count_ipps: u32, pub reserved: u32, pub ipp_id_ptr: u64 }
pub const DRM_EXYNOS_IPP_FORMAT_SOURCE: u32 = 0x01;
pub const DRM_EXYNOS_IPP_FORMAT_DESTINATION: u32 = 0x02;
#[repr(C)] pub struct drm_exynos_ipp_format { pub fourcc: u32, pub type_: u32, pub modifier: u64 }
pub const DRM_EXYNOS_IPP_CAP_CROP: u32 = 0x01;
pub const DRM_EXYNOS_IPP_CAP_ROTATE: u32 = 0x02;
pub const DRM_EXYNOS_IPP_CAP_SCALE: u32 = 0x04;
pub const DRM_EXYNOS_IPP_CAP_CONVERT: u32 = 0x08;
pub const DRM_EXYNOS_IPP_LIMIT_TYPE_SIZE: u32 = 0x0001;
pub const DRM_EXYNOS_IPP_LIMIT_TYPE_SCALE: u32 = 0x0002;
pub const DRM_EXYNOS_IPP_LIMIT_SIZE_BUFFER: u32 = 0x0001 << 16;
pub const DRM_EXYNOS_IPP_LIMIT_SIZE_AREA: u32 = 0x0002 << 16;
pub const DRM_EXYNOS_IPP_LIMIT_SIZE_ROTATED: u32 = 0x0003 << 16;
pub const DRM_EXYNOS_IPP_LIMIT_TYPE_MASK: u32 = 0x000f;
pub const DRM_EXYNOS_IPP_LIMIT_SIZE_MASK: u32 = 0x000f << 16;
#[repr(C)] pub struct drm_exynos_ipp_limit_val { pub min: u32, pub max: u32, pub align: u32, pub reserved: u32 }
#[repr(C)] pub struct drm_exynos_ipp_limit { pub type_: u32, pub reserved: u32, pub h: drm_exynos_ipp_limit_val, pub v: drm_exynos_ipp_limit_val }
#[repr(C)] pub struct drm_exynos_ioctl_ipp_get_limits { pub ipp_id: u32, pub fourcc: u32, pub modifier: u64, pub type_: u32, pub limits_count: u32, pub limits_ptr: u64 }

pub const DRM_EXYNOS_IPP_TASK_BUFFER: u32 = 0x0001;
pub const DRM_EXYNOS_IPP_TASK_RECTANGLE: u32 = 0x0002;
pub const DRM_EXYNOS_IPP_TASK_TRANSFORM: u32 = 0x0003;
pub const DRM_EXYNOS_IPP_TASK_ALPHA: u32 = 0x0004;
pub const DRM_EXYNOS_IPP_TASK_TYPE_SOURCE: u32 = 0x0001 << 16;
pub const DRM_EXYNOS_IPP_TASK_TYPE_DESTINATION: u32 = 0x0002 << 16;
#[repr(C)] pub struct drm_exynos_ipp_task_buffer { pub id: u32, pub fourcc: u32, pub width: u32, pub height: u32, pub gem_id: [u32; 4], pub offset: [u32; 4], pub pitch: [u32; 4], pub modifier: u64 }
#[repr(C)] pub struct drm_exynos_ipp_task_rect { pub id: u32, pub reserved: u32, pub x: u32, pub y: u32, pub w: u32, pub h: u32 }
#[repr(C)] pub struct drm_exynos_ipp_task_transform { pub id: u32, pub rotation: u32 }
#[repr(C)] pub struct drm_exynos_ipp_task_alpha { pub id: u32, pub value: u32 }
pub const DRM_EXYNOS_IPP_FLAG_EVENT: u32 = 0x01;
pub const DRM_EXYNOS_IPP_FLAG_TEST_ONLY: u32 = 0x02;
pub const DRM_EXYNOS_IPP_FLAG_NONBLOCK: u32 = 0x04;
pub const DRM_EXYNOS_IPP_FLAGS: u32 = DRM_EXYNOS_IPP_FLAG_EVENT | DRM_EXYNOS_IPP_FLAG_TEST_ONLY | DRM_EXYNOS_IPP_FLAG_NONBLOCK;
#[repr(C)] pub struct drm_exynos_ioctl_ipp_get_caps { pub ipp_id: u32, pub capabilities: u32, pub reserved: u32, pub formats_count: u32, pub formats_ptr: u64 }
#[repr(C)] pub struct drm_exynos_ioctl_ipp_commit { pub ipp_id: u32, pub flags: u32, pub reserved: u32, pub params_size: u32, pub params_ptr: u64, pub user_data: u64 }

pub const DRM_EXYNOS_GEM_CREATE: u32 = 0x00;
pub const DRM_EXYNOS_GEM_MAP: u32 = 0x01;
pub const DRM_EXYNOS_GEM_GET: u32 = 0x04;
pub const DRM_EXYNOS_VIDI_CONNECTION: u32 = 0x07;
pub const DRM_EXYNOS_G2D_GET_VER: u32 = 0x20;
pub const DRM_EXYNOS_G2D_SET_CMDLIST: u32 = 0x21;
pub const DRM_EXYNOS_G2D_EXEC: u32 = 0x22;
pub const DRM_EXYNOS_IPP_GET_RESOURCES: u32 = 0x40;
pub const DRM_EXYNOS_IPP_GET_CAPS: u32 = 0x41;
pub const DRM_EXYNOS_IPP_GET_LIMITS: u32 = 0x42;
pub const DRM_EXYNOS_IPP_COMMIT: u32 = 0x43;
pub const DRM_EXYNOS_G2D_EVENT: u32 = 0x80000000;
pub const DRM_EXYNOS_IPP_EVENT: u32 = 0x80000002;

// ioctl encodings use the DRM_IOWR macro supplied by drm.h.
pub const DRM_IOCTL_EXYNOS_GEM_CREATE: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_EXYNOS_GEM_CREATE, drm_exynos_gem_create);
pub const DRM_IOCTL_EXYNOS_GEM_MAP: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_EXYNOS_GEM_MAP, drm_exynos_gem_map);
pub const DRM_IOCTL_EXYNOS_GEM_GET: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_EXYNOS_GEM_GET, drm_exynos_gem_info);
pub const DRM_IOCTL_EXYNOS_VIDI_CONNECTION: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_EXYNOS_VIDI_CONNECTION, drm_exynos_vidi_connection);
pub const DRM_IOCTL_EXYNOS_G2D_GET_VER: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_EXYNOS_G2D_GET_VER, drm_exynos_g2d_get_ver);
pub const DRM_IOCTL_EXYNOS_G2D_SET_CMDLIST: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_EXYNOS_G2D_SET_CMDLIST, drm_exynos_g2d_set_cmdlist);
pub const DRM_IOCTL_EXYNOS_G2D_EXEC: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_EXYNOS_G2D_EXEC, drm_exynos_g2d_exec);
pub const DRM_IOCTL_EXYNOS_IPP_GET_RESOURCES: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_EXYNOS_IPP_GET_RESOURCES, drm_exynos_ioctl_ipp_get_res);
pub const DRM_IOCTL_EXYNOS_IPP_GET_CAPS: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_EXYNOS_IPP_GET_CAPS, drm_exynos_ioctl_ipp_get_caps);
pub const DRM_IOCTL_EXYNOS_IPP_GET_LIMITS: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_EXYNOS_IPP_GET_LIMITS, drm_exynos_ioctl_ipp_get_limits);
pub const DRM_IOCTL_EXYNOS_IPP_COMMIT: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_EXYNOS_IPP_COMMIT, drm_exynos_ioctl_ipp_commit);

#[repr(C)] pub struct drm_exynos_g2d_event { pub base: drm_event, pub user_data: u64, pub tv_sec: u32, pub tv_usec: u32, pub cmdlist_no: u32, pub reserved: u32 }
#[repr(C)] pub struct drm_exynos_ipp_event { pub base: drm_event, pub user_data: u64, pub tv_sec: u32, pub tv_usec: u32, pub ipp_id: u32, pub sequence: u32, pub reserved: u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
