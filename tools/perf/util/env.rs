// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/env.c. Original C includes:
// cpumap.h, dwarf-regs.h, debug.h, env.h, util/header.h, util/rwsem.h,
// linux/compiler.h, linux/kernel.h, linux/ctype.h, linux/rbtree.h,
// linux/string.h, linux/zalloc.h, cgroup.h, errno.h, sys/utsname.h,
// stdlib.h, string.h, pmu.h, pmus.h, strbuf.h, trace/beauty/beauty.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type bool_ = bool;
type u32 = u32;
type __u32 = u32;
type uint16_t = u16;
type uint32_t = u32;
type size_t = usize;

const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ULONG_MAX: c_ulong = c_ulong::MAX;

// ELF constants are supplied by the translated header dependencies.
extern "C" {
    static perf_version_string: *const c_char;

    static EM_HOST: uint16_t;
    static EM_NONE: uint16_t;
    static EM_AARCH64: uint16_t;
    static EM_ALPHA: uint16_t;
    static EM_ARC: uint16_t;
    static EM_ARM: uint16_t;
    static EM_AVR: uint16_t;
    static EM_AVR32: uint16_t;
    static EM_BLACKFIN: uint16_t;
    static EM_CRIS: uint16_t;
    static EM_CSKY: uint16_t;
    static EM_PARISC: uint16_t;
    static EM_386: uint16_t;
    static EM_LOONGARCH: uint16_t;
    static EM_M32R: uint16_t;
    static EM_68K: uint16_t;
    static EM_MICROBLAZE: uint16_t;
    static EM_MIPS: uint16_t;
    static EM_MSP430: uint16_t;
    static EM_PPC: uint16_t;
    static EM_PPC64: uint16_t;
    static EM_RISCV: uint16_t;
    static EM_S390: uint16_t;
    static EM_SH: uint16_t;
    static EM_SPARC: uint16_t;
    static EM_SPARCV9: uint16_t;
    static EM_X86_64: uint16_t;
    static EM_XTENSA: uint16_t;
    static EF_HOST: uint32_t;
}

#[repr(C)]
pub struct rb_node {
    pub rb_left: *mut rb_node,
    pub rb_right: *mut rb_node,
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
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
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct bpf_prog_info {
    pub id: __u32,
}

#[repr(C)]
pub struct bpf_prog_info_linear {
    pub info: bpf_prog_info,
}

#[repr(C)]
pub struct bpf_prog_info_node {
    pub rb_node: rb_node,
    pub info_linear: *mut bpf_prog_info_linear,
    pub metadata: *mut c_void,
}

#[repr(C)]
pub struct btf_node {
    pub rb_node: rb_node,
    pub id: __u32,
}

#[repr(C)]
pub struct bpf_progs {
    pub lock: rw_semaphore,
    pub infos: rb_root,
    pub infos_cnt: u32,
    pub btfs: rb_root,
    pub btfs_cnt: u32,
}

#[repr(C)]
pub struct domain_info {
    pub dname: *mut c_char,
    pub cpumask: *mut c_char,
    pub cpulist: *mut c_char,
}

#[repr(C)]
pub struct cpu_domain_map {
    pub nr_domains: u32,
    pub domains: *mut *mut domain_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct cpu_topology {
    pub core_id: c_int,
    pub socket_id: c_int,
    pub die_id: c_int,
}

#[repr(C)]
pub struct numa_node {
    pub map: *mut perf_cpu_map,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpu_cache_level {
    pub type_: *mut c_char,
    pub map: *mut c_char,
    pub size: *mut c_char,
}

#[repr(C)]
pub struct memory_node {
    pub set: *mut c_void,
}

#[repr(C)]
pub struct hybrid_node {
    pub pmu_name: *mut c_char,
    pub cpus: *mut c_char,
}

#[repr(C)]
pub struct pmu_caps {
    pub nr_caps: c_int,
    pub caps: *mut *mut c_char,
    pub max_branches: c_uint,
    pub br_cntr_nr: c_uint,
    pub br_cntr_width: c_uint,
    pub pmu_name: *mut c_char,
}

#[repr(C)]
pub struct perf_env {
    pub lock: mutex,
    pub bpf_progs: bpf_progs,
    pub hostname: *mut c_char,
    pub os_release: *mut c_char,
    pub version: *mut c_char,
    pub arch: *mut c_char,
    pub cpu_desc: *mut c_char,
    pub cpuid: *mut c_char,
    pub cmdline: *mut c_char,
    pub cmdline_argv: *mut *const c_char,
    pub sibling_dies: *mut c_char,
    pub sibling_cores: *mut c_char,
    pub sibling_threads: *mut c_char,
    pub pmu_mappings: *mut c_char,
    pub cpu: *mut cpu_topology,
    pub nr_cpu_pmu_caps: c_int,
    pub cpu_pmu_caps: *mut *mut c_char,
    pub numa_map: *mut c_int,
    pub nr_numa_map: c_int,
    pub nr_numa_nodes: c_int,
    pub numa_nodes: *mut numa_node,
    pub caches_cnt: c_int,
    pub caches: *mut cpu_cache_level,
    pub nr_memory_nodes: c_int,
    pub memory_nodes: *mut memory_node,
    pub nr_hybrid_nodes: c_int,
    pub hybrid_nodes: *mut hybrid_node,
    pub nr_pmus_with_caps: c_int,
    pub pmu_caps: *mut pmu_caps,
    pub cpu_domain: *mut *mut cpu_domain_map,
    pub schedstat_version: u32,
    pub nr_cpus_avail: c_int,
    pub kernel_is_64_bit: c_int,
    pub nr_cmdline: c_int,
    pub nr_pmu_mappings: c_int,
    pub max_branches: c_uint,
    pub br_cntr_nr: c_uint,
    pub br_cntr_width: c_uint,
    pub e_machine: uint16_t,
    pub e_flags: uint32_t,
    pub total_mem: u64,
}

#[repr(C)]
pub struct perf_pmu_caps {
    pub list: list_head,
    pub name: *const c_char,
    pub value: *const c_char,
}

#[repr(C)]
pub struct perf_pmu {
    pub type_: u32,
    pub name: *const c_char,
    pub nr_caps: c_int,
    pub caps: list_head,
}

#[repr(C)]
pub struct strbuf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
}

extern "C" {
    fn down_write(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn bpf_metadata_free(metadata: *mut c_void);
    fn free(ptr: *mut c_void);
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn atoi(nptr: *const c_char) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn bsearch(
        key: *const c_void,
        base: *const c_void,
        nmemb: size_t,
        size: size_t,
        compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int,
    ) -> *mut c_void;
    fn uname(buf: *mut utsname) -> c_int;
    fn mutex_destroy(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn perf_env__purge_cgroups(env: *mut perf_env);
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn cpu__max_present_cpu() -> perf_cpu;
    fn cpu__get_core_id(cpu: perf_cpu) -> c_int;
    fn cpu__get_socket_id(cpu: perf_cpu) -> c_int;
    fn cpu__get_die_id(cpu: perf_cpu) -> c_int;
    fn perf_pmus__scan(pmu: *mut perf_pmu) -> *mut perf_pmu;
    fn perf_pmus__scan_core(pmu: *mut perf_pmu) -> *mut perf_pmu;
    fn perf_pmus__num_core_pmus() -> c_int;
    fn perf_pmus__find_core_pmu() -> *mut perf_pmu;
    fn perf_pmu__caps_parse(pmu: *mut perf_pmu) -> c_int;
    fn strbuf_init(sb: *mut strbuf, hint: size_t) -> c_int;
    fn strbuf_addf(sb: *mut strbuf, fmt: *const c_char, ...) -> c_int;
    fn strbuf_add(sb: *mut strbuf, data: *const c_void, len: size_t) -> c_int;
    fn strbuf_detach(sb: *mut strbuf, sz: *mut size_t) -> *mut c_char;
    fn strbuf_release(sb: *mut strbuf);
    fn get_cpuid(buf: *mut c_char, size: size_t, cpu: perf_cpu) -> c_int;
    fn perf_cpu_map__max(map: *mut perf_cpu_map) -> perf_cpu;
    fn perf_cpu_map__for_each_cpu_next(cpu: *mut perf_cpu, idx: *mut c_uint, map: *mut perf_cpu_map) -> bool;
    fn str_ends_with(str_: *const c_char, suffix: *const c_char) -> bool;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn zalloc(size: size_t) -> *mut c_void;
    fn arch_syscalls__strerrno(e_machine: uint16_t, err: c_int) -> *const c_char;
}

extern "C" {
    fn pr_debug(fmt: *const c_char, ...);
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn zfree<T>(ptr: *mut *mut T) {
    if !(*ptr).is_null() {
        free(*ptr as *mut c_void);
        *ptr = null_mut();
    }
}

// Equivalent to Linux rb_entry/container_of; the concrete offset is supplied by
// the surrounding translated rbtree support in the complete repository.
macro_rules! rb_entry {
    ($ptr:expr, $type:ty, $field:ident) => {
        $ptr as *mut $type
    };
}

#[cfg(HAVE_LIBBPF_SUPPORT)]
#[no_mangle]
pub unsafe extern "C" fn perf_env__insert_bpf_prog_info(
    env: *mut perf_env,
    info_node: *mut bpf_prog_info_node,
) -> bool_ {
    let ret: bool_;
    down_write(&mut (*env).bpf_progs.lock);
    ret = __perf_env__insert_bpf_prog_info(env, info_node);
    up_write(&mut (*env).bpf_progs.lock);
    ret
}

#[cfg(HAVE_LIBBPF_SUPPORT)]
#[no_mangle]
pub unsafe extern "C" fn __perf_env__insert_bpf_prog_info(
    env: *mut perf_env,
    info_node: *mut bpf_prog_info_node,
) -> bool_ {
    let prog_id: __u32 = (*(*info_node).info_linear).info.id;
    let mut node: *mut bpf_prog_info_node;
    let mut parent: *mut rb_node = null_mut();
    let mut p: *mut *mut rb_node = &mut (*env).bpf_progs.infos.rb_node;

    while !(*p).is_null() {
        parent = *p;
        node = rb_entry!(parent, bpf_prog_info_node, rb_node);
        if prog_id < (*(*node).info_linear).info.id {
            p = &mut (**p).rb_left;
        } else if prog_id > (*(*node).info_linear).info.id {
            p = &mut (**p).rb_right;
        } else {
            pr_debug(cstr!("duplicated bpf prog info %u\n"), prog_id);
            return false;
        }
    }

    rb_link_node(&mut (*info_node).rb_node, parent, p);
    rb_insert_color(&mut (*info_node).rb_node, &mut (*env).bpf_progs.infos);
    (*env).bpf_progs.infos_cnt += 1;
    true
}

#[cfg(HAVE_LIBBPF_SUPPORT)]
#[no_mangle]
pub unsafe extern "C" fn perf_env__find_bpf_prog_info(
    env: *mut perf_env,
    prog_id: __u32,
) -> *mut bpf_prog_info_node {
    let mut node: *mut bpf_prog_info_node = null_mut();
    let mut n: *mut rb_node;

    down_read(&mut (*env).bpf_progs.lock);
    n = (*env).bpf_progs.infos.rb_node;

    while !n.is_null() {
        node = rb_entry!(n, bpf_prog_info_node, rb_node);
        if prog_id < (*(*node).info_linear).info.id {
            n = (*n).rb_left;
        } else if prog_id > (*(*node).info_linear).info.id {
            n = (*n).rb_right;
        } else {
            up_read(&mut (*env).bpf_progs.lock);
            return node;
        }
    }
    node = null_mut();

    up_read(&mut (*env).bpf_progs.lock);
    node
}

#[cfg(HAVE_LIBBPF_SUPPORT)]
#[no_mangle]
pub unsafe extern "C" fn perf_env__iterate_bpf_prog_info(
    env: *mut perf_env,
    cb: Option<unsafe extern "C" fn(*mut bpf_prog_info_node, *mut c_void)>,
    data: *mut c_void,
) {
    let first: *mut rb_node;

    down_read(&mut (*env).bpf_progs.lock);
    first = rb_first(&(*env).bpf_progs.infos);
    let mut node = first;
    while !node.is_null() {
        if let Some(cb_fn) = cb {
            cb_fn(rb_entry!(node, bpf_prog_info_node, rb_node), data);
        }
        node = rb_next(node);
    }
    up_read(&mut (*env).bpf_progs.lock);
}

#[cfg(HAVE_LIBBPF_SUPPORT)]
#[no_mangle]
pub unsafe extern "C" fn perf_env__insert_btf(env: *mut perf_env, btf_node: *mut btf_node) -> bool_ {
    let ret: bool_;
    down_write(&mut (*env).bpf_progs.lock);
    ret = __perf_env__insert_btf(env, btf_node);
    up_write(&mut (*env).bpf_progs.lock);
    ret
}

#[cfg(HAVE_LIBBPF_SUPPORT)]
#[no_mangle]
pub unsafe extern "C" fn __perf_env__insert_btf(env: *mut perf_env, btf_node: *mut btf_node) -> bool_ {
    let mut parent: *mut rb_node = null_mut();
    let btf_id: __u32 = (*btf_node).id;
    let mut node: *mut btf_node;
    let mut p: *mut *mut rb_node = &mut (*env).bpf_progs.btfs.rb_node;

    while !(*p).is_null() {
        parent = *p;
        node = rb_entry!(parent, btf_node, rb_node);
        if btf_id < (*node).id {
            p = &mut (**p).rb_left;
        } else if btf_id > (*node).id {
            p = &mut (**p).rb_right;
        } else {
            pr_debug(cstr!("duplicated btf %u\n"), btf_id);
            return false;
        }
    }

    rb_link_node(&mut (*btf_node).rb_node, parent, p);
    rb_insert_color(&mut (*btf_node).rb_node, &mut (*env).bpf_progs.btfs);
    (*env).bpf_progs.btfs_cnt += 1;
    true
}

#[cfg(HAVE_LIBBPF_SUPPORT)]
#[no_mangle]
pub unsafe extern "C" fn perf_env__find_btf(env: *mut perf_env, btf_id: __u32) -> *mut btf_node {
    let res: *mut btf_node;
    down_read(&mut (*env).bpf_progs.lock);
    res = __perf_env__find_btf(env, btf_id);
    up_read(&mut (*env).bpf_progs.lock);
    res
}

#[cfg(HAVE_LIBBPF_SUPPORT)]
#[no_mangle]
pub unsafe extern "C" fn __perf_env__find_btf(env: *mut perf_env, btf_id: __u32) -> *mut btf_node {
    let mut node: *mut btf_node = null_mut();
    let mut n: *mut rb_node = (*env).bpf_progs.btfs.rb_node;

    while !n.is_null() {
        node = rb_entry!(n, btf_node, rb_node);
        if btf_id < (*node).id {
            n = (*n).rb_left;
        } else if btf_id > (*node).id {
            n = (*n).rb_right;
        } else {
            return node;
        }
    }
    null_mut()
}

/* purge data in bpf_progs.infos tree */
#[cfg(HAVE_LIBBPF_SUPPORT)]
unsafe fn perf_env__purge_bpf(env: *mut perf_env) {
    let mut root: *mut rb_root;
    let mut next: *mut rb_node;

    down_write(&mut (*env).bpf_progs.lock);

    root = &mut (*env).bpf_progs.infos;
    next = rb_first(root);

    while !next.is_null() {
        let node: *mut bpf_prog_info_node = rb_entry!(next, bpf_prog_info_node, rb_node);
        next = rb_next(&(*node).rb_node);
        rb_erase(&mut (*node).rb_node, root);
        zfree(&mut (*node).info_linear);
        bpf_metadata_free((*node).metadata);
        free(node as *mut c_void);
    }

    (*env).bpf_progs.infos_cnt = 0;

    root = &mut (*env).bpf_progs.btfs;
    next = rb_first(root);

    while !next.is_null() {
        let node: *mut btf_node = rb_entry!(next, btf_node, rb_node);
        next = rb_next(&(*node).rb_node);
        rb_erase(&mut (*node).rb_node, root);
        free(node as *mut c_void);
    }

    (*env).bpf_progs.btfs_cnt = 0;

    up_write(&mut (*env).bpf_progs.lock);
}

#[cfg(not(HAVE_LIBBPF_SUPPORT))]
unsafe fn perf_env__purge_bpf(_env: *mut perf_env) {}

#[no_mangle]
pub unsafe extern "C" fn free_cpu_domain_info(
    mut cd_map: *mut *mut cpu_domain_map,
    schedstat_version: u32,
    nr: u32,
) {
    if cd_map.is_null() {
        return;
    }

    let mut i: u32 = 0;
    while i < nr {
        if (*cd_map.add(i as usize)).is_null() {
            i += 1;
            continue;
        }

        let mut j: u32 = 0;
        while j < (**cd_map.add(i as usize)).nr_domains {
            let mut d_info: *mut domain_info = *(**cd_map.add(i as usize)).domains.add(j as usize);

            if d_info.is_null() {
                j += 1;
                continue;
            }

            if schedstat_version >= 17 {
                zfree(&mut (*d_info).dname);
            }

            zfree(&mut (*d_info).cpumask);
            zfree(&mut (*d_info).cpulist);
            zfree(&mut d_info);
            j += 1;
        }
        zfree(&mut (**cd_map.add(i as usize)).domains);
        let slot = cd_map.add(i as usize);
        zfree(slot);
        i += 1;
    }
    zfree(&mut cd_map);
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__exit(env: *mut perf_env) {
    let mut i: c_int;
    let mut j: c_int;

    mutex_destroy(&mut (*env).lock);

    perf_env__purge_bpf(env);
    perf_env__purge_cgroups(env);
    zfree(&mut (*env).hostname);
    zfree(&mut (*env).os_release);
    zfree(&mut (*env).version);
    zfree(&mut (*env).arch);
    zfree(&mut (*env).cpu_desc);
    zfree(&mut (*env).cpuid);
    zfree(&mut (*env).cmdline);
    zfree(&mut (*env).cmdline_argv);
    zfree(&mut (*env).sibling_dies);
    zfree(&mut (*env).sibling_cores);
    zfree(&mut (*env).sibling_threads);
    zfree(&mut (*env).pmu_mappings);
    zfree(&mut (*env).cpu);
    i = 0;
    while i < (*env).nr_cpu_pmu_caps {
        zfree(&mut *(*env).cpu_pmu_caps.add(i as usize));
        i += 1;
    }
    zfree(&mut (*env).cpu_pmu_caps);
    zfree(&mut (*env).numa_map);

    i = 0;
    while i < (*env).nr_numa_nodes {
        perf_cpu_map__put((*(*env).numa_nodes.add(i as usize)).map);
        i += 1;
    }
    zfree(&mut (*env).numa_nodes);

    i = 0;
    while i < (*env).caches_cnt {
        cpu_cache_level__free(&mut *(*env).caches.add(i as usize));
        i += 1;
    }
    zfree(&mut (*env).caches);

    i = 0;
    while i < (*env).nr_memory_nodes {
        zfree(&mut (*(*env).memory_nodes.add(i as usize)).set);
        i += 1;
    }
    zfree(&mut (*env).memory_nodes);

    i = 0;
    while i < (*env).nr_hybrid_nodes {
        zfree(&mut (*(*env).hybrid_nodes.add(i as usize)).pmu_name);
        zfree(&mut (*(*env).hybrid_nodes.add(i as usize)).cpus);
        i += 1;
    }
    zfree(&mut (*env).hybrid_nodes);

    i = 0;
    while i < (*env).nr_pmus_with_caps {
        j = 0;
        while j < (*(*env).pmu_caps.add(i as usize)).nr_caps {
            zfree(&mut *(*(*env).pmu_caps.add(i as usize)).caps.add(j as usize));
            j += 1;
        }
        zfree(&mut (*(*env).pmu_caps.add(i as usize)).caps);
        zfree(&mut (*(*env).pmu_caps.add(i as usize)).pmu_name);
        i += 1;
    }
    zfree(&mut (*env).pmu_caps);
    free_cpu_domain_info((*env).cpu_domain, (*env).schedstat_version, (*env).nr_cpus_avail as u32);
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__init(env: *mut perf_env) {
    memset(env as *mut c_void, 0, size_of::<perf_env>());
    #[cfg(HAVE_LIBBPF_SUPPORT)]
    {
        (*env).bpf_progs.infos = rb_root { rb_node: null_mut() };
        (*env).bpf_progs.btfs = rb_root { rb_node: null_mut() };
        init_rwsem(&mut (*env).bpf_progs.lock);
    }
    (*env).kernel_is_64_bit = -1;
    mutex_init(&mut (*env).lock);
}

#[cfg(HAVE_LIBBPF_SUPPORT)]
extern "C" {
    fn init_rwsem(sem: *mut rw_semaphore);
}

unsafe fn perf_env__init_kernel_mode(env: *mut perf_env) {
    let mut arch: *const c_char = (*env).arch;

    if arch.is_null() {
        static mut UTS: utsname = utsname {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
        };

        if UTS.machine[0] == 0 {
            uname(&mut UTS);
        }
        if UTS.machine[0] != 0 {
            arch = UTS.machine.as_ptr();
        }
    }

    if !arch.is_null() {
        if !strstr(arch, cstr!("64")).is_null() || !strstr(arch, cstr!("s390x")).is_null() {
            (*env).kernel_is_64_bit = 1;
        } else {
            (*env).kernel_is_64_bit = 0;
        }
        return;
    }

    /* Fallback if completely unresolvable (assume host-bitness) */
    (*env).kernel_is_64_bit = if size_of::<*mut c_void>() == 8 { 1 } else { 0 };
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__kernel_is_64_bit(env: *mut perf_env) -> c_int {
    if (*env).kernel_is_64_bit == -1 {
        perf_env__init_kernel_mode(env);
    }

    (*env).kernel_is_64_bit
}

#[no_mangle]
pub unsafe extern "C" fn perf_arch_is_big_endian(arch: *const c_char) -> bool_ {
    if arch.is_null() {
        return cfg!(target_endian = "big");
    }

    if str_ends_with(arch, cstr!("_be"))
        || strcmp(arch, cstr!("sparc")) == 0
        || strcmp(arch, cstr!("sparc64")) == 0
        || strcmp(arch, cstr!("s390")) == 0
        || strcmp(arch, cstr!("s390x")) == 0
        || strcmp(arch, cstr!("powerpc")) == 0
        || strcmp(arch, cstr!("ppc")) == 0
        || strcmp(arch, cstr!("ppc64")) == 0
        || strcmp(arch, cstr!("mips")) == 0
        || strcmp(arch, cstr!("mips64")) == 0
        || strcmp(arch, cstr!("parisc")) == 0
        || strcmp(arch, cstr!("parisc64")) == 0
        || strcmp(arch, cstr!("m68k")) == 0
        || strcmp(arch, cstr!("armeb")) == 0
        || strcmp(arch, cstr!("mipseb")) == 0
        || strcmp(arch, cstr!("mips64eb")) == 0
    {
        return true;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__os_release(env: *mut perf_env) -> *const c_char {
    let mut uts: utsname = zeroed();
    let ret: c_int;
    let release: *const c_char;

    if env.is_null() {
        return perf_version_string;
    }

    mutex_lock(&mut (*env).lock);
    if !(*env).os_release.is_null() {
        release = (*env).os_release;
        mutex_unlock(&mut (*env).lock);
        return release;
    }

    /*
     * If env->arch is set, this is an offline target environment.
     * If the os_release is not populated in the file, we do not want
     * to poison it with the host's release which would break guest checks.
     */
    if !(*env).arch.is_null() {
        release = null();
        mutex_unlock(&mut (*env).lock);
        return release;
    }

    /*
     * The os_release is being accessed but wasn't initialized from a data
     * file, assume this is 'live' mode and use the release from uname. If
     * uname or strdup fails then use the current perf tool version.
     */
    ret = uname(&mut uts);
    (*env).os_release = strdup(if ret < 0 { perf_version_string } else { uts.release.as_ptr() });
    release = if !(*env).os_release.is_null() {
        (*env).os_release
    } else {
        perf_version_string
    };
    mutex_unlock(&mut (*env).lock);
    release
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__set_cmdline(
    env: *mut perf_env,
    argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    let mut i: c_int;

    /* do not include NULL termination */
    (*env).cmdline_argv = calloc(argc as size_t, size_of::<*mut c_char>()) as *mut *const c_char;
    if (*env).cmdline_argv.is_null() {
        return -ENOMEM;
    }

    /*
     * Must copy argv contents because it gets moved around during option
     * parsing:
     */
    i = 0;
    while i < argc {
        *(*env).cmdline_argv.add(i as usize) = *argv.add(i as usize);
        if (*(*env).cmdline_argv.add(i as usize)).is_null() {
            zfree(&mut (*env).cmdline_argv);
            return -ENOMEM;
        }
        i += 1;
    }

    (*env).nr_cmdline = argc;

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__read_cpu_topology_map(env: *mut perf_env) -> c_int {
    let mut idx: c_int;
    let nr_cpus: c_int;

    if !(*env).cpu.is_null() {
        return 0;
    }

    if (*env).nr_cpus_avail == 0 {
        (*env).nr_cpus_avail = cpu__max_present_cpu().cpu;
    }

    nr_cpus = (*env).nr_cpus_avail;
    if nr_cpus == -1 {
        return -EINVAL;
    }

    (*env).cpu = calloc(nr_cpus as size_t, size_of::<cpu_topology>()) as *mut cpu_topology;
    if (*env).cpu.is_null() {
        return -ENOMEM;
    }

    idx = 0;
    while idx < nr_cpus {
        let cpu = perf_cpu { cpu: idx };
        let core_id = cpu__get_core_id(cpu);
        let socket_id = cpu__get_socket_id(cpu);
        let die_id = cpu__get_die_id(cpu);

        (*(*env).cpu.add(idx as usize)).core_id = if core_id >= 0 { core_id } else { -1 };
        (*(*env).cpu.add(idx as usize)).socket_id = if socket_id >= 0 { socket_id } else { -1 };
        (*(*env).cpu.add(idx as usize)).die_id = if die_id >= 0 { die_id } else { -1 };
        idx += 1;
    }

    (*env).nr_cpus_avail = nr_cpus;
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__read_pmu_mappings(env: *mut perf_env) -> c_int {
    let mut pmu: *mut perf_pmu = null_mut();
    let mut pmu_num: u32 = 0;
    let mut sb: strbuf = zeroed();

    while {
        pmu = perf_pmus__scan(pmu);
        !pmu.is_null()
    } {
        pmu_num += 1;
    }

    if pmu_num == 0 {
        pr_debug(cstr!("pmu mappings not available\n"));
        return -ENOENT;
    }
    (*env).nr_pmu_mappings = pmu_num as c_int;

    if strbuf_init(&mut sb, (128 * pmu_num) as size_t) < 0 {
        return -ENOMEM;
    }

    while {
        pmu = perf_pmus__scan(pmu);
        !pmu.is_null()
    } {
        if strbuf_addf(&mut sb, cstr!("%u:%s"), (*pmu).type_, (*pmu).name) < 0 {
            strbuf_release(&mut sb);
            return -1;
        }
        /* include a NULL character at the end */
        if strbuf_add(&mut sb, cstr!("") as *const c_void, 1) < 0 {
            strbuf_release(&mut sb);
            return -1;
        }
    }

    (*env).pmu_mappings = strbuf_detach(&mut sb, null_mut());

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__read_cpuid(env: *mut perf_env) -> c_int {
    let mut cpuid: [c_char; 128] = [0; 128];
    let cpu = perf_cpu { cpu: -1 };
    let err = get_cpuid(cpuid.as_mut_ptr(), cpuid.len(), cpu);

    if err != 0 {
        return err;
    }

    free((*env).cpuid as *mut c_void);
    (*env).cpuid = strdup(cpuid.as_ptr());
    if (*env).cpuid.is_null() {
        return ENOMEM;
    }
    0
}

unsafe fn perf_env__read_nr_cpus_avail(env: *mut perf_env) -> c_int {
    if (*env).nr_cpus_avail == 0 {
        (*env).nr_cpus_avail = cpu__max_present_cpu().cpu;
    }

    if (*env).nr_cpus_avail != 0 { 0 } else { -ENOENT }
}

unsafe fn __perf_env__read_core_pmu_caps(
    pmu: *const perf_pmu,
    nr_caps: *mut c_int,
    caps: *mut *mut *mut c_char,
    max_branches: *mut c_uint,
    br_cntr_nr: *mut c_uint,
    br_cntr_width: *mut c_uint,
) -> c_int {
    let mut pcaps: *mut perf_pmu_caps = null_mut();
    let mut ptr: *mut c_char = null_mut();
    let mut tmp: *mut *mut c_char;
    let mut ret: c_int = 0;

    *nr_caps = 0;
    *caps = null_mut();

    if (*pmu).nr_caps == 0 {
        return 0;
    }

    *caps = calloc((*pmu).nr_caps as size_t, size_of::<*mut c_char>()) as *mut *mut c_char;
    if (*caps).is_null() {
        return -ENOMEM;
    }

    tmp = *caps;
    // list_for_each_entry(pcaps, &pmu->caps, list)
    while list_for_each_entry_perf_pmu_caps(&mut pcaps, &(*pmu).caps) {
        if asprintf(&mut ptr, cstr!("%s=%s"), (*pcaps).name, (*pcaps).value) < 0 {
            ret = -ENOMEM;
            break;
        }

        *tmp = ptr;
        tmp = tmp.add(1);

        if strcmp((*pcaps).name, cstr!("branches")) == 0 {
            *max_branches = atoi((*pcaps).value) as c_uint;
        } else if strcmp((*pcaps).name, cstr!("branch_counter_nr")) == 0 {
            *br_cntr_nr = atoi((*pcaps).value) as c_uint;
        } else if strcmp((*pcaps).name, cstr!("branch_counter_width")) == 0 {
            *br_cntr_width = atoi((*pcaps).value) as c_uint;
        }
    }
    if ret == 0 {
        *nr_caps = (*pmu).nr_caps;
        return 0;
    }
    while tmp != *caps {
        tmp = tmp.sub(1);
        zfree(tmp);
    }
    zfree(caps);
    *nr_caps = 0;
    ret
}

extern "C" {
    fn list_for_each_entry_perf_pmu_caps(pos: *mut *mut perf_pmu_caps, head: *const list_head) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__read_core_pmu_caps(env: *mut perf_env) -> c_int {
    let mut pmu_caps_ptr: *mut pmu_caps;
    let mut pmu: *mut perf_pmu = null_mut();
    let nr_pmu: c_int;
    let mut i: c_int = 0;
    let mut j: c_int;
    let mut ret: c_int;

    nr_pmu = perf_pmus__num_core_pmus();

    if nr_pmu == 0 {
        return -ENODEV;
    }

    if nr_pmu == 1 {
        pmu = perf_pmus__find_core_pmu();
        if pmu.is_null() {
            return -ENODEV;
        }
        ret = perf_pmu__caps_parse(pmu);
        if ret < 0 {
            return ret;
        }
        return __perf_env__read_core_pmu_caps(
            pmu,
            &mut (*env).nr_cpu_pmu_caps,
            &mut (*env).cpu_pmu_caps,
            &mut (*env).max_branches,
            &mut (*env).br_cntr_nr,
            &mut (*env).br_cntr_width,
        );
    }

    pmu_caps_ptr = calloc(nr_pmu as size_t, size_of::<pmu_caps>()) as *mut pmu_caps;
    if pmu_caps_ptr.is_null() {
        return -ENOMEM;
    }

    while {
        pmu = perf_pmus__scan_core(pmu);
        !pmu.is_null()
    } {
        if perf_pmu__caps_parse(pmu) <= 0 {
            continue;
        }
        ret = __perf_env__read_core_pmu_caps(
            pmu,
            &mut (*pmu_caps_ptr.add(i as usize)).nr_caps,
            &mut (*pmu_caps_ptr.add(i as usize)).caps,
            &mut (*pmu_caps_ptr.add(i as usize)).max_branches,
            &mut (*pmu_caps_ptr.add(i as usize)).br_cntr_nr,
            &mut (*pmu_caps_ptr.add(i as usize)).br_cntr_width,
        );
        if ret != 0 {
            let mut ei = 0;
            while ei < nr_pmu {
                j = 0;
                while j < (*pmu_caps_ptr.add(ei as usize)).nr_caps {
                    zfree(&mut *(*pmu_caps_ptr.add(ei as usize)).caps.add(j as usize));
                    j += 1;
                }
                zfree(&mut (*pmu_caps_ptr.add(ei as usize)).caps);
                zfree(&mut (*pmu_caps_ptr.add(ei as usize)).pmu_name);
                ei += 1;
            }
            zfree(&mut pmu_caps_ptr);
            return ret;
        }

        (*pmu_caps_ptr.add(i as usize)).pmu_name = strdup((*pmu).name);
        if (*pmu_caps_ptr.add(i as usize)).pmu_name.is_null() {
            ret = -ENOMEM;
            let mut ei = 0;
            while ei < nr_pmu {
                j = 0;
                while j < (*pmu_caps_ptr.add(ei as usize)).nr_caps {
                    zfree(&mut *(*pmu_caps_ptr.add(ei as usize)).caps.add(j as usize));
                    j += 1;
                }
                zfree(&mut (*pmu_caps_ptr.add(ei as usize)).caps);
                zfree(&mut (*pmu_caps_ptr.add(ei as usize)).pmu_name);
                ei += 1;
            }
            zfree(&mut pmu_caps_ptr);
            return ret;
        }
        i += 1;
    }

    (*env).nr_pmus_with_caps = nr_pmu;
    (*env).pmu_caps = pmu_caps_ptr;

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__nr_cpus_avail(env: *mut perf_env) -> c_int {
    if !env.is_null() && perf_env__read_nr_cpus_avail(env) == 0 {
        (*env).nr_cpus_avail
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpu_cache_level__free(cache: *mut cpu_cache_level) {
    zfree(&mut (*cache).type_);
    zfree(&mut (*cache).map);
    zfree(&mut (*cache).size);
}

#[repr(C)]
struct arch_to_e_machine {
    prefix: *const c_char,
    e_machine: uint16_t,
}

/*
 * A mapping from an arch prefix string to an ELF machine that can be used in a
 * bsearch. Some arch prefixes are shared an need additional processing as
 * marked next to the architecture. The prefixes handle both perf's architecture
 * naming and those from uname.
 */
static prefix_to_e_machine: [arch_to_e_machine; 30] = unsafe {
    [
        arch_to_e_machine { prefix: cstr!("aarch64"), e_machine: EM_AARCH64 },
        arch_to_e_machine { prefix: cstr!("alpha"), e_machine: EM_ALPHA },
        arch_to_e_machine { prefix: cstr!("arc"), e_machine: EM_ARC },
        arch_to_e_machine { prefix: cstr!("arm"), e_machine: EM_ARM }, /* Check also for EM_AARCH64. */
        arch_to_e_machine { prefix: cstr!("avr"), e_machine: EM_AVR }, /* Check also for EM_AVR32. */
        arch_to_e_machine { prefix: cstr!("bfin"), e_machine: EM_BLACKFIN },
        arch_to_e_machine { prefix: cstr!("blackfin"), e_machine: EM_BLACKFIN },
        arch_to_e_machine { prefix: cstr!("cris"), e_machine: EM_CRIS },
        arch_to_e_machine { prefix: cstr!("csky"), e_machine: EM_CSKY },
        arch_to_e_machine { prefix: cstr!("hppa"), e_machine: EM_PARISC },
        arch_to_e_machine { prefix: cstr!("i386"), e_machine: EM_386 },
        arch_to_e_machine { prefix: cstr!("i486"), e_machine: EM_386 },
        arch_to_e_machine { prefix: cstr!("i586"), e_machine: EM_386 },
        arch_to_e_machine { prefix: cstr!("i686"), e_machine: EM_386 },
        arch_to_e_machine { prefix: cstr!("loongarch"), e_machine: EM_LOONGARCH },
        arch_to_e_machine { prefix: cstr!("m32r"), e_machine: EM_M32R },
        arch_to_e_machine { prefix: cstr!("m68k"), e_machine: EM_68K },
        arch_to_e_machine { prefix: cstr!("microblaze"), e_machine: EM_MICROBLAZE },
        arch_to_e_machine { prefix: cstr!("mips"), e_machine: EM_MIPS },
        arch_to_e_machine { prefix: cstr!("msp430"), e_machine: EM_MSP430 },
        arch_to_e_machine { prefix: cstr!("parisc"), e_machine: EM_PARISC },
        arch_to_e_machine { prefix: cstr!("powerpc"), e_machine: EM_PPC }, /* Check also for EM_PPC64. */
        arch_to_e_machine { prefix: cstr!("ppc"), e_machine: EM_PPC }, /* Check also for EM_PPC64. */
        arch_to_e_machine { prefix: cstr!("riscv"), e_machine: EM_RISCV },
        arch_to_e_machine { prefix: cstr!("s390"), e_machine: EM_S390 },
        arch_to_e_machine { prefix: cstr!("sa110"), e_machine: EM_ARM },
        arch_to_e_machine { prefix: cstr!("sh"), e_machine: EM_SH },
        arch_to_e_machine { prefix: cstr!("sparc"), e_machine: EM_SPARC }, /* Check also for EM_SPARCV9. */
        arch_to_e_machine { prefix: cstr!("sun4u"), e_machine: EM_SPARC },
        arch_to_e_machine { prefix: cstr!("x86"), e_machine: EM_X86_64 }, /* Check also for EM_386. */
    ]
};

unsafe extern "C" fn compare_prefix(key: *const c_void, element: *const c_void) -> c_int {
    let search_key = key as *const c_char;
    let map_element = element as *const arch_to_e_machine;
    let prefix_len = strlen((*map_element).prefix);

    strncmp(search_key, (*map_element).prefix, prefix_len)
}

unsafe fn perf_arch_to_e_machine(perf_arch: *const c_char, is_64_bit: c_int) -> uint16_t {
    /* Binary search for a matching prefix. */
    let result: *const arch_to_e_machine;

    if perf_arch.is_null() {
        return EM_HOST;
    }

    result = bsearch(
        perf_arch as *const c_void,
        prefix_to_e_machine.as_ptr() as *const c_void,
        prefix_to_e_machine.len(),
        size_of::<arch_to_e_machine>(),
        compare_prefix,
    ) as *const arch_to_e_machine;

    if result.is_null() {
        pr_debug(cstr!("Unknown perf arch for ELF machine mapping: %s\n"), perf_arch);
        return EM_NONE;
    }

    /*
     * Handle conflicting prefixes. If the is_64_bit is unknown (-1) then
     * assume 64-bit. We can't use perf_env__kernel_is_64_bit as that
     * depends on the arch string.
     */
    if (*result).e_machine == EM_ARM {
        if strcmp(perf_arch, cstr!("arm64")) == 0 || strcmp(perf_arch, cstr!("aarch64")) == 0 {
            EM_AARCH64
        } else {
            EM_ARM
        }
    } else if (*result).e_machine == EM_AVR {
        if strcmp(perf_arch, cstr!("avr32")) == 0 { EM_AVR32 } else { EM_AVR }
    } else if (*result).e_machine == EM_PPC {
        if is_64_bit == 1 {
            EM_PPC64
        } else if is_64_bit == 0 {
            EM_PPC
        } else if strstarts(perf_arch, cstr!("ppc64")) {
            EM_PPC64
        } else {
            EM_PPC
        }
    } else if (*result).e_machine == EM_SPARC {
        if is_64_bit == 1 {
            EM_SPARCV9
        } else if is_64_bit == 0 {
            EM_SPARC
        } else if strcmp(perf_arch, cstr!("sparc64")) == 0 || strcmp(perf_arch, cstr!("sun4u")) == 0 {
            EM_SPARCV9
        } else {
            EM_SPARC
        }
    } else if (*result).e_machine == EM_X86_64 {
        if is_64_bit == 1 {
            EM_X86_64
        } else if is_64_bit == 0 {
            EM_386
        } else if strcmp(perf_arch, cstr!("x86_64")) == 0 || strcmp(perf_arch, cstr!("x86")) == 0 {
            EM_X86_64
        } else {
            EM_386
        }
    } else {
        (*result).e_machine
    }
}

unsafe fn e_machine_to_perf_arch(e_machine: uint16_t) -> *const c_char {
    /*
     * Table for if either the perf arch string differs from uname or there
     * are >1 ELF machine with the prefix.
     */
    let extras: [arch_to_e_machine; 8] = [
        arch_to_e_machine { prefix: cstr!("arm64"), e_machine: EM_AARCH64 },
        arch_to_e_machine { prefix: cstr!("avr32"), e_machine: EM_AVR32 },
        arch_to_e_machine { prefix: cstr!("powerpc"), e_machine: EM_PPC },
        arch_to_e_machine { prefix: cstr!("powerpc"), e_machine: EM_PPC64 },
        arch_to_e_machine { prefix: cstr!("sparc"), e_machine: EM_SPARCV9 },
        arch_to_e_machine { prefix: cstr!("x86"), e_machine: EM_386 },
        arch_to_e_machine { prefix: cstr!("x86"), e_machine: EM_X86_64 },
        arch_to_e_machine { prefix: cstr!("none"), e_machine: EM_NONE },
    ];

    let mut i: size_t = 0;
    while i < extras.len() {
        if extras[i].e_machine == e_machine {
            return extras[i].prefix;
        }
        i += 1;
    }

    i = 0;
    while i < prefix_to_e_machine.len() {
        if prefix_to_e_machine[i].e_machine == e_machine {
            return prefix_to_e_machine[i].prefix;
        }
        i += 1;
    }
    cstr!("unknown")
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__e_machine_nocache(env: *mut perf_env, e_flags: *mut uint32_t) -> uint16_t {
    let mut e_machine: uint16_t = EM_NONE;
    let mut arch: *const c_char = null();
    let mut is_64_bit: c_int = -1;

    if !e_flags.is_null() {
        *e_flags = 0;
    }

    if !env.is_null() {
        arch = (*env).arch;
        is_64_bit = (*env).kernel_is_64_bit;
    }

    if arch.is_null() {
        static mut UTS: utsname = utsname {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
        };

        if UTS.machine[0] == 0 {
            uname(&mut UTS);
        }
        if UTS.machine[0] != 0 {
            arch = UTS.machine.as_ptr();
        }
    }

    e_machine = perf_arch_to_e_machine(arch, is_64_bit);

    if !e_flags.is_null() {
        *e_flags = if e_machine == EM_HOST { EF_HOST } else { 0 };
    }

    e_machine
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__e_machine(env: *mut perf_env, e_flags: *mut uint32_t) -> uint16_t {
    let e_machine: uint16_t;
    let mut local_e_flags: uint32_t = 0;

    if !env.is_null() && (*env).e_machine != EM_NONE {
        if !e_flags.is_null() {
            *e_flags = (*env).e_flags;
        }

        return (*env).e_machine;
    }
    e_machine = perf_env__e_machine_nocache(env, &mut local_e_flags);
    /*
     * Only cache the e_machine in perf_env if env->arch is not NULL.
     * If env->arch is NULL, the e_machine is just a fallback to EM_HOST.
     * Caching it permanently would prevent dynamic, more accurate
     * thread-based session e_machine scanning later in
     * perf_session__e_machine().
     */
    if !env.is_null() && !(*env).arch.is_null() {
        (*env).e_machine = e_machine;
        (*env).e_flags = local_e_flags;
    }
    if !e_flags.is_null() {
        *e_flags = local_e_flags;
    }

    e_machine
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__arch(env: *mut perf_env) -> *const c_char {
    let e_machine: uint16_t;
    let mut arch: *const c_char;

    if env.is_null() {
        static mut UTS: utsname = utsname {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
        };
        let host_e_machine: uint16_t;

        if UTS.machine[0] == 0 {
            uname(&mut UTS);
        }
        if UTS.machine[0] != 0 {
            host_e_machine = perf_arch_to_e_machine(UTS.machine.as_ptr(), -1);
            return e_machine_to_perf_arch(host_e_machine);
        }
        return e_machine_to_perf_arch(EM_HOST);
    }

    /*
     * Lazily compute/allocate arch. The e_machine may have been
     * read from a data file and so may not be EM_HOST.
     */
    e_machine = perf_env__e_machine(env, null_mut());
    arch = e_machine_to_perf_arch(e_machine);

    if e_machine == EM_RISCV && perf_env__kernel_is_64_bit(env) == 1 {
        arch = cstr!("riscv64");
    } else if e_machine == EM_MIPS && perf_env__kernel_is_64_bit(env) == 1 {
        arch = cstr!("mips64");
    } else if e_machine == EM_PARISC && perf_env__kernel_is_64_bit(env) == 1 {
        arch = cstr!("parisc64");
    }

    arch
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__arch_strerrno(e_machine: uint16_t, err: c_int) -> *const c_char {
    arch_syscalls__strerrno(e_machine, err)
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__cpuid(env: *mut perf_env) -> *const c_char {
    let status: c_int;

    if (*env).cpuid.is_null() {
        /* Assume local operation */
        status = perf_env__read_cpuid(env);
        if status != 0 {
            return null();
        }
    }

    (*env).cpuid
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__nr_pmu_mappings(env: *mut perf_env) -> c_int {
    let status: c_int;

    if (*env).nr_pmu_mappings == 0 {
        /* Assume local operation */
        status = perf_env__read_pmu_mappings(env);
        if status != 0 {
            return 0;
        }
    }

    (*env).nr_pmu_mappings
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__pmu_mappings(env: *mut perf_env) -> *const c_char {
    let status: c_int;

    if (*env).pmu_mappings.is_null() {
        /* Assume local operation */
        status = perf_env__read_pmu_mappings(env);
        if status != 0 {
            return null();
        }
    }

    (*env).pmu_mappings
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__numa_node(env: *mut perf_env, cpu: perf_cpu) -> c_int {
    if (*env).nr_numa_map == 0 {
        let mut nn: *mut numa_node;
        let mut i: c_int;
        let mut nr: c_int = 0;

        i = 0;
        while i < (*env).nr_numa_nodes {
            nn = &mut *(*env).numa_nodes.add(i as usize);
            nr = core::cmp::max(nr, perf_cpu_map__max((*nn).map).cpu as c_int);
            i += 1;
        }

        nr += 1;

        /*
         * We initialize the numa_map array to prepare
         * it for missing cpus, which return node -1
         */
        (*env).numa_map = malloc((nr as size_t) * size_of::<c_int>()) as *mut c_int;
        if (*env).numa_map.is_null() {
            return -1;
        }

        i = 0;
        while i < nr {
            *(*env).numa_map.add(i as usize) = -1;
            i += 1;
        }

        (*env).nr_numa_map = nr;

        i = 0;
        while i < (*env).nr_numa_nodes {
            let mut tmp: perf_cpu = zeroed();
            let mut j: c_uint = 0;

            nn = &mut *(*env).numa_nodes.add(i as usize);
            while perf_cpu_map__for_each_cpu_next(&mut tmp, &mut j, (*nn).map) {
                *(*env).numa_map.add(tmp.cpu as usize) = i;
            }
            i += 1;
        }
    }

    if cpu.cpu >= 0 && cpu.cpu < (*env).nr_numa_map {
        *(*env).numa_map.add(cpu.cpu as usize)
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__has_pmu_mapping(env: *mut perf_env, pmu_name: *const c_char) -> bool_ {
    let mut pmu_mapping: *mut c_char = (*env).pmu_mappings;
    let mut colon: *mut c_char = null_mut();

    let mut i = 0;
    while i < (*env).nr_pmu_mappings {
        if strtoul(pmu_mapping, &mut colon, 0) == ULONG_MAX || *colon != b':' as c_char {
            return false;
        }

        pmu_mapping = colon.add(1);
        if strcmp(pmu_mapping, pmu_name) == 0 {
            return true;
        }

        pmu_mapping = pmu_mapping.add(strlen(pmu_mapping) + 1);
        i += 1;
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__find_pmu_cap(
    env: *mut perf_env,
    pmu_name: *const c_char,
    cap: *const c_char,
) -> *mut c_char {
    let mut cap_eq: *mut c_char;
    let cap_size: c_int;
    let mut ptr: *mut *mut c_char;
    let mut i: c_int;
    let mut j: c_int;

    if pmu_name.is_null() || cap.is_null() {
        return null_mut();
    }

    cap_size = strlen(cap) as c_int;
    cap_eq = zalloc((cap_size + 2) as size_t) as *mut c_char;
    if cap_eq.is_null() {
        return null_mut();
    }

    memcpy(cap_eq as *mut c_void, cap as *const c_void, cap_size as size_t);
    *cap_eq.add(cap_size as usize) = b'=' as c_char;

    if strcmp(pmu_name, cstr!("cpu")) == 0 {
        i = 0;
        while i < (*env).nr_cpu_pmu_caps {
            if strncmp(*(*env).cpu_pmu_caps.add(i as usize), cap_eq, (cap_size + 1) as size_t) == 0 {
                free(cap_eq as *mut c_void);
                return (*(*env).cpu_pmu_caps.add(i as usize)).add((cap_size + 1) as usize);
            }
            i += 1;
        }
        free(cap_eq as *mut c_void);
        return null_mut();
    }

    i = 0;
    while i < (*env).nr_pmus_with_caps {
        if strcmp((*(*env).pmu_caps.add(i as usize)).pmu_name, pmu_name) != 0 {
            i += 1;
            continue;
        }

        ptr = (*(*env).pmu_caps.add(i as usize)).caps;

        j = 0;
        while j < (*(*env).pmu_caps.add(i as usize)).nr_caps {
            if strncmp(*ptr.add(j as usize), cap_eq, (cap_size + 1) as size_t) == 0 {
                free(cap_eq as *mut c_void);
                return (*ptr.add(j as usize)).add((cap_size + 1) as usize);
            }
            j += 1;
        }
        i += 1;
    }

    free(cap_eq as *mut c_void);
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__find_br_cntr_info(
    env: *mut perf_env,
    nr: *mut c_uint,
    width: *mut c_uint,
) {
    if !nr.is_null() {
        *nr = if !(*env).cpu_pmu_caps.is_null() {
            (*env).br_cntr_nr
        } else {
            (*(*env).pmu_caps).br_cntr_nr
        };
    }

    if !width.is_null() {
        *width = if !(*env).cpu_pmu_caps.is_null() {
            (*env).br_cntr_width
        } else {
            (*(*env).pmu_caps).br_cntr_width
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__is_x86_amd_cpu(env: *mut perf_env) -> bool_ {
    static mut IS_AMD: c_int = 0; /* 0: Uninitialized, 1: Yes, -1: No */

    if IS_AMD == 0 {
        IS_AMD = if !(*env).cpuid.is_null() && strstarts((*env).cpuid, cstr!("AuthenticAMD")) {
            1
        } else {
            -1
        };
    }

    if IS_AMD >= 1 { true } else { false }
}

#[no_mangle]
pub unsafe extern "C" fn x86__is_amd_cpu() -> bool_ {
    let mut env: perf_env = zeroed();
    let is_amd: bool_;

    env.total_mem = 0;
    perf_env__init(&mut env);
    perf_env__cpuid(&mut env);
    is_amd = perf_env__is_x86_amd_cpu(&mut env);
    perf_env__exit(&mut env);

    is_amd
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__is_x86_intel_cpu(env: *mut perf_env) -> bool_ {
    static mut IS_INTEL: c_int = 0; /* 0: Uninitialized, 1: Yes, -1: No */

    if IS_INTEL == 0 {
        IS_INTEL = if !(*env).cpuid.is_null() && strstarts((*env).cpuid, cstr!("GenuineIntel")) {
            1
        } else {
            -1
        };
    }

    if IS_INTEL >= 1 { true } else { false }
}

#[no_mangle]
pub unsafe extern "C" fn x86__is_intel_cpu() -> bool_ {
    let mut env: perf_env = zeroed();
    let is_intel: bool_;

    env.total_mem = 0;
    perf_env__init(&mut env);
    perf_env__cpuid(&mut env);
    is_intel = perf_env__is_x86_intel_cpu(&mut env);
    perf_env__exit(&mut env);

    is_intel
}
