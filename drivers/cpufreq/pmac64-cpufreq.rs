// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level translation of pmac64-cpufreq.c. */

const SCOM_PCR: u32 = 0x0aa001;
const PCR_HILO_SELECT: u32 = 0x80000000;
const PCR_SPEED_SHIFT: u32 = 17;
const PCR_FREQ_REQ_VALID: u32 = 0x00010000;
const PCR_VOLT_REQ_VALID: u32 = 0x00008000;
const SCOM_PSR: u32 = 0x408001;
const PSR_CMD_RECEIVED: u64 = 0x2000000000000000;
const PSR_CMD_COMPLETED: u64 = 0x1000000000000000;
const PSR_CUR_SPEED_SHIFT: u32 = 56;
const CPUFREQ_HIGH: usize = 0;
const CPUFREQ_LOW: usize = 1;

#[repr(C)]
pub struct CpufreqFrequencyTable { pub flags: u32, pub driver_data: u32, pub frequency: u64 }
#[repr(C)]
pub struct CpufreqPolicy { _private: [u8; 0] }
#[repr(C)]
pub struct DeviceNode { _private: [u8; 0] }
#[repr(C)]
pub struct PmfFunction { _private: [u8; 0] }
#[repr(C)]
pub struct PmfArgs { pub count: u32, pub u: [PmfArg; 16] }
#[repr(C)]
pub union PmfArg { pub p: *mut u32, pub v: u32 }
#[repr(C)]
pub struct SmuSimpleCmd { _private: [u8; 0] }
#[repr(C)]
pub struct SmuSdbpHeader { pub len: u32 }
#[repr(C)]
pub struct SmuSdbpFvt { _private: [u8; 0] }
#[repr(C)]
pub struct CpufreqDriver { _private: [u8; 0] }

extern "C" {
    static mut jiffies: usize; static HZ: usize; static mut ppc_proc_freq: u64;
    fn scom970_write(a: u32, v: u32); fn scom970_read(a: u32) -> u64;
    fn local_irq_save(f: *mut usize); fn local_irq_restore(f: usize);
    fn udelay(v: u32); fn usleep_range(a: u32, b: u32); fn msleep(v: u32);
    fn pmf_call_one(f: *mut PmfFunction, a: *mut PmfArgs) -> i32;
    fn pmf_find_function(n: *mut DeviceNode, s: *const u8) -> *mut PmfFunction;
    fn pmf_put_function(f: *mut PmfFunction);
    fn of_get_property(n: *mut DeviceNode, s: *const u8, len: *mut u32) -> *const u32;
    fn of_property_present(n: *mut DeviceNode, s: *const u8) -> bool;
    fn of_find_node_by_path(s: *const u8) -> *mut DeviceNode; fn of_node_put(n: *mut DeviceNode);
    fn of_cpu_device_node_get(i: u32) -> *mut DeviceNode; fn of_machine_is_compatible(s: *const u8) -> bool;
    fn cpufreq_generic_init(p: *mut CpufreqPolicy, t: *mut CpufreqFrequencyTable, l: u64);
    fn cpufreq_generic_frequency_table_verify(p: *mut CpufreqPolicy) -> i32;
    fn cpufreq_register_driver(d: *mut CpufreqDriver) -> i32;
}

static mut g5_cpu_freqs: [CpufreqFrequencyTable; 3] = [
    CpufreqFrequencyTable { flags: 0, driver_data: 0, frequency: 0 },
    CpufreqFrequencyTable { flags: 0, driver_data: 1, frequency: 0 },
    CpufreqFrequencyTable { flags: 0, driver_data: 0, frequency: 0xffff_ffff },
];
static mut g5_pmode_cur: i32 = 0;
static mut g5_switch_volt: Option<unsafe extern "C" fn(i32)> = None;
static mut g5_switch_freq: Option<unsafe extern "C" fn(i32) -> i32> = None;
static mut g5_query_freq: Option<unsafe extern "C" fn() -> i32> = None;
static mut transition_latency: u64 = 0;

static mut pfunc_cpu0_volt_high: *mut PmfFunction = core::ptr::null_mut();
static mut pfunc_cpu0_volt_low: *mut PmfFunction = core::ptr::null_mut();
static mut pfunc_cpu1_volt_high: *mut PmfFunction = core::ptr::null_mut();
static mut pfunc_cpu1_volt_low: *mut PmfFunction = core::ptr::null_mut();
static mut pfunc_cpu_setfreq_high: *mut PmfFunction = core::ptr::null_mut();
static mut pfunc_cpu_setfreq_low: *mut PmfFunction = core::ptr::null_mut();
static mut pfunc_cpu_getfreq: *mut PmfFunction = core::ptr::null_mut();
static mut pfunc_slewing_done: *mut PmfFunction = core::ptr::null_mut();

unsafe extern "C" fn g5_pfunc_switch_volt(speed_mode: i32) {
    let (h, l) = if speed_mode == CPUFREQ_HIGH as i32 { (pfunc_cpu0_volt_high, pfunc_cpu1_volt_high) } else { (pfunc_cpu0_volt_low, pfunc_cpu1_volt_low) };
    if !h.is_null() { pmf_call_one(h, core::ptr::null_mut()); } if !l.is_null() { pmf_call_one(l, core::ptr::null_mut()); }
    usleep_range(10000, 10000);
}

unsafe extern "C" fn g5_pfunc_switch_freq(speed_mode: i32) -> i32 {
    if speed_mode < g5_pmode_cur { if let Some(f) = g5_switch_volt { f(speed_mode); } }
    let rc = if speed_mode == CPUFREQ_HIGH as i32 { pmf_call_one(pfunc_cpu_setfreq_high, core::ptr::null_mut()) } else { pmf_call_one(pfunc_cpu_setfreq_low, core::ptr::null_mut()) };
    let mut args = PmfArgs { count: 1, u: [PmfArg { v: 0 }; 16] }; let mut done = 0u32;
    while jiffies <= jiffies.wrapping_add(HZ / 10) { args.u[0].p = &mut done; pmf_call_one(pfunc_slewing_done, &mut args); if done != 0 { break; } usleep_range(500, 500); }
    if speed_mode > g5_pmode_cur { if let Some(f) = g5_switch_volt { f(speed_mode); } }
    g5_pmode_cur = speed_mode; ppc_proc_freq = g5_cpu_freqs[speed_mode as usize].frequency * 1000; rc
}

unsafe extern "C" fn g5_pfunc_query_freq() -> i32 {
    let mut v = 0u32; let mut a = PmfArgs { count: 1, u: [PmfArg { p: &mut v }; 16] }; pmf_call_one(pfunc_cpu_getfreq, &mut a); if v != 0 { CPUFREQ_HIGH as i32 } else { CPUFREQ_LOW as i32 }
}

unsafe extern "C" fn g5_dummy_switch_volt(_: i32) {}

static mut g5_pmode_data: *const u32 = core::ptr::null();
static mut g5_pmode_max: i32 = 0;
unsafe extern "C" fn g5_scom_switch_freq(speed_mode: i32) -> i32 {
    if speed_mode < g5_pmode_cur { if let Some(f) = g5_switch_volt { f(speed_mode); } }
    let mut flags = 0usize; local_irq_save(&mut flags);
    scom970_write(SCOM_PCR, 0); scom970_write(SCOM_PCR, PCR_HILO_SELECT);
    scom970_write(SCOM_PCR, PCR_HILO_SELECT | *g5_pmode_data.add(speed_mode as usize));
    let mut to = 0; while to < 10 { let psr = scom970_read(SCOM_PSR); let p = *g5_pmode_data.add(speed_mode as usize); if (psr & PSR_CMD_RECEIVED as u64) == 0 && ((((psr >> PSR_CUR_SPEED_SHIFT) ^ ((p as u64) >> PCR_SPEED_SHIFT)) & 3) == 0) { break; } if psr & PSR_CMD_COMPLETED != 0 { break; } udelay(100); to += 1; }
    local_irq_restore(flags); if speed_mode > g5_pmode_cur { if let Some(f) = g5_switch_volt { f(speed_mode); } }
    g5_pmode_cur = speed_mode; ppc_proc_freq = g5_cpu_freqs[speed_mode as usize].frequency * 1000; 0
}
unsafe extern "C" fn g5_scom_query_freq() -> i32 {
    let psr = scom970_read(SCOM_PSR); let mut i = 0; while i <= g5_pmode_max { let p = *g5_pmode_data.add(i as usize); if (((psr >> PSR_CUR_SPEED_SHIFT) ^ ((p as u64) >> PCR_SPEED_SHIFT)) & 3) == 0 { break; } i += 1; } i
}

unsafe extern "C" fn g5_cpufreq_target(_: *mut CpufreqPolicy, index: u32) -> i32 { (g5_switch_freq.unwrap())(index as i32) }
unsafe extern "C" fn g5_cpufreq_get_speed(_: u32) -> u64 { g5_cpu_freqs[g5_pmode_cur as usize].frequency }
unsafe extern "C" fn g5_cpufreq_cpu_init(p: *mut CpufreqPolicy) -> i32 { cpufreq_generic_init(p, g5_cpu_freqs.as_mut_ptr(), transition_latency); 0 }

#[no_mangle] pub unsafe extern "C" fn g5_cpufreq_init() -> i32 {
    let n = of_cpu_device_node_get(0); if n.is_null() { return -19; }
    let mut rc = 0; if of_machine_is_compatible(b"PowerMac7,2\0".as_ptr()) || of_machine_is_compatible(b"PowerMac7,3\0".as_ptr()) || of_machine_is_compatible(b"RackMac3,1\0".as_ptr()) { rc = g5_pm72_cpufreq_init(n); } rc
}

unsafe extern "C" fn g5_pm72_cpufreq_init(n: *mut DeviceNode) -> i32 {
    let v = of_get_property(n, b"clock-frequency\0".as_ptr(), core::ptr::null_mut()); if v.is_null() { of_node_put(n); return -19; }
    g5_cpu_freqs[0].frequency = (*v as u64) / 1000; g5_cpu_freqs[1].frequency = g5_cpu_freqs[0].frequency / 2;
    g5_switch_volt = Some(g5_pfunc_switch_volt); g5_switch_freq = Some(g5_pfunc_switch_freq); g5_query_freq = Some(g5_pfunc_query_freq); g5_pmode_cur = -1; (g5_switch_freq.unwrap())((g5_query_freq.unwrap())()); of_node_put(n); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
