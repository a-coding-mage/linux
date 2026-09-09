// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
//		http://www.samsung.com
//
// Based on arch/arm/mach-vexpress/dcscb.c

// Dependencies supplied by the surrounding kernel translation.

const EXYNOS5420_CPUS_PER_CLUSTER: u32 = 4;
const EXYNOS5420_NR_CLUSTERS: u32 = 2;

const EXYNOS5420_ENABLE_AUTOMATIC_CORE_DOWN: u32 = 1 << 9;
const EXYNOS5420_USE_ARM_CORE_DOWN_STATE: u32 = 1 << 29;
const EXYNOS5420_USE_L2_COMMON_UP_STATE: u32 = 1 << 30;

static mut ns_sram_base_addr: *mut core::ffi::c_void = core::ptr::null_mut();
static mut secure_firmware: bool = false;

/*
 * The common v7_exit_coherency_flush API could not be used because of the
 * Erratum 799270 workaround. This is the same operation as the common one,
 * except for the erratum handling.
 */
unsafe fn exynos_v7_exit_coherency_flush(_level: &str) {
    // C inline assembly is architecture-specific and depends on kernel symbols.
    // Preserve its required side effects through the external low-level hook.
    exynos_v7_exit_coherency_flush_impl();
}

unsafe extern "C" {
    fn exynos_v7_exit_coherency_flush_impl();
    fn exynos_cpu_power_state(cpu: u32) -> u32;
    fn exynos_cpu_power_up(cpu: u32);
    fn exynos_cpu_power_down(cpu: u32);
    fn exynos_cluster_power_up(cluster: u32);
    fn exynos_cluster_power_down(cluster: u32);
    fn cpu_logical_map(cpu: u32) -> u32;
    fn mpidr_affinity_level(mpidr: u32, level: u32) -> u32;
    fn pmu_raw_readl(reg: u32) -> u32;
    fn pmu_raw_writel(value: u32, reg: u32);
    fn udelay(usecs: u32);
    fn msleep(msecs: u32);
    fn read_cpuid_part() -> u32;
    fn read_cpuid_mpidr() -> u32;
    fn cci_disable_port_by_cpu(mpidr: u32);
    fn cci_enable_port_for_self();
    fn mcpm_platform_register(ops: *const mcpm_platform_ops) -> i32;
    fn mcpm_sync_init(setup: unsafe extern "C" fn(u32)) -> i32;
    fn mcpm_loopback(disable: unsafe extern "C" fn()) -> i32;
    fn mcpm_smp_set_ops();
    fn exynos_secure_firmware_available() -> bool;
    fn mcpm_entry_point();
    fn __raw_writel(value: u32, addr: *mut core::ffi::c_void);
    fn __pa_symbol(entry: unsafe extern "C" fn()) -> u32;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn register_syscore(syscore: *mut syscore);
    fn pr_debug(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn bug_on(condition: bool);
    fn of_find_matching_node(prev: *mut device_node, match_table: *const of_device_id) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn cci_probed() -> bool;
    fn of_find_compatible_node(prev: *mut device_node, ty: *const u8, compatible: *const u8) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: u32) -> *mut core::ffi::c_void;
}

#[repr(C)]
struct device_node;

#[repr(C)]
struct of_device_id {
    compatible: *const u8,
}

#[repr(C)]
struct mcpm_platform_ops {
    cpu_powerup: unsafe extern "C" fn(u32, u32) -> i32,
    cluster_powerup: unsafe extern "C" fn(u32) -> i32,
    cpu_powerdown_prepare: unsafe extern "C" fn(u32, u32),
    cluster_powerdown_prepare: unsafe extern "C" fn(u32),
    cpu_cache_disable: unsafe extern "C" fn(),
    cluster_cache_disable: unsafe extern "C" fn(),
    wait_for_powerdown: unsafe extern "C" fn(u32, u32) -> i32,
    cpu_is_up: unsafe extern "C" fn(u32, u32),
}

#[repr(C)]
struct syscore_ops {
    resume: unsafe extern "C" fn(*mut core::ffi::c_void),
}

#[repr(C)]
struct syscore {
    ops: *const syscore_ops,
}

const EINVAL: i32 = 22;
const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;
const ETIMEDOUT: i32 = 110;
const ARM_CPU_PART_CORTEX_A15: u32 = 0xc0f;
const EXYNOS5420_KFC_CORE_RESET: fn(u32) -> u32 = |cpu| 0; // external register macro
const EXYNOS_SWRESET: u32 = 0;
const S5P_PMU_SPARE2: u32 = 0;
const S5P_PMU_SPARE3: u32 = 0;
const EXYNOS5420_SWRESET_KFC_SEL: u32 = 0;

unsafe extern "C" fn exynos_cpu_powerup(cpu: u32, cluster: u32) -> i32 {
    let cpunr = cpu + cluster * EXYNOS5420_CPUS_PER_CLUSTER;
    if cpu >= EXYNOS5420_CPUS_PER_CLUSTER || cluster >= EXYNOS5420_NR_CLUSTERS { return -EINVAL; }
    let state = exynos_cpu_power_state(cpunr) != 0;
    exynos_cpu_power_up(cpunr);
    if !state && secure_firmware {
        if cluster != 0 && cluster == mpidr_affinity_level(cpu_logical_map(0), 1) {
            let mut timeout = 16;
            while timeout != 0 && pmu_raw_readl(S5P_PMU_SPARE2) == 0 { timeout -= 1; udelay(10); }
            if timeout == 0 { exynos_cpu_power_down(cpunr); return -ETIMEDOUT; }
            pmu_raw_writel(EXYNOS5420_KFC_CORE_RESET(cpu), EXYNOS_SWRESET);
        }
    }
    0
}

unsafe extern "C" fn exynos_cluster_powerup(cluster: u32) -> i32 {
    if cluster >= EXYNOS5420_NR_CLUSTERS { return -EINVAL; }
    exynos_cluster_power_up(cluster); 0
}

unsafe extern "C" fn exynos_cpu_powerdown_prepare(cpu: u32, cluster: u32) {
    bug_on(cpu >= EXYNOS5420_CPUS_PER_CLUSTER || cluster >= EXYNOS5420_NR_CLUSTERS);
    exynos_cpu_power_down(cpu + cluster * EXYNOS5420_CPUS_PER_CLUSTER);
}

unsafe extern "C" fn exynos_cluster_powerdown_prepare(cluster: u32) {
    bug_on(cluster >= EXYNOS5420_NR_CLUSTERS); exynos_cluster_power_down(cluster);
}

unsafe extern "C" fn exynos_cpu_cache_disable() { exynos_v7_exit_coherency_flush("louis"); }

unsafe extern "C" fn exynos_cluster_cache_disable() {
    if read_cpuid_part() == ARM_CPU_PART_CORTEX_A15 { /* disable A15 L2 prefetching */ }
    exynos_v7_exit_coherency_flush("all");
    cci_disable_port_by_cpu(read_cpuid_mpidr());
}

unsafe extern "C" fn exynos_wait_for_powerdown(cpu: u32, cluster: u32) -> i32 {
    bug_on(cpu >= EXYNOS5420_CPUS_PER_CLUSTER || cluster >= EXYNOS5420_NR_CLUSTERS);
    let cpunr = cpu + cluster * EXYNOS5420_CPUS_PER_CLUSTER;
    let mut tries = 100;
    while tries != 0 { if exynos_cpu_power_state(cpunr) == 0 { return 0; } tries -= 1; msleep(1); }
    -ETIMEDOUT
}

unsafe extern "C" fn exynos_cpu_is_up(cpu: u32, cluster: u32) { exynos_cpu_powerup(cpu, cluster); }

static exynos_power_ops: mcpm_platform_ops = mcpm_platform_ops {
    cpu_powerup: exynos_cpu_powerup, cluster_powerup: exynos_cluster_powerup,
    cpu_powerdown_prepare: exynos_cpu_powerdown_prepare, cluster_powerdown_prepare: exynos_cluster_powerdown_prepare,
    cpu_cache_disable: exynos_cpu_cache_disable, cluster_cache_disable: exynos_cluster_cache_disable,
    wait_for_powerdown: exynos_wait_for_powerdown, cpu_is_up: exynos_cpu_is_up,
};

unsafe extern "C" fn exynos_pm_power_up_setup(affinity_level: u32) { if affinity_level == 1 { cci_enable_port_for_self(); } }

static exynos_dt_mcpm_match: [of_device_id; 3] = [
    of_device_id { compatible: b"samsung,exynos5420\0".as_ptr() },
    of_device_id { compatible: b"samsung,exynos5800\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

unsafe extern "C" fn exynos_mcpm_setup_entry_point(_data: *mut core::ffi::c_void) {
    __raw_writel(0xe59f0000, ns_sram_base_addr);
    __raw_writel(0xe12fff10, ns_sram_base_addr.add(4));
    __raw_writel(__pa_symbol(mcpm_entry_point), ns_sram_base_addr.add(8));
}

static exynos_mcpm_syscore_ops: syscore_ops = syscore_ops { resume: exynos_mcpm_setup_entry_point };
static mut exynos_mcpm_syscore: syscore = syscore { ops: &exynos_mcpm_syscore_ops };

unsafe extern "C" fn exynos_mcpm_init() -> i32 {
    let mut node = of_find_matching_node(core::ptr::null_mut(), exynos_dt_mcpm_match.as_ptr());
    if node.is_null() { return -ENODEV; } of_node_put(node);
    if !cci_probed() { return -ENODEV; }
    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"samsung,exynos4210-sysram-ns\0".as_ptr());
    if node.is_null() { return -ENODEV; }
    ns_sram_base_addr = of_iomap(node, 0); of_node_put(node);
    if ns_sram_base_addr.is_null() { return -ENOMEM; }
    secure_firmware = exynos_secure_firmware_available();
    pmu_raw_writel(EXYNOS5420_SWRESET_KFC_SEL, S5P_PMU_SPARE3);
    let mut ret = mcpm_platform_register(&exynos_power_ops);
    if ret == 0 { ret = mcpm_sync_init(exynos_pm_power_up_setup); }
    if ret == 0 { ret = mcpm_loopback(exynos_cluster_cache_disable); }
    if ret != 0 { iounmap(ns_sram_base_addr); return ret; }
    mcpm_smp_set_ops();
    for i in 0..EXYNOS5420_NR_CLUSTERS {
        let mut value = pmu_raw_readl(i);
        value |= EXYNOS5420_ENABLE_AUTOMATIC_CORE_DOWN | EXYNOS5420_USE_ARM_CORE_DOWN_STATE | EXYNOS5420_USE_L2_COMMON_UP_STATE;
        pmu_raw_writel(value, i);
    }
    exynos_mcpm_setup_entry_point(core::ptr::null_mut());
    register_syscore(&mut exynos_mcpm_syscore);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
