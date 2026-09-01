/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * db-export.h: Support for exporting data suitable for import to a database
 * Copyright (c) 2014, Intel Corporation.
 */

/* Translated from a C header. Original includes:
 * <linux/types.h>
 * <linux/list.h>
 */

use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comm {
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
pub struct call_return_processor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct call_path_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct call_path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct call_return {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _private: [u8; 0],
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
    pub offset: u64, /* ip offset from symbol start */
    pub addr_dso_db_id: u64,
    pub addr_sym_db_id: u64,
    pub addr_offset: u64, /* addr offset from symbol start */
    pub call_path_id: u64,
}

#[repr(C)]
pub struct db_export {
    pub export_evsel: Option<unsafe extern "C" fn(dbe: *mut db_export, evsel: *mut evsel) -> c_int>,
    pub export_machine:
        Option<unsafe extern "C" fn(dbe: *mut db_export, machine: *mut machine) -> c_int>,
    pub export_thread: Option<
        unsafe extern "C" fn(
            dbe: *mut db_export,
            thread: *mut thread,
            main_thread_db_id: u64,
            machine: *mut machine,
        ) -> c_int,
    >,
    pub export_comm: Option<
        unsafe extern "C" fn(dbe: *mut db_export, comm: *mut comm, thread: *mut thread) -> c_int,
    >,
    pub export_comm_thread: Option<
        unsafe extern "C" fn(
            dbe: *mut db_export,
            db_id: u64,
            comm: *mut comm,
            thread: *mut thread,
        ) -> c_int,
    >,
    pub export_dso: Option<
        unsafe extern "C" fn(dbe: *mut db_export, dso: *mut dso, machine: *mut machine) -> c_int,
    >,
    pub export_symbol: Option<
        unsafe extern "C" fn(dbe: *mut db_export, sym: *mut symbol, dso: *mut dso) -> c_int,
    >,
    pub export_branch_type: Option<
        unsafe extern "C" fn(dbe: *mut db_export, branch_type: u32, name: *const c_char) -> c_int,
    >,
    pub export_sample:
        Option<unsafe extern "C" fn(dbe: *mut db_export, es: *mut export_sample) -> c_int>,
    pub export_call_path:
        Option<unsafe extern "C" fn(dbe: *mut db_export, cp: *mut call_path) -> c_int>,
    pub export_call_return:
        Option<unsafe extern "C" fn(dbe: *mut db_export, cr: *mut call_return) -> c_int>,
    pub export_context_switch: Option<
        unsafe extern "C" fn(
            dbe: *mut db_export,
            db_id: u64,
            machine: *mut machine,
            sample: *mut perf_sample,
            th_out_id: u64,
            comm_out_id: u64,
            th_in_id: u64,
            comm_in_id: u64,
            flags: c_int,
        ) -> c_int,
    >,
    pub crp: *mut call_return_processor,
    pub cpr: *mut call_path_root,
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
}

extern "C" {
    pub fn db_export__init(dbe: *mut db_export) -> c_int;
    pub fn db_export__exit(dbe: *mut db_export);
    pub fn db_export__evsel(dbe: *mut db_export, evsel: *mut evsel) -> c_int;
    pub fn db_export__machine(dbe: *mut db_export, machine: *mut machine) -> c_int;
    pub fn db_export__thread(
        dbe: *mut db_export,
        thread: *mut thread,
        machine: *mut machine,
        main_thread: *mut thread,
    ) -> c_int;
    pub fn db_export__comm(
        dbe: *mut db_export,
        comm: *mut comm,
        thread: *mut thread,
    ) -> c_int;
    pub fn db_export__exec_comm(
        dbe: *mut db_export,
        comm: *mut comm,
        main_thread: *mut thread,
    ) -> c_int;
    pub fn db_export__comm_thread(
        dbe: *mut db_export,
        comm: *mut comm,
        thread: *mut thread,
    ) -> c_int;
    pub fn db_export__dso(dbe: *mut db_export, dso: *mut dso, machine: *mut machine) -> c_int;
    pub fn db_export__symbol(dbe: *mut db_export, sym: *mut symbol, dso: *mut dso) -> c_int;
    pub fn db_export__branch_type(
        dbe: *mut db_export,
        branch_type: u32,
        name: *const c_char,
    ) -> c_int;
    pub fn db_export__sample(
        dbe: *mut db_export,
        event: *mut perf_event,
        sample: *mut perf_sample,
        al: *mut addr_location,
        addr_al: *mut addr_location,
    ) -> c_int;

    pub fn db_export__branch_types(dbe: *mut db_export) -> c_int;

    pub fn db_export__call_path(dbe: *mut db_export, cp: *mut call_path) -> c_int;
    pub fn db_export__call_return(
        dbe: *mut db_export,
        cr: *mut call_return,
        parent_db_id: *mut u64,
    ) -> c_int;
    pub fn db_export__switch(
        dbe: *mut db_export,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
