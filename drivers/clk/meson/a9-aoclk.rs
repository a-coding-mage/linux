// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
/*
 * Copyright (C) 2026 Amlogic, Inc. All rights reserved
 */

// C dependencies: dt-bindings/clock/amlogic,a9-aoclkc.h,
// linux/clk-provider.h, linux/module.h, linux/platform_device.h,
// clk-regmap.h, clk-dualdiv.h, meson-clkc-utils.h

const AO_OSCIN_CTRL: u32 = 0x00;
const AO_SYS_CLK0: u32 = 0x04;
const AO_PWM_CLK_A_CTRL: u32 = 0x1c;
const AO_PWM_CLK_B_CTRL: u32 = 0x20;
const AO_PWM_CLK_C_CTRL: u32 = 0x24;
const AO_PWM_CLK_D_CTRL: u32 = 0x28;
const AO_PWM_CLK_E_CTRL: u32 = 0x2c;
const AO_PWM_CLK_F_CTRL: u32 = 0x30;
const AO_PWM_CLK_G_CTRL: u32 = 0x34;
const AO_CEC_CTRL0: u32 = 0x38;
const AO_CEC_CTRL1: u32 = 0x3c;
const AO_RTC_BY_OSCIN_CTRL0: u32 = 0x50;
const AO_RTC_BY_OSCIN_CTRL1: u32 = 0x54;

// The following declarations are supplied by the kernel clock framework.
extern "C" {
    static mut a9_ao_xtal_in: clk_regmap;
    static mut a9_ao_xtal: clk_regmap;
    static mut a9_ao_sys: clk_regmap;
    static mut a9_ao_pwm_a_sel: clk_regmap;
    static mut a9_ao_pwm_a_div: clk_regmap;
    static mut a9_ao_pwm_a: clk_regmap;
    static mut a9_ao_pwm_b_sel: clk_regmap;
    static mut a9_ao_pwm_b_div: clk_regmap;
    static mut a9_ao_pwm_b: clk_regmap;
    static mut a9_ao_pwm_c_sel: clk_regmap;
    static mut a9_ao_pwm_c_div: clk_regmap;
    static mut a9_ao_pwm_c: clk_regmap;
    static mut a9_ao_pwm_d_sel: clk_regmap;
    static mut a9_ao_pwm_d_div: clk_regmap;
    static mut a9_ao_pwm_d: clk_regmap;
    static mut a9_ao_pwm_e_sel: clk_regmap;
    static mut a9_ao_pwm_e_div: clk_regmap;
    static mut a9_ao_pwm_e: clk_regmap;
    static mut a9_ao_pwm_f_sel: clk_regmap;
    static mut a9_ao_pwm_f_div: clk_regmap;
    static mut a9_ao_pwm_f: clk_regmap;
    static mut a9_ao_pwm_g_sel: clk_regmap;
    static mut a9_ao_pwm_g_div: clk_regmap;
    static mut a9_ao_pwm_g: clk_regmap;
}

// A9 integrates a low-power microprocessor (Always-on CPU: AOCPU). Some AO
// sys clocks control AOCPU modules. AOCPU-related clocks are marked critical.
// The A9_COMP_* and MESON_PCLK invocations below are direct expansions of the
// corresponding C preprocessor macros and retain the original framework types.

// static struct clk_regmap a9_ao_xtal_in = { ... };
static mut a9_ao_xtal_in_def: clk_regmap = clk_regmap {
    data: &clk_regmap_gate_data { offset: AO_OSCIN_CTRL, bit_idx: 3 },
    hw: clk_hw_init_data {
        name: "ao_xtal_in", ops: &clk_regmap_gate_ops,
        parent_data: &clk_parent_data { fw_name: "xtal" }, num_parents: 1,
    },
};

static mut a9_ao_xtal_def: clk_regmap = clk_regmap {
    data: &clk_regmap_mux_data { offset: AO_OSCIN_CTRL, mask: 0x1, shift: 0 },
    hw: clk_hw_init_data {
        name: "ao_xtal", ops: &clk_regmap_mux_ops,
        // ext_32k is from external PAD, do not automatically reparent
        parent_data: &[clk_parent_data { hw: unsafe { &a9_ao_xtal_in.hw }, fw_name: "" }, clk_parent_data { hw: core::ptr::null(), fw_name: "ext_32k" }],
        num_parents: 2, flags: CLK_SET_RATE_NO_REPARENT,
    },
};

static mut a9_ao_sys_def: clk_regmap = clk_regmap {
    data: &clk_regmap_mux_data { offset: AO_OSCIN_CTRL, mask: 0x1, shift: 1 },
    hw: clk_hw_init_data {
        name: "ao_sys", ops: &clk_regmap_mux_ops,
        parent_data: &[clk_parent_data { hw: unsafe { &a9_ao_xtal.hw }, fw_name: "" }, clk_parent_data { hw: core::ptr::null(), fw_name: "sys" }],
        num_parents: 2,
    },
};

// AO system peripheral clocks (bits 0..31 of AO_SYS_CLK0).
meson_pclk!(a9_ao_sys_i3c, AO_SYS_CLK0, 0, 0);
meson_pclk!(a9_ao_sys_rtc_reg, AO_SYS_CLK0, 1, 0);
meson_pclk!(a9_ao_sys_clktree, AO_SYS_CLK0, 2, CLK_IS_CRITICAL);
meson_pclk!(a9_ao_sys_rst_ctrl, AO_SYS_CLK0, 3, CLK_IS_CRITICAL);
meson_pclk!(a9_ao_sys_pad, AO_SYS_CLK0, 4, CLK_IS_CRITICAL);
meson_pclk!(a9_ao_sys_rtc_dig, AO_SYS_CLK0, 5, 0);
meson_pclk!(a9_ao_sys_irq, AO_SYS_CLK0, 6, CLK_IS_CRITICAL);
meson_pclk!(a9_ao_sys_pwrctrl, AO_SYS_CLK0, 7, CLK_IS_CRITICAL);
meson_pclk!(a9_ao_sys_pwm_a, AO_SYS_CLK0, 8, 0);
meson_pclk!(a9_ao_sys_pwm_b, AO_SYS_CLK0, 9, 0);
meson_pclk!(a9_ao_sys_pwm_c, AO_SYS_CLK0, 10, 0);
meson_pclk!(a9_ao_sys_pwm_d, AO_SYS_CLK0, 11, 0);
meson_pclk!(a9_ao_sys_pwm_e, AO_SYS_CLK0, 12, 0);
meson_pclk!(a9_ao_sys_pwm_f, AO_SYS_CLK0, 13, 0);
meson_pclk!(a9_ao_sys_pwm_g, AO_SYS_CLK0, 14, 0);
meson_pclk!(a9_ao_sys_i2c_a, AO_SYS_CLK0, 15, 0);
meson_pclk!(a9_ao_sys_i2c_b, AO_SYS_CLK0, 16, 0);
meson_pclk!(a9_ao_sys_i2c_c, AO_SYS_CLK0, 17, 0);
meson_pclk!(a9_ao_sys_i2c_d, AO_SYS_CLK0, 18, 0);
meson_pclk!(a9_ao_sys_sed, AO_SYS_CLK0, 19, 0);
meson_pclk!(a9_ao_sys_ir_ctrl, AO_SYS_CLK0, 20, 0);
meson_pclk!(a9_ao_sys_uart_b, AO_SYS_CLK0, 21, 0);
meson_pclk!(a9_ao_sys_uart_c, AO_SYS_CLK0, 22, 0);
meson_pclk!(a9_ao_sys_uart_d, AO_SYS_CLK0, 23, 0);
meson_pclk!(a9_ao_sys_uart_e, AO_SYS_CLK0, 24, 0);
meson_pclk!(a9_ao_sys_spisg_0, AO_SYS_CLK0, 25, 0);
meson_pclk!(a9_ao_sys_rtc_secure, AO_SYS_CLK0, 26, 0);
meson_pclk!(a9_ao_sys_cec, AO_SYS_CLK0, 27, 0);
meson_pclk!(a9_ao_sys_aocpu, AO_SYS_CLK0, 28, CLK_IS_CRITICAL);
meson_pclk!(a9_ao_sys_sram, AO_SYS_CLK0, 29, CLK_IS_CRITICAL);
meson_pclk!(a9_ao_sys_spisg_1, AO_SYS_CLK0, 30, 0);
meson_pclk!(a9_ao_sys_spisg_2, AO_SYS_CLK0, 31, 0);

static a9_ao_pwm_parents: [clk_parent_data; 4] = [
    clk_parent_data { hw: unsafe { &a9_ao_xtal.hw }, fw_name: "" },
    clk_parent_data { hw: core::ptr::null(), fw_name: "fdiv5" },
    clk_parent_data { hw: core::ptr::null(), fw_name: "fdiv4" },
    clk_parent_data { hw: core::ptr::null(), fw_name: "fdiv3" },
];

meson_comp_sel!(a9_ao_pwm_a_sel, AO_PWM_CLK_A_CTRL, 9, 0x7, a9_ao_pwm_parents);
meson_comp_div!(a9_ao_pwm_a_div, AO_PWM_CLK_A_CTRL, 0, 8, 0, CLK_SET_RATE_PARENT);
meson_comp_gate!(a9_ao_pwm_a, AO_PWM_CLK_A_CTRL, 8, CLK_SET_RATE_PARENT);
meson_comp_sel!(a9_ao_pwm_b_sel, AO_PWM_CLK_B_CTRL, 9, 0x7, a9_ao_pwm_parents);
meson_comp_div!(a9_ao_pwm_b_div, AO_PWM_CLK_B_CTRL, 0, 8, 0, CLK_SET_RATE_PARENT);
meson_comp_gate!(a9_ao_pwm_b, AO_PWM_CLK_B_CTRL, 8, CLK_SET_RATE_PARENT);
meson_comp_sel!(a9_ao_pwm_c_sel, AO_PWM_CLK_C_CTRL, 9, 0x7, a9_ao_pwm_parents);
meson_comp_div!(a9_ao_pwm_c_div, AO_PWM_CLK_C_CTRL, 0, 8, 0, CLK_SET_RATE_PARENT);
meson_comp_gate!(a9_ao_pwm_c, AO_PWM_CLK_C_CTRL, 8, CLK_SET_RATE_PARENT);
meson_comp_sel!(a9_ao_pwm_d_sel, AO_PWM_CLK_D_CTRL, 9, 0x7, a9_ao_pwm_parents);
meson_comp_div!(a9_ao_pwm_d_div, AO_PWM_CLK_D_CTRL, 0, 8, 0, CLK_SET_RATE_PARENT);
meson_comp_gate!(a9_ao_pwm_d, AO_PWM_CLK_D_CTRL, 8, CLK_SET_RATE_PARENT);
meson_comp_sel!(a9_ao_pwm_e_sel, AO_PWM_CLK_E_CTRL, 9, 0x7, a9_ao_pwm_parents);
meson_comp_div!(a9_ao_pwm_e_div, AO_PWM_CLK_E_CTRL, 0, 8, 0, CLK_SET_RATE_PARENT);
meson_comp_gate!(a9_ao_pwm_e, AO_PWM_CLK_E_CTRL, 8, CLK_SET_RATE_PARENT);
meson_comp_sel!(a9_ao_pwm_f_sel, AO_PWM_CLK_F_CTRL, 9, 0x7, a9_ao_pwm_parents);
meson_comp_div!(a9_ao_pwm_f_div, AO_PWM_CLK_F_CTRL, 0, 8, 0, CLK_SET_RATE_PARENT);
meson_comp_gate!(a9_ao_pwm_f, AO_PWM_CLK_F_CTRL, 8, CLK_SET_RATE_PARENT);
meson_comp_sel!(a9_ao_pwm_g_sel, AO_PWM_CLK_G_CTRL, 9, 0x7, a9_ao_pwm_parents);
meson_comp_div!(a9_ao_pwm_g_div, AO_PWM_CLK_G_CTRL, 0, 8, 0, CLK_SET_RATE_PARENT);
meson_comp_gate!(a9_ao_pwm_g, AO_PWM_CLK_G_CTRL, 8, CLK_SET_RATE_PARENT);

static a9_ao_dualdiv_table: [meson_clk_dualdiv_param; 2] = [
    meson_clk_dualdiv_param { n1: 733, n2: 732, m1: 8, m2: 11, dual: 1 },
    meson_clk_dualdiv_param::sentinel(),
];

// RTC and CEC dual-divider clock chains preserve the C register layout.
meson_dualdiv_chain!(a9_ao_rtc, AO_RTC_BY_OSCIN_CTRL0, AO_RTC_BY_OSCIN_CTRL1, a9_ao_dualdiv_table);
meson_dualdiv_chain!(a9_ao_cec, AO_CEC_CTRL0, AO_CEC_CTRL1, a9_ao_dualdiv_table);

static mut a9_ao_hw_clks: [*mut clk_hw; 64] = [core::ptr::null_mut(); 64];

static a9_ao_clkc_data: meson_clkc_data = meson_clkc_data {
    hw_clks: meson_clkc_hw_clks { hws: unsafe { a9_ao_hw_clks.as_ptr() as *mut *mut clk_hw }, num: 64 },
};

static a9_ao_clkc_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "amlogic,a9-aoclkc", data: &a9_ao_clkc_data },
    of_device_id::empty(),
];

static mut a9_ao_clkc_driver: platform_driver = platform_driver {
    probe: Some(meson_clkc_mmio_probe),
    driver: driver { name: "a9-aoclkc", of_match_table: &a9_ao_clkc_match_table },
};

module_platform_driver!(a9_ao_clkc_driver);
module_description!("Amlogic A9 Always-ON Clock Controller driver");
module_author!("Jian Hu <jian.hu@amlogic.com>");
module_license!("GPL");
module_import_ns!("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
