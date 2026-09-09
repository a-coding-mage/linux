/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright IBM Corp. 2015
 *
 * Authors: Gavin Shan <gwshan@linux.vnet.ibm.com>
 */

/* PE states */
pub const EEH_PE_STATE_NORMAL: u32 = 0; /* Normal state */
pub const EEH_PE_STATE_RESET: u32 = 1; /* PE reset asserted */
pub const EEH_PE_STATE_STOPPED_IO_DMA: u32 = 2; /* Frozen PE */
pub const EEH_PE_STATE_STOPPED_DMA: u32 = 4; /* Stopped DMA only */
pub const EEH_PE_STATE_UNAVAIL: u32 = 5; /* Unavailable */

/* EEH error types and functions */
pub const EEH_ERR_TYPE_32: u32 = 0; /* 32-bits error */
pub const EEH_ERR_TYPE_64: u32 = 1; /* 64-bits error */
pub const EEH_ERR_FUNC_MIN: u32 = 0;
pub const EEH_ERR_FUNC_LD_MEM_ADDR: u32 = 0; /* Memory load */
pub const EEH_ERR_FUNC_LD_MEM_DATA: u32 = 1;
pub const EEH_ERR_FUNC_LD_IO_ADDR: u32 = 2; /* IO load */
pub const EEH_ERR_FUNC_LD_IO_DATA: u32 = 3;
pub const EEH_ERR_FUNC_LD_CFG_ADDR: u32 = 4; /* Config load */
pub const EEH_ERR_FUNC_LD_CFG_DATA: u32 = 5;
pub const EEH_ERR_FUNC_ST_MEM_ADDR: u32 = 6; /* Memory store */
pub const EEH_ERR_FUNC_ST_MEM_DATA: u32 = 7;
pub const EEH_ERR_FUNC_ST_IO_ADDR: u32 = 8; /* IO store */
pub const EEH_ERR_FUNC_ST_IO_DATA: u32 = 9;
pub const EEH_ERR_FUNC_ST_CFG_ADDR: u32 = 10; /* Config store */
pub const EEH_ERR_FUNC_ST_CFG_DATA: u32 = 11;
pub const EEH_ERR_FUNC_DMA_RD_ADDR: u32 = 12; /* DMA read */
pub const EEH_ERR_FUNC_DMA_RD_DATA: u32 = 13;
pub const EEH_ERR_FUNC_DMA_RD_MASTER: u32 = 14;
pub const EEH_ERR_FUNC_DMA_RD_TARGET: u32 = 15;
pub const EEH_ERR_FUNC_DMA_WR_ADDR: u32 = 16; /* DMA write */
pub const EEH_ERR_FUNC_DMA_WR_DATA: u32 = 17;
pub const EEH_ERR_FUNC_DMA_WR_MASTER: u32 = 18;
pub const EEH_ERR_FUNC_DMA_WR_TARGET: u32 = 19;
pub const EEH_ERR_FUNC_MAX: u32 = 19;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
