// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (c) 2023 Neil Armstrong <neil.armstrong@linaro.org>
 */

// Dependencies supplied by the surrounding kernel/Rust bindings.

pub unsafe fn meson_clk_hw_get(
    clkspec: *mut of_phandle_args,
    clk_hw_data: *mut core::ffi::c_void,
) -> *mut clk_hw {
    let data = &*(clk_hw_data as *const meson_clk_hw_data);
    let idx = (*clkspec).args[0] as usize;

    if idx >= data.num as usize {
        pr_err!("meson_clk_hw_get: invalid index {}\n", idx);
        return ERR_PTR(-EINVAL);
    }

    *data.hws.add(idx)
}

static mut _MESON_CLK_HW_GET_EXPORT: () = ();

unsafe fn meson_clkc_init(dev: *mut device, map: *mut regmap) -> i32 {
    let data: *const meson_clkc_data = of_device_get_match_data(dev);
    let data = if data.is_null() {
        return -EINVAL;
    } else {
        &*data
    };
    let mut hw: *mut clk_hw;
    let mut ret: i32;
    let mut i: usize = 0;

    if data.init_count != 0 {
        regmap_multi_reg_write(map, data.init_regs, data.init_count);
    }

    while i < data.hw_clks.num as usize {
        hw = *data.hw_clks.hws.add(i);

        // array might be sparse
        if hw.is_null() {
            i += 1;
            continue;
        }

        ret = devm_clk_hw_register(dev, hw);
        if ret != 0 {
            dev_err!(dev, "registering {} clock failed\n", (*(*hw).init).name);
            return ret;
        }
        i += 1;
    }

    devm_of_clk_add_hw_provider(
        dev,
        Some(meson_clk_hw_get),
        &data.hw_clks as *const meson_clk_hw_data as *mut core::ffi::c_void,
    )
}

pub unsafe fn meson_clkc_syscon_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let mut np: *mut device_node;
    let mut map: *mut regmap;

    np = of_get_parent((*dev).of_node);
    map = syscon_node_to_regmap(np);
    of_node_put(np);
    if IS_ERR(map) {
        dev_err!(dev, "failed to get parent syscon regmap\n");
        return PTR_ERR(map);
    }

    meson_clkc_init(dev, map)
}

static mut _MESON_CLKC_SYSCON_PROBE_EXPORT: () = ();

pub unsafe fn meson_clkc_mmio_probe(pdev: *mut platform_device) -> i32 {
    let data: *const meson_clkc_data;
    let dev = &mut (*pdev).dev as *mut device;
    let mut res: *mut resource = core::ptr::null_mut();
    let mut base: *mut core::ffi::c_void;
    let mut map: *mut regmap;
    let mut regmap_cfg = regmap_config {
        reg_bits: 32,
        val_bits: 32,
        reg_stride: 4,
        max_register: 0,
    };

    data = of_device_get_match_data(dev);
    if data.is_null() {
        return -EINVAL;
    }

    base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    regmap_cfg.max_register = resource_size(res) - regmap_cfg.reg_stride;

    map = devm_regmap_init_mmio(dev, base, &regmap_cfg);
    if IS_ERR(map) {
        return PTR_ERR(map);
    }

    meson_clkc_init(dev, map)
}

static mut _MESON_CLKC_MMIO_PROBE_EXPORT: () = ();

// MODULE_DESCRIPTION("Amlogic Clock Controller Utilities");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
