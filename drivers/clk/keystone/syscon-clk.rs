// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 Texas Instruments Incorporated - https://www.ti.com/
 */

// External Linux kernel types, functions, constants, and macros are supplied by
// the surrounding kernel translation.

#[repr(C)]
struct TiSysconGateClkPriv {
    hw: clk_hw,
    regmap: *mut regmap,
    reg: u32,
    idx: u32,
}

#[repr(C)]
struct TiSysconGateClkData {
    name: *mut c_char,
    offset: u32,
    bit_idx: u32,
}

unsafe fn to_ti_syscon_gate_clk_priv(hw: *mut clk_hw) -> *mut TiSysconGateClkPriv {
    container_of!(hw, TiSysconGateClkPriv, hw)
}

unsafe fn ti_syscon_gate_clk_enable(hw: *mut clk_hw) -> c_int {
    let priv_ = &mut *to_ti_syscon_gate_clk_priv(hw);
    regmap_write_bits(priv_.regmap, priv_.reg, priv_.idx, priv_.idx)
}

unsafe fn ti_syscon_gate_clk_disable(hw: *mut clk_hw) {
    let priv_ = &mut *to_ti_syscon_gate_clk_priv(hw);
    regmap_write_bits(priv_.regmap, priv_.reg, priv_.idx, 0);
}

unsafe fn ti_syscon_gate_clk_is_enabled(hw: *mut clk_hw) -> c_int {
    let mut val: c_uint = 0;
    let priv_ = &mut *to_ti_syscon_gate_clk_priv(hw);
    regmap_read(priv_.regmap, priv_.reg, &mut val);
    if (val & priv_.idx) != 0 { 1 } else { 0 }
}

static TI_SYSCON_GATE_CLK_OPS: clk_ops = clk_ops {
    enable: Some(ti_syscon_gate_clk_enable),
    disable: Some(ti_syscon_gate_clk_disable),
    is_enabled: Some(ti_syscon_gate_clk_is_enabled),
};

unsafe fn ti_syscon_gate_clk_register(
    dev: *mut device,
    regmap: *mut regmap,
    parent_name: *const c_char,
    data: *const TiSysconGateClkData,
) -> *mut clk_hw {
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<TiSysconGateClkPriv>(), GFP_KERNEL)
        as *mut TiSysconGateClkPriv;
    if priv_.is_null() { return ERR_PTR(-ENOMEM); }

    let mut init: clk_init_data = core::mem::zeroed();
    let mut name: *mut c_char = core::ptr::null_mut();
    init.ops = &TI_SYSCON_GATE_CLK_OPS;
    if !parent_name.is_null() {
        name = kasprintf(GFP_KERNEL, c"%s:%s", (*data).name, parent_name);
        init.name = name;
        init.parent_names = &parent_name;
        init.num_parents = 1;
        init.flags = CLK_SET_RATE_PARENT;
    } else {
        init.name = (*data).name;
        init.parent_names = core::ptr::null();
        init.num_parents = 0;
        init.flags = 0;
    }
    (*priv_).regmap = regmap;
    (*priv_).reg = (*data).offset;
    (*priv_).idx = 1u32.wrapping_shl((*data).bit_idx);
    (*priv_).hw.init = &init;
    let ret = devm_clk_hw_register(dev, &mut (*priv_).hw);
    if !name.is_null() { kfree(init.name); }
    if ret != 0 { return ERR_PTR(ret); }
    &mut (*priv_).hw
}

static TI_SYSCON_REGMAP_CFG: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
};

unsafe fn ti_syscon_gate_clk_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev;
    let data = device_get_match_data(dev) as *const TiSysconGateClkData;
    if data.is_null() { return -EINVAL; }
    let base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) { return PTR_ERR(base); }
    let regmap = devm_regmap_init_mmio(dev, base, &TI_SYSCON_REGMAP_CFG);
    if IS_ERR(regmap) { return dev_err_probe(dev, PTR_ERR(regmap), c"failed to get regmap\n"); }
    let mut num_clks = 0;
    while !(*data.add(num_clks)).name.is_null() { num_clks += 1; }
    let num_parents = of_clk_get_parent_count((*dev).of_node);
    if of_device_is_compatible((*dev).of_node, c"ti,am62-audio-refclk") && num_parents == 0 {
        return dev_err_probe(dev, -EINVAL, c"must specify a parent clock\n");
    }
    let hw_data = devm_kzalloc(dev, struct_size!(clk_hw_onecell_data, hws, num_clks), GFP_KERNEL)
        as *mut clk_hw_onecell_data;
    if hw_data.is_null() { return -ENOMEM; }
    (*hw_data).num = num_clks;
    let parent_name = of_clk_get_parent_name((*dev).of_node, 0);
    for i in 0..num_clks {
        (*hw_data).hws[i] = ti_syscon_gate_clk_register(dev, regmap, parent_name, data.add(i));
        if IS_ERR((*hw_data).hws[i]) { dev_warn(dev, c"failed to register %s\n", (*data.add(i)).name); }
    }
    if num_clks == 1 { return devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, (*hw_data).hws[0]); }
    devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get, hw_data)
}

macro_rules! ti_syscon_clk_gate { ($name:expr, $offset:expr, $bit_idx:expr) => { TiSysconGateClkData { name: $name, offset: $offset, bit_idx: $bit_idx } }; }

static AM654_CLK_DATA: [TiSysconGateClkData; 7] = [
    ti_syscon_clk_gate!(c"ehrpwm_tbclk0", 0x0, 0), ti_syscon_clk_gate!(c"ehrpwm_tbclk1", 0x4, 0),
    ti_syscon_clk_gate!(c"ehrpwm_tbclk2", 0x8, 0), ti_syscon_clk_gate!(c"ehrpwm_tbclk3", 0xc, 0),
    ti_syscon_clk_gate!(c"ehrpwm_tbclk4", 0x10, 0), ti_syscon_clk_gate!(c"ehrpwm_tbclk5", 0x14, 0),
    TiSysconGateClkData { name: core::ptr::null_mut(), offset: 0, bit_idx: 0 },
];
static AM64_CLK_DATA: [TiSysconGateClkData; 10] = [
    ti_syscon_clk_gate!(c"epwm_tbclk0",0,0), ti_syscon_clk_gate!(c"epwm_tbclk1",0,1), ti_syscon_clk_gate!(c"epwm_tbclk2",0,2), ti_syscon_clk_gate!(c"epwm_tbclk3",0,3), ti_syscon_clk_gate!(c"epwm_tbclk4",0,4), ti_syscon_clk_gate!(c"epwm_tbclk5",0,5), ti_syscon_clk_gate!(c"epwm_tbclk6",0,6), ti_syscon_clk_gate!(c"epwm_tbclk7",0,7), ti_syscon_clk_gate!(c"epwm_tbclk8",0,8), TiSysconGateClkData { name: core::ptr::null_mut(), offset:0, bit_idx:0 } ];
static AM62_CLK_DATA: [TiSysconGateClkData; 4] = [ti_syscon_clk_gate!(c"epwm_tbclk0",0,0),ti_syscon_clk_gate!(c"epwm_tbclk1",0,1),ti_syscon_clk_gate!(c"epwm_tbclk2",0,2),TiSysconGateClkData{name:core::ptr::null_mut(),offset:0,bit_idx:0}];
static AM62_AUDIO_CLK_DATA: [TiSysconGateClkData; 2] = [ti_syscon_clk_gate!(c"audio_refclk",0,15),TiSysconGateClkData{name:core::ptr::null_mut(),offset:0,bit_idx:0}];

static TI_SYSCON_GATE_CLK_IDS: [of_device_id; 5] = [
    of_device_id { compatible: c"ti,am654-ehrpwm-tbclk", data: &AM654_CLK_DATA as *const _ as *const c_void },
    of_device_id { compatible: c"ti,am64-epwm-tbclk", data: &AM64_CLK_DATA as *const _ as *const c_void },
    of_device_id { compatible: c"ti,am62-epwm-tbclk", data: &AM62_CLK_DATA as *const _ as *const c_void },
    of_device_id { compatible: c"ti,am62-audio-refclk", data: &AM62_AUDIO_CLK_DATA as *const _ as *const c_void },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

static mut TI_SYSCON_GATE_CLK_DRIVER: platform_driver = platform_driver {
    probe: Some(ti_syscon_gate_clk_probe),
    driver: device_driver { name: c"ti-syscon-gate-clk", of_match_table: TI_SYSCON_GATE_CLK_IDS.as_ptr() },
};

// MODULE_DEVICE_TABLE(of, ti_syscon_gate_clk_ids);
// module_platform_driver(ti_syscon_gate_clk_driver);
// MODULE_AUTHOR("Vignesh Raghavendra <vigneshr@ti.com");
// MODULE_DESCRIPTION("Syscon backed gate-clock driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
