// SPDX-License-Identifier: GPL-2.0-only
/* OMAP54XX Clock domains framework; automatically generated from hardware databases. */

// Dependencies supplied by the surrounding kernel translation unit.

static mut c2c_wkup_sleep_deps: [clkdm_dep; 7] = [dep!("abe_clkdm"), dep!("emif_clkdm"), dep!("iva_clkdm"), dep!("l3init_clkdm"), dep!("l3main1_clkdm"), dep!("l3main2_clkdm"), dep!("l4cfg_clkdm")];
static mut cam_wkup_sleep_deps: [clkdm_dep; 4] = [dep!("emif_clkdm"), dep!("iva_clkdm"), dep!("l3main1_clkdm"), dep!("")];
static mut dma_wkup_sleep_deps: [clkdm_dep; 12] = [dep!("abe_clkdm"), dep!("dss_clkdm"), dep!("emif_clkdm"), dep!("ipu_clkdm"), dep!("iva_clkdm"), dep!("l3init_clkdm"), dep!("l3main1_clkdm"), dep!("l4cfg_clkdm"), dep!("l4per_clkdm"), dep!("l4sec_clkdm"), dep!("wkupaon_clkdm"), dep!("")];
static mut dsp_wkup_sleep_deps: [clkdm_dep; 10] = [dep!("abe_clkdm"), dep!("emif_clkdm"), dep!("iva_clkdm"), dep!("l3init_clkdm"), dep!("l3main1_clkdm"), dep!("l3main2_clkdm"), dep!("l4cfg_clkdm"), dep!("l4per_clkdm"), dep!("wkupaon_clkdm"), dep!("")];
static mut dss_wkup_sleep_deps: [clkdm_dep; 4] = [dep!("emif_clkdm"), dep!("iva_clkdm"), dep!("l3main2_clkdm"), dep!("")];
static mut gpu_wkup_sleep_deps: [clkdm_dep; 4] = [dep!("emif_clkdm"), dep!("iva_clkdm"), dep!("l3main1_clkdm"), dep!("")];
static mut ipu_wkup_sleep_deps: [clkdm_dep; 14] = [dep!("abe_clkdm"), dep!("dsp_clkdm"), dep!("dss_clkdm"), dep!("emif_clkdm"), dep!("gpu_clkdm"), dep!("iva_clkdm"), dep!("l3init_clkdm"), dep!("l3main1_clkdm"), dep!("l3main2_clkdm"), dep!("l4cfg_clkdm"), dep!("l4per_clkdm"), dep!("l4sec_clkdm"), dep!("wkupaon_clkdm"), dep!("")];
static mut iva_wkup_sleep_deps: [clkdm_dep; 3] = [dep!("emif_clkdm"), dep!("l3main1_clkdm"), dep!("")];
static mut l3init_wkup_sleep_deps: [clkdm_dep; 8] = [dep!("abe_clkdm"), dep!("emif_clkdm"), dep!("iva_clkdm"), dep!("l4cfg_clkdm"), dep!("l4per_clkdm"), dep!("l4sec_clkdm"), dep!("wkupaon_clkdm"), dep!("")];
static mut l4sec_wkup_sleep_deps: [clkdm_dep; 4] = [dep!("emif_clkdm"), dep!("l3main1_clkdm"), dep!("l4per_clkdm"), dep!("")];
static mut mipiext_wkup_sleep_deps: [clkdm_dep; 9] = [dep!("abe_clkdm"), dep!("emif_clkdm"), dep!("iva_clkdm"), dep!("l3init_clkdm"), dep!("l3main1_clkdm"), dep!("l3main2_clkdm"), dep!("l4cfg_clkdm"), dep!("l4per_clkdm"), dep!("")];
static mut mpu_wkup_sleep_deps: [clkdm_dep; 15] = [dep!("abe_clkdm"), dep!("dsp_clkdm"), dep!("dss_clkdm"), dep!("emif_clkdm"), dep!("gpu_clkdm"), dep!("ipu_clkdm"), dep!("iva_clkdm"), dep!("l3init_clkdm"), dep!("l3main1_clkdm"), dep!("l3main2_clkdm"), dep!("l4cfg_clkdm"), dep!("l4per_clkdm"), dep!("l4sec_clkdm"), dep!("wkupaon_clkdm"), dep!("")];

macro_rules! dep { ($n:expr) => { clkdm_dep { clkdm_name: $n } }; }
macro_rules! cd { ($n:ident, $name:expr, $pw:expr, $part:expr, $inst:expr, $off:expr, $flags:expr) => { static mut $n: clockdomain = clockdomain { name: $name, pwrdm: powerdomain { name: $pw }, prcm_partition: $part, cm_inst: $inst, clkdm_offs: $off, dep_bit: 0, wkdep_srcs: core::ptr::null_mut(), sleepdep_srcs: core::ptr::null_mut(), flags: $flags }; }; }

cd!(l4sec_54xx_clkdm, "l4sec_clkdm", "core_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_CORE_INST, OMAP54XX_CM_CORE_CORE_L4SEC_CDOFFS, CLKDM_CAN_SWSUP);
cd!(iva_54xx_clkdm, "iva_clkdm", "iva_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_IVA_INST, OMAP54XX_CM_CORE_IVA_IVA_CDOFFS, CLKDM_CAN_HWSUP_SWSUP);
cd!(mipiext_54xx_clkdm, "mipiext_clkdm", "core_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_CORE_INST, OMAP54XX_CM_CORE_CORE_MIPIEXT_CDOFFS, CLKDM_CAN_FORCE_WAKEUP | CLKDM_CAN_HWSUP);
cd!(l3main2_54xx_clkdm, "l3main2_clkdm", "core_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_CORE_INST, OMAP54XX_CM_CORE_CORE_L3MAIN2_CDOFFS, CLKDM_CAN_HWSUP);
cd!(l3main1_54xx_clkdm, "l3main1_clkdm", "core_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_CORE_INST, OMAP54XX_CM_CORE_CORE_L3MAIN1_CDOFFS, CLKDM_CAN_HWSUP);
cd!(custefuse_54xx_clkdm, "custefuse_clkdm", "custefuse_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_CUSTEFUSE_INST, OMAP54XX_CM_CORE_CUSTEFUSE_CUSTEFUSE_CDOFFS, CLKDM_CAN_FORCE_WAKEUP | CLKDM_CAN_HWSUP);
cd!(ipu_54xx_clkdm, "ipu_clkdm", "core_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_CORE_INST, OMAP54XX_CM_CORE_CORE_IPU_CDOFFS, CLKDM_CAN_HWSUP_SWSUP);
cd!(l4cfg_54xx_clkdm, "l4cfg_clkdm", "core_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_CORE_INST, OMAP54XX_CM_CORE_CORE_L4CFG_CDOFFS, CLKDM_CAN_HWSUP);
cd!(abe_54xx_clkdm, "abe_clkdm", "abe_pwrdm", OMAP54XX_CM_CORE_AON_PARTITION, OMAP54XX_CM_CORE_AON_ABE_INST, OMAP54XX_CM_CORE_AON_ABE_ABE_CDOFFS, CLKDM_CAN_HWSUP_SWSUP);
cd!(dss_54xx_clkdm, "dss_clkdm", "dss_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_DSS_INST, OMAP54XX_CM_CORE_DSS_DSS_CDOFFS, CLKDM_CAN_HWSUP_SWSUP);
cd!(dsp_54xx_clkdm, "dsp_clkdm", "dsp_pwrdm", OMAP54XX_CM_CORE_AON_PARTITION, OMAP54XX_CM_CORE_AON_DSP_INST, OMAP54XX_CM_CORE_AON_DSP_DSP_CDOFFS, CLKDM_CAN_HWSUP_SWSUP);
cd!(c2c_54xx_clkdm, "c2c_clkdm", "core_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_CORE_INST, OMAP54XX_CM_CORE_CORE_C2C_CDOFFS, CLKDM_CAN_FORCE_WAKEUP | CLKDM_CAN_HWSUP);
cd!(l4per_54xx_clkdm, "l4per_clkdm", "core_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_CORE_INST, OMAP54XX_CM_CORE_CORE_L4PER_CDOFFS, CLKDM_CAN_HWSUP_SWSUP);
cd!(gpu_54xx_clkdm, "gpu_clkdm", "gpu_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_GPU_INST, OMAP54XX_CM_CORE_GPU_GPU_CDOFFS, CLKDM_CAN_HWSUP_SWSUP);
cd!(wkupaon_54xx_clkdm, "wkupaon_clkdm", "wkupaon_pwrdm", OMAP54XX_PRM_PARTITION, OMAP54XX_PRM_WKUPAON_CM_INST, OMAP54XX_PRM_WKUPAON_CM_WKUPAON_CDOFFS, CLKDM_CAN_FORCE_WAKEUP | CLKDM_CAN_HWSUP);
cd!(mpu0_54xx_clkdm, "mpu0_clkdm", "cpu0_pwrdm", OMAP54XX_PRCM_MPU_PARTITION, OMAP54XX_PRCM_MPU_CM_C0_INST, OMAP54XX_PRCM_MPU_CM_C0_CPU0_CDOFFS, CLKDM_CAN_FORCE_WAKEUP | CLKDM_CAN_HWSUP);
cd!(mpu1_54xx_clkdm, "mpu1_clkdm", "cpu1_pwrdm", OMAP54XX_PRCM_MPU_PARTITION, OMAP54XX_PRCM_MPU_CM_C1_INST, OMAP54XX_PRCM_MPU_CM_C1_CPU1_CDOFFS, CLKDM_CAN_FORCE_WAKEUP | CLKDM_CAN_HWSUP);
cd!(coreaon_54xx_clkdm, "coreaon_clkdm", "coreaon_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_COREAON_INST, OMAP54XX_CM_CORE_COREAON_COREAON_CDOFFS, CLKDM_CAN_FORCE_WAKEUP | CLKDM_CAN_HWSUP);
cd!(mpu_54xx_clkdm, "mpu_clkdm", "mpu_pwrdm", OMAP54XX_CM_CORE_AON_PARTITION, OMAP54XX_CM_CORE_AON_MPU_INST, OMAP54XX_CM_CORE_AON_MPU_MPU_CDOFFS, CLKDM_CAN_FORCE_WAKEUP | CLKDM_CAN_HWSUP);
cd!(l3init_54xx_clkdm, "l3init_clkdm", "l3init_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_L3INIT_INST, OMAP54XX_CM_CORE_L3INIT_L3INIT_CDOFFS, CLKDM_CAN_HWSUP_SWSUP);
cd!(dma_54xx_clkdm, "dma_clkdm", "core_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_CORE_INST, OMAP54XX_CM_CORE_CORE_DMA_CDOFFS, CLKDM_CAN_FORCE_WAKEUP | CLKDM_CAN_HWSUP);
cd!(l3instr_54xx_clkdm, "l3instr_clkdm", "core_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_CORE_INST, OMAP54XX_CM_CORE_CORE_L3INSTR_CDOFFS, 0);
cd!(emif_54xx_clkdm, "emif_clkdm", "core_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_CORE_INST, OMAP54XX_CM_CORE_CORE_EMIF_CDOFFS, CLKDM_CAN_FORCE_WAKEUP | CLKDM_CAN_HWSUP);
cd!(emu_54xx_clkdm, "emu_clkdm", "emu_pwrdm", OMAP54XX_PRM_PARTITION, OMAP54XX_PRM_EMU_CM_INST, OMAP54XX_PRM_EMU_CM_EMU_CDOFFS, CLKDM_CAN_FORCE_WAKEUP | CLKDM_CAN_HWSUP);
cd!(cam_54xx_clkdm, "cam_clkdm", "cam_pwrdm", OMAP54XX_CM_CORE_PARTITION, OMAP54XX_CM_CORE_CAM_INST, OMAP54XX_CM_CORE_CAM_CAM_CDOFFS, CLKDM_CAN_HWSUP_SWSUP);

static mut clockdomains_omap54xx: [*mut clockdomain; 26] = [
    &raw mut l4sec_54xx_clkdm, &raw mut iva_54xx_clkdm, &raw mut mipiext_54xx_clkdm,
    &raw mut l3main2_54xx_clkdm, &raw mut l3main1_54xx_clkdm, &raw mut custefuse_54xx_clkdm,
    &raw mut ipu_54xx_clkdm, &raw mut l4cfg_54xx_clkdm, &raw mut abe_54xx_clkdm,
    &raw mut dss_54xx_clkdm, &raw mut dsp_54xx_clkdm, &raw mut c2c_54xx_clkdm,
    &raw mut l4per_54xx_clkdm, &raw mut gpu_54xx_clkdm, &raw mut wkupaon_54xx_clkdm,
    &raw mut mpu0_54xx_clkdm, &raw mut mpu1_54xx_clkdm, &raw mut coreaon_54xx_clkdm,
    &raw mut mpu_54xx_clkdm, &raw mut l3init_54xx_clkdm, &raw mut dma_54xx_clkdm,
    &raw mut l3instr_54xx_clkdm, &raw mut emif_54xx_clkdm, &raw mut emu_54xx_clkdm,
    &raw mut cam_54xx_clkdm, core::ptr::null_mut(),
];

pub unsafe fn omap54xx_clockdomains_init() {
    clkdm_register_platform_funcs(&omap4_clkdm_operations);
    clkdm_register_clkdms(clockdomains_omap54xx.as_mut_ptr());
    clkdm_complete_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
