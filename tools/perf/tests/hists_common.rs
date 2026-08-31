// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/hists_common.c.
// C dependencies: inttypes.h, util/debug.h, util/dso.h, util/event.h,
// util/map.h, util/symbol.h, util/sort.h, util/evsel.h, util/machine.h,
// util/thread.h, tests/hists_common.h, linux/kernel.h, linux/perf_event.h.

use core::ffi::{c_char, c_int, c_ulonglong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type u32 = u32;
type u64 = u64;
type size_t = usize;

const PERF_RECORD_MISC_USER: u32 = 1 << 0;
const STB_GLOBAL: u8 = 1;
const STT_FUNC: u8 = 2;

extern "C" {
    static HOST_KERNEL_ID: *const c_char;

    static FAKE_PID_PERF1: u32;
    static FAKE_PID_PERF2: u32;
    static FAKE_PID_BASH: u32;
    static FAKE_MAP_PERF: u64;
    static FAKE_MAP_LIBC: u64;
    static FAKE_MAP_KERNEL: u64;
    static FAKE_MAP_BASH: u64;
    static FAKE_MAP_LENGTH: u64;
    static FAKE_SYM_OFFSET1: u64;
    static FAKE_SYM_OFFSET2: u64;
    static FAKE_SYM_OFFSET3: u64;
    static FAKE_SYM_LENGTH: u64;

    fn machines__find(machines: *mut machines, id: *const c_char) -> *mut machine;
    fn machine__findnew_thread(machine: *mut machine, pid: u32, tid: u32) -> *mut thread;
    fn thread__set_comm(thread: *mut thread, comm: *const c_char, exec: u64) -> c_int;
    fn thread__put(thread: *mut thread);
    fn machine__process_mmap_event(
        machine: *mut machine,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> c_int;
    fn machine__findnew_dso(machine: *mut machine, filename: *const c_char) -> *mut dso;
    fn dso__set_loaded(dso: *mut dso);
    fn symbol__new(
        start: u64,
        len: u64,
        binding: u8,
        type_: u8,
        name: *const c_char,
    ) -> *mut symbol;
    fn symbols__insert(symbols: *mut c_void, sym: *mut symbol);
    fn dso__symbols(dso: *mut dso) -> *mut c_void;
    fn dso__put(dso: *mut dso);
    fn machine__delete_threads(machine: *mut machine);
    fn hists__has(hists: *mut hists, flag: c_int) -> bool;
    static need_collapse: c_int;
    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn map__dso(map: *mut map) -> *mut dso;
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn thread__tid(thread: *mut thread) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
}

#[repr(C)]
pub struct machines {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    pub cpumode: u32,
}

#[repr(C)]
pub struct mmap_event {
    pub pid: u32,
    pub tid: u32,
    pub start: u64,
    pub len: u64,
    pub pgoff: u64,
    pub filename: [c_char; 4096],
}

#[repr(C)]
pub union perf_event {
    pub mmap: core::mem::ManuallyDrop<mmap_event>,
}

#[repr(C)]
pub struct hist_entry_stat {
    pub period: u64,
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct hist_entry {
    pub rb_node: rb_node,
    pub rb_node_in: rb_node,
    pub filtered: bool,
    pub thread: *mut thread,
    pub ms: map_symbol,
    pub stat: hist_entry_stat,
    pub stat_acc: *mut hist_entry_stat,
}

#[repr(C)]
pub struct hists {
    pub entries: rb_root_cached,
    pub entries_collapsed: rb_root_cached,
    pub entries_in: *mut rb_root_cached,
}

#[repr(C)]
struct fake_thread {
    pid: u32,
    comm: *const c_char,
}

#[repr(C)]
struct fake_mmap {
    pid: u32,
    start: u64,
    filename: *const c_char,
}

#[repr(C)]
struct fake_sym {
    start: u64,
    length: u64,
    name: *const c_char,
}

#[repr(C)]
struct fake_symbol {
    dso_name: *const c_char,
    syms: *mut fake_sym,
    nr_syms: size_t,
}

static mut fake_threads: [fake_thread; 3] = [
    fake_thread { pid: unsafe { FAKE_PID_PERF1 }, comm: b"perf\0".as_ptr() as *const c_char },
    fake_thread { pid: unsafe { FAKE_PID_PERF2 }, comm: b"perf\0".as_ptr() as *const c_char },
    fake_thread { pid: unsafe { FAKE_PID_BASH }, comm: b"bash\0".as_ptr() as *const c_char },
];

static mut fake_mmap_info: [fake_mmap; 9] = [
    fake_mmap { pid: unsafe { FAKE_PID_PERF1 }, start: unsafe { FAKE_MAP_PERF }, filename: b"perf\0".as_ptr() as *const c_char },
    fake_mmap { pid: unsafe { FAKE_PID_PERF1 }, start: unsafe { FAKE_MAP_LIBC }, filename: b"libc\0".as_ptr() as *const c_char },
    fake_mmap { pid: unsafe { FAKE_PID_PERF1 }, start: unsafe { FAKE_MAP_KERNEL }, filename: b"[kernel]\0".as_ptr() as *const c_char },
    fake_mmap { pid: unsafe { FAKE_PID_PERF2 }, start: unsafe { FAKE_MAP_PERF }, filename: b"perf\0".as_ptr() as *const c_char },
    fake_mmap { pid: unsafe { FAKE_PID_PERF2 }, start: unsafe { FAKE_MAP_LIBC }, filename: b"libc\0".as_ptr() as *const c_char },
    fake_mmap { pid: unsafe { FAKE_PID_PERF2 }, start: unsafe { FAKE_MAP_KERNEL }, filename: b"[kernel]\0".as_ptr() as *const c_char },
    fake_mmap { pid: unsafe { FAKE_PID_BASH }, start: unsafe { FAKE_MAP_BASH }, filename: b"bash\0".as_ptr() as *const c_char },
    fake_mmap { pid: unsafe { FAKE_PID_BASH }, start: unsafe { FAKE_MAP_LIBC }, filename: b"libc\0".as_ptr() as *const c_char },
    fake_mmap { pid: unsafe { FAKE_PID_BASH }, start: unsafe { FAKE_MAP_KERNEL }, filename: b"[kernel]\0".as_ptr() as *const c_char },
];

static mut perf_syms: [fake_sym; 3] = [
    fake_sym { start: unsafe { FAKE_SYM_OFFSET1 }, length: unsafe { FAKE_SYM_LENGTH }, name: b"main\0".as_ptr() as *const c_char },
    fake_sym { start: unsafe { FAKE_SYM_OFFSET2 }, length: unsafe { FAKE_SYM_LENGTH }, name: b"run_command\0".as_ptr() as *const c_char },
    fake_sym { start: unsafe { FAKE_SYM_OFFSET3 }, length: unsafe { FAKE_SYM_LENGTH }, name: b"cmd_record\0".as_ptr() as *const c_char },
];

static mut bash_syms: [fake_sym; 3] = [
    fake_sym { start: unsafe { FAKE_SYM_OFFSET1 }, length: unsafe { FAKE_SYM_LENGTH }, name: b"main\0".as_ptr() as *const c_char },
    fake_sym { start: unsafe { FAKE_SYM_OFFSET2 }, length: unsafe { FAKE_SYM_LENGTH }, name: b"xmalloc\0".as_ptr() as *const c_char },
    fake_sym { start: unsafe { FAKE_SYM_OFFSET3 }, length: unsafe { FAKE_SYM_LENGTH }, name: b"xfree\0".as_ptr() as *const c_char },
];

static mut libc_syms: [fake_sym; 6] = [
    fake_sym { start: 700, length: 100, name: b"malloc\0".as_ptr() as *const c_char },
    fake_sym { start: 800, length: 100, name: b"free\0".as_ptr() as *const c_char },
    fake_sym { start: 900, length: 100, name: b"realloc\0".as_ptr() as *const c_char },
    fake_sym { start: unsafe { FAKE_SYM_OFFSET1 }, length: unsafe { FAKE_SYM_LENGTH }, name: b"malloc\0".as_ptr() as *const c_char },
    fake_sym { start: unsafe { FAKE_SYM_OFFSET2 }, length: unsafe { FAKE_SYM_LENGTH }, name: b"free\0".as_ptr() as *const c_char },
    fake_sym { start: unsafe { FAKE_SYM_OFFSET3 }, length: unsafe { FAKE_SYM_LENGTH }, name: b"realloc\0".as_ptr() as *const c_char },
];

static mut kernel_syms: [fake_sym; 3] = [
    fake_sym { start: unsafe { FAKE_SYM_OFFSET1 }, length: unsafe { FAKE_SYM_LENGTH }, name: b"schedule\0".as_ptr() as *const c_char },
    fake_sym { start: unsafe { FAKE_SYM_OFFSET2 }, length: unsafe { FAKE_SYM_LENGTH }, name: b"page_fault\0".as_ptr() as *const c_char },
    fake_sym { start: unsafe { FAKE_SYM_OFFSET3 }, length: unsafe { FAKE_SYM_LENGTH }, name: b"sys_perf_event_open\0".as_ptr() as *const c_char },
];

static mut fake_symbols: [fake_symbol; 4] = [
    fake_symbol { dso_name: b"perf\0".as_ptr() as *const c_char, syms: unsafe { perf_syms.as_mut_ptr() }, nr_syms: 3 },
    fake_symbol { dso_name: b"bash\0".as_ptr() as *const c_char, syms: unsafe { bash_syms.as_mut_ptr() }, nr_syms: 3 },
    fake_symbol { dso_name: b"libc\0".as_ptr() as *const c_char, syms: unsafe { libc_syms.as_mut_ptr() }, nr_syms: 6 },
    fake_symbol { dso_name: b"[kernel]\0".as_ptr() as *const c_char, syms: unsafe { kernel_syms.as_mut_ptr() }, nr_syms: 3 },
];

unsafe fn rb_entry_hist_entry_rb_node_in(node: *mut rb_node) -> *mut hist_entry {
    (node as *mut u8).sub(core::mem::offset_of!(hist_entry, rb_node_in)) as *mut hist_entry
}

unsafe fn rb_entry_hist_entry_rb_node(node: *mut rb_node) -> *mut hist_entry {
    (node as *mut u8).sub(core::mem::offset_of!(hist_entry, rb_node)) as *mut hist_entry
}

#[no_mangle]
pub unsafe extern "C" fn setup_fake_machine(machines: *mut machines) -> *mut machine {
    let machine = machines__find(machines, HOST_KERNEL_ID);

    if machine.is_null() {
        pr_debug(b"Not enough memory for machine setup\n\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    let mut i: size_t = 0;
    while i < fake_threads.len() {
        let thread = machine__findnew_thread(machine, fake_threads[i].pid, fake_threads[i].pid);
        if thread.is_null() {
            goto_out(machine);
            return ptr::null_mut();
        }

        thread__set_comm(thread, fake_threads[i].comm, 0);
        thread__put(thread);
        i += 1;
    }

    i = 0;
    while i < fake_mmap_info.len() {
        let mut sample = perf_sample {
            cpumode: PERF_RECORD_MISC_USER,
        };
        let mut fake_mmap_event = MaybeUninit::<perf_event>::zeroed().assume_init();
        {
            let mmap = &mut fake_mmap_event.mmap;
            mmap.pid = fake_mmap_info[i].pid;
            mmap.tid = fake_mmap_info[i].pid;
            mmap.start = fake_mmap_info[i].start;
            mmap.len = FAKE_MAP_LENGTH;
            mmap.pgoff = 0 as c_ulonglong as u64;
        }

        strcpy(
            fake_mmap_event.mmap.filename.as_mut_ptr(),
            fake_mmap_info[i].filename,
        );

        machine__process_mmap_event(machine, &mut fake_mmap_event, &mut sample);
        i += 1;
    }

    i = 0;
    while i < fake_symbols.len() {
        let dso = machine__findnew_dso(machine, fake_symbols[i].dso_name);
        if dso.is_null() {
            goto_out(machine);
            return ptr::null_mut();
        }

        /* emulate dso__load() */
        dso__set_loaded(dso);

        let mut k: size_t = 0;
        while k < fake_symbols[i].nr_syms {
            let fsym = fake_symbols[i].syms.add(k);

            let sym = symbol__new(
                (*fsym).start,
                (*fsym).length,
                STB_GLOBAL,
                STT_FUNC,
                (*fsym).name,
            );
            if sym.is_null() {
                dso__put(dso);
                goto_out(machine);
                return ptr::null_mut();
            }

            symbols__insert(dso__symbols(dso), sym);
            k += 1;
        }

        dso__put(dso);
        i += 1;
    }

    machine
}

unsafe fn goto_out(machine: *mut machine) {
    pr_debug(b"Not enough memory for machine setup\n\0".as_ptr() as *const c_char);
    machine__delete_threads(machine);
}

#[no_mangle]
pub unsafe extern "C" fn print_hists_in(hists: *mut hists) {
    let mut i: c_int = 0;
    let root: *mut rb_root_cached;
    let mut node: *mut rb_node;

    if hists__has(hists, need_collapse) {
        root = &mut (*hists).entries_collapsed;
    } else {
        root = (*hists).entries_in;
    }

    pr_info(b"----- %s --------\n\0".as_ptr() as *const c_char, b"print_hists_in\0".as_ptr() as *const c_char);
    node = rb_first_cached(root);
    while !node.is_null() {
        let he = rb_entry_hist_entry_rb_node_in(node);

        if !(*he).filtered {
            let dso = map__dso((*he).ms.map);

            pr_info(
                b"%2d: entry: %-8s [%-8s] %20s: period = %llu\n\0".as_ptr() as *const c_char,
                i,
                thread__comm_str((*he).thread),
                dso__short_name(dso),
                (*(*he).ms.sym).name,
                (*he).stat.period as c_ulonglong,
            );
        }

        i += 1;
        node = rb_next(node);
    }
}

#[no_mangle]
pub unsafe extern "C" fn print_hists_out(hists: *mut hists) {
    let mut i: c_int = 0;
    let root: *mut rb_root_cached;
    let mut node: *mut rb_node;

    root = &mut (*hists).entries;

    pr_info(b"----- %s --------\n\0".as_ptr() as *const c_char, b"print_hists_out\0".as_ptr() as *const c_char);
    node = rb_first_cached(root);
    while !node.is_null() {
        let he = rb_entry_hist_entry_rb_node(node);

        if !(*he).filtered {
            let dso = map__dso((*he).ms.map);

            pr_info(
                b"%2d: entry: %8s:%5d [%-8s] %20s: period = %llu/%llu\n\0".as_ptr() as *const c_char,
                i,
                thread__comm_str((*he).thread),
                thread__tid((*he).thread),
                dso__short_name(dso),
                (*(*he).ms.sym).name,
                (*he).stat.period as c_ulonglong,
                if !(*he).stat_acc.is_null() {
                    (*(*he).stat_acc).period
                } else {
                    0
                } as c_ulonglong,
            );
        }

        i += 1;
        node = rb_next(node);
    }
}
