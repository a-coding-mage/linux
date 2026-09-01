// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/bpf_off_cpu.c.
// C include dependencies are intentionally preserved as external declarations.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const MAX_STACKS: usize = 32;
const MAX_PROC: c_int = 4096;
/* we don't need actual timestamp, just want to put the samples at last */
const OFF_CPU_TIMESTAMP: u64 = (!0u64) << 32;

const BPF_ANY: u64 = 0;
const BTF_KIND_TYPEDEF: c_uint = 8;
const PERF_RECORD_SAMPLE: u32 = 9;
const PERF_RECORD_MISC_USER: u16 = 1 << 13;
const PERF_CONTEXT_USER: u64 = (-512i64) as u64;
const PERF_SAMPLE_IP: u64 = 1 << 0;
const PERF_SAMPLE_TID: u64 = 1 << 1;
const PERF_SAMPLE_TIME: u64 = 1 << 2;
const PERF_SAMPLE_ID: u64 = 1 << 6;
const PERF_SAMPLE_CPU: u64 = 1 << 7;
const PERF_SAMPLE_PERIOD: u64 = 1 << 8;
const PERF_SAMPLE_RAW: u64 = 1 << 10;
const PERF_SAMPLE_IDENTIFIER: u64 = 1 << 16;
const PERF_SAMPLE_CGROUP: u64 = 1 << 21;

extern "C" {
    static OFFCPU_EVENT: *const c_char;
    static OFFCPU_SAMPLE_TYPES: u64;

    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn parse_event(evlist: *mut evlist, event: *const c_char) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn evsel__is_offcpu_event(evsel: *mut evsel) -> bool;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn perf_thread_map__pid(threads: *mut perf_thread_map, idx: c_int) -> c_int;
    fn perf_thread_map__nr(threads: *mut perf_thread_map) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn evlist__find_evsel_by_str(evlist: *mut evlist, name: *const c_char) -> *mut evsel;
    fn perf_cpu_map__nr(cpus: *mut perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(cpus: *mut perf_cpu_map, idx: c_int) -> perf_cpu;
    fn xyarray__entry(array: *mut xyarray, x: c_int, y: c_int) -> *mut c_void;
    fn bpf_map__update_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: usize,
        value: *const c_void,
        value_sz: usize,
        flags: u64,
    ) -> c_int;
    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: c_uint) -> c_int;
    fn btf__load_vmlinux_btf() -> *mut btf;
    fn btf__find_by_name_kind(btf: *mut btf, name: *const c_char, kind: c_uint) -> c_int;
    fn btf__type_by_id(btf: *mut btf, type_id: c_uint) -> *const btf_type;
    fn btf_is_ptr(t: *const btf_type) -> bool;
    fn btf_is_func_proto(t: *const btf_type) -> bool;
    fn btf_vlen(t: *const btf_type) -> u16;
    fn btf__free(btf: *mut btf);
    fn off_cpu_bpf__open() -> *mut off_cpu_bpf;
    fn off_cpu_bpf__load(skel: *mut off_cpu_bpf) -> c_int;
    fn off_cpu_bpf__attach(skel: *mut off_cpu_bpf) -> c_int;
    fn off_cpu_bpf__destroy(skel: *mut off_cpu_bpf);
    fn strlist__new(s: *const c_char, dupstr: *mut c_void) -> *mut strlist;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn target__has_task(target: *mut target) -> bool;
    fn target__none(target: *mut target) -> bool;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn cgroup_is_v2(subsys: *const c_char) -> bool;
    fn read_cgroup_id(cgrp: *mut cgroup) -> c_int;
    fn set_max_rlimit();
    fn perf_hooks__set_hook(
        hook: *const c_char,
        func: Option<unsafe extern "C" fn(*mut c_void)>,
        arg: *mut c_void,
    ) -> c_int;
    fn perf_data_file__write(file: *mut perf_data_file, buf: *const c_void, size: usize) -> isize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

type c_long = i64;
type u8_t = u8;
type u16_t = u16;
type u32_t = u32;
type u64_t = u64;

#[repr(C)]
pub struct evlist {
    pub core: evlist_core,
}

#[repr(C)]
pub struct evlist_core {
    pub threads: *mut perf_thread_map,
    pub user_requested_cpus: *mut perf_cpu_map,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub cgrp: *mut cgroup,
}

#[repr(C)]
pub struct evsel_core {
    pub system_wide: bool,
    pub cpus: *mut perf_cpu_map,
    pub fd: *mut xyarray,
    pub attr: perf_event_attr,
    pub id: *mut u64_t,
}

#[repr(C)]
pub struct perf_event_attr {
    pub sample_type: u64_t,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct target {
    pub cpu_list: *const c_char,
    pub pid: *const c_char,
}

#[repr(C)]
pub struct record_opts {
    pub record_cgroup: bool,
    pub off_cpu_thresh_ns: u64_t,
}

#[repr(C)]
pub struct perf_session {
    pub data: *mut perf_data,
    pub evlist: *mut evlist,
}

#[repr(C)]
pub struct perf_data {
    pub file: perf_data_file,
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32_t,
    pub misc: u16_t,
    pub size: u16_t,
}

#[repr(C)]
pub struct off_cpu_key {
    pub pid: u32_t,
    pub tgid: u32_t,
    pub stack_id: u32_t,
    pub state: u32_t,
    pub cgroup_id: u64_t,
}

#[repr(C)]
pub union off_cpu_data {
    pub hdr: perf_event_header,
    pub array: [u64_t; 1024 / size_of::<u64_t>()],
}

#[repr(C)]
pub struct off_cpu_bpf {
    pub rodata: *mut off_cpu_rodata,
    pub bss: *mut off_cpu_bss,
    pub maps: off_cpu_maps,
}

#[repr(C)]
pub struct off_cpu_rodata {
    pub has_task: bool,
    pub uses_tgid: bool,
    pub has_prev_state: bool,
    pub has_cpu: u8_t,
    pub uses_cgroup_v1: bool,
    pub has_cgroup: u8_t,
    pub needs_cgroup: bool,
}

#[repr(C)]
pub struct off_cpu_bss {
    pub enabled: c_int,
    pub offcpu_thresh_ns: u64_t,
}

#[repr(C)]
pub struct off_cpu_maps {
    pub task_filter: *mut bpf_map,
    pub offcpu_output: *mut bpf_map,
    pub cpu_filter: *mut bpf_map,
    pub cgroup_filter: *mut bpf_map,
    pub off_cpu: *mut bpf_map,
    pub stacks: *mut bpf_map,
}

#[repr(C)]
pub struct btf_type {
    pub name_off: u32_t,
    pub info: u32_t,
    pub size: u32_t,
    pub type_: u32_t,
}

#[repr(C)]
pub struct cgroup {
    pub id: u64_t,
    pub name: *const c_char,
}

pub enum bpf_map {}
pub enum btf {}
pub enum perf_thread_map {}
pub enum perf_cpu_map {}
pub enum xyarray {}
pub enum strlist {}
pub enum str_node {}
pub enum perf_data_file {}

static mut skel: *mut off_cpu_bpf = ptr::null_mut();
static mut off_cpu_raw: [u64_t; MAX_STACKS + 5] = [0; MAX_STACKS + 5];

macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

// Placeholder for the C evlist__for_each_entry macro.
unsafe fn evlist_for_each_entry<F: FnMut(*mut evsel)>(_evlist: *mut evlist, mut _f: F) {
    todo!("external macro evlist__for_each_entry must be provided by surrounding bindings")
}

// Placeholder for the C strlist__for_each_entry macro.
unsafe fn strlist_for_each_entry<F: FnMut(*mut str_node)>(_list: *mut strlist, mut _f: F) {
    todo!("external macro strlist__for_each_entry must be provided by surrounding bindings")
}

extern "C" {
    fn str_node_s(pos: *mut str_node) -> *const c_char;
}

unsafe fn off_cpu_config(evlist: *mut evlist) -> c_int {
    let mut off_cpu_event = [0 as c_char; 64];
    let mut found = false;

    scnprintf(
        off_cpu_event.as_mut_ptr(),
        off_cpu_event.len(),
        cstr!("bpf-output/name=%s/"),
        OFFCPU_EVENT,
    );
    if parse_event(evlist, off_cpu_event.as_ptr()) != 0 {
        pr_err(cstr!("Failed to open off-cpu event\n"));
        return -1;
    }

    evlist_for_each_entry(evlist, |evsel| {
        if !found && evsel__is_offcpu_event(evsel) {
            (*evsel).core.system_wide = true;
            found = true;
        }
    });

    0
}

unsafe extern "C" fn off_cpu_start(arg: *mut c_void) {
    let evlist = arg as *mut evlist;
    let mut evsel: *mut evsel;
    let mut i: c_uint = 0;

    /* update task filter for the given workload */
    if (*(*skel).rodata).has_task
        && (*(*skel).rodata).uses_tgid
        && perf_thread_map__pid((*evlist__core(evlist)).threads, 0) != -1
    {
        let fd: c_int;
        let mut pid: u32_t;
        let val: u8_t = 1;

        fd = bpf_map__fd((*skel).maps.task_filter);
        pid = perf_thread_map__pid((*evlist__core(evlist)).threads, 0) as u32_t;
        bpf_map_update_elem(
            fd,
            &mut pid as *mut _ as *const c_void,
            &val as *const _ as *const c_void,
            BPF_ANY,
        );
    }

    /* update BPF perf_event map */
    evsel = evlist__find_evsel_by_str(evlist, OFFCPU_EVENT);
    if evsel.is_null() {
        pr_err(cstr!("%s evsel not found\n"), OFFCPU_EVENT);
        return;
    }

    while (i as c_int) < perf_cpu_map__nr((*evsel).core.cpus) {
        let mut err: c_int;
        let mut pcpu: perf_cpu = perf_cpu_map__cpu((*evsel).core.cpus, i as c_int);
        let mut cpu_nr: c_int = pcpu.cpu;

        err = bpf_map__update_elem(
            (*skel).maps.offcpu_output,
            &mut cpu_nr as *mut _ as *const c_void,
            size_of::<c_int>(),
            xyarray__entry((*evsel).core.fd, cpu_nr, 0),
            size_of::<c_int>(),
            BPF_ANY,
        );
        if err != 0 {
            pr_err(cstr!(
                "Failed to update perf event map for direct off-cpu dumping\n"
            ));
            return;
        }
        i += 1;
    }

    (*(*skel).bss).enabled = 1;
}

unsafe extern "C" fn off_cpu_finish(_arg: *mut c_void) {
    (*(*skel).bss).enabled = 0;
    off_cpu_bpf__destroy(skel);
}

/* v5.18 kernel added prev_state arg, so it needs to check the signature */
unsafe fn check_sched_switch_args() {
    let btf = btf__load_vmlinux_btf();
    let mut t1: *const btf_type;
    let mut t2: *const btf_type;
    let mut t3: *const btf_type;
    let type_id: u32_t;

    if btf.is_null() {
        pr_debug(cstr!(
            "Missing btf, check if CONFIG_DEBUG_INFO_BTF is enabled\n"
        ));
        btf__free(btf);
        return;
    }

    type_id = btf__find_by_name_kind(btf, cstr!("btf_trace_sched_switch"), BTF_KIND_TYPEDEF) as u32_t;
    if (type_id as i32) < 0 {
        btf__free(btf);
        return;
    }

    t1 = btf__type_by_id(btf, type_id);
    if t1.is_null() {
        btf__free(btf);
        return;
    }

    t2 = btf__type_by_id(btf, (*t1).type_);
    if t2.is_null() || !btf_is_ptr(t2) {
        btf__free(btf);
        return;
    }

    t3 = btf__type_by_id(btf, (*t2).type_);
    /* btf_trace func proto has one more argument for the context */
    if !t3.is_null() && btf_is_func_proto(t3) && btf_vlen(t3) == 5 {
        /* new format: pass prev_state as 4th arg */
        (*(*skel).rodata).has_prev_state = true;
    }
    btf__free(btf);
}

#[no_mangle]
pub unsafe extern "C" fn off_cpu_prepare(
    evlist: *mut evlist,
    target: *mut target,
    opts: *mut record_opts,
) -> c_int {
    let mut err: c_int;
    let mut fd: c_int;
    let mut i: c_int;
    let mut ncpus: c_int = 1;
    let mut ntasks: c_int = 1;
    let mut ncgrps: c_int = 1;
    let mut pid_slist: *mut strlist = ptr::null_mut();
    let mut pos: *mut str_node = ptr::null_mut();

    if off_cpu_config(evlist) < 0 {
        pr_err(cstr!("Failed to config off-cpu BPF event\n"));
        return -1;
    }

    skel = off_cpu_bpf__open();
    if skel.is_null() {
        pr_err(cstr!("Failed to open off-cpu BPF skeleton\n"));
        return -1;
    }

    /* don't need to set cpu filter for system-wide mode */
    if !(*target).cpu_list.is_null() {
        ncpus = perf_cpu_map__nr((*evlist__core(evlist)).user_requested_cpus);
        bpf_map__set_max_entries((*skel).maps.cpu_filter, ncpus as c_uint);
        (*(*skel).rodata).has_cpu = 1;
    }

    if !(*target).pid.is_null() {
        pid_slist = strlist__new((*target).pid, ptr::null_mut());
        if pid_slist.is_null() {
            pr_err(cstr!("Failed to create a strlist for pid\n"));
            return -1;
        }

        ntasks = 0;
        strlist_for_each_entry(pid_slist, |p| {
            let mut end_ptr: *mut c_char = ptr::null_mut();
            let pid = strtol(str_node_s(p), &mut end_ptr, 10) as c_int;

            if pid == c_int::MIN
                || pid == c_int::MAX
                || (!(*end_ptr == 0) && *end_ptr != b',' as c_char)
            {
                return;
            }

            ntasks += 1;
        });

        if ntasks < MAX_PROC {
            ntasks = MAX_PROC;
        }

        bpf_map__set_max_entries((*skel).maps.task_filter, ntasks as c_uint);
        (*(*skel).rodata).has_task = true;
        (*(*skel).rodata).uses_tgid = true;
    } else if target__has_task(target) {
        ntasks = perf_thread_map__nr((*evlist__core(evlist)).threads);
        bpf_map__set_max_entries((*skel).maps.task_filter, ntasks as c_uint);
        (*(*skel).rodata).has_task = true;
    } else if target__none(target) {
        bpf_map__set_max_entries((*skel).maps.task_filter, MAX_PROC as c_uint);
        (*(*skel).rodata).has_task = true;
        (*(*skel).rodata).uses_tgid = true;
    }

    if !(*evlist__first(evlist)).cgrp.is_null() {
        ncgrps = evlist__nr_entries(evlist) - 1; /* excluding a dummy */
        bpf_map__set_max_entries((*skel).maps.cgroup_filter, ncgrps as c_uint);

        if !cgroup_is_v2(cstr!("perf_event")) {
            (*(*skel).rodata).uses_cgroup_v1 = true;
        }
        (*(*skel).rodata).has_cgroup = 1;
    }

    if (*opts).record_cgroup {
        (*(*skel).rodata).needs_cgroup = true;

        if !cgroup_is_v2(cstr!("perf_event")) {
            (*(*skel).rodata).uses_cgroup_v1 = true;
        }
    }

    set_max_rlimit();
    check_sched_switch_args();

    err = off_cpu_bpf__load(skel);
    if err != 0 {
        pr_err(cstr!("Failed to load off-cpu skeleton\n"));
        off_cpu_bpf__destroy(skel);
        return -1;
    }

    if !(*target).cpu_list.is_null() {
        let mut cpu: u32_t;
        let val: u8_t = 1;

        fd = bpf_map__fd((*skel).maps.cpu_filter);

        i = 0;
        while i < ncpus {
            cpu = perf_cpu_map__cpu((*evlist__core(evlist)).user_requested_cpus, i).cpu as u32_t;
            bpf_map_update_elem(
                fd,
                &mut cpu as *mut _ as *const c_void,
                &val as *const _ as *const c_void,
                BPF_ANY,
            );
            i += 1;
        }
    }

    if !(*target).pid.is_null() {
        let val: u8_t = 1;

        fd = bpf_map__fd((*skel).maps.task_filter);

        strlist_for_each_entry(pid_slist, |p| {
            let mut end_ptr: *mut c_char = ptr::null_mut();
            let mut tgid: u32_t;
            let pid = strtol(str_node_s(p), &mut end_ptr, 10) as c_int;

            if pid == c_int::MIN
                || pid == c_int::MAX
                || (!(*end_ptr == 0) && *end_ptr != b',' as c_char)
            {
                return;
            }

            tgid = pid as u32_t;
            bpf_map_update_elem(
                fd,
                &mut tgid as *mut _ as *const c_void,
                &val as *const _ as *const c_void,
                BPF_ANY,
            );
        });
    } else if target__has_task(target) {
        let mut pid: u32_t;
        let val: u8_t = 1;

        fd = bpf_map__fd((*skel).maps.task_filter);

        i = 0;
        while i < ntasks {
            pid = perf_thread_map__pid((*evlist__core(evlist)).threads, i) as u32_t;
            bpf_map_update_elem(
                fd,
                &mut pid as *mut _ as *const c_void,
                &val as *const _ as *const c_void,
                BPF_ANY,
            );
            i += 1;
        }
    }

    if !(*evlist__first(evlist)).cgrp.is_null() {
        let val: u8_t = 1;

        fd = bpf_map__fd((*skel).maps.cgroup_filter);

        evlist_for_each_entry(evlist, |evsel| {
            let cgrp = (*evsel).cgrp;

            if cgrp.is_null() {
                return;
            }

            if (*cgrp).id == 0 && read_cgroup_id(cgrp) < 0 {
                pr_err(cstr!("Failed to read cgroup id of %s\n"), (*cgrp).name);
                err = -1;
                return;
            }

            bpf_map_update_elem(
                fd,
                &(*cgrp).id as *const _ as *const c_void,
                &val as *const _ as *const c_void,
                BPF_ANY,
            );
        });
        if err == -1 {
            off_cpu_bpf__destroy(skel);
            return -1;
        }
    }

    (*(*skel).bss).offcpu_thresh_ns = (*opts).off_cpu_thresh_ns;

    err = off_cpu_bpf__attach(skel);
    if err != 0 {
        pr_err(cstr!("Failed to attach off-cpu BPF skeleton\n"));
        off_cpu_bpf__destroy(skel);
        return -1;
    }

    if perf_hooks__set_hook(cstr!("record_start"), Some(off_cpu_start), evlist as *mut c_void) != 0
        || perf_hooks__set_hook(cstr!("record_end"), Some(off_cpu_finish), evlist as *mut c_void) != 0
    {
        pr_err(cstr!("Failed to attach off-cpu skeleton\n"));
        off_cpu_bpf__destroy(skel);
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn off_cpu_write(session: *mut perf_session) -> c_int {
    let mut bytes: c_int = 0;
    let mut size: c_int;
    let fd: c_int;
    let stack: c_int;
    let mut raw_size: u32_t;
    let sample_type: u64_t;
    let mut val: u64_t = 0;
    let mut sid: u64_t = 0;
    let mut evsel: *mut evsel;
    let file: *mut perf_data_file = &mut (*(*session).data).file;
    let mut prev: off_cpu_key = zeroed();
    let mut key: off_cpu_key = zeroed();
    let mut data = off_cpu_data {
        hdr: perf_event_header {
            type_: PERF_RECORD_SAMPLE,
            misc: PERF_RECORD_MISC_USER,
            size: 0,
        },
    };
    let mut tstamp: u64_t = OFF_CPU_TIMESTAMP;

    (*(*skel).bss).enabled = 0;

    evsel = evlist__find_evsel_by_str((*session).evlist, OFFCPU_EVENT);
    if evsel.is_null() {
        pr_err(cstr!("%s evsel not found\n"), OFFCPU_EVENT);
        return 0;
    }

    sample_type = (*evsel).core.attr.sample_type;

    if sample_type & !OFFCPU_SAMPLE_TYPES != 0 {
        pr_err(
            cstr!("not supported sample type: %llx\n"),
            sample_type as u64_t,
        );
        return -1;
    }

    if sample_type & (PERF_SAMPLE_ID | PERF_SAMPLE_IDENTIFIER) != 0 {
        if !(*evsel).core.id.is_null() {
            sid = *(*evsel).core.id;
        }
    }

    fd = bpf_map__fd((*skel).maps.off_cpu);
    stack = bpf_map__fd((*skel).maps.stacks);
    memset(
        &mut prev as *mut _ as *mut c_void,
        0,
        size_of::<off_cpu_key>(),
    );

    while bpf_map_get_next_key(
        fd,
        &prev as *const _ as *const c_void,
        &mut key as *mut _ as *mut c_void,
    ) == 0
    {
        let mut n: c_int = 1; /* start from perf_event_header */

        bpf_map_lookup_elem(
            fd,
            &key as *const _ as *const c_void,
            &mut val as *mut _ as *mut c_void,
        );

        /* zero-fill some of the fields, will be overwritten by raw_data when parsing */
        if sample_type & PERF_SAMPLE_IDENTIFIER != 0 {
            data.array[n as usize] = sid;
            n += 1;
        }
        if sample_type & PERF_SAMPLE_IP != 0 {
            data.array[n as usize] = 0; /* will be updated */
            n += 1;
        }
        if sample_type & PERF_SAMPLE_TID != 0 {
            data.array[n as usize] = 0;
            n += 1;
        }
        if sample_type & PERF_SAMPLE_TIME != 0 {
            data.array[n as usize] = tstamp;
            n += 1;
        }
        if sample_type & PERF_SAMPLE_CPU != 0 {
            data.array[n as usize] = 0;
            n += 1;
        }
        if sample_type & PERF_SAMPLE_PERIOD != 0 {
            data.array[n as usize] = 0;
            n += 1;
        }
        if sample_type & PERF_SAMPLE_RAW != 0 {
            /*
             *  [ size ][ data ]
             *  [     data     ]
             *  [     data     ]
             *  [     data     ]
             *  [ data ][ empty]
             */
            let mut len: c_int = 0;
            let mut i: c_int = 0;
            let raw_data = (data.array.as_mut_ptr() as *mut u8).add(n as usize * size_of::<u64_t>());

            off_cpu_raw[i as usize] = ((key.pid as u64_t) << 32) | key.tgid as u64_t;
            i += 1;
            off_cpu_raw[i as usize] = val;
            i += 1;

            /* off_cpu_raw[i] is callchain->nr (updated later) */
            off_cpu_raw[(i + 1) as usize] = PERF_CONTEXT_USER;
            off_cpu_raw[(i + 2) as usize] = 0;

            bpf_map_lookup_elem(
                stack,
                &key.stack_id as *const _ as *const c_void,
                &mut off_cpu_raw[(i + 2) as usize] as *mut _ as *mut c_void,
            );
            while off_cpu_raw[(i + 2 + len) as usize] != 0 {
                len += 1;
            }

            off_cpu_raw[i as usize] = (len + 1) as u64_t;
            i += len + 2;

            off_cpu_raw[i as usize] = key.cgroup_id;
            i += 1;

            raw_size = (i as usize * size_of::<u64_t>() + size_of::<u32_t>()) as u32_t;
            memcpy(
                raw_data as *mut c_void,
                &raw_size as *const _ as *const c_void,
                size_of::<u32_t>(),
            );
            memcpy(
                raw_data.add(size_of::<u32_t>()) as *mut c_void,
                off_cpu_raw.as_ptr() as *const c_void,
                i as usize * size_of::<u64_t>(),
            );

            n += i + 1;
        }
        if sample_type & PERF_SAMPLE_CGROUP != 0 {
            data.array[n as usize] = key.cgroup_id;
            n += 1;
        }

        size = n * size_of::<u64_t>() as c_int;
        data.hdr.size = size as u16_t;
        bytes += size;

        if perf_data_file__write(file, &data as *const _ as *const c_void, size as usize) < 0 {
            pr_err(cstr!("failed to write perf data, error: %m\n"));
            return bytes;
        }

        prev = key;
        /* increase dummy timestamp to sort later samples */
        tstamp = tstamp.wrapping_add(1);
    }
    bytes
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
