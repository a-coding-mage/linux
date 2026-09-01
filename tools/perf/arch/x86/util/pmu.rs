// SPDX-License-Identifier: GPL-2.0
//
// Translated from ./perf/arch/x86/util/pmu.c.
// C include dependencies are preserved here as external declarations.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const MAX_SNCS: usize = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map_entry {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    pub map: *mut perf_cpu_map_entry,
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
    pub cpus: *mut perf_cpu_map,
    pub auxtrace: bool,
    pub selectable: bool,
    pub perf_event_attr_init_default: Option<unsafe extern "C" fn()>,
    pub mem_events: *mut c_void,
    pub is_core: bool,
}

#[repr(C)]
pub struct perf_pmu_caps {
    pub value: *const c_char,
}

#[repr(C)]
pub struct io_dir {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_dirent64 {
    pub d_name: [c_char; 256],
}

unsafe extern "C" {
    static INTEL_PT_PMU_NAME: *const c_char;
    static INTEL_BTS_PMU_NAME: *const c_char;
    static mut perf_mem_events_amd: *mut c_void;
    static mut perf_mem_events_amd_ldlat: *mut c_void;
    static mut perf_mem_events_intel_aux: *mut c_void;
    static mut perf_mem_events_intel: *mut c_void;
    static mut perf_mem_events__loads_ldlat: c_int;

    fn get_cpuid_str(cpu: perf_cpu) -> *mut c_char;
    fn strcmp_cpuid_str(s1: *const c_char, s2: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);
    fn sysfs__read_str(path: *const c_char, buf: *mut *mut c_char, len: *mut usize) -> c_int;
    fn perf_cpu_map__new(buf: *const c_char) -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(cpus: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_cpu_map__cpu(cpus: *const perf_cpu_map, idx: c_uint) -> perf_cpu;
    fn perf_cpu_map__empty_new(nr: c_int) -> *mut perf_cpu_map;
    fn perf_cpu_map__get(cpus: *mut perf_cpu_map) -> *mut perf_cpu_map;
    fn perf_pmu__event_source_devices_fd() -> c_int;
    fn io_dir__init(dir: *mut io_dir, fd: c_int);
    fn io_dir__readdir(dir: *mut io_dir) -> *mut io_dirent64;
    fn close(fd: c_int) -> c_int;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn pr_warning(format: *const c_char, ...);
    fn pr_debug(format: *const c_char, ...);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn x86__is_amd_cpu() -> bool;
    fn perf_pmu__caps_parse(pmu: *mut perf_pmu) -> bool;
    fn perf_pmu__get_cap(pmu: *mut perf_pmu, name: *const c_char) -> *mut perf_pmu_caps;
    fn perf_pmu__have_event(pmu: *mut perf_pmu, name: *const c_char) -> bool;
    fn intel_pt_pmu_default_config();
}

unsafe fn x86__is_intel_graniterapids() -> bool {
    static mut CHECKED_IF_GRANITERAPIDS: bool = false;
    static mut IS_GRANITERAPIDS: bool = false;

    unsafe {
        if !CHECKED_IF_GRANITERAPIDS {
            let graniterapids_cpuid = c"GenuineIntel-6-A[DE]".as_ptr();
            let cpuid = get_cpuid_str(perf_cpu { cpu: 0 });

            IS_GRANITERAPIDS = !cpuid.is_null() && strcmp_cpuid_str(graniterapids_cpuid, cpuid) == 0;
            free(cpuid as *mut c_void);
            CHECKED_IF_GRANITERAPIDS = true;
        }
        IS_GRANITERAPIDS
    }
}

unsafe fn read_sysfs_cpu_map(sysfs_path: *const c_char) -> *mut perf_cpu_map {
    unsafe {
        let mut cpus: *mut perf_cpu_map;
        let mut buf: *mut c_char = ptr::null_mut();
        let mut buf_len: usize = 0;

        if sysfs__read_str(sysfs_path, &mut buf, &mut buf_len) < 0 {
            return ptr::null_mut();
        }

        cpus = perf_cpu_map__new(buf);
        free(buf as *mut c_void);
        cpus
    }
}

unsafe fn snc_nodes_per_l3_cache() -> c_int {
    static mut CHECKED_SNC: bool = false;
    static mut SNC_NODES: c_int = 0;

    unsafe {
        if !CHECKED_SNC {
            let node_cpus =
                read_sysfs_cpu_map(c"devices/system/node/node0/cpulist".as_ptr());
            let cache_cpus =
                read_sysfs_cpu_map(c"devices/system/cpu/cpu0/cache/index3/shared_cpu_list".as_ptr());

            SNC_NODES = perf_cpu_map__nr(cache_cpus) / perf_cpu_map__nr(node_cpus);
            perf_cpu_map__put(cache_cpus);
            perf_cpu_map__put(node_cpus);
            CHECKED_SNC = true;
        }
        SNC_NODES
    }
}

unsafe fn num_chas() -> c_int {
    static mut CHECKED_CHAS: bool = false;
    static mut NUM_CHAS: c_int = 0;

    unsafe {
        if !CHECKED_CHAS {
            let fd = perf_pmu__event_source_devices_fd();
            let mut dir = core::mem::MaybeUninit::<io_dir>::uninit();
            let mut dent: *mut io_dirent64;

            if fd < 0 {
                return -1;
            }

            io_dir__init(dir.as_mut_ptr(), fd);
            let dir = dir.as_mut_ptr();

            loop {
                dent = io_dir__readdir(dir);
                if dent.is_null() {
                    break;
                }
                /* Note, dent->d_type will be DT_LNK and so isn't a useful filter. */
                if strstarts((*dent).d_name.as_ptr(), c"uncore_cha_".as_ptr()) {
                    NUM_CHAS += 1;
                }
            }
            close(fd);
            CHECKED_CHAS = true;
        }
        NUM_CHAS
    }
}

unsafe fn uncore_cha_snc(pmu: *mut perf_pmu) -> c_int {
    unsafe {
        // CHA SNC numbers are ordered correspond to the CHAs number.
        let mut cha_num: c_uint = 0;
        let num_cha: c_int;
        let chas_per_node: c_int;
        let cha_snc: c_int;
        let snc_nodes = snc_nodes_per_l3_cache();

        if snc_nodes <= 1 {
            return 0;
        }

        num_cha = num_chas();
        if num_cha <= 0 {
            pr_warning(c"Unexpected: no CHAs found\n".as_ptr());
            return 0;
        }

        /* Compute SNC for PMU. */
        if sscanf((*pmu).name, c"uncore_cha_%u".as_ptr(), &mut cha_num) != 1 {
            pr_warning(
                c"Unexpected: unable to compute CHA number '%s'\n".as_ptr(),
                (*pmu).name,
            );
            return 0;
        }
        chas_per_node = num_cha / snc_nodes;
        cha_snc = cha_num as c_int / chas_per_node;

        /* Range check cha_snc. for unexpected out of bounds. */
        if cha_snc >= MAX_SNCS as c_int {
            0
        } else {
            cha_snc
        }
    }
}

unsafe fn uncore_imc_snc(pmu: *mut perf_pmu) -> c_int {
    unsafe {
        // Compute the IMC SNC using lookup tables.
        let mut imc_num: c_uint = 0;
        let snc_nodes = snc_nodes_per_l3_cache();
        let snc2_map: [u8; 8] = [1, 1, 0, 0, 1, 1, 0, 0];
        let snc3_map: [u8; 12] = [1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 2];
        let snc_map: &[u8];

        match snc_nodes {
            2 => {
                snc_map = &snc2_map;
            }
            3 => {
                snc_map = &snc3_map;
            }
            _ => {
                /* Error or no lookup support for SNC with >3 nodes. */
                return 0;
            }
        }

        /* Compute SNC for PMU. */
        if sscanf((*pmu).name, c"uncore_imc_%u".as_ptr(), &mut imc_num) != 1 {
            pr_warning(
                c"Unexpected: unable to compute IMC number '%s'\n".as_ptr(),
                (*pmu).name,
            );
            return 0;
        }
        if imc_num as usize >= snc_map.len() {
            pr_warning(
                c"Unexpected IMC %d for SNC%d mapping\n".as_ptr(),
                imc_num,
                snc_nodes,
            );
            return 0;
        }
        snc_map[imc_num as usize] as c_int
    }
}

unsafe fn uncore_cha_imc_compute_cpu_adjust(pmu_snc: c_int) -> c_int {
    static mut CHECKED_CPU_ADJUST: [bool; MAX_SNCS] = [false; MAX_SNCS];
    static mut CPU_ADJUST: [c_int; MAX_SNCS] = [0; MAX_SNCS];

    unsafe {
        let mut node_cpus: *mut perf_cpu_map;
        let mut node_path = *b"devices/system/node/node0/cpulist\0";
        let pmu_snc_idx = pmu_snc as usize;

        /* Was adjust already computed? */
        if CHECKED_CPU_ADJUST[pmu_snc_idx] {
            return CPU_ADJUST[pmu_snc_idx];
        }

        /* SNC0 doesn't need an adjust. */
        if pmu_snc == 0 {
            CPU_ADJUST[0] = 0;
            CHECKED_CPU_ADJUST[0] = true;
            return 0;
        }

        /*
         * Use NUMA topology to compute first CPU of the NUMA node, we want to
         * adjust CPU 0 to be this and similarly for other CPUs if there is >1
         * socket.
         */
        debug_assert!(pmu_snc >= 0 && pmu_snc <= 9);
        node_path[24] = node_path[24].wrapping_add(pmu_snc as u8); // Shift node0 to be node<pmu_snc>.
        node_cpus = read_sysfs_cpu_map(node_path.as_ptr() as *const c_char);
        CPU_ADJUST[pmu_snc_idx] = perf_cpu_map__cpu(node_cpus, 0).cpu;
        if CPU_ADJUST[pmu_snc_idx] < 0 {
            pr_debug(
                c"Failed to read valid CPU list from <sysfs>/%s\n".as_ptr(),
                node_path.as_ptr() as *const c_char,
            );
            CPU_ADJUST[pmu_snc_idx] = 0;
        } else {
            CHECKED_CPU_ADJUST[pmu_snc_idx] = true;
        }
        perf_cpu_map__put(node_cpus);
        CPU_ADJUST[pmu_snc_idx]
    }
}

unsafe fn gnr_uncore_cha_imc_adjust_cpumask_for_snc(pmu: *mut perf_pmu, cha: bool) {
    unsafe {
        // With sub-NUMA clustering (SNC) there is a NUMA node per SNC in the
        // topology. For example, a two socket graniterapids machine may be set
        // up with 3-way SNC meaning there are 6 NUMA nodes that should be
        // displayed with --per-node. The cpumask of the CHA and IMC PMUs
        // reflects per-socket information meaning, for example, uncore_cha_60
        // on a two socket graniterapids machine with 120 cores per socket will
        // have a cpumask of "0,120". This cpumask needs adjusting to "40,160"
        // to reflect that uncore_cha_60 is used for the 2nd SNC of each
        // socket. Without the adjustment events on uncore_cha_60 will appear in
        // node 0 and node 3 (in our example 2 socket 3-way set up), but with
        // the adjustment they will appear in node 1 and node 4. The number of
        // CHAs is typically larger than the number of cores. The CHA numbers
        // are assumed to split evenly and inorder wrt core numbers. There are
        // fewer memory IMC PMUs than cores and mapping is handled using lookup
        // tables.
        static mut CHA_ADJUSTED: [*mut perf_cpu_map; MAX_SNCS] = [ptr::null_mut(); MAX_SNCS];
        static mut IMC_ADJUSTED: [*mut perf_cpu_map; MAX_SNCS] = [ptr::null_mut(); MAX_SNCS];

        let adjusted: *mut [*mut perf_cpu_map; MAX_SNCS] = if cha {
            &raw mut CHA_ADJUSTED
        } else {
            &raw mut IMC_ADJUSTED
        };
        let mut idx: c_uint;
        let pmu_snc: c_int;
        let mut cpu_adjust: c_int = 0;
        let mut cpu: perf_cpu;
        let alloc: bool;

        // Cpus from the kernel holds first CPU of each socket. e.g. 0,120.
        if perf_cpu_map__cpu((*pmu).cpus, 0).cpu != 0 {
            pr_debug(
                c"Ignoring cpumask adjust for %s as unexpected first CPU\n".as_ptr(),
                (*pmu).name,
            );
            return;
        }

        pmu_snc = if cha {
            uncore_cha_snc(pmu)
        } else {
            uncore_imc_snc(pmu)
        };
        if pmu_snc == 0 {
            // No adjustment necessary for the first SNC.
            return;
        }

        alloc = (*adjusted)[pmu_snc as usize].is_null();
        if alloc {
            // Hold onto the perf_cpu_map globally to avoid recomputation.
            cpu_adjust = uncore_cha_imc_compute_cpu_adjust(pmu_snc);
            (*adjusted)[pmu_snc as usize] = perf_cpu_map__empty_new(perf_cpu_map__nr((*pmu).cpus));
            if (*adjusted)[pmu_snc as usize].is_null() {
                return;
            }
        }

        idx = 0;
        while idx < perf_cpu_map__nr((*pmu).cpus) as c_uint {
            cpu = perf_cpu_map__cpu((*pmu).cpus, idx);
            // Compute the new cpu map values or if not allocating, assert
            // that they match expectations. asserts will be removed to
            // avoid overhead in NDEBUG builds.
            if alloc {
                (*(*adjusted)[pmu_snc as usize]).map.add(idx as usize).as_mut().unwrap().cpu =
                    cpu.cpu + cpu_adjust;
            } else if idx == 0 {
                cpu_adjust = perf_cpu_map__cpu((*adjusted)[pmu_snc as usize], idx).cpu - cpu.cpu;
                debug_assert!(uncore_cha_imc_compute_cpu_adjust(pmu_snc) == cpu_adjust);
            } else {
                debug_assert!(
                    perf_cpu_map__cpu((*adjusted)[pmu_snc as usize], idx).cpu
                        == cpu.cpu + cpu_adjust
                );
            }
            idx += 1;
        }

        perf_cpu_map__put((*pmu).cpus);
        (*pmu).cpus = perf_cpu_map__get((*adjusted)[pmu_snc as usize]);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_pmu__arch_init(pmu: *mut perf_pmu) {
    unsafe {
        let mut ldlat_cap: *mut perf_pmu_caps;

        if strcmp((*pmu).name, INTEL_PT_PMU_NAME) == 0 {
            (*pmu).auxtrace = true;
            (*pmu).selectable = true;
            (*pmu).perf_event_attr_init_default = Some(intel_pt_pmu_default_config);
        }
        if strcmp((*pmu).name, INTEL_BTS_PMU_NAME) == 0 {
            (*pmu).auxtrace = true;
            (*pmu).selectable = true;
        }

        if x86__is_amd_cpu() {
            if strcmp((*pmu).name, c"ibs_op".as_ptr()) != 0 {
                return;
            }

            (*pmu).mem_events = perf_mem_events_amd;

            if !perf_pmu__caps_parse(pmu) {
                return;
            }

            ldlat_cap = perf_pmu__get_cap(pmu, c"ldlat".as_ptr());
            if ldlat_cap.is_null() || strcmp((*ldlat_cap).value, c"1".as_ptr()) != 0 {
                return;
            }

            perf_mem_events__loads_ldlat = 0;
            (*pmu).mem_events = perf_mem_events_amd_ldlat;
        } else {
            if (*pmu).is_core {
                if perf_pmu__have_event(pmu, c"mem-loads-aux".as_ptr()) {
                    (*pmu).mem_events = perf_mem_events_intel_aux;
                } else {
                    (*pmu).mem_events = perf_mem_events_intel;
                }
            } else if x86__is_intel_graniterapids() {
                if strstarts((*pmu).name, c"uncore_cha_".as_ptr()) {
                    gnr_uncore_cha_imc_adjust_cpumask_for_snc(pmu, true);
                } else if strstarts((*pmu).name, c"uncore_imc_".as_ptr()) {
                    gnr_uncore_cha_imc_adjust_cpumask_for_snc(pmu, false);
                }
            }
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
