// SPDX-License-Identifier: GPL-2.0-only
/* OMAP Voltage Controller (VC) interface */

// Kernel and platform dependencies are supplied by the surrounding translation unit.

const OMAP4430_VDD_IVA_I2C_DISABLE: u32 = 1 << 14;
const OMAP4430_VDD_MPU_I2C_DISABLE: u32 = 1 << 13;
const OMAP4430_VDD_CORE_I2C_DISABLE: u32 = 1 << 12;
const OMAP4430_VDD_IVA_PRESENCE: u32 = 1 << 9;
const OMAP4430_VDD_MPU_PRESENCE: u32 = 1 << 8;
const OMAP4430_AUTO_CTRL_VDD_IVA: u32 = 2 << 4;
const OMAP4430_AUTO_CTRL_VDD_MPU: u32 = 2 << 2;
const OMAP4430_AUTO_CTRL_VDD_CORE: u32 = 2;
const OMAP4430_VDD_I2C_DISABLE_MASK: u32 = OMAP4430_VDD_IVA_I2C_DISABLE | OMAP4430_VDD_MPU_I2C_DISABLE | OMAP4430_VDD_CORE_I2C_DISABLE;
const OMAP4_VDD_DEFAULT_VAL: u32 = OMAP4430_VDD_I2C_DISABLE_MASK | OMAP4430_VDD_IVA_PRESENCE | OMAP4430_VDD_MPU_PRESENCE | OMAP4430_AUTO_CTRL_VDD_IVA | OMAP4430_AUTO_CTRL_VDD_MPU | OMAP4430_AUTO_CTRL_VDD_CORE;
const OMAP4_VDD_RET_VAL: u32 = OMAP4_VDD_DEFAULT_VAL & !OMAP4430_VDD_I2C_DISABLE_MASK;
const CFG_CHANNEL_MASK: u32 = 0x1f;

#[repr(C)]
struct omap_vc_channel_cfg { sa: u8, rav: u8, rac: u8, racen: u8, cmd: u8 }
static mut vc_default_channel_cfg: omap_vc_channel_cfg = omap_vc_channel_cfg { sa: 1, rav: 2, rac: 4, racen: 8, cmd: 16 };
static mut vc_mutant_channel_cfg: omap_vc_channel_cfg = omap_vc_channel_cfg { sa: 1, rav: 4, rac: 8, racen: 16, cmd: 2 };
static mut vc_cfg_bits: *mut omap_vc_channel_cfg = core::ptr::null_mut();
static mut sr_i2c_pcb_length: u32 = 63;

unsafe fn omap_vc_config_channel(voltdm: *mut voltagedomain) -> i32 {
    let vc = (*voltdm).vc;
    if ((*vc).flags & OMAP_VC_CHANNEL_DEFAULT) != 0 { (*vc).cfg_channel &= (*vc_cfg_bits).racen; }
    ((*voltdm).rmw)(CFG_CHANNEL_MASK << (*vc).cfg_channel_sa_shift, (*vc).cfg_channel << (*vc).cfg_channel_sa_shift, (*vc).cfg_channel_reg);
    0
}

pub unsafe fn omap_vc_pre_scale(voltdm: *mut voltagedomain, target_volt: usize, target_vsel: *mut u8, current_vsel: *mut u8) -> i32 {
    let vc = (*voltdm).vc;
    if (*voltdm).pmic.is_null() { pr_err!("%s: Insufficient pmic info to scale the vdd_%s\n", "omap_vc_pre_scale", (*voltdm).name); return -EINVAL; }
    if (*(*voltdm).pmic).uv_to_vsel.is_none() { pr_err!("%s: PMIC function to convert voltage in uV to vsel not registered. Hence unable to scale voltage for vdd_%s\n", "omap_vc_pre_scale", (*voltdm).name); return -ENODATA; }
    if (*voltdm).read.is_none() || (*voltdm).write.is_none() { pr_err!("%s: No read/write API for accessing vdd_%s regs\n", "omap_vc_pre_scale", (*voltdm).name); return -EINVAL; }
    *target_vsel = ((*(*voltdm).pmic).uv_to_vsel.unwrap())(target_volt);
    *current_vsel = ((*(*voltdm).pmic).uv_to_vsel.unwrap())((*voltdm).nominal_volt);
    let mut val = ((*voltdm).read.unwrap())((*vc).cmdval_reg);
    val &= !(*(*vc).common).cmd_on_mask;
    val |= (*target_vsel as u32) << (*(*vc).common).cmd_on_shift;
    ((*voltdm).write.unwrap())(val, (*vc).cmdval_reg);
    (*voltdm).vc_param.on = target_volt;
    omap_vp_update_errorgain(voltdm, target_volt);
    0
}

pub unsafe fn omap_vc_post_scale(voltdm: *mut voltagedomain, _target_volt: usize, target_vsel: u8, current_vsel: u8) {
    let steps = (target_vsel as i32 - current_vsel as i32).unsigned_abs();
    let delay = (steps * (*(*voltdm).pmic).step_size) / (*(*voltdm).pmic).slew_rate + 2;
    udelay(delay);
}

pub unsafe fn omap_vc_bypass_scale(voltdm: *mut voltagedomain, target_volt: usize) -> i32 {
    let vc = (*voltdm).vc; let mut loop_cnt = 0u32; let mut retries_cnt = 0u32;
    let (mut target_vsel, mut current_vsel) = (0u8, 0u8);
    let ret = omap_vc_pre_scale(voltdm, target_volt, &mut target_vsel, &mut current_vsel); if ret != 0 { return ret; }
    let valid = (*(*vc).common).valid; let reg = (*(*vc).common).bypass_val_reg;
    let mut value = ((target_vsel as u32) << (*(*vc).common).data_shift) | ((*vc).volt_reg_addr << (*(*vc).common).regaddr_shift) | ((*vc).i2c_slave_addr << (*(*vc).common).slaveaddr_shift);
    ((*voltdm).write.unwrap())(value, reg); ((*voltdm).write.unwrap())(value | valid, reg); value = ((*voltdm).read.unwrap())(reg);
    while value & valid == 0 { loop_cnt += 1; if retries_cnt > 10 { pr_warn!("%s: Retry count exceeded\n", "omap_vc_bypass_scale"); return -ETIMEDOUT; } if loop_cnt > 50 { retries_cnt += 1; loop_cnt = 0; udelay(10); } value = ((*voltdm).read.unwrap())(reg); }
    omap_vc_post_scale(voltdm, target_volt, target_vsel, current_vsel); 0
}

#[inline] unsafe fn omap_usec_to_32k(usec: u32) -> u32 { (32768u64 * usec as u64).div_ceil(1_000_000) as u32 }

#[repr(C)] struct omap3_vc_timings { voltsetup1: u32, voltsetup2: u32 }
#[repr(C)] struct omap3_vc { vd: *mut voltagedomain, voltctrl: u32, voltsetup1: u32, voltsetup2: u32, timings: [omap3_vc_timings; 2] }
static mut vc: omap3_vc = omap3_vc { vd: core::ptr::null_mut(), voltctrl: 0, voltsetup1: 0, voltsetup2: 0, timings: [omap3_vc_timings { voltsetup1: 0, voltsetup2: 0 }; 2] };

pub unsafe fn omap3_vc_set_pmic_signaling(core_next_state: i32) { let vd = vc.vd; let c = &mut vc.timings; let mut voltctrl = vc.voltctrl; let mut v1 = vc.voltsetup1; let mut v2 = vc.voltsetup2; match core_next_state { PWRDM_POWER_OFF => { voltctrl &= !(OMAP3430_PRM_VOLTCTRL_AUTO_RET | OMAP3430_PRM_VOLTCTRL_AUTO_SLEEP); voltctrl |= OMAP3430_PRM_VOLTCTRL_AUTO_OFF; if voltctrl & OMAP3430_PRM_VOLTCTRL_SEL_OFF != 0 { v2 = c[0].voltsetup2; } else { v1 = c[0].voltsetup1; } }, _ => { voltctrl &= !(OMAP3430_PRM_VOLTCTRL_AUTO_OFF | OMAP3430_PRM_VOLTCTRL_AUTO_SLEEP); voltctrl |= OMAP3430_PRM_VOLTCTRL_AUTO_RET; v1 = c[1].voltsetup1; } } if voltctrl != vc.voltctrl { ((*vd).write.unwrap())(voltctrl, OMAP3_PRM_VOLTCTRL_OFFSET); vc.voltctrl = voltctrl; } if v1 != vc.voltsetup1 { ((*vd).write.unwrap())(v1, OMAP3_PRM_VOLTSETUP1_OFFSET); vc.voltsetup1 = v1; } if v2 != vc.voltsetup2 { ((*vd).write.unwrap())(v2, OMAP3_PRM_VOLTSETUP2_OFFSET); vc.voltsetup2 = v2; } }

pub unsafe fn omap4_vc_set_pmic_signaling(core_next_state: i32) { if vc.vd.is_null() { return; } let val = if core_next_state == PWRDM_POWER_RET { OMAP4_VDD_RET_VAL } else { OMAP4_VDD_DEFAULT_VAL }; ((*vc.vd).write.unwrap())(val, OMAP4_PRM_VOLTCTRL_OFFSET); }

unsafe fn omap3_vc_init_pmic_signaling(voltdm: *mut voltagedomain) { if !vc.vd.is_null() { return; } vc.vd = voltdm; let mut val = ((*voltdm).read.unwrap())(OMAP3_PRM_POLCTRL_OFFSET); if val & OMAP3430_PRM_POLCTRL_CLKREQ_POL == 0 || val & OMAP3430_PRM_POLCTRL_OFFMODE_POL != 0 { val |= OMAP3430_PRM_POLCTRL_CLKREQ_POL; val &= !OMAP3430_PRM_POLCTRL_OFFMODE_POL; pr_debug!("PM: fixing sys_clkreq and sys_off_mode polarity to 0x%x\n", val); ((*voltdm).write.unwrap())(val, OMAP3_PRM_POLCTRL_OFFSET); } val = ((*voltdm).read.unwrap())(OMAP3_PRM_VOLTCTRL_OFFSET); if val & OMAP3430_PRM_VOLTCTRL_SEL_OFF == 0 { val |= OMAP3430_PRM_VOLTCTRL_SEL_OFF; pr_debug!("PM: setting voltctrl sys_off_mode signaling to 0x%x\n", val); ((*voltdm).write.unwrap())(val, OMAP3_PRM_VOLTCTRL_OFFSET); } vc.voltctrl = val; omap3_vc_set_pmic_signaling(PWRDM_POWER_ON); }

unsafe fn omap3_init_voltsetup1(voltdm: *mut voltagedomain, c: *mut omap3_vc_timings, idle: usize) { let mut val = ((*voltdm).vc_param.on - idle) / (*(*voltdm).pmic).slew_rate as usize; val *= (*voltdm).sys_clk.rate / 8 / 1_000_000 + 1; val <<= __ffs((*voltdm).vfsm.voltsetup_mask); (*c).voltsetup1 &= !(*voltdm).vfsm.voltsetup_mask; (*c).voltsetup1 |= val as u32; }
unsafe fn omap3_set_i2c_timings(voltdm: *mut voltagedomain) { omap3_init_voltsetup1(voltdm, &mut vc.timings[0], (*voltdm).vc_param.off); omap3_init_voltsetup1(voltdm, &mut vc.timings[1], (*voltdm).vc_param.ret); }
unsafe fn omap3_set_off_timings(voltdm: *mut voltagedomain) { let c = &mut vc.timings[0]; if c.voltsetup2 != 0 { return; } let (mut start, mut shut) = (0, 0); omap_pm_get_oscillator(&mut start, &mut shut); let clk = if start == ULONG_MAX { pr_debug!("PM: oscillator start-up time not initialized, using 10ms\n"); omap_usec_to_32k(10000) } else { omap_usec_to_32k(start) }; let off = omap_usec_to_32k(488); c.voltsetup2 = clk - off; ((*voltdm).write.unwrap())(clk, OMAP3_PRM_CLKSETUP_OFFSET); ((*voltdm).write.unwrap())(off, OMAP3_PRM_VOLTOFFSET_OFFSET); }
unsafe fn omap3_vc_init_channel_internal(voltdm: *mut voltagedomain) { omap3_vc_init_pmic_signaling(voltdm); omap3_set_off_timings(voltdm); omap3_set_i2c_timings(voltdm); }

unsafe fn omap4_calc_volt_ramp(voltdm: *mut voltagedomain, diff: u32) -> u32 { let time = diff / (*(*voltdm).pmic).slew_rate; let mut cycles = (*voltdm).sys_clk.rate / 1000 * time / 1000 / 64; let mut prescaler = 0; if cycles > 63 { cycles /= 4; prescaler += 1; } if cycles > 63 { cycles /= 2; prescaler += 1; } if cycles > 63 { cycles /= 4; prescaler += 1; } if cycles > 63 { pr_warn!("%s: invalid setuptime for vdd_%s\n", "omap4_calc_volt_ramp", (*voltdm).name); return 0; } (prescaler << OMAP4430_RAMP_UP_PRESCAL_SHIFT) | ((cycles + 1) << OMAP4430_RAMP_UP_COUNT_SHIFT) }
unsafe fn omap4_usec_to_val_scrm(usec: u32, shift: i32, mask: u32) -> u32 { let mut val = omap_usec_to_32k(usec) << shift; if val > mask { val = mask; } val }
unsafe fn omap4_set_timings(voltdm: *mut voltagedomain, off_mode: bool) { let (diff, offset) = if off_mode { ((*voltdm).vc_param.on - (*voltdm).vc_param.off, (*voltdm).vfsm.voltsetup_off_reg) } else { ((*voltdm).vc_param.on - (*voltdm).vc_param.ret, (*voltdm).vfsm.voltsetup_reg) }; let ramp = omap4_calc_volt_ramp(voltdm, diff as u32); if ramp == 0 { return; } let mut val = ((*voltdm).read.unwrap())(offset); val |= ramp << OMAP4430_RAMP_DOWN_COUNT_SHIFT; val |= ramp << OMAP4430_RAMP_UP_COUNT_SHIFT; ((*voltdm).write.unwrap())(val, offset); let (mut tstart, mut tshut) = (0, 0); omap_pm_get_oscillator(&mut tstart, &mut tshut); val = omap4_usec_to_val_scrm(tstart, OMAP4_SETUPTIME_SHIFT, OMAP4_SETUPTIME_MASK) | omap4_usec_to_val_scrm(tshut, OMAP4_DOWNTIME_SHIFT, OMAP4_DOWNTIME_MASK); writel_relaxed(val, OMAP4_SCRM_CLKSETUPTIME); }
unsafe fn omap4_vc_init_pmic_signaling(voltdm: *mut voltagedomain) { if !vc.vd.is_null() { return; } vc.vd = voltdm; ((*voltdm).write.unwrap())(OMAP4_VDD_DEFAULT_VAL, OMAP4_PRM_VOLTCTRL_OFFSET); }
unsafe fn omap4_vc_init_channel_internal(voltdm: *mut voltagedomain) { omap4_vc_init_pmic_signaling(voltdm); omap4_set_timings(voltdm, true); omap4_set_timings(voltdm, false); }

#[repr(C)] struct i2c_init_data { loadbits: u8, load: u8, hsscll_38_4: u8, hsscll_26: u8, hsscll_19_2: u8, hsscll_16_8: u8, hsscll_12: u8 }
static omap4_i2c_timing_data: [i2c_init_data; 4] = [
    i2c_init_data { load: 50, loadbits: 3, hsscll_38_4: 13, hsscll_26: 11, hsscll_19_2: 9, hsscll_16_8: 9, hsscll_12: 8 },
    i2c_init_data { load: 25, loadbits: 2, hsscll_38_4: 13, hsscll_26: 11, hsscll_19_2: 9, hsscll_16_8: 9, hsscll_12: 8 },
    i2c_init_data { load: 12, loadbits: 1, hsscll_38_4: 11, hsscll_26: 10, hsscll_19_2: 9, hsscll_16_8: 9, hsscll_12: 8 },
    i2c_init_data { load: 0, loadbits: 0, hsscll_38_4: 12, hsscll_26: 10, hsscll_19_2: 9, hsscll_16_8: 8, hsscll_12: 8 },
];
unsafe fn omap4_vc_i2c_timing_init(voltdm: *mut voltagedomain) { if !(*(*voltdm).pmic).i2c_high_speed { pr_info!("%s: using bootloader low-speed timings\n", "omap4_vc_i2c_timing_init"); return; } let cap = (sr_i2c_pcb_length + 7) / 8 + 4 + (*(*voltdm).pmic).i2c_pad_load; let mut d = &omap4_i2c_timing_data[0]; while d.load as u32 > cap { d = &*(&omap4_i2c_timing_data[(d as *const _ as usize - omap4_i2c_timing_data.as_ptr() as usize) / core::mem::size_of::<i2c_init_data>() + 1] as *const _); } let h = match (*voltdm).sys_clk.rate { 38400000 => d.hsscll_38_4, 26000000 => d.hsscll_26, 19200000 => d.hsscll_19_2, 16800000 => d.hsscll_16_8, 12000000 => d.hsscll_12, _ => { pr_warn!("%s: unsupported sysclk rate: %d!\n", "omap4_vc_i2c_timing_init", (*voltdm).sys_clk.rate); return; } }; let val = (d.loadbits as u32) << 25 | (d.loadbits as u32) << 29; writel_relaxed(val, OMAP2_L4_IO_ADDRESS(OMAP4_CTRL_MODULE_PAD_WKUP + OMAP4_CTRL_MODULE_PAD_WKUP_CONTROL_I2C_2)); let val = (h as u32) << OMAP4430_HSSCLL_SHIFT | 0x28 << OMAP4430_SCLL_SHIFT | 0x2c << OMAP4430_SCLH_SHIFT; ((*voltdm).write.unwrap())(val, OMAP4_PRM_VC_CFG_I2C_CLK_OFFSET); }
unsafe fn omap_vc_i2c_init(voltdm: *mut voltagedomain) { let vcch = (*voltdm).vc; static mut initialized: bool = false; static mut i2c_high_speed: bool = false; if initialized { if (*(*voltdm).pmic).i2c_high_speed != i2c_high_speed { pr_warn!("%s: I2C config for vdd_%s does not match other channels (%u).\n", "omap_vc_i2c_init", (*voltdm).name, i2c_high_speed); } return; } i2c_high_speed = (*(*voltdm).pmic).i2c_high_speed; if i2c_high_speed { ((*voltdm).rmw)((*(*vcch).common).i2c_cfg_clear_mask, (*(*vcch).common).i2c_cfg_hsen_mask, (*(*vcch).common).i2c_cfg_reg); } let m = (*(*voltdm).pmic).i2c_mcode; if m != 0 { ((*voltdm).rmw)((*(*vcch).common).i2c_mcode_mask, (m as u32) << __ffs((*(*vcch).common).i2c_mcode_mask), (*(*vcch).common).i2c_cfg_reg); } if cpu_is_omap44xx() { omap4_vc_i2c_timing_init(voltdm); } initialized = true; }

unsafe fn omap_vc_calc_vsel(voltdm: *mut voltagedomain, mut uvolt: u32) -> u8 { if (*(*voltdm).pmic).vddmin > uvolt { uvolt = (*(*voltdm).pmic).vddmin; } if (*(*voltdm).pmic).vddmax < uvolt { WARN!(1, "%s: voltage not supported by pmic: %u vs max %u\n", "omap_vc_calc_vsel", uvolt, (*(*voltdm).pmic).vddmax); uvolt = (*(*voltdm).pmic).vddmax; } ((*(*voltdm).pmic).uv_to_vsel.unwrap())(uvolt as usize) }

pub unsafe fn omap_vc_init_channel(voltdm: *mut voltagedomain) { let vcch = (*voltdm).vc; if (*voltdm).pmic.is_null() || (*(*voltdm).pmic).uv_to_vsel.is_none() || (*voltdm).read.is_none() || (*voltdm).write.is_none() { pr_err!("%s: No PMIC info or read/write API for vdd_%s\n", "omap_vc_init_channel", (*voltdm).name); return; } (*vcch).cfg_channel = 0; vc_cfg_bits = if (*vcch).flags & OMAP_VC_CHANNEL_CFG_MUTANT != 0 { &mut vc_mutant_channel_cfg } else { &mut vc_default_channel_cfg }; (*vcch).i2c_slave_addr = (*(*voltdm).pmic).i2c_slave_addr; (*vcch).volt_reg_addr = (*(*voltdm).pmic).volt_reg_addr; (*vcch).cmd_reg_addr = (*(*voltdm).pmic).cmd_reg_addr; ((*voltdm).rmw)((*vcch).smps_sa_mask, (*vcch).i2c_slave_addr << __ffs((*vcch).smps_sa_mask), (*vcch).smps_sa_reg); (*vcch).cfg_channel |= (*vc_cfg_bits).sa; ((*voltdm).rmw)((*vcch).smps_volra_mask, (*vcch).volt_reg_addr << __ffs((*vcch).smps_volra_mask), (*vcch).smps_volra_reg); (*vcch).cfg_channel |= (*vc_cfg_bits).rav; if (*vcch).cmd_reg_addr != 0 { ((*voltdm).rmw)((*vcch).smps_cmdra_mask, (*vcch).cmd_reg_addr << __ffs((*vcch).smps_cmdra_mask), (*vcch).smps_cmdra_reg); (*vcch).cfg_channel |= (*vc_cfg_bits).rac; } if (*vcch).cmd_reg_addr == (*vcch).volt_reg_addr { (*vcch).cfg_channel |= (*vc_cfg_bits).racen; } let val = ((*omap_vc_calc_vsel)(voltdm, (*voltdm).vc_param.on) as u32) << (*(*vcch).common).cmd_on_shift | ((*omap_vc_calc_vsel)(voltdm, (*voltdm).vc_param.onlp) as u32) << (*(*vcch).common).cmd_onlp_shift | ((*omap_vc_calc_vsel)(voltdm, (*voltdm).vc_param.ret) as u32) << (*(*vcch).common).cmd_ret_shift | ((*omap_vc_calc_vsel)(voltdm, (*voltdm).vc_param.off) as u32) << (*(*vcch).common).cmd_off_shift; ((*voltdm).write.unwrap())(val, (*vcch).cmdval_reg); (*vcch).cfg_channel |= (*vc_cfg_bits).cmd; omap_vc_config_channel(voltdm); if cpu_is_omap34xx() { omap3_vc_init_channel_internal(voltdm); } else if cpu_is_omap44xx() { omap4_vc_init_channel_internal(voltdm); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
