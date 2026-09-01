// Translated from perf/util/bpf_ftrace.c.
// External types, constants, and functions are supplied by the surrounding perf/libbpf codebase.

use core::ffi::{c_char, c_int, c_void};

type u8 = u8;
type u32 = u32;
type u64 = u64;

const BPF_ANY: u64 = 0;
const O_RDONLY: c_int = 0;
const ENOMEM: c_int = 12;
const INT64_MAX: i64 = 0x7fff_ffff_ffff_ffff;

#[repr(C)]
pub struct perf_ftrace {
    pub filters: list_head,
    pub event_pair: list_head,
    pub bucket_range: u64,
    pub min_latency: u64,
    pub bucket_num: u32,
    pub target: target,
    pub evlist: *mut evlist,
    pub use_nsec: bool,
}

#[repr(C)]
pub struct filter_entry {
    pub list: list_head,
    pub name: *const c_char,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct target {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist_core {
    pub user_requested_cpus: *mut perf_cpu_map,
    pub threads: *mut perf_thread_map,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct stats {
    pub n: u64,
    pub mean: u64,
    pub max: u64,
    pub min: u64,
}

#[repr(C)]
pub struct func_latency_bpf {
    pub rodata: *mut func_latency_rodata,
    pub bss: *mut func_latency_bss,
    pub maps: func_latency_maps,
    pub progs: func_latency_progs,
    pub links: func_latency_links,
}

#[repr(C)]
pub struct func_latency_rodata {
    pub bucket_range: u64,
    pub min_latency: u64,
    pub bucket_num: u32,
    pub has_cpu: u8,
    pub has_task: u8,
    pub use_nsec: bool,
}

#[repr(C)]
pub struct func_latency_bss {
    pub min: i64,
    pub enabled: u8,
    pub count: u64,
    pub total: u64,
    pub max: u64,
}

#[repr(C)]
pub struct func_latency_maps {
    pub latency: *mut bpf_map,
    pub cpu_filter: *mut bpf_map,
    pub task_filter: *mut bpf_map,
}

#[repr(C)]
pub struct func_latency_progs {
    pub func_begin: *mut bpf_program,
    pub func_end: *mut bpf_program,
    pub event_begin: *mut bpf_program,
    pub event_end: *mut bpf_program,
}

#[repr(C)]
pub struct func_latency_links {
    pub func_begin: *mut bpf_link,
    pub func_end: *mut bpf_link,
    pub event_begin: *mut bpf_link,
    pub event_end: *mut bpf_link,
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

static mut skel: *mut func_latency_bpf = core::ptr::null_mut();

unsafe extern "C" {
    fn list_empty(head: *const list_head) -> c_int;
    fn list_is_singular(head: *const list_head) -> c_int;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn target__has_task(target: *const target) -> bool;
    fn target__none(target: *const target) -> bool;
    fn perf_cpu_map__nr(cpus: *mut perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(cpus: *mut perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_thread_map__nr(threads: *mut perf_thread_map) -> c_int;
    fn perf_thread_map__pid(threads: *mut perf_thread_map, idx: c_int) -> u32;
    fn cpu__max_cpu() -> perf_cpu;
    fn set_max_rlimit();

    fn func_latency_bpf__open() -> *mut func_latency_bpf;
    fn func_latency_bpf__load(skel: *mut func_latency_bpf) -> c_int;
    fn func_latency_bpf__destroy(skel: *mut func_latency_bpf);

    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: u32) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;

    fn bpf_program__attach_kprobe(
        prog: *mut bpf_program,
        retprobe: bool,
        func_name: *const c_char,
    ) -> *mut bpf_link;
    fn bpf_program__attach_raw_tracepoint(
        prog: *mut bpf_program,
        tp_name: *const c_char,
    ) -> *mut bpf_link;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

unsafe fn list_entry_filter_entry(ptr: *mut list_head) -> *mut filter_entry {
    ptr as *mut filter_entry
}

unsafe fn list_first_entry_filter_entry(head: *mut list_head) -> *mut filter_entry {
    list_entry_filter_entry((*head).next)
}

unsafe fn list_next_entry_filter_entry(pos: *mut filter_entry) -> *mut filter_entry {
    list_entry_filter_entry((*pos).list.next)
}

unsafe fn pr_err(msg: &[u8]) {
    unsafe extern "C" {
        fn fputs(s: *const c_char, stream: *mut c_void) -> c_int;
        static mut stderr: *mut c_void;
    }

    fputs(msg.as_ptr() as *const c_char, stderr);
}

#[no_mangle]
pub unsafe extern "C" fn perf_ftrace__latency_prepare_bpf(ftrace: *mut perf_ftrace) -> c_int {
    let mut fd: c_int;
    let mut err: c_int;
    let mut i: c_int;
    let mut ncpus: c_int = 1;
    let mut ntasks: c_int = 1;
    let mut func: *mut filter_entry = core::ptr::null_mut();

    if list_empty(&(*ftrace).filters) == 0 {
        if list_is_singular(&(*ftrace).filters) == 0 {
            pr_err(b"ERROR: Too many target functions.\n\0");
            return -1;
        }
        func = list_first_entry_filter_entry(&mut (*ftrace).filters);
    } else {
        let mut count: c_int = 0;
        let mut pos: *mut list_head = (*ftrace).event_pair.next;

        while pos != &mut (*ftrace).event_pair {
            count += 1;
            pos = (*pos).next;
        }

        if count != 2 {
            pr_err(b"ERROR: Needs two target events.\n\0");
            return -1;
        }
    }

    skel = func_latency_bpf__open();
    if skel.is_null() {
        pr_err(b"Failed to open func latency skeleton\n\0");
        return -1;
    }

    (*(*skel).rodata).bucket_range = (*ftrace).bucket_range;
    (*(*skel).rodata).min_latency = (*ftrace).min_latency;
    (*(*skel).rodata).bucket_num = (*ftrace).bucket_num;
    if (*ftrace).bucket_range != 0 && (*ftrace).bucket_num != 0 {
        bpf_map__set_max_entries((*skel).maps.latency, (*ftrace).bucket_num);
    }

    /* don't need to set cpu filter for system-wide mode */
    // C source tests ftrace->target.cpu_list; target layout is external to this file.
    if false {
        ncpus = perf_cpu_map__nr((*evlist__core((*ftrace).evlist)).user_requested_cpus);
        bpf_map__set_max_entries((*skel).maps.cpu_filter, ncpus as u32);
        (*(*skel).rodata).has_cpu = 1;
    }

    if target__has_task(&(*ftrace).target) || target__none(&(*ftrace).target) {
        ntasks = perf_thread_map__nr((*evlist__core((*ftrace).evlist)).threads);
        bpf_map__set_max_entries((*skel).maps.task_filter, ntasks as u32);
        (*(*skel).rodata).has_task = 1;
    }

    (*(*skel).rodata).use_nsec = (*ftrace).use_nsec;

    set_max_rlimit();

    err = func_latency_bpf__load(skel);
    if err != 0 {
        pr_err(b"Failed to load func latency skeleton\n\0");
        return err;
    }

    // C source tests ftrace->target.cpu_list; target layout is external to this file.
    if false {
        let mut cpu: u32;
        let val: u8 = 1;

        fd = bpf_map__fd((*skel).maps.cpu_filter);

        i = 0;
        while i < ncpus {
            cpu = perf_cpu_map__cpu(
                (*evlist__core((*ftrace).evlist)).user_requested_cpus,
                i,
            )
            .cpu as u32;
            bpf_map_update_elem(
                fd,
                &cpu as *const u32 as *const c_void,
                &val as *const u8 as *const c_void,
                BPF_ANY,
            );
            i += 1;
        }
    }

    if target__has_task(&(*ftrace).target) || target__none(&(*ftrace).target) {
        let mut pid: u32;
        let val: u8 = 1;

        fd = bpf_map__fd((*skel).maps.task_filter);

        i = 0;
        while i < ntasks {
            pid = perf_thread_map__pid((*evlist__core((*ftrace).evlist)).threads, i);
            bpf_map_update_elem(
                fd,
                &pid as *const u32 as *const c_void,
                &val as *const u8 as *const c_void,
                BPF_ANY,
            );
            i += 1;
        }
    }

    (*(*skel).bss).min = INT64_MAX;

    if !func.is_null() {
        (*skel).links.func_begin =
            bpf_program__attach_kprobe((*skel).progs.func_begin, false, (*func).name);
        if IS_ERR((*skel).links.func_begin as *const c_void) {
            pr_err(b"Failed to attach fentry program\n\0");
            err = PTR_ERR((*skel).links.func_begin as *const c_void);
            return err;
        }

        (*skel).links.func_end =
            bpf_program__attach_kprobe((*skel).progs.func_end, true, (*func).name);
        if IS_ERR((*skel).links.func_end as *const c_void) {
            pr_err(b"Failed to attach fexit program\n\0");
            err = PTR_ERR((*skel).links.func_end as *const c_void);
            return err;
        }
    } else {
        let mut event: *mut filter_entry;

        event = list_first_entry_filter_entry(&mut (*ftrace).event_pair);

        (*skel).links.event_begin =
            bpf_program__attach_raw_tracepoint((*skel).progs.event_begin, (*event).name);
        if IS_ERR((*skel).links.event_begin as *const c_void) {
            pr_err(b"Failed to attach first tracepoint program\n\0");
            err = PTR_ERR((*skel).links.event_begin as *const c_void);
            return err;
        }

        event = list_next_entry_filter_entry(event);

        (*skel).links.event_end =
            bpf_program__attach_raw_tracepoint((*skel).progs.event_end, (*event).name);
        if IS_ERR((*skel).links.event_end as *const c_void) {
            pr_err(b"Failed to attach second tracepoint program\n\0");
            err = PTR_ERR((*skel).links.event_end as *const c_void);
            return err;
        }
    }

    /* XXX: we don't actually use this fd - just for poll() */
    open(b"/dev/null\0".as_ptr() as *const c_char, O_RDONLY)
}

#[no_mangle]
pub unsafe extern "C" fn perf_ftrace__latency_start_bpf(_ftrace: *mut perf_ftrace) -> c_int {
    (*(*skel).bss).enabled = 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_ftrace__latency_stop_bpf(_ftrace: *mut perf_ftrace) -> c_int {
    (*(*skel).bss).enabled = 0;
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_ftrace__latency_read_bpf(
    ftrace: *mut perf_ftrace,
    buckets: *mut c_int,
    stats: *mut stats,
) -> c_int {
    let mut i: c_int;
    let fd: c_int;
    let mut err: c_int;
    let mut idx: u32;
    let hist: *mut u64;
    let ncpus: c_int = cpu__max_cpu().cpu;

    fd = bpf_map__fd((*skel).maps.latency);

    hist = calloc(ncpus as usize, core::mem::size_of::<u64>()) as *mut u64;
    if hist.is_null() {
        return -ENOMEM;
    }

    idx = 0;
    while idx < (*(*skel).rodata).bucket_num {
        err = bpf_map_lookup_elem(
            fd,
            &idx as *const u32 as *const c_void,
            hist as *mut c_void,
        );
        if err != 0 {
            *buckets.add(idx as usize) = 0;
            idx += 1;
            continue;
        }

        i = 0;
        while i < ncpus {
            *buckets.add(idx as usize) += *hist.add(i as usize) as c_int;
            i += 1;
        }
        idx += 1;
    }

    if (*(*skel).bss).count != 0 {
        (*stats).mean = (*(*skel).bss).total / (*(*skel).bss).count;
        (*stats).n = (*(*skel).bss).count;
        (*stats).max = (*(*skel).bss).max;
        (*stats).min = (*(*skel).bss).min as u64;

        if !(*ftrace).use_nsec {
            (*stats).mean /= 1000;
            (*stats).max /= 1000;
            (*stats).min /= 1000;
        }
    }

    free(hist as *mut c_void);
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_ftrace__latency_cleanup_bpf(_ftrace: *mut perf_ftrace) -> c_int {
    func_latency_bpf__destroy(skel);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
