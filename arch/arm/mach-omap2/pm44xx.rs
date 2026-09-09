// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP4+ Power Management Routines
 *
 * Copyright (C) 2010-2013 Texas Instruments, Inc.
 * Rajendra Nayak <rnayak@ti.com>
 * Santosh Shilimkar <santosh.shilimkar@ti.com>
 */

// External kernel declarations and build-time configuration are supplied by
// the surrounding translation unit.

#[repr(C)]
pub struct powerdomain {
    pub name: *const core::ffi::c_char,
    pub pwrsts: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct clockdomain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct power_state {
    pub pwrdm: *mut powerdomain,
    pub next_state: u32,
    pub next_logic_state: u32,
    // Present when CONFIG_SUSPEND is enabled.
    #[cfg(CONFIG_SUSPEND)]
    pub saved_state: u32,
    #[cfg(CONFIG_SUSPEND)]
    pub saved_logic_state: u32,
    pub node: list_head,
}

#[repr(C)]
pub struct static_dep_map {
    pub from: *const core::ffi::c_char,
    pub to: *const core::ffi::c_char,
}

pub static mut pm44xx_errata: u16 = 0;
static mut cpu_suspend_state: u32 = PWRDM_POWER_OFF;
static mut pwrst_list: list_head = list_head {
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};

extern "C" {
    static mut arm_pm_idle: Option<unsafe extern "C" fn()>;

    fn smp_processor_id() -> u32;
    fn pwrdm_read_next_pwrst(pwrdm: *mut powerdomain) -> u32;
    fn pwrdm_read_logic_retst(pwrdm: *mut powerdomain) -> u32;
    fn omap_set_pwrdm_state(pwrdm: *mut powerdomain, state: u32) -> i32;
    fn pwrdm_set_logic_retst(pwrdm: *mut powerdomain, state: u32);
    fn omap4_enter_lowpower(cpu_id: u32, state: u32, save_state: bool);
    fn pwrdm_read_prev_pwrst(pwrdm: *mut powerdomain) -> i32;
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn pwrdm_get_valid_lp_state(pwrdm: *mut powerdomain, logic: bool, state: u32) -> u32;
    fn omap_do_wfi();
    fn clkdm_lookup(name: *const core::ffi::c_char) -> *mut clockdomain;
    fn clkdm_add_wkdep(from: *mut clockdomain, to: *mut clockdomain) -> i32;
    fn cpu_is_omap446x() -> bool;
    fn soc_is_omap54xx() -> bool;
    fn soc_is_dra7xx() -> bool;
    fn cpu_is_omap44xx() -> bool;
    fn omap_rev() -> u32;
    fn pwrdm_for_each(callback: unsafe extern "C" fn(*mut powerdomain, *mut core::ffi::c_void) -> i32, data: *mut core::ffi::c_void) -> i32;
    fn omap4_mpuss_init() -> i32;
    fn clkdm_for_each(callback: unsafe extern "C" fn(*mut clockdomain, *mut core::ffi::c_void) -> i32, data: *mut core::ffi::c_void) -> i32;
    fn omap_pm_clkdms_setup(clkdm: *mut clockdomain, unused: *mut core::ffi::c_void) -> i32;
    fn omap_common_suspend_init(suspend: Option<unsafe extern "C" fn() -> i32>);
    fn omap4_idle_init();
}

const PWRDM_POWER_OFF: u32 = 0;
const PWRDM_POWER_RET: u32 = 1;
const GFP_ATOMIC: u32 = 0;

#[cfg(CONFIG_SUSPEND)]
unsafe extern "C" fn omap4_pm_suspend() -> i32 {
    let mut pwrst: *mut power_state;
    let mut state: i32;
    let mut ret: i32 = 0;
    let cpu_id = smp_processor_id();

    // Save current powerdomain state
    pwrst = (*core::ptr::addr_of_mut!(pwrst_list)).next as *mut power_state;
    while !pwrst.is_null() {
        (*pwrst).saved_state = pwrdm_read_next_pwrst((*pwrst).pwrdm);
        (*pwrst).saved_logic_state = pwrdm_read_logic_retst((*pwrst).pwrdm);
        pwrst = (*pwrst).node.next as *mut power_state;
    }

    // Set targeted power domain states by suspend
    pwrst = pwrst_list.next as *mut power_state;
    while !pwrst.is_null() {
        omap_set_pwrdm_state((*pwrst).pwrdm, (*pwrst).next_state);
        pwrdm_set_logic_retst((*pwrst).pwrdm, (*pwrst).next_logic_state);
        pwrst = (*pwrst).node.next as *mut power_state;
    }

    /* Only master CPU follows suspend path; other CPUs follow hotplug path. */
    omap4_enter_lowpower(cpu_id, cpu_suspend_state, false);

    // Restore next powerdomain state
    pwrst = pwrst_list.next as *mut power_state;
    while !pwrst.is_null() {
        state = pwrdm_read_prev_pwrst((*pwrst).pwrdm);
        if state as u32 > (*pwrst).next_state {
            ret = -1;
        }
        omap_set_pwrdm_state((*pwrst).pwrdm, (*pwrst).saved_state);
        pwrdm_set_logic_retst((*pwrst).pwrdm, (*pwrst).saved_logic_state);
        pwrst = (*pwrst).node.next as *mut power_state;
    }
    ret;
}

#[cfg(not(CONFIG_SUSPEND))]
static omap4_pm_suspend: Option<unsafe extern "C" fn() -> i32> = None;

unsafe extern "C" fn pwrdms_setup(pwrdm: *mut powerdomain, _unused: *mut core::ffi::c_void) -> i32 {
    if (*pwrdm).pwrsts.is_null() {
        return 0;
    }
    if !core::slice::from_raw_parts((*pwrdm).name as *const u8, 3).is_empty()
        && core::slice::from_raw_parts((*pwrdm).name as *const u8, 3) == b"cpu"
    {
        cpu_suspend_state = PWRDM_POWER_RET;
        return 0;
    }
    let pwrst = kmalloc(core::mem::size_of::<power_state>(), GFP_ATOMIC) as *mut power_state;
    if pwrst.is_null() { return -12; }
    (*pwrst).pwrdm = pwrdm;
    (*pwrst).next_state = pwrdm_get_valid_lp_state(pwrdm, false, PWRDM_POWER_RET);
    (*pwrst).next_logic_state = pwrdm_get_valid_lp_state(pwrdm, true, PWRDM_POWER_OFF);
    (*pwrst).node.next = pwrst as *mut list_head;
    (*pwrst).node.prev = pwrst_list.prev;
    pwrst_list.next = pwrst as *mut list_head;
    omap_set_pwrdm_state((*pwrst).pwrdm, (*pwrst).next_state)
}

unsafe extern "C" fn omap_default_idle() { omap_do_wfi(); }

static omap4_static_dep_map: [static_dep_map; 6] = [
    static_dep_map { from: b"mpuss_clkdm\0".as_ptr() as _, to: b"l3_emif_clkdm\0".as_ptr() as _ },
    static_dep_map { from: b"mpuss_clkdm\0".as_ptr() as _, to: b"l3_1_clkdm\0".as_ptr() as _ },
    static_dep_map { from: b"mpuss_clkdm\0".as_ptr() as _, to: b"l3_2_clkdm\0".as_ptr() as _ },
    static_dep_map { from: b"ducati_clkdm\0".as_ptr() as _, to: b"l3_1_clkdm\0".as_ptr() as _ },
    static_dep_map { from: b"ducati_clkdm\0".as_ptr() as _, to: b"l3_2_clkdm\0".as_ptr() as _ },
    static_dep_map { from: core::ptr::null(), to: core::ptr::null() },
];

static omap5_dra7_static_dep_map: [static_dep_map; 2] = [
    static_dep_map { from: b"mpu_clkdm\0".as_ptr() as _, to: b"emif_clkdm\0".as_ptr() as _ },
    static_dep_map { from: core::ptr::null(), to: core::ptr::null() },
];

unsafe fn omap4plus_init_static_deps(map: *const static_dep_map) -> i32 {
    if map.is_null() { return 0; }
    let mut current = map;
    while !(*current).from.is_null() {
        let from = clkdm_lookup((*current).from);
        let to = clkdm_lookup((*current).to);
        if from.is_null() || to.is_null() { return -22; }
        let ret = clkdm_add_wkdep(from, to);
        if ret != 0 { return ret; }
        current = current.add(1);
    }
    0
}

pub unsafe extern "C" fn omap4_pm_init_early() -> i32 {
    if cpu_is_omap446x() { pm44xx_errata |= PM_OMAP4_ROM_SMP_BOOT_ERRATUM_GICD; }
    if soc_is_omap54xx() || soc_is_dra7xx() { pm44xx_errata |= PM_OMAP4_CPU_OSWR_DISABLE; }
    0
}

pub unsafe extern "C" fn omap4_pm_init() -> i32 {
    let mut ret = 0;
    if omap_rev() == OMAP4430_REV_ES1_0 { return -19; }
    ret = pwrdm_for_each(pwrdms_setup, core::ptr::null_mut());
    if ret != 0 { return ret; }
    ret = if cpu_is_omap44xx() { omap4plus_init_static_deps(omap4_static_dep_map.as_ptr()) }
          else if soc_is_omap54xx() || soc_is_dra7xx() { omap4plus_init_static_deps(omap5_dra7_static_dep_map.as_ptr()) }
          else { 0 };
    if ret != 0 { return ret; }
    ret = omap4_mpuss_init();
    if ret != 0 { return ret; }
    clkdm_for_each(omap_pm_clkdms_setup, core::ptr::null_mut());
    omap_common_suspend_init(Some(omap4_pm_suspend));
    arm_pm_idle = Some(omap_default_idle);
    if cpu_is_omap44xx() || soc_is_omap54xx() { omap4_idle_init(); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
