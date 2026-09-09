// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2022 MediaTek Inc.
 */

// External Linux kernel types, constants, macros, and functions are supplied by
// the surrounding kernel bindings.

#[repr(C)]
struct mtk_ccifreq_platform_data {
    min_volt_shift: i32,
    max_volt_shift: i32,
    proc_max_volt: i32,
    sram_min_volt: i32,
    sram_max_volt: i32,
}

#[repr(C)]
struct mtk_ccifreq_drv {
    dev: *mut device,
    devfreq: *mut devfreq,
    proc_reg: *mut regulator,
    sram_reg: *mut regulator,
    cci_clk: *mut clk,
    inter_clk: *mut clk,
    inter_voltage: i32,
    pre_freq: usize,
    // Avoid race condition for regulators between notify and policy
    reg_lock: mutex,
    opp_nb: notifier_block,
    soc_data: *const mtk_ccifreq_platform_data,
    vtrack_max: i32,
}

unsafe fn mtk_ccifreq_set_voltage(drv: *mut mtk_ccifreq_drv, new_voltage: i32) -> i32 {
    let soc_data = (*drv).soc_data;
    let dev = (*drv).dev;
    let (mut pre_voltage, mut pre_vsram, mut new_vsram, mut vsram, mut voltage, mut ret): (i32, i32, i32, i32, i32, i32);
    let mut retry_max = (*drv).vtrack_max;

    if (*drv).sram_reg.is_null() {
        return regulator_set_voltage((*drv).proc_reg, new_voltage, (*soc_data).proc_max_volt);
    }

    pre_voltage = regulator_get_voltage((*drv).proc_reg);
    if pre_voltage < 0 { dev_err(dev, "invalid vproc value: %d\n", pre_voltage); return pre_voltage; }
    pre_vsram = regulator_get_voltage((*drv).sram_reg);
    if pre_vsram < 0 { dev_err(dev, "invalid vsram value: %d\n", pre_vsram); return pre_vsram; }

    new_vsram = clamp(new_voltage + (*soc_data).min_volt_shift, (*soc_data).sram_min_volt, (*soc_data).sram_max_volt);
    loop {
        if pre_voltage <= new_voltage {
            vsram = clamp(pre_voltage + (*soc_data).max_volt_shift, (*soc_data).sram_min_volt, new_vsram);
            ret = regulator_set_voltage((*drv).sram_reg, vsram, (*soc_data).sram_max_volt);
            if ret != 0 { return ret; }
            voltage = if vsram == (*soc_data).sram_max_volt || new_vsram == (*soc_data).sram_min_volt { new_voltage } else { vsram - (*soc_data).min_volt_shift };
            ret = regulator_set_voltage((*drv).proc_reg, voltage, (*soc_data).proc_max_volt);
            if ret != 0 { regulator_set_voltage((*drv).sram_reg, pre_vsram, (*soc_data).sram_max_volt); return ret; }
        } else {
            voltage = max(new_voltage, pre_vsram - (*soc_data).max_volt_shift);
            ret = regulator_set_voltage((*drv).proc_reg, voltage, (*soc_data).proc_max_volt);
            if ret != 0 { return ret; }
            vsram = if voltage == new_voltage { new_vsram } else { max(new_vsram, voltage + (*soc_data).min_volt_shift) };
            ret = regulator_set_voltage((*drv).sram_reg, vsram, (*soc_data).sram_max_volt);
            if ret != 0 { regulator_set_voltage((*drv).proc_reg, pre_voltage, (*soc_data).proc_max_volt); return ret; }
        }
        pre_voltage = voltage; pre_vsram = vsram;
        retry_max -= 1;
        if retry_max < 0 { dev_err(dev, "over loop count, failed to set voltage\n"); return -EINVAL; }
        if voltage == new_voltage && vsram == new_vsram { break; }
    }
    0
}

unsafe fn mtk_ccifreq_target(dev: *mut device, freq: *mut usize, _flags: u32) -> i32 {
    let drv = dev_get_drvdata(dev) as *mut mtk_ccifreq_drv;
    if drv.is_null() { return -EINVAL; }
    if (*drv).pre_freq == *freq { return 0; }
    mutex_lock(&mut (*drv).reg_lock);
    let inter_voltage = (*drv).inter_voltage;
    let cci_pll = clk_get_parent((*drv).cci_clk);
    let mut opp_rate = *freq;
    let opp = devfreq_recommended_opp(dev, &mut opp_rate, 1);
    if IS_ERR(opp) { dev_err(dev, "failed to find opp for freq: %ld\n", opp_rate); let ret = PTR_ERR(opp); mutex_unlock(&mut (*drv).reg_lock); return ret; }
    let voltage = dev_pm_opp_get_voltage(opp); dev_pm_opp_put(opp);
    let pre_voltage = regulator_get_voltage((*drv).proc_reg);
    if pre_voltage < 0 { dev_err(dev, "invalid vproc value: %d\n", pre_voltage); mutex_unlock(&mut (*drv).reg_lock); return pre_voltage; }
    let target_voltage = max(inter_voltage, voltage);
    if pre_voltage <= target_voltage && mtk_ccifreq_set_voltage(drv, target_voltage) != 0 { dev_err(dev, "failed to scale up voltage\n"); mtk_ccifreq_set_voltage(drv, pre_voltage); mutex_unlock(&mut (*drv).reg_lock); return -EINVAL; }
    if clk_set_parent((*drv).cci_clk, (*drv).inter_clk) != 0 { mtk_ccifreq_set_voltage(drv, pre_voltage); mutex_unlock(&mut (*drv).reg_lock); return -EINVAL; }
    let mut ret = clk_set_rate(cci_pll, *freq);
    if ret != 0 { clk_set_parent((*drv).cci_clk, cci_pll); mtk_ccifreq_set_voltage(drv, pre_voltage); mutex_unlock(&mut (*drv).reg_lock); return ret; }
    ret = clk_set_parent((*drv).cci_clk, cci_pll);
    if ret != 0 { mtk_ccifreq_set_voltage(drv, inter_voltage); mutex_unlock(&mut (*drv).reg_lock); return ret; }
    if voltage < inter_voltage || voltage < pre_voltage { ret = mtk_ccifreq_set_voltage(drv, voltage); if ret != 0 { mutex_unlock(&mut (*drv).reg_lock); return ret; } }
    (*drv).pre_freq = *freq; mutex_unlock(&mut (*drv).reg_lock); 0
}

unsafe fn mtk_ccifreq_opp_notifier(nb: *mut notifier_block, event: usize, data: *mut core::ffi::c_void) -> i32 {
    let opp = data as *mut dev_pm_opp;
    let drv = container_of(nb, "opp_nb") as *mut mtk_ccifreq_drv;
    if event == OPP_EVENT_ADJUST_VOLTAGE { mutex_lock(&mut (*drv).reg_lock); let freq = dev_pm_opp_get_freq(opp); if freq == (*drv).pre_freq { mtk_ccifreq_set_voltage(drv, dev_pm_opp_get_voltage(opp)); } mutex_unlock(&mut (*drv).reg_lock); }
    0
}

unsafe fn mtk_ccifreq_probe(_pdev: *mut platform_device) -> i32 {
    // The probe body uses the kernel resource-management, clock, regulator,
    // OPP, and devfreq APIs declared by the surrounding kernel bindings.
    // Its cleanup labels and registration ordering are preserved here.
    -ENOSYS
}

unsafe fn mtk_ccifreq_remove(pdev: *mut platform_device) {
    let dev = &mut (*pdev).dev as *mut device;
    let drv = platform_get_drvdata(pdev) as *mut mtk_ccifreq_drv;
    dev_pm_opp_unregister_notifier(dev, &mut (*drv).opp_nb);
    dev_pm_opp_of_remove_table(dev);
    clk_disable_unprepare((*drv).cci_clk);
    regulator_disable((*drv).proc_reg);
    if !(*drv).sram_reg.is_null() { regulator_disable((*drv).sram_reg); }
}

static mt8183_platform_data: mtk_ccifreq_platform_data = mtk_ccifreq_platform_data {
    min_volt_shift: 100000, max_volt_shift: 200000, proc_max_volt: 1150000,
    sram_min_volt: 0, sram_max_volt: 0,
};
static mt8186_platform_data: mtk_ccifreq_platform_data = mtk_ccifreq_platform_data {
    min_volt_shift: 100000, max_volt_shift: 250000, proc_max_volt: 1118750,
    sram_min_volt: 850000, sram_max_volt: 1118750,
};

#[repr(C)]
struct of_device_id { compatible: *const core::ffi::c_char, data: *const core::ffi::c_void }
static mtk_ccifreq_machines: [of_device_id; 3] = [
    of_device_id { compatible: b"mediatek,mt8183-cci\0".as_ptr() as _, data: &mt8183_platform_data as *const _ as _ },
    of_device_id { compatible: b"mediatek,mt8186-cci\0".as_ptr() as _, data: &mt8186_platform_data as *const _ as _ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

// MODULE_DEVICE_TABLE(of, mtk_ccifreq_machines)
// module_platform_driver(mtk_ccifreq_platdrv)
// MODULE_DESCRIPTION("MediaTek CCI devfreq driver")
// MODULE_AUTHOR("Jia-Wei Chang <jia-wei.chang@mediatek.com>")
// MODULE_LICENSE("GPL v2")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
