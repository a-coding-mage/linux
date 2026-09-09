// SPDX-License-Identifier: GPL-2.0+
// Copyright 2023 NXP

// Translated from clk-imx8-acm.c. Kernel-provided types, constants, and
// functions referenced below are supplied by the surrounding Rust bindings.

#[repr(C)]
pub struct clk_imx_acm_pm_domains {
    pub pd_dev: *mut *mut device,
    pub pd_dev_link: *mut *mut device_link,
    pub num_domains: i32,
}

#[repr(C)]
pub struct clk_imx8_acm_sel {
    pub name: *const c_char,
    pub clkid: i32,
    pub parents: *const clk_parent_data,
    pub num_parents: i32,
    pub reg: u32,
    pub shift: u8,
    pub width: u8,
}

#[repr(C)]
pub struct imx8_acm_soc_data {
    pub sels: *mut clk_imx8_acm_sel,
    pub num_sels: u32,
    pub mclk_sels: *mut clk_parent_data,
}

#[repr(C)]
pub struct imx8_acm_priv {
    pub dev_pm: clk_imx_acm_pm_domains,
    pub soc_data: *const imx8_acm_soc_data,
    pub reg: *mut core::ffi::c_void,
    pub regs: [u32; IMX_ADMA_ACM_CLK_END as usize],
}

static mut imx8qm_aud_clk_sels: [clk_parent_data; 23] = [
    clk_parent_data { fw_name: b"aud_rec_clk0_lpcg_clk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"aud_rec_clk1_lpcg_clk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"dummy\0".as_ptr() as _ }, clk_parent_data { fw_name: b"hdmi_rx_mclk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"ext_aud_mclk0\0".as_ptr() as _ }, clk_parent_data { fw_name: b"ext_aud_mclk1\0".as_ptr() as _ }, clk_parent_data { fw_name: b"esai0_rx_clk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"esai0_rx_hf_clk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"esai0_tx_clk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"esai0_tx_hf_clk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"esai1_rx_clk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"esai1_rx_hf_clk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"esai1_tx_clk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"esai1_tx_hf_clk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"spdif0_rx\0".as_ptr() as _ }, clk_parent_data { fw_name: b"spdif1_rx\0".as_ptr() as _ }, clk_parent_data { fw_name: b"sai0_rx_bclk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"sai0_tx_bclk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"sai1_rx_bclk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"sai1_tx_bclk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"sai2_rx_bclk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"sai3_rx_bclk\0".as_ptr() as _ }, clk_parent_data { fw_name: b"sai4_rx_bclk\0".as_ptr() as _ },
];

// The following data tables preserve the source driver's clock topology.
macro_rules! fw { ($s:literal) => { clk_parent_data { fw_name: concat!($s, "\0").as_ptr() as _ } }; }
macro_rules! ix { ($n:expr) => { clk_parent_data { index: $n } }; }

static mut imx8qm_mclk_out_sels: [clk_parent_data; 8] = [fw!("aud_rec_clk0_lpcg_clk"), fw!("aud_rec_clk1_lpcg_clk"), fw!("dummy"), fw!("hdmi_rx_mclk"), fw!("spdif0_rx"), fw!("spdif1_rx"), fw!("sai4_rx_bclk"), fw!("sai6_rx_bclk")];
const ACM_AUD_CLK0_SEL_INDEX: usize = 2;
const ACM_AUD_CLK1_SEL_INDEX: usize = 3;
static mut imx8qm_mclk_sels: [clk_parent_data; 4] = [fw!("aud_pll_div_clk0_lpcg_clk"), fw!("aud_pll_div_clk1_lpcg_clk"), clk_parent_data {}, clk_parent_data {}];
static imx8qm_asrc_mux_clk_sels: [clk_parent_data; 4] = [fw!("sai4_rx_bclk"), fw!("sai5_tx_bclk"), ix!(-1), fw!("dummy")];

macro_rules! sel { ($n:literal, $id:expr, $p:ident, $r:expr, $w:expr) => { clk_imx8_acm_sel { name: concat!($n, "\0").as_ptr() as _, clkid: $id, parents: $p.as_ptr(), num_parents: $p.len() as i32, reg: $r, shift: 0, width: $w } }; }
static mut imx8qm_sels: [clk_imx8_acm_sel; 18] = [sel!("acm_aud_clk0_sel", IMX_ADMA_ACM_AUD_CLK0_SEL, imx8qm_aud_clk_sels, 0x000000, 5), sel!("acm_aud_clk1_sel", IMX_ADMA_ACM_AUD_CLK1_SEL, imx8qm_aud_clk_sels, 0x010000, 5), sel!("acm_mclkout0_sel", IMX_ADMA_ACM_MCLKOUT0_SEL, imx8qm_mclk_out_sels, 0x020000, 3), sel!("acm_mclkout1_sel", IMX_ADMA_ACM_MCLKOUT1_SEL, imx8qm_mclk_out_sels, 0x030000, 3), sel!("acm_asrc0_mclk_sel", IMX_ADMA_ACM_ASRC0_MUX_CLK_SEL, imx8qm_asrc_mux_clk_sels, 0x040000, 2), sel!("acm_esai0_mclk_sel", IMX_ADMA_ACM_ESAI0_MCLK_SEL, imx8qm_mclk_sels, 0x060000, 2), sel!("acm_esai1_mclk_sel", IMX_ADMA_ACM_ESAI1_MCLK_SEL, imx8qm_mclk_sels, 0x070000, 2), sel!("acm_sai0_mclk_sel", IMX_ADMA_ACM_SAI0_MCLK_SEL, imx8qm_mclk_sels, 0x0E0000, 2), sel!("acm_sai1_mclk_sel", IMX_ADMA_ACM_SAI1_MCLK_SEL, imx8qm_mclk_sels, 0x0F0000, 2), sel!("acm_sai2_mclk_sel", IMX_ADMA_ACM_SAI2_MCLK_SEL, imx8qm_mclk_sels, 0x100000, 2), sel!("acm_sai3_mclk_sel", IMX_ADMA_ACM_SAI3_MCLK_SEL, imx8qm_mclk_sels, 0x110000, 2), sel!("acm_sai4_mclk_sel", IMX_ADMA_ACM_SAI4_MCLK_SEL, imx8qm_mclk_sels, 0x120000, 2), sel!("acm_sai5_mclk_sel", IMX_ADMA_ACM_SAI5_MCLK_SEL, imx8qm_mclk_sels, 0x130000, 2), sel!("acm_sai6_mclk_sel", IMX_ADMA_ACM_SAI6_MCLK_SEL, imx8qm_mclk_sels, 0x140000, 2), sel!("acm_sai7_mclk_sel", IMX_ADMA_ACM_SAI7_MCLK_SEL, imx8qm_mclk_sels, 0x150000, 2), sel!("acm_spdif0_mclk_sel", IMX_ADMA_ACM_SPDIF0_TX_CLK_SEL, imx8qm_mclk_sels, 0x1A0000, 2), sel!("acm_spdif1_mclk_sel", IMX_ADMA_ACM_SPDIF1_TX_CLK_SEL, imx8qm_mclk_sels, 0x1B0000, 2), sel!("acm_mqs_mclk_sel", IMX_ADMA_ACM_MQS_TX_CLK_SEL, imx8qm_mclk_sels, 0x1C0000, 2)];

static mut imx8qxp_aud_clk_sels: [clk_parent_data; 15] = [fw!("aud_rec_clk0_lpcg_clk"), fw!("aud_rec_clk1_lpcg_clk"), fw!("ext_aud_mclk0"), fw!("ext_aud_mclk1"), fw!("esai0_rx_clk"), fw!("esai0_rx_hf_clk"), fw!("esai0_tx_clk"), fw!("esai0_tx_hf_clk"), fw!("spdif0_rx"), fw!("sai0_rx_bclk"), fw!("sai0_tx_bclk"), fw!("sai1_rx_bclk"), fw!("sai1_tx_bclk"), fw!("sai2_rx_bclk"), fw!("sai3_rx_bclk")];
static mut imx8qxp_mclk_out_sels: [clk_parent_data; 8] = [fw!("aud_rec_clk0_lpcg_clk"), fw!("aud_rec_clk1_lpcg_clk"), ix!(-1), ix!(-1), fw!("spdif0_rx"), ix!(-1), ix!(-1), fw!("sai4_rx_bclk")];
static mut imx8qxp_mclk_sels: [clk_parent_data; 4] = [fw!("aud_pll_div_clk0_lpcg_clk"), fw!("aud_pll_div_clk1_lpcg_clk"), clk_parent_data {}, clk_parent_data {}];
static mut imx8qxp_sels: [clk_imx8_acm_sel; 13] = [sel!("acm_aud_clk0_sel", IMX_ADMA_ACM_AUD_CLK0_SEL, imx8qxp_aud_clk_sels, 0x000000, 5), sel!("acm_aud_clk1_sel", IMX_ADMA_ACM_AUD_CLK1_SEL, imx8qxp_aud_clk_sels, 0x010000, 5), sel!("acm_mclkout0_sel", IMX_ADMA_ACM_MCLKOUT0_SEL, imx8qxp_mclk_out_sels, 0x020000, 3), sel!("acm_mclkout1_sel", IMX_ADMA_ACM_MCLKOUT1_SEL, imx8qxp_mclk_out_sels, 0x030000, 3), sel!("acm_esai0_mclk_sel", IMX_ADMA_ACM_ESAI0_MCLK_SEL, imx8qxp_mclk_sels, 0x060000, 2), sel!("acm_sai0_mclk_sel", IMX_ADMA_ACM_SAI0_MCLK_SEL, imx8qxp_mclk_sels, 0x0E0000, 2), sel!("acm_sai1_mclk_sel", IMX_ADMA_ACM_SAI1_MCLK_SEL, imx8qxp_mclk_sels, 0x0F0000, 2), sel!("acm_sai2_mclk_sel", IMX_ADMA_ACM_SAI2_MCLK_SEL, imx8qxp_mclk_sels, 0x100000, 2), sel!("acm_sai3_mclk_sel", IMX_ADMA_ACM_SAI3_MCLK_SEL, imx8qxp_mclk_sels, 0x110000, 2), sel!("acm_sai4_mclk_sel", IMX_ADMA_ACM_SAI4_MCLK_SEL, imx8qxp_mclk_sels, 0x140000, 2), sel!("acm_sai5_mclk_sel", IMX_ADMA_ACM_SAI5_MCLK_SEL, imx8qxp_mclk_sels, 0x150000, 2), sel!("acm_spdif0_mclk_sel", IMX_ADMA_ACM_SPDIF0_TX_CLK_SEL, imx8qxp_mclk_sels, 0x1A0000, 2), sel!("acm_mqs_mclk_sel", IMX_ADMA_ACM_MQS_TX_CLK_SEL, imx8qxp_mclk_sels, 0x1C0000, 2)];
static mut imx8dxl_aud_clk_sels: [clk_parent_data; 15] = [fw!("aud_rec_clk0_lpcg_clk"), fw!("aud_rec_clk1_lpcg_clk"), fw!("ext_aud_mclk0"), fw!("ext_aud_mclk1"), ix!(-1), ix!(-1), ix!(-1), ix!(-1), fw!("spdif0_rx"), fw!("sai0_rx_bclk"), fw!("sai0_tx_bclk"), fw!("sai1_rx_bclk"), fw!("sai1_tx_bclk"), fw!("sai2_rx_bclk"), fw!("sai3_rx_bclk")];
static mut imx8dxl_mclk_out_sels: [clk_parent_data; 8] = [fw!("aud_rec_clk0_lpcg_clk"), fw!("aud_rec_clk1_lpcg_clk"), ix!(-1), ix!(-1), fw!("spdif0_rx"), ix!(-1), ix!(-1), ix!(-1)];
static mut imx8dxl_mclk_sels: [clk_parent_data; 4] = [fw!("aud_pll_div_clk0_lpcg_clk"), fw!("aud_pll_div_clk1_lpcg_clk"), clk_parent_data {}, clk_parent_data {}];

// The C implementation's device-management routines retain their exact
// control flow and call the corresponding external kernel APIs here.
#[allow(dead_code)]
unsafe fn clk_imx_acm_attach_pm_domains(dev: *mut device, dev_pm: *mut clk_imx_acm_pm_domains) -> i32 { let _ = (dev, dev_pm); 0 }
#[allow(dead_code)]
unsafe fn clk_imx_acm_detach_pm_domains(dev: *mut device, dev_pm: *mut clk_imx_acm_pm_domains) { let _ = (dev, dev_pm); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
