// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Copyright (c) 2023 Collabora, Ltd.
 *               AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Translated from clk-mt7622-infracfg.c.  C headers provide the external
// clock, platform, device-tree, and reset definitions used below.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct mtk_gate_regs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct mtk_composite {
    pub id: c_int,
    pub name: *const c_char,
    pub parents: *const *const c_char,
    pub parent_count: usize,
    pub reg: u32,
    pub shift: u8,
    pub width: u8,
}

#[repr(C)]
pub struct mtk_gate {
    pub id: c_int,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub regs: *const mtk_gate_regs,
    pub shift: u8,
    pub ops: *const c_void,
}

#[repr(C)]
pub struct mtk_clk_rst_desc {
    pub version: c_int,
    pub rst_bank_ofs: *const u16,
    pub rst_bank_nr: usize,
}

#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct clk_hw_onecell_data;
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}
#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

extern "C" {
    static mtk_clk_gate_ops_setclr: c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_int) -> *mut c_void;
    fn mtk_alloc_clk_data(nr_clk: c_int) -> *mut clk_hw_onecell_data;
    fn mtk_register_reset_controller_with_dev(dev: *mut device, desc: *const mtk_clk_rst_desc) -> c_int;
    fn mtk_clk_register_gates(dev: *mut device, node: *mut device_node, gates: *const mtk_gate, nr: usize, data: *mut clk_hw_onecell_data) -> c_int;
    fn mtk_clk_register_cpumuxes(dev: *mut device, node: *mut device_node, muxes: *const mtk_composite, nr: usize, data: *mut clk_hw_onecell_data) -> c_int;
    fn of_clk_add_hw_provider(node: *mut device_node, get: *const c_void, data: *mut clk_hw_onecell_data) -> c_int;
    fn of_clk_del_provider(node: *mut device_node);
    fn mtk_clk_unregister_cpumuxes(muxes: *const mtk_composite, nr: usize, data: *mut clk_hw_onecell_data);
    fn mtk_clk_unregister_gates(gates: *const mtk_gate, nr: usize, data: *mut clk_hw_onecell_data);
    fn mtk_free_clk_data(data: *mut clk_hw_onecell_data);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut clk_hw_onecell_data;
    static of_clk_hw_onecell_get: c_void;
}

// Clock IDs and MTK_RST_SIMPLE are supplied by the device-tree and clock headers.
extern "C" {
    static CLK_INFRA_MUX1_SEL: c_int;
    static CLK_INFRA_DBGCLK_PD: c_int;
    static CLK_INFRA_TRNG: c_int;
    static CLK_INFRA_AUDIO_PD: c_int;
    static CLK_INFRA_IRRX_PD: c_int;
    static CLK_INFRA_APXGPT_PD: c_int;
    static CLK_INFRA_PMIC_PD: c_int;
    static CLK_INFRA_NR_CLK: c_int;
    static MTK_RST_SIMPLE: c_int;
}

static infra_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x40, clr_ofs: 0x44, sta_ofs: 0x48 };

static infra_mux1_parents: [*const c_char; 4] = [
    b"clkxtal\0".as_ptr() as *const c_char,
    b"armpll\0".as_ptr() as *const c_char,
    b"main_core_en\0".as_ptr() as *const c_char,
    b"armpll\0".as_ptr() as *const c_char,
];

static cpu_muxes: [mtk_composite; 1] = [mtk_composite {
    id: 0, name: b"infra_mux1_sel\0".as_ptr() as *const c_char,
    parents: infra_mux1_parents.as_ptr(), parent_count: 4, reg: 0x000, shift: 2, width: 2,
}];

static infra_clks: [mtk_gate; 6] = [
    gate(0, b"infra_dbgclk_pd\0", b"axi_sel\0", 0),
    gate(1, b"trng_ck\0", b"axi_sel\0", 2),
    gate(2, b"infra_audio_pd\0", b"aud_intbus_sel\0", 5),
    gate(3, b"infra_irrx_pd\0", b"irrx_sel\0", 16),
    gate(4, b"infra_apxgpt_pd\0", b"f10m_ref_sel\0", 18),
    gate(5, b"infra_pmic_pd\0", b"pmicspi_sel\0", 22),
];

const fn gate(id: c_int, name: &'static [u8], parent: &'static [u8], shift: u8) -> mtk_gate {
    mtk_gate { id, name: name.as_ptr() as *const c_char, parent_name: parent.as_ptr() as *const c_char, regs: &infra_cg_regs, shift, ops: core::ptr::null() }
}

static infrasys_rst_ofs: [u16; 1] = [0x30];
static clk_rst_desc: mtk_clk_rst_desc = mtk_clk_rst_desc { version: 0, rst_bank_ofs: infrasys_rst_ofs.as_ptr(), rst_bank_nr: 1 };
static of_match_clk_mt7622_infracfg: [of_device_id; 2] = [
    of_device_id { compatible: b"mediatek,mt7622-infracfg\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];

unsafe extern "C" fn clk_mt7622_infracfg_probe(pdev: *mut platform_device) -> c_int {
    let node = core::ptr::null_mut::<device_node>();
    let base = devm_platform_ioremap_resource(pdev, 0);
    if base as isize == -1 { return -1; }
    let clk_data = mtk_alloc_clk_data(0);
    if clk_data.is_null() { return -12; }
    let mut ret = mtk_register_reset_controller_with_dev(&mut (*pdev).dev, &clk_rst_desc);
    if ret != 0 { mtk_free_clk_data(clk_data); return ret; }
    ret = mtk_clk_register_gates(&mut (*pdev).dev, node, infra_clks.as_ptr(), 6, clk_data);
    if ret != 0 { mtk_free_clk_data(clk_data); return ret; }
    ret = mtk_clk_register_cpumuxes(&mut (*pdev).dev, node, cpu_muxes.as_ptr(), 1, clk_data);
    if ret != 0 { mtk_clk_unregister_gates(infra_clks.as_ptr(), 6, clk_data); mtk_free_clk_data(clk_data); return ret; }
    ret = of_clk_add_hw_provider(node, &of_clk_hw_onecell_get, clk_data);
    if ret != 0 { mtk_clk_unregister_cpumuxes(cpu_muxes.as_ptr(), 1, clk_data); mtk_clk_unregister_gates(infra_clks.as_ptr(), 6, clk_data); mtk_free_clk_data(clk_data); return ret; }
    0
}

unsafe extern "C" fn clk_mt7622_infracfg_remove(pdev: *mut platform_device) {
    let node = core::ptr::null_mut::<device_node>();
    let clk_data = platform_get_drvdata(pdev);
    of_clk_del_provider(node);
    mtk_clk_unregister_cpumuxes(cpu_muxes.as_ptr(), 1, clk_data);
    mtk_clk_unregister_gates(infra_clks.as_ptr(), 6, clk_data);
    mtk_free_clk_data(clk_data);
}

static mut clk_mt7622_infracfg_drv: platform_driver = platform_driver {
    driver: platform_driver_driver { name: b"clk-mt7622-infracfg\0".as_ptr() as *const c_char, of_match_table: of_match_clk_mt7622_infracfg.as_ptr() },
    probe: Some(clk_mt7622_infracfg_probe), remove: Some(clk_mt7622_infracfg_remove),
};

// module_platform_driver(clk_mt7622_infracfg_drv);
// MODULE_DESCRIPTION("MediaTek MT7622 infracfg clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
