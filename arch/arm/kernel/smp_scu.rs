// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/smp_scu.c
 *
 *  Copyright (C) 2002 ARM Ltd.
 *  All Rights Reserved
 */

// C headers omitted; their symbols are supplied by the surrounding kernel.

const SCU_CTRL: usize = 0x00;
const SCU_ENABLE: u32 = 1 << 0;
const SCU_STANDBY_ENABLE: u32 = 1 << 5;
const SCU_CONFIG: usize = 0x04;
const SCU_CPU_STATUS: usize = 0x08;
const SCU_CPU_STATUS_MASK: u8 = (1 << 1) | (1 << 0);
const SCU_INVALIDATE: usize = 0x0c;
const SCU_FPGA_REVISION: usize = 0x10;

extern "C" {
    fn readl_relaxed(addr: *const u8) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn readb_relaxed(addr: *const u8) -> u8;
    fn writeb_relaxed(value: u8, addr: *mut u8);
    fn read_cpuid_id() -> u32;
    fn flush_cache_all();
    fn cpu_logical_map(cpu: u32) -> u32;
    fn smp_processor_id() -> u32;
    fn MPIDR_AFFINITY_LEVEL(mpidr: u32, level: u32) -> i32;
}

// CONFIG_SMP
/*
 * Get the number of CPU cores from the SCU configuration
 */
pub unsafe fn scu_get_core_count(scu_base: *mut u8) -> u32 {
    let ncores = readl_relaxed(scu_base.add(SCU_CONFIG));
    (ncores & 0x03) + 1
}

/*
 * Enable the SCU
 */
pub unsafe fn scu_enable(scu_base: *mut u8) {
    let mut scu_ctrl: u32;

    // CONFIG_ARM_ERRATA_764369
    // Cortex-A9 only
    if (read_cpuid_id() & 0xff0ffff0) == 0x410fc090 {
        scu_ctrl = readl_relaxed(scu_base.add(0x30));
        if (scu_ctrl & 1) == 0 {
            writel_relaxed(scu_ctrl | 0x1, scu_base.add(0x30));
        }
    }

    scu_ctrl = readl_relaxed(scu_base.add(SCU_CTRL));
    /* already enabled? */
    if (scu_ctrl & SCU_ENABLE) != 0 {
        return;
    }

    scu_ctrl |= SCU_ENABLE;

    /* Cortex-A9 earlier than r2p0 has no standby bit in SCU */
    if (read_cpuid_id() & 0xff0ffff0) == 0x410fc090
        && (read_cpuid_id() & 0x00f0000f) >= 0x00200000
    {
        scu_ctrl |= SCU_STANDBY_ENABLE;
    }

    writel_relaxed(scu_ctrl, scu_base.add(SCU_CTRL));

    /*
     * Ensure that the data accessed by CPU0 before the SCU was
     * initialised is visible to the other CPUs.
     */
    flush_cache_all();
}

unsafe fn scu_set_power_mode_internal(
    scu_base: *mut u8,
    logical_cpu: u32,
    mode: u32,
) -> i32 {
    let mut val: u8;
    let cpu = MPIDR_AFFINITY_LEVEL(cpu_logical_map(logical_cpu), 0);

    if mode > 3 || mode == 1 || cpu > 3 {
        return -EINVAL;
    }

    val = readb_relaxed(scu_base.add(SCU_CPU_STATUS + cpu as usize));
    val &= !SCU_CPU_STATUS_MASK;
    val |= mode as u8;
    writeb_relaxed(val, scu_base.add(SCU_CPU_STATUS + cpu as usize));

    0
}

/*
 * Set the executing CPUs power mode as defined.  This will be in
 * preparation for it executing a WFI instruction.
 *
 * This function must be called with preemption disabled, and as it
 * has the side effect of disabling coherency, caches must have been
 * flushed.  Interrupts must also have been disabled.
 */
pub unsafe fn scu_power_mode(scu_base: *mut u8, mode: u32) -> i32 {
    scu_set_power_mode_internal(scu_base, smp_processor_id(), mode)
}

/*
 * Set the given (logical) CPU's power mode to SCU_PM_NORMAL.
 */
pub unsafe fn scu_cpu_power_enable(scu_base: *mut u8, cpu: u32) -> i32 {
    scu_set_power_mode_internal(scu_base, cpu, SCU_PM_NORMAL)
}

pub unsafe fn scu_get_cpu_power_mode(scu_base: *mut u8, logical_cpu: u32) -> i32 {
    let mut val: u8;
    let cpu = MPIDR_AFFINITY_LEVEL(cpu_logical_map(logical_cpu), 0);

    if cpu > 3 {
        return -EINVAL;
    }

    val = readb_relaxed(scu_base.add(SCU_CPU_STATUS + cpu as usize));
    val &= SCU_CPU_STATUS_MASK;

    val as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
