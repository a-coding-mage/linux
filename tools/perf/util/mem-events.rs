// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/mem-events.c.
// C include dependencies are expected to be supplied by the surrounding tree:
// cpumap.h, map_symbol.h, mem-events.h, mem-info.h, debug.h, evsel.h,
// symbol.h, pmu.h, pmus.h, api/fs/fs.h, linux/kernel.h, and libc/sys APIs.

use core::ffi::{c_char, c_int, c_uint, c_ulong};
use core::mem::MaybeUninit;
use core::ptr;

pub type size_t = usize;
pub type u64 = u64;

const PATH_MAX: usize = 4096;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;

extern "C" {
    static mut stderr: *mut FILE;
    static mut verbose: c_int;

    fn malloc(size: size_t) -> *mut core::ffi::c_void;
    fn free(ptr: *mut core::ffi::c_void);
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: size_t)
        -> *mut core::ffi::c_void;
    fn stat(path: *const c_char, st: *mut stat) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;

    fn sysfs__mount() -> *const c_char;
    fn perf_pmus__scan(pmu: *mut perf_pmu) -> *mut perf_pmu;
    fn pr_err(format: *const c_char, ...);
    fn pr_warning(format: *const c_char, ...);

    fn perf_cpu_map__merge(dst: *mut *mut perf_cpu_map, src: *mut perf_cpu_map) -> c_int;
    fn cpu_map__online() -> *mut perf_cpu_map;
    fn perf_cpu_map__equal(a: *mut perf_cpu_map, b: *mut perf_cpu_map) -> bool;
    fn cpu_map__snprint(map: *mut perf_cpu_map, buf: *mut c_char, size: size_t) -> c_int;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);

    fn mem_info__const_data_src(mem_info: *const mem_info) -> *const perf_mem_data_src;
    fn mem_info__data_src(mem_info: *mut mem_info) -> *mut perf_mem_data_src;
    fn mem_info__daddr(mem_info: *mut mem_info) -> *mut mem_addr;
    fn mem_info__iaddr(mem_info: *mut mem_info) -> *mut mem_addr;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
    pub mem_events: *mut perf_mem_event,
    pub cpus: *mut perf_cpu_map,
}

#[repr(C)]
pub struct perf_mem_event {
    pub tag: *const c_char,
    pub name: *const c_char,
    pub event_name: *const c_char,
    pub ldlat: bool,
    pub aux_event: u64,
    pub supported: bool,
}

#[repr(C)]
pub struct perf_event_attr {
    pub config: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub pmu: *mut perf_pmu,
    pub core: evsel_core,
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct mem_addr {
    pub addr: u64,
    pub ms: map_symbol,
}

#[repr(C)]
pub struct mem_info {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_mem_data_src {
    pub val: u64,
    pub bits: perf_mem_data_src_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_mem_data_src_bits {
    pub mem_op: u64,
    pub mem_lvl: u64,
    pub mem_snoop: u64,
    pub mem_lock: u64,
    pub mem_dtlb: u64,
    pub mem_lvl_num: c_int,
    pub mem_remote: u64,
    pub mem_snoopx: u64,
    pub mem_blk: u64,
    pub mem_hops: usize,
}

impl perf_mem_data_src {
    unsafe fn mem_op(&self) -> u64 {
        self.bits.mem_op
    }
    unsafe fn mem_lvl(&self) -> u64 {
        self.bits.mem_lvl
    }
    unsafe fn mem_snoop(&self) -> u64 {
        self.bits.mem_snoop
    }
    unsafe fn mem_lock(&self) -> u64 {
        self.bits.mem_lock
    }
    unsafe fn mem_dtlb(&self) -> u64 {
        self.bits.mem_dtlb
    }
    unsafe fn mem_lvl_num(&self) -> c_int {
        self.bits.mem_lvl_num
    }
    unsafe fn mem_remote(&self) -> u64 {
        self.bits.mem_remote
    }
    unsafe fn mem_snoopx(&self) -> u64 {
        self.bits.mem_snoopx
    }
    unsafe fn mem_blk(&self) -> u64 {
        self.bits.mem_blk
    }
    unsafe fn mem_hops(&self) -> usize {
        self.bits.mem_hops
    }
}

#[repr(C)]
pub struct c2c_stats {
    pub nr_entries: u64,
    pub locks: u64,
    pub store: u64,
    pub st_uncache: u64,
    pub st_noadrs: u64,
    pub st_l1hit: u64,
    pub st_l1miss: u64,
    pub st_na: u64,
    pub load: u64,
    pub ld_excl: u64,
    pub ld_shared: u64,
    pub ld_uncache: u64,
    pub ld_io: u64,
    pub ld_miss: u64,
    pub ld_noadrs: u64,
    pub ld_fbhit: u64,
    pub ld_l1hit: u64,
    pub ld_l2hit: u64,
    pub ld_llchit: u64,
    pub lcl_hitm: u64,
    pub rmt_hitm: u64,
    pub tot_hitm: u64,
    pub lcl_peer: u64,
    pub rmt_peer: u64,
    pub tot_peer: u64,
    pub rmt_hit: u64,
    pub lcl_dram: u64,
    pub rmt_dram: u64,
    pub blk_data: u64,
    pub blk_addr: u64,
    pub nomap: u64,
    pub noparse: u64,
}

pub type mem_stat_type = c_uint;

extern "C" {
    static PERF_MEM_EVENTS__MAX: c_int;
    static PERF_MEM_EVENTS__LOAD: c_int;
    static PERF_MEM_EVENTS__LOAD_STORE: c_int;
    static PERF_MEM_EVENTS__STORE: c_int;

    static PERF_MEM_TLB_NA: u64;
    static PERF_MEM_TLB_HIT: u64;
    static PERF_MEM_TLB_MISS: u64;
    static PERF_MEM_TLB_L1: u64;
    static PERF_MEM_TLB_L2: u64;

    static PERF_MEM_LVL_HIT: u64;
    static PERF_MEM_LVL_MISS: u64;
    static PERF_MEM_LVL_NA: u64;
    static PERF_MEM_LVL_UNC: u64;
    static PERF_MEM_LVL_IO: u64;
    static PERF_MEM_LVL_LFB: u64;
    static PERF_MEM_LVL_L1: u64;
    static PERF_MEM_LVL_L2: u64;
    static PERF_MEM_LVL_L3: u64;
    static PERF_MEM_LVL_LOC_RAM: u64;
    static PERF_MEM_LVL_REM_RAM1: u64;
    static PERF_MEM_LVL_REM_RAM2: u64;
    static PERF_MEM_LVL_REM_CCE1: u64;
    static PERF_MEM_LVL_REM_CCE2: u64;

    static PERF_MEM_LVLNUM_L1: c_int;
    static PERF_MEM_LVLNUM_L2: c_int;
    static PERF_MEM_LVLNUM_L3: c_int;
    static PERF_MEM_LVLNUM_L4: c_int;
    static PERF_MEM_LVLNUM_L2_MHB: c_int;
    static PERF_MEM_LVLNUM_MSC: c_int;
    static PERF_MEM_LVLNUM_UNC: c_int;
    static PERF_MEM_LVLNUM_CXL: c_int;
    static PERF_MEM_LVLNUM_IO: c_int;
    static PERF_MEM_LVLNUM_ANY_CACHE: c_int;
    static PERF_MEM_LVLNUM_LFB: c_int;
    static PERF_MEM_LVLNUM_RAM: c_int;
    static PERF_MEM_LVLNUM_PMEM: c_int;
    static PERF_MEM_LVLNUM_NA: c_int;

    static PERF_MEM_LOCK_NA: u64;
    static PERF_MEM_LOCK_LOCKED: u64;

    static PERF_MEM_OP_NA: u64;
    static PERF_MEM_OP_LOAD: u64;
    static PERF_MEM_OP_STORE: u64;
    static PERF_MEM_OP_PFETCH: u64;
    static PERF_MEM_OP_EXEC: u64;

    static PERF_MEM_SNOOP_NA: u64;
    static PERF_MEM_SNOOP_HIT: u64;
    static PERF_MEM_SNOOP_HITM: u64;
    static PERF_MEM_SNOOP_MISS: u64;

    static PERF_MEM_SNOOPX_PEER: u64;

    static PERF_MEM_BLK_NA: u64;
    static PERF_MEM_BLK_DATA: u64;
    static PERF_MEM_BLK_ADDR: u64;

    static PERF_MEM_STAT_OP: mem_stat_type;
    static PERF_MEM_STAT_CACHE: mem_stat_type;
    static PERF_MEM_STAT_MEMORY: mem_stat_type;
    static PERF_MEM_STAT_SNOOP: mem_stat_type;
    static PERF_MEM_STAT_DTLB: mem_stat_type;

    static MEM_STAT_OP_LOAD: c_int;
    static MEM_STAT_OP_STORE: c_int;
    static MEM_STAT_OP_LDST: c_int;
    static MEM_STAT_OP_PFETCH: c_int;
    static MEM_STAT_OP_EXEC: c_int;
    static MEM_STAT_OP_OTHER: c_int;
    static MEM_STAT_CACHE_L1: c_int;
    static MEM_STAT_CACHE_L2: c_int;
    static MEM_STAT_CACHE_L3: c_int;
    static MEM_STAT_CACHE_L4: c_int;
    static MEM_STAT_CACHE_L1_BUF: c_int;
    static MEM_STAT_CACHE_L2_BUF: c_int;
    static MEM_STAT_CACHE_OTHER: c_int;
    static MEM_STAT_MEMORY_RAM: c_int;
    static MEM_STAT_MEMORY_MSC: c_int;
    static MEM_STAT_MEMORY_UNC: c_int;
    static MEM_STAT_MEMORY_CXL: c_int;
    static MEM_STAT_MEMORY_IO: c_int;
    static MEM_STAT_MEMORY_PMEM: c_int;
    static MEM_STAT_MEMORY_OTHER: c_int;
    static MEM_STAT_SNOOP_HIT: c_int;
    static MEM_STAT_SNOOP_HITM: c_int;
    static MEM_STAT_SNOOP_MISS: c_int;
    static MEM_STAT_SNOOP_OTHER: c_int;
    static MEM_STAT_DTLB_L1_HIT: c_int;
    static MEM_STAT_DTLB_L2_HIT: c_int;
    static MEM_STAT_DTLB_ANY_HIT: c_int;
    static MEM_STAT_DTLB_MISS: c_int;
    static MEM_STAT_DTLB_OTHER: c_int;
}

pub static mut perf_mem_events__loads_ldlat: c_uint = 30;

#[no_mangle]
pub static mut perf_mem_events: [perf_mem_event; 3] = [
    perf_mem_event {
        tag: b"ldlat-loads\0".as_ptr() as *const c_char,
        name: b"%s/mem-loads,ldlat=%u/P\0".as_ptr() as *const c_char,
        event_name: b"mem-loads\0".as_ptr() as *const c_char,
        ldlat: true,
        aux_event: 0,
        supported: false,
    },
    perf_mem_event {
        tag: b"ldlat-stores\0".as_ptr() as *const c_char,
        name: b"%s/mem-stores/P\0".as_ptr() as *const c_char,
        event_name: b"mem-stores\0".as_ptr() as *const c_char,
        ldlat: false,
        aux_event: 0,
        supported: false,
    },
    perf_mem_event {
        tag: ptr::null(),
        name: ptr::null(),
        event_name: ptr::null(),
        ldlat: false,
        aux_event: 0,
        supported: false,
    },
];

#[no_mangle]
pub static mut perf_mem_record: [bool; 3] = [false; 3];

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__mem_events_ptr(
    pmu: *mut perf_pmu,
    i: c_int,
) -> *mut perf_mem_event {
    if i >= PERF_MEM_EVENTS__MAX || pmu.is_null() {
        return ptr::null_mut();
    }

    (*pmu).mem_events.add(i as usize)
}

unsafe fn perf_pmus__scan_mem(mut pmu: *mut perf_pmu) -> *mut perf_pmu {
    loop {
        pmu = perf_pmus__scan(pmu);
        if pmu.is_null() {
            break;
        }
        if !(*pmu).mem_events.is_null() {
            return pmu;
        }
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn perf_mem_events_find_pmu() -> *mut perf_pmu {
    /*
     * The current perf mem doesn't support per-PMU configuration.
     * The exact same configuration is applied to all the
     * mem_events supported PMUs.
     * Return the first mem_events supported PMU.
     *
     * Notes: The only case which may support multiple mem_events
     * supported PMUs is Intel hybrid. The exact same mem_events
     * is shared among the PMUs. Only configure the first PMU
     * is good enough as well.
     */
    perf_pmus__scan_mem(ptr::null_mut())
}

/**
 * perf_pmu__mem_events_num_mem_pmus - Get the number of mem PMUs since the given pmu
 * @pmu: Start pmu. If it's NULL, search the entire PMU list.
 */
#[no_mangle]
pub unsafe extern "C" fn perf_pmu__mem_events_num_mem_pmus(mut pmu: *mut perf_pmu) -> c_int {
    let mut num: c_int = 0;

    loop {
        pmu = perf_pmus__scan_mem(pmu);
        if pmu.is_null() {
            break;
        }
        num += 1;
    }

    num
}

unsafe fn perf_pmu__mem_events_name(
    pmu: *mut perf_pmu,
    i: c_int,
    buf: *mut c_char,
    buf_size: size_t,
) -> *const c_char {
    let e: *mut perf_mem_event;

    if i >= PERF_MEM_EVENTS__MAX || pmu.is_null() {
        return ptr::null();
    }

    e = (*pmu).mem_events.add(i as usize);
    if e.is_null() || (*e).name.is_null() {
        return ptr::null();
    }

    if i == PERF_MEM_EVENTS__LOAD || i == PERF_MEM_EVENTS__LOAD_STORE {
        if (*e).ldlat {
            if (*e).aux_event == 0 {
                /* ARM and Most of Intel */
                scnprintf(
                    buf,
                    buf_size,
                    (*e).name,
                    (*pmu).name,
                    perf_mem_events__loads_ldlat,
                );
            } else {
                /* Intel with mem-loads-aux event */
                scnprintf(
                    buf,
                    buf_size,
                    (*e).name,
                    (*pmu).name,
                    (*pmu).name,
                    perf_mem_events__loads_ldlat,
                );
            }
        } else {
            if (*e).aux_event == 0 {
                /* AMD and POWER */
                scnprintf(buf, buf_size, (*e).name, (*pmu).name);
            } else {
                return ptr::null();
            }
        }
        return buf;
    }

    if i == PERF_MEM_EVENTS__STORE {
        scnprintf(buf, buf_size, (*e).name, (*pmu).name);
        return buf;
    }

    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn is_mem_loads_aux_event(leader: *mut evsel) -> bool {
    let pmu = (*leader).pmu;
    let e: *mut perf_mem_event;

    if pmu.is_null() || (*pmu).mem_events.is_null() {
        return false;
    }

    e = (*pmu).mem_events.add(PERF_MEM_EVENTS__LOAD as usize);
    if (*e).aux_event == 0 {
        return false;
    }

    (*leader).core.attr.config == (*e).aux_event
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__mem_events_parse(
    pmu: *mut perf_pmu,
    str_: *const c_char,
) -> c_int {
    let mut tok: *mut c_char;
    let mut saveptr: *mut c_char = ptr::null_mut();
    let mut found = false;
    let buf: *mut c_char;
    let mut j: c_int;

    /* We need buffer that we know we can write to. */
    buf = malloc(strlen(str_) + 1) as *mut c_char;
    if buf.is_null() {
        return -ENOMEM;
    }

    strcpy(buf, str_);

    tok = strtok_r(buf, b",\0".as_ptr() as *const c_char, &mut saveptr);

    while !tok.is_null() {
        j = 0;
        while j < PERF_MEM_EVENTS__MAX {
            let e = perf_pmu__mem_events_ptr(pmu, j);

            if (*e).tag.is_null() {
                j += 1;
                continue;
            }

            if !strstr((*e).tag, tok).is_null() {
                perf_mem_record[j as usize] = true;
                found = true;
            }
            j += 1;
        }

        tok = strtok_r(ptr::null_mut(), b",\0".as_ptr() as *const c_char, &mut saveptr);
    }

    free(buf as *mut core::ffi::c_void);

    if found {
        return 0;
    }

    pr_err(
        b"failed: event '%s' not found, use '-e list' to get list of available events\n\0"
            .as_ptr() as *const c_char,
        str_,
    );
    -1
}

unsafe fn perf_pmu__mem_events_supported(
    mnt: *const c_char,
    pmu: *mut perf_pmu,
    e: *mut perf_mem_event,
) -> bool {
    let mut path = [0 as c_char; PATH_MAX];
    let mut st = MaybeUninit::<stat>::uninit();

    if (*e).event_name.is_null() {
        return true;
    }

    scnprintf(
        path.as_mut_ptr(),
        PATH_MAX,
        b"%s/bus/event_source/devices/%s/events/%s\0".as_ptr() as *const c_char,
        mnt,
        (*pmu).name,
        (*e).event_name,
    );

    stat(path.as_ptr(), st.as_mut_ptr()) == 0
}

unsafe fn __perf_pmu__mem_events_init(pmu: *mut perf_pmu) -> c_int {
    let mnt = sysfs__mount();
    let mut found = false;
    let mut j: c_int;

    if mnt.is_null() {
        return -ENOENT;
    }

    j = 0;
    while j < PERF_MEM_EVENTS__MAX {
        let e = perf_pmu__mem_events_ptr(pmu, j);

        /*
         * If the event entry isn't valid, skip initialization
         * and "e->supported" will keep false.
         */
        if (*e).tag.is_null() {
            j += 1;
            continue;
        }

        (*e).supported |= perf_pmu__mem_events_supported(mnt, pmu, e);
        if (*e).supported {
            found = true;
        }
        j += 1;
    }

    if found { 0 } else { -ENOENT }
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__mem_events_init() -> c_int {
    let mut pmu: *mut perf_pmu = ptr::null_mut();

    loop {
        pmu = perf_pmus__scan_mem(pmu);
        if pmu.is_null() {
            break;
        }
        if __perf_pmu__mem_events_init(pmu) != 0 {
            return -ENOENT;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__mem_events_list(pmu: *mut perf_pmu) {
    let mut j: c_int = 0;

    while j < PERF_MEM_EVENTS__MAX {
        let mut buf = [0 as c_char; 128];
        let e = perf_pmu__mem_events_ptr(pmu, j);

        fprintf(
            stderr,
            b"%-*s%-*s%s\0".as_ptr() as *const c_char,
            if !(*e).tag.is_null() { 13 } else { 0 },
            if !(*e).tag.is_null() {
                (*e).tag
            } else {
                b"\0".as_ptr() as *const c_char
            },
            if !(*e).tag.is_null() && verbose > 0 { 25 } else { 0 },
            if !(*e).tag.is_null() && verbose > 0 {
                perf_pmu__mem_events_name(pmu, j, buf.as_mut_ptr(), buf.len())
            } else {
                b"\0".as_ptr() as *const c_char
            },
            if (*e).supported {
                b": available\n\0".as_ptr() as *const c_char
            } else {
                b"\0".as_ptr() as *const c_char
            },
        );
        j += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_mem_events__record_args(
    rec_argv: *mut *const c_char,
    argv_nr: *mut c_int,
    event_name_storage_out: *mut *mut c_char,
) -> c_int {
    let mnt = sysfs__mount();
    let mut pmu: *mut perf_pmu = ptr::null_mut();
    let mut i = *argv_nr;
    let mut cpu_map: *mut perf_cpu_map = ptr::null_mut();
    let event_name_storage_size =
        perf_pmu__mem_events_num_mem_pmus(ptr::null_mut()) as size_t * PERF_MEM_EVENTS__MAX as size_t * 128;
    let mut event_name_storage_remaining = event_name_storage_size;
    let event_name_storage = malloc(event_name_storage_size) as *mut c_char;
    let mut event_name_storage_ptr = event_name_storage;

    if event_name_storage.is_null() {
        return -ENOMEM;
    }

    *event_name_storage_out = ptr::null_mut();
    loop {
        pmu = perf_pmus__scan_mem(pmu);
        if pmu.is_null() {
            break;
        }
        let mut j: c_int = 0;
        while j < PERF_MEM_EVENTS__MAX {
            let s: *const c_char;
            let e = perf_pmu__mem_events_ptr(pmu, j);
            let ret: c_int;

            if !perf_mem_record[j as usize] {
                j += 1;
                continue;
            }

            if !(*e).supported {
                let mut buf = [0 as c_char; 128];

                pr_err(
                    b"failed: event '%s' not supported\n\0".as_ptr() as *const c_char,
                    perf_pmu__mem_events_name(pmu, j, buf.as_mut_ptr(), buf.len()),
                );
                free(event_name_storage as *mut core::ffi::c_void);
                return -1;
            }

            s = perf_pmu__mem_events_name(
                pmu,
                j,
                event_name_storage_ptr,
                event_name_storage_remaining,
            );
            if s.is_null() || !perf_pmu__mem_events_supported(mnt, pmu, e) {
                j += 1;
                continue;
            }

            *rec_argv.add(i as usize) = b"-e\0".as_ptr() as *const c_char;
            i += 1;
            *rec_argv.add(i as usize) = event_name_storage_ptr;
            i += 1;
            event_name_storage_remaining -= strlen(event_name_storage_ptr) + 1;
            event_name_storage_ptr =
                event_name_storage_ptr.add(strlen(event_name_storage_ptr) + 1);

            ret = perf_cpu_map__merge(&mut cpu_map, (*pmu).cpus);
            if ret < 0 {
                free(event_name_storage as *mut core::ffi::c_void);
                return ret;
            }
            j += 1;
        }
    }

    if !cpu_map.is_null() {
        let online = cpu_map__online();

        if !perf_cpu_map__equal(cpu_map, online) {
            let mut buf = [0 as c_char; 200];

            cpu_map__snprint(cpu_map, buf.as_mut_ptr(), buf.len());
            pr_warning(
                b"Memory events are enabled on a subset of CPUs: %s\n\0".as_ptr()
                    as *const c_char,
                buf.as_ptr(),
            );
        }
        perf_cpu_map__put(online);
        perf_cpu_map__put(cpu_map);
    }

    *argv_nr = i;
    *event_name_storage_out = event_name_storage;
    0
}

static tlb_access: [*const c_char; 7] = [
    b"N/A\0".as_ptr() as *const c_char,
    b"HIT\0".as_ptr() as *const c_char,
    b"MISS\0".as_ptr() as *const c_char,
    b"L1\0".as_ptr() as *const c_char,
    b"L2\0".as_ptr() as *const c_char,
    b"Walker\0".as_ptr() as *const c_char,
    b"Fault\0".as_ptr() as *const c_char,
];

#[no_mangle]
pub unsafe extern "C" fn perf_mem__tlb_scnprintf(
    out: *mut c_char,
    mut sz: size_t,
    mem_info: *const mem_info,
) -> c_int {
    let mut l: size_t = 0;
    let mut i: size_t;
    let mut m: u64 = PERF_MEM_TLB_NA;
    let hit: u64;
    let miss: u64;

    sz -= 1; /* -1 for null termination */
    *out = 0;

    if !mem_info.is_null() {
        m = (*mem_info__const_data_src(mem_info)).mem_dtlb();
    }

    hit = m & PERF_MEM_TLB_HIT;
    miss = m & PERF_MEM_TLB_MISS;

    /* already taken care of */
    m &= !(PERF_MEM_TLB_HIT | PERF_MEM_TLB_MISS);

    i = 0;
    while m != 0 && i < tlb_access.len() {
        if (m & 0x1) == 0 {
            i += 1;
            m >>= 1;
            continue;
        }
        if l != 0 {
            strcat(out, b" or \0".as_ptr() as *const c_char);
            l += 4;
        }
        l += scnprintf(out.add(l), sz - l, tlb_access[i]) as size_t;
        i += 1;
        m >>= 1;
    }
    if *out == 0 {
        l += scnprintf(out, sz - l, b"N/A\0".as_ptr() as *const c_char) as size_t;
    }
    if hit != 0 {
        l += scnprintf(out.add(l), sz - l, b" hit\0".as_ptr() as *const c_char) as size_t;
    }
    if miss != 0 {
        l += scnprintf(out.add(l), sz - l, b" miss\0".as_ptr() as *const c_char) as size_t;
    }

    l as c_int
}

static mem_lvl: [*const c_char; 14] = [
    b"N/A\0".as_ptr() as *const c_char,
    b"HIT\0".as_ptr() as *const c_char,
    b"MISS\0".as_ptr() as *const c_char,
    b"L1\0".as_ptr() as *const c_char,
    b"LFB/MAB\0".as_ptr() as *const c_char,
    b"L2\0".as_ptr() as *const c_char,
    b"L3\0".as_ptr() as *const c_char,
    b"Local RAM\0".as_ptr() as *const c_char,
    b"Remote RAM (1 hop)\0".as_ptr() as *const c_char,
    b"Remote RAM (2 hops)\0".as_ptr() as *const c_char,
    b"Remote Cache (1 hop)\0".as_ptr() as *const c_char,
    b"Remote Cache (2 hops)\0".as_ptr() as *const c_char,
    b"I/O\0".as_ptr() as *const c_char,
    b"Uncached\0".as_ptr() as *const c_char,
];

static mut mem_lvlnum: [*const c_char; 64] = [ptr::null(); 64];

unsafe fn mem_lvlnum_init() {
    mem_lvlnum[PERF_MEM_LVLNUM_L1 as usize] = b"L1\0".as_ptr() as *const c_char;
    mem_lvlnum[PERF_MEM_LVLNUM_L2 as usize] = b"L2\0".as_ptr() as *const c_char;
    mem_lvlnum[PERF_MEM_LVLNUM_L3 as usize] = b"L3\0".as_ptr() as *const c_char;
    mem_lvlnum[PERF_MEM_LVLNUM_L4 as usize] = b"L4\0".as_ptr() as *const c_char;
    mem_lvlnum[PERF_MEM_LVLNUM_L2_MHB as usize] = b"L2 MHB\0".as_ptr() as *const c_char;
    mem_lvlnum[PERF_MEM_LVLNUM_MSC as usize] = b"Memory-side Cache\0".as_ptr() as *const c_char;
    mem_lvlnum[PERF_MEM_LVLNUM_UNC as usize] = b"Uncached\0".as_ptr() as *const c_char;
    mem_lvlnum[PERF_MEM_LVLNUM_CXL as usize] = b"CXL\0".as_ptr() as *const c_char;
    mem_lvlnum[PERF_MEM_LVLNUM_IO as usize] = b"I/O\0".as_ptr() as *const c_char;
    mem_lvlnum[PERF_MEM_LVLNUM_ANY_CACHE as usize] = b"Any cache\0".as_ptr() as *const c_char;
    mem_lvlnum[PERF_MEM_LVLNUM_LFB as usize] = b"LFB/MAB\0".as_ptr() as *const c_char;
    mem_lvlnum[PERF_MEM_LVLNUM_RAM as usize] = b"RAM\0".as_ptr() as *const c_char;
    mem_lvlnum[PERF_MEM_LVLNUM_PMEM as usize] = b"PMEM\0".as_ptr() as *const c_char;
    mem_lvlnum[PERF_MEM_LVLNUM_NA as usize] = b"N/A\0".as_ptr() as *const c_char;
}

static mem_hops: [*const c_char; 5] = [
    b"N/A\0".as_ptr() as *const c_char,
    /*
     * While printing, 'Remote' will be added to represent
     * 'Remote core, same node' accesses as remote field need
     * to be set with mem_hops field.
     */
    b"core, same node\0".as_ptr() as *const c_char,
    b"node, same socket\0".as_ptr() as *const c_char,
    b"socket, same board\0".as_ptr() as *const c_char,
    b"board\0".as_ptr() as *const c_char,
];

unsafe fn perf_mem__op_scnprintf(
    out: *mut c_char,
    sz: size_t,
    mem_info: *const mem_info,
) -> c_int {
    let mut op: u64 = PERF_MEM_LOCK_NA;
    let l: c_int;

    if !mem_info.is_null() {
        op = (*mem_info__const_data_src(mem_info)).mem_op();
    }

    if op & PERF_MEM_OP_NA != 0 {
        l = scnprintf(out, sz, b"N/A\0".as_ptr() as *const c_char);
    } else if op & PERF_MEM_OP_LOAD != 0 {
        l = scnprintf(out, sz, b"LOAD\0".as_ptr() as *const c_char);
    } else if op & PERF_MEM_OP_STORE != 0 {
        l = scnprintf(out, sz, b"STORE\0".as_ptr() as *const c_char);
    } else if op & PERF_MEM_OP_PFETCH != 0 {
        l = scnprintf(out, sz, b"PFETCH\0".as_ptr() as *const c_char);
    } else if op & PERF_MEM_OP_EXEC != 0 {
        l = scnprintf(out, sz, b"EXEC\0".as_ptr() as *const c_char);
    } else {
        l = scnprintf(out, sz, b"No\0".as_ptr() as *const c_char);
    }

    l
}

#[no_mangle]
pub unsafe extern "C" fn perf_mem__lvl_scnprintf(
    out: *mut c_char,
    mut sz: size_t,
    mem_info: *const mem_info,
) -> c_int {
    let data_src: perf_mem_data_src;
    let mut printed: c_int = 0;
    let mut l: size_t = 0;
    let mut i: size_t;
    let mut lvl: c_int;
    let mut hit_miss = [0 as c_char; 5];

    sz -= 1; /* -1 for null termination */
    *out = 0;

    if mem_info.is_null() {
        strcat(out, b"N/A\0".as_ptr() as *const c_char);
        return 3;
    }

    data_src = *mem_info__const_data_src(mem_info);

    if data_src.mem_lvl() & PERF_MEM_LVL_HIT != 0 {
        memcpy(
            hit_miss.as_mut_ptr() as *mut core::ffi::c_void,
            b"hit\0".as_ptr() as *const core::ffi::c_void,
            3,
        );
    } else if data_src.mem_lvl() & PERF_MEM_LVL_MISS != 0 {
        memcpy(
            hit_miss.as_mut_ptr() as *mut core::ffi::c_void,
            b"miss\0".as_ptr() as *const core::ffi::c_void,
            4,
        );
    }

    lvl = data_src.mem_lvl_num();
    if lvl != 0 && lvl != PERF_MEM_LVLNUM_NA {
        mem_lvlnum_init();
        if data_src.mem_remote() != 0 {
            strcat(out, b"Remote \0".as_ptr() as *const c_char);
            l += 7;
        }

        if data_src.mem_hops() != 0 {
            l += scnprintf(
                out.add(l),
                sz - l,
                b"%s \0".as_ptr() as *const c_char,
                mem_hops[data_src.mem_hops()],
            ) as size_t;
        }

        if !mem_lvlnum[lvl as usize].is_null() {
            l += scnprintf(out.add(l), sz - l, mem_lvlnum[lvl as usize]) as size_t;
        } else {
            l += scnprintf(
                out.add(l),
                sz - l,
                b"Unknown level %d\0".as_ptr() as *const c_char,
                lvl,
            ) as size_t;
        }

        l += scnprintf(
            out.add(l),
            sz - l,
            b" %s\0".as_ptr() as *const c_char,
            hit_miss.as_ptr(),
        ) as size_t;
        return l as c_int;
    }

    lvl = data_src.mem_lvl() as c_int;
    if lvl == 0 {
        strcat(out, b"N/A\0".as_ptr() as *const c_char);
        return 3;
    }

    lvl &= !(PERF_MEM_LVL_NA | PERF_MEM_LVL_HIT | PERF_MEM_LVL_MISS) as c_int;
    if lvl == 0 {
        strcat(out, b"N/A\0".as_ptr() as *const c_char);
        return 3;
    }

    i = 0;
    while lvl != 0 && i < mem_lvl.len() {
        if (lvl & 0x1) == 0 {
            i += 1;
            lvl >>= 1;
            continue;
        }
        if printed != 0 {
            strcat(out, b" or \0".as_ptr() as *const c_char);
            l += 4;
        }
        printed += 1;
        l += scnprintf(out.add(l), sz - l, mem_lvl[i]) as size_t;
        i += 1;
        lvl >>= 1;
    }

    if printed != 0 {
        l += scnprintf(
            out.add(l),
            sz - l,
            b" %s\0".as_ptr() as *const c_char,
            hit_miss.as_ptr(),
        ) as size_t;
        return l as c_int;
    }

    strcat(out, b"N/A\0".as_ptr() as *const c_char);
    3
}

static snoop_access: [*const c_char; 5] = [
    b"N/A\0".as_ptr() as *const c_char,
    b"None\0".as_ptr() as *const c_char,
    b"Hit\0".as_ptr() as *const c_char,
    b"Miss\0".as_ptr() as *const c_char,
    b"HitM\0".as_ptr() as *const c_char,
];

static snoopx_access: [*const c_char; 2] = [
    b"Fwd\0".as_ptr() as *const c_char,
    b"Peer\0".as_ptr() as *const c_char,
];

#[no_mangle]
pub unsafe extern "C" fn perf_mem__snp_scnprintf(
    out: *mut c_char,
    mut sz: size_t,
    mem_info: *const mem_info,
) -> c_int {
    let mut i: size_t;
    let mut l: size_t = 0;
    let mut m: u64 = PERF_MEM_SNOOP_NA;

    sz -= 1; /* -1 for null termination */
    *out = 0;

    if !mem_info.is_null() {
        m = (*mem_info__const_data_src(mem_info)).mem_snoop();
    }

    i = 0;
    while m != 0 && i < snoop_access.len() {
        if (m & 0x1) == 0 {
            i += 1;
            m >>= 1;
            continue;
        }
        if l != 0 {
            strcat(out, b" or \0".as_ptr() as *const c_char);
            l += 4;
        }
        l += scnprintf(out.add(l), sz - l, snoop_access[i]) as size_t;
        i += 1;
        m >>= 1;
    }

    m = 0;
    if !mem_info.is_null() {
        m = (*mem_info__const_data_src(mem_info)).mem_snoopx();
    }

    i = 0;
    while m != 0 && i < snoopx_access.len() {
        if (m & 0x1) == 0 {
            i += 1;
            m >>= 1;
            continue;
        }

        if l != 0 {
            strcat(out, b" or \0".as_ptr() as *const c_char);
            l += 4;
        }
        l += scnprintf(out.add(l), sz - l, snoopx_access[i]) as size_t;
        i += 1;
        m >>= 1;
    }

    if *out == 0 {
        l += scnprintf(out, sz - l, b"N/A\0".as_ptr() as *const c_char) as size_t;
    }

    l as c_int
}

#[no_mangle]
pub unsafe extern "C" fn perf_mem__lck_scnprintf(
    out: *mut c_char,
    sz: size_t,
    mem_info: *const mem_info,
) -> c_int {
    let mut mask: u64 = PERF_MEM_LOCK_NA;
    let l: c_int;

    if !mem_info.is_null() {
        mask = (*mem_info__const_data_src(mem_info)).mem_lock();
    }

    if mask & PERF_MEM_LOCK_NA != 0 {
        l = scnprintf(out, sz, b"N/A\0".as_ptr() as *const c_char);
    } else if mask & PERF_MEM_LOCK_LOCKED != 0 {
        l = scnprintf(out, sz, b"Yes\0".as_ptr() as *const c_char);
    } else {
        l = scnprintf(out, sz, b"No\0".as_ptr() as *const c_char);
    }

    l
}

#[no_mangle]
pub unsafe extern "C" fn perf_mem__blk_scnprintf(
    out: *mut c_char,
    mut sz: size_t,
    mem_info: *const mem_info,
) -> c_int {
    let mut l: size_t = 0;
    let mut mask: u64 = PERF_MEM_BLK_NA;

    sz -= 1; /* -1 for null termination */
    *out = 0;

    if !mem_info.is_null() {
        mask = (*mem_info__const_data_src(mem_info)).mem_blk();
    }

    if mask == 0 || (mask & PERF_MEM_BLK_NA) != 0 {
        l += scnprintf(out.add(l), sz - l, b" N/A\0".as_ptr() as *const c_char) as size_t;
        return l as c_int;
    }
    if mask & PERF_MEM_BLK_DATA != 0 {
        l += scnprintf(out.add(l), sz - l, b" Data\0".as_ptr() as *const c_char) as size_t;
    }
    if mask & PERF_MEM_BLK_ADDR != 0 {
        l += scnprintf(out.add(l), sz - l, b" Addr\0".as_ptr() as *const c_char) as size_t;
    }

    l as c_int
}

#[no_mangle]
pub unsafe extern "C" fn perf_script__meminfo_scnprintf(
    out: *mut c_char,
    sz: size_t,
    mem_info: *const mem_info,
) -> c_int {
    let mut i: c_int = 0;

    i += scnprintf(out, sz, b"|OP \0".as_ptr() as *const c_char);
    i += perf_mem__op_scnprintf(out.add(i as usize), sz - i as usize, mem_info);
    i += scnprintf(
        out.add(i as usize),
        sz - i as usize,
        b"|LVL \0".as_ptr() as *const c_char,
    );
    i += perf_mem__lvl_scnprintf(out.add(i as usize), sz, mem_info);
    i += scnprintf(
        out.add(i as usize),
        sz - i as usize,
        b"|SNP \0".as_ptr() as *const c_char,
    );
    i += perf_mem__snp_scnprintf(out.add(i as usize), sz - i as usize, mem_info);
    i += scnprintf(
        out.add(i as usize),
        sz - i as usize,
        b"|TLB \0".as_ptr() as *const c_char,
    );
    i += perf_mem__tlb_scnprintf(out.add(i as usize), sz - i as usize, mem_info);
    i += scnprintf(
        out.add(i as usize),
        sz - i as usize,
        b"|LCK \0".as_ptr() as *const c_char,
    );
    i += perf_mem__lck_scnprintf(out.add(i as usize), sz - i as usize, mem_info);
    i += scnprintf(
        out.add(i as usize),
        sz - i as usize,
        b"|BLK \0".as_ptr() as *const c_char,
    );
    i += perf_mem__blk_scnprintf(out.add(i as usize), sz - i as usize, mem_info);

    i
}

#[no_mangle]
pub unsafe extern "C" fn c2c_decode_stats(stats: *mut c2c_stats, mi: *mut mem_info) -> c_int {
    let data_src = mem_info__data_src(mi);
    let daddr = (*mem_info__daddr(mi)).addr;
    let op = (*data_src).mem_op();
    let lvl = (*data_src).mem_lvl();
    let snoop = (*data_src).mem_snoop();
    let snoopx = (*data_src).mem_snoopx();
    let lock = (*data_src).mem_lock();
    let blk = (*data_src).mem_blk();
    /*
     * Skylake might report unknown remote level via this
     * bit, consider it when evaluating remote HITMs.
     *
     * Incase of power, remote field can also be used to denote cache
     * accesses from the another core of same node. Hence, setting
     * mrem only when HOPS is zero along with set remote field.
     */
    let mrem = (*data_src).mem_remote() != 0 && (*data_src).mem_hops() == 0;
    let err: c_int = 0;

    (*stats).nr_entries += 1;

    if lock & PERF_MEM_LOCK_LOCKED != 0 {
        (*stats).locks += 1;
    }

    if blk & PERF_MEM_BLK_DATA != 0 {
        (*stats).blk_data += 1;
    }
    if blk & PERF_MEM_BLK_ADDR != 0 {
        (*stats).blk_addr += 1;
    }

    if op & PERF_MEM_OP_LOAD != 0 {
        /* load */
        (*stats).load += 1;

        if daddr == 0 {
            (*stats).ld_noadrs += 1;
            return -1;
        }

        if lvl & PERF_MEM_LVL_HIT != 0 {
            if lvl & PERF_MEM_LVL_UNC != 0 {
                (*stats).ld_uncache += 1;
            }
            if lvl & PERF_MEM_LVL_IO != 0 {
                (*stats).ld_io += 1;
            }
            if lvl & PERF_MEM_LVL_LFB != 0 {
                (*stats).ld_fbhit += 1;
            }
            if lvl & PERF_MEM_LVL_L1 != 0 {
                (*stats).ld_l1hit += 1;
            }
            if lvl & PERF_MEM_LVL_L2 != 0 {
                if snoop & PERF_MEM_SNOOP_HITM != 0 {
                    (*stats).lcl_hitm += 1;
                    (*stats).tot_hitm += 1;
                } else {
                    (*stats).ld_l2hit += 1;
                }

                if snoopx & PERF_MEM_SNOOPX_PEER != 0 {
                    (*stats).lcl_peer += 1;
                    (*stats).tot_peer += 1;
                }
            }
            if lvl & PERF_MEM_LVL_L3 != 0 {
                if snoop & PERF_MEM_SNOOP_HITM != 0 {
                    (*stats).lcl_hitm += 1;
                    (*stats).tot_hitm += 1;
                } else {
                    (*stats).ld_llchit += 1;
                }

                if snoopx & PERF_MEM_SNOOPX_PEER != 0 {
                    (*stats).lcl_peer += 1;
                    (*stats).tot_peer += 1;
                }
            }

            if lvl & PERF_MEM_LVL_LOC_RAM != 0 {
                (*stats).lcl_dram += 1;
                if snoop & PERF_MEM_SNOOP_HIT != 0 {
                    (*stats).ld_shared += 1;
                } else {
                    (*stats).ld_excl += 1;
                }
            }

            if (lvl & PERF_MEM_LVL_REM_RAM1 != 0) || (lvl & PERF_MEM_LVL_REM_RAM2 != 0) || mrem {
                (*stats).rmt_dram += 1;
                if snoop & PERF_MEM_SNOOP_HIT != 0 {
                    (*stats).ld_shared += 1;
                } else {
                    (*stats).ld_excl += 1;
                }
            }
        }

        if (lvl & PERF_MEM_LVL_REM_CCE1 != 0) || (lvl & PERF_MEM_LVL_REM_CCE2 != 0) || mrem {
            if snoop & PERF_MEM_SNOOP_HIT != 0 {
                (*stats).rmt_hit += 1;
            } else if snoop & PERF_MEM_SNOOP_HITM != 0 {
                (*stats).rmt_hitm += 1;
                (*stats).tot_hitm += 1;
            } else if snoopx & PERF_MEM_SNOOPX_PEER != 0 {
                (*stats).rmt_hit += 1;
                (*stats).rmt_peer += 1;
                (*stats).tot_peer += 1;
            }
        }

        if lvl & PERF_MEM_LVL_MISS != 0 {
            (*stats).ld_miss += 1;
        }
    } else if op & PERF_MEM_OP_STORE != 0 {
        /* store */
        (*stats).store += 1;

        if daddr == 0 {
            (*stats).st_noadrs += 1;
            return -1;
        }

        if lvl & PERF_MEM_LVL_HIT != 0 {
            if lvl & PERF_MEM_LVL_UNC != 0 {
                (*stats).st_uncache += 1;
            }
            if lvl & PERF_MEM_LVL_L1 != 0 {
                (*stats).st_l1hit += 1;
            }
        }
        if lvl & PERF_MEM_LVL_MISS != 0 {
            if lvl & PERF_MEM_LVL_L1 != 0 {
                (*stats).st_l1miss += 1;
            }
        }
        if lvl & PERF_MEM_LVL_NA != 0 {
            (*stats).st_na += 1;
        }
    } else {
        /* unparsable data_src? */
        (*stats).noparse += 1;
        return -1;
    }

    if (*mem_info__daddr(mi)).ms.map.is_null() || (*mem_info__iaddr(mi)).ms.map.is_null() {
        (*stats).nomap += 1;
        return -1;
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn c2c_add_stats(stats: *mut c2c_stats, add: *mut c2c_stats) {
    (*stats).nr_entries += (*add).nr_entries;

    (*stats).locks += (*add).locks;
    (*stats).store += (*add).store;
    (*stats).st_uncache += (*add).st_uncache;
    (*stats).st_noadrs += (*add).st_noadrs;
    (*stats).st_l1hit += (*add).st_l1hit;
    (*stats).st_l1miss += (*add).st_l1miss;
    (*stats).st_na += (*add).st_na;
    (*stats).load += (*add).load;
    (*stats).ld_excl += (*add).ld_excl;
    (*stats).ld_shared += (*add).ld_shared;
    (*stats).ld_uncache += (*add).ld_uncache;
    (*stats).ld_io += (*add).ld_io;
    (*stats).ld_miss += (*add).ld_miss;
    (*stats).ld_noadrs += (*add).ld_noadrs;
    (*stats).ld_fbhit += (*add).ld_fbhit;
    (*stats).ld_l1hit += (*add).ld_l1hit;
    (*stats).ld_l2hit += (*add).ld_l2hit;
    (*stats).ld_llchit += (*add).ld_llchit;
    (*stats).lcl_hitm += (*add).lcl_hitm;
    (*stats).rmt_hitm += (*add).rmt_hitm;
    (*stats).tot_hitm += (*add).tot_hitm;
    (*stats).lcl_peer += (*add).lcl_peer;
    (*stats).rmt_peer += (*add).rmt_peer;
    (*stats).tot_peer += (*add).tot_peer;
    (*stats).rmt_hit += (*add).rmt_hit;
    (*stats).lcl_dram += (*add).lcl_dram;
    (*stats).rmt_dram += (*add).rmt_dram;
    (*stats).blk_data += (*add).blk_data;
    (*stats).blk_addr += (*add).blk_addr;
    (*stats).nomap += (*add).nomap;
    (*stats).noparse += (*add).noparse;
}

/*
 * It returns an index in hist_entry->mem_stat array for the given val which
 * represents a data-src based on the mem_stat_type.
 */
#[no_mangle]
pub unsafe extern "C" fn mem_stat_index(mst: mem_stat_type, val: u64) -> c_int {
    let src = perf_mem_data_src { val };

    if mst == PERF_MEM_STAT_OP {
        let mem_op = src.mem_op();
        if mem_op == PERF_MEM_OP_LOAD {
            return MEM_STAT_OP_LOAD;
        }
        if mem_op == PERF_MEM_OP_STORE {
            return MEM_STAT_OP_STORE;
        }
        if mem_op == (PERF_MEM_OP_LOAD | PERF_MEM_OP_STORE) {
            return MEM_STAT_OP_LDST;
        }
        if src.mem_op() & PERF_MEM_OP_PFETCH != 0 {
            return MEM_STAT_OP_PFETCH;
        }
        if src.mem_op() & PERF_MEM_OP_EXEC != 0 {
            return MEM_STAT_OP_EXEC;
        }
        return MEM_STAT_OP_OTHER;
    } else if mst == PERF_MEM_STAT_CACHE {
        let mem_lvl_num = src.mem_lvl_num();
        if mem_lvl_num == PERF_MEM_LVLNUM_L1 {
            return MEM_STAT_CACHE_L1;
        }
        if mem_lvl_num == PERF_MEM_LVLNUM_L2 {
            return MEM_STAT_CACHE_L2;
        }
        if mem_lvl_num == PERF_MEM_LVLNUM_L3 {
            return MEM_STAT_CACHE_L3;
        }
        if mem_lvl_num == PERF_MEM_LVLNUM_L4 {
            return MEM_STAT_CACHE_L4;
        }
        if mem_lvl_num == PERF_MEM_LVLNUM_LFB {
            return MEM_STAT_CACHE_L1_BUF;
        }
        if mem_lvl_num == PERF_MEM_LVLNUM_L2_MHB {
            return MEM_STAT_CACHE_L2_BUF;
        }
        return MEM_STAT_CACHE_OTHER;
    } else if mst == PERF_MEM_STAT_MEMORY {
        let mem_lvl_num = src.mem_lvl_num();
        if mem_lvl_num == PERF_MEM_LVLNUM_MSC {
            return MEM_STAT_MEMORY_MSC;
        }
        if mem_lvl_num == PERF_MEM_LVLNUM_RAM {
            return MEM_STAT_MEMORY_RAM;
        }
        if mem_lvl_num == PERF_MEM_LVLNUM_UNC {
            return MEM_STAT_MEMORY_UNC;
        }
        if mem_lvl_num == PERF_MEM_LVLNUM_CXL {
            return MEM_STAT_MEMORY_CXL;
        }
        if mem_lvl_num == PERF_MEM_LVLNUM_IO {
            return MEM_STAT_MEMORY_IO;
        }
        if mem_lvl_num == PERF_MEM_LVLNUM_PMEM {
            return MEM_STAT_MEMORY_PMEM;
        }
        return MEM_STAT_MEMORY_OTHER;
    } else if mst == PERF_MEM_STAT_SNOOP {
        let mem_snoop = src.mem_snoop();
        if mem_snoop == PERF_MEM_SNOOP_HIT {
            return MEM_STAT_SNOOP_HIT;
        }
        if mem_snoop == PERF_MEM_SNOOP_HITM {
            return MEM_STAT_SNOOP_HITM;
        }
        if mem_snoop == PERF_MEM_SNOOP_MISS {
            return MEM_STAT_SNOOP_MISS;
        }
        return MEM_STAT_SNOOP_OTHER;
    } else if mst == PERF_MEM_STAT_DTLB {
        let mem_dtlb = src.mem_dtlb();
        if mem_dtlb == (PERF_MEM_TLB_L1 | PERF_MEM_TLB_HIT) {
            return MEM_STAT_DTLB_L1_HIT;
        }
        if mem_dtlb == (PERF_MEM_TLB_L2 | PERF_MEM_TLB_HIT) {
            return MEM_STAT_DTLB_L2_HIT;
        }
        if mem_dtlb == (PERF_MEM_TLB_L1 | PERF_MEM_TLB_L2 | PERF_MEM_TLB_HIT) {
            return MEM_STAT_DTLB_ANY_HIT;
        }
        if src.mem_dtlb() & PERF_MEM_TLB_MISS != 0 {
            return MEM_STAT_DTLB_MISS;
        }
        return MEM_STAT_DTLB_OTHER;
    }
    -1
}

/* To align output, returned string should be shorter than MEM_STAT_PRINT_LEN */
#[no_mangle]
pub unsafe extern "C" fn mem_stat_name(mst: mem_stat_type, idx: c_int) -> *const c_char {
    if mst == PERF_MEM_STAT_OP {
        if idx == MEM_STAT_OP_LOAD {
            return b"Load\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_OP_STORE {
            return b"Store\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_OP_LDST {
            return b"Ld+St\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_OP_PFETCH {
            return b"Pfetch\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_OP_EXEC {
            return b"Exec\0".as_ptr() as *const c_char;
        }
        return b"Other\0".as_ptr() as *const c_char;
    } else if mst == PERF_MEM_STAT_CACHE {
        if idx == MEM_STAT_CACHE_L1 {
            return b"L1\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_CACHE_L2 {
            return b"L2\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_CACHE_L3 {
            return b"L3\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_CACHE_L4 {
            return b"L4\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_CACHE_L1_BUF {
            return b"L1-buf\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_CACHE_L2_BUF {
            return b"L2-buf\0".as_ptr() as *const c_char;
        }
        return b"Other\0".as_ptr() as *const c_char;
    } else if mst == PERF_MEM_STAT_MEMORY {
        if idx == MEM_STAT_MEMORY_RAM {
            return b"RAM\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_MEMORY_MSC {
            return b"MSC\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_MEMORY_UNC {
            return b"Uncach\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_MEMORY_CXL {
            return b"CXL\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_MEMORY_IO {
            return b"IO\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_MEMORY_PMEM {
            return b"PMEM\0".as_ptr() as *const c_char;
        }
        return b"Other\0".as_ptr() as *const c_char;
    } else if mst == PERF_MEM_STAT_SNOOP {
        if idx == MEM_STAT_SNOOP_HIT {
            return b"Hit\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_SNOOP_HITM {
            return b"HitM\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_SNOOP_MISS {
            return b"Miss\0".as_ptr() as *const c_char;
        }
        return b"Other\0".as_ptr() as *const c_char;
    } else if mst == PERF_MEM_STAT_DTLB {
        if idx == MEM_STAT_DTLB_L1_HIT {
            return b"L1-Hit\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_DTLB_L2_HIT {
            return b"L2-Hit\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_DTLB_ANY_HIT {
            return b"L?-Hit\0".as_ptr() as *const c_char;
        }
        if idx == MEM_STAT_DTLB_MISS {
            return b"Miss\0".as_ptr() as *const c_char;
        }
        return b"Other\0".as_ptr() as *const c_char;
    }
    b"N/A\0".as_ptr() as *const c_char
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
