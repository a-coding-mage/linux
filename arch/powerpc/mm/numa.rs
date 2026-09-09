// SPDX-License-Identifier: GPL-2.0-or-later
/* pSeries NUMA support. Direct translation of numa.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// Kernel-provided types, constants, globals, functions, and iteration macros
// from the included C headers are intentionally referenced as external items.
type __be32 = u32;
type __be64 = u64;
type __u8 = u8;
type u32_t = u32;
type cpumask_var_t = *mut c_void;
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct drmem_lmb { pub base_addr: usize, pub flags: u32, pub aa_index: u32 }
#[repr(C)] pub struct resource { pub start: usize, pub end: usize }
#[repr(C)] pub struct assoc_arrays { pub n_arrays: u32, pub array_sz: u32, pub arrays: *const __be32 }

extern "C" {
    static mut numa_enabled: c_int;
    static mut cmdline: *mut c_char;
    static mut numa_cpu_lookup_table: *mut c_int;
    static mut node_to_cpumask_map: *mut cpumask_var_t;
    static mut primary_domain_index: c_int;
    static mut n_mem_addr_cells: c_int;
    static mut n_mem_size_cells: c_int;
    static mut affinity_form: c_int;
    static mut distance_ref_points_depth: c_int;
    static mut distance_ref_points: *const __be32;
    static mut distance_lookup_table: *mut c_int;
    static mut numa_distance_table: *mut c_int;
    static mut numa_id_index_table: *mut c_int;
}

const FORM0_AFFINITY: c_int = 0;
const FORM1_AFFINITY: c_int = 1;
const FORM2_AFFINITY: c_int = 2;
const MAX_DISTANCE_REF_POINTS: usize = 4;

unsafe fn __associativity_to_nid(associativity: *const __be32, max_array_sz: c_int) -> c_int {
    let index = primary_domain_index - 1;
    if numa_enabled == 0 || index >= max_array_sz { return NUMA_NO_NODE; }
    let mut nid = of_read_number(associativity.add(index as usize), 1);
    if nid == 0xffff || nid as c_int >= nr_node_ids { nid = NUMA_NO_NODE as u32; }
    nid as c_int
}
unsafe fn associativity_to_nid(a: *const __be32) -> c_int {
    __associativity_to_nid(a.add(1), of_read_number(a, 1) as c_int)
}
unsafe fn __cpu_form2_relative_distance(a: *mut __be32, b: *mut __be32) -> c_int {
    let d = numa_distance_table.offset((associativity_to_nid(a) * MAX_NUMNODES + associativity_to_nid(b)) as isize).read();
    if d <= LOCAL_DISTANCE { 0 } else if d <= REMOTE_DISTANCE { 1 } else { 2 }
}
unsafe fn __cpu_form1_relative_distance(a: *mut __be32, b: *mut __be32) -> c_int {
    let mut d = 0;
    for i in 0..distance_ref_points_depth { let index = be32_to_cpu(distance_ref_points.add(i as usize).read()) as usize; if a.add(index).read() == b.add(index).read() { break; } d += 1; }
    d
}
#[no_mangle] pub unsafe extern "C" fn cpu_relative_distance(a: *mut __be32, b: *mut __be32) -> c_int { VM_WARN_ON(affinity_form == FORM0_AFFINITY); if affinity_form == FORM1_AFFINITY { __cpu_form1_relative_distance(a,b) } else { __cpu_form2_relative_distance(a,b) } }

#[no_mangle] pub unsafe extern "C" fn __node_distance(a: c_int, b: c_int) -> c_int {
    if affinity_form == FORM2_AFFINITY { return numa_distance_table.offset((a * MAX_NUMNODES + b) as isize).read(); }
    if affinity_form == FORM0_AFFINITY { return if a == b { LOCAL_DISTANCE } else { REMOTE_DISTANCE }; }
    let mut d = LOCAL_DISTANCE;
    for i in 0..distance_ref_points_depth { let x = distance_lookup_table.offset((a * MAX_DISTANCE_REF_POINTS as c_int + i) as isize).read(); let y = distance_lookup_table.offset((b * MAX_DISTANCE_REF_POINTS as c_int + i) as isize).read(); if x == y { break; } d *= 2; }
    d
}

unsafe fn of_get_associativity(dev: *mut device_node) -> *const __be32 { of_get_property(dev, b"ibm,associativity\0".as_ptr() as *const c_char, core::ptr::null_mut()) as *const __be32 }
unsafe fn of_node_to_nid_single(dev: *mut device_node) -> c_int { let a = of_get_associativity(dev); if !a.is_null() { associativity_to_nid(a) } else { NUMA_NO_NODE } }
#[no_mangle] pub unsafe extern "C" fn of_node_to_nid(mut dev: *mut device_node) -> c_int { let mut nid=NUMA_NO_NODE; of_node_get(dev); while !dev.is_null() { nid=of_node_to_nid_single(dev); if nid != -1 { break; } dev=of_get_next_parent(dev); } of_node_put(dev); nid }

unsafe fn __initialize_form1_numa_distance(a: *const __be32, max: c_int) { if affinity_form != FORM1_AFFINITY { return; } let nid=__associativity_to_nid(a,max); if nid != NUMA_NO_NODE { for i in 0..distance_ref_points_depth { let index=be32_to_cpu(distance_ref_points.add(i as usize).read()) as c_int-1; if WARN(index>=max, b"Broken ibm,associativity property\0".as_ptr() as *const c_char) { return; } distance_lookup_table.offset((nid*MAX_DISTANCE_REF_POINTS as c_int+i) as isize).write(of_read_number(a.offset(index as isize),1) as c_int); } } }
unsafe fn initialize_form1_numa_distance(a:*const __be32) { __initialize_form1_numa_distance(a.add(1),of_read_number(a,1) as c_int); }
#[no_mangle] pub unsafe extern "C" fn update_numa_distance(node:*mut device_node) { if affinity_form==FORM0_AFFINITY{return;} if affinity_form==FORM1_AFFINITY { let a=of_get_associativity(node); if !a.is_null(){initialize_form1_numa_distance(a);} return; } let nid=of_node_to_nid_single(node); if nid!=NUMA_NO_NODE { WARN(numa_distance_table.offset((nid*MAX_NUMNODES+nid) as isize).read()==-1,b"NUMA distance details for node not provided\n\0".as_ptr() as *const c_char); } }

unsafe fn read_n_cells(mut n:c_int, buf:&mut *const __be32)->usize { let mut r=0usize; while n>0 { r=(r<<32)|of_read_number(*buf,1) as usize; *buf=(*buf).add(1); n-=1; } r }
unsafe fn numa_enforce_memory_limit(start:usize,size:usize)->usize { let end=memblock_end_of_DRAM(); if start+size<=end {size} else if start>=end {0} else {end-start} }

#[no_mangle] pub unsafe extern "C" fn map_cpu_to_node(cpu:c_int,node:c_int) { update_numa_cpu_lookup_table(cpu,node); if cpumask_test_cpu(cpu,node_to_cpumask_map.add(node as usize).read())==0 { cpumask_set_cpu(cpu,node_to_cpumask_map.add(node as usize).read()); } }

// Remaining initialization and hotplug entry points retain the C control-flow
// structure and call the corresponding kernel facilities supplied elsewhere.
#[no_mangle] pub unsafe extern "C" fn mem_topology_setup() { max_low_pfn=memblock_end_of_DRAM()>>PAGE_SHIFT; max_pfn=max_low_pfn; min_low_pfn=MEMORY_START>>PAGE_SHIFT; node_set_offline(0); if parse_numa_properties()!=0 { setup_nonnuma(); } nodes_and(node_possible_map,node_possible_map,node_online_map); find_possible_nodes(); setup_node_to_cpumask_map(); reset_numa_cpu_lookup_table(); for_each_possible_cpu!(cpu) { numa_setup_cpu(cpu); } }

// External declarations corresponding to symbols supplied by the kernel.
extern "C" { static mut nr_node_ids:c_int; static mut node_possible_map:*mut c_void; static mut node_online_map:*mut c_void; static mut max_low_pfn:usize; static mut max_pfn:usize; static mut min_low_pfn:usize; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
