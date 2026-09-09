// SPDX-License-Identifier: GPL-2.0-only
/*
 * AM33XX Clock Domain data.
 *
 * Copyright (C) 2011-2012 Texas Instruments Incorporated - https://www.ti.com/
 * Vaibhav Hiremath <hvaibhav@ti.com>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/kernel.h, linux/io.h, clockdomain.h, cm.h, cm33xx.h,
// and cm-regbits-33xx.h.

static mut l4ls_am33xx_clkdm: clockdomain = clockdomain {
    name: b"l4ls_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() },
    cm_inst: AM33XX_CM_PER_MOD, clkdm_offs: AM33XX_CM_PER_L4LS_CLKSTCTRL_OFFSET,
    flags: CLKDM_CAN_SWSUP | CLKDM_STANDBY_FORCE_WAKEUP,
};
static mut l3s_am33xx_clkdm: clockdomain = clockdomain { name: b"l3s_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_PER_MOD, clkdm_offs: AM33XX_CM_PER_L3S_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut l4fw_am33xx_clkdm: clockdomain = clockdomain { name: b"l4fw_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_PER_MOD, clkdm_offs: AM33XX_CM_PER_L4FW_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut l3_am33xx_clkdm: clockdomain = clockdomain { name: b"l3_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_PER_MOD, clkdm_offs: AM33XX_CM_PER_L3_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut l4hs_am33xx_clkdm: clockdomain = clockdomain { name: b"l4hs_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_PER_MOD, clkdm_offs: AM33XX_CM_PER_L4HS_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut ocpwp_l3_am33xx_clkdm: clockdomain = clockdomain { name: b"ocpwp_l3_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_PER_MOD, clkdm_offs: AM33XX_CM_PER_OCPWP_L3_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut pruss_ocp_am33xx_clkdm: clockdomain = clockdomain { name: b"pruss_ocp_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_PER_MOD, clkdm_offs: AM33XX_CM_PER_PRUSS_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut cpsw_125mhz_am33xx_clkdm: clockdomain = clockdomain { name: b"cpsw_125mhz_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_PER_MOD, clkdm_offs: AM33XX_CM_PER_CPSW_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut lcdc_am33xx_clkdm: clockdomain = clockdomain { name: b"lcdc_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_PER_MOD, clkdm_offs: AM33XX_CM_PER_LCDC_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut clk_24mhz_am33xx_clkdm: clockdomain = clockdomain { name: b"clk_24mhz_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_PER_MOD, clkdm_offs: AM33XX_CM_PER_CLK_24MHZ_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut l4_wkup_am33xx_clkdm: clockdomain = clockdomain { name: b"l4_wkup_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"wkup_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_WKUP_MOD, clkdm_offs: AM33XX_CM_WKUP_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut l3_aon_am33xx_clkdm: clockdomain = clockdomain { name: b"l3_aon_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"wkup_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_WKUP_MOD, clkdm_offs: AM33XX_CM_L3_AON_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut l4_wkup_aon_am33xx_clkdm: clockdomain = clockdomain { name: b"l4_wkup_aon_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"wkup_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_WKUP_MOD, clkdm_offs: AM33XX_CM_L4_WKUP_AON_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut mpu_am33xx_clkdm: clockdomain = clockdomain { name: b"mpu_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"mpu_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_MPU_MOD, clkdm_offs: AM33XX_CM_MPU_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut l4_rtc_am33xx_clkdm: clockdomain = clockdomain { name: b"l4_rtc_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"rtc_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_RTC_MOD, clkdm_offs: AM33XX_CM_RTC_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut gfx_l3_am33xx_clkdm: clockdomain = clockdomain { name: b"gfx_l3_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"gfx_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_GFX_MOD, clkdm_offs: AM33XX_CM_GFX_L3_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut gfx_l4ls_gfx_am33xx_clkdm: clockdomain = clockdomain { name: b"gfx_l4ls_gfx_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"gfx_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_GFX_MOD, clkdm_offs: AM33XX_CM_GFX_L4LS_GFX_CLKSTCTRL__1_OFFSET, flags: CLKDM_CAN_SWSUP };
static mut l4_cefuse_am33xx_clkdm: clockdomain = clockdomain { name: b"l4_cefuse_clkdm\0".as_ptr(), pwrdm: powerdomain { name: b"cefuse_pwrdm\0".as_ptr() }, cm_inst: AM33XX_CM_CEFUSE_MOD, clkdm_offs: AM33XX_CM_CEFUSE_CLKSTCTRL_OFFSET, flags: CLKDM_CAN_SWSUP };

static mut clockdomains_am33xx: [*mut clockdomain; 19] = [
    &raw mut l4ls_am33xx_clkdm, &raw mut l3s_am33xx_clkdm, &raw mut l4fw_am33xx_clkdm,
    &raw mut l3_am33xx_clkdm, &raw mut l4hs_am33xx_clkdm, &raw mut ocpwp_l3_am33xx_clkdm,
    &raw mut pruss_ocp_am33xx_clkdm, &raw mut cpsw_125mhz_am33xx_clkdm, &raw mut lcdc_am33xx_clkdm,
    &raw mut clk_24mhz_am33xx_clkdm, &raw mut l4_wkup_am33xx_clkdm, &raw mut l3_aon_am33xx_clkdm,
    &raw mut l4_wkup_aon_am33xx_clkdm, &raw mut mpu_am33xx_clkdm, &raw mut l4_rtc_am33xx_clkdm,
    &raw mut gfx_l3_am33xx_clkdm, &raw mut gfx_l4ls_gfx_am33xx_clkdm, &raw mut l4_cefuse_am33xx_clkdm,
    core::ptr::null_mut(),
];

pub unsafe fn am33xx_clockdomains_init() {
    clkdm_register_platform_funcs(&am33xx_clkdm_operations);
    clkdm_register_clkdms(clockdomains_am33xx.as_mut_ptr());
    clkdm_complete_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
