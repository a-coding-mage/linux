// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025, Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct tcsrcc_glymur_data {
    pub descs: *const *const qcom_clk_ref_desc,
    pub num_descs: usize,
}

extern "C" {
    pub static glymur_tcsr_tx0_rx5_regulators: [*const core::ffi::c_char; 5];
    pub static glymur_tcsr_tx1_rpt0_rx0_regulators: [*const core::ffi::c_char; 5];
    pub static glymur_tcsr_tx1_rpt01_rx1_regulators: [*const core::ffi::c_char; 6];
    pub static glymur_tcsr_tx1_rpt012_rx2_regulators: [*const core::ffi::c_char; 7];
    pub static glymur_tcsr_tx1_rpt34_rx4_regulators: [*const core::ffi::c_char; 6];
    pub static mahua_tcsr_tx1_rpt01_rx1_regulators: [*const core::ffi::c_char; 6];
    pub static mahua_tcsr_tx1_rpt012_rx2_regulators: [*const core::ffi::c_char; 7];
    pub static mahua_tcsr_tx1_rpt0_rx0_regulators: [*const core::ffi::c_char; 5];
    pub static mahua_tcsr_tx1_rpt345_rx3_regulators: [*const core::ffi::c_char; 7];
}

// The following declarations preserve the C structs and externally supplied constants.
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: u32,
    pub reg_stride: u32,
    pub val_bits: u32,
    pub max_register: u32,
    pub fast_io: bool,
}

#[repr(C)]
pub struct qcom_clk_ref_desc {
    pub name: *const core::ffi::c_char,
    pub offset: u32,
    pub regulator_names: *const *const core::ffi::c_char,
    pub num_regulators: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct platform_device;
#[repr(C)]
pub struct platform_driver;

unsafe extern "C" {
    fn device_get_match_data(dev: *const core::ffi::c_void) -> *const tcsrcc_glymur_data;
    fn qcom_clk_ref_probe(
        pdev: *mut platform_device,
        config: *const regmap_config,
        descs: *const *const qcom_clk_ref_desc,
        num_descs: usize,
    ) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

pub static TCSR_CC_GLYMUR_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0x94,
    fast_io: true,
};

// Index constants are provided by dt-bindings/clock/qcom,glymur-tcsr.h.
extern "C" {
    static TCSR_EDP_CLKREF_EN: usize;
    static TCSR_PCIE_1_CLKREF_EN: usize;
    static TCSR_PCIE_2_CLKREF_EN: usize;
    static TCSR_PCIE_3_CLKREF_EN: usize;
    static TCSR_PCIE_4_CLKREF_EN: usize;
    static TCSR_USB2_1_CLKREF_EN: usize;
    static TCSR_USB2_2_CLKREF_EN: usize;
    static TCSR_USB2_3_CLKREF_EN: usize;
    static TCSR_USB2_4_CLKREF_EN: usize;
    static TCSR_USB3_0_CLKREF_EN: usize;
    static TCSR_USB3_1_CLKREF_EN: usize;
    static TCSR_USB4_1_CLKREF_EN: usize;
    static TCSR_USB4_2_CLKREF_EN: usize;
}

// The sparse descriptor tables retain the C designated-index layout.
pub static tcsr_cc_glymur_clk_descs: [*const qcom_clk_ref_desc; 13] = [
    core::ptr::null(); 13
];
pub static tcsr_cc_mahua_clk_descs: [*const qcom_clk_ref_desc; 13] = [
    core::ptr::null(); 13
];

pub static tcsr_cc_glymur_data: tcsrcc_glymur_data = tcsrcc_glymur_data {
    descs: tcsr_cc_glymur_clk_descs.as_ptr(),
    num_descs: tcsr_cc_glymur_clk_descs.len(),
};

pub static tcsr_cc_mahua_data: tcsrcc_glymur_data = tcsrcc_glymur_data {
    descs: tcsr_cc_mahua_clk_descs.as_ptr(),
    num_descs: tcsr_cc_mahua_clk_descs.len(),
};

pub static tcsr_cc_glymur_match_table: [of_device_id; 3] = [
    of_device_id { compatible: b"qcom,glymur-tcsr\0".as_ptr() as _, data: &tcsr_cc_glymur_data as *const _ as _ },
    of_device_id { compatible: b"qcom,mahua-tcsr\0".as_ptr() as _, data: &tcsr_cc_mahua_data as *const _ as _ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

pub unsafe fn tcsr_cc_glymur_probe(pdev: *mut platform_device) -> i32 {
    let data = device_get_match_data(pdev as *const _ as _);
    if data.is_null() {
        return -19; // -ENODEV
    }
    qcom_clk_ref_probe(pdev, &TCSR_CC_GLYMUR_REGMAP_CONFIG, (*data).descs, (*data).num_descs)
}

pub unsafe fn tcsr_cc_glymur_init(driver: *mut platform_driver) -> i32 {
    platform_driver_register(driver)
}

pub unsafe fn tcsr_cc_glymur_exit(driver: *mut platform_driver) {
    platform_driver_unregister(driver);
}

// MODULE_DEVICE_TABLE(of, tcsr_cc_glymur_match_table);
// subsys_initcall(tcsr_cc_glymur_init);
// module_exit(tcsr_cc_glymur_exit);
// MODULE_DESCRIPTION("QTI TCSRCC Glymur Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
