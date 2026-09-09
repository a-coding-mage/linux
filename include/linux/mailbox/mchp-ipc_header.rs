/* SPDX-License-Identifier: GPL-2.0 */
/*
 *Copyright (c) 2024 Microchip Technology Inc. All rights reserved.
 */

// Translated from mchp-ipc.h.
// External Linux mailbox and type definitions are supplied by other files.

#[repr(C)]
pub struct mchp_ipc_msg {
    pub buf: *mut u32,
    pub size: u16,
}

#[repr(C)]
pub struct mchp_ipc_sbi_chan {
    pub buf_base_tx: *mut core::ffi::c_void,
    pub buf_base_rx: *mut core::ffi::c_void,
    pub msg_buf_tx: *mut core::ffi::c_void,
    pub msg_buf_rx: *mut core::ffi::c_void,
    pub buf_base_tx_addr: phys_addr_t,
    pub buf_base_rx_addr: phys_addr_t,
    pub msg_buf_tx_addr: phys_addr_t,
    pub msg_buf_rx_addr: phys_addr_t,
    pub chan_aggregated_irq: i32,
    pub mp_irq: i32,
    pub mc_irq: i32,
    pub id: u32,
    pub max_msg_size: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
