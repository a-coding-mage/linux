// SPDX-License-Identifier: GPL-2.0
/*
 * JZ47xx SoCs TCU clocks driver
 * Copyright (C) 2019 Paul Cercueil <paul@crapouillou.net>
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const TCU_CLK_COUNT: usize = 10;

#[repr(C)]
enum TcuClkParent { TcuParentPclk, TcuParentRtc, TcuParentExt }

#[repr(C)]
struct IngenicSocInfo {
    num_channels: u32,
    has_ost: bool,
    has_tcu_clk: bool,
    allow_missing_tcu_clk: bool,
}

#[repr(C)]
struct IngenicTcuClkInfo {
    init_data: ClkInitData,
    gate_bit: u8,
    tcsr_reg: u8,
}

#[repr(C)]
struct IngenicTcuClk {
    hw: ClkHw,
    idx: u32,
    tcu: *mut IngenicTcu,
    info: *const IngenicTcuClkInfo,
}

#[repr(C)]
struct IngenicTcu {
    soc_info: *const IngenicSocInfo,
    map: *mut Regmap,
    clk: *mut Clk,
    clocks: *mut ClkHwOnecellData,
}

static mut INGENIC_TCU: *mut IngenicTcu = core::ptr::null_mut();

unsafe fn to_tcu_clk(hw: *mut ClkHw) -> *mut IngenicTcuClk {
    (hw as *mut IngenicTcuClk).cast()
}

unsafe fn ingenic_tcu_enable(hw: *mut ClkHw) -> i32 {
    let tcu_clk = to_tcu_clk(hw); let info = (*tcu_clk).info; let tcu = (*tcu_clk).tcu;
    regmap_write((*tcu).map, TCU_REG_TSCR, BIT((*info).gate_bit as u32)); 0
}

unsafe fn ingenic_tcu_disable(hw: *mut ClkHw) {
    let tcu_clk = to_tcu_clk(hw); let info = (*tcu_clk).info; let tcu = (*tcu_clk).tcu;
    regmap_write((*tcu).map, TCU_REG_TSSR, BIT((*info).gate_bit as u32));
}

unsafe fn ingenic_tcu_is_enabled(hw: *mut ClkHw) -> i32 {
    let tcu_clk = to_tcu_clk(hw); let info = (*tcu_clk).info; let mut value = 0u32;
    regmap_read((*(*tcu_clk).tcu).map, TCU_REG_TSR, &mut value);
    if value & BIT((*info).gate_bit as u32) == 0 { 1 } else { 0 }
}

unsafe fn ingenic_tcu_enable_regs(hw: *mut ClkHw) -> bool {
    let tcu_clk = to_tcu_clk(hw); let info = (*tcu_clk).info; let tcu = (*tcu_clk).tcu;
    let enabled = ingenic_tcu_is_enabled(hw) != 0;
    regmap_write((*tcu).map, TCU_REG_TSCR, BIT((*info).gate_bit as u32)); enabled
}

unsafe fn ingenic_tcu_disable_regs(hw: *mut ClkHw) {
    let tcu_clk = to_tcu_clk(hw); let info = (*tcu_clk).info;
    regmap_write((*(*tcu_clk).tcu).map, TCU_REG_TSSR, BIT((*info).gate_bit as u32));
}

unsafe fn ingenic_tcu_get_parent(hw: *mut ClkHw) -> u8 {
    let tcu_clk = to_tcu_clk(hw); let info = (*tcu_clk).info; let mut val = 0u32;
    let ret = regmap_read((*(*tcu_clk).tcu).map, (*info).tcsr_reg, &mut val);
    WARN_ONCE(ret < 0, "Unable to read TCSR %d", (*tcu_clk).idx);
    (ffs(val & TCU_TCSR_PARENT_CLOCK_MASK) - 1) as u8
}

unsafe fn ingenic_tcu_set_parent(hw: *mut ClkHw, idx: u8) -> i32 {
    let tcu_clk = to_tcu_clk(hw); let info = (*tcu_clk).info;
    let was_enabled = ingenic_tcu_enable_regs(hw);
    let ret = regmap_update_bits((*(*tcu_clk).tcu).map, (*info).tcsr_reg,
        TCU_TCSR_PARENT_CLOCK_MASK, BIT(idx as u32));
    WARN_ONCE(ret < 0, "Unable to update TCSR %d", (*tcu_clk).idx);
    if !was_enabled { ingenic_tcu_disable_regs(hw); } 0
}

unsafe fn ingenic_tcu_recalc_rate(hw: *mut ClkHw, parent_rate: usize) -> usize {
    let tcu_clk = to_tcu_clk(hw); let info = (*tcu_clk).info; let mut prescale = 0u32;
    let ret = regmap_read((*(*tcu_clk).tcu).map, (*info).tcsr_reg, &mut prescale);
    WARN_ONCE(ret < 0, "Unable to read TCSR %d", (*tcu_clk).idx);
    prescale = (prescale & TCU_TCSR_PRESCALE_MASK) >> TCU_TCSR_PRESCALE_LSB;
    parent_rate >> (prescale * 2)
}

fn ingenic_tcu_get_prescale(rate: usize, req_rate: usize) -> u8 {
    for prescale in 0..5u8 { if rate >> (prescale * 2) <= req_rate { return prescale; } } 5
}

unsafe fn ingenic_tcu_determine_rate(_hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    let rate = (*req).best_parent_rate;
    if (*req).rate > rate { (*req).rate = rate; return 0; }
    let prescale = ingenic_tcu_get_prescale(rate, (*req).rate);
    (*req).rate = rate >> (prescale * 2); 0
}

unsafe fn ingenic_tcu_set_rate(hw: *mut ClkHw, req_rate: usize, parent_rate: usize) -> i32 {
    let tcu_clk = to_tcu_clk(hw); let info = (*tcu_clk).info;
    let prescale = ingenic_tcu_get_prescale(parent_rate, req_rate);
    let was_enabled = ingenic_tcu_enable_regs(hw);
    let ret = regmap_update_bits((*(*tcu_clk).tcu).map, (*info).tcsr_reg,
        TCU_TCSR_PRESCALE_MASK, (prescale as u32) << TCU_TCSR_PRESCALE_LSB);
    WARN_ONCE(ret < 0, "Unable to update TCSR %d", (*tcu_clk).idx);
    if !was_enabled { ingenic_tcu_disable_regs(hw); } 0
}

static INGENIC_TCU_TIMER_PARENTS: [&str; 3] = ["pclk", "rtc", "ext"];

const fn def_timer(name: &'static str, gate_bit: u8, tcsr_reg: u8) -> IngenicTcuClkInfo {
    IngenicTcuClkInfo { init_data: ClkInitData { name, parent_names: &INGENIC_TCU_TIMER_PARENTS,
        num_parents: 3, ops: &INGENIC_TCU_CLK_OPS, flags: CLK_SET_RATE_UNGATE }, gate_bit, tcsr_reg }
}

static INGENIC_TCU_CLK_INFO: [IngenicTcuClkInfo; 8] = [
    def_timer("timer0", 0, TCU_REG_TCSRc(0)), def_timer("timer1", 1, TCU_REG_TCSRc(1)),
    def_timer("timer2", 2, TCU_REG_TCSRc(2)), def_timer("timer3", 3, TCU_REG_TCSRc(3)),
    def_timer("timer4", 4, TCU_REG_TCSRc(4)), def_timer("timer5", 5, TCU_REG_TCSRc(5)),
    def_timer("timer6", 6, TCU_REG_TCSRc(6)), def_timer("timer7", 7, TCU_REG_TCSRc(7)),
];
static INGENIC_TCU_WATCHDOG_CLK_INFO: IngenicTcuClkInfo = def_timer("wdt", 16, TCU_REG_WDT_TCSR);
static INGENIC_TCU_OST_CLK_INFO: IngenicTcuClkInfo = def_timer("ost", 15, TCU_REG_OST_TCSR);

static JZ4740_SOC_INFO: IngenicSocInfo = IngenicSocInfo { num_channels: 8, has_ost: false, has_tcu_clk: true, allow_missing_tcu_clk: false };
static JZ4725B_SOC_INFO: IngenicSocInfo = IngenicSocInfo { num_channels: 6, has_ost: true, has_tcu_clk: true, allow_missing_tcu_clk: false };
static JZ4770_SOC_INFO: IngenicSocInfo = IngenicSocInfo { num_channels: 8, has_ost: true, has_tcu_clk: false, allow_missing_tcu_clk: false };
static X1000_SOC_INFO: IngenicSocInfo = IngenicSocInfo { num_channels: 8, has_ost: false, has_tcu_clk: true, allow_missing_tcu_clk: true };

unsafe fn ingenic_tcu_register_clock(tcu: *mut IngenicTcu, idx: u32, parent: TcuClkParent,
                                     info: *const IngenicTcuClkInfo, clocks: *mut ClkHwOnecellData) -> i32 {
    let tcu_clk = kzalloc::<IngenicTcuClk>(); if tcu_clk.is_null() { return -ENOMEM; }
    (*tcu_clk).hw.init = &(*info).init_data; (*tcu_clk).idx = idx;
    (*tcu_clk).info = info; (*tcu_clk).tcu = tcu;
    ingenic_tcu_enable_regs(&mut (*tcu_clk).hw);
    regmap_update_bits((*tcu).map, (*info).tcsr_reg, 0xffff, BIT(parent as u32));
    ingenic_tcu_disable_regs(&mut (*tcu_clk).hw);
    let err = clk_hw_register(core::ptr::null_mut(), &mut (*tcu_clk).hw);
    if err != 0 { kfree(tcu_clk); return err; }
    (*clocks).hws[idx as usize] = &mut (*tcu_clk).hw; 0
}

unsafe fn ingenic_tcu_probe(np: *mut DeviceNode) -> i32 {
    let id = of_match_node(INGENIC_TCU_OF_MATCH.as_ptr(), np); let map = device_node_to_regmap(np);
    if IS_ERR(map) { return PTR_ERR(map); }
    let tcu = kzalloc::<IngenicTcu>(); if tcu.is_null() { return -ENOMEM; }
    (*tcu).map = map; (*tcu).soc_info = (*id).data;
    if (*(*tcu).soc_info).has_tcu_clk {
        (*tcu).clk = of_clk_get_by_name(np, "tcu");
        if IS_ERR((*tcu).clk) {
            let ret = PTR_ERR((*tcu).clk);
            if (*(*tcu).soc_info).allow_missing_tcu_clk && ret == -EINVAL {
                pr_warn!("TCU clock missing from device tree, please update your device tree\n");
                (*tcu).clk = core::ptr::null_mut();
            } else { pr_crit!("Cannot get TCU clock from device tree\n"); kfree(tcu); return ret; }
        } else if clk_prepare_enable((*tcu).clk) != 0 { pr_crit!("Unable to enable TCU clock\n"); clk_put((*tcu).clk); kfree(tcu); return -EIO; }
    }
    (*tcu).clocks = kzalloc_flex::<ClkHwOnecellData>(TCU_CLK_COUNT); if (*tcu).clocks.is_null() { kfree(tcu); return -ENOMEM; }
    (*(*tcu).clocks).num = TCU_CLK_COUNT;
    for i in 0..(*(*tcu).soc_info).num_channels { if ingenic_tcu_register_clock(tcu, i, TcuClkParent::TcuParentExt, &INGENIC_TCU_CLK_INFO[i as usize], (*tcu).clocks) != 0 { return -EIO; } }
    if ingenic_tcu_register_clock(tcu, TCU_CLK_WDT, TcuClkParent::TcuParentRtc, &INGENIC_TCU_WATCHDOG_CLK_INFO, (*tcu).clocks) != 0 { return -EIO; }
    if (*(*tcu).soc_info).has_ost { ingenic_tcu_register_clock(tcu, TCU_CLK_OST, TcuClkParent::TcuParentExt, &INGENIC_TCU_OST_CLK_INFO, (*tcu).clocks); }
    let ret = of_clk_add_hw_provider(np, of_clk_hw_onecell_get, (*tcu).clocks); if ret != 0 { return ret; }
    INGENIC_TCU = tcu; 0
}

unsafe fn ingenic_tcu_init(np: *mut DeviceNode) {
    let ret = ingenic_tcu_probe(np);
    if ret != 0 { pr_crit!("Failed to initialize TCU clocks: %d\n", ret); }
    if IS_ENABLED_CONFIG_PM_SLEEP { register_syscore(&TCU_PM); }
}

unsafe fn tcu_pm_suspend(_data: *mut core::ffi::c_void) -> i32 { if !INGENIC_TCU.is_null() && !(*INGENIC_TCU).clk.is_null() { clk_disable((*INGENIC_TCU).clk); } 0 }
unsafe fn tcu_pm_resume(_data: *mut core::ffi::c_void) { if !INGENIC_TCU.is_null() && !(*INGENIC_TCU).clk.is_null() { clk_enable((*INGENIC_TCU).clk); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
