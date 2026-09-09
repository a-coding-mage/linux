// SPDX-License-Identifier: GPL-2.0
//
//  cpuidle-pseries - idle state cpuidle driver.
//  Adapted from drivers/idle/intel_idle.c and
//  drivers/acpi/processor_idle.c
//
// Dependencies are supplied by the surrounding kernel translation.

use core::ffi::c_void;

const CEDE_LATENCY_TOKEN: u32 = 45;
const NR_DEDICATED_STATES: usize = 2;

#[repr(C, packed)]
struct xcede_latency_record {
    hint: u8,
    latency_ticks: u64,
    wake_on_irqs: u8,
}

#[repr(C, packed)]
struct xcede_latency_payload {
    record_size: u8,
    records: [xcede_latency_record; 16],
}

#[repr(C, packed)]
struct xcede_latency_parameter {
    payload_size: u16,
    payload: xcede_latency_payload,
    null_char: u8,
}

static mut pseries_idle_driver: cpuidle_driver = cpuidle_driver {
    name: "pseries_idle",
    owner: THIS_MODULE,
    ..cpuidle_driver::zeroed()
};
static mut max_idle_state: i32 = 0;
static mut cpuidle_state_table: *mut cpuidle_state = core::ptr::null_mut();
static mut snooze_timeout: u64 = 0;
static mut snooze_timeout_en: bool = false;
static mut nr_xcede_records: u32 = 0;
static mut xcede_latency_parameter: xcede_latency_parameter = xcede_latency_parameter {
    payload_size: 0,
    payload: xcede_latency_payload {
        record_size: 0,
        records: [xcede_latency_record { hint: 0, latency_ticks: 0, wake_on_irqs: 0 }; 16],
    },
    null_char: 0,
};
static mut cede_latency_hint: [u8; NR_DEDICATED_STATES] = [0; NR_DEDICATED_STATES];

unsafe extern "C" {
    static cpuidle_devices: *mut cpuidle_device;
    static cpuidle_disable: i32;
    static tb_ticks_per_usec: u64;
    const IDLE_NO_OVERRIDE: i32;
    const FW_FEATURE_SPLPAR: u64;
    const CPU_FTR_ARCH_31: u64;
    const PVR_POWER10: u32;
    const CPUHP_AP_ONLINE_DYN: i32;
    const CPUHP_CPUIDLE_DEAD: i32;
    const CPUIDLE_FLAG_POLLING: u32;
    const MSR_EE: u64;
    const UINT_MAX: u64;
    const NSEC_PER_USEC: u64;
    static THIS_MODULE: *mut c_void;
    fn set_thread_flag(flag: i32);
    fn clear_thread_flag(flag: i32);
    fn pseries_idle_prolog();
    fn pseries_idle_epilog();
    fn raw_local_irq_enable();
    fn raw_local_irq_disable();
    fn get_tb() -> u64;
    fn need_resched() -> bool;
    fn HMT_low(); fn HMT_very_low(); fn HMT_medium();
    fn smp_mb();
    fn prep_irq_for_idle() -> bool;
    fn cede_processor(); fn mfmsr() -> u64; fn __hard_irq_enable();
    fn get_lppaca() -> *mut lppaca;
    fn rtas_token(name: *const u8) -> i32;
    fn rtas_call(token: i32, nargs: i32, nret: i32, retbuf: *mut c_void, ... ) -> i32;
    fn __pa(addr: *const c_void) -> u64;
    fn be16_to_cpu(v: u16) -> u16; fn be64_to_cpu(v: u64) -> u64;
    fn tb_to_ns(v: u64) -> u64;
    fn firmware_has_feature(v: u64) -> bool; fn lppaca_shared_proc() -> bool;
    fn cpu_has_feature(v: u64) -> bool; fn pvr_version_is(v: u32) -> bool;
    fn cpuidle_get_driver() -> *mut cpuidle_driver;
    fn cpuidle_pause_and_lock(); fn cpuidle_resume_and_unlock();
    fn cpuidle_enable_device(dev: *mut cpuidle_device);
    fn cpuidle_disable_device(dev: *mut cpuidle_device);
    fn cpuidle_register(drv: *mut cpuidle_driver, state: *mut c_void) -> i32;
    fn cpuhp_setup_state_nocalls(state: i32, name: *const u8,
        online: Option<unsafe extern "C" fn(u32) -> i32>,
        dead: Option<unsafe extern "C" fn(u32) -> i32>) -> i32;
}

#[repr(C)] struct cpuidle_device { poll_time_limit: bool, _rest: [u8; 0] }
#[repr(C)] struct cpuidle_state {
    name: *const u8, desc: *const u8, exit_latency: u64, target_residency: u64,
    enter: Option<unsafe extern "C" fn(*mut cpuidle_device, *mut cpuidle_driver, i32) -> i32>, flags: u32,
}
#[repr(C)] struct cpuidle_driver { name: *const u8, owner: *mut c_void, state_count: i32, states: [cpuidle_state; 8] }
#[repr(C)] struct lppaca { donate_dedicated_cpu: u8, cede_latency_hint: u8 }

unsafe extern "C" fn snooze_loop(dev: *mut cpuidle_device, _drv: *mut cpuidle_driver, index: i32) -> i32 {
    set_thread_flag(0); pseries_idle_prolog(); raw_local_irq_enable();
    let exit = get_tb().wrapping_add(snooze_timeout); (*dev).poll_time_limit = false;
    while !need_resched() { HMT_low(); HMT_very_low(); if snooze_timeout_en && get_tb() > exit { (*dev).poll_time_limit = true; clear_thread_flag(0); smp_mb(); break; } }
    HMT_medium(); if !(*dev).poll_time_limit { clear_thread_flag(0); } raw_local_irq_disable(); pseries_idle_epilog(); index
}

unsafe extern "C" fn check_and_cede_processor() { if prep_irq_for_idle() { cede_processor(); } }

unsafe fn parse_cede_parameters() -> i32 {
    let ret = rtas_call(rtas_token(b"ibm,get-system-parameter\0".as_ptr()), 3, 1, core::ptr::null_mut(), CEDE_LATENCY_TOKEN, __pa(&xcede_latency_parameter as *const _ as *const c_void), core::mem::size_of::<xcede_latency_parameter>());
    if ret != 0 { return ret; }
    let size = xcede_latency_parameter.payload.record_size.wrapping_add(1);
    if size as usize != core::mem::size_of::<xcede_latency_record>() { return -22; }
    let total = be16_to_cpu(xcede_latency_parameter.payload_size).wrapping_sub(2) as u32;
    nr_xcede_records = total / size as u32; 0
}

unsafe extern "C" fn dedicated_cede_loop(_dev: *mut cpuidle_device, _drv: *mut cpuidle_driver, index: i32) -> i32 { pseries_idle_prolog(); let lp = get_lppaca(); (*lp).donate_dedicated_cpu = 1; let old = (*lp).cede_latency_hint; (*lp).cede_latency_hint = cede_latency_hint[index as usize]; HMT_medium(); check_and_cede_processor(); raw_local_irq_disable(); (*lp).donate_dedicated_cpu = 0; (*lp).cede_latency_hint = old; pseries_idle_epilog(); index }
unsafe extern "C" fn shared_cede_loop(_dev: *mut cpuidle_device, _drv: *mut cpuidle_driver, index: i32) -> i32 { pseries_idle_prolog(); check_and_cede_processor(); raw_local_irq_disable(); pseries_idle_epilog(); index }

static mut dedicated_states: [cpuidle_state; 2] = [
    cpuidle_state { name: b"snooze\0".as_ptr(), desc: b"snooze\0".as_ptr(), exit_latency: 0, target_residency: 0, enter: Some(snooze_loop), flags: CPUIDLE_FLAG_POLLING },
    cpuidle_state { name: b"CEDE\0".as_ptr(), desc: b"CEDE\0".as_ptr(), exit_latency: 10, target_residency: 100, enter: Some(dedicated_cede_loop), flags: 0 },
];
static mut shared_states: [cpuidle_state; 2] = [
    cpuidle_state { name: b"snooze\0".as_ptr(), desc: b"snooze\0".as_ptr(), exit_latency: 0, target_residency: 0, enter: Some(snooze_loop), flags: CPUIDLE_FLAG_POLLING },
    cpuidle_state { name: b"Shared Cede\0".as_ptr(), desc: b"Shared Cede\0".as_ptr(), exit_latency: 10, target_residency: 100, enter: Some(shared_cede_loop), flags: 0 },
];

unsafe extern "C" fn pseries_cpuidle_cpu_online(cpu: u32) -> i32 { let dev = cpuidle_devices.add(cpu as usize); if !dev.is_null() && !cpuidle_get_driver().is_null() { cpuidle_pause_and_lock(); cpuidle_enable_device(dev); cpuidle_resume_and_unlock(); } 0 }
unsafe extern "C" fn pseries_cpuidle_cpu_dead(cpu: u32) -> i32 { let dev = cpuidle_devices.add(cpu as usize); if !dev.is_null() && !cpuidle_get_driver().is_null() { cpuidle_pause_and_lock(); cpuidle_disable_device(dev); cpuidle_resume_and_unlock(); } 0 }

unsafe fn pseries_cpuidle_driver_init() -> i32 { pseries_idle_driver.state_count = 0; for i in 0..max_idle_state { let s = &*cpuidle_state_table.add(i as usize); if s.enter.is_none() { continue; } pseries_idle_driver.states[pseries_idle_driver.state_count as usize] = core::ptr::read(s); pseries_idle_driver.state_count += 1; } 0 }
unsafe fn fixup_cede0_latency() { if parse_cede_parameters() != 0 { return; } let mut min = u64::MAX; for i in 0..nr_xcede_records { let r = &xcede_latency_parameter.payload.records[i as usize]; let us = (tb_to_ns(be64_to_cpu(r.latency_ticks)) + NSEC_PER_USEC - 1) / NSEC_PER_USEC; if us != 0 && us < min { min = us; } } if min != u64::MAX { dedicated_states[1].exit_latency = min; dedicated_states[1].target_residency = 10 * min; } }
unsafe fn pseries_idle_probe() -> i32 { if cpuidle_disable != IDLE_NO_OVERRIDE { return -19; } if !firmware_has_feature(FW_FEATURE_SPLPAR) { return -19; } if lppaca_shared_proc() { cpuidle_state_table = shared_states.as_mut_ptr(); max_idle_state = 2; } else { if cpu_has_feature(CPU_FTR_ARCH_31) || pvr_version_is(PVR_POWER10) { fixup_cede0_latency(); } cpuidle_state_table = dedicated_states.as_mut_ptr(); max_idle_state = 2; } if max_idle_state > 1 { snooze_timeout_en = true; snooze_timeout = (*cpuidle_state_table.add(1)).target_residency * tb_ticks_per_usec; } 0 }
unsafe fn pseries_processor_idle_init() -> i32 { let r = pseries_idle_probe(); if r != 0 { return r; } pseries_cpuidle_driver_init(); let r = cpuidle_register(&mut pseries_idle_driver, core::ptr::null_mut()); if r != 0 { return r; } cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN, b"cpuidle/pseries:online\0".as_ptr(), Some(pseries_cpuidle_cpu_online), None); cpuhp_setup_state_nocalls(CPUHP_CPUIDLE_DEAD, b"cpuidle/pseries:DEAD\0".as_ptr(), None, Some(pseries_cpuidle_cpu_dead)); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
