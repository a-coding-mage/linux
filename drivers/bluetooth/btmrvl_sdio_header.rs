/* SPDX-License-Identifier: GPL-2.0-only */
/**
 * Marvell BT-over-SDIO driver: SDIO interface related definitions
 *
 * Copyright (C) 2009, Marvell International Ltd.
 **/

pub const SDIO_HEADER_LEN: usize = 4;

/* SD block size can not bigger than 64 due to buf size limit in firmware */
/* define SD block size for data Tx/Rx */
pub const SDIO_BLOCK_SIZE: usize = 64;

/* Number of blocks for firmware transfer */
pub const FIRMWARE_TRANSFER_NBLOCK: usize = 2;

/* This is for firmware specific length */
pub const FW_EXTRA_LEN: usize = 36;

pub const MRVDRV_SIZE_OF_CMD_BUFFER: usize = 2 * 1024;

pub const MRVDRV_BT_RX_PACKET_BUFFER_SIZE: usize = HCI_MAX_FRAME_SIZE + FW_EXTRA_LEN;

pub const ALLOC_BUF_SIZE: usize =
    (((if MRVDRV_BT_RX_PACKET_BUFFER_SIZE > MRVDRV_SIZE_OF_CMD_BUFFER {
        MRVDRV_BT_RX_PACKET_BUFFER_SIZE
    } else {
        MRVDRV_SIZE_OF_CMD_BUFFER
    }) + SDIO_HEADER_LEN + SDIO_BLOCK_SIZE - 1) / SDIO_BLOCK_SIZE) * SDIO_BLOCK_SIZE;

/* The number of times to try when polling for status */
pub const MAX_POLL_TRIES: usize = 100;

/* Max retry number of CMD53 write */
pub const MAX_WRITE_IOMEM_RETRY: usize = 2;

/* register bitmasks */
pub const HOST_POWER_UP: u8 = 1 << 1;
pub const HOST_CMD53_FIN: u8 = 1 << 2;

pub const HIM_DISABLE: u8 = 0xff;
pub const HIM_ENABLE: u8 = (1 << 0) | (1 << 1);

pub const UP_LD_HOST_INT_STATUS: u8 = 1 << 0;
pub const DN_LD_HOST_INT_STATUS: u8 = 1 << 1;

pub const DN_LD_CARD_RDY: u8 = 1 << 0;
pub const CARD_IO_READY: u8 = 1 << 3;

pub const FIRMWARE_READY: u16 = 0xfedc;

#[repr(C)]
pub struct btmrvl_plt_wake_cfg {
    pub irq_bt: i32,
    pub wake_by_bt: bool,
}

#[repr(C)]
pub struct btmrvl_sdio_card_reg {
    pub cfg: u8,
    pub host_int_mask: u8,
    pub host_intstatus: u8,
    pub card_status: u8,
    pub sq_read_base_addr_a0: u8,
    pub sq_read_base_addr_a1: u8,
    pub card_revision: u8,
    pub card_fw_status0: u8,
    pub card_fw_status1: u8,
    pub card_rx_len: u8,
    pub card_rx_unit: u8,
    pub io_port_0: u8,
    pub io_port_1: u8,
    pub io_port_2: u8,
    pub int_read_to_clear: bool,
    pub host_int_rsr: u8,
    pub card_misc_cfg: u8,
    pub fw_dump_ctrl: u8,
    pub fw_dump_start: u8,
    pub fw_dump_end: u8,
}

#[repr(C)]
pub struct btmrvl_sdio_card {
    pub func: *mut sdio_func,
    pub ioport: u32,
    pub helper: *const core::ffi::c_char,
    pub firmware: *const core::ffi::c_char,
    pub reg: *const btmrvl_sdio_card_reg,
    pub support_pscan_win_report: bool,
    pub supports_fw_dump: bool,
    pub sd_blksz_fw_dl: u16,
    pub rx_unit: u8,
    pub priv_: *mut btmrvl_private,
    pub plt_of_node: *mut device_node,
    pub plt_wake_cfg: *mut btmrvl_plt_wake_cfg,
}

#[repr(C)]
pub struct btmrvl_sdio_device {
    pub helper: *const core::ffi::c_char,
    pub firmware: *const core::ffi::c_char,
    pub reg: *const btmrvl_sdio_card_reg,
    pub support_pscan_win_report: bool,
    pub sd_blksz_fw_dl: u16,
    pub supports_fw_dump: bool,
}

/* Platform specific DMA alignment */
pub const BTSDIO_DMA_ALIGN: usize = 8;

/* Macros for Data Alignment : size */
#[macro_export]
macro_rules! ALIGN_SZ {
    ($p:expr, $a:expr) => {
        (($p + ($a - 1)) & !($a - 1))
    };
}

/* Macros for Data Alignment : address */
#[macro_export]
macro_rules! ALIGN_ADDR {
    ($p:expr, $a:expr) => {
        (($p as usize + ($a as usize - 1)) & !($a as usize - 1))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
