// SPDX-License-Identifier: GPL-2.0-only
/*
 * Symmetric Multi Processing (SMP) support for Marvell EBU Cortex-A9
 * based SOCs (Armada 375/38x).
 *
 * Copyright (C) 2014 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 */

// Linux and architecture headers, plus "common.h" and "pmsu.h", provide the
// declarations referenced below.

extern "C" {
    fn mvebu_cortex_a9_secondary_startup();
}

unsafe fn mvebu_cortex_a9_boot_secondary(
    cpu: core::ffi::c_uint,
    _idle: *mut task_struct,
) -> core::ffi::c_int {
    let ret: core::ffi::c_int;
    let hw_cpu: core::ffi::c_int;

    pr_info!("Booting CPU %d\n", cpu);

    /*
     * Write the address of secondary startup into the system-wide
     * flags register. The boot monitor waits until it receives a
     * soft interrupt, and then the secondary CPU branches to this
     * address.
     */
    hw_cpu = cpu_logical_map(cpu) as core::ffi::c_int;
    if of_machine_is_compatible(c"marvell,armada375".as_ptr() as *const core::ffi::c_char) {
        mvebu_system_controller_set_cpu_boot_addr(mvebu_cortex_a9_secondary_startup);
    } else {
        mvebu_pmsu_set_cpu_boot_addr(hw_cpu, mvebu_cortex_a9_secondary_startup);
    }
    smp_wmb();

    /*
     * Doing this before deasserting the CPUs is needed to wake up CPUs
     * in the offline state after using CPU hotplug.
     */
    arch_send_wakeup_ipi_mask(cpumask_of(cpu));

    ret = mvebu_cpu_reset_deassert(hw_cpu);
    if ret != 0 {
        pr_err!("Could not start the secondary CPU: %d\n", ret);
        return ret;
    }

    0
}

/*
 * When a CPU is brought back online, either through CPU hotplug, or
 * because of the boot of a kexec'ed kernel, the PMSU configuration
 * for this CPU might be in the deep idle state, preventing this CPU
 * from receiving interrupts. Here, we therefore take out the current
 * CPU from this state, which was entered by armada_38x_cpu_die()
 * below.
 */
unsafe fn armada_38x_secondary_init(_cpu: core::ffi::c_uint) {
    mvebu_v7_pmsu_idle_exit();
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn armada_38x_cpu_die(_cpu: core::ffi::c_uint) {
    /*
     * CPU hotplug is implemented by putting offline CPUs into the
     * deep idle sleep state.
     */
    armada_38x_do_cpu_suspend(true);
}

/*
 * We need a dummy function, so that platform_can_cpu_hotplug() knows
 * we support CPU hotplug. However, the function does not need to do
 * anything, because CPUs going offline can enter the deep idle state
 * by themselves, without any help from a still alive CPU.
 */
#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn armada_38x_cpu_kill(_cpu: core::ffi::c_uint) -> core::ffi::c_int {
    1
}

// `struct task_struct`, `struct smp_operations`, and the kernel functions
// below are supplied by the surrounding kernel translation.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    fn cpu_logical_map(cpu: core::ffi::c_uint) -> core::ffi::c_uint;
    fn of_machine_is_compatible(compat: *const core::ffi::c_char) -> bool;
    fn mvebu_system_controller_set_cpu_boot_addr(entry: unsafe extern "C" fn());
    fn mvebu_pmsu_set_cpu_boot_addr(cpu: core::ffi::c_int, entry: unsafe extern "C" fn());
    fn smp_wmb();
    fn cpumask_of(cpu: core::ffi::c_uint) -> *const core::ffi::c_void;
    fn arch_send_wakeup_ipi_mask(mask: *const core::ffi::c_void);
    fn mvebu_cpu_reset_deassert(cpu: core::ffi::c_int) -> core::ffi::c_int;
    fn mvebu_v7_pmsu_idle_exit();
    fn armada_38x_do_cpu_suspend(power_down: bool);
}

#[repr(C)]
pub struct smp_operations {
    pub smp_boot_secondary:
        Option<unsafe fn(core::ffi::c_uint, *mut task_struct) -> core::ffi::c_int>,
    pub smp_secondary_init: Option<unsafe fn(core::ffi::c_uint)>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_die: Option<unsafe fn(core::ffi::c_uint)>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_kill: Option<unsafe fn(core::ffi::c_uint) -> core::ffi::c_int>,
}

static MVEBU_CORTEX_A9_SMP_OPS: smp_operations = smp_operations {
    smp_boot_secondary: Some(mvebu_cortex_a9_boot_secondary),
    smp_secondary_init: None,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_die: None,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_kill: None,
};

static ARMADA_38X_SMP_OPS: smp_operations = smp_operations {
    smp_boot_secondary: Some(mvebu_cortex_a9_boot_secondary),
    smp_secondary_init: Some(armada_38x_secondary_init),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_die: Some(armada_38x_cpu_die),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_kill: Some(armada_38x_cpu_kill),
};

// CPU_METHOD_OF_DECLARE(mvebu_armada_375_smp, "marvell,armada-375-smp",
//                       &mvebu_cortex_a9_smp_ops);
// CPU_METHOD_OF_DECLARE(mvebu_armada_380_smp, "marvell,armada-380-smp",
//                       &armada_38x_smp_ops);
// CPU_METHOD_OF_DECLARE(mvebu_armada_390_smp, "marvell,armada-390-smp",
//                       &armada_38x_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
