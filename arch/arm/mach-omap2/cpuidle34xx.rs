// SPDX-License-Identifier: GPL-2.0-only
/* linux/arch/arm/mach-omap2/cpuidle34xx.c - OMAP3 CPU IDLE Routines */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct powerdomain { pub pwrdm_clkdms: *mut *mut c_void }
#[repr(C)]
pub struct cpuidle_device;
#[repr(C)]
pub struct cpuidle_driver {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub states: [cpuidle_state; 7],
    pub state_count: usize,
    pub safe_state_index: usize,
}
#[repr(C)]
pub struct cpuidle_state {
    pub flags: c_uint,
    pub enter: Option<unsafe extern "C" fn(*mut cpuidle_device, *mut cpuidle_driver, c_int) -> c_int>,
    pub exit_latency: c_uint,
    pub target_residency: c_uint,
    pub name: *const c_char,
    pub desc: *const c_char,
}

extern "C" {
    fn omap_irq_pending() -> bool;
    fn need_resched() -> bool;
    fn clkdm_deny_idle(clkdm: *mut c_void);
    fn clkdm_allow_idle(clkdm: *mut c_void);
    fn pwrdm_set_next_pwrst(pd: *mut powerdomain, state: u8);
    fn pwrdm_read_prev_pwrst(pd: *mut powerdomain) -> u8;
    fn pwrdm_read_pwrst(pd: *mut powerdomain) -> u8;
    fn pwrdm_read_next_pwrst(pd: *mut powerdomain) -> u8;
    fn cpu_pm_enter() -> c_int;
    fn cpu_pm_exit();
    fn omap_sram_idle(save_state: bool);
    fn IS_PM34XX_ERRATUM(erratum: c_uint) -> bool;
    fn pwrdm_lookup(name: *const c_char) -> *mut powerdomain;
    fn cpu_is_omap3430() -> bool;
    fn cpuidle_register(driver: *mut cpuidle_driver, dev: *mut c_void) -> c_int;
    static mut enable_off_mode: bool;
    static mut THIS_MODULE: c_void;
}

const PWRDM_POWER_ON: u8 = 0;
const PWRDM_POWER_RET: u8 = 1;
const PWRDM_POWER_OFF: u8 = 2;
const CPUIDLE_FLAG_RCU_IDLE: c_uint = 1 << 0;
const PM_SDRC_WAKEUP_ERRATUM_i583: c_uint = 0;
const ENODEV: c_int = 19;
const OMAP_CPUIDLE_CX_NO_CLKDM_IDLE: u8 = 1 << 0;

#[repr(C)]
struct omap3_idle_statedata { mpu_state: u8, core_state: u8, per_min_state: u8, flags: u8 }

static mut mpu_pd: *mut powerdomain = core::ptr::null_mut();
static mut core_pd: *mut powerdomain = core::ptr::null_mut();
static mut per_pd: *mut powerdomain = core::ptr::null_mut();
static mut cam_pd: *mut powerdomain = core::ptr::null_mut();

static mut omap3_idle_data: [omap3_idle_statedata; 7] = [
    omap3_idle_statedata { mpu_state: PWRDM_POWER_ON, core_state: PWRDM_POWER_ON, per_min_state: PWRDM_POWER_ON, flags: OMAP_CPUIDLE_CX_NO_CLKDM_IDLE },
    omap3_idle_statedata { mpu_state: PWRDM_POWER_ON, core_state: PWRDM_POWER_ON, per_min_state: PWRDM_POWER_RET, flags: 0 },
    omap3_idle_statedata { mpu_state: PWRDM_POWER_RET, core_state: PWRDM_POWER_ON, per_min_state: PWRDM_POWER_RET, flags: 0 },
    omap3_idle_statedata { mpu_state: PWRDM_POWER_OFF, core_state: PWRDM_POWER_ON, per_min_state: PWRDM_POWER_RET, flags: 0 },
    omap3_idle_statedata { mpu_state: PWRDM_POWER_RET, core_state: PWRDM_POWER_RET, per_min_state: PWRDM_POWER_OFF, flags: 0 },
    omap3_idle_statedata { mpu_state: PWRDM_POWER_OFF, core_state: PWRDM_POWER_RET, per_min_state: PWRDM_POWER_OFF, flags: 0 },
    omap3_idle_statedata { mpu_state: PWRDM_POWER_OFF, core_state: PWRDM_POWER_OFF, per_min_state: PWRDM_POWER_OFF, flags: 0 },
];

unsafe extern "C" fn omap3_enter_idle(_dev: *mut cpuidle_device, _drv: *mut cpuidle_driver, index: c_int) -> c_int {
    let cx = &omap3_idle_data[index as usize];
    if omap_irq_pending() || need_resched() { return index; }
    if cx.flags & OMAP_CPUIDLE_CX_NO_CLKDM_IDLE != 0 {
        clkdm_deny_idle((*mpu_pd).pwrdm_clkdms[0]);
    } else {
        pwrdm_set_next_pwrst(mpu_pd, cx.mpu_state);
        pwrdm_set_next_pwrst(core_pd, cx.core_state);
    }
    if cx.mpu_state == PWRDM_POWER_OFF {
        let error = cpu_pm_enter();
        if error != 0 {
            if cx.flags & OMAP_CPUIDLE_CX_NO_CLKDM_IDLE != 0 { clkdm_allow_idle((*mpu_pd).pwrdm_clkdms[0]); }
            return index;
        }
    }
    omap_sram_idle(true);
    if cx.mpu_state == PWRDM_POWER_OFF && pwrdm_read_prev_pwrst(mpu_pd) == PWRDM_POWER_OFF { cpu_pm_exit(); }
    if cx.flags & OMAP_CPUIDLE_CX_NO_CLKDM_IDLE != 0 { clkdm_allow_idle((*mpu_pd).pwrdm_clkdms[0]); }
    index
}

unsafe extern "C" fn next_valid_state(_dev: *mut cpuidle_device, _drv: *mut cpuidle_driver, index: c_int) -> c_int {
    let mut mpu_deepest_state = PWRDM_POWER_RET;
    let mut core_deepest_state = PWRDM_POWER_RET;
    if enable_off_mode {
        mpu_deepest_state = PWRDM_POWER_OFF;
        if !IS_PM34XX_ERRATUM(PM_SDRC_WAKEUP_ERRATUM_i583) { core_deepest_state = PWRDM_POWER_OFF; }
    }
    let cx = &omap3_idle_data[index as usize];
    if cx.mpu_state >= mpu_deepest_state && cx.core_state >= core_deepest_state { return index; }
    let mut next_index = 0;
    let mut idx = index - 1;
    while idx >= 0 {
        let state = &omap3_idle_data[idx as usize];
        if state.mpu_state >= mpu_deepest_state && state.core_state >= core_deepest_state { next_index = idx; break; }
        idx -= 1;
    }
    next_index
}

unsafe extern "C" fn omap3_enter_idle_bm(dev: *mut cpuidle_device, drv: *mut cpuidle_driver, index: c_int) -> c_int {
    let new_state_idx = if pwrdm_read_pwrst(cam_pd) == PWRDM_POWER_ON { (*drv).safe_state_index as c_int } else { next_valid_state(dev, drv, index) };
    let cx = &omap3_idle_data[new_state_idx as usize];
    let per_saved_state = pwrdm_read_next_pwrst(per_pd);
    let per_next_state = if per_saved_state < cx.per_min_state { pwrdm_set_next_pwrst(per_pd, cx.per_min_state); cx.per_min_state } else { per_saved_state };
    let ret = omap3_enter_idle(dev, drv, new_state_idx);
    if per_next_state != per_saved_state { pwrdm_set_next_pwrst(per_pd, per_saved_state); }
    ret
}

macro_rules! state { ($lat:expr, $res:expr, $name:literal, $desc:literal) => { cpuidle_state { flags: CPUIDLE_FLAG_RCU_IDLE, enter: Some(omap3_enter_idle_bm), exit_latency: $lat, target_residency: $res, name: concat!($name, "\0").as_ptr() as *const c_char, desc: concat!($desc, "\0").as_ptr() as *const c_char } }; }
static mut omap3_idle_driver: cpuidle_driver = cpuidle_driver { name: b"omap3_idle\0".as_ptr() as *const c_char, owner: core::ptr::addr_of_mut!(THIS_MODULE), states: [state!(4,5,"C1","MPU ON + CORE ON"),state!(20,30,"C2","MPU ON + CORE ON"),state!(100,300,"C3","MPU RET + CORE ON"),state!(3300,4000,"C4","MPU OFF + CORE ON"),state!(10000,12000,"C5","MPU RET + CORE RET"),state!(11500,15000,"C6","MPU OFF + CORE RET"),state!(40000,30000,"C7","MPU OFF + CORE OFF")], state_count: 7, safe_state_index: 0 };
static mut omap3430_idle_driver: cpuidle_driver = cpuidle_driver { name: b"omap3430_idle\0".as_ptr() as *const c_char, owner: core::ptr::addr_of_mut!(THIS_MODULE), states: [state!(272,5,"C1","MPU ON + CORE ON"),state!(286,309,"C2","MPU ON + CORE ON"),state!(517,46057,"C3","MPU RET + CORE ON"),state!(3495,46057,"C4","MPU OFF + CORE ON"),state!(2001,46057,"C5","MPU RET + CORE RET"),state!(11714,484329,"C6","MPU OFF + CORE RET"),state!(22779,484329,"C7","MPU OFF + CORE OFF")], state_count: 7, safe_state_index: 0 };

pub unsafe extern "C" fn omap3_idle_init() -> c_int {
    mpu_pd = pwrdm_lookup(b"mpu_pwrdm\0".as_ptr() as *const c_char); core_pd = pwrdm_lookup(b"core_pwrdm\0".as_ptr() as *const c_char); per_pd = pwrdm_lookup(b"per_pwrdm\0".as_ptr() as *const c_char); cam_pd = pwrdm_lookup(b"cam_pwrdm\0".as_ptr() as *const c_char);
    if mpu_pd.is_null() || core_pd.is_null() || per_pd.is_null() || cam_pd.is_null() { return -ENODEV; }
    if cpu_is_omap3430() { cpuidle_register(&mut omap3430_idle_driver, core::ptr::null_mut()) } else { cpuidle_register(&mut omap3_idle_driver, core::ptr::null_mut()) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
