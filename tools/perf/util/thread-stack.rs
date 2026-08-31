// SPDX-License-Identifier: GPL-2.0-only
/*
 * thread-stack.c: Synthesize a thread's stack using call / return events
 * Copyright (c) 2014, Intel Corporation.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{copy_nonoverlapping, null_mut};

type u16 = u16;
type u32 = u32;
type u64 = u64;
type size_t = usize;

const STACK_GROWTH: size_t = 2048;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

const EM_386: u16 = 3;
const EM_X86_64: u16 = 62;

const PERF_CONTEXT_KERNEL: u64 = !0u64 - 127;
const PERF_CONTEXT_USER: u64 = !0u64 - 511;

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
const PERF_IP_FLAG_IN_TX: u32 = 1 << 10;
const PERF_IP_FLAG_VMENTRY: u32 = 1 << 11;
const PERF_IP_FLAG_VMEXIT: u32 = 1 << 12;

const CALL_RETURN_NO_CALL: u32 = 1 << 0;
const CALL_RETURN_NO_RETURN: u32 = 1 << 1;
const CALL_RETURN_NON_CALL: u32 = 1 << 2;

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
    pub start: u64,
}

#[repr(C)]
pub struct call_path {
    pub rb_node: [usize; 3],
    pub parent: *mut call_path,
    pub children: [usize; 2],
    pub sym: *mut symbol,
    pub ip: u64,
    pub db_id: u64,
    pub in_kernel: bool,
}

#[repr(C)]
pub struct call_path_root {
    pub call_path: call_path,
    pub blocks: *mut c_void,
}

#[repr(C)]
pub struct call_return_processor {
    pub cpr: *mut call_path_root,
    pub process: Option<
        unsafe extern "C" fn(
            cr: *mut call_return,
            parent_db_id: *mut u64,
            data: *mut c_void,
        ) -> c_int,
    >,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct call_return {
    pub thread: *mut thread,
    pub comm: *mut comm,
    pub cp: *mut call_path,
    pub call_time: u64,
    pub return_time: u64,
    pub branch_count: u64,
    pub insn_count: u64,
    pub cyc_count: u64,
    pub call_ref: u64,
    pub return_ref: u64,
    pub db_id: u64,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct branch_flags {
    pub value: u64,
}

impl branch_flags {
    unsafe fn set_abort(&mut self, v: bool) {
        if v {
            self.value |= 1 << 2;
        } else {
            self.value &= !(1 << 2);
        }
    }

    unsafe fn set_in_tx(&mut self, v: bool) {
        if v {
            self.value |= 1 << 3;
        } else {
            self.value &= !(1 << 3);
        }
    }

    unsafe fn set_mispred(&mut self, v: bool) {
        if v {
            self.value |= 1;
        } else {
            self.value &= !1;
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct branch_entry {
    pub from: u64,
    pub to: u64,
    pub flags: branch_flags,
}

#[repr(C)]
pub struct branch_stack {
    pub nr: u64,
    pub entries: [branch_entry; 0],
}

#[repr(C)]
pub struct ip_callchain {
    pub nr: u64,
    pub ips: [u64; 0],
}

#[repr(C)]
pub struct perf_sample {
    pub ip: u64,
    pub pid: u32,
    pub tid: u32,
    pub time: u64,
    pub addr: u64,
    pub id: u64,
    pub stream_id: u64,
    pub period: u64,
    pub cpu: u32,
    pub raw_size: u32,
    pub data_src: u64,
    pub weight: u64,
    pub insn_cnt: u64,
    pub cyc_cnt: u64,
    pub flags: u32,
    pub insn_len: u16,
}

#[repr(C)]
pub struct addr_location {
    pub sym: *mut symbol,
    pub addr: u64,
}

unsafe extern "C" {
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);

    fn thread__tid(thread: *mut thread) -> c_int;
    fn thread__pid(thread: *mut thread) -> c_int;
    fn thread__ts(thread: *mut thread) -> *mut thread_stack;
    fn thread__set_ts(thread: *mut thread, ts: *mut thread_stack);
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn thread__e_machine(thread: *mut thread, machine: *mut machine, e_flags: *mut u32) -> u16;
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn machine__kernel_start(machine: *mut machine) -> u64;

    fn call_path_root__new() -> *mut call_path_root;
    fn call_path_root__free(cpr: *mut call_path_root);
    fn call_path__findnew(
        cpr: *mut call_path_root,
        parent: *mut call_path,
        sym: *mut symbol,
        ip: u64,
        kernel_start: u64,
    ) -> *mut call_path;
}

/*
 * State of retpoline detection.
 *
 * RETPOLINE_NONE: no retpoline detection
 * X86_RETPOLINE_POSSIBLE: x86 retpoline possible
 * X86_RETPOLINE_DETECTED: x86 retpoline detected
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum retpoline_state_t {
    RETPOLINE_NONE,
    X86_RETPOLINE_POSSIBLE,
    X86_RETPOLINE_DETECTED,
}

/**
 * struct thread_stack_entry - thread stack entry.
 * @ret_addr: return address
 * @timestamp: timestamp (if known)
 * @ref: external reference (e.g. db_id of sample)
 * @branch_count: the branch count when the entry was created
 * @insn_count: the instruction count when the entry was created
 * @cyc_count the cycle count when the entry was created
 * @db_id: id used for db-export
 * @cp: call path
 * @no_call: a 'call' was not seen
 * @trace_end: a 'call' but trace ended
 * @non_call: a branch but not a 'call' to the start of a different symbol
 */
#[repr(C)]
struct thread_stack_entry {
    ret_addr: u64,
    timestamp: u64,
    ref_: u64,
    branch_count: u64,
    insn_count: u64,
    cyc_count: u64,
    db_id: u64,
    cp: *mut call_path,
    no_call: bool,
    trace_end: bool,
    non_call: bool,
}

/**
 * struct thread_stack - thread stack constructed from 'call' and 'return'
 *                       branch samples.
 * @stack: array that holds the stack
 * @cnt: number of entries in the stack
 * @sz: current maximum stack size
 * @trace_nr: current trace number
 * @branch_count: running branch count
 * @insn_count: running  instruction count
 * @cyc_count running  cycle count
 * @kernel_start: kernel start address
 * @last_time: last timestamp
 * @crp: call/return processor
 * @comm: current comm
 * @arr_sz: size of array if this is the first element of an array
 * @rstate: used to detect retpolines
 * @br_stack_rb: branch stack (ring buffer)
 * @br_stack_sz: maximum branch stack size
 * @br_stack_pos: current position in @br_stack_rb
 * @mispred_all: mark all branches as mispredicted
 */
#[repr(C)]
pub struct thread_stack {
    stack: *mut thread_stack_entry,
    cnt: size_t,
    sz: size_t,
    trace_nr: u64,
    branch_count: u64,
    insn_count: u64,
    cyc_count: u64,
    kernel_start: u64,
    last_time: u64,
    crp: *mut call_return_processor,
    comm: *mut comm,
    arr_sz: c_uint,
    rstate: retpoline_state_t,
    br_stack_rb: *mut branch_stack,
    br_stack_sz: c_uint,
    br_stack_pos: c_uint,
    mispred_all: bool,
}

/*
 * Assume pid == tid == 0 identifies the idle task as defined by
 * perf_session__register_idle_thread(). The idle task is really 1 task per cpu,
 * and therefore requires a stack for each cpu.
 */
unsafe fn thread_stack__per_cpu(thread: *mut thread) -> bool {
    !(thread__tid(thread) != 0 || thread__pid(thread) != 0)
}

fn roundup_pow_of_two(mut x: c_uint) -> c_uint {
    if x <= 1 {
        return 1;
    }
    x -= 1;
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    x + 1
}

unsafe fn thread_stack__grow(ts: *mut thread_stack) -> c_int {
    let new_sz = (*ts).sz + STACK_GROWTH;
    let sz = new_sz * size_of::<thread_stack_entry>();

    let new_stack = realloc((*ts).stack as *mut c_void, sz) as *mut thread_stack_entry;
    if new_stack.is_null() {
        return -ENOMEM;
    }

    (*ts).stack = new_stack;
    (*ts).sz = new_sz;

    0
}

unsafe fn thread_stack__init(
    ts: *mut thread_stack,
    thread: *mut thread,
    crp: *mut call_return_processor,
    callstack: bool,
    br_stack_sz: c_uint,
) -> c_int {
    if callstack {
        let err = thread_stack__grow(ts);
        if err != 0 {
            return err;
        }
    }

    if br_stack_sz != 0 {
        let mut sz = size_of::<branch_stack>();

        sz += br_stack_sz as size_t * size_of::<branch_entry>();
        (*ts).br_stack_rb = zalloc(sz) as *mut branch_stack;
        if (*ts).br_stack_rb.is_null() {
            return -ENOMEM;
        }
        (*ts).br_stack_sz = br_stack_sz;
    }

    if !thread__maps(thread).is_null() && !maps__machine(thread__maps(thread)).is_null() {
        let machine = maps__machine(thread__maps(thread));
        let e_machine = thread__e_machine(thread, machine, null_mut());

        (*ts).kernel_start = machine__kernel_start(machine);
        if e_machine == EM_X86_64 || e_machine == EM_386 {
            (*ts).rstate = retpoline_state_t::X86_RETPOLINE_POSSIBLE;
        }
    } else {
        (*ts).kernel_start = 1u64 << 63;
    }
    (*ts).crp = crp;

    0
}

unsafe fn thread_stack__new(
    thread: *mut thread,
    cpu: c_int,
    crp: *mut call_return_processor,
    callstack: bool,
    br_stack_sz: c_uint,
) -> *mut thread_stack {
    let mut ts = thread__ts(thread);
    let old_sz = if !ts.is_null() { (*ts).arr_sz } else { 0 };
    let mut new_sz = 1;

    if thread_stack__per_cpu(thread) && cpu > 0 {
        new_sz = roundup_pow_of_two((cpu + 1) as c_uint);
    }

    if ts.is_null() || new_sz > old_sz {
        let new_ts = calloc(new_sz as size_t, size_of::<thread_stack>()) as *mut thread_stack;
        if new_ts.is_null() {
            return null_mut();
        }
        if !ts.is_null() {
            copy_nonoverlapping(ts, new_ts, old_sz as size_t);
        }
        (*new_ts).arr_sz = new_sz;
        free(thread__ts(thread) as *mut c_void);
        thread__set_ts(thread, new_ts);
        ts = new_ts;
    }

    if thread_stack__per_cpu(thread) && cpu > 0 && (cpu as c_uint) < (*ts).arr_sz {
        ts = ts.add(cpu as usize);
    }

    if (*ts).stack.is_null()
        && thread_stack__init(ts, thread, crp, callstack, br_stack_sz) != 0
    {
        return null_mut();
    }

    ts
}

unsafe fn thread__cpu_stack(thread: *mut thread, mut cpu: c_int) -> *mut thread_stack {
    let mut ts = thread__ts(thread);

    if cpu < 0 {
        cpu = 0;
    }

    if ts.is_null() || (cpu as c_uint) >= (*ts).arr_sz {
        return null_mut();
    }

    ts = ts.add(cpu as usize);

    if (*ts).stack.is_null() {
        return null_mut();
    }

    ts
}

unsafe fn thread__stack(thread: *mut thread, cpu: c_int) -> *mut thread_stack {
    if thread.is_null() {
        return null_mut();
    }

    if thread_stack__per_cpu(thread) {
        return thread__cpu_stack(thread, cpu);
    }

    thread__ts(thread)
}

unsafe fn thread_stack__push(ts: *mut thread_stack, ret_addr: u64, trace_end: bool) -> c_int {
    let mut err = 0;

    if (*ts).cnt == (*ts).sz {
        err = thread_stack__grow(ts);
        if err != 0 {
            pr_warning(c"Out of memory: discarding thread stack\n".as_ptr());
            (*ts).cnt = 0;
        }
    }

    (*(*ts).stack.add((*ts).cnt)).trace_end = trace_end;
    (*(*ts).stack.add((*ts).cnt)).ret_addr = ret_addr;
    (*ts).cnt += 1;

    err
}

unsafe fn thread_stack__pop(ts: *mut thread_stack, ret_addr: u64) {
    /*
     * In some cases there may be functions which are not seen to return.
     * For example when setjmp / longjmp has been used.  Or the perf context
     * switch in the kernel which doesn't stop and start tracing in exactly
     * the same code path.  When that happens the return address will be
     * further down the stack.  If the return address is not found at all,
     * we assume the opposite (i.e. this is a return for a call that wasn't
     * seen for some reason) and leave the stack alone.
     */
    let mut i = (*ts).cnt;
    while i != 0 {
        i -= 1;
        if (*(*ts).stack.add(i)).ret_addr == ret_addr {
            (*ts).cnt = i;
            return;
        }
    }
}

unsafe fn thread_stack__pop_trace_end(ts: *mut thread_stack) {
    let mut i = (*ts).cnt;

    while i != 0 {
        i -= 1;
        if (*(*ts).stack.add(i)).trace_end {
            (*ts).cnt = i;
        } else {
            return;
        }
    }
}

unsafe fn thread_stack__in_kernel(ts: *mut thread_stack) -> bool {
    if (*ts).cnt == 0 {
        return false;
    }

    (*(*(*ts).stack.add((*ts).cnt - 1)).cp).in_kernel
}

unsafe fn thread_stack__call_return(
    thread: *mut thread,
    ts: *mut thread_stack,
    idx: size_t,
    timestamp: u64,
    ref_: u64,
    no_return: bool,
) -> c_int {
    let crp = (*ts).crp;
    let tse = (*ts).stack.add(idx);
    let mut cr = call_return {
        thread,
        comm: (*ts).comm,
        cp: null_mut(),
        call_time: 0,
        return_time: 0,
        branch_count: 0,
        insn_count: 0,
        cyc_count: 0,
        call_ref: 0,
        return_ref: 0,
        db_id: 0,
        flags: 0,
    };
    let parent_db_id: *mut u64;

    cr.cp = (*tse).cp;
    cr.call_time = (*tse).timestamp;
    cr.return_time = timestamp;
    cr.branch_count = (*ts).branch_count - (*tse).branch_count;
    cr.insn_count = (*ts).insn_count - (*tse).insn_count;
    cr.cyc_count = (*ts).cyc_count - (*tse).cyc_count;
    cr.db_id = (*tse).db_id;
    cr.call_ref = (*tse).ref_;
    cr.return_ref = ref_;
    if (*tse).no_call {
        cr.flags |= CALL_RETURN_NO_CALL;
    }
    if no_return {
        cr.flags |= CALL_RETURN_NO_RETURN;
    }
    if (*tse).non_call {
        cr.flags |= CALL_RETURN_NON_CALL;
    }

    /*
     * The parent db_id must be assigned before exporting the child. Note
     * it is not possible to export the parent first because its information
     * is not yet complete because its 'return' has not yet been processed.
     */
    parent_db_id = if idx != 0 {
        &mut (*tse.sub(1)).db_id
    } else {
        null_mut()
    };

    ((*crp).process.unwrap())(&mut cr, parent_db_id, (*crp).data)
}

unsafe fn __thread_stack__flush(thread: *mut thread, ts: *mut thread_stack) -> c_int {
    let crp = (*ts).crp;
    let mut err;

    if crp.is_null() {
        (*ts).cnt = 0;
        (*ts).br_stack_pos = 0;
        if !(*ts).br_stack_rb.is_null() {
            (*(*ts).br_stack_rb).nr = 0;
        }
        return 0;
    }

    while (*ts).cnt != 0 {
        (*ts).cnt -= 1;
        err = thread_stack__call_return(thread, ts, (*ts).cnt, (*ts).last_time, 0, true);
        if err != 0 {
            pr_err(c"Error flushing thread stack!\n".as_ptr());
            (*ts).cnt = 0;
            return err;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn thread_stack__flush(thread: *mut thread) -> c_int {
    let ts = thread__ts(thread);
    let mut err = 0;

    if !ts.is_null() {
        let mut pos = 0;
        while pos < (*ts).arr_sz {
            let ret = __thread_stack__flush(thread, ts.add(pos as usize));

            if ret != 0 {
                err = ret;
            }
            pos += 1;
        }
    }

    err
}

unsafe fn thread_stack__update_br_stack(
    ts: *mut thread_stack,
    flags: u32,
    from_ip: u64,
    to_ip: u64,
) {
    let bs = (*ts).br_stack_rb;

    if (*ts).br_stack_pos == 0 {
        (*ts).br_stack_pos = (*ts).br_stack_sz;
    }

    (*ts).br_stack_pos -= 1;

    let be = (*bs).entries.as_mut_ptr().add((*ts).br_stack_pos as usize);
    (*be).from = from_ip;
    (*be).to = to_ip;
    (*be).flags.value = 0;
    (*be).flags.set_abort((flags & PERF_IP_FLAG_TX_ABORT) != 0);
    (*be).flags.set_in_tx((flags & PERF_IP_FLAG_IN_TX) != 0);
    /* No support for mispredict */
    (*be).flags.set_mispred((*ts).mispred_all);

    if (*bs).nr < (*ts).br_stack_sz as u64 {
        (*bs).nr += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn thread_stack__event(
    thread: *mut thread,
    cpu: c_int,
    flags: u32,
    from_ip: u64,
    to_ip: u64,
    insn_len: u16,
    trace_nr: u64,
    callstack: bool,
    br_stack_sz: c_uint,
    mispred_all: bool,
) -> c_int {
    let mut ts = thread__stack(thread, cpu);

    if thread.is_null() {
        return -EINVAL;
    }

    if ts.is_null() {
        ts = thread_stack__new(thread, cpu, null_mut(), callstack, br_stack_sz);
        if ts.is_null() {
            pr_warning(c"Out of memory: no thread stack\n".as_ptr());
            return -ENOMEM;
        }
        (*ts).trace_nr = trace_nr;
        (*ts).mispred_all = mispred_all;
    }

    /*
     * When the trace is discontinuous, the trace_nr changes.  In that case
     * the stack might be completely invalid.  Better to report nothing than
     * to report something misleading, so flush the stack.
     */
    if trace_nr != (*ts).trace_nr {
        if (*ts).trace_nr != 0 {
            __thread_stack__flush(thread, ts);
        }
        (*ts).trace_nr = trace_nr;
    }

    if br_stack_sz != 0 {
        thread_stack__update_br_stack(ts, flags, from_ip, to_ip);
    }

    /*
     * Stop here if thread_stack__process() is in use, or not recording call
     * stack.
     */
    if !(*ts).crp.is_null() || !callstack {
        return 0;
    }

    if (flags & PERF_IP_FLAG_CALL) != 0 {
        if to_ip == 0 {
            return 0;
        }
        let ret_addr = from_ip + insn_len as u64;
        if ret_addr == to_ip {
            return 0; /* Zero-length calls are excluded */
        }
        return thread_stack__push(ts, ret_addr, (flags & PERF_IP_FLAG_TRACE_END) != 0);
    } else if (flags & PERF_IP_FLAG_TRACE_BEGIN) != 0 {
        /*
         * If the caller did not change the trace number (which would
         * have flushed the stack) then try to make sense of the stack.
         * Possibly, tracing began after returning to the current
         * address, so try to pop that. Also, do not expect a call made
         * when the trace ended, to return, so pop that.
         */
        thread_stack__pop(ts, to_ip);
        thread_stack__pop_trace_end(ts);
    } else if (flags & PERF_IP_FLAG_RETURN) != 0 && from_ip != 0 {
        thread_stack__pop(ts, to_ip);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn thread_stack__set_trace_nr(
    thread: *mut thread,
    cpu: c_int,
    trace_nr: u64,
) {
    let ts = thread__stack(thread, cpu);

    if ts.is_null() {
        return;
    }

    if trace_nr != (*ts).trace_nr {
        if (*ts).trace_nr != 0 {
            __thread_stack__flush(thread, ts);
        }
        (*ts).trace_nr = trace_nr;
    }
}

unsafe fn __thread_stack__free(thread: *mut thread, ts: *mut thread_stack) {
    __thread_stack__flush(thread, ts);
    zfree(&mut (*ts).stack as *mut *mut thread_stack_entry as *mut *mut c_void);
    zfree(&mut (*ts).br_stack_rb as *mut *mut branch_stack as *mut *mut c_void);
}

unsafe fn thread_stack__reset(thread: *mut thread, ts: *mut thread_stack) {
    let arr_sz = (*ts).arr_sz;

    __thread_stack__free(thread, ts);
    memset(ts as *mut c_void, 0, size_of::<thread_stack>());
    (*ts).arr_sz = arr_sz;
}

#[no_mangle]
pub unsafe extern "C" fn thread_stack__free(thread: *mut thread) {
    let ts = thread__ts(thread);

    if !ts.is_null() {
        let mut pos = 0;
        while pos < (*ts).arr_sz {
            __thread_stack__free(thread, ts.add(pos as usize));
            pos += 1;
        }
        free(thread__ts(thread) as *mut c_void);
        thread__set_ts(thread, null_mut());
    }
}

unsafe fn callchain_context(ip: u64, kernel_start: u64) -> u64 {
    if ip < kernel_start {
        PERF_CONTEXT_USER
    } else {
        PERF_CONTEXT_KERNEL
    }
}

#[no_mangle]
pub unsafe extern "C" fn thread_stack__sample(
    thread: *mut thread,
    cpu: c_int,
    chain: *mut ip_callchain,
    sz: size_t,
    mut ip: u64,
    kernel_start: u64,
) {
    let ts = thread__stack(thread, cpu);
    let mut context = callchain_context(ip, kernel_start);
    let mut last_context;
    let mut i;
    let mut j;

    if sz < 2 {
        (*chain).nr = 0;
        return;
    }

    *(*chain).ips.as_mut_ptr().add(0) = context;
    *(*chain).ips.as_mut_ptr().add(1) = ip;

    if ts.is_null() {
        (*chain).nr = 2;
        return;
    }

    last_context = context;

    i = 2;
    j = 1;
    while i < sz && j <= (*ts).cnt {
        ip = (*(*ts).stack.add((*ts).cnt - j)).ret_addr;
        context = callchain_context(ip, kernel_start);
        if context != last_context {
            if i >= sz - 1 {
                break;
            }
            *(*chain).ips.as_mut_ptr().add(i) = context;
            i += 1;
            last_context = context;
        }
        *(*chain).ips.as_mut_ptr().add(i) = ip;
        i += 1;
        j += 1;
    }

    (*chain).nr = i as u64;
}

/*
 * Hardware sample records, created some time after the event occurred, need to
 * have subsequent addresses removed from the call chain.
 */
#[no_mangle]
pub unsafe extern "C" fn thread_stack__sample_late(
    thread: *mut thread,
    cpu: c_int,
    chain: *mut ip_callchain,
    sz: size_t,
    sample_ip: u64,
    kernel_start: u64,
) {
    let ts = thread__stack(thread, cpu);
    let sample_context = callchain_context(sample_ip, kernel_start);
    let mut last_context;
    let mut context;
    let mut ip;
    let mut nr = 0usize;
    let mut j;

    if sz < 2 {
        (*chain).nr = 0;
        return;
    }

    if !ts.is_null() {
        /*
         * When tracing kernel space, kernel addresses occur at the top of the
         * call chain after the event occurred but before tracing stopped.
         * Skip them.
         */
        j = 1;
        while j <= (*ts).cnt {
            ip = (*(*ts).stack.add((*ts).cnt - j)).ret_addr;
            context = callchain_context(ip, kernel_start);
            if context == PERF_CONTEXT_USER || (context == sample_context && ip == sample_ip) {
                break;
            }
            j += 1;
        }

        last_context = sample_ip; /* Use sample_ip as an invalid context */

        while nr < sz && j <= (*ts).cnt {
            ip = (*(*ts).stack.add((*ts).cnt - j)).ret_addr;
            context = callchain_context(ip, kernel_start);
            if context != last_context {
                if nr >= sz - 1 {
                    break;
                }
                *(*chain).ips.as_mut_ptr().add(nr) = context;
                nr += 1;
                last_context = context;
            }
            *(*chain).ips.as_mut_ptr().add(nr) = ip;
            nr += 1;
            j += 1;
        }
    }

    if nr != 0 {
        (*chain).nr = nr as u64;
    } else {
        *(*chain).ips.as_mut_ptr().add(0) = sample_context;
        *(*chain).ips.as_mut_ptr().add(1) = sample_ip;
        (*chain).nr = 2;
    }
}

#[no_mangle]
pub unsafe extern "C" fn thread_stack__br_sample(
    thread: *mut thread,
    cpu: c_int,
    dst: *mut branch_stack,
    mut sz: c_uint,
) {
    let ts = thread__stack(thread, cpu);
    let bsz = size_of::<branch_entry>();
    let src;
    let mut be;
    let mut nr;

    (*dst).nr = 0;

    if ts.is_null() {
        return;
    }

    src = (*ts).br_stack_rb;
    if (*src).nr == 0 {
        return;
    }

    (*dst).nr = core::cmp::min((*src).nr as c_uint, sz) as u64;

    be = (*dst).entries.as_mut_ptr();
    nr = core::cmp::min((*ts).br_stack_sz - (*ts).br_stack_pos, (*dst).nr as c_uint);
    copy_nonoverlapping(
        (*src).entries.as_ptr().add((*ts).br_stack_pos as usize) as *const u8,
        be as *mut u8,
        bsz * nr as usize,
    );

    if (*src).nr >= (*ts).br_stack_sz as u64 {
        sz -= nr;
        be = (*dst).entries.as_mut_ptr().add(nr as usize);
        nr = core::cmp::min((*ts).br_stack_pos, sz);
        copy_nonoverlapping(
            (*src).entries.as_ptr() as *const u8,
            be as *mut u8,
            bsz * nr as usize,
        );
    }
}

/* Start of user space branch entries */
unsafe fn us_start(be: *mut branch_entry, kernel_start: u64, start: *mut bool) -> bool {
    if !*start {
        *start = (*be).to != 0 && (*be).to < kernel_start;
    }

    *start
}

/*
 * Start of branch entries after the ip fell in between 2 branches, or user
 * space branch entries.
 */
unsafe fn ks_start(
    be: *mut branch_entry,
    sample_ip: u64,
    kernel_start: u64,
    start: *mut bool,
    nb: *mut branch_entry,
) -> bool {
    if !*start {
        *start = (!nb.is_null() && sample_ip >= (*be).to && sample_ip <= (*nb).from)
            || (*be).from < kernel_start
            || ((*be).to != 0 && (*be).to < kernel_start);
    }

    *start
}

/*
 * Hardware sample records, created some time after the event occurred, need to
 * have subsequent addresses removed from the branch stack.
 */
#[no_mangle]
pub unsafe extern "C" fn thread_stack__br_sample_late(
    thread: *mut thread,
    cpu: c_int,
    dst: *mut branch_stack,
    sz: c_uint,
    ip: u64,
    kernel_start: u64,
) {
    let ts = thread__stack(thread, cpu);
    let mut d;
    let mut s;
    let spos;
    let ssz;
    let src;
    let mut nr = 0;
    let mut start = false;

    (*dst).nr = 0;

    if ts.is_null() {
        return;
    }

    src = (*ts).br_stack_rb;
    if (*src).nr == 0 {
        return;
    }

    spos = (*src).entries.as_mut_ptr().add((*ts).br_stack_pos as usize);
    ssz = (*src).entries.as_mut_ptr().add((*ts).br_stack_sz as usize);

    d = (*dst).entries.as_mut_ptr();
    s = spos;

    if ip < kernel_start {
        /*
         * User space sample: start copying branch entries when the
         * branch is in user space.
         */
        s = spos;
        while s < ssz && nr < sz {
            if us_start(s, kernel_start, &mut start) {
                *d = *s;
                d = d.add(1);
                nr += 1;
            }
            s = s.add(1);
        }

        if (*src).nr >= (*ts).br_stack_sz as u64 {
            s = (*src).entries.as_mut_ptr();
            while s < spos && nr < sz {
                if us_start(s, kernel_start, &mut start) {
                    *d = *s;
                    d = d.add(1);
                    nr += 1;
                }
                s = s.add(1);
            }
        }
    } else {
        let mut nb: *mut branch_entry = null_mut();

        /*
         * Kernel space sample: start copying branch entries when the ip
         * falls in between 2 branches (or the branch is in user space
         * because then the start must have been missed).
         */
        s = spos;
        while s < ssz && nr < sz {
            if ks_start(s, ip, kernel_start, &mut start, nb) {
                *d = *s;
                d = d.add(1);
                nr += 1;
            }
            nb = s;
            s = s.add(1);
        }

        if (*src).nr >= (*ts).br_stack_sz as u64 {
            s = (*src).entries.as_mut_ptr();
            while s < spos && nr < sz {
                if ks_start(s, ip, kernel_start, &mut start, nb) {
                    *d = *s;
                    d = d.add(1);
                    nr += 1;
                }
                nb = s;
                s = s.add(1);
            }
        }
    }

    (*dst).nr = nr as u64;
}

#[no_mangle]
pub unsafe extern "C" fn call_return_processor__new(
    process: Option<
        unsafe extern "C" fn(
            cr: *mut call_return,
            parent_db_id: *mut u64,
            data: *mut c_void,
        ) -> c_int,
    >,
    data: *mut c_void,
) -> *mut call_return_processor {
    let crp = zalloc(size_of::<call_return_processor>()) as *mut call_return_processor;

    if crp.is_null() {
        return null_mut();
    }
    (*crp).cpr = call_path_root__new();
    if (*crp).cpr.is_null() {
        free(crp as *mut c_void);
        return null_mut();
    }
    (*crp).process = process;
    (*crp).data = data;
    crp
}

#[no_mangle]
pub unsafe extern "C" fn call_return_processor__free(crp: *mut call_return_processor) {
    if !crp.is_null() {
        call_path_root__free((*crp).cpr);
        free(crp as *mut c_void);
    }
}

unsafe fn thread_stack__push_cp(
    ts: *mut thread_stack,
    ret_addr: u64,
    timestamp: u64,
    ref_: u64,
    cp: *mut call_path,
    no_call: bool,
    trace_end: bool,
) -> c_int {
    if cp.is_null() {
        return -ENOMEM;
    }

    if (*ts).cnt == (*ts).sz {
        let err = thread_stack__grow(ts);
        if err != 0 {
            return err;
        }
    }

    let tse = (*ts).stack.add((*ts).cnt);
    (*ts).cnt += 1;
    (*tse).ret_addr = ret_addr;
    (*tse).timestamp = timestamp;
    (*tse).ref_ = ref_;
    (*tse).branch_count = (*ts).branch_count;
    (*tse).insn_count = (*ts).insn_count;
    (*tse).cyc_count = (*ts).cyc_count;
    (*tse).cp = cp;
    (*tse).no_call = no_call;
    (*tse).trace_end = trace_end;
    (*tse).non_call = false;
    (*tse).db_id = 0;

    0
}

unsafe fn thread_stack__pop_cp(
    thread: *mut thread,
    ts: *mut thread_stack,
    ret_addr: u64,
    timestamp: u64,
    ref_: u64,
    sym: *mut symbol,
) -> c_int {
    if (*ts).cnt == 0 {
        return 1;
    }

    if (*ts).cnt == 1 {
        let tse = (*ts).stack.add(0);

        if (*(*tse).cp).sym == sym {
            (*ts).cnt -= 1;
            return thread_stack__call_return(thread, ts, (*ts).cnt, timestamp, ref_, false);
        }
    }

    if (*(*ts).stack.add((*ts).cnt - 1)).ret_addr == ret_addr
        && !(*(*ts).stack.add((*ts).cnt - 1)).non_call
    {
        (*ts).cnt -= 1;
        return thread_stack__call_return(thread, ts, (*ts).cnt, timestamp, ref_, false);
    } else {
        let mut i = (*ts).cnt - 1;

        while {
            let old = i;
            i = i.wrapping_sub(1);
            old != 0
        } {
            if (*(*ts).stack.add(i)).ret_addr != ret_addr || (*(*ts).stack.add(i)).non_call {
                continue;
            }
            i += 1;
            while (*ts).cnt > i {
                (*ts).cnt -= 1;
                let err =
                    thread_stack__call_return(thread, ts, (*ts).cnt, timestamp, ref_, true);
                if err != 0 {
                    return err;
                }
            }
            (*ts).cnt -= 1;
            return thread_stack__call_return(thread, ts, (*ts).cnt, timestamp, ref_, false);
        }
    }

    1
}

unsafe fn thread_stack__bottom(
    ts: *mut thread_stack,
    sample: *mut perf_sample,
    from_al: *mut addr_location,
    to_al: *mut addr_location,
    ref_: u64,
) -> c_int {
    let cpr = (*(*ts).crp).cpr;
    let sym;
    let ip;

    if (*sample).ip != 0 {
        ip = (*sample).ip;
        sym = (*from_al).sym;
    } else if (*sample).addr != 0 {
        ip = (*sample).addr;
        sym = (*to_al).sym;
    } else {
        return 0;
    }

    let cp = call_path__findnew(cpr, &mut (*cpr).call_path, sym, ip, (*ts).kernel_start);

    thread_stack__push_cp(ts, ip, (*sample).time, ref_, cp, true, false)
}

unsafe fn thread_stack__pop_ks(
    thread: *mut thread,
    ts: *mut thread_stack,
    sample: *mut perf_sample,
    ref_: u64,
) -> c_int {
    let tm = (*sample).time;

    /* Return to userspace, so pop all kernel addresses */
    while thread_stack__in_kernel(ts) {
        (*ts).cnt -= 1;
        let err = thread_stack__call_return(thread, ts, (*ts).cnt, tm, ref_, true);
        if err != 0 {
            return err;
        }
    }

    0
}

unsafe fn thread_stack__no_call_return(
    thread: *mut thread,
    ts: *mut thread_stack,
    sample: *mut perf_sample,
    from_al: *mut addr_location,
    to_al: *mut addr_location,
    ref_: u64,
) -> c_int {
    let cpr = (*(*ts).crp).cpr;
    let root = &mut (*cpr).call_path;
    let fsym = (*from_al).sym;
    let tsym = (*to_al).sym;
    let mut cp;
    let parent;
    let ks = (*ts).kernel_start;
    let addr = (*sample).addr;
    let tm = (*sample).time;
    let ip = (*sample).ip;
    let mut err;

    if ip >= ks && addr < ks {
        /* Return to userspace, so pop all kernel addresses */
        err = thread_stack__pop_ks(thread, ts, sample, ref_);
        if err != 0 {
            return err;
        }

        /* If the stack is empty, push the userspace address */
        if (*ts).cnt == 0 {
            cp = call_path__findnew(cpr, root, tsym, addr, ks);
            return thread_stack__push_cp(ts, 0, tm, ref_, cp, true, false);
        }
    } else if thread_stack__in_kernel(ts) && ip < ks {
        /* Return to userspace, so pop all kernel addresses */
        err = thread_stack__pop_ks(thread, ts, sample, ref_);
        if err != 0 {
            return err;
        }
    }

    if (*ts).cnt != 0 {
        parent = (*(*ts).stack.add((*ts).cnt - 1)).cp;
    } else {
        parent = root;
    }

    if (*parent).sym == (*from_al).sym {
        /*
         * At the bottom of the stack, assume the missing 'call' was
         * before the trace started. So, pop the current symbol and push
         * the 'to' symbol.
         */
        if (*ts).cnt == 1 {
            (*ts).cnt -= 1;
            err = thread_stack__call_return(thread, ts, (*ts).cnt, tm, ref_, false);
            if err != 0 {
                return err;
            }
        }

        if (*ts).cnt == 0 {
            cp = call_path__findnew(cpr, root, tsym, addr, ks);

            return thread_stack__push_cp(ts, addr, tm, ref_, cp, true, false);
        }

        /*
         * Otherwise assume the 'return' is being used as a jump (e.g.
         * retpoline) and just push the 'to' symbol.
         */
        cp = call_path__findnew(cpr, parent, tsym, addr, ks);

        err = thread_stack__push_cp(ts, 0, tm, ref_, cp, true, false);
        if err == 0 {
            (*(*ts).stack.add((*ts).cnt - 1)).non_call = true;
        }

        return err;
    }

    /*
     * Assume 'parent' has not yet returned, so push 'to', and then push and
     * pop 'from'.
     */

    cp = call_path__findnew(cpr, parent, tsym, addr, ks);

    err = thread_stack__push_cp(ts, addr, tm, ref_, cp, true, false);
    if err != 0 {
        return err;
    }

    cp = call_path__findnew(cpr, cp, fsym, ip, ks);

    err = thread_stack__push_cp(ts, ip, tm, ref_, cp, true, false);
    if err != 0 {
        return err;
    }

    (*ts).cnt -= 1;
    thread_stack__call_return(thread, ts, (*ts).cnt, tm, ref_, false)
}

unsafe fn thread_stack__trace_begin(
    thread: *mut thread,
    ts: *mut thread_stack,
    timestamp: u64,
    ref_: u64,
) -> c_int {
    if (*ts).cnt == 0 {
        return 0;
    }

    /* Pop trace end */
    let tse = (*ts).stack.add((*ts).cnt - 1);
    if (*tse).trace_end {
        (*ts).cnt -= 1;
        let err = thread_stack__call_return(thread, ts, (*ts).cnt, timestamp, ref_, false);
        if err != 0 {
            return err;
        }
    }

    0
}

unsafe fn thread_stack__trace_end(
    ts: *mut thread_stack,
    sample: *mut perf_sample,
    ref_: u64,
) -> c_int {
    let cpr = (*(*ts).crp).cpr;
    let ret_addr;

    /* No point having 'trace end' on the bottom of the stack */
    if (*ts).cnt == 0 || ((*ts).cnt == 1 && (*(*ts).stack.add(0)).ref_ == ref_) {
        return 0;
    }

    let cp = call_path__findnew(
        cpr,
        (*(*ts).stack.add((*ts).cnt - 1)).cp,
        null_mut(),
        0,
        (*ts).kernel_start,
    );

    ret_addr = (*sample).ip + (*sample).insn_len as u64;

    thread_stack__push_cp(ts, ret_addr, (*sample).time, ref_, cp, false, true)
}

unsafe fn is_x86_retpoline(name: *const c_char) -> bool {
    strstr(name, c"__x86_indirect_thunk_".as_ptr()) == name as *mut c_char
}

/*
 * x86 retpoline functions pollute the call graph. This function removes them.
 * This does not handle function return thunks, nor is there any improvement
 * for the handling of inline thunks or extern thunks.
 */
unsafe fn thread_stack__x86_retpoline(
    ts: *mut thread_stack,
    sample: *mut perf_sample,
    to_al: *mut addr_location,
) -> c_int {
    let tse = (*ts).stack.add((*ts).cnt - 1);
    let cpr = (*(*ts).crp).cpr;
    let mut sym = (*(*tse).cp).sym;
    let tsym = (*to_al).sym;
    let cp;

    if !sym.is_null() && is_x86_retpoline((*sym).name) {
        /*
         * This is a x86 retpoline fn. It pollutes the call graph by
         * showing up everywhere there is an indirect branch, but does
         * not itself mean anything. Here the top-of-stack is removed,
         * by decrementing the stack count, and then further down, the
         * resulting top-of-stack is replaced with the actual target.
         * The result is that the retpoline functions will no longer
         * appear in the call graph. Note this only affects the call
         * graph, since all the original branches are left unchanged.
         */
        (*ts).cnt -= 1;
        sym = (*(*(*ts).stack.add((*ts).cnt - 2)).cp).sym;
        if !sym.is_null() && sym == tsym && (*to_al).addr != (*tsym).start {
            /*
             * Target is back to the middle of the symbol we came
             * from so assume it is an indirect jmp and forget it
             * altogether.
             */
            (*ts).cnt -= 1;
            return 0;
        }
    } else if !sym.is_null() && sym == tsym {
        /*
         * Target is back to the symbol we came from so assume it is an
         * indirect jmp and forget it altogether.
         */
        (*ts).cnt -= 1;
        return 0;
    }

    cp = call_path__findnew(
        cpr,
        (*(*ts).stack.add((*ts).cnt - 2)).cp,
        tsym,
        (*sample).addr,
        (*ts).kernel_start,
    );
    if cp.is_null() {
        return -ENOMEM;
    }

    /* Replace the top-of-stack with the actual target */
    (*(*ts).stack.add((*ts).cnt - 1)).cp = cp;

    0
}

#[no_mangle]
pub unsafe extern "C" fn thread_stack__process(
    thread: *mut thread,
    comm: *mut comm,
    sample: *mut perf_sample,
    from_al: *mut addr_location,
    to_al: *mut addr_location,
    ref_: u64,
    crp: *mut call_return_processor,
) -> c_int {
    let mut ts = thread__stack(thread, (*sample).cpu as c_int);
    let rstate;
    let mut err = 0;

    if !ts.is_null() && (*ts).crp.is_null() {
        /* Supersede thread_stack__event() */
        thread_stack__reset(thread, ts);
        ts = null_mut();
    }

    if ts.is_null() {
        ts = thread_stack__new(thread, (*sample).cpu as c_int, crp, true, 0);
        if ts.is_null() {
            return -ENOMEM;
        }
        (*ts).comm = comm;
    }

    rstate = (*ts).rstate;
    if rstate == retpoline_state_t::X86_RETPOLINE_DETECTED {
        (*ts).rstate = retpoline_state_t::X86_RETPOLINE_POSSIBLE;
    }

    /* Flush stack on exec */
    if (*ts).comm != comm && thread__pid(thread) == thread__tid(thread) {
        err = __thread_stack__flush(thread, ts);
        if err != 0 {
            return err;
        }
        (*ts).comm = comm;
    }

    /* If the stack is empty, put the current symbol on the stack */
    if (*ts).cnt == 0 {
        err = thread_stack__bottom(ts, sample, from_al, to_al, ref_);
        if err != 0 {
            return err;
        }
    }

    (*ts).branch_count += 1;
    (*ts).insn_count += (*sample).insn_cnt;
    (*ts).cyc_count += (*sample).cyc_cnt;
    (*ts).last_time = (*sample).time;

    if ((*sample).flags & PERF_IP_FLAG_CALL) != 0 {
        let trace_end = ((*sample).flags & PERF_IP_FLAG_TRACE_END) != 0;
        let cpr = (*(*ts).crp).cpr;
        let cp;
        let ret_addr;

        if (*sample).ip == 0 || (*sample).addr == 0 {
            return 0;
        }

        ret_addr = (*sample).ip + (*sample).insn_len as u64;
        if ret_addr == (*sample).addr {
            return 0; /* Zero-length calls are excluded */
        }

        cp = call_path__findnew(
            cpr,
            (*(*ts).stack.add((*ts).cnt - 1)).cp,
            (*to_al).sym,
            (*sample).addr,
            (*ts).kernel_start,
        );
        err = thread_stack__push_cp(ts, ret_addr, (*sample).time, ref_, cp, false, trace_end);

        /*
         * A call to the same symbol but not the start of the symbol,
         * may be the start of a x86 retpoline.
         */
        if err == 0
            && rstate == retpoline_state_t::X86_RETPOLINE_POSSIBLE
            && !(*to_al).sym.is_null()
            && (*from_al).sym == (*to_al).sym
            && (*to_al).addr != (*(*to_al).sym).start
        {
            (*ts).rstate = retpoline_state_t::X86_RETPOLINE_DETECTED;
        }
    } else if ((*sample).flags & PERF_IP_FLAG_RETURN) != 0 {
        if (*sample).addr == 0 {
            let return_from_kernel: u32 = PERF_IP_FLAG_SYSCALLRET | PERF_IP_FLAG_INTERRUPT;

            if ((*sample).flags & return_from_kernel) == 0 {
                return 0;
            }

            /* Pop kernel stack */
            return thread_stack__pop_ks(thread, ts, sample, ref_);
        }

        if (*sample).ip == 0 {
            return 0;
        }

        /* x86 retpoline 'return' doesn't match the stack */
        if rstate == retpoline_state_t::X86_RETPOLINE_DETECTED
            && (*ts).cnt > 2
            && (*(*ts).stack.add((*ts).cnt - 1)).ret_addr != (*sample).addr
        {
            return thread_stack__x86_retpoline(ts, sample, to_al);
        }

        err = thread_stack__pop_cp(thread, ts, (*sample).addr, (*sample).time, ref_, (*from_al).sym);
        if err != 0 {
            if err < 0 {
                return err;
            }
            err = thread_stack__no_call_return(thread, ts, sample, from_al, to_al, ref_);
        }
    } else if ((*sample).flags & PERF_IP_FLAG_TRACE_BEGIN) != 0 {
        err = thread_stack__trace_begin(thread, ts, (*sample).time, ref_);
    } else if ((*sample).flags & PERF_IP_FLAG_TRACE_END) != 0 {
        err = thread_stack__trace_end(ts, sample, ref_);
    } else if ((*sample).flags & PERF_IP_FLAG_BRANCH) != 0
        && (*from_al).sym != (*to_al).sym
        && !(*to_al).sym.is_null()
        && (*to_al).addr == (*(*to_al).sym).start
    {
        let cpr = (*(*ts).crp).cpr;
        let cp;

        /*
         * The compiler might optimize a call/ret combination by making
         * it a jmp. Make that visible by recording on the stack a
         * branch to the start of a different symbol. Note, that means
         * when a ret pops the stack, all jmps must be popped off first.
         */
        cp = call_path__findnew(
            cpr,
            (*(*ts).stack.add((*ts).cnt - 1)).cp,
            (*to_al).sym,
            (*sample).addr,
            (*ts).kernel_start,
        );
        err = thread_stack__push_cp(ts, 0, (*sample).time, ref_, cp, false, false);
        if err == 0 {
            (*(*ts).stack.add((*ts).cnt - 1)).non_call = true;
        }
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn thread_stack__depth(thread: *mut thread, cpu: c_int) -> size_t {
    let ts = thread__stack(thread, cpu);

    if ts.is_null() {
        return 0;
    }
    (*ts).cnt
}
