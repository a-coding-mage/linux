// SPDX-License-Identifier: GPL-2.0
/*
 * builtin-annotate.c
 *
 * Builtin annotate command: Analyze the perf.data input file,
 * look up and read DSOs and symbol information and display
 * a histogram of results, along various sorting keys.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_float, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type bool_ = bool;
type u32 = u32;
type u64 = u64;

const MAX_NR_CPUS: usize = 4096;
const PERF_MAX_STACK_DEPTH: c_int = 127;
const PERF_DATA_MODE_READ: c_int = 0;
const HEADER_BRANCH_STACK: c_int = 0;
const CALLCHAIN: c_int = 0;
const SORT_MODE__BRANCH: c_int = 0;
const K_RIGHT: c_int = 0x105;
const K_LEFT: c_int = 0x104;
const NO_ADDR: u64 = !0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOTSUP: c_int = 95;
const PARSE_OPT_EXCLUSIVE: c_int = 1;

#[repr(C)]
pub struct perf_tool {
    pub sample: Option<
        unsafe extern "C" fn(
            *const perf_tool,
            *mut perf_event,
            *mut perf_sample,
            *mut machine,
        ) -> c_int,
    >,
    pub mmap: Option<unsafe extern "C" fn() -> c_int>,
    pub mmap2: Option<unsafe extern "C" fn() -> c_int>,
    pub comm: Option<unsafe extern "C" fn() -> c_int>,
    pub exit: Option<unsafe extern "C" fn() -> c_int>,
    pub fork: Option<unsafe extern "C" fn() -> c_int>,
    pub namespaces: Option<unsafe extern "C" fn() -> c_int>,
    pub attr: Option<unsafe extern "C" fn() -> c_int>,
    pub build_id: Option<unsafe extern "C" fn() -> c_int>,
    /* HAVE_LIBTRACEEVENT: tracing_data callback is present in C builds with libtraceevent. */
    pub tracing_data: Option<unsafe extern "C" fn() -> c_int>,
    pub id_index: Option<unsafe extern "C" fn() -> c_int>,
    pub auxtrace_info: Option<unsafe extern "C" fn() -> c_int>,
    pub auxtrace: Option<unsafe extern "C" fn() -> c_int>,
    pub feature: Option<unsafe extern "C" fn() -> c_int>,
    pub ordering_requires_timestamps: bool_,
}

#[repr(C)]
pub struct perf_session {
    pub header: perf_header,
    pub evlist: *mut evlist,
    pub data: *mut perf_data,
    pub itrace_synth_opts: *mut itrace_synth_opts,
}

#[repr(C)]
pub struct perf_header {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_annotate {
    pub tool: perf_tool,
    pub session: *mut perf_session,
    /* HAVE_SLANG_SUPPORT */
    pub use_tui: bool_,
    pub use_stdio: bool_,
    pub use_stdio2: bool_,
    /* HAVE_GTK2_SUPPORT */
    pub use_gtk: bool_,
    pub skip_missing: bool_,
    pub has_br_stack: bool_,
    pub group_set: bool_,
    pub data_type: bool_,
    pub type_stat: bool_,
    pub insn_stat: bool_,
    pub min_percent: c_float,
    pub sym_hist_filter: *const c_char,
    pub cpu_list: *const c_char,
    pub target_data_type: *const c_char,
    pub cpu_bitmap: [c_ulong; (MAX_NR_CPUS + c_ulong::BITS as usize - 1) / c_ulong::BITS as usize],
}

#[repr(C)]
pub struct symbol {
    pub rb_node: rb_node,
    pub name: *const c_char,
}

#[repr(C)]
pub struct annotation {
    pub src: *mut c_void,
}

#[repr(C)]
pub struct block_range_iter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct block_range {
    pub is_target: bool_,
    pub is_branch: bool_,
    pub entry: c_int,
    pub coverage: c_int,
    pub sym: *mut symbol,
    pub taken: c_int,
    pub pred: c_int,
}

#[repr(C)]
pub struct annotated_branch {
    pub max_coverage: c_int,
}

#[repr(C)]
pub struct branch_flags {
    pub predicted: bool_,
}

#[repr(C)]
pub struct branch_stack {
    pub nr: c_int,
}

#[repr(C)]
pub struct branch_info {
    pub from: addr_map_symbol,
    pub to: addr_map_symbol,
    pub flags: branch_flags,
}

#[repr(C)]
pub struct addr_map_symbol {
    pub addr: u64,
    pub ms: map_symbol,
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct addr_location {
    pub addr: u64,
    pub map: *mut map,
    pub sym: *mut symbol,
    pub filtered: bool_,
}

#[repr(C)]
pub struct perf_sample {
    pub evsel: *mut evsel,
    pub branch_stack: *mut branch_stack,
    pub file_offset: u64,
    pub cpu: c_uint,
}

#[repr(C)]
pub struct hist_entry_iter {
    pub sample: *mut perf_sample,
    pub add_entry_cb: Option<
        unsafe extern "C" fn(*mut hist_entry_iter, *mut addr_location, bool_, *mut c_void) -> c_int,
    >,
    pub hide_unresolved: bool_,
    pub ops: *const c_void,
    pub he: *mut hist_entry,
}

#[repr(C)]
pub struct hist_entry {
    pub rb_node: rb_node,
    pub branch_info: *mut branch_info,
    pub ms: map_symbol,
    pub stat: hist_entry_stat,
    pub mem_type: *mut annotated_data_type,
}

#[repr(C)]
pub struct hist_entry_stat {
    pub period: u64,
}

#[repr(C)]
pub struct annotated_data_type {
    pub histograms: *mut c_void,
    pub self_: annotated_data_type_self,
}

#[repr(C)]
pub struct annotated_data_type_self {
    pub type_name: *const c_char,
}

#[repr(C)]
pub struct hists {
    pub entries: rb_root_cached,
    pub stats: hists_stats,
}

#[repr(C)]
pub struct hists_stats {
    pub nr_samples: u32,
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
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
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
pub struct perf_data {
    pub mode: c_int,
    pub force: bool_,
    pub path: *const c_char,
}

#[repr(C)]
pub struct itrace_synth_opts {
    pub set: c_int,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_header {
    pub type_: c_uint,
}

#[repr(C)]
pub struct option {
    pub value: *mut c_void,
}

#[repr(C)]
pub struct annotated_data_stat {
    pub total: c_int,
    pub no_sym: c_int,
    pub no_insn: c_int,
    pub no_insn_ops: c_int,
    pub no_mem_ops: c_int,
    pub no_reg: c_int,
    pub no_dbginfo: c_int,
    pub no_cuinfo: c_int,
    pub no_var: c_int,
    pub no_typeinfo: c_int,
    pub invalid_size: c_int,
    pub bad_offset: c_int,
    pub insn_track: c_int,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct annotated_item_stat {
    pub list: list_head,
    pub good: c_int,
    pub bad: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub struct ui_progress {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hist_browser_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct annotation_options {
    pub objdump_path: *mut c_char,
    pub show_br_cntr: bool_,
    pub print_lines: bool_,
    pub full_path: bool_,
    pub annotate_src: bool_,
    pub show_asm_raw: bool_,
    pub prefix: *const c_char,
    pub prefix_strip: *const c_char,
    pub disassembler_style: *mut c_char,
    pub code_with_type: bool_,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub hide_unresolved: bool_,
    pub dso_list_str: *const c_char,
    pub ignore_vmlinux: bool_,
    pub vmlinux_name: *const c_char,
    pub use_modules: bool_,
    pub demangle: bool_,
    pub demangle_kernel: bool_,
    pub show_total_period: bool_,
    pub show_nr_samples: bool_,
    pub addr2line_path: *mut c_char,
    pub skip_empty: bool_,
    pub event_group: bool_,
    pub annotate_data_member: bool_,
    pub annotate_data_sample: bool_,
    pub try_vmlinux_path: bool_,
}

unsafe extern "C" {
    static mut symbol_conf: symbol_conf_t;
    static mut annotate_opts: annotation_options;
    static mut ann_data_stat: annotated_data_stat;
    static mut ann_insn_stat: list_head;
    static mut hist_iter_branch: c_void;
    static mut use_browser: c_int;
    static mut perf_gtk_handle: *mut c_void;
    static mut dump_trace: bool_;
    static mut verbose: c_int;
    static mut stdout: *mut c_void;
    static mut input_name: *const c_char;
    static mut quiet: bool_;
    static mut sort_order: *const c_char;
    static mut sort__mode: c_int;

    fn symbol__annotation(sym: *mut symbol) -> *mut annotation;
    fn block_range__create(start: u64, end: u64) -> block_range_iter;
    fn block_range_iter__valid(iter: *const block_range_iter) -> bool_;
    fn annotation__get_branch(notes: *mut annotation) -> *mut annotated_branch;
    fn block_range_iter(iter: *mut block_range_iter) -> *mut block_range;
    fn block_range_iter__next(iter: *mut block_range_iter) -> bool_;
    fn sample__resolve_bstack(sample: *mut perf_sample, al: *mut addr_location) -> *mut branch_info;
    fn free(ptr: *mut c_void);
    fn addr_map_symbol__inc_samples(ams: *mut addr_map_symbol, sample: *mut perf_sample) -> c_int;
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn machine__resolve(machine: *mut machine, al: *mut addr_location, sample: *mut perf_sample) -> c_int;
    fn dso__set_hit(dso: *mut dso);
    fn map__dso(map: *mut map) -> *mut dso;
    fn hist__account_cycles(
        bs: *mut branch_stack,
        al: *mut addr_location,
        sample: *mut perf_sample,
        nonany_branch_mode: bool_,
        total_cycles: *mut u64,
    );
    fn hist_entry_iter__add(
        iter: *mut hist_entry_iter,
        al: *mut addr_location,
        max_stack_depth: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn ui__has_annotation() -> bool_;
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn rb_erase_cached(node: *mut rb_node, root: *mut rb_root_cached);
    fn dso__symbols(dso: *mut dso) -> *mut rb_root_cached;
    fn symbol__delete(sym: *mut symbol);
    fn dso__reset_find_symbol_cache(dso: *mut dso);
    fn hists__add_entry(
        hists: *mut hists,
        al: *mut addr_location,
        a: *mut c_void,
        b: *mut c_void,
        c: *mut c_void,
        d: *mut c_void,
        sample: *mut perf_sample,
        sample_self: bool_,
    ) -> *mut hist_entry;
    fn hist_entry__inc_addr_samples(he: *mut hist_entry, sample: *mut perf_sample, addr: u64) -> c_int;
    fn hists__inc_nr_samples(hists: *mut hists, sample_self: bool_);
    fn perf_event__name(type_: c_uint) -> *const c_char;
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn hist_entry__tty_annotate2(he: *mut hist_entry, evsel: *mut evsel) -> c_int;
    fn hist_entry__tty_annotate(he: *mut hist_entry, evsel: *mut evsel) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn rb_prev(node: *mut rb_node) -> *mut rb_node;
    fn dso__annotate_warned(dso: *mut dso) -> bool_;
    fn hists__total_period(hists: *mut hists) -> u64;
    fn hist_entry__annotate_data_tui(
        he: *mut hist_entry,
        evsel: *mut evsel,
        hbt: *mut hist_browser_timer,
    ) -> c_int;
    fn hist_entry__annotate_data_tty(he: *mut hist_entry, evsel: *mut evsel) -> c_int;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn ui__error(fmt: *const c_char, ...);
    fn hist_entry__tui_annotate(
        he: *mut hist_entry,
        evsel: *mut evsel,
        hbt: *mut hist_browser_timer,
        addr: u64,
    ) -> c_int;
    fn perf_session__cpu_bitmap(session: *mut perf_session, cpu_list: *const c_char, bitmap: *mut c_ulong) -> c_int;
    fn perf_env__lookup_objdump(env: *mut c_void, path: *mut *mut c_char) -> c_int;
    fn perf_session__env(session: *mut perf_session) -> *mut c_void;
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn evlist__nr_br_cntr(evlist: *mut evlist) -> c_int;
    fn perf_session__fprintf_nr_events(session: *mut perf_session, fp: *mut c_void);
    fn evlist__fprintf_nr_events(evlist: *mut evlist, fp: *mut c_void);
    fn perf_session__fprintf(session: *mut perf_session, fp: *mut c_void);
    fn perf_session__fprintf_dsos(session: *mut perf_session, fp: *mut c_void);
    fn ui_progress__init(prog: *mut ui_progress, total: u32, title: *const c_char);
    fn hists__collapse_resort(hists: *mut hists, prog: *mut ui_progress);
    fn ui_progress__finish();
    fn evsel__reset_sample_bit(evsel: *mut evsel, bit: c_int);
    fn evsel__output_resort(evsel: *mut evsel, prog: *mut ui_progress);
    fn evsel__is_group_leader(evsel: *mut evsel) -> bool_;
    fn evsel__leader(evsel: *mut evsel) -> *mut evsel;
    fn hists__match(leader_hists: *mut hists, hists: *mut hists);
    fn hists__link(leader_hists: *mut hists, hists: *mut hists);
    fn strtof(str_: *const c_char, endptr: *mut *mut c_char) -> c_float;
    fn strdup(str_: *const c_char) -> *mut c_char;
    fn set_option_flag(options: *mut option, shorts: c_int, long: *const c_char, flag: c_int);
    fn annotation_options__init();
    fn hists__init() -> c_int;
    fn annotation_config__init();
    fn parse_options(
        argc: c_int,
        argv: *mut *const c_char,
        options: *mut option,
        usage: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usage: *const *const c_char, options: *mut option) -> !;
    fn annotate_check_args() -> c_int;
    fn symbol__validate_sym_arguments() -> c_int;
    fn perf_quiet_option();
    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool_);
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn perf_header__has_feat(header: *mut perf_header, feat: c_int) -> bool_;
    fn evlist__force_leader(evlist: *mut evlist);
    fn symbol__annotation_init() -> c_int;
    fn symbol__init(env: *mut c_void) -> c_int;
    fn setup_browser(fallback_to_pager: bool_);
    fn setup_sorting(evlist: *mut evlist, env: *mut c_void) -> c_int;
    fn perf_session__delete(session: *mut perf_session);
    fn annotation_options__exit();
    fn perf_event__process_mmap() -> c_int;
    fn perf_event__process_mmap2() -> c_int;
    fn perf_event__process_comm() -> c_int;
    fn perf_event__process_exit() -> c_int;
    fn perf_event__process_fork() -> c_int;
    fn perf_event__process_namespaces() -> c_int;
    fn perf_event__process_attr() -> c_int;
    fn perf_event__process_build_id() -> c_int;
    fn perf_event__process_tracing_data() -> c_int;
    fn perf_event__process_id_index() -> c_int;
    fn perf_event__process_auxtrace_info() -> c_int;
    fn perf_event__process_auxtrace() -> c_int;
    fn perf_event__process_feature() -> c_int;
}

#[inline]
unsafe fn test_bit(bit: c_uint, bitmap: *const c_ulong) -> bool_ {
    let word = bit as usize / c_ulong::BITS as usize;
    let shift = bit as usize % c_ulong::BITS as usize;
    ((*bitmap.add(word)) & ((1 as c_ulong) << shift)) != 0
}

#[inline]
unsafe fn rb_entry_hist_entry(node: *mut rb_node) -> *mut hist_entry {
    node as *mut hist_entry
}

#[inline]
unsafe fn list_entry_annotated_item_stat(ptr: *mut list_head) -> *mut annotated_item_stat {
    ptr as *mut annotated_item_stat
}

/*
 * Given one basic block:
 *
 *	from	to		branch_i
 *	* ----> *
 *		|
 *		| block
 *		v
 *		* ----> *
 *		from	to	branch_i+1
 *
 * where the horizontal are the branches and the vertical is the executed
 * block of instructions.
 *
 * We count, for each 'instruction', the number of blocks that covered it as
 * well as count the ratio each branch is taken.
 *
 * We can do this without knowing the actual instruction stream by keeping
 * track of the address ranges. We break down ranges such that there is no
 * overlap and iterate from the start until the end.
 *
 * @acme: once we parse the objdump output _before_ processing the samples,
 * we can easily fold the branch.cycles IPC bits in.
 */
unsafe extern "C" fn process_basic_block(
    start: *mut addr_map_symbol,
    end: *mut addr_map_symbol,
    flags: *mut branch_flags,
) {
    let sym = (*start).ms.sym;
    let notes = if !sym.is_null() {
        symbol__annotation(sym)
    } else {
        ptr::null_mut()
    };
    let mut iter: block_range_iter;
    let mut entry: *mut block_range;
    let branch: *mut annotated_branch;

    /*
     * Sanity; NULL isn't executable and the CPU cannot execute backwards
     */
    if (*start).addr == 0 || (*start).addr > (*end).addr {
        return;
    }

    iter = block_range__create((*start).addr, (*end).addr);
    if !block_range_iter__valid(&iter) {
        return;
    }

    branch = annotation__get_branch(notes);

    /*
     * First block in range is a branch target.
     */
    entry = block_range_iter(&mut iter);
    debug_assert!((*entry).is_target);
    (*entry).entry += 1;

    loop {
        entry = block_range_iter(&mut iter);

        (*entry).coverage += 1;
        (*entry).sym = sym;

        if !branch.is_null() {
            (*branch).max_coverage = (*branch).max_coverage.max((*entry).coverage);
        }

        if !block_range_iter__next(&mut iter) {
            break;
        }
    }

    /*
     * Last block in rage is a branch.
     */
    entry = block_range_iter(&mut iter);
    debug_assert!((*entry).is_branch);
    (*entry).taken += 1;
    if (*flags).predicted {
        (*entry).pred += 1;
    }
}

unsafe extern "C" fn process_branch_stack(
    bs: *mut branch_stack,
    al: *mut addr_location,
    sample: *mut perf_sample,
) {
    let mut prev: *mut addr_map_symbol = ptr::null_mut();
    let bi: *mut branch_info;
    let mut i: c_int;

    if bs.is_null() || (*bs).nr == 0 {
        return;
    }

    bi = sample__resolve_bstack(sample, al);
    if bi.is_null() {
        return;
    }

    i = (*bs).nr - 1;
    while i >= 0 {
        /*
         * XXX filter against symbol
         */
        if !prev.is_null() {
            process_basic_block(prev, &mut (*bi.add(i as usize)).from, &mut (*bi.add(i as usize)).flags);
        }
        prev = &mut (*bi.add(i as usize)).to;
        i -= 1;
    }

    free(bi as *mut c_void);
}

unsafe extern "C" fn hist_iter__branch_callback(
    iter: *mut hist_entry_iter,
    _al: *mut addr_location,
    _single: bool_,
    _arg: *mut c_void,
) -> c_int {
    let he = (*iter).he;
    let bi: *mut branch_info;
    let sample = (*iter).sample;
    let mut err: c_int;

    bi = (*he).branch_info;
    err = addr_map_symbol__inc_samples(&mut (*bi).from, sample);

    if err != 0 {
        return err;
    }

    err = addr_map_symbol__inc_samples(&mut (*bi).to, sample);

    err
}

unsafe extern "C" fn process_branch_callback(
    sample: *mut perf_sample,
    al: *mut addr_location,
    ann: *mut perf_annotate,
    machine: *mut machine,
) -> c_int {
    let mut iter = hist_entry_iter {
        sample,
        add_entry_cb: Some(hist_iter__branch_callback),
        hide_unresolved: symbol_conf.hide_unresolved,
        ops: &hist_iter_branch as *const c_void,
        he: ptr::null_mut(),
    };
    let mut a: addr_location = mem::zeroed();
    let ret: c_int;

    addr_location__init(&mut a);
    if machine__resolve(machine, &mut a, sample) < 0 {
        ret = -1;
    } else if a.sym.is_null() {
        ret = 0;
    } else {
        if !a.map.is_null() {
            dso__set_hit(map__dso(a.map));
        }

        hist__account_cycles((*sample).branch_stack, al, sample, false, ptr::null_mut());

        ret = hist_entry_iter__add(&mut iter, &mut a, PERF_MAX_STACK_DEPTH, ann as *mut c_void);
    }
    addr_location__exit(&mut a);
    ret
}

unsafe extern "C" fn has_annotation(ann: *mut perf_annotate) -> bool_ {
    ui__has_annotation() || (*ann).use_stdio2
}

unsafe extern "C" fn add_sample(
    sample: *mut perf_sample,
    al: *mut addr_location,
    ann: *mut perf_annotate,
    machine: *mut machine,
) -> c_int {
    let hists = evsel__hists((*sample).evsel);
    let he: *mut hist_entry;
    let ret: c_int;

    if ((!(*ann).has_br_stack || !has_annotation(ann))
        && !(*ann).sym_hist_filter.is_null()
        && ((*al).sym.is_null() || strcmp((*ann).sym_hist_filter, (*(*al).sym).name) != 0))
    {
        /* We're only interested in a symbol named sym_hist_filter */
        /*
         * FIXME: why isn't this done in the symbol_filter when loading
         * the DSO?
         */
        if !(*al).sym.is_null() {
            let dso = map__dso((*al).map);

            rb_erase_cached(&mut (*(*al).sym).rb_node, dso__symbols(dso));
            symbol__delete((*al).sym);
            dso__reset_find_symbol_cache(dso);
        }
        return 0;
    }

    /*
     * XXX filtered samples can still have branch entries pointing into our
     * symbol and are missed.
     */
    process_branch_stack((*sample).branch_stack, al, sample);

    if (*ann).has_br_stack && has_annotation(ann) {
        return process_branch_callback(sample, al, ann, machine);
    }

    he = hists__add_entry(
        hists,
        al,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        sample,
        true,
    );
    if he.is_null() {
        return -ENOMEM;
    }

    ret = hist_entry__inc_addr_samples(he, sample, (*al).addr);
    hists__inc_nr_samples(hists, true);
    ret
}

unsafe extern "C" fn process_sample_event(
    tool: *const perf_tool,
    event: *mut perf_event,
    sample: *mut perf_sample,
    machine: *mut machine,
) -> c_int {
    let ann = tool as *mut perf_annotate;
    let mut al: addr_location = mem::zeroed();
    let mut ret: c_int = 0;

    addr_location__init(&mut al);
    if machine__resolve(machine, &mut al, sample) < 0 {
        pr_warning(
            c"problem processing %s (%u) event at offset %#lx, skipping it.\n".as_ptr(),
            perf_event__name((*event).header.type_),
            (*event).header.type_,
            (*sample).file_offset,
        );
        ret = -1;
    } else {
        if !(*ann).cpu_list.is_null()
            && ((*sample).cpu >= MAX_NR_CPUS as c_uint
                || !test_bit((*sample).cpu, (*ann).cpu_bitmap.as_ptr()))
        {
            addr_location__exit(&mut al);
            return ret;
        }

        if !al.filtered && add_sample(sample, &mut al, ann, machine) != 0 {
            pr_warning(c"problem incrementing symbol count, skipping event\n".as_ptr());
            ret = -1;
        }
    }
    addr_location__exit(&mut al);
    ret
}

unsafe extern "C" fn hist_entry__stdio_annotate(
    he: *mut hist_entry,
    evsel: *mut evsel,
    ann: *mut perf_annotate,
) -> c_int {
    if (*ann).use_stdio2 {
        return hist_entry__tty_annotate2(he, evsel);
    }

    hist_entry__tty_annotate(he, evsel)
}

unsafe extern "C" fn print_annotate_data_stat(s: *mut annotated_data_stat) {
    let bad = (*s).no_sym
        + (*s).no_insn
        + (*s).no_insn_ops
        + (*s).no_mem_ops
        + (*s).no_reg
        + (*s).no_dbginfo
        + (*s).no_cuinfo
        + (*s).no_var
        + (*s).no_typeinfo
        + (*s).invalid_size
        + (*s).bad_offset;
    let ok = (*s).total - bad;
    let denom = if (*s).total != 0 { (*s).total } else { 1 };

    printf(c"Annotate data type stats:\n".as_ptr());
    printf(
        c"total %d, ok %d (%.1f%%), bad %d (%.1f%%)\n".as_ptr(),
        (*s).total,
        ok,
        100.0f64 * ok as c_double / denom as c_double,
        bad,
        100.0f64 * bad as c_double / denom as c_double,
    );
    printf(c"-----------------------------------------------------------\n".as_ptr());

    macro_rules! print_stat {
        ($fld:ident, $name:literal) => {
            if (*s).$fld != 0 {
                printf(c"%10d : %s\n".as_ptr(), (*s).$fld, concat!($name, "\0").as_ptr());
            }
        };
    }

    print_stat!(no_sym, "no_sym");
    print_stat!(no_insn, "no_insn");
    print_stat!(no_insn_ops, "no_insn_ops");
    print_stat!(no_mem_ops, "no_mem_ops");
    print_stat!(no_reg, "no_reg");
    print_stat!(no_dbginfo, "no_dbginfo");
    print_stat!(no_cuinfo, "no_cuinfo");
    print_stat!(no_var, "no_var");
    print_stat!(no_typeinfo, "no_typeinfo");
    print_stat!(invalid_size, "invalid_size");
    print_stat!(bad_offset, "bad_offset");
    print_stat!(insn_track, "insn_track");
    printf(c"\n".as_ptr());
}

unsafe extern "C" fn print_annotate_item_stat(head: *mut list_head, title: *const c_char) {
    let mut istat: *mut annotated_item_stat;
    let mut pos: *mut annotated_item_stat;
    let mut iter: *mut annotated_item_stat;
    let mut total_good: c_int;
    let mut total_bad: c_int;
    let total: c_int;
    let mut sum1: c_int;
    let mut sum2: c_int;
    let mut tmp = list_head {
        next: ptr::null_mut(),
        prev: ptr::null_mut(),
    };

    /* sort the list by count */
    list_splice_init(head, &mut tmp);
    total_good = 0;
    total_bad = 0;

    pos = list_entry_annotated_item_stat(tmp.next);
    while &mut (*pos).list as *mut list_head != &mut tmp {
        istat = pos;
        pos = list_entry_annotated_item_stat((*(*pos).list.next).next);
        total_good += (*istat).good;
        total_bad += (*istat).bad;
        sum1 = (*istat).good + (*istat).bad;

        iter = list_entry_annotated_item_stat((*head).next);
        while &mut (*iter).list as *mut list_head != head {
            sum2 = (*iter).good + (*iter).bad;
            if sum1 > sum2 {
                break;
            }
            iter = list_entry_annotated_item_stat((*iter).list.next);
        }
        list_move_tail(&mut (*istat).list, &mut (*iter).list);
    }
    total = total_good + total_bad;
    let denom = if total != 0 { total } else { 1 };

    printf(c"Annotate %s stats\n".as_ptr(), title);
    printf(
        c"total %d, ok %d (%.1f%%), bad %d (%.1f%%)\n\n".as_ptr(),
        total,
        total_good,
        100.0f64 * total_good as c_double / denom as c_double,
        total_bad,
        100.0f64 * total_bad as c_double / denom as c_double,
    );
    printf(c"  %-20s: %5s %5s\n".as_ptr(), c"Name/opcode".as_ptr(), c"Good".as_ptr(), c"Bad".as_ptr());
    printf(c"-----------------------------------------------------------\n".as_ptr());
    iter = list_entry_annotated_item_stat((*head).next);
    while &mut (*iter).list as *mut list_head != head {
        printf(c"  %-20s: %5d %5d\n".as_ptr(), (*iter).name, (*iter).good, (*iter).bad);
        iter = list_entry_annotated_item_stat((*iter).list.next);
    }
    printf(c"\n".as_ptr());
}

unsafe fn list_splice_init(head: *mut list_head, list: *mut list_head) {
    if (*head).next == head {
        (*list).next = list;
        (*list).prev = list;
    } else {
        (*list).next = (*head).next;
        (*list).prev = (*head).prev;
        (*(*list).next).prev = list;
        (*(*list).prev).next = list;
        (*head).next = head;
        (*head).prev = head;
    }
}

unsafe fn list_move_tail(entry: *mut list_head, head: *mut list_head) {
    (*(*entry).prev).next = (*entry).next;
    (*(*entry).next).prev = (*entry).prev;
    (*entry).next = head;
    (*entry).prev = (*head).prev;
    (*(*head).prev).next = entry;
    (*head).prev = entry;
}

unsafe extern "C" fn hists__find_annotations(
    hists: *mut hists,
    evsel: *mut evsel,
    ann: *mut perf_annotate,
) {
    let mut nd = rb_first_cached(&mut (*hists).entries);
    let mut next: *mut rb_node;
    let mut key = K_RIGHT;

    if (*ann).type_stat {
        print_annotate_data_stat(&mut ann_data_stat);
    }
    if (*ann).insn_stat {
        print_annotate_item_stat(&mut ann_insn_stat, c"Instruction".as_ptr());
    }

    while !nd.is_null() {
        let he = rb_entry_hist_entry(nd);
        let notes: *mut annotation;

        if (*he).ms.sym.is_null() || dso__annotate_warned(map__dso((*he).ms.map)) {
            if key == K_LEFT || key == b'<' as c_int {
                nd = rb_prev(nd);
            } else {
                nd = rb_next(nd);
            }
            continue;
        }

        if !(*ann).sym_hist_filter.is_null()
            && strcmp((*(*he).ms.sym).name, (*ann).sym_hist_filter) != 0
        {
            if key == K_LEFT || key == b'<' as c_int {
                nd = rb_prev(nd);
            } else {
                nd = rb_next(nd);
            }
            continue;
        }

        if (*ann).min_percent != 0.0 {
            let mut percent: c_float = 0.0;
            let total = hists__total_period(hists);

            if total != 0 {
                percent = (100.0 * (*he).stat.period as c_float) / total as c_float;
            }

            if percent < (*ann).min_percent {
                if key == K_LEFT || key == b'<' as c_int {
                    nd = rb_prev(nd);
                } else {
                    nd = rb_next(nd);
                }
                continue;
            }
        }

        notes = symbol__annotation((*he).ms.sym);
        if (*notes).src.is_null() {
            if key == K_LEFT || key == b'<' as c_int {
                nd = rb_prev(nd);
            } else {
                nd = rb_next(nd);
            }
            continue;
        }

        if (*ann).data_type {
            /* skip unknown type */
            if (*(*he).mem_type).histograms.is_null() {
                if key == K_LEFT || key == b'<' as c_int {
                    nd = rb_prev(nd);
                } else {
                    nd = rb_next(nd);
                }
                continue;
            }

            if !(*ann).target_data_type.is_null() {
                let mut type_name = (*(*he).mem_type).self_.type_name;

                /* skip 'struct ' prefix in the type name */
                if strncmp((*ann).target_data_type, c"struct ".as_ptr(), 7) != 0
                    && strncmp(type_name, c"struct ".as_ptr(), 7) == 0
                {
                    type_name = type_name.add(7);
                }

                /* skip 'union ' prefix in the type name */
                if strncmp((*ann).target_data_type, c"union ".as_ptr(), 6) != 0
                    && strncmp(type_name, c"union ".as_ptr(), 6) == 0
                {
                    type_name = type_name.add(6);
                }

                if strcmp((*ann).target_data_type, type_name) != 0 {
                    if key == K_LEFT || key == b'<' as c_int {
                        nd = rb_prev(nd);
                    } else {
                        nd = rb_next(nd);
                    }
                    continue;
                }
            }

            if use_browser == 1 {
                key = hist_entry__annotate_data_tui(he, evsel, ptr::null_mut());
            } else {
                key = hist_entry__annotate_data_tty(he, evsel);
            }

            match key {
                -1 => {
                    if !(*ann).skip_missing {
                        return;
                    }
                    next = rb_next(nd);
                }
                k if k == K_RIGHT || k == b'>' as c_int => {
                    next = rb_next(nd);
                }
                k if k == K_LEFT || k == b'<' as c_int => {
                    next = rb_prev(nd);
                }
                _ => return,
            }

            if use_browser == 0 || !next.is_null() {
                nd = next;
            }

            continue;
        }

        if use_browser == 2 {
            let ret: c_int;
            type AnnotateFn = unsafe extern "C" fn(*mut hist_entry, *mut evsel, *mut hist_browser_timer) -> c_int;
            let annotate_sym = dlsym(perf_gtk_handle, c"hist_entry__gtk_annotate".as_ptr());
            if annotate_sym.is_null() {
                ui__error(c"GTK browser not found!\n".as_ptr());
                return;
            }
            let annotate: AnnotateFn = mem::transmute(annotate_sym);

            ret = annotate(he, evsel, ptr::null_mut());
            if ret == 0 || !(*ann).skip_missing {
                return;
            }

            /* skip missing symbols */
            nd = rb_next(nd);
        } else if use_browser == 1 {
            key = hist_entry__tui_annotate(he, evsel, ptr::null_mut(), NO_ADDR);

            match key {
                -1 => {
                    if !(*ann).skip_missing {
                        return;
                    }
                    next = rb_next(nd);
                }
                k if k == K_RIGHT || k == b'>' as c_int => {
                    next = rb_next(nd);
                }
                k if k == K_LEFT || k == b'<' as c_int => {
                    next = rb_prev(nd);
                }
                _ => return,
            }

            if !next.is_null() {
                nd = next;
            }
        } else {
            hist_entry__stdio_annotate(he, evsel, ann);
            nd = rb_next(nd);
        }
    }
}

unsafe extern "C" fn __cmd_annotate(ann: *mut perf_annotate) -> c_int {
    let mut ret: c_int;
    let session = (*ann).session;
    let mut pos: *mut evsel;
    let mut total_nr_samples: u64;

    if !(*ann).cpu_list.is_null() {
        ret = perf_session__cpu_bitmap(session, (*ann).cpu_list, (*ann).cpu_bitmap.as_mut_ptr());
        if ret != 0 {
            return ret;
        }
    }

    if annotate_opts.objdump_path.is_null() {
        ret = perf_env__lookup_objdump(perf_session__env(session), &mut annotate_opts.objdump_path);
        if ret != 0 {
            return ret;
        }
    }

    ret = perf_session__process_events(session);
    if ret != 0 {
        return ret;
    }

    if (use_browser == 1 || (*ann).use_stdio2) && (*ann).has_br_stack {
        if evlist__nr_br_cntr((*session).evlist) > 0 {
            annotate_opts.show_br_cntr = true;
        }
    }

    if dump_trace {
        perf_session__fprintf_nr_events(session, stdout);
        evlist__fprintf_nr_events((*session).evlist, stdout);
        return ret;
    }

    if verbose > 3 {
        perf_session__fprintf(session, stdout);
    }

    if verbose > 2 {
        perf_session__fprintf_dsos(session, stdout);
    }

    total_nr_samples = 0;
    pos = evlist_first((*session).evlist);
    while !pos.is_null() {
        let hists = evsel__hists(pos);
        let nr_samples = (*hists).stats.nr_samples;
        let mut prog: ui_progress = mem::zeroed();

        if nr_samples > 0 {
            total_nr_samples += nr_samples as u64;

            ui_progress__init(&mut prog, nr_samples, c"Merging related events...".as_ptr());
            hists__collapse_resort(hists, &mut prog);
            ui_progress__finish();

            /* Don't sort callchain */
            evsel__reset_sample_bit(pos, CALLCHAIN);

            ui_progress__init(&mut prog, nr_samples, c"Sorting events for output...".as_ptr());
            evsel__output_resort(pos, &mut prog);
            ui_progress__finish();

            /*
             * An event group needs to display other events too.
             * Let's delay printing until other events are processed.
             */
            if symbol_conf.event_group {
                if !evsel__is_group_leader(pos) {
                    let leader_hists = evsel__hists(evsel__leader(pos));
                    hists__match(leader_hists, hists);
                    hists__link(leader_hists, hists);
                }
                pos = evlist_next((*session).evlist, pos);
                continue;
            }

            hists__find_annotations(hists, pos, ann);
        }
        pos = evlist_next((*session).evlist, pos);
    }

    if total_nr_samples == 0 {
        ui__error(c"The %s data has no samples!\n".as_ptr(), (*(*session).data).path);
        return ret;
    }

    /* Display group events together */
    pos = evlist_first((*session).evlist);
    while !pos.is_null() {
        let hists = evsel__hists(pos);
        let mut nr_samples = (*hists).stats.nr_samples;
        let mut prog: ui_progress = mem::zeroed();
        let mut evsel_: *mut evsel;

        if !symbol_conf.event_group || !evsel__is_group_leader(pos) {
            pos = evlist_next((*session).evlist, pos);
            continue;
        }

        evsel_ = group_first(pos);
        while !evsel_.is_null() {
            nr_samples += (*evsel__hists(evsel_)).stats.nr_samples;
            evsel_ = group_next(pos, evsel_);
        }

        if nr_samples == 0 {
            pos = evlist_next((*session).evlist, pos);
            continue;
        }

        ui_progress__init(&mut prog, nr_samples, c"Sorting group events for output...".as_ptr());
        evsel__output_resort(pos, &mut prog);
        ui_progress__finish();

        hists__find_annotations(hists, pos, ann);
        pos = evlist_next((*session).evlist, pos);
    }

    if use_browser == 2 {
        type ShowAnnotationsFn = unsafe extern "C" fn();
        let show_annotations_sym = dlsym(perf_gtk_handle, c"perf_gtk__show_annotations".as_ptr());
        if show_annotations_sym.is_null() {
            ui__error(c"GTK browser not found!\n".as_ptr());
            return ret;
        }
        let show_annotations: ShowAnnotationsFn = mem::transmute(show_annotations_sym);
        show_annotations();
    }

    ret
}

unsafe extern "C" {
    fn evlist_first(evlist: *mut evlist) -> *mut evsel;
    fn evlist_next(evlist: *mut evlist, pos: *mut evsel) -> *mut evsel;
    fn group_first(leader: *mut evsel) -> *mut evsel;
    fn group_next(leader: *mut evsel, pos: *mut evsel) -> *mut evsel;
}

unsafe extern "C" fn parse_percent_limit(
    opt: *const option,
    str_: *const c_char,
    _unset: c_int,
) -> c_int {
    let ann = (*opt).value as *mut perf_annotate;
    let pcnt = strtof(str_, ptr::null_mut());

    (*ann).min_percent = pcnt;
    0
}

unsafe extern "C" fn parse_data_type(opt: *const option, str_: *const c_char, unset: c_int) -> c_int {
    let ann = (*opt).value as *mut perf_annotate;

    (*ann).data_type = unset == 0;
    if !str_.is_null() {
        (*ann).target_data_type = strdup(str_);
    }

    0
}

static annotate_usage_0: &[u8] = b"perf annotate [<options>]\0";
static annotate_usage: [*const c_char; 2] = [annotate_usage_0.as_ptr() as *const c_char, ptr::null()];

unsafe extern "C" {
    fn OPT_STRING(short: c_int, long: *const c_char, value: *mut *const c_char, metavar: *const c_char, help: *const c_char) -> option;
    fn OPT_BOOLEAN(short: c_int, long: *const c_char, value: *mut bool_, help: *const c_char) -> option;
    fn OPT_INCR(short: c_int, long: *const c_char, value: *mut c_int, help: *const c_char) -> option;
    fn OPT_BOOLEAN_SET(
        short: c_int,
        long: *const c_char,
        value: *mut bool_,
        set: *mut bool_,
        help: *const c_char,
    ) -> option;
    fn OPT_CALLBACK(
        short: c_int,
        long: *const c_char,
        value: *mut c_void,
        metavar: *const c_char,
        help: *const c_char,
        callback: unsafe extern "C" fn(*const option, *const c_char, c_int) -> c_int,
    ) -> option;
    fn OPT_CALLBACK_DEFAULT(
        short: c_int,
        long: *const c_char,
        value: *mut c_void,
        metavar: *const c_char,
        help: *const c_char,
        callback: unsafe extern "C" fn(*const option, *const c_char, c_int) -> c_int,
        defval: *const c_char,
    ) -> option;
    fn OPT_CALLBACK_OPTARG(
        short: c_int,
        long: *const c_char,
        value: *mut c_void,
        metavar: *const c_char,
        help: *const c_char,
        callback: unsafe extern "C" fn(*const option, *const c_char, c_int) -> c_int,
    ) -> option;
    fn OPT_END() -> option;
    fn symbol__config_symfs(opt: *const option, str_: *const c_char, unset: c_int) -> c_int;
    fn stdio__config_color(opt: *const option, str_: *const c_char, unset: c_int) -> c_int;
    fn annotate_parse_percent_type(opt: *const option, str_: *const c_char, unset: c_int) -> c_int;
    fn itrace_parse_synth_opts(opt: *const option, str_: *const c_char, unset: c_int) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_annotate(mut argc: c_int, argv: *mut *const c_char) -> c_int {
    let mut annotate: perf_annotate = mem::zeroed();
    let mut data = perf_data {
        mode: PERF_DATA_MODE_READ,
        force: false,
        path: ptr::null(),
    };
    let mut itrace_synth_opts = itrace_synth_opts { set: 0 };
    let mut disassembler_style: *const c_char = ptr::null();
    let mut objdump_path: *const c_char = ptr::null();
    let mut addr2line_path: *const c_char = ptr::null();
    let mut options: [option; 45] = [
        OPT_STRING(b'i' as c_int, c"input".as_ptr(), &mut input_name, c"file".as_ptr(), c"input file name".as_ptr()),
        OPT_STRING(b'd' as c_int, c"dsos".as_ptr(), &mut symbol_conf.dso_list_str, c"dso[,dso...]".as_ptr(), c"only consider symbols in these dsos".as_ptr()),
        OPT_STRING(b's' as c_int, c"symbol".as_ptr(), &mut annotate.sym_hist_filter, c"symbol".as_ptr(), c"symbol to annotate".as_ptr()),
        OPT_BOOLEAN(b'f' as c_int, c"force".as_ptr(), &mut data.force, c"don't complain, do it".as_ptr()),
        OPT_INCR(b'v' as c_int, c"verbose".as_ptr(), &mut verbose, c"be more verbose (show symbol address, etc)".as_ptr()),
        OPT_BOOLEAN(b'q' as c_int, c"quiet".as_ptr(), &mut quiet, c"do now show any warnings or messages".as_ptr()),
        OPT_BOOLEAN(b'D' as c_int, c"dump-raw-trace".as_ptr(), &mut dump_trace, c"dump raw trace in ASCII".as_ptr()),
        /* HAVE_GTK2_SUPPORT */
        OPT_BOOLEAN(0, c"gtk".as_ptr(), &mut annotate.use_gtk, c"Use the GTK interface".as_ptr()),
        /* HAVE_SLANG_SUPPORT */
        OPT_BOOLEAN(0, c"tui".as_ptr(), &mut annotate.use_tui, c"Use the TUI interface".as_ptr()),
        OPT_BOOLEAN(0, c"stdio".as_ptr(), &mut annotate.use_stdio, c"Use the stdio interface".as_ptr()),
        OPT_BOOLEAN(0, c"stdio2".as_ptr(), &mut annotate.use_stdio2, c"Use the stdio interface".as_ptr()),
        OPT_BOOLEAN(0, c"ignore-vmlinux".as_ptr(), &mut symbol_conf.ignore_vmlinux, c"don't load vmlinux even if found".as_ptr()),
        OPT_STRING(b'k' as c_int, c"vmlinux".as_ptr(), &mut symbol_conf.vmlinux_name, c"file".as_ptr(), c"vmlinux pathname".as_ptr()),
        OPT_BOOLEAN(b'm' as c_int, c"modules".as_ptr(), &mut symbol_conf.use_modules, c"load module symbols - WARNING: use only with -k and LIVE kernel".as_ptr()),
        OPT_BOOLEAN(b'l' as c_int, c"print-line".as_ptr(), &mut annotate_opts.print_lines, c"print matching source lines (may be slow)".as_ptr()),
        OPT_BOOLEAN(b'P' as c_int, c"full-paths".as_ptr(), &mut annotate_opts.full_path, c"Don't shorten the displayed pathnames".as_ptr()),
        OPT_BOOLEAN(0, c"skip-missing".as_ptr(), &mut annotate.skip_missing, c"Skip symbols that cannot be annotated".as_ptr()),
        OPT_BOOLEAN_SET(0, c"group".as_ptr(), &mut symbol_conf.event_group, &mut annotate.group_set, c"Show event group information together".as_ptr()),
        OPT_STRING(b'C' as c_int, c"cpu".as_ptr(), &mut annotate.cpu_list, c"cpu".as_ptr(), c"list of cpus to profile".as_ptr()),
        OPT_CALLBACK(0, c"symfs".as_ptr(), ptr::null_mut(), c"directory[,layout]".as_ptr(), c"SYMFS_HELP".as_ptr(), symbol__config_symfs),
        OPT_BOOLEAN(0, c"source".as_ptr(), &mut annotate_opts.annotate_src, c"Interleave source code with assembly code (default)".as_ptr()),
        OPT_BOOLEAN(0, c"asm-raw".as_ptr(), &mut annotate_opts.show_asm_raw, c"Display raw encoding of assembly instructions (default)".as_ptr()),
        OPT_STRING(b'M' as c_int, c"disassembler-style".as_ptr(), &mut disassembler_style, c"disassembler style".as_ptr(), c"Specify disassembler style (e.g. -M intel for intel syntax)".as_ptr()),
        OPT_STRING(0, c"prefix".as_ptr(), &mut annotate_opts.prefix, c"prefix".as_ptr(), c"Add prefix to source file path names in programs (with --prefix-strip)".as_ptr()),
        OPT_STRING(0, c"prefix-strip".as_ptr(), &mut annotate_opts.prefix_strip, c"N".as_ptr(), c"Strip first N entries of source file path name in programs (with --prefix)".as_ptr()),
        OPT_STRING(0, c"objdump".as_ptr(), &mut objdump_path, c"path".as_ptr(), c"objdump binary to use for disassembly and annotations".as_ptr()),
        OPT_STRING(0, c"addr2line".as_ptr(), &mut addr2line_path, c"path".as_ptr(), c"addr2line binary to use for line numbers".as_ptr()),
        OPT_BOOLEAN(0, c"demangle".as_ptr(), &mut symbol_conf.demangle, c"Enable symbol demangling".as_ptr()),
        OPT_BOOLEAN(0, c"demangle-kernel".as_ptr(), &mut symbol_conf.demangle_kernel, c"Enable kernel symbol demangling".as_ptr()),
        OPT_BOOLEAN(0, c"show-total-period".as_ptr(), &mut symbol_conf.show_total_period, c"Show a column with the sum of periods".as_ptr()),
        OPT_BOOLEAN(b'n' as c_int, c"show-nr-samples".as_ptr(), &mut symbol_conf.show_nr_samples, c"Show a column with the number of samples".as_ptr()),
        OPT_CALLBACK_DEFAULT(0, c"stdio-color".as_ptr(), ptr::null_mut(), c"mode".as_ptr(), c"'always' (default), 'never' or 'auto' only applicable to --stdio mode".as_ptr(), stdio__config_color, c"always".as_ptr()),
        OPT_CALLBACK(0, c"percent-type".as_ptr(), &mut annotate_opts as *mut _ as *mut c_void, c"local-period".as_ptr(), c"Set percent type local/global-period/hits".as_ptr(), annotate_parse_percent_type),
        OPT_CALLBACK(0, c"percent-limit".as_ptr(), &mut annotate as *mut _ as *mut c_void, c"percent".as_ptr(), c"Don't show entries under that percent".as_ptr(), parse_percent_limit),
        OPT_CALLBACK_OPTARG(0, c"itrace".as_ptr(), &mut itrace_synth_opts as *mut _ as *mut c_void, ptr::null(), c"Instruction Tracing options\nITRACE_HELP".as_ptr(), itrace_parse_synth_opts),
        OPT_CALLBACK_OPTARG(0, c"data-type".as_ptr(), &mut annotate as *mut _ as *mut c_void, ptr::null(), c"Show data type annotate for the memory accesses".as_ptr(), parse_data_type),
        OPT_BOOLEAN(0, c"type-stat".as_ptr(), &mut annotate.type_stat, c"Show stats for the data type annotation".as_ptr()),
        OPT_BOOLEAN(0, c"insn-stat".as_ptr(), &mut annotate.insn_stat, c"Show instruction stats for the data type annotation".as_ptr()),
        OPT_BOOLEAN(0, c"skip-empty".as_ptr(), &mut symbol_conf.skip_empty, c"Do not display empty (or dummy) events in the output".as_ptr()),
        OPT_BOOLEAN(0, c"code-with-type".as_ptr(), &mut annotate_opts.code_with_type, c"Show data type info in code annotation (memory instructions only)".as_ptr()),
        OPT_END(),
        OPT_END(),
        OPT_END(),
        OPT_END(),
        OPT_END(),
    ];
    let mut ret: c_int;

    set_option_flag(options.as_mut_ptr(), 0, c"show-total-period".as_ptr(), PARSE_OPT_EXCLUSIVE);
    set_option_flag(options.as_mut_ptr(), 0, c"show-nr-samples".as_ptr(), PARSE_OPT_EXCLUSIVE);

    annotation_options__init();

    ret = hists__init();
    if ret < 0 {
        return ret;
    }

    annotation_config__init();

    argc = parse_options(argc, argv, options.as_mut_ptr(), annotate_usage.as_ptr(), 0);
    if argc != 0 {
        /*
         * Special case: if there's an argument left then assume that
         * it's a symbol filter:
         */
        if argc > 1 {
            usage_with_options(annotate_usage.as_ptr(), options.as_mut_ptr());
        }

        annotate.sym_hist_filter = *argv;
    }

    if !disassembler_style.is_null() {
        annotate_opts.disassembler_style = strdup(disassembler_style);
        if annotate_opts.disassembler_style.is_null() {
            return -ENOMEM;
        }
    }
    if !objdump_path.is_null() {
        annotate_opts.objdump_path = strdup(objdump_path);
        if annotate_opts.objdump_path.is_null() {
            return -ENOMEM;
        }
    }
    if !addr2line_path.is_null() {
        symbol_conf.addr2line_path = strdup(addr2line_path);
        if symbol_conf.addr2line_path.is_null() {
            return -ENOMEM;
        }
    }

    if annotate_check_args() < 0 {
        return -EINVAL;
    }

    /* HAVE_GTK2_SUPPORT */
    if symbol_conf.show_nr_samples && annotate.use_gtk {
        pr_err(c"--show-nr-samples is not available in --gtk mode at this time\n".as_ptr());
        return ret;
    }

    /* !HAVE_LIBDW_SUPPORT */
    if false && annotate.data_type {
        pr_err(c"Error: Data type profiling is disabled due to missing DWARF support\n".as_ptr());
        return -ENOTSUP;
    }

    ret = symbol__validate_sym_arguments();
    if ret != 0 {
        return ret;
    }

    if quiet {
        perf_quiet_option();
    }

    data.path = input_name;

    perf_tool__init(&mut annotate.tool, true);
    annotate.tool.sample = Some(process_sample_event);
    annotate.tool.mmap = Some(perf_event__process_mmap);
    annotate.tool.mmap2 = Some(perf_event__process_mmap2);
    annotate.tool.comm = Some(perf_event__process_comm);
    annotate.tool.exit = Some(perf_event__process_exit);
    annotate.tool.fork = Some(perf_event__process_fork);
    annotate.tool.namespaces = Some(perf_event__process_namespaces);
    annotate.tool.attr = Some(perf_event__process_attr);
    annotate.tool.build_id = Some(perf_event__process_build_id);
    /* HAVE_LIBTRACEEVENT */
    annotate.tool.tracing_data = Some(perf_event__process_tracing_data);
    annotate.tool.id_index = Some(perf_event__process_id_index);
    annotate.tool.auxtrace_info = Some(perf_event__process_auxtrace_info);
    annotate.tool.auxtrace = Some(perf_event__process_auxtrace);
    annotate.tool.feature = Some(perf_event__process_feature);
    annotate.tool.ordering_requires_timestamps = true;

    annotate.session = perf_session__new(&mut data, &mut annotate.tool);
    if IS_ERR(annotate.session as *const c_void) {
        return PTR_ERR(annotate.session as *const c_void);
    }

    (*annotate.session).itrace_synth_opts = &mut itrace_synth_opts;

    annotate.has_br_stack =
        perf_header__has_feat(&mut (*annotate.session).header, HEADER_BRANCH_STACK);

    if annotate.group_set {
        evlist__force_leader((*annotate.session).evlist);
    }

    ret = symbol__annotation_init();
    if ret < 0 {
        annotation_options__exit();
        return ret;
    }

    symbol_conf.try_vmlinux_path = true;

    ret = symbol__init(perf_session__env(annotate.session));
    if ret < 0 {
        annotation_options__exit();
        return ret;
    }

    if annotate.use_stdio || annotate.use_stdio2 {
        use_browser = 0;
    /* HAVE_SLANG_SUPPORT */
    } else if annotate.use_tui {
        use_browser = 1;
    /* HAVE_GTK2_SUPPORT */
    } else if annotate.use_gtk {
        use_browser = 2;
    }

    if annotate.data_type {
        annotate_opts.annotate_src = false;
        symbol_conf.annotate_data_member = true;
        symbol_conf.annotate_data_sample = true;
    } else if annotate_opts.code_with_type {
        symbol_conf.annotate_data_member = true;
    }

    setup_browser(true);

    /*
     * Events of different processes may correspond to the same
     * symbol, we do not care about the processes in annotate,
     * set sort order to avoid repeated output.
     */
    if annotate.data_type {
        sort_order = c"dso,type".as_ptr();
    } else {
        sort_order = c"dso,symbol".as_ptr();
    }

    /*
     * Set SORT_MODE__BRANCH so that annotate displays IPC/Cycle and
     * branch counters, if the corresponding branch info is available
     * in the perf data in the TUI mode.
     */
    if (use_browser == 1 || annotate.use_stdio2) && annotate.has_br_stack {
        sort__mode = SORT_MODE__BRANCH;
        if evlist__nr_br_cntr((*annotate.session).evlist) > 0 {
            annotate_opts.show_br_cntr = true;
        }
    }

    if setup_sorting(ptr::null_mut(), perf_session__env(annotate.session)) < 0 {
        usage_with_options(annotate_usage.as_ptr(), options.as_mut_ptr());
    }

    ret = __cmd_annotate(&mut annotate);

    /*
     * Speed up the exit process by only deleting for debug builds. For
     * large files this can save time.
     */
    if cfg!(debug_assertions) {
        perf_session__delete(annotate.session);
    }
    annotation_options__exit();

    ret
}
