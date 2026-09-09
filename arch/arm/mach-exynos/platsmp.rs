// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2010-2011 Samsung Electronics Co., Ltd.
//		http://www.samsung.com
//
// Cloned from linux/arch/arm/mach-vexpress/platsmp.c
//
//  Copyright (C) 2002 ARM Ltd.
//  All Rights Reserved

// C headers and build-time configuration are supplied by the surrounding kernel translation.

extern "C" {
    fn exynos4_secondary_startup();
}

/* XXX exynos_pen_release is cargo culted code - DO NOT COPY XXX */
pub static mut exynos_pen_release: i32 = -1;

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
#[inline]
unsafe fn cpu_leave_lowpower(_core_id: u32) {
    // ARM coprocessor assembly from the original source; preserve its intent here.
    core::arch::asm!(
        "mrc p15, 0, {v}, c1, c0, 0",
        "orr {v}, {v}, {c}",
        "mcr p15, 0, {v}, c1, c0, 0",
        "mrc p15, 0, {v}, c1, c0, 1",
        "orr {v}, {v}, {imm}",
        "mcr p15, 0, {v}, c1, c0, 1",
        v = out(reg) _, c = in(reg) CR_C, imm = in(reg) 0x40u32,
        options(nostack)
    );
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
#[inline]
unsafe fn platform_do_lowpower(cpu: u32, spurious: *mut i32) {
    let mpidr = cpu_logical_map(cpu);
    let core_id = MPIDR_AFFINITY_LEVEL(mpidr, 0);
    loop {
        /* Turn the CPU off on next WFI instruction. */
        exynos_cpu_power_down(core_id as i32);
        wfi();
        if exynos_pen_release == core_id as i32 {
            /* OK, proper wakeup, we're done */
            break;
        }
        /* Getting here means that we have come out of WFI without having been woken up. */
        *spurious += 1;
    }
}

pub unsafe fn exynos_cpu_power_down(cpu: i32) {
    let mut core_conf: u32;
    if cpu == 0 && (soc_is_exynos5420() || soc_is_exynos5800()) {
        let val = pmu_raw_readl(EXYNOS5_ARM_CORE0_SYS_PWR_REG);
        if (val & S5P_CORE_LOCAL_PWR_EN) == 0 { return; }
    }
    core_conf = pmu_raw_readl(EXYNOS_ARM_CORE_CONFIGURATION(cpu));
    core_conf &= !S5P_CORE_LOCAL_PWR_EN;
    pmu_raw_writel(core_conf, EXYNOS_ARM_CORE_CONFIGURATION(cpu));
}

pub unsafe fn exynos_cpu_power_up(cpu: i32) {
    let mut core_conf = S5P_CORE_LOCAL_PWR_EN;
    if soc_is_exynos3250() { core_conf |= S5P_CORE_AUTOWAKEUP_EN; }
    pmu_raw_writel(core_conf, EXYNOS_ARM_CORE_CONFIGURATION(cpu));
}

pub unsafe fn exynos_cpu_power_state(cpu: i32) -> i32 {
    (pmu_raw_readl(EXYNOS_ARM_CORE_STATUS(cpu)) & S5P_CORE_LOCAL_PWR_EN) as i32
}

pub unsafe fn exynos_cluster_power_down(cluster: i32) {
    pmu_raw_writel(0, EXYNOS_COMMON_CONFIGURATION(cluster));
}

pub unsafe fn exynos_cluster_power_up(cluster: i32) {
    pmu_raw_writel(S5P_CORE_LOCAL_PWR_EN, EXYNOS_COMMON_CONFIGURATION(cluster));
}

pub unsafe fn exynos_cluster_power_state(cluster: i32) -> i32 {
    (pmu_raw_readl(EXYNOS_COMMON_STATUS(cluster)) & S5P_CORE_LOCAL_PWR_EN) as i32
}

pub unsafe fn exynos_scu_enable() {
    static mut scu_base: *mut core::ffi::c_void = core::ptr::null_mut();
    if scu_base.is_null() {
        let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "arm,cortex-a9-scu\0".as_ptr() as *const i8);
        if !np.is_null() {
            scu_base = of_iomap(np, 0);
            of_node_put(np);
        } else {
            scu_base = ioremap(scu_a9_get_base(), SZ_4K);
        }
    }
    scu_enable(scu_base);
}

unsafe fn cpu_boot_reg_base() -> *mut core::ffi::c_void {
    if soc_is_exynos4210() && exynos_rev() == EXYNOS4210_REV_1_1 { return pmu_base_addr.add(S5P_INFORM5 as usize); }
    sysram_base_addr
}

unsafe fn cpu_boot_reg(cpu: i32) -> *mut core::ffi::c_void {
    let mut boot_reg = cpu_boot_reg_base();
    if boot_reg.is_null() { return IOMEM_ERR_PTR(-ENODEV); }
    if soc_is_exynos4412() { boot_reg = boot_reg.add((4 * cpu) as usize); }
    else if soc_is_exynos5420() || soc_is_exynos5800() { boot_reg = boot_reg.add(4); }
    boot_reg
}

pub unsafe fn exynos_core_restart(core_id: u32) {
    let mut timeout = 16;
    let mut val: u32;
    if !soc_is_exynos3250() { return; }
    while timeout != 0 && pmu_raw_readl(S5P_PMU_SPARE2) == 0 { timeout -= 1; udelay(10); }
    if timeout == 0 { pr_err("cpu core %u restart failed\0".as_ptr() as *const i8, core_id); return; }
    udelay(10);
    val = pmu_raw_readl(EXYNOS_ARM_CORE_STATUS(core_id as i32));
    val |= S5P_CORE_WAKEUP_FROM_LOCAL_CFG;
    pmu_raw_writel(val, EXYNOS_ARM_CORE_STATUS(core_id as i32));
    pmu_raw_writel(EXYNOS_CORE_PO_RESET(core_id as i32), EXYNOS_SWRESET);
}

unsafe fn exynos_write_pen_release(val: i32) {
    exynos_pen_release = val;
    smp_wmb();
    sync_cache_w(&raw mut exynos_pen_release);
}

static mut boot_lock: DEFINE_SPINLOCK_TYPE = DEFINE_SPINLOCK!();

unsafe fn exynos_secondary_init(_cpu: u32) {
    exynos_write_pen_release(-1);
    spin_lock(&raw mut boot_lock);
    spin_unlock(&raw mut boot_lock);
}

pub unsafe fn exynos_set_boot_addr(core_id: u32, boot_addr: usize) -> i32 {
    let mut ret = call_firmware_op(set_cpu_boot_addr, core_id, boot_addr);
    if ret != 0 && ret != -ENOSYS { return ret; }
    if ret == -ENOSYS {
        let boot_reg = cpu_boot_reg(core_id as i32);
        if IS_ERR(boot_reg) { return PTR_ERR(boot_reg); }
        writel_relaxed(boot_addr as u32, boot_reg);
        ret = 0;
    }
    ret
}

pub unsafe fn exynos_get_boot_addr(core_id: u32, boot_addr: *mut usize) -> i32 {
    let mut ret = call_firmware_op(get_cpu_boot_addr, core_id, boot_addr);
    if ret != 0 && ret != -ENOSYS { return ret; }
    if ret == -ENOSYS {
        let boot_reg = cpu_boot_reg(core_id as i32);
        if IS_ERR(boot_reg) { return PTR_ERR(boot_reg); }
        *boot_addr = readl_relaxed(boot_reg) as usize;
        ret = 0;
    }
    ret
}

unsafe fn exynos_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    let mut timeout: usize;
    let mpidr = cpu_logical_map(cpu);
    let core_id = MPIDR_AFFINITY_LEVEL(mpidr, 0);
    let mut ret = -ENOSYS;
    spin_lock(&raw mut boot_lock);
    exynos_write_pen_release(core_id as i32);
    if exynos_cpu_power_state(core_id as i32) == 0 {
        exynos_cpu_power_up(core_id as i32);
        timeout = 10;
        while exynos_cpu_power_state(core_id as i32) != S5P_CORE_LOCAL_PWR_EN as i32 {
            if timeout == 0 { break; }
            timeout -= 1;
            mdelay(1);
        }
        if timeout == 0 {
            printk(KERN_ERR, "cpu1 power enable failed\0".as_ptr() as *const i8);
            spin_unlock(&raw mut boot_lock);
            return -ETIMEDOUT;
        }
    }
    exynos_core_restart(core_id);
    timeout = jiffies() + HZ;
    while time_before(jiffies(), timeout) {
        smp_rmb();
        let boot_addr = __pa_symbol(exynos4_secondary_startup);
        ret = exynos_set_boot_addr(core_id, boot_addr);
        if ret != 0 { break; }
        call_firmware_op(cpu_boot, core_id);
        if soc_is_exynos3250() { dsb_sev(); }
        else { arch_send_wakeup_ipi_mask(cpumask_of(cpu)); }
        if exynos_pen_release == -1 { break; }
        udelay(10);
    }
    if exynos_pen_release != -1 { ret = -ETIMEDOUT; }
    spin_unlock(&raw mut boot_lock);
    if exynos_pen_release != -1 { ret } else { 0 }
}

unsafe fn exynos_smp_prepare_cpus(_max_cpus: u32) {
    exynos_sysram_init();
    exynos_set_delayed_reset_assertion(true);
    if read_cpuid_part() == ARM_CPU_PART_CORTEX_A9 { exynos_scu_enable(); }
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn exynos_cpu_die(cpu: u32) {
    let mut spurious = 0;
    let mpidr = cpu_logical_map(cpu);
    let core_id = MPIDR_AFFINITY_LEVEL(mpidr, 0);
    v7_exit_coherency_flush(louis);
    platform_do_lowpower(cpu, &mut spurious);
    cpu_leave_lowpower(core_id);
    if spurious != 0 { pr_warn("CPU%u: %u spurious wakeup calls\n\0".as_ptr() as *const i8, cpu, spurious); }
}

// Equivalent to the C smp_operations initializer; its concrete type is supplied by the kernel bindings.
pub static exynos_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(exynos_smp_prepare_cpus),
    smp_secondary_init: Some(exynos_secondary_init),
    smp_boot_secondary: Some(exynos_boot_secondary),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_die: Some(exynos_cpu_die),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
