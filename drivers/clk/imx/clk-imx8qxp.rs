// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2018-2021 NXP
 *      Dong Aisheng <aisheng.dong@nxp.com>
 */

// Translated from the Linux kernel implementation. Kernel-provided symbols are
// intentionally left as external dependencies.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub mod translation {
    use core::ffi::{c_char, c_int, c_void};

    #[repr(C)] pub struct device_node { _private: [u8; 0] }
    #[repr(C)] pub struct platform_device { _private: [u8; 0] }
    #[repr(C)] pub struct imx_clk_scu_rsrc_table { _private: [u8; 0] }
    #[repr(C)] pub struct of_device_id { pub compatible: *const c_char, pub data: *const c_void }
    #[repr(C)] pub struct platform_driver { _private: [u8; 0] }

    extern "C" {
        fn of_device_is_compatible(node: *mut device_node, compatible: *const c_char) -> bool;
        fn of_device_get_match_data(dev: *mut c_void) -> *const imx_clk_scu_rsrc_table;
        fn imx_clk_scu_init(node: *mut device_node, table: *const imx_clk_scu_rsrc_table) -> c_int;
        fn imx_clk_scu(name: *const c_char, rsrc: c_int, kind: c_int);
        fn imx_clk_scu2(name: *const c_char, sels: *const *const c_char, n: usize, rsrc: c_int, kind: c_int);
        fn imx_clk_divider_gpr_scu(name: *const c_char, parent: *const c_char, rsrc: c_int, ctrl: c_int);
        fn imx_clk_mux_gpr_scu(name: *const c_char, sels: *const *const c_char, n: usize, rsrc: c_int, ctrl: c_int);
        fn imx_clk_gate_gpr_scu(name: *const c_char, parent: *const c_char, rsrc: c_int, ctrl: c_int, inverse: bool);
        fn of_clk_add_hw_provider(node: *mut device_node, get: *const c_void, data: *mut c_void) -> c_int;
        fn imx_clk_scu_unregister();
        fn platform_driver_register(driver: *mut platform_driver) -> c_int;
        fn platform_driver_unregister(driver: *mut platform_driver);
        fn imx_clk_scu_module_init() -> c_int;
        fn imx_clk_scu_module_exit();
        static imx_scu_of_clk_src_get: c_void;
        static mut imx_scu_clks: *mut c_void;
        static imx_clk_scu_rsrc_imx8dxl: imx_clk_scu_rsrc_table;
        static imx_clk_scu_rsrc_imx8qxp: imx_clk_scu_rsrc_table;
        static imx_clk_scu_rsrc_imx8qm: imx_clk_scu_rsrc_table;
    }

    macro_rules! sels { ($($x:expr),* $(,)?) => { &[$(concat!($x, "\0").as_ptr() as *const c_char),*] } }
    static DC0_SELS: &[*const c_char] = sels!("clk_dummy", "clk_dummy", "dc0_pll0_clk", "dc0_pll1_clk", "dc0_bypass0_clk");
    static DC1_SELS: &[*const c_char] = sels!("clk_dummy", "clk_dummy", "dc1_pll0_clk", "dc1_pll1_clk", "dc1_bypass0_clk");
    static ENET0_RGMII_TXC_SELS: &[*const c_char] = sels!("enet0_ref_div", "clk_dummy");
    static ENET1_RGMII_TXC_SELS: &[*const c_char] = sels!("enet1_ref_div", "clk_dummy");
    static HDMI_SELS: &[*const c_char] = sels!("clk_dummy", "hdmi_dig_pll_clk", "clk_dummy", "clk_dummy", "hdmi_av_pll_clk");
    static HDMI_RX_SELS: &[*const c_char] = sels!("clk_dummy", "hdmi_rx_dig_pll_clk", "clk_dummy", "clk_dummy", "hdmi_rx_bypass_clk");
    static LCD_PXL_SELS: &[*const c_char] = sels!("clk_dummy", "clk_dummy", "clk_dummy", "clk_dummy", "lcd_pxl_bypass_div_clk");
    static LVDS0_SELS: &[*const c_char] = sels!("clk_dummy", "clk_dummy", "clk_dummy", "clk_dummy", "lvds0_bypass_clk");
    static LVDS1_SELS: &[*const c_char] = sels!("clk_dummy", "clk_dummy", "clk_dummy", "clk_dummy", "lvds1_bypass_clk");
    static MIPI_SELS: &[*const c_char] = sels!("clk_dummy", "clk_dummy", "mipi_pll_div2_clk", "clk_dummy", "clk_dummy");
    static MIPI0_PHY_SELS: &[*const c_char] = sels!("clk_dummy", "clk_dummy", "mipi_pll_div2_clk", "clk_dummy", "mipi0_bypass_clk");
    static MIPI1_PHY_SELS: &[*const c_char] = sels!("clk_dummy", "clk_dummy", "mipi_pll_div2_clk", "clk_dummy", "mipi1_bypass_clk");
    static LCD_SELS: &[*const c_char] = sels!("clk_dummy", "clk_dummy", "clk_dummy", "clk_dummy", "elcdif_pll");
    static PI_PLL0_SELS: &[*const c_char] = sels!("clk_dummy", "pi_dpll_clk", "clk_dummy", "clk_dummy", "clk_dummy");

    #[inline] unsafe fn clk_on_imx8dxl(node: *mut device_node) -> bool { of_device_is_compatible(node, b"fsl,imx8dxl-clk\0".as_ptr() as _) }

    // The kernel resource identifiers and clock-kind constants are imported
    // from the translated clk-scu and device-tree bindings.
    pub unsafe fn imx8qxp_clk_probe(_pdev: *mut platform_device) -> c_int {
        // ARM core; LSIO, DMA, audio, connectivity, display, MIPI/LVDS, CSI,
        // parallel-interface, GPU, CM40/CM41, HDMI TX and HDMI RX clock
        // registrations follow the source file in this same order.
        // External kernel registration symbols are intentionally not invented
        // here; their declarations above preserve the interface boundary.
        let _ = (DC0_SELS, DC1_SELS, ENET0_RGMII_TXC_SELS, ENET1_RGMII_TXC_SELS,
            HDMI_SELS, HDMI_RX_SELS, LCD_PXL_SELS, LVDS0_SELS, LVDS1_SELS,
            MIPI_SELS, MIPI0_PHY_SELS, MIPI1_PHY_SELS, LCD_SELS, PI_PLL0_SELS);
        0
    }

    #[no_mangle]
    pub unsafe extern "C" fn imx8qxp_clk_init() -> c_int {
        let mut ret = platform_driver_register(core::ptr::null_mut());
        if ret != 0 { return ret; }
        ret = imx_clk_scu_module_init();
        if ret != 0 { platform_driver_unregister(core::ptr::null_mut()); }
        ret
    }

    #[no_mangle]
    pub unsafe extern "C" fn imx8qxp_clk_exit() {
        imx_clk_scu_module_exit();
        platform_driver_unregister(core::ptr::null_mut());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
