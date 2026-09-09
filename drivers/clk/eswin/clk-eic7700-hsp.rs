// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2026, Beijing ESWIN Computing Technology Co., Ltd..
 * All rights reserved.
 *
 * ESWIN EIC7700 HSP Clock Driver
 *
 * Authors: Xuyang Dong <dongxuyang@eswincomputing.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const EIC7700_HSP_SATA_REG: u32 = 0x300;
const EIC7700_HSP_MSHC0_REG: u32 = 0x510;
const EIC7700_HSP_MSHC1_REG: u32 = 0x610;
const EIC7700_HSP_MSHC2_REG: u32 = 0x710;
const EIC7700_HSP_USB0_REG: u32 = 0x800;
const EIC7700_HSP_USB0_REF_REG: u32 = 0x83c;
const EIC7700_HSP_USB1_REG: u32 = 0x900;
const EIC7700_HSP_USB1_REF_REG: u32 = 0x93c;
const USB_REF_XTAL24M: u32 = 0x2a;
const EIC7700_HSP_NR_CLKS: usize = EIC7700_HSP_CLK_GATE_SATA as usize + 1;

#[repr(C)]
struct eic7700_hsp_clk_gate {
    hw: clk_hw,
    id: u32,
    regmap: *mut regmap,
    reg: u32,
    ref_reg: u32,
    name: *const c_char,
    parent_data: *const clk_parent_data,
    flags: c_ulong,
    offset: u32,
    ref_offset: u32,
    bit_idx: u8,
}

static eic7700_hsp_regmap_config: regmap_config = regmap_config {
    reg_bits: 32, val_bits: 32, max_register: 0x1ffc, reg_stride: 4,
    fast_io: true, use_raw_spinlock: true,
};

unsafe fn to_gate_clk(hw: *mut clk_hw) -> *mut eic7700_hsp_clk_gate {
    container_of!(hw, eic7700_hsp_clk_gate, hw)
}

unsafe fn hsp_clk_gate_endisable(hw: *mut clk_hw, enable: bool) {
    let gate = &mut *to_gate_clk(hw);
    if enable {
        // Hardware requires the USB reference clock to be 24MHz.
        regmap_update_bits(gate.regmap, gate.ref_reg, 0x3f, USB_REF_XTAL24M);
    }
    regmap_assign_bits(gate.regmap, gate.reg, BIT(gate.bit_idx), enable);
}

unsafe fn hsp_clk_gate_enable(hw: *mut clk_hw) -> c_int { hsp_clk_gate_endisable(hw, true); 0 }
unsafe fn hsp_clk_gate_disable(hw: *mut clk_hw) { hsp_clk_gate_endisable(hw, false); }
unsafe fn hsp_clk_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    let gate = &mut *to_gate_clk(hw);
    let mut val = 0u32;
    let ret = regmap_read(gate.regmap, gate.reg, &mut val);
    if ret != 0 { return ret; }
    !!(val & BIT(gate.bit_idx)) as c_int
}

static hsp_clk_gate_ops: clk_ops = clk_ops {
    enable: Some(hsp_clk_gate_enable), disable: Some(hsp_clk_gate_disable),
    is_enabled: Some(hsp_clk_gate_is_enabled),
};

unsafe fn hsp_clk_register_gate(dev: *mut device, id: u32, name: *const c_char,
    parent_data: *const clk_parent_data, flags: c_ulong, regmap: *mut regmap,
    reg: u32, ref_reg: u32, bit_idx: u8) -> *mut clk_hw {
    let gate = devm_kzalloc(dev, core::mem::size_of::<eic7700_hsp_clk_gate>(), GFP_KERNEL);
    if gate.is_null() { return ERR_PTR(-ENOMEM); }
    let gate = gate as *mut eic7700_hsp_clk_gate;
    (*gate).id = id; (*gate).regmap = regmap; (*gate).reg = reg;
    (*gate).ref_reg = ref_reg; (*gate).bit_idx = bit_idx;
    (*gate).hw.init = &mut clk_init_data { name, ops: &hsp_clk_gate_ops,
        flags, parent_data, num_parents: 1 };
    let ret = devm_clk_hw_register(dev, &mut (*gate).hw);
    if ret != 0 { return ERR_PTR(ret); }
    &mut (*gate).hw
}

static hsp_cfg: [clk_parent_data; 1] = [clk_parent_data { index: 0 }];
static hsp_mmc: [clk_parent_data; 1] = [clk_parent_data { index: 1 }];
static hsp_usb_sata: [clk_parent_data; 1] = [clk_parent_data { index: 2 }];

// ESWIN_FACTOR/ESWIN_GATE/ESWIN_MUX and related clock descriptor initializers
// are preserved as the corresponding declarations supplied by common.h.
static mut eic7700_hsp_factor_clks: [eswin_fixed_factor_clock; 3] = [
    ESWIN_FACTOR!(EIC7700_HSP_CLK_FAC_CFG_DIV2, "factor_hsp_cfg_div2", hsp_cfg, 1, 2, 0),
    ESWIN_FACTOR!(EIC7700_HSP_CLK_FAC_CFG_DIV4, "factor_hsp_cfg_div4", hsp_cfg, 1, 4, 0),
    ESWIN_FACTOR!(EIC7700_HSP_CLK_FAC_MMC_DIV10, "factor_hsp_mmc_div10", hsp_mmc, 1, 10, 0),
];

static mut eic7700_hsp_gate_clks: [eswin_gate_clock; 4] = [
    ESWIN_GATE!(EIC7700_HSP_CLK_GATE_SATA, "gate_clk_hsp_sata", hsp_usb_sata, CLK_SET_RATE_PARENT | CLK_IGNORE_UNUSED, EIC7700_HSP_SATA_REG, 28, 0),
    ESWIN_GATE!(EIC7700_HSP_CLK_GATE_MSHC0_TMR, "gate_clk_hsp_mshc0_tmr", hsp_mmc, CLK_SET_RATE_PARENT | CLK_IGNORE_UNUSED, EIC7700_HSP_MSHC0_REG, 8, 0),
    ESWIN_GATE!(EIC7700_HSP_CLK_GATE_MSHC1_TMR, "gate_clk_hsp_mshc1_tmr", hsp_mmc, CLK_SET_RATE_PARENT | CLK_IGNORE_UNUSED, EIC7700_HSP_MSHC1_REG, 8, 0),
    ESWIN_GATE!(EIC7700_HSP_CLK_GATE_MSHC2_TMR, "gate_clk_hsp_mshc2_tmr", hsp_mmc, CLK_SET_RATE_PARENT | CLK_IGNORE_UNUSED, EIC7700_HSP_MSHC2_REG, 8, 0),
];

static mut eic7700_hsp_spec_gate_clks: [eic7700_hsp_clk_gate; 2] = [
    EIC7700_HSP_GATE!(EIC7700_HSP_CLK_GATE_USB0, "gate_clk_hsp_usb0", hsp_usb_sata, CLK_SET_RATE_PARENT | CLK_IGNORE_UNUSED, EIC7700_HSP_USB0_REG, 28, EIC7700_HSP_USB0_REF_REG),
    EIC7700_HSP_GATE!(EIC7700_HSP_CLK_GATE_USB1, "gate_clk_hsp_usb1", hsp_usb_sata, CLK_SET_RATE_PARENT | CLK_IGNORE_UNUSED, EIC7700_HSP_USB1_REG, 28, EIC7700_HSP_USB1_REF_REG),
];

static mux_mmc_3mux1_tbl: [u32; 3] = [0x0, 0x1, 0x3];
static mut eic7700_hsp_mux_clks: [eswin_mux_clock; 6] = [
    ESWIN_MUX_TBL!(EIC7700_HSP_CLK_MUX_EMMC_3MUX1, "mux_hsp_emmc_3mux1", mux_mmc_3mux1_p, 3, CLK_SET_RATE_PARENT, EIC7700_HSP_MSHC0_REG, 16, 2, 0, mux_mmc_3mux1_tbl),
    ESWIN_MUX_TBL!(EIC7700_HSP_CLK_MUX_SD0_3MUX1, "mux_hsp_sd0_3mux1", mux_mmc_3mux1_p, 3, CLK_SET_RATE_PARENT, EIC7700_HSP_MSHC1_REG, 16, 2, 0, mux_mmc_3mux1_tbl),
    ESWIN_MUX_TBL!(EIC7700_HSP_CLK_MUX_SD1_3MUX1, "mux_hsp_sd1_3mux1", mux_mmc_3mux1_p, 3, CLK_SET_RATE_PARENT, EIC7700_HSP_MSHC2_REG, 16, 2, 0, mux_mmc_3mux1_tbl),
    ESWIN_MUX!(EIC7700_HSP_CLK_MUX_EMMC_CQE_2MUX1, "mux_hsp_emmc_cqe_2mux1", mux_mmc_2mux1_p, 2, CLK_SET_RATE_PARENT, EIC7700_HSP_MSHC0_REG, 0, 1, 0),
    ESWIN_MUX!(EIC7700_HSP_CLK_MUX_SD0_CQE_2MUX1, "mux_hsp_sd0_cqe_2mux1", mux_mmc_2mux1_p, 2, CLK_SET_RATE_PARENT, EIC7700_HSP_MSHC1_REG, 0, 1, 0),
    ESWIN_MUX!(EIC7700_HSP_CLK_MUX_SD1_CQE_2MUX1, "mux_hsp_sd1_cqe_2mux1", mux_mmc_2mux1_p, 2, CLK_SET_RATE_PARENT, EIC7700_HSP_MSHC2_REG, 0, 1, 0),
];

static mut eic7700_hsp_clks: [eswin_clk_info; 3] = [
    ESWIN_GATE_TYPE!(EIC7700_HSP_CLK_GATE_EMMC, "gate_clk_hsp_emmc", EIC7700_HSP_CLK_MUX_EMMC_3MUX1, CLK_SET_RATE_PARENT | CLK_IGNORE_UNUSED, EIC7700_HSP_MSHC0_REG, 24, 0),
    ESWIN_GATE_TYPE!(EIC7700_HSP_CLK_GATE_SD0, "gate_clk_hsp_sd0", EIC7700_HSP_CLK_MUX_SD0_3MUX1, CLK_SET_RATE_PARENT | CLK_IGNORE_UNUSED, EIC7700_HSP_MSHC1_REG, 24, 0),
    ESWIN_GATE_TYPE!(EIC7700_HSP_CLK_GATE_SD1, "gate_clk_hsp_sd1", EIC7700_HSP_CLK_MUX_SD1_3MUX1, CLK_SET_RATE_PARENT | CLK_IGNORE_UNUSED, EIC7700_HSP_MSHC2_REG, 24, 0),
];

unsafe fn eic7700_hsp_clk_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev;
    let data = eswin_clk_init(pdev, EIC7700_HSP_NR_CLKS);
    if IS_ERR(data) { return dev_err_probe(dev, PTR_ERR(data), c"failed to get clk data!\n"); }
    let regmap = devm_regmap_init_mmio(dev, (*data).base, &eic7700_hsp_regmap_config);
    if IS_ERR(regmap) { return dev_err_probe(dev, PTR_ERR(regmap), c"failed to get regmap!\n"); }
    let mut ret = eswin_clk_register_fixed_factor(dev, eic7700_hsp_factor_clks.as_mut_ptr(), 3, data);
    if ret != 0 { return dev_err_probe(dev, ret, c"failed to register fixed factor clock\n"); }
    ret = eswin_clk_register_gate(dev, eic7700_hsp_gate_clks.as_mut_ptr(), 4, data);
    if ret != 0 { return dev_err_probe(dev, ret, c"failed to register gate clock\n"); }
    ret = eswin_clk_register_mux(dev, eic7700_hsp_mux_clks.as_mut_ptr(), 6, data);
    if ret != 0 { return dev_err_probe(dev, ret, c"failed to register mux clock\n"); }
    ret = eswin_clk_register_clks(dev, eic7700_hsp_clks.as_mut_ptr(), 3, data);
    if ret != 0 { return dev_err_probe(dev, ret, c"failed to register clock\n"); }
    for gate in eic7700_hsp_spec_gate_clks.iter_mut() {
        let hw = hsp_clk_register_gate(dev, gate.id, gate.name, gate.parent_data, gate.flags, regmap, gate.offset, gate.ref_offset, gate.bit_idx);
        if IS_ERR(hw) { return dev_err_probe(dev, PTR_ERR(hw), c"failed to register gate clock\n"); }
        (*data).clk_data.hws[gate.id as usize] = hw;
    }
    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get, &(*data).clk_data);
    if ret != 0 { return dev_err_probe(dev, ret, c"add clk provider failed\n"); }
    if devm_auxiliary_device_create(dev, c"hsp-reset", core::ptr::null_mut()).is_null() { return dev_err_probe(dev, -ENODEV, c"register hsp-reset device failed\n"); }
    0
}

// Device-table, platform-driver, module registration, and metadata declarations
// correspond directly to the C module declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
