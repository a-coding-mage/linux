/*
 * arch/arm/kernel/topology.c
 *
 * Copyright (C) 2011 Linaro Limited.
 * Written by: Vincent Guittot
 *
 * based on arch/sh/kernel/topology.c
 */

/* Linux headers are external dependencies of this translation. */

#[cfg(CONFIG_OF)]
#[repr(C)]
struct CpuEfficiency {
    compatible: *const core::ffi::c_char,
    efficiency: usize,
}

#[cfg(CONFIG_OF)]
static TABLE_EFFICIENCY: [CpuEfficiency; 3] = [
    CpuEfficiency { compatible: b"arm,cortex-a15\0".as_ptr() as *const _, efficiency: 3891 },
    CpuEfficiency { compatible: b"arm,cortex-a7\0".as_ptr() as *const _, efficiency: 2048 },
    CpuEfficiency { compatible: core::ptr::null(), efficiency: 0 },
];

#[cfg(CONFIG_OF)]
static mut __CPU_CAPACITY: *mut usize = core::ptr::null_mut();
#[cfg(CONFIG_OF)]
static mut MIDDLE_CAPACITY: usize = 1;
#[cfg(CONFIG_OF)]
static mut CAP_FROM_DT: bool = true;

#[cfg(CONFIG_OF)]
extern "C" {
    static nr_cpu_ids: usize;
    static cpu_topology: CpuTopology;
    fn kcalloc(n: usize, size: usize, flags: u32) -> *mut usize;
    fn of_get_cpu_node(cpu: i32, thread: *mut core::ffi::c_void) -> *mut DeviceNode;
    fn of_node_put(node: *mut DeviceNode);
    fn topology_parse_cpu_capacity(node: *mut DeviceNode, cpu: i32) -> i32;
    fn of_device_is_compatible(node: *mut DeviceNode, compatible: *const core::ffi::c_char) -> bool;
    fn of_get_property(node: *mut DeviceNode, name: *const core::ffi::c_char, len: *mut i32) -> *const u32;
    fn be32_to_cpup(value: *const u32) -> u32;
    fn topology_normalize_cpu_scale();
    fn topology_set_cpu_scale(cpu: u32, scale: usize);
    fn topology_get_cpu_scale(cpu: u32) -> usize;
}

#[cfg(CONFIG_OF)]
#[repr(C)]
struct DeviceNode { _private: [u8; 0] }

#[repr(C)]
struct CpuTopology {
    thread_id: i32,
    core_id: i32,
    package_id: i32,
}

extern "C" {
    static mut cpu_topology: *mut CpuTopology;
    fn read_cpuid_mpidr() -> u32;
    fn update_siblings_masks(cpu: u32);
    fn reset_cpu_topology();
    fn smp_wmb();
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

#[cfg(CONFIG_OF)]
unsafe fn cpu_capacity(cpu: usize) -> usize {
    *__CPU_CAPACITY.add(cpu)
}

#[cfg(CONFIG_OF)]
unsafe fn parse_dt_topology() {
    let mut cpu_eff: *const CpuEfficiency;
    let mut cn: *mut DeviceNode = core::ptr::null_mut();
    let mut min_capacity = usize::MAX;
    let mut max_capacity = 0usize;
    let mut capacity = 0usize;
    let mut cpu = 0i32;

    __CPU_CAPACITY = kcalloc(nr_cpu_ids, core::mem::size_of::<usize>(), 0);

    while (cpu as usize) < nr_cpu_ids {
        let mut len = 0i32;
        cn = of_get_cpu_node(cpu, core::ptr::null_mut());
        if cn.is_null() {
            cpu += 1;
            continue;
        }

        if topology_parse_cpu_capacity(cn, cpu) != 0 {
            of_node_put(cn);
            cpu += 1;
            continue;
        }

        CAP_FROM_DT = false;
        cpu_eff = TABLE_EFFICIENCY.as_ptr();
        while !(*cpu_eff).compatible.is_null() {
            if of_device_is_compatible(cn, (*cpu_eff).compatible) { break; }
            cpu_eff = cpu_eff.add(1);
        }
        if (*cpu_eff).compatible.is_null() { cpu += 1; continue; }

        let rate = of_get_property(cn, b"clock-frequency\0".as_ptr() as *const _, &mut len);
        if rate.is_null() || len != 4 { cpu += 1; continue; }

        capacity = ((be32_to_cpup(rate) >> 20) as usize) * (*cpu_eff).efficiency;
        if capacity < min_capacity { min_capacity = capacity; }
        if capacity > max_capacity { max_capacity = capacity; }
        *__CPU_CAPACITY.add(cpu as usize) = capacity;
        cpu += 1;
    }

    if 4 * max_capacity < 3 * (max_capacity + min_capacity) {
        MIDDLE_CAPACITY = (min_capacity + max_capacity) >> (SCHED_CAPACITY_SHIFT + 1);
    } else {
        MIDDLE_CAPACITY = (max_capacity / 3) >> (SCHED_CAPACITY_SHIFT - 1);
        MIDDLE_CAPACITY += 1;
    }
    if CAP_FROM_DT { topology_normalize_cpu_scale(); }
}

#[cfg(CONFIG_OF)]
unsafe fn update_cpu_capacity(cpu: u32) {
    if cpu_capacity(cpu as usize) == 0 || CAP_FROM_DT { return; }
    topology_set_cpu_scale(cpu, cpu_capacity(cpu as usize) / MIDDLE_CAPACITY);
}

#[cfg(not(CONFIG_OF))]
unsafe fn parse_dt_topology() {}
#[cfg(not(CONFIG_OF))]
unsafe fn update_cpu_capacity(_cpuid: u32) {}

pub unsafe extern "C" fn store_cpu_topology(cpuid: u32) {
    let cpuid_topo = &mut *cpu_topology.add(cpuid as usize);
    if cpuid_topo.package_id != -1 { update_siblings_masks(cpuid); return; }

    let mpidr = read_cpuid_mpidr();
    if (mpidr & MPIDR_SMP_BITMASK) == MPIDR_SMP_VALUE {
        if (mpidr & MPIDR_MT_BITMASK) != 0 {
            cpuid_topo.thread_id = MPIDR_AFFINITY_LEVEL(mpidr, 0) as i32;
            cpuid_topo.core_id = MPIDR_AFFINITY_LEVEL(mpidr, 1) as i32;
            cpuid_topo.package_id = MPIDR_AFFINITY_LEVEL(mpidr, 2) as i32;
        } else {
            cpuid_topo.thread_id = -1;
            cpuid_topo.core_id = MPIDR_AFFINITY_LEVEL(mpidr, 0) as i32;
            cpuid_topo.package_id = MPIDR_AFFINITY_LEVEL(mpidr, 1) as i32;
        }
    } else {
        cpuid_topo.thread_id = -1;
        cpuid_topo.core_id = 0;
        cpuid_topo.package_id = -1;
    }
    update_cpu_capacity(cpuid);
    update_siblings_masks(cpuid);
}

pub unsafe extern "C" fn init_cpu_topology() {
    reset_cpu_topology();
    smp_wmb();
    parse_dt_topology();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
