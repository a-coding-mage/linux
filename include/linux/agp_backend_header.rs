/*
 * AGPGART backend specific includes. Not for userspace consumption.
 *
 * Copyright (C) 2004 Silicon Graphics, Inc.
 * Copyright (C) 2002-2003 Dave Jones
 * Copyright (C) 1999 Jeff Hartmann
 * Copyright (C) 1999 Precision Insight, Inc.
 * Copyright (C) 1999 Xi Graphics, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * JEFF HARTMANN, OR ANY OTHER CONTRIBUTORS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
 * OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#[repr(C)]
pub enum chipset_type {
    NOT_SUPPORTED = 0,
    SUPPORTED = 1,
}

#[repr(C)]
pub struct agp_version {
    pub major: u16,
    pub minor: u16,
}

#[repr(C)]
pub struct agp_kern_info {
    pub version: agp_version,
    pub device: *mut pci_dev,
    pub chipset: chipset_type,
    pub mode: libc::c_ulong,
    pub aper_base: libc::c_ulong,
    pub aper_size: usize,
    pub max_memory: libc::c_int, // In pages
    pub current_memory: libc::c_int,
    pub cant_use_aperture: bool,
    pub page_mask: libc::c_ulong,
    pub vm_ops: *const vm_operations_struct,
}

/*
 * The agp_memory structure has information about the block of agp memory
 * allocated.  A caller may manipulate the next and prev pointers to link
 * each allocated item into a list.  These pointers are ignored by the backend.
 * Everything else should never be written to, but the caller may read any
 * of the items to determine the status of this block of agp memory.
 */

#[repr(C)]
pub struct agp_bridge_data;

#[repr(C)]
pub struct agp_memory {
    pub next: *mut agp_memory,
    pub prev: *mut agp_memory,
    pub bridge: *mut agp_bridge_data,
    pub pages: *mut *mut page,
    pub page_count: usize,
    pub key: libc::c_int,
    pub num_scratch_pages: libc::c_int,
    pub pg_start: libc::off_t,
    pub type_: u32,
    pub physical: u32,
    pub is_bound: bool,
    pub is_flushed: bool,
    /* list of agp_memory mapped to the aperture */
    pub mapped_list: list_head,
    /* DMA-mapped addresses */
    pub sg_list: *mut scatterlist,
    pub num_sg: libc::c_int,
}

pub const AGP_NORMAL_MEMORY: u32 = 0;
pub const AGP_USER_TYPES: u32 = 1 << 16;
pub const AGP_USER_MEMORY: u32 = AGP_USER_TYPES;
pub const AGP_USER_CACHED_MEMORY: u32 = AGP_USER_TYPES + 1;

extern "C" {
    pub static mut agp_bridge: *mut agp_bridge_data;
    pub static mut agp_bridges: list_head;

    pub static mut agp_find_bridge: Option<unsafe extern "C" fn(*mut pci_dev) -> *mut agp_bridge_data>;

    pub fn agp_free_memory(arg1: *mut agp_memory);
    pub fn agp_allocate_memory(arg1: *mut agp_bridge_data, arg2: usize, arg3: u32) -> *mut agp_memory;
    pub fn agp_copy_info(arg1: *mut agp_bridge_data, arg2: *mut agp_kern_info) -> libc::c_int;
    pub fn agp_bind_memory(arg1: *mut agp_memory, arg2: libc::off_t) -> libc::c_int;
    pub fn agp_unbind_memory(arg1: *mut agp_memory) -> libc::c_int;
    pub fn agp_enable(arg1: *mut agp_bridge_data, arg2: u32);
    pub fn agp_backend_acquire(arg1: *mut pci_dev) -> *mut agp_bridge_data;
    pub fn agp_backend_release(arg1: *mut agp_bridge_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
