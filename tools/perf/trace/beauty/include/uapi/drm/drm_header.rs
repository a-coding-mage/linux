/* SPDX-License-Identifier: MIT */
/*
 * Header for the Direct Rendering Manager
 *
 * Author: Rickard E. (Rik) Faith <faith@valinux.com>
 *
 * Acknowledgments:
 * Dec 1999, Richard Henderson <rth@twiddle.net>, move to generic cmpxchg.
 */

/*
 * Copyright 1999 Precision Insight, Inc., Cedar Park, Texas.
 * Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
 * All rights reserved.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::c_void;
use core::mem::size_of;
use std::os::raw::{c_char, c_int, c_long, c_ulong};

/* C header dependency intent:
 * - Linux/kernel builds include <linux/types.h> and <asm/ioctl.h>.
 * - BSD builds provide the fixed-width aliases below and <sys/ioccom.h>.
 * - drm_mode.h supplies the drm_mode_* structs referenced by ioctl macros.
 */
pub type __s8 = i8;
pub type __u8 = u8;
pub type __s16 = i16;
pub type __u16 = u16;
pub type __s32 = i32;
pub type __u32 = u32;
pub type __s64 = i64;
pub type __u64 = u64;
pub type __kernel_size_t = usize;
pub type drm_handle_t = c_uint;
pub type c_uint = u32;

pub const DRM_NAME: &[u8; 4] = b"drm\0"; /**< Name in kernel, /dev, and /proc */
pub const DRM_MIN_ORDER: c_int = 5; /**< At least 2^5 bytes = 32 bytes */
pub const DRM_MAX_ORDER: c_int = 22; /**< Up to 2^22 bytes = 4MB */
pub const DRM_RAM_PERCENT: c_int = 10; /**< How much system ram can we lock? */

pub const _DRM_LOCK_HELD: c_uint = 0x80000000u32; /**< Hardware lock is held */
pub const _DRM_LOCK_CONT: c_uint = 0x40000000u32; /**< Hardware lock is contended */
pub const fn _DRM_LOCK_IS_HELD(lock: c_uint) -> c_uint {
    lock & _DRM_LOCK_HELD
}
pub const fn _DRM_LOCK_IS_CONT(lock: c_uint) -> c_uint {
    lock & _DRM_LOCK_CONT
}
pub const fn _DRM_LOCKING_CONTEXT(lock: c_uint) -> c_uint {
    lock & !(_DRM_LOCK_HELD | _DRM_LOCK_CONT)
}

pub type drm_context_t = c_uint;
pub type drm_drawable_t = c_uint;
pub type drm_magic_t = c_uint;

/*
 * Cliprect.
 *
 * \warning: If you change this structure, make sure you change
 * XF86DRIClipRectRec in the server as well
 *
 * \note KW: Actually it's illegal to change either for
 * backwards-compatibility reasons.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_clip_rect {
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
}

/*
 * Drawable information.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_drawable_info {
    pub num_rects: c_uint,
    pub rects: *mut drm_clip_rect,
}

/*
 * Texture region,
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tex_region {
    pub next: u8,
    pub prev: u8,
    pub in_use: u8,
    pub padding: u8,
    pub age: c_uint,
}

/*
 * Hardware lock.
 *
 * The lock structure is a simple cache-line aligned integer.  To avoid
 * processor bus contention on a multiprocessor system, there should not be any
 * other data stored in the same cache line.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_hw_lock {
    pub lock: c_uint, /* volatile in C */
    pub padding: [c_char; 60],
}

/*
 * DRM_IOCTL_VERSION ioctl argument type.
 *
 * \sa drmGetVersion().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_version {
    pub version_major: c_int,
    pub version_minor: c_int,
    pub version_patchlevel: c_int,
    pub name_len: __kernel_size_t,
    pub name: *mut c_char, /* __user */
    pub date_len: __kernel_size_t,
    pub date: *mut c_char, /* __user */
    pub desc_len: __kernel_size_t,
    pub desc: *mut c_char, /* __user */
}

/*
 * DRM_IOCTL_GET_UNIQUE ioctl argument type.
 *
 * \sa drmGetBusid() and drmSetBusId().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_unique {
    pub unique_len: __kernel_size_t,
    pub unique: *mut c_char, /* __user */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_list {
    pub count: c_int,
    pub version: *mut drm_version, /* __user */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_block {
    pub unused: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_control_func {
    DRM_ADD_COMMAND,
    DRM_RM_COMMAND,
    DRM_INST_HANDLER,
    DRM_UNINST_HANDLER,
}

/*
 * DRM_IOCTL_CONTROL ioctl argument type.
 *
 * \sa drmCtlInstHandler() and drmCtlUninstHandler().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_control {
    pub func: drm_control_func,
    pub irq: c_int,
}

/*
 * Type of memory to map.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_map_type {
    _DRM_FRAME_BUFFER = 0,
    _DRM_REGISTERS = 1,
    _DRM_SHM = 2,
    _DRM_AGP = 3,
    _DRM_SCATTER_GATHER = 4,
    _DRM_CONSISTENT = 5,
}

/*
 * Memory mapping flags.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_map_flags {
    _DRM_RESTRICTED = 0x01,
    _DRM_READ_ONLY = 0x02,
    _DRM_LOCKED = 0x04,
    _DRM_KERNEL = 0x08,
    _DRM_WRITE_COMBINING = 0x10,
    _DRM_CONTAINS_LOCK = 0x20,
    _DRM_REMOVABLE = 0x40,
    _DRM_DRIVER = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ctx_priv_map {
    pub ctx_id: c_uint,
    pub handle: *mut c_void,
}

/*
 * DRM_IOCTL_GET_MAP, DRM_IOCTL_ADD_MAP and DRM_IOCTL_RM_MAP ioctls
 * argument type.
 *
 * \sa drmAddMap().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_map {
    pub offset: c_ulong,
    pub size: c_ulong,
    pub type_: drm_map_type,
    pub flags: drm_map_flags,
    pub handle: *mut c_void,
    pub mtrr: c_int,
}

/*
 * DRM_IOCTL_GET_CLIENT ioctl argument type.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_client {
    pub idx: c_int,
    pub auth: c_int,
    pub pid: c_ulong,
    pub uid: c_ulong,
    pub magic: c_ulong,
    pub iocs: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_stat_type {
    _DRM_STAT_LOCK,
    _DRM_STAT_OPENS,
    _DRM_STAT_CLOSES,
    _DRM_STAT_IOCTLS,
    _DRM_STAT_LOCKS,
    _DRM_STAT_UNLOCKS,
    _DRM_STAT_VALUE,
    _DRM_STAT_BYTE,
    _DRM_STAT_COUNT,
    _DRM_STAT_IRQ,
    _DRM_STAT_PRIMARY,
    _DRM_STAT_SECONDARY,
    _DRM_STAT_DMA,
    _DRM_STAT_SPECIAL,
    _DRM_STAT_MISSED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_stats_data {
    pub value: c_ulong,
    pub type_: drm_stat_type,
}

/*
 * DRM_IOCTL_GET_STATS ioctl argument type.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_stats {
    pub count: c_ulong,
    pub data: [drm_stats_data; 15],
}

/*
 * Hardware locking flags.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_lock_flags {
    _DRM_LOCK_READY = 0x01,
    _DRM_LOCK_QUIESCENT = 0x02,
    _DRM_LOCK_FLUSH = 0x04,
    _DRM_LOCK_FLUSH_ALL = 0x08,
    _DRM_HALT_ALL_QUEUES = 0x10,
    _DRM_HALT_CUR_QUEUES = 0x20,
}

/*
 * DRM_IOCTL_LOCK, DRM_IOCTL_UNLOCK and DRM_IOCTL_FINISH ioctl argument type.
 *
 * \sa drmGetLock() and drmUnlock().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lock {
    pub context: c_int,
    pub flags: drm_lock_flags,
}

/*
 * DMA flags
 *
 * \warning
 * These values \e must match xf86drm.h.
 *
 * \sa drm_dma.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_dma_flags {
    _DRM_DMA_BLOCK = 0x01,
    _DRM_DMA_WHILE_LOCKED = 0x02,
    _DRM_DMA_PRIORITY = 0x04,
    _DRM_DMA_WAIT = 0x10,
    _DRM_DMA_SMALLER_OK = 0x20,
    _DRM_DMA_LARGER_OK = 0x40,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_buf_desc_flags {
    _DRM_PAGE_ALIGN = 0x01,
    _DRM_AGP_BUFFER = 0x02,
    _DRM_SG_BUFFER = 0x04,
    _DRM_FB_BUFFER = 0x08,
    _DRM_PCI_BUFFER_RO = 0x10,
}

/*
 * DRM_IOCTL_ADD_BUFS and DRM_IOCTL_MARK_BUFS ioctl argument type.
 *
 * \sa drmAddBufs().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_buf_desc {
    pub count: c_int,
    pub size: c_int,
    pub low_mark: c_int,
    pub high_mark: c_int,
    pub flags: drm_buf_desc_flags,
    pub agp_start: c_ulong,
}

/*
 * DRM_IOCTL_INFO_BUFS ioctl argument type.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_buf_info {
    pub count: c_int,
    pub list: *mut drm_buf_desc, /* __user */
}

/*
 * DRM_IOCTL_FREE_BUFS ioctl argument type.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_buf_free {
    pub count: c_int,
    pub list: *mut c_int, /* __user */
}

/*
 * Buffer information
 *
 * \sa drm_buf_map.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_buf_pub {
    pub idx: c_int,
    pub total: c_int,
    pub used: c_int,
    pub address: *mut c_void, /* __user */
}

/*
 * DRM_IOCTL_MAP_BUFS ioctl argument type.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_buf_map {
    pub count: c_int,
    /* C++ uses field name virt; C uses virtual. */
    pub virtual_: *mut c_void, /* __user */
    pub list: *mut drm_buf_pub, /* __user */
}

/*
 * DRM_IOCTL_DMA ioctl argument type.
 *
 * Indices here refer to the offset into the buffer list in drm_buf_get.
 *
 * \sa drmDMA().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dma {
    pub context: c_int,
    pub send_count: c_int,
    pub send_indices: *mut c_int, /* __user */
    pub send_sizes: *mut c_int, /* __user */
    pub flags: drm_dma_flags,
    pub request_count: c_int,
    pub request_size: c_int,
    pub request_indices: *mut c_int, /* __user */
    pub request_sizes: *mut c_int, /* __user */
    pub granted_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_ctx_flags {
    _DRM_CONTEXT_PRESERVED = 0x01,
    _DRM_CONTEXT_2DONLY = 0x02,
}

/*
 * DRM_IOCTL_ADD_CTX ioctl argument type.
 *
 * \sa drmCreateContext() and drmDestroyContext().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ctx {
    pub handle: drm_context_t,
    pub flags: drm_ctx_flags,
}

/*
 * DRM_IOCTL_RES_CTX ioctl argument type.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ctx_res {
    pub count: c_int,
    pub contexts: *mut drm_ctx, /* __user */
}

/*
 * DRM_IOCTL_ADD_DRAW and DRM_IOCTL_RM_DRAW ioctl argument type.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_draw {
    pub handle: drm_drawable_t,
}

/*
 * DRM_IOCTL_UPDATE_DRAW ioctl argument type.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_drawable_info_type_t {
    DRM_DRAWABLE_CLIPRECTS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_update_draw {
    pub handle: drm_drawable_t,
    pub type_: c_uint,
    pub num: c_uint,
    pub data: u64,
}

/*
 * DRM_IOCTL_GET_MAGIC and DRM_IOCTL_AUTH_MAGIC ioctl argument type.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_auth {
    pub magic: drm_magic_t,
}

/*
 * DRM_IOCTL_IRQ_BUSID ioctl argument type.
 *
 * \sa drmGetInterruptFromBusID().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_irq_busid {
    pub irq: c_int,
    pub busnum: c_int,
    pub devnum: c_int,
    pub funcnum: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_vblank_seq_type {
    _DRM_VBLANK_ABSOLUTE = 0x0,
    _DRM_VBLANK_RELATIVE = 0x1,
    _DRM_VBLANK_HIGH_CRTC_MASK = 0x0000003e,
    _DRM_VBLANK_EVENT = 0x4000000,
    _DRM_VBLANK_FLIP = 0x8000000,
    _DRM_VBLANK_NEXTONMISS = 0x10000000,
    _DRM_VBLANK_SECONDARY = 0x20000000,
    _DRM_VBLANK_SIGNAL = 0x40000000,
}
pub const _DRM_VBLANK_HIGH_CRTC_SHIFT: c_int = 1;
pub const _DRM_VBLANK_TYPES_MASK: c_uint =
    drm_vblank_seq_type::_DRM_VBLANK_ABSOLUTE as c_uint
        | drm_vblank_seq_type::_DRM_VBLANK_RELATIVE as c_uint;
pub const _DRM_VBLANK_FLAGS_MASK: c_uint =
    drm_vblank_seq_type::_DRM_VBLANK_EVENT as c_uint
        | drm_vblank_seq_type::_DRM_VBLANK_SIGNAL as c_uint
        | drm_vblank_seq_type::_DRM_VBLANK_SECONDARY as c_uint
        | drm_vblank_seq_type::_DRM_VBLANK_NEXTONMISS as c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_wait_vblank_request {
    pub type_: drm_vblank_seq_type,
    pub sequence: c_uint,
    pub signal: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_wait_vblank_reply {
    pub type_: drm_vblank_seq_type,
    pub sequence: c_uint,
    pub tval_sec: c_long,
    pub tval_usec: c_long,
}

/*
 * DRM_IOCTL_WAIT_VBLANK ioctl argument type.
 *
 * \sa drmWaitVBlank().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_wait_vblank {
    pub request: drm_wait_vblank_request,
    pub reply: drm_wait_vblank_reply,
}

pub const _DRM_PRE_MODESET: c_int = 1;
pub const _DRM_POST_MODESET: c_int = 2;

/*
 * DRM_IOCTL_MODESET_CTL ioctl argument type
 *
 * \sa drmModesetCtl().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_modeset_ctl {
    pub crtc: __u32,
    pub cmd: __u32,
}

/*
 * DRM_IOCTL_AGP_ENABLE ioctl argument type.
 *
 * \sa drmAgpEnable().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_agp_mode {
    pub mode: c_ulong,
}

/*
 * DRM_IOCTL_AGP_ALLOC and DRM_IOCTL_AGP_FREE ioctls argument type.
 *
 * \sa drmAgpAlloc() and drmAgpFree().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_agp_buffer {
    pub size: c_ulong,
    pub handle: c_ulong,
    pub type_: c_ulong,
    pub physical: c_ulong,
}

/*
 * DRM_IOCTL_AGP_BIND and DRM_IOCTL_AGP_UNBIND ioctls argument type.
 *
 * \sa drmAgpBind() and drmAgpUnbind().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_agp_binding {
    pub handle: c_ulong,
    pub offset: c_ulong,
}

/*
 * DRM_IOCTL_AGP_INFO ioctl argument type.
 *
 * \sa drmAgpVersionMajor(), drmAgpVersionMinor(), drmAgpGetMode(),
 * drmAgpBase(), drmAgpSize(), drmAgpMemoryUsed(), drmAgpMemoryAvail(),
 * drmAgpVendorId() and drmAgpDeviceId().
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_agp_info {
    pub agp_version_major: c_int,
    pub agp_version_minor: c_int,
    pub mode: c_ulong,
    pub aperture_base: c_ulong,
    pub aperture_size: c_ulong,
    pub memory_allowed: c_ulong,
    pub memory_used: c_ulong,
    pub id_vendor: u16,
    pub id_device: u16,
}

/*
 * DRM_IOCTL_SG_ALLOC ioctl argument type.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_scatter_gather {
    pub size: c_ulong,
    pub handle: c_ulong,
}

/*
 * DRM_IOCTL_SET_VERSION ioctl argument type.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_set_version {
    pub drm_di_major: c_int,
    pub drm_di_minor: c_int,
    pub drm_dd_major: c_int,
    pub drm_dd_minor: c_int,
}

/**
 * struct drm_gem_close - Argument for &DRM_IOCTL_GEM_CLOSE ioctl.
 * @handle: Handle of the object to be closed.
 * @pad: Padding.
 *
 * Releases the handle to an mm object.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_close {
    pub handle: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_flink {
    pub handle: __u32,
    pub name: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_open {
    pub name: __u32,
    pub handle: __u32,
    pub size: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_change_handle {
    pub handle: __u32,
    pub new_handle: __u32,
}

pub const DRM_CAP_DUMB_BUFFER: c_int = 0x1;
pub const DRM_CAP_VBLANK_HIGH_CRTC: c_int = 0x2;
pub const DRM_CAP_DUMB_PREFERRED_DEPTH: c_int = 0x3;
pub const DRM_CAP_DUMB_PREFER_SHADOW: c_int = 0x4;
pub const DRM_CAP_PRIME: c_int = 0x5;
pub const DRM_PRIME_CAP_IMPORT: c_int = 0x1;
pub const DRM_PRIME_CAP_EXPORT: c_int = 0x2;
pub const DRM_CAP_TIMESTAMP_MONOTONIC: c_int = 0x6;
pub const DRM_CAP_ASYNC_PAGE_FLIP: c_int = 0x7;
pub const DRM_CAP_CURSOR_WIDTH: c_int = 0x8;
pub const DRM_CAP_CURSOR_HEIGHT: c_int = 0x9;
pub const DRM_CAP_ADDFB2_MODIFIERS: c_int = 0x10;
pub const DRM_CAP_PAGE_FLIP_TARGET: c_int = 0x11;
pub const DRM_CAP_CRTC_IN_VBLANK_EVENT: c_int = 0x12;
pub const DRM_CAP_SYNCOBJ: c_int = 0x13;
pub const DRM_CAP_SYNCOBJ_TIMELINE: c_int = 0x14;
pub const DRM_CAP_ATOMIC_ASYNC_PAGE_FLIP: c_int = 0x15;

/* DRM_IOCTL_GET_CAP ioctl argument type */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_get_cap {
    pub capability: __u64,
    pub value: __u64,
}

pub const DRM_CLIENT_CAP_STEREO_3D: c_int = 1;
pub const DRM_CLIENT_CAP_UNIVERSAL_PLANES: c_int = 2;
pub const DRM_CLIENT_CAP_ATOMIC: c_int = 3;
pub const DRM_CLIENT_CAP_ASPECT_RATIO: c_int = 4;
pub const DRM_CLIENT_CAP_WRITEBACK_CONNECTORS: c_int = 5;
pub const DRM_CLIENT_CAP_CURSOR_PLANE_HOTSPOT: c_int = 6;
pub const DRM_CLIENT_CAP_PLANE_COLOR_PIPELINE: c_int = 7;

/* DRM_IOCTL_SET_CLIENT_CAP ioctl argument type */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_set_client_cap {
    pub capability: __u64,
    pub value: __u64,
}

pub const DRM_RDWR: c_int = O_RDWR;
pub const DRM_CLOEXEC: c_int = O_CLOEXEC;
pub const O_RDWR: c_int = 0o2;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub const O_CLOEXEC: c_int = 0o2000000;
#[cfg(not(any(target_os = "linux", target_os = "android")))]
pub const O_CLOEXEC: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_prime_handle {
    pub handle: __u32,
    pub flags: __u32,
    pub fd: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_create {
    pub handle: __u32,
    pub flags: __u32,
}
pub const DRM_SYNCOBJ_CREATE_SIGNALED: c_int = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_destroy {
    pub handle: __u32,
    pub pad: __u32,
}

pub const DRM_SYNCOBJ_FD_TO_HANDLE_FLAGS_IMPORT_SYNC_FILE: c_int = 1 << 0;
pub const DRM_SYNCOBJ_FD_TO_HANDLE_FLAGS_TIMELINE: c_int = 1 << 1;
pub const DRM_SYNCOBJ_HANDLE_TO_FD_FLAGS_EXPORT_SYNC_FILE: c_int = 1 << 0;
pub const DRM_SYNCOBJ_HANDLE_TO_FD_FLAGS_TIMELINE: c_int = 1 << 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_handle {
    pub handle: __u32,
    pub flags: __u32,
    pub fd: __s32,
    pub pad: __u32,
    pub point: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_transfer {
    pub src_handle: __u32,
    pub dst_handle: __u32,
    pub src_point: __u64,
    pub dst_point: __u64,
    pub flags: __u32,
    pub pad: __u32,
}

pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_ALL: c_int = 1 << 0;
pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT: c_int = 1 << 1;
pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_AVAILABLE: c_int = 1 << 2;
pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_DEADLINE: c_int = 1 << 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_wait {
    pub handles: __u64,
    pub timeout_nsec: __s64,
    pub count_handles: __u32,
    pub flags: __u32,
    pub first_signaled: __u32,
    pub pad: __u32,
    pub deadline_nsec: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_timeline_wait {
    pub handles: __u64,
    pub points: __u64,
    pub timeout_nsec: __s64,
    pub count_handles: __u32,
    pub flags: __u32,
    pub first_signaled: __u32,
    pub pad: __u32,
    pub deadline_nsec: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_eventfd {
    pub handle: __u32,
    pub flags: __u32,
    pub point: __u64,
    pub fd: __s32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_array {
    pub handles: __u64,
    pub count_handles: __u32,
    pub pad: __u32,
}

pub const DRM_SYNCOBJ_QUERY_FLAGS_LAST_SUBMITTED: c_int = 1 << 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_timeline_array {
    pub handles: __u64,
    pub points: __u64,
    pub count_handles: __u32,
    pub flags: __u32,
}

/* Query current scanout sequence number */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_crtc_get_sequence {
    pub crtc_id: __u32,
    pub active: __u32,
    pub sequence: __u64,
    pub sequence_ns: __s64,
}

pub const DRM_CRTC_SEQUENCE_RELATIVE: c_int = 0x00000001;
pub const DRM_CRTC_SEQUENCE_NEXT_ON_MISS: c_int = 0x00000002;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_crtc_queue_sequence {
    pub crtc_id: __u32,
    pub flags: __u32,
    pub sequence: __u64,
    pub user_data: __u64,
}

pub const DRM_CLIENT_NAME_MAX_LEN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_set_client_name {
    pub name_len: __u64,
    pub name: __u64,
}

pub const DRM_IOCTL_BASE: u8 = b'd';

const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_DIRBITS: u32 = 2;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_NONE: u32 = 0;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

pub const fn _IOC(dir: u32, type_: u32, nr: u32, size: u32) -> c_ulong {
    ((dir << IOC_DIRSHIFT) | (type_ << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT))
        as c_ulong
}
pub const fn DRM_IO(nr: u32) -> c_ulong {
    _IOC(IOC_NONE, DRM_IOCTL_BASE as u32, nr, 0)
}
pub const fn DRM_IOR_size(nr: u32, size: usize) -> c_ulong {
    _IOC(IOC_READ, DRM_IOCTL_BASE as u32, nr, size as u32)
}
pub const fn DRM_IOW_size(nr: u32, size: usize) -> c_ulong {
    _IOC(IOC_WRITE, DRM_IOCTL_BASE as u32, nr, size as u32)
}
pub const fn DRM_IOWR_size(nr: u32, size: usize) -> c_ulong {
    _IOC(IOC_READ | IOC_WRITE, DRM_IOCTL_BASE as u32, nr, size as u32)
}

pub const DRM_IOCTL_VERSION: c_ulong = DRM_IOWR_size(0x00, size_of::<drm_version>());
pub const DRM_IOCTL_GET_UNIQUE: c_ulong = DRM_IOWR_size(0x01, size_of::<drm_unique>());
pub const DRM_IOCTL_GET_MAGIC: c_ulong = DRM_IOR_size(0x02, size_of::<drm_auth>());
pub const DRM_IOCTL_IRQ_BUSID: c_ulong = DRM_IOWR_size(0x03, size_of::<drm_irq_busid>());
pub const DRM_IOCTL_GET_MAP: c_ulong = DRM_IOWR_size(0x04, size_of::<drm_map>());
pub const DRM_IOCTL_GET_CLIENT: c_ulong = DRM_IOWR_size(0x05, size_of::<drm_client>());
pub const DRM_IOCTL_GET_STATS: c_ulong = DRM_IOR_size(0x06, size_of::<drm_stats>());
pub const DRM_IOCTL_SET_VERSION: c_ulong = DRM_IOWR_size(0x07, size_of::<drm_set_version>());
pub const DRM_IOCTL_MODESET_CTL: c_ulong = DRM_IOW_size(0x08, size_of::<drm_modeset_ctl>());
pub const DRM_IOCTL_GEM_CLOSE: c_ulong = DRM_IOW_size(0x09, size_of::<drm_gem_close>());
pub const DRM_IOCTL_GEM_FLINK: c_ulong = DRM_IOWR_size(0x0a, size_of::<drm_gem_flink>());
pub const DRM_IOCTL_GEM_OPEN: c_ulong = DRM_IOWR_size(0x0b, size_of::<drm_gem_open>());
pub const DRM_IOCTL_GET_CAP: c_ulong = DRM_IOWR_size(0x0c, size_of::<drm_get_cap>());
pub const DRM_IOCTL_SET_CLIENT_CAP: c_ulong = DRM_IOW_size(0x0d, size_of::<drm_set_client_cap>());
pub const DRM_IOCTL_SET_UNIQUE: c_ulong = DRM_IOW_size(0x10, size_of::<drm_unique>());
pub const DRM_IOCTL_AUTH_MAGIC: c_ulong = DRM_IOW_size(0x11, size_of::<drm_auth>());
pub const DRM_IOCTL_BLOCK: c_ulong = DRM_IOWR_size(0x12, size_of::<drm_block>());
pub const DRM_IOCTL_UNBLOCK: c_ulong = DRM_IOWR_size(0x13, size_of::<drm_block>());
pub const DRM_IOCTL_CONTROL: c_ulong = DRM_IOW_size(0x14, size_of::<drm_control>());
pub const DRM_IOCTL_ADD_MAP: c_ulong = DRM_IOWR_size(0x15, size_of::<drm_map>());
pub const DRM_IOCTL_ADD_BUFS: c_ulong = DRM_IOWR_size(0x16, size_of::<drm_buf_desc>());
pub const DRM_IOCTL_MARK_BUFS: c_ulong = DRM_IOW_size(0x17, size_of::<drm_buf_desc>());
pub const DRM_IOCTL_INFO_BUFS: c_ulong = DRM_IOWR_size(0x18, size_of::<drm_buf_info>());
pub const DRM_IOCTL_MAP_BUFS: c_ulong = DRM_IOWR_size(0x19, size_of::<drm_buf_map>());
pub const DRM_IOCTL_FREE_BUFS: c_ulong = DRM_IOW_size(0x1a, size_of::<drm_buf_free>());
pub const DRM_IOCTL_RM_MAP: c_ulong = DRM_IOW_size(0x1b, size_of::<drm_map>());
pub const DRM_IOCTL_SET_SAREA_CTX: c_ulong = DRM_IOW_size(0x1c, size_of::<drm_ctx_priv_map>());
pub const DRM_IOCTL_GET_SAREA_CTX: c_ulong = DRM_IOWR_size(0x1d, size_of::<drm_ctx_priv_map>());
pub const DRM_IOCTL_SET_MASTER: c_ulong = DRM_IO(0x1e);
pub const DRM_IOCTL_DROP_MASTER: c_ulong = DRM_IO(0x1f);
pub const DRM_IOCTL_ADD_CTX: c_ulong = DRM_IOWR_size(0x20, size_of::<drm_ctx>());
pub const DRM_IOCTL_RM_CTX: c_ulong = DRM_IOWR_size(0x21, size_of::<drm_ctx>());
pub const DRM_IOCTL_MOD_CTX: c_ulong = DRM_IOW_size(0x22, size_of::<drm_ctx>());
pub const DRM_IOCTL_GET_CTX: c_ulong = DRM_IOWR_size(0x23, size_of::<drm_ctx>());
pub const DRM_IOCTL_SWITCH_CTX: c_ulong = DRM_IOW_size(0x24, size_of::<drm_ctx>());
pub const DRM_IOCTL_NEW_CTX: c_ulong = DRM_IOW_size(0x25, size_of::<drm_ctx>());
pub const DRM_IOCTL_RES_CTX: c_ulong = DRM_IOWR_size(0x26, size_of::<drm_ctx_res>());
pub const DRM_IOCTL_ADD_DRAW: c_ulong = DRM_IOWR_size(0x27, size_of::<drm_draw>());
pub const DRM_IOCTL_RM_DRAW: c_ulong = DRM_IOWR_size(0x28, size_of::<drm_draw>());
pub const DRM_IOCTL_DMA: c_ulong = DRM_IOWR_size(0x29, size_of::<drm_dma>());
pub const DRM_IOCTL_LOCK: c_ulong = DRM_IOW_size(0x2a, size_of::<drm_lock>());
pub const DRM_IOCTL_UNLOCK: c_ulong = DRM_IOW_size(0x2b, size_of::<drm_lock>());
pub const DRM_IOCTL_FINISH: c_ulong = DRM_IOW_size(0x2c, size_of::<drm_lock>());
pub const DRM_IOCTL_PRIME_HANDLE_TO_FD: c_ulong = DRM_IOWR_size(0x2d, size_of::<drm_prime_handle>());
pub const DRM_IOCTL_PRIME_FD_TO_HANDLE: c_ulong = DRM_IOWR_size(0x2e, size_of::<drm_prime_handle>());
pub const DRM_IOCTL_AGP_ACQUIRE: c_ulong = DRM_IO(0x30);
pub const DRM_IOCTL_AGP_RELEASE: c_ulong = DRM_IO(0x31);
pub const DRM_IOCTL_AGP_ENABLE: c_ulong = DRM_IOW_size(0x32, size_of::<drm_agp_mode>());
pub const DRM_IOCTL_AGP_INFO: c_ulong = DRM_IOR_size(0x33, size_of::<drm_agp_info>());
pub const DRM_IOCTL_AGP_ALLOC: c_ulong = DRM_IOWR_size(0x34, size_of::<drm_agp_buffer>());
pub const DRM_IOCTL_AGP_FREE: c_ulong = DRM_IOW_size(0x35, size_of::<drm_agp_buffer>());
pub const DRM_IOCTL_AGP_BIND: c_ulong = DRM_IOW_size(0x36, size_of::<drm_agp_binding>());
pub const DRM_IOCTL_AGP_UNBIND: c_ulong = DRM_IOW_size(0x37, size_of::<drm_agp_binding>());
pub const DRM_IOCTL_SG_ALLOC: c_ulong = DRM_IOWR_size(0x38, size_of::<drm_scatter_gather>());
pub const DRM_IOCTL_SG_FREE: c_ulong = DRM_IOW_size(0x39, size_of::<drm_scatter_gather>());
pub const DRM_IOCTL_WAIT_VBLANK: c_ulong = DRM_IOWR_size(0x3a, size_of::<drm_wait_vblank>());
pub const DRM_IOCTL_CRTC_GET_SEQUENCE: c_ulong = DRM_IOWR_size(0x3b, size_of::<drm_crtc_get_sequence>());
pub const DRM_IOCTL_CRTC_QUEUE_SEQUENCE: c_ulong = DRM_IOWR_size(0x3c, size_of::<drm_crtc_queue_sequence>());
pub const DRM_IOCTL_UPDATE_DRAW: c_ulong = DRM_IOW_size(0x3f, size_of::<drm_update_draw>());

/* The following DRM_IOCTL_MODE_* constants depend on types supplied by drm_mode.h. */
pub const DRM_IOCTL_MODE_GETRESOURCES_NR: u32 = 0xA0;
pub const DRM_IOCTL_MODE_GETCRTC_NR: u32 = 0xA1;
pub const DRM_IOCTL_MODE_SETCRTC_NR: u32 = 0xA2;
pub const DRM_IOCTL_MODE_CURSOR_NR: u32 = 0xA3;
pub const DRM_IOCTL_MODE_GETGAMMA_NR: u32 = 0xA4;
pub const DRM_IOCTL_MODE_SETGAMMA_NR: u32 = 0xA5;
pub const DRM_IOCTL_MODE_GETENCODER_NR: u32 = 0xA6;
pub const DRM_IOCTL_MODE_GETCONNECTOR_NR: u32 = 0xA7;
pub const DRM_IOCTL_MODE_ATTACHMODE_NR: u32 = 0xA8;
pub const DRM_IOCTL_MODE_DETACHMODE_NR: u32 = 0xA9;
pub const DRM_IOCTL_MODE_GETPROPERTY_NR: u32 = 0xAA;
pub const DRM_IOCTL_MODE_SETPROPERTY_NR: u32 = 0xAB;
pub const DRM_IOCTL_MODE_GETPROPBLOB_NR: u32 = 0xAC;
pub const DRM_IOCTL_MODE_GETFB_NR: u32 = 0xAD;
pub const DRM_IOCTL_MODE_ADDFB_NR: u32 = 0xAE;
pub const DRM_IOCTL_MODE_RMFB: c_ulong = DRM_IOWR_size(0xAF, size_of::<c_uint>());
pub const DRM_IOCTL_MODE_PAGE_FLIP_NR: u32 = 0xB0;
pub const DRM_IOCTL_MODE_DIRTYFB_NR: u32 = 0xB1;
pub const DRM_IOCTL_MODE_CREATE_DUMB_NR: u32 = 0xB2;
pub const DRM_IOCTL_MODE_MAP_DUMB_NR: u32 = 0xB3;
pub const DRM_IOCTL_MODE_DESTROY_DUMB_NR: u32 = 0xB4;
pub const DRM_IOCTL_MODE_GETPLANERESOURCES_NR: u32 = 0xB5;
pub const DRM_IOCTL_MODE_GETPLANE_NR: u32 = 0xB6;
pub const DRM_IOCTL_MODE_SETPLANE_NR: u32 = 0xB7;
pub const DRM_IOCTL_MODE_ADDFB2_NR: u32 = 0xB8;
pub const DRM_IOCTL_MODE_OBJ_GETPROPERTIES_NR: u32 = 0xB9;
pub const DRM_IOCTL_MODE_OBJ_SETPROPERTY_NR: u32 = 0xBA;
pub const DRM_IOCTL_MODE_CURSOR2_NR: u32 = 0xBB;
pub const DRM_IOCTL_MODE_ATOMIC_NR: u32 = 0xBC;
pub const DRM_IOCTL_MODE_CREATEPROPBLOB_NR: u32 = 0xBD;
pub const DRM_IOCTL_MODE_DESTROYPROPBLOB_NR: u32 = 0xBE;
pub const DRM_IOCTL_SYNCOBJ_CREATE: c_ulong = DRM_IOWR_size(0xBF, size_of::<drm_syncobj_create>());
pub const DRM_IOCTL_SYNCOBJ_DESTROY: c_ulong = DRM_IOWR_size(0xC0, size_of::<drm_syncobj_destroy>());
pub const DRM_IOCTL_SYNCOBJ_HANDLE_TO_FD: c_ulong = DRM_IOWR_size(0xC1, size_of::<drm_syncobj_handle>());
pub const DRM_IOCTL_SYNCOBJ_FD_TO_HANDLE: c_ulong = DRM_IOWR_size(0xC2, size_of::<drm_syncobj_handle>());
pub const DRM_IOCTL_SYNCOBJ_WAIT: c_ulong = DRM_IOWR_size(0xC3, size_of::<drm_syncobj_wait>());
pub const DRM_IOCTL_SYNCOBJ_RESET: c_ulong = DRM_IOWR_size(0xC4, size_of::<drm_syncobj_array>());
pub const DRM_IOCTL_SYNCOBJ_SIGNAL: c_ulong = DRM_IOWR_size(0xC5, size_of::<drm_syncobj_array>());
pub const DRM_IOCTL_MODE_CREATE_LEASE_NR: u32 = 0xC6;
pub const DRM_IOCTL_MODE_LIST_LESSEES_NR: u32 = 0xC7;
pub const DRM_IOCTL_MODE_GET_LEASE_NR: u32 = 0xC8;
pub const DRM_IOCTL_MODE_REVOKE_LEASE_NR: u32 = 0xC9;
pub const DRM_IOCTL_SYNCOBJ_TIMELINE_WAIT: c_ulong = DRM_IOWR_size(0xCA, size_of::<drm_syncobj_timeline_wait>());
pub const DRM_IOCTL_SYNCOBJ_QUERY: c_ulong = DRM_IOWR_size(0xCB, size_of::<drm_syncobj_timeline_array>());
pub const DRM_IOCTL_SYNCOBJ_TRANSFER: c_ulong = DRM_IOWR_size(0xCC, size_of::<drm_syncobj_transfer>());
pub const DRM_IOCTL_SYNCOBJ_TIMELINE_SIGNAL: c_ulong = DRM_IOWR_size(0xCD, size_of::<drm_syncobj_timeline_array>());
pub const DRM_IOCTL_MODE_GETFB2_NR: u32 = 0xCE;
pub const DRM_IOCTL_SYNCOBJ_EVENTFD: c_ulong = DRM_IOWR_size(0xCF, size_of::<drm_syncobj_eventfd>());
pub const DRM_IOCTL_MODE_CLOSEFB_NR: u32 = 0xD0;
pub const DRM_IOCTL_SET_CLIENT_NAME: c_ulong = DRM_IOWR_size(0xD1, size_of::<drm_set_client_name>());
pub const DRM_IOCTL_GEM_CHANGE_HANDLE: c_ulong = DRM_IOWR_size(0xD2, size_of::<drm_gem_change_handle>());

/*
 * Device specific ioctls should only be in their respective headers
 * The device specific ioctl range is from 0x40 to 0x9f.
 * Generic IOCTLS restart at 0xA0.
 *
 * \sa drmCommandNone(), drmCommandRead(), drmCommandWrite(), and
 * drmCommandReadWrite().
 */
pub const DRM_COMMAND_BASE: c_int = 0x40;
pub const DRM_COMMAND_END: c_int = 0xA0;

/**
 * struct drm_event - Header for DRM events
 * @type: event type.
 * @length: total number of payload bytes (including header).
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_event {
    pub type_: __u32,
    pub length: __u32,
}

pub const DRM_EVENT_VBLANK: c_int = 0x01;
pub const DRM_EVENT_FLIP_COMPLETE: c_int = 0x02;
pub const DRM_EVENT_CRTC_SEQUENCE: c_int = 0x03;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_event_vblank {
    pub base: drm_event,
    pub user_data: __u64,
    pub tv_sec: __u32,
    pub tv_usec: __u32,
    pub sequence: __u32,
    pub crtc_id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_event_crtc_sequence {
    pub base: drm_event,
    pub user_data: __u64,
    pub time_ns: __s64,
    pub sequence: __u64,
}

/* typedef area */
pub type drm_clip_rect_t = drm_clip_rect;
pub type drm_drawable_info_t = drm_drawable_info;
pub type drm_tex_region_t = drm_tex_region;
pub type drm_hw_lock_t = drm_hw_lock;
pub type drm_version_t = drm_version;
pub type drm_unique_t = drm_unique;
pub type drm_list_t = drm_list;
pub type drm_block_t = drm_block;
pub type drm_control_t = drm_control;
pub type drm_map_type_t = drm_map_type;
pub type drm_map_flags_t = drm_map_flags;
pub type drm_ctx_priv_map_t = drm_ctx_priv_map;
pub type drm_map_t = drm_map;
pub type drm_client_t = drm_client;
pub type drm_stat_type_t = drm_stat_type;
pub type drm_stats_t = drm_stats;
pub type drm_lock_flags_t = drm_lock_flags;
pub type drm_lock_t = drm_lock;
pub type drm_dma_flags_t = drm_dma_flags;
pub type drm_buf_desc_t = drm_buf_desc;
pub type drm_buf_info_t = drm_buf_info;
pub type drm_buf_free_t = drm_buf_free;
pub type drm_buf_pub_t = drm_buf_pub;
pub type drm_buf_map_t = drm_buf_map;
pub type drm_dma_t = drm_dma;
pub type drm_wait_vblank_t = drm_wait_vblank;
pub type drm_agp_mode_t = drm_agp_mode;
pub type drm_ctx_flags_t = drm_ctx_flags;
pub type drm_ctx_t = drm_ctx;
pub type drm_ctx_res_t = drm_ctx_res;
pub type drm_draw_t = drm_draw;
pub type drm_update_draw_t = drm_update_draw;
pub type drm_auth_t = drm_auth;
pub type drm_irq_busid_t = drm_irq_busid;
pub type drm_vblank_seq_type_t = drm_vblank_seq_type;
pub type drm_agp_buffer_t = drm_agp_buffer;
pub type drm_agp_binding_t = drm_agp_binding;
pub type drm_agp_info_t = drm_agp_info;
pub type drm_scatter_gather_t = drm_scatter_gather;
pub type drm_set_version_t = drm_set_version;
