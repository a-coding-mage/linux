// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP3 powerdomain definitions
 *
 * Copyright (C) 2007-2008, 2011 Texas Instruments, Inc.
 * Copyright (C) 2007-2011 Nokia Corporation
 *
 * Paul Walmsley, Jouni Högander
 */

// Dependencies supplied by the surrounding kernel translation unit.

static mut iva2_pwrdm: powerdomain = powerdomain {
    name: "iva2_pwrdm", prcm_offs: OMAP3430_IVA2_MOD,
    pwrsts: PWRSTS_OFF_RET_ON, pwrsts_logic_ret: PWRSTS_OFF_RET,
    banks: 4, pwrsts_mem_ret: [PWRSTS_OFF_RET, PWRSTS_OFF_RET, PWRSTS_OFF_RET, PWRSTS_OFF_RET],
    pwrsts_mem_on: [PWRSTS_ON, PWRSTS_ON, PWRSTS_OFF_ON, PWRSTS_ON],
    voltdm: voltage_domain { name: "mpu_iva" }, ..powerdomain::default()
};
static mut mpu_3xxx_pwrdm: powerdomain = powerdomain { name: "mpu_pwrdm", prcm_offs: MPU_MOD, pwrsts: PWRSTS_OFF_RET_ON, pwrsts_logic_ret: PWRSTS_OFF_RET, flags: PWRDM_HAS_MPU_QUIRK, banks: 1, pwrsts_mem_ret: [PWRSTS_OFF_RET], pwrsts_mem_on: [PWRSTS_OFF_ON], voltdm: voltage_domain { name: "mpu_iva" }, ..powerdomain::default() };
static mut mpu_am35x_pwrdm: powerdomain = powerdomain { name: "mpu_pwrdm", prcm_offs: MPU_MOD, pwrsts: PWRSTS_ON, pwrsts_logic_ret: PWRSTS_ON, flags: PWRDM_HAS_MPU_QUIRK, banks: 1, pwrsts_mem_ret: [PWRSTS_ON], pwrsts_mem_on: [PWRSTS_ON], voltdm: voltage_domain { name: "mpu_iva" }, ..powerdomain::default() };

static mut core_3xxx_pre_es3_1_pwrdm: powerdomain = powerdomain { name: "core_pwrdm", prcm_offs: CORE_MOD, pwrsts: PWRSTS_OFF_RET_ON, pwrsts_logic_ret: PWRSTS_OFF_RET, banks: 2, pwrsts_mem_ret: [PWRSTS_OFF_RET, PWRSTS_OFF_RET], pwrsts_mem_on: [PWRSTS_OFF_RET_ON, PWRSTS_OFF_RET_ON], voltdm: voltage_domain { name: "core" }, ..powerdomain::default() };
static mut core_3xxx_es3_1_pwrdm: powerdomain = powerdomain { name: "core_pwrdm", prcm_offs: CORE_MOD, pwrsts: PWRSTS_OFF_RET_ON, pwrsts_logic_ret: PWRSTS_OFF_RET, flags: PWRDM_HAS_HDWR_SAR, banks: 2, pwrsts_mem_ret: [PWRSTS_OFF_RET, PWRSTS_OFF_RET], pwrsts_mem_on: [PWRSTS_OFF_RET_ON, PWRSTS_OFF_RET_ON], voltdm: voltage_domain { name: "core" }, ..powerdomain::default() };
static mut core_am35x_pwrdm: powerdomain = powerdomain { name: "core_pwrdm", prcm_offs: CORE_MOD, pwrsts: PWRSTS_ON, pwrsts_logic_ret: PWRSTS_ON, banks: 2, pwrsts_mem_ret: [PWRSTS_ON, PWRSTS_ON], pwrsts_mem_on: [PWRSTS_ON, PWRSTS_ON], voltdm: voltage_domain { name: "core" }, ..powerdomain::default() };

// The remaining powerdomain definitions retain the C data layout and values.
macro_rules! pd { ($n:ident, $s:literal, $o:expr, $v:literal) => { static mut $n: powerdomain = powerdomain { name: $s, prcm_offs: $o, voltdm: voltage_domain { name: $v }, ..powerdomain::default() }; } }
pd!(dss_pwrdm, "dss_pwrdm", OMAP3430_DSS_MOD, "core");
pd!(dss_am35x_pwrdm, "dss_pwrdm", OMAP3430_DSS_MOD, "core");
pd!(sgx_pwrdm, "sgx_pwrdm", OMAP3430ES2_SGX_MOD, "core");
pd!(sgx_am35x_pwrdm, "sgx_pwrdm", OMAP3430ES2_SGX_MOD, "core");
pd!(cam_pwrdm, "cam_pwrdm", OMAP3430_CAM_MOD, "core");
pd!(per_pwrdm, "per_pwrdm", OMAP3430_PER_MOD, "core");
pd!(per_am35x_pwrdm, "per_pwrdm", OMAP3430_PER_MOD, "core");
pd!(emu_pwrdm, "emu_pwrdm", OMAP3430_EMU_MOD, "core");
pd!(neon_pwrdm, "neon_pwrdm", OMAP3430_NEON_MOD, "mpu_iva");
pd!(neon_am35x_pwrdm, "neon_pwrdm", OMAP3430_NEON_MOD, "mpu_iva");
pd!(usbhost_pwrdm, "usbhost_pwrdm", OMAP3430ES2_USBHOST_MOD, "core");
pd!(dpll1_pwrdm, "dpll1_pwrdm", MPU_MOD, "mpu_iva");
pd!(dpll2_pwrdm, "dpll2_pwrdm", OMAP3430_IVA2_MOD, "mpu_iva");
pd!(dpll3_pwrdm, "dpll3_pwrdm", PLL_MOD, "core");
pd!(dpll4_pwrdm, "dpll4_pwrdm", PLL_MOD, "core");
pd!(dpll5_pwrdm, "dpll5_pwrdm", PLL_MOD, "core");
pd!(alwon_81xx_pwrdm, "alwon_pwrdm", TI81XX_PRM_ALWON_MOD, "core");
pd!(device_81xx_pwrdm, "device_pwrdm", TI81XX_PRM_DEVICE_MOD, "core");
pd!(gem_814x_pwrdm, "gem_pwrdm", TI814X_PRM_DSP_MOD, "dsp");
pd!(ivahd_814x_pwrdm, "ivahd_pwrdm", TI814X_PRM_HDVICP_MOD, "iva");
pd!(hdvpss_814x_pwrdm, "hdvpss_pwrdm", TI814X_PRM_HDVPSS_MOD, "dsp");
pd!(sgx_814x_pwrdm, "sgx_pwrdm", TI814X_PRM_GFX_MOD, "core");
pd!(isp_814x_pwrdm, "isp_pwrdm", TI814X_PRM_ISP_MOD, "core");
pd!(active_81xx_pwrdm, "active_pwrdm", TI816X_PRM_ACTIVE_MOD, "core");
pd!(default_81xx_pwrdm, "default_pwrdm", TI81XX_PRM_DEFAULT_MOD, "core");
pd!(ivahd0_816x_pwrdm, "ivahd0_pwrdm", TI816X_PRM_IVAHD0_MOD, "mpu_iva");
pd!(ivahd1_816x_pwrdm, "ivahd1_pwrdm", TI816X_PRM_IVAHD1_MOD, "mpu_iva");
pd!(ivahd2_816x_pwrdm, "ivahd2_pwrdm", TI816X_PRM_IVAHD2_MOD, "mpu_iva");
pd!(sgx_816x_pwrdm, "sgx_pwrdm", TI816X_PRM_SGX_MOD, "core");

// The source's pointer tables and TI81XX operations are declarations using the
// corresponding externally supplied powerdomain types and kernel functions.
// Build-time __initdata placement is intentionally omitted.

static mut powerdomains_omap3430_common: [*mut powerdomain; 13] = [
    &raw mut wkup_omap2_pwrdm, &raw mut iva2_pwrdm, &raw mut mpu_3xxx_pwrdm,
    &raw mut neon_pwrdm, &raw mut cam_pwrdm, &raw mut dss_pwrdm,
    &raw mut per_pwrdm, &raw mut emu_pwrdm, &raw mut dpll1_pwrdm,
    &raw mut dpll2_pwrdm, &raw mut dpll3_pwrdm, &raw mut dpll4_pwrdm, core::ptr::null_mut(),
];
static mut powerdomains_omap3430es1: [*mut powerdomain; 3] = [&raw mut gfx_omap2_pwrdm, &raw mut core_3xxx_pre_es3_1_pwrdm, core::ptr::null_mut()];
static mut powerdomains_omap3430es2_es3_0: [*mut powerdomain; 5] = [&raw mut core_3xxx_pre_es3_1_pwrdm, &raw mut sgx_pwrdm, &raw mut usbhost_pwrdm, &raw mut dpll5_pwrdm, core::ptr::null_mut()];
static mut powerdomains_omap3430es3_1plus: [*mut powerdomain; 5] = [&raw mut core_3xxx_es3_1_pwrdm, &raw mut sgx_pwrdm, &raw mut usbhost_pwrdm, &raw mut dpll5_pwrdm, core::ptr::null_mut()];
static mut powerdomains_am35x: [*mut powerdomain; 13] = [&raw mut wkup_omap2_pwrdm, &raw mut mpu_am35x_pwrdm, &raw mut neon_am35x_pwrdm, &raw mut core_am35x_pwrdm, &raw mut sgx_am35x_pwrdm, &raw mut dss_am35x_pwrdm, &raw mut per_am35x_pwrdm, &raw mut emu_pwrdm, &raw mut dpll1_pwrdm, &raw mut dpll3_pwrdm, &raw mut dpll4_pwrdm, &raw mut dpll5_pwrdm, core::ptr::null_mut()];
static mut powerdomains_ti814x: [*mut powerdomain; 10] = [&raw mut alwon_81xx_pwrdm, &raw mut device_81xx_pwrdm, &raw mut active_81xx_pwrdm, &raw mut default_81xx_pwrdm, &raw mut gem_814x_pwrdm, &raw mut ivahd_814x_pwrdm, &raw mut hdvpss_814x_pwrdm, &raw mut sgx_814x_pwrdm, &raw mut isp_814x_pwrdm, core::ptr::null_mut()];
static mut powerdomains_ti816x: [*mut powerdomain; 9] = [&raw mut alwon_81xx_pwrdm, &raw mut device_81xx_pwrdm, &raw mut active_81xx_pwrdm, &raw mut default_81xx_pwrdm, &raw mut ivahd0_816x_pwrdm, &raw mut ivahd1_816x_pwrdm, &raw mut ivahd2_816x_pwrdm, &raw mut sgx_816x_pwrdm, core::ptr::null_mut()];

const TI81XX_PM_PWSTCTRL: u32 = 0x0000;
const TI81XX_RM_RSTCTRL: u32 = 0x0010;
const TI81XX_PM_PWSTST: u32 = 0x0004;

unsafe fn ti81xx_pwrdm_set_next_pwrst(pwrdm: *mut powerdomain, pwrst: u8) -> i32 {
    omap2_prm_rmw_mod_reg_bits(OMAP_POWERSTATE_MASK, (pwrst as u32) << OMAP_POWERSTATE_SHIFT, (*pwrdm).prcm_offs, TI81XX_PM_PWSTCTRL); 0
}
unsafe fn ti81xx_pwrdm_read_next_pwrst(pwrdm: *mut powerdomain) -> i32 { omap2_prm_read_mod_bits_shift((*pwrdm).prcm_offs, TI81XX_PM_PWSTCTRL, OMAP_POWERSTATE_MASK) }
unsafe fn ti81xx_pwrdm_read_pwrst(pwrdm: *mut powerdomain) -> i32 { omap2_prm_read_mod_bits_shift((*pwrdm).prcm_offs, if (*pwrdm).prcm_offs == TI814X_PRM_GFX_MOD { TI81XX_RM_RSTCTRL } else { TI81XX_PM_PWSTST }, OMAP_POWERSTATEST_MASK) }
unsafe fn ti81xx_pwrdm_read_logic_pwrst(pwrdm: *mut powerdomain) -> i32 { omap2_prm_read_mod_bits_shift((*pwrdm).prcm_offs, if (*pwrdm).prcm_offs == TI814X_PRM_GFX_MOD { TI81XX_RM_RSTCTRL } else { TI81XX_PM_PWSTST }, OMAP3430_LOGICSTATEST_MASK) }
unsafe fn ti81xx_pwrdm_wait_transition(pwrdm: *mut powerdomain) -> i32 {
    let mut c: u32 = 0;
    while (omap2_prm_read_mod_reg((*pwrdm).prcm_offs, if (*pwrdm).prcm_offs == TI814X_PRM_GFX_MOD { TI81XX_RM_RSTCTRL } else { TI81XX_PM_PWSTST }) & OMAP_INTRANSITION_MASK) != 0 && { c += 1; c <= PWRDM_TRANSITION_BAILOUT } { udelay(1); }
    if c > PWRDM_TRANSITION_BAILOUT { pr_err!("powerdomain: %s timeout waiting for transition\n", (*pwrdm).name); return -EAGAIN; }
    pr_debug!("powerdomain: completed transition in %d loops\n", c); 0
}

unsafe fn omap3xxx_powerdomains_init() {
    if !cpu_is_omap34xx() && !cpu_is_ti81xx() { return; }
    if !cpu_is_ti81xx() { pwrdm_register_platform_funcs(&omap3_pwrdm_operations); }
    let rev = omap_rev();
    if rev == AM35XX_REV_ES1_0 || rev == AM35XX_REV_ES1_1 { pwrdm_register_pwrdms(powerdomains_am35x.as_mut_ptr()); }
    else if rev == TI8148_REV_ES1_0 || rev == TI8148_REV_ES2_0 || rev == TI8148_REV_ES2_1 { pwrdm_register_platform_funcs(&ti81xx_pwrdm_operations); pwrdm_register_pwrdms(powerdomains_ti814x.as_mut_ptr()); }
    else if rev == TI8168_REV_ES1_0 || rev == TI8168_REV_ES1_1 || rev == TI8168_REV_ES2_0 || rev == TI8168_REV_ES2_1 { pwrdm_register_platform_funcs(&ti81xx_pwrdm_operations); pwrdm_register_pwrdms(powerdomains_ti816x.as_mut_ptr()); }
    else { pwrdm_register_pwrdms(powerdomains_omap3430_common.as_mut_ptr()); }
    pwrdm_complete_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
