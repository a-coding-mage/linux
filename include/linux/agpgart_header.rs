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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * JEFF HARTMANN, OR ANY OTHER CONTRIBUTORS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
 * OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the corresponding kernel headers:
// linux/mutex.h, linux/agp_backend.h, and uapi/linux/agpgart.h.

#[repr(C)]
pub struct agp_info {
    pub version: agp_version,       /* version of the driver        */
    pub bridge_id: u32,             /* bridge vendor/device         */
    pub agp_mode: u32,              /* mode info of bridge          */
    pub aper_base: usize,           /* base of aperture             */
    pub aper_size: usize,           /* size of aperture             */
    pub pg_total: usize,            /* max pages (swap + system)    */
    pub pg_system: usize,           /* max pages (system)           */
    pub pg_used: usize,             /* current pages used           */
}

#[repr(C)]
pub struct agp_setup {
    pub agp_mode: u32,              /* mode info of bridge          */
}

/*
 * The "prot" down below needs still a "sleep" flag somehow ...
 */
#[repr(C)]
pub struct agp_segment {
    pub pg_start: isize,             /* starting page to populate    */
    pub pg_count: usize,             /* number of pages              */
    pub prot: i32,                   /* prot flags for mmap          */
}

#[repr(C)]
pub struct agp_segment_priv {
    pub pg_start: isize,
    pub pg_count: usize,
    pub prot: pgprot_t,
}

#[repr(C)]
pub struct agp_region {
    pub pid: i32,                    /* pid of process               */
    pub seg_count: usize,            /* number of segments           */
    pub seg_list: *mut agp_segment,
}

#[repr(C)]
pub struct agp_allocate {
    pub key: i32,                    /* tag of allocation            */
    pub pg_count: usize,             /* number of pages              */
    pub type_: u32,                  /* 0 == normal, other devspec   */
    pub physical: u32,               /* device specific (some devices */
                                      /* need a phys address of the     */
                                      /* actual page behind the gatt    */
                                      /* table)                        */
}

#[repr(C)]
pub struct agp_bind {
    pub key: i32,                    /* tag of allocation            */
    pub pg_start: isize,             /* starting page to populate    */
}

#[repr(C)]
pub struct agp_unbind {
    pub key: i32,                    /* tag of allocation            */
    pub priority: u32,               /* priority for paging out      */
}

#[repr(C)]
pub struct agp_client {
    pub next: *mut agp_client,
    pub prev: *mut agp_client,
    pub pid: i32,
    pub num_segments: i32,
    pub segments: *mut *mut agp_segment_priv,
}

#[repr(C)]
pub struct agp_controller {
    pub next: *mut agp_controller,
    pub prev: *mut agp_controller,
    pub pid: i32,
    pub num_clients: i32,
    pub pool: *mut agp_memory,
    pub clients: *mut agp_client,
}

pub const AGP_FF_ALLOW_CLIENT: i32 = 0;
pub const AGP_FF_ALLOW_CONTROLLER: i32 = 1;
pub const AGP_FF_IS_CLIENT: i32 = 2;
pub const AGP_FF_IS_CONTROLLER: i32 = 3;
pub const AGP_FF_IS_VALID: i32 = 4;

#[repr(C)]
pub struct agp_file_private {
    pub next: *mut agp_file_private,
    pub prev: *mut agp_file_private,
    pub my_pid: i32,
    pub access_flags: usize,         /* long req'd for set_bit --RR */
}

#[repr(C)]
pub struct agp_front_data {
    pub agp_mutex: mutex,
    pub current_controller: *mut agp_controller,
    pub controllers: *mut agp_controller,
    pub file_priv_list: *mut agp_file_private,
    pub used_by_controller: bool,
    pub backend_acquired: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
