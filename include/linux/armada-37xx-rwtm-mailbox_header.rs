/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * rWTM BIU Mailbox driver for Armada 37xx
 *
 * Author: Marek Behún <kabel@kernel.org>
 */

// Translated from the C header; `u16` and `u32` correspond to Linux kernel
// fixed-width integer types used by the original declarations.

#[repr(C)]
pub struct armada_37xx_rwtm_tx_msg {
    pub command: u16,
    pub args: [u32; 16],
}

#[repr(C)]
pub struct armada_37xx_rwtm_rx_msg {
    pub retval: u32,
    pub status: [u32; 16],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
