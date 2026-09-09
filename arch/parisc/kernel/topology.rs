/*
 * arch/parisc/kernel/topology.c
 *
 * Copyright (C) 2017 Helge Deller <deller@gmx.de>
 *
 * based on arch/arm/kernel/topology.c
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct cpu {
    pub hotpluggable: i32,
}

#[repr(C)]
pub struct cpu_topology {
    pub thread_id: i32,
    pub core_id: i32,
    pub package_id: i32,
}

#[repr(C)]
pub struct cpuinfo_parisc {
    pub cpu_loc: i32,
}

extern "C" {
    static mut cpu_devices: cpu;
    static mut cpu_topology: *mut cpu_topology;
    static mut cpu_data: *mut cpuinfo_parisc;

    fn register_cpu(cpu: *mut cpu, cpu_id: u32) -> i32;
    fn update_siblings_masks(cpu_id: u32);
    fn reset_cpu_topology();
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);

    fn first_online_cpu() -> u64;
    fn next_online_cpu(cpu: u64) -> u64;
}

// DEFINE_PER_CPU(struct cpu, cpu_devices);

/*
 * store_cpu_topology is called at boot when only one cpu is running
 * and with the mutex cpu_hotplug.lock locked, when several cpus have booted,
 * which prevents simultaneous write access to cpu_topology array
 */
pub unsafe extern "C" fn store_cpu_topology(cpuid: u32) {
    let cpuid_topo: *mut cpu_topology = cpu_topology.add(cpuid as usize);
    let p: *mut cpuinfo_parisc;
    let mut max_socket: i32 = -1;
    let mut cpu: u64;

    /* If the cpu topology has been already set, just return */
    if (*cpuid_topo).core_id != -1 {
        return;
    }

    // #ifdef CONFIG_HOTPLUG_CPU
    (*core::ptr::addr_of_mut!(cpu_devices)).hotpluggable = 1;
    // #endif
    if register_cpu(core::ptr::addr_of_mut!(cpu_devices), cpuid) != 0 {
        pr_warn(b"Failed to register CPU%d device\0".as_ptr() as *const core::ffi::c_char, cpuid);
    }

    /* create cpu topology mapping */
    (*cpuid_topo).thread_id = -1;
    (*cpuid_topo).core_id = 0;

    p = cpu_data.add(cpuid as usize);
    // Translation of for_each_online_cpu(cpu).
    cpu = first_online_cpu();
    while cpu != u64::MAX {
        let cpuinfo: *const cpuinfo_parisc = cpu_data.add(cpu as usize);

        if cpu == cpuid as u64 { /* ignore current cpu */
            cpu = next_online_cpu(cpu);
            continue;
        }

        if (*cpuinfo).cpu_loc == (*p).cpu_loc {
            (*cpuid_topo).core_id = (*cpu_topology.add(cpu as usize)).core_id;
            if (*p).cpu_loc != 0 {
                (*cpuid_topo).core_id += 1;
                (*cpuid_topo).package_id = (*cpu_topology.add(cpu as usize)).package_id;
                cpu = next_online_cpu(cpu);
                continue;
            }
        }

        if (*cpuid_topo).package_id == -1 {
            max_socket = core::cmp::max(max_socket, (*cpu_topology.add(cpu as usize)).package_id);
        }
        cpu = next_online_cpu(cpu);
    }

    if (*cpuid_topo).package_id == -1 {
        (*cpuid_topo).package_id = max_socket + 1;
    }

    update_siblings_masks(cpuid);

    pr_info(
        b"CPU%u: cpu core %d of socket %d\n\0".as_ptr() as *const core::ffi::c_char,
        cpuid,
        (*cpu_topology.add(cpuid as usize)).core_id,
        (*cpu_topology.add(cpuid as usize)).package_id,
    );
}

/*
 * init_cpu_topology is called at boot when only one cpu is running
 * which prevent simultaneous write access to cpu_topology array
 */
pub unsafe extern "C" fn init_cpu_topology() {
    reset_cpu_topology();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
