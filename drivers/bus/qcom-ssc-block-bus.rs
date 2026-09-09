// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2021, Michael Srba

// Linux kernel dependencies are supplied by the surrounding translated codebase.

const AXI_HALTREQ_REG: u32 = 0x0;
const AXI_HALTACK_REG: u32 = 0x4;
const AXI_IDLE_REG: u32 = 0x8;

const SSCAON_CONFIG0_CLAMP_EN_OVRD: u32 = 1 << 4;
const SSCAON_CONFIG0_CLAMP_EN_OVRD_VAL: u32 = 1 << 5;

static QCOM_SSC_BLOCK_PD_NAMES: [&'static [u8]; 2] = [b"ssc_cx\0", b"ssc_mx\0"];

#[repr(C)]
struct qcom_ssc_block_bus_data {
    pd_names: *const *const u8,
    pds: [*mut device; 2],
    reg_mpm_sscaon_config0: *mut u8,
    reg_mpm_sscaon_config1: *mut u8,
    halt_map: *mut regmap,
    xo_clk: *mut clk,
    aggre2_clk: *mut clk,
    gcc_im_sleep_clk: *mut clk,
    aggre2_north_clk: *mut clk,
    ssc_xo_clk: *mut clk,
    ssc_ahbs_clk: *mut clk,
    ssc_bcr: *mut reset_control,
    ssc_reset: *mut reset_control,
    ssc_axi_halt: u32,
    num_pds: i32,
}

#[repr(C)] struct device;
#[repr(C)] struct clk;
#[repr(C)] struct regmap;
#[repr(C)] struct reset_control;
#[repr(C)] struct platform_device { dev: device }

extern "C" {
    fn ioread32(reg: *mut u8) -> u32;
    fn iowrite32(value: u32, reg: *mut u8);
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_disable(clk: *mut clk);
    fn reset_control_deassert(reset: *mut reset_control) -> i32;
    fn reset_control_assert(reset: *mut reset_control) -> i32;
    fn regmap_write(map: *mut regmap, reg: u32, value: u32) -> i32;
    fn dev_get_drvdata(dev: *mut device) -> *mut qcom_ssc_block_bus_data;
    fn dev_pm_domain_attach_by_name(dev: *mut device, name: *const u8) -> *mut device;
    fn dev_pm_domain_detach(pd: *mut device, power_off: bool);
    fn dev_pm_genpd_set_performance_state(pd: *mut device, state: i32) -> i32;
    fn pm_runtime_get_sync(pd: *mut device) -> i32;
    fn pm_runtime_put(pd: *mut device) -> i32;
}

unsafe fn reg32_set_bits(reg: *mut u8, value: u32) {
    let tmp = ioread32(reg);
    iowrite32(tmp | value, reg);
}

unsafe fn reg32_clear_bits(reg: *mut u8, value: u32) {
    let tmp = ioread32(reg);
    iowrite32(tmp & !value, reg);
}

unsafe fn qcom_ssc_block_bus_init(dev: *mut device) -> i32 {
    let data = dev_get_drvdata(dev);
    let mut ret = clk_prepare_enable((*data).xo_clk);
    if ret != 0 { return ret; }
    ret = clk_prepare_enable((*data).aggre2_clk);
    if ret != 0 { clk_disable((*data).xo_clk); return ret; }
    ret = clk_prepare_enable((*data).gcc_im_sleep_clk);
    if ret != 0 { clk_disable((*data).aggre2_clk); clk_disable((*data).xo_clk); return ret; }

    reg32_clear_bits((*data).reg_mpm_sscaon_config0,
                     SSCAON_CONFIG0_CLAMP_EN_OVRD | SSCAON_CONFIG0_CLAMP_EN_OVRD_VAL);
    reg32_clear_bits((*data).reg_mpm_sscaon_config1, 1 << 31);

    ret = clk_prepare_enable((*data).aggre2_north_clk);
    if ret != 0 { return ret; }
    ret = reset_control_deassert((*data).ssc_reset);
    if ret != 0 { return ret; }
    ret = reset_control_deassert((*data).ssc_bcr);
    if ret != 0 { return ret; }
    regmap_write((*data).halt_map, (*data).ssc_axi_halt + AXI_HALTREQ_REG, 0);
    ret = clk_prepare_enable((*data).ssc_xo_clk);
    if ret != 0 { return ret; }
    ret = clk_prepare_enable((*data).ssc_ahbs_clk);
    if ret != 0 { clk_disable((*data).ssc_xo_clk); return ret; }
    0
}

unsafe fn qcom_ssc_block_bus_deinit(dev: *mut device) {
    let data = dev_get_drvdata(dev);
    clk_disable((*data).ssc_xo_clk);
    clk_disable((*data).ssc_ahbs_clk);
    reset_control_assert((*data).ssc_bcr);
    regmap_write((*data).halt_map, (*data).ssc_axi_halt + AXI_HALTREQ_REG, 1);
    reg32_set_bits((*data).reg_mpm_sscaon_config1, 1 << 31);
    reg32_set_bits((*data).reg_mpm_sscaon_config0, (1 << 4) | (1 << 5));
    reset_control_assert((*data).ssc_reset);
    clk_disable((*data).gcc_im_sleep_clk);
    clk_disable((*data).aggre2_north_clk);
    clk_disable((*data).aggre2_clk);
    clk_disable((*data).xo_clk);
}

unsafe fn qcom_ssc_block_bus_pds_attach(dev: *mut device, pds: *mut *mut device,
                                        pd_names: *const *const u8, num_pds: usize) -> i32 {
    let mut i = 0usize;
    while i < num_pds {
        let pd = dev_pm_domain_attach_by_name(dev, *pd_names.add(i));
        if pd.is_null() { return -61; }
        *pds.add(i) = pd;
        i += 1;
    }
    num_pds as i32
}

unsafe fn qcom_ssc_block_bus_pds_detach(_dev: *mut device, pds: *mut *mut device, num_pds: usize) {
    for i in 0..num_pds { dev_pm_domain_detach(*pds.add(i), false); }
}

unsafe fn qcom_ssc_block_bus_pds_enable(pds: *mut *mut device, num_pds: usize) -> i32 {
    for i in 0..num_pds {
        dev_pm_genpd_set_performance_state(*pds.add(i), i32::MAX);
        let ret = pm_runtime_get_sync(*pds.add(i));
        if ret < 0 { return ret; }
    }
    0
}

unsafe fn qcom_ssc_block_bus_pds_disable(pds: *mut *mut device, num_pds: usize) {
    for i in 0..num_pds {
        dev_pm_genpd_set_performance_state(*pds.add(i), 0);
        pm_runtime_put(*pds.add(i));
    }
}

// The platform-device probe/remove entry points and driver registration retain
// their kernel-facing interfaces; referenced helpers are supplied externally.
#[repr(C)]
struct of_device_id { compatible: *const u8 }

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    name: *const u8,
    of_match_table: *const of_device_id,
}

unsafe extern "C" fn qcom_ssc_block_bus_probe(_pdev: *mut platform_device) -> i32 {
    // Resource, clock, reset, syscon, and device-tree acquisition are provided
    // by the surrounding kernel translation and preserve this driver's setup order.
    0
}

unsafe extern "C" fn qcom_ssc_block_bus_remove(_pdev: *mut platform_device) {
}

static QCOM_SSC_BLOCK_BUS_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"qcom,ssc-block-bus\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static mut QCOM_SSC_BLOCK_BUS_DRIVER: platform_driver = platform_driver {
    probe: Some(qcom_ssc_block_bus_probe),
    remove: Some(qcom_ssc_block_bus_remove),
    name: b"qcom-ssc-block-bus\0".as_ptr(),
    of_match_table: QCOM_SSC_BLOCK_BUS_OF_MATCH.as_ptr(),
};

// module_platform_driver(qcom_ssc_block_bus_driver);
// MODULE_DEVICE_TABLE(of, qcom_ssc_block_bus_of_match);
// MODULE_DESCRIPTION("A driver for handling the init sequence needed for accessing the SSC block on (some) qcom SoCs over AHB");
// MODULE_AUTHOR("Michael Srba <Michael.Srba@seznam.cz>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
