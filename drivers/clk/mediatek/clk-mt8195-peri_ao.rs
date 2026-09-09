// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the surrounding clock-driver environment:
// clk-gate.h, clk-mtk.h, dt-bindings/clock/mt8195-clk.h,
// linux/clk-provider.h, and linux/platform_device.h.

static PERI_AO_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x10,
    clr_ofs: 0x14,
    sta_ofs: 0x18,
};

// Direct translation of GATE_PERI_AO(_id, _name, _parent, _shift).
macro_rules! gate_peri_ao {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &PERI_AO_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static PERI_AO_CLKS: [mtk_gate; 16] = [
    gate_peri_ao!(CLK_PERI_AO_ETHERNET, "peri_ao_ethernet", "top_axi", 0),
    gate_peri_ao!(CLK_PERI_AO_ETHERNET_BUS, "peri_ao_ethernet_bus", "top_axi", 1),
    gate_peri_ao!(CLK_PERI_AO_FLASHIF_BUS, "peri_ao_flashif_bus", "top_axi", 3),
    gate_peri_ao!(CLK_PERI_AO_FLASHIF_FLASH, "peri_ao_flashif_flash", "top_spinor", 5),
    gate_peri_ao!(CLK_PERI_AO_SSUSB_1P_BUS, "peri_ao_ssusb_1p_bus", "top_usb_top_1p", 7),
    gate_peri_ao!(CLK_PERI_AO_SSUSB_1P_XHCI, "peri_ao_ssusb_1p_xhci", "top_ssusb_xhci_1p", 8),
    gate_peri_ao!(CLK_PERI_AO_SSUSB_2P_BUS, "peri_ao_ssusb_2p_bus", "top_usb_top_2p", 9),
    gate_peri_ao!(CLK_PERI_AO_SSUSB_2P_XHCI, "peri_ao_ssusb_2p_xhci", "top_ssusb_xhci_2p", 10),
    gate_peri_ao!(CLK_PERI_AO_SSUSB_3P_BUS, "peri_ao_ssusb_3p_bus", "top_usb_top_3p", 11),
    gate_peri_ao!(CLK_PERI_AO_SSUSB_3P_XHCI, "peri_ao_ssusb_3p_xhci", "top_ssusb_xhci_3p", 12),
    gate_peri_ao!(CLK_PERI_AO_SPINFI, "peri_ao_spinfi", "top_spinfi_bclk", 15),
    gate_peri_ao!(CLK_PERI_AO_ETHERNET_MAC, "peri_ao_ethernet_mac", "top_snps_eth_250m", 16),
    gate_peri_ao!(CLK_PERI_AO_NFI_H, "peri_ao_nfi_h", "top_axi", 19),
    gate_peri_ao!(CLK_PERI_AO_FNFI1X, "peri_ao_fnfi1x", "top_nfi1x", 20),
    gate_peri_ao!(CLK_PERI_AO_PCIE_P0_MEM, "peri_ao_pcie_p0_mem", "mem_466m", 24),
    gate_peri_ao!(CLK_PERI_AO_PCIE_P1_MEM, "peri_ao_pcie_p1_mem", "mem_466m", 25),
];

static PERI_AO_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: PERI_AO_CLKS.as_ptr(),
    num_clks: PERI_AO_CLKS.len(),
};

static OF_MATCH_CLK_MT8195_PERI_AO: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8195-pericfg_ao",
        data: &PERI_AO_DESC,
    },
    of_device_id::SENTINEL,
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT8195_PERI_AO);

static mut CLK_MT8195_PERI_AO_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: "clk-mt8195-peri_ao",
        of_match_table: OF_MATCH_CLK_MT8195_PERI_AO.as_ptr(),
    },
};

module_platform_driver!(CLK_MT8195_PERI_AO_DRV);

MODULE_DESCRIPTION!("MediaTek MT8195 pericfg clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
