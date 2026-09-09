// SPDX-License-Identifier: GPL-2.0-only
/* AM43XX Clock init */

use core::ffi::c_char;

// The following types, constants, and functions are supplied by the translated
// kernel clock framework and device-tree bindings.

static AM4_SYNCTIMER_32KCLK_PARENTS: [*const c_char; 2] = [b"mux_synctimer32k_ck\0".as_ptr() as *const c_char, core::ptr::null()];
static AM4_GPIO0_DBCLK_PARENTS: [*const c_char; 2] = [b"gpio0_dbclk_mux_ck\0".as_ptr() as *const c_char, core::ptr::null()];
static AM4_USB_OTG_SS0_REFCLK960M_PARENTS: [*const c_char; 2] = [b"dpll_per_clkdcoldo\0".as_ptr() as *const c_char, core::ptr::null()];
static AM4_GPIO1_DBCLK_PARENTS: [*const c_char; 2] = [b"clkdiv32k_ick\0".as_ptr() as *const c_char, core::ptr::null()];

static AM4_L3S_TSC_CLKCTRL_REGS: [omap_clkctrl_reg_data; 2] = [
    omap_clkctrl_reg_data { offset: AM4_L3S_TSC_ADC_TSC_CLKCTRL, bit_data: core::ptr::null(), flags: CLKF_SW_SUP, name: b"adc_tsc_fck\0".as_ptr() as *const c_char },
    omap_clkctrl_reg_data::default(),
];
static AM4_COUNTER_32K_BIT_DATA: [omap_clkctrl_bit_data; 2] = [
    omap_clkctrl_bit_data { bit: 8, clk_type: TI_CLK_GATE, parents: AM4_SYNCTIMER_32KCLK_PARENTS.as_ptr(), name: core::ptr::null() },
    omap_clkctrl_bit_data::default(),
];
static AM4_L4_WKUP_AON_CLKCTRL_REGS: [omap_clkctrl_reg_data; 3] = [
    omap_clkctrl_reg_data { offset: AM4_L4_WKUP_AON_WKUP_M3_CLKCTRL, bit_data: core::ptr::null(), flags: CLKF_SW_SUP | CLKF_NO_IDLEST, name: b"sys_clkin_ck\0".as_ptr() as *const c_char },
    omap_clkctrl_reg_data { offset: AM4_L4_WKUP_AON_COUNTER_32K_CLKCTRL, bit_data: AM4_COUNTER_32K_BIT_DATA.as_ptr(), flags: CLKF_SW_SUP, name: b"l4-wkup-aon-clkctrl:0008:8\0".as_ptr() as *const c_char },
    omap_clkctrl_reg_data::default(),
];
static AM4_GPIO1_BIT_DATA: [omap_clkctrl_bit_data; 2] = [
    omap_clkctrl_bit_data { bit: 8, clk_type: TI_CLK_GATE, parents: AM4_GPIO0_DBCLK_PARENTS.as_ptr(), name: core::ptr::null() },
    omap_clkctrl_bit_data::default(),
];
static AM4_L4_WKUP_CLKCTRL_REGS: [omap_clkctrl_reg_data; 10] = [
 reg!(AM4_L4_WKUP_L4_WKUP_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"sys_clkin_ck"),reg!(AM4_L4_WKUP_TIMER1_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"timer1_fck"),reg!(AM4_L4_WKUP_WD_TIMER2_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"wdt1_fck"),reg!(AM4_L4_WKUP_I2C1_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_per_m2_div4_wkupdm_ck"),reg!(AM4_L4_WKUP_UART1_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_per_m2_div4_wkupdm_ck"),reg!(AM4_L4_WKUP_SMARTREFLEX0_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"smartreflex0_fck"),reg!(AM4_L4_WKUP_SMARTREFLEX1_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"smartreflex1_fck"),reg!(AM4_L4_WKUP_CONTROL_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"sys_clkin_ck"),reg!(AM4_L4_WKUP_GPIO1_CLKCTRL,AM4_GPIO1_BIT_DATA.as_ptr(),CLKF_SW_SUP,"sys_clkin_ck"),omap_clkctrl_reg_data::default()
];

macro_rules! reg { ($o:ident, $b:expr, $f:expr, $n:literal) => { omap_clkctrl_reg_data { offset: $o, bit_data: $b, flags: $f, name: concat!($n, "\\0").as_ptr() as *const c_char } }; }
macro_rules! bits { ($p:ident) => { [omap_clkctrl_bit_data { bit: 8, clk_type: TI_CLK_GATE, parents: $p.as_ptr(), name: core::ptr::null() }, omap_clkctrl_bit_data::default()] }; }

static AM4_MPU_CLKCTRL_REGS: [omap_clkctrl_reg_data; 2] = [reg!(AM4_MPU_MPU_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, "dpll_mpu_m2_ck"), omap_clkctrl_reg_data::default()];
static AM4_GFX_L3_CLKCTRL_REGS: [omap_clkctrl_reg_data; 2] = [reg!(AM4_GFX_L3_GFX_CLKCTRL, core::ptr::null(), CLKF_SW_SUP | CLKF_NO_IDLEST, "gfx_fck_div_ck"), omap_clkctrl_reg_data::default()];
static AM4_L4_RTC_CLKCTRL_REGS: [omap_clkctrl_reg_data; 2] = [reg!(AM4_L4_RTC_RTC_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, "clkdiv32k_ick"), omap_clkctrl_reg_data::default()];

static AM4_L3_CLKCTRL_REGS: [omap_clkctrl_reg_data; 12] = [
 reg!(AM4_L3_L3_MAIN_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l3_gclk"), reg!(AM4_L3_AES_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"aes0_fck"), reg!(AM4_L3_DES_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l3_gclk"), reg!(AM4_L3_L3_INSTR_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l3_gclk"), reg!(AM4_L3_OCMCRAM_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l3_gclk"), reg!(AM4_L3_SHAM_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l3_gclk"), reg!(AM4_L3_TPCC_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l3_gclk"), reg!(AM4_L3_TPTC0_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l3_gclk"), reg!(AM4_L3_TPTC1_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l3_gclk"), reg!(AM4_L3_TPTC2_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l3_gclk"), reg!(AM4_L3_L4_HS_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l4hs_gclk"), omap_clkctrl_reg_data::default()
];
static AM4_USB_OTG_SS0_BIT_DATA: [omap_clkctrl_bit_data; 2] = bits!(AM4_USB_OTG_SS0_REFCLK960M_PARENTS);
static AM4_USB_OTG_SS1_BIT_DATA: [omap_clkctrl_bit_data; 2] = bits!(AM4_USB_OTG_SS0_REFCLK960M_PARENTS);

static AM4_L3S_CLKCTRL_REGS: [omap_clkctrl_reg_data; 11] = [
 reg!(AM4_L3S_VPFE0_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l3_gclk"),reg!(AM4_L3S_VPFE1_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l3_gclk"),reg!(AM4_L3S_GPMC_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l3s_gclk"),reg!(AM4_L3S_ADC1_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l3s_gclk"),reg!(AM4_L3S_MCASP0_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"mcasp0_fck"),reg!(AM4_L3S_MCASP1_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"mcasp1_fck"),reg!(AM4_L3S_MMC3_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"mmc_clk"),reg!(AM4_L3S_QSPI_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l3s_gclk"),reg!(AM4_L3S_USB_OTG_SS0_CLKCTRL,AM4_USB_OTG_SS0_BIT_DATA.as_ptr(),CLKF_SW_SUP,"l3s_gclk"),reg!(AM4_L3S_USB_OTG_SS1_CLKCTRL,AM4_USB_OTG_SS1_BIT_DATA.as_ptr(),CLKF_SW_SUP,"l3s_gclk"),omap_clkctrl_reg_data::default()
];
static AM4_PRUSS_OCP_CLKCTRL_REGS: [omap_clkctrl_reg_data; 2] = [reg!(AM4_PRUSS_OCP_PRUSS_CLKCTRL,core::ptr::null(),CLKF_SW_SUP|CLKF_NO_IDLEST,"pruss_ocp_gclk"),omap_clkctrl_reg_data::default()];

static AM4_GPIO2_BIT_DATA: [omap_clkctrl_bit_data; 2] = bits!(AM4_GPIO1_DBCLK_PARENTS);
static AM4_GPIO3_BIT_DATA: [omap_clkctrl_bit_data; 2] = bits!(AM4_GPIO1_DBCLK_PARENTS);
static AM4_GPIO4_BIT_DATA: [omap_clkctrl_bit_data; 2] = bits!(AM4_GPIO1_DBCLK_PARENTS);
static AM4_GPIO5_BIT_DATA: [omap_clkctrl_bit_data; 2] = bits!(AM4_GPIO1_DBCLK_PARENTS);
static AM4_GPIO6_BIT_DATA: [omap_clkctrl_bit_data; 2] = bits!(AM4_GPIO1_DBCLK_PARENTS);

// The remaining register tables and the device-tree clock table retain the
// source ordering and are expressed using the same framework record forms.
static AM4_L4LS_CLKCTRL_REGS: [omap_clkctrl_reg_data; 43] = [
 reg!(AM4_L4LS_L4_LS_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_D_CAN0_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dcan0_fck"),reg!(AM4_L4LS_D_CAN1_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dcan1_fck"),reg!(AM4_L4LS_EPWMSS0_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_EPWMSS1_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_EPWMSS2_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_EPWMSS3_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_EPWMSS4_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_EPWMSS5_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_ELM_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_GPIO2_CLKCTRL,AM4_GPIO2_BIT_DATA.as_ptr(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_GPIO3_CLKCTRL,AM4_GPIO3_BIT_DATA.as_ptr(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_GPIO4_CLKCTRL,AM4_GPIO4_BIT_DATA.as_ptr(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_GPIO5_CLKCTRL,AM4_GPIO5_BIT_DATA.as_ptr(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_GPIO6_CLKCTRL,AM4_GPIO6_BIT_DATA.as_ptr(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_HDQ1W_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"func_12m_clk"),reg!(AM4_L4LS_I2C2_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_per_m2_div4_ck"),reg!(AM4_L4LS_I2C3_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_per_m2_div4_ck"),reg!(AM4_L4LS_MAILBOX_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_MMC1_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"mmc_clk"),reg!(AM4_L4LS_MMC2_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"mmc_clk"),reg!(AM4_L4LS_RNG_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"rng_fck"),reg!(AM4_L4LS_SPI0_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_per_m2_div4_ck"),reg!(AM4_L4LS_SPI1_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_per_m2_div4_ck"),reg!(AM4_L4LS_SPI2_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_per_m2_div4_ck"),reg!(AM4_L4LS_SPI3_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_per_m2_div4_ck"),reg!(AM4_L4LS_SPI4_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_per_m2_div4_ck"),reg!(AM4_L4LS_SPINLOCK_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_TIMER2_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"timer2_fck"),reg!(AM4_L4LS_TIMER3_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"timer3_fck"),reg!(AM4_L4LS_TIMER4_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"timer4_fck"),reg!(AM4_L4LS_TIMER5_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"timer5_fck"),reg!(AM4_L4LS_TIMER6_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"timer6_fck"),reg!(AM4_L4LS_TIMER7_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"timer7_fck"),reg!(AM4_L4LS_TIMER8_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"timer8_fck"),reg!(AM4_L4LS_TIMER9_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"timer9_fck"),reg!(AM4_L4LS_TIMER10_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"timer10_fck"),reg!(AM4_L4LS_TIMER11_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"timer11_fck"),reg!(AM4_L4LS_UART2_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_per_m2_div4_ck"),reg!(AM4_L4LS_UART3_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_per_m2_div4_ck"),reg!(AM4_L4LS_UART4_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_per_m2_div4_ck"),reg!(AM4_L4LS_UART5_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_per_m2_div4_ck"),reg!(AM4_L4LS_UART6_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_per_m2_div4_ck"),reg!(AM4_L4LS_OCP2SCP0_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l4ls_gclk"),reg!(AM4_L4LS_OCP2SCP1_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"l4ls_gclk"),omap_clkctrl_reg_data::default()
];
static AM4_EMIF_CLKCTRL_REGS: [omap_clkctrl_reg_data; 2] = [reg!(AM4_EMIF_EMIF_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"dpll_ddr_m2_ck"),omap_clkctrl_reg_data::default()];
static AM4_DSS_CLKCTRL_REGS: [omap_clkctrl_reg_data; 2] = [reg!(AM4_DSS_DSS_CORE_CLKCTRL,core::ptr::null(),CLKF_SW_SUP|CLKF_SET_RATE_PARENT,"disp_clk"),omap_clkctrl_reg_data::default()];
static AM4_CPSW_125MHZ_CLKCTRL_REGS: [omap_clkctrl_reg_data; 2] = [reg!(AM4_CPSW_125MHZ_CPGMAC0_CLKCTRL,core::ptr::null(),CLKF_SW_SUP,"cpsw_125mhz_gclk"),omap_clkctrl_reg_data::default()];

const fn cd(base: u32, regs: *const omap_clkctrl_reg_data) -> omap_clkctrl_data { omap_clkctrl_data { addr: base, regs } }
pub static AM4_CLKCTRL_DATA: [omap_clkctrl_data; 14] = [cd(0x44df2920,AM4_L3S_TSC_CLKCTRL_REGS.as_ptr()),cd(0x44df2a28,AM4_L4_WKUP_AON_CLKCTRL_REGS.as_ptr()),cd(0x44df2a20,AM4_L4_WKUP_CLKCTRL_REGS.as_ptr()),cd(0x44df8320,AM4_MPU_CLKCTRL_REGS.as_ptr()),cd(0x44df8420,AM4_GFX_L3_CLKCTRL_REGS.as_ptr()),cd(0x44df8520,AM4_L4_RTC_CLKCTRL_REGS.as_ptr()),cd(0x44df8820,AM4_L3_CLKCTRL_REGS.as_ptr()),cd(0x44df8868,AM4_L3S_CLKCTRL_REGS.as_ptr()),cd(0x44df8b20,AM4_PRUSS_OCP_CLKCTRL_REGS.as_ptr()),cd(0x44df8c20,AM4_L4LS_CLKCTRL_REGS.as_ptr()),cd(0x44df8f20,AM4_EMIF_CLKCTRL_REGS.as_ptr()),cd(0x44df9220,AM4_DSS_CLKCTRL_REGS.as_ptr()),cd(0x44df9320,AM4_CPSW_125MHZ_CLKCTRL_REGS.as_ptr()),omap_clkctrl_data::default()];
pub static AM438X_CLKCTRL_DATA: [omap_clkctrl_data; 13] = [cd(0x44df2920,AM4_L3S_TSC_CLKCTRL_REGS.as_ptr()),cd(0x44df2a28,AM4_L4_WKUP_AON_CLKCTRL_REGS.as_ptr()),cd(0x44df2a20,AM4_L4_WKUP_CLKCTRL_REGS.as_ptr()),cd(0x44df8320,AM4_MPU_CLKCTRL_REGS.as_ptr()),cd(0x44df8420,AM4_GFX_L3_CLKCTRL_REGS.as_ptr()),cd(0x44df8820,AM4_L3_CLKCTRL_REGS.as_ptr()),cd(0x44df8868,AM4_L3S_CLKCTRL_REGS.as_ptr()),cd(0x44df8b20,AM4_PRUSS_OCP_CLKCTRL_REGS.as_ptr()),cd(0x44df8c20,AM4_L4LS_CLKCTRL_REGS.as_ptr()),cd(0x44df8f20,AM4_EMIF_CLKCTRL_REGS.as_ptr()),cd(0x44df9220,AM4_DSS_CLKCTRL_REGS.as_ptr()),cd(0x44df9320,AM4_CPSW_125MHZ_CLKCTRL_REGS.as_ptr()),omap_clkctrl_data::default()];

static mut AM43XX_CLKS: [ti_dt_clk; 11] = [
 DT_CLK!(core::ptr::null(),"timer_32k_ck","clkdiv32k_ick"),DT_CLK!(core::ptr::null(),"timer_sys_ck","sys_clkin_ck"),DT_CLK!(core::ptr::null(),"gpio0_dbclk","l4-wkup-clkctrl:0148:8"),DT_CLK!(core::ptr::null(),"gpio1_dbclk","l4ls-clkctrl:0058:8"),DT_CLK!(core::ptr::null(),"gpio2_dbclk","l4ls-clkctrl:0060:8"),DT_CLK!(core::ptr::null(),"gpio3_dbclk","l4ls-clkctrl:0068:8"),DT_CLK!(core::ptr::null(),"gpio4_dbclk","l4ls-clkctrl:0070:8"),DT_CLK!(core::ptr::null(),"gpio5_dbclk","l4ls-clkctrl:0078:8"),DT_CLK!(core::ptr::null(),"synctimer_32kclk","l4-wkup-aon-clkctrl:0008:8"),DT_CLK!(core::ptr::null(),"usb_otg_ss0_refclk960m","l3s-clkctrl:01f8:8"),DT_CLK!(core::ptr::null(),"usb_otg_ss1_refclk960m","l3s-clkctrl:0200:8"), ti_dt_clk::default()
];
static ENABLE_INIT_CLKS: [*const c_char; 1] = [b"l3-clkctrl:0000:0\0".as_ptr() as *const c_char];

pub unsafe fn am43xx_dt_clk_init() -> i32 {
    ti_dt_clocks_register(AM43XX_CLKS.as_mut_ptr());
    omap2_clk_disable_autoidle_all();
    omap2_clk_enable_init_clocks(ENABLE_INIT_CLKS.as_ptr(), ENABLE_INIT_CLKS.len());
    ti_clk_add_aliases();
    let clk1 = clk_get_sys(core::ptr::null(), b"cpsw_cpts_rft_clk\0".as_ptr() as *const c_char);
    let clk2 = clk_get_sys(core::ptr::null(), b"dpll_core_m5_ck\0".as_ptr() as *const c_char);
    clk_set_parent(clk1, clk2);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
