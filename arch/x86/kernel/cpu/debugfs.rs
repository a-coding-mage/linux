// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation unit.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct seq_file {
    pub private: *mut c_void,
}

#[repr(C)]
pub struct inode {
    pub i_private: *mut c_void,
}

#[repr(C)]
pub struct file;
#[repr(C)]
pub struct dentry;

#[repr(C)]
pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub read: Option<unsafe extern "C" fn()>,
    pub llseek: Option<unsafe extern "C" fn()>,
    pub release: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct topology {
    pub initial_apicid: c_uint,
    pub apicid: c_uint,
    pub pkg_id: c_uint,
    pub die_id: c_uint,
    pub cu_id: c_uint,
    pub core_id: c_uint,
    pub logical_pkg_id: c_uint,
    pub logical_die_id: c_uint,
    pub logical_core_id: c_uint,
    pub llc_id: c_uint,
    pub l2c_id: c_uint,
    pub amd_node_id: c_uint,
}

#[repr(C)]
pub struct cpuinfo_x86 {
    pub initialized: bool,
    pub topo: topology,
}

extern "C" {
    static mut cpu_info: cpuinfo_x86;
    static mut x86_topo_system: x86_topology_system;
    static mut __num_threads_per_package: c_uint;
    static mut __num_cores_per_package: c_uint;
    static mut __max_dies_per_package: c_uint;
    static mut __max_threads_per_core: c_uint;
    static mut arch_debugfs_dir: *mut dentry;

    fn per_cpu_ptr(ptr: *mut cpuinfo_x86, cpu: c_ulong) -> *mut cpuinfo_x86;
    fn cpu_online(cpu: c_ulong) -> c_int;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn get_topology_cpu_type_name(c: *mut cpuinfo_x86) -> *const c_char;
    fn topology_amd_nodes_per_pkg() -> c_uint;
    fn single_open(file: *mut file, show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn seq_read();
    fn seq_lseek();
    fn single_release();
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(name: *const c_char, mode: c_uint, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn possible_cpu_count() -> c_ulong;
}

#[repr(C)]
pub struct x86_topology_system {
    pub dom_size: [c_uint; 7],
    pub dom_shifts: [c_uint; 7],
}

const TOPO_MAX_DOMAIN: usize = 7;
const TOPO_SMT_DOMAIN: usize = 0;
const TOPO_CORE_DOMAIN: usize = 1;
const TOPO_MODULE_DOMAIN: usize = 2;
const TOPO_TILE_DOMAIN: usize = 3;
const TOPO_DIE_DOMAIN: usize = 4;
const TOPO_DIEGRP_DOMAIN: usize = 5;
const TOPO_PKG_DOMAIN: usize = 6;

unsafe extern "C" fn cpu_debug_show(m: *mut seq_file, _p: *mut c_void) -> c_int {
    let cpu = (*m).private as c_ulong;
    let c = per_cpu_ptr(&mut cpu_info, cpu);

    seq_printf(m, c"online:              %d\n".as_ptr(), cpu_online(cpu));
    if !(*c).initialized { return 0; }

    seq_printf(m, c"initial_apicid:\t    0x%x\n".as_ptr(), (*c).topo.initial_apicid);
    seq_printf(m, c"apicid:\t\t    0x%x\n".as_ptr(), (*c).topo.apicid);
    seq_printf(m, c"pkg_id:              %u\n".as_ptr(), (*c).topo.pkg_id);
    seq_printf(m, c"die_id:              %u\n".as_ptr(), (*c).topo.die_id);
    seq_printf(m, c"cu_id:               %u\n".as_ptr(), (*c).topo.cu_id);
    seq_printf(m, c"core_id:             %u\n".as_ptr(), (*c).topo.core_id);
    seq_printf(m, c"cpu_type:            %s\n".as_ptr(), get_topology_cpu_type_name(c));
    seq_printf(m, c"logical_pkg_id:      %u\n".as_ptr(), (*c).topo.logical_pkg_id);
    seq_printf(m, c"logical_die_id:      %u\n".as_ptr(), (*c).topo.logical_die_id);
    seq_printf(m, c"logical_core_id:     %u\n".as_ptr(), (*c).topo.logical_core_id);
    seq_printf(m, c"llc_id:              %u\n".as_ptr(), (*c).topo.llc_id);
    seq_printf(m, c"l2c_id:              %u\n".as_ptr(), (*c).topo.l2c_id);
    seq_printf(m, c"amd_node_id:         %u\n".as_ptr(), (*c).topo.amd_node_id);
    seq_printf(m, c"amd_nodes_per_pkg:   %u\n".as_ptr(), topology_amd_nodes_per_pkg());
    seq_printf(m, c"num_threads:         %u\n".as_ptr(), __num_threads_per_package);
    seq_printf(m, c"num_cores:           %u\n".as_ptr(), __num_cores_per_package);
    seq_printf(m, c"max_dies_per_pkg:    %u\n".as_ptr(), __max_dies_per_package);
    seq_printf(m, c"max_threads_per_core:%u\n".as_ptr(), __max_threads_per_core);
    0
}

unsafe extern "C" fn cpu_debug_open(inode: *mut inode, file: *mut file) -> c_int {
    single_open(file, cpu_debug_show, (*inode).i_private)
}

static DFS_CPU_OPS: file_operations = file_operations { open: Some(cpu_debug_open), read: None, llseek: None, release: None };

unsafe extern "C" fn dom_debug_show(m: *mut seq_file, _p: *mut c_void) -> c_int {
    static DOMAIN_NAMES: [*const c_char; TOPO_MAX_DOMAIN] = [c"Thread".as_ptr(), c"Core".as_ptr(), c"Module".as_ptr(), c"Tile".as_ptr(), c"Die".as_ptr(), c"DieGrp".as_ptr(), c"Package".as_ptr()];
    let mut nthreads: c_uint = 1;
    for dom in 0..TOPO_MAX_DOMAIN {
        nthreads *= x86_topo_system.dom_size[dom];
        seq_printf(m, c"domain: %-10s shift: %u dom_size: %5u max_threads: %5u\n".as_ptr(), DOMAIN_NAMES[dom], x86_topo_system.dom_shifts[dom], x86_topo_system.dom_size[dom], nthreads);
    }
    0
}

unsafe extern "C" fn dom_debug_open(inode: *mut inode, file: *mut file) -> c_int {
    single_open(file, dom_debug_show, (*inode).i_private)
}

static DFS_DOM_OPS: file_operations = file_operations { open: Some(dom_debug_open), read: None, llseek: None, release: None };

#[allow(non_snake_case)]
unsafe fn cpu_init_debugfs() -> c_int {
    let base = debugfs_create_dir(c"topo".as_ptr(), arch_debugfs_dir);
    debugfs_create_file(c"domains".as_ptr(), 0o444, base, core::ptr::null_mut(), &DFS_DOM_OPS);
    let dir = debugfs_create_dir(c"cpus".as_ptr(), base);
    let mut id: c_ulong = 0;
    // for_each_possible_cpu(id): the surrounding kernel supplies this build-time iteration.
    while id < possible_cpu_count() {
        let mut name = [0 as c_char; 24];
        sprintf(name.as_mut_ptr(), c"%lu".as_ptr(), id);
        debugfs_create_file(name.as_ptr(), 0o444, dir, id as *mut c_void, &DFS_CPU_OPS);
        id += 1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
