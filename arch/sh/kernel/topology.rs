// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/topology.c
 *
 *  Copyright (C) 2007  Paul Mundt
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::c_int;

extern "C" {
    static mut cpu_possible_mask: cpumask_t;
    fn register_cpu(cpu: *mut cpu, cpu_number: c_int) -> c_int;
    fn printk(format: *const u8, ...) -> c_int;
    fn register_cpu_under_node(cpu: c_int, node: c_int) -> c_int;
    fn numa_node_id() -> c_int;
    fn raw_smp_processor_id() -> c_int;
}

// The concrete kernel definitions are supplied by the corresponding headers.
#[repr(C)]
pub struct cpu {
    pub hotpluggable: c_int,
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpumask_t {
    _opaque: [u8; 0],
}

extern "C" {
    // `for_each_possible_cpu`, `for_each_present_cpu`, and `for_each_online_node`
    // are build-time kernel iteration macros and are represented by the loops below.
    fn per_cpu_cpu_devices(cpu: c_int) -> *mut cpu;
    fn possible_cpu_count() -> c_int;
    fn present_cpu_count() -> c_int;
    fn online_node_count() -> c_int;
}

static mut cpu_devices: cpu = cpu {
    hotpluggable: 0,
    _opaque: [],
};

#[no_mangle]
pub static mut cpu_core_map: [cpumask_t; NR_CPUS] = [
    cpumask_t { _opaque: [] };
    NR_CPUS
];

// EXPORT_SYMBOL(cpu_core_map);

fn cpu_coregroup_map(_cpu: c_int) -> cpumask_t {
    /*
     * Presently all SH-X3 SMP cores are multi-cores, so just keep it
     * simple until we have a method for determining topology..
     */
    unsafe { core::ptr::read(core::ptr::addr_of!(cpu_possible_mask)) }
}

pub unsafe fn cpu_coregroup_mask(cpu: c_int) -> *const cpumask_t {
    core::ptr::addr_of!(cpu_core_map[cpu as usize])
}

pub unsafe fn arch_update_cpu_topology() -> c_int {
    let mut cpu_number: u32 = 0;

    while (cpu_number as c_int) < possible_cpu_count() {
        cpu_core_map[cpu_number as usize] = cpu_coregroup_map(cpu_number as c_int);
        cpu_number = cpu_number.wrapping_add(1);
    }

    0
}

pub unsafe fn topology_init() -> c_int {
    let mut i: c_int = 0;
    let mut ret: c_int;

    while i < present_cpu_count() {
        let c: *mut cpu = per_cpu_cpu_devices(i);

        (*c).hotpluggable = 1;

        ret = register_cpu(c, i);
        if ret != 0 {
            static FUNCTION_NAME: &[u8] = b"topology_init\0";
            static WARNING: &[u8] = b"%s: register_cpu %d failed (%d)\n\0";
            printk(
                WARNING.as_ptr(),
                FUNCTION_NAME.as_ptr(),
                i,
                ret,
            );
        }

        i += 1;
    }

    // Preserve the CONFIG_NUMA && !CONFIG_SMP conditional from the source.
    // In the UP case, retain CPU association registration under each node.
    #[cfg(all(feature = "CONFIG_NUMA", not(feature = "CONFIG_SMP")))]
    {
        i = 0;
        while i < online_node_count() {
            if i != numa_node_id() {
                register_cpu_under_node(raw_smp_processor_id(), i);
            }
            i += 1;
        }
    }

    0
}

// subsys_initcall(topology_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
