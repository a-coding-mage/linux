// SPDX-License-Identifier: GPL-2.0-only
/*
 * Samsung Exynos ACPM protocol based clock driver.
 *
 * Copyright 2025 Linaro Ltd.
 */

// Linux dependencies supplied by other files are intentionally not redefined here.

#[repr(C)]
pub struct acpm_clk {
    pub id: u32,
    pub hw: clk_hw,
    pub mbox_chan_id: c_uint,
    pub handle: *mut acpm_handle,
}

#[repr(C)]
pub struct acpm_clk_variant {
    pub name: *const c_char,
}

#[repr(C)]
pub struct acpm_clk_driver_data {
    pub clks: *const acpm_clk_variant,
    pub nr_clks: c_uint,
    pub mbox_chan_id: c_uint,
}

// `container_of(clk, struct acpm_clk, hw)`
#[inline]
unsafe fn to_acpm_clk(clk: *mut clk_hw) -> *mut acpm_clk {
    (clk as *mut u8).sub(core::mem::offset_of!(acpm_clk, hw)) as *mut acpm_clk
}

macro_rules! ACPM_CLK {
    ($cname:expr) => {
        acpm_clk_variant { name: $cname.as_ptr() as *const c_char }
    };
}

static gs101_acpm_clks: [acpm_clk_variant; 14] = [
    ACPM_CLK!("mif\0"),
    ACPM_CLK!("int\0"),
    ACPM_CLK!("cpucl0\0"),
    ACPM_CLK!("cpucl1\0"),
    ACPM_CLK!("cpucl2\0"),
    ACPM_CLK!("g3d\0"),
    ACPM_CLK!("g3dl2\0"),
    ACPM_CLK!("tpu\0"),
    ACPM_CLK!("intcam\0"),
    ACPM_CLK!("tnr\0"),
    ACPM_CLK!("cam\0"),
    ACPM_CLK!("mfc\0"),
    ACPM_CLK!("disp\0"),
    ACPM_CLK!("bo\0"),
];

static acpm_clk_gs101: acpm_clk_driver_data = acpm_clk_driver_data {
    clks: gs101_acpm_clks.as_ptr(),
    nr_clks: gs101_acpm_clks.len() as c_uint,
    mbox_chan_id: 0,
};

unsafe fn acpm_clk_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    let clk = &mut *to_acpm_clk(hw);
    ((*(*clk.handle).ops).dvfs.get_rate)(clk.handle, clk.mbox_chan_id, clk.id)
}

unsafe fn acpm_clk_set_rate(hw: *mut clk_hw, rate: c_ulong, _parent_rate: c_ulong) -> c_int {
    let clk = &mut *to_acpm_clk(hw);
    ((*(*clk.handle).ops).dvfs.set_rate)(clk.handle, clk.mbox_chan_id, clk.id, rate)
}

static acpm_clk_ops: clk_ops = clk_ops {
    recalc_rate: Some(acpm_clk_recalc_rate),
    determine_rate: Some(clk_determine_rate_noop),
    set_rate: Some(acpm_clk_set_rate),
};

unsafe fn acpm_clk_register(dev: *mut device, aclk: *mut acpm_clk, name: *const c_char) -> c_int {
    let mut init: clk_init_data = core::mem::zeroed();
    init.name = name;
    init.ops = &acpm_clk_ops;
    (*aclk).hw.init = &init;
    devm_clk_hw_register(dev, &mut (*aclk).hw)
}

unsafe fn acpm_clk_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev;
    let acpm_handle = devm_acpm_get_by_node(dev, (*(*dev).parent).of_node);
    if IS_ERR(acpm_handle) {
        return dev_err_probe(dev, PTR_ERR(acpm_handle), "Failed to get acpm handle\0".as_ptr() as *const c_char);
    }

    let count = acpm_clk_gs101.nr_clks as c_int;
    let mbox_chan_id = acpm_clk_gs101.mbox_chan_id;
    let clk_data = devm_kzalloc(dev, struct_size_clk_hw_onecell_data(count), GFP_KERNEL);
    if clk_data.is_null() { return -ENOMEM; }
    (*clk_data).num = count as c_uint;
    let hws = (*clk_data).hws;
    let aclks = devm_kcalloc(dev, count as usize, core::mem::size_of::<acpm_clk>(), GFP_KERNEL);
    if aclks.is_null() { return -ENOMEM; }

    for i in 0..count {
        let aclk = aclks.add(i as usize);
        // The code assumes the clock IDs start from zero, are sequential and do not have gaps.
        (*aclk).id = i as u32;
        (*aclk).handle = acpm_handle;
        (*aclk).mbox_chan_id = mbox_chan_id;
        *hws.add(i as usize) = &mut (*aclk).hw;
        let err = acpm_clk_register(dev, aclk, (*acpm_clk_gs101.clks.add(i as usize)).name);
        if err != 0 { return dev_err_probe(dev, err, "Failed to register clock\0".as_ptr() as *const c_char); }
    }
    devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get, clk_data)
}

static acpm_clk_id: [platform_device_id; 2] = [
    platform_device_id { name: "gs101-acpm-clk\0".as_ptr() as *const c_char },
    platform_device_id { name: core::ptr::null() },
];

static mut acpm_clk_driver: platform_driver = platform_driver {
    driver: device_driver { name: "acpm-clocks\0".as_ptr() as *const c_char },
    probe: Some(acpm_clk_probe),
    id_table: acpm_clk_id.as_ptr(),
};

// MODULE_DEVICE_TABLE(platform, acpm_clk_id);
// module_platform_driver(acpm_clk_driver);
// MODULE_AUTHOR("Tudor Ambarus <tudor.ambarus@linaro.org>");
// MODULE_DESCRIPTION("Samsung Exynos ACPM clock driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
