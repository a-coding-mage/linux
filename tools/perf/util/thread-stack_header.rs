/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * thread-stack.h: Synthesize a thread's stack using call / return events
 * Copyright (c) 2014, Intel Corporation.
 */

use core::ffi::{c_int, c_uint, c_void};

pub type size_t = usize;
pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip_callchain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct addr_location {
    _private: [u8; 0],
}

#[repr(C)]
pub struct call_path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct call_path_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct branch_stack {
    _private: [u8; 0],
}

/*
 * Call/Return flags.
 *
 * CALL_RETURN_NO_CALL: 'return' but no matching 'call'
 * CALL_RETURN_NO_RETURN: 'call' but no matching 'return'
 * CALL_RETURN_NON_CALL: a branch but not a 'call' to the start of a different
 *                       symbol
 */
pub const CALL_RETURN_NO_CALL: c_uint = 1 << 0;
pub const CALL_RETURN_NO_RETURN: c_uint = 1 << 1;
pub const CALL_RETURN_NON_CALL: c_uint = 1 << 2;

/**
 * struct call_return - paired call/return information.
 * @thread: thread in which call/return occurred
 * @comm: comm in which call/return occurred
 * @cp: call path
 * @call_time: timestamp of call (if known)
 * @return_time: timestamp of return (if known)
 * @branch_count: number of branches seen between call and return
 * @insn_count: approx. number of instructions between call and return
 * @cyc_count: approx. number of cycles between call and return
 * @call_ref: external reference to 'call' sample (e.g. db_id)
 * @return_ref:  external reference to 'return' sample (e.g. db_id)
 * @db_id: id used for db-export
 * @parent_db_id: id of parent call used for db-export
 * @flags: Call/Return flags
 */
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
    pub parent_db_id: u64,
    pub flags: u32,
}

/**
 * struct call_return_processor - provides a call-back to consume call-return
 *                                information.
 * @cpr: call path root
 * @process: call-back that accepts call/return information
 * @data: anonymous data for call-back
 */
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

unsafe extern "C" {
    pub fn thread_stack__event(
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
    ) -> c_int;

    pub fn thread_stack__set_trace_nr(thread: *mut thread, cpu: c_int, trace_nr: u64);

    pub fn thread_stack__sample(
        thread: *mut thread,
        cpu: c_int,
        chain: *mut ip_callchain,
        sz: size_t,
        ip: u64,
        kernel_start: u64,
    );

    pub fn thread_stack__sample_late(
        thread: *mut thread,
        cpu: c_int,
        chain: *mut ip_callchain,
        sz: size_t,
        ip: u64,
        kernel_start: u64,
    );

    pub fn thread_stack__br_sample(
        thread: *mut thread,
        cpu: c_int,
        dst: *mut branch_stack,
        sz: c_uint,
    );

    pub fn thread_stack__br_sample_late(
        thread: *mut thread,
        cpu: c_int,
        dst: *mut branch_stack,
        sz: c_uint,
        sample_ip: u64,
        kernel_start: u64,
    );

    pub fn thread_stack__flush(thread: *mut thread) -> c_int;
    pub fn thread_stack__free(thread: *mut thread);
    pub fn thread_stack__depth(thread: *mut thread, cpu: c_int) -> size_t;

    pub fn call_return_processor__new(
        process: Option<
            unsafe extern "C" fn(
                cr: *mut call_return,
                parent_db_id: *mut u64,
                data: *mut c_void,
            ) -> c_int,
        >,
        data: *mut c_void,
    ) -> *mut call_return_processor;

    pub fn call_return_processor__free(crp: *mut call_return_processor);

    pub fn thread_stack__process(
        thread: *mut thread,
        comm: *mut comm,
        sample: *mut perf_sample,
        from_al: *mut addr_location,
        to_al: *mut addr_location,
        ref_: u64,
        crp: *mut call_return_processor,
    ) -> c_int;
}
