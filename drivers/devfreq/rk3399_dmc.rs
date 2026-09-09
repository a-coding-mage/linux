// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016, Fuzhou Rockchip Electronics Co., Ltd.
 * Author: Lin Huang <hl@rock-chips.com>
 */

// Translated from rk3399_dmc.c. Kernel-provided types, constants, and
// functions referenced below are supplied by the surrounding Rust bindings.

const RK3399_SET_ODT_PD_0_SR_IDLE: u32 = 0xff;
const RK3399_SET_ODT_PD_0_SR_MC_GATE_IDLE: u32 = 0xff00;
const RK3399_SET_ODT_PD_0_STANDBY_IDLE: u32 = 0xffff0000;
const RK3399_SET_ODT_PD_1_PD_IDLE: u32 = 0xfff;
const RK3399_SET_ODT_PD_1_SRPD_LITE_IDLE: u32 = 0x0fff0000;
const RK3399_SET_ODT_PD_2_ODT_ENABLE: u32 = 1;

#[repr(C)]
pub struct Rk3399Dmcfreq {
    pub dev: *mut device, pub devfreq: *mut devfreq,
    pub profile: devfreq_dev_profile, pub ondemand_data: devfreq_simple_ondemand_data,
    pub dmc_clk: *mut clk, pub edev: *mut devfreq_event_dev, pub lock: mutex,
    pub vdd_center: *mut regulator, pub regmap_pmu: *mut regmap,
    pub rate: usize, pub target_rate: usize, pub volt: usize, pub target_volt: usize,
    pub odt_dis_freq: u32, pub pd_idle_ns: u32, pub sr_idle_ns: u32,
    pub sr_mc_gate_idle_ns: u32, pub srpd_lite_idle_ns: u32, pub standby_idle_ns: u32,
    pub ddr3_odt_dis_freq: u32, pub lpddr3_odt_dis_freq: u32, pub lpddr4_odt_dis_freq: u32,
    pub pd_idle_dis_freq: u32, pub sr_idle_dis_freq: u32,
    pub sr_mc_gate_idle_dis_freq: u32, pub srpd_lite_idle_dis_freq: u32,
    pub standby_idle_dis_freq: u32,
}

unsafe fn rk3399_dmcfreq_target(dev: *mut device, freq: *mut usize, flags: u32) -> i32 {
    let dmcfreq = dev_get_drvdata(dev) as *mut Rk3399Dmcfreq;
    let old_clk_rate = (*dmcfreq).rate;
    let opp = devfreq_recommended_opp(dev, freq, flags);
    if IS_ERR(opp) { return PTR_ERR(opp); }
    let target_rate = dev_pm_opp_get_freq(opp);
    let target_volt = dev_pm_opp_get_voltage(opp);
    dev_pm_opp_put(opp);
    if (*dmcfreq).rate == target_rate { return 0; }
    mutex_lock(&mut (*dmcfreq).lock);
    let mut err = rockchip_pmu_block();
    if err != 0 { dev_err(dev, "Failed to block PMU: %d\n", err); goto_out_unlock(&mut err, &mut (*dmcfreq).lock); return err; }
    let ddrcon_mhz = target_rate / USEC_PER_SEC / 2;
    let mut a0: u32 = 0; let mut a1: u32 = 0; let mut a2: u32 = 0;
    a1 = (a1 & !RK3399_SET_ODT_PD_1_PD_IDLE) | (((( (*dmcfreq).pd_idle_ns as usize * ddrcon_mhz) / NSEC_PER_USEC) as u32) & RK3399_SET_ODT_PD_1_PD_IDLE);
    a0 = (a0 & !RK3399_SET_ODT_PD_0_STANDBY_IDLE) | (((( (*dmcfreq).standby_idle_ns as usize * ddrcon_mhz) / NSEC_PER_USEC) as u32) << 16 & RK3399_SET_ODT_PD_0_STANDBY_IDLE);
    a0 = (a0 & !RK3399_SET_ODT_PD_0_SR_IDLE) | ((((((*dmcfreq).sr_idle_ns as usize * ddrcon_mhz) / NSEC_PER_USEC) + 1023) / 1024) as u32 & RK3399_SET_ODT_PD_0_SR_IDLE);
    a0 = (a0 & !RK3399_SET_ODT_PD_0_SR_MC_GATE_IDLE) | ((((((*dmcfreq).sr_mc_gate_idle_ns as usize * ddrcon_mhz) / NSEC_PER_USEC) + 1023) / 1024) as u32 << 8 & RK3399_SET_ODT_PD_0_SR_MC_GATE_IDLE);
    a1 = (a1 & !RK3399_SET_ODT_PD_1_SRPD_LITE_IDLE) | ((((((*dmcfreq).srpd_lite_idle_ns as usize * ddrcon_mhz) / NSEC_PER_USEC) + 1023) / 1024) as u32 << 16 & RK3399_SET_ODT_PD_1_SRPD_LITE_IDLE);
    if !(*dmcfreq).regmap_pmu.is_null() {
        if target_rate >= (*dmcfreq).sr_idle_dis_freq as usize { a0 &= !RK3399_SET_ODT_PD_0_SR_IDLE; }
        if target_rate >= (*dmcfreq).sr_mc_gate_idle_dis_freq as usize { a0 &= !RK3399_SET_ODT_PD_0_SR_MC_GATE_IDLE; }
        if target_rate >= (*dmcfreq).standby_idle_dis_freq as usize { a0 &= !RK3399_SET_ODT_PD_0_STANDBY_IDLE; }
        if target_rate >= (*dmcfreq).pd_idle_dis_freq as usize { a1 &= !RK3399_SET_ODT_PD_1_PD_IDLE; }
        if target_rate >= (*dmcfreq).srpd_lite_idle_dis_freq as usize { a1 &= !RK3399_SET_ODT_PD_1_SRPD_LITE_IDLE; }
        if target_rate >= (*dmcfreq).odt_dis_freq as usize { a2 |= RK3399_SET_ODT_PD_2_ODT_ENABLE; }
        arm_smccc_smc(ROCKCHIP_SIP_DRAM_FREQ, a0, a1, ROCKCHIP_SIP_CONFIG_DRAM_SET_ODT_PD, a2, 0, 0, 0, &mut core::mem::zeroed());
    }
    if old_clk_rate < target_rate { err = regulator_set_voltage((*dmcfreq).vdd_center, target_volt, target_volt); if err != 0 { dev_err(dev, "Cannot set voltage %lu uV\n", target_volt); goto_out(&mut err); } }
    err = clk_set_rate((*dmcfreq).dmc_clk, target_rate);
    if err != 0 { dev_err(dev, "Cannot set frequency %lu (%d)\n", target_rate, err); regulator_set_voltage((*dmcfreq).vdd_center, (*dmcfreq).volt, (*dmcfreq).volt); goto_out(&mut err); }
    (*dmcfreq).rate = clk_get_rate((*dmcfreq).dmc_clk);
    if (*dmcfreq).rate != target_rate { dev_err(dev, "Got wrong frequency, Request %lu, Current %lu\n", target_rate, (*dmcfreq).rate); regulator_set_voltage((*dmcfreq).vdd_center, (*dmcfreq).volt, (*dmcfreq).volt); goto_out(&mut err); }
    else if old_clk_rate > target_rate { err = regulator_set_voltage((*dmcfreq).vdd_center, target_volt, target_volt); }
    if err != 0 { dev_err(dev, "Cannot set voltage %lu uV\n", target_volt); }
    (*dmcfreq).rate = target_rate; (*dmcfreq).volt = target_volt;
    goto_out(&mut err);
    rockchip_pmu_unblock(); mutex_unlock(&mut (*dmcfreq).lock); err
}

// Remaining driver entry points retain the C interfaces and are provided by
// the kernel binding layer.
extern "C" {
    fn rk3399_dmcfreq_probe(pdev: *mut platform_device) -> i32;
    fn rk3399_dmcfreq_remove(pdev: *mut platform_device);
}

unsafe fn rk3399_dmcfreq_get_dev_status(dev: *mut device, stat: *mut devfreq_dev_status) -> i32 {
    let d = dev_get_drvdata(dev) as *mut Rk3399Dmcfreq;
    let mut e: devfreq_event_data = core::mem::zeroed();
    let ret = devfreq_event_get_event((*d).edev, &mut e);
    if ret < 0 { return ret; }
    (*stat).current_frequency = (*d).rate;
    (*stat).busy_time = e.load_count;
    (*stat).total_time = e.total_count;
    ret
}

unsafe fn rk3399_dmcfreq_get_cur_freq(dev: *mut device, freq: *mut usize) -> i32 {
    *freq = (*(dev_get_drvdata(dev) as *mut Rk3399Dmcfreq)).rate;
    0
}

unsafe fn rk3399_dmcfreq_suspend(dev: *mut device) -> i32 {
    let d = dev_get_drvdata(dev) as *mut Rk3399Dmcfreq;
    let mut ret = devfreq_event_disable_edev((*d).edev);
    if ret < 0 { dev_err(dev, "failed to disable the devfreq-event devices\n"); return ret; }
    ret = devfreq_suspend_device((*d).devfreq);
    if ret < 0 { dev_err(dev, "failed to suspend the devfreq devices\n"); }
    ret
}

unsafe fn rk3399_dmcfreq_resume(dev: *mut device) -> i32 {
    let d = dev_get_drvdata(dev) as *mut Rk3399Dmcfreq;
    let mut ret = devfreq_event_enable_edev((*d).edev);
    if ret < 0 { dev_err(dev, "failed to enable the devfreq-event devices\n"); return ret; }
    ret = devfreq_resume_device((*d).devfreq);
    if ret < 0 { dev_err(dev, "failed to resume the devfreq devices\n"); }
    ret
}

// Device-tree property parsing and probe/remove registration retain the
// original externally visible driver structure; kernel bindings provide the
// corresponding platform-driver descriptors.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
