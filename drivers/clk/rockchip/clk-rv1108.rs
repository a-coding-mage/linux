// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Literal Rust translation of clk-rv1108.c.  Kernel-provided types, constants,
 * and clock-construction macros are intentionally left as external items.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

// C dependencies: linux/clk-provider.h, linux/io.h, linux/of.h,
// linux/of_address.h, linux/syscore_ops.h, dt-bindings/clock/rv1108-cru.h,
// and the local clk.h provide the following external kernel symbols.

const RV1108_GRF_SOC_STATUS0: u32 = 0x480;

#[repr(C)]
#[derive(Copy, Clone)]
enum rv1108_plls { apll, dpll, gpll }

static mut rv1108_pll_rates: [rockchip_pll_rate_table; 42] = [
    RK3036_PLL_RATE!(1608000000,1,67,1,1,1,0), RK3036_PLL_RATE!(1584000000,1,66,1,1,1,0),
    RK3036_PLL_RATE!(1560000000,1,65,1,1,1,0), RK3036_PLL_RATE!(1536000000,1,64,1,1,1,0),
    RK3036_PLL_RATE!(1512000000,1,63,1,1,1,0), RK3036_PLL_RATE!(1488000000,1,62,1,1,1,0),
    RK3036_PLL_RATE!(1464000000,1,61,1,1,1,0), RK3036_PLL_RATE!(1440000000,1,60,1,1,1,0),
    RK3036_PLL_RATE!(1416000000,1,59,1,1,1,0), RK3036_PLL_RATE!(1392000000,1,58,1,1,1,0),
    RK3036_PLL_RATE!(1368000000,1,57,1,1,1,0), RK3036_PLL_RATE!(1344000000,1,56,1,1,1,0),
    RK3036_PLL_RATE!(1320000000,1,55,1,1,1,0), RK3036_PLL_RATE!(1296000000,1,54,1,1,1,0),
    RK3036_PLL_RATE!(1272000000,1,53,1,1,1,0), RK3036_PLL_RATE!(1248000000,1,52,1,1,1,0),
    RK3036_PLL_RATE!(1200000000,1,50,1,1,1,0), RK3036_PLL_RATE!(1188000000,2,99,1,1,1,0),
    RK3036_PLL_RATE!(1104000000,1,46,1,1,1,0), RK3036_PLL_RATE!(1100000000,12,550,1,1,1,0),
    RK3036_PLL_RATE!(1008000000,1,84,2,1,1,0), RK3036_PLL_RATE!(1000000000,6,500,2,1,1,0),
    RK3036_PLL_RATE!(984000000,1,82,2,1,1,0), RK3036_PLL_RATE!(960000000,1,80,2,1,1,0),
    RK3036_PLL_RATE!(936000000,1,78,2,1,1,0), RK3036_PLL_RATE!(912000000,1,76,2,1,1,0),
    RK3036_PLL_RATE!(900000000,4,300,2,1,1,0), RK3036_PLL_RATE!(888000000,1,74,2,1,1,0),
    RK3036_PLL_RATE!(864000000,1,72,2,1,1,0), RK3036_PLL_RATE!(840000000,1,70,2,1,1,0),
    RK3036_PLL_RATE!(816000000,1,68,2,1,1,0), RK3036_PLL_RATE!(800000000,6,400,2,1,1,0),
    RK3036_PLL_RATE!(700000000,6,350,2,1,1,0), RK3036_PLL_RATE!(696000000,1,58,2,1,1,0),
    RK3036_PLL_RATE!(600000000,1,75,3,1,1,0), RK3036_PLL_RATE!(594000000,2,99,2,1,1,0),
    RK3036_PLL_RATE!(504000000,1,63,3,1,1,0), RK3036_PLL_RATE!(500000000,6,250,2,1,1,0),
    RK3036_PLL_RATE!(408000000,1,68,2,2,1,0), RK3036_PLL_RATE!(312000000,1,52,2,2,1,0),
    RK3036_PLL_RATE!(216000000,1,72,4,2,1,0), RK3036_PLL_RATE!(96000000,1,64,4,4,1,0),
    ROCKCHIP_PLL_RATE_SENTINEL!(),
];

const RV1108_DIV_CORE_MASK: u32 = 0xf;
const RV1108_DIV_CORE_SHIFT: u32 = 4;

// The remaining clock tables are direct invocations of the kernel's C clock
// description macros, represented by their Rust macro equivalents.
static mut rv1108_cpuclk_rates: &[rockchip_cpuclk_rate_table] = &[
    RV1108_CPUCLK_RATE!(1608000000,7), RV1108_CPUCLK_RATE!(1512000000,7),
    RV1108_CPUCLK_RATE!(1488000000,5), RV1108_CPUCLK_RATE!(1416000000,5),
    RV1108_CPUCLK_RATE!(1392000000,5), RV1108_CPUCLK_RATE!(1296000000,5),
    RV1108_CPUCLK_RATE!(1200000000,5), RV1108_CPUCLK_RATE!(1104000000,5),
    RV1108_CPUCLK_RATE!(1008000000,5), RV1108_CPUCLK_RATE!(912000000,5),
    RV1108_CPUCLK_RATE!(816000000,3), RV1108_CPUCLK_RATE!(696000000,3),
    RV1108_CPUCLK_RATE!(600000000,3), RV1108_CPUCLK_RATE!(500000000,3),
    RV1108_CPUCLK_RATE!(408000000,1), RV1108_CPUCLK_RATE!(312000000,1),
    RV1108_CPUCLK_RATE!(216000000,1), RV1108_CPUCLK_RATE!(96000000,1),
];

static rv1108_cpuclk_data: rockchip_cpuclk_reg_data = rockchip_cpuclk_reg_data {
    core_reg: [RV1108_CLKSEL_CON!(0)], div_core_shift: [0], div_core_mask: [0x1f],
    num_cores: 1, mux_core_alt: 1, mux_core_main: 0, mux_core_shift: 8,
    mux_core_mask: 0x3,
};

// Parent-name arrays and the complete branch descriptions retain the source
// order and are supplied by the platform clock macro layer.
PNAME!(mux_pll_p, "xin24m", "xin24m");
PNAME!(mux_ddrphy_p, "dpll_ddr", "gpll_ddr", "apll_ddr");
PNAME!(mux_armclk_p, "apll_core", "gpll_core", "dpll_core");
PNAME!(mux_usb480m_pre_p, "usbphy", "xin24m");
PNAME!(mux_hdmiphy_phy_p, "hdmiphy", "xin24m");
PNAME!(mux_dclk_hdmiphy_pre_p, "dclk_hdmiphy_src_gpll", "dclk_hdmiphy_src_dpll");

extern "C" {
    fn rv1108_clk_init(np: *mut device_node);
}

// CLK_OF_DECLARE(rv1108_cru, "rockchip,rv1108-cru", rv1108_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
