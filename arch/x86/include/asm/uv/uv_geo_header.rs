/* SPDX-License-Identifier: GPL-2.0-or-later
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2020 Hewlett Packard Enterprise Development LP. All rights reserved.
 */

/* Type declarations */

/* Size of a geoid_s structure (must be before decl. of geoid_u) */
pub const GEOID_SIZE: usize = 8;

/* Fields common to all substructures */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct geo_common_s {
    pub type_: u8, /* What type of h/w is named by this geoid_s */
    pub blade: u8,
    pub slot: u8, /* slot is IRU */
    pub upos: u8,
    pub rack: u8,
}

/* Additional fields for particular types of hardware */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct geo_node_s {
    pub common: geo_common_s, /* No additional fields needed */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct geo_rtr_s {
    pub common: geo_common_s, /* No additional fields needed */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct geo_iocntl_s {
    pub common: geo_common_s, /* No additional fields needed */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct geo_pcicard_s {
    pub common: geo_iocntl_s,
    pub bus: i8, /* Bus/widget number */
    pub slot: i8, /* PCI slot number */
}

/* Subcomponents of a node */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct geo_cpu_s {
    pub node: geo_node_s,
    /* C bit-fields: socket:4 (Which CPU on the node), thread:4 */
    pub socket_thread: u8,
    pub core: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct geo_mem_s {
    pub node: geo_node_s,
    pub membus: i8, /* The memory bus on the node */
    pub memslot: i8, /* The memory slot on the bus */
}

#[repr(C)]
pub union geoid_u {
    pub common: geo_common_s,
    pub node: geo_node_s,
    pub iocntl: geo_iocntl_s,
    pub pcicard: geo_pcicard_s,
    pub rtr: geo_rtr_s,
    pub cpu: geo_cpu_s,
    pub mem: geo_mem_s,
    pub padsize: [i8; GEOID_SIZE],
}

/* Defined constants */

pub const GEO_MAX_LEN: usize = 48;

pub const GEO_TYPE_INVALID: i32 = 0;
pub const GEO_TYPE_MODULE: i32 = 1;
pub const GEO_TYPE_NODE: i32 = 2;
pub const GEO_TYPE_RTR: i32 = 3;
pub const GEO_TYPE_IOCNTL: i32 = 4;
pub const GEO_TYPE_IOCARD: i32 = 5;
pub const GEO_TYPE_CPU: i32 = 6;
pub const GEO_TYPE_MEM: i32 = 7;
pub const GEO_TYPE_MAX: i32 = GEO_TYPE_MEM + 1;

pub unsafe fn geo_rack(g: geoid_u) -> i32 {
    (if g.common.type_ == GEO_TYPE_INVALID as u8 {
        -1
    } else {
        g.common.rack as i32
    })
}

pub unsafe fn geo_slot(g: geoid_u) -> i32 {
    (if g.common.type_ == GEO_TYPE_INVALID as u8 {
        -1
    } else {
        g.common.upos as i32
    })
}

pub unsafe fn geo_blade(g: geoid_u) -> i32 {
    (if g.common.type_ == GEO_TYPE_INVALID as u8 {
        -1
    } else {
        g.common.blade as i32 * 2 + g.common.slot as i32
    })
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
