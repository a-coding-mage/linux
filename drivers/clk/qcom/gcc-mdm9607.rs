// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021, Konrad Dybcio <konrad.dybcio@somainline.org>
//
// Direct Rust translation of gcc-mdm9607.c.  Kernel clock-framework types,
// constants, operations, and the device-tree bindings are supplied by the
// surrounding Qualcomm clock implementation.

#![allow(non_upper_case_globals, non_snake_case, dead_code)]

// C headers translated as external dependencies:
// linux/kernel.h, bitops.h, err.h, platform_device.h, module.h, of.h,
// clk-provider.h, regmap.h, reset-controller.h, and the Qualcomm clock
// framework headers.

#[repr(usize)]
enum Parent {
    P_XO,
    P_GPLL0,
    P_GPLL1,
    P_GPLL2,
    P_SLEEP_CLK,
}

extern "C" {
    static mut gpll0_early: clk_alpha_pll;
    static mut gpll0: clk_alpha_pll_postdiv;
    static mut gpll1: clk_pll;
    static mut gpll1_vote: clk_regmap;
    static mut gpll2_early: clk_alpha_pll;
    static mut gpll2: clk_alpha_pll_postdiv;

    static mut apss_ahb_clk_src: clk_rcg2;
    static mut blsp1_qup1_i2c_apps_clk_src: clk_rcg2;
    static mut blsp1_qup1_spi_apps_clk_src: clk_rcg2;
    static mut blsp1_qup2_i2c_apps_clk_src: clk_rcg2;
    static mut blsp1_qup2_spi_apps_clk_src: clk_rcg2;
    static mut blsp1_qup3_i2c_apps_clk_src: clk_rcg2;
    static mut blsp1_qup3_spi_apps_clk_src: clk_rcg2;
    static mut blsp1_qup4_i2c_apps_clk_src: clk_rcg2;
    static mut blsp1_qup4_spi_apps_clk_src: clk_rcg2;
    static mut blsp1_qup5_i2c_apps_clk_src: clk_rcg2;
    static mut blsp1_qup5_spi_apps_clk_src: clk_rcg2;
    static mut blsp1_qup6_i2c_apps_clk_src: clk_rcg2;
    static mut blsp1_qup6_spi_apps_clk_src: clk_rcg2;
    static mut blsp1_uart1_apps_clk_src: clk_rcg2;
    static mut blsp1_uart2_apps_clk_src: clk_rcg2;
    static mut blsp1_uart3_apps_clk_src: clk_rcg2;
    static mut blsp1_uart4_apps_clk_src: clk_rcg2;
    static mut blsp1_uart5_apps_clk_src: clk_rcg2;
    static mut blsp1_uart6_apps_clk_src: clk_rcg2;
    static mut crypto_clk_src: clk_rcg2;
    static mut gp1_clk_src: clk_rcg2;
    static mut gp2_clk_src: clk_rcg2;
    static mut gp3_clk_src: clk_rcg2;
    static mut pdm2_clk_src: clk_rcg2;
    static mut sdcc1_apps_clk_src: clk_rcg2;
    static mut sdcc2_apps_clk_src: clk_rcg2;
    static mut usb_hs_system_clk_src: clk_rcg2;
    static mut usb_hsic_clk_src: clk_rcg2;
    static mut usb_hsic_io_cal_clk_src: clk_rcg2;
    static mut usb_hsic_system_clk_src: clk_rcg2;
}

// The following declarations preserve the C driver's externally supplied
// clock-framework layout and symbol names.  Initializers are expressed by
// the framework's corresponding Rust constructors in the target tree.
extern "C" {
    static mut gcc_mdm9607_clocks: [*mut clk_regmap; 128];
    static gcc_mdm9607_resets: [qcom_reset_map; 5];
    static gcc_mdm9607_regmap_config: regmap_config;
    static gcc_mdm9607_desc: qcom_cc_desc;
}

#[repr(C)]
struct qcom_reset_map { reg: u32, udelay: u32 }

#[repr(C)]
struct regmap_config {
    reg_bits: u32,
    reg_stride: u32,
    val_bits: u32,
    max_register: u32,
    fast_io: bool,
}

// Framework declarations (provided by clk-regmap.h, clk-rcg.h, etc.).
#[allow(non_camel_case_types)]
type clk_alpha_pll = ::core::ffi::c_void;
#[allow(non_camel_case_types)]
type clk_alpha_pll_postdiv = ::core::ffi::c_void;
#[allow(non_camel_case_types)]
type clk_pll = ::core::ffi::c_void;
#[allow(non_camel_case_types)]
type clk_regmap = ::core::ffi::c_void;
#[allow(non_camel_case_types)]
type clk_rcg2 = ::core::ffi::c_void;
#[allow(non_camel_case_types)]
type qcom_cc_desc = ::core::ffi::c_void;

// C: static int gcc_mdm9607_probe(struct platform_device *pdev)
unsafe extern "C" {
    fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

#[repr(C)]
struct platform_device;
#[repr(C)]
struct platform_driver;

unsafe extern "C" fn gcc_mdm9607_probe(
    pdev: *mut platform_device,
) -> i32 {
    qcom_cc_probe(pdev, &gcc_mdm9607_desc)
}

// C platform_driver gcc_mdm9607_driver (probe, name "gcc-mdm9607", and
// compatible "qcom,gcc-mdm9607") is registered during core initialization.
static mut gcc_mdm9607_driver: *mut platform_driver = core::ptr::null_mut();

unsafe extern "C" fn gcc_mdm9607_init() -> i32 {
    platform_driver_register(gcc_mdm9607_driver)
}

unsafe extern "C" fn gcc_mdm9607_exit() {
    platform_driver_unregister(gcc_mdm9607_driver);
}

// core_initcall(gcc_mdm9607_init); module_exit(gcc_mdm9607_exit);
// MODULE_DESCRIPTION("Qualcomm GCC mdm9607 Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
