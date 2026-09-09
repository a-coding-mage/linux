// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright: (C) 2018 Socionext Inc.
 * Copyright: (C) 2015 Linaro Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

const M10V_MAX_CPU: u32 = 4;
const KERNEL_UNBOOT_FLAG: u32 = 0x12345678;

static mut M10V_SMP_BASE: *mut core::ffi::c_void = core::ptr::null_mut();

extern "C" {
    static mut louis: u32;
    static mut phys_reset: phys_reset_t;

    fn cpu_logical_map(cpu: u32) -> u32;
    fn secondary_startup();
    fn __pa_symbol(value: unsafe extern "C" fn());
    fn writel(value: u32, address: *mut core::ffi::c_void);
    fn arch_send_wakeup_ipi_mask(mask: *const core::ffi::c_void);
    fn cpumask_of(cpu: u32) -> *const core::ffi::c_void;
    fn read_cpuid_mpidr() -> u32;
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const core::ffi::c_char,
        compatible: *const core::ffi::c_char,
    ) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn gic_cpu_if_down(cpu: u32);
    fn v7_exit_coherency_flush(level: u32);
    fn wfi();
    fn setup_mm_for_reboot();
    fn virt_to_phys(value: unsafe extern "C" fn()) -> u32;
    fn cpu_reset();
    fn cpu_resume();
    fn cpu_pm_enter();
    fn cpu_suspend(arg: u32, fn_: unsafe extern "C" fn(u32) -> i32) -> i32;
    fn cpu_pm_exit();
    fn suspend_set_ops(ops: *const platform_suspend_ops);
    fn of_machine_is_compatible(compatible: *const core::ffi::c_char) -> bool;
    fn pr_info(format: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smp_operations {
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(u32)>,
    pub smp_boot_secondary:
        Option<unsafe extern "C" fn(u32, *mut task_struct) -> i32>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_die: Option<unsafe extern "C" fn(u32)>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_kill: Option<unsafe extern "C" fn(u32) -> i32>,
}

pub type suspend_state_t = i32;
#[repr(C)]
pub struct platform_suspend_ops {
    pub valid: Option<unsafe extern "C" fn(suspend_state_t) -> bool>,
    pub enter: Option<unsafe extern "C" fn(suspend_state_t) -> i32>,
}

pub type phys_reset_t = unsafe extern "C" fn(u32);

const PM_SUSPEND_STANDBY: suspend_state_t = 1;
const PM_SUSPEND_MEM: suspend_state_t = 3;

#[inline]
fn mpidr_affinity_level(mpidr: u32, level: u32) -> u32 {
    (mpidr >> (level * 8)) & 0xff
}

unsafe extern "C" fn m10v_boot_secondary(
    l_cpu: u32,
    _idle: *mut task_struct,
) -> i32 {
    let (mpidr, cpu, cluster);

    if M10V_SMP_BASE.is_null() {
        return -6; // -ENXIO
    }

    mpidr = cpu_logical_map(l_cpu);
    cpu = mpidr_affinity_level(mpidr, 0);
    cluster = mpidr_affinity_level(mpidr, 1);

    if cpu >= M10V_MAX_CPU {
        return -22; // -EINVAL
    }

    pr_info(
        b"%s: cpu %u l_cpu %u cluster %u\0".as_ptr() as *const _,
        b"m10v_boot_secondary\0".as_ptr(), cpu, l_cpu, cluster,
    );

    writel(__pa_symbol(secondary_startup) as u32, M10V_SMP_BASE.add((cpu * 4) as usize));
    arch_send_wakeup_ipi_mask(cpumask_of(l_cpu));
    0
}

unsafe extern "C" fn m10v_smp_init(_max_cpus: u32) {
    let np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"socionext,milbeaut-smp-sram\0".as_ptr() as *const _,
    );
    if np.is_null() {
        return;
    }

    M10V_SMP_BASE = of_iomap(np, 0);
    if M10V_SMP_BASE.is_null() {
        return;
    }

    let mpidr = read_cpuid_mpidr();
    let cpu = mpidr_affinity_level(mpidr, 0);
    let cluster = mpidr_affinity_level(mpidr, 1);
    pr_info(b"MCPM boot on cpu_%u cluster_%u\0".as_ptr() as *const _, cpu, cluster);

    for cpu in 0..M10V_MAX_CPU {
        writel(KERNEL_UNBOOT_FLAG, M10V_SMP_BASE.add((cpu * 4) as usize));
    }
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe extern "C" fn m10v_cpu_die(_l_cpu: u32) {
    gic_cpu_if_down(0);
    v7_exit_coherency_flush(louis);
    wfi();
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe extern "C" fn m10v_cpu_kill(l_cpu: u32) -> i32 {
    let mpidr = cpu_logical_map(l_cpu);
    let cpu = mpidr_affinity_level(mpidr, 0);
    writel(KERNEL_UNBOOT_FLAG, M10V_SMP_BASE.add((cpu * 4) as usize));
    1
}

#[used]
static mut M10V_SMP_OPS: smp_operations = smp_operations {
    smp_prepare_cpus: Some(m10v_smp_init),
    smp_boot_secondary: Some(m10v_boot_secondary),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_die: Some(m10v_cpu_die),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_kill: Some(m10v_cpu_kill),
};

// CPU_METHOD_OF_DECLARE(m10v_smp, "socionext,milbeaut-m10v-smp", &m10v_smp_ops);

unsafe extern "C" fn m10v_pm_valid(state: suspend_state_t) -> bool {
    state == PM_SUSPEND_STANDBY || state == PM_SUSPEND_MEM
}

static mut PHYS_RESET: phys_reset_t = core::mem::transmute(0usize);

unsafe extern "C" fn m10v_die(_arg: u32) -> i32 {
    setup_mm_for_reboot();
    core::arch::asm!("wfi");
    // Boot just like a secondary
    PHYS_RESET = core::mem::transmute(virt_to_phys(cpu_reset));
    PHYS_RESET(virt_to_phys(cpu_resume));
    0
}

unsafe extern "C" fn m10v_pm_enter(state: suspend_state_t) -> i32 {
    match state {
        PM_SUSPEND_STANDBY => core::arch::asm!("wfi"),
        PM_SUSPEND_MEM => {
            cpu_pm_enter();
            cpu_suspend(0, m10v_die);
            cpu_pm_exit();
        }
        _ => {}
    }
    0
}

static M10V_PM_OPS: platform_suspend_ops = platform_suspend_ops {
    valid: Some(m10v_pm_valid),
    enter: Some(m10v_pm_enter),
};

extern "C" {
    pub fn m10v_clclk_register(cpu_dev: *mut core::ffi::c_void) -> *mut clk;
}

unsafe extern "C" fn m10v_pm_init() -> i32 {
    if of_machine_is_compatible(b"socionext,milbeaut-evb\0".as_ptr() as *const _) {
        suspend_set_ops(&M10V_PM_OPS);
    }
    0
}

// late_initcall(m10v_pm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
