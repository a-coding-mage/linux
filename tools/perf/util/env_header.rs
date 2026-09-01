/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

pub type u32 = u32;
pub type u64 = u64;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type __u32 = u32;

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpu_topology_map {
    pub socket_id: c_int,
    pub die_id: c_int,
    pub cluster_id: c_int,
    pub core_id: c_int,
}

#[repr(C)]
pub struct cpu_cache_level {
    pub level: u32,
    pub line_size: u32,
    pub sets: u32,
    pub ways: u32,
    pub type_: *mut c_char,
    pub size: *mut c_char,
    pub map: *mut c_char,
}

#[repr(C)]
pub struct numa_node {
    pub node: u32,
    pub mem_total: u64,
    pub mem_free: u64,
    pub map: *mut perf_cpu_map,
}

#[repr(C)]
pub struct memory_node {
    pub node: u64,
    pub size: u64,
    pub set: *mut c_ulong,
}

#[repr(C)]
pub struct hybrid_node {
    pub pmu_name: *mut c_char,
    pub cpus: *mut c_char,
}

#[repr(C)]
pub struct pmu_caps {
    pub nr_caps: c_int,
    pub max_branches: c_uint,
    pub br_cntr_nr: c_uint,
    pub br_cntr_width: c_uint,

    pub caps: *mut *mut c_char,
    pub pmu_name: *mut c_char,
}

#[repr(C)]
pub struct domain_info {
    pub domain: u32,
    pub dname: *mut c_char,
    pub cpumask: *mut c_char,
    pub cpulist: *mut c_char,
}

#[repr(C)]
pub struct cpu_domain_map {
    pub cpu: u32,
    pub nr_domains: u32,
    pub domains: *mut *mut domain_info,
}

#[repr(C)]
pub struct perf_env_bpf_progs {
    pub lock: rw_semaphore,
    pub infos: rb_root,
    pub infos_cnt: u32,
    pub btfs: rb_root,
    pub btfs_cnt: u32,
}

#[repr(C)]
pub struct perf_env_cgroups {
    pub lock: rw_semaphore,
    pub tree: rb_root,
}

#[repr(C)]
pub struct perf_env_clock {
    pub tod_ns: u64,
    pub clockid_ns: u64,
    pub clockid_res_ns: u64,
    pub clockid: c_int,
    /*
     * enabled is valid for report mode, and is true if above
     * values are set, it's set in process_clock_data
     */
    pub enabled: bool,
}

#[repr(C)]
pub struct perf_env {
    pub hostname: *mut c_char,
    pub os_release: *mut c_char,
    pub version: *mut c_char,
    pub arch: *mut c_char,
    /* e_machine expanded from 16 to 32-bits for alignment. */
    pub e_machine: u32,
    pub e_flags: u32,
    pub nr_cpus_online: c_int,
    pub nr_cpus_avail: c_int,
    pub cpu_desc: *mut c_char,
    pub cpuid: *mut c_char,
    pub total_mem: c_ulonglong,
    pub msr_pmu_type: c_uint,
    pub max_branches: c_uint,
    pub br_cntr_nr: c_uint,
    pub br_cntr_width: c_uint,
    pub schedstat_version: c_uint,
    pub max_sched_domains: c_uint,
    pub kernel_is_64_bit: c_int,

    pub nr_cmdline: c_int,
    pub nr_sibling_cores: c_int,
    pub nr_sibling_dies: c_int,
    pub nr_sibling_threads: c_int,
    pub nr_numa_nodes: c_int,
    pub nr_memory_nodes: c_int,
    pub nr_pmu_mappings: c_int,
    pub nr_groups: c_int,
    pub nr_cpu_pmu_caps: c_int,
    pub nr_hybrid_nodes: c_int,
    pub nr_pmus_with_caps: c_int,
    pub cmdline: *mut c_char,
    pub cmdline_argv: *mut *const c_char,
    pub sibling_cores: *mut c_char,
    pub sibling_dies: *mut c_char,
    pub sibling_threads: *mut c_char,
    pub pmu_mappings: *mut c_char,
    pub cpu_pmu_caps: *mut *mut c_char,
    pub cpu: *mut cpu_topology_map,
    pub caches: *mut cpu_cache_level,
    pub cpu_domain: *mut *mut cpu_domain_map,
    pub caches_cnt: c_int,
    pub cln_size: c_uint,
    pub comp_ratio: u32,
    pub comp_ver: u32,
    pub comp_type: u32,
    pub comp_level: u32,
    pub comp_mmap_len: u32,
    pub numa_nodes: *mut numa_node,
    pub memory_nodes: *mut memory_node,
    pub memory_bsize: c_ulonglong,
    pub hybrid_nodes: *mut hybrid_node,
    pub pmu_caps: *mut pmu_caps,
    /*
     * Present in C only when HAVE_LIBBPF_SUPPORT is defined.
     *
     * bpf_info_lock protects bpf rbtrees. This is needed because the
     * trees are accessed by different threads in perf-top
     */
    pub bpf_progs: perf_env_bpf_progs,
    /* same reason as above (for perf-top) */
    pub cgroups: perf_env_cgroups,

    /* For fast cpu to numa node lookup via perf_env__numa_node */
    pub numa_map: *mut c_int,
    pub nr_numa_map: c_int,

    /* For real clock time reference. */
    pub clock: perf_env_clock,
    /* Protects lazy environment initialization (e.g. os_release, e_machine). */
    pub lock: mutex,
}

#[repr(C)]
pub enum perf_compress_type {
    PERF_COMP_NONE = 0,
    PERF_COMP_ZSTD,
    PERF_COMP_MAX,
}

#[repr(C)]
pub struct bpf_prog_info_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_node {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn perf_env__read_core_pmu_caps(env: *mut perf_env) -> c_int;
    pub fn free_cpu_domain_info(cd_map: *mut *mut cpu_domain_map, schedstat_version: u32, nr: u32);
    pub fn perf_env__exit(env: *mut perf_env);

    pub fn perf_env__kernel_is_64_bit(env: *mut perf_env) -> c_int;
    pub fn perf_arch_is_big_endian(arch: *const c_char) -> bool;
    pub fn perf_env__os_release(env: *mut perf_env) -> *const c_char;

    pub fn perf_env__set_cmdline(env: *mut perf_env, argc: c_int, argv: *mut *const c_char) -> c_int;

    pub fn perf_env__read_cpuid(env: *mut perf_env) -> c_int;
    pub fn perf_env__read_pmu_mappings(env: *mut perf_env) -> c_int;
    pub fn perf_env__nr_pmu_mappings(env: *mut perf_env) -> c_int;
    pub fn perf_env__pmu_mappings(env: *mut perf_env) -> *const c_char;

    pub fn perf_env__read_cpu_topology_map(env: *mut perf_env) -> c_int;
}

/*
 * Safe accessor for env->cpu[] topology array.  env->cpu can be NULL when
 * reading old-format perf.data that predates topology information -
 * process_cpu_topology() in header.c frees it while nr_cpus_avail remains
 * set, so callers must not index env->cpu[] without this check.
 */
pub unsafe extern "C" fn perf_env__get_cpu_topology(
    env: *mut perf_env,
    cpu: perf_cpu,
) -> *mut cpu_topology_map {
    if !(*env).cpu.is_null() && cpu.cpu >= 0 && cpu.cpu < (*env).nr_cpus_avail {
        unsafe { (*env).cpu.add(cpu.cpu as usize) }
    } else {
        core::ptr::null_mut()
    }
}

unsafe extern "C" {
    pub fn cpu_cache_level__free(cache: *mut cpu_cache_level);

    pub fn perf_env__e_machine_nocache(env: *mut perf_env, e_flags: *mut uint32_t) -> uint16_t;
    pub fn perf_env__e_machine(env: *mut perf_env, e_flags: *mut uint32_t) -> uint16_t;
    pub fn perf_env__arch(env: *mut perf_env) -> *const c_char;
    pub fn perf_env__arch_strerrno(e_machine: uint16_t, err: c_int) -> *const c_char;
    pub fn perf_env__cpuid(env: *mut perf_env) -> *const c_char;
    pub fn perf_env__nr_cpus_avail(env: *mut perf_env) -> c_int;

    pub fn perf_env__init(env: *mut perf_env);
}

/* The following declarations are present in C only when HAVE_LIBBPF_SUPPORT is defined. */
unsafe extern "C" {
    pub fn __perf_env__insert_bpf_prog_info(
        env: *mut perf_env,
        info_node: *mut bpf_prog_info_node,
    ) -> bool;
    pub fn perf_env__insert_bpf_prog_info(
        env: *mut perf_env,
        info_node: *mut bpf_prog_info_node,
    ) -> bool;
    pub fn perf_env__find_bpf_prog_info(
        env: *mut perf_env,
        prog_id: __u32,
    ) -> *mut bpf_prog_info_node;
    pub fn perf_env__iterate_bpf_prog_info(
        env: *mut perf_env,
        cb: Option<unsafe extern "C" fn(node: *mut bpf_prog_info_node, data: *mut c_void)>,
        data: *mut c_void,
    );
    pub fn perf_env__insert_btf(env: *mut perf_env, btf_node: *mut btf_node) -> bool;
    pub fn __perf_env__insert_btf(env: *mut perf_env, btf_node: *mut btf_node) -> bool;
    pub fn perf_env__find_btf(env: *mut perf_env, btf_id: __u32) -> *mut btf_node;
    pub fn __perf_env__find_btf(env: *mut perf_env, btf_id: __u32) -> *mut btf_node;
}

unsafe extern "C" {
    pub fn perf_env__numa_node(env: *mut perf_env, cpu: perf_cpu) -> c_int;
    pub fn perf_env__find_pmu_cap(
        env: *mut perf_env,
        pmu_name: *const c_char,
        cap: *const c_char,
    ) -> *mut c_char;

    pub fn perf_env__has_pmu_mapping(env: *mut perf_env, pmu_name: *const c_char) -> bool;
    pub fn perf_env__find_br_cntr_info(env: *mut perf_env, nr: *mut c_uint, width: *mut c_uint);

    pub fn x86__is_amd_cpu() -> bool;
    pub fn perf_env__is_x86_amd_cpu(env: *mut perf_env) -> bool;
    pub fn x86__is_intel_cpu() -> bool;
    pub fn perf_env__is_x86_intel_cpu(env: *mut perf_env) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
