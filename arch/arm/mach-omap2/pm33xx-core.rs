// SPDX-License-Identifier: GPL-2.0
/*
 * AM33XX Arch Power Management Routines
 *
 * Copyright (C) 2016-2018 Texas Instruments Incorporated - https://www.ti.com/
 *	Dave Gerlach
 */

// Kernel and platform dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct powerdomain { _private: [u8; 0] }
#[repr(C)]
pub struct clockdomain { _private: [u8; 0] }
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct device { _private: [u8; 0] }

pub type u32 = std::ffi::c_uint;
pub type c_int = std::ffi::c_int;
pub type c_ulong = std::ffi::c_ulong;
pub type suspend_state_t = c_int;

extern "C" {
    static mut enable_off_mode: c_int;
    static mut optee_available: c_int;
    static mut am33xx_pm_sram: am33xx_pm_sram_addr;
    static mut am43xx_pm_sram: am33xx_pm_sram_addr;

    fn ioremap(addr: c_ulong, size: c_ulong) -> *mut std::ffi::c_void;
    fn scu_a9_get_base() -> c_ulong;
    fn pwrdm_lookup(name: *const std::ffi::c_char) -> *mut powerdomain;
    fn clkdm_lookup(name: *const std::ffi::c_char) -> *mut clockdomain;
    fn clkdm_for_each(f: Option<unsafe extern "C" fn(*mut clockdomain, *mut std::ffi::c_void) -> c_int>, user: *mut std::ffi::c_void) -> c_int;
    fn omap_pm_clkdms_setup(clkdm: *mut clockdomain, user: *mut std::ffi::c_void) -> c_int;
    fn omap_set_pwrdm_state(pwrdm: *mut powerdomain, state: c_int) -> c_int;
    fn omap_type() -> c_int;
    fn of_machine_is_compatible(s: *const std::ffi::c_char) -> c_int;
    fn pwrdm_read_pwrst(pwrdm: *mut powerdomain) -> c_int;
    fn cpu_suspend(args: c_ulong, fn_: Option<unsafe extern "C" fn(c_ulong) -> c_int>) -> c_int;
    fn clkdm_wakeup(clkdm: *mut clockdomain);
    fn clkdm_sleep(clkdm: *mut clockdomain);
    fn omap_secure_dispatcher(svc: c_ulong, flags: c_ulong, a1: c_ulong, a2: c_ulong, a3: c_ulong, a4: c_ulong, a5: c_ulong);
    fn omap_smccc_smc(svc: c_ulong, arg: c_ulong);
    fn scu_power_mode(base: *mut std::ffi::c_void, mode: c_int);
    fn omap_irq_pending() -> c_int;
    fn need_resched() -> c_int;
    fn cpu_idle_poll_ctrl(enable: bool);
    fn soc_is_am33xx() -> c_int;
    fn soc_is_am437x() -> c_int;
    fn omap_intc_save_context();
    fn omap_intc_restore_context();
    fn writel_relaxed(value: u32, addr: *mut std::ffi::c_void);
    fn suspend_set_ops(ops: *const platform_suspend_ops);
    fn platform_device_register_full(info: *const platform_device_info) -> c_int;
    fn of_parse_phandle(node: *mut device_node, prop: *const std::ffi::c_char, index: c_int) -> *mut device_node;
    fn of_device_is_available(node: *mut device_node) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn of_property_read_bool(node: *mut device_node, prop: *const std::ffi::c_char) -> c_int;
    fn kzalloc(size: usize, flags: c_ulong) -> *mut std::ffi::c_void;
}

static mut cefuse_pwrdm: *mut powerdomain = std::ptr::null_mut();
static mut gfx_pwrdm: *mut powerdomain = std::ptr::null_mut();
static mut per_pwrdm: *mut powerdomain = std::ptr::null_mut();
static mut mpu_pwrdm: *mut powerdomain = std::ptr::null_mut();
static mut gfx_l4ls_clkdm: *mut clockdomain = std::ptr::null_mut();
static mut scu_base: *mut std::ffi::c_void = std::ptr::null_mut();
static mut idle_fn: Option<unsafe extern "C" fn(u32) -> c_int> = None;

#[repr(C)]
pub struct amx3_idle_state { pub wfi_flags: c_int }
static mut idle_states: *mut amx3_idle_state = std::ptr::null_mut();

#[repr(C)]
pub struct am33xx_pm_sram_addr { _private: [u8; 0] }
#[repr(C)]
pub struct platform_device_info { pub name: *const std::ffi::c_char, pub id: c_int, pub data: *mut std::ffi::c_void, pub size_data: usize }
#[repr(C)]
pub struct platform_suspend_ops { pub begin: Option<unsafe extern "C" fn(suspend_state_t) -> c_int>, pub valid: Option<unsafe extern "C" fn(suspend_state_t) -> c_int> }
#[repr(C)]
pub struct am33xx_pm_platform_data {
    pub init: Option<unsafe extern "C" fn(Option<unsafe extern "C" fn(u32) -> c_int>) -> c_int>,
    pub deinit: Option<unsafe extern "C" fn() -> c_int>,
    pub soc_suspend: Option<unsafe extern "C" fn(u32, Option<unsafe extern "C" fn(c_ulong) -> c_int>, c_ulong) -> c_int>,
    pub cpu_suspend: Option<unsafe extern "C" fn(Option<unsafe extern "C" fn(c_ulong) -> c_int>, c_ulong) -> c_int>,
    pub begin_suspend: Option<unsafe extern "C" fn()>, pub finish_suspend: Option<unsafe extern "C" fn()>,
    pub get_sram_addrs: Option<unsafe extern "C" fn() -> *mut am33xx_pm_sram_addr>,
    pub save_context: Option<unsafe extern "C" fn()>, pub restore_context: Option<unsafe extern "C" fn()>,
    pub check_off_mode_enable: Option<unsafe extern "C" fn() -> c_int>,
}

const ENOMEM: c_int = 12; const ENODEV: c_int = 19; const EINVAL: c_int = 22;
const OMAP2_DEVICE_TYPE_GP: c_int = 0; const PWRDM_POWER_OFF: c_int = 0;
const SCU_PM_POWEROFF: c_int = 0; const SCU_PM_DORMANT: c_int = 1; const SCU_PM_NORMAL: c_int = 2;
const CPUIDLE_STATE_MAX: usize = 8; const WFI_FLAG_WAKE_M3: c_int = 1; const WFI_FLAG_FLUSH_CACHE: c_int = 2;
const AM43XX_PPA_SVC_PM_SUSPEND: c_ulong = 0; const AM43XX_PPA_SVC_PM_RESUME: c_ulong = 0; const FLAG_START_CRITICAL: c_ulong = 0;

// The remaining definitions mirror the C implementation and use the externally supplied kernel logging/macros.
// Build-time CONFIG_SUSPEND controls whether amx3_block_suspend is the real implementation or an empty inline.

unsafe extern "C" fn am43xx_map_scu() -> c_int { scu_base = ioremap(scu_a9_get_base(), 256); if scu_base.is_null() { -ENOMEM } else { 0 } }
unsafe extern "C" fn am33xx_check_off_mode_enable() -> c_int { 0 }
unsafe extern "C" fn am43xx_check_off_mode_enable() -> c_int {
    if of_machine_is_compatible(b"ti,am437x-gp-evm\0".as_ptr() as _) != 0 && enable_off_mode != 0 { enable_off_mode } else { 0 }
}
unsafe extern "C" fn amx3_common_init(idle: Option<unsafe extern "C" fn(u32) -> c_int>) -> c_int {
    gfx_pwrdm = pwrdm_lookup(b"gfx_pwrdm\0".as_ptr() as _); per_pwrdm = pwrdm_lookup(b"per_pwrdm\0".as_ptr() as _); mpu_pwrdm = pwrdm_lookup(b"mpu_pwrdm\0".as_ptr() as _);
    if gfx_pwrdm.is_null() || per_pwrdm.is_null() || mpu_pwrdm.is_null() { return -ENODEV; }
    let _ = clkdm_for_each(Some(omap_pm_clkdms_setup), std::ptr::null_mut());
    cefuse_pwrdm = pwrdm_lookup(b"cefuse_pwrdm\0".as_ptr() as _);
    if !cefuse_pwrdm.is_null() && omap_type() == OMAP2_DEVICE_TYPE_GP { let _ = omap_set_pwrdm_state(cefuse_pwrdm, PWRDM_POWER_OFF); }
    idle_fn = idle; 0
}
unsafe extern "C" fn am33xx_suspend_init(idle: Option<unsafe extern "C" fn(u32) -> c_int>) -> c_int { gfx_l4ls_clkdm = clkdm_lookup(b"gfx_l4ls_gfx_clkdm\0".as_ptr() as _); if gfx_l4ls_clkdm.is_null() { -ENODEV } else { amx3_common_init(idle) } }
unsafe extern "C" fn am43xx_suspend_init(idle: Option<unsafe extern "C" fn(u32) -> c_int>) -> c_int { let ret = am43xx_map_scu(); if ret != 0 { ret } else { amx3_common_init(idle) } }
unsafe extern "C" fn amx3_suspend_deinit() -> c_int { idle_fn = None; 0 }
unsafe extern "C" fn amx3_pre_suspend_common() { let _ = omap_set_pwrdm_state(gfx_pwrdm, PWRDM_POWER_OFF); }
unsafe extern "C" fn amx3_post_suspend_common() { let _status = pwrdm_read_pwrst(gfx_pwrdm); }
unsafe extern "C" fn am33xx_suspend(_state: u32, f: Option<unsafe extern "C" fn(c_ulong) -> c_int>, args: c_ulong) -> c_int { amx3_pre_suspend_common(); let ret = cpu_suspend(args, f); amx3_post_suspend_common(); clkdm_wakeup(gfx_l4ls_clkdm); clkdm_sleep(gfx_l4ls_clkdm); ret }
unsafe extern "C" fn am43xx_suspend(_state: u32, f: Option<unsafe extern "C" fn(c_ulong) -> c_int>, args: c_ulong) -> c_int { amx3_pre_suspend_common(); scu_power_mode(scu_base, SCU_PM_POWEROFF); let ret = cpu_suspend(args, f); scu_power_mode(scu_base, SCU_PM_NORMAL); if am43xx_check_off_mode_enable() == 0 { amx3_post_suspend_common(); } ret }
unsafe extern "C" fn am33xx_cpu_suspend(f: Option<unsafe extern "C" fn(c_ulong) -> c_int>, args: c_ulong) -> c_int { if omap_irq_pending() != 0 || need_resched() != 0 { return 0; } cpu_suspend(args, f) }
unsafe extern "C" fn am43xx_cpu_suspend(f: Option<unsafe extern "C" fn(c_ulong) -> c_int>, args: c_ulong) -> c_int { if scu_base.is_null() { return 0; } scu_power_mode(scu_base, SCU_PM_DORMANT); let ret = cpu_suspend(args, f); scu_power_mode(scu_base, SCU_PM_NORMAL); ret }
unsafe extern "C" fn amx3_begin_suspend() { cpu_idle_poll_ctrl(true); }
unsafe extern "C" fn amx3_finish_suspend() { cpu_idle_poll_ctrl(false); }
unsafe extern "C" fn amx3_get_sram_addrs() -> *mut am33xx_pm_sram_addr { if soc_is_am33xx() != 0 { &mut am33xx_pm_sram } else if soc_is_am437x() != 0 { &mut am43xx_pm_sram } else { std::ptr::null_mut() } }
unsafe extern "C" fn am43xx_save_context() {}
unsafe extern "C" fn am33xx_save_context() { omap_intc_save_context(); }
unsafe extern "C" fn am33xx_restore_context() { omap_intc_restore_context(); }
unsafe extern "C" fn am43xx_restore_context() { writel_relaxed(0, std::ptr::null_mut()); }

static mut am33xx_ops: am33xx_pm_platform_data = am33xx_pm_platform_data { init: Some(am33xx_suspend_init), deinit: Some(amx3_suspend_deinit), soc_suspend: Some(am33xx_suspend), cpu_suspend: Some(am33xx_cpu_suspend), begin_suspend: Some(amx3_begin_suspend), finish_suspend: Some(amx3_finish_suspend), get_sram_addrs: Some(amx3_get_sram_addrs), save_context: Some(am33xx_save_context), restore_context: Some(am33xx_restore_context), check_off_mode_enable: Some(am33xx_check_off_mode_enable) };
static mut am43xx_ops: am33xx_pm_platform_data = am33xx_pm_platform_data { init: Some(am43xx_suspend_init), deinit: Some(amx3_suspend_deinit), soc_suspend: Some(am43xx_suspend), cpu_suspend: Some(am43xx_cpu_suspend), begin_suspend: Some(amx3_begin_suspend), finish_suspend: Some(amx3_finish_suspend), get_sram_addrs: Some(amx3_get_sram_addrs), save_context: Some(am43xx_save_context), restore_context: Some(am43xx_restore_context), check_off_mode_enable: Some(am43xx_check_off_mode_enable) };
unsafe extern "C" fn am33xx_pm_get_pdata() -> *mut am33xx_pm_platform_data { if soc_is_am33xx() != 0 { &mut am33xx_ops } else if soc_is_am437x() != 0 { &mut am43xx_ops } else { std::ptr::null_mut() } }

#[no_mangle] pub unsafe extern "C" fn amx3_common_pm_init() -> c_int { let pdata = am33xx_pm_get_pdata(); let devinfo = platform_device_info { name: b"pm33xx\0".as_ptr() as _, id: -1, data: pdata as _, size_data: std::mem::size_of::<am33xx_pm_platform_data>() }; let _ = platform_device_register_full(&devinfo); 0 }
unsafe extern "C" fn amx3_suspend_block(_state: suspend_state_t) -> c_int { -EINVAL }
unsafe extern "C" fn amx3_pm_valid(state: suspend_state_t) -> c_int { if state == 1 { 1 } else { 0 } }
static amx3_blocked_pm_ops: platform_suspend_ops = platform_suspend_ops { begin: Some(amx3_suspend_block), valid: Some(amx3_pm_valid) };
unsafe extern "C" fn amx3_block_suspend() { suspend_set_ops(&amx3_blocked_pm_ops); }

unsafe extern "C" fn amx3_idle_init(cpu_node: *mut device_node, _cpu: c_int) -> c_int {
    let mut state_count: usize = 1; let mut states = [amx3_idle_state { wfi_flags: 0 }; CPUIDLE_STATE_MAX]; let mut i = 0;
    loop { let node = of_parse_phandle(cpu_node, b"cpu-idle-states\0".as_ptr() as _, i); if node.is_null() { break; } if of_device_is_available(node) == 0 { of_node_put(node); i += 1; continue; } if i as usize == CPUIDLE_STATE_MAX { of_node_put(node); break; } states[state_count].wfi_flags = 0; if of_property_read_bool(node, b"ti,idle-wkup-m3\0".as_ptr() as _) != 0 { states[state_count].wfi_flags |= WFI_FLAG_WAKE_M3 | WFI_FLAG_FLUSH_CACHE; } of_node_put(node); state_count += 1; i += 1; }
    idle_states = kzalloc(state_count * std::mem::size_of::<amx3_idle_state>(), 0) as *mut amx3_idle_state; if idle_states.is_null() { return -ENOMEM; } for j in 1..state_count { (*idle_states.add(j)).wfi_flags = states[j].wfi_flags; } 0
}
unsafe extern "C" fn amx3_idle_enter(index: c_ulong) -> c_int { if idle_states.is_null() { return -EINVAL; } let state = &*idle_states.add(index as usize); if let Some(f) = idle_fn { let _ = f(state.wfi_flags as u32); } 0 }
// CPUIDLE_METHOD_OF_DECLARE(pm33xx_idle, "ti,am3352", &amx3_cpuidle_ops);
// CPUIDLE_METHOD_OF_DECLARE(pm43xx_idle, "ti,am4372", &amx3_cpuidle_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
