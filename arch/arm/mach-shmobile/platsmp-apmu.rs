// SPDX-License-Identifier: GPL-2.0
/*
 * SMP support for SoCs with APMU
 *
 * Copyright (C) 2014  Renesas Electronics Corporation
 * Copyright (C) 2013  Magnus Damm
 */

// Kernel dependencies supplied by other translation units.

#[repr(C)]
struct ApmuCpu {
    iomem: *mut core::ffi::c_void,
    bit: i32,
}

static mut APMU_CPUS: [ApmuCpu; NR_CPUS] = [
    ApmuCpu { iomem: core::ptr::null_mut(), bit: 0 };
    NR_CPUS
];

const WUPCR_OFFS: usize = 0x10; // Wake Up Control Register
const PSTR_OFFS: usize = 0x40; // Power Status Register
const DBGRCR_OFFS: usize = 0x180; // Debug Resource Reset Control Reg.

const fn cpuncr_offs(n: i32) -> usize { 0x100 + (0x10 * n as usize) }
const fn cpunst(r: u32, n: i32) -> u32 { (r >> (n * 4)) & 3 }
const CPUST_RUN: u32 = 0;
const CPUST_STANDBY: u32 = 3;
const DBGCPUREN: u32 = 1 << 24;
const fn dbgcpunren(n: i32) -> u32 { 1 << (n + 20) }
const DBGCPUPREN: u32 = 1 << 19;

unsafe fn apmu_power_on(p: *mut core::ffi::c_void, bit: i32) -> i32 {
    // request power on
    writel_relaxed(1u32.wrapping_shl(bit as u32), p.add(WUPCR_OFFS));

    // wait for APMU to finish
    while readl_relaxed(p.add(WUPCR_OFFS)) != 0 {}
    0
}

unsafe fn apmu_power_off(p: *mut core::ffi::c_void, bit: i32) -> i32 {
    // request Core Standby for next WFI
    writel_relaxed(3, p.add(cpuncr_offs(bit)));
    0
}

unsafe fn apmu_power_off_poll(p: *mut core::ffi::c_void, bit: i32) -> i32 {
    for _k in 0..1000 {
        if cpunst(readl_relaxed(p.add(PSTR_OFFS)), bit) == CPUST_STANDBY { return 1; }
        mdelay(1);
    }
    0
}

unsafe fn apmu_wrap(cpu: i32, f: unsafe fn(*mut core::ffi::c_void, i32) -> i32) -> i32 {
    let entry = &APMU_CPUS[cpu as usize];
    if !entry.iomem.is_null() { f(entry.iomem, entry.bit) } else { -EINVAL }
}

#[cfg(any(CONFIG_HOTPLUG_CPU, CONFIG_SUSPEND))]
#[inline]
unsafe fn cpu_enter_lowpower_a15() {
    let mut v: u32;
    core::arch::asm!("mrc p15, 0, {0}, c1, c0, 0", "bic {0}, {0}, {1}", "mcr p15, 0, {0}, c1, c0, 0", out(reg) v, const CR_C);
    flush_cache_louis();
    core::arch::asm!("mrc p15, 0, {0}, c1, c0, 1", "bic {0}, {0}, {1}", "mcr p15, 0, {0}, c1, c0, 1", out(reg) v, const 0x40u32);
    isb();
    dsb();
}

#[cfg(any(CONFIG_HOTPLUG_CPU, CONFIG_SUSPEND))]
unsafe fn shmobile_smp_apmu_cpu_shutdown(cpu: u32) {
    apmu_wrap(cpu as i32, apmu_power_off);
    cpu_enter_lowpower_a15();
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn shmobile_smp_apmu_cpu_die(cpu: u32) {
    shmobile_smp_hook(cpu, 0, 0);
    shmobile_smp_apmu_cpu_shutdown(cpu);
    shmobile_smp_sleep();
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn shmobile_smp_apmu_cpu_kill(cpu: u32) -> i32 {
    apmu_wrap(cpu as i32, apmu_power_off_poll)
}

#[cfg(CONFIG_SUSPEND)]
unsafe fn shmobile_smp_apmu_do_suspend(cpu: usize) -> i32 {
    shmobile_smp_hook(cpu as u32, __pa_symbol(cpu_resume), 0);
    shmobile_smp_apmu_cpu_shutdown(cpu as u32);
    cpu_do_idle(); // WFI selects Core Standby
    1
}

#[cfg(CONFIG_SUSPEND)]
#[inline]
unsafe fn cpu_leave_lowpower() {
    let mut v: u32;
    core::arch::asm!("mrc p15, 0, {0}, c1, c0, 0", "orr {0}, {0}, {1}", "mcr p15, 0, {0}, c1, c0, 0", "mrc p15, 0, {0}, c1, c0, 1", "orr {0}, {0}, {2}", "mcr p15, 0, {0}, c1, c0, 1", out(reg) v, const CR_C, const 0x40u32);
}

#[cfg(CONFIG_SUSPEND)]
unsafe fn shmobile_smp_apmu_enter_suspend(_state: suspend_state_t) -> i32 {
    cpu_suspend(smp_processor_id(), shmobile_smp_apmu_do_suspend);
    cpu_leave_lowpower();
    0
}

#[cfg(CONFIG_SUSPEND)]
unsafe fn shmobile_smp_apmu_suspend_init() {
    shmobile_suspend_ops.enter = Some(shmobile_smp_apmu_enter_suspend);
}

#[cfg(CONFIG_SMP)]
unsafe fn apmu_init_cpu(res: *mut resource, cpu: i32, bit: i32) {
    if cpu >= NR_CPUS as i32 || !APMU_CPUS[cpu as usize].iomem.is_null() { return; }
    APMU_CPUS[cpu as usize].iomem = ioremap((*res).start, resource_size(res));
    APMU_CPUS[cpu as usize].bit = bit;
    pr_debug!("apmu ioremap {} {} %pr\n", cpu, bit, res);
    let mut x = readl(APMU_CPUS[cpu as usize].iomem.add(DBGRCR_OFFS));
    x |= DBGCPUREN | dbgcpunren(bit) | DBGCPUPREN;
    writel(x, APMU_CPUS[cpu as usize].iomem.add(DBGRCR_OFFS));
}

#[cfg(CONFIG_SMP)]
static APMU_IDS: &[&str] = &["renesas,apmu", ""]; // sentinel

#[cfg(CONFIG_SMP)]
unsafe fn apmu_parse_dt(f: unsafe fn(*mut resource, i32, i32)) {
    // for_each_matching_node(np_apmu, apmu_ids)
    // Device-tree iteration and phandle handling are supplied by the kernel bindings.
    todo!("translate kernel device-tree iteration")
}

#[cfg(CONFIG_SMP)]
unsafe fn shmobile_smp_apmu_setup_boot() {
    shmobile_boot_fn = __pa_symbol(shmobile_smp_boot);
    shmobile_boot_fn_gen2 = shmobile_boot_fn;
}

#[cfg(CONFIG_SMP)]
unsafe fn shmobile_smp_apmu_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    shmobile_smp_hook(cpu, __pa_symbol(shmobile_boot_apmu), 0);
    apmu_wrap(cpu as i32, apmu_power_on)
}

#[cfg(CONFIG_SMP)]
unsafe fn shmobile_smp_apmu_prepare_cpus_dt(_max_cpus: u32) {
    shmobile_smp_apmu_setup_boot();
    apmu_parse_dt(apmu_init_cpu);
    rcar_gen2_pm_init();
}

// struct smp_operations apmu_smp_ops and CPU_METHOD_OF_DECLARE are kernel registration items.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
