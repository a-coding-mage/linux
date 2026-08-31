/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * ERSPAN Tunnel Metadata
 *
 * Copyright (c) 2018 VMware
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2
 * as published by the Free Software Foundation.
 *
 * Userspace API for metadata mode ERSPAN tunnel
 */

/* Depends on linux/types.h for __beXX and __uXX userspace types. */
/* Depends on asm/byteorder.h for the original C bitfield byte order. */

/* ERSPAN version 2 metadata header */
#[repr(C)]
pub struct erspan_md2 {
    pub timestamp: __be32,
    pub sgt: __be16, /* security group tag */

    /*
     * Original C bitfield layout:
     *
     * Little endian bitfield:
     *   __u8 hwid_upper:2, ft:5, p:1;
     *   __u8 o:1, gra:2, dir:1, hwid:4;
     *
     * Big endian bitfield:
     *   __u8 p:1, ft:5, hwid_upper:2;
     *   __u8 hwid:4, dir:1, gra:2, o:1;
     */
    pub _bitfield_1: __u8,
    pub _bitfield_2: __u8,
}

#[repr(C)]
pub union erspan_metadata_u {
    pub index: __be32,        /* Version 1 (type II)*/
    pub md2: erspan_md2,      /* Version 2 (type III) */
}

#[repr(C)]
pub struct erspan_metadata {
    pub version: ::std::os::raw::c_int,
    pub u: erspan_metadata_u,
}
