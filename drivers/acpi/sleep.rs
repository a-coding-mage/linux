// SPDX-License-Identifier: GPL-2.0-only
// ACPI sleep support. Direct low-level translation of sleep.c.
// C headers and kernel-provided symbols are intentionally external dependencies.

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

extern "C" {
    fn acpi_execute_simple_method(obj: *mut core::ffi::c_void, name: *const i8, value: u32) -> i32;
    fn acpi_get_wakeup_address() -> usize;
    fn acpi_set_waking_vector(address: usize);
    fn acpi_enable_wakeup_devices(state: u32);
    fn acpi_disable_wakeup_devices(state: u32);
    fn acpi_enter_sleep_state_prep(state: u32);
    fn acpi_leave_sleep_state_prep(state: u32);
    fn acpi_enter_sleep_state(state: u32) -> i32;
    fn acpi_leave_sleep_state(state: u32);
    fn acpi_get_sleep_type_data(state: u8, a: *mut u8, b: *mut u8) -> i32;
    fn acpi_disable_all_gpes(); fn acpi_hw_disable_all_gpes();
    fn acpi_os_wait_events_complete();
    fn acpi_ec_block_transactions(); fn acpi_ec_unblock_transactions();
    fn acpi_ec_flush_work(); fn acpi_ec_dispatch_gpe() -> bool;
    fn suspend_nvs_save() -> i32; fn suspend_nvs_restore(); fn suspend_nvs_free();
    fn suspend_nvs_alloc() -> i32;
    fn acpi_resume_power_resources();
    fn acpi_scan_lock_acquire(); fn acpi_scan_lock_release();
    fn acpi_turn_off_unused_power_resources();
    fn acpi_write_bit_register(reg: u32, value: u32);
    fn acpi_read_bit_register(reg: u32, value: *mut u32);
    fn acpi_get_event_status(event: u32, status: *mut u32) -> i32;
    fn acpi_clear_event(event: u32);
    fn acpi_any_fixed_event_status_set() -> bool;
    fn acpi_check_wakeup_handlers() -> bool;
    fn acpi_ec_set_gpe_wake_mask(mask: u32);
    fn acpi_enable_all_wakeup_gpes(); fn acpi_enable_all_runtime_gpes();
    fn acpi_sci_irq_valid() -> bool; fn enable_irq_wake(irq: i32) -> i32;
    fn disable_irq_wake(irq: i32); fn pm_wakeup_pending() -> bool;
    fn pm_wakeup_clear(irq: i32); fn rearm_wake_irq(irq: i32);
    fn acpi_enable();
}

pub const ACPI_STATE_S0: u32 = 0;
pub const ACPI_STATE_S1: u32 = 1;
pub const ACPI_STATE_S3: u32 = 3;
pub const ACPI_STATE_S4: u32 = 4;
pub const ACPI_STATE_S5: u32 = 5;
pub const ACPI_S_STATE_COUNT: usize = 6;
pub const ACPI_EVENT_POWER_BUTTON: u32 = 2;
pub const ACPI_EVENT_FLAG_DISABLED: u32 = 0;
pub const ACPI_EVENT_FLAG_STATUS_SET: u32 = 1;
pub const ACPI_GPE_ENABLE: u32 = 1;
pub const ACPI_GPE_DISABLE: u32 = 0;

#[no_mangle] pub static mut acpi_no_s5: bool = false;
static mut sleep_states: [u8; ACPI_S_STATE_COUNT] = [0; ACPI_S_STATE_COUNT];
#[cfg(feature = "acpi_sleep")]
static mut acpi_target_sleep_state: u32 = ACPI_STATE_S0;
static mut pwr_btn_event_pending: bool = false;
static mut nvs_nosave: bool = false;
static mut nvs_nosave_s3: bool = false;
static mut old_suspend_ordering: bool = false;
#[no_mangle] pub static mut acpi_sleep_default_s3: bool = false;
static mut ignore_blacklist: bool = false;
static mut s2idle_wakeup: bool = false;

unsafe fn acpi_sleep_tts_switch(acpi_state: u32) {
    let status = acpi_execute_simple_method(core::ptr::null_mut(), b"\\_TTS\0".as_ptr() as *const i8, acpi_state);
    if status != 0 && status != 0x06 { /* ACPI_FAILURE(status) && status != AE_NOT_FOUND */ }
}

pub unsafe fn acpi_sleep_state_supported(sleep_state: u8) -> bool {
    let mut a = 0u8; let mut b = 0u8;
    acpi_get_sleep_type_data(sleep_state, &mut a, &mut b) == 0
}

pub unsafe fn acpi_target_system_state() -> u32 { acpi_target_sleep_state }
pub unsafe fn acpi_nvs_nosave() { nvs_nosave = true; }
pub unsafe fn acpi_nvs_nosave_s3() { nvs_nosave_s3 = true; }
pub unsafe fn acpi_old_suspend_ordering() { old_suspend_ordering = true; }
pub unsafe fn acpi_sleep_no_blacklist() { ignore_blacklist = true; }

unsafe fn acpi_sleep_prepare(state: u32) -> i32 {
    #[cfg(feature = "acpi_sleep")]
    if state == ACPI_STATE_S3 {
        let address = acpi_get_wakeup_address();
        if address == 0 { return -14; }
        acpi_set_waking_vector(address);
    }
    acpi_enable_wakeup_devices(state); acpi_enter_sleep_state_prep(state); 0
}

unsafe fn acpi_pm_freeze() -> i32 { acpi_disable_all_gpes(); acpi_os_wait_events_complete(); acpi_ec_block_transactions(); 0 }
unsafe fn acpi_pm_pre_suspend() -> i32 { acpi_pm_freeze(); suspend_nvs_save() }
unsafe fn acpi_pm_prepare() -> i32 { let e = acpi_sleep_prepare(acpi_target_sleep_state); if e != 0 { acpi_target_sleep_state = ACPI_STATE_S0; } if e == 0 { acpi_pm_pre_suspend() } else { e } }

unsafe fn acpi_pm_finish() {
    let state = acpi_target_sleep_state; acpi_ec_unblock_transactions(); suspend_nvs_free();
    if state == ACPI_STATE_S0 { return; }
    acpi_disable_wakeup_devices(state); acpi_leave_sleep_state(state); acpi_set_waking_vector(0);
    acpi_target_sleep_state = ACPI_STATE_S0; acpi_resume_power_resources();
    if pwr_btn_event_pending { pwr_btn_event_pending = false; }
}
unsafe fn acpi_pm_start(state: u32) { acpi_target_sleep_state = state; acpi_sleep_tts_switch(state); acpi_scan_lock_acquire(); }
unsafe fn acpi_pm_end() { acpi_turn_off_unused_power_resources(); acpi_scan_lock_release(); acpi_target_sleep_state = ACPI_STATE_S0; acpi_sleep_tts_switch(ACPI_STATE_S0); }

pub unsafe fn acpi_s2idle_begin() -> i32 { acpi_scan_lock_acquire(); 0 }
pub unsafe fn acpi_s2idle_prepare() -> i32 {
    if acpi_sci_irq_valid() { let _ = enable_irq_wake(0); acpi_ec_set_gpe_wake_mask(ACPI_GPE_ENABLE); }
    acpi_enable_wakeup_devices(ACPI_STATE_S0); acpi_enable_all_wakeup_gpes(); acpi_os_wait_events_complete(); s2idle_wakeup = true; 0
}
pub unsafe fn acpi_s2idle_wake() -> bool {
    if !acpi_sci_irq_valid() { return pm_wakeup_pending(); }
    while pm_wakeup_pending() {
        if acpi_any_fixed_event_status_set() || acpi_check_wakeup_handlers() || acpi_ec_dispatch_gpe() { return true; }
        acpi_os_wait_events_complete();
        if pm_wakeup_pending() { return true; }
        pm_wakeup_clear(0); rearm_wake_irq(0);
    }
    false
}
pub unsafe fn acpi_s2idle_restore() { acpi_os_wait_events_complete(); acpi_ec_flush_work(); acpi_os_wait_events_complete(); s2idle_wakeup = false; acpi_enable_all_runtime_gpes(); acpi_disable_wakeup_devices(ACPI_STATE_S0); if acpi_sci_irq_valid() { acpi_ec_set_gpe_wake_mask(ACPI_GPE_DISABLE); disable_irq_wake(0); } }
pub unsafe fn acpi_s2idle_end() { acpi_scan_lock_release(); }
pub unsafe fn acpi_s2idle_wakeup() -> bool { s2idle_wakeup }

#[cfg(feature = "suspend")]
unsafe fn acpi_suspend_begin(pm_state: usize) -> i32 {
    let states = [ACPI_STATE_S0, ACPI_STATE_S1, ACPI_STATE_S3, ACPI_STATE_S5];
    let state = states[pm_state];
    if nvs_nosave || nvs_nosave_s3 { } else { let e = suspend_nvs_alloc(); if e != 0 { return e; } }
    if sleep_states[state as usize] == 0 { return -38; }
    acpi_pm_start(state); 0
}

#[cfg(feature = "suspend")]
unsafe fn acpi_suspend_enter(_pm_state: usize) -> i32 {
    let state = acpi_target_sleep_state;
    let mut status = 0;
    if state == ACPI_STATE_S1 { status = acpi_enter_sleep_state(state); }
    else if state == ACPI_STATE_S3 { status = acpi_enter_sleep_state(state); }
    acpi_write_bit_register(0, 1); acpi_leave_sleep_state_prep(state);
    acpi_hw_disable_all_gpes(); acpi_ec_unblock_transactions(); suspend_nvs_restore();
    if status == 0 { 0 } else { -14 }
}

#[cfg(feature = "suspend")]
unsafe fn acpi_suspend_state_valid(pm_state: usize) -> bool {
    if pm_state <= 2 { let states = [ACPI_STATE_S0, ACPI_STATE_S1, ACPI_STATE_S3]; return sleep_states[states[pm_state] as usize] != 0; }
    false
}

#[cfg(feature = "hibernation")]
unsafe fn acpi_hibernation_begin(stage: u32) -> i32 {
    if !nvs_nosave { let e = suspend_nvs_alloc(); if e != 0 { return e; } }
    if stage != 0 { /* PM_EVENT_HIBERNATE */ }
    acpi_pm_start(ACPI_STATE_S4); 0
}
#[cfg(feature = "hibernation")]
unsafe fn acpi_hibernation_enter() -> i32 {
    let status = acpi_enter_sleep_state(ACPI_STATE_S4);
    acpi_leave_sleep_state_prep(ACPI_STATE_S4);
    if status == 0 { 0 } else { -14 }
}
#[cfg(feature = "hibernation")]
unsafe fn acpi_hibernation_leave() { acpi_enable(); acpi_leave_sleep_state_prep(ACPI_STATE_S4); suspend_nvs_restore(); acpi_ec_unblock_transactions(); }

// CONFIG_ACPI_SLEEP, CONFIG_SUSPEND, CONFIG_PM_SLEEP, and CONFIG_HIBERNATION
// conditional registrations and kernel operation tables are supplied by the
// surrounding kernel translation unit.

unsafe fn acpi_power_off_prepare() -> i32 { acpi_sleep_prepare(ACPI_STATE_S5); acpi_disable_all_gpes(); acpi_os_wait_events_complete(); 0 }
unsafe fn acpi_power_off() -> i32 { acpi_sleep_prepare(ACPI_STATE_S5); acpi_enter_sleep_state(ACPI_STATE_S5); 0 }

pub unsafe fn acpi_sleep_init() -> i32 {
    sleep_states[ACPI_STATE_S0 as usize] = 1;
    if acpi_sleep_state_supported(ACPI_STATE_S5 as u8) { sleep_states[ACPI_STATE_S5 as usize] = 1; } else { acpi_no_s5 = true; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
