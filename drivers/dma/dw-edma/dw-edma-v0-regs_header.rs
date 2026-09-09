/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018-2019 Synopsys, Inc. and/or its affiliates.
 * Synopsys DesignWare eDMA v0 core
 *
 * Author: Gustavo Pimentel <gustavo.pimentel@synopsys.com>
 */

// Dependency supplied by the surrounding kernel translation.

pub const EDMA_V0_MAX_NR_CH: usize = 8;
pub const EDMA_V0_VIEWPORT_MASK: u32 = 0x7;
pub const EDMA_V0_DONE_INT_MASK: u32 = 0xff;
pub const EDMA_V0_ABORT_INT_MASK: u32 = 0x00ff_0000;
pub const EDMA_V0_WRITE_CH_COUNT_MASK: u32 = 0xf;
pub const EDMA_V0_READ_CH_COUNT_MASK: u32 = 0x000f_0000;
pub const EDMA_V0_CH_STATUS_MASK: u32 = 0x60;
pub const EDMA_V0_DOORBELL_CH_MASK: u32 = 0x7;
pub const EDMA_V0_LINKED_LIST_ERR_MASK: u32 = 0xff;
pub const EDMA_V0_CH_ODD_MSI_DATA_MASK: u32 = 0xffff_0000;
pub const EDMA_V0_CH_EVEN_MSI_DATA_MASK: u32 = 0xffff;

#[repr(C)]
pub struct DwEdmaV0U32Pair { pub lsb: u32, pub msb: u32 }
#[repr(C)]
pub union DwEdmaV0U64 { pub reg: u64, pub pair: DwEdmaV0U32Pair }

#[repr(C, packed)]
pub struct dw_edma_v0_ch_regs {
    pub ch_control1: u32, pub ch_control2: u32, pub transfer_size: u32,
    pub sar: DwEdmaV0U64, pub dar: DwEdmaV0U64, pub llp: DwEdmaV0U64,
}

#[repr(C, packed)]
pub struct dw_edma_v0_ch {
    pub wr: dw_edma_v0_ch_regs,
    pub padding_1: [u32; 55],
    pub rd: dw_edma_v0_ch_regs,
    pub padding_2: [u32; 55],
}

#[repr(C, packed)]
pub struct dw_edma_v0_unroll {
    pub padding_1: u32, pub wr_engine_chgroup: u32, pub rd_engine_chgroup: u32,
    pub wr_engine_hshake_cnt: DwEdmaV0U64, pub padding_2: [u32; 2],
    pub rd_engine_hshake_cnt: DwEdmaV0U64, pub padding_3: [u32; 2],
    pub wr_ch0_pwr_en: u32, pub wr_ch1_pwr_en: u32, pub wr_ch2_pwr_en: u32,
    pub wr_ch3_pwr_en: u32, pub wr_ch4_pwr_en: u32, pub wr_ch5_pwr_en: u32,
    pub wr_ch6_pwr_en: u32, pub wr_ch7_pwr_en: u32, pub padding_4: [u32; 8],
    pub rd_ch0_pwr_en: u32, pub rd_ch1_pwr_en: u32, pub rd_ch2_pwr_en: u32,
    pub rd_ch3_pwr_en: u32, pub rd_ch4_pwr_en: u32, pub rd_ch5_pwr_en: u32,
    pub rd_ch6_pwr_en: u32, pub rd_ch7_pwr_en: u32, pub padding_5: [u32; 30],
    pub ch: [dw_edma_v0_ch; EDMA_V0_MAX_NR_CH],
}

#[repr(C, packed)]
pub struct dw_edma_v0_legacy { pub viewport_sel: u32, pub ch: dw_edma_v0_ch_regs }

#[repr(C)]
pub union dw_edma_v0_type { pub legacy: dw_edma_v0_legacy, pub unroll: dw_edma_v0_unroll }

#[repr(C, packed)]
pub struct dw_edma_v0_regs {
    pub ctrl_data_arb_prior: u32, pub padding_1: u32, pub ctrl: u32,
    pub wr_engine_en: u32, pub wr_doorbell: u32, pub padding_2: u32,
    pub wr_ch_arb_weight: DwEdmaV0U64, pub padding_3: [u32; 3],
    pub rd_engine_en: u32, pub rd_doorbell: u32, pub padding_4: u32,
    pub rd_ch_arb_weight: DwEdmaV0U64, pub padding_5: [u32; 3],
    pub wr_int_status: u32, pub padding_6: u32, pub wr_int_mask: u32,
    pub wr_int_clear: u32, pub wr_err_status: u32, pub wr_done_imwr: DwEdmaV0U64,
    pub wr_abort_imwr: DwEdmaV0U64, pub wr_ch01_imwr_data: u32,
    pub wr_ch23_imwr_data: u32, pub wr_ch45_imwr_data: u32, pub wr_ch67_imwr_data: u32,
    pub padding_7: [u32; 4], pub wr_linked_list_err_en: u32, pub padding_8: [u32; 3],
    pub rd_int_status: u32, pub padding_9: u32, pub rd_int_mask: u32,
    pub rd_int_clear: u32, pub padding_10: u32, pub rd_err_status: DwEdmaV0U64,
    pub padding_11: [u32; 2], pub rd_linked_list_err_en: u32, pub padding_12: u32,
    pub rd_done_imwr: DwEdmaV0U64, pub rd_abort_imwr: DwEdmaV0U64,
    pub rd_ch01_imwr_data: u32, pub rd_ch23_imwr_data: u32,
    pub rd_ch45_imwr_data: u32, pub rd_ch67_imwr_data: u32,
    pub padding_13: [u32; 4], pub r#type: dw_edma_v0_type,
}

#[repr(C, packed)]
pub struct dw_edma_v0_lli {
    pub control: u32, pub transfer_size: u32, pub sar: DwEdmaV0U64, pub dar: DwEdmaV0U64,
}

#[repr(C, packed)]
pub struct dw_edma_v0_llp { pub control: u32, pub reserved: u32, pub llp: DwEdmaV0U64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
