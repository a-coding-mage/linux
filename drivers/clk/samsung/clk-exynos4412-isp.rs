// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 Samsung Electronics Co., Ltd.
 * Author: Marek Szyprowski <m.szyprowski@samsung.com>
 *
 * Common Clock Framework support for Exynos4412 ISP module.
 */

// Dependencies supplied by the surrounding kernel clock framework:
// dt-bindings/clock/exynos4.h, linux/slab.h, linux/clk-provider.h,
// linux/of.h, linux/platform_device.h, linux/pm_runtime.h, and clk.h.

/* Exynos4x12 specific registers, which belong to ISP power domain */
const E4X12_DIV_ISP0: u32 = 0x0300;
const E4X12_DIV_ISP1: u32 = 0x0304;
const E4X12_GATE_ISP0: u32 = 0x0800;
const E4X12_GATE_ISP1: u32 = 0x0804;

/* NOTE: Must be equal to the last clock ID increased by one */
const CLKS_NR_ISP: usize = CLK_ISP_DIV_MCUISP1 as usize + 1;

/*
 * Support for CMU save/restore across system suspends
 */
static mut exynos4x12_save_isp: *mut samsung_clk_reg_dump = core::ptr::null_mut();

static exynos4x12_clk_isp_save: [c_ulong; 4] = [
    E4X12_DIV_ISP0 as c_ulong,
    E4X12_DIV_ISP1 as c_ulong,
    E4X12_GATE_ISP0 as c_ulong,
    E4X12_GATE_ISP1 as c_ulong,
];

static mut exynos4x12_isp_div_clks: [samsung_div_clock; 5] = [
    DIV!(CLK_ISP_DIV_ISP0, "div_isp0", "aclk200", E4X12_DIV_ISP0, 0, 3),
    DIV!(CLK_ISP_DIV_ISP1, "div_isp1", "aclk200", E4X12_DIV_ISP0, 4, 3),
    DIV!(CLK_ISP_DIV_MCUISP0, "div_mcuisp0", "aclk400_mcuisp", E4X12_DIV_ISP1, 4, 3),
    DIV!(CLK_ISP_DIV_MCUISP1, "div_mcuisp1", "div_mcuisp0", E4X12_DIV_ISP1, 8, 3),
    DIV!(0, "div_mpwm", "div_isp1", E4X12_DIV_ISP1, 0, 3),
];

static mut exynos4x12_isp_gate_clks: [samsung_gate_clock; 26] = [
    GATE!(CLK_ISP_FIMC_ISP, "isp", "aclk200", E4X12_GATE_ISP0, 0, 0, 0),
    GATE!(CLK_ISP_FIMC_DRC, "drc", "aclk200", E4X12_GATE_ISP0, 1, 0, 0),
    GATE!(CLK_ISP_FIMC_FD, "fd", "aclk200", E4X12_GATE_ISP0, 2, 0, 0),
    GATE!(CLK_ISP_FIMC_LITE0, "lite0", "aclk200", E4X12_GATE_ISP0, 3, 0, 0),
    GATE!(CLK_ISP_FIMC_LITE1, "lite1", "aclk200", E4X12_GATE_ISP0, 4, 0, 0),
    GATE!(CLK_ISP_MCUISP, "mcuisp", "aclk200", E4X12_GATE_ISP0, 5, 0, 0),
    GATE!(CLK_ISP_GICISP, "gicisp", "aclk200", E4X12_GATE_ISP0, 7, 0, 0),
    GATE!(CLK_ISP_SMMU_ISP, "smmu_isp", "aclk200", E4X12_GATE_ISP0, 8, 0, 0),
    GATE!(CLK_ISP_SMMU_DRC, "smmu_drc", "aclk200", E4X12_GATE_ISP0, 9, 0, 0),
    GATE!(CLK_ISP_SMMU_FD, "smmu_fd", "aclk200", E4X12_GATE_ISP0, 10, 0, 0),
    GATE!(CLK_ISP_SMMU_LITE0, "smmu_lite0", "aclk200", E4X12_GATE_ISP0, 11, 0, 0),
    GATE!(CLK_ISP_SMMU_LITE1, "smmu_lite1", "aclk200", E4X12_GATE_ISP0, 12, 0, 0),
    GATE!(CLK_ISP_PPMUISPMX, "ppmuispmx", "aclk200", E4X12_GATE_ISP0, 20, 0, 0),
    GATE!(CLK_ISP_PPMUISPX, "ppmuispx", "aclk200", E4X12_GATE_ISP0, 21, 0, 0),
    GATE!(CLK_ISP_MCUCTL_ISP, "mcuctl_isp", "aclk200", E4X12_GATE_ISP0, 23, 0, 0),
    GATE!(CLK_ISP_MPWM_ISP, "mpwm_isp", "aclk200", E4X12_GATE_ISP0, 24, 0, 0),
    GATE!(CLK_ISP_I2C0_ISP, "i2c0_isp", "aclk200", E4X12_GATE_ISP0, 25, 0, 0),
    GATE!(CLK_ISP_I2C1_ISP, "i2c1_isp", "aclk200", E4X12_GATE_ISP0, 26, 0, 0),
    GATE!(CLK_ISP_MTCADC_ISP, "mtcadc_isp", "aclk200", E4X12_GATE_ISP0, 27, 0, 0),
    GATE!(CLK_ISP_PWM_ISP, "pwm_isp", "aclk200", E4X12_GATE_ISP0, 28, 0, 0),
    GATE!(CLK_ISP_WDT_ISP, "wdt_isp", "aclk200", E4X12_GATE_ISP0, 30, 0, 0),
    GATE!(CLK_ISP_UART_ISP, "uart_isp", "aclk200", E4X12_GATE_ISP0, 31, 0, 0),
    GATE!(CLK_ISP_ASYNCAXIM, "asyncaxim", "aclk200", E4X12_GATE_ISP1, 0, 0, 0),
    GATE!(CLK_ISP_SMMU_ISPCX, "smmu_ispcx", "aclk200", E4X12_GATE_ISP1, 4, 0, 0),
    GATE!(CLK_ISP_SPI0_ISP, "spi0_isp", "aclk200", E4X12_GATE_ISP1, 12, 0, 0),
    GATE!(CLK_ISP_SPI1_ISP, "spi1_isp", "aclk200", E4X12_GATE_ISP1, 13, 0, 0),
];

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn samsung_clk_save(reg_base: *mut c_void, unused: *mut c_void,
                        dump: *mut samsung_clk_reg_dump, count: usize);
    fn samsung_clk_restore(reg_base: *mut c_void, unused: *mut c_void,
                           dump: *mut samsung_clk_reg_dump, count: usize);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut c_void;
    fn ptr_err(ptr: *mut c_void) -> i32;
    fn samsung_clk_alloc_reg_dump(regs: *const c_ulong, count: usize) -> *mut samsung_clk_reg_dump;
    fn samsung_clk_init(dev: *mut device, reg_base: *mut c_void, count: usize) -> *mut samsung_clk_provider;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut samsung_clk_provider);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_get_sync(dev: *mut device);
    fn samsung_clk_register_div(ctx: *mut samsung_clk_provider, clks: *mut samsung_div_clock, count: usize);
    fn samsung_clk_register_gate(ctx: *mut samsung_clk_provider, clks: *mut samsung_gate_clock, count: usize);
    fn samsung_clk_of_add_provider(np: *mut device_node, ctx: *mut samsung_clk_provider);
    fn pm_runtime_put(dev: *mut device);
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn pm_runtime_force_suspend(dev: *mut device) -> i32;
    fn pm_runtime_force_resume(dev: *mut device) -> i32;
}

unsafe extern "C" fn exynos4x12_isp_clk_suspend(dev: *mut device) -> i32 {
    let ctx = dev_get_drvdata(dev) as *mut samsung_clk_provider;
    samsung_clk_save((*ctx).reg_base, core::ptr::null_mut(), exynos4x12_save_isp,
                     exynos4x12_clk_isp_save.len());
    0
}

unsafe extern "C" fn exynos4x12_isp_clk_resume(dev: *mut device) -> i32 {
    let ctx = dev_get_drvdata(dev) as *mut samsung_clk_provider;
    samsung_clk_restore((*ctx).reg_base, core::ptr::null_mut(), exynos4x12_save_isp,
                        exynos4x12_clk_isp_save.len());
    0
}

unsafe extern "C" fn exynos4x12_isp_clk_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let np = dev.of_node;
    let reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (reg_base as usize) >= (-4095isize as usize) {
        return ptr_err(reg_base);
    }

    exynos4x12_save_isp = samsung_clk_alloc_reg_dump(
        exynos4x12_clk_isp_save.as_ptr(), exynos4x12_clk_isp_save.len());
    if exynos4x12_save_isp.is_null() {
        return -12;
    }

    let ctx = samsung_clk_init(dev, reg_base, CLKS_NR_ISP);
    platform_set_drvdata(pdev, ctx);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    pm_runtime_get_sync(dev);
    samsung_clk_register_div(ctx, exynos4x12_isp_div_clks.as_mut_ptr(), exynos4x12_isp_div_clks.len());
    samsung_clk_register_gate(ctx, exynos4x12_isp_gate_clks.as_mut_ptr(), exynos4x12_isp_gate_clks.len());
    samsung_clk_of_add_provider(np, ctx);
    pm_runtime_put(dev);
    0
}

static exynos4x12_isp_clk_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c_str!("samsung,exynos4412-isp-clock"), ..Default::default() },
    of_device_id::default(),
];

static exynos4x12_isp_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(exynos4x12_isp_clk_suspend),
    runtime_resume: Some(exynos4x12_isp_clk_resume),
    suspend: Some(pm_runtime_force_suspend),
    resume: Some(pm_runtime_force_resume),
    ..Default::default()
};

static mut exynos4x12_isp_clk_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c_str!("exynos4x12-isp-clk"),
        of_match_table: exynos4x12_isp_clk_of_match.as_ptr(),
        suppress_bind_attrs: true,
        pm: &exynos4x12_isp_pm_ops,
        ..Default::default()
    },
    probe: Some(exynos4x12_isp_clk_probe),
    ..Default::default()
};

unsafe extern "C" fn exynos4x12_isp_clk_init() -> i32 {
    platform_driver_register(&raw mut exynos4x12_isp_clk_driver)
}

// core_initcall(exynos4x12_isp_clk_init);
core_initcall!(exynos4x12_isp_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
