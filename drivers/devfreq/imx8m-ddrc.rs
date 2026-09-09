// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 NXP
 */

// Linux kernel dependencies supplied by other files.

const IMX_SIP_DDR_DVFS: u64 = 0xc2000004;
const IMX_SIP_DDR_DVFS_GET_FREQ_COUNT: u64 = 0x10;
const IMX_SIP_DDR_DVFS_GET_FREQ_INFO: u64 = 0x11;

#[repr(C)]
pub struct Imx8mDdrcFreq {
    pub rate: c_ulong,
    pub smcarg: c_ulong,
    pub dram_core_parent_index: c_int,
    pub dram_alt_parent_index: c_int,
    pub dram_apb_parent_index: c_int,
}

const IMX8M_DDRC_MAX_FREQ_COUNT: usize = 4;

#[repr(C)]
pub struct Imx8mDdrc {
    pub profile: DevfreqDevProfile,
    pub devfreq: *mut Devfreq,
    pub dram_core: *mut Clk,
    pub dram_pll: *mut Clk,
    pub dram_alt: *mut Clk,
    pub dram_apb: *mut Clk,
    pub freq_count: c_int,
    pub freq_table: [Imx8mDdrcFreq; IMX8M_DDRC_MAX_FREQ_COUNT],
}

unsafe fn imx8m_ddrc_find_freq(priv_: *mut Imx8mDdrc, mut rate: c_ulong) -> *mut Imx8mDdrcFreq {
    rate = div_round_closest(rate, 250000);
    let mut i = 0;
    while i < (*priv_).freq_count {
        let freq = &mut (*priv_).freq_table[i as usize];
        if freq.rate == rate || freq.rate.wrapping_add(1) == rate || freq.rate.wrapping_sub(1) == rate {
            return freq;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

unsafe fn imx8m_ddrc_smc_set_freq(target_freq: c_int) {
    let mut res = ArmSmcccRes::default();
    let mut online_cpus: u32 = 0;
    local_irq_disable();
    for_each_online_cpu!(cpu => { online_cpus |= 1u32 << ((cpu as u32) * 8); });
    arm_smccc_smc(IMX_SIP_DDR_DVFS, target_freq as u64, online_cpus as u64, 0, 0, 0, 0, 0, &mut res);
    local_irq_enable();
}

unsafe fn clk_get_parent_by_index(clk: *mut Clk, index: c_int) -> *mut Clk {
    let hw = clk_hw_get_parent_by_index(__clk_get_hw(clk), index);
    if hw.is_null() { core::ptr::null_mut() } else { (*hw).clk }
}

unsafe fn imx8m_ddrc_set_freq(dev: *mut Device, freq: *mut Imx8mDdrcFreq) -> c_int {
    let priv_ = dev_get_drvdata(dev) as *mut Imx8mDdrc;
    let new_core = clk_get_parent_by_index((*priv_).dram_core, (*freq).dram_core_parent_index - 1);
    if new_core.is_null() { dev_err(dev, "failed to fetch new dram_core parent\n"); return -EINVAL; }
    let new_alt = if (*freq).dram_alt_parent_index != 0 { let p = clk_get_parent_by_index((*priv_).dram_alt, (*freq).dram_alt_parent_index - 1); if p.is_null() { dev_err(dev, "failed to fetch new dram_alt parent\n"); return -EINVAL; } p } else { core::ptr::null_mut() };
    let new_apb = if (*freq).dram_apb_parent_index != 0 { let p = clk_get_parent_by_index((*priv_).dram_apb, (*freq).dram_apb_parent_index - 1); if p.is_null() { dev_err(dev, "failed to fetch new dram_apb parent\n"); return -EINVAL; } p } else { core::ptr::null_mut() };
    let mut ret = clk_prepare_enable(new_core);
    if ret != 0 { dev_err(dev, "failed to enable new dram_core parent: %d\n", ret); return ret; }
    ret = clk_prepare_enable(new_alt);
    if ret != 0 { dev_err(dev, "failed to enable new dram_alt parent: %d\n", ret); clk_disable_unprepare(new_core); return ret; }
    ret = clk_prepare_enable(new_apb);
    if ret != 0 { dev_err(dev, "failed to enable new dram_apb parent: %d\n", ret); clk_disable_unprepare(new_alt); clk_disable_unprepare(new_core); return ret; }
    imx8m_ddrc_smc_set_freq((*freq).smcarg as c_int);
    ret = clk_set_parent((*priv_).dram_core, new_core);
    if ret != 0 { dev_warn(dev, "failed to set dram_core parent: %d\n", ret); }
    if !new_alt.is_null() { ret = clk_set_parent((*priv_).dram_alt, new_alt); if ret != 0 { dev_warn(dev, "failed to set dram_alt parent: %d\n", ret); } }
    if !new_apb.is_null() { ret = clk_set_parent((*priv_).dram_apb, new_apb); if ret != 0 { dev_warn(dev, "failed to set dram_apb parent: %d\n", ret); } }
    clk_get_rate((*priv_).dram_pll);
    clk_disable_unprepare(new_apb);
    clk_disable_unprepare(new_alt);
    clk_disable_unprepare(new_core);
    ret
}

unsafe fn imx8m_ddrc_target(dev: *mut Device, freq: *mut c_ulong, flags: u32) -> c_int {
    let priv_ = dev_get_drvdata(dev) as *mut Imx8mDdrc;
    let new_opp = devfreq_recommended_opp(dev, freq, flags);
    if is_err(new_opp) { let ret = ptr_err(new_opp); dev_err(dev, "failed to get recommended opp: %d\n", ret); return ret; }
    dev_pm_opp_put(new_opp);
    let old_freq = clk_get_rate((*priv_).dram_core);
    if *freq == old_freq { return 0; }
    let info = imx8m_ddrc_find_freq(priv_, *freq);
    if info.is_null() { return -EINVAL; }
    let ret = imx8m_ddrc_set_freq(dev, info);
    let new_freq = clk_get_rate((*priv_).dram_core);
    if ret != 0 { dev_err(dev, "ddrc failed freq switch to %lu from %lu: error %d. now at %lu\n", *freq, old_freq, ret, new_freq); }
    else if *freq != new_freq { dev_err(dev, "ddrc failed freq update to %lu from %lu, now at %lu\n", *freq, old_freq, new_freq); }
    else { dev_dbg(dev, "ddrc freq set to %lu (was %lu)\n", *freq, old_freq); }
    ret
}

unsafe fn imx8m_ddrc_get_cur_freq(dev: *mut Device, freq: *mut c_ulong) -> c_int {
    *freq = clk_get_rate((dev_get_drvdata(dev) as *mut Imx8mDdrc).as_ref().unwrap().dram_core); 0
}

unsafe fn imx8m_ddrc_init_freq_info(dev: *mut Device) -> c_int {
    let p = dev_get_drvdata(dev) as *mut Imx8mDdrc; let mut res = ArmSmcccRes::default();
    arm_smccc_smc(IMX_SIP_DDR_DVFS, IMX_SIP_DDR_DVFS_GET_FREQ_COUNT, 0,0,0,0,0,0, &mut res); (*p).freq_count = res.a0 as c_int;
    if (*p).freq_count <= 0 || (*p).freq_count > 4 { return -ENODEV; }
    for i in 0..(*p).freq_count as usize { arm_smccc_smc(IMX_SIP_DDR_DVFS, IMX_SIP_DDR_DVFS_GET_FREQ_INFO, i as u64,0,0,0,0,0,&mut res); if (res.a0 as i64) <= 0 { return -ENODEV; } let f=&mut (*p).freq_table[i]; f.rate=res.a0 as c_ulong; f.smcarg=i as c_ulong; f.dram_core_parent_index=res.a1 as c_int; f.dram_alt_parent_index=res.a2 as c_int; f.dram_apb_parent_index=res.a3 as c_int; if f.dram_core_parent_index != 1 && f.dram_core_parent_index != 2 || f.dram_alt_parent_index > 8 || f.dram_apb_parent_index > 8 || f.dram_core_parent_index == 2 && f.dram_alt_parent_index == 0 { return -ENODEV; } }
    0
}

unsafe fn imx8m_ddrc_check_opps(dev: *mut Device) -> c_int {
    let p=dev_get_drvdata(dev) as *mut Imx8mDdrc; let n=dev_pm_opp_get_opp_count(dev); if n<0{return n}; let mut freq=0;
    for _ in 0..n { let opp=dev_pm_opp_find_freq_ceil(dev,&mut freq); if is_err(opp){return ptr_err(opp)}; dev_pm_opp_put(opp); if imx8m_ddrc_find_freq(p,freq).is_null(){dev_info(dev,"Disable unsupported OPP %luHz %luMT/s\n",freq,div_round_closest(freq,250000));dev_pm_opp_disable(dev,freq);} freq+=1; } 0
}

unsafe fn imx8m_ddrc_exit(dev:*mut Device){dev_pm_opp_of_remove_table(dev)}

unsafe fn imx8m_ddrc_probe(pdev: *mut PlatformDevice) -> c_int {
    let dev = &mut (*pdev).dev;
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<Imx8mDdrc>(), GFP_KERNEL) as *mut Imx8mDdrc;
    if priv_.is_null() { return -ENOMEM; }
    platform_set_drvdata(pdev, priv_);
    let mut ret = imx8m_ddrc_init_freq_info(dev); if ret != 0 { dev_err(dev,"failed to init firmware freq info: %d\n",ret); return ret; }
    (*priv_).dram_core=devm_clk_get(dev,"core"); (*priv_).dram_pll=devm_clk_get(dev,"pll"); (*priv_).dram_alt=devm_clk_get(dev,"alt"); (*priv_).dram_apb=devm_clk_get(dev,"apb");
    if is_err((*priv_).dram_core)||is_err((*priv_).dram_pll)||is_err((*priv_).dram_alt)||is_err((*priv_).dram_apb){return -ENODEV;}
    ret=dev_pm_opp_of_add_table(dev); if ret<0{return ret;} ret=imx8m_ddrc_check_opps(dev); if ret<0{dev_pm_opp_of_remove_table(dev);return ret;}
    (*priv_).profile.target=Some(imx8m_ddrc_target); (*priv_).profile.exit=Some(imx8m_ddrc_exit); (*priv_).profile.get_cur_freq=Some(imx8m_ddrc_get_cur_freq); (*priv_).profile.initial_freq=clk_get_rate((*priv_).dram_core);
    (*priv_).devfreq=devm_devfreq_add_device(dev,&mut (*priv_).profile,"userspace",core::ptr::null_mut()); if is_err((*priv_).devfreq){ret=ptr_err((*priv_).devfreq);dev_pm_opp_of_remove_table(dev);return ret;} 0
}

// static platform driver, OF match table, module_device_table, and module metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
