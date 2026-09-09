/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2023 Cai Huoqing
 * Synopsys DesignWare HDMA v0 reg
 *
 * Author: Cai Huoqing <cai.huoqing@linux.dev>
 */

// Dependency intent: linux/dmaengine.h

pub const HDMA_V0_MAX_NR_CH: usize = 64;
pub const HDMA_V0_CH_EN: u32 = 1 << 0;
pub const HDMA_V0_LOCAL_ABORT_INT_EN: u32 = 1 << 6;
pub const HDMA_V0_REMOTE_ABORT_INT_EN: u32 = 1 << 5;
pub const HDMA_V0_LOCAL_STOP_INT_EN: u32 = 1 << 4;
pub const HDMA_V0_REMOTE_STOP_INT_EN: u32 = 1 << 3;
pub const HDMA_V0_ABORT_INT_MASK: u32 = 1 << 2;
pub const HDMA_V0_STOP_INT_MASK: u32 = 1 << 0;
pub const HDMA_V0_LINKLIST_EN: u32 = 1 << 0;
pub const HDMA_V0_CONSUMER_CYCLE_STAT: u32 = 1 << 1;
pub const HDMA_V0_CONSUMER_CYCLE_BIT: u32 = 1 << 0;
pub const HDMA_V0_DOORBELL_START: u32 = 1 << 0;
pub const HDMA_V0_CH_STATUS_MASK: u32 = (1 << 2) - 1;
pub const HDMA_V0_FUNC_NUM_PF_MASK: u32 = (1 << 8) - 1;

#[repr(C)]
pub struct DwHdmaV0Reg64Parts {
    pub lsb: u32,
    pub msb: u32,
}

#[repr(C)]
pub union DwHdmaV0Reg64 {
    pub reg: u64,
    pub parts: DwHdmaV0Reg64Parts,
    pub lsb: u32,
    pub msb: u32,
}

#[repr(C, packed)]
pub struct DwHdmaV0ChRegs {
    pub ch_en: u32,
    pub doorbell: u32,
    pub prefetch: u32,
    pub handshake: u32,
    pub llp: DwHdmaV0Reg64,
    pub cycle_sync: u32,
    pub transfer_size: u32,
    pub sar: DwHdmaV0Reg64,
    pub dar: DwHdmaV0Reg64,
    pub watermark_en: u32,
    pub control1: u32,
    pub func_num: u32,
    pub qos: u32,
    pub padding_1: [u32; 16],
    pub ch_stat: u32,
    pub int_stat: u32,
    pub int_setup: u32,
    pub int_clear: u32,
    pub msi_stop: DwHdmaV0Reg64,
    pub msi_watermark: DwHdmaV0Reg64,
    pub msi_abort: DwHdmaV0Reg64,
    pub msi_msgdata: u32,
    pub padding_2: [u32; 21],
}

#[repr(C, packed)]
pub struct DwHdmaV0Ch {
    pub wr: DwHdmaV0ChRegs,
    pub rd: DwHdmaV0ChRegs,
}

#[repr(C, packed)]
pub struct DwHdmaV0Regs {
    pub ch: [DwHdmaV0Ch; HDMA_V0_MAX_NR_CH],
}

#[repr(C, packed)]
pub struct DwHdmaV0Lli {
    pub control: u32,
    pub transfer_size: u32,
    pub sar: DwHdmaV0Reg64,
    pub dar: DwHdmaV0Reg64,
}

#[repr(C, packed)]
pub struct DwHdmaV0Llp {
    pub control: u32,
    pub reserved: u32,
    pub llp: DwHdmaV0Reg64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
