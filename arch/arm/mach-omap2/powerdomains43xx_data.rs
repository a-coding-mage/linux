// SPDX-License-Identifier: GPL-2.0-only
/*
 * AM43xx Power domains framework
 *
 * Copyright (C) 2013 Texas Instruments, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

static mut gfx_43xx_pwrdm: struct_powerdomain = struct_powerdomain {
	name: "gfx_pwrdm",
	voltdm: struct_voltdm { name: "core" },
	prcm_offs: AM43XX_PRM_GFX_INST,
	prcm_partition: AM43XX_PRM_PARTITION,
	pwrsts: PWRSTS_OFF_ON,
	banks: 1,
	pwrsts_mem_on: [PWRSTS_ON],
	flags: PWRDM_HAS_LOWPOWERSTATECHANGE,
};

static mut mpu_43xx_pwrdm: struct_powerdomain = struct_powerdomain {
	name: "mpu_pwrdm",
	voltdm: struct_voltdm { name: "mpu" },
	prcm_offs: AM43XX_PRM_MPU_INST,
	prcm_partition: AM43XX_PRM_PARTITION,
	pwrsts: PWRSTS_OFF_RET_ON,
	pwrsts_logic_ret: PWRSTS_OFF_RET,
	banks: 3,
	pwrsts_mem_ret: [PWRSTS_OFF_RET, PWRSTS_OFF_RET, PWRSTS_OFF_RET],
	pwrsts_mem_on: [PWRSTS_ON, PWRSTS_ON, PWRSTS_ON],
	flags: PWRDM_HAS_LOWPOWERSTATECHANGE,
};

static mut rtc_43xx_pwrdm: struct_powerdomain = struct_powerdomain {
	name: "rtc_pwrdm",
	voltdm: struct_voltdm { name: "rtc" },
	prcm_offs: AM43XX_PRM_RTC_INST,
	prcm_partition: AM43XX_PRM_PARTITION,
	pwrsts: PWRSTS_ON,
};

static mut wkup_43xx_pwrdm: struct_powerdomain = struct_powerdomain {
	name: "wkup_pwrdm",
	voltdm: struct_voltdm { name: "core" },
	prcm_offs: AM43XX_PRM_WKUP_INST,
	prcm_partition: AM43XX_PRM_PARTITION,
	pwrsts: PWRSTS_ON,
	banks: 1,
	pwrsts_mem_on: [PWRSTS_ON],
};

static mut tamper_43xx_pwrdm: struct_powerdomain = struct_powerdomain {
	name: "tamper_pwrdm",
	voltdm: struct_voltdm { name: "tamper" },
	prcm_offs: AM43XX_PRM_TAMPER_INST,
	prcm_partition: AM43XX_PRM_PARTITION,
	pwrsts: PWRSTS_ON,
};

static mut cefuse_43xx_pwrdm: struct_powerdomain = struct_powerdomain {
	name: "cefuse_pwrdm",
	voltdm: struct_voltdm { name: "core" },
	prcm_offs: AM43XX_PRM_CEFUSE_INST,
	prcm_partition: AM43XX_PRM_PARTITION,
	pwrsts: PWRSTS_OFF_ON,
	flags: PWRDM_HAS_LOWPOWERSTATECHANGE,
};

static mut per_43xx_pwrdm: struct_powerdomain = struct_powerdomain {
	name: "per_pwrdm",
	voltdm: struct_voltdm { name: "core" },
	prcm_offs: AM43XX_PRM_PER_INST,
	prcm_partition: AM43XX_PRM_PARTITION,
	pwrsts: PWRSTS_OFF_RET_ON,
	pwrsts_logic_ret: PWRSTS_OFF_RET,
	banks: 4,
	pwrsts_mem_ret: [PWRSTS_OFF_RET, PWRSTS_OFF_RET, PWRSTS_OFF_RET, PWRSTS_OFF_RET],
	pwrsts_mem_on: [PWRSTS_ON, PWRSTS_ON, PWRSTS_ON, PWRSTS_ON],
	flags: PWRDM_HAS_LOWPOWERSTATECHANGE,
};

static mut powerdomains_am43xx: [*mut struct_powerdomain; 8] = [
	unsafe { &raw mut gfx_43xx_pwrdm },
	unsafe { &raw mut mpu_43xx_pwrdm },
	unsafe { &raw mut rtc_43xx_pwrdm },
	unsafe { &raw mut wkup_43xx_pwrdm },
	unsafe { &raw mut tamper_43xx_pwrdm },
	unsafe { &raw mut cefuse_43xx_pwrdm },
	unsafe { &raw mut per_43xx_pwrdm },
	core::ptr::null_mut(),
];

unsafe extern "C" fn am43xx_check_vcvp() -> i32 {
	0
}

pub unsafe extern "C" fn am43xx_powerdomains_init() {
	omap4_pwrdm_operations.pwrdm_has_voltdm = Some(am43xx_check_vcvp);
	pwrdm_register_platform_funcs(&raw const omap4_pwrdm_operations);
	pwrdm_register_pwrdms(&raw const powerdomains_am43xx);
	pwrdm_complete_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
