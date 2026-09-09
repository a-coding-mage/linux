// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Socionext Inc.
 *   Author: Masahiro Yamada <yamada.masahiro@socionext.com>
 */

// Translated from clk-uniphier-core.c. Kernel and clk-uniphier.h definitions
// are supplied by the surrounding build environment.

unsafe fn uniphier_clk_register(
    dev: *mut device,
    regmap: *mut regmap,
    data: *const uniphier_clk_data,
) -> *mut clk_hw {
    match (*data).type_ {
        UNIPHIER_CLK_TYPE_CPUGEAR => uniphier_clk_register_cpugear(
            dev,
            regmap,
            (*data).name,
            &(*data).data.cpugear,
        ),
        UNIPHIER_CLK_TYPE_FIXED_FACTOR => uniphier_clk_register_fixed_factor(
            dev,
            (*data).name,
            &(*data).data.factor,
        ),
        UNIPHIER_CLK_TYPE_FIXED_RATE => uniphier_clk_register_fixed_rate(
            dev,
            (*data).name,
            &(*data).data.rate,
        ),
        UNIPHIER_CLK_TYPE_GATE => uniphier_clk_register_gate(
            dev,
            regmap,
            (*data).name,
            &(*data).data.gate,
        ),
        UNIPHIER_CLK_TYPE_MUX => uniphier_clk_register_mux(
            dev,
            regmap,
            (*data).name,
            &(*data).data.mux,
        ),
        _ => {
            dev_err(dev, c"unsupported clock type\n".as_ptr());
            ERR_PTR(-EINVAL)
        }
    }
}

unsafe fn uniphier_clk_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let mut hw_data: *mut clk_hw_onecell_data;
    let mut p: *const uniphier_clk_data;
    let mut data: *const uniphier_clk_data;
    let mut regmap: *mut regmap;
    let mut parent: *mut device_node;
    let mut clk_num: i32 = 0;

    data = of_device_get_match_data(dev);
    if WARN_ON(data.is_null()) {
        return -EINVAL;
    }

    parent = of_get_parent((*dev).of_node); // parent should be syscon node
    regmap = syscon_node_to_regmap(parent);
    of_node_put(parent);
    if IS_ERR(regmap) {
        dev_err(dev, c"failed to get regmap (error %ld)\n".as_ptr(), PTR_ERR(regmap));
        return PTR_ERR(regmap) as i32;
    }

    p = data;
    while !(*p).name.is_null() {
        clk_num = max(clk_num, (*p).idx + 1);
        p = p.add(1);
    }

    hw_data = devm_kzalloc(
        dev,
        struct_size::<clk_hw_onecell_data>(hw_data, hws, clk_num),
        GFP_KERNEL,
    );
    if hw_data.is_null() {
        return -ENOMEM;
    }

    (*hw_data).num = clk_num as _;

    // avoid returning NULL for unused idx
    clk_num -= 1;
    while clk_num >= 0 {
        (*hw_data).hws[clk_num as usize] = ERR_PTR(-EINVAL);
        clk_num -= 1;
    }

    p = data;
    while !(*p).name.is_null() {
        let hw: *mut clk_hw;

        dev_dbg(dev, c"register %s (index=%d)\n".as_ptr(), (*p).name, (*p).idx);
        hw = uniphier_clk_register(dev, regmap, p);
        if WARN(IS_ERR(hw), c"failed to register %s".as_ptr(), (*p).name) {
            p = p.add(1);
            continue;
        }

        if (*p).idx >= 0 {
            (*hw_data).hws[(*p).idx as usize] = hw;
        }
        p = p.add(1);
    }

    devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get, hw_data)
}

static mut uniphier_clk_match: [of_device_id; 31] = [
    of_device_id { compatible: c"socionext,uniphier-ld4-clock".as_ptr(), data: uniphier_ld4_sys_clk_data },
    of_device_id { compatible: c"socionext,uniphier-pro4-clock".as_ptr(), data: uniphier_pro4_sys_clk_data },
    of_device_id { compatible: c"socionext,uniphier-sld8-clock".as_ptr(), data: uniphier_sld8_sys_clk_data },
    of_device_id { compatible: c"socionext,uniphier-pro5-clock".as_ptr(), data: uniphier_pro5_sys_clk_data },
    of_device_id { compatible: c"socionext,uniphier-pxs2-clock".as_ptr(), data: uniphier_pxs2_sys_clk_data },
    of_device_id { compatible: c"socionext,uniphier-ld11-clock".as_ptr(), data: uniphier_ld11_sys_clk_data },
    of_device_id { compatible: c"socionext,uniphier-ld20-clock".as_ptr(), data: uniphier_ld20_sys_clk_data },
    of_device_id { compatible: c"socionext,uniphier-pxs3-clock".as_ptr(), data: uniphier_pxs3_sys_clk_data },
    of_device_id { compatible: c"socionext,uniphier-nx1-clock".as_ptr(), data: uniphier_nx1_sys_clk_data },
    of_device_id { compatible: c"socionext,uniphier-ld4-mio-clock".as_ptr(), data: uniphier_ld4_mio_clk_data },
    of_device_id { compatible: c"socionext,uniphier-pro4-mio-clock".as_ptr(), data: uniphier_ld4_mio_clk_data },
    of_device_id { compatible: c"socionext,uniphier-sld8-mio-clock".as_ptr(), data: uniphier_ld4_mio_clk_data },
    of_device_id { compatible: c"socionext,uniphier-pro5-sd-clock".as_ptr(), data: uniphier_pro5_sd_clk_data },
    of_device_id { compatible: c"socionext,uniphier-pxs2-sd-clock".as_ptr(), data: uniphier_pro5_sd_clk_data },
    of_device_id { compatible: c"socionext,uniphier-ld11-mio-clock".as_ptr(), data: uniphier_ld4_mio_clk_data },
    of_device_id { compatible: c"socionext,uniphier-ld20-sd-clock".as_ptr(), data: uniphier_pro5_sd_clk_data },
    of_device_id { compatible: c"socionext,uniphier-pxs3-sd-clock".as_ptr(), data: uniphier_pro5_sd_clk_data },
    of_device_id { compatible: c"socionext,uniphier-nx1-sd-clock".as_ptr(), data: uniphier_pro5_sd_clk_data },
    of_device_id { compatible: c"socionext,uniphier-ld4-peri-clock".as_ptr(), data: uniphier_ld4_peri_clk_data },
    of_device_id { compatible: c"socionext,uniphier-pro4-peri-clock".as_ptr(), data: uniphier_pro4_peri_clk_data },
    of_device_id { compatible: c"socionext,uniphier-sld8-peri-clock".as_ptr(), data: uniphier_ld4_peri_clk_data },
    of_device_id { compatible: c"socionext,uniphier-pro5-peri-clock".as_ptr(), data: uniphier_pro4_peri_clk_data },
    of_device_id { compatible: c"socionext,uniphier-pxs2-peri-clock".as_ptr(), data: uniphier_pro4_peri_clk_data },
    of_device_id { compatible: c"socionext,uniphier-ld11-peri-clock".as_ptr(), data: uniphier_pro4_peri_clk_data },
    of_device_id { compatible: c"socionext,uniphier-ld20-peri-clock".as_ptr(), data: uniphier_pro4_peri_clk_data },
    of_device_id { compatible: c"socionext,uniphier-pxs3-peri-clock".as_ptr(), data: uniphier_pro4_peri_clk_data },
    of_device_id { compatible: c"socionext,uniphier-nx1-peri-clock".as_ptr(), data: uniphier_pro4_peri_clk_data },
    of_device_id { compatible: c"socionext,uniphier-pro4-sg-clock".as_ptr(), data: uniphier_pro4_sg_clk_data },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

static mut uniphier_clk_driver: platform_driver = platform_driver {
    probe: Some(uniphier_clk_probe),
    driver: driver {
        name: c"uniphier-clk".as_ptr(),
        of_match_table: uniphier_clk_match.as_ptr(),
    },
};

// builtin_platform_driver(uniphier_clk_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
