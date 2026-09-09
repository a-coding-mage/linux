// SPDX-License-Identifier: GPL-2.0+
//
// clk-s2mps11.c - Clock driver for S2MPS11.
//
// Copyright (C) 2013,2014 Samsung Electornics

// External Linux kernel declarations and constants are supplied by the
// corresponding kernel headers.

#[repr(C)]
struct S2mps11Clk {
    iodev: *mut SecPmicDev,
    clk_np: *mut DeviceNode,
    hw: ClkHw,
    clk: *mut Clk,
    lookup: *mut ClkLookup,
    mask: u32,
    reg: u32,
}

unsafe fn to_s2mps11_clk(hw: *mut ClkHw) -> *mut S2mps11Clk {
    container_of!(hw, S2mps11Clk, hw)
}

unsafe extern "C" fn s2mps11_clk_prepare(hw: *mut ClkHw) -> i32 {
    let s2mps11 = &mut *to_s2mps11_clk(hw);
    regmap_update_bits((*s2mps11.iodev).regmap_pmic, s2mps11.reg,
                       s2mps11.mask, s2mps11.mask)
}

unsafe extern "C" fn s2mps11_clk_unprepare(hw: *mut ClkHw) {
    let s2mps11 = &mut *to_s2mps11_clk(hw);
    regmap_update_bits((*s2mps11.iodev).regmap_pmic, s2mps11.reg,
                       s2mps11.mask, !s2mps11.mask);
}

unsafe extern "C" fn s2mps11_clk_is_prepared(hw: *mut ClkHw) -> i32 {
    let mut val: u32 = 0;
    let s2mps11 = &mut *to_s2mps11_clk(hw);
    let ret = regmap_read((*s2mps11.iodev).regmap_pmic, s2mps11.reg, &mut val);
    if ret < 0 {
        return -EINVAL;
    }
    (val & s2mps11.mask) as i32
}

unsafe extern "C" fn s2mps11_clk_recalc_rate(
    _hw: *mut ClkHw,
    _parent_rate: usize,
) -> usize {
    32768
}

static S2MPS11_CLK_OPS: ClkOps = ClkOps {
    prepare: Some(s2mps11_clk_prepare),
    unprepare: Some(s2mps11_clk_unprepare),
    is_prepared: Some(s2mps11_clk_is_prepared),
    recalc_rate: Some(s2mps11_clk_recalc_rate),
};

/* This s2mps11_clks_init structure is common to s2mps11, s2mps13 and s2mps14 */
static mut S2MPS11_CLKS_INIT: [ClkInitData; S2MPS11_CLKS_NUM as usize] = [
    ClkInitData { name: "s2mps11_ap", ops: &S2MPS11_CLK_OPS },
    ClkInitData { name: "s2mps11_cp", ops: &S2MPS11_CLK_OPS },
    ClkInitData { name: "s2mps11_bt", ops: &S2MPS11_CLK_OPS },
];

unsafe fn s2mps11_clk_parse_dt(
    pdev: *mut PlatformDevice,
    clks_init: *mut ClkInitData,
) -> *mut DeviceNode {
    let iodev = dev_get_drvdata((*pdev).dev.parent) as *mut SecPmicDev;
    if (*(*iodev).dev).of_node.is_null() {
        return ERR_PTR(-EINVAL);
    }
    let clk_np = of_get_child_by_name((*(*iodev).dev).of_node, "clocks");
    if clk_np.is_null() {
        dev_err(&(*pdev).dev, "could not find clock sub-node\n");
        return ERR_PTR(-EINVAL);
    }
    for i in 0..S2MPS11_CLKS_NUM as usize {
        of_property_read_string_index(clk_np, "clock-output-names", i,
                                      &mut (*clks_init.add(i)).name);
    }
    clk_np
}

unsafe extern "C" fn s2mps11_clk_probe(pdev: *mut PlatformDevice) -> i32 {
    let iodev = dev_get_drvdata((*pdev).dev.parent) as *mut SecPmicDev;
    let mut s2mps11_clks = devm_kcalloc(&mut (*pdev).dev, S2MPS11_CLKS_NUM,
                                         core::mem::size_of::<S2mps11Clk>(), GFP_KERNEL)
        as *mut S2mps11Clk;
    if s2mps11_clks.is_null() { return -ENOMEM; }
    let clk_data = devm_kzalloc(&mut (*pdev).dev,
        struct_size!(ClkHwOnecellData, hws, S2MPS11_CLKS_NUM), GFP_KERNEL)
        as *mut ClkHwOnecellData;
    if clk_data.is_null() { return -ENOMEM; }
    (*clk_data).num = S2MPS11_CLKS_NUM;
    let hwid = platform_get_device_id(pdev).driver_data;
    let s2mps11_reg = match hwid {
        S2MPG10 => S2MPG10_PMIC_RTCBUF,
        S2MPS11X => S2MPS11_REG_RTC_CTRL,
        S2MPS13X => S2MPS13_REG_RTCCTRL,
        S2MPS14X => S2MPS14_REG_RTCCTRL,
        S5M8767X => S5M8767_REG_CTRL1,
        _ => { dev_err(&(*pdev).dev, "Invalid device type\n"); return -EINVAL; }
    };
    (*s2mps11_clks).clk_np = s2mps11_clk_parse_dt(pdev, S2MPS11_CLKS_INIT.as_mut_ptr());
    if IS_ERR!((*s2mps11_clks).clk_np) { return PTR_ERR!((*s2mps11_clks).clk_np); }
    let mut i = 0usize;
    while i < S2MPS11_CLKS_NUM as usize {
        if i == S2MPS11_CLK_CP as usize && hwid == S2MPS14X { i += 1; continue; }
        (*s2mps11_clks.add(i)).iodev = iodev;
        (*s2mps11_clks.add(i)).hw.init = &mut S2MPS11_CLKS_INIT[i];
        (*s2mps11_clks.add(i)).mask = 1u32 << i;
        (*s2mps11_clks.add(i)).reg = s2mps11_reg;
        (*s2mps11_clks.add(i)).clk = devm_clk_register(&mut (*pdev).dev, &mut (*s2mps11_clks.add(i)).hw);
        if IS_ERR!((*s2mps11_clks.add(i)).clk) { dev_err(&(*pdev).dev, "Fail to register : %s\n", S2MPS11_CLKS_INIT[i].name); return PTR_ERR!((*s2mps11_clks.add(i)).clk); }
        (*s2mps11_clks.add(i)).lookup = clkdev_hw_create(&mut (*s2mps11_clks.add(i)).hw, S2MPS11_CLKS_INIT[i].name, core::ptr::null());
        if (*s2mps11_clks.add(i)).lookup.is_null() { return -ENOMEM; }
        (*clk_data).hws[i] = &mut (*s2mps11_clks.add(i)).hw;
        i += 1;
    }
    of_clk_add_hw_provider((*s2mps11_clks).clk_np, of_clk_hw_onecell_get, clk_data);
    platform_set_drvdata(pdev, s2mps11_clks as *mut core::ffi::c_void);
    0
}

unsafe extern "C" fn s2mps11_clk_remove(pdev: *mut PlatformDevice) {
    let clks = platform_get_drvdata(pdev) as *mut S2mps11Clk;
    of_clk_del_provider((*clks).clk_np);
    of_node_put((*clks).clk_np);
    for i in 0..S2MPS11_CLKS_NUM as usize {
        if (*clks.add(i)).lookup.is_null() { continue; }
        clkdev_drop((*clks.add(i)).lookup);
    }
}

static S2MPS11_CLK_ID: [PlatformDeviceId; 6] = [
    PlatformDeviceId { name: "s2mpg10-clk", driver_data: S2MPG10 },
    PlatformDeviceId { name: "s2mps11-clk", driver_data: S2MPS11X },
    PlatformDeviceId { name: "s2mps13-clk", driver_data: S2MPS13X },
    PlatformDeviceId { name: "s2mps14-clk", driver_data: S2MPS14X },
    PlatformDeviceId { name: "s5m8767-clk", driver_data: S5M8767X },
    PlatformDeviceId { name: "", driver_data: 0 },
];

#[cfg(CONFIG_OF)]
static S2MPS11_DT_MATCH: [OfDeviceId; 6] = [
    OfDeviceId { compatible: "samsung,s2mpg10-clk", data: S2MPG10 as *const _ },
    OfDeviceId { compatible: "samsung,s2mps11-clk", data: S2MPS11X as *const _ },
    OfDeviceId { compatible: "samsung,s2mps13-clk", data: S2MPS13X as *const _ },
    OfDeviceId { compatible: "samsung,s2mps14-clk", data: S2MPS14X as *const _ },
    OfDeviceId { compatible: "samsung,s5m8767-clk", data: S5M8767X as *const _ },
    OfDeviceId { compatible: "", data: core::ptr::null() },
];

static mut S2MPS11_CLK_DRIVER: PlatformDriver = PlatformDriver {
    driver: Driver { name: "s2mps11-clk" },
    probe: Some(s2mps11_clk_probe),
    remove: Some(s2mps11_clk_remove),
    id_table: S2MPS11_CLK_ID.as_ptr(),
};

module_platform_driver!(S2MPS11_CLK_DRIVER);
// MODULE_DESCRIPTION("S2MPS11 Clock Driver");
// MODULE_AUTHOR("Yadwinder Singh Brar <yadi.brar@samsung.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
