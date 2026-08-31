// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/hists_output.c.
// Original C dependencies:
// util/debug.h, util/dso.h, util/event.h, util/map.h, util/symbol.h,
// util/sort.h, util/evsel.h, util/evlist.h, util/machine.h,
// util/thread.h, util/parse-events.h, tests/tests.h,
// tests/hists_common.h, linux/kernel.h.

use core::ffi::{c_char, c_int, c_void};
use core::mem::MaybeUninit;
use core::ptr;

#[repr(C)]
struct sample {
    cpu: u32,
    pid: u32,
    ip: u64,
    thread: *mut thread,
    map: *mut map,
    sym: *mut symbol,
}

/* For the numbers, see hists_common.c */
static mut fake_samples: [sample; 10] = [
    /* perf [kernel] schedule() */
    sample { cpu: 0, pid: FAKE_PID_PERF1, ip: FAKE_IP_KERNEL_SCHEDULE, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* perf [perf]   main() */
    sample { cpu: 1, pid: FAKE_PID_PERF1, ip: FAKE_IP_PERF_MAIN, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* perf [perf]   cmd_record() */
    sample { cpu: 1, pid: FAKE_PID_PERF1, ip: FAKE_IP_PERF_CMD_RECORD, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* perf [libc]   malloc() */
    sample { cpu: 1, pid: FAKE_PID_PERF1, ip: FAKE_IP_LIBC_MALLOC, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* perf [libc]   free() */
    sample { cpu: 2, pid: FAKE_PID_PERF1, ip: FAKE_IP_LIBC_FREE, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* perf [perf]   main() */
    sample { cpu: 2, pid: FAKE_PID_PERF2, ip: FAKE_IP_PERF_MAIN, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* perf [kernel] page_fault() */
    sample { cpu: 2, pid: FAKE_PID_PERF2, ip: FAKE_IP_KERNEL_PAGE_FAULT, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* bash [bash]   main() */
    sample { cpu: 3, pid: FAKE_PID_BASH, ip: FAKE_IP_BASH_MAIN, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* bash [bash]   xmalloc() */
    sample { cpu: 0, pid: FAKE_PID_BASH, ip: FAKE_IP_BASH_XMALLOC, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* bash [kernel] page_fault() */
    sample { cpu: 1, pid: FAKE_PID_BASH, ip: FAKE_IP_KERNEL_PAGE_FAULT, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
];

unsafe fn add_hist_entries(hists: *mut hists, machine: *mut machine) -> c_int {
    let mut al = MaybeUninit::<addr_location>::uninit();
    let evsel = hists_to_evsel(hists);
    let mut sample = perf_sample {
        evsel,
        period: 100,
        ..MaybeUninit::<perf_sample>::zeroed().assume_init()
    };
    let mut i: usize;

    addr_location__init(al.as_mut_ptr());
    let al = al.as_mut_ptr();
    i = 0;
    while i < fake_samples.len() {
        let mut iter = hist_entry_iter {
            sample: &mut sample,
            ops: &hist_iter_normal,
            hide_unresolved: false,
            ..MaybeUninit::<hist_entry_iter>::zeroed().assume_init()
        };

        sample.cpumode = PERF_RECORD_MISC_USER;
        sample.cpu = fake_samples[i].cpu;
        sample.pid = fake_samples[i].pid;
        sample.tid = fake_samples[i].pid;
        sample.ip = fake_samples[i].ip;

        if machine__resolve(machine, al, &mut sample) < 0 {
            pr_debug(c"Not enough memory for adding a hist entry\n".as_ptr());
            addr_location__exit(al);
            return TEST_FAIL;
        }

        if hist_entry_iter__add(&mut iter, al, sysctl_perf_event_max_stack, ptr::null_mut()) < 0 {
            pr_debug(c"Not enough memory for adding a hist entry\n".as_ptr());
            addr_location__exit(al);
            return TEST_FAIL;
        }

        fake_samples[i].thread = (*al).thread;
        map__put(fake_samples[i].map);
        fake_samples[i].map = map__get((*al).map);
        fake_samples[i].sym = (*al).sym;
        i += 1;
    }

    addr_location__exit(al);
    TEST_OK
}

unsafe fn del_hist_entries(hists: *mut hists) {
    let mut he: *mut hist_entry;
    let root_in: *mut rb_root_cached;
    let root_out: *mut rb_root_cached;
    let mut node: *mut rb_node;

    if hists__has(hists, need_collapse) {
        root_in = &mut (*hists).entries_collapsed;
    } else {
        root_in = (*hists).entries_in;
    }

    root_out = &mut (*hists).entries;

    while !RB_EMPTY_ROOT(&mut (*root_out).rb_root) {
        node = rb_first_cached(root_out);

        he = rb_entry_hist_entry_rb_node(node);
        rb_erase_cached(node, root_out);
        rb_erase_cached(&mut (*he).rb_node_in, root_in);
        hist_entry__delete(he);
    }
}

unsafe fn put_fake_samples() {
    let mut i: usize = 0;

    while i < fake_samples.len() {
        map__put(fake_samples[i].map);
        fake_samples[i].map = ptr::null_mut();
        i += 1;
    }
}

type test_fn_t = unsafe fn(*mut evsel, *mut machine) -> c_int;

unsafe fn COMM(he: *mut hist_entry) -> *const c_char {
    thread__comm_str((*he).thread)
}

unsafe fn DSO(he: *mut hist_entry) -> *const c_char {
    dso__short_name(map__dso((*he).ms.map))
}

unsafe fn SYM(he: *mut hist_entry) -> *const c_char {
    (*(*he).ms.sym).name
}

unsafe fn CPU(he: *mut hist_entry) -> u32 {
    (*he).cpu
}

unsafe fn PID(he: *mut hist_entry) -> i32 {
    thread__tid((*he).thread)
}

/* default sort keys (no field) */
unsafe fn test1(evsel: *mut evsel, machine: *mut machine) -> c_int {
    let mut err: c_int;
    let hists = evsel__hists(evsel);
    let mut he: *mut hist_entry;
    let root: *mut rb_root_cached;
    let mut node: *mut rb_node;

    field_order = ptr::null();
    sort_order = ptr::null(); /* equivalent to sort_order = "comm,dso,sym" */

    setup_sorting(ptr::null_mut(), (*machine).env);

    /*
     * expected output:
     *
     * Overhead  Command  Shared Object          Symbol
     * ========  =======  =============  ==============
     *   20.00%     perf  perf           [.] main
     *   10.00%     bash  [kernel]       [k] page_fault
     *   10.00%     bash  bash           [.] main
     *   10.00%     bash  bash           [.] xmalloc
     *   10.00%     perf  [kernel]       [k] page_fault
     *   10.00%     perf  [kernel]       [k] schedule
     *   10.00%     perf  libc           [.] free
     *   10.00%     perf  libc           [.] malloc
     *   10.00%     perf  perf           [.] cmd_record
     */
    err = add_hist_entries(hists, machine);
    if err < 0 {
        del_hist_entries(hists);
        reset_output_field();
        return err;
    }

    hists__collapse_resort(hists, ptr::null_mut());
    evsel__output_resort(evsel, ptr::null_mut());

    if verbose > 2 {
        pr_info(c"[fields = %s, sort = %s]\n".as_ptr(), field_order, sort_order);
        print_hists_out(hists);
    }

    root = &mut (*hists).entries;
    node = rb_first_cached(root);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"perf".as_ptr()) == 0 && strcmp(SYM(he), c"main".as_ptr()) == 0 && (*he).stat.period == 200);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(COMM(he), c"bash".as_ptr()) == 0 && strcmp(DSO(he), c"[kernel]".as_ptr()) == 0 && strcmp(SYM(he), c"page_fault".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(COMM(he), c"bash".as_ptr()) == 0 && strcmp(DSO(he), c"bash".as_ptr()) == 0 && strcmp(SYM(he), c"main".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(COMM(he), c"bash".as_ptr()) == 0 && strcmp(DSO(he), c"bash".as_ptr()) == 0 && strcmp(SYM(he), c"xmalloc".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"[kernel]".as_ptr()) == 0 && strcmp(SYM(he), c"page_fault".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"[kernel]".as_ptr()) == 0 && strcmp(SYM(he), c"schedule".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"libc".as_ptr()) == 0 && strcmp(SYM(he), c"free".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"libc".as_ptr()) == 0 && strcmp(SYM(he), c"malloc".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"perf".as_ptr()) == 0 && strcmp(SYM(he), c"cmd_record".as_ptr()) == 0 && (*he).stat.period == 100);

    del_hist_entries(hists);
    reset_output_field();
    err
}

/* mixed fields and sort keys */
unsafe fn test2(evsel: *mut evsel, machine: *mut machine) -> c_int {
    let mut err: c_int;
    let hists = evsel__hists(evsel);
    let mut he: *mut hist_entry;
    let root: *mut rb_root_cached;
    let mut node: *mut rb_node;

    field_order = c"overhead,cpu".as_ptr();
    sort_order = c"pid".as_ptr();

    setup_sorting(ptr::null_mut(), (*machine).env);

    /*
     * expected output:
     *
     * Overhead  CPU  Command:  Pid
     * ========  ===  =============
     *   30.00%    1  perf   :  100
     *   10.00%    0  perf   :  100
     *   10.00%    2  perf   :  100
     *   20.00%    2  perf   :  200
     *   10.00%    0  bash   :  300
     *   10.00%    1  bash   :  300
     *   10.00%    3  bash   :  300
     */
    err = add_hist_entries(hists, machine);
    if err < 0 {
        del_hist_entries(hists);
        reset_output_field();
        return err;
    }

    hists__collapse_resort(hists, ptr::null_mut());
    evsel__output_resort(evsel, ptr::null_mut());

    if verbose > 2 {
        pr_info(c"[fields = %s, sort = %s]\n".as_ptr(), field_order, sort_order);
        print_hists_out(hists);
    }

    root = &mut (*hists).entries;
    node = rb_first_cached(root);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), CPU(he) == 1 && PID(he) == 100 && (*he).stat.period == 300);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), CPU(he) == 0 && PID(he) == 100 && (*he).stat.period == 100);

    del_hist_entries(hists);
    reset_output_field();
    err
}

/* fields only (no sort key) */
unsafe fn test3(evsel: *mut evsel, machine: *mut machine) -> c_int {
    let mut err: c_int;
    let hists = evsel__hists(evsel);
    let mut he: *mut hist_entry;
    let root: *mut rb_root_cached;
    let mut node: *mut rb_node;

    field_order = c"comm,overhead,dso".as_ptr();
    sort_order = ptr::null();

    setup_sorting(ptr::null_mut(), (*machine).env);

    /*
     * expected output:
     *
     * Command  Overhead  Shared Object
     * =======  ========  =============
     *    bash    20.00%  bash
     *    bash    10.00%  [kernel]
     *    perf    30.00%  perf
     *    perf    20.00%  [kernel]
     *    perf    20.00%  libc
     */
    err = add_hist_entries(hists, machine);
    if err < 0 {
        del_hist_entries(hists);
        reset_output_field();
        return err;
    }

    hists__collapse_resort(hists, ptr::null_mut());
    evsel__output_resort(evsel, ptr::null_mut());

    if verbose > 2 {
        pr_info(c"[fields = %s, sort = %s]\n".as_ptr(), field_order, sort_order);
        print_hists_out(hists);
    }

    root = &mut (*hists).entries;
    node = rb_first_cached(root);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(COMM(he), c"bash".as_ptr()) == 0 && strcmp(DSO(he), c"bash".as_ptr()) == 0 && (*he).stat.period == 200);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(COMM(he), c"bash".as_ptr()) == 0 && strcmp(DSO(he), c"[kernel]".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"perf".as_ptr()) == 0 && (*he).stat.period == 300);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"[kernel]".as_ptr()) == 0 && (*he).stat.period == 200);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"libc".as_ptr()) == 0 && (*he).stat.period == 200);

    del_hist_entries(hists);
    reset_output_field();
    err
}

/* handle duplicate 'dso' field */
unsafe fn test4(evsel: *mut evsel, machine: *mut machine) -> c_int {
    let mut err: c_int;
    let hists = evsel__hists(evsel);
    let mut he: *mut hist_entry;
    let root: *mut rb_root_cached;
    let mut node: *mut rb_node;

    field_order = c"dso,sym,comm,overhead,dso".as_ptr();
    sort_order = c"sym".as_ptr();

    setup_sorting(ptr::null_mut(), (*machine).env);

    /*
     * expected output:
     *
     * Shared Object          Symbol  Command  Overhead
     * =============  ==============  =======  ========
     *          perf  [.] cmd_record     perf    10.00%
     *          libc  [.] free           perf    10.00%
     *          bash  [.] main           bash    10.00%
     *          perf  [.] main           perf    20.00%
     *          libc  [.] malloc         perf    10.00%
     *      [kernel]  [k] page_fault     bash    10.00%
     *      [kernel]  [k] page_fault     perf    10.00%
     *      [kernel]  [k] schedule       perf    10.00%
     *          bash  [.] xmalloc        bash    10.00%
     */
    err = add_hist_entries(hists, machine);
    if err < 0 {
        del_hist_entries(hists);
        reset_output_field();
        return err;
    }

    hists__collapse_resort(hists, ptr::null_mut());
    evsel__output_resort(evsel, ptr::null_mut());

    if verbose > 2 {
        pr_info(c"[fields = %s, sort = %s]\n".as_ptr(), field_order, sort_order);
        print_hists_out(hists);
    }

    root = &mut (*hists).entries;
    node = rb_first_cached(root);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(DSO(he), c"perf".as_ptr()) == 0 && strcmp(SYM(he), c"cmd_record".as_ptr()) == 0 && strcmp(COMM(he), c"perf".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(DSO(he), c"libc".as_ptr()) == 0 && strcmp(SYM(he), c"free".as_ptr()) == 0 && strcmp(COMM(he), c"perf".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(DSO(he), c"bash".as_ptr()) == 0 && strcmp(SYM(he), c"main".as_ptr()) == 0 && strcmp(COMM(he), c"bash".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(DSO(he), c"perf".as_ptr()) == 0 && strcmp(SYM(he), c"main".as_ptr()) == 0 && strcmp(COMM(he), c"perf".as_ptr()) == 0 && (*he).stat.period == 200);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(DSO(he), c"libc".as_ptr()) == 0 && strcmp(SYM(he), c"malloc".as_ptr()) == 0 && strcmp(COMM(he), c"perf".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(DSO(he), c"[kernel]".as_ptr()) == 0 && strcmp(SYM(he), c"page_fault".as_ptr()) == 0 && strcmp(COMM(he), c"bash".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(DSO(he), c"[kernel]".as_ptr()) == 0 && strcmp(SYM(he), c"page_fault".as_ptr()) == 0 && strcmp(COMM(he), c"perf".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(DSO(he), c"[kernel]".as_ptr()) == 0 && strcmp(SYM(he), c"schedule".as_ptr()) == 0 && strcmp(COMM(he), c"perf".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), strcmp(DSO(he), c"bash".as_ptr()) == 0 && strcmp(SYM(he), c"xmalloc".as_ptr()) == 0 && strcmp(COMM(he), c"bash".as_ptr()) == 0 && (*he).stat.period == 100);

    del_hist_entries(hists);
    reset_output_field();
    err
}

/* full sort keys w/o overhead field */
unsafe fn test5(evsel: *mut evsel, machine: *mut machine) -> c_int {
    let mut err: c_int;
    let hists = evsel__hists(evsel);
    let mut he: *mut hist_entry;
    let root: *mut rb_root_cached;
    let mut node: *mut rb_node;

    field_order = c"cpu,pid,comm,dso,sym".as_ptr();
    sort_order = c"dso,pid".as_ptr();

    setup_sorting(ptr::null_mut(), (*machine).env);

    /*
     * expected output:
     *
     * CPU  Command:  Pid  Command  Shared Object          Symbol
     * ===  =============  =======  =============  ==============
     *   0     perf:  100     perf       [kernel]  [k] schedule
     *   2     perf:  200     perf       [kernel]  [k] page_fault
     *   1     bash:  300     bash       [kernel]  [k] page_fault
     *   0     bash:  300     bash           bash  [.] xmalloc
     *   3     bash:  300     bash           bash  [.] main
     *   1     perf:  100     perf           libc  [.] malloc
     *   2     perf:  100     perf           libc  [.] free
     *   1     perf:  100     perf           perf  [.] cmd_record
     *   1     perf:  100     perf           perf  [.] main
     *   2     perf:  200     perf           perf  [.] main
     */
    err = add_hist_entries(hists, machine);
    if err < 0 {
        del_hist_entries(hists);
        reset_output_field();
        return err;
    }

    hists__collapse_resort(hists, ptr::null_mut());
    evsel__output_resort(evsel, ptr::null_mut());

    if verbose > 2 {
        pr_info(c"[fields = %s, sort = %s]\n".as_ptr(), field_order, sort_order);
        print_hists_out(hists);
    }

    root = &mut (*hists).entries;
    node = rb_first_cached(root);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), CPU(he) == 0 && PID(he) == 100 && strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"[kernel]".as_ptr()) == 0 && strcmp(SYM(he), c"schedule".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), CPU(he) == 2 && PID(he) == 200 && strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"[kernel]".as_ptr()) == 0 && strcmp(SYM(he), c"page_fault".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), CPU(he) == 1 && PID(he) == 300 && strcmp(COMM(he), c"bash".as_ptr()) == 0 && strcmp(DSO(he), c"[kernel]".as_ptr()) == 0 && strcmp(SYM(he), c"page_fault".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), CPU(he) == 0 && PID(he) == 300 && strcmp(COMM(he), c"bash".as_ptr()) == 0 && strcmp(DSO(he), c"bash".as_ptr()) == 0 && strcmp(SYM(he), c"xmalloc".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), CPU(he) == 3 && PID(he) == 300 && strcmp(COMM(he), c"bash".as_ptr()) == 0 && strcmp(DSO(he), c"bash".as_ptr()) == 0 && strcmp(SYM(he), c"main".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), CPU(he) == 1 && PID(he) == 100 && strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"libc".as_ptr()) == 0 && strcmp(SYM(he), c"malloc".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), CPU(he) == 2 && PID(he) == 100 && strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"libc".as_ptr()) == 0 && strcmp(SYM(he), c"free".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), CPU(he) == 1 && PID(he) == 100 && strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"perf".as_ptr()) == 0 && strcmp(SYM(he), c"cmd_record".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), CPU(he) == 1 && PID(he) == 100 && strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"perf".as_ptr()) == 0 && strcmp(SYM(he), c"main".as_ptr()) == 0 && (*he).stat.period == 100);

    node = rb_next(node);
    he = rb_entry_hist_entry_rb_node(node);
    TEST_ASSERT_VAL(c"Invalid hist entry".as_ptr(), CPU(he) == 2 && PID(he) == 200 && strcmp(COMM(he), c"perf".as_ptr()) == 0 && strcmp(DSO(he), c"perf".as_ptr()) == 0 && strcmp(SYM(he), c"main".as_ptr()) == 0 && (*he).stat.period == 100);

    del_hist_entries(hists);
    reset_output_field();
    err
}

unsafe fn test__hists_output(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut err: c_int = TEST_FAIL;
    let mut machines: machines = MaybeUninit::<machines>::zeroed().assume_init();
    let machine: *mut machine;
    let evsel: *mut evsel;
    let evlist: *mut evlist = evlist__new();
    let mut i: usize;
    let testcases: [test_fn_t; 5] = [
        test1,
        test2,
        test3,
        test4,
        test5,
    ];

    TEST_ASSERT_VAL(c"No memory".as_ptr(), !evlist.is_null());

    err = parse_event(evlist, c"cpu-clock".as_ptr());
    if err != 0 {
        evlist__put(evlist);
        machines__exit(&mut machines);
        put_fake_samples();
        return err;
    }
    err = TEST_FAIL;

    if machines__init(&mut machines) != 0 {
        evlist__put(evlist);
        machines__exit(&mut machines);
        put_fake_samples();
        return err;
    }

    /* setup threads/dso/map/symbols also */
    machine = setup_fake_machine(&mut machines);
    if machine.is_null() {
        evlist__put(evlist);
        machines__exit(&mut machines);
        put_fake_samples();
        return err;
    }

    if verbose > 1 {
        machine__fprintf(machine, stderr);
    }

    evsel = evlist__first(evlist);

    i = 0;
    while i < testcases.len() {
        err = testcases[i](evsel, machine);
        if err < 0 {
            break;
        }
        i += 1;
    }

    /* tear down everything */
    evlist__put(evlist);
    machines__exit(&mut machines);
    put_fake_samples();

    err
}

DEFINE_SUITE(c"Sort output of hist entries".as_ptr(), hists_output);
