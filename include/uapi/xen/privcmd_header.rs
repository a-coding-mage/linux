/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR MIT) */
/*
 * privcmd.h
 *
 * Interface to /proc/xen/privcmd.
 *
 * Copyright (c) 2003-2005, K A Fraser
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
pub struct privcmd_hypercall {
    pub op: u64,
    pub arg: [u64; 5],
}

#[repr(C)]
pub struct privcmd_mmap_entry {
    pub va: u64,
    /* This should be a GFN. It's not possible to change the name because
     * it's exposed to the user-space. */
    pub mfn: u64,
    pub npages: u64,
}

#[repr(C)]
pub struct privcmd_mmap {
    pub num: i32,
    pub dom: domid_t, /* target domain */
    pub entry: *mut privcmd_mmap_entry,
}

#[repr(C)]
pub struct privcmd_mmapbatch {
    pub num: i32, /* number of pages to populate */
    pub dom: domid_t, /* target domain */
    pub addr: u64, /* virtual address */
    pub arr: *mut xen_pfn_t, /* array of mfns - or'd with PRIVCMD_MMAPBATCH_*_ERROR on err */
}

pub const PRIVCMD_MMAPBATCH_MFN_ERROR: u32 = 0xf0000000u32;
pub const PRIVCMD_MMAPBATCH_PAGED_ERROR: u32 = 0x80000000u32;

#[repr(C)]
pub struct privcmd_mmapbatch_v2 {
    pub num: u32, /* number of pages to populate */
    pub dom: domid_t, /* target domain */
    pub addr: u64, /* virtual address */
    pub arr: *const xen_pfn_t, /* array of mfns */
    pub err: *mut i32, /* array of error codes */
}

#[repr(C)]
pub struct privcmd_dm_op_buf {
    pub uptr: *mut core::ffi::c_void,
    pub size: usize,
}

#[repr(C)]
pub struct privcmd_dm_op {
    pub dom: domid_t,
    pub num: u16,
    pub ubufs: *const privcmd_dm_op_buf,
}

#[repr(C)]
pub struct privcmd_mmap_resource {
    pub dom: domid_t,
    pub type_: u32,
    pub id: u32,
    pub idx: u32,
    pub num: u64,
    pub addr: u64,
}

/* For privcmd_irqfd::flags */
pub const PRIVCMD_IRQFD_FLAG_DEASSIGN: u32 = 1 << 0;

#[repr(C)]
pub struct privcmd_irqfd {
    pub dm_op: u64,
    pub size: u32, /* Size of structure pointed by dm_op */
    pub fd: u32,
    pub flags: u32,
    pub dom: domid_t,
    pub pad: [u8; 2],
}

/* For privcmd_ioeventfd::flags */
pub const PRIVCMD_IOEVENTFD_FLAG_DEASSIGN: u32 = 1 << 0;

#[repr(C)]
pub struct privcmd_ioeventfd {
    pub ioreq: u64,
    pub ports: u64,
    pub addr: u64,
    pub addr_len: u32,
    pub event_fd: u32,
    pub vcpus: u32,
    pub vq: u32,
    pub flags: u32,
    pub dom: domid_t,
    pub pad: [u8; 2],
}

#[repr(C)]
pub struct privcmd_pcidev_get_gsi {
    pub sbdf: u32,
    pub gsi: u32,
}

/* IOCTL_PRIVCMD_* use the Linux _IOC/_IOW encoding supplied by dependencies. */
pub const IOCTL_PRIVCMD_HYPERCALL: usize = _IOC(_IOC_NONE, 'P' as u32, 0, core::mem::size_of::<privcmd_hypercall>());
pub const IOCTL_PRIVCMD_MMAP: usize = _IOC(_IOC_NONE, 'P' as u32, 2, core::mem::size_of::<privcmd_mmap>());
pub const IOCTL_PRIVCMD_MMAPBATCH: usize = _IOC(_IOC_NONE, 'P' as u32, 3, core::mem::size_of::<privcmd_mmapbatch>());
pub const IOCTL_PRIVCMD_MMAPBATCH_V2: usize = _IOC(_IOC_NONE, 'P' as u32, 4, core::mem::size_of::<privcmd_mmapbatch_v2>());
pub const IOCTL_PRIVCMD_DM_OP: usize = _IOC(_IOC_NONE, 'P' as u32, 5, core::mem::size_of::<privcmd_dm_op>());
pub const IOCTL_PRIVCMD_RESTRICT: usize = _IOC(_IOC_NONE, 'P' as u32, 6, core::mem::size_of::<domid_t>());
pub const IOCTL_PRIVCMD_MMAP_RESOURCE: usize = _IOC(_IOC_NONE, 'P' as u32, 7, core::mem::size_of::<privcmd_mmap_resource>());
pub const IOCTL_PRIVCMD_IRQFD: usize = _IOW('P' as u32, 8, privcmd_irqfd);
pub const IOCTL_PRIVCMD_IOEVENTFD: usize = _IOW('P' as u32, 9, privcmd_ioeventfd);
pub const IOCTL_PRIVCMD_PCIDEV_GET_GSI: usize = _IOC(_IOC_NONE, 'P' as u32, 10, core::mem::size_of::<privcmd_pcidev_get_gsi>());

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
