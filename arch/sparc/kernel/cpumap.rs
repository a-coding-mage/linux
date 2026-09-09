// SPDX-License-Identifier: GPL-2.0
/* cpumap.c: used for optimizing CPU assignment
 *
 * Copyright (C) 2009 Hong H. Pham <hong.pham@windriver.com>
 */

use core::ffi::{c_int, c_uint, c_ulong, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
struct CpuinfoNode {
    id: c_int,
    level: c_int,
    num_cpus: c_int,
    parent_index: c_int,
    child_start: c_int,
    child_end: c_int,
    rover: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CpuinfoLevel {
    start_index: c_int,
    end_index: c_int,
    num_nodes: c_int,
}

#[repr(C)]
struct CpuinfoTree {
    total_nodes: c_int,
    level: [CpuinfoLevel; CPUINFO_LVL_MAX as usize],
    nodes: [CpuinfoNode; 0],
}

const CPUINFO_LVL_ROOT: c_int = 0;
const CPUINFO_LVL_NODE: c_int = 1;
const CPUINFO_LVL_CORE: c_int = 2;
const CPUINFO_LVL_PROC: c_int = 3;
const CPUINFO_LVL_MAX: c_int = 4;

const ROVER_NO_OP: c_int = 0;
const ROVER_INC_ON_VISIT: c_int = 1 << 0;
const ROVER_INC_PARENT_ON_LOOP: c_int = 1 << 1;

extern "C" {
    static mut cpuinfo_tree: *mut CpuinfoTree;
    static mut cpu_distribution_map: [u16; NR_CPUS as usize];
    static mut sun4v_chip_type: c_int;
    static mut cpu_online_mask: *const c_void;

    fn cpu_to_node(cpu: c_int) -> c_int;
    fn cpu_data(cpu: c_int) -> CpuData;
    fn num_possible_cpus() -> c_int;
    fn num_online_cpus() -> c_int;
    fn cpu_online(cpu: c_int) -> bool;
    fn cpumask_first(mask: *const c_void) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);
    static mut cpu_map_lock: [u8; 0];
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CpuData { core_id: c_int, proc_id: c_int }

const NR_CPUS: c_int = 0;
const GFP_ATOMIC: c_uint = 0;
const EINVAL: c_int = 22;
const SUN4V_CHIP_NIAGARA1: c_int = 0;
const SUN4V_CHIP_NIAGARA2: c_int = 0;
const SUN4V_CHIP_NIAGARA3: c_int = 0;
const SUN4V_CHIP_NIAGARA4: c_int = 0;
const SUN4V_CHIP_NIAGARA5: c_int = 0;
const SUN4V_CHIP_SPARC_M6: c_int = 0;
const SUN4V_CHIP_SPARC_M7: c_int = 0;
const SUN4V_CHIP_SPARC_M8: c_int = 0;
const SUN4V_CHIP_SPARC_SN: c_int = 0;
const SUN4V_CHIP_SPARC64X: c_int = 0;

static NIAGARA_ITERATE_METHOD: [c_int; CPUINFO_LVL_MAX as usize] = [
    ROVER_NO_OP, ROVER_INC_ON_VISIT | ROVER_INC_PARENT_ON_LOOP,
    ROVER_INC_ON_VISIT, ROVER_INC_ON_VISIT,
];
static GENERIC_ITERATE_METHOD: [c_int; CPUINFO_LVL_MAX as usize] = [
    ROVER_INC_ON_VISIT, ROVER_NO_OP, ROVER_INC_PARENT_ON_LOOP,
    ROVER_INC_ON_VISIT | ROVER_INC_PARENT_ON_LOOP,
];

unsafe fn tree_node(t: *mut CpuinfoTree, index: c_int) -> *mut CpuinfoNode {
    (*t).nodes.as_mut_ptr().add(index as usize)
}

unsafe fn cpuinfo_id(cpu: c_int, level: c_int) -> c_int {
    match level {
        CPUINFO_LVL_ROOT => 0,
        CPUINFO_LVL_NODE => cpu_to_node(cpu),
        CPUINFO_LVL_CORE => cpu_data(cpu).core_id,
        CPUINFO_LVL_PROC => cpu_data(cpu).proc_id,
        _ => -EINVAL,
    }
}

unsafe fn enumerate_cpuinfo_nodes(tree_level: *mut CpuinfoLevel) -> c_int {
    let mut prev_id = [-1; CPUINFO_LVL_MAX as usize];
    for i in 0..CPUINFO_LVL_MAX as usize {
        (*tree_level.add(i)).start_index = 0;
        (*tree_level.add(i)).end_index = 0;
        (*tree_level.add(i)).num_nodes = 0;
    }
    let mut num_nodes = 1;
    for i in 0..num_possible_cpus() {
        if !cpu_online(i) { continue; }
        for level in CPUINFO_LVL_NODE..CPUINFO_LVL_MAX {
            let n = cpuinfo_id(i, level);
            if n > prev_id[level as usize] {
                (*tree_level.add(level as usize)).num_nodes += 1;
                prev_id[level as usize] = n;
                num_nodes += 1;
            }
        }
    }
    (*tree_level.add(CPUINFO_LVL_ROOT as usize)).num_nodes = 1;
    let n = (*tree_level.add(CPUINFO_LVL_NODE as usize)).num_nodes;
    (*tree_level.add(CPUINFO_LVL_NODE as usize)).start_index = 1;
    (*tree_level.add(CPUINFO_LVL_NODE as usize)).end_index = n;
    let mut n = n + 1;
    (*tree_level.add(CPUINFO_LVL_CORE as usize)).start_index = n;
    n += (*tree_level.add(CPUINFO_LVL_CORE as usize)).num_nodes;
    (*tree_level.add(CPUINFO_LVL_CORE as usize)).end_index = n - 1;
    (*tree_level.add(CPUINFO_LVL_PROC as usize)).start_index = n;
    n += (*tree_level.add(CPUINFO_LVL_PROC as usize)).num_nodes;
    (*tree_level.add(CPUINFO_LVL_PROC as usize)).end_index = n - 1;
    num_nodes
}

unsafe fn build_cpuinfo_tree() -> *mut CpuinfoTree {
    let mut levels = [CpuinfoLevel { start_index: 0, end_index: 0, num_nodes: 0 }; CPUINFO_LVL_MAX as usize];
    let n = enumerate_cpuinfo_nodes(levels.as_mut_ptr());
    let size = core::mem::size_of::<CpuinfoTree>() + n as usize * core::mem::size_of::<CpuinfoNode>();
    let tree = kzalloc(size, GFP_ATOMIC) as *mut CpuinfoTree;
    if tree.is_null() { return core::ptr::null_mut(); }
    (*tree).total_nodes = n;
    (*tree).level = levels;
    let first_cpu = cpumask_first(cpu_online_mask);
    let mut level_rover = [0; CPUINFO_LVL_MAX as usize];
    let mut prev_id = [0; CPUINFO_LVL_MAX as usize];
    let mut num_cpus = [0; CPUINFO_LVL_MAX as usize];
    for level in (CPUINFO_LVL_ROOT..=CPUINFO_LVL_PROC).rev() {
        let idx = (*tree).level[level as usize].start_index;
        level_rover[level as usize] = idx;
        let node = &mut *tree_node(tree, idx);
        let id = cpuinfo_id(first_cpu, level);
        if id < 0 { kfree(tree as *mut c_void); return core::ptr::null_mut(); }
        node.id = id; node.level = level; node.num_cpus = 1;
        node.parent_index = if level > CPUINFO_LVL_ROOT { (*tree).level[(level-1) as usize].start_index } else { -1 };
        node.child_start = if level == CPUINFO_LVL_PROC { first_cpu } else { (*tree).level[(level+1) as usize].start_index };
        node.child_end = node.child_start; node.rover = node.child_start;
        prev_id[level as usize] = id; num_cpus[level as usize] = 1;
    }
    let mut prev_cpu = first_cpu;
    let mut cpu = first_cpu;
    let mut last_cpu = num_possible_cpus() - 1;
    while last_cpu >= 0 && !cpu_online(last_cpu) { last_cpu -= 1; }
    while { cpu += 1; cpu <= last_cpu } {
        if !cpu_online(cpu) { continue; }
        for level in (CPUINFO_LVL_ROOT..=CPUINFO_LVL_PROC).rev() {
            let id = cpuinfo_id(cpu, level);
            if id < 0 { kfree(tree as *mut c_void); return core::ptr::null_mut(); }
            if id != prev_id[level as usize] || cpu == last_cpu {
                prev_id[level as usize] = id;
                let node = &mut *tree_node(tree, level_rover[level as usize]);
                node.num_cpus = num_cpus[level as usize]; num_cpus[level as usize] = 1;
                if cpu == last_cpu { node.num_cpus += 1; }
                node.parent_index = if level == CPUINFO_LVL_ROOT { -1 } else { level_rover[(level-1) as usize] };
                node.child_end = if level == CPUINFO_LVL_PROC { if cpu == last_cpu { cpu } else { prev_cpu } } else { level_rover[(level+1) as usize] - 1 };
                let next = level_rover[level as usize] + 1; level_rover[level as usize] = next;
                if next <= (*tree).level[level as usize].end_index {
                    let next_node = &mut *tree_node(tree, next);
                    next_node.id = id; next_node.level = level;
                    next_node.child_start = if level == CPUINFO_LVL_PROC { cpu } else { level_rover[(level+1) as usize] };
                    next_node.child_end = next_node.child_start; next_node.rover = next_node.child_start;
                }
            } else { num_cpus[level as usize] += 1; }
        }
        prev_cpu = cpu;
    }
    tree
}

unsafe fn increment_rover(t: *mut CpuinfoTree, node_index: c_int, root_index: c_int, table: *const c_int) {
    let top = (*tree_node(t, root_index)).level;
    let mut node = tree_node(t, node_index);
    for level in ((*node).level..=top).rev() {
        (*node).rover += 1;
        if (*node).rover <= (*node).child_end { return; }
        (*node).rover = (*node).child_start;
        if level == top || (*table.add(level as usize) & ROVER_INC_PARENT_ON_LOOP) == 0 { return; }
        node = tree_node(t, (*node).parent_index);
    }
}

unsafe fn iterate_cpu(t: *mut CpuinfoTree, root_index: c_uint) -> c_int {
    let table = match sun4v_chip_type {
        SUN4V_CHIP_NIAGARA1 | SUN4V_CHIP_NIAGARA2 | SUN4V_CHIP_NIAGARA3 | SUN4V_CHIP_NIAGARA4 |
        SUN4V_CHIP_NIAGARA5 | SUN4V_CHIP_SPARC_M6 | SUN4V_CHIP_SPARC_M7 | SUN4V_CHIP_SPARC_M8 |
        SUN4V_CHIP_SPARC_SN | SUN4V_CHIP_SPARC64X => NIAGARA_ITERATE_METHOD.as_ptr(),
        _ => GENERIC_ITERATE_METHOD.as_ptr(),
    };
    let mut index = root_index as c_int;
    let start = (*tree_node(t, index)).level;
    for level in start..CPUINFO_LVL_MAX {
        let new_index = (*tree_node(t, index)).rover;
        if (*table.add(level as usize) & ROVER_INC_ON_VISIT) != 0 { increment_rover(t, index, root_index as c_int, table); }
        index = new_index;
    }
    index
}

unsafe fn _cpu_map_rebuild() {
    if !cpuinfo_tree.is_null() { kfree(cpuinfo_tree as *mut c_void); cpuinfo_tree = core::ptr::null_mut(); }
    cpuinfo_tree = build_cpuinfo_tree();
    if cpuinfo_tree.is_null() { return; }
    for i in 0..(*tree_node(cpuinfo_tree, 0)).num_cpus { cpu_distribution_map[i as usize] = iterate_cpu(cpuinfo_tree, 0) as u16; }
}

unsafe fn simple_map_to_cpu(index: c_uint) -> c_int {
    let end = (index % num_online_cpus() as c_uint) as c_int;
    let mut rover = 0;
    for _ in 0..num_possible_cpus() { if cpu_online(rover) { if rover >= end { return rover; } rover += 1; } }
    cpumask_first(cpu_online_mask)
}

unsafe fn _map_to_cpu(index: c_uint) -> c_int {
    if cpuinfo_tree.is_null() { _cpu_map_rebuild(); if cpuinfo_tree.is_null() { return simple_map_to_cpu(index); } }
    let root = tree_node(cpuinfo_tree, 0);
    if (*root).num_cpus != num_online_cpus() { _cpu_map_rebuild(); if cpuinfo_tree.is_null() { return simple_map_to_cpu(index); } }
    cpu_distribution_map[(index as usize) % (*tree_node(cpuinfo_tree, 0)).num_cpus as usize] as c_int
}

#[no_mangle]
pub unsafe extern "C" fn map_to_cpu(index: c_uint) -> c_int {
    let mut flags = 0;
    spin_lock_irqsave(cpu_map_lock.as_mut_ptr() as *mut c_void, &mut flags);
    let mut mapped = _map_to_cpu(index);
    while !cpu_online(mapped) { mapped = _map_to_cpu(index); }
    spin_unlock_irqrestore(cpu_map_lock.as_mut_ptr() as *mut c_void, flags);
    mapped
}

#[no_mangle]
pub unsafe extern "C" fn cpu_map_rebuild() {
    let mut flags = 0;
    spin_lock_irqsave(cpu_map_lock.as_mut_ptr() as *mut c_void, &mut flags);
    _cpu_map_rebuild();
    spin_unlock_irqrestore(cpu_map_lock.as_mut_ptr() as *mut c_void, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
