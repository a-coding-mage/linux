// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022, 2023 Linaro Limited
 */

// C dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn of_device_get_match_data(dev: *const device) -> *const qcom_cc_desc;
    fn qcom_cc_probe_by_index(
        pdev: *mut platform_device,
        index: i32,
        desc: *const qcom_cc_desc,
    ) -> i32;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    _private: [u8; 0],
}

#[repr(C)]
pub struct qcom_reset_map {
    pub reg: u32,
    pub bit: u32,
    pub udelay: u32,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: u32,
    pub reg_stride: u32,
    pub val_bits: u32,
    pub name: *const u8,
    pub max_register: u32,
}

#[repr(C)]
pub struct qcom_cc_desc {
    pub config: *mut regmap_config,
    pub resets: *const qcom_reset_map,
    pub num_resets: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
    pub data: *const qcom_cc_desc,
}

// Values are supplied by <dt-bindings/clock/qcom,sm6115-lpasscc.h>.
extern "C" {
    static LPASS_AUDIO_SWR_RX_CGCR: usize;
    static LPASS_SWR_TX_CONFIG_CGCR: usize;
}

static mut lpass_audiocc_sm6115_resets: [qcom_reset_map; 1] = [qcom_reset_map {
    reg: 0x98,
    bit: 1,
    udelay: 500,
}];

static mut lpass_audiocc_sm6115_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    name: b"lpass-audio-csr\0".as_ptr(),
    max_register: 0x1000,
};

static lpass_audiocc_sm6115_reset_desc: qcom_cc_desc = qcom_cc_desc {
    config: unsafe { &raw mut lpass_audiocc_sm6115_regmap_config },
    resets: unsafe { lpass_audiocc_sm6115_resets.as_ptr() },
    num_resets: 1,
};

static mut lpasscc_sm6115_resets: [qcom_reset_map; 1] = [qcom_reset_map {
    reg: 0x100,
    bit: 1,
    udelay: 500,
}];

static mut lpasscc_sm6115_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    name: b"lpass-tcsr\0".as_ptr(),
    max_register: 0x1000,
};

static lpasscc_sm6115_reset_desc: qcom_cc_desc = qcom_cc_desc {
    config: unsafe { &raw mut lpasscc_sm6115_regmap_config },
    resets: unsafe { lpasscc_sm6115_resets.as_ptr() },
    num_resets: 1,
};

static lpasscc_sm6115_match_table: [of_device_id; 3] = [
    of_device_id {
        compatible: b"qcom,sm6115-lpassaudiocc\0".as_ptr(),
        data: &lpass_audiocc_sm6115_reset_desc,
    },
    of_device_id {
        compatible: b"qcom,sm6115-lpasscc\0".as_ptr(),
        data: &lpasscc_sm6115_reset_desc,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

unsafe extern "C" fn lpasscc_sm6115_probe(pdev: *mut platform_device) -> i32 {
    let desc = of_device_get_match_data(&raw const (*pdev).dev);
    qcom_cc_probe_by_index(pdev, 0, desc)
}

#[repr(C)]
struct driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    name: *const u8,
    of_match_table: *const of_device_id,
}

static mut lpasscc_sm6115_driver: driver = driver {
    probe: Some(lpasscc_sm6115_probe),
    name: b"lpasscc-sm6115\0".as_ptr(),
    of_match_table: lpasscc_sm6115_match_table.as_ptr(),
};

// module_platform_driver(lpasscc_sm6115_driver);
// MODULE_DESCRIPTION("QTI LPASSCC SM6115 Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
