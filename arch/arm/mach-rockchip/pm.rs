// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014, Fuzhou Rockchip Electronics Co., Ltd
 * Author: Tony Xie <tony.xie@rock-chips.com>
 */

// Kernel headers and "pm.h" are supplied by the surrounding translation unit.

const ROCKCHIP_ARM_OFF_LOGIC_NORMAL: i32 = 0;
const ROCKCHIP_ARM_OFF_LOGIC_DEEP: i32 = 1;

#[repr(C)]
struct RockchipPmData {
    ops: *const PlatformSuspendOps,
    init: Option<unsafe extern "C" fn(*mut DeviceNode) -> i32>,
}

static mut RK3288_BOOTRAM_BASE: *mut core::ffi::c_void = core::ptr::null_mut();
static mut RK3288_BOOTRAM_PHY: PhysAddr = 0;

static mut PMU_REGMAP: *mut Regmap = core::ptr::null_mut();
static mut SGRF_REGMAP: *mut Regmap = core::ptr::null_mut();
static mut GRF_REGMAP: *mut Regmap = core::ptr::null_mut();

static mut RK3288_PMU_PWR_MODE_CON: u32 = 0;
static mut RK3288_SGRF_SOC_CON0: u32 = 0;
static mut RK3288_SGRF_CPU_CON0: u32 = 0;

#[inline]
unsafe fn rk3288_l2_config() -> u32 {
    let l2ctlr: u32;
    core::arch::asm!("mrc p15, 1, {0}, c9, c0, 2", out(reg) l2ctlr);
    l2ctlr
}

unsafe fn rk3288_config_bootdata() {
    rkpm_bootdata_cpusp = RK3288_BOOTRAM_PHY + (SZ_4K - 8);
    rkpm_bootdata_cpu_code = __pa_symbol(cpu_resume);
    rkpm_bootdata_l2ctlr_f = 1;
    rkpm_bootdata_l2ctlr = rk3288_l2_config();
}

const GRF_UOC0_CON0: u32 = 0x320;
const GRF_UOC1_CON0: u32 = 0x334;
const GRF_UOC2_CON0: u32 = 0x348;
const GRF_SIDDQ: u32 = BIT(13);

unsafe fn rk3288_slp_disable_osc() -> bool {
    let reg_offset = [GRF_UOC0_CON0, GRF_UOC1_CON0, GRF_UOC2_CON0];
    let mut reg = 0u32;
    for offset in reg_offset {
        regmap_read(GRF_REGMAP, offset, &mut reg);
        if reg & GRF_SIDDQ == 0 {
            return false;
        }
    }
    true
}

unsafe fn rk3288_slp_mode_set(level: i32) {
    let mut mode_set: u32;
    let mut mode_set1: u32;
    let osc_disable = rk3288_slp_disable_osc();

    regmap_read(SGRF_REGMAP, RK3288_SGRF_CPU_CON0, &mut RK3288_SGRF_CPU_CON0);
    regmap_read(SGRF_REGMAP, RK3288_SGRF_SOC_CON0, &mut RK3288_SGRF_SOC_CON0);
    regmap_read(PMU_REGMAP, RK3288_PMU_PWRMODE_CON, &mut RK3288_PMU_PWR_MODE_CON);

    regmap_write(SGRF_REGMAP, RK3288_SGRF_SOC_CON0,
        SGRF_PCLK_WDT_GATE | SGRF_FAST_BOOT_EN |
        SGRF_PCLK_WDT_GATE_WRITE | SGRF_FAST_BOOT_EN_WRITE);
    regmap_write(SGRF_REGMAP, RK3288_SGRF_CPU_CON0, SGRF_DAPDEVICEEN_WRITE);
    regmap_write(SGRF_REGMAP, RK3288_SGRF_FAST_BOOT_ADDR, RK3288_BOOTRAM_PHY);

    mode_set = BIT(PMU_GLOBAL_INT_DISABLE) | BIT(PMU_L2FLUSH_EN) |
        BIT(PMU_SREF0_ENTER_EN) | BIT(PMU_SREF1_ENTER_EN) |
        BIT(PMU_DDR0_GATING_EN) | BIT(PMU_DDR1_GATING_EN) |
        BIT(PMU_PWR_MODE_EN) | BIT(PMU_CHIP_PD_EN) | BIT(PMU_SCU_EN);
    mode_set1 = BIT(PMU_CLR_CORE) | BIT(PMU_CLR_CPUP);

    if level == ROCKCHIP_ARM_OFF_LOGIC_DEEP {
        mode_set |= BIT(PMU_BUS_PD_EN) | BIT(PMU_PMU_USE_LF) |
            BIT(PMU_DDR1IO_RET_EN) | BIT(PMU_DDR0IO_RET_EN) |
            BIT(PMU_ALIVE_USE_LF) | BIT(PMU_PLL_PD_EN);
        if osc_disable { mode_set |= BIT(PMU_OSC_24M_DIS); }
        mode_set1 |= BIT(PMU_CLR_ALIVE) | BIT(PMU_CLR_BUS) |
            BIT(PMU_CLR_PERI) | BIT(PMU_CLR_DMA);
        regmap_write(PMU_REGMAP, RK3288_PMU_WAKEUP_CFG1, PMU_ARMINT_WAKEUP_EN);
        regmap_write(PMU_REGMAP, RK3288_PMU_STABL_CNT, 32 * 30);
        regmap_write(PMU_REGMAP, RK3288_PMU_OSC_CNT, if osc_disable { 32 * 30 } else { 0 });
    } else {
        mode_set |= BIT(PMU_CLK_CORE_SRC_GATE_EN);
        regmap_write(PMU_REGMAP, RK3288_PMU_WAKEUP_CFG1,
            PMU_ARMINT_WAKEUP_EN | PMU_GPIOINT_WAKEUP_EN);
        regmap_write(PMU_REGMAP, RK3288_PMU_STABL_CNT, 24000 * 30);
        regmap_write(PMU_REGMAP, RK3288_PMU_OSC_CNT, 0);
    }
    regmap_write(PMU_REGMAP, RK3288_PMU_PWRMODE_CON, mode_set);
    regmap_write(PMU_REGMAP, RK3288_PMU_PWRMODE_CON1, mode_set1);
}

unsafe fn rk3288_slp_mode_set_resume() {
    regmap_write(SGRF_REGMAP, RK3288_SGRF_CPU_CON0,
        RK3288_SGRF_CPU_CON0 | SGRF_DAPDEVICEEN_WRITE);
    regmap_write(PMU_REGMAP, RK3288_PMU_PWRMODE_CON, RK3288_PMU_PWR_MODE_CON);
    regmap_write(SGRF_REGMAP, RK3288_SGRF_SOC_CON0,
        RK3288_SGRF_SOC_CON0 | SGRF_PCLK_WDT_GATE_WRITE | SGRF_FAST_BOOT_EN_WRITE);
}

unsafe extern "C" fn rockchip_lpmode_enter(_arg: usize) -> i32 {
    flush_cache_all();
    cpu_do_idle();
    pr_err!("%s: Failed to suspend\n", "rockchip_lpmode_enter");
    1
}

unsafe extern "C" fn rk3288_suspend_enter(_state: SuspendState) -> i32 {
    local_fiq_disable();
    rk3288_slp_mode_set(ROCKCHIP_ARM_OFF_LOGIC_NORMAL);
    cpu_suspend(0, Some(rockchip_lpmode_enter));
    rk3288_slp_mode_set_resume();
    local_fiq_enable();
    0
}

unsafe extern "C" fn rk3288_suspend_prepare() -> i32 { regulator_suspend_prepare(PM_SUSPEND_MEM) }

unsafe extern "C" fn rk3288_suspend_finish() {
    if regulator_suspend_finish() != 0 { pr_err!("%s: Suspend finish failed\n", "rk3288_suspend_finish"); }
}

unsafe extern "C" fn rk3288_suspend_init(np: *mut DeviceNode) -> i32 {
    let mut sram_np: *mut DeviceNode;
    let mut res = Resource::default();
    let mut ret: i32;

    PMU_REGMAP = syscon_node_to_regmap(np);
    if IS_ERR(PMU_REGMAP) { pr_err!("%s: could not find pmu regmap\n", "rk3288_suspend_init"); return PTR_ERR(PMU_REGMAP); }
    SGRF_REGMAP = syscon_regmap_lookup_by_compatible("rockchip,rk3288-sgrf\0".as_ptr() as *const i8);
    if IS_ERR(SGRF_REGMAP) { pr_err!("%s: could not find sgrf regmap\n", "rk3288_suspend_init"); return PTR_ERR(SGRF_REGMAP); }
    GRF_REGMAP = syscon_regmap_lookup_by_compatible("rockchip,rk3288-grf\0".as_ptr() as *const i8);
    if IS_ERR(GRF_REGMAP) { pr_err!("%s: could not find grf regmap\n", "rk3288_suspend_init"); return PTR_ERR(GRF_REGMAP); }
    sram_np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "rockchip,rk3288-pmu-sram\0".as_ptr() as *const i8);
    if sram_np.is_null() { pr_err!("%s: could not find bootram dt node\n", "rk3288_suspend_init"); return -ENODEV; }
    RK3288_BOOTRAM_BASE = of_iomap(sram_np, 0);
    if RK3288_BOOTRAM_BASE.is_null() { pr_err!("%s: could not map bootram base\n", "rk3288_suspend_init"); of_node_put(sram_np); return -ENOMEM; }
    ret = of_address_to_resource(sram_np, 0, &mut res);
    if ret != 0 { pr_err!("%s: could not get bootram phy addr\n", "rk3288_suspend_init"); of_node_put(sram_np); return ret; }
    RK3288_BOOTRAM_PHY = res.start;
    of_node_put(sram_np);
    rk3288_config_bootdata();
    core::ptr::copy_nonoverlapping(rockchip_slp_cpu_resume as *const u8, RK3288_BOOTRAM_BASE as *mut u8, rk3288_bootram_sz as usize);
    0
}

static RK3288_SUSPEND_OPS: PlatformSuspendOps = PlatformSuspendOps { enter: Some(rk3288_suspend_enter), valid: Some(suspend_valid_only_mem), prepare: Some(rk3288_suspend_prepare), finish: Some(rk3288_suspend_finish) };
static RK3288_PM_DATA: RockchipPmData = RockchipPmData { ops: &RK3288_SUSPEND_OPS, init: Some(rk3288_suspend_init) };

static ROCKCHIP_PMU_OF_DEVICE_IDS: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "rockchip,rk3288-pmu\0".as_ptr() as *const i8, data: &RK3288_PM_DATA as *const _ as *const core::ffi::c_void },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

pub unsafe extern "C" fn rockchip_suspend_init() {
    let mut match_: *const OfDeviceId = core::ptr::null();
    let np = of_find_matching_node_and_match(core::ptr::null_mut(), ROCKCHIP_PMU_OF_DEVICE_IDS.as_ptr(), &mut match_);
    if match_.is_null() { pr_err!("Failed to find PMU node\n"); of_node_put(np); return; }
    let pm_data = (*(match_)).data as *const RockchipPmData;
    if let Some(init) = (*pm_data).init {
        let ret = init(np);
        if ret != 0 { pr_err!("%s: matches init error %d\n", "rockchip_suspend_init", ret); of_node_put(np); return; }
    }
    suspend_set_ops((*pm_data).ops);
    of_node_put(np);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
