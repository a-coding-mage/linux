// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 MediaTek Inc. */

// Linux kernel dependencies are supplied by the surrounding translation.

const LUT_MAX_ENTRIES: u32 = 32;
const LUT_FREQ: u32 = 0xfff;
const LUT_ROW_SIZE: usize = 0x4;
const CPUFREQ_HW_STATUS: i32 = 1 << 0;
const SVS_HW_STATUS: i32 = 1 << 1;
const POLL_USEC: u32 = 1000;
const TIMEOUT_USEC: u32 = 300000;
const FDVFS_FDIV_HZ: u32 = 26 * 1000;

#[repr(usize)]
enum RegIndex {
    RegFreqLutTable,
    RegFreqEnable,
    RegFreqPerfState,
    RegFreqHwState,
    RegEmPowerTbl,
    RegFreqLatency,
    RegArraySize,
}

#[repr(C)]
struct MtkCpufreqPriv {
    dev: *mut Device,
    variant: *const MtkCpufreqVariant,
    fdvfs: *mut u8,
}

#[repr(C)]
struct MtkCpufreqDomain {
    parent: *mut MtkCpufreqPriv,
    table: *mut CpufreqFrequencyTable,
    reg_bases: [*mut u8; RegIndex::RegArraySize as usize],
    res: *mut Resource,
    base: *mut u8,
    nr_opp: i32,
}

#[repr(C)]
struct MtkCpufreqVariant {
    init: Option<unsafe extern "C" fn(*mut MtkCpufreqPriv) -> i32>,
    reg_offsets: [u16; RegIndex::RegArraySize as usize],
    is_hybrid_dvfs: bool,
}

#[allow(non_camel_case_types, dead_code)]
type Device = core::ffi::c_void;
#[allow(non_camel_case_types, dead_code)]
type PlatformDevice = core::ffi::c_void;
#[allow(non_camel_case_types, dead_code)]
type CpufreqPolicy = core::ffi::c_void;
#[allow(non_camel_case_types, dead_code)]
type CpufreqFrequencyTable = core::ffi::c_void;
#[allow(non_camel_case_types, dead_code)]
type Resource = core::ffi::c_void;

extern "C" {
    fn devm_of_iomap(dev: *mut Device, node: *mut core::ffi::c_void, index: i32, arg: *mut core::ffi::c_void) -> *mut u8;
    fn is_err(ptr: *mut u8) -> bool;
    fn ptr_err(ptr: *mut u8) -> i32;
    fn dev_err_probe(dev: *mut Device, err: i32, fmt: *const u8, ...) -> i32;
    fn cpufreq_cpu_get_raw(cpu: u32) -> *mut CpufreqPolicy;
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn cpufreq_table_find_index_dl(policy: *mut CpufreqPolicy, freq: u32, relation: bool) -> u32;
    fn cpufreq_get_driver_data() -> *mut PlatformDevice;
    fn of_perf_domain_get_sharing_cpumask(cpu: u32, name: *const u8, cells: *const u8, cpus: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> i32;
    fn of_node_put(node: *mut core::ffi::c_void);
    fn platform_get_resource(pdev: *mut PlatformDevice, typ: u32, index: i32) -> *mut Resource;
    fn resource_start(res: *mut Resource) -> usize;
    fn resource_size(res: *mut Resource) -> usize;
    fn request_mem_region(start: usize, size: usize, name: *const u8) -> bool;
    fn release_mem_region(start: usize, size: usize);
    fn ioremap(start: usize, size: usize) -> *mut u8;
    fn iounmap(base: *mut u8);
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_kcalloc(dev: *mut Device, n: usize, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn of_device_get_match_data(dev: *mut Device) -> *const core::ffi::c_void;
    fn platform_get_drvdata(pdev: *mut PlatformDevice) -> *mut MtkCpufreqPriv;
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut MtkCpufreqPriv);
    fn cpufreq_register_driver(driver: *mut CpufreqDriver) -> i32;
    fn cpufreq_unregister_driver(driver: *mut CpufreqDriver);
    fn get_cpu_device(cpu: i32) -> *mut Device;
    fn devm_regulator_get(dev: *mut Device, name: *const u8) -> *mut core::ffi::c_void;
    fn em_dev_register_perf_domain(dev: *mut Device, nr: i32, cb: *const core::ffi::c_void, cpus: *mut core::ffi::c_void, freq: bool);
}

static CPufreq_MTK_BASE_VARIANT: MtkCpufreqVariant = MtkCpufreqVariant {
    init: None, reg_offsets: [0, 0x84, 0x88, 0x8c, 0x90, 0x110], is_hybrid_dvfs: false,
};
static CPufreq_MTK_MT8196_VARIANT: MtkCpufreqVariant = MtkCpufreqVariant {
    init: Some(mtk_cpufreq_hw_mt8196_init), reg_offsets: [0, 0x84, 0x88, 0x8c, 0x90, 0x114], is_hybrid_dvfs: true,
};

unsafe extern "C" fn mtk_cpufreq_hw_mt8196_init(priv_: *mut MtkCpufreqPriv) -> i32 {
    (*priv_).fdvfs = devm_of_iomap((*priv_).dev, core::ptr::null_mut(), 0, core::ptr::null_mut());
    if is_err((*priv_).fdvfs) { return dev_err_probe((*priv_).dev, ptr_err((*priv_).fdvfs), b"failed to get fdvfs iomem\0".as_ptr()); }
    0
}

unsafe fn mtk_cpufreq_hw_fdvfs_switch(target_freq: u32, policy: *mut CpufreqPolicy) {
    let data = policy_driver_data(policy);
    let priv_ = (*data).parent;
    let target_freq = (target_freq + FDVFS_FDIV_HZ - 1) / FDVFS_FDIV_HZ;
    for cpu in real_cpus(policy) { writel_relaxed(target_freq, (*priv_).fdvfs.add(cpu as usize * 4)); }
}

unsafe fn mtk_cpufreq_hw_target_index(policy: *mut CpufreqPolicy, index: u32) -> i32 {
    let data = policy_driver_data(policy);
    if !(*(*data).parent).fdvfs.is_null() { mtk_cpufreq_hw_fdvfs_switch(freq_table_frequency(policy, index), policy); }
    else { writel_relaxed(index, (*data).reg_bases[RegIndex::RegFreqPerfState as usize]); }
    0
}

unsafe fn mtk_cpufreq_hw_get(cpu: u32) -> u32 {
    let policy = cpufreq_cpu_get_raw(cpu); if policy.is_null() { return 0; }
    let data = policy_driver_data(policy);
    let index = core::cmp::min(readl_relaxed((*data).reg_bases[RegIndex::RegFreqPerfState as usize]), LUT_MAX_ENTRIES - 1);
    table_frequency((*data).table, index)
}

unsafe fn mtk_cpufreq_hw_fast_switch(policy: *mut CpufreqPolicy, target_freq: u32) -> u32 {
    let data = policy_driver_data(policy); let index = cpufreq_table_find_index_dl(policy, target_freq, false);
    if !(*(*data).parent).fdvfs.is_null() { mtk_cpufreq_hw_fdvfs_switch(target_freq, policy); }
    else { writel_relaxed(index, (*data).reg_bases[RegIndex::RegFreqPerfState as usize]); }
    freq_table_frequency(policy, index)
}

// The remaining driver registration and resource-management routines retain the C interfaces;
// their kernel object layouts and helper macros are supplied by the surrounding translation.
extern "C" {
    fn mtk_cpu_create_freq_table(pdev: *mut PlatformDevice, data: *mut MtkCpufreqDomain) -> i32;
    fn mtk_cpu_resources_init(pdev: *mut PlatformDevice, policy: *mut CpufreqPolicy, priv_: *mut MtkCpufreqPriv) -> i32;
    fn mtk_cpufreq_hw_cpu_init(policy: *mut CpufreqPolicy) -> i32;
    fn mtk_cpufreq_hw_cpu_exit(policy: *mut CpufreqPolicy);
    fn mtk_cpufreq_register_em(policy: *mut CpufreqPolicy);
}

extern "C" {
    fn policy_driver_data(policy: *mut CpufreqPolicy) -> *mut MtkCpufreqDomain;
    fn real_cpus(policy: *mut CpufreqPolicy) -> CpuIterator;
    fn freq_table_frequency(policy: *mut CpufreqPolicy, index: u32) -> u32;
    fn table_frequency(table: *mut CpufreqFrequencyTable, index: u32) -> u32;
}
#[repr(C)] pub struct CpuIterator { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
