/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Copyright (c) 2025 MediaTek Inc.
 */

pub const MTK_VCP_MBOX_SLOT_MAX_SIZE: u32 = 0x100; /* mbox max slot size */

/**
 * struct mtk_ipi_info - mailbox message info for mtk-vcp-mailbox
 * @msg: The share buffer between IPC and mailbox driver
 * @len: Message length
 * @id: This is for identification purposes and not actually used
 *	by the mailbox hardware.
 * @index: The signal number of the mailbox message.
 * @slot_ofs: Data slot offset.
 * @irq_status: Captures incoming signals for the RX path.
 *
 * It is used between IPC with mailbox driver.
 */
#[repr(C)]
pub struct mtk_ipi_info {
    pub msg: *mut core::ffi::c_void,
    pub len: u32,
    pub id: u32,
    pub index: u32,
    pub slot_ofs: u32,
    pub irq_status: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
