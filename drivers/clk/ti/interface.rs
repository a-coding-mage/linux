// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP interface clock support
 *
 * Copyright (C) 2013 Texas Instruments, Inc.
 *
 * Tero Kristo <t-kristo@ti.com>
 */

// Dependencies are supplied by the surrounding kernel translation unit.

static TI_INTERFACE_CLK_OPS: clk_ops = clk_ops {
    init: Some(omap2_init_clk_clkdm),
    enable: Some(omap2_dflt_clk_enable),
    disable: Some(omap2_dflt_clk_disable),
    is_enabled: Some(omap2_dflt_clk_is_enabled),
};

unsafe fn _register_interface(
    node: *mut device_node,
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    reg: *mut clk_omap_reg,
    bit_idx: u8,
    ops: *const clk_hw_omap_ops,
) -> *mut clk {
    let mut init: clk_init_data = core::mem::zeroed();
    let clk_hw = kzalloc(core::mem::size_of::<clk_hw_omap>()) as *mut clk_hw_omap;
    if clk_hw.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*clk_hw).hw.init = &mut init;
    (*clk_hw).ops = ops;
    core::ptr::copy_nonoverlapping(
        reg,
        &mut (*clk_hw).enable_reg,
        1,
    );
    (*clk_hw).enable_bit = bit_idx;

    init.name = name;
    init.ops = &TI_INTERFACE_CLK_OPS;
    init.flags = 0;
    init.num_parents = 1;
    init.parent_names = &parent_name;

    let clk = of_ti_clk_register_omap_hw(node, &mut (*clk_hw).hw, name);
    if IS_ERR(clk) {
        kfree(clk_hw as *mut core::ffi::c_void);
    }
    clk
}

unsafe fn __of_ti_interface_clk_setup(
    node: *mut device_node,
    ops: *const clk_hw_omap_ops,
) {
    let mut reg: clk_omap_reg = core::mem::zeroed();
    if ti_clk_get_reg_addr(node, 0, &mut reg) != 0 {
        return;
    }

    let enable_bit = reg.bit;
    let parent_name = of_clk_get_parent_name(node, 0);
    if parent_name.is_null() {
        pr_err!("%pOFn must have a parent\n", node);
        return;
    }

    let name = ti_dt_clk_name(node);
    let clk = _register_interface(node, name, parent_name, &mut reg, enable_bit, ops);
    if !IS_ERR(clk) {
        of_clk_add_provider(node, of_clk_src_simple_get, clk);
    }
}

unsafe fn of_ti_interface_clk_setup(node: *mut device_node) {
    __of_ti_interface_clk_setup(node, &clkhwops_iclk_wait);
}
// CLK_OF_DECLARE(ti_interface_clk, "ti,omap3-interface-clock", of_ti_interface_clk_setup);

unsafe fn of_ti_no_wait_interface_clk_setup(node: *mut device_node) {
    __of_ti_interface_clk_setup(node, &clkhwops_iclk);
}
// CLK_OF_DECLARE(ti_no_wait_interface_clk, "ti,omap3-no-wait-interface-clock", of_ti_no_wait_interface_clk_setup);

#[cfg(CONFIG_ARCH_OMAP3)]
unsafe fn of_ti_hsotgusb_interface_clk_setup(node: *mut device_node) {
    __of_ti_interface_clk_setup(node, &clkhwops_omap3430es2_iclk_hsotgusb_wait);
}
// CLK_OF_DECLARE(ti_hsotgusb_interface_clk, "ti,omap3-hsotgusb-interface-clock", of_ti_hsotgusb_interface_clk_setup);

#[cfg(CONFIG_ARCH_OMAP3)]
unsafe fn of_ti_dss_interface_clk_setup(node: *mut device_node) {
    __of_ti_interface_clk_setup(node, &clkhwops_omap3430es2_iclk_dss_usbhost_wait);
}
// CLK_OF_DECLARE(ti_dss_interface_clk, "ti,omap3-dss-interface-clock", of_ti_dss_interface_clk_setup);

#[cfg(CONFIG_ARCH_OMAP3)]
unsafe fn of_ti_ssi_interface_clk_setup(node: *mut device_node) {
    __of_ti_interface_clk_setup(node, &clkhwops_omap3430es2_iclk_ssi_wait);
}
// CLK_OF_DECLARE(ti_ssi_interface_clk, "ti,omap3-ssi-interface-clock", of_ti_ssi_interface_clk_setup);

#[cfg(CONFIG_ARCH_OMAP3)]
unsafe fn of_ti_am35xx_interface_clk_setup(node: *mut device_node) {
    __of_ti_interface_clk_setup(node, &clkhwops_am35xx_ipss_wait);
}
// CLK_OF_DECLARE(ti_am35xx_interface_clk, "ti,am35xx-interface-clock", of_ti_am35xx_interface_clk_setup);

#[cfg(CONFIG_SOC_OMAP2430)]
unsafe fn of_ti_omap2430_interface_clk_setup(node: *mut device_node) {
    __of_ti_interface_clk_setup(node, &clkhwops_omap2430_i2chs_wait);
}
// CLK_OF_DECLARE(ti_omap2430_interface_clk, "ti,omap2430-interface-clock", of_ti_omap2430_interface_clk_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
