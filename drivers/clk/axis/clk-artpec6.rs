// SPDX-License-Identifier: GPL-2.0-only
/*
 * ARTPEC-6 clock initialization
 *
 * Copyright 2015-2016 Axis Communications AB.
 */

// C dependencies supplied by the surrounding kernel translation.

const NUM_I2S_CLOCKS: usize = 2;

#[repr(C)]
struct Artpec6ClkctrlDrvdata {
    clk_table: [*mut Clk; ARTPEC6_CLK_NUMCLOCKS as usize],
    syscon_base: *mut core::ffi::c_void,
    clk_data: ClkOnecellData,
    i2scfg_lock: Spinlock,
}

static mut clkdata: *mut Artpec6ClkctrlDrvdata = core::ptr::null_mut();

static i2s_clk_names: [&str; NUM_I2S_CLOCKS] = ["i2s0", "i2s1"];
static i2s_clk_indexes: [i32; NUM_I2S_CLOCKS] = [ARTPEC6_CLK_I2S0_CLK, ARTPEC6_CLK_I2S1_CLK];

unsafe fn of_artpec6_clkctrl_setup(np: *mut DeviceNode) {
    let mut i: i32;
    let sys_refclk_name: *const core::ffi::c_char;
    let (mut pll_mode, mut pll_m, mut pll_n): (u32, u32, u32);
    let clks: *mut *mut Clk;

    /* Mandatory parent clock. */
    i = of_property_match_string(np, "clock-names", "sys_refclk");
    if i < 0 { return; }
    sys_refclk_name = of_clk_get_parent_name(np, i);

    clkdata = kzalloc_obj::<Artpec6ClkctrlDrvdata>();
    if clkdata.is_null() { return; }
    clks = (*clkdata).clk_table.as_mut_ptr();

    for i in 0..ARTPEC6_CLK_NUMCLOCKS {
        *clks.add(i as usize) = ERR_PTR(-EPROBE_DEFER);
    }

    (*clkdata).syscon_base = of_iomap(np, 0);
    BUG_ON((*clkdata).syscon_base.is_null());

    /* Read PLL1 factors configured by boot strap pins. */
    pll_mode = (readl((*clkdata).syscon_base) >> 6) & 3;
    match pll_mode {
        0 => { /* DDR3-2133 mode */ pll_m = 4; pll_n = 85; }
        1 => { /* DDR3-1866 mode */ pll_m = 6; pll_n = 112; }
        2 => { /* DDR3-1600 mode */ pll_m = 4; pll_n = 64; }
        3 => { /* DDR3-1333 mode */ pll_m = 8; pll_n = 106; }
        _ => unreachable!(),
    }

    *clks.add(ARTPEC6_CLK_CPU as usize) = clk_register_fixed_factor(core::ptr::null_mut(), "cpu", sys_refclk_name, 0, pll_n, pll_m);
    *clks.add(ARTPEC6_CLK_CPU_PERIPH as usize) = clk_register_fixed_factor(core::ptr::null_mut(), "cpu_periph", cstr!("cpu"), 0, 1, 2);
    /* EPROBE_DEFER on the apb_clock is not handled in amba devices. */
    *clks.add(ARTPEC6_CLK_UART_PCLK as usize) = clk_register_fixed_factor(core::ptr::null_mut(), "uart_pclk", cstr!("cpu"), 0, 1, 8);
    *clks.add(ARTPEC6_CLK_UART_REFCLK as usize) = clk_register_fixed_rate(core::ptr::null_mut(), "uart_ref", sys_refclk_name, 0, 50000000);
    *clks.add(ARTPEC6_CLK_SPI_PCLK as usize) = clk_register_fixed_factor(core::ptr::null_mut(), "spi_pclk", cstr!("cpu"), 0, 1, 8);
    *clks.add(ARTPEC6_CLK_SPI_SSPCLK as usize) = clk_register_fixed_rate(core::ptr::null_mut(), "spi_sspclk", sys_refclk_name, 0, 50000000);
    *clks.add(ARTPEC6_CLK_DBG_PCLK as usize) = clk_register_fixed_factor(core::ptr::null_mut(), "dbg_pclk", cstr!("cpu"), 0, 1, 8);
    (*clkdata).clk_data.clks = (*clkdata).clk_table.as_mut_ptr();
    (*clkdata).clk_data.clk_num = ARTPEC6_CLK_NUMCLOCKS;
    of_clk_add_provider(np, of_clk_src_onecell_get, &mut (*clkdata).clk_data);
}

// CLK_OF_DECLARE_DRIVER(artpec6_clkctrl, "axis,artpec6-clkctrl", of_artpec6_clkctrl_setup);

unsafe fn artpec6_clkctrl_probe(pdev: *mut PlatformDevice) -> i32 {
    let np = (*pdev).dev.of_node;
    let dev = &mut (*pdev).dev;
    let clks = (*clkdata).clk_table.as_mut_ptr();
    let mut propidx: i32;
    let sys_refclk_name: *const core::ffi::c_char;
    let mut i2s_refclk_name: *const core::ffi::c_char = core::ptr::null();
    let mut frac_clk_name: [*const core::ffi::c_char; 2] = [core::ptr::null(); 2];
    let mut i2s_mux_parents: [*const core::ffi::c_char; 2];
    let mut muxreg: u32;
    let mut err = 0;

    /* Mandatory parent clock. */
    propidx = of_property_match_string(np, "clock-names", "sys_refclk");
    if propidx < 0 { return -EINVAL; }
    sys_refclk_name = of_clk_get_parent_name(np, propidx);
    /* Find clock names of optional parent clocks. */
    propidx = of_property_match_string(np, "clock-names", "i2s_refclk");
    if propidx >= 0 { i2s_refclk_name = of_clk_get_parent_name(np, propidx); }
    propidx = of_property_match_string(np, "clock-names", "frac_clk0");
    if propidx >= 0 { frac_clk_name[0] = of_clk_get_parent_name(np, propidx); }
    propidx = of_property_match_string(np, "clock-names", "frac_clk1");
    if propidx >= 0 { frac_clk_name[1] = of_clk_get_parent_name(np, propidx); }
    spin_lock_init(&mut (*clkdata).i2scfg_lock);

    *clks.add(ARTPEC6_CLK_NAND_CLKA as usize) = clk_register_fixed_factor(dev, "nand_clka", cstr!("cpu"), 0, 1, 8);
    *clks.add(ARTPEC6_CLK_NAND_CLKB as usize) = clk_register_fixed_rate(dev, "nand_clkb", sys_refclk_name, 0, 100000000);
    *clks.add(ARTPEC6_CLK_ETH_ACLK as usize) = clk_register_fixed_factor(dev, "eth_aclk", cstr!("cpu"), 0, 1, 4);
    *clks.add(ARTPEC6_CLK_DMA_ACLK as usize) = clk_register_fixed_factor(dev, "dma_aclk", cstr!("cpu"), 0, 1, 4);
    *clks.add(ARTPEC6_CLK_PTP_REF as usize) = clk_register_fixed_rate(dev, "ptp_ref", sys_refclk_name, 0, 100000000);
    *clks.add(ARTPEC6_CLK_SD_PCLK as usize) = clk_register_fixed_rate(dev, "sd_pclk", sys_refclk_name, 0, 100000000);
    *clks.add(ARTPEC6_CLK_SD_IMCLK as usize) = clk_register_fixed_rate(dev, "sd_imclk", sys_refclk_name, 0, 100000000);
    *clks.add(ARTPEC6_CLK_I2S_HST as usize) = clk_register_fixed_factor(dev, "i2s_hst", cstr!("cpu"), 0, 1, 8);

    for i in 0..NUM_I2S_CLOCKS {
        if !i2s_refclk_name.is_null() && !frac_clk_name[i].is_null() {
            i2s_mux_parents = [frac_clk_name[i], i2s_refclk_name];
            *clks.add(i2s_clk_indexes[i] as usize) = clk_register_mux(dev, i2s_clk_names[i], i2s_mux_parents.as_mut_ptr(), 2, CLK_SET_RATE_NO_REPARENT | CLK_SET_RATE_PARENT, (*clkdata).syscon_base.add(0x14), i as i32, 1, 0, &mut (*clkdata).i2scfg_lock);
        } else if !frac_clk_name[i].is_null() {
            /* Lock the mux for internal clock reference. */
            muxreg = readl((*clkdata).syscon_base.add(0x14)); muxreg &= !BIT(i as u32); writel(muxreg, (*clkdata).syscon_base.add(0x14));
            *clks.add(i2s_clk_indexes[i] as usize) = clk_register_fixed_factor(dev, i2s_clk_names[i], frac_clk_name[i], 0, 1, 1);
        } else if !i2s_refclk_name.is_null() {
            /* Lock the mux for external clock reference. */
            muxreg = readl((*clkdata).syscon_base.add(0x14)); muxreg |= BIT(i as u32); writel(muxreg, (*clkdata).syscon_base.add(0x14));
            *clks.add(i2s_clk_indexes[i] as usize) = clk_register_fixed_factor(dev, i2s_clk_names[i], i2s_refclk_name, 0, 1, 1);
        }
    }
    *clks.add(ARTPEC6_CLK_I2C as usize) = clk_register_fixed_rate(dev, "i2c", sys_refclk_name, 0, 100000000);
    *clks.add(ARTPEC6_CLK_SYS_TIMER as usize) = clk_register_fixed_rate(dev, "timer", sys_refclk_name, 0, 100000000);
    *clks.add(ARTPEC6_CLK_FRACDIV_IN as usize) = clk_register_fixed_rate(dev, "fracdiv_in", sys_refclk_name, 0, 600000000);
    for i in 0..ARTPEC6_CLK_NUMCLOCKS {
        if IS_ERR(*clks.add(i as usize)) && PTR_ERR(*clks.add(i as usize)) != -EPROBE_DEFER {
            dev_err(dev, "Failed to register clock at index %d err=%ld\n", i, PTR_ERR(*clks.add(i as usize)));
            err = PTR_ERR(*clks.add(i as usize));
        }
    }
    err
}

// static const struct of_device_id artpec_clkctrl_of_match[] = { { .compatible = "axis,artpec6-clkctrl" }, {} };
// static struct platform_driver artpec6_clkctrl_driver = { .probe = artpec6_clkctrl_probe, .driver = { .name = "artpec6_clkctrl", .of_match_table = artpec_clkctrl_of_match } };
// builtin_platform_driver(artpec6_clkctrl_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
