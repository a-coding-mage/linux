// SPDX-License-Identifier: GPL-2.0-only
/*
 * TI81XX Clock Domain data.
 *
 * Copyright (C) 2010 Texas Instruments, Inc. - https://www.ti.com/
 * Copyright (C) 2013 SKTB SKiT, http://www.skitlab.ru/
 */

/* Translated from the C implementation. C includes and build-time header
 * dependencies are supplied by the surrounding kernel translation. */

/*
 * Note that 814x seems to have HWSUP_SWSUP for many clockdomains
 * while 816x does not. According to the TRM, 816x only has HWSUP
 * for ALWON_L3_FAST. Also note that the TI tree clockdomains81xx.h
 * seems to have the related ifdef the wrong way around claiming
 * 816x supports HWSUP while 814x does not. For now, we only set
 * HWSUP for ALWON_L3_FAST as that seems to be supported for both
 * dm814x and dm816x.
 */

/* Common for 81xx */

static mut alwon_l3_slow_81xx_clkdm: clockdomain = clockdomain {
    name: "alwon_l3s_clkdm",
    pwrdm: powerdomain { name: "alwon_pwrdm" },
    cm_inst: TI81XX_CM_ALWON_MOD,
    clkdm_offs: TI81XX_CM_ALWON_L3_SLOW_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut alwon_l3_med_81xx_clkdm: clockdomain = clockdomain {
    name: "alwon_l3_med_clkdm",
    pwrdm: powerdomain { name: "alwon_pwrdm" },
    cm_inst: TI81XX_CM_ALWON_MOD,
    clkdm_offs: TI81XX_CM_ALWON_L3_MED_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut alwon_l3_fast_81xx_clkdm: clockdomain = clockdomain {
    name: "alwon_l3_fast_clkdm",
    pwrdm: powerdomain { name: "alwon_pwrdm" },
    cm_inst: TI81XX_CM_ALWON_MOD,
    clkdm_offs: TI81XX_CM_ALWON_L3_FAST_CLKDM,
    flags: CLKDM_CAN_HWSUP_SWSUP,
};

static mut alwon_ethernet_81xx_clkdm: clockdomain = clockdomain {
    name: "alwon_ethernet_clkdm",
    pwrdm: powerdomain { name: "alwon_pwrdm" },
    cm_inst: TI81XX_CM_ALWON_MOD,
    clkdm_offs: TI81XX_CM_ETHERNET_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut mmu_81xx_clkdm: clockdomain = clockdomain {
    name: "mmu_clkdm",
    pwrdm: powerdomain { name: "alwon_pwrdm" },
    cm_inst: TI81XX_CM_ALWON_MOD,
    clkdm_offs: TI81XX_CM_MMU_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut mmu_cfg_81xx_clkdm: clockdomain = clockdomain {
    name: "mmu_cfg_clkdm",
    pwrdm: powerdomain { name: "alwon_pwrdm" },
    cm_inst: TI81XX_CM_ALWON_MOD,
    clkdm_offs: TI81XX_CM_MMUCFG_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut default_l3_slow_81xx_clkdm: clockdomain = clockdomain {
    name: "default_l3_slow_clkdm",
    pwrdm: powerdomain { name: "default_pwrdm" },
    cm_inst: TI81XX_CM_DEFAULT_MOD,
    clkdm_offs: TI816X_CM_DEFAULT_L3_SLOW_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut default_sata_81xx_clkdm: clockdomain = clockdomain {
    name: "default_clkdm",
    pwrdm: powerdomain { name: "default_pwrdm" },
    cm_inst: TI81XX_CM_DEFAULT_MOD,
    clkdm_offs: TI816X_CM_DEFAULT_SATA_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

/* 816x only */

static mut alwon_mpu_816x_clkdm: clockdomain = clockdomain {
    name: "alwon_mpu_clkdm",
    pwrdm: powerdomain { name: "alwon_pwrdm" },
    cm_inst: TI81XX_CM_ALWON_MOD,
    clkdm_offs: TI81XX_CM_ALWON_MPU_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut active_gem_816x_clkdm: clockdomain = clockdomain {
    name: "active_gem_clkdm",
    pwrdm: powerdomain { name: "active_pwrdm" },
    cm_inst: TI81XX_CM_ACTIVE_MOD,
    clkdm_offs: TI816X_CM_ACTIVE_GEM_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut ivahd0_816x_clkdm: clockdomain = clockdomain {
    name: "ivahd0_clkdm",
    pwrdm: powerdomain { name: "ivahd0_pwrdm" },
    cm_inst: TI816X_CM_IVAHD0_MOD,
    clkdm_offs: TI816X_CM_IVAHD0_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut ivahd1_816x_clkdm: clockdomain = clockdomain {
    name: "ivahd1_clkdm",
    pwrdm: powerdomain { name: "ivahd1_pwrdm" },
    cm_inst: TI816X_CM_IVAHD1_MOD,
    clkdm_offs: TI816X_CM_IVAHD1_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut ivahd2_816x_clkdm: clockdomain = clockdomain {
    name: "ivahd2_clkdm",
    pwrdm: powerdomain { name: "ivahd2_pwrdm" },
    cm_inst: TI816X_CM_IVAHD2_MOD,
    clkdm_offs: TI816X_CM_IVAHD2_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut sgx_816x_clkdm: clockdomain = clockdomain {
    name: "sgx_clkdm",
    pwrdm: powerdomain { name: "sgx_pwrdm" },
    cm_inst: TI81XX_CM_SGX_MOD,
    clkdm_offs: TI816X_CM_SGX_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut default_l3_med_816x_clkdm: clockdomain = clockdomain {
    name: "default_l3_med_clkdm",
    pwrdm: powerdomain { name: "default_pwrdm" },
    cm_inst: TI81XX_CM_DEFAULT_MOD,
    clkdm_offs: TI816X_CM_DEFAULT_L3_MED_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut default_ducati_816x_clkdm: clockdomain = clockdomain {
    name: "default_ducati_clkdm",
    pwrdm: powerdomain { name: "default_pwrdm" },
    cm_inst: TI81XX_CM_DEFAULT_MOD,
    clkdm_offs: TI816X_CM_DEFAULT_DUCATI_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut default_pci_816x_clkdm: clockdomain = clockdomain {
    name: "default_pci_clkdm",
    pwrdm: powerdomain { name: "default_pwrdm" },
    cm_inst: TI81XX_CM_DEFAULT_MOD,
    clkdm_offs: TI816X_CM_DEFAULT_PCI_CLKDM,
    flags: CLKDM_CAN_SWSUP,
};

static mut clockdomains_ti814x: [*mut clockdomain; 9] = [
    &raw mut alwon_l3_slow_81xx_clkdm,
    &raw mut alwon_l3_med_81xx_clkdm,
    &raw mut alwon_l3_fast_81xx_clkdm,
    &raw mut alwon_ethernet_81xx_clkdm,
    &raw mut mmu_81xx_clkdm,
    &raw mut mmu_cfg_81xx_clkdm,
    &raw mut default_l3_slow_81xx_clkdm,
    &raw mut default_sata_81xx_clkdm,
    core::ptr::null_mut(),
];

pub unsafe fn ti814x_clockdomains_init() {
    clkdm_register_platform_funcs(&am33xx_clkdm_operations);
    clkdm_register_clkdms(clockdomains_ti814x.as_mut_ptr());
    clkdm_complete_init();
}

static mut clockdomains_ti816x: [*mut clockdomain; 18] = [
    &raw mut alwon_mpu_816x_clkdm,
    &raw mut alwon_l3_slow_81xx_clkdm,
    &raw mut alwon_l3_med_81xx_clkdm,
    &raw mut alwon_l3_fast_81xx_clkdm,
    &raw mut alwon_ethernet_81xx_clkdm,
    &raw mut mmu_81xx_clkdm,
    &raw mut mmu_cfg_81xx_clkdm,
    &raw mut active_gem_816x_clkdm,
    &raw mut ivahd0_816x_clkdm,
    &raw mut ivahd1_816x_clkdm,
    &raw mut ivahd2_816x_clkdm,
    &raw mut sgx_816x_clkdm,
    &raw mut default_l3_med_816x_clkdm,
    &raw mut default_ducati_816x_clkdm,
    &raw mut default_pci_816x_clkdm,
    &raw mut default_l3_slow_81xx_clkdm,
    &raw mut default_sata_81xx_clkdm,
    core::ptr::null_mut(),
];

pub unsafe fn ti816x_clockdomains_init() {
    clkdm_register_platform_funcs(&am33xx_clkdm_operations);
    clkdm_register_clkdms(clockdomains_ti816x.as_mut_ptr());
    clkdm_complete_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
