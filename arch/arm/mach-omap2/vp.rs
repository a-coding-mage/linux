// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding OMAP voltage-management code.

unsafe fn _vp_set_init_voltage(voltdm: *mut voltagedomain, volt: u32) -> u32 {
    let vp = (*voltdm).vp;
    let vsel: i8 = ((*voltdm).pmic.unwrap()).uv_to_vsel.unwrap()(volt);

    let mut vpconfig = ((*voltdm).read.unwrap())((*vp).vpconfig);
    vpconfig &= !((*vp).common.vpconfig_initvoltage_mask
        | (*vp).common.vpconfig_forceupdate
        | (*vp).common.vpconfig_initvdd);
    vpconfig |= (vsel as u32) << ffs((*vp).common.vpconfig_initvoltage_mask);
    ((*voltdm).write.unwrap())(vpconfig, (*vp).vpconfig);

    // Trigger initVDD value copy to voltage processor
    ((*voltdm).write.unwrap())(vpconfig | (*vp).common.vpconfig_initvdd, (*vp).vpconfig);

    // Clear initVDD copy trigger bit
    ((*voltdm).write.unwrap())(vpconfig, (*vp).vpconfig);

    vpconfig
}

// Generic voltage init functions
pub unsafe extern "C" fn omap_vp_init(voltdm: *mut voltagedomain) {
    let vp = (*voltdm).vp;
    let (mut val, sys_clk_rate, timeout, waittime): (u32, u32, u32, u32);
    let (mut vddmin, mut vddmax, vstepmin, vstepmax): (u32, u32, u32, u32);

    if (*voltdm).pmic.is_none() || (*(*voltdm).pmic.as_ref().unwrap()).uv_to_vsel.is_none() {
        pr_err!("{}: No PMIC info for vdd_{}\n", "omap_vp_init", (*voltdm).name);
        return;
    }
    if (*voltdm).read.is_none() || (*voltdm).write.is_none() {
        pr_err!("{}: No read/write API for accessing vdd_{} regs\n", "omap_vp_init", (*voltdm).name);
        return;
    }

    (*vp).enabled = false;
    // Divide to avoid overflow
    sys_clk_rate = (*voltdm).sys_clk.rate / 1000;
    timeout = (sys_clk_rate * (*(*voltdm).pmic.as_ref().unwrap()).vp_timeout_us) / 1000;
    vddmin = core::cmp::max((*voltdm).vp_param.vddmin, (*(*voltdm).pmic.as_ref().unwrap()).vddmin);
    vddmax = core::cmp::min((*voltdm).vp_param.vddmax, (*(*voltdm).pmic.as_ref().unwrap()).vddmax);
    let pmic = (*voltdm).pmic.as_ref().unwrap();
    vddmin = pmic.uv_to_vsel.unwrap()(vddmin) as u32;
    vddmax = pmic.uv_to_vsel.unwrap()(vddmax) as u32;
    waittime = div_round_up(pmic.step_size * sys_clk_rate, 1000 * pmic.slew_rate);
    vstepmin = pmic.vp_vstepmin;
    vstepmax = pmic.vp_vstepmax;

    // VP_CONFIG: error gain is not set here; it is updated on each scale, based on OPP.
    val = (pmic.vp_erroroffset << ffs((*vp).common.vpconfig_erroroffset_mask))
        | (*vp).common.vpconfig_timeouten;
    (*voltdm).write.unwrap()(val, (*vp).vpconfig);
    val = (waittime << (*vp).common.vstepmin_smpswaittimemin_shift)
        | (vstepmin << (*vp).common.vstepmin_stepmin_shift);
    (*voltdm).write.unwrap()(val, (*vp).vstepmin);
    val = (vstepmax << (*vp).common.vstepmax_stepmax_shift)
        | (waittime << (*vp).common.vstepmax_smpswaittimemax_shift);
    (*voltdm).write.unwrap()(val, (*vp).vstepmax);
    val = (vddmax << (*vp).common.vlimitto_vddmax_shift)
        | (vddmin << (*vp).common.vlimitto_vddmin_shift)
        | (timeout << (*vp).common.vlimitto_timeout_shift);
    (*voltdm).write.unwrap()(val, (*vp).vlimitto);
}

pub unsafe extern "C" fn omap_vp_update_errorgain(voltdm: *mut voltagedomain, target_volt: c_ulong) -> c_int {
    if (*voltdm).vp.is_null() { return -EINVAL; }
    let volt_data = omap_voltage_get_voltdata(voltdm, target_volt);
    if is_err(volt_data) { return -EINVAL; }
    let common = (*(*voltdm).vp).common;
    (*voltdm).rmw.unwrap()(common.vpconfig_errorgain_mask,
        (*volt_data).vp_errgain << ffs(common.vpconfig_errorgain_mask), (*(*voltdm).vp).vpconfig);
    0
}

pub unsafe extern "C" fn omap_vp_forceupdate_scale(voltdm: *mut voltagedomain, target_volt: c_ulong) -> c_int {
    let vp = (*voltdm).vp;
    let mut target_vsel = 0u8;
    let mut current_vsel = 0u8;
    let ret = omap_vc_pre_scale(voltdm, target_volt, &mut target_vsel, &mut current_vsel);
    if ret != 0 { return ret; }
    let mut timeout = 0;
    while { timeout += 1; timeout } < VP_TRANXDONE_TIMEOUT {
        (*vp).common.ops.clear_txdone((*vp).id);
        if !((*vp).common.ops.check_txdone)((*vp).id) { break; }
        udelay(1);
    }
    if timeout >= VP_TRANXDONE_TIMEOUT { pr_warn!("{}: vdd_{} TRANXDONE timeout exceeded. Voltage change aborted\n", "omap_vp_forceupdate_scale", (*voltdm).name); return -ETIMEDOUT; }
    let vpconfig = _vp_set_init_voltage(voltdm, target_volt as u32);
    (*voltdm).write.unwrap()(vpconfig | (*vp).common.vpconfig_forceupdate, (*vp).vpconfig);
    timeout = 0;
    omap_test_timeout!((*vp).common.ops.check_txdone((*vp).id), VP_TRANXDONE_TIMEOUT, timeout);
    if timeout >= VP_TRANXDONE_TIMEOUT { pr_err!("{}: vdd_{} TRANXDONE timeout exceeded. TRANXDONE never got set after the voltage update\n", "omap_vp_forceupdate_scale", (*voltdm).name); }
    omap_vc_post_scale(voltdm, target_volt, target_vsel, current_vsel);
    timeout = 0;
    while { timeout += 1; timeout } < VP_TRANXDONE_TIMEOUT { (*vp).common.ops.clear_txdone((*vp).id); if !((*vp).common.ops.check_txdone)((*vp).id) { break; } udelay(1); }
    if timeout >= VP_TRANXDONE_TIMEOUT { pr_warn!("{}: vdd_{} TRANXDONE timeout exceeded while trying to clear the TRANXDONE status\n", "omap_vp_forceupdate_scale", (*voltdm).name); }
    (*voltdm).write.unwrap()(vpconfig, (*vp).vpconfig);
    0
}

pub unsafe extern "C" fn omap_vp_enable(voltdm: *mut voltagedomain) {
    if is_err_or_null(voltdm) { pr_warn!("{}: VDD specified does not exist!\n", "omap_vp_enable"); return; }
    let vp = (*voltdm).vp;
    if (*voltdm).read.is_none() || (*voltdm).write.is_none() { pr_err!("{}: No read/write API for accessing vdd_{} regs\n", "omap_vp_enable", (*voltdm).name); return; }
    if (*vp).enabled { return; }
    let volt = voltdm_get_voltage(voltdm); if volt == 0 { pr_warn!("{}: unable to find current voltage for {}\n", "omap_vp_enable", (*voltdm).name); return; }
    let mut vpconfig = _vp_set_init_voltage(voltdm, volt);
    vpconfig |= (*vp).common.vpconfig_vpenable;
    (*voltdm).write.unwrap()(vpconfig, (*vp).vpconfig); (*vp).enabled = true;
}

pub unsafe extern "C" fn omap_vp_disable(voltdm: *mut voltagedomain) {
    if is_err_or_null(voltdm) { pr_warn!("{}: VDD specified does not exist!\n", "omap_vp_disable"); return; }
    let vp = (*voltdm).vp;
    if (*voltdm).read.is_none() || (*voltdm).write.is_none() { pr_err!("{}: No read/write API for accessing vdd_{} regs\n", "omap_vp_disable", (*voltdm).name); return; }
    if !(*vp).enabled { pr_warn!("{}: Trying to disable VP for vdd_{} when it is already disabled\n", "omap_vp_disable", (*voltdm).name); return; }
    let mut vpconfig = (*voltdm).read.unwrap()((*vp).vpconfig);
    vpconfig &= !(*vp).common.vpconfig_vpenable;
    (*voltdm).write.unwrap()(vpconfig, (*vp).vpconfig);
    let mut timeout = 0;
    omap_test_timeout!((*voltdm).read.unwrap()((*vp).vstatus), VP_IDLE_TIMEOUT, timeout);
    if timeout >= VP_IDLE_TIMEOUT { pr_warn!("{}: vdd_{} idle timedout\n", "omap_vp_disable", (*voltdm).name); }
    (*vp).enabled = false;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
