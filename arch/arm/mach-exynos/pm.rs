// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2011-2014 Samsung Electronics Co., Ltd.
// Exynos - Power Management support
// Based on arch/arm/mach-s3c2410/pm.c

// Linux and architecture dependencies supplied by the surrounding kernel.

unsafe fn exynos_boot_vector_addr() -> *mut core::ffi::c_void {
    if exynos_rev() == EXYNOS4210_REV_1_1 {
        pmu_base_addr.add(S5P_INFORM7 as usize)
    } else if exynos_rev() == EXYNOS4210_REV_1_0 {
        sysram_base_addr.add(0x24)
    } else {
        pmu_base_addr.add(S5P_INFORM0 as usize)
    }
}

unsafe fn exynos_boot_vector_flag() -> *mut core::ffi::c_void {
    if exynos_rev() == EXYNOS4210_REV_1_1 {
        pmu_base_addr.add(S5P_INFORM6 as usize)
    } else if exynos_rev() == EXYNOS4210_REV_1_0 {
        sysram_base_addr.add(0x20)
    } else {
        pmu_base_addr.add(S5P_INFORM1 as usize)
    }
}

const S5P_CHECK_AFTR: u32 = 0xFCBA0D10;

/* For Cortex-A9 Diagnostic and Power control register */
static mut save_arm_register: [u32; 2] = [0; 2];

pub unsafe fn exynos_cpu_save_register() {
    let mut tmp: u32;
    // ARM coprocessor register access: mrc p15, 0, tmp, c15, c0, 0
    core::arch::asm!("mrc p15, 0, {0}, c15, c0, 0", out(reg) tmp, options(nomem, nostack));
    save_arm_register[0] = tmp;
    // ARM coprocessor register access: mrc p15, 0, tmp, c15, c0, 1
    core::arch::asm!("mrc p15, 0, {0}, c15, c0, 1", out(reg) tmp, options(nomem, nostack));
    save_arm_register[1] = tmp;
}

pub unsafe fn exynos_cpu_restore_register() {
    let mut tmp = save_arm_register[0];
    core::arch::asm!("mcr p15, 0, {0}, c15, c0, 0", in(reg) tmp, options(nomem, nostack));
    tmp = save_arm_register[1];
    core::arch::asm!("mcr p15, 0, {0}, c15, c0, 1", in(reg) tmp, options(nomem, nostack));
}

pub unsafe fn exynos_pm_central_suspend() {
    let mut tmp = pmu_raw_readl(S5P_CENTRAL_SEQ_CONFIGURATION);
    tmp &= !S5P_CENTRAL_LOWPWR_CFG;
    pmu_raw_writel(tmp, S5P_CENTRAL_SEQ_CONFIGURATION);
}

pub unsafe fn exynos_pm_central_resume() -> i32 {
    let mut tmp = pmu_raw_readl(S5P_CENTRAL_SEQ_CONFIGURATION);
    if tmp & S5P_CENTRAL_LOWPWR_CFG == 0 {
        tmp |= S5P_CENTRAL_LOWPWR_CFG;
        pmu_raw_writel(tmp, S5P_CENTRAL_SEQ_CONFIGURATION);
        pmu_raw_writel(0, S5P_WAKEUP_STAT);
        return -1;
    }
    0
}

/* Ext-GIC nIRQ/nFIQ is the only wakeup source in AFTR */
unsafe fn exynos_set_wakeupmask(mask: i64) {
    pmu_raw_writel(mask, S5P_WAKEUP_MASK);
    if soc_is_exynos3250() { pmu_raw_writel(0, S5P_WAKEUP_MASK2); }
}

unsafe fn exynos_cpu_set_boot_vector(flags: i64) {
    writel_relaxed(__pa_symbol(exynos_cpu_resume), exynos_boot_vector_addr());
    writel_relaxed(flags, exynos_boot_vector_flag());
}

unsafe fn exynos_aftr_finisher(_flags: usize) -> i32 {
    exynos_set_wakeupmask(if soc_is_exynos3250() { 0x40003ffe } else { 0x0000ff3e });
    exynos_sys_powerdown_conf(SYS_AFTR);
    let ret = call_firmware_op(do_idle, FW_DO_IDLE_AFTR);
    if ret == -ENOSYS {
        if read_cpuid_part() == ARM_CPU_PART_CORTEX_A9 { exynos_cpu_save_register(); }
        exynos_cpu_set_boot_vector(S5P_CHECK_AFTR as i64);
        cpu_do_idle();
    }
    1
}

pub unsafe fn exynos_enter_aftr() {
    let cpuid = smp_processor_id();
    cpu_pm_enter();
    if soc_is_exynos3250() { exynos_set_boot_flag(cpuid, C2_STATE); }
    exynos_pm_central_suspend();
    if soc_is_exynos4212() || soc_is_exynos4412() {
        pmu_raw_writel(S5P_USE_STANDBY_WFI0 | S5P_USE_STANDBY_WFE0, S5P_CENTRAL_SEQ_OPTION);
    }
    cpu_suspend(0, exynos_aftr_finisher);
    if read_cpuid_part() == ARM_CPU_PART_CORTEX_A9 {
        exynos_scu_enable();
        if call_firmware_op(resume) == -ENOSYS { exynos_cpu_restore_register(); }
    }
    exynos_pm_central_resume();
    if soc_is_exynos3250() { exynos_clear_boot_flag(cpuid, C2_STATE); }
    cpu_pm_exit();
}

#[cfg(all(feature = "SMP", feature = "ARM_EXYNOS_CPUIDLE"))]
static mut cpu1_wakeup: atomic_t = ATOMIC_INIT(0);

#[cfg(all(feature = "SMP", feature = "ARM_EXYNOS_CPUIDLE"))]
unsafe fn exynos_cpu0_enter_aftr() -> i32 {
    let mut ret = -1;
    if cpu_online(1) {
        while exynos_cpu_power_state(1) != 0 {
            let mut boot_addr = 0;
            if atomic_read(&cpu1_wakeup) != 0 { break; }
            ret = exynos_get_boot_addr(1, &mut boot_addr);
            if ret != 0 { return ret; }
            ret = -1;
            if boot_addr == 0 { break; }
            cpu_relax();
        }
    }
    exynos_enter_aftr();
    ret = 0;
    if cpu_online(1) {
        let boot_addr = __pa_symbol(exynos_cpu_resume);
        ret = exynos_set_boot_addr(1, boot_addr);
        if ret != 0 { return ret; }
        dsb();
        exynos_cpu_power_up(1);
        while exynos_cpu_power_state(1) != S5P_CORE_LOCAL_PWR_EN { cpu_relax(); }
        if soc_is_exynos3250() {
            while pmu_raw_readl(S5P_PMU_SPARE2) == 0 && atomic_read(&cpu1_wakeup) == 0 { cpu_relax(); }
            if atomic_read(&cpu1_wakeup) == 0 { exynos_core_restart(1); }
        }
        while atomic_read(&cpu1_wakeup) == 0 {
            smp_rmb();
            ret = exynos_set_boot_addr(1, boot_addr);
            if ret != 0 { return ret; }
            call_firmware_op(cpu_boot, 1);
            dsb_sev();
        }
    }
    ret
}

#[cfg(all(feature = "SMP", feature = "ARM_EXYNOS_CPUIDLE"))]
unsafe fn exynos_wfi_finisher(_flags: usize) -> i32 {
    if soc_is_exynos3250() { flush_cache_all(); }
    cpu_do_idle();
    -1
}

#[cfg(all(feature = "SMP", feature = "ARM_EXYNOS_CPUIDLE"))]
unsafe fn exynos_cpu1_powerdown() -> i32 {
    let mut ret = -1;
    if cpu_pm_enter() != 0 {
        dsb();
        atomic_set(&mut cpu1_wakeup, 1);
        return ret;
    }
    exynos_cpu_power_down(1);
    if soc_is_exynos3250() { pmu_raw_writel(0, S5P_PMU_SPARE2); }
    ret = cpu_suspend(0, exynos_wfi_finisher);
    cpu_pm_exit();
    dsb();
    atomic_set(&mut cpu1_wakeup, 1);
    ret
}

#[cfg(all(feature = "SMP", feature = "ARM_EXYNOS_CPUIDLE"))]
unsafe fn exynos_pre_enter_aftr() { let boot_addr = __pa_symbol(exynos_cpu_resume); let _ = exynos_set_boot_addr(1, boot_addr); }

#[cfg(all(feature = "SMP", feature = "ARM_EXYNOS_CPUIDLE"))]
unsafe fn exynos_post_enter_aftr() { atomic_set(&mut cpu1_wakeup, 0); }

#[cfg(all(feature = "SMP", feature = "ARM_EXYNOS_CPUIDLE"))]
pub static mut cpuidle_coupled_exynos_data: cpuidle_exynos_data = cpuidle_exynos_data {
    cpu0_enter_aftr: exynos_cpu0_enter_aftr,
    cpu1_powerdown: exynos_cpu1_powerdown,
    pre_enter_aftr: exynos_pre_enter_aftr,
    post_enter_aftr: exynos_post_enter_aftr,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
