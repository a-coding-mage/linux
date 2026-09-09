/* SPDX-License-Identifier: GPL-2.0 */
/*
 * BlueZ - Bluetooth protocol stack for Linux
 *
 * Copyright (C) 2022 Intel Corporation
 */

/* ISO defaults */
pub const ISO_DEFAULT_MTU: i32 = 251;
pub const ISO_MAX_NUM_BIS: usize = 0x1f;

/* ISO socket broadcast address */
#[repr(C)]
pub struct sockaddr_iso_bc {
    pub bc_bdaddr: bdaddr_t,
    pub bc_bdaddr_type: __u8,
    pub bc_sid: __u8,
    pub bc_num_bis: __u8,
    pub bc_bis: [__u8; ISO_MAX_NUM_BIS],
}

/* ISO socket address */
#[repr(C)]
pub struct sockaddr_iso {
    pub iso_family: sa_family_t,
    pub iso_bdaddr: bdaddr_t,
    pub iso_bdaddr_type: __u8,
    /* Flexible array member: struct sockaddr_iso_bc iso_bc[]; */
    pub iso_bc: [sockaddr_iso_bc; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
