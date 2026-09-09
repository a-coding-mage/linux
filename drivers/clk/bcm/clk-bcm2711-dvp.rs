// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright 2020 Cerno

// Dependencies supplied by the surrounding kernel Rust bindings.

const DVP_HT_RPI_SW_INIT: usize = 0x04;
const DVP_HT_RPI_MISC_CONFIG: usize = 0x08;

const NR_CLOCKS: usize = 2;
const NR_RESETS: u32 = 6;

#[repr(C)]
struct ClkDvp {
    data: *mut clk_hw_onecell_data,
    reset: reset_simple_data,
}

static mut CLK_DVP_PARENT: clk_parent_data = clk_parent_data { index: 0 };

unsafe fn clk_dvp_probe(pdev: *mut platform_device) -> c_int {
    let mut data: *mut clk_hw_onecell_data;
    let dvp: *mut ClkDvp;
    let base: *mut core::ffi::c_void;
    let mut ret: c_int;

    dvp = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<ClkDvp>(),
        GFP_KERNEL,
    ) as *mut ClkDvp;
    if dvp.is_null() {
        return -ENOMEM;
    }
    platform_set_drvdata(pdev, dvp as *mut core::ffi::c_void);

    (*dvp).data = devm_kzalloc(
        &mut (*pdev).dev,
        struct_size::<clk_hw_onecell_data>(NR_CLOCKS),
        GFP_KERNEL,
    ) as *mut clk_hw_onecell_data;
    if (*dvp).data.is_null() {
        return -ENOMEM;
    }
    data = (*dvp).data;

    base = devm_platform_ioremap_resource(pdev, 0);
    if is_err(base) {
        return ptr_err(base);
    }

    (*dvp).reset.rcdev.owner = THIS_MODULE;
    (*dvp).reset.rcdev.nr_resets = NR_RESETS;
    (*dvp).reset.rcdev.ops = &reset_simple_ops;
    (*dvp).reset.rcdev.of_node = (*pdev).dev.of_node;
    (*dvp).reset.membase = (base as *mut u8).add(DVP_HT_RPI_SW_INIT);
    spin_lock_init(&mut (*dvp).reset.lock);

    ret = devm_reset_controller_register(&mut (*pdev).dev, &mut (*dvp).reset.rcdev);
    if ret != 0 {
        return ret;
    }

    (*data).num = NR_CLOCKS;

    (*data).hws[0] = clk_hw_register_gate_parent_data(
        &mut (*pdev).dev,
        c"hdmi0-108MHz".as_ptr(),
        &raw mut CLK_DVP_PARENT,
        0,
        (base as *mut u8).add(DVP_HT_RPI_MISC_CONFIG),
        3,
        CLK_GATE_SET_TO_DISABLE,
        &mut (*dvp).reset.lock,
    );
    if is_err((*data).hws[0]) {
        return ptr_err((*data).hws[0]);
    }

    (*data).hws[1] = clk_hw_register_gate_parent_data(
        &mut (*pdev).dev,
        c"hdmi1-108MHz".as_ptr(),
        &raw mut CLK_DVP_PARENT,
        0,
        (base as *mut u8).add(DVP_HT_RPI_MISC_CONFIG),
        4,
        CLK_GATE_SET_TO_DISABLE,
        &mut (*dvp).reset.lock,
    );
    if is_err((*data).hws[1]) {
        ret = ptr_err((*data).hws[1]);
        goto_unregister_clk0(data);
    }

    ret = of_clk_add_hw_provider((*pdev).dev.of_node, of_clk_hw_onecell_get, data);
    if ret != 0 {
        goto_unregister_clk1(data);
    }

    0
}

unsafe fn goto_unregister_clk1(data: *mut clk_hw_onecell_data) -> ! {
    clk_hw_unregister_gate((*data).hws[1]);
    goto_unregister_clk0(data)
}

unsafe fn goto_unregister_clk0(data: *mut clk_hw_onecell_data) -> ! {
    clk_hw_unregister_gate((*data).hws[0]);
    panic!("C control-flow label reached")
}

unsafe fn clk_dvp_remove(pdev: *mut platform_device) {
    let dvp = platform_get_drvdata(pdev) as *mut ClkDvp;
    let data = (*dvp).data;

    clk_hw_unregister_gate((*data).hws[1]);
    clk_hw_unregister_gate((*data).hws[0]);
}

static CLK_DVP_DT_IDS: [of_device_id; 2] = [
    of_device_id { compatible: c"brcm,brcm2711-dvp".as_ptr() },
    of_device_id::sentinel(),
];

static mut CLK_DVP_DRIVER: platform_driver = platform_driver {
    probe: Some(clk_dvp_probe),
    remove: Some(clk_dvp_remove),
    driver: driver {
        name: c"brcm2711-dvp".as_ptr(),
        of_match_table: CLK_DVP_DT_IDS.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, clk_dvp_dt_ids);
// module_platform_driver(clk_dvp_driver);
// MODULE_AUTHOR("Maxime Ripard <maxime@cerno.tech>");
// MODULE_DESCRIPTION("BCM2711 DVP clock driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
