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

/* Dependency intent: __beXX and byte-order definitions are supplied externally. */

/* ERSPAN version 2 metadata header */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erspan_md2 {
    pub timestamp: u32,
    pub sgt: u16, /* security group tag */

    /*
     * These bytes contain the C bitfields. On little endian bitfields:
     *   byte 0: hwid_upper:2, ft:5, p:1
     *   byte 1: o:1, gra:2, dir:1, hwid:4
     * On big endian bitfields:
     *   byte 0: p:1, ft:5, hwid_upper:2
     *   byte 1: hwid:4, dir:1, gra:2, o:1
     */
    pub bitfield_0: u8,
    pub bitfield_1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union erspan_metadata_u {
    /* Version 1 (type II) */
    pub index: u32,
    /* Version 2 (type III) */
    pub md2: erspan_md2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erspan_metadata {
    pub version: i32,
    pub u: erspan_metadata_u,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
