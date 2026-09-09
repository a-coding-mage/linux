// SPDX-License-Identifier: GPL-2.0-only
/*
 * AM33XX Power domain data
 *
 * Copyright (C) 2011-2012 Texas Instruments Incorporated - https://www.ti.com/
 */

// Dependencies supplied by the surrounding kernel translation.

static mut gfx_33xx_pwrdm: powerdomain = powerdomain {
	name: "gfx_pwrdm",
	voltdm: voltagedomain { name: "core" },
	prcm_offs: AM33XX_PRM_GFX_MOD,
	pwrstctrl_offs: AM33XX_PM_GFX_PWRSTCTRL_OFFSET,
	pwrstst_offs: AM33XX_PM_GFX_PWRSTST_OFFSET,
	pwrsts: PWRSTS_OFF_RET_ON,
	pwrsts_logic_ret: PWRSTS_OFF_RET,
	flags: PWRDM_HAS_LOWPOWERSTATECHANGE,
	banks: 1,
	logicretstate_mask: AM33XX_LOGICRETSTATE_MASK,
	mem_on_mask: [AM33XX_GFX_MEM_ONSTATE_MASK],
	mem_ret_mask: [AM33XX_GFX_MEM_RETSTATE_MASK],
	mem_pwrst_mask: [AM33XX_GFX_MEM_STATEST_MASK],
	mem_retst_mask: [AM33XX_GFX_MEM_RETSTATE_MASK],
	pwrsts_mem_ret: [PWRSTS_OFF_RET],
	pwrsts_mem_on: [PWRSTS_ON],
};

static mut rtc_33xx_pwrdm: powerdomain = powerdomain {
	name: "rtc_pwrdm",
	voltdm: voltagedomain { name: "rtc" },
	prcm_offs: AM33XX_PRM_RTC_MOD,
	pwrstctrl_offs: AM33XX_PM_RTC_PWRSTCTRL_OFFSET,
	pwrstst_offs: AM33XX_PM_RTC_PWRSTST_OFFSET,
	pwrsts: PWRSTS_ON,
	logicretstate_mask: AM33XX_LOGICRETSTATE_MASK,
};

static mut wkup_33xx_pwrdm: powerdomain = powerdomain {
	name: "wkup_pwrdm",
	voltdm: voltagedomain { name: "core" },
	prcm_offs: AM33XX_PRM_WKUP_MOD,
	pwrstctrl_offs: AM33XX_PM_WKUP_PWRSTCTRL_OFFSET,
	pwrstst_offs: AM33XX_PM_WKUP_PWRSTST_OFFSET,
	pwrsts: PWRSTS_ON,
	logicretstate_mask: AM33XX_LOGICRETSTATE_3_3_MASK,
};

static mut per_33xx_pwrdm: powerdomain = powerdomain {
	name: "per_pwrdm",
	voltdm: voltagedomain { name: "core" },
	prcm_offs: AM33XX_PRM_PER_MOD,
	pwrstctrl_offs: AM33XX_PM_PER_PWRSTCTRL_OFFSET,
	pwrstst_offs: AM33XX_PM_PER_PWRSTST_OFFSET,
	pwrsts: PWRSTS_OFF_RET_ON,
	pwrsts_logic_ret: PWRSTS_OFF_RET,
	flags: PWRDM_HAS_LOWPOWERSTATECHANGE,
	banks: 3,
	logicretstate_mask: AM33XX_LOGICRETSTATE_3_3_MASK,
	mem_on_mask: [
		AM33XX_PRUSS_MEM_ONSTATE_MASK, // pruss_mem
		AM33XX_PER_MEM_ONSTATE_MASK, // per_mem
		AM33XX_RAM_MEM_ONSTATE_MASK, // ram_mem
	],
	mem_ret_mask: [
		AM33XX_PRUSS_MEM_RETSTATE_MASK, // pruss_mem
		AM33XX_PER_MEM_RETSTATE_MASK, // per_mem
		AM33XX_RAM_MEM_RETSTATE_MASK, // ram_mem
	],
	mem_pwrst_mask: [
		AM33XX_PRUSS_MEM_STATEST_MASK, // pruss_mem
		AM33XX_PER_MEM_STATEST_MASK, // per_mem
		AM33XX_RAM_MEM_STATEST_MASK, // ram_mem
	],
	mem_retst_mask: [
		AM33XX_PRUSS_MEM_RETSTATE_MASK, // pruss_mem
		AM33XX_PER_MEM_RETSTATE_MASK, // per_mem
		AM33XX_RAM_MEM_RETSTATE_MASK, // ram_mem
	],
	pwrsts_mem_ret: [PWRSTS_OFF_RET, PWRSTS_OFF_RET, PWRSTS_OFF_RET],
	pwrsts_mem_on: [PWRSTS_ON, PWRSTS_ON, PWRSTS_ON],
};

static mut mpu_33xx_pwrdm: powerdomain = powerdomain {
	name: "mpu_pwrdm",
	voltdm: voltagedomain { name: "mpu" },
	prcm_offs: AM33XX_PRM_MPU_MOD,
	pwrstctrl_offs: AM33XX_PM_MPU_PWRSTCTRL_OFFSET,
	pwrstst_offs: AM33XX_PM_MPU_PWRSTST_OFFSET,
	pwrsts: PWRSTS_OFF_RET_ON,
	pwrsts_logic_ret: PWRSTS_OFF_RET,
	flags: PWRDM_HAS_LOWPOWERSTATECHANGE,
	banks: 3,
	logicretstate_mask: AM33XX_LOGICRETSTATE_MASK,
	mem_on_mask: [AM33XX_MPU_L1_ONSTATE_MASK, AM33XX_MPU_L2_ONSTATE_MASK, AM33XX_MPU_RAM_ONSTATE_MASK],
	mem_ret_mask: [AM33XX_MPU_L1_RETSTATE_MASK, AM33XX_MPU_L2_RETSTATE_MASK, AM33XX_MPU_RAM_RETSTATE_MASK],
	mem_pwrst_mask: [AM33XX_MPU_L1_STATEST_MASK, AM33XX_MPU_L2_STATEST_MASK, AM33XX_MPU_RAM_STATEST_MASK],
	mem_retst_mask: [AM33XX_MPU_L1_RETSTATE_MASK, AM33XX_MPU_L2_RETSTATE_MASK, AM33XX_MPU_RAM_RETSTATE_MASK],
	pwrsts_mem_ret: [PWRSTS_OFF_RET, PWRSTS_OFF_RET, PWRSTS_OFF_RET],
	pwrsts_mem_on: [PWRSTS_ON, PWRSTS_ON, PWRSTS_ON],
};

static mut cefuse_33xx_pwrdm: powerdomain = powerdomain {
	name: "cefuse_pwrdm",
	voltdm: voltagedomain { name: "core" },
	prcm_offs: AM33XX_PRM_CEFUSE_MOD,
	pwrstctrl_offs: AM33XX_PM_CEFUSE_PWRSTCTRL_OFFSET,
	pwrstst_offs: AM33XX_PM_CEFUSE_PWRSTST_OFFSET,
	pwrsts: PWRSTS_OFF_ON,
};

static mut powerdomains_am33xx: [*mut powerdomain; 7] = [
	&raw mut gfx_33xx_pwrdm,
	&raw mut rtc_33xx_pwrdm,
	&raw mut wkup_33xx_pwrdm,
	&raw mut per_33xx_pwrdm,
	&raw mut mpu_33xx_pwrdm,
	&raw mut cefuse_33xx_pwrdm,
	core::ptr::null_mut(),
];

pub unsafe fn am33xx_powerdomains_init() {
	pwrdm_register_platform_funcs(&am33xx_pwrdm_operations);
	pwrdm_register_pwrdms(powerdomains_am33xx.as_mut_ptr());
	pwrdm_complete_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
