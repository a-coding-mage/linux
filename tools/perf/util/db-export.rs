// SPDX-License-Identifier: GPL-2.0-only
/*
 * db-export.c: Support for exporting data suitable for import to a database
 * Copyright (c) 2014, Intel Corporation.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

type u32 = u32;
type u64 = u64;
type pid_t = i32;

#[repr(C)]
pub struct db_export {
    pub evsel_last_db_id: u64,
    pub machine_last_db_id: u64,
    pub thread_last_db_id: u64,
    pub comm_last_db_id: u64,
    pub comm_thread_last_db_id: u64,
    pub dso_last_db_id: u64,
    pub symbol_last_db_id: u64,
    pub sample_last_db_id: u64,
    pub call_path_last_db_id: u64,
    pub call_return_last_db_id: u64,
    pub context_switch_last_db_id: u64,
    pub cpr: *mut call_path_root,
    pub crp: *mut call_return_processor,
    pub export_evsel: Option<unsafe extern "C" fn(*mut db_export, *mut evsel) -> c_int>,
    pub export_machine: Option<unsafe extern "C" fn(*mut db_export, *mut machine) -> c_int>,
    pub export_thread:
        Option<unsafe extern "C" fn(*mut db_export, *mut thread, u64, *mut machine) -> c_int>,
    pub export_comm: Option<unsafe extern "C" fn(*mut db_export, *mut comm, *mut thread) -> c_int>,
    pub export_comm_thread:
        Option<unsafe extern "C" fn(*mut db_export, u64, *mut comm, *mut thread) -> c_int>,
    pub export_dso: Option<unsafe extern "C" fn(*mut db_export, *mut dso, *mut machine) -> c_int>,
    pub export_symbol: Option<unsafe extern "C" fn(*mut db_export, *mut symbol, *mut dso) -> c_int>,
    pub export_branch_type: Option<unsafe extern "C" fn(*mut db_export, u32, *const c_char) -> c_int>,
    pub export_sample: Option<unsafe extern "C" fn(*mut db_export, *mut export_sample) -> c_int>,
    pub export_call_path: Option<unsafe extern "C" fn(*mut db_export, *mut call_path) -> c_int>,
    pub export_call_return: Option<unsafe extern "C" fn(*mut db_export, *mut call_return) -> c_int>,
    pub export_context_switch: Option<
        unsafe extern "C" fn(
            *mut db_export,
            u64,
            *mut machine,
            *mut perf_sample,
            u64,
            u64,
            u64,
            u64,
            c_int,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct evsel {
    pub db_id: u64,
}

#[repr(C)]
pub struct machine {
    pub db_id: u64,
}

#[repr(C)]
pub struct comm {
    pub db_id: u64,
}

#[repr(C)]
pub struct symbol {
    pub start: u64,
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct call_path_root {
    pub call_path: call_path,
}

#[repr(C)]
pub struct call_path {
    pub db_id: u64,
    pub parent: *mut call_path,
}

#[repr(C)]
pub struct call_return {
    pub db_id: u64,
    pub parent_db_id: u64,
    pub cp: *mut call_path,
}

#[repr(C)]
pub struct call_return_processor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
}

#[repr(C)]
pub struct perf_event_context_switch {
    pub next_prev_pid: pid_t,
    pub next_prev_tid: pid_t,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
    pub context_switch: perf_event_context_switch,
}

#[repr(C)]
pub struct perf_sample {
    pub callchain: *mut c_void,
    pub evsel: *mut evsel,
    pub pid: pid_t,
    pub tid: pid_t,
}

#[repr(C)]
pub struct addr_location {
    pub map: *mut map,
    pub sym: *mut symbol,
    pub addr: u64,
    pub thread: *mut thread,
}

#[repr(C)]
pub struct export_sample {
    pub event: *mut perf_event,
    pub sample: *mut perf_sample,
    pub al: *mut addr_location,
    pub db_id: u64,
    pub comm_db_id: u64,
    pub dso_db_id: u64,
    pub sym_db_id: u64,
    pub offset: u64,
    pub call_path_id: u64,
    pub addr_dso_db_id: u64,
    pub addr_sym_db_id: u64,
    pub addr_offset: u64,
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct callchain_cursor_node {
    pub ip: u64,
    pub ms: map_symbol,
}

#[repr(C)]
pub struct callchain_cursor {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum chain_order {
    ORDER_CALLER = 0,
}

#[repr(C)]
pub struct callchain_param_t {
    pub order: chain_order,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub use_callchain: bool,
}

const PERF_MAX_STACK_DEPTH: c_int = 127;
const PERF_IP_FLAG_BRANCH: u32 = 1 << 0;
const PERF_IP_FLAG_CALL: u32 = 1 << 1;
const PERF_IP_FLAG_RETURN: u32 = 1 << 2;
const PERF_IP_FLAG_CONDITIONAL: u32 = 1 << 3;
const PERF_IP_FLAG_SYSCALLRET: u32 = 1 << 4;
const PERF_IP_FLAG_ASYNC: u32 = 1 << 5;
const PERF_IP_FLAG_INTERRUPT: u32 = 1 << 6;
const PERF_IP_FLAG_TX_ABORT: u32 = 1 << 7;
const PERF_IP_FLAG_TRACE_BEGIN: u32 = 1 << 8;
const PERF_IP_FLAG_TRACE_END: u32 = 1 << 9;
const PERF_IP_FLAG_VMENTRY: u32 = 1 << 10;
const PERF_IP_FLAG_VMEXIT: u32 = 1 << 11;
const PERF_RECORD_MISC_SWITCH_OUT: u16 = 1 << 13;
const PERF_RECORD_MISC_SWITCH_OUT_PREEMPT: u16 = 1 << 14;
const PERF_RECORD_SWITCH_CPU_WIDE: u32 = 15;

unsafe extern "C" {
    static mut callchain_param: callchain_param_t;
    static mut symbol_conf: symbol_conf_t;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn call_return_processor__free(crp: *mut call_return_processor);
    fn thread__db_id(thread: *mut thread) -> u64;
    fn thread__set_db_id(thread: *mut thread, db_id: u64);
    fn dso__db_id(dso: *mut dso) -> u64;
    fn dso__set_db_id(dso: *mut dso, db_id: u64);
    fn symbol__priv(sym: *mut symbol) -> *mut u64;
    fn map__dso(map: *mut map) -> *mut dso;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn symbol__new(addr: u64, len: u64, binding: u8, type_: u8, name: *const c_char) -> *mut symbol;
    fn dso__insert_symbol(dso: *mut dso, sym: *mut symbol);
    fn machine__kernel_start(machine: *mut machine) -> u64;
    fn get_tls_callchain_cursor() -> *mut callchain_cursor;
    fn thread__resolve_callchain(
        thread: *mut thread,
        cursor: *mut callchain_cursor,
        sample: *mut perf_sample,
        parent: *mut c_void,
        root_al: *mut c_void,
        max_stack: c_int,
    ) -> c_int;
    fn callchain_cursor_commit(cursor: *mut callchain_cursor);
    fn callchain_cursor_current(cursor: *mut callchain_cursor) -> *mut callchain_cursor_node;
    fn callchain_cursor_advance(cursor: *mut callchain_cursor);
    fn addr_location__init(al: *mut addr_location);
    fn map__get(map: *mut map) -> *mut map;
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn dso__find_symbol(dso: *mut dso, addr: u64) -> *mut symbol;
    fn addr_location__exit(al: *mut addr_location);
    fn call_path__findnew(
        cpr: *mut call_path_root,
        parent: *mut call_path,
        sym: *mut symbol,
        ip: u64,
        kernel_start: u64,
    ) -> *mut call_path;
    fn machine__thread_exec_comm(machine: *mut machine, thread: *mut thread) -> *mut comm;
    fn thread__comm(thread: *mut thread) -> *mut comm;
    fn thread__main_thread(machine: *mut machine, thread: *mut thread) -> *mut thread;
    fn thread__put(thread: *mut thread);
    fn thread_stack__process(
        thread: *mut thread,
        comm: *mut comm,
        sample: *mut perf_sample,
        al: *mut addr_location,
        addr_al: *mut addr_location,
        sample_db_id: u64,
        crp: *mut call_return_processor,
    ) -> c_int;
    fn machine__find_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    fn thread__comm_set(thread: *mut thread) -> bool;
    fn thread__pid(thread: *mut thread) -> pid_t;
    fn thread__tid(thread: *mut thread) -> pid_t;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__init(dbe: *mut db_export) -> c_int {
    unsafe {
        memset(dbe as *mut c_void, 0, mem::size_of::<db_export>());
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__exit(dbe: *mut db_export) {
    unsafe {
        call_return_processor__free((*dbe).crp);
        (*dbe).crp = ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__evsel(dbe: *mut db_export, evsel: *mut evsel) -> c_int {
    unsafe {
        if (*evsel).db_id != 0 {
            return 0;
        }

        (*dbe).evsel_last_db_id += 1;
        (*evsel).db_id = (*dbe).evsel_last_db_id;

        if let Some(export_evsel) = (*dbe).export_evsel {
            return export_evsel(dbe, evsel);
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__machine(dbe: *mut db_export, machine: *mut machine) -> c_int {
    unsafe {
        if (*machine).db_id != 0 {
            return 0;
        }

        (*dbe).machine_last_db_id += 1;
        (*machine).db_id = (*dbe).machine_last_db_id;

        if let Some(export_machine) = (*dbe).export_machine {
            return export_machine(dbe, machine);
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__thread(
    dbe: *mut db_export,
    thread: *mut thread,
    machine: *mut machine,
    main_thread: *mut thread,
) -> c_int {
    let mut main_thread_db_id: u64 = 0;

    unsafe {
        if thread__db_id(thread) != 0 {
            return 0;
        }

        (*dbe).thread_last_db_id += 1;
        thread__set_db_id(thread, (*dbe).thread_last_db_id);

        if !main_thread.is_null() {
            main_thread_db_id = thread__db_id(main_thread);
        }

        if let Some(export_thread) = (*dbe).export_thread {
            return export_thread(dbe, thread, main_thread_db_id, machine);
        }
    }

    0
}

unsafe extern "C" fn __db_export__comm(
    dbe: *mut db_export,
    comm: *mut comm,
    thread: *mut thread,
) -> c_int {
    unsafe {
        (*dbe).comm_last_db_id += 1;
        (*comm).db_id = (*dbe).comm_last_db_id;

        if let Some(export_comm) = (*dbe).export_comm {
            return export_comm(dbe, comm, thread);
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__comm(
    dbe: *mut db_export,
    comm: *mut comm,
    thread: *mut thread,
) -> c_int {
    unsafe {
        if (*comm).db_id != 0 {
            return 0;
        }

        __db_export__comm(dbe, comm, thread)
    }
}

/*
 * Export the "exec" comm. The "exec" comm is the program / application command
 * name at the time it first executes. It is used to group threads for the same
 * program. Note that the main thread pid (or thread group id tgid) cannot be
 * used because it does not change when a new program is exec'ed.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__exec_comm(
    dbe: *mut db_export,
    comm: *mut comm,
    main_thread: *mut thread,
) -> c_int {
    let err: c_int;

    unsafe {
        if (*comm).db_id != 0 {
            return 0;
        }

        err = __db_export__comm(dbe, comm, main_thread);
        if err != 0 {
            return err;
        }

        /*
         * Record the main thread for this comm. Note that the main thread can
         * have many "exec" comms because there will be a new one every time it
         * exec's. An "exec" comm however will only ever have 1 main thread.
         * That is different to any other threads for that same program because
         * exec() will effectively kill them, so the relationship between the
         * "exec" comm and non-main threads is 1-to-1. That is why
         * db_export__comm_thread() is called here for the main thread, but it
         * is called for non-main threads when they are exported.
         */
        db_export__comm_thread(dbe, comm, main_thread)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__comm_thread(
    dbe: *mut db_export,
    comm: *mut comm,
    thread: *mut thread,
) -> c_int {
    unsafe {
        (*dbe).comm_thread_last_db_id += 1;
        let db_id = (*dbe).comm_thread_last_db_id;

        if let Some(export_comm_thread) = (*dbe).export_comm_thread {
            return export_comm_thread(dbe, db_id, comm, thread);
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__dso(
    dbe: *mut db_export,
    dso: *mut dso,
    machine: *mut machine,
) -> c_int {
    unsafe {
        if dso__db_id(dso) != 0 {
            return 0;
        }

        (*dbe).dso_last_db_id += 1;
        dso__set_db_id(dso, (*dbe).dso_last_db_id);

        if let Some(export_dso) = (*dbe).export_dso {
            return export_dso(dbe, dso, machine);
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__symbol(
    dbe: *mut db_export,
    sym: *mut symbol,
    dso: *mut dso,
) -> c_int {
    unsafe {
        let sym_db_id = symbol__priv(sym) as *mut u64;

        if *sym_db_id != 0 {
            return 0;
        }

        (*dbe).symbol_last_db_id += 1;
        *sym_db_id = (*dbe).symbol_last_db_id;

        if let Some(export_symbol) = (*dbe).export_symbol {
            return export_symbol(dbe, sym, dso);
        }
    }

    0
}

unsafe extern "C" fn db_ids_from_al(
    dbe: *mut db_export,
    al: *mut addr_location,
    dso_db_id: *mut u64,
    sym_db_id: *mut u64,
    offset: *mut u64,
) -> c_int {
    let mut err: c_int;

    unsafe {
        if !(*al).map.is_null() {
            let dso = map__dso((*al).map);

            err = db_export__dso(dbe, dso, maps__machine(thread__maps((*al).thread)));
            if err != 0 {
                return err;
            }
            *dso_db_id = dso__db_id(dso);

            if (*al).sym.is_null() {
                (*al).sym = symbol__new((*al).addr, 0, 0, 0, c"unknown".as_ptr());
                if !(*al).sym.is_null() {
                    dso__insert_symbol(dso, (*al).sym);
                }
            }

            if !(*al).sym.is_null() {
                let db_id = symbol__priv((*al).sym) as *mut u64;

                err = db_export__symbol(dbe, (*al).sym, dso);
                if err != 0 {
                    return err;
                }
                *sym_db_id = *db_id;
                *offset = (*al).addr - (*(*al).sym).start;
            }
        }
    }

    0
}

unsafe extern "C" fn call_path_from_sample(
    dbe: *mut db_export,
    machine: *mut machine,
    thread: *mut thread,
    sample: *mut perf_sample,
) -> *mut call_path {
    unsafe {
        let kernel_start = machine__kernel_start(machine);
        let mut current = &mut (*(*dbe).cpr).call_path as *mut call_path;
        let saved_order = callchain_param.order;
        let cursor: *mut callchain_cursor;
        let err: c_int;

        if !symbol_conf.use_callchain || (*sample).callchain.is_null() {
            return ptr::null_mut();
        }

        /*
         * Since the call path tree must be built starting with the root, we
         * must use ORDER_CALL for call chain resolution, in order to process
         * the callchain starting with the root node and ending with the leaf.
         */
        callchain_param.order = chain_order::ORDER_CALLER;
        cursor = get_tls_callchain_cursor();
        err = thread__resolve_callchain(
            thread,
            cursor,
            sample,
            ptr::null_mut(),
            ptr::null_mut(),
            PERF_MAX_STACK_DEPTH,
        );
        if err != 0 {
            callchain_param.order = saved_order;
            return ptr::null_mut();
        }
        callchain_cursor_commit(cursor);

        loop {
            let node: *mut callchain_cursor_node;
            let mut al: addr_location = mem::zeroed();
            let mut dso_db_id: u64 = 0;
            let mut sym_db_id: u64 = 0;
            let mut offset: u64 = 0;

            node = callchain_cursor_current(cursor);
            if node.is_null() {
                break;
            }

            /*
             * Handle export of symbol and dso for this node by
             * constructing an addr_location struct and then passing it to
             * db_ids_from_al() to perform the export.
             */
            addr_location__init(&mut al);
            al.sym = (*node).ms.sym;
            al.map = map__get((*node).ms.map);
            al.addr = (*node).ip;
            al.thread = thread__get(thread);

            if !al.map.is_null() && al.sym.is_null() {
                al.sym = dso__find_symbol(map__dso(al.map), al.addr);
            }

            db_ids_from_al(dbe, &mut al, &mut dso_db_id, &mut sym_db_id, &mut offset);

            /* add node to the call path tree if it doesn't exist */
            current = call_path__findnew((*dbe).cpr, current, al.sym, (*node).ip, kernel_start);

            callchain_cursor_advance(cursor);
            addr_location__exit(&mut al);
        }

        /* Reset the callchain order to its prior value. */
        callchain_param.order = saved_order;

        if current == &mut (*(*dbe).cpr).call_path as *mut call_path {
            /* Bail because the callchain was empty. */
            return ptr::null_mut();
        }

        current
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__branch_type(
    dbe: *mut db_export,
    branch_type: u32,
    name: *const c_char,
) -> c_int {
    unsafe {
        if let Some(export_branch_type) = (*dbe).export_branch_type {
            return export_branch_type(dbe, branch_type, name);
        }
    }

    0
}

unsafe extern "C" fn db_export__threads(
    dbe: *mut db_export,
    thread: *mut thread,
    main_thread: *mut thread,
    machine: *mut machine,
    comm_ptr: *mut *mut comm,
) -> c_int {
    let mut comm: *mut comm = ptr::null_mut();
    let curr_comm: *mut comm;
    let mut err: c_int;

    unsafe {
        if !main_thread.is_null() {
            /*
             * A thread has a reference to the main thread, so export the
             * main thread first.
             */
            err = db_export__thread(dbe, main_thread, machine, main_thread);
            if err != 0 {
                return err;
            }
            /*
             * Export comm before exporting the non-main thread because
             * db_export__comm_thread() can be called further below.
             */
            comm = machine__thread_exec_comm(machine, main_thread);
            if !comm.is_null() {
                err = db_export__exec_comm(dbe, comm, main_thread);
                if err != 0 {
                    return err;
                }
                *comm_ptr = comm;
            }
        }

        if thread != main_thread {
            /*
             * For a non-main thread, db_export__comm_thread() must be
             * called only if thread has not previously been exported.
             */
            let export_comm_thread = !comm.is_null() && thread__db_id(thread) == 0;

            err = db_export__thread(dbe, thread, machine, main_thread);
            if err != 0 {
                return err;
            }

            if export_comm_thread {
                err = db_export__comm_thread(dbe, comm, thread);
                if err != 0 {
                    return err;
                }
            }
        }

        curr_comm = thread__comm(thread);
        if !curr_comm.is_null() {
            return db_export__comm(dbe, curr_comm, thread);
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__sample(
    dbe: *mut db_export,
    event: *mut perf_event,
    sample: *mut perf_sample,
    al: *mut addr_location,
    addr_al: *mut addr_location,
) -> c_int {
    unsafe {
        let thread = (*al).thread;
        let mut es: export_sample = mem::zeroed();
        es.event = event;
        es.sample = sample;
        es.al = al;
        let main_thread: *mut thread;
        let mut comm: *mut comm = ptr::null_mut();
        let mut machine: *mut machine = ptr::null_mut();
        let mut err: c_int;

        if !thread__maps(thread).is_null() {
            machine = maps__machine(thread__maps(thread));
        }
        if machine.is_null() {
            return -1;
        }

        err = db_export__evsel(dbe, (*sample).evsel);
        if err != 0 {
            return err;
        }

        err = db_export__machine(dbe, machine);
        if err != 0 {
            return err;
        }

        main_thread = thread__main_thread(machine, thread);

        err = db_export__threads(dbe, thread, main_thread, machine, &mut comm);
        if err != 0 {
            thread__put(main_thread);
            return err;
        }

        if !comm.is_null() {
            es.comm_db_id = (*comm).db_id;
        }

        (*dbe).sample_last_db_id += 1;
        es.db_id = (*dbe).sample_last_db_id;

        err = db_ids_from_al(dbe, al, &mut es.dso_db_id, &mut es.sym_db_id, &mut es.offset);
        if err != 0 {
            thread__put(main_thread);
            return err;
        }

        if !(*dbe).cpr.is_null() {
            let cp = call_path_from_sample(dbe, machine, thread, sample);
            if !cp.is_null() {
                db_export__call_path(dbe, cp);
                es.call_path_id = (*cp).db_id;
            }
        }

        if !addr_al.is_null() {
            err = db_ids_from_al(
                dbe,
                addr_al,
                &mut es.addr_dso_db_id,
                &mut es.addr_sym_db_id,
                &mut es.addr_offset,
            );
            if err != 0 {
                thread__put(main_thread);
                return err;
            }
            if !(*dbe).crp.is_null() {
                err = thread_stack__process(thread, comm, sample, al, addr_al, es.db_id, (*dbe).crp);
                if err != 0 {
                    thread__put(main_thread);
                    return err;
                }
            }
        }

        if let Some(export_sample) = (*dbe).export_sample {
            err = export_sample(dbe, &mut es);
        }

        thread__put(main_thread);
        err
    }
}

#[repr(C)]
struct branch_type_name {
    branch_type: u32,
    name: *const c_char,
}

static branch_types: [branch_type_name; 17] = [
    branch_type_name { branch_type: 0, name: c"no branch".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL, name: c"call".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN, name: c"return".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CONDITIONAL, name: c"conditional jump".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH, name: c"unconditional jump".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_INTERRUPT, name: c"software interrupt".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN | PERF_IP_FLAG_INTERRUPT, name: c"return from interrupt".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_SYSCALLRET, name: c"system call".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN | PERF_IP_FLAG_SYSCALLRET, name: c"return from system call".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_ASYNC, name: c"asynchronous branch".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_ASYNC | PERF_IP_FLAG_INTERRUPT, name: c"hardware interrupt".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_TX_ABORT, name: c"transaction abort".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_TRACE_BEGIN, name: c"trace begin".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_TRACE_END, name: c"trace end".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_VMENTRY, name: c"vm entry".as_ptr() },
    branch_type_name { branch_type: PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_VMEXIT, name: c"vm exit".as_ptr() },
    branch_type_name { branch_type: 0, name: ptr::null() },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__branch_types(dbe: *mut db_export) -> c_int {
    let mut i: usize;
    let mut err: c_int = 0;

    unsafe {
        i = 0;
        while !branch_types[i].name.is_null() {
            err = db_export__branch_type(dbe, branch_types[i].branch_type, branch_types[i].name);
            if err != 0 {
                break;
            }
            i += 1;
        }

        /* Add trace begin / end variants */
        i = 0;
        while !branch_types[i].name.is_null() {
            let name = branch_types[i].name;
            let type_ = branch_types[i].branch_type;
            let mut buf = [0 as c_char; 64];

            if type_ == PERF_IP_FLAG_BRANCH
                || (type_ & (PERF_IP_FLAG_TRACE_BEGIN | PERF_IP_FLAG_TRACE_END)) != 0
            {
                i += 1;
                continue;
            }

            snprintf(buf.as_mut_ptr(), mem::size_of_val(&buf), c"trace begin / %s".as_ptr(), name);
            err = db_export__branch_type(dbe, type_ | PERF_IP_FLAG_TRACE_BEGIN, buf.as_ptr());
            if err != 0 {
                break;
            }

            snprintf(buf.as_mut_ptr(), mem::size_of_val(&buf), c"%s / trace end".as_ptr(), name);
            err = db_export__branch_type(dbe, type_ | PERF_IP_FLAG_TRACE_END, buf.as_ptr());
            if err != 0 {
                break;
            }
            i += 1;
        }
    }

    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__call_path(dbe: *mut db_export, cp: *mut call_path) -> c_int {
    let mut err: c_int;

    unsafe {
        if (*cp).db_id != 0 {
            return 0;
        }

        if !(*cp).parent.is_null() {
            err = db_export__call_path(dbe, (*cp).parent);
            if err != 0 {
                return err;
            }
        }

        (*dbe).call_path_last_db_id += 1;
        (*cp).db_id = (*dbe).call_path_last_db_id;

        if let Some(export_call_path) = (*dbe).export_call_path {
            return export_call_path(dbe, cp);
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__call_return(
    dbe: *mut db_export,
    cr: *mut call_return,
    parent_db_id: *mut u64,
) -> c_int {
    let err: c_int;

    unsafe {
        err = db_export__call_path(dbe, (*cr).cp);
        if err != 0 {
            return err;
        }

        if (*cr).db_id == 0 {
            (*dbe).call_return_last_db_id += 1;
            (*cr).db_id = (*dbe).call_return_last_db_id;
        }

        if !parent_db_id.is_null() {
            if *parent_db_id == 0 {
                (*dbe).call_return_last_db_id += 1;
                *parent_db_id = (*dbe).call_return_last_db_id;
            }
            (*cr).parent_db_id = *parent_db_id;
        }

        if let Some(export_call_return) = (*dbe).export_call_return {
            return export_call_return(dbe, cr);
        }
    }

    0
}

unsafe extern "C" fn db_export__pid_tid(
    dbe: *mut db_export,
    machine: *mut machine,
    pid: pid_t,
    tid: pid_t,
    db_id: *mut u64,
    comm_ptr: *mut *mut comm,
    is_idle: *mut bool,
) -> c_int {
    unsafe {
        let thread = machine__find_thread(machine, pid, tid);
        let main_thread: *mut thread;
        let mut err: c_int = 0;

        if thread.is_null() || !thread__comm_set(thread) {
            thread__put(thread);
            return err;
        }

        *is_idle = thread__pid(thread) == 0 && thread__tid(thread) == 0;

        main_thread = thread__main_thread(machine, thread);

        err = db_export__threads(dbe, thread, main_thread, machine, comm_ptr);

        *db_id = thread__db_id(thread);

        thread__put(main_thread);
        thread__put(thread);

        err
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_export__switch(
    dbe: *mut db_export,
    event: *mut perf_event,
    sample: *mut perf_sample,
    machine: *mut machine,
) -> c_int {
    unsafe {
        let out = ((*event).header.misc & PERF_RECORD_MISC_SWITCH_OUT) != 0;
        let out_preempt =
            out && ((*event).header.misc & PERF_RECORD_MISC_SWITCH_OUT_PREEMPT) != 0;
        let flags = (out as c_int) | ((out_preempt as c_int) << 1);
        let mut is_idle_a = false;
        let mut is_idle_b = false;
        let mut th_a_id: u64 = 0;
        let mut th_b_id: u64 = 0;
        let comm_out_id: u64;
        let comm_in_id: u64;
        let mut comm_a: *mut comm = ptr::null_mut();
        let mut comm_b: *mut comm = ptr::null_mut();
        let th_out_id: u64;
        let th_in_id: u64;
        let db_id: u64;
        let mut err: c_int;

        err = db_export__machine(dbe, machine);
        if err != 0 {
            return err;
        }

        err = db_export__pid_tid(
            dbe,
            machine,
            (*sample).pid,
            (*sample).tid,
            &mut th_a_id,
            &mut comm_a,
            &mut is_idle_a,
        );
        if err != 0 {
            return err;
        }

        if (*event).header.type_ == PERF_RECORD_SWITCH_CPU_WIDE {
            let pid = (*event).context_switch.next_prev_pid;
            let tid = (*event).context_switch.next_prev_tid;

            err = db_export__pid_tid(
                dbe,
                machine,
                pid,
                tid,
                &mut th_b_id,
                &mut comm_b,
                &mut is_idle_b,
            );
            if err != 0 {
                return err;
            }
        }

        /*
         * Do not export if both threads are unknown (i.e. not being traced),
         * or one is unknown and the other is the idle task.
         */
        if (th_a_id == 0 || is_idle_a) && (th_b_id == 0 || is_idle_b) {
            return 0;
        }

        (*dbe).context_switch_last_db_id += 1;
        db_id = (*dbe).context_switch_last_db_id;

        if out {
            th_out_id = th_a_id;
            th_in_id = th_b_id;
            comm_out_id = if !comm_a.is_null() { (*comm_a).db_id } else { 0 };
            comm_in_id = if !comm_b.is_null() { (*comm_b).db_id } else { 0 };
        } else {
            th_out_id = th_b_id;
            th_in_id = th_a_id;
            comm_out_id = if !comm_b.is_null() { (*comm_b).db_id } else { 0 };
            comm_in_id = if !comm_a.is_null() { (*comm_a).db_id } else { 0 };
        }

        if let Some(export_context_switch) = (*dbe).export_context_switch {
            return export_context_switch(
                dbe,
                db_id,
                machine,
                sample,
                th_out_id,
                comm_out_id,
                th_in_id,
                comm_in_id,
                flags,
            );
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
