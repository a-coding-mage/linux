// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/cputopo.c. C include dependencies are represented
// by extern declarations and compatible C-layout types below.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const MAXPATHLEN: usize = 4096;
const F_OK: c_int = 0;
const UINT_MAX: u64 = c_uint::MAX as u64;

const PACKAGE_CPUS_FMT: &[u8] = b"%s/devices/system/cpu/cpu%d/topology/package_cpus_list\0";
const PACKAGE_CPUS_FMT_OLD: &[u8] =
    b"%s/devices/system/cpu/cpu%d/topology/core_siblings_list\0";
const DIE_CPUS_FMT: &[u8] = b"%s/devices/system/cpu/cpu%d/topology/die_cpus_list\0";
const CORE_CPUS_FMT: &[u8] = b"%s/devices/system/cpu/cpu%d/topology/core_cpus_list\0";
const CORE_CPUS_FMT_OLD: &[u8] =
    b"%s/devices/system/cpu/cpu%d/topology/thread_siblings_list\0";
const NODE_ONLINE_FMT: &[u8] = b"%s/devices/system/node/online\0";
const NODE_MEMINFO_FMT: &[u8] = b"%s/devices/system/node/node%d/meminfo\0";
const NODE_CPULIST_FMT: &[u8] = b"%s/devices/system/node/node%d/cpulist\0";
const NUMA_MEMINFO_SCAN_FMT: &[u8] = b"%*s %*d %31s %lu\0";
const CPUS_FILE: &[u8] = b"cpus\0";

type U32 = u32;
type U64 = u64;
type SizeT = usize;
type SSizeT = isize;
type FILE = c_void;

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *mut c_char,
}

#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}

#[repr(C)]
pub struct cpu_topology {
    pub package_cpus_lists: U32,
    pub die_cpus_lists: U32,
    pub core_cpus_lists: U32,
    pub package_cpus_list: *mut *mut c_char,
    pub die_cpus_list: *mut *mut c_char,
    pub core_cpus_list: *mut *mut c_char,
}

#[repr(C)]
pub struct numa_topology_node {
    pub node: U32,
    pub mem_total: U64,
    pub mem_free: U64,
    pub cpus: *mut c_char,
}

#[repr(C)]
pub struct numa_topology {
    pub nr: U32,
    pub nodes: [numa_topology_node; 0],
}

#[repr(C)]
pub struct hybrid_topology_node {
    pub pmu_name: *mut c_char,
    pub cpus: *mut c_char,
}

#[repr(C)]
pub struct hybrid_topology {
    pub nr: c_int,
    pub nodes: [hybrid_topology_node; 0],
}

unsafe extern "C" {
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn abort() -> !;
    fn calloc(nmemb: SizeT, size: SizeT) -> *mut c_void;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn free(ptr: *mut c_void);
    fn getline(lineptr: *mut *mut c_char, n: *mut SizeT, stream: *mut FILE) -> SSizeT;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: SizeT) -> c_int;
    fn uname(buf: *mut utsname) -> c_int;

    fn scnprintf(buf: *mut c_char, size: SizeT, fmt: *const c_char, ...) -> c_int;
    fn sysfs__mountpoint() -> *const c_char;
    fn zalloc(size: SizeT) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_char);

    fn cpu__max_present_cpu() -> perf_cpu;
    fn perf_cpu_map__new(cpu_list: *const c_char) -> *mut perf_cpu_map;
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__has(map: *const perf_cpu_map, cpu: perf_cpu) -> bool;
    fn perf_cpu_map__nr(map: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(map: *const perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_pmu__open_file(pmu: *mut perf_pmu, name: *const c_char) -> *mut FILE;
    fn perf_pmus__num_core_pmus() -> c_int;
    fn perf_pmus__scan_core(pmu: *mut perf_pmu) -> *mut perf_pmu;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe fn build_cpu_topology(tp: *mut cpu_topology, cpu: c_int) -> c_int {
    let mut fp: *mut FILE;
    let mut filename = [0 as c_char; MAXPATHLEN];
    let mut buf: *mut c_char = core::ptr::null_mut();
    let mut p: *mut c_char;
    let mut len: SizeT = 0;
    let mut sret: SSizeT;
    let mut i: U32 = 0;
    let mut ret: c_int = -1;

    unsafe {
        scnprintf(
            filename.as_mut_ptr(),
            MAXPATHLEN,
            PACKAGE_CPUS_FMT.as_ptr() as *const c_char,
            sysfs__mountpoint(),
            cpu,
        );
        if access(filename.as_ptr(), F_OK) == -1 {
            scnprintf(
                filename.as_mut_ptr(),
                MAXPATHLEN,
                PACKAGE_CPUS_FMT_OLD.as_ptr() as *const c_char,
                sysfs__mountpoint(),
                cpu,
            );
        }
        fp = fopen(filename.as_ptr(), c"r".as_ptr());
        if fp.is_null() {
            if (*tp).die_cpus_list.is_null() {
                return build_cpu_topology_try_threads(tp, cpu, filename, buf, len, ret);
            }
        } else {
            sret = getline(&mut buf, &mut len, fp);
            fclose(fp);
            if sret > 0 {
                p = strchr(buf, '\n' as c_int);
                if !p.is_null() {
                    *p = '\0' as c_char;
                }

                while i < (*tp).package_cpus_lists {
                    if strcmp(buf, *(*tp).package_cpus_list.add(i as usize)) == 0 {
                        break;
                    }
                    i += 1;
                }
                if i == (*tp).package_cpus_lists {
                    *(*tp).package_cpus_list.add(i as usize) = buf;
                    (*tp).package_cpus_lists += 1;
                    buf = core::ptr::null_mut();
                    len = 0;
                }
                ret = 0;
            }
        }

        if !(*tp).die_cpus_list.is_null() {
            scnprintf(
                filename.as_mut_ptr(),
                MAXPATHLEN,
                DIE_CPUS_FMT.as_ptr() as *const c_char,
                sysfs__mountpoint(),
                cpu,
            );
            fp = fopen(filename.as_ptr(), c"r".as_ptr());
            if !fp.is_null() {
                sret = getline(&mut buf, &mut len, fp);
                fclose(fp);
                if sret > 0 {
                    p = strchr(buf, '\n' as c_int);
                    if !p.is_null() {
                        *p = '\0' as c_char;
                    }

                    i = 0;
                    while i < (*tp).die_cpus_lists {
                        if strcmp(buf, *(*tp).die_cpus_list.add(i as usize)) == 0 {
                            break;
                        }
                        i += 1;
                    }
                    if i == (*tp).die_cpus_lists {
                        *(*tp).die_cpus_list.add(i as usize) = buf;
                        (*tp).die_cpus_lists += 1;
                        buf = core::ptr::null_mut();
                        len = 0;
                    }
                    ret = 0;
                }
            }
        }

        build_cpu_topology_try_threads(tp, cpu, filename, buf, len, ret)
    }
}

unsafe fn build_cpu_topology_try_threads(
    tp: *mut cpu_topology,
    cpu: c_int,
    mut filename: [c_char; MAXPATHLEN],
    mut buf: *mut c_char,
    mut len: SizeT,
    mut ret: c_int,
) -> c_int {
    let mut fp: *mut FILE;
    let mut p: *mut c_char;
    let mut i: U32 = 0;

    unsafe {
        scnprintf(
            filename.as_mut_ptr(),
            MAXPATHLEN,
            CORE_CPUS_FMT.as_ptr() as *const c_char,
            sysfs__mountpoint(),
            cpu,
        );
        if access(filename.as_ptr(), F_OK) == -1 {
            scnprintf(
                filename.as_mut_ptr(),
                MAXPATHLEN,
                CORE_CPUS_FMT_OLD.as_ptr() as *const c_char,
                sysfs__mountpoint(),
                cpu,
            );
        }
        fp = fopen(filename.as_ptr(), c"r".as_ptr());
        if !fp.is_null() {
            if getline(&mut buf, &mut len, fp) > 0 {
                p = strchr(buf, '\n' as c_int);
                if !p.is_null() {
                    *p = '\0' as c_char;
                }

                while i < (*tp).core_cpus_lists {
                    if strcmp(buf, *(*tp).core_cpus_list.add(i as usize)) == 0 {
                        break;
                    }
                    i += 1;
                }
                if i == (*tp).core_cpus_lists {
                    *(*tp).core_cpus_list.add(i as usize) = buf;
                    (*tp).core_cpus_lists += 1;
                    buf = core::ptr::null_mut();
                }
                ret = 0;
            }
            fclose(fp);
        }
        free(buf as *mut c_void);
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_topology__delete(tp: *mut cpu_topology) {
    let mut i: U32;

    unsafe {
        if tp.is_null() {
            return;
        }

        i = 0;
        while i < (*tp).package_cpus_lists {
            zfree((*tp).package_cpus_list.add(i as usize));
            i += 1;
        }

        i = 0;
        while i < (*tp).die_cpus_lists {
            zfree((*tp).die_cpus_list.add(i as usize));
            i += 1;
        }

        i = 0;
        while i < (*tp).core_cpus_lists {
            zfree((*tp).core_cpus_list.add(i as usize));
            i += 1;
        }

        free(tp as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_topology__smt_on(topology: *const cpu_topology) -> bool {
    unsafe {
        let mut i: U32 = 0;
        while i < (*topology).core_cpus_lists {
            let cpu_list = *(*topology).core_cpus_list.add(i as usize);

            /*
             * If there is a need to separate siblings in a core then SMT is
             * enabled.
             */
            if !strchr(cpu_list, ',' as c_int).is_null()
                || !strchr(cpu_list, '-' as c_int).is_null()
            {
                return true;
            }
            i += 1;
        }
    }
    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_topology__core_wide(
    topology: *const cpu_topology,
    user_requested_cpu_list: *const c_char,
) -> bool {
    let user_requested_cpus: *mut perf_cpu_map;

    /*
     * If user_requested_cpu_list is empty then all CPUs are recorded and so
     * core_wide is true.
     */
    if user_requested_cpu_list.is_null() {
        return true;
    }

    unsafe {
        user_requested_cpus = perf_cpu_map__new(user_requested_cpu_list);
        /* Check that every user requested CPU is the complete set of SMT threads on a core. */
        let mut i: U32 = 0;
        while i < (*topology).core_cpus_lists {
            let core_cpu_list = *(*topology).core_cpus_list.add(i as usize);
            let core_cpus = perf_cpu_map__new(core_cpu_list);
            let mut idx: c_int = 0;
            let mut has_first = false;
            let mut first = true;
            let nr = perf_cpu_map__nr(core_cpus);

            while idx < nr {
                let cpu = perf_cpu_map__cpu(core_cpus, idx);
                if first {
                    has_first = perf_cpu_map__has(user_requested_cpus, cpu);
                    first = false;
                } else {
                    /*
                     * If the first core CPU is user requested then
                     * all subsequent CPUs in the core must be user
                     * requested too. If the first CPU isn't user
                     * requested then none of the others must be
                     * too.
                     */
                    if perf_cpu_map__has(user_requested_cpus, cpu) != has_first {
                        perf_cpu_map__put(core_cpus);
                        perf_cpu_map__put(user_requested_cpus);
                        return false;
                    }
                }
                idx += 1;
            }
            perf_cpu_map__put(core_cpus);
            i += 1;
        }
        perf_cpu_map__put(user_requested_cpus);
    }
    true
}

unsafe fn has_die_topology() -> bool {
    let mut filename = [0 as c_char; MAXPATHLEN];
    let mut uts = core::mem::MaybeUninit::<utsname>::uninit();

    unsafe {
        if uname(uts.as_mut_ptr()) < 0 {
            return false;
        }

        let uts = uts.assume_init();
        if strncmp(uts.machine.as_ptr(), c"x86_64".as_ptr(), 6) != 0
            && strncmp(uts.machine.as_ptr(), c"s390x".as_ptr(), 5) != 0
        {
            return false;
        }

        scnprintf(
            filename.as_mut_ptr(),
            MAXPATHLEN,
            DIE_CPUS_FMT.as_ptr() as *const c_char,
            sysfs__mountpoint(),
            0,
        );
        if access(filename.as_ptr(), F_OK) == -1 {
            return false;
        }
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn online_topology() -> *const cpu_topology {
    static mut TOPOLOGY: *const cpu_topology = core::ptr::null();

    unsafe {
        if TOPOLOGY.is_null() {
            TOPOLOGY = cpu_topology__new();
            if TOPOLOGY.is_null() {
                pr_err(c"Error creating CPU topology".as_ptr());
                abort();
            }
        }
        TOPOLOGY
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_topology__new() -> *mut cpu_topology {
    let mut tp: *mut cpu_topology = core::ptr::null_mut();
    let mut addr: *mut c_void;
    let nr: U32;
    let mut i: U32;
    let nr_addr: U32;
    let sz: SizeT;
    let ncpus: c_long;
    let mut ret: c_int = -1;
    let map: *mut perf_cpu_map;
    let has_die = unsafe { has_die_topology() };

    unsafe {
        ncpus = cpu__max_present_cpu().cpu as c_long;

        /* build online CPU map */
        map = perf_cpu_map__new_online_cpus();
        if map.is_null() {
            pr_debug(c"failed to get system cpumap\n".as_ptr());
            return core::ptr::null_mut();
        }

        nr = (ncpus as u64 & UINT_MAX) as U32;

        sz = nr as SizeT * core::mem::size_of::<*mut c_char>();
        if has_die {
            nr_addr = 3;
        } else {
            nr_addr = 2;
        }
        addr = calloc(1, core::mem::size_of::<cpu_topology>() + nr_addr as SizeT * sz);
        if !addr.is_null() {
            tp = addr as *mut cpu_topology;
            addr = (addr as *mut u8).add(core::mem::size_of::<cpu_topology>()) as *mut c_void;
            (*tp).package_cpus_list = addr as *mut *mut c_char;
            addr = (addr as *mut u8).add(sz) as *mut c_void;
            if has_die {
                (*tp).die_cpus_list = addr as *mut *mut c_char;
                addr = (addr as *mut u8).add(sz) as *mut c_void;
            }
            (*tp).core_cpus_list = addr as *mut *mut c_char;

            i = 0;
            while i < nr {
                if !perf_cpu_map__has(map, perf_cpu { cpu: i as c_int }) {
                    i += 1;
                    continue;
                }

                ret = build_cpu_topology(tp, i as c_int);
                if ret < 0 {
                    break;
                }
                i += 1;
            }
        }

        perf_cpu_map__put(map);
        if ret != 0 {
            cpu_topology__delete(tp);
            tp = core::ptr::null_mut();
        }
    }
    tp
}

unsafe fn load_numa_node(node: *mut numa_topology_node, nr: c_int) -> c_int {
    let mut str_ = [0 as c_char; MAXPATHLEN];
    let mut field = [0 as c_char; 32];
    let mut buf: *mut c_char = core::ptr::null_mut();
    let mut p: *mut c_char;
    let mut len: SizeT = 0;
    let ret: c_int = -1;
    let mut fp: *mut FILE;
    let mut mem: U64 = 0;

    unsafe {
        (*node).node = nr as U32;

        scnprintf(
            str_.as_mut_ptr(),
            MAXPATHLEN,
            NODE_MEMINFO_FMT.as_ptr() as *const c_char,
            sysfs__mountpoint(),
            nr,
        );
        fp = fopen(str_.as_ptr(), c"r".as_ptr());
        if fp.is_null() {
            return -1;
        }

        while getline(&mut buf, &mut len, fp) > 0 {
            /* skip over invalid lines */
            if strchr(buf, ':' as c_int).is_null() {
                continue;
            }
            if sscanf(
                buf,
                NUMA_MEMINFO_SCAN_FMT.as_ptr() as *const c_char,
                field.as_mut_ptr(),
                &mut mem,
            ) != 2
            {
                free(buf as *mut c_void);
                if !fp.is_null() {
                    fclose(fp);
                }
                return ret;
            }
            if strcmp(field.as_ptr(), c"MemTotal:".as_ptr()) == 0 {
                (*node).mem_total = mem;
            }
            if strcmp(field.as_ptr(), c"MemFree:".as_ptr()) == 0 {
                (*node).mem_free = mem;
            }
            if (*node).mem_total != 0 && (*node).mem_free != 0 {
                break;
            }
        }

        fclose(fp);
        fp = core::ptr::null_mut();

        scnprintf(
            str_.as_mut_ptr(),
            MAXPATHLEN,
            NODE_CPULIST_FMT.as_ptr() as *const c_char,
            sysfs__mountpoint(),
            nr,
        );

        fp = fopen(str_.as_ptr(), c"r".as_ptr());
        if fp.is_null() {
            return -1;
        }

        if getline(&mut buf, &mut len, fp) <= 0 {
            free(buf as *mut c_void);
            if !fp.is_null() {
                fclose(fp);
            }
            return ret;
        }

        p = strchr(buf, '\n' as c_int);
        if !p.is_null() {
            *p = '\0' as c_char;
        }

        (*node).cpus = buf;
        fclose(fp);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn numa_topology__new() -> *mut numa_topology {
    let mut node_map: *mut perf_cpu_map = core::ptr::null_mut();
    let mut tp: *mut numa_topology = core::ptr::null_mut();
    let mut path = [0 as c_char; MAXPATHLEN];
    let mut buf: *mut c_char = core::ptr::null_mut();
    let mut len: SizeT = 0;
    let mut nr: U32;
    let mut i: U32;
    let mut fp: *mut FILE;
    let mut c: *mut c_char;

    unsafe {
        scnprintf(
            path.as_mut_ptr(),
            MAXPATHLEN,
            NODE_ONLINE_FMT.as_ptr() as *const c_char,
            sysfs__mountpoint(),
        );

        fp = fopen(path.as_ptr(), c"r".as_ptr());
        if fp.is_null() {
            return core::ptr::null_mut();
        }

        if getline(&mut buf, &mut len, fp) > 0 {
            c = strchr(buf, '\n' as c_int);
            if !c.is_null() {
                *c = '\0' as c_char;
            }

            node_map = perf_cpu_map__new(buf);
            if !node_map.is_null() {
                nr = perf_cpu_map__nr(node_map) as U32;

                tp = zalloc(
                    core::mem::size_of::<numa_topology>()
                        + core::mem::size_of::<numa_topology_node>() * nr as usize,
                ) as *mut numa_topology;
                if !tp.is_null() {
                    (*tp).nr = nr;

                    i = 0;
                    while i < nr {
                        if load_numa_node(
                            ((*tp).nodes.as_mut_ptr()).add(i as usize),
                            perf_cpu_map__cpu(node_map, i as c_int).cpu,
                        ) != 0
                        {
                            numa_topology__delete(tp);
                            tp = core::ptr::null_mut();
                            break;
                        }
                        i += 1;
                    }
                }
            }
        }

        free(buf as *mut c_void);
        fclose(fp);
        perf_cpu_map__put(node_map);
    }
    tp
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn numa_topology__delete(tp: *mut numa_topology) {
    let mut i: U32;

    unsafe {
        i = 0;
        while i < (*tp).nr {
            zfree(&mut (*(*tp).nodes.as_mut_ptr().add(i as usize)).cpus);
            i += 1;
        }

        free(tp as *mut c_void);
    }
}

unsafe fn load_hybrid_node(node: *mut hybrid_topology_node, pmu: *mut perf_pmu) -> c_int {
    let mut buf: *mut c_char = core::ptr::null_mut();
    let mut p: *mut c_char;
    let mut fp: *mut FILE;
    let mut len: SizeT = 0;

    unsafe {
        (*node).pmu_name = strdup((*pmu).name);
        if (*node).pmu_name.is_null() {
            return -1;
        }

        fp = perf_pmu__open_file(pmu, CPUS_FILE.as_ptr() as *const c_char);
        if fp.is_null() {
            zfree(&mut (*node).pmu_name);
            free(buf as *mut c_void);
            return -1;
        }

        if getline(&mut buf, &mut len, fp) <= 0 {
            fclose(fp);
            zfree(&mut (*node).pmu_name);
            free(buf as *mut c_void);
            return -1;
        }

        p = strchr(buf, '\n' as c_int);
        if !p.is_null() {
            *p = '\0' as c_char;
        }

        fclose(fp);
        (*node).cpus = buf;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hybrid_topology__new() -> *mut hybrid_topology {
    let mut pmu: *mut perf_pmu = core::ptr::null_mut();
    let mut tp: *mut hybrid_topology;
    let nr: c_int;
    let mut i: c_int = 0;

    unsafe {
        nr = perf_pmus__num_core_pmus();

        if nr <= 1 {
            return core::ptr::null_mut();
        }

        tp = zalloc(
            core::mem::size_of::<hybrid_topology>()
                + core::mem::size_of::<hybrid_topology_node>() * nr as usize,
        ) as *mut hybrid_topology;
        if tp.is_null() {
            return core::ptr::null_mut();
        }

        (*tp).nr = nr;
        loop {
            pmu = perf_pmus__scan_core(pmu);
            if pmu.is_null() {
                break;
            }
            if load_hybrid_node(((*tp).nodes.as_mut_ptr()).add(i as usize), pmu) != 0 {
                hybrid_topology__delete(tp);
                return core::ptr::null_mut();
            }
            i += 1;
        }
    }

    tp
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hybrid_topology__delete(tp: *mut hybrid_topology) {
    let mut i: U32;

    unsafe {
        i = 0;
        while i < (*tp).nr as U32 {
            zfree(&mut (*(*tp).nodes.as_mut_ptr().add(i as usize)).pmu_name);
            zfree(&mut (*(*tp).nodes.as_mut_ptr().add(i as usize)).cpus);
            i += 1;
        }

        free(tp as *mut c_void);
    }
}
