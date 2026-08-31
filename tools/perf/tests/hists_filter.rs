// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/hists_filter.c.
// C includes referenced: util/debug.h, util/map.h, util/symbol.h, util/sort.h,
// util/evsel.h, util/event.h, util/evlist.h, util/machine.h,
// util/parse-events.h, util/thread.h, tests/tests.h, tests/hists_common.h,
// linux/kernel.h.

use core::ffi::{c_char, c_int, c_void};
use core::mem::zeroed;
use core::ptr::{null, null_mut};

#[repr(C)]
struct sample {
    pid: u32,
    ip: u64,
    thread: *mut thread,
    map: *mut map,
    sym: *mut symbol,
    socket: c_int,
}

unsafe extern "C" {
    static mut verbose: c_int;
    static mut stderr: *mut c_void;
    static mut sysctl_perf_event_max_stack: c_int;
    static hist_iter_normal: hist_iter_ops;

    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn machine__resolve(
        machine: *mut machine,
        al: *mut addr_location,
        sample: *mut perf_sample,
    ) -> c_int;
    fn hist_entry_iter__add(
        iter: *mut hist_entry_iter,
        al: *mut addr_location,
        max_stack: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn thread__put(thread: *mut thread);
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn map__put(map: *mut map);
    fn map__get(map: *mut map) -> *mut map;
    fn map__dso(map: *mut map) -> *mut dso;
    fn parse_event(evlist: *mut evlist, event: *const c_char) -> c_int;
    fn machines__init(machines: *mut machines) -> c_int;
    fn machines__exit(machines: *mut machines);
    fn setup_fake_machine(machines: *mut machines) -> *mut machine;
    fn machine__fprintf(machine: *mut machine, fp: *mut c_void);
    fn setup_sorting(evlist: *mut evlist, env: *mut perf_env) -> c_int;
    fn hists__collapse_resort(hists: *mut hists, arg: *mut c_void);
    fn evsel__output_resort(evsel: *mut evsel, arg: *mut c_void);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn print_hists_out(hists: *mut hists);
    fn hists__filter_by_thread(hists: *mut hists);
    fn hists__filter_by_dso(hists: *mut hists);
    fn hists__filter_by_symbol(hists: *mut hists);
    fn hists__filter_by_socket(hists: *mut hists);
    fn reset_output_field();
}

// External repository types and constants are supplied by translated headers.
#[allow(non_camel_case_types)]
type size_t = usize;

const TEST_FAIL: c_int = -1;
const TEST_OK: c_int = 0;
const PERF_RECORD_MISC_USER: u32 = 2;

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
struct dso {
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
struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
struct machine {
    env: *mut perf_env,
}

#[repr(C)]
struct machines {
    _private: [u8; 0],
}

#[repr(C)]
struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
struct hist_iter_ops {
    _private: [u8; 0],
}

#[repr(C)]
struct perf_sample {
    period: u64,
    evsel: *mut evsel,
    cpumode: u32,
    pid: u32,
    tid: u32,
    ip: u64,
}

#[repr(C)]
struct addr_location {
    thread: *mut thread,
    map: *mut map,
    sym: *mut symbol,
    socket: c_int,
}

#[repr(C)]
struct hist_entry_iter {
    sample: *mut perf_sample,
    ops: *const hist_iter_ops,
    hide_unresolved: bool,
}

#[repr(C)]
struct hists_stats {
    nr_samples: u64,
    total_period: u64,
    nr_non_filtered_samples: u64,
    total_non_filtered_period: u64,
}

#[repr(C)]
struct hists {
    thread_filter: *mut thread,
    dso_filter: *mut dso,
    symbol_filter_str: *const c_char,
    socket_filter: c_int,
    stats: hists_stats,
    nr_entries: u64,
    nr_non_filtered_entries: u64,
}

// For the numbers, see hists_common.c
static mut fake_samples: [sample; 10] = [
    // perf [kernel] schedule()
    sample {
        pid: FAKE_PID_PERF1,
        ip: FAKE_IP_KERNEL_SCHEDULE,
        thread: null_mut(),
        map: null_mut(),
        sym: null_mut(),
        socket: 0,
    },
    // perf [perf]   main()
    sample {
        pid: FAKE_PID_PERF1,
        ip: FAKE_IP_PERF_MAIN,
        thread: null_mut(),
        map: null_mut(),
        sym: null_mut(),
        socket: 0,
    },
    // perf [libc]   malloc()
    sample {
        pid: FAKE_PID_PERF1,
        ip: FAKE_IP_LIBC_MALLOC,
        thread: null_mut(),
        map: null_mut(),
        sym: null_mut(),
        socket: 0,
    },
    // perf [perf]   main()
    sample {
        pid: FAKE_PID_PERF2,
        ip: FAKE_IP_PERF_MAIN,
        thread: null_mut(),
        map: null_mut(),
        sym: null_mut(),
        socket: 0,
    }, // will be merged
    // perf [perf]   cmd_record()
    sample {
        pid: FAKE_PID_PERF2,
        ip: FAKE_IP_PERF_CMD_RECORD,
        thread: null_mut(),
        map: null_mut(),
        sym: null_mut(),
        socket: 1,
    },
    // perf [kernel] page_fault()
    sample {
        pid: FAKE_PID_PERF2,
        ip: FAKE_IP_KERNEL_PAGE_FAULT,
        thread: null_mut(),
        map: null_mut(),
        sym: null_mut(),
        socket: 1,
    },
    // bash [bash]   main()
    sample {
        pid: FAKE_PID_BASH,
        ip: FAKE_IP_BASH_MAIN,
        thread: null_mut(),
        map: null_mut(),
        sym: null_mut(),
        socket: 2,
    },
    // bash [bash]   xmalloc()
    sample {
        pid: FAKE_PID_BASH,
        ip: FAKE_IP_BASH_XMALLOC,
        thread: null_mut(),
        map: null_mut(),
        sym: null_mut(),
        socket: 2,
    },
    // bash [libc]   malloc()
    sample {
        pid: FAKE_PID_BASH,
        ip: FAKE_IP_LIBC_MALLOC,
        thread: null_mut(),
        map: null_mut(),
        sym: null_mut(),
        socket: 3,
    },
    // bash [kernel] page_fault()
    sample {
        pid: FAKE_PID_BASH,
        ip: FAKE_IP_KERNEL_PAGE_FAULT,
        thread: null_mut(),
        map: null_mut(),
        sym: null_mut(),
        socket: 3,
    },
];

unsafe fn add_hist_entries(evlist: *mut evlist, machine: *mut machine) -> c_int {
    let mut evsel: *mut evsel;
    let mut al: addr_location = zeroed();
    let mut sample: perf_sample = zeroed();
    sample.period = 100;

    addr_location__init(&mut al);
    /*
     * each evsel will have 10 samples but the 4th sample
     * (perf [perf] main) will be collapsed to an existing entry
     * so total 9 entries will be in the tree.
     */
    evlist__for_each_entry!(evlist, evsel, {
        let mut i: size_t = 0;
        while i < fake_samples.len() {
            let mut iter = hist_entry_iter {
                sample: &mut sample,
                ops: &hist_iter_normal,
                hide_unresolved: false,
            };
            let hists = evsel__hists(evsel);

            sample.evsel = evsel;
            // make sure it has no filter at first
            (*hists).thread_filter = null_mut();
            (*hists).dso_filter = null_mut();
            (*hists).symbol_filter_str = null();

            sample.cpumode = PERF_RECORD_MISC_USER;
            sample.pid = fake_samples[i].pid;
            sample.tid = fake_samples[i].pid;
            sample.ip = fake_samples[i].ip;

            if machine__resolve(machine, &mut al, &mut sample) < 0 {
                pr_debug(b"Not enough memory for adding a hist entry\n\0".as_ptr() as *const c_char);
                addr_location__exit(&mut al);
                return TEST_FAIL;
            }

            al.socket = fake_samples[i].socket;
            if hist_entry_iter__add(
                &mut iter,
                &mut al,
                sysctl_perf_event_max_stack,
                null_mut(),
            ) < 0
            {
                pr_debug(b"Not enough memory for adding a hist entry\n\0".as_ptr() as *const c_char);
                addr_location__exit(&mut al);
                return TEST_FAIL;
            }

            thread__put(fake_samples[i].thread);
            fake_samples[i].thread = thread__get(al.thread);
            map__put(fake_samples[i].map);
            fake_samples[i].map = map__get(al.map);
            fake_samples[i].sym = al.sym;
            i += 1;
        }
    });
    addr_location__exit(&mut al);
    0
}

unsafe fn put_fake_samples() {
    let mut i: size_t = 0;

    while i < fake_samples.len() {
        map__put(fake_samples[i].map);
        i += 1;
    }
}

unsafe fn test__hists_filter(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut err: c_int = TEST_FAIL;
    let mut machines: machines = zeroed();
    let mut machine: *mut machine;
    let mut evsel: *mut evsel;
    let evlist: *mut evlist = evlist__new();

    TEST_ASSERT_VAL!("No memory", !evlist.is_null());

    err = parse_event(evlist, b"cpu-clock\0".as_ptr() as *const c_char);
    if err != 0 {
        evlist__put(evlist);
        reset_output_field();
        machines__exit(&mut machines);
        put_fake_samples();
        return err;
    }
    err = parse_event(evlist, b"task-clock\0".as_ptr() as *const c_char);
    if err != 0 {
        evlist__put(evlist);
        reset_output_field();
        machines__exit(&mut machines);
        put_fake_samples();
        return err;
    }
    err = TEST_FAIL;

    if machines__init(&mut machines) != 0 {
        evlist__put(evlist);
        reset_output_field();
        machines__exit(&mut machines);
        put_fake_samples();
        return err;
    }

    // setup threads/dso/map/symbols also
    machine = setup_fake_machine(&mut machines);
    if machine.is_null() {
        evlist__put(evlist);
        reset_output_field();
        machines__exit(&mut machines);
        put_fake_samples();
        return err;
    }

    if verbose > 1 {
        machine__fprintf(machine, stderr);
    }

    // default sort order (comm,dso,sym) will be used
    if setup_sorting(evlist, (*machine).env) < 0 {
        evlist__put(evlist);
        reset_output_field();
        machines__exit(&mut machines);
        put_fake_samples();
        return err;
    }

    // process sample events
    err = add_hist_entries(evlist, machine);
    if err < 0 {
        evlist__put(evlist);
        reset_output_field();
        machines__exit(&mut machines);
        put_fake_samples();
        return err;
    }

    evlist__for_each_entry!(evlist, evsel, {
        let hists = evsel__hists(evsel);

        hists__collapse_resort(hists, null_mut());
        evsel__output_resort(evsel, null_mut());

        if verbose > 2 {
            pr_info(b"Normal histogram\n\0".as_ptr() as *const c_char);
            print_hists_out(hists);
        }

        TEST_ASSERT_VAL!("Invalid nr samples", (*hists).stats.nr_samples == 10);
        TEST_ASSERT_VAL!("Invalid nr hist entries", (*hists).nr_entries == 9);
        TEST_ASSERT_VAL!("Invalid total period", (*hists).stats.total_period == 1000);
        TEST_ASSERT_VAL!(
            "Unmatched nr samples",
            (*hists).stats.nr_samples == (*hists).stats.nr_non_filtered_samples
        );
        TEST_ASSERT_VAL!(
            "Unmatched nr hist entries",
            (*hists).nr_entries == (*hists).nr_non_filtered_entries
        );
        TEST_ASSERT_VAL!(
            "Unmatched total period",
            (*hists).stats.total_period == (*hists).stats.total_non_filtered_period
        );

        // now applying thread filter for 'bash'
        (*hists).thread_filter = fake_samples[9].thread;
        hists__filter_by_thread(hists);

        if verbose > 2 {
            pr_info(b"Histogram for thread filter\n\0".as_ptr() as *const c_char);
            print_hists_out(hists);
        }

        // normal stats should be invariant
        TEST_ASSERT_VAL!("Invalid nr samples", (*hists).stats.nr_samples == 10);
        TEST_ASSERT_VAL!("Invalid nr hist entries", (*hists).nr_entries == 9);
        TEST_ASSERT_VAL!("Invalid total period", (*hists).stats.total_period == 1000);

        // but filter stats are changed
        TEST_ASSERT_VAL!(
            "Unmatched nr samples for thread filter",
            (*hists).stats.nr_non_filtered_samples == 4
        );
        TEST_ASSERT_VAL!(
            "Unmatched nr hist entries for thread filter",
            (*hists).nr_non_filtered_entries == 4
        );
        TEST_ASSERT_VAL!(
            "Unmatched total period for thread filter",
            (*hists).stats.total_non_filtered_period == 400
        );

        // remove thread filter first
        (*hists).thread_filter = null_mut();
        hists__filter_by_thread(hists);

        // now applying dso filter for 'kernel'
        (*hists).dso_filter = map__dso(fake_samples[0].map);
        hists__filter_by_dso(hists);

        if verbose > 2 {
            pr_info(b"Histogram for dso filter\n\0".as_ptr() as *const c_char);
            print_hists_out(hists);
        }

        // normal stats should be invariant
        TEST_ASSERT_VAL!("Invalid nr samples", (*hists).stats.nr_samples == 10);
        TEST_ASSERT_VAL!("Invalid nr hist entries", (*hists).nr_entries == 9);
        TEST_ASSERT_VAL!("Invalid total period", (*hists).stats.total_period == 1000);

        // but filter stats are changed
        TEST_ASSERT_VAL!(
            "Unmatched nr samples for dso filter",
            (*hists).stats.nr_non_filtered_samples == 3
        );
        TEST_ASSERT_VAL!(
            "Unmatched nr hist entries for dso filter",
            (*hists).nr_non_filtered_entries == 3
        );
        TEST_ASSERT_VAL!(
            "Unmatched total period for dso filter",
            (*hists).stats.total_non_filtered_period == 300
        );

        // remove dso filter first
        (*hists).dso_filter = null_mut();
        hists__filter_by_dso(hists);

        /*
         * now applying symbol filter for 'main'.  Also note that
         * there's 3 samples that have 'main' symbol but the 4th
         * entry of fake_samples was collapsed already so it won't
         * be counted as a separate entry but the sample count and
         * total period will be remained.
         */
        (*hists).symbol_filter_str = b"main\0".as_ptr() as *const c_char;
        hists__filter_by_symbol(hists);

        if verbose > 2 {
            pr_info(b"Histogram for symbol filter\n\0".as_ptr() as *const c_char);
            print_hists_out(hists);
        }

        // normal stats should be invariant
        TEST_ASSERT_VAL!("Invalid nr samples", (*hists).stats.nr_samples == 10);
        TEST_ASSERT_VAL!("Invalid nr hist entries", (*hists).nr_entries == 9);
        TEST_ASSERT_VAL!("Invalid total period", (*hists).stats.total_period == 1000);

        // but filter stats are changed
        TEST_ASSERT_VAL!(
            "Unmatched nr samples for symbol filter",
            (*hists).stats.nr_non_filtered_samples == 3
        );
        TEST_ASSERT_VAL!(
            "Unmatched nr hist entries for symbol filter",
            (*hists).nr_non_filtered_entries == 2
        );
        TEST_ASSERT_VAL!(
            "Unmatched total period for symbol filter",
            (*hists).stats.total_non_filtered_period == 300
        );

        // remove symbol filter first
        (*hists).symbol_filter_str = null();
        hists__filter_by_symbol(hists);

        // now applying socket filters
        (*hists).socket_filter = 2;
        hists__filter_by_socket(hists);

        if verbose > 2 {
            pr_info(b"Histogram for socket filters\n\0".as_ptr() as *const c_char);
            print_hists_out(hists);
        }

        // normal stats should be invariant
        TEST_ASSERT_VAL!("Invalid nr samples", (*hists).stats.nr_samples == 10);
        TEST_ASSERT_VAL!("Invalid nr hist entries", (*hists).nr_entries == 9);
        TEST_ASSERT_VAL!("Invalid total period", (*hists).stats.total_period == 1000);

        // but filter stats are changed
        TEST_ASSERT_VAL!(
            "Unmatched nr samples for socket filter",
            (*hists).stats.nr_non_filtered_samples == 2
        );
        TEST_ASSERT_VAL!(
            "Unmatched nr hist entries for socket filter",
            (*hists).nr_non_filtered_entries == 2
        );
        TEST_ASSERT_VAL!(
            "Unmatched total period for socket filter",
            (*hists).stats.total_non_filtered_period == 200
        );

        // remove socket filter first
        (*hists).socket_filter = -1;
        hists__filter_by_socket(hists);

        // now applying all filters at once.
        (*hists).thread_filter = fake_samples[1].thread;
        (*hists).dso_filter = map__dso(fake_samples[1].map);
        hists__filter_by_thread(hists);
        hists__filter_by_dso(hists);

        if verbose > 2 {
            pr_info(b"Histogram for all filters\n\0".as_ptr() as *const c_char);
            print_hists_out(hists);
        }

        // normal stats should be invariant
        TEST_ASSERT_VAL!("Invalid nr samples", (*hists).stats.nr_samples == 10);
        TEST_ASSERT_VAL!("Invalid nr hist entries", (*hists).nr_entries == 9);
        TEST_ASSERT_VAL!("Invalid total period", (*hists).stats.total_period == 1000);

        // but filter stats are changed
        TEST_ASSERT_VAL!(
            "Unmatched nr samples for all filter",
            (*hists).stats.nr_non_filtered_samples == 2
        );
        TEST_ASSERT_VAL!(
            "Unmatched nr hist entries for all filter",
            (*hists).nr_non_filtered_entries == 1
        );
        TEST_ASSERT_VAL!(
            "Unmatched total period for all filter",
            (*hists).stats.total_non_filtered_period == 200
        );
    });

    err = TEST_OK;

    // tear down everything
    evlist__put(evlist);
    reset_output_field();
    machines__exit(&mut machines);
    put_fake_samples();

    err
}

DEFINE_SUITE!("Filter hist entries", hists_filter);
