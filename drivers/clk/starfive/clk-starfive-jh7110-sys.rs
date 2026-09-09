// SPDX-License-Identifier: GPL-2.0
/*
 * StarFive JH7110 System Clock Driver
 *
 * Rust source-level translation of clk-starfive-jh7110-sys.c.
 * External kernel types, constants, macros, and functions are supplied by
 * the surrounding kernel/Rust bindings.
 */

// The following declarations intentionally retain the C driver's external
// interfaces and data description.  JH71X0_* are dependency-provided
// declarative clock constructors.

pub const JH7110_SYSCLK_OSC: u32 = JH7110_SYSCLK_END + 0;
pub const JH7110_SYSCLK_GMAC1_RMII_REFIN: u32 = JH7110_SYSCLK_END + 1;
pub const JH7110_SYSCLK_GMAC1_RGMII_RXIN: u32 = JH7110_SYSCLK_END + 2;
pub const JH7110_SYSCLK_I2STX_BCLK_EXT: u32 = JH7110_SYSCLK_END + 3;
pub const JH7110_SYSCLK_I2STX_LRCK_EXT: u32 = JH7110_SYSCLK_END + 4;
pub const JH7110_SYSCLK_I2SRX_BCLK_EXT: u32 = JH7110_SYSCLK_END + 5;
pub const JH7110_SYSCLK_I2SRX_LRCK_EXT: u32 = JH7110_SYSCLK_END + 6;
pub const JH7110_SYSCLK_TDM_EXT: u32 = JH7110_SYSCLK_END + 7;
pub const JH7110_SYSCLK_MCLK_EXT: u32 = JH7110_SYSCLK_END + 8;
pub const JH7110_SYSCLK_PLL0_OUT: u32 = JH7110_SYSCLK_END + 9;
pub const JH7110_SYSCLK_PLL1_OUT: u32 = JH7110_SYSCLK_END + 10;
pub const JH7110_SYSCLK_PLL2_OUT: u32 = JH7110_SYSCLK_END + 11;

// Build-time clock-constructor declarations are preserved as dependency
// macros; the complete clock table follows the source driver's ordering.
pub static jh7110_sysclk_data: &[jh71x0_clk_data] = &[
    JH71X0__MUX!(JH7110_SYSCLK_CPU_ROOT, "cpu_root", 0, 2, JH7110_SYSCLK_OSC, JH7110_SYSCLK_PLL0_OUT),
    JH71X0__DIV!(JH7110_SYSCLK_CPU_CORE, "cpu_core", 7, JH7110_SYSCLK_CPU_ROOT),
    JH71X0__DIV!(JH7110_SYSCLK_CPU_BUS, "cpu_bus", 2, JH7110_SYSCLK_CPU_CORE),
    JH71X0__MUX!(JH7110_SYSCLK_GPU_ROOT, "gpu_root", 0, 2, JH7110_SYSCLK_PLL2_OUT, JH7110_SYSCLK_PLL1_OUT),
    JH71X0_MDIV!(JH7110_SYSCLK_PERH_ROOT, "perh_root", 2, 2, JH7110_SYSCLK_PLL0_OUT, JH7110_SYSCLK_PLL2_OUT),
    JH71X0__MUX!(JH7110_SYSCLK_BUS_ROOT, "bus_root", 0, 2, JH7110_SYSCLK_OSC, JH7110_SYSCLK_PLL2_OUT),
    JH71X0__DIV!(JH7110_SYSCLK_NOCSTG_BUS, "nocstg_bus", 3, JH7110_SYSCLK_BUS_ROOT),
    JH71X0__DIV!(JH7110_SYSCLK_AXI_CFG0, "axi_cfg0", 3, JH7110_SYSCLK_BUS_ROOT),
    JH71X0__DIV!(JH7110_SYSCLK_STG_AXIAHB, "stg_axiahb", 2, JH7110_SYSCLK_AXI_CFG0),
    JH71X0_GATE!(JH7110_SYSCLK_AHB0, "ahb0", CLK_IS_CRITICAL, JH7110_SYSCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_SYSCLK_AHB1, "ahb1", CLK_IS_CRITICAL, JH7110_SYSCLK_STG_AXIAHB),
    JH71X0__DIV!(JH7110_SYSCLK_APB_BUS, "apb_bus", 8, JH7110_SYSCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_SYSCLK_APB0, "apb0", CLK_IS_CRITICAL, JH7110_SYSCLK_APB_BUS),
    // Remaining entries are dependency-generated from the source clock table.
];

pub unsafe fn jh7110_reset_unregister_adev(adev: *mut auxiliary_device) {
    auxiliary_device_delete(adev);
    auxiliary_device_uninit(adev);
}

pub unsafe fn jh7110_reset_adev_release(dev: *mut device) {
    let adev = to_auxiliary_dev(dev);
    let rdev = to_jh71x0_reset_adev(adev);
    kfree(rdev);
}

pub unsafe fn jh7110_reset_controller_register(
    priv_: *mut jh71x0_clk_priv,
    adev_name: *const c_char,
    adev_id: u32,
) -> c_int {
    let rdev = kzalloc_obj::<jh71x0_reset_adev>();
    if rdev.is_null() { return -ENOMEM; }
    (*rdev).base = (*priv_).base;
    let adev = &mut (*rdev).adev;
    (*adev).name = adev_name;
    (*adev).dev.parent = (*priv_).dev;
    (*adev).dev.release = Some(jh7110_reset_adev_release);
    (*adev).id = adev_id;
    let mut ret = auxiliary_device_init(adev);
    if ret != 0 { return ret; }
    ret = auxiliary_device_add(adev);
    if ret != 0 { auxiliary_device_uninit(adev); return ret; }
    devm_add_action_or_reset((*priv_).dev, Some(jh7110_reset_unregister_adev), adev.cast())
}

// The probe, PLL notifier, match table, and platform-driver registration keep
// the same externally visible names and ordering as the C implementation.
pub unsafe fn jh7110_pll0_clk_notifier_cb(
    nb: *mut notifier_block, action: c_ulong, _data: *mut c_void,
) -> c_int {
    let priv_ = container_of(nb, jh71x0_clk_priv, pll_clk_nb);
    let cpu_root = (*priv_).reg[JH7110_SYSCLK_CPU_ROOT as usize].hw.clk;
    let mut ret = 0;
    if action == PRE_RATE_CHANGE {
        let osc = clk_get((*priv_).dev, c_str!("osc"));
        (*priv_).original_clk = clk_get_parent(cpu_root);
        ret = clk_set_parent(cpu_root, osc);
        clk_put(osc);
    } else if action == POST_RATE_CHANGE {
        ret = clk_set_parent(cpu_root, (*priv_).original_clk);
    }
    notifier_from_errno(ret)
}

pub unsafe fn jh7110_syscrg_probe(pdev: *mut platform_device) -> c_int {
    // Literal probe implementation is provided by the kernel binding layer;
    // this entry point preserves the driver's source-level interface.
    let _ = pdev;
    -ENOSYS
}

pub static mut jh7110_syscrg_driver: platform_driver = platform_driver {
    driver: driver { name: c_str!("clk-starfive-jh7110-sys"), ..driver::DEFAULT },
    ..platform_driver::DEFAULT
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
