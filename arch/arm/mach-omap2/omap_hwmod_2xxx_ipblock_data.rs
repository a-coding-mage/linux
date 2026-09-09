// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap_hwmod_2xxx_ipblock_data.c - common IP block data for OMAP2xxx
 *
 * Copyright (C) 2011 Nokia Corporation
 * Paul Walmsley
 */

// C dependencies: linux/types.h, omap_hwmod.h, omap_hwmod_common_data.h,
// cm-regbits-24xx.h, prm-regbits-24xx.h, and wd_timer.h.

static mut omap2_dispc_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x0000, sysc_offs: 0x0010, syss_offs: 0x0014,
    sysc_flags: SYSC_HAS_SIDLEMODE | SYSC_HAS_MIDLEMODE | SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE,
    idlemodes: SIDLE_FORCE | SIDLE_NO | SIDLE_SMART | MSTANDBY_FORCE | MSTANDBY_NO | MSTANDBY_SMART,
    sysc_fields: &omap_hwmod_sysc_type1,
};
static mut omap2_dispc_hwmod_class: omap_hwmod_class = omap_hwmod_class {
    name: "dispc", sysc: &omap2_dispc_sysc,
};

// OMAP2xxx Timer Common
static mut omap2xxx_timer_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x0000, sysc_offs: 0x0010, syss_offs: 0x0014,
    sysc_flags: SYSC_HAS_SIDLEMODE | SYSC_HAS_CLOCKACTIVITY | SYSC_HAS_ENAWAKEUP | SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE | SYSS_HAS_RESET_STATUS,
    idlemodes: SIDLE_FORCE | SIDLE_NO | SIDLE_SMART,
    sysc_fields: &omap_hwmod_sysc_type1,
};
static mut omap2xxx_timer_hwmod_class: omap_hwmod_class = omap_hwmod_class {
    name: "timer", sysc: &omap2xxx_timer_sysc,
};

// 'wd_timer' class: 32-bit watchdog upward counter that generates a pulse on reset overflow.
static mut omap2xxx_wd_timer_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x0000, sysc_offs: 0x0010, syss_offs: 0x0014,
    sysc_flags: SYSC_HAS_EMUFREE | SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE | SYSS_HAS_RESET_STATUS,
    sysc_fields: &omap_hwmod_sysc_type1,
};
static mut omap2xxx_wd_timer_hwmod_class: omap_hwmod_class = omap_hwmod_class {
    name: "wd_timer", sysc: &omap2xxx_wd_timer_sysc,
    pre_shutdown: Some(omap2_wd_timer_disable), reset: Some(omap2_wd_timer_reset),
};

// 'gpio' class: general purpose io module
static mut omap2xxx_gpio_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x0000, sysc_offs: 0x0010, syss_offs: 0x0014,
    sysc_flags: SYSC_HAS_ENAWAKEUP | SYSC_HAS_SIDLEMODE | SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE | SYSS_HAS_RESET_STATUS,
    idlemodes: SIDLE_FORCE | SIDLE_NO | SIDLE_SMART, sysc_fields: &omap_hwmod_sysc_type1,
};
pub static mut omap2xxx_gpio_hwmod_class: omap_hwmod_class = omap_hwmod_class { name: "gpio", sysc: &omap2xxx_gpio_sysc };

// 'mailbox' class: mailbox module allowing communication between on-chip processors.
static mut omap2xxx_mailbox_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x000, sysc_offs: 0x010, syss_offs: 0x014,
    sysc_flags: SYSC_HAS_CLOCKACTIVITY | SYSC_HAS_SIDLEMODE | SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE,
    idlemodes: SIDLE_FORCE | SIDLE_NO | SIDLE_SMART, sysc_fields: &omap_hwmod_sysc_type1,
};
pub static mut omap2xxx_mailbox_hwmod_class: omap_hwmod_class = omap_hwmod_class { name: "mailbox", sysc: &omap2xxx_mailbox_sysc };

// 'mcspi' class: multichannel serial port interface / master/slave synchronous serial bus.
static mut omap2xxx_mcspi_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x0000, sysc_offs: 0x0010, syss_offs: 0x0014,
    sysc_flags: SYSC_HAS_CLOCKACTIVITY | SYSC_HAS_SIDLEMODE | SYSC_HAS_ENAWAKEUP | SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE | SYSS_HAS_RESET_STATUS,
    idlemodes: SIDLE_FORCE | SIDLE_NO | SIDLE_SMART, sysc_fields: &omap_hwmod_sysc_type1,
};
pub static mut omap2xxx_mcspi_class: omap_hwmod_class = omap_hwmod_class { name: "mcspi", sysc: &omap2xxx_mcspi_sysc };

// 'gpmc' class: general purpose memory controller.
static mut omap2xxx_gpmc_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x0000, sysc_offs: 0x0010, syss_offs: 0x0014,
    sysc_flags: SYSC_HAS_AUTOIDLE | SYSC_HAS_SIDLEMODE | SYSC_HAS_SOFTRESET | SYSS_HAS_RESET_STATUS,
    idlemodes: SIDLE_FORCE | SIDLE_NO | SIDLE_SMART, sysc_fields: &omap_hwmod_sysc_type1,
};
static mut omap2xxx_gpmc_hwmod_class: omap_hwmod_class = omap_hwmod_class { name: "gpmc", sysc: &omap2xxx_gpmc_sysc };

// IP blocks

pub static mut omap2xxx_l3_main_hwmod: omap_hwmod = omap_hwmod { name: "l3_main", class: &l3_hwmod_class, flags: HWMOD_NO_IDLEST };
pub static mut omap2xxx_l4_core_hwmod: omap_hwmod = omap_hwmod { name: "l4_core", class: &l4_hwmod_class, flags: HWMOD_NO_IDLEST };
pub static mut omap2xxx_l4_wkup_hwmod: omap_hwmod = omap_hwmod { name: "l4_wkup", class: &l4_hwmod_class, flags: HWMOD_NO_IDLEST };
pub static mut omap2xxx_mpu_hwmod: omap_hwmod = omap_hwmod { name: "mpu", class: &mpu_hwmod_class, main_clk: "mpu_ck" };

macro_rules! timer_hwmod {
    ($n:ident, $s:literal, $shift:ident) => {
        pub static mut $n: omap_hwmod = omap_hwmod {
            name: $s, main_clk: concat!("gpt", stringify!($n), "_fck"),
            prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: $shift } },
            class: &omap2xxx_timer_hwmod_class, flags: HWMOD_SET_DEFAULT_CLOCKACT,
        };
    };
}

// Timer definitions retain their explicit source-level names and clock metadata.
pub static mut omap2xxx_timer3_hwmod: omap_hwmod = omap_hwmod { name: "timer3", main_clk: "gpt3_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_GPT3_SHIFT } }, class: &omap2xxx_timer_hwmod_class, flags: HWMOD_SET_DEFAULT_CLOCKACT };
pub static mut omap2xxx_timer4_hwmod: omap_hwmod = omap_hwmod { name: "timer4", main_clk: "gpt4_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_GPT4_SHIFT } }, class: &omap2xxx_timer_hwmod_class, flags: HWMOD_SET_DEFAULT_CLOCKACT };
pub static mut omap2xxx_timer5_hwmod: omap_hwmod = omap_hwmod { name: "timer5", main_clk: "gpt5_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_GPT5_SHIFT } }, class: &omap2xxx_timer_hwmod_class, flags: HWMOD_SET_DEFAULT_CLOCKACT };
pub static mut omap2xxx_timer6_hwmod: omap_hwmod = omap_hwmod { name: "timer6", main_clk: "gpt6_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_GPT6_SHIFT } }, class: &omap2xxx_timer_hwmod_class, flags: HWMOD_SET_DEFAULT_CLOCKACT };
pub static mut omap2xxx_timer7_hwmod: omap_hwmod = omap_hwmod { name: "timer7", main_clk: "gpt7_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_GPT7_SHIFT } }, class: &omap2xxx_timer_hwmod_class, flags: HWMOD_SET_DEFAULT_CLOCKACT };
pub static mut omap2xxx_timer8_hwmod: omap_hwmod = omap_hwmod { name: "timer8", main_clk: "gpt8_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_GPT8_SHIFT } }, class: &omap2xxx_timer_hwmod_class, flags: HWMOD_SET_DEFAULT_CLOCKACT };
pub static mut omap2xxx_timer9_hwmod: omap_hwmod = omap_hwmod { name: "timer9", main_clk: "gpt9_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_GPT9_SHIFT } }, class: &omap2xxx_timer_hwmod_class, flags: HWMOD_SET_DEFAULT_CLOCKACT };
pub static mut omap2xxx_timer10_hwmod: omap_hwmod = omap_hwmod { name: "timer10", main_clk: "gpt10_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_GPT10_SHIFT } }, class: &omap2xxx_timer_hwmod_class, flags: HWMOD_SET_DEFAULT_CLOCKACT };
pub static mut omap2xxx_timer11_hwmod: omap_hwmod = omap_hwmod { name: "timer11", main_clk: "gpt11_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_GPT11_SHIFT } }, class: &omap2xxx_timer_hwmod_class, flags: HWMOD_SET_DEFAULT_CLOCKACT };
pub static mut omap2xxx_timer12_hwmod: omap_hwmod = omap_hwmod { name: "timer12", main_clk: "gpt12_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_GPT12_SHIFT } }, class: &omap2xxx_timer_hwmod_class, flags: HWMOD_SET_DEFAULT_CLOCKACT };

pub static mut omap2xxx_wd_timer2_hwmod: omap_hwmod = omap_hwmod { name: "wd_timer2", class: &omap2xxx_wd_timer_hwmod_class, main_clk: "mpu_wdt_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: WKUP_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_MPU_WDT_SHIFT } } };

pub static mut omap2xxx_uart1_hwmod: omap_hwmod = omap_hwmod { name: "uart1", main_clk: "uart1_fck", flags: DEBUG_OMAP2UART1_FLAGS | HWMOD_SWSUP_SIDLE_ACT, prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_EN_UART1_SHIFT } }, class: &omap2_uart_class };
pub static mut omap2xxx_uart2_hwmod: omap_hwmod = omap_hwmod { name: "uart2", main_clk: "uart2_fck", flags: DEBUG_OMAP2UART2_FLAGS | HWMOD_SWSUP_SIDLE_ACT, prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_EN_UART2_SHIFT } }, class: &omap2_uart_class };
pub static mut omap2xxx_uart3_hwmod: omap_hwmod = omap_hwmod { name: "uart3", main_clk: "uart3_fck", flags: DEBUG_OMAP2UART3_FLAGS | HWMOD_SWSUP_SIDLE_ACT, prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 2, idlest_idle_bit: OMAP24XX_EN_UART3_SHIFT } }, class: &omap2_uart_class };

static mut dss_opt_clks: [omap_hwmod_opt_clk; 2] = [
    omap_hwmod_opt_clk { role: "tv_clk", clk: "dss_54m_fck" },
    omap_hwmod_opt_clk { role: "sys_clk", clk: "dss2_fck" },
];
pub static mut omap2xxx_dss_core_hwmod: omap_hwmod = omap_hwmod { name: "dss_core", class: &omap2_dss_hwmod_class, main_clk: "dss1_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1 } }, opt_clks: dss_opt_clks.as_ptr(), opt_clks_cnt: 2, flags: HWMOD_NO_IDLEST | HWMOD_CONTROL_OPT_CLKS_IN_RESET };
pub static mut omap2xxx_dss_dispc_hwmod: omap_hwmod = omap_hwmod { name: "dss_dispc", class: &omap2_dispc_hwmod_class, main_clk: "dss1_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1 } }, flags: HWMOD_NO_IDLEST, dev_attr: &omap2_3_dss_dispc_dev_attr };
static mut dss_rfbi_opt_clks: [omap_hwmod_opt_clk; 1] = [omap_hwmod_opt_clk { role: "ick", clk: "dss_ick" }];
pub static mut omap2xxx_dss_rfbi_hwmod: omap_hwmod = omap_hwmod { name: "dss_rfbi", class: &omap2_rfbi_hwmod_class, main_clk: "dss1_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD } }, opt_clks: dss_rfbi_opt_clks.as_ptr(), opt_clks_cnt: 1, flags: HWMOD_NO_IDLEST };
pub static mut omap2xxx_dss_venc_hwmod: omap_hwmod = omap_hwmod { name: "dss_venc", class: &omap2_venc_hwmod_class, main_clk: "dss_54m_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD } }, flags: HWMOD_NO_IDLEST };

macro_rules! gpio_hwmod { ($n:ident, $s:literal) => { pub static mut $n: omap_hwmod = omap_hwmod { name: $s, flags: HWMOD_CONTROL_OPT_CLKS_IN_RESET, main_clk: "gpios_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: WKUP_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_GPIOS_SHIFT } }, class: &omap2xxx_gpio_hwmod_class }; }; }
gpio_hwmod!(omap2xxx_gpio1_hwmod, "gpio1"); gpio_hwmod!(omap2xxx_gpio2_hwmod, "gpio2"); gpio_hwmod!(omap2xxx_gpio3_hwmod, "gpio3"); gpio_hwmod!(omap2xxx_gpio4_hwmod, "gpio4");

pub static mut omap2xxx_mcspi1_hwmod: omap_hwmod = omap_hwmod { name: "mcspi1", main_clk: "mcspi1_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_MCSPI1_SHIFT } }, class: &omap2xxx_mcspi_class };
pub static mut omap2xxx_mcspi2_hwmod: omap_hwmod = omap_hwmod { name: "mcspi2", main_clk: "mcspi2_fck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 1, idlest_idle_bit: OMAP24XX_ST_MCSPI2_SHIFT } }, class: &omap2xxx_mcspi_class };
pub static mut omap2xxx_gpmc_hwmod: omap_hwmod = omap_hwmod { name: "gpmc", class: &omap2xxx_gpmc_hwmod_class, main_clk: "gpmc_fck", flags: HWMOD_NO_IDLEST | DEBUG_OMAP_GPMC_HWMOD_FLAGS, prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD } } };

static mut omap2_rng_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig { rev_offs: 0x3c, sysc_offs: 0x40, syss_offs: 0x44, sysc_flags: SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE | SYSS_HAS_RESET_STATUS, sysc_fields: &omap_hwmod_sysc_type1 };
static mut omap2_rng_hwmod_class: omap_hwmod_class = omap_hwmod_class { name: "rng", sysc: &omap2_rng_sysc };
pub static mut omap2xxx_rng_hwmod: omap_hwmod = omap_hwmod { name: "rng", main_clk: "l4_ck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 4, idlest_idle_bit: OMAP24XX_ST_RNG_SHIFT } }, flags: HWMOD_INIT_NO_RESET, class: &omap2_rng_hwmod_class };
static mut omap2_sham_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig { rev_offs: 0x5c, sysc_offs: 0x60, syss_offs: 0x64, sysc_flags: SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE | SYSS_HAS_RESET_STATUS, sysc_fields: &omap_hwmod_sysc_type1 };
static mut omap2xxx_sham_class: omap_hwmod_class = omap_hwmod_class { name: "sham", sysc: &omap2_sham_sysc };
pub static mut omap2xxx_sham_hwmod: omap_hwmod = omap_hwmod { name: "sham", main_clk: "l4_ck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 4, idlest_idle_bit: OMAP24XX_ST_SHA_SHIFT } }, class: &omap2xxx_sham_class };
static mut omap2_aes_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig { rev_offs: 0x44, sysc_offs: 0x48, syss_offs: 0x4c, sysc_flags: SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE | SYSS_HAS_RESET_STATUS, sysc_fields: &omap_hwmod_sysc_type1 };
static mut omap2xxx_aes_class: omap_hwmod_class = omap_hwmod_class { name: "aes", sysc: &omap2_aes_sysc };
pub static mut omap2xxx_aes_hwmod: omap_hwmod = omap_hwmod { name: "aes", main_clk: "l4_ck", prcm: omap_hwmod_prcm { omap2: omap_hwmod_prcm_omap2 { module_offs: CORE_MOD, idlest_reg_id: 4, idlest_idle_bit: OMAP24XX_ST_AES_SHIFT } }, class: &omap2xxx_aes_class };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
