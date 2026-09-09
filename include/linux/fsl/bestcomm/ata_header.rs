/*
 * Header for Bestcomm ATA task driver
 *
 *
 * Copyright (C) 2006 Freescale - John Rigby
 * Copyright (C) 2006 Sylvain Munaut <tnt@246tNt.com>
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

#[repr(C)]
pub struct bcom_ata_bd {
    pub status: u32,
    pub src_pa: u32,
    pub dst_pa: u32,
}

extern "C" {
    pub fn bcom_ata_init(queue_len: i32, maxbufsize: i32) -> *mut bcom_task;
    pub fn bcom_ata_rx_prepare(tsk: *mut bcom_task);
    pub fn bcom_ata_tx_prepare(tsk: *mut bcom_task);
    pub fn bcom_ata_reset_bd(tsk: *mut bcom_task);
    pub fn bcom_ata_release(tsk: *mut bcom_task);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
