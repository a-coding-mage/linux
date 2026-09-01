// SPDX-License-Identifier: GPL-2.0
// C dependencies translated from:
// "util/debug.h", "util/dso.h", "util/event.h", "util/map.h",
// "util/symbol.h", "util/sort.h", "util/evsel.h", "util/evlist.h",
// "util/machine.h", "util/parse-events.h", "util/thread.h",
// "tests/tests.h", "tests/hists_common.h", and <linux/kernel.h>.

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
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
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    pub env: *mut machine_env,
}

#[repr(C)]
pub struct machines {
    _data: [u8; 0],
}

#[repr(C)]
pub struct ip_callchain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    pub evsel: *mut evsel,
    pub period: u64,
    pub cpumode: u32,
    pub pid: u32,
    pub tid: u32,
    pub ip: u64,
    pub callchain: *mut ip_callchain,
}

#[repr(C)]
pub struct addr_location {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct hist_entry_iter_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hist_entry_iter {
    pub sample: *mut perf_sample,
    pub hide_unresolved: bool,
    pub ops: *const hist_entry_iter_ops,
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
pub struct rb_root_cached {
    pub rb_root: rb_root,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct hist_entry_stat {
    pub period: u64,
}

#[repr(C)]
pub struct callchain_root {
    pub node: rb_root,
    pub max_depth: c_int,
}

#[repr(C)]
pub struct hist_entry {
    pub rb_node: rb_node,
    pub rb_node_in: rb_node,
    pub thread: *mut thread,
    pub ms: map_symbol,
    pub cpu: c_int,
    pub callchain: *mut callchain_root,
    pub stat: hist_entry_stat,
    pub stat_acc: *mut hist_entry_stat,
}

#[repr(C)]
pub struct callchain_node {
    pub rb_node: rb_node,
    pub val: list_head,
}

#[repr(C)]
pub struct callchain_list {
    pub list: list_head,
    pub ms: map_symbol,
}

#[repr(C)]
pub struct hists {
    pub entries_collapsed: rb_root_cached,
    pub entries_in: *mut rb_root_cached,
    pub entries: rb_root_cached,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub use_callchain: bool,
    pub cumulate_callchain: bool,
}

#[repr(C)]
pub struct callchain_param_t {
    _private: [u8; 0],
}

#[repr(C)]
struct sample {
    pid: u32,
    ip: u64,
    thread: *mut thread,
    map: *mut map,
    sym: *mut symbol,
}

/* For the numbers, see hists_common.c */
static mut fake_samples: [sample; 10] = [
    /* perf [kernel] schedule() */
    sample { pid: FAKE_PID_PERF1, ip: FAKE_IP_KERNEL_SCHEDULE, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* perf [perf]   main() */
    sample { pid: FAKE_PID_PERF1, ip: FAKE_IP_PERF_MAIN, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* perf [perf]   cmd_record() */
    sample { pid: FAKE_PID_PERF1, ip: FAKE_IP_PERF_CMD_RECORD, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* perf [libc]   malloc() */
    sample { pid: FAKE_PID_PERF1, ip: FAKE_IP_LIBC_MALLOC, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* perf [libc]   free() */
    sample { pid: FAKE_PID_PERF1, ip: FAKE_IP_LIBC_FREE, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* perf [perf]   main() */
    sample { pid: FAKE_PID_PERF2, ip: FAKE_IP_PERF_MAIN, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* perf [kernel] page_fault() */
    sample { pid: FAKE_PID_PERF2, ip: FAKE_IP_KERNEL_PAGE_FAULT, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* bash [bash]   main() */
    sample { pid: FAKE_PID_BASH, ip: FAKE_IP_BASH_MAIN, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* bash [bash]   xmalloc() */
    sample { pid: FAKE_PID_BASH, ip: FAKE_IP_BASH_XMALLOC, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
    /* bash [kernel] page_fault() */
    sample { pid: FAKE_PID_BASH, ip: FAKE_IP_KERNEL_PAGE_FAULT, thread: ptr::null_mut(), map: ptr::null_mut(), sym: ptr::null_mut() },
];

/*
 * Will be cast to struct ip_callchain which has all 64 bit entries
 * of nr and ips[].
 */
static mut fake_callchains: [[u64; 10]; 10] = [
    /*   schedule => run_command => main */
    [3, FAKE_IP_KERNEL_SCHEDULE, FAKE_IP_PERF_RUN_COMMAND, FAKE_IP_PERF_MAIN, 0, 0, 0, 0, 0, 0],
    /*   main  */
    [1, FAKE_IP_PERF_MAIN, 0, 0, 0, 0, 0, 0, 0, 0],
    /*   cmd_record => run_command => main */
    [3, FAKE_IP_PERF_CMD_RECORD, FAKE_IP_PERF_RUN_COMMAND, FAKE_IP_PERF_MAIN, 0, 0, 0, 0, 0, 0],
    /*   malloc => cmd_record => run_command => main */
    [4, FAKE_IP_LIBC_MALLOC, FAKE_IP_PERF_CMD_RECORD, FAKE_IP_PERF_RUN_COMMAND, FAKE_IP_PERF_MAIN, 0, 0, 0, 0, 0],
    /*   free => cmd_record => run_command => main */
    [4, FAKE_IP_LIBC_FREE, FAKE_IP_PERF_CMD_RECORD, FAKE_IP_PERF_RUN_COMMAND, FAKE_IP_PERF_MAIN, 0, 0, 0, 0, 0],
    /*   main */
    [1, FAKE_IP_PERF_MAIN, 0, 0, 0, 0, 0, 0, 0, 0],
    /*   page_fault => sys_perf_event_open => run_command => main */
    [4, FAKE_IP_KERNEL_PAGE_FAULT, FAKE_IP_KERNEL_SYS_PERF_EVENT_OPEN, FAKE_IP_PERF_RUN_COMMAND, FAKE_IP_PERF_MAIN, 0, 0, 0, 0, 0],
    /*   main */
    [1, FAKE_IP_BASH_MAIN, 0, 0, 0, 0, 0, 0, 0, 0],
    /*   xmalloc => malloc => xmalloc => malloc => xmalloc => main */
    [6, FAKE_IP_BASH_XMALLOC, FAKE_IP_LIBC_MALLOC, FAKE_IP_BASH_XMALLOC, FAKE_IP_LIBC_MALLOC, FAKE_IP_BASH_XMALLOC, FAKE_IP_BASH_MAIN, 0, 0, 0],
    /*   page_fault => malloc => main */
    [3, FAKE_IP_KERNEL_PAGE_FAULT, FAKE_IP_LIBC_MALLOC, FAKE_IP_BASH_MAIN, 0, 0, 0, 0, 0, 0],
];

unsafe fn add_hist_entries(hists: *mut hists, machine: *mut machine) -> c_int {
    let mut al: addr_location = core::mem::zeroed();
    let evsel = hists_to_evsel(hists);
    let mut sample = perf_sample {
        evsel,
        period: 1000,
        cpumode: 0,
        pid: 0,
        tid: 0,
        ip: 0,
        callchain: ptr::null_mut(),
    };
    let mut i: usize;

    addr_location__init(&mut al);
    i = 0;
    while i < fake_samples.len() {
        let mut iter = hist_entry_iter {
            sample: &mut sample,
            hide_unresolved: false,
            ops: ptr::null(),
        };

        if symbol_conf.cumulate_callchain {
            iter.ops = &hist_iter_cumulative;
        } else {
            iter.ops = &hist_iter_normal;
        }

        sample.cpumode = PERF_RECORD_MISC_USER;
        sample.pid = fake_samples[i].pid;
        sample.tid = fake_samples[i].pid;
        sample.ip = fake_samples[i].ip;
        sample.callchain = fake_callchains[i].as_mut_ptr() as *mut ip_callchain;

        if machine__resolve(machine, &mut al, &mut sample) < 0 {
            goto_out_add(&mut al);
            return TEST_FAIL;
        }

        if hist_entry_iter__add(&mut iter, &mut al, sysctl_perf_event_max_stack, ptr::null_mut()) < 0 {
            goto_out_add(&mut al);
            return TEST_FAIL;
        }

        thread__put(fake_samples[i].thread);
        fake_samples[i].thread = thread__get(al.thread);
        map__put(fake_samples[i].map);
        fake_samples[i].map = map__get(al.map);
        fake_samples[i].sym = al.sym;

        i += 1;
    }

    addr_location__exit(&mut al);
    TEST_OK
}

unsafe fn goto_out_add(al: *mut addr_location) {
    pr_debug(c"Not enough memory for adding a hist entry\n".as_ptr());
    addr_location__exit(al);
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
        map__zput(&mut fake_samples[i].map);
        thread__zput(&mut fake_samples[i].thread);
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

unsafe fn CPU(he: *mut hist_entry) -> c_int {
    (*he).cpu
}

unsafe fn DEPTH(he: *mut hist_entry) -> c_int {
    (*(*he).callchain).max_depth
}

unsafe fn CDSO(cl: *mut callchain_list) -> *const c_char {
    dso__short_name(map__dso((*cl).ms.map))
}

unsafe fn CSYM(cl: *mut callchain_list) -> *const c_char {
    (*(*cl).ms.sym).name
}

#[repr(C)]
struct result {
    children: u64,
    self_: u64,
    comm: *const c_char,
    dso: *const c_char,
    sym: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct callchain_result_node {
    dso: *const c_char,
    sym: *const c_char,
}

#[repr(C)]
struct callchain_result {
    nr: u64,
    node: [callchain_result_node; 10],
}

const EMPTY_CALLCHAIN_NODE: callchain_result_node = callchain_result_node {
    dso: ptr::null(),
    sym: ptr::null(),
};

unsafe fn do_test(
    hists: *mut hists,
    expected: *mut result,
    nr_expected: usize,
    expected_callchain: *mut callchain_result,
    nr_callchain: usize,
) -> c_int {
    let mut buf = [0 as c_char; 32];
    let mut i: usize;
    let mut c: usize;
    let mut he: *mut hist_entry;
    let mut root: *mut rb_root;
    let mut node: *mut rb_node;
    let mut cnode: *mut callchain_node;
    let mut clist: *mut callchain_list;

    /*
     * adding and deleting hist entries must be done outside of this
     * function since TEST_ASSERT_VAL() returns in case of failure.
     */
    hists__collapse_resort(hists, ptr::null_mut());
    evsel__output_resort(hists_to_evsel(hists), ptr::null_mut());

    if verbose > 2 {
        pr_info(
            c"use callchain: %d, cumulate callchain: %d\n".as_ptr(),
            symbol_conf.use_callchain as c_int,
            symbol_conf.cumulate_callchain as c_int,
        );
        print_hists_out(hists);
    }

    root = &mut (*hists).entries.rb_root;
    node = rb_first(root);
    i = 0;
    while !node.is_null() {
        he = rb_entry_hist_entry_rb_node(node);
        scnprintf(buf.as_mut_ptr(), size_of::<[c_char; 32]>(), c"Invalid hist entry #%zd".as_ptr(), i);

        TEST_ASSERT_VAL(c"Incorrect number of hist entry".as_ptr(), i < nr_expected);
        TEST_ASSERT_VAL(
            buf.as_ptr(),
            (*he).stat.period == (*expected.add(i)).self_
                && strcmp(COMM(he), (*expected.add(i)).comm) == 0
                && strcmp(DSO(he), (*expected.add(i)).dso) == 0
                && strcmp(SYM(he), (*expected.add(i)).sym) == 0,
        );

        if symbol_conf.cumulate_callchain {
            TEST_ASSERT_VAL(buf.as_ptr(), (*(*he).stat_acc).period == (*expected.add(i)).children);
        }

        if !symbol_conf.use_callchain {
            node = rb_next(node);
            i += 1;
            continue;
        }

        /* check callchain entries */
        root = &mut (*(*he).callchain).node;

        TEST_ASSERT_VAL(c"callchains expected".as_ptr(), !RB_EMPTY_ROOT(root));
        cnode = rb_entry_callchain_node_rb_node(rb_first(root));

        c = 0;
        clist = list_first_entry_callchain_list(&mut (*cnode).val);
        while &mut (*clist).list as *mut list_head != &mut (*cnode).val as *mut list_head {
            scnprintf(
                buf.as_mut_ptr(),
                size_of::<[c_char; 32]>(),
                c"Invalid callchain entry #%zd/%zd".as_ptr(),
                i,
                c,
            );

            TEST_ASSERT_VAL(
                c"Incorrect number of callchain entry".as_ptr(),
                (c as u64) < (*expected_callchain.add(i)).nr,
            );
            TEST_ASSERT_VAL(
                buf.as_ptr(),
                strcmp(CDSO(clist), (*expected_callchain.add(i)).node[c].dso) == 0
                    && strcmp(CSYM(clist), (*expected_callchain.add(i)).node[c].sym) == 0,
            );
            c += 1;
            clist = list_next_entry_callchain_list(clist);
        }
        /* TODO: handle multiple child nodes properly */
        TEST_ASSERT_VAL(
            c"Incorrect number of callchain entry".as_ptr(),
            (c as u64) <= (*expected_callchain.add(i)).nr,
        );

        node = rb_next(node);
        i += 1;
    }
    TEST_ASSERT_VAL(c"Incorrect number of hist entry".as_ptr(), i == nr_expected);
    TEST_ASSERT_VAL(
        c"Incorrect number of callchain entry".as_ptr(),
        !symbol_conf.use_callchain || nr_expected == nr_callchain,
    );
    0
}

/* NO callchain + NO children */
unsafe fn test1(evsel: *mut evsel, machine: *mut machine) -> c_int {
    let mut err: c_int;
    let hists = evsel__hists(evsel);
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
    let mut expected = [
        result { children: 0, self_: 2000, comm: c"perf".as_ptr(), dso: c"perf".as_ptr(), sym: c"main".as_ptr() },
        result { children: 0, self_: 1000, comm: c"bash".as_ptr(), dso: c"[kernel]".as_ptr(), sym: c"page_fault".as_ptr() },
        result { children: 0, self_: 1000, comm: c"bash".as_ptr(), dso: c"bash".as_ptr(), sym: c"main".as_ptr() },
        result { children: 0, self_: 1000, comm: c"bash".as_ptr(), dso: c"bash".as_ptr(), sym: c"xmalloc".as_ptr() },
        result { children: 0, self_: 1000, comm: c"perf".as_ptr(), dso: c"[kernel]".as_ptr(), sym: c"page_fault".as_ptr() },
        result { children: 0, self_: 1000, comm: c"perf".as_ptr(), dso: c"[kernel]".as_ptr(), sym: c"schedule".as_ptr() },
        result { children: 0, self_: 1000, comm: c"perf".as_ptr(), dso: c"libc".as_ptr(), sym: c"free".as_ptr() },
        result { children: 0, self_: 1000, comm: c"perf".as_ptr(), dso: c"libc".as_ptr(), sym: c"malloc".as_ptr() },
        result { children: 0, self_: 1000, comm: c"perf".as_ptr(), dso: c"perf".as_ptr(), sym: c"cmd_record".as_ptr() },
    ];

    symbol_conf.use_callchain = false;
    symbol_conf.cumulate_callchain = false;
    evsel__reset_sample_bit(evsel, CALLCHAIN);

    setup_sorting(ptr::null_mut(), (*machine).env);
    callchain_register_param(&mut callchain_param);

    err = add_hist_entries(hists, machine);
    if err < 0 {
        del_hist_entries(hists);
        reset_output_field();
        return err;
    }

    err = do_test(hists, expected.as_mut_ptr(), expected.len(), ptr::null_mut(), 0);

    del_hist_entries(hists);
    reset_output_field();
    err
}

/* callchain + NO children */
unsafe fn test2(evsel: *mut evsel, machine: *mut machine) -> c_int {
    let mut err: c_int;
    let hists = evsel__hists(evsel);
    let mut expected = [
        result { children: 0, self_: 2000, comm: c"perf".as_ptr(), dso: c"perf".as_ptr(), sym: c"main".as_ptr() },
        result { children: 0, self_: 1000, comm: c"bash".as_ptr(), dso: c"[kernel]".as_ptr(), sym: c"page_fault".as_ptr() },
        result { children: 0, self_: 1000, comm: c"bash".as_ptr(), dso: c"bash".as_ptr(), sym: c"main".as_ptr() },
        result { children: 0, self_: 1000, comm: c"bash".as_ptr(), dso: c"bash".as_ptr(), sym: c"xmalloc".as_ptr() },
        result { children: 0, self_: 1000, comm: c"perf".as_ptr(), dso: c"[kernel]".as_ptr(), sym: c"page_fault".as_ptr() },
        result { children: 0, self_: 1000, comm: c"perf".as_ptr(), dso: c"[kernel]".as_ptr(), sym: c"schedule".as_ptr() },
        result { children: 0, self_: 1000, comm: c"perf".as_ptr(), dso: c"libc".as_ptr(), sym: c"free".as_ptr() },
        result { children: 0, self_: 1000, comm: c"perf".as_ptr(), dso: c"libc".as_ptr(), sym: c"malloc".as_ptr() },
        result { children: 0, self_: 1000, comm: c"perf".as_ptr(), dso: c"perf".as_ptr(), sym: c"cmd_record".as_ptr() },
    ];
    let mut expected_callchain = [
        cc(1, &[(c"perf".as_ptr(), c"main".as_ptr())]),
        cc(3, &[(c"[kernel]".as_ptr(), c"page_fault".as_ptr()), (c"libc".as_ptr(), c"malloc".as_ptr()), (c"bash".as_ptr(), c"main".as_ptr())]),
        cc(1, &[(c"bash".as_ptr(), c"main".as_ptr())]),
        cc(6, &[(c"bash".as_ptr(), c"xmalloc".as_ptr()), (c"libc".as_ptr(), c"malloc".as_ptr()), (c"bash".as_ptr(), c"xmalloc".as_ptr()), (c"libc".as_ptr(), c"malloc".as_ptr()), (c"bash".as_ptr(), c"xmalloc".as_ptr()), (c"bash".as_ptr(), c"main".as_ptr())]),
        cc(4, &[(c"[kernel]".as_ptr(), c"page_fault".as_ptr()), (c"[kernel]".as_ptr(), c"sys_perf_event_open".as_ptr()), (c"perf".as_ptr(), c"run_command".as_ptr()), (c"perf".as_ptr(), c"main".as_ptr())]),
        cc(3, &[(c"[kernel]".as_ptr(), c"schedule".as_ptr()), (c"perf".as_ptr(), c"run_command".as_ptr()), (c"perf".as_ptr(), c"main".as_ptr())]),
        cc(4, &[(c"libc".as_ptr(), c"free".as_ptr()), (c"perf".as_ptr(), c"cmd_record".as_ptr()), (c"perf".as_ptr(), c"run_command".as_ptr()), (c"perf".as_ptr(), c"main".as_ptr())]),
        cc(4, &[(c"libc".as_ptr(), c"malloc".as_ptr()), (c"perf".as_ptr(), c"cmd_record".as_ptr()), (c"perf".as_ptr(), c"run_command".as_ptr()), (c"perf".as_ptr(), c"main".as_ptr())]),
        cc(3, &[(c"perf".as_ptr(), c"cmd_record".as_ptr()), (c"perf".as_ptr(), c"run_command".as_ptr()), (c"perf".as_ptr(), c"main".as_ptr())]),
    ];

    symbol_conf.use_callchain = true;
    symbol_conf.cumulate_callchain = false;
    evsel__set_sample_bit(evsel, CALLCHAIN);

    setup_sorting(ptr::null_mut(), (*machine).env);
    callchain_register_param(&mut callchain_param);

    err = add_hist_entries(hists, machine);
    if err < 0 {
        del_hist_entries(hists);
        reset_output_field();
        return err;
    }

    err = do_test(hists, expected.as_mut_ptr(), expected.len(), expected_callchain.as_mut_ptr(), expected_callchain.len());

    del_hist_entries(hists);
    reset_output_field();
    err
}

/* NO callchain + children */
unsafe fn test3(evsel: *mut evsel, machine: *mut machine) -> c_int {
    let mut err: c_int;
    let hists = evsel__hists(evsel);
    let mut expected = [
        result { children: 7000, self_: 2000, comm: c"perf".as_ptr(), dso: c"perf".as_ptr(), sym: c"main".as_ptr() },
        result { children: 5000, self_: 0, comm: c"perf".as_ptr(), dso: c"perf".as_ptr(), sym: c"run_command".as_ptr() },
        result { children: 3000, self_: 1000, comm: c"bash".as_ptr(), dso: c"bash".as_ptr(), sym: c"main".as_ptr() },
        result { children: 3000, self_: 1000, comm: c"perf".as_ptr(), dso: c"perf".as_ptr(), sym: c"cmd_record".as_ptr() },
        result { children: 2000, self_: 0, comm: c"bash".as_ptr(), dso: c"libc".as_ptr(), sym: c"malloc".as_ptr() },
        result { children: 1000, self_: 1000, comm: c"bash".as_ptr(), dso: c"[kernel]".as_ptr(), sym: c"page_fault".as_ptr() },
        result { children: 1000, self_: 1000, comm: c"bash".as_ptr(), dso: c"bash".as_ptr(), sym: c"xmalloc".as_ptr() },
        result { children: 1000, self_: 1000, comm: c"perf".as_ptr(), dso: c"[kernel]".as_ptr(), sym: c"page_fault".as_ptr() },
        result { children: 1000, self_: 1000, comm: c"perf".as_ptr(), dso: c"[kernel]".as_ptr(), sym: c"schedule".as_ptr() },
        result { children: 1000, self_: 1000, comm: c"perf".as_ptr(), dso: c"libc".as_ptr(), sym: c"free".as_ptr() },
        result { children: 1000, self_: 1000, comm: c"perf".as_ptr(), dso: c"libc".as_ptr(), sym: c"malloc".as_ptr() },
        result { children: 1000, self_: 0, comm: c"perf".as_ptr(), dso: c"[kernel]".as_ptr(), sym: c"sys_perf_event_open".as_ptr() },
    ];

    symbol_conf.use_callchain = false;
    symbol_conf.cumulate_callchain = true;
    evsel__reset_sample_bit(evsel, CALLCHAIN);

    setup_sorting(ptr::null_mut(), (*machine).env);
    callchain_register_param(&mut callchain_param);

    err = add_hist_entries(hists, machine);
    if err < 0 {
        del_hist_entries(hists);
        reset_output_field();
        return err;
    }

    err = do_test(hists, expected.as_mut_ptr(), expected.len(), ptr::null_mut(), 0);

    del_hist_entries(hists);
    reset_output_field();
    err
}

/* callchain + children */
unsafe fn test4(evsel: *mut evsel, machine: *mut machine) -> c_int {
    let mut err: c_int;
    let hists = evsel__hists(evsel);
    let mut expected = [
        result { children: 7000, self_: 2000, comm: c"perf".as_ptr(), dso: c"perf".as_ptr(), sym: c"main".as_ptr() },
        result { children: 5000, self_: 0, comm: c"perf".as_ptr(), dso: c"perf".as_ptr(), sym: c"run_command".as_ptr() },
        result { children: 3000, self_: 1000, comm: c"bash".as_ptr(), dso: c"bash".as_ptr(), sym: c"main".as_ptr() },
        result { children: 3000, self_: 1000, comm: c"perf".as_ptr(), dso: c"perf".as_ptr(), sym: c"cmd_record".as_ptr() },
        result { children: 2000, self_: 0, comm: c"bash".as_ptr(), dso: c"libc".as_ptr(), sym: c"malloc".as_ptr() },
        result { children: 1000, self_: 1000, comm: c"bash".as_ptr(), dso: c"[kernel]".as_ptr(), sym: c"page_fault".as_ptr() },
        result { children: 1000, self_: 1000, comm: c"bash".as_ptr(), dso: c"bash".as_ptr(), sym: c"xmalloc".as_ptr() },
        result { children: 1000, self_: 0, comm: c"perf".as_ptr(), dso: c"[kernel]".as_ptr(), sym: c"sys_perf_event_open".as_ptr() },
        result { children: 1000, self_: 1000, comm: c"perf".as_ptr(), dso: c"[kernel]".as_ptr(), sym: c"page_fault".as_ptr() },
        result { children: 1000, self_: 1000, comm: c"perf".as_ptr(), dso: c"[kernel]".as_ptr(), sym: c"schedule".as_ptr() },
        result { children: 1000, self_: 1000, comm: c"perf".as_ptr(), dso: c"libc".as_ptr(), sym: c"free".as_ptr() },
        result { children: 1000, self_: 1000, comm: c"perf".as_ptr(), dso: c"libc".as_ptr(), sym: c"malloc".as_ptr() },
    ];
    let mut expected_callchain = [
        cc(1, &[(c"perf".as_ptr(), c"main".as_ptr())]),
        cc(2, &[(c"perf".as_ptr(), c"run_command".as_ptr()), (c"perf".as_ptr(), c"main".as_ptr())]),
        cc(1, &[(c"bash".as_ptr(), c"main".as_ptr())]),
        cc(3, &[(c"perf".as_ptr(), c"cmd_record".as_ptr()), (c"perf".as_ptr(), c"run_command".as_ptr()), (c"perf".as_ptr(), c"main".as_ptr())]),
        cc(4, &[(c"libc".as_ptr(), c"malloc".as_ptr()), (c"bash".as_ptr(), c"xmalloc".as_ptr()), (c"bash".as_ptr(), c"main".as_ptr()), (c"bash".as_ptr(), c"main".as_ptr())]),
        cc(3, &[(c"[kernel]".as_ptr(), c"page_fault".as_ptr()), (c"libc".as_ptr(), c"malloc".as_ptr()), (c"bash".as_ptr(), c"main".as_ptr())]),
        cc(6, &[(c"bash".as_ptr(), c"xmalloc".as_ptr()), (c"libc".as_ptr(), c"malloc".as_ptr()), (c"bash".as_ptr(), c"xmalloc".as_ptr()), (c"libc".as_ptr(), c"malloc".as_ptr()), (c"bash".as_ptr(), c"xmalloc".as_ptr()), (c"bash".as_ptr(), c"main".as_ptr())]),
        cc(3, &[(c"[kernel]".as_ptr(), c"sys_perf_event_open".as_ptr()), (c"perf".as_ptr(), c"run_command".as_ptr()), (c"perf".as_ptr(), c"main".as_ptr())]),
        cc(4, &[(c"[kernel]".as_ptr(), c"page_fault".as_ptr()), (c"[kernel]".as_ptr(), c"sys_perf_event_open".as_ptr()), (c"perf".as_ptr(), c"run_command".as_ptr()), (c"perf".as_ptr(), c"main".as_ptr())]),
        cc(3, &[(c"[kernel]".as_ptr(), c"schedule".as_ptr()), (c"perf".as_ptr(), c"run_command".as_ptr()), (c"perf".as_ptr(), c"main".as_ptr())]),
        cc(4, &[(c"libc".as_ptr(), c"free".as_ptr()), (c"perf".as_ptr(), c"cmd_record".as_ptr()), (c"perf".as_ptr(), c"run_command".as_ptr()), (c"perf".as_ptr(), c"main".as_ptr())]),
        cc(4, &[(c"libc".as_ptr(), c"malloc".as_ptr()), (c"perf".as_ptr(), c"cmd_record".as_ptr()), (c"perf".as_ptr(), c"run_command".as_ptr()), (c"perf".as_ptr(), c"main".as_ptr())]),
    ];

    symbol_conf.use_callchain = true;
    symbol_conf.cumulate_callchain = true;
    evsel__set_sample_bit(evsel, CALLCHAIN);

    setup_sorting(ptr::null_mut(), (*machine).env);

    callchain_param = callchain_param_default;
    callchain_register_param(&mut callchain_param);

    err = add_hist_entries(hists, machine);
    if err < 0 {
        del_hist_entries(hists);
        reset_output_field();
        return err;
    }

    err = do_test(hists, expected.as_mut_ptr(), expected.len(), expected_callchain.as_mut_ptr(), expected_callchain.len());

    del_hist_entries(hists);
    reset_output_field();
    err
}

unsafe fn cc(nr: u64, vals: &[(*const c_char, *const c_char)]) -> callchain_result {
    let mut out = callchain_result {
        nr,
        node: [EMPTY_CALLCHAIN_NODE; 10],
    };
    let mut i = 0;
    while i < vals.len() {
        out.node[i] = callchain_result_node {
            dso: vals[i].0,
            sym: vals[i].1,
        };
        i += 1;
    }
    out
}

unsafe fn test__hists_cumulate(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut err: c_int = TEST_FAIL;
    let mut machines: machines = core::mem::zeroed();
    let mut machine: *mut machine;
    let mut evsel: *mut evsel;
    let evlist = evlist__new();
    let mut i: usize;
    let testcases: [test_fn_t; 4] = [
        test1,
        test2,
        test3,
        test4,
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

// DEFINE_SUITE("Cumulate child hist entries", hists_cumulate);
DEFINE_SUITE(c"Cumulate child hist entries".as_ptr(), hists_cumulate);

unsafe extern "C" {
    static mut symbol_conf: symbol_conf_t;
    static mut callchain_param: callchain_param_t;
    static callchain_param_default: callchain_param_t;
    static hist_iter_cumulative: hist_entry_iter_ops;
    static hist_iter_normal: hist_entry_iter_ops;
    static mut verbose: c_int;
    static mut sysctl_perf_event_max_stack: c_int;
    static mut stderr: *mut c_void;

    static FAKE_PID_PERF1: u32;
    static FAKE_PID_PERF2: u32;
    static FAKE_PID_BASH: u32;
    static FAKE_IP_KERNEL_SCHEDULE: u64;
    static FAKE_IP_PERF_MAIN: u64;
    static FAKE_IP_PERF_CMD_RECORD: u64;
    static FAKE_IP_LIBC_MALLOC: u64;
    static FAKE_IP_LIBC_FREE: u64;
    static FAKE_IP_KERNEL_PAGE_FAULT: u64;
    static FAKE_IP_BASH_MAIN: u64;
    static FAKE_IP_BASH_XMALLOC: u64;
    static FAKE_IP_PERF_RUN_COMMAND: u64;
    static FAKE_IP_KERNEL_SYS_PERF_EVENT_OPEN: u64;

    static PERF_RECORD_MISC_USER: u32;
    static CALLCHAIN: c_int;
    static TEST_OK: c_int;
    static TEST_FAIL: c_int;
    static need_collapse: c_int;

    fn hists_to_evsel(hists: *mut hists) -> *mut evsel;
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn machine__resolve(machine: *mut machine, al: *mut addr_location, sample: *mut perf_sample) -> c_int;
    fn hist_entry_iter__add(iter: *mut hist_entry_iter, al: *mut addr_location, max_stack: c_int, arg: *mut c_void) -> c_int;
    fn thread__put(thread: *mut thread);
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn map__put(map: *mut map);
    fn map__get(map: *mut map) -> *mut map;
    fn hists__has(hists: *mut hists, flag: c_int) -> bool;
    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn rb_erase_cached(node: *mut rb_node, root: *mut rb_root_cached);
    fn hist_entry__delete(he: *mut hist_entry);
    fn map__zput(map: *mut *mut map);
    fn thread__zput(thread: *mut *mut thread);
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn hists__collapse_resort(hists: *mut hists, arg: *mut c_void);
    fn evsel__output_resort(evsel: *mut evsel, arg: *mut c_void);
    fn print_hists_out(hists: *mut hists);
    fn rb_first(root: *mut rb_root) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn evsel__reset_sample_bit(evsel: *mut evsel, bit: c_int);
    fn evsel__set_sample_bit(evsel: *mut evsel, bit: c_int);
    fn setup_sorting(evlist: *mut evlist, env: *mut machine_env);
    fn callchain_register_param(param: *mut callchain_param_t);
    fn reset_output_field();
    fn evlist__new() -> *mut evlist;
    fn parse_event(evlist: *mut evlist, event: *const c_char) -> c_int;
    fn machines__init(machines: *mut machines) -> c_int;
    fn setup_fake_machine(machines: *mut machines) -> *mut machine;
    fn machine__fprintf(machine: *mut machine, fp: *mut c_void);
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__put(evlist: *mut evlist);
    fn machines__exit(machines: *mut machines);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn TEST_ASSERT_VAL(msg: *const c_char, cond: bool);
    fn DEFINE_SUITE(desc: *const c_char, name: unsafe fn(*mut test_suite, c_int) -> c_int);

    fn rb_entry_hist_entry_rb_node(node: *mut rb_node) -> *mut hist_entry;
    fn rb_entry_callchain_node_rb_node(node: *mut rb_node) -> *mut callchain_node;
    fn list_first_entry_callchain_list(head: *mut list_head) -> *mut callchain_list;
    fn list_next_entry_callchain_list(pos: *mut callchain_list) -> *mut callchain_list;
    fn RB_EMPTY_ROOT(root: *mut rb_root) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
