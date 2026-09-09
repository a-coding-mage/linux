/*
 * AGPGART module version 0.99
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * JEFF HARTMANN, OR ANY OTHER CONTRIBUTORS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
 * OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

pub const AGPIOC_BASE: u8 = b'A';

// `_IOR`, `_IO`, `_IOW`, and `_IOWR` are supplied by the target ioctl
// environment; these declarations preserve the source macro invocations.
pub const AGPIOC_INFO: _ = _IOR!(AGPIOC_BASE, 0, *mut agp_info);
pub const AGPIOC_ACQUIRE: _ = _IO!(AGPIOC_BASE, 1);
pub const AGPIOC_RELEASE: _ = _IO!(AGPIOC_BASE, 2);
pub const AGPIOC_SETUP: _ = _IOW!(AGPIOC_BASE, 3, *mut agp_setup);
pub const AGPIOC_RESERVE: _ = _IOW!(AGPIOC_BASE, 4, *mut agp_region);
pub const AGPIOC_PROTECT: _ = _IOW!(AGPIOC_BASE, 5, *mut agp_region);
pub const AGPIOC_ALLOCATE: _ = _IOWR!(AGPIOC_BASE, 6, *mut agp_allocate);
pub const AGPIOC_DEALLOCATE: _ = _IOW!(AGPIOC_BASE, 7, i32);
pub const AGPIOC_BIND: _ = _IOW!(AGPIOC_BASE, 8, *mut agp_bind);
pub const AGPIOC_UNBIND: _ = _IOW!(AGPIOC_BASE, 9, *mut agp_unbind);
pub const AGPIOC_CHIPSET_FLUSH: _ = _IO!(AGPIOC_BASE, 10);

pub const AGP_DEVICE: &str = "/dev/agpgart";

pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;

#[repr(C)]
pub struct agp_version {
    pub major: __u16,
    pub minor: __u16,
}

#[repr(C)]
pub struct agp_info {
    pub version: agp_version,
    pub bridge_id: __u32,
    pub agp_mode: __u32,
    pub aper_base: c_ulong,
    pub aper_size: __kernel_size_t,
    pub pg_total: __kernel_size_t,
    pub pg_system: __kernel_size_t,
    pub pg_used: __kernel_size_t,
}

#[repr(C)]
pub struct agp_setup {
    pub agp_mode: __u32,
}

/* The "prot" down below needs still a "sleep" flag somehow ... */
#[repr(C)]
pub struct agp_segment {
    pub pg_start: __kernel_off_t,
    pub pg_count: __kernel_size_t,
    pub prot: i32,
}

#[repr(C)]
pub struct agp_region {
    pub pid: __kernel_pid_t,
    pub seg_count: __kernel_size_t,
    pub seg_list: *mut agp_segment,
}

#[repr(C)]
pub struct agp_allocate {
    pub key: i32,
    pub pg_count: __kernel_size_t,
    pub type_: __u32,
    pub physical: __u32,
}

#[repr(C)]
pub struct agp_bind {
    pub key: i32,
    pub pg_start: __kernel_off_t,
}

#[repr(C)]
pub struct agp_unbind {
    pub key: i32,
    pub priority: __u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
