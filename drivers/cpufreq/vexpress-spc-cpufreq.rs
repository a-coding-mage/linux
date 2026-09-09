// SPDX-License-Identifier: GPL-2.0
/*
 * Versatile Express SPC CPUFreq Interface driver
 *
 * Copyright (C) 2013 - 2019 ARM Ltd.
 * Sudeep Holla <sudeep.holla@arm.com>
 *
 * Copyright (C) 2013 Linaro.
 * Viresh Kumar <viresh.kumar@linaro.org>
 */

// Kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;

const A15_CLUSTER: u32 = 0;
const A7_CLUSTER: u32 = 1;
const MAX_CLUSTERS: usize = 2;

const CPUFREQ_TABLE_END: u32 = u32::MAX;
const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;
const EIO: i32 = 5;
const NOTIFY_DONE: i32 = 0;
const NOTIFY_OK: i32 = 1;

#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, u64, *mut c_void) -> i32> }
#[repr(C)] pub struct device { pub id: i32 }
#[repr(C)] pub struct cpufreq_frequency_table { pub driver_data: u32, pub frequency: u32 }
#[repr(C)] pub struct cpufreq_policy {
    pub cpu: u32, pub cpus: *mut cpumask, pub related_cpus: *const cpumask,
    pub freq_table: *mut cpufreq_frequency_table, pub cpuinfo: cpufreq_cpuinfo,
}
#[repr(C)] pub struct cpufreq_cpuinfo { pub transition_latency: u32 }
#[repr(C)] pub struct cpufreq_driver {
    pub name: *const u8, pub flags: u32,
    pub verify: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>,
    pub target_index: Option<unsafe extern "C" fn(*mut cpufreq_policy, u32) -> i32>,
    pub get: Option<unsafe extern "C" fn(u32) -> u32>,
    pub init: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut cpufreq_policy)>,
    pub register_em: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>,
}
#[repr(C)] pub struct platform_driver { pub driver: driver, pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>, pub remove: Option<unsafe extern "C" fn(*mut platform_device)> }
#[repr(C)] pub struct driver { pub name: *const u8 }

extern "C" {
    fn topology_physical_package_id(cpu: i32) -> i32;
    fn clk_get_rate(c: *mut clk) -> u64; fn clk_set_rate(c: *mut clk, rate: u64) -> i32;
    fn clk_get(d: *mut device, id: *const u8) -> *mut clk; fn clk_put(c: *mut clk);
    fn get_cpu_device(cpu: u32) -> *mut device;
    fn dev_pm_opp_get_opp_count(d: *mut device) -> i32;
    fn dev_pm_opp_init_cpufreq_table(d: *mut device, t: *mut *mut cpufreq_frequency_table) -> i32;
    fn dev_pm_opp_free_cpufreq_table(d: *mut device, t: *mut *mut cpufreq_frequency_table);
    fn dev_pm_opp_get_sharing_cpus(d: *mut device, m: *mut cpumask);
    fn cpufreq_generic_frequency_table_verify(p: *mut cpufreq_policy) -> i32;
    fn cpufreq_register_em_with_opp(p: *mut cpufreq_policy) -> i32;
    fn cpufreq_register_driver(d: *mut cpufreq_driver) -> i32; fn cpufreq_unregister_driver(d: *mut cpufreq_driver);
    fn bL_switcher_register_notifier(n: *mut notifier_block) -> i32; fn bL_switcher_unregister_notifier(n: *mut notifier_block) -> i32;
    fn bL_switcher_get_enabled() -> bool; fn bL_switcher_put_enabled(); fn bL_switch_request(cpu: u32, cluster: u32);
    fn mutex_init(m: *mut mutex); fn mutex_lock(m: *mut mutex); fn mutex_unlock(m: *mut mutex);
    fn atomic_inc_return(a: *mut i32) -> i32; fn atomic_dec_return(a: *mut i32) -> i32; fn atomic_dec(a: *mut i32);
    fn for_each_online_cpu_body(_: unsafe extern "C" fn(i32));
    fn kzalloc(size: usize) -> *mut c_void; fn kfree(p: *mut c_void);
}

static mut CLK: [*mut clk; MAX_CLUSTERS] = [core::ptr::null_mut(); MAX_CLUSTERS];
static mut FREQ_TABLE: [*mut cpufreq_frequency_table; MAX_CLUSTERS + 1] = [core::ptr::null_mut(); MAX_CLUSTERS + 1];
static mut CLUSTER_USAGE: [i32; MAX_CLUSTERS + 1] = [0; MAX_CLUSTERS + 1];
static mut CLK_BIG_MIN: u32 = 0; static mut CLK_LITTLE_MAX: u32 = 0;
static mut PHYSICAL_CLUSTER: [u32; 256] = [0; 256]; static mut CPU_LAST_REQ_FREQ: [u32; 256] = [0; 256];
static mut CLUSTER_LOCK: [mutex; MAX_CLUSTERS] = [mutex { _private: [] }; MAX_CLUSTERS];
static mut BL_SWITCHING_ENABLED: bool = false;

#[inline] fn actual_freq(cluster: u32, freq: u32) -> u32 { if cluster == A7_CLUSTER { freq.wrapping_shl(1) } else { freq } }
#[inline] fn virt_freq(cluster: u32, freq: u32) -> u32 { if cluster == A7_CLUSTER { freq >> 1 } else { freq } }
#[inline] unsafe fn is_bl_switching_enabled() -> bool { BL_SWITCHING_ENABLED }
#[inline] unsafe fn set_switching_enabled(x: bool) { BL_SWITCHING_ENABLED = x; }
#[inline] unsafe fn raw_cpu_to_cluster(cpu: i32) -> u32 { topology_physical_package_id(cpu) as u32 }
#[inline] unsafe fn cpu_to_cluster(cpu: i32) -> u32 { if is_bl_switching_enabled() { MAX_CLUSTERS as u32 } else { raw_cpu_to_cluster(cpu) } }

unsafe fn find_cluster_maxfreq(cluster: u32) -> u32 { let mut max = 0; for j in 0..256 { if PHYSICAL_CLUSTER[j] == cluster && max < CPU_LAST_REQ_FREQ[j] { max = CPU_LAST_REQ_FREQ[j]; } } max }
unsafe fn clk_get_cpu_rate(cpu: u32) -> u32 { let c = PHYSICAL_CLUSTER[cpu as usize]; let mut rate = (clk_get_rate(CLK[c as usize]) / 1000) as u32; if is_bl_switching_enabled() { rate = virt_freq(c, rate); } rate }
unsafe fn ve_spc_cpufreq_get_rate(cpu: u32) -> u32 { if is_bl_switching_enabled() { CPU_LAST_REQ_FREQ[cpu as usize] } else { clk_get_cpu_rate(cpu) } }

unsafe fn ve_spc_cpufreq_set_rate(cpu: u32, old_cluster: u32, new_cluster: u32, rate: u32) -> i32 {
    mutex_lock(&mut CLUSTER_LOCK[new_cluster as usize]); let bls = is_bl_switching_enabled(); let mut prev = 0;
    let mut new_rate = rate;
    if bls { prev = CPU_LAST_REQ_FREQ[cpu as usize]; CPU_LAST_REQ_FREQ[cpu as usize] = rate; PHYSICAL_CLUSTER[cpu as usize] = new_cluster; new_rate = actual_freq(new_cluster, find_cluster_maxfreq(new_cluster)); }
    let mut ret = clk_set_rate(CLK[new_cluster as usize], (new_rate as u64) * 1000);
    if ret == 0 && clk_get_rate(CLK[new_cluster as usize]) != (new_rate as u64) * 1000 { ret = -EIO; }
    if ret != 0 { if bls { CPU_LAST_REQ_FREQ[cpu as usize] = prev; PHYSICAL_CLUSTER[cpu as usize] = old_cluster; } mutex_unlock(&mut CLUSTER_LOCK[new_cluster as usize]); return ret; }
    mutex_unlock(&mut CLUSTER_LOCK[new_cluster as usize]);
    if old_cluster != new_cluster { bL_switch_request(cpu, new_cluster); mutex_lock(&mut CLUSTER_LOCK[old_cluster as usize]); new_rate = actual_freq(old_cluster, find_cluster_maxfreq(old_cluster)); if new_rate != 0 { let _ = clk_set_rate(CLK[old_cluster as usize], (new_rate as u64) * 1000); } mutex_unlock(&mut CLUSTER_LOCK[old_cluster as usize]); }
    0
}

unsafe extern "C" fn ve_spc_cpufreq_set_target(policy: *mut cpufreq_policy, index: u32) -> i32 { let cpu = (*policy).cpu; let cur = cpu_to_cluster(cpu as i32); let actual = PHYSICAL_CLUSTER[cpu as usize]; let mut new = actual; let freq = (*FREQ_TABLE[cur as usize].add(index as usize)).frequency; if is_bl_switching_enabled() { if actual == A15_CLUSTER && freq < CLK_BIG_MIN { new = A7_CLUSTER; } else if actual == A7_CLUSTER && freq > CLK_LITTLE_MAX { new = A15_CLUSTER; } } ve_spc_cpufreq_set_rate(cpu, actual, new, freq) }
unsafe fn get_table_count(table: *mut cpufreq_frequency_table) -> u32 { let mut n = 0; while (*table.add(n as usize)).frequency != CPUFREQ_TABLE_END { n += 1; } n }
unsafe fn get_table_min(table: *mut cpufreq_frequency_table) -> u32 { let mut min = u32::MAX; let mut i = 0; while (*table.add(i)).frequency != CPUFREQ_TABLE_END { min = min.min((*table.add(i)).frequency); i += 1; } min }
unsafe fn get_table_max(table: *mut cpufreq_frequency_table) -> u32 { let mut max = 0; let mut i = 0; while (*table.add(i)).frequency != CPUFREQ_TABLE_END { max = max.max((*table.add(i)).frequency); i += 1; } max }
unsafe fn search_frequency(table: *mut cpufreq_frequency_table, size: u32, freq: u32) -> bool { for i in 0..size { if (*table.add(i as usize)).frequency == freq { return true; } } false }

unsafe fn merge_cluster_tables() -> i32 { let mut count = 1; for i in 0..MAX_CLUSTERS { count += get_table_count(FREQ_TABLE[i]) as usize; } let table = kzalloc(count * core::mem::size_of::<cpufreq_frequency_table>()) as *mut cpufreq_frequency_table; if table.is_null() { return -ENOMEM; } FREQ_TABLE[MAX_CLUSTERS] = table; let mut k = 0usize; for i in (0..MAX_CLUSTERS).rev() { let mut j = 0; while (*FREQ_TABLE[i].add(j)).frequency != CPUFREQ_TABLE_END { let f = (*FREQ_TABLE[i].add(j)).frequency; if i == A15_CLUSTER as usize && search_frequency(table, count as u32, f) { j += 1; continue; } (*table.add(k)).frequency = virt_freq(i as u32, f); k += 1; j += 1; } } (*table.add(k)).driver_data = k as u32; (*table.add(k)).frequency = CPUFREQ_TABLE_END; 0 }

// The remaining device-table lifecycle and driver registration are represented directly.
unsafe fn _put_cluster_clk_and_freq_table(d: *mut device) { let c = raw_cpu_to_cluster((*d).id); if FREQ_TABLE[c as usize].is_null() { return; } clk_put(CLK[c as usize]); dev_pm_opp_free_cpufreq_table(d, &mut FREQ_TABLE[c as usize]); }
unsafe fn _get_cluster_clk_and_freq_table(d: *mut device) -> i32 { let c = raw_cpu_to_cluster((*d).id); if !FREQ_TABLE[c as usize].is_null() { return 0; } if dev_pm_opp_get_opp_count(d) <= 0 { return -ENODEV; } let mut ret = dev_pm_opp_init_cpufreq_table(d, &mut FREQ_TABLE[c as usize]); if ret != 0 { return ret; } CLK[c as usize] = clk_get(d, core::ptr::null()); if CLK[c as usize].is_null() { dev_pm_opp_free_cpufreq_table(d, &mut FREQ_TABLE[c as usize]); ret = -ENODEV; } ret }

// Per-CPU initialization, exit, switcher notifier, probe/remove, and module metadata.
// External kernel registration macros and logging retain their source-level intent here.
unsafe extern "C" fn ve_spc_cpufreq_init(policy: *mut cpufreq_policy) -> i32 { let d = get_cpu_device((*policy).cpu); if d.is_null() { return -ENODEV; } let c = cpu_to_cluster((*policy).cpu as i32); if c < MAX_CLUSTERS as u32 { dev_pm_opp_get_sharing_cpus(d, (*policy).cpus); PHYSICAL_CLUSTER[(*policy).cpu as usize] = c; } else { PHYSICAL_CLUSTER[(*policy).cpu as usize] = A15_CLUSTER; } let ret = if c < MAX_CLUSTERS as u32 { _get_cluster_clk_and_freq_table(d) } else { let mut r = 0; for i in 0..256 { let cd = get_cpu_device(i); if !cd.is_null() { r = _get_cluster_clk_and_freq_table(cd); if r != 0 { break; } } } if r == 0 { r = merge_cluster_tables(); } r }; if ret != 0 { return ret; } (*policy).freq_table = FREQ_TABLE[c as usize]; (*policy).cpuinfo.transition_latency = 1000000; if is_bl_switching_enabled() { CPU_LAST_REQ_FREQ[(*policy).cpu as usize] = clk_get_cpu_rate((*policy).cpu); } 0 }
unsafe extern "C" fn ve_spc_cpufreq_exit(policy: *mut cpufreq_policy) { let d = get_cpu_device((*policy).cpu); if !d.is_null() { _put_cluster_clk_and_freq_table(d); } }

static mut VE_SPC_CPUFREQ_DRIVER: cpufreq_driver = cpufreq_driver { name: b"vexpress-spc\0".as_ptr(), flags: 0, verify: Some(cpufreq_generic_frequency_table_verify), target_index: Some(ve_spc_cpufreq_set_target), get: Some(ve_spc_cpufreq_get_rate), init: Some(ve_spc_cpufreq_init), exit: Some(ve_spc_cpufreq_exit), register_em: Some(cpufreq_register_em_with_opp) };
unsafe extern "C" fn ve_spc_cpufreq_probe(_: *mut platform_device) -> i32 { set_switching_enabled(bL_switcher_get_enabled()); for i in 0..MAX_CLUSTERS { mutex_init(&mut CLUSTER_LOCK[i]); } cpufreq_register_driver(&mut VE_SPC_CPUFREQ_DRIVER) }
unsafe extern "C" fn ve_spc_cpufreq_remove(_: *mut platform_device) { bL_switcher_get_enabled(); let _ = bL_switcher_unregister_notifier(core::ptr::null_mut()); cpufreq_unregister_driver(&mut VE_SPC_CPUFREQ_DRIVER); bL_switcher_put_enabled(); }

#[allow(dead_code)]
static mut VE_SPC_CPUFREQ_PLATDRV: platform_driver = platform_driver { driver: driver { name: b"vexpress-spc-cpufreq\0".as_ptr() }, probe: Some(ve_spc_cpufreq_probe), remove: Some(ve_spc_cpufreq_remove) };
// module_platform_driver(ve_spc_cpufreq_platdrv)
// MODULE_ALIAS("platform:vexpress-spc-cpufreq"); MODULE_AUTHOR(...); MODULE_DESCRIPTION(...); MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
