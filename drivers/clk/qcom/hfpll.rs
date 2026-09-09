// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

// External Linux kernel and Qualcomm clock declarations are supplied by other files.

#[repr(C)]
pub struct hfpll_data {
    pub mode_reg: u32,
    pub l_reg: u32,
    pub m_reg: u32,
    pub n_reg: u32,
    pub user_reg: u32,
    pub config_reg: u32,
    pub config_val: u32,
    pub status_reg: u32,
    pub lock_bit: u32,
    pub l_val: u32,
    pub user_val: u32,
    pub user_vco_mask: u32,
    pub low_vco_max_rate: u64,
    pub min_rate: u64,
    pub max_rate: u64,
}

extern "C" {
    static clk_ops_hfpll: clk_ops;
}

#[repr(C)] pub struct clk_ops { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk_hfpll { pub d: *const hfpll_data, pub clkr: clk_regmap, pub lock: spinlock_t }
#[repr(C)] pub struct clk_regmap { pub hw: clk_hw, pub init: *mut clk_init_data }
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk_init_data {
    pub num_parents: u32,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub name: *const core::ffi::c_char,
    pub parent_data: *mut clk_parent_data,
}
#[repr(C)] pub struct clk_parent_data { pub index: u32 }
#[repr(C)] pub struct of_device_id { pub compatible: *const core::ffi::c_char, pub data: *const core::ffi::c_void }
#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub max_register: u32 }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct platform_driver { pub probe: Option<unsafe fn(*mut platform_device) -> i32>, pub name: *const core::ffi::c_char, pub of_match_table: *const of_device_id }

const CLK_IGNORE_UNUSED: u32 = 1 << 6;

static qcs404: hfpll_data = hfpll_data {
    mode_reg: 0x00, l_reg: 0x04, m_reg: 0x08, n_reg: 0x0c, user_reg: 0x10,
    config_reg: 0x14, config_val: 0x430405d, status_reg: 0x1c, lock_bit: 16,
    l_val: 0, user_val: 0x8, user_vco_mask: 0x100000,
    low_vco_max_rate: 1248000000, min_rate: 537600000, max_rate: 2900000000,
};
static msm8976_a53: hfpll_data = hfpll_data {
    mode_reg: 0x00, l_reg: 0x04, m_reg: 0x08, n_reg: 0x0c, user_reg: 0x10,
    config_reg: 0x14, config_val: 0x341600, status_reg: 0x1c, lock_bit: 16,
    l_val: 0x35, user_val: 0x109, user_vco_mask: 0, low_vco_max_rate: 0,
    min_rate: 902400000, max_rate: 1478400000,
};
static msm8976_a72: hfpll_data = hfpll_data {
    mode_reg: 0x00, l_reg: 0x04, m_reg: 0x08, n_reg: 0x0c, user_reg: 0x10,
    config_reg: 0x14, config_val: 0x4e0405d, status_reg: 0x1c, lock_bit: 16,
    l_val: 0x3e, user_val: 0x100109, user_vco_mask: 0, low_vco_max_rate: 0,
    min_rate: 940800000, max_rate: 2016000000,
};
static msm8976_cci: hfpll_data = hfpll_data {
    mode_reg: 0x00, l_reg: 0x04, m_reg: 0x08, n_reg: 0x0c, user_reg: 0x10,
    config_reg: 0x14, config_val: 0x141400, status_reg: 0x1c, lock_bit: 16,
    l_val: 0x20, user_val: 0x100109, user_vco_mask: 0, low_vco_max_rate: 0,
    min_rate: 556800000, max_rate: 902400000,
};

static qcom_hfpll_match_table: [of_device_id; 6] = [
    of_device_id { compatible: b"qcom,msm8976-hfpll-a53\0".as_ptr() as _, data: &msm8976_a53 as *const _ as _ },
    of_device_id { compatible: b"qcom,msm8976-hfpll-a72\0".as_ptr() as _, data: &msm8976_a72 as *const _ as _ },
    of_device_id { compatible: b"qcom,msm8976-hfpll-cci\0".as_ptr() as _, data: &msm8976_cci as *const _ as _ },
    of_device_id { compatible: b"qcom,qcs404-hfpll\0".as_ptr() as _, data: &qcs404 as *const _ as _ },
    // Deprecated in bindings
    of_device_id { compatible: b"qcom,hfpll\0".as_ptr() as _, data: &qcs404 as *const _ as _ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

static hfpll_regmap_config: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x30,
};

extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: u32, res: *mut *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn devm_regmap_init_mmio(dev: *mut device, base: *mut core::ffi::c_void, config: *const regmap_config) -> *mut regmap;
    fn of_property_read_string_index(node: *mut core::ffi::c_void, propname: *const core::ffi::c_char, index: u32, output: *mut *const core::ffi::c_char) -> i32;
    fn of_device_get_match_data(dev: *mut device) -> *const core::ffi::c_void;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn devm_clk_register_regmap(dev: *mut device, clk: *mut clk_regmap) -> i32;
    fn devm_of_clk_add_hw_provider(dev: *mut device, get: *const core::ffi::c_void, data: *mut clk_hw) -> i32;
}

unsafe fn qcom_hfpll_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let mut init = clk_init_data { num_parents: 1, ops: &clk_ops_hfpll, flags: CLK_IGNORE_UNUSED, name: core::ptr::null(), parent_data: core::ptr::null_mut() };
    let mut pdata = clk_parent_data { index: 0 };
    let h = devm_kzalloc(dev, core::mem::size_of::<clk_hfpll>(), 0) as *mut clk_hfpll;
    if h.is_null() { return -12; }
    let base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    let regmap = devm_regmap_init_mmio(dev, base, &hfpll_regmap_config);
    if regmap.is_null() { return -1; }
    if of_property_read_string_index(core::ptr::null_mut(), b"clock-output-names\0".as_ptr() as _, 0, &mut init.name) != 0 { return -19; }
    init.parent_data = &mut pdata;
    (*h).d = of_device_get_match_data(dev) as *const hfpll_data;
    (*h).clkr.init = &mut init;
    spin_lock_init(&mut (*h).lock);
    let ret = devm_clk_register_regmap(dev, &mut (*h).clkr);
    if ret != 0 { return ret; }
    devm_of_clk_add_hw_provider(dev, core::ptr::null(), &mut (*h).clkr.hw)
}

static qcom_hfpll_driver: platform_driver = platform_driver {
    probe: Some(qcom_hfpll_probe), name: b"qcom-hfpll\0".as_ptr() as _, of_match_table: qcom_hfpll_match_table.as_ptr(),
};
// module_platform_driver(qcom_hfpll_driver)
// MODULE_DEVICE_TABLE(of, qcom_hfpll_match_table)
// MODULE_DESCRIPTION("QCOM HFPLL Clock Driver")
// MODULE_LICENSE("GPL v2")
// MODULE_ALIAS("platform:qcom-hfpll")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
