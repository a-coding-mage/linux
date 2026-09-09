/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright 2021 NXP
 *
 * Header file for the IPC implementation.
 */

#[repr(C)]
pub struct imx_s4_ipc {
    _private: [u8; 0],
}

#[repr(C, packed)]
pub struct imx_s4_rpc_msg {
    pub ver: u8,
    pub size: u8,
    pub cmd: u8,
    pub tag: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
