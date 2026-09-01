// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/builtin-mem.c.
// C include dependencies are intentionally represented as external items below.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const MEM_OPERATION_LOAD: c_int = 0x1;
const MEM_OPERATION_STORE: c_int = 0x2;

const PERF_MEM_EVENTS__LOAD_STORE: usize = 0;
const PERF_MEM_EVENTS__LOAD: usize = 1;
const PERF_MEM_EVENTS__STORE: usize = 2;
const PERF_DATA_MODE_READ: c_int = 0;
const PARSE_OPT_KEEP_UNKNOWN: c_int = 0;
const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 0;
const SORT_MODE__MEMORY: c_int = 0;
const STDIN_FILENO: c_int = 0;
const PAGE_SIZE_NAME_LEN: usize = 32;
const MAX_NR_CPUS: usize = 4096;
const BITS_PER_LONG: usize = size_of::<c_ulong>() * 8;
const CPU_BITMAP_LONGS: usize = (MAX_NR_CPUS + BITS_PER_LONG - 1) / BITS_PER_LONG;

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
    pub lost: Option<unsafe extern "C" fn() -> c_int>,
    pub fork: Option<unsafe extern "C" fn() -> c_int>,
    pub attr: Option<unsafe extern "C" fn() -> c_int>,
    pub build_id: Option<unsafe extern "C" fn() -> c_int>,
    pub namespaces: Option<unsafe extern "C" fn() -> c_int>,
    pub auxtrace_info: Option<unsafe extern "C" fn() -> c_int>,
    pub auxtrace: Option<unsafe extern "C" fn() -> c_int>,
    pub auxtrace_error: Option<unsafe extern "C" fn() -> c_int>,
}

#[repr(C)]
pub struct perf_mem {
    pub tool: perf_tool,
    pub input_name: *const c_char,
    pub sort_key: *const c_char,
    pub hide_unresolved: bool,
    pub dump_raw: bool,
    pub force: bool,
    pub phys_addr: bool,
    pub data_page_size: bool,
    pub all_kernel: bool,
    pub all_user: bool,
    pub data_type: bool,
    pub operation: c_int,
    pub cpu_list: *const c_char,
    pub cpu_bitmap: [c_ulong; CPU_BITMAP_LONGS],
}

#[repr(C)]
pub struct option {
    pub value: *mut c_void,
}

#[repr(C)]
pub struct perf_pmu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_mem_event {
    pub tag: *const c_char,
}

#[repr(C)]
pub struct perf_data {
    pub path: *const c_char,
    pub mode: c_int,
    pub force: bool,
}

#[repr(C)]
pub struct itrace_synth_opts {
    pub set: bool,
    pub mem: bool,
    pub default_no_sample: bool,
}

#[repr(C)]
pub struct perf_session {
    pub itrace_synth_opts: *mut itrace_synth_opts,
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: c_uint,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
}

#[repr(C)]
pub struct perf_sample {
    pub pid: c_int,
    pub tid: c_int,
    pub ip: u64,
    pub addr: u64,
    pub phys_addr: u64,
    pub data_page_size: u64,
    pub weight: u64,
    pub data_src: u64,
}

#[repr(C)]
pub struct machine {
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
pub struct addr_location {
    pub filtered: bool,
    pub sym: *mut symbol,
    pub map: *mut map,
}

#[repr(C)]
pub struct stat {
    pub st_mode: c_uint,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub field_sep: *const c_char,
}

#[repr(C)]
struct mem_mode {
    name: *const c_char,
    mode: c_int,
}

unsafe extern "C" {
    static mut input_name: *const c_char;
    static mut verbose: c_int;
    static mut symbol_conf: symbol_conf_t;
    static mut perf_mem_record: [bool; 3];
    static mut perf_mem_events__loads_ldlat: c_uint;

    fn perf_mem_events_find_pmu() -> *mut perf_pmu;
    fn perf_pmu__mem_events_list(pmu: *mut perf_pmu);
    fn perf_pmu__mem_events_parse(pmu: *mut perf_pmu, str_: *const c_char) -> c_int;
    fn perf_pmu__mem_events_init() -> c_int;
    fn perf_pmu__mem_events_num_mem_pmus(pmu: *mut perf_pmu) -> c_int;
    fn perf_pmu__mem_events_ptr(pmu: *mut perf_pmu, idx: usize) -> *mut perf_mem_event;
    fn perf_mem_events__record_args(
        rec_argv: *mut *const c_char,
        i: *mut c_int,
        event_name_storage: *mut *mut c_char,
    ) -> c_int;

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn parse_options_subcommand(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        subcommands: *const *const c_char,
        usagestr: *mut *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option) -> !;
    fn sort_help(prefix: *const c_char, mode: c_int) -> *mut c_char;

    fn cmd_record(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn cmd_report(argc: c_int, argv: *mut *const c_char) -> c_int;

    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool);
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn perf_session__delete(session: *mut perf_session);
    fn perf_session__cpu_bitmap(
        session: *mut perf_session,
        cpu_list: *const c_char,
        cpu_bitmap: *mut c_ulong,
    ) -> c_int;
    fn perf_session__env(session: *mut perf_session) -> *mut c_void;
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn symbol__init(env: *mut c_void) -> c_int;

    fn perf_event__process_mmap() -> c_int;
    fn perf_event__process_mmap2() -> c_int;
    fn perf_event__process_comm() -> c_int;
    fn perf_event__process_lost() -> c_int;
    fn perf_event__process_fork() -> c_int;
    fn perf_event__process_attr() -> c_int;
    fn perf_event__process_build_id() -> c_int;
    fn perf_event__process_namespaces() -> c_int;
    fn perf_event__process_auxtrace_info() -> c_int;
    fn perf_event__process_auxtrace() -> c_int;
    fn perf_event__process_auxtrace_error() -> c_int;

    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn machine__resolve(
        machine: *mut machine,
        al: *mut addr_location,
        sample: *mut perf_sample,
    ) -> c_int;
    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__set_hit(dso: *mut dso);
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn get_page_size_name(size: u64, str_: *mut c_char) -> *const c_char;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;

    // Option-construction macros from parse-options.h are dependency-provided.
    fn OPT_CALLBACK(
        short_name: c_char,
        long_name: *const c_char,
        value: *mut c_void,
        argh: *const c_char,
        help: *const c_char,
        callback: unsafe extern "C" fn(*const option, *const c_char, c_int) -> c_int,
    ) -> option;
    fn OPT_STRING(
        short_name: c_char,
        long_name: *const c_char,
        value: *mut *const c_char,
        argh: *const c_char,
        help: *const c_char,
    ) -> option;
    fn OPT_STRING_NOEMPTY(
        short_name: c_char,
        long_name: *const c_char,
        value: *mut *const c_char,
        argh: *const c_char,
        help: *const c_char,
    ) -> option;
    fn OPT_BOOLEAN(
        short_name: c_char,
        long_name: *const c_char,
        value: *mut bool,
        help: *const c_char,
    ) -> option;
    fn OPT_INCR(
        short_name: c_char,
        long_name: *const c_char,
        value: *mut c_int,
        help: *const c_char,
    ) -> option;
    fn OPT_UINTEGER(
        short_name: c_char,
        long_name: *const c_char,
        value: *mut c_uint,
        help: *const c_char,
    ) -> option;
    fn OPT_PARENT(parent: *const option) -> option;
    fn OPT_END() -> option;
}

unsafe fn is_err(ptr_: *mut perf_session) -> bool {
    (ptr_ as isize) < 0 && (ptr_ as isize) >= -4095
}

unsafe fn ptr_err<T>(ptr_: *mut T) -> c_int {
    ptr_ as isize as c_int
}

unsafe fn s_isfifo(mode: c_uint) -> bool {
    const S_IFMT: c_uint = 0o170000;
    const S_IFIFO: c_uint = 0o010000;
    (mode & S_IFMT) == S_IFIFO
}

unsafe fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool {
    strncmp(str_, prefix, strlen(prefix)) == 0
}

unsafe extern "C" fn parse_record_events(
    opt: *const option,
    str_: *const c_char,
    _unset: c_int,
) -> c_int {
    let mem = (*opt).value as *mut perf_mem;
    let pmu: *mut perf_pmu;

    pmu = perf_mem_events_find_pmu();
    if pmu.is_null() {
        pr_err(c"failed: there is no PMU that supports perf mem\n".as_ptr());
        exit(-1);
    }

    if strcmp(str_, c"list".as_ptr()) == 0 {
        perf_pmu__mem_events_list(pmu);
        exit(0);
    }
    if perf_pmu__mem_events_parse(pmu, str_) != 0 {
        exit(-1);
    }

    (*mem).operation = 0;
    0
}

unsafe fn __cmd_record(
    mut argc: c_int,
    argv: *const *const c_char,
    mem: *mut perf_mem,
    options: *const option,
) -> c_int {
    let mut rec_argc: c_int;
    let mut i: c_int = 0;
    let mut j: c_int;
    let start: c_int;
    let end: c_int;
    let rec_argv: *mut *const c_char;
    let mut event_name_storage: *mut c_char = ptr::null_mut();
    let mut ret: c_int;
    let e: *mut perf_mem_event;
    let pmu: *mut perf_pmu;
    let record_usage: [*const c_char; 3] = [
        c"perf mem record [<options>] [<command>]".as_ptr(),
        c"perf mem record [<options>] -- <command> [<options>]".as_ptr(),
        ptr::null(),
    ];

    pmu = perf_mem_events_find_pmu();
    if pmu.is_null() {
        pr_err(c"failed: no PMU supports the memory events\n".as_ptr());
        return -1;
    }

    if perf_pmu__mem_events_init() != 0 {
        pr_err(c"failed: memory events not supported\n".as_ptr());
        return -1;
    }

    argc = parse_options(
        argc,
        argv,
        options,
        record_usage.as_ptr(),
        PARSE_OPT_KEEP_UNKNOWN,
    );

    /* Max number of arguments multiplied by number of PMUs that can support them. */
    rec_argc = argc + 9 * (perf_pmu__mem_events_num_mem_pmus(pmu) + 1);

    if !(*mem).cpu_list.is_null() {
        rec_argc += 2;
    }

    rec_argv = calloc((rec_argc + 1) as usize, size_of::<*const c_char>()) as *mut *const c_char;
    if rec_argv.is_null() {
        return -1;
    }

    *rec_argv.add(i as usize) = c"record".as_ptr();
    i += 1;

    e = perf_pmu__mem_events_ptr(pmu, PERF_MEM_EVENTS__LOAD_STORE);

    /*
     * The load and store operations are required, use the event
     * PERF_MEM_EVENTS__LOAD_STORE if it is supported.
     */
    if !(*e).tag.is_null()
        && ((*mem).operation & MEM_OPERATION_LOAD) != 0
        && ((*mem).operation & MEM_OPERATION_STORE) != 0
    {
        perf_mem_record[PERF_MEM_EVENTS__LOAD_STORE] = true;
        *rec_argv.add(i as usize) = c"-W".as_ptr();
        i += 1;
    } else {
        if ((*mem).operation & MEM_OPERATION_LOAD) != 0 {
            perf_mem_record[PERF_MEM_EVENTS__LOAD] = true;
        }

        if ((*mem).operation & MEM_OPERATION_STORE) != 0 {
            perf_mem_record[PERF_MEM_EVENTS__STORE] = true;
        }
    }

    if perf_mem_record[PERF_MEM_EVENTS__LOAD] {
        *rec_argv.add(i as usize) = c"-W".as_ptr();
        i += 1;
    }

    *rec_argv.add(i as usize) = c"-d".as_ptr();
    i += 1;

    if (*mem).phys_addr {
        *rec_argv.add(i as usize) = c"--phys-data".as_ptr();
        i += 1;
    }

    if (*mem).data_page_size {
        *rec_argv.add(i as usize) = c"--data-page-size".as_ptr();
        i += 1;
    }

    start = i;
    ret = perf_mem_events__record_args(rec_argv, &mut i, &mut event_name_storage);
    if ret != 0 {
        free(event_name_storage as *mut c_void);
        free(rec_argv as *mut c_void);
        return ret;
    }
    end = i;

    if (*mem).all_user {
        *rec_argv.add(i as usize) = c"--all-user".as_ptr();
        i += 1;
    }

    if (*mem).all_kernel {
        *rec_argv.add(i as usize) = c"--all-kernel".as_ptr();
        i += 1;
    }

    if !(*mem).cpu_list.is_null() {
        *rec_argv.add(i as usize) = c"-C".as_ptr();
        i += 1;
        *rec_argv.add(i as usize) = (*mem).cpu_list;
        i += 1;
    }

    j = 0;
    while j < argc {
        *rec_argv.add(i as usize) = *argv.add(j as usize);
        j += 1;
        i += 1;
    }

    if verbose > 0 {
        pr_debug(c"calling: record ".as_ptr());

        j = start;
        while j < end {
            pr_debug(c"%s ".as_ptr(), *rec_argv.add(j as usize));
            j += 1;
        }

        pr_debug(c"\n".as_ptr());
    }

    ret = cmd_record(i, rec_argv);
    free(event_name_storage as *mut c_void);
    free(rec_argv as *mut c_void);
    ret
}

unsafe extern "C" fn dump_raw_samples(
    tool: *const perf_tool,
    event: *mut perf_event,
    sample: *mut perf_sample,
    machine: *mut machine,
) -> c_int {
    let mem = tool as *mut perf_mem;
    let mut al: addr_location = core::mem::zeroed();
    let mut fmt: *const c_char;
    let field_sep: *const c_char;
    let mut str_: [c_char; PAGE_SIZE_NAME_LEN] = [0; PAGE_SIZE_NAME_LEN];
    let mut dso: *mut dso = ptr::null_mut();

    addr_location__init(&mut al);
    if machine__resolve(machine, &mut al, sample) < 0 {
        fprintf(
            stderr,
            c"problem processing %d event, skipping it.\n".as_ptr(),
            (*event).header.type_,
        );
        addr_location__exit(&mut al);
        return -1;
    }

    if al.filtered || ((*mem).hide_unresolved && al.sym.is_null()) {
        addr_location__exit(&mut al);
        return 0;
    }

    if !al.map.is_null() {
        dso = map__dso(al.map);
        if !dso.is_null() {
            dso__set_hit(dso);
        }
    }

    field_sep = symbol_conf.field_sep;
    if !field_sep.is_null() {
        fmt = c"%d%s%d%s0x%llx%s0x%llx%s".as_ptr();
    } else {
        fmt = c"%5d%s%5d%s0x%016llx%s0x016%llx%s".as_ptr();
        symbol_conf.field_sep = c" ".as_ptr();
    }
    printf(
        fmt,
        (*sample).pid,
        symbol_conf.field_sep,
        (*sample).tid,
        symbol_conf.field_sep,
        (*sample).ip,
        symbol_conf.field_sep,
        (*sample).addr,
        symbol_conf.field_sep,
    );

    if (*mem).phys_addr {
        printf(
            c"0x%016llx%s".as_ptr(),
            (*sample).phys_addr,
            symbol_conf.field_sep,
        );
    }

    if (*mem).data_page_size {
        printf(
            c"%s%s".as_ptr(),
            get_page_size_name((*sample).data_page_size, str_.as_mut_ptr()),
            symbol_conf.field_sep,
        );
    }

    if !field_sep.is_null() {
        fmt = c"%llu%s0x%llx%s%s:%s\n".as_ptr();
    } else {
        fmt = c"%5llu%s0x%06llx%s%s:%s\n".as_ptr();
    }

    printf(
        fmt,
        (*sample).weight,
        symbol_conf.field_sep,
        (*sample).data_src,
        symbol_conf.field_sep,
        if !dso.is_null() {
            dso__long_name(dso)
        } else {
            c"???".as_ptr()
        },
        if !al.sym.is_null() {
            (*al.sym).name
        } else {
            c"???".as_ptr()
        },
    );

    addr_location__exit(&mut al);
    0
}

unsafe extern "C" fn process_sample_event(
    tool: *const perf_tool,
    event: *mut perf_event,
    sample: *mut perf_sample,
    machine: *mut machine,
) -> c_int {
    dump_raw_samples(tool, event, sample, machine)
}

unsafe fn report_raw_events(mem: *mut perf_mem) -> c_int {
    let mut itrace_synth_opts = itrace_synth_opts {
        set: true,
        mem: true, /* Only enable memory event */
        default_no_sample: true,
    };

    let mut data = perf_data {
        path: input_name,
        mode: PERF_DATA_MODE_READ,
        force: (*mem).force,
    };
    let mut ret: c_int;
    let session: *mut perf_session;

    perf_tool__init(&mut (*mem).tool, true);
    (*mem).tool.sample = Some(process_sample_event);
    (*mem).tool.mmap = Some(perf_event__process_mmap);
    (*mem).tool.mmap2 = Some(perf_event__process_mmap2);
    (*mem).tool.comm = Some(perf_event__process_comm);
    (*mem).tool.lost = Some(perf_event__process_lost);
    (*mem).tool.fork = Some(perf_event__process_fork);
    (*mem).tool.attr = Some(perf_event__process_attr);
    (*mem).tool.build_id = Some(perf_event__process_build_id);
    (*mem).tool.namespaces = Some(perf_event__process_namespaces);
    (*mem).tool.auxtrace_info = Some(perf_event__process_auxtrace_info);
    (*mem).tool.auxtrace = Some(perf_event__process_auxtrace);
    (*mem).tool.auxtrace_error = Some(perf_event__process_auxtrace_error);

    session = perf_session__new(&mut data, &mut (*mem).tool);

    if is_err(session) {
        return ptr_err(session);
    }

    (*session).itrace_synth_opts = &mut itrace_synth_opts;

    if !(*mem).cpu_list.is_null() {
        ret = perf_session__cpu_bitmap(session, (*mem).cpu_list, (*mem).cpu_bitmap.as_mut_ptr());
        if ret < 0 {
            perf_session__delete(session);
            return ret;
        }
    }

    ret = symbol__init(perf_session__env(session));
    if ret < 0 {
        perf_session__delete(session);
        return ret;
    }

    printf(c"# PID, TID, IP, ADDR, ".as_ptr());

    if (*mem).phys_addr {
        printf(c"PHYS ADDR, ".as_ptr());
    }

    if (*mem).data_page_size {
        printf(c"DATA PAGE SIZE, ".as_ptr());
    }

    printf(c"LOCAL WEIGHT, DSRC, SYMBOL\n".as_ptr());

    ret = perf_session__process_events(session);

    perf_session__delete(session);
    ret
}

unsafe fn get_sort_order(mem: *mut perf_mem) -> *mut c_char {
    let has_extra_options: bool = ((*mem).phys_addr | (*mem).data_page_size) != false;
    let mut sort: [c_char; 128] = [0; 128];

    if !(*mem).sort_key.is_null() {
        scnprintf(
            sort.as_mut_ptr(),
            size_of::<[c_char; 128]>(),
            c"--sort=%s".as_ptr(),
            (*mem).sort_key,
        );
    } else if (*mem).data_type {
        strcpy(sort.as_mut_ptr(), c"--sort=mem,snoop,tlb,type".as_ptr());
    /*
     * there is no weight (cost) associated with stores, so don't print
     * the column
     */
    } else if ((*mem).operation & MEM_OPERATION_LOAD) == 0 {
        strcpy(
            sort.as_mut_ptr(),
            c"--sort=mem,sym,dso,symbol_daddr,dso_daddr,tlb,locked".as_ptr(),
        );
    } else if has_extra_options {
        strcpy(
            sort.as_mut_ptr(),
            c"--sort=local_weight,mem,sym,dso,symbol_daddr,dso_daddr,snoop,tlb,locked,blocked"
                .as_ptr(),
        );
    } else {
        return ptr::null_mut();
    }

    if (*mem).phys_addr {
        strcat(sort.as_mut_ptr(), c",phys_daddr".as_ptr());
    }

    if (*mem).data_page_size {
        strcat(sort.as_mut_ptr(), c",data_page_size".as_ptr());
    }

    /* make sure it has 'type' sort key even -s option is used */
    if (*mem).data_type && strstr(sort.as_ptr(), c"type".as_ptr()).is_null() {
        strcat(sort.as_mut_ptr(), c",type".as_ptr());
    }

    strdup(sort.as_ptr())
}

unsafe fn __cmd_report(
    mut argc: c_int,
    argv: *const *const c_char,
    mem: *mut perf_mem,
    options: *const option,
) -> c_int {
    let rep_argv: *mut *const c_char;
    let mut ret: c_int;
    let mut i: c_int = 0;
    let mut j: c_int;
    let rep_argc: c_int;
    let new_sort_order: *mut c_char;
    let report_usage: [*const c_char; 2] = [c"perf mem report [<options>]".as_ptr(), ptr::null()];

    argc = parse_options(
        argc,
        argv,
        options,
        report_usage.as_ptr(),
        PARSE_OPT_KEEP_UNKNOWN,
    );

    if (*mem).dump_raw {
        return report_raw_events(mem);
    }

    rep_argc = argc + 3;
    rep_argv = calloc((rep_argc + 1) as usize, size_of::<*const c_char>()) as *mut *const c_char;
    if rep_argv.is_null() {
        return -1;
    }

    *rep_argv.add(i as usize) = c"report".as_ptr();
    i += 1;
    *rep_argv.add(i as usize) = c"--mem-mode".as_ptr();
    i += 1;
    *rep_argv.add(i as usize) = c"-n".as_ptr(); /* display number of samples */
    i += 1;

    new_sort_order = get_sort_order(mem);
    if !new_sort_order.is_null() {
        *rep_argv.add(i as usize) = new_sort_order as *const c_char;
        i += 1;
    }

    j = 0;
    while j < argc {
        *rep_argv.add(i as usize) = *argv.add(j as usize);
        j += 1;
        i += 1;
    }

    ret = cmd_report(i, rep_argv);
    free(new_sort_order as *mut c_void);
    free(rep_argv as *mut c_void);
    ret
}

static MEM_MODES: [mem_mode; 3] = [
    mem_mode {
        name: c"load".as_ptr(),
        mode: MEM_OPERATION_LOAD,
    },
    mem_mode {
        name: c"store".as_ptr(),
        mode: MEM_OPERATION_STORE,
    },
    mem_mode {
        name: ptr::null(),
        mode: 0,
    },
];

unsafe extern "C" fn parse_mem_ops(opt: *const option, str_: *const c_char, unset: c_int) -> c_int {
    let mode = (*opt).value as *mut c_int;
    let mut m: *const mem_mode;
    let mut s: *mut c_char;
    let mut os: *mut c_char = ptr::null_mut();
    let mut p: *mut c_char;
    let mut ret: c_int = -1;

    if unset != 0 {
        return 0;
    }

    /* str may be NULL in case no arg is passed to -t */
    if !str_.is_null() {
        /* because str is read-only */
        os = strdup(str_);
        s = os;
        if s.is_null() {
            return -1;
        }

        /* reset mode */
        *mode = 0;

        loop {
            p = strchr(s, ',' as c_int);
            if !p.is_null() {
                *p = 0;
            }

            m = MEM_MODES.as_ptr();
            while !(*m).name.is_null() {
                if strcasecmp(s, (*m).name) == 0 {
                    break;
                }
                m = m.add(1);
            }
            if (*m).name.is_null() {
                fprintf(
                    stderr,
                    c"unknown sampling op %s, check man page\n".as_ptr(),
                    s,
                );
                free(os as *mut c_void);
                return ret;
            }

            *mode |= (*m).mode;

            if p.is_null() {
                break;
            }

            s = p.add(1);
        }
    }
    ret = 0;

    if *mode == 0 {
        *mode = MEM_OPERATION_LOAD;
    }
    free(os as *mut c_void);
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_mem(mut argc: c_int, argv: *const *const c_char) -> c_int {
    let mut st: stat = core::mem::zeroed();
    let mut mem: perf_mem = core::mem::zeroed();
    mem.input_name = c"perf.data".as_ptr();
    /*
     * default to both load an store sampling
     */
    mem.operation = MEM_OPERATION_LOAD | MEM_OPERATION_STORE;

    let sort_order_help = sort_help(c"sort by key(s):".as_ptr(), SORT_MODE__MEMORY);

    let mem_options: [option; 8] = [
        OPT_CALLBACK(
            't' as c_char,
            c"type".as_ptr(),
            &mut mem.operation as *mut c_int as *mut c_void,
            c"type".as_ptr(),
            c"memory operations(load,store) Default load,store".as_ptr(),
            parse_mem_ops,
        ),
        OPT_STRING(
            'C' as c_char,
            c"cpu".as_ptr(),
            &mut mem.cpu_list,
            c"cpu".as_ptr(),
            c"list of cpus to profile".as_ptr(),
        ),
        OPT_BOOLEAN(
            'f' as c_char,
            c"force".as_ptr(),
            &mut mem.force,
            c"don't complain, do it".as_ptr(),
        ),
        OPT_INCR(
            'v' as c_char,
            c"verbose".as_ptr(),
            &mut verbose,
            c"be more verbose (show counter open errors, etc)".as_ptr(),
        ),
        OPT_BOOLEAN(
            'p' as c_char,
            c"phys-data".as_ptr(),
            &mut mem.phys_addr,
            c"Record/Report sample physical addresses".as_ptr(),
        ),
        OPT_BOOLEAN(
            0,
            c"data-page-size".as_ptr(),
            &mut mem.data_page_size,
            c"Record/Report sample data address page size".as_ptr(),
        ),
        OPT_END(),
        OPT_END(),
    ];
    let record_options: [option; 6] = [
        OPT_CALLBACK(
            'e' as c_char,
            c"event".as_ptr(),
            &mut mem as *mut perf_mem as *mut c_void,
            c"event".as_ptr(),
            c"event selector. use 'perf mem record -e list' to list available events".as_ptr(),
            parse_record_events,
        ),
        OPT_UINTEGER(
            0,
            c"ldlat".as_ptr(),
            &mut perf_mem_events__loads_ldlat,
            c"mem-loads latency".as_ptr(),
        ),
        OPT_BOOLEAN(
            'U' as c_char,
            c"all-user".as_ptr(),
            &mut mem.all_user,
            c"collect only user level data".as_ptr(),
        ),
        OPT_BOOLEAN(
            'K' as c_char,
            c"all-kernel".as_ptr(),
            &mut mem.all_kernel,
            c"collect only kernel level data".as_ptr(),
        ),
        OPT_PARENT(mem_options.as_ptr()),
        OPT_END(),
    ];
    let report_options: [option; 8] = [
        OPT_BOOLEAN(
            'D' as c_char,
            c"dump-raw-samples".as_ptr(),
            &mut mem.dump_raw,
            c"dump raw samples in ASCII".as_ptr(),
        ),
        OPT_BOOLEAN(
            'U' as c_char,
            c"hide-unresolved".as_ptr(),
            &mut mem.hide_unresolved,
            c"Only display entries resolved to a symbol".as_ptr(),
        ),
        OPT_STRING(
            'i' as c_char,
            c"input".as_ptr(),
            &mut input_name,
            c"file".as_ptr(),
            c"input file name".as_ptr(),
        ),
        OPT_STRING_NOEMPTY(
            'x' as c_char,
            c"field-separator".as_ptr(),
            &mut symbol_conf.field_sep,
            c"separator".as_ptr(),
            c"separator for columns, no spaces will be added between columns '.' is reserved.".as_ptr(),
        ),
        OPT_STRING(
            's' as c_char,
            c"sort".as_ptr(),
            &mut mem.sort_key,
            c"key[,key2...]".as_ptr(),
            sort_order_help,
        ),
        OPT_BOOLEAN(
            'T' as c_char,
            c"type-profile".as_ptr(),
            &mut mem.data_type,
            c"Show data-type profile result".as_ptr(),
        ),
        OPT_PARENT(mem_options.as_ptr()),
        OPT_END(),
    ];
    let mem_subcommands: [*const c_char; 3] =
        [c"record".as_ptr(), c"report".as_ptr(), ptr::null()];
    let mut mem_usage: [*const c_char; 2] = [ptr::null(), ptr::null()];
    let ret: c_int;

    argc = parse_options_subcommand(
        argc,
        argv,
        mem_options.as_ptr(),
        mem_subcommands.as_ptr(),
        mem_usage.as_mut_ptr(),
        PARSE_OPT_STOP_AT_NON_OPTION,
    );

    if argc == 0 || (strncmp(*argv, c"rec".as_ptr(), 3) == 0 && mem.operation == 0) {
        usage_with_options(mem_usage.as_ptr(), mem_options.as_ptr());
    }

    if mem.input_name.is_null() || strlen(mem.input_name) == 0 {
        if fstat(STDIN_FILENO, &mut st) == 0 && s_isfifo(st.st_mode) {
            mem.input_name = c"-".as_ptr();
        } else {
            mem.input_name = c"perf.data".as_ptr();
        }
    }

    if strlen(*argv) > 2 && strstarts(c"record".as_ptr(), *argv) {
        ret = __cmd_record(argc, argv, &mut mem, record_options.as_ptr());
    } else if strlen(*argv) > 2 && strstarts(c"report".as_ptr(), *argv) {
        ret = __cmd_report(argc, argv, &mut mem, report_options.as_ptr());
    } else {
        usage_with_options(mem_usage.as_ptr(), mem_options.as_ptr());
    }

    /* free usage string allocated by parse_options_subcommand */
    free(mem_usage[0] as *mut c_void);
    free(sort_order_help as *mut c_void);

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
