/*
 * Copyright 2013 Red Hat
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

/* Dependency: symbols from drm.h are supplied by the surrounding bindings. */

/* Please note that modifications to all structs defined here are
 * subject to backwards-compatibility constraints.
 *
 * Do not use pointers, use u64 instead for 32 bit / 64 bit user/kernel
 * compatibility. Keep fields aligned to their size.
 */

pub const QXL_GEM_DOMAIN_CPU: u32 = 0;
pub const QXL_GEM_DOMAIN_VRAM: u32 = 1;
pub const QXL_GEM_DOMAIN_SURFACE: u32 = 2;

pub const DRM_QXL_ALLOC: u32 = 0x00;
pub const DRM_QXL_MAP: u32 = 0x01;
pub const DRM_QXL_EXECBUFFER: u32 = 0x02;
pub const DRM_QXL_UPDATE_AREA: u32 = 0x03;
pub const DRM_QXL_GETPARAM: u32 = 0x04;
pub const DRM_QXL_CLIENTCAP: u32 = 0x05;
pub const DRM_QXL_ALLOC_SURF: u32 = 0x06;

#[repr(C)]
pub struct drm_qxl_alloc {
    pub size: u32,
    pub handle: u32, /* 0 is an invalid handle */
}

#[repr(C)]
pub struct drm_qxl_map {
    pub offset: u64, /* use for mmap system call */
    pub handle: u32,
    pub pad: u32,
}

/*
 * dest is the bo we are writing the relocation into
 * src is bo we are relocating.
 * *(dest_handle.base_addr + dest_offset) = physical_address(src_handle.addr +
 * src_offset)
 */
pub const QXL_RELOC_TYPE_BO: u32 = 1;
pub const QXL_RELOC_TYPE_SURF: u32 = 2;

#[repr(C)]
pub struct drm_qxl_reloc {
    pub src_offset: u64, /* offset into src_handle or src buffer */
    pub dst_offset: u64, /* offset in dest handle */
    pub src_handle: u32, /* dest handle to compute address from */
    pub dst_handle: u32, /* 0 if to command buffer */
    pub reloc_type: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct drm_qxl_command {
    pub command: u64, /* void* */
    pub relocs: u64, /* struct drm_qxl_reloc* */
    pub r#type: u32,
    pub command_size: u32,
    pub relocs_num: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct drm_qxl_execbuffer {
    pub flags: u32, /* for future use */
    pub commands_num: u32,
    pub commands: u64, /* struct drm_qxl_command* */
}

#[repr(C)]
pub struct drm_qxl_update_area {
    pub handle: u32,
    pub top: u32,
    pub left: u32,
    pub bottom: u32,
    pub right: u32,
    pub pad: u32,
}

pub const QXL_PARAM_NUM_SURFACES: u32 = 1; /* rom->n_surfaces */
pub const QXL_PARAM_MAX_RELOCS: u32 = 2;

#[repr(C)]
pub struct drm_qxl_getparam {
    pub param: u64,
    pub value: u64,
}

/* these are one bit values */
#[repr(C)]
pub struct drm_qxl_clientcap {
    pub index: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct drm_qxl_alloc_surf {
    pub format: u32,
    pub width: u32,
    pub height: u32,
    pub stride: i32,
    pub handle: u32,
    pub pad: u32,
}

/* DRM_IOWR/DRM_IOW and DRM_COMMAND_BASE are supplied by drm.h. */
#[macro_export]
macro_rules! DRM_IOCTL_QXL_ALLOC { () => { DRM_IOWR!(DRM_COMMAND_BASE + DRM_QXL_ALLOC, drm_qxl_alloc) }; }
#[macro_export]
macro_rules! DRM_IOCTL_QXL_MAP { () => { DRM_IOWR!(DRM_COMMAND_BASE + DRM_QXL_MAP, drm_qxl_map) }; }
#[macro_export]
macro_rules! DRM_IOCTL_QXL_EXECBUFFER { () => { DRM_IOW!(DRM_COMMAND_BASE + DRM_QXL_EXECBUFFER, drm_qxl_execbuffer) }; }
#[macro_export]
macro_rules! DRM_IOCTL_QXL_UPDATE_AREA { () => { DRM_IOW!(DRM_COMMAND_BASE + DRM_QXL_UPDATE_AREA, drm_qxl_update_area) }; }
#[macro_export]
macro_rules! DRM_IOCTL_QXL_GETPARAM { () => { DRM_IOWR!(DRM_COMMAND_BASE + DRM_QXL_GETPARAM, drm_qxl_getparam) }; }
#[macro_export]
macro_rules! DRM_IOCTL_QXL_CLIENTCAP { () => { DRM_IOW!(DRM_COMMAND_BASE + DRM_QXL_CLIENTCAP, drm_qxl_clientcap) }; }
#[macro_export]
macro_rules! DRM_IOCTL_QXL_ALLOC_SURF { () => { DRM_IOWR!(DRM_COMMAND_BASE + DRM_QXL_ALLOC_SURF, drm_qxl_alloc_surf) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
