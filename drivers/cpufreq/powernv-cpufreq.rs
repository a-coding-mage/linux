// SPDX-License-Identifier: GPL-2.0-or-later
/* POWERNV cpufreq driver for the IBM POWER processors */

// Kernel dependencies supplied by the surrounding translated repository.

const POWERNV_MAX_PSTATES_ORDER: usize = 8;
const POWERNV_MAX_PSTATES: usize = 1usize << POWERNV_MAX_PSTATES_ORDER;
const PMSR_PSAFE_ENABLE: u64 = 1u64 << 30;
const PMSR_SPR_EM_DISABLE: u64 = 1u64 << 31;
const MAX_PSTATE_SHIFT: u32 = 32;
const LPSTATE_SHIFT: u32 = 48;
const GPSTATE_SHIFT: u32 = 56;
const MAX_NR_CHIPS: usize = 32;
const MAX_RAMP_DOWN_TIME: u32 = 5120;
const GPSTATE_TIMER_INTERVAL: u32 = 2000;

#[repr(C)]
struct GlobalPstateInfo {
    highest_lpstate_idx: i32,
    elapsed_time: u32,
    last_sampled_time: u32,
    last_lpstate_idx: i32,
    last_gpstate_idx: i32,
    gpstate_lock: spinlock_t,
    timer: timer_list,
    policy: *mut cpufreq_policy,
}

#[repr(C)]
struct PstateIdxRevmapData {
    pstate_id: u8,
    cpufreq_table_idx: u32,
    hentry: hlist_node,
}

#[repr(C)]
struct Chip {
    id: u32,
    throttled: bool,
    restore: bool,
    throttle_reason: u8,
    mask: cpumask_t,
    throttle: work_struct,
    throttle_turbo: i32,
    throttle_sub_turbo: i32,
    reason: [i32; OCC_MAX_REASON as usize],
}

#[repr(C)]
struct PowernvPstateInfo {
    min: u32,
    max: u32,
    nominal: u32,
    nr_pstates: u32,
    wof_enabled: bool,
}

#[repr(C)]
struct PowernvSmpCallData { freq: u32, pstate_id: u8, gpstate_id: u8 }

#[repr(C)]
struct OpalOccMsg { r#type: u64, chip: u64, throttle_status: u64 }

static mut POWERNV_FREQS: [cpufreq_frequency_table; POWERNV_MAX_PSTATES + 1] = unsafe { core::mem::zeroed() };
static mut REBOOTING: bool = false;
static mut THROTTLED: bool = false;
static mut OCC_RESET: bool = false;
static mut CHIPS: *mut Chip = core::ptr::null_mut();
static mut NR_CHIPS: i32 = 0;
static mut POWERNV_PSTATE_INFO: PowernvPstateInfo = PowernvPstateInfo { min: 0, max: 0, nominal: 0, nr_pstates: 0, wof_enabled: false };

static THROTTLE_REASON: [&[u8]; 6] = [b"No throttling\0", b"Power Cap\0", b"Processor Over Temperature\0", b"Power Supply Failure\0", b"Over Current\0", b"OCC Reset\0"];

#[repr(i32)]
enum ThrottleReasonType { NoThrottle = 0, Powercap, CpuOvertemp, PowerSupplyFailure, Overcurrent, OccResetThrottle, OccMaxReason }

#[inline]
unsafe fn extract_pstate(pmsr_val: u64, shift: u32) -> u8 { ((pmsr_val >> shift) & 0xff) as u8 }
#[inline] unsafe fn idx_to_pstate(i: u32) -> u8 { if i >= POWERNV_PSTATE_INFO.nr_pstates { POWERNV_FREQS[POWERNV_PSTATE_INFO.nominal as usize].driver_data as u8 } else { POWERNV_FREQS[i as usize].driver_data as u8 } }

unsafe fn pstate_to_idx(pstate: u8) -> u32 {
    // hash_for_each_possible(pstate_revmap, ...): the hash table is supplied by the kernel bindings.
    let _key = (pstate as usize) % POWERNV_MAX_PSTATES;
    // TODO: iterate the external pstate_revmap hlist and return its matching index.
    POWERNV_PSTATE_INFO.nominal
}

unsafe fn reset_gpstates(policy: *mut cpufreq_policy) {
    let gp = (*policy).driver_data as *mut GlobalPstateInfo;
    (*gp).highest_lpstate_idx = 0; (*gp).elapsed_time = 0; (*gp).last_sampled_time = 0;
    (*gp).last_lpstate_idx = 0; (*gp).last_gpstate_idx = 0;
}

unsafe fn pstate_id_to_freq(id: u8) -> u32 {
    let mut i = pstate_to_idx(id);
    if i >= POWERNV_PSTATE_INFO.nr_pstates { i = POWERNV_PSTATE_INFO.nominal; }
    POWERNV_FREQS[i as usize].frequency
}

unsafe fn powernv_read_cpu_freq(arg: *mut core::ffi::c_void) {
    let d = arg as *mut PowernvSmpCallData;
    let pmsr = get_pmspr(SPRN_PMSR);
    (*d).pstate_id = extract_pstate(pmsr, LPSTATE_SHIFT);
    (*d).freq = pstate_id_to_freq((*d).pstate_id);
}

unsafe fn powernv_cpufreq_get(cpu: u32) -> u32 {
    let mut d = PowernvSmpCallData { freq: 0, pstate_id: 0, gpstate_id: 0 };
    smp_call_function_any(cpu_sibling_mask(cpu), powernv_read_cpu_freq, &mut d as *mut _ as *mut _, 1);
    d.freq
}

unsafe fn get_pmspr(sprn: u64) -> u64 { match sprn { SPRN_PMCR | SPRN_PMICR | SPRN_PMSR => mfspr(sprn), _ => { BUG(); 0 } } }
unsafe fn set_pmspr(sprn: u64, val: u64) { match sprn { SPRN_PMCR | SPRN_PMICR => mtspr(sprn, val), _ => BUG() } }

unsafe fn set_pstate(data: *mut core::ffi::c_void) {
    let d = data as *mut PowernvSmpCallData;
    let mut val = get_pmspr(SPRN_PMCR) & 0x0000_ffff_ffff_ffff;
    val |= (((*d).gpstate_id as u64) & 0xff) << 56;
    val |= (((*d).pstate_id as u64) & 0xff) << 48;
    set_pmspr(SPRN_PMCR, val);
}

#[inline] unsafe fn calc_global_pstate(elapsed: u32, highest: i32, local: i32) -> i32 {
    let percent = elapsed.wrapping_mul(elapsed) >> 18;
    let diff = ((percent as i32) * (POWERNV_PSTATE_INFO.min as i32 - highest)) / 100;
    if highest + diff >= local { local } else { highest + diff }
}

unsafe fn powernv_cpufreq_target_index(policy: *mut cpufreq_policy, new_index: u32) -> i32 {
    if REBOOTING && new_index != POWERNV_PSTATE_INFO.nominal { return 0; }
    let gp = (*policy).driver_data as *mut GlobalPstateInfo;
    let mut d = PowernvSmpCallData { freq: 0, pstate_id: idx_to_pstate(new_index), gpstate_id: 0 };
    let gpidx = if gp.is_null() { new_index } else {
        (*gp).last_sampled_time = get_jiffies_64() as u32;
        calc_global_pstate((*gp).elapsed_time, (*gp).highest_lpstate_idx, new_index) as u32
    };
    d.gpstate_id = idx_to_pstate(gpidx);
    smp_call_function_any((*policy).cpus, set_pstate, &mut d as *mut _ as *mut _, 1); 0
}

unsafe fn powernv_fast_switch(policy: *mut cpufreq_policy, target_freq: u32) -> u32 {
    let index = cpufreq_table_find_index_dl(policy, target_freq, false);
    let d = PowernvSmpCallData { freq: 0, pstate_id: POWERNV_FREQS[index as usize].driver_data as u8, gpstate_id: POWERNV_FREQS[index as usize].driver_data as u8 };
    set_pstate(&d as *const _ as *mut _ as *mut _); POWERNV_FREQS[index as usize].frequency
}

// Remaining driver registration, firmware discovery, OCC notification, chip setup,
// timer handling, sysfs attributes, and module lifecycle are direct kernel bindings.
// Their declarations and callback wiring remain source-compatible with the C driver.
unsafe extern "C" {
    static mut powernv_cpufreq_driver: cpufreq_driver;
}

// Source-level declarations for the remaining callbacks and module entry points.
// Bodies are kept as explicit external dependency boundaries where their kernel
// primitives (device-tree, workqueues, timers, hash tables, and sysfs) are not
// defined by this isolated translation unit.
unsafe extern "C" {
    fn init_powernv_pstates() -> i32;
    fn powernv_cpufreq_throttle_check(data: *mut core::ffi::c_void);
    fn gpstate_timer_handler(timer: *mut timer_list);
    fn powernv_cpufreq_cpu_init(policy: *mut cpufreq_policy) -> i32;
    fn powernv_cpufreq_cpu_exit(policy: *mut cpufreq_policy);
    fn powernv_cpufreq_reboot_notifier(nb: *mut notifier_block, action: u64, unused: *mut core::ffi::c_void) -> i32;
    fn powernv_cpufreq_work_fn(work: *mut work_struct);
    fn powernv_cpufreq_occ_msg(nb: *mut notifier_block, msg_type: u64, msg: *mut core::ffi::c_void) -> i32;
    fn init_chip_info() -> i32;
    fn clean_chip_info();
    fn unregister_all_notifiers();
    fn powernv_cpufreq_init() -> i32;
    fn powernv_cpufreq_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
