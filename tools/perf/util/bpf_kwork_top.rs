// SPDX-License-Identifier: GPL-2.0
/*
 * bpf_kwork_top.c
 *
 * Copyright (c) 2022  Huawei Inc,  Yang Jihong <yangjihong1@huawei.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type u8 = __u8;
type u64 = __u64;

const CLOCK_MONOTONIC: c_int = 1;
const NSEC_PER_SEC: u64 = 1_000_000_000;
const BPF_ANY: u64 = 0;

/*
 * This should be in sync with "util/kwork_top.bpf.c"
 */
const MAX_COMMAND_LEN: usize = 16;

#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct time_data {
    pub timestamp: __u64,
}

#[repr(C)]
pub struct work_data {
    pub runtime: __u64,
}

#[repr(C)]
pub struct task_data {
    pub tgid: __u32,
    pub is_kthread: __u32,
    pub comm: [c_char; MAX_COMMAND_LEN],
}

#[repr(C)]
pub struct work_key {
    pub type_: __u32,
    pub pid: __u32,
    pub task_p: __u64,
}

#[repr(C)]
pub struct task_key {
    pub pid: __u32,
    pub cpu: __u32,
}

#[repr(C)]
pub struct kwork_class_bpf {
    pub class: *mut kwork_class,
    pub load_prepare: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kwork_top_bpf_bss {
    pub from_timestamp: u64,
    pub enabled: c_int,
    pub to_timestamp: u64,
}

#[repr(C)]
pub struct kwork_top_bpf_rodata {
    pub has_cpu_filter: c_int,
}

#[repr(C)]
pub struct kwork_top_bpf_progs {
    pub on_irq_handler_entry: *mut bpf_program,
    pub on_irq_handler_exit: *mut bpf_program,
    pub on_softirq_entry: *mut bpf_program,
    pub on_softirq_exit: *mut bpf_program,
    pub on_switch: *mut bpf_program,
}

#[repr(C)]
pub struct kwork_top_bpf_maps {
    pub kwork_top_cpu_filter: *mut bpf_map,
    pub kwork_top_tasks: *mut bpf_map,
    pub kwork_top_works: *mut bpf_map,
}

#[repr(C)]
pub struct kwork_top_bpf {
    pub obj: *mut bpf_object,
    pub progs: kwork_top_bpf_progs,
    pub maps: kwork_top_bpf_maps,
    pub bss: *mut kwork_top_bpf_bss,
    pub rodata: *mut kwork_top_bpf_rodata,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct kwork_class {
    pub type_: kwork_class_type,
    pub name: *const c_char,
    pub list: list_head,
}

#[repr(C)]
pub struct kwork_work {
    pub id: __u32,
    pub cpu: __u32,
    pub name: *mut c_char,
    pub class: *mut kwork_class,
    pub tgid: __u32,
    pub is_kthread: __u32,
    pub total_runtime: __u64,
}

#[repr(C)]
pub struct perf_kwork {
    pub cpu_list: *const c_char,
    pub class_list: list_head,
    pub add_work: unsafe extern "C" fn(
        *mut perf_kwork,
        *mut kwork_class,
        *mut kwork_work,
    ) -> *mut kwork_work,
}

pub type kwork_class_type = c_int;

const KWORK_CLASS_IRQ: usize = 0;
const KWORK_CLASS_SOFTIRQ: usize = 1;
const KWORK_CLASS_SCHED: usize = 2;
const KWORK_CLASS_MAX: usize = 3;

unsafe extern "C" {
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);

    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn libbpf_num_possible_cpus() -> c_int;

    fn perf_cpu_map__new(cpu_list: *const c_char) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__nr(map: *const perf_cpu_map) -> c_uint;
    fn perf_cpu_map__cpu(map: *const perf_cpu_map, idx: c_uint) -> perf_cpu;

    fn kwork_top_bpf__open() -> *mut kwork_top_bpf;
    fn kwork_top_bpf__load(skel: *mut kwork_top_bpf) -> c_int;
    fn kwork_top_bpf__attach(skel: *mut kwork_top_bpf) -> c_int;
    fn kwork_top_bpf__destroy(skel: *mut kwork_top_bpf);

    fn bpf_object__next_program(
        obj: *mut bpf_object,
        prog: *mut bpf_program,
    ) -> *mut bpf_program;
    fn list_entry_kwork_class(pos: *mut list_head) -> *mut kwork_class;
}

static mut skel: *mut kwork_top_bpf = ptr::null_mut();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_kwork__top_start() {
    let mut ts: timespec = mem::zeroed();

    clock_gettime(CLOCK_MONOTONIC, &mut ts);
    (*(*skel).bss).from_timestamp = (ts.tv_sec as u64)
        .wrapping_mul(NSEC_PER_SEC)
        .wrapping_add(ts.tv_nsec as u64);
    (*(*skel).bss).enabled = 1;
    pr_debug(
        c"perf kwork top start at: %lld\n".as_ptr(),
        (*(*skel).bss).from_timestamp,
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_kwork__top_finish() {
    let mut ts: timespec = mem::zeroed();

    (*(*skel).bss).enabled = 0;
    clock_gettime(CLOCK_MONOTONIC, &mut ts);
    (*(*skel).bss).to_timestamp = (ts.tv_sec as u64)
        .wrapping_mul(NSEC_PER_SEC)
        .wrapping_add(ts.tv_nsec as u64);
    pr_debug(
        c"perf kwork top finish at: %lld\n".as_ptr(),
        (*(*skel).bss).to_timestamp,
    );
}

unsafe extern "C" fn irq_load_prepare() {
    bpf_program__set_autoload((*skel).progs.on_irq_handler_entry, true);
    bpf_program__set_autoload((*skel).progs.on_irq_handler_exit, true);
}

static mut kwork_irq_bpf: kwork_class_bpf = kwork_class_bpf {
    class: ptr::null_mut(),
    load_prepare: Some(irq_load_prepare),
};

unsafe extern "C" fn softirq_load_prepare() {
    bpf_program__set_autoload((*skel).progs.on_softirq_entry, true);
    bpf_program__set_autoload((*skel).progs.on_softirq_exit, true);
}

static mut kwork_softirq_bpf: kwork_class_bpf = kwork_class_bpf {
    class: ptr::null_mut(),
    load_prepare: Some(softirq_load_prepare),
};

unsafe extern "C" fn sched_load_prepare() {
    bpf_program__set_autoload((*skel).progs.on_switch, true);
}

static mut kwork_sched_bpf: kwork_class_bpf = kwork_class_bpf {
    class: ptr::null_mut(),
    load_prepare: Some(sched_load_prepare),
};

static mut kwork_class_bpf_supported_list: [*mut kwork_class_bpf; KWORK_CLASS_MAX] = [
    unsafe { &raw mut kwork_irq_bpf },
    unsafe { &raw mut kwork_softirq_bpf },
    unsafe { &raw mut kwork_sched_bpf },
];

unsafe fn valid_kwork_class_type(type_: kwork_class_type) -> bool {
    type_ >= 0 && (type_ as usize) < KWORK_CLASS_MAX
}

unsafe fn setup_filters(kwork: *mut perf_kwork) -> c_int {
    if !(*kwork).cpu_list.is_null() {
        let mut idx: c_uint;
        let nr_cpus: c_int;
        let fd: c_int;
        let map: *mut perf_cpu_map;
        let mut cpu: perf_cpu;

        fd = bpf_map__fd((*skel).maps.kwork_top_cpu_filter);
        if fd < 0 {
            pr_debug(c"Invalid cpu filter fd\n".as_ptr());
            return -1;
        }

        map = perf_cpu_map__new((*kwork).cpu_list);
        if map.is_null() {
            pr_debug(c"Invalid cpu_list\n".as_ptr());
            return -1;
        }

        nr_cpus = libbpf_num_possible_cpus();
        idx = 0;
        while idx < perf_cpu_map__nr(map) {
            let val: u8 = 1;
            cpu = perf_cpu_map__cpu(map, idx);

            if cpu.cpu >= nr_cpus {
                perf_cpu_map__put(map);
                pr_err(c"Requested cpu %d too large\n".as_ptr(), cpu.cpu);
                return -1;
            }
            bpf_map_update_elem(
                fd,
                &cpu.cpu as *const c_int as *const c_void,
                &val as *const u8 as *const c_void,
                BPF_ANY,
            );
            idx += 1;
        }
        perf_cpu_map__put(map);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_kwork__top_prepare_bpf(kwork: *mut perf_kwork) -> c_int {
    let mut prog: *mut bpf_program;
    let mut class: *mut kwork_class;
    let mut class_bpf: *mut kwork_class_bpf;
    let mut type_: kwork_class_type;

    skel = kwork_top_bpf__open();
    if skel.is_null() {
        pr_debug(c"Failed to open kwork top skeleton\n".as_ptr());
        return -1;
    }

    /*
     * set all progs to non-autoload,
     * then set corresponding progs according to config
     */
    prog = ptr::null_mut();
    loop {
        prog = bpf_object__next_program((*skel).obj, prog);
        if prog.is_null() {
            break;
        }
        bpf_program__set_autoload(prog, false);
    }

    let mut node = (*kwork).class_list.next;
    while node != &raw mut (*kwork).class_list {
        class = list_entry_kwork_class(node);
        type_ = (*class).type_;
        if !valid_kwork_class_type(type_)
            || kwork_class_bpf_supported_list[type_ as usize].is_null()
        {
            pr_err(c"Unsupported bpf trace class %s\n".as_ptr(), (*class).name);
            kwork_top_bpf__destroy(skel);
            return -1;
        }

        class_bpf = kwork_class_bpf_supported_list[type_ as usize];
        (*class_bpf).class = class;

        if let Some(load_prepare) = (*class_bpf).load_prepare {
            load_prepare();
        }
        node = (*node).next;
    }

    if !(*kwork).cpu_list.is_null() {
        (*(*skel).rodata).has_cpu_filter = 1;
    }

    if kwork_top_bpf__load(skel) != 0 {
        pr_debug(c"Failed to load kwork top skeleton\n".as_ptr());
        kwork_top_bpf__destroy(skel);
        return -1;
    }

    if setup_filters(kwork) != 0 {
        kwork_top_bpf__destroy(skel);
        return -1;
    }

    if kwork_top_bpf__attach(skel) != 0 {
        pr_debug(c"Failed to attach kwork top skeleton\n".as_ptr());
        kwork_top_bpf__destroy(skel);
        return -1;
    }

    0
}

unsafe fn read_task_info(work: *mut kwork_work) {
    let fd: c_int;
    let mut data: task_data = mem::zeroed();
    let key = task_key {
        pid: (*work).id,
        cpu: (*work).cpu,
    };

    fd = bpf_map__fd((*skel).maps.kwork_top_tasks);
    if fd < 0 {
        pr_debug(c"Invalid top tasks map fd\n".as_ptr());
        return;
    }

    if bpf_map_lookup_elem(
        fd,
        &key as *const task_key as *const c_void,
        &mut data as *mut task_data as *mut c_void,
    ) == 0
    {
        (*work).tgid = data.tgid;
        (*work).is_kthread = data.is_kthread;
        (*work).name = strdup(data.comm.as_ptr());
    }
}

unsafe fn add_work(
    kwork: *mut perf_kwork,
    key: *mut work_key,
    data: *mut work_data,
    cpu: c_int,
) -> c_int {
    let bpf_trace: *mut kwork_class_bpf;
    let work: *mut kwork_work;
    let mut tmp = kwork_work {
        id: (*key).pid,
        cpu: cpu as __u32,
        name: ptr::null_mut(),
        class: ptr::null_mut(),
        tgid: 0,
        is_kthread: 0,
        total_runtime: 0,
    };
    let type_: kwork_class_type = (*key).type_ as kwork_class_type;

    if !valid_kwork_class_type(type_) {
        pr_debug(c"Invalid class type %d to add work\n".as_ptr(), type_);
        return -1;
    }

    bpf_trace = kwork_class_bpf_supported_list[type_ as usize];
    tmp.class = (*bpf_trace).class;

    work = ((*kwork).add_work)(kwork, tmp.class, &mut tmp);
    if work.is_null() {
        return -1;
    }

    (*work).total_runtime = (*data).runtime;
    read_task_info(work);

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_kwork__top_read_bpf(kwork: *mut perf_kwork) -> c_int {
    let mut i: c_int;
    let fd: c_int;
    let nr_cpus: c_int;
    let data: *mut work_data;
    let mut key: work_key = mem::zeroed();
    let mut prev: work_key = mem::zeroed();

    fd = bpf_map__fd((*skel).maps.kwork_top_works);
    if fd < 0 {
        pr_debug(c"Invalid top runtime fd\n".as_ptr());
        return -1;
    }

    nr_cpus = libbpf_num_possible_cpus();
    data = calloc(nr_cpus as usize, mem::size_of::<work_data>()) as *mut work_data;
    if data.is_null() {
        return -1;
    }

    memset(
        &mut prev as *mut work_key as *mut c_void,
        0,
        mem::size_of_val(&prev),
    );
    while bpf_map_get_next_key(
        fd,
        &prev as *const work_key as *const c_void,
        &mut key as *mut work_key as *mut c_void,
    ) == 0
    {
        if bpf_map_lookup_elem(
            fd,
            &key as *const work_key as *const c_void,
            data as *mut c_void,
        ) != 0
        {
            pr_debug(c"Failed to lookup top elem\n".as_ptr());
            return -1;
        }

        i = 0;
        while i < nr_cpus {
            let cur = data.add(i as usize);
            if (*cur).runtime == 0 {
                i += 1;
                continue;
            }

            if add_work(kwork, &mut key, cur, i) != 0 {
                return -1;
            }
            i += 1;
        }
        prev = key;
    }
    free(data as *mut c_void);

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_kwork__top_cleanup_bpf() {
    kwork_top_bpf__destroy(skel);
}
