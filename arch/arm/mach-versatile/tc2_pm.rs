// SPDX-License-Identifier: GPL-2.0-only
/*
 * Created by: Nicolas Pitre, October 2012
 * Copyright: (C) 2012-2013 Linaro Limited
 *
 * Some portions of this file were originally written by Achin Gupta
 * Copyright: (C) 2012 ARM Limited
 */

// Kernel and architecture dependencies supplied by other translation units.

const RESET_CTRL: usize = 0x018;
const RESET_A15_NCORERESET: fn(u32) -> u32 = |cpu| 1u32 << (2 + cpu);
const RESET_A7_NCORERESET: fn(u32) -> u32 = |cpu| 1u32 << (16 + cpu);
const A15_CONF: usize = 0x400;
const A7_CONF: usize = 0x500;
const SYS_INFO: usize = 0x700;
const SPC_BASE: usize = 0xb00;

static mut scc: *mut core::ffi::c_void = core::ptr::null_mut();
const TC2_CLUSTERS: usize = 2;
const TC2_MAX_CPUS_PER_CLUSTER: u32 = 3;
static mut tc2_nr_cpus: [u32; TC2_CLUSTERS] = [0; TC2_CLUSTERS];

unsafe fn tc2_pm_cpu_powerup(cpu: u32, cluster: u32) -> i32 {
    pr_debug!("%s: cpu %u cluster %u\n", "tc2_pm_cpu_powerup", cpu, cluster);
    if cluster as usize >= TC2_CLUSTERS || cpu >= tc2_nr_cpus[cluster as usize] { return -EINVAL; }
    ve_spc_set_resume_addr(cluster, cpu, __pa_symbol(mcpm_entry_point));
    ve_spc_cpu_wakeup_irq(cluster, cpu, true);
    0
}

unsafe fn tc2_pm_cluster_powerup(cluster: u32) -> i32 {
    pr_debug!("%s: cluster %u\n", "tc2_pm_cluster_powerup", cluster);
    if cluster as usize >= TC2_CLUSTERS { return -EINVAL; }
    ve_spc_powerdown(cluster, false);
    0
}

unsafe fn tc2_pm_cpu_powerdown_prepare(cpu: u32, cluster: u32) {
    pr_debug!("%s: cpu %u cluster %u\n", "tc2_pm_cpu_powerdown_prepare", cpu, cluster);
    BUG_ON!(cluster as usize >= TC2_CLUSTERS || cpu >= TC2_MAX_CPUS_PER_CLUSTER);
    ve_spc_cpu_wakeup_irq(cluster, cpu, true);
    gic_cpu_if_down(0);
}

unsafe fn tc2_pm_cluster_powerdown_prepare(cluster: u32) {
    pr_debug!("%s: cluster %u\n", "tc2_pm_cluster_powerdown_prepare", cluster);
    BUG_ON!(cluster as usize >= TC2_CLUSTERS);
    ve_spc_powerdown(cluster, true);
    ve_spc_global_wakeup_irq(true);
}

unsafe fn tc2_pm_cpu_cache_disable() { v7_exit_coherency_flush(louis); }

unsafe fn tc2_pm_cluster_cache_disable() {
    if read_cpuid_part() == ARM_CPU_PART_CORTEX_A15 {
        // On Cortex-A15, disable L2 prefetching before flushing the cache.
        core::arch::asm!("mcr p15, 1, {0}, c15, c0, 3", "isb", "dsb", in(reg) 0x400u32);
    }
    v7_exit_coherency_flush(all);
    cci_disable_port_by_cpu(read_cpuid_mpidr());
}

unsafe fn tc2_core_in_reset(cpu: u32, cluster: u32) -> i32 {
    let mask = if cluster != 0 { RESET_A7_NCORERESET(cpu) } else { RESET_A15_NCORERESET(cpu) };
    if (readl_relaxed((scc as *mut u8).add(RESET_CTRL)) & mask) == 0 { 1 } else { 0 }
}

const POLL_MSEC: u32 = 10;
const TIMEOUT_MSEC: u32 = 1000;

unsafe fn tc2_pm_wait_for_powerdown(cpu: u32, cluster: u32) -> i32 {
    BUG_ON!(cluster as usize >= TC2_CLUSTERS || cpu >= TC2_MAX_CPUS_PER_CLUSTER);
    for _tries in 0..(TIMEOUT_MSEC / POLL_MSEC) {
        if tc2_core_in_reset(cpu, cluster) != 0 || ve_spc_cpu_in_wfi(cpu, cluster) { return 0; }
        msleep(POLL_MSEC);
    }
    -ETIMEDOUT
}

unsafe fn tc2_pm_cpu_suspend_prepare(cpu: u32, cluster: u32) {
    ve_spc_set_resume_addr(cluster, cpu, __pa_symbol(mcpm_entry_point));
}

unsafe fn tc2_pm_cpu_is_up(cpu: u32, cluster: u32) {
    BUG_ON!(cluster as usize >= TC2_CLUSTERS || cpu >= TC2_MAX_CPUS_PER_CLUSTER);
    ve_spc_cpu_wakeup_irq(cluster, cpu, false);
    ve_spc_set_resume_addr(cluster, cpu, 0);
}

unsafe fn tc2_pm_cluster_is_up(cluster: u32) {
    BUG_ON!(cluster as usize >= TC2_CLUSTERS);
    ve_spc_powerdown(cluster, false);
    ve_spc_global_wakeup_irq(false);
}

// The mcpm_platform_ops table and its callbacks are supplied by the kernel ABI.
static tc2_pm_power_ops: mcpm_platform_ops = mcpm_platform_ops {
    cpu_powerup: Some(tc2_pm_cpu_powerup), cluster_powerup: Some(tc2_pm_cluster_powerup),
    cpu_suspend_prepare: Some(tc2_pm_cpu_suspend_prepare), cpu_powerdown_prepare: Some(tc2_pm_cpu_powerdown_prepare),
    cluster_powerdown_prepare: Some(tc2_pm_cluster_powerdown_prepare), cpu_cache_disable: Some(tc2_pm_cpu_cache_disable),
    cluster_cache_disable: Some(tc2_pm_cluster_cache_disable), wait_for_powerdown: Some(tc2_pm_wait_for_powerdown),
    cpu_is_up: Some(tc2_pm_cpu_is_up), cluster_is_up: Some(tc2_pm_cluster_is_up),
};

unsafe fn tc2_pm_power_up_setup(affinity_level: u32) {
    if affinity_level != 1 { return; }
    cci_enable_port_for_self();
}

unsafe fn tc2_pm_init() -> i32 {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "arm,vexpress-scc,v2p-ca15_a7");
    scc = of_iomap(np, 0);
    if scc.is_null() { return -ENODEV; }
    let a15_cluster_id = readl_relaxed((scc as *mut u8).add(A15_CONF)) & 0xf;
    let a7_cluster_id = readl_relaxed((scc as *mut u8).add(A7_CONF)) & 0xf;
    if a15_cluster_id as usize >= TC2_CLUSTERS || a7_cluster_id as usize >= TC2_CLUSTERS { return -EINVAL; }
    let sys_info = readl_relaxed((scc as *mut u8).add(SYS_INFO));
    tc2_nr_cpus[a15_cluster_id as usize] = (sys_info >> 16) & 0xf;
    tc2_nr_cpus[a7_cluster_id as usize] = (sys_info >> 20) & 0xf;
    let irq = irq_of_parse_and_map(np, 0);
    let ret = ve_spc_init((scc as *mut u8).add(SPC_BASE), a15_cluster_id, irq);
    if ret != 0 { return ret; }
    if !cci_probed() { return -ENODEV; }
    let mpidr = read_cpuid_mpidr();
    let cpu = MPIDR_AFFINITY_LEVEL(mpidr, 0);
    let cluster = MPIDR_AFFINITY_LEVEL(mpidr, 1);
    if cluster as usize >= TC2_CLUSTERS || cpu >= tc2_nr_cpus[cluster as usize] { return -EINVAL; }
    let ret = mcpm_platform_register(&tc2_pm_power_ops);
    if ret == 0 { mcpm_sync_init(tc2_pm_power_up_setup); BUG_ON!(mcpm_loopback(tc2_pm_cluster_cache_disable) != 0); }
    ret
}

// early_initcall(tc2_pm_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
