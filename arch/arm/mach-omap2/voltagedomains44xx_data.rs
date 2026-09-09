// SPDX-License-Identifier: GPL-2.0-only
/* OMAP3/OMAP4 Voltage Management Routines */
static omap4_vdd_mpu_vfsm: omap_vfsm_instance = omap_vfsm_instance { voltsetup_reg: OMAP4_PRM_VOLTSETUP_MPU_RET_SLEEP_OFFSET, voltsetup_off_reg: OMAP4_PRM_VOLTSETUP_MPU_OFF_OFFSET };
static omap4_vdd_iva_vfsm: omap_vfsm_instance = omap_vfsm_instance { voltsetup_reg: OMAP4_PRM_VOLTSETUP_IVA_RET_SLEEP_OFFSET, voltsetup_off_reg: OMAP4_PRM_VOLTSETUP_IVA_OFF_OFFSET };
static omap4_vdd_core_vfsm: omap_vfsm_instance = omap_vfsm_instance { voltsetup_reg: OMAP4_PRM_VOLTSETUP_CORE_RET_SLEEP_OFFSET, voltsetup_off_reg: OMAP4_PRM_VOLTSETUP_CORE_OFF_OFFSET };
static mut omap4_voltdm_mpu: voltagedomain = voltagedomain { name: "mpu", scalable: true, read: omap4_prm_vcvp_read, write: omap4_prm_vcvp_write, rmw: omap4_prm_vcvp_rmw, vc: &raw const omap4_vc_mpu, vfsm: &raw const omap4_vdd_mpu_vfsm, vp: &raw const omap4_vp_mpu };
static mut omap4_voltdm_iva: voltagedomain = voltagedomain { name: "iva", scalable: true, read: omap4_prm_vcvp_read, write: omap4_prm_vcvp_write, rmw: omap4_prm_vcvp_rmw, vc: &raw const omap4_vc_iva, vfsm: &raw const omap4_vdd_iva_vfsm, vp: &raw const omap4_vp_iva };
static mut omap4_voltdm_core: voltagedomain = voltagedomain { name: "core", scalable: true, read: omap4_prm_vcvp_read, write: omap4_prm_vcvp_write, rmw: omap4_prm_vcvp_rmw, vc: &raw const omap4_vc_core, vfsm: &raw const omap4_vdd_core_vfsm, vp: &raw const omap4_vp_core };
static mut omap4_voltdm_wkup: voltagedomain = voltagedomain { name: "wakeup" };
static mut voltagedomains_omap4: [*mut voltagedomain; 5] = [&raw mut omap4_voltdm_mpu, &raw mut omap4_voltdm_iva, &raw mut omap4_voltdm_core, &raw mut omap4_voltdm_wkup, core::ptr::null_mut()];
static sys_clk_name: &str = "sys_clkin_ck";
pub unsafe extern "C" fn omap44xx_voltagedomains_init() {
    #[cfg(CONFIG_PM_OPP)] { if cpu_is_omap443x() { omap4_voltdm_mpu.volt_data = omap443x_vdd_mpu_volt_data; omap4_voltdm_iva.volt_data = omap443x_vdd_iva_volt_data; omap4_voltdm_core.volt_data = omap443x_vdd_core_volt_data; } else if cpu_is_omap446x() { omap4_voltdm_mpu.volt_data = omap446x_vdd_mpu_volt_data; omap4_voltdm_iva.volt_data = omap446x_vdd_iva_volt_data; omap4_voltdm_core.volt_data = omap446x_vdd_core_volt_data; } }
    omap4_voltdm_mpu.vp_param = &raw const omap4_mpu_vp_data; omap4_voltdm_iva.vp_param = &raw const omap4_iva_vp_data; omap4_voltdm_core.vp_param = &raw const omap4_core_vp_data;
    omap4_voltdm_mpu.vc_param = &raw const omap4_mpu_vc_data; omap4_voltdm_iva.vc_param = &raw const omap4_iva_vc_data; omap4_voltdm_core.vc_param = &raw const omap4_core_vc_data;
    let mut i = 0; loop { let voltdm = voltagedomains_omap4[i]; if voltdm.is_null() { break; } (*voltdm).sys_clk.name = sys_clk_name; i += 1; }
    voltdm_init(voltagedomains_omap4.as_mut_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
