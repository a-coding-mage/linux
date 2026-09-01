// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type bool_ = bool;
type u64 = u64;
type __u64 = u64;

const INVALID_START_TIME: __u64 = !0u64;
const PERF_PMU_TYPE_TOOL: c_int = 5;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const SEEK_SET: c_int = 0;
const O_RDONLY: c_int = 0;
const F_OK: c_int = 0;
const PATH_MAX: usize = 4096;
const _SC_CLK_TCK: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tool_pmu_event {
    TOOL_PMU__EVENT_NONE = 0,
    TOOL_PMU__EVENT_DURATION_TIME,
    TOOL_PMU__EVENT_USER_TIME,
    TOOL_PMU__EVENT_SYSTEM_TIME,
    TOOL_PMU__EVENT_HAS_PMEM,
    TOOL_PMU__EVENT_NUM_CORES,
    TOOL_PMU__EVENT_NUM_CPUS,
    TOOL_PMU__EVENT_NUM_CPUS_ONLINE,
    TOOL_PMU__EVENT_NUM_DIES,
    TOOL_PMU__EVENT_NUM_PACKAGES,
    TOOL_PMU__EVENT_SLOTS,
    TOOL_PMU__EVENT_SMT_ON,
    TOOL_PMU__EVENT_SYSTEM_TSC_FREQ,
    TOOL_PMU__EVENT_CORE_WIDE,
    TOOL_PMU__EVENT_TARGET_CPU,
    TOOL_PMU__EVENT_MAX,
}

const TOOL_PMU__EVENT_MAX_USIZE: usize = tool_pmu_event::TOOL_PMU__EVENT_MAX as usize;

static TOOL_PMU__EVENT_NAMES: [*const c_char; TOOL_PMU__EVENT_MAX_USIZE] = [
    core::ptr::null(),
    b"duration_time\0".as_ptr() as *const c_char,
    b"user_time\0".as_ptr() as *const c_char,
    b"system_time\0".as_ptr() as *const c_char,
    b"has_pmem\0".as_ptr() as *const c_char,
    b"num_cores\0".as_ptr() as *const c_char,
    b"num_cpus\0".as_ptr() as *const c_char,
    b"num_cpus_online\0".as_ptr() as *const c_char,
    b"num_dies\0".as_ptr() as *const c_char,
    b"num_packages\0".as_ptr() as *const c_char,
    b"slots\0".as_ptr() as *const c_char,
    b"smt_on\0".as_ptr() as *const c_char,
    b"system_tsc_freq\0".as_ptr() as *const c_char,
    b"core_wide\0".as_ptr() as *const c_char,
    b"target_cpu\0".as_ptr() as *const c_char,
];

#[repr(C)]
pub struct perf_pmu {
    pub type_: c_int,
    pub events_table: *mut c_void,
}

#[repr(C)]
pub struct perf_event_attr {
    pub config: u64,
    pub sample_period: u64,
    pub disabled: c_int,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
    pub fd: *mut xyarray,
    pub cpus: *mut perf_cpu_map,
    pub pmu_cpus: *mut perf_cpu_map,
    pub system_wide: bool_,
}

#[repr(C)]
pub struct duration_time {
    pub accumulated_time: __u64,
    pub start_time: __u64,
}

#[repr(C)]
pub struct process_time {
    pub start_times: *mut xyarray,
    pub accumulated_times: *mut xyarray,
}

#[repr(C)]
pub struct cgroup {
    pub fd: c_int,
}

#[repr(C)]
pub struct evsel {
    pub pmu: *mut perf_pmu,
    pub core: evsel_core,
    pub process_time: process_time,
    pub duration_time: duration_time,
    pub disabled: bool_,
    pub pid_stat: bool_,
    pub cgrp: *mut cgroup,
    pub counts: *mut perf_counts,
    pub prev_raw_counts: *mut perf_counts,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_thread_map {
    pub err_thread: c_int,
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
    pub lost: u64,
}

#[repr(C)]
pub struct cpu_topology {
    pub core_cpus_lists: u64,
    pub die_cpus_lists: u64,
    pub package_cpus_lists: u64,
}

#[repr(C)]
pub struct stat_config_t {
    pub system_wide: bool_,
    pub user_requested_cpu_list: *const c_char,
}

pub enum io {}
pub enum xyarray {}
pub enum perf_cpu_map {}
pub enum perf_counts {}

unsafe extern "C" {
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn lseek(fd: c_int, offset: c_long, whence: c_int) -> c_long;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn __errno_location() -> *mut c_int;

    fn io__init(io: *mut io, fd: c_int, buf: *mut c_char, size: usize);
    fn io__get_char(io: *mut io) -> c_int;
    fn io__get_dec(io: *mut io, val: *mut __u64) -> c_int;

    fn xyarray__new(xlen: c_int, ylen: c_int, entry_size: usize) -> *mut xyarray;
    fn xyarray__entry(xy: *mut xyarray, x: c_int, y: c_int) -> *mut c_void;
    fn xyarray__max_y(xy: *mut xyarray) -> c_int;

    fn perf_cpu_map__nr(cpus: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(cpus: *const perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__is_empty(cpus: *const perf_cpu_map) -> bool_;
    fn perf_cpu_map__has_any_cpu(cpus: *const perf_cpu_map) -> bool_;
    fn perf_cpu_map__intersect(a: *mut perf_cpu_map, b: *mut perf_cpu_map) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn cpu_map__online() -> *mut perf_cpu_map;
    fn cpu__max_present_cpu() -> perf_cpu;

    fn perf_thread_map__nr(threads: *const perf_thread_map) -> c_int;
    fn perf_thread_map__pid(threads: *const perf_thread_map, thread: c_int) -> c_int;

    fn rdclock() -> __u64;
    fn sysfs__mountpoint() -> *const c_char;
    fn online_topology() -> *const cpu_topology;
    fn smt_on() -> bool_;
    fn core_wide(system_wide: bool_, user_requested_cpu_list: *const c_char) -> bool_;
    fn perf_counts(counts: *mut perf_counts, cpu_map_idx: c_int, thread: c_int)
        -> *mut perf_counts_values;
    fn zalloc(size: usize) -> *mut c_void;
    fn perf_pmu__init(pmu: *mut perf_pmu, type_: c_int, name: *const c_char) -> c_int;
    fn perf_pmu__delete(pmu: *mut perf_pmu);
    fn find_core_events_table(arch: *const c_char, cpuid: *const c_char) -> *mut c_void;

    static mut stat_config: stat_config_t;
}

#[inline]
unsafe fn errno() -> c_int {
    *__errno_location()
}

#[inline]
unsafe fn set_errno(val: c_int) {
    *__errno_location() = val;
}

#[inline]
unsafe fn FD(e: *mut evsel, x: c_int, y: c_int) -> *mut c_int {
    xyarray__entry((*e).core.fd, x, y) as *mut c_int
}

#[inline]
unsafe fn tool_pmu_event_from_int(v: c_int) -> tool_pmu_event {
    core::mem::transmute::<c_int, tool_pmu_event>(v)
}

#[inline]
unsafe fn tool_pmu_event_from_u64(v: u64) -> tool_pmu_event {
    core::mem::transmute::<c_int, tool_pmu_event>(v as c_int)
}

#[no_mangle]
pub unsafe extern "C" fn tool_pmu__skip_event(name: *const c_char) -> bool_ {
    /* The slots event should only appear on arm64. */
    #[cfg(not(target_arch = "aarch64"))]
    {
        if strcasecmp(name, b"slots\0".as_ptr() as *const c_char) == 0 {
            return true;
        }
    }
    /* The system_tsc_freq event should only appear on x86. */
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        if strcasecmp(name, b"system_tsc_freq\0".as_ptr() as *const c_char) == 0 {
            return true;
        }
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn tool_pmu__num_skip_events() -> c_int {
    let mut num: c_int = 0;

    #[cfg(not(target_arch = "aarch64"))]
    {
        num += 1;
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        num += 1;
    }
    num
}

#[no_mangle]
pub unsafe extern "C" fn tool_pmu__event_to_str(ev: tool_pmu_event) -> *const c_char {
    let idx = ev as usize;

    if ev as c_int > tool_pmu_event::TOOL_PMU__EVENT_NONE as c_int
        && ev as c_int < tool_pmu_event::TOOL_PMU__EVENT_MAX as c_int
        && !tool_pmu__skip_event(TOOL_PMU__EVENT_NAMES[idx])
    {
        return TOOL_PMU__EVENT_NAMES[idx];
    }

    core::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn tool_pmu__str_to_event(str_: *const c_char) -> tool_pmu_event {
    let mut i: c_int;

    if tool_pmu__skip_event(str_) {
        return tool_pmu_event::TOOL_PMU__EVENT_NONE;
    }

    i = tool_pmu_event::TOOL_PMU__EVENT_NONE as c_int + 1;
    while i < tool_pmu_event::TOOL_PMU__EVENT_MAX as c_int {
        if strcasecmp(str_, TOOL_PMU__EVENT_NAMES[i as usize]) == 0 {
            return tool_pmu_event_from_int(i);
        }
        i += 1;
    }
    tool_pmu_event::TOOL_PMU__EVENT_NONE
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__is_tool(pmu: *const perf_pmu) -> bool_ {
    !pmu.is_null() && (*pmu).type_ == PERF_PMU_TYPE_TOOL
}

#[no_mangle]
pub unsafe extern "C" fn evsel__is_tool(evsel: *const evsel) -> bool_ {
    perf_pmu__is_tool((*evsel).pmu)
}

#[no_mangle]
pub unsafe extern "C" fn evsel__tool_event(evsel: *const evsel) -> tool_pmu_event {
    if !evsel__is_tool(evsel) {
        return tool_pmu_event::TOOL_PMU__EVENT_NONE;
    }

    tool_pmu_event_from_u64((*evsel).core.attr.config)
}

#[no_mangle]
pub unsafe extern "C" fn evsel__tool_pmu_event_name(evsel: *const evsel) -> *const c_char {
    tool_pmu__event_to_str(tool_pmu_event_from_u64((*evsel).core.attr.config))
}

unsafe fn read_until_char(io_: *mut io, e: c_char) -> bool_ {
    let mut c: c_int;

    loop {
        c = io__get_char(io_);
        if c == -1 {
            return false;
        }
        if c == e as c_int {
            break;
        }
    }
    true
}

unsafe fn read_stat_field(fd: c_int, cpu: perf_cpu, field: c_int, val: *mut __u64) -> c_int {
    let mut buf = [0 as c_char; 256];
    let mut io_: core::mem::MaybeUninit<io> = core::mem::MaybeUninit::uninit();
    let mut i: c_int;

    io__init(io_.as_mut_ptr(), fd, buf.as_mut_ptr(), buf.len());

    /* Skip lines to relevant CPU. */
    i = -1;
    while i < cpu.cpu {
        if !read_until_char(io_.as_mut_ptr(), b'\n' as c_char) {
            return -EINVAL;
        }
        i += 1;
    }
    /* Skip to "cpu". */
    if io__get_char(io_.as_mut_ptr()) != b'c' as c_int {
        return -EINVAL;
    }
    if io__get_char(io_.as_mut_ptr()) != b'p' as c_int {
        return -EINVAL;
    }
    if io__get_char(io_.as_mut_ptr()) != b'u' as c_int {
        return -EINVAL;
    }

    /* Skip N of cpuN. */
    if !read_until_char(io_.as_mut_ptr(), b' ' as c_char) {
        return -EINVAL;
    }

    i = 1;
    loop {
        if io__get_dec(io_.as_mut_ptr(), val) != b' ' as c_int {
            break;
        }
        if field == i {
            return 0;
        }
        i += 1;
    }
    -EINVAL
}

unsafe fn read_pid_stat_field(fd: c_int, field: c_int, val: *mut __u64) -> c_int {
    let mut buf = [0 as c_char; 256];
    let mut io_: core::mem::MaybeUninit<io> = core::mem::MaybeUninit::uninit();
    let mut c: c_int;
    let mut i: c_int;

    io__init(io_.as_mut_ptr(), fd, buf.as_mut_ptr(), buf.len());
    if io__get_dec(io_.as_mut_ptr(), val) != b' ' as c_int {
        return -EINVAL;
    }
    if field == 1 {
        return 0;
    }

    /* Skip comm. */
    if io__get_char(io_.as_mut_ptr()) != b'(' as c_int
        || !read_until_char(io_.as_mut_ptr(), b')' as c_char)
    {
        return -EINVAL;
    }
    if field == 2 {
        return -EINVAL; /* String can't be returned. */
    }

    /* Skip state */
    if io__get_char(io_.as_mut_ptr()) != b' ' as c_int || io__get_char(io_.as_mut_ptr()) == -1 {
        return -EINVAL;
    }
    if field == 3 {
        return -EINVAL; /* String can't be returned. */
    }

    /* Loop over numeric fields*/
    if io__get_char(io_.as_mut_ptr()) != b' ' as c_int {
        return -EINVAL;
    }

    i = 4;
    loop {
        c = io__get_dec(io_.as_mut_ptr(), val);
        if c == -1 {
            return -EINVAL;
        }
        if c == -2 {
            /* Assume a -ve was read */
            c = io__get_dec(io_.as_mut_ptr(), val);
            *val = (*val).wrapping_mul((-1i64) as u64);
        }
        if c != b' ' as c_int {
            return -EINVAL;
        }
        if field == i {
            return 0;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn evsel__tool_pmu_prepare_open(
    evsel: *mut evsel,
    cpus: *mut perf_cpu_map,
    nthreads: c_int,
) -> c_int {
    let ev = evsel__tool_event(evsel);

    if ev == tool_pmu_event::TOOL_PMU__EVENT_SYSTEM_TIME
        || ev == tool_pmu_event::TOOL_PMU__EVENT_USER_TIME
    {
        if (*evsel).process_time.start_times.is_null() {
            (*evsel).process_time.start_times =
                xyarray__new(perf_cpu_map__nr(cpus), nthreads, core::mem::size_of::<__u64>());
            if (*evsel).process_time.start_times.is_null() {
                return -ENOMEM;
            }
        }
        if (*evsel).process_time.accumulated_times.is_null() {
            (*evsel).process_time.accumulated_times =
                xyarray__new(perf_cpu_map__nr(cpus), nthreads, core::mem::size_of::<__u64>());
            if (*evsel).process_time.accumulated_times.is_null() {
                return -ENOMEM;
            }
        }
    }
    0
}

unsafe fn tool_pmu__read_stat(
    evsel: *mut evsel,
    cpu_map_idx: c_int,
    thread: c_int,
    val: *mut __u64,
) -> c_int {
    let ev = evsel__tool_event(evsel);
    let system = ev == tool_pmu_event::TOOL_PMU__EVENT_SYSTEM_TIME;
    let fd = *FD(evsel, cpu_map_idx, thread);
    let mut err = 0;

    if fd < 0 {
        *val = 0;
        return 0;
    }

    lseek(fd, 0, SEEK_SET as c_long);
    if (*evsel).pid_stat {
        if cpu_map_idx == 0 {
            err = read_pid_stat_field(fd, if system { 15 } else { 14 }, val);
        } else {
            *val = 0;
        }
    } else if thread == 0 {
        let cpu = perf_cpu_map__cpu((*evsel).core.cpus, cpu_map_idx);

        err = read_stat_field(fd, cpu, if system { 3 } else { 1 }, val);
    } else {
        *val = 0;
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn evsel__tool_pmu_open(
    evsel: *mut evsel,
    threads: *mut perf_thread_map,
    start_cpu_map_idx: c_int,
    end_cpu_map_idx: c_int,
) -> c_int {
    let ev = evsel__tool_event(evsel);
    let mut pid: c_int = -1;
    let mut idx: c_int = 0;
    let mut thread: c_int = 0;
    let nthreads: c_int;
    let mut err: c_int = 0;
    let old_errno: c_int;

    if ev == tool_pmu_event::TOOL_PMU__EVENT_NUM_CPUS {
        return 0;
    }

    if ev == tool_pmu_event::TOOL_PMU__EVENT_DURATION_TIME {
        if (*evsel).core.attr.sample_period != 0 {
            /* no sampling */
            return -EINVAL;
        }
        (*evsel).duration_time.accumulated_time = 0;
        if (*evsel).core.attr.disabled != 0 {
            (*evsel).disabled = true;
            (*evsel).duration_time.start_time = INVALID_START_TIME;
        } else {
            (*evsel).disabled = false;
            (*evsel).duration_time.start_time = rdclock();
        }
        return 0;
    }

    if !(*evsel).cgrp.is_null() {
        pid = (*(*evsel).cgrp).fd;
    }

    nthreads = perf_thread_map__nr(threads);
    idx = start_cpu_map_idx;
    while idx < end_cpu_map_idx {
        thread = 0;
        while thread < nthreads {
            if (*evsel).cgrp.is_null() && !(*evsel).core.system_wide {
                pid = perf_thread_map__pid(threads, thread);
            }

            if ev == tool_pmu_event::TOOL_PMU__EVENT_USER_TIME
                || ev == tool_pmu_event::TOOL_PMU__EVENT_SYSTEM_TIME
            {
                let start_time: *mut __u64;
                let accumulated_time: *mut __u64;
                let fd: c_int;

                if (*evsel).core.attr.sample_period != 0 {
                    /* no sampling */
                    err = -EINVAL;
                    break;
                }
                if pid > -1 {
                    let mut buf = [0 as c_char; 64];

                    snprintf(
                        buf.as_mut_ptr(),
                        buf.len(),
                        b"/proc/%d/stat\0".as_ptr() as *const c_char,
                        pid,
                    );
                    fd = open(buf.as_ptr(), O_RDONLY);
                    (*evsel).pid_stat = true;
                } else {
                    fd = open(b"/proc/stat\0".as_ptr() as *const c_char, O_RDONLY);
                }
                *FD(evsel, idx, thread) = fd;
                if fd < 0 {
                    err = -errno();
                    break;
                }
                start_time = xyarray__entry((*evsel).process_time.start_times, idx, thread)
                    as *mut __u64;
                accumulated_time =
                    xyarray__entry((*evsel).process_time.accumulated_times, idx, thread)
                        as *mut __u64;
                *accumulated_time = 0;

                if (*evsel).core.attr.disabled != 0 {
                    (*evsel).disabled = true;
                    *start_time = INVALID_START_TIME;
                } else {
                    (*evsel).disabled = false;
                    err = tool_pmu__read_stat(evsel, idx, thread, start_time);
                    if err != 0 {
                        close(fd);
                        *FD(evsel, idx, thread) = -1;
                        break;
                    }
                }
            }
            thread += 1;
        }
        if err != 0 {
            break;
        }
        idx += 1;
    }
    if err == 0 {
        return 0;
    }

    (*threads).err_thread = thread;

    old_errno = errno();
    loop {
        thread -= 1;
        while thread >= 0 {
            if *FD(evsel, idx, thread) >= 0 {
                close(*FD(evsel, idx, thread));
            }
            *FD(evsel, idx, thread) = -1;
            thread -= 1;
        }
        thread = nthreads;
        idx -= 1;
        if idx < 0 {
            break;
        }
    }
    set_errno(old_errno);
    err
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[no_mangle]
pub unsafe extern "C" fn arch_get_tsc_freq() -> u64 {
    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" {
    fn arch_get_tsc_freq() -> u64;
}

#[cfg(not(target_arch = "aarch64"))]
#[no_mangle]
pub unsafe extern "C" fn tool_pmu__cpu_slots_per_cycle() -> u64 {
    0
}

#[cfg(target_arch = "aarch64")]
unsafe extern "C" {
    fn tool_pmu__cpu_slots_per_cycle() -> u64;
}

unsafe fn has_pmem() -> bool_ {
    static mut HAS_PMEM: bool_ = false;
    static mut CACHED: bool_ = false;
    let sysfs = sysfs__mountpoint();
    let mut path = [0 as c_char; PATH_MAX];

    if !CACHED {
        snprintf(
            path.as_mut_ptr(),
            path.len(),
            b"%s/firmware/acpi/tables/NFIT\0".as_ptr() as *const c_char,
            sysfs,
        );
        HAS_PMEM = access(path.as_ptr(), F_OK) == 0;
        CACHED = true;
    }
    HAS_PMEM
}

#[no_mangle]
pub unsafe extern "C" fn tool_pmu__read_event(
    ev: tool_pmu_event,
    evsel: *mut evsel,
    system_wide: bool_,
    user_requested_cpu_list: *const c_char,
    result: *mut u64,
) -> bool_ {
    let mut topology: *const cpu_topology;

    match ev {
        tool_pmu_event::TOOL_PMU__EVENT_HAS_PMEM => {
            *result = if has_pmem() { 1 } else { 0 };
            true
        }
        tool_pmu_event::TOOL_PMU__EVENT_NUM_CORES => {
            topology = online_topology();
            *result = (*topology).core_cpus_lists;
            true
        }
        tool_pmu_event::TOOL_PMU__EVENT_NUM_CPUS => {
            if evsel.is_null() || perf_cpu_map__is_empty((*evsel).core.cpus) {
                /* No evsel to be specific to. */
                *result = cpu__max_present_cpu().cpu as u64;
            } else if !perf_cpu_map__has_any_cpu((*evsel).core.cpus) {
                /* Evsel just has specific CPUs. */
                *result = perf_cpu_map__nr((*evsel).core.cpus) as u64;
            } else {
                /*
                 * "Any CPU" event that can be scheduled on any CPU in
                 * the PMU's cpumask. The PMU cpumask should be saved in
                 * pmu_cpus. If not present fall back to max.
                 */
                if !perf_cpu_map__is_empty((*evsel).core.pmu_cpus) {
                    *result = perf_cpu_map__nr((*evsel).core.pmu_cpus) as u64;
                } else {
                    *result = cpu__max_present_cpu().cpu as u64;
                }
            }
            true
        }
        tool_pmu_event::TOOL_PMU__EVENT_NUM_CPUS_ONLINE => {
            let online = cpu_map__online();

            if online.is_null() {
                return false;
            }

            if evsel.is_null() || perf_cpu_map__is_empty((*evsel).core.cpus) {
                /* No evsel to be specific to. */
                *result = perf_cpu_map__nr(online) as u64;
            } else if !perf_cpu_map__has_any_cpu((*evsel).core.cpus) {
                /* Evsel just has specific CPUs. */
                let tmp = perf_cpu_map__intersect(online, (*evsel).core.cpus);

                *result = perf_cpu_map__nr(tmp) as u64;
                perf_cpu_map__put(tmp);
            } else {
                /*
                 * "Any CPU" event that can be scheduled on any CPU in
                 * the PMU's cpumask. The PMU cpumask should be saved in
                 * pmu_cpus, if not present then just the online cpu
                 * mask.
                 */
                if !perf_cpu_map__is_empty((*evsel).core.pmu_cpus) {
                    let tmp = perf_cpu_map__intersect(online, (*evsel).core.pmu_cpus);

                    *result = perf_cpu_map__nr(tmp) as u64;
                    perf_cpu_map__put(tmp);
                } else {
                    *result = perf_cpu_map__nr(online) as u64;
                }
            }
            perf_cpu_map__put(online);
            true
        }
        tool_pmu_event::TOOL_PMU__EVENT_NUM_DIES => {
            topology = online_topology();
            *result = (*topology).die_cpus_lists;
            true
        }
        tool_pmu_event::TOOL_PMU__EVENT_NUM_PACKAGES => {
            topology = online_topology();
            *result = (*topology).package_cpus_lists;
            true
        }
        tool_pmu_event::TOOL_PMU__EVENT_SLOTS => {
            *result = tool_pmu__cpu_slots_per_cycle();
            *result != 0
        }
        tool_pmu_event::TOOL_PMU__EVENT_SMT_ON => {
            *result = if smt_on() { 1 } else { 0 };
            true
        }
        tool_pmu_event::TOOL_PMU__EVENT_SYSTEM_TSC_FREQ => {
            *result = arch_get_tsc_freq();
            true
        }
        tool_pmu_event::TOOL_PMU__EVENT_CORE_WIDE => {
            *result = if core_wide(system_wide, user_requested_cpu_list) { 1 } else { 0 };
            true
        }
        tool_pmu_event::TOOL_PMU__EVENT_TARGET_CPU => {
            *result = if system_wide || !user_requested_cpu_list.is_null() {
                1
            } else {
                0
            };
            true
        }
        tool_pmu_event::TOOL_PMU__EVENT_NONE
        | tool_pmu_event::TOOL_PMU__EVENT_DURATION_TIME
        | tool_pmu_event::TOOL_PMU__EVENT_USER_TIME
        | tool_pmu_event::TOOL_PMU__EVENT_SYSTEM_TIME
        | tool_pmu_event::TOOL_PMU__EVENT_MAX => false,
    }
}

unsafe fn perf_counts__update(
    count: *mut perf_counts_values,
    old_count: *const perf_counts_values,
    raw: bool_,
    val: u64,
) {
    /*
     * The values of enabled and running must make a ratio of 100%. The
     * exact values don't matter as long as they are non-zero to avoid
     * issues with evsel__count_has_error.
     */
    if !old_count.is_null() {
        (*count).val = if raw { val } else { (*old_count).val + val };
        (*count).run = (*old_count).run + 1;
        (*count).ena = (*old_count).ena + 1;
        (*count).lost = (*old_count).lost;
    } else {
        (*count).val = val;
        (*count).run += 1;
        (*count).ena += 1;
        (*count).lost = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn evsel__tool_pmu_enable_cpu(
    evsel: *mut evsel,
    cpu_map_idx: c_int,
) -> c_int {
    let ev = evsel__tool_event(evsel);
    let mut thread: c_int;
    let nthreads: c_int;

    if !(*evsel).disabled {
        return 0;
    }

    if ev == tool_pmu_event::TOOL_PMU__EVENT_DURATION_TIME {
        if cpu_map_idx == 0 {
            (*evsel).duration_time.start_time = rdclock();
        }
        return 0;
    }

    if ev == tool_pmu_event::TOOL_PMU__EVENT_USER_TIME
        || ev == tool_pmu_event::TOOL_PMU__EVENT_SYSTEM_TIME
    {
        nthreads = xyarray__max_y((*evsel).process_time.start_times);
        thread = 0;
        while thread < nthreads {
            let start_time =
                xyarray__entry((*evsel).process_time.start_times, cpu_map_idx, thread)
                    as *mut __u64;
            let mut val: __u64 = 0;
            let err: c_int;

            err = tool_pmu__read_stat(evsel, cpu_map_idx, thread, &mut val);
            if err == 0 {
                *start_time = val;
            } else {
                *start_time = INVALID_START_TIME;
            }
            thread += 1;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn evsel__tool_pmu_enable(evsel: *mut evsel) -> c_int {
    let mut idx: c_uint;
    let mut err: c_int = 0;

    if !(*evsel).disabled {
        return 0;
    }

    idx = 0;
    while idx < perf_cpu_map__nr((*evsel).core.cpus) as c_uint {
        err = evsel__tool_pmu_enable_cpu(evsel, idx as c_int);
        if err != 0 {
            break;
        }
        idx += 1;
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn evsel__tool_pmu_disable_cpu(
    evsel: *mut evsel,
    cpu_map_idx: c_int,
) -> c_int {
    let ev = evsel__tool_event(evsel);
    let mut thread: c_int;
    let nthreads: c_int;

    if (*evsel).disabled {
        return 0;
    }

    if ev == tool_pmu_event::TOOL_PMU__EVENT_DURATION_TIME {
        if cpu_map_idx == 0 {
            let delta: __u64 = rdclock() - (*evsel).duration_time.start_time;

            (*evsel).duration_time.accumulated_time += delta;
        }
        return 0;
    }

    if ev == tool_pmu_event::TOOL_PMU__EVENT_USER_TIME
        || ev == tool_pmu_event::TOOL_PMU__EVENT_SYSTEM_TIME
    {
        nthreads = xyarray__max_y((*evsel).process_time.start_times);
        thread = 0;
        while thread < nthreads {
            let start_time =
                xyarray__entry((*evsel).process_time.start_times, cpu_map_idx, thread)
                    as *mut __u64;
            let accumulated_time =
                xyarray__entry((*evsel).process_time.accumulated_times, cpu_map_idx, thread)
                    as *mut __u64;
            let mut val: __u64 = 0;
            let err: c_int;

            err = tool_pmu__read_stat(evsel, cpu_map_idx, thread, &mut val);
            if err == 0 {
                if *start_time != INVALID_START_TIME && val >= *start_time {
                    *accumulated_time += val - *start_time;
                }
            }
            *start_time = INVALID_START_TIME;
            thread += 1;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn evsel__tool_pmu_disable(evsel: *mut evsel) -> c_int {
    let mut idx: c_uint;
    let mut err: c_int = 0;

    if (*evsel).disabled {
        return 0;
    }

    idx = 0;
    while idx < perf_cpu_map__nr((*evsel).core.cpus) as c_uint {
        err = evsel__tool_pmu_disable_cpu(evsel, idx as c_int);
        if err != 0 {
            break;
        }
        idx += 1;
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn evsel__tool_pmu_read(
    evsel: *mut evsel,
    cpu_map_idx: c_int,
    thread: c_int,
) -> c_int {
    let mut delta_start: __u64 = 0;
    let mut err: c_int = 0;
    let count: *mut perf_counts_values;
    let mut old_count: *mut perf_counts_values = core::ptr::null_mut();
    let mut adjust = false;
    let ev = evsel__tool_event(evsel);

    count = perf_counts((*evsel).counts, cpu_map_idx, thread);
    if !(*evsel).prev_raw_counts.is_null() {
        old_count = perf_counts((*evsel).prev_raw_counts, cpu_map_idx, thread);
    }

    match ev {
        tool_pmu_event::TOOL_PMU__EVENT_HAS_PMEM
        | tool_pmu_event::TOOL_PMU__EVENT_NUM_CORES
        | tool_pmu_event::TOOL_PMU__EVENT_NUM_CPUS
        | tool_pmu_event::TOOL_PMU__EVENT_NUM_CPUS_ONLINE
        | tool_pmu_event::TOOL_PMU__EVENT_NUM_DIES
        | tool_pmu_event::TOOL_PMU__EVENT_NUM_PACKAGES
        | tool_pmu_event::TOOL_PMU__EVENT_SLOTS
        | tool_pmu_event::TOOL_PMU__EVENT_SMT_ON
        | tool_pmu_event::TOOL_PMU__EVENT_CORE_WIDE
        | tool_pmu_event::TOOL_PMU__EVENT_TARGET_CPU
        | tool_pmu_event::TOOL_PMU__EVENT_SYSTEM_TSC_FREQ => {
            let mut val: u64 = 0;

            if cpu_map_idx == 0 && thread == 0 {
                if !tool_pmu__read_event(
                    ev,
                    evsel,
                    stat_config.system_wide,
                    stat_config.user_requested_cpu_list,
                    &mut val,
                ) {
                    (*count).lost += 1;
                    val = 0;
                }
            }
            perf_counts__update(count, old_count, false, val);
            return 0;
        }
        tool_pmu_event::TOOL_PMU__EVENT_DURATION_TIME => {
            if cpu_map_idx == 0 && thread == 0 {
                delta_start = (*evsel).duration_time.accumulated_time;
                if !(*evsel).disabled && (*evsel).duration_time.start_time != INVALID_START_TIME {
                    delta_start += rdclock() - (*evsel).duration_time.start_time;
                }
            } else {
                delta_start = 0;
            }
        }
        tool_pmu_event::TOOL_PMU__EVENT_USER_TIME
        | tool_pmu_event::TOOL_PMU__EVENT_SYSTEM_TIME => {
            let accumulated = *(xyarray__entry(
                (*evsel).process_time.accumulated_times,
                cpu_map_idx,
                thread,
            ) as *mut __u64);

            if (*evsel).disabled {
                delta_start = accumulated;
            } else {
                let start_time =
                    xyarray__entry((*evsel).process_time.start_times, cpu_map_idx, thread)
                        as *mut __u64;
                let mut cur_time: __u64 = 0;

                err = tool_pmu__read_stat(evsel, cpu_map_idx, thread, &mut cur_time);
                if err == 0 {
                    if *start_time != INVALID_START_TIME && cur_time >= *start_time {
                        delta_start = accumulated + (cur_time - *start_time);
                    } else {
                        delta_start = accumulated;
                    }
                }
            }
            adjust = true;
        }
        tool_pmu_event::TOOL_PMU__EVENT_NONE | tool_pmu_event::TOOL_PMU__EVENT_MAX => {
            err = -EINVAL;
        }
    }
    if err != 0 {
        return err;
    }

    if adjust {
        let ticks_per_sec: __u64 = sysconf(_SC_CLK_TCK) as __u64;

        delta_start *= 1_000_000_000u64 / ticks_per_sec;
    }
    perf_counts__update(count, old_count, true, delta_start);
    0
}

#[no_mangle]
pub unsafe extern "C" fn tool_pmu__new() -> *mut perf_pmu {
    let tool = zalloc(core::mem::size_of::<perf_pmu>()) as *mut perf_pmu;

    if tool.is_null() {
        return core::ptr::null_mut();
    }

    if perf_pmu__init(tool, PERF_PMU_TYPE_TOOL, b"tool\0".as_ptr() as *const c_char) != 0 {
        perf_pmu__delete(tool);
        return core::ptr::null_mut();
    }
    (*tool).events_table = find_core_events_table(
        b"common\0".as_ptr() as *const c_char,
        b"common\0".as_ptr() as *const c_char,
    );
    tool
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
