// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2014 Marvell Technology Group Ltd.
 *
 * Antoine Ténart <antoine.tenart@free-electrons.com>
 */

// Linux dependencies supplied by other translation units.
use core::ffi::c_void;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smp_operations {
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(max_cpus: u32)>,
    pub smp_boot_secondary:
        Option<unsafe extern "C" fn(cpu: u32, idle: *mut task_struct) -> i32>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_die: Option<unsafe extern "C" fn(cpu: u32)>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_kill: Option<unsafe extern "C" fn(cpu: u32) -> i32>,
}

extern "C" {
    static mut boot_inst: u32;
    static mut louis: u32;

    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn cpu_logical_map(cpu: u32) -> u32;
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const i8,
        compatible: *const i8,
    ) -> *mut device_node;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut c_void;
    fn of_node_put(np: *mut device_node);
    fn ioremap(addr: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn scu_enable(scu_base: *mut c_void);
    fn __pa_symbol(symbol: unsafe extern "C" fn()) -> u32;
    fn secondary_startup();
    fn v7_exit_coherency_flush(louis: u32);
    fn cpu_do_idle();
}

/*
 * There are two reset registers, one with self-clearing (SC)
 * reset and one with non-self-clearing reset (NON_SC).
 */
const CPU_RESET_SC: usize = 0x00;
const CPU_RESET_NON_SC: usize = 0x20;

const RESET_VECT: usize = 0x00;
const SW_RESET_ADDR: usize = 0x94;

static mut cpu_ctrl: *mut c_void = core::ptr::null_mut();

#[inline]
unsafe fn berlin_perform_reset_cpu(cpu: u32) {
    let mut val: u32;

    val = readl(cpu_ctrl.add(CPU_RESET_NON_SC));
    val &= !(1u32 << cpu_logical_map(cpu));
    writel(val, cpu_ctrl.add(CPU_RESET_NON_SC));
    val |= 1u32 << cpu_logical_map(cpu);
    writel(val, cpu_ctrl.add(CPU_RESET_NON_SC));
}

unsafe extern "C" fn berlin_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    if cpu_ctrl.is_null() {
        return -14; // -EFAULT
    }

    /*
     * Reset the CPU, making it to execute the instruction in the reset
     * exception vector.
     */
    berlin_perform_reset_cpu(cpu);

    0
}

unsafe extern "C" fn berlin_smp_prepare_cpus(_max_cpus: u32) {
    let mut np: *mut device_node;
    let scu_base: *mut c_void;
    let vectors_base: *mut c_void;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"arm,cortex-a9-scu\0".as_ptr() as *const i8,
    );
    scu_base = of_iomap(np, 0);
    of_node_put(np);
    if scu_base.is_null() {
        return;
    }

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"marvell,berlin-cpu-ctrl\0".as_ptr() as *const i8,
    );
    cpu_ctrl = of_iomap(np, 0);
    of_node_put(np);
    if cpu_ctrl.is_null() {
        iounmap(scu_base);
        return;
    }

    vectors_base = ioremap(VECTORS_BASE, SZ_32K);
    if vectors_base.is_null() {
        iounmap(scu_base);
        return;
    }

    scu_enable(scu_base);

    /*
     * Write the first instruction the CPU will execute after being reset
     * in the reset exception vector.
     */
    writel(boot_inst, vectors_base.add(RESET_VECT));

    /*
     * Write the secondary startup address into the SW reset address
     * vector. This is used by boot_inst.
     */
    writel(__pa_symbol(secondary_startup), vectors_base.add(SW_RESET_ADDR));

    iounmap(vectors_base);
    iounmap(scu_base);
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe extern "C" fn berlin_cpu_die(_cpu: u32) {
    v7_exit_coherency_flush(louis);
    loop {
        cpu_do_idle();
    }
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe extern "C" fn berlin_cpu_kill(cpu: u32) -> i32 {
    let mut val: u32;

    val = readl(cpu_ctrl.add(CPU_RESET_NON_SC));
    val &= !(1u32 << cpu_logical_map(cpu));
    writel(val, cpu_ctrl.add(CPU_RESET_NON_SC));

    1
}

static berlin_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(berlin_smp_prepare_cpus),
    smp_boot_secondary: Some(berlin_boot_secondary),
    #[cfg(CONFIG_HOTPLUG_CPU)]
    cpu_die: Some(berlin_cpu_die),
    #[cfg(CONFIG_HOTPLUG_CPU)]
    cpu_kill: Some(berlin_cpu_kill),
};

// CPU_METHOD_OF_DECLARE(berlin_smp, "marvell,berlin-smp", &berlin_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
