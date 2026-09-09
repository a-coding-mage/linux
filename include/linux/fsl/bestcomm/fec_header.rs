/*
 * Header for Bestcomm FEC tasks driver
 *
 *
 * Copyright (C) 2006-2007 Sylvain Munaut <tnt@246tNt.com>
 * Copyright (C) 2003-2004 MontaVista, Software, Inc.
 *                         ( by Dale Farnsworth <dfarnsworth@mvista.com> )
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

// #ifndef __BESTCOMM_FEC_H__
// #define __BESTCOMM_FEC_H__

#[repr(C)]
pub struct bcom_fec_bd {
    pub status: u32,
    pub skb_pa: u32,
}

pub const BCOM_FEC_TX_BD_TFD: u32 = 0x08000000u32; /* transmit frame done */
pub const BCOM_FEC_TX_BD_TC: u32 = 0x04000000u32; /* transmit CRC */
pub const BCOM_FEC_TX_BD_ABC: u32 = 0x02000000u32; /* append bad CRC */

pub const BCOM_FEC_RX_BD_L: u32 = 0x08000000u32; /* buffer is last in frame */
pub const BCOM_FEC_RX_BD_BC: u32 = 0x00800000u32; /* DA is broadcast */
pub const BCOM_FEC_RX_BD_MC: u32 = 0x00400000u32; /* DA is multicast and not broadcast */
pub const BCOM_FEC_RX_BD_LG: u32 = 0x00200000u32; /* Rx frame length violation */
pub const BCOM_FEC_RX_BD_NO: u32 = 0x00100000u32; /* Rx non-octet aligned frame */
pub const BCOM_FEC_RX_BD_CR: u32 = 0x00040000u32; /* Rx CRC error */
pub const BCOM_FEC_RX_BD_OV: u32 = 0x00020000u32; /* overrun */
pub const BCOM_FEC_RX_BD_TR: u32 = 0x00010000u32; /* Rx frame truncated */
pub const BCOM_FEC_RX_BD_LEN_MASK: u32 = 0x000007ffu32; /* mask for length of received frame */
pub const BCOM_FEC_RX_BD_ERRORS: u32 = BCOM_FEC_RX_BD_LG
    | BCOM_FEC_RX_BD_NO
    | BCOM_FEC_RX_BD_CR
    | BCOM_FEC_RX_BD_OV
    | BCOM_FEC_RX_BD_TR;

extern "C" {
    pub fn bcom_fec_rx_init(
        queue_len: ::core::ffi::c_int,
        fifo: phys_addr_t,
        maxbufsize: ::core::ffi::c_int,
    ) -> *mut bcom_task;

    pub fn bcom_fec_rx_reset(tsk: *mut bcom_task) -> ::core::ffi::c_int;

    pub fn bcom_fec_rx_release(tsk: *mut bcom_task);

    pub fn bcom_fec_tx_init(queue_len: ::core::ffi::c_int, fifo: phys_addr_t)
        -> *mut bcom_task;

    pub fn bcom_fec_tx_reset(tsk: *mut bcom_task) -> ::core::ffi::c_int;

    pub fn bcom_fec_tx_release(tsk: *mut bcom_task);
}

// #endif /* __BESTCOMM_FEC_H__ */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
