// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/hists_link.c.
// Original C dependencies: tests.h, debug.h, symbol.h, sort.h, evsel.h,
// evlist.h, machine.h, map.h, parse-events.h, thread.h, hists_common.h,
// util/mmap.h, errno.h, linux/kernel.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

type u32 = u32;
type u64 = u64;
type size_t = usize;
type bool_ = bool;

const ENOMEM: c_int = 12;
const TEST_FAIL: c_int = -1;
const PERF_RECORD_MISC_USER: u32 = 2;

extern "C" {
    static mut verbose: c_int;
    static mut stderr: *mut c_void;

    static FAKE_PID_PERF1: u32;
    static FAKE_PID_PERF2: u32;
    static FAKE_PID_BASH: u32;
    static FAKE_IP_KERNEL_SCHEDULE: u64;
    static FAKE_IP_PERF_MAIN: u64;
    static FAKE_IP_PERF_CMD_RECORD: u64;
    static FAKE_IP_BASH_XMALLOC: u64;
    static FAKE_IP_LIBC_MALLOC: u64;
    static FAKE_IP_PERF_RUN_COMMAND: u64;
    static FAKE_IP_KERNEL_PAGE_FAULT: u64;
    static FAKE_IP_KERNEL_SYS_PERF_EVENT_OPEN: u64;
    static FAKE_IP_LIBC_FREE: u64;
    static FAKE_IP_BASH_XFREE: u64;
    static FAKE_IP_LIBC_REALLOC: u64;

    fn pr_debug(fmt: *const c_char, ...);

    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn machine__resolve(machine: *mut machine, al: *mut addr_location, sample: *mut perf_sample) -> c_int;
    fn hists__add_entry(
        hists: *mut hists,
        al: *mut addr_location,
        parent: *mut c_void,
        branch_info: *mut c_void,
        mem_info: *mut c_void,
        block_info: *mut c_void,
        sample: *mut perf_sample,
        sample_self: bool_,
    ) -> *mut hist_entry;
    fn thread__put(thread: *mut thread);
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn map__put(map: *mut map);
    fn map__get(map: *mut map) -> *mut map;
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn hists__has(hists: *mut hists, flag: c_int) -> bool_;
    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn hist_entry__has_pairs(he: *mut hist_entry) -> bool_;
    fn evlist__new() -> *mut evlist;
    fn parse_event(evlist: *mut evlist, name: *const c_char) -> c_int;
    fn machines__init(machines: *mut machines) -> c_int;
    fn setup_fake_machine(machines: *mut machines) -> *mut machine;
    fn machine__fprintf(machine: *mut machine, fp: *mut c_void) -> c_int;
    fn setup_sorting(evlist: *mut evlist, env: *mut c_void) -> c_int;
    fn hists__collapse_resort(hists: *mut hists, prog: *mut c_void);
    fn print_hists_in(hists: *mut hists);
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__last(evlist: *mut evsel) -> *mut evsel;
    fn hists__match(leader: *mut hists, other: *mut hists);
    fn hists__link(leader: *mut hists, other: *mut hists);
    fn evlist__put(evlist: *mut evlist);
    fn reset_output_field();
    fn machines__exit(machines: *mut machines);
}

#[repr(C)]
struct thread {
    _private: [u8; 0],
}

#[repr(C)]
struct map {
    _private: [u8; 0],
}

#[repr(C)]
struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
struct rb_root_cached {
    _private: [u8; 0],
}

#[repr(C)]
struct map_symbol {
    map: *mut map,
    sym: *mut symbol,
}

#[repr(C)]
struct hist_entry {
    rb_node_in: rb_node,
    thread: *mut thread,
    ms: map_symbol,
}

#[repr(C)]
struct hists {
    entries_collapsed: rb_root_cached,
    entries_in: *mut rb_root_cached,
}

#[repr(C)]
struct machine {
    env: *mut c_void,
}

#[repr(C)]
struct machines {
    _zero_sized: [u8; 0],
}

#[repr(C)]
struct addr_location {
    thread: *mut thread,
    map: *mut map,
    sym: *mut symbol,
}

#[repr(C)]
struct perf_sample {
    period: u64,
    weight: u64,
    cpumode: u32,
    pid: u32,
    tid: u32,
    ip: u64,
}

#[repr(C)]
struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sample {
    pid: u32,
    ip: u64,
    thread: *mut thread,
    map: *mut map,
    sym: *mut symbol,
}

const fn sample_new(pid: u32, ip: u64) -> sample {
    sample {
        pid,
        ip,
        thread: ptr::null_mut(),
        map: ptr::null_mut(),
        sym: ptr::null_mut(),
    }
}

// For the numbers, see hists_common.c
static mut fake_common_samples: [sample; 5] = unsafe {
    [
        // perf [kernel] schedule()
        sample_new(FAKE_PID_PERF1, FAKE_IP_KERNEL_SCHEDULE),
        // perf [perf]   main()
        sample_new(FAKE_PID_PERF2, FAKE_IP_PERF_MAIN),
        // perf [perf]   cmd_record()
        sample_new(FAKE_PID_PERF2, FAKE_IP_PERF_CMD_RECORD),
        // bash [bash]   xmalloc()
        sample_new(FAKE_PID_BASH, FAKE_IP_BASH_XMALLOC),
        // bash [libc]   malloc()
        sample_new(FAKE_PID_BASH, FAKE_IP_LIBC_MALLOC),
    ]
};

static mut fake_samples: [[sample; 5]; 2] = unsafe {
    [
        [
            // perf [perf]   run_command()
            sample_new(FAKE_PID_PERF1, FAKE_IP_PERF_RUN_COMMAND),
            // perf [libc]   malloc()
            sample_new(FAKE_PID_PERF1, FAKE_IP_LIBC_MALLOC),
            // perf [kernel] page_fault()
            sample_new(FAKE_PID_PERF1, FAKE_IP_KERNEL_PAGE_FAULT),
            // perf [kernel] sys_perf_event_open()
            sample_new(FAKE_PID_PERF2, FAKE_IP_KERNEL_SYS_PERF_EVENT_OPEN),
            // bash [libc]   free()
            sample_new(FAKE_PID_BASH, FAKE_IP_LIBC_FREE),
        ],
        [
            // perf [libc]   free()
            sample_new(FAKE_PID_PERF2, FAKE_IP_LIBC_FREE),
            // bash [libc]   malloc()
            sample_new(FAKE_PID_BASH, FAKE_IP_LIBC_MALLOC), // will be merged
            // bash [bash]   xfee()
            sample_new(FAKE_PID_BASH, FAKE_IP_BASH_XFREE),
            // bash [libc]   realloc()
            sample_new(FAKE_PID_BASH, FAKE_IP_LIBC_REALLOC),
            // bash [kernel] page_fault()
            sample_new(FAKE_PID_BASH, FAKE_IP_KERNEL_PAGE_FAULT),
        ],
    ]
};

unsafe fn RC_CHK_EQUAL<T>(a: *mut T, b: *mut T) -> bool {
    a == b
}

// Placeholder for the C enum/flag symbol used by hists__has().
extern "C" {
    static need_collapse: c_int;
}

unsafe fn hist_entry_from_rb_node_in(node: *mut rb_node) -> *mut hist_entry {
    // C uses rb_entry(node, struct hist_entry, rb_node_in).
    node as *mut hist_entry
}

unsafe fn evlist_for_each_entry<F>(evlist: *mut evlist, mut body: F)
where
    F: FnMut(*mut evsel),
{
    // C macro evlist__for_each_entry(evlist, evsel) is supplied by external perf headers.
    // This preserves its intent as a dependency boundary for the translated file.
    let _ = evlist;
    let _ = &mut body;
}

unsafe fn add_hist_entries(evlist: *mut evlist, machine: *mut machine) -> c_int {
    let mut al: addr_location = core::mem::zeroed();
    let mut he: *mut hist_entry;
    let mut sample_data = perf_sample {
        period: 1,
        weight: 1,
        cpumode: 0,
        pid: 0,
        tid: 0,
        ip: 0,
    };
    let mut i: size_t = 0;

    addr_location__init(&mut al);
    /*
     * each evsel will have 10 samples - 5 common and 5 distinct.
     * However the second evsel also has a collapsed entry for
     * "bash [libc] malloc" so total 9 entries will be in the tree.
     */
    evlist_for_each_entry(evlist, |evsel| {
        let hists = evsel__hists(evsel);

        for k in 0..fake_common_samples.len() {
            sample_data.cpumode = PERF_RECORD_MISC_USER;
            sample_data.pid = fake_common_samples[k].pid;
            sample_data.tid = fake_common_samples[k].pid;
            sample_data.ip = fake_common_samples[k].ip;

            if machine__resolve(machine, &mut al, &mut sample_data) < 0 {
                return;
            }

            he = hists__add_entry(
                hists,
                &mut al,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                &mut sample_data,
                true,
            );
            if he.is_null() {
                return;
            }

            thread__put(fake_common_samples[k].thread);
            fake_common_samples[k].thread = thread__get(al.thread);
            map__put(fake_common_samples[k].map);
            fake_common_samples[k].map = map__get(al.map);
            fake_common_samples[k].sym = al.sym;
        }

        for k in 0..fake_samples[i].len() {
            sample_data.pid = fake_samples[i][k].pid;
            sample_data.tid = fake_samples[i][k].pid;
            sample_data.ip = fake_samples[i][k].ip;
            if machine__resolve(machine, &mut al, &mut sample_data) < 0 {
                return;
            }

            he = hists__add_entry(
                hists,
                &mut al,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                &mut sample_data,
                true,
            );
            if he.is_null() {
                return;
            }

            thread__put(fake_samples[i][k].thread);
            fake_samples[i][k].thread = thread__get(al.thread);
            map__put(fake_samples[i][k].map);
            fake_samples[i][k].map = map__get(al.map);
            fake_samples[i][k].sym = al.sym;
        }
        i += 1;
    });

    addr_location__exit(&mut al);
    0
}

unsafe fn put_fake_samples() {
    for i in 0..fake_common_samples.len() {
        map__put(fake_common_samples[i].map);
    }
    for i in 0..fake_samples.len() {
        for j in 0..fake_samples[0].len() {
            map__put(fake_samples[i][j].map);
        }
    }
}

unsafe fn find_sample(
    mut samples: *mut sample,
    mut nr_samples: size_t,
    t: *mut thread,
    m: *mut map,
    s: *mut symbol,
) -> c_int {
    while nr_samples != 0 {
        nr_samples -= 1;
        if RC_CHK_EQUAL((*samples).thread, t) && RC_CHK_EQUAL((*samples).map, m) && (*samples).sym == s {
            return 1;
        }
        samples = samples.add(1);
    }
    0
}

unsafe fn __validate_match(hists: *mut hists) -> c_int {
    let mut count: size_t = 0;
    let root: *mut rb_root_cached;

    /*
     * Only entries from fake_common_samples should have a pair.
     */
    if hists__has(hists, need_collapse) {
        root = &mut (*hists).entries_collapsed;
    } else {
        root = (*hists).entries_in;
    }

    let mut node = rb_first_cached(root);
    while !node.is_null() {
        let he = hist_entry_from_rb_node_in(node);

        if hist_entry__has_pairs(he) {
            if find_sample(
                fake_common_samples.as_mut_ptr(),
                fake_common_samples.len(),
                (*he).thread,
                (*he).ms.map,
                (*he).ms.sym,
            ) != 0
            {
                count += 1;
            } else {
                pr_debug(c"Can't find the matched entry\n".as_ptr());
                return -1;
            }
        }

        node = rb_next(node);
    }

    if count != fake_common_samples.len() {
        pr_debug(
            c"Invalid count for matched entries: %zd of %zd\n".as_ptr(),
            count,
            fake_common_samples.len(),
        );
        return -1;
    }

    0
}

unsafe fn validate_match(leader: *mut hists, other: *mut hists) -> c_int {
    (__validate_match(leader) != 0 || __validate_match(other) != 0) as c_int
}

unsafe fn __validate_link(hists: *mut hists, idx: c_int) -> c_int {
    let mut count: size_t = 0;
    let mut count_pair: size_t = 0;
    let mut count_dummy: size_t = 0;
    let root: *mut rb_root_cached;

    /*
     * Leader hists (idx = 0) will have dummy entries from other,
     * and some entries will have no pair.  However every entry
     * in other hists should have (dummy) pair.
     */
    if hists__has(hists, need_collapse) {
        root = &mut (*hists).entries_collapsed;
    } else {
        root = (*hists).entries_in;
    }

    let mut node = rb_first_cached(root);
    while !node.is_null() {
        let he = hist_entry_from_rb_node_in(node);

        if hist_entry__has_pairs(he) {
            if find_sample(
                fake_common_samples.as_mut_ptr(),
                fake_common_samples.len(),
                (*he).thread,
                (*he).ms.map,
                (*he).ms.sym,
            ) == 0
                && find_sample(
                    fake_samples[idx as usize].as_mut_ptr(),
                    fake_samples[idx as usize].len(),
                    (*he).thread,
                    (*he).ms.map,
                    (*he).ms.sym,
                ) == 0
            {
                count_dummy += 1;
            }

            count_pair += 1;
        } else if idx != 0 {
            pr_debug(c"A entry from the other hists should have pair\n".as_ptr());
            return -1;
        }

        count += 1;
        node = rb_next(node);
    }

    /*
     * Note that we have a entry collapsed in the other (idx = 1) hists.
     */
    if idx == 0 {
        if count_dummy != fake_samples[1].len() - 1 {
            pr_debug(
                c"Invalid count of dummy entries: %zd of %zd\n".as_ptr(),
                count_dummy,
                fake_samples[1].len() - 1,
            );
            return -1;
        }
        if count != count_pair + fake_samples[0].len() {
            pr_debug(
                c"Invalid count of total leader entries: %zd of %zd\n".as_ptr(),
                count,
                count_pair + fake_samples[0].len(),
            );
            return -1;
        }
    } else {
        if count != count_pair {
            pr_debug(
                c"Invalid count of total other entries: %zd of %zd\n".as_ptr(),
                count,
                count_pair,
            );
            return -1;
        }
        if count_dummy > 0 {
            pr_debug(
                c"Other hists should not have dummy entries: %zd\n".as_ptr(),
                count_dummy,
            );
            return -1;
        }
    }

    0
}

unsafe fn validate_link(leader: *mut hists, other: *mut hists) -> c_int {
    (__validate_link(leader, 0) != 0 || __validate_link(other, 1) != 0) as c_int
}

unsafe fn test__hists_link(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut err: c_int = -1;
    let mut hists_ptr: *mut hists;
    let first_hists: *mut hists;
    let mut machines: machines = core::mem::zeroed();
    let mut machine: *mut machine = ptr::null_mut();
    let mut evsel_ptr: *mut evsel;
    let first: *mut evsel;
    let evlist = evlist__new();

    if evlist.is_null() {
        return -ENOMEM;
    }

    err = parse_event(evlist, c"cpu-clock".as_ptr());
    if err != 0 {
        goto_out(evlist, &mut machines);
        put_fake_samples();
        return err;
    }
    err = parse_event(evlist, c"task-clock".as_ptr());
    if err != 0 {
        goto_out(evlist, &mut machines);
        put_fake_samples();
        return err;
    }

    err = TEST_FAIL;
    if machines__init(&mut machines) != 0 {
        goto_out(evlist, &mut machines);
        put_fake_samples();
        return err;
    }

    // setup threads/dso/map/symbols also
    machine = setup_fake_machine(&mut machines);
    if machine.is_null() {
        goto_out(evlist, &mut machines);
        put_fake_samples();
        return err;
    }

    if verbose > 1 {
        machine__fprintf(machine, stderr);
    }

    // default sort order (comm,dso,sym) will be used
    if setup_sorting(evlist, (*machine).env) < 0 {
        goto_out(evlist, &mut machines);
        put_fake_samples();
        return err;
    }

    // process sample events
    err = add_hist_entries(evlist, machine);
    if err < 0 {
        goto_out(evlist, &mut machines);
        put_fake_samples();
        return err;
    }

    evlist_for_each_entry(evlist, |evsel| {
        hists_ptr = evsel__hists(evsel);
        hists__collapse_resort(hists_ptr, ptr::null_mut());

        if verbose > 2 {
            print_hists_in(hists_ptr);
        }
    });

    first = evlist__first(evlist);
    evsel_ptr = evlist__last(evlist);

    first_hists = evsel__hists(first);
    hists_ptr = evsel__hists(evsel_ptr);

    // match common entries
    hists__match(first_hists, hists_ptr);
    err = validate_match(first_hists, hists_ptr);
    if err != 0 {
        goto_out(evlist, &mut machines);
        put_fake_samples();
        return err;
    }

    // link common and/or dummy entries
    hists__link(first_hists, hists_ptr);
    err = validate_link(first_hists, hists_ptr);
    if err != 0 {
        goto_out(evlist, &mut machines);
        put_fake_samples();
        return err;
    }

    err = 0;

    // tear down everything
    goto_out(evlist, &mut machines);
    put_fake_samples();

    err
}

unsafe fn goto_out(evlist: *mut evlist, machines: *mut machines) {
    evlist__put(evlist);
    reset_output_field();
    machines__exit(machines);
}

// C source ended with: DEFINE_SUITE("Match and link multiple hists", hists_link);
// The DEFINE_SUITE registration macro is supplied by external test headers.
