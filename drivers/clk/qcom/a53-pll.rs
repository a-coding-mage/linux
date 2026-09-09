// SPDX-License-Identifier: GPL-2.0
/*
 * Qualcomm A53 PLL driver
 *
 * Copyright (c) 2017, Linaro Limited
 * Author: Georgi Djakov <georgi.djakov@linaro.org>
 */

// Dependencies supplied by the surrounding kernel/Rust bindings.

static A53PLL_FREQ: [pll_freq_tbl; 8] = [
    pll_freq_tbl { freq: 998400000, l: 52, m: 0x0, n: 0x1, vco: 0 },
    pll_freq_tbl { freq: 1094400000, l: 57, m: 0x0, n: 0x1, vco: 0 },
    pll_freq_tbl { freq: 1152000000, l: 60, m: 0x0, n: 0x1, vco: 0 },
    pll_freq_tbl { freq: 1209600000, l: 63, m: 0x0, n: 0x1, vco: 0 },
    pll_freq_tbl { freq: 1248000000, l: 65, m: 0x0, n: 0x1, vco: 0 },
    pll_freq_tbl { freq: 1363200000, l: 71, m: 0x0, n: 0x1, vco: 0 },
    pll_freq_tbl { freq: 1401600000, l: 73, m: 0x0, n: 0x1, vco: 0 },
    pll_freq_tbl { ..Default::default() },
];

static A53PLL_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0x40,
};

unsafe fn qcom_a53pll_get_freq_tbl(dev: *mut device) -> *mut pll_freq_tbl {
    let mut freq_tbl: *mut pll_freq_tbl;
    let xo_freq: c_ulong;
    let mut freq: c_ulong;
    let xo_clk: *mut clk;
    let count: c_int;
    let ret: c_int;
    let mut i: c_int;

    xo_clk = devm_clk_get(dev, b"xo\0".as_ptr() as *const c_char);
    if IS_ERR(xo_clk) {
        return core::ptr::null_mut();
    }

    xo_freq = clk_get_rate(xo_clk);

    ret = devm_pm_opp_of_add_table(dev);
    if ret != 0 {
        return core::ptr::null_mut();
    }

    count = dev_pm_opp_get_opp_count(dev);
    if count <= 0 {
        return core::ptr::null_mut();
    }

    freq_tbl = devm_kcalloc(dev, (count + 1) as usize, core::mem::size_of::<pll_freq_tbl>(), GFP_KERNEL) as *mut pll_freq_tbl;
    if freq_tbl.is_null() {
        return core::ptr::null_mut();
    }

    i = 0;
    freq = 0;
    while i < count {
        let opp: *mut dev_pm_opp;

        opp = dev_pm_opp_find_freq_ceil(dev, &mut freq);
        if IS_ERR(opp) {
            return core::ptr::null_mut();
        }

        /* Skip the freq that is not divisible */
        if freq % xo_freq != 0 {
            i += 1;
            freq += 1;
            continue;
        }

        (*freq_tbl.add(i as usize)).freq = freq;
        (*freq_tbl.add(i as usize)).l = freq / xo_freq;
        (*freq_tbl.add(i as usize)).n = 1;

        dev_pm_opp_put(opp);
        i += 1;
        freq += 1;
    }

    freq_tbl
}

unsafe fn qcom_a53pll_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let np: *mut device_node = (*dev).of_node;
    let mut regmap: *mut regmap;
    let mut pll: *mut clk_pll;
    let base: *mut core::ffi::c_void;
    let mut init: clk_init_data = core::mem::zeroed();
    let mut ret: c_int;

    pll = devm_kzalloc(dev, core::mem::size_of::<clk_pll>(), GFP_KERNEL) as *mut clk_pll;
    if pll.is_null() {
        return -ENOMEM;
    }

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    regmap = devm_regmap_init_mmio(dev, base, &A53PLL_REGMAP_CONFIG);
    if IS_ERR(regmap) {
        return PTR_ERR(regmap);
    }

    (*pll).l_reg = 0x04;
    (*pll).m_reg = 0x08;
    (*pll).n_reg = 0x0c;
    (*pll).config_reg = 0x14;
    (*pll).mode_reg = 0x00;
    (*pll).status_reg = 0x1c;
    (*pll).status_bit = 16;

    (*pll).freq_tbl = qcom_a53pll_get_freq_tbl(dev);
    if (*pll).freq_tbl.is_null() {
        /* Fall on a53pll_freq if no freq_tbl is found from OPP */
        (*pll).freq_tbl = A53PLL_FREQ.as_ptr() as *mut pll_freq_tbl;
    }

    /* Use an unique name by appending @unit-address */
    init.name = devm_kasprintf(dev, GFP_KERNEL, b"a53pll%s\0".as_ptr() as *const c_char,
                               strchrnul((*np).full_name, b'@' as c_int));
    if init.name.is_null() {
        return -ENOMEM;
    }

    init.parent_data = &clk_parent_data { fw_name: b"xo\0".as_ptr() as *const c_char, name: b"xo_board\0".as_ptr() as *const c_char };
    init.num_parents = 1;
    init.ops = &clk_pll_sr2_ops;
    (*pll).clkr.hw.init = &mut init;

    ret = devm_clk_register_regmap(dev, &mut (*pll).clkr);
    if ret != 0 {
        dev_err(dev, b"failed to register regmap clock: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, &mut (*pll).clkr.hw);
    if ret != 0 {
        dev_err(dev, b"failed to add clock provider: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

static QCOM_A53PLL_MATCH_TABLE: [of_device_id; 4] = [
    of_device_id { compatible: b"qcom,msm8226-a7pll\0".as_ptr() as *const c_char },
    of_device_id { compatible: b"qcom,msm8916-a53pll\0".as_ptr() as *const c_char },
    of_device_id { compatible: b"qcom,msm8939-a53pll\0".as_ptr() as *const c_char },
    of_device_id { ..Default::default() },
];

MODULE_DEVICE_TABLE!(of, QCOM_A53PLL_MATCH_TABLE);

static mut QCOM_A53PLL_DRIVER: platform_driver = platform_driver {
    probe: Some(qcom_a53pll_probe),
    driver: device_driver {
        name: b"qcom-a53pll\0".as_ptr() as *const c_char,
        of_match_table: QCOM_A53PLL_MATCH_TABLE.as_ptr(),
        ..Default::default()
    },
};

module_platform_driver!(QCOM_A53PLL_DRIVER);

MODULE_DESCRIPTION!("Qualcomm A53 PLL Driver");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
