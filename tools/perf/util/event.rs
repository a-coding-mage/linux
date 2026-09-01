// Translated from perf/util/event.c.
// C include dependencies are expected to be provided by the surrounding crate.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

unsafe extern "C" {
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn kallsyms__is_function(type_: c_char) -> bool;
    fn kallsyms__parse(
        filename: *const c_char,
        arg: *mut c_void,
        cb: Option<unsafe extern "C" fn(*mut c_void, *const c_char, c_char, u64) -> c_int>,
    ) -> c_int;

    fn pr_warning(format: *const c_char, ...);
    fn perf_ns__name(idx: u32) -> *const c_char;
    fn machine__process_comm_event(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn machine__process_namespaces_event(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn machine__process_cgroup_event(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn machine__process_lost_event(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn machine__process_aux_event(machine: *mut machine, event: *mut perf_event) -> c_int;
    fn machine__process_itrace_start_event(machine: *mut machine, event: *mut perf_event) -> c_int;
    fn machine__process_aux_output_hw_id_event(machine: *mut machine, event: *mut perf_event) -> c_int;
    fn machine__process_lost_samples_event(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn machine__process_switch_event(machine: *mut machine, event: *mut perf_event) -> c_int;
    fn machine__process_ksymbol(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn machine__process_bpf(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn machine__process_text_poke(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn machine__process_mmap_event(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn machine__process_mmap2_event(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn machine__process_fork_event(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn machine__process_exit_event(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn machine__process_event(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample) -> c_int;

    fn build_id__init(bid: *mut build_id, data: *const u8, size: u8);
    fn build_id__snprintf(bid: *const build_id, s: *mut c_char, size: usize) -> c_int;
    fn thread_map__new_event(event: *mut perf_record_thread_map) -> *mut perf_thread_map;
    fn thread_map__fprintf(threads: *mut perf_thread_map, fp: *mut FILE) -> usize;
    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn cpu_map__new_data(data: *mut perf_record_cpu_map_data) -> *mut perf_cpu_map;
    fn cpu_map__fprintf(cpus: *mut perf_cpu_map, fp: *mut FILE) -> usize;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn machine__findnew_thread(machine: *mut machine, pid: i32, tid: i32) -> *mut thread;
    fn machine__findnew_guest_code(machine: *mut machine, pid: i32) -> *mut thread;
    fn machine__remove_thread(machine: *mut machine, thread: *mut thread);
    fn thread__put(thread: *mut thread);
    fn dump_printf(format: *const c_char, ...);
    fn binary__fprintf(
        data: *const u8,
        len: u32,
        bytes_per_line: u32,
        printer: Option<unsafe extern "C" fn(binary_printer_ops, c_uint, *mut c_void, *mut FILE) -> c_int>,
        extra: *mut c_void,
        fp: *mut FILE,
    ) -> usize;
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn machine__kernel_maps(machine: *mut machine) -> *mut maps;
    fn maps__find(maps: *mut maps, addr: u64) -> *mut map;
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn map__load(map: *mut map) -> c_int;
    fn map__map_ip(map: *mut map, addr: u64) -> u64;
    fn map__unmap_ip(map: *mut map, addr: u64) -> u64;
    fn map__find_symbol(map: *mut map, addr: u64) -> *mut symbol;
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__zput(map: *mut *mut map);
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn thread__zput(thread: *mut *mut thread);
    fn machine__addr_cpumode(machine: *mut machine, cpumode: u8, addr: u64) -> u8;
    fn symbol__fprintf_symname_offs(sym: *mut symbol, al: *mut addr_location, fp: *mut FILE) -> usize;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn thread__tid(thread: *mut thread) -> i32;
    fn thread__is_filtered(thread: *mut thread) -> bool;
    fn machine__is_host(machine: *mut machine) -> bool;
    fn machine__nr_cpus_avail(machine: *mut machine) -> i32;
    fn perf_env__get_cpu_topology(env: *mut perf_env, cpu: perf_cpu) -> *mut cpu_topology_map;
    fn test_bit(nr: c_int, addr: *const c_ulong) -> bool;
    fn strlist__has_entry(list: *mut strlist, entry: *const c_char) -> c_int;
    fn intlist__has_entry(list: *mut intlist, entry: c_ulong) -> c_int;
}

const fn c(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

// Opaque dependency types supplied by the surrounding perf crate/bindings.
pub enum perf_tool {}
pub enum machine {}
pub enum thread {}
pub enum maps {}
pub enum map {}
pub enum dso {}
pub enum symbol {}
pub enum perf_thread_map {}
pub enum perf_cpu_map {}
pub enum strlist {}
pub enum intlist {}
pub enum int_node {}
pub enum build_id {}
pub enum perf_env {}
pub enum cpu_topology_map {}
pub enum perf_record_thread_map {}
pub enum perf_record_cpu_map_data {}
pub enum binary_printer_ops {}

#[allow(non_camel_case_types)]
type u64 = u64;

#[repr(C)]
pub struct process_symbol_args {
    pub name: *const c_char,
    pub start: u64,
}

// The following structs/unions/constants are external C layout dependencies from perf headers.
use crate::*;

#[no_mangle]
pub unsafe extern "C" fn perf_event__name(id: c_uint) -> *const c_char {
    match id {
        0 => c(b"TOTAL\0"),
        PERF_RECORD_MMAP => c(b"MMAP\0"),
        PERF_RECORD_MMAP2 => c(b"MMAP2\0"),
        PERF_RECORD_LOST => c(b"LOST\0"),
        PERF_RECORD_COMM => c(b"COMM\0"),
        PERF_RECORD_EXIT => c(b"EXIT\0"),
        PERF_RECORD_THROTTLE => c(b"THROTTLE\0"),
        PERF_RECORD_UNTHROTTLE => c(b"UNTHROTTLE\0"),
        PERF_RECORD_FORK => c(b"FORK\0"),
        PERF_RECORD_READ => c(b"READ\0"),
        PERF_RECORD_SAMPLE => c(b"SAMPLE\0"),
        PERF_RECORD_AUX => c(b"AUX\0"),
        PERF_RECORD_ITRACE_START => c(b"ITRACE_START\0"),
        PERF_RECORD_LOST_SAMPLES => c(b"LOST_SAMPLES\0"),
        PERF_RECORD_SWITCH => c(b"SWITCH\0"),
        PERF_RECORD_SWITCH_CPU_WIDE => c(b"SWITCH_CPU_WIDE\0"),
        PERF_RECORD_NAMESPACES => c(b"NAMESPACES\0"),
        PERF_RECORD_KSYMBOL => c(b"KSYMBOL\0"),
        PERF_RECORD_BPF_EVENT => c(b"BPF_EVENT\0"),
        PERF_RECORD_CGROUP => c(b"CGROUP\0"),
        PERF_RECORD_TEXT_POKE => c(b"TEXT_POKE\0"),
        PERF_RECORD_AUX_OUTPUT_HW_ID => c(b"AUX_OUTPUT_HW_ID\0"),
        PERF_RECORD_CALLCHAIN_DEFERRED => c(b"CALLCHAIN_DEFERRED\0"),
        PERF_RECORD_HEADER_ATTR => c(b"ATTR\0"),
        PERF_RECORD_HEADER_EVENT_TYPE => c(b"EVENT_TYPE\0"),
        PERF_RECORD_HEADER_TRACING_DATA => c(b"TRACING_DATA\0"),
        PERF_RECORD_HEADER_BUILD_ID => c(b"BUILD_ID\0"),
        PERF_RECORD_FINISHED_ROUND => c(b"FINISHED_ROUND\0"),
        PERF_RECORD_ID_INDEX => c(b"ID_INDEX\0"),
        PERF_RECORD_AUXTRACE_INFO => c(b"AUXTRACE_INFO\0"),
        PERF_RECORD_AUXTRACE => c(b"AUXTRACE\0"),
        PERF_RECORD_AUXTRACE_ERROR => c(b"AUXTRACE_ERROR\0"),
        PERF_RECORD_THREAD_MAP => c(b"THREAD_MAP\0"),
        PERF_RECORD_CPU_MAP => c(b"CPU_MAP\0"),
        PERF_RECORD_STAT_CONFIG => c(b"STAT_CONFIG\0"),
        PERF_RECORD_STAT => c(b"STAT\0"),
        PERF_RECORD_STAT_ROUND => c(b"STAT_ROUND\0"),
        PERF_RECORD_EVENT_UPDATE => c(b"EVENT_UPDATE\0"),
        PERF_RECORD_TIME_CONV => c(b"TIME_CONV\0"),
        PERF_RECORD_HEADER_FEATURE => c(b"FEATURE\0"),
        PERF_RECORD_COMPRESSED => c(b"COMPRESSED\0"),
        PERF_RECORD_FINISHED_INIT => c(b"FINISHED_INIT\0"),
        PERF_RECORD_COMPRESSED2 => c(b"COMPRESSED2\0"),
        PERF_RECORD_BPF_METADATA => c(b"BPF_METADATA\0"),
        PERF_RECORD_SCHEDSTAT_CPU => c(b"SCHEDSTAT_CPU\0"),
        PERF_RECORD_SCHEDSTAT_DOMAIN => c(b"SCHEDSTAT_DOMAIN\0"),
        _ => c(b"INVALID\0"),
    }
}

unsafe extern "C" fn find_func_symbol_cb(arg: *mut c_void, name: *const c_char, type_: c_char, start: u64) -> c_int {
    let args = arg as *mut process_symbol_args;

    /*
     * Must be a function or at least an alias, as in PARISC64, where "_text" is
     * an 'A' to the same address as "_stext".
     */
    if (!(kallsyms__is_function(type_) || type_ == b'A' as c_char)) || strcmp(name, (*args).name) != 0 {
        return 0;
    }

    (*args).start = start;
    1
}

unsafe extern "C" fn find_any_symbol_cb(arg: *mut c_void, name: *const c_char, _type: c_char, start: u64) -> c_int {
    let args = arg as *mut process_symbol_args;

    if strcmp(name, (*args).name) != 0 {
        return 0;
    }

    (*args).start = start;
    1
}

#[no_mangle]
pub unsafe extern "C" fn kallsyms__get_function_start(kallsyms_filename: *const c_char, symbol_name: *const c_char, addr: *mut u64) -> c_int {
    let mut args = process_symbol_args { name: symbol_name, start: 0 };

    if kallsyms__parse(kallsyms_filename, &mut args as *mut _ as *mut c_void, Some(find_func_symbol_cb)) <= 0 {
        return -1;
    }

    *addr = args.start;
    0
}

#[no_mangle]
pub unsafe extern "C" fn kallsyms__get_symbol_start(kallsyms_filename: *const c_char, symbol_name: *const c_char, addr: *mut u64) -> c_int {
    let mut args = process_symbol_args { name: symbol_name, start: 0 };

    if kallsyms__parse(kallsyms_filename, &mut args as *mut _ as *mut c_void, Some(find_any_symbol_cb)) <= 0 {
        return -1;
    }

    *addr = args.start;
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__read_stat_config(config: *mut perf_stat_config, event: *mut perf_record_stat_config) {
    let mut i: c_uint = 0;

    while i < (*event).nr {
        match (*event).data[i as usize].tag {
            PERF_STAT_CONFIG_TERM__AGGR_MODE => (*config).aggr_mode = (*event).data[i as usize].val,
            PERF_STAT_CONFIG_TERM__SCALE => (*config).scale = (*event).data[i as usize].val,
            PERF_STAT_CONFIG_TERM__INTERVAL => (*config).interval = (*event).data[i as usize].val,
            PERF_STAT_CONFIG_TERM__AGGR_LEVEL => (*config).aggr_level = (*event).data[i as usize].val,
            _ => pr_warning(c(b"unknown stat config term %lu\n\0"), (*event).data[i as usize].tag),
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_comm(event: *mut perf_event, fp: *mut FILE) -> usize {
    let s = if (*event).header.misc & PERF_RECORD_MISC_COMM_EXEC != 0 { c(b" exec\0") } else { c(b"\0") };
    fprintf(fp, c(b"%s: %s:%d/%d\n\0"), s, (*event).comm.comm.as_ptr(), (*event).comm.pid, (*event).comm.tid) as usize
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_namespaces(event: *mut perf_event, fp: *mut FILE) -> usize {
    let ns_link_info = (*event).namespaces.link_info.as_mut_ptr();
    let nr_namespaces = (*event).namespaces.nr_namespaces;
    let mut ret = fprintf(fp, c(b" %d/%d - nr_namespaces: %u\n\t\t[\0"), (*event).namespaces.pid, (*event).namespaces.tid, nr_namespaces) as usize;
    let mut idx: u32 = 0;

    while idx < nr_namespaces {
        if idx != 0 && idx % 4 == 0 {
            ret += fprintf(fp, c(b"\n\t\t \0")) as usize;
        }

        ret += fprintf(
            fp,
            c(b"%u/%s: %lu/%#lx%s\0"),
            idx,
            perf_ns__name(idx),
            (*ns_link_info.add(idx as usize)).dev as u64,
            (*ns_link_info.add(idx as usize)).ino as u64,
            if idx + 1 != nr_namespaces { c(b", \0") } else { c(b"]\n\0") },
        ) as usize;
        idx += 1;
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_cgroup(event: *mut perf_event, fp: *mut FILE) -> usize {
    fprintf(fp, c(b" cgroup: %lu %s\n\0"), (*event).cgroup.id, (*event).cgroup.path.as_ptr()) as usize
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_comm(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_comm_event(machine, event, sample)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_namespaces(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_namespaces_event(machine, event, sample)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_cgroup(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_cgroup_event(machine, event, sample)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_lost(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_lost_event(machine, event, sample)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_aux(_tool: *const perf_tool, event: *mut perf_event, _sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_aux_event(machine, event)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_itrace_start(_tool: *const perf_tool, event: *mut perf_event, _sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_itrace_start_event(machine, event)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_aux_output_hw_id(_tool: *const perf_tool, event: *mut perf_event, _sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_aux_output_hw_id_event(machine, event)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_lost_samples(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_lost_samples_event(machine, event, sample)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_switch(_tool: *const perf_tool, event: *mut perf_event, _sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_switch_event(machine, event)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_ksymbol(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_ksymbol(machine, event, sample)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_bpf(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_bpf(machine, event, sample)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_text_poke(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_text_poke(machine, event, sample)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_mmap(event: *mut perf_event, fp: *mut FILE) -> usize {
    fprintf(
        fp,
        c(b" %d/%d: [%#lx(%#lx) @ %#lx]: %c %s\n\0"),
        (*event).mmap.pid,
        (*event).mmap.tid,
        (*event).mmap.start,
        (*event).mmap.len,
        (*event).mmap.pgoff,
        if (*event).header.misc & PERF_RECORD_MISC_MMAP_DATA != 0 { b'r' as c_int } else { b'x' as c_int },
        (*event).mmap.filename.as_ptr(),
    ) as usize
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_mmap2(event: *mut perf_event, fp: *mut FILE) -> usize {
    if (*event).header.misc & PERF_RECORD_MISC_MMAP_BUILD_ID != 0 {
        let mut sbuild_id = [0 as c_char; SBUILD_ID_SIZE as usize];
        let mut bid = core::mem::MaybeUninit::<build_id>::uninit();

        build_id__init(bid.as_mut_ptr(), (*event).mmap2.build_id.as_ptr(), (*event).mmap2.build_id_size);
        build_id__snprintf(bid.as_ptr(), sbuild_id.as_mut_ptr(), sbuild_id.len());

        fprintf(
            fp,
            c(b" %d/%d: [%#lx(%#lx) @ %#lx <%s>]: %c%c%c%c %s\n\0"),
            (*event).mmap2.pid,
            (*event).mmap2.tid,
            (*event).mmap2.start,
            (*event).mmap2.len,
            (*event).mmap2.pgoff,
            sbuild_id.as_ptr(),
            if (*event).mmap2.prot & PROT_READ != 0 { b'r' as c_int } else { b'-' as c_int },
            if (*event).mmap2.prot & PROT_WRITE != 0 { b'w' as c_int } else { b'-' as c_int },
            if (*event).mmap2.prot & PROT_EXEC != 0 { b'x' as c_int } else { b'-' as c_int },
            if (*event).mmap2.flags & MAP_SHARED != 0 { b's' as c_int } else { b'p' as c_int },
            (*event).mmap2.filename.as_ptr(),
        ) as usize
    } else {
        fprintf(
            fp,
            c(b" %d/%d: [%#lx(%#lx) @ %#lx %02x:%02x %lu %lu]: %c%c%c%c %s\n\0"),
            (*event).mmap2.pid,
            (*event).mmap2.tid,
            (*event).mmap2.start,
            (*event).mmap2.len,
            (*event).mmap2.pgoff,
            (*event).mmap2.maj,
            (*event).mmap2.min,
            (*event).mmap2.ino,
            (*event).mmap2.ino_generation,
            if (*event).mmap2.prot & PROT_READ != 0 { b'r' as c_int } else { b'-' as c_int },
            if (*event).mmap2.prot & PROT_WRITE != 0 { b'w' as c_int } else { b'-' as c_int },
            if (*event).mmap2.prot & PROT_EXEC != 0 { b'x' as c_int } else { b'-' as c_int },
            if (*event).mmap2.flags & MAP_SHARED != 0 { b's' as c_int } else { b'p' as c_int },
            (*event).mmap2.filename.as_ptr(),
        ) as usize
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_thread_map(event: *mut perf_event, fp: *mut FILE) -> usize {
    let threads = thread_map__new_event(&mut (*event).thread_map);
    let mut ret = fprintf(fp, c(b" nr: \0")) as usize;

    if !threads.is_null() {
        ret += thread_map__fprintf(threads, fp);
    } else {
        ret += fprintf(fp, c(b"failed to get threads from event\n\0")) as usize;
    }

    perf_thread_map__put(threads);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_cpu_map(event: *mut perf_event, fp: *mut FILE) -> usize {
    let cpus = cpu_map__new_data(&mut (*event).cpu_map.data);
    let mut ret = fprintf(fp, c(b": \0")) as usize;

    if !cpus.is_null() {
        ret += cpu_map__fprintf(cpus, fp);
    } else {
        ret += fprintf(fp, c(b"failed to get cpumap from event\n\0")) as usize;
    }

    perf_cpu_map__put(cpus);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_mmap(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_mmap_event(machine, event, sample)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_mmap2(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_mmap2_event(machine, event, sample)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_task(event: *mut perf_event, fp: *mut FILE) -> usize {
    fprintf(fp, c(b"(%d:%d):(%d:%d)\n\0"), (*event).fork.pid, (*event).fork.tid, (*event).fork.ppid, (*event).fork.ptid) as usize
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_fork(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_fork_event(machine, event, sample)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_exit(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_exit_event(machine, event, sample)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__exit_del_thread(_tool: *const perf_tool, event: *mut perf_event, _sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let thread = machine__findnew_thread(machine, (*event).fork.pid, (*event).fork.tid);

    dump_printf(c(b"(%d:%d):(%d:%d)\n\0"), (*event).fork.pid, (*event).fork.tid, (*event).fork.ppid, (*event).fork.ptid);

    if !thread.is_null() {
        machine__remove_thread(machine, thread);
        thread__put(thread);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_aux(event: *mut perf_event, fp: *mut FILE) -> usize {
    fprintf(
        fp,
        c(b" offset: %#lx size: %#lx flags: %#lx [%s%s%s%s]\n\0"),
        (*event).aux.aux_offset,
        (*event).aux.aux_size,
        (*event).aux.flags,
        if (*event).aux.flags & PERF_AUX_FLAG_TRUNCATED != 0 { c(b"T\0") } else { c(b"\0") },
        if (*event).aux.flags & PERF_AUX_FLAG_OVERWRITE != 0 { c(b"O\0") } else { c(b"\0") },
        if (*event).aux.flags & PERF_AUX_FLAG_PARTIAL != 0 { c(b"P\0") } else { c(b"\0") },
        if (*event).aux.flags & PERF_AUX_FLAG_COLLISION != 0 { c(b"C\0") } else { c(b"\0") },
    ) as usize
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_itrace_start(event: *mut perf_event, fp: *mut FILE) -> usize {
    fprintf(fp, c(b" pid: %u tid: %u\n\0"), (*event).itrace_start.pid, (*event).itrace_start.tid) as usize
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_aux_output_hw_id(event: *mut perf_event, fp: *mut FILE) -> usize {
    fprintf(fp, c(b" hw_id: %#lx\n\0"), (*event).aux_output_hw_id.hw_id) as usize
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_switch(event: *mut perf_event, fp: *mut FILE) -> usize {
    let out = (*event).header.misc & PERF_RECORD_MISC_SWITCH_OUT != 0;
    let in_out = if !out {
        c(b"IN         \0")
    } else if (*event).header.misc & PERF_RECORD_MISC_SWITCH_OUT_PREEMPT == 0 {
        c(b"OUT        \0")
    } else {
        c(b"OUT preempt\0")
    };

    if (*event).header.type_ == PERF_RECORD_SWITCH {
        return fprintf(fp, c(b" %s\n\0"), in_out) as usize;
    }

    fprintf(
        fp,
        c(b" %s  %s pid/tid: %5d/%-5d\n\0"),
        in_out,
        if out { c(b"next\0") } else { c(b"prev\0") },
        (*event).context_switch.next_prev_pid,
        (*event).context_switch.next_prev_tid,
    ) as usize
}

unsafe extern "C" fn perf_event__fprintf_lost(event: *mut perf_event, fp: *mut FILE) -> usize {
    fprintf(fp, c(b" lost %lu\n\0"), (*event).lost.lost) as usize
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_ksymbol(event: *mut perf_event, fp: *mut FILE) -> usize {
    fprintf(fp, c(b" addr %lx len %u type %u flags 0x%x name %s\n\0"), (*event).ksymbol.addr, (*event).ksymbol.len, (*event).ksymbol.ksym_type, (*event).ksymbol.flags, (*event).ksymbol.name.as_ptr()) as usize
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_bpf(event: *mut perf_event, fp: *mut FILE) -> usize {
    fprintf(fp, c(b" type %u, flags %u, id %u\n\0"), (*event).bpf.type_, (*event).bpf.flags, (*event).bpf.id) as usize
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_bpf_metadata(event: *mut perf_event, fp: *mut FILE) -> usize {
    let metadata = &mut (*event).bpf_metadata as *mut perf_record_bpf_metadata;
    let mut ret = fprintf(fp, c(b" prog %s\n\0"), (*metadata).prog_name.as_ptr()) as usize;

    let mut i: u32 = 0;
    while i < (*metadata).nr_entries {
        ret += fprintf(fp, c(b"  entry %d: %20s = %s\n\0"), i, (*metadata).entries[i as usize].key.as_ptr(), (*metadata).entries[i as usize].value.as_ptr()) as usize;
        i += 1;
    }
    ret
}

unsafe extern "C" fn text_poke_printer(op: binary_printer_ops, val: c_uint, extra: *mut c_void, fp: *mut FILE) -> c_int {
    let old = *(extra as *mut bool);

    match op as c_int {
        BINARY_PRINT_LINE_BEGIN => fprintf(fp, c(b"            %s bytes:\0"), if old { c(b"Old\0") } else { c(b"New\0") }),
        BINARY_PRINT_NUM_DATA => fprintf(fp, c(b" %02x\0"), val),
        BINARY_PRINT_LINE_END => fprintf(fp, c(b"\n\0")),
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_text_poke(event: *mut perf_event, machine: *mut machine, fp: *mut FILE) -> usize {
    let tp = &mut (*event).text_poke as *mut perf_record_text_poke_event;
    let mut ret = fprintf(fp, c(b" %lx \0"), (*tp).addr) as usize;

    if !machine.is_null() {
        let mut al = core::mem::MaybeUninit::<addr_location>::uninit();

        addr_location__init(al.as_mut_ptr());
        (*al.as_mut_ptr()).map = maps__find(machine__kernel_maps(machine), (*tp).addr);
        if !(*al.as_mut_ptr()).map.is_null() && map__load((*al.as_mut_ptr()).map) >= 0 {
            (*al.as_mut_ptr()).addr = map__map_ip((*al.as_mut_ptr()).map, (*tp).addr);
            (*al.as_mut_ptr()).sym = map__find_symbol((*al.as_mut_ptr()).map, (*al.as_mut_ptr()).addr);
            if !(*al.as_mut_ptr()).sym.is_null() {
                ret += symbol__fprintf_symname_offs((*al.as_mut_ptr()).sym, al.as_mut_ptr(), fp);
            }
        }
        addr_location__exit(al.as_mut_ptr());
    }
    ret += fprintf(fp, c(b" old len %u new len %u\n\0"), (*tp).old_len, (*tp).new_len) as usize;
    let mut old = true;
    ret += binary__fprintf((*tp).bytes.as_ptr(), (*tp).old_len, 16, Some(text_poke_printer), &mut old as *mut _ as *mut c_void, fp);
    old = false;
    ret += binary__fprintf((*tp).bytes.as_ptr().add((*tp).old_len as usize), (*tp).new_len, 16, Some(text_poke_printer), &mut old as *mut _ as *mut c_void, fp);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_schedstat_cpu(event: *mut perf_event, fp: *mut FILE) -> usize {
    let cs = &mut (*event).schedstat_cpu as *mut perf_record_schedstat_cpu;
    let version = (*cs).version;
    let mut size = fprintf(fp, c(b"\ncpu%u \0"), (*cs).cpu) as usize;

    // The C source expands CPU_FIELD via <perf/schedstat-v15.h>, v16 and v17 here.
    // Those generated field lists are external build-time dependencies for this isolated translation.
    if version == 15 {
        return size;
    } else if version == 16 {
        return size;
    } else if version == 17 {
        return size;
    }

    fprintf(fp, c(b"Unsupported /proc/schedstat version %d.\n\0"), (*event).schedstat_cpu.version) as usize
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_schedstat_domain(event: *mut perf_event, fp: *mut FILE) -> usize {
    let ds = &mut (*event).schedstat_domain as *mut perf_record_schedstat_domain;
    let version = (*ds).version;
    let mut size = fprintf(fp, c(b"\ndomain%u \0"), (*ds).domain) as usize;

    // The C source expands DOMAIN_FIELD via <perf/schedstat-v15.h>, v16 and v17 here.
    // Those generated field lists are external build-time dependencies for this isolated translation.
    if version == 15 {
        return size;
    } else if version == 16 {
        return size;
    } else if version == 17 {
        return size;
    }

    fprintf(fp, c(b"Unsupported /proc/schedstat version %d.\n\0"), (*event).schedstat_domain.version) as usize
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf(event: *mut perf_event, machine: *mut machine, fp: *mut FILE) -> usize {
    let mut ret = fprintf(fp, c(b"PERF_RECORD_%s\0"), perf_event__name((*event).header.type_)) as usize;

    match (*event).header.type_ {
        PERF_RECORD_COMM => ret += perf_event__fprintf_comm(event, fp),
        PERF_RECORD_FORK | PERF_RECORD_EXIT => ret += perf_event__fprintf_task(event, fp),
        PERF_RECORD_MMAP => ret += perf_event__fprintf_mmap(event, fp),
        PERF_RECORD_NAMESPACES => ret += perf_event__fprintf_namespaces(event, fp),
        PERF_RECORD_CGROUP => ret += perf_event__fprintf_cgroup(event, fp),
        PERF_RECORD_MMAP2 => ret += perf_event__fprintf_mmap2(event, fp),
        PERF_RECORD_AUX => ret += perf_event__fprintf_aux(event, fp),
        PERF_RECORD_ITRACE_START => ret += perf_event__fprintf_itrace_start(event, fp),
        PERF_RECORD_SWITCH | PERF_RECORD_SWITCH_CPU_WIDE => ret += perf_event__fprintf_switch(event, fp),
        PERF_RECORD_LOST => ret += perf_event__fprintf_lost(event, fp),
        PERF_RECORD_KSYMBOL => ret += perf_event__fprintf_ksymbol(event, fp),
        PERF_RECORD_BPF_EVENT => ret += perf_event__fprintf_bpf(event, fp),
        PERF_RECORD_TEXT_POKE => ret += perf_event__fprintf_text_poke(event, machine, fp),
        PERF_RECORD_AUX_OUTPUT_HW_ID => ret += perf_event__fprintf_aux_output_hw_id(event, fp),
        PERF_RECORD_BPF_METADATA => ret += perf_event__fprintf_bpf_metadata(event, fp),
        _ => ret += fprintf(fp, c(b"\n\0")) as usize,
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    machine__process_event(machine, event, sample)
}

#[no_mangle]
pub unsafe extern "C" fn thread__find_map(thread: *mut thread, cpumode: u8, addr: u64, al: *mut addr_location) -> *mut map {
    let mut maps = thread__maps(thread);
    let machine = maps__machine(maps);
    let mut load_map = false;

    map__zput(&mut (*al).map);
    thread__zput(&mut (*al).thread);
    (*al).thread = thread__get(thread);

    (*al).addr = addr;
    (*al).cpumode = cpumode;
    (*al).filtered = 0;

    if machine.is_null() {
        return core::ptr::null_mut();
    }

    if cpumode == PERF_RECORD_MISC_KERNEL && perf_host {
        (*al).level = b'k' as c_char;
        maps = machine__kernel_maps(machine);
        load_map = !symbol_conf.lazy_load_kernel_maps;
    } else if cpumode == PERF_RECORD_MISC_USER && perf_host {
        (*al).level = b'.' as c_char;
    } else if cpumode == PERF_RECORD_MISC_GUEST_KERNEL && perf_guest {
        (*al).level = b'g' as c_char;
        maps = machine__kernel_maps(machine);
        load_map = !symbol_conf.lazy_load_kernel_maps;
    } else if cpumode == PERF_RECORD_MISC_GUEST_USER && perf_guest {
        (*al).level = b'u' as c_char;
    } else {
        (*al).level = b'H' as c_char;

        if (cpumode == PERF_RECORD_MISC_GUEST_USER || cpumode == PERF_RECORD_MISC_GUEST_KERNEL) && !perf_guest {
            (*al).filtered |= 1 << HIST_FILTER__GUEST;
        }
        if (cpumode == PERF_RECORD_MISC_USER || cpumode == PERF_RECORD_MISC_KERNEL) && !perf_host {
            (*al).filtered |= 1 << HIST_FILTER__HOST;
        }

        return core::ptr::null_mut();
    }
    (*al).map = maps__find(maps, (*al).addr);
    if !(*al).map.is_null() {
        /*
         * Kernel maps might be changed when loading symbols so loading
         * must be done prior to using kernel maps.
         */
        if load_map {
            map__load((*al).map);
        }
        (*al).addr = map__map_ip((*al).map, (*al).addr);
    }

    (*al).map
}

/*
 * For branch stacks or branch samples, the sample cpumode might not be correct
 * because it applies only to the sample 'ip' and not necessary to 'addr' or
 * branch stack addresses. If possible, use a fallback to deal with those cases.
 */
#[no_mangle]
pub unsafe extern "C" fn thread__find_map_fb(thread: *mut thread, cpumode: u8, addr: u64, al: *mut addr_location) -> *mut map {
    let map = thread__find_map(thread, cpumode, addr, al);
    let machine = maps__machine(thread__maps(thread));
    let addr_cpumode = machine__addr_cpumode(machine, cpumode, addr);

    if !map.is_null() || addr_cpumode == cpumode {
        return map;
    }

    thread__find_map(thread, addr_cpumode, addr, al)
}

#[no_mangle]
pub unsafe extern "C" fn thread__find_symbol(thread: *mut thread, cpumode: u8, addr: u64, al: *mut addr_location) -> *mut symbol {
    (*al).sym = core::ptr::null_mut();
    if !thread__find_map(thread, cpumode, addr, al).is_null() {
        (*al).sym = map__find_symbol((*al).map, (*al).addr);
    }
    (*al).sym
}

#[no_mangle]
pub unsafe extern "C" fn thread__find_symbol_fb(thread: *mut thread, cpumode: u8, addr: u64, al: *mut addr_location) -> *mut symbol {
    (*al).sym = core::ptr::null_mut();
    if !thread__find_map_fb(thread, cpumode, addr, al).is_null() {
        (*al).sym = map__find_symbol((*al).map, (*al).addr);
    }
    (*al).sym
}

unsafe fn check_address_range(addr_list: *mut intlist, addr_range: c_int, addr: c_ulong) -> bool {
    let mut pos: *mut int_node = core::ptr::null_mut();

    // Translates intlist__for_each_entry(pos, addr_list); iteration primitive is external.
    intlist__for_each_entry!(pos, addr_list, {
        if addr >= (*pos).i && addr < (*pos).i + addr_range as c_ulong {
            return true;
        }
    });

    false
}

/*
 * Callers need to drop the reference to al->thread, obtained in
 * machine__findnew_thread()
 */
#[no_mangle]
pub unsafe extern "C" fn machine__resolve(machine: *mut machine, al: *mut addr_location, sample: *mut perf_sample) -> c_int {
    let mut thread: *mut thread;
    let mut dso: *mut dso;

    if symbol_conf.guest_code && !machine__is_host(machine) {
        thread = machine__findnew_guest_code(machine, (*sample).pid);
    } else {
        thread = machine__findnew_thread(machine, (*sample).pid, (*sample).tid);
    }
    if thread.is_null() {
        return -1;
    }

    dump_printf(c(b" ... thread: %s:%d\n\0"), thread__comm_str(thread), thread__tid(thread));
    thread__find_map(thread, (*sample).cpumode, (*sample).ip, al);
    dso = if !(*al).map.is_null() { map__dso((*al).map) } else { core::ptr::null_mut() };
    dump_printf(
        c(b" ...... dso: %s\n\0"),
        if !dso.is_null() {
            dso__long_name(dso)
        } else if (*al).level == b'H' as c_char {
            c(b"[hypervisor]\0")
        } else {
            c(b"<not found>\0")
        },
    );

    if thread__is_filtered(thread) {
        (*al).filtered |= 1 << HIST_FILTER__THREAD;
    }

    thread__put(thread);
    thread = core::ptr::null_mut();

    (*al).sym = core::ptr::null_mut();
    (*al).cpu = (*sample).cpu;
    (*al).socket = -1;
    (*al).srcline = core::ptr::null_mut();

    if (*al).cpu >= 0 {
        let env = (*machine).env;

        /*
         * Bounds-check al->cpu (s32) before casting to struct perf_cpu
         * (int16_t): without this, e.g. 65536 truncates to 0 and silently
         * returns CPU 0's topology.  Can go once perf_cpu.cpu is widened.
         */
        if !env.is_null() && (*al).cpu < (*env).nr_cpus_avail {
            let topo = perf_env__get_cpu_topology(env, perf_cpu { cpu: (*al).cpu });
            if !topo.is_null() {
                (*al).socket = (*topo).socket_id;
            }
        }
    }

    /* Account for possible out-of-order switch events. */
    (*al).parallelism = core::cmp::max(1, core::cmp::min((*machine).parallelism, machine__nr_cpus_avail(machine)));
    if test_bit((*al).parallelism, symbol_conf.parallelism_filter) {
        (*al).filtered |= 1 << HIST_FILTER__PARALLELISM;
    }
    /*
     * Multiply it by some const to avoid precision loss or dealing
     * with floats. The multiplier does not matter otherwise since
     * we only print it as percents.
     */
    (*al).latency = (*sample).period * 1000 / (*al).parallelism as u64;

    if !(*al).map.is_null() {
        if !symbol_conf.dso_list.is_null()
            && (dso.is_null()
                || !(strlist__has_entry(symbol_conf.dso_list, dso__short_name(dso)) != 0
                    || (dso__short_name(dso) != dso__long_name(dso)
                        && strlist__has_entry(symbol_conf.dso_list, dso__long_name(dso)) != 0)))
        {
            (*al).filtered |= 1 << HIST_FILTER__DSO;
        }

        (*al).sym = map__find_symbol((*al).map, (*al).addr);
    } else if !symbol_conf.dso_list.is_null() {
        (*al).filtered |= 1 << HIST_FILTER__DSO;
    }

    if !symbol_conf.sym_list.is_null() {
        let mut ret = 0;
        let mut al_addr_str = [0 as c_char; 32];
        let sz = al_addr_str.len();

        if !(*al).sym.is_null() {
            ret = strlist__has_entry(symbol_conf.sym_list, (*(*al).sym).name);
        }
        if ret == 0 && !(*al).sym.is_null() {
            snprintf(al_addr_str.as_mut_ptr(), sz, c(b"0x%lx\0"), map__unmap_ip((*al).map, (*(*al).sym).start));
            ret = strlist__has_entry(symbol_conf.sym_list, al_addr_str.as_ptr());
        }
        if ret == 0 && !symbol_conf.addr_list.is_null() && !(*al).map.is_null() {
            let addr = map__unmap_ip((*al).map, (*al).addr) as c_ulong;

            ret = intlist__has_entry(symbol_conf.addr_list, addr);
            if ret == 0 && symbol_conf.addr_range != 0 {
                ret = check_address_range(symbol_conf.addr_list, symbol_conf.addr_range, addr) as c_int;
            }
        }

        if ret == 0 {
            (*al).filtered |= 1 << HIST_FILTER__SYMBOL;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn is_bts_event(attr: *mut perf_event_attr) -> bool {
    (*attr).type_ == PERF_TYPE_HARDWARE
        && ((*attr).config & PERF_COUNT_HW_BRANCH_INSTRUCTIONS) != 0
        && (*attr).sample_period == 1
}

#[no_mangle]
pub unsafe extern "C" fn sample_addr_correlates_sym(attr: *mut perf_event_attr) -> bool {
    if (*attr).type_ == PERF_TYPE_SOFTWARE
        && ((*attr).config == PERF_COUNT_SW_PAGE_FAULTS
            || (*attr).config == PERF_COUNT_SW_PAGE_FAULTS_MIN
            || (*attr).config == PERF_COUNT_SW_PAGE_FAULTS_MAJ)
    {
        return true;
    }

    if is_bts_event(attr) {
        return true;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn thread__resolve(thread: *mut thread, al: *mut addr_location, sample: *mut perf_sample) {
    thread__find_map_fb(thread, (*sample).cpumode, (*sample).addr, al);

    (*al).cpu = (*sample).cpu;
    (*al).sym = core::ptr::null_mut();

    if !(*al).map.is_null() {
        (*al).sym = map__find_symbol((*al).map, (*al).addr);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
