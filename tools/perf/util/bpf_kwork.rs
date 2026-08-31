// SPDX-License-Identifier: GPL-2.0
/*
 * bpf_kwork.c
 *
 * Copyright (c) 2022  Huawei Inc,  Yang Jihong <yangjihong1@huawei.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::MaybeUninit;
use core::ptr;

// C includes translated as external dependencies:
// <time.h>, <fcntl.h>, <signal.h>, <stdio.h>, <unistd.h>
// <linux/time64.h>
// "util/debug.h", "util/evsel.h", "util/kwork.h"
// <bpf/bpf.h>, <perf/cpumap.h>
// "util/bpf_skel/kwork_trace.skel.h"

/*
 * This should be in sync with "util/kwork_trace.bpf.c"
 */
const MAX_KWORKNAME: usize = 128;

type u8 = u8;
type u32 = u32;
type u64 = u64;

const CLOCK_MONOTONIC: c_int = 1;
const BPF_ANY: u64 = 0;
const NSEC_PER_SEC: u64 = 1_000_000_000;

extern "C" {
    static KWORK_CLASS_MAX: c_int;
    static KWORK_CLASS_IRQ: kwork_class_type;
    static KWORK_CLASS_SOFTIRQ: kwork_class_type;
    static KWORK_CLASS_WORKQUEUE: kwork_class_type;
    static KWORK_REPORT_RUNTIME: c_int;
    static KWORK_REPORT_LATENCY: c_int;

    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strdup(s: *const c_char) -> *mut c_char;

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);

    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;

    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_object__next_program(prev: *mut bpf_program, obj: *mut bpf_object) -> *mut bpf_program;

    fn perf_cpu_map__new(cpu_list: *const c_char) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__nr(map: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(map: *const perf_cpu_map, idx: c_int) -> perf_cpu;
    fn libbpf_num_possible_cpus() -> c_int;

    fn kwork_trace_bpf__open() -> *mut kwork_trace_bpf;
    fn kwork_trace_bpf__load(skel: *mut kwork_trace_bpf) -> c_int;
    fn kwork_trace_bpf__attach(skel: *mut kwork_trace_bpf) -> c_int;
    fn kwork_trace_bpf__destroy(skel: *mut kwork_trace_bpf);

    fn work_exit(work: *mut kwork_work);
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct work_key {
    pub type_: u32,
    pub cpu: u32,
    pub id: u64,
}

#[repr(C)]
pub struct report_data {
    pub nr: u64,
    pub total_time: u64,
    pub max_time: u64,
    pub max_time_start: u64,
    pub max_time_end: u64,
}

#[repr(C)]
pub struct kwork_class_bpf {
    pub class: *mut kwork_class,
    pub load_prepare: Option<unsafe extern "C" fn(kwork: *mut perf_kwork)>,
    pub get_work_name: Option<unsafe extern "C" fn(key: *mut work_key, ret_name: *mut *mut c_char) -> c_int>,
}

#[repr(C)]
pub struct kwork_trace_bpf {
    pub obj: *mut bpf_object,
    pub maps: kwork_trace_bpf_maps,
    pub progs: kwork_trace_bpf_progs,
    pub rodata: *mut kwork_trace_bpf_rodata,
    pub bss: *mut kwork_trace_bpf_bss,
}

#[repr(C)]
pub struct kwork_trace_bpf_maps {
    pub perf_kwork_names: *mut bpf_map,
    pub perf_kwork_cpu_filter: *mut bpf_map,
    pub perf_kwork_name_filter: *mut bpf_map,
    pub perf_kwork_report: *mut bpf_map,
}

#[repr(C)]
pub struct kwork_trace_bpf_progs {
    pub report_irq_handler_entry: *mut bpf_program,
    pub report_irq_handler_exit: *mut bpf_program,
    pub report_softirq_entry: *mut bpf_program,
    pub report_softirq_exit: *mut bpf_program,
    pub latency_softirq_raise: *mut bpf_program,
    pub latency_softirq_entry: *mut bpf_program,
    pub report_workqueue_execute_start: *mut bpf_program,
    pub report_workqueue_execute_end: *mut bpf_program,
    pub latency_workqueue_activate_work: *mut bpf_program,
    pub latency_workqueue_execute_start: *mut bpf_program,
}

#[repr(C)]
pub struct kwork_trace_bpf_rodata {
    pub has_cpu_filter: u8,
    pub has_name_filter: u8,
}

#[repr(C)]
pub struct kwork_trace_bpf_bss {
    pub enabled: u8,
}

#[repr(C)]
pub struct perf_kwork {
    pub report: c_int,
    pub cpu_list: *const c_char,
    pub profile_name: *const c_char,
    pub class_list: list_head,
    pub add_work: unsafe extern "C" fn(
        kwork: *mut perf_kwork,
        class: *mut kwork_class,
        tmp: *mut kwork_work,
    ) -> *mut kwork_work,
    pub timestart: u64,
    pub timeend: u64,
}

#[repr(C)]
pub struct kwork_class {
    pub type_: kwork_class_type,
    pub name: *const c_char,
    pub list: list_head,
}

#[repr(C)]
pub struct kwork_work {
    pub id: u64,
    pub name: *mut c_char,
    pub cpu: u32,
    pub class: *mut kwork_class,
    pub nr_atoms: u64,
    pub total_runtime: u64,
    pub max_runtime: u64,
    pub max_runtime_start: u64,
    pub max_runtime_end: u64,
    pub total_latency: u64,
    pub max_latency: u64,
    pub max_latency_start: u64,
    pub max_latency_end: u64,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

pub enum bpf_map {}
pub enum bpf_program {}
pub enum bpf_object {}
pub enum perf_cpu_map {}

pub type kwork_class_type = c_int;

static mut skel: *mut kwork_trace_bpf = ptr::null_mut();

static mut ts_start: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
static mut ts_end: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};

#[no_mangle]
pub unsafe extern "C" fn perf_kwork__trace_start() {
    clock_gettime(CLOCK_MONOTONIC, &mut ts_start);
    (*(*skel).bss).enabled = 1;
}

#[no_mangle]
pub unsafe extern "C" fn perf_kwork__trace_finish() {
    clock_gettime(CLOCK_MONOTONIC, &mut ts_end);
    (*(*skel).bss).enabled = 0;
}

unsafe extern "C" fn get_work_name_from_map(key: *mut work_key, ret_name: *mut *mut c_char) -> c_int {
    let mut name: [c_char; MAX_KWORKNAME] = [0; MAX_KWORKNAME];
    let fd = bpf_map__fd((*skel).maps.perf_kwork_names);

    *ret_name = ptr::null_mut();

    if fd < 0 {
        pr_debug(b"Invalid names map fd\n\0".as_ptr() as *const c_char);
        return 0;
    }

    if bpf_map_lookup_elem(fd, key as *const c_void, name.as_mut_ptr() as *mut c_void) == 0
        && strlen(name.as_ptr()) != 0
    {
        *ret_name = strdup(name.as_ptr());
        if (*ret_name).is_null() {
            pr_err(b"Failed to copy work name\n\0".as_ptr() as *const c_char);
            return -1;
        }
    }

    0
}

unsafe extern "C" fn irq_load_prepare(kwork: *mut perf_kwork) {
    if (*kwork).report == KWORK_REPORT_RUNTIME {
        bpf_program__set_autoload((*skel).progs.report_irq_handler_entry, true);
        bpf_program__set_autoload((*skel).progs.report_irq_handler_exit, true);
    }
}

static mut kwork_irq_bpf: kwork_class_bpf = kwork_class_bpf {
    class: ptr::null_mut(),
    load_prepare: Some(irq_load_prepare),
    get_work_name: Some(get_work_name_from_map),
};

unsafe extern "C" fn softirq_load_prepare(kwork: *mut perf_kwork) {
    if (*kwork).report == KWORK_REPORT_RUNTIME {
        bpf_program__set_autoload((*skel).progs.report_softirq_entry, true);
        bpf_program__set_autoload((*skel).progs.report_softirq_exit, true);
    } else if (*kwork).report == KWORK_REPORT_LATENCY {
        bpf_program__set_autoload((*skel).progs.latency_softirq_raise, true);
        bpf_program__set_autoload((*skel).progs.latency_softirq_entry, true);
    }
}

static mut kwork_softirq_bpf: kwork_class_bpf = kwork_class_bpf {
    class: ptr::null_mut(),
    load_prepare: Some(softirq_load_prepare),
    get_work_name: Some(get_work_name_from_map),
};

unsafe extern "C" fn workqueue_load_prepare(kwork: *mut perf_kwork) {
    if (*kwork).report == KWORK_REPORT_RUNTIME {
        bpf_program__set_autoload((*skel).progs.report_workqueue_execute_start, true);
        bpf_program__set_autoload((*skel).progs.report_workqueue_execute_end, true);
    } else if (*kwork).report == KWORK_REPORT_LATENCY {
        bpf_program__set_autoload((*skel).progs.latency_workqueue_activate_work, true);
        bpf_program__set_autoload((*skel).progs.latency_workqueue_execute_start, true);
    }
}

static mut kwork_workqueue_bpf: kwork_class_bpf = kwork_class_bpf {
    class: ptr::null_mut(),
    load_prepare: Some(workqueue_load_prepare),
    get_work_name: Some(get_work_name_from_map),
};

static mut kwork_class_bpf_supported_list: [*mut kwork_class_bpf; 3] = [
    ptr::null_mut(),
    ptr::null_mut(),
    ptr::null_mut(),
];

unsafe fn init_kwork_class_bpf_supported_list() {
    kwork_class_bpf_supported_list[KWORK_CLASS_IRQ as usize] = &mut kwork_irq_bpf;
    kwork_class_bpf_supported_list[KWORK_CLASS_SOFTIRQ as usize] = &mut kwork_softirq_bpf;
    kwork_class_bpf_supported_list[KWORK_CLASS_WORKQUEUE as usize] = &mut kwork_workqueue_bpf;
}

unsafe fn valid_kwork_class_type(type_: kwork_class_type) -> bool {
    if type_ >= 0 && type_ < KWORK_CLASS_MAX {
        true
    } else {
        false
    }
}

unsafe fn setup_filters(kwork: *mut perf_kwork) -> c_int {
    if !(*kwork).cpu_list.is_null() {
        let mut idx: c_uint;
        let nr_cpus: c_int;
        let map: *mut perf_cpu_map;
        let mut cpu: perf_cpu;
        let fd = bpf_map__fd((*skel).maps.perf_kwork_cpu_filter);

        if fd < 0 {
            pr_debug(b"Invalid cpu filter fd\n\0".as_ptr() as *const c_char);
            return -1;
        }

        map = perf_cpu_map__new((*kwork).cpu_list);
        if map.is_null() {
            pr_debug(b"Invalid cpu_list\n\0".as_ptr() as *const c_char);
            return -1;
        }

        nr_cpus = libbpf_num_possible_cpus();
        idx = 0;
        while (idx as c_int) < perf_cpu_map__nr(map) {
            cpu = perf_cpu_map__cpu(map, idx as c_int);
            let val: u8 = 1;

            if cpu.cpu >= nr_cpus {
                perf_cpu_map__put(map);
                pr_err(
                    b"Requested cpu %d too large\n\0".as_ptr() as *const c_char,
                    cpu.cpu,
                );
                return -1;
            }
            bpf_map_update_elem(
                fd,
                &cpu.cpu as *const c_int as *const c_void,
                &val as *const u8 as *const c_void,
                BPF_ANY,
            );
            idx = idx.wrapping_add(1);
        }
        perf_cpu_map__put(map);
    }

    if !(*kwork).profile_name.is_null() {
        let mut key: c_int;
        let fd: c_int;

        if strlen((*kwork).profile_name) >= MAX_KWORKNAME {
            pr_err(
                b"Requested name filter %s too large, limit to %d\n\0".as_ptr() as *const c_char,
                (*kwork).profile_name,
                (MAX_KWORKNAME - 1) as c_int,
            );
            return -1;
        }

        fd = bpf_map__fd((*skel).maps.perf_kwork_name_filter);
        if fd < 0 {
            pr_debug(b"Invalid name filter fd\n\0".as_ptr() as *const c_char);
            return -1;
        }

        key = 0;
        bpf_map_update_elem(
            fd,
            &key as *const c_int as *const c_void,
            (*kwork).profile_name as *const c_void,
            BPF_ANY,
        );
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_kwork__trace_prepare_bpf(kwork: *mut perf_kwork) -> c_int {
    let mut prog: *mut bpf_program;
    let mut class: *mut kwork_class;
    let mut class_bpf: *mut kwork_class_bpf;
    let mut type_: kwork_class_type;

    init_kwork_class_bpf_supported_list();

    skel = kwork_trace_bpf__open();
    if skel.is_null() {
        pr_debug(b"Failed to open kwork trace skeleton\n\0".as_ptr() as *const c_char);
        return -1;
    }

    /*
     * set all progs to non-autoload,
     * then set corresponding progs according to config
     */
    prog = ptr::null_mut();
    loop {
        prog = bpf_object__next_program(prog, (*skel).obj);
        if prog.is_null() {
            break;
        }
        bpf_program__set_autoload(prog, false);
    }

    // Translation of list_for_each_entry(class, &kwork->class_list, list).
    class = (*kwork).class_list.next as *mut kwork_class;
    while !class.is_null() && (&mut (*class).list as *mut list_head) != &mut (*kwork).class_list {
        type_ = (*class).type_;
        if !valid_kwork_class_type(type_)
            || kwork_class_bpf_supported_list[type_ as usize].is_null()
        {
            pr_err(
                b"Unsupported bpf trace class %s\n\0".as_ptr() as *const c_char,
                (*class).name,
            );
            goto_out();
            return -1;
        }

        class_bpf = kwork_class_bpf_supported_list[type_ as usize];
        (*class_bpf).class = class;

        if let Some(load_prepare) = (*class_bpf).load_prepare {
            load_prepare(kwork);
        }

        class = (*class).list.next as *mut kwork_class;
    }

    if !(*kwork).cpu_list.is_null() {
        (*(*skel).rodata).has_cpu_filter = 1;
    }
    if !(*kwork).profile_name.is_null() {
        (*(*skel).rodata).has_name_filter = 1;
    }

    if kwork_trace_bpf__load(skel) != 0 {
        pr_debug(b"Failed to load kwork trace skeleton\n\0".as_ptr() as *const c_char);
        goto_out();
        return -1;
    }

    if setup_filters(kwork) != 0 {
        goto_out();
        return -1;
    }

    if kwork_trace_bpf__attach(skel) != 0 {
        pr_debug(b"Failed to attach kwork trace skeleton\n\0".as_ptr() as *const c_char);
        goto_out();
        return -1;
    }

    0
}

unsafe fn goto_out() {
    kwork_trace_bpf__destroy(skel);
}

unsafe fn add_work(kwork: *mut perf_kwork, key: *mut work_key, data: *mut report_data) -> c_int {
    let work: *mut kwork_work;
    let bpf_trace: *mut kwork_class_bpf;
    let mut tmp = kwork_work {
        id: (*key).id,
        name: ptr::null_mut(),
        cpu: (*key).cpu,
        class: ptr::null_mut(),
        nr_atoms: 0,
        total_runtime: 0,
        max_runtime: 0,
        max_runtime_start: 0,
        max_runtime_end: 0,
        total_latency: 0,
        max_latency: 0,
        max_latency_start: 0,
        max_latency_end: 0,
    };
    let type_: kwork_class_type = (*key).type_ as kwork_class_type;
    let mut ret: c_int = 0;

    if !valid_kwork_class_type(type_) {
        pr_debug(
            b"Invalid class type %d to add work\n\0".as_ptr() as *const c_char,
            type_,
        );
        return -1;
    }

    bpf_trace = kwork_class_bpf_supported_list[type_ as usize];
    tmp.class = (*bpf_trace).class;

    if let Some(get_work_name) = (*bpf_trace).get_work_name {
        if get_work_name(key, &mut tmp.name) != 0 {
            return -1;
        }
    }

    work = ((*kwork).add_work)(kwork, tmp.class, &mut tmp);
    if work.is_null() {
        ret = -1;
        work_exit(&mut tmp);
        return ret;
    }

    if (*kwork).report == KWORK_REPORT_RUNTIME {
        (*work).nr_atoms = (*data).nr;
        (*work).total_runtime = (*data).total_time;
        (*work).max_runtime = (*data).max_time;
        (*work).max_runtime_start = (*data).max_time_start;
        (*work).max_runtime_end = (*data).max_time_end;
    } else if (*kwork).report == KWORK_REPORT_LATENCY {
        (*work).nr_atoms = (*data).nr;
        (*work).total_latency = (*data).total_time;
        (*work).max_latency = (*data).max_time;
        (*work).max_latency_start = (*data).max_time_start;
        (*work).max_latency_end = (*data).max_time_end;
    } else {
        pr_debug(
            b"Invalid bpf report type %d\n\0".as_ptr() as *const c_char,
            (*kwork).report,
        );
        ret = -1;
        work_exit(&mut tmp);
        return ret;
    }

    (*kwork).timestart = ts_start.tv_sec as u64 * NSEC_PER_SEC + ts_start.tv_nsec as u64;
    (*kwork).timeend = ts_end.tv_sec as u64 * NSEC_PER_SEC + ts_end.tv_nsec as u64;

    work_exit(&mut tmp);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_kwork__report_read_bpf(kwork: *mut perf_kwork) -> c_int {
    let mut data = MaybeUninit::<report_data>::uninit();
    let mut key = work_key {
        type_: 0,
        cpu: 0,
        id: 0,
    };
    let mut prev = work_key {
        type_: 0,
        cpu: 0,
        id: 0,
    };
    let fd = bpf_map__fd((*skel).maps.perf_kwork_report);

    if fd < 0 {
        pr_debug(b"Invalid report fd\n\0".as_ptr() as *const c_char);
        return -1;
    }

    while bpf_map_get_next_key(
        fd,
        &prev as *const work_key as *const c_void,
        &mut key as *mut work_key as *mut c_void,
    ) == 0
    {
        if bpf_map_lookup_elem(
            fd,
            &key as *const work_key as *const c_void,
            data.as_mut_ptr() as *mut c_void,
        ) != 0
        {
            pr_debug(b"Failed to lookup report elem\n\0".as_ptr() as *const c_char);
            return -1;
        }

        let data_ptr = data.as_mut_ptr();
        if (*data_ptr).nr != 0 && add_work(kwork, &mut key, data_ptr) != 0 {
            return -1;
        }

        prev = key;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_kwork__report_cleanup_bpf() {
    kwork_trace_bpf__destroy(skel);
}
