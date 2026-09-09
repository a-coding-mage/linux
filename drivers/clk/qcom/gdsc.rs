// SPDX-License-Identifier: GPL-2.0-only
/* Translated from gdsc.c; Linux kernel dependencies are supplied externally. */

const PWR_ON_MASK: u32 = 1 << 31;
const EN_REST_WAIT_MASK: u32 = 0x00f0_0000;
const EN_FEW_WAIT_MASK: u32 = 0x000f_0000;
const CLK_DIS_WAIT_MASK: u32 = 0x0000_f000;
const SW_OVERRIDE_MASK: u32 = 1 << 2;
const HW_CONTROL_MASK: u32 = 1 << 1;
const SW_COLLAPSE_MASK: u32 = 1;
const GMEM_CLAMP_IO_MASK: u32 = 1;
const GMEM_RESET_MASK: u32 = 1 << 4;
const GDSC_POWER_UP_COMPLETE: u32 = 1 << 16;
const GDSC_POWER_DOWN_COMPLETE: u32 = 1 << 15;
const GDSC_RETAIN_FF_ENABLE: u32 = 1 << 11;
const CFG_GDSCR_OFFSET: u32 = 0x4;
const EN_REST_WAIT_VAL: u32 = 0x2;
const EN_FEW_WAIT_VAL: u32 = 0x8;
const CLK_DIS_WAIT_VAL: u32 = 0x2;
const EN_REST_WAIT_SHIFT: u32 = 20;
const EN_FEW_WAIT_SHIFT: u32 = 16;
const CLK_DIS_WAIT_SHIFT: u32 = 12;
const RETAIN_MEM: u32 = 1 << 14;
const RETAIN_PERIPH: u32 = 1 << 13;
const STATUS_POLL_TIMEOUT_US: i64 = 2000;
const TIMEOUT_US: u32 = 500;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum GdscStatus { GDSC_OFF, GDSC_ON }

unsafe fn domain_to_gdsc(domain: *mut generic_pm_domain) -> *mut gdsc {
    /* container_of(domain, struct gdsc, pd); layout is supplied by the kernel binding. */
    domain as *mut gdsc
}

unsafe fn gdsc_check_status(sc: *mut gdsc, status: GdscStatus) -> i32 {
    let reg: u32;
    let mut val: u32 = 0;
    if ((*sc).flags & POLL_CFG_GDSCR) != 0 { reg = (*sc).gdscr + CFG_GDSCR_OFFSET; }
    else if (*sc).gds_hw_ctrl != 0 { reg = (*sc).gds_hw_ctrl; }
    else { reg = (*sc).gdscr; }
    let ret = regmap_read((*sc).regmap, reg, &mut val);
    if ret != 0 { return ret; }
    if ((*sc).flags & POLL_CFG_GDSCR) != 0 {
        return match status { GdscStatus::GDSC_ON => ((val & GDSC_POWER_UP_COMPLETE) != 0) as i32,
            GdscStatus::GDSC_OFF => ((val & GDSC_POWER_DOWN_COMPLETE) != 0) as i32 };
    }
    match status { GdscStatus::GDSC_ON => ((val & PWR_ON_MASK) != 0) as i32,
        GdscStatus::GDSC_OFF => ((val & PWR_ON_MASK) == 0) as i32 }
}

unsafe fn gdsc_hwctrl(sc: *mut gdsc, en: bool) -> i32 {
    regmap_update_bits((*sc).regmap, (*sc).gdscr, HW_CONTROL_MASK, if en { HW_CONTROL_MASK } else { 0 })
}

unsafe fn gdsc_poll_status(sc: *mut gdsc, status: GdscStatus) -> i32 {
    let start = ktime_get();
    loop {
        let ret = gdsc_check_status(sc, status);
        if ret < 0 { return ret; } if ret != 0 { return 0; }
        if ktime_us_delta(ktime_get(), start) >= STATUS_POLL_TIMEOUT_US { break; }
    }
    let ret = gdsc_check_status(sc, status);
    if ret < 0 { ret } else if ret != 0 { 0 } else { -ETIMEDOUT }
}

unsafe fn gdsc_update_collapse_bit(sc: *mut gdsc, val: bool) -> i32 {
    let (reg, mask) = if (*sc).collapse_mask != 0 { ((*sc).collapse_ctrl, (*sc).collapse_mask) }
        else { ((*sc).gdscr, SW_COLLAPSE_MASK) };
    let ret = regmap_update_bits((*sc).regmap, reg, mask, if val { mask } else { 0 });
    if ret != 0 { return ret; } 0
}

unsafe fn gdsc_toggle_logic(sc: *mut gdsc, status: GdscStatus, wait: bool) -> i32 {
    let mut ret;
    if status == GdscStatus::GDSC_ON && !(*sc).rsupply.is_null() { ret = regulator_enable((*sc).rsupply); if ret < 0 { return ret; } }
    if status == GdscStatus::GDSC_ON { ret = icc_set_bw((*sc).icc_path, 1, 1); if ret != 0 { if !(*sc).rsupply.is_null() { regulator_disable((*sc).rsupply); } return ret; } }
    ret = gdsc_update_collapse_bit(sc, status == GdscStatus::GDSC_OFF);
    if ((*sc).flags & VOTABLE) != 0 && status == GdscStatus::GDSC_OFF && !wait { udelay(TIMEOUT_US); return 0; }
    if (*sc).gds_hw_ctrl != 0 { udelay(1); }
    ret = gdsc_poll_status(sc, status);
    WARN(ret != 0, "%s status stuck at 'o%s'", (*sc).pd.name, if status == GdscStatus::GDSC_OFF { "ff" } else { "n" });
    if ret == 0 && status == GdscStatus::GDSC_OFF { ret = icc_set_bw((*sc).icc_path, 0, 0); if ret != 0 { return ret; } }
    if ret == 0 && status == GdscStatus::GDSC_OFF && !(*sc).rsupply.is_null() { ret = regulator_disable((*sc).rsupply); if ret < 0 { return ret; } }
    ret
}

unsafe fn gdsc_deassert_reset(sc: *mut gdsc) -> i32 { for i in 0..(*sc).reset_count { ((*sc).rcdev).ops.deassert((*sc).rcdev, (*sc).resets.add(i)); } 0 }
unsafe fn gdsc_assert_reset(sc: *mut gdsc) -> i32 { for i in 0..(*sc).reset_count { ((*sc).rcdev).ops.assert((*sc).rcdev, (*sc).resets.add(i)); } 0 }
unsafe fn gdsc_force_mem_on(sc: *mut gdsc) { let mut mask = RETAIN_MEM; if ((*sc).flags & NO_RET_PERIPH) == 0 { mask |= RETAIN_PERIPH; } for i in 0..(*sc).cxc_count { regmap_update_bits((*sc).regmap, *(*sc).cxcs.add(i), mask, mask); } }
unsafe fn gdsc_clear_mem_on(sc: *mut gdsc) { let mut mask = RETAIN_MEM; if ((*sc).flags & NO_RET_PERIPH) == 0 { mask |= RETAIN_PERIPH; } for i in 0..(*sc).cxc_count { regmap_update_bits((*sc).regmap, *(*sc).cxcs.add(i), mask, 0); } }
unsafe fn gdsc_deassert_clamp_io(sc: *mut gdsc) { regmap_update_bits((*sc).regmap, (*sc).clamp_io_ctrl, GMEM_CLAMP_IO_MASK, 0); }
unsafe fn gdsc_assert_clamp_io(sc: *mut gdsc) { regmap_update_bits((*sc).regmap, (*sc).clamp_io_ctrl, GMEM_CLAMP_IO_MASK, 1); }
unsafe fn gdsc_assert_reset_aon(sc: *mut gdsc) { regmap_update_bits((*sc).regmap, (*sc).clamp_io_ctrl, GMEM_RESET_MASK, 1); udelay(1); regmap_update_bits((*sc).regmap, (*sc).clamp_io_ctrl, GMEM_RESET_MASK, 0); }
unsafe fn gdsc_retain_ff_on(sc: *mut gdsc) { regmap_update_bits((*sc).regmap, (*sc).gdscr, GDSC_RETAIN_FF_ENABLE, GDSC_RETAIN_FF_ENABLE); }

unsafe fn gdsc_enable(domain: *mut generic_pm_domain) -> i32 {
    let sc = domain_to_gdsc(domain); if (*sc).pwrsts == PWRSTS_ON { return gdsc_deassert_reset(sc); }
    if ((*sc).flags & SW_RESET) != 0 { gdsc_assert_reset(sc); udelay(1); gdsc_deassert_reset(sc); }
    if ((*sc).flags & CLAMP_IO) != 0 { if ((*sc).flags & AON_RESET) != 0 { gdsc_assert_reset_aon(sc); } gdsc_deassert_clamp_io(sc); }
    let mut ret = gdsc_toggle_logic(sc, GdscStatus::GDSC_ON, false); if ret != 0 { return ret; }
    if ((*sc).pwrsts & PWRSTS_OFF) != 0 { gdsc_force_mem_on(sc); }
    udelay(1); if ((*sc).flags & RETAIN_FF_ENABLE) != 0 { gdsc_retain_ff_on(sc); }
    if ((*sc).flags & HW_CTRL) != 0 { ret = gdsc_hwctrl(sc, true); if ret != 0 { return ret; } udelay(1); } 0
}

unsafe fn gdsc_disable(domain: *mut generic_pm_domain) -> i32 {
    let sc = domain_to_gdsc(domain); if (*sc).pwrsts == PWRSTS_ON { return gdsc_assert_reset(sc); }
    let mut ret;
    if ((*sc).flags & HW_CTRL) != 0 { ret = gdsc_hwctrl(sc, false); if ret < 0 { return ret; } udelay(1); ret = gdsc_poll_status(sc, GdscStatus::GDSC_ON); if ret != 0 { return ret; } }
    if ((*sc).pwrsts & PWRSTS_OFF) != 0 { gdsc_clear_mem_on(sc); }
    if (*sc).pwrsts == PWRSTS_RET_ON { return 0; }
    ret = gdsc_toggle_logic(sc, GdscStatus::GDSC_OFF, (*domain).synced_poweroff); if ret != 0 { return ret; }
    if ((*sc).flags & CLAMP_IO) != 0 { gdsc_assert_clamp_io(sc); } 0
}

unsafe fn gdsc_set_hwmode(domain: *mut generic_pm_domain, _dev: *mut device, mode: bool) -> i32 { let sc = domain_to_gdsc(domain); let ret = gdsc_hwctrl(sc, mode); if ret != 0 { return ret; } udelay(1); if !mode { gdsc_poll_status(sc, GdscStatus::GDSC_ON) } else { 0 } }
unsafe fn gdsc_get_hwmode(domain: *mut generic_pm_domain, _dev: *mut device) -> bool { let sc = domain_to_gdsc(domain); let mut val = 0; regmap_read((*sc).regmap, (*sc).gdscr, &mut val); (val & HW_CONTROL_MASK) != 0 }

unsafe fn gdsc_init(sc: *mut gdsc) -> i32 {
    let mask = HW_CONTROL_MASK | SW_OVERRIDE_MASK | EN_REST_WAIT_MASK | EN_FEW_WAIT_MASK | CLK_DIS_WAIT_MASK;
    if (*sc).en_rest_wait_val == 0 { (*sc).en_rest_wait_val = EN_REST_WAIT_VAL; } if (*sc).en_few_wait_val == 0 { (*sc).en_few_wait_val = EN_FEW_WAIT_VAL; } if (*sc).clk_dis_wait_val == 0 { (*sc).clk_dis_wait_val = CLK_DIS_WAIT_VAL; }
    let val = (*sc).en_rest_wait_val << EN_REST_WAIT_SHIFT | (*sc).en_few_wait_val << EN_FEW_WAIT_SHIFT | (*sc).clk_dis_wait_val << CLK_DIS_WAIT_SHIFT;
    let mut ret = regmap_update_bits((*sc).regmap, (*sc).gdscr, mask, val); if ret != 0 { return ret; }
    if (*sc).pwrsts == PWRSTS_ON { ret = gdsc_toggle_logic(sc, GdscStatus::GDSC_ON, false); if ret != 0 { return ret; } }
    let mut on = gdsc_check_status(sc, GdscStatus::GDSC_ON); if on < 0 { return on; }
    if on != 0 { if !(*sc).rsupply.is_null() { ret = regulator_enable((*sc).rsupply); if ret < 0 { return ret; } } if ((*sc).flags & VOTABLE) != 0 { ret = gdsc_update_collapse_bit(sc, false); if ret != 0 { if !(*sc).rsupply.is_null() { regulator_disable((*sc).rsupply); } return ret; } } if ((*sc).flags & RETAIN_FF_ENABLE) != 0 { gdsc_retain_ff_on(sc); } if ((*sc).flags & HW_CTRL) != 0 { ret = gdsc_hwctrl(sc, true); if ret < 0 { if !(*sc).rsupply.is_null() { regulator_disable((*sc).rsupply); } return ret; } } }
    else if ((*sc).flags & ALWAYS_ON) != 0 { ret = gdsc_enable(&mut (*sc).pd); if ret != 0 { return ret; } on = 1; }
    if on != 0 || ((*sc).pwrsts & PWRSTS_RET) != 0 { gdsc_force_mem_on(sc); } else { gdsc_clear_mem_on(sc); } 0
}

pub unsafe fn gdsc_gx_do_nothing_enable(domain: *mut generic_pm_domain) -> i32 { let sc = domain_to_gdsc(domain); if !(*sc).rsupply.is_null() { regulator_enable((*sc).rsupply) } else { 0 } }
pub unsafe fn gdsc_gx_disable(domain: *mut generic_pm_domain) -> i32 { let sc = domain_to_gdsc(domain); if (*domain).synced_poweroff { gdsc_disable(domain) } else if !(*sc).rsupply.is_null() { regulator_disable((*sc).rsupply) } else { 0 } }

/* External kernel allocation/provider and subdomain APIs used by the original
 * registration path remain declarations supplied by the surrounding binding. */
extern "C" {
    pub fn gdsc_register(desc: *mut gdsc_desc, rcdev: *mut reset_controller_dev, regmap: *mut regmap) -> i32;
    pub fn gdsc_unregister(desc: *mut gdsc_desc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
