/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2023-2026 Intel Corporation
 * Intel Silicon Security Engine Interface (ISSEI) Linux driver:
 * ISSEI Interface Header
 */

/*
 * This ioctl is used to associate the current file descriptor with a
 * FW Client (given by UUID). This opens a communication channel between a
 * host client and a FW client. From this point every read and write will
 * communicate with the associated FW client.
 *
 * The ioctl argument is a struct with a union that contains the input
 * parameter and the output parameter for this ioctl.
 *
 * The input parameter is UUID of the FW Client. The output parameter is the
 * properties of the FW client (FW protocol version, max message size and
 * client flags).
 *
 * C source: _IOWR('H', 0x01, struct issei_connect_client_data)
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct issei_client {
    pub max_msg_length: u32,
    pub protocol_version: u8,
    pub reserved1: [u8; 3],
    pub flags: u32,
    pub reserved2: u32,
}

pub const ISSEI_IOCTL_UUID_LEN: usize = 16;

/*
 * struct issei_connect_client_data - ioctl Connect Client Data structure
 * @in_client_uuid: unique id of the firmware client to connect to (from user
 * space to kernel)
 * @out_client_properties: connected firmware client properties (from kernel
 * to user space)
 */
#[repr(C)]
pub union issei_connect_client_data {
    pub in_client_uuid: [u8; ISSEI_IOCTL_UUID_LEN],
    pub out_client_properties: issei_client,
}

/* Linux _IOC encoding: direction 3 (read/write), type 'H', number 1. */
pub const IOCTL_ISSEI_CONNECT_CLIENT: u32 =
    (3u32 << 30) | ((core::mem::size_of::<issei_connect_client_data>() as u32) << 16)
        | (('H' as u32) << 8)
        | 0x01;

/* This ioctl terminates association between the host client and the FW client. */
/* C source: _IO('H', 0x02) */
pub const IOCTL_ISSEI_DISCONNECT_CLIENT: u32 = (('H' as u32) << 8) | 0x02;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
