/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR MIT) */
/*
 * Interface to /dev/xen/gntdev.
 *
 * Copyright (c) 2007, D G Murray
 * Copyright (c) 2018, Oleksandr Andrushchenko, EPAM Systems Inc.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License version 2
 * as published by the Free Software Foundation; or, when distributed
 * separately from the Linux kernel or incorporated into other
 * software packages, subject to the following license:
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this source file (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use, copy, modify,
 * merge, publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

#[repr(C)]
pub struct ioctl_gntdev_grant_ref {
    /* The domain ID of the grant to be mapped. */
    pub domid: __u32,
    /* The grant reference of the grant to be mapped. */
    pub ref_: __u32,
}

#[repr(C)]
pub struct ioctl_gntdev_map_grant_ref {
    /* IN parameters */
    /* The number of grants to be mapped. */
    pub count: __u32,
    pub pad: __u32,
    /* OUT parameters */
    /* The offset to be used on a subsequent call to mmap(). */
    pub index: __u64,
    /* Variable IN parameter. */
    /* Array of grant references, of size @count. */
    pub refs: [ioctl_gntdev_grant_ref; 1],
}

#[repr(C)]
pub struct ioctl_gntdev_unmap_grant_ref {
    /* IN parameters */
    /* The offset was returned by the corresponding map operation. */
    pub index: __u64,
    /* The number of pages to be unmapped. */
    pub count: __u32,
    pub pad: __u32,
}

#[repr(C)]
pub struct ioctl_gntdev_get_offset_for_vaddr {
    /* IN parameters */
    /* The virtual address of the first mapped page in a range. */
    pub vaddr: __u64,
    /* OUT parameters */
    /* The offset that was used in the initial mmap() operation. */
    pub offset: __u64,
    /* The number of pages mapped in the VM area that begins at @vaddr. */
    pub count: __u32,
    pub pad: __u32,
}

#[repr(C)]
pub struct ioctl_gntdev_set_max_grants {
    /* IN parameter */
    /* The maximum number of grants that may be mapped at once. */
    pub count: __u32,
}

#[repr(C)]
pub struct ioctl_gntdev_unmap_notify {
    /* IN parameters */
    /* Offset in the file descriptor for a byte within the page. */
    pub index: __u64,
    /* Action(s) to take on unmap */
    pub action: __u32,
    /* Event channel to notify */
    pub event_channel_port: __u32,
}

#[repr(C)]
pub union gntdev_grant_copy_segment_source_dest {
    pub virt: *mut core::ffi::c_void,
    pub foreign: gntdev_grant_copy_segment_foreign,
}

#[repr(C)]
pub struct gntdev_grant_copy_segment_foreign {
    pub ref_: grant_ref_t,
    pub offset: __u16,
    pub domid: domid_t,
}

#[repr(C)]
pub struct gntdev_grant_copy_segment {
    pub source: gntdev_grant_copy_segment_source_dest,
    pub dest: gntdev_grant_copy_segment_source_dest,
    pub len: __u16,
    pub flags: __u16,  /* GNTCOPY_* */
    pub status: __s16, /* GNTST_* */
}

#[repr(C)]
pub struct ioctl_gntdev_grant_copy {
    pub count: core::ffi::c_uint,
    pub segments: *mut gntdev_grant_copy_segment,
}

/* Clear (set to zero) the byte specified by index */
pub const UNMAP_NOTIFY_CLEAR_BYTE: u32 = 0x1;
/* Send an interrupt on the indicated event channel */
pub const UNMAP_NOTIFY_SEND_EVENT: u32 = 0x2;

/* The buffer is backed with memory allocated with dma_alloc_wc. */
pub const GNTDEV_DMA_FLAG_WC: u32 = 1 << 0;
/* The buffer is backed with memory allocated with dma_alloc_coherent. */
pub const GNTDEV_DMA_FLAG_COHERENT: u32 = 1 << 1;

#[repr(C)]
pub struct ioctl_gntdev_dmabuf_exp_from_refs {
    pub flags: __u32,
    pub count: __u32,
    pub fd: __u32,
    pub domid: __u32,
    pub refs: [__u32; 1],
}

#[repr(C)]
pub struct ioctl_gntdev_dmabuf_exp_wait_released {
    pub fd: __u32,
    pub wait_to_ms: __u32,
}

#[repr(C)]
pub struct ioctl_gntdev_dmabuf_imp_to_refs {
    pub fd: __u32,
    pub count: __u32,
    pub domid: __u32,
    pub reserved: __u32,
    pub refs: [__u32; 1],
}

#[repr(C)]
pub struct ioctl_gntdev_dmabuf_imp_release {
    pub fd: __u32,
    pub reserved: __u32,
}

/* ioctl values retain the source header's dependency on the Linux _IOC ABI. */
pub const IOCTL_GNTDEV_MAP_GRANT_REF: _IOC_TYPE = _IOC(_IOC_NONE, 'G' as _IOC_TYPE, 0, core::mem::size_of::<ioctl_gntdev_map_grant_ref>());
pub const IOCTL_GNTDEV_UNMAP_GRANT_REF: _IOC_TYPE = _IOC(_IOC_NONE, 'G' as _IOC_TYPE, 1, core::mem::size_of::<ioctl_gntdev_unmap_grant_ref>());
pub const IOCTL_GNTDEV_GET_OFFSET_FOR_VADDR: _IOC_TYPE = _IOC(_IOC_NONE, 'G' as _IOC_TYPE, 2, core::mem::size_of::<ioctl_gntdev_get_offset_for_vaddr>());
pub const IOCTL_GNTDEV_SET_MAX_GRANTS: _IOC_TYPE = _IOC(_IOC_NONE, 'G' as _IOC_TYPE, 3, core::mem::size_of::<ioctl_gntdev_set_max_grants>());
pub const IOCTL_GNTDEV_SET_UNMAP_NOTIFY: _IOC_TYPE = _IOC(_IOC_NONE, 'G' as _IOC_TYPE, 7, core::mem::size_of::<ioctl_gntdev_unmap_notify>());
pub const IOCTL_GNTDEV_GRANT_COPY: _IOC_TYPE = _IOC(_IOC_NONE, 'G' as _IOC_TYPE, 8, core::mem::size_of::<ioctl_gntdev_grant_copy>());
pub const IOCTL_GNTDEV_DMABUF_EXP_FROM_REFS: _IOC_TYPE = _IOC(_IOC_NONE, 'G' as _IOC_TYPE, 9, core::mem::size_of::<ioctl_gntdev_dmabuf_exp_from_refs>());
pub const IOCTL_GNTDEV_DMABUF_EXP_WAIT_RELEASED: _IOC_TYPE = _IOC(_IOC_NONE, 'G' as _IOC_TYPE, 10, core::mem::size_of::<ioctl_gntdev_dmabuf_exp_wait_released>());
pub const IOCTL_GNTDEV_DMABUF_IMP_TO_REFS: _IOC_TYPE = _IOC(_IOC_NONE, 'G' as _IOC_TYPE, 11, core::mem::size_of::<ioctl_gntdev_dmabuf_imp_to_refs>());
pub const IOCTL_GNTDEV_DMABUF_IMP_RELEASE: _IOC_TYPE = _IOC(_IOC_NONE, 'G' as _IOC_TYPE, 12, core::mem::size_of::<ioctl_gntdev_dmabuf_imp_release>());

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
