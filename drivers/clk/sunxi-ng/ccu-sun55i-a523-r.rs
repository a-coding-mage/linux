// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Arm Ltd.
 * Based on the D1 CCU driver:
 *   Copyright (c) 2020 huangzhenwei@allwinnertech.com
 *   Copyright (C) 2021 Samuel Holland <samuel@sholland.org>
 */

// Translated from the Linux kernel C implementation.  The included kernel
// types and CCU construction macros are supplied by the surrounding crate.

static R_AHB_APB_PARENTS: [clk_parent_data; 5] = [
    clk_parent_data { fw_name: "hosc" },
    clk_parent_data { fw_name: "losc" },
    clk_parent_data { fw_name: "iosc" },
    clk_parent_data { fw_name: "pll-periph" },
    clk_parent_data { fw_name: "pll-audio" },
];
SUNXI_CCU_M_DATA_WITH_MUX!(r_ahb_clk, "r-ahb", R_AHB_APB_PARENTS, 0x000, 0, 5, 24, 3, 0);
SUNXI_CCU_M_DATA_WITH_MUX!(r_apb0_clk, "r-apb0", R_AHB_APB_PARENTS, 0x00c, 0, 5, 24, 3, 0);
SUNXI_CCU_M_DATA_WITH_MUX!(r_apb1_clk, "r-apb1", R_AHB_APB_PARENTS, 0x010, 0, 5, 24, 3, 0);

SUNXI_CCU_MP_DATA_WITH_MUX_GATE!(r_cpu_timer0, "r-timer0", R_AHB_APB_PARENTS, 0x100, 0, 0, 1, 3, 4, 3, BIT!(0), 0);
SUNXI_CCU_MP_DATA_WITH_MUX_GATE!(r_cpu_timer1, "r-timer1", R_AHB_APB_PARENTS, 0x104, 0, 0, 1, 3, 4, 3, BIT!(0), 0);
SUNXI_CCU_MP_DATA_WITH_MUX_GATE!(r_cpu_timer2, "r-timer2", R_AHB_APB_PARENTS, 0x108, 0, 0, 1, 3, 4, 3, BIT!(0), 0);

SUNXI_CCU_GATE_HW!(bus_r_timer_clk, "bus-r-timer", &r_ahb_clk.common.hw, 0x11c, BIT!(0), 0);
SUNXI_CCU_GATE_HW!(bus_r_twd_clk, "bus-r-twd", &r_apb0_clk.common.hw, 0x12c, BIT!(0), 0);

static R_PWMCTRL_PARENTS: [clk_parent_data; 3] = [
    clk_parent_data { fw_name: "hosc" },
    clk_parent_data { fw_name: "losc" },
    clk_parent_data { fw_name: "iosc" },
];
SUNXI_CCU_MUX_DATA_WITH_GATE!(r_pwmctrl_clk, "r-pwmctrl", R_PWMCTRL_PARENTS, 0x130, 24, 2, BIT!(31), 0);
SUNXI_CCU_GATE_HW!(bus_r_pwmctrl_clk, "bus-r-pwmctrl", &r_apb0_clk.common.hw, 0x13c, BIT!(0), 0);

static R_SPI_PARENTS: [clk_parent_data; 5] = [
    clk_parent_data { fw_name: "hosc" },
    clk_parent_data { fw_name: "pll-periph" },
    clk_parent_data { name: "pll-periph0-300M" },
    clk_parent_data { name: "pll-periph1-300M" },
    clk_parent_data { fw_name: "pll-audio" },
];
SUNXI_CCU_DUALDIV_MUX_GATE!(r_spi_clk, "r-spi", R_SPI_PARENTS, 0x150, 0, 5, 8, 5, 24, 3, BIT!(31), 0);
SUNXI_CCU_GATE_HW!(bus_r_spi_clk, "bus-r-spi", &r_ahb_clk.common.hw, 0x15c, BIT!(0), 0);

SUNXI_CCU_GATE_HW!(bus_r_spinlock_clk, "bus-r-spinlock", &r_ahb_clk.common.hw, 0x16c, BIT!(0), 0);
SUNXI_CCU_GATE_HW!(bus_r_msgbox_clk, "bus-r-msgbox", &r_ahb_clk.common.hw, 0x17c, BIT!(0), 0);
SUNXI_CCU_GATE_HW!(bus_r_uart0_clk, "bus-r-uart0", &r_apb1_clk.common.hw, 0x18c, BIT!(0), 0);
SUNXI_CCU_GATE_HW!(bus_r_uart1_clk, "bus-r-uart1", &r_apb1_clk.common.hw, 0x18c, BIT!(1), 0);
SUNXI_CCU_GATE_HW!(bus_r_i2c0_clk, "bus-r-i2c0", &r_apb1_clk.common.hw, 0x19c, BIT!(0), 0);
SUNXI_CCU_GATE_HW!(bus_r_i2c1_clk, "bus-r-i2c1", &r_apb1_clk.common.hw, 0x19c, BIT!(1), 0);
SUNXI_CCU_GATE_HW!(bus_r_i2c2_clk, "bus-r-i2c2", &r_apb1_clk.common.hw, 0x19c, BIT!(2), 0);
SUNXI_CCU_GATE_HW!(bus_r_ppu0_clk, "bus-r-ppu0", &r_apb0_clk.common.hw, 0x1ac, BIT!(0), 0);
SUNXI_CCU_GATE_HW!(bus_r_ppu1_clk, "bus-r-ppu1", &r_apb0_clk.common.hw, 0x1ac, BIT!(1), 0);
SUNXI_CCU_GATE_HW!(bus_r_cpu_bist_clk, "bus-r-cpu-bist", &r_apb0_clk.common.hw, 0x1bc, BIT!(0), 0);

static R_IR_RX_PARENTS: [clk_parent_data; 2] = [
    clk_parent_data { fw_name: "losc" },
    clk_parent_data { fw_name: "hosc" },
];
SUNXI_CCU_M_DATA_WITH_MUX_GATE!(r_ir_rx_clk, "r-ir-rx", R_IR_RX_PARENTS, 0x1c0, 0, 5, 24, 2, BIT!(31), 0);
SUNXI_CCU_GATE_HW!(bus_r_ir_rx_clk, "bus-r-ir-rx", &r_apb0_clk.common.hw, 0x1cc, BIT!(0), 0);
SUNXI_CCU_GATE_HW!(bus_r_dma_clk, "bus-r-dma", &r_apb0_clk.common.hw, 0x1dc, BIT!(0), CLK_IS_CRITICAL);
SUNXI_CCU_GATE_HW!(bus_r_rtc_clk, "bus-r-rtc", &r_apb0_clk.common.hw, 0x20c, BIT!(0), 0);
SUNXI_CCU_GATE_HW!(bus_r_cpucfg_clk, "bus-r-cpucfg", &r_apb0_clk.common.hw, 0x22c, BIT!(0), CLK_IS_CRITICAL);

static mut SUN55I_A523_R_CCU_CLKS: [*mut ccu_common; 27] = [
    &raw mut r_ahb_clk.common, &raw mut r_apb0_clk.common, &raw mut r_apb1_clk.common,
    &raw mut r_cpu_timer0.common, &raw mut r_cpu_timer1.common, &raw mut r_cpu_timer2.common,
    &raw mut bus_r_timer_clk.common, &raw mut bus_r_twd_clk.common, &raw mut r_pwmctrl_clk.common,
    &raw mut bus_r_pwmctrl_clk.common, &raw mut r_spi_clk.common, &raw mut bus_r_spi_clk.common,
    &raw mut bus_r_spinlock_clk.common, &raw mut bus_r_msgbox_clk.common, &raw mut bus_r_uart0_clk.common,
    &raw mut bus_r_uart1_clk.common, &raw mut bus_r_i2c0_clk.common, &raw mut bus_r_i2c1_clk.common,
    &raw mut bus_r_i2c2_clk.common, &raw mut bus_r_ppu0_clk.common, &raw mut bus_r_ppu1_clk.common,
    &raw mut bus_r_cpu_bist_clk.common, &raw mut r_ir_rx_clk.common, &raw mut bus_r_ir_rx_clk.common,
    &raw mut bus_r_dma_clk.common, &raw mut bus_r_rtc_clk.common, &raw mut bus_r_cpucfg_clk.common,
];

static mut SUN55I_A523_R_CCU_RESETS: [ccu_reset_map; 16] = [
    [RST_BUS_R_TIMER] = ccu_reset_map { reg: 0x11c, bit: BIT!(16) },
    [RST_BUS_R_TWD] = ccu_reset_map { reg: 0x12c, bit: BIT!(16) },
    [RST_BUS_R_PWMCTRL] = ccu_reset_map { reg: 0x13c, bit: BIT!(16) },
    [RST_BUS_R_SPI] = ccu_reset_map { reg: 0x15c, bit: BIT!(16) },
    [RST_BUS_R_SPINLOCK] = ccu_reset_map { reg: 0x16c, bit: BIT!(16) },
    [RST_BUS_R_MSGBOX] = ccu_reset_map { reg: 0x17c, bit: BIT!(16) },
    [RST_BUS_R_UART0] = ccu_reset_map { reg: 0x18c, bit: BIT!(16) },
    [RST_BUS_R_UART1] = ccu_reset_map { reg: 0x18c, bit: BIT!(17) },
    [RST_BUS_R_I2C0] = ccu_reset_map { reg: 0x19c, bit: BIT!(16) },
    [RST_BUS_R_I2C1] = ccu_reset_map { reg: 0x19c, bit: BIT!(17) },
    [RST_BUS_R_I2C2] = ccu_reset_map { reg: 0x19c, bit: BIT!(18) },
    [RST_BUS_R_PPU1] = ccu_reset_map { reg: 0x1ac, bit: BIT!(17) },
    [RST_BUS_R_IR_RX] = ccu_reset_map { reg: 0x1cc, bit: BIT!(16) },
    [RST_BUS_R_RTC] = ccu_reset_map { reg: 0x20c, bit: BIT!(16) },
    [RST_BUS_R_CPUCFG] = ccu_reset_map { reg: 0x22c, bit: BIT!(16) },
    [RST_BUS_R_PPU0] = ccu_reset_map { reg: 0x1ac, bit: BIT!(16) },
];

static SUN55I_A523_R_CCU_DESC: sunxi_ccu_desc = sunxi_ccu_desc {
    ccu_clks: SUN55I_A523_R_CCU_CLKS.as_ptr(),
    num_ccu_clks: SUN55I_A523_R_CCU_CLKS.len(),
    hw_clks: &SUN55I_A523_R_HW_CLKS,
    resets: SUN55I_A523_R_CCU_RESETS.as_ptr(),
    num_resets: SUN55I_A523_R_CCU_RESETS.len(),
};

unsafe fn sun55i_a523_r_ccu_probe(pdev: *mut platform_device) -> i32 {
    let reg: *mut core::ffi::c_void = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR!(reg) { return PTR_ERR!(reg); }
    devm_sunxi_ccu_probe(&mut (*pdev).dev, reg, &SUN55I_A523_R_CCU_DESC)
}

static SUN55I_A523_R_CCU_IDS: [of_device_id; 2] = [
    of_device_id { compatible: "allwinner,sun55i-a523-r-ccu" },
    of_device_id::default(),
];
MODULE_DEVICE_TABLE!(of, SUN55I_A523_R_CCU_IDS);

static mut SUN55I_A523_R_CCU_DRIVER: platform_driver = platform_driver {
    probe: Some(sun55i_a523_r_ccu_probe),
    driver: driver { name: "sun55i-a523-r-ccu", suppress_bind_attrs: true, of_match_table: &SUN55I_A523_R_CCU_IDS },
};
module_platform_driver!(SUN55I_A523_R_CCU_DRIVER);
MODULE_IMPORT_NS!("SUNXI_CCU");
MODULE_DESCRIPTION!("Support for the Allwinner A523 PRCM CCU");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
