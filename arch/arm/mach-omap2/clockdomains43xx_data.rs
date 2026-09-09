// SPDX-License-Identifier: GPL-2.0-only
/*
 * AM43xx Clock domains framework
 *
 * Copyright (C) 2013 Texas Instruments, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

static mut l4_cefuse_43xx_clkdm: clockdomain = clockdomain {
    name: b"l4_cefuse_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"cefuse_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_CEFUSE_INST,
    clkdm_offs: AM43XX_CM_CEFUSE_CEFUSE_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut mpu_43xx_clkdm: clockdomain = clockdomain {
    name: b"mpu_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"mpu_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_MPU_INST,
    clkdm_offs: AM43XX_CM_MPU_MPU_CDOFFS,
    flags: CLKDM_CAN_HWSUP_SWSUP,
};

static mut l4ls_43xx_clkdm: clockdomain = clockdomain {
    name: b"l4ls_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_PER_INST,
    clkdm_offs: AM43XX_CM_PER_L4LS_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut tamper_43xx_clkdm: clockdomain = clockdomain {
    name: b"tamper_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"tamper_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_TAMPER_INST,
    clkdm_offs: AM43XX_CM_TAMPER_TAMPER_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut l4_rtc_43xx_clkdm: clockdomain = clockdomain {
    name: b"l4_rtc_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"rtc_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_RTC_INST,
    clkdm_offs: AM43XX_CM_RTC_RTC_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut pruss_ocp_43xx_clkdm: clockdomain = clockdomain {
    name: b"pruss_ocp_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_PER_INST,
    clkdm_offs: AM43XX_CM_PER_ICSS_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut ocpwp_l3_43xx_clkdm: clockdomain = clockdomain {
    name: b"ocpwp_l3_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_PER_INST,
    clkdm_offs: AM43XX_CM_PER_OCPWP_L3_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut l3s_tsc_43xx_clkdm: clockdomain = clockdomain {
    name: b"l3s_tsc_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"wkup_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_WKUP_INST,
    clkdm_offs: AM43XX_CM_WKUP_L3S_TSC_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut lcdc_43xx_clkdm: clockdomain = clockdomain {
    name: b"lcdc_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_PER_INST,
    clkdm_offs: AM43XX_CM_PER_LCDC_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut dss_43xx_clkdm: clockdomain = clockdomain {
    name: b"dss_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_PER_INST,
    clkdm_offs: AM43XX_CM_PER_DSS_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut l3_aon_43xx_clkdm: clockdomain = clockdomain {
    name: b"l3_aon_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"wkup_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_WKUP_INST,
    clkdm_offs: AM43XX_CM_WKUP_L3_AON_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut emif_43xx_clkdm: clockdomain = clockdomain {
    name: b"emif_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_PER_INST,
    clkdm_offs: AM43XX_CM_PER_EMIF_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut l4_wkup_aon_43xx_clkdm: clockdomain = clockdomain {
    name: b"l4_wkup_aon_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"wkup_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_WKUP_INST,
    clkdm_offs: AM43XX_CM_WKUP_L4_WKUP_AON_CDOFFS,
};

static mut l3_43xx_clkdm: clockdomain = clockdomain {
    name: b"l3_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_PER_INST,
    clkdm_offs: AM43XX_CM_PER_L3_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut l4_wkup_43xx_clkdm: clockdomain = clockdomain {
    name: b"l4_wkup_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"wkup_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_WKUP_INST,
    clkdm_offs: AM43XX_CM_WKUP_WKUP_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut cpsw_125mhz_43xx_clkdm: clockdomain = clockdomain {
    name: b"cpsw_125mhz_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_PER_INST,
    clkdm_offs: AM43XX_CM_PER_CPSW_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut gfx_l3_43xx_clkdm: clockdomain = clockdomain {
    name: b"gfx_l3_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"gfx_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_GFX_INST,
    clkdm_offs: AM43XX_CM_GFX_GFX_L3_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut l3s_43xx_clkdm: clockdomain = clockdomain {
    name: b"l3s_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"per_pwrdm\0".as_ptr() as *const _ },
    prcm_partition: AM43XX_CM_PARTITION,
    cm_inst: AM43XX_CM_PER_INST,
    clkdm_offs: AM43XX_CM_PER_L3S_CDOFFS,
    flags: CLKDM_CAN_SWSUP,
};

static mut clockdomains_am43xx: [*mut clockdomain; 19] = [
    &raw mut l4_cefuse_43xx_clkdm,
    &raw mut mpu_43xx_clkdm,
    &raw mut l4ls_43xx_clkdm,
    &raw mut tamper_43xx_clkdm,
    &raw mut l4_rtc_43xx_clkdm,
    &raw mut pruss_ocp_43xx_clkdm,
    &raw mut ocpwp_l3_43xx_clkdm,
    &raw mut l3s_tsc_43xx_clkdm,
    &raw mut lcdc_43xx_clkdm,
    &raw mut dss_43xx_clkdm,
    &raw mut l3_aon_43xx_clkdm,
    &raw mut emif_43xx_clkdm,
    &raw mut l4_wkup_aon_43xx_clkdm,
    &raw mut l3_43xx_clkdm,
    &raw mut l4_wkup_43xx_clkdm,
    &raw mut cpsw_125mhz_43xx_clkdm,
    &raw mut gfx_l3_43xx_clkdm,
    &raw mut l3s_43xx_clkdm,
    core::ptr::null_mut(),
];

pub unsafe fn am43xx_clockdomains_init() {
    clkdm_register_platform_funcs(&am43xx_clkdm_operations);
    clkdm_register_clkdms(clockdomains_am43xx.as_mut_ptr());
    clkdm_complete_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
