// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022, 2023 Linaro Ltd.
 */
// Translated from clk-cbf-8996.c. Kernel declarations and constants are
// supplied by the surrounding Linux clock/interconnect implementation.

const DT_XO: usize = 0;
const DT_APCS_AUX: usize = 1;

const CBF_XO_INDEX: usize = 0;
const CBF_PLL_INDEX: usize = 1;
const CBF_DIV_INDEX: usize = 2;
const CBF_APCS_AUX_INDEX: usize = 3;

const DIV_THRESHOLD: u64 = 600_000_000;
const CBF_MUX_OFFSET: u32 = 0x18;
const CBF_MUX_PARENT_MASK: u32 = 0x3;
const CBF_MUX_AUTO_CLK_SEL_ALWAYS_ON_MASK: u32 = 0x30;
const CBF_MUX_AUTO_CLK_SEL_ALWAYS_ON_GPLL0_SEL: u32 = 0x30;
const CBF_MUX_AUTO_CLK_SEL_BIT: u32 = 1 << 6;
const CBF_PLL_OFFSET: u32 = 0xf000;

static mut cbfpll_config: alpha_pll_config = alpha_pll_config {
    l: 72,
    config_ctl_val: 0x200d4828,
    config_ctl_hi_val: 0x006,
    test_ctl_val: 0x1c000000,
    test_ctl_hi_val: 0x00004000,
    pre_div_mask: 1 << 12,
    post_div_mask: 0x3 << 8,
    post_div_val: 0x1 << 8,
    main_output_mask: 1,
    early_output_mask: 1 << 3,
};

static mut cbf_pll: clk_alpha_pll = clk_alpha_pll {
    offset: CBF_PLL_OFFSET,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_HUAYRA_APSS],
    flags: SUPPORTS_DYNAMIC_UPDATE | SUPPORTS_FSM_MODE,
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "cbf_pll", parent_data: &[clk_parent_data { index: DT_XO, hw: core::ptr::null() }],
        num_parents: 1, ops: &clk_alpha_pll_hwfsm_ops,
    } } },
};

static mut cbf_pll_postdiv: clk_fixed_factor = clk_fixed_factor {
    mult: 1,
    div: 2,
    hw: clk_hw { init: &clk_init_data {
        name: "cbf_pll_postdiv", parent_hws: &[&cbf_pll.clkr.hw as *const clk_hw],
        num_parents: 1, ops: &clk_fixed_factor_ops, flags: CLK_SET_RATE_PARENT,
    } },
};

static cbf_mux_parent_data: [clk_parent_data; 4] = [
    clk_parent_data { index: DT_XO, hw: core::ptr::null() },
    clk_parent_data { index: 0, hw: unsafe { &cbf_pll.clkr.hw } },
    clk_parent_data { index: 0, hw: unsafe { &cbf_pll_postdiv.hw } },
    clk_parent_data { index: DT_APCS_AUX, hw: core::ptr::null() },
];

#[repr(C)]
struct clk_cbf_8996_mux { reg: u32, nb: notifier_block, clkr: clk_regmap }

unsafe fn to_clk_cbf_8996_mux(clkr: *mut clk_regmap) -> *mut clk_cbf_8996_mux {
    (clkr as *mut u8).sub(core::mem::offset_of!(clk_cbf_8996_mux, clkr)) as *mut clk_cbf_8996_mux
}

unsafe extern "C" fn cbf_clk_notifier_cb(nb: *mut notifier_block, event: c_ulong, data: *mut c_void) -> c_int;

unsafe extern "C" fn clk_cbf_8996_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let clkr = to_clk_regmap(hw);
    let mux = to_clk_cbf_8996_mux(clkr);
    let mut val = 0u32;
    regmap_read((*clkr).regmap, (*mux).reg, &mut val);
    ((val & CBF_MUX_PARENT_MASK) as u8)
}

unsafe extern "C" fn clk_cbf_8996_mux_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    let clkr = to_clk_regmap(hw);
    let mux = to_clk_cbf_8996_mux(clkr);
    regmap_update_bits((*clkr).regmap, (*mux).reg, CBF_MUX_PARENT_MASK, (index as u32) & CBF_MUX_PARENT_MASK)
}

unsafe extern "C" fn clk_cbf_8996_mux_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    if (*req).rate < DIV_THRESHOLD / cbf_pll_postdiv.div { return -EINVAL; }
    let index = if (*req).rate < DIV_THRESHOLD { CBF_DIV_INDEX } else { CBF_PLL_INDEX };
    let parent = clk_hw_get_parent_by_index(hw, index as u8);
    if parent.is_null() { return -EINVAL; }
    (*req).best_parent_rate = clk_hw_round_rate(parent, (*req).rate);
    (*req).best_parent_hw = parent;
    0
}

static clk_cbf_8996_mux cbf_mux: clk_cbf_8996_mux = clk_cbf_8996_mux {
    reg: CBF_MUX_OFFSET,
    nb: notifier_block { notifier_call: Some(cbf_clk_notifier_cb) },
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "cbf_mux", parent_data: &cbf_mux_parent_data, num_parents: 4,
        ops: &clk_cbf_8996_mux_ops, flags: CLK_SET_RATE_PARENT | CLK_IS_CRITICAL,
    } } },
};

static clk_ops clk_cbf_8996_mux_ops: clk_ops = clk_ops {
    set_parent: Some(clk_cbf_8996_mux_set_parent), get_parent: Some(clk_cbf_8996_mux_get_parent),
    determine_rate: Some(clk_cbf_8996_mux_determine_rate),
};

unsafe extern "C" fn cbf_clk_notifier_cb(nb: *mut notifier_block, event: c_ulong, data: *mut c_void) -> c_int {
    let cnd = data as *mut clk_notifier_data;
    match event {
        PRE_RATE_CHANGE if (*cnd).old_rate > DIV_THRESHOLD && (*cnd).new_rate < DIV_THRESHOLD => {
            clk_cbf_8996_mux_set_parent(&mut cbf_mux.clkr.hw, CBF_DIV_INDEX as u8);
        },
        ABORT_RATE_CHANGE if (*cnd).new_rate < DIV_THRESHOLD && (*cnd).old_rate > DIV_THRESHOLD => {
            clk_cbf_8996_mux_set_parent(&mut cbf_mux.clkr.hw, CBF_PLL_INDEX as u8);
        },
        _ => {}
    }
    notifier_from_errno(0)
}

static mut cbf_msm8996_hw_clks: [*mut clk_hw; 1] = [unsafe { &mut cbf_pll_postdiv.hw }];
static mut cbf_msm8996_clks: [*mut clk_regmap; 2] = [unsafe { &mut cbf_pll.clkr }, unsafe { &mut cbf_mux.clkr }];

static cbf_msm8996_regmap_config: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x10000,
    val_format_endian: REGMAP_ENDIAN_LITTLE,
};

// CONFIG_INTERCONNECT selects the real registration/removal and sync-state
// implementations; the alternate definitions retain the disabled behavior.
unsafe extern "C" fn qcom_msm8996_cbf_icc_register(pdev: *mut platform_device, cbf_hw: *mut clk_hw) -> c_int;
unsafe extern "C" fn qcom_msm8996_cbf_icc_remove(pdev: *mut platform_device);

unsafe extern "C" fn qcom_msm8996_cbf_probe(pdev: *mut platform_device) -> c_int {
    let base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) { return PTR_ERR(base); }
    let dev = &mut (*pdev).dev;
    let regmap = devm_regmap_init_mmio(dev, base, &cbf_msm8996_regmap_config);
    if IS_ERR(regmap) { return PTR_ERR(regmap); }
    regmap_write(regmap, CBF_MUX_OFFSET, 0x3);
    udelay(5);
    regmap_update_bits(regmap, CBF_MUX_OFFSET, CBF_MUX_AUTO_CLK_SEL_ALWAYS_ON_MASK,
        CBF_MUX_AUTO_CLK_SEL_ALWAYS_ON_GPLL0_SEL);
    clk_alpha_pll_configure(&mut cbf_pll, regmap, &cbfpll_config);
    udelay(50);
    regmap_update_bits(regmap, CBF_MUX_OFFSET, CBF_MUX_AUTO_CLK_SEL_BIT, CBF_MUX_AUTO_CLK_SEL_BIT);
    udelay(5);
    regmap_update_bits(regmap, CBF_MUX_OFFSET, CBF_MUX_PARENT_MASK, 0x1);
    if of_device_is_compatible((*dev).of_node, "qcom,msm8996pro-cbf") {
        cbfpll_config.post_div_val = 0x3 << 8;
        cbf_pll_postdiv.div = 4;
    }
    for hw in cbf_msm8996_hw_clks.iter() {
        let ret = devm_clk_hw_register(dev, *hw);
        if ret != 0 { return ret; }
    }
    for clkr in cbf_msm8996_clks.iter() {
        let ret = devm_clk_register_regmap(dev, *clkr);
        if ret != 0 { return ret; }
    }
    let ret = devm_clk_notifier_register(dev, cbf_mux.clkr.hw.clk, &mut cbf_mux.nb);
    if ret != 0 { return ret; }
    let ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, &mut cbf_mux.clkr.hw);
    if ret != 0 { return ret; }
    qcom_msm8996_cbf_icc_register(pdev, &mut cbf_mux.clkr.hw)
}
unsafe extern "C" fn qcom_msm8996_cbf_remove(pdev: *mut platform_device) { qcom_msm8996_cbf_icc_remove(pdev); }

static qcom_msm8996_cbf_match_table: [of_device_id; 3] = [
    of_device_id { compatible: "qcom,msm8996-cbf" },
    of_device_id { compatible: "qcom,msm8996pro-cbf" },
    of_device_id { compatible: core::ptr::null() },
];

static mut qcom_msm8996_cbf_driver: platform_driver = platform_driver {
    probe: Some(qcom_msm8996_cbf_probe), remove: Some(qcom_msm8996_cbf_remove),
    driver: device_driver { name: "qcom-msm8996-cbf", of_match_table: &qcom_msm8996_cbf_match_table },
};

unsafe extern "C" fn qcom_msm8996_cbf_init() -> c_int { platform_driver_register(&mut qcom_msm8996_cbf_driver) }
unsafe extern "C" fn qcom_msm8996_cbf_exit() { platform_driver_unregister(&mut qcom_msm8996_cbf_driver); }

// MODULE_DEVICE_TABLE(of, qcom_msm8996_cbf_match_table);
// postcore_initcall(qcom_msm8996_cbf_init);
// module_exit(qcom_msm8996_cbf_exit);
// MODULE_DESCRIPTION("QCOM MSM8996 CPU Bus Fabric Clock Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
