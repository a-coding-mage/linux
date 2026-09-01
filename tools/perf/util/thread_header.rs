// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/thread.h.
// C include dependencies are represented here as external/opaque C-compatible
// types or declarations where this header only referenced them.

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr;

pub type pid_t = c_int;
pub type u8 = u8;
pub type u64 = u64;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type size_t = usize;
pub type FILE = c_void;

#[repr(C)]
pub struct addr_location {
    _private: [u8; 0],
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
pub struct perf_record_namespaces {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread_stack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct unwind_libunwind_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct namespaces {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct intlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct callchain_cursor_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct srccode_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol_conf {
    pub comm_list: *mut strlist,
    pub pid_list: *mut intlist,
    pub tid_list: *mut intlist,
}

unsafe extern "C" {
    pub static mut symbol_conf: symbol_conf;

    pub fn list_empty(head: *const list_head) -> c_int;
    pub fn strlist__has_entry(list: *mut strlist, entry: *const c_char) -> c_int;
    pub fn intlist__has_entry(list: *mut intlist, i: c_int) -> c_int;
}

#[repr(C)]
pub struct lbr_stitch {
    pub lists: list_head,
    pub free_lists: list_head,
    pub prev_sample: perf_sample,
    pub prev_lbr_cursor: *mut callchain_cursor_node,
    pub prev_lbr_cursor_size: c_uint,
}

#[repr(C)]
pub struct thread {
    /// @maps: mmaps associated with this thread.
    pub maps: *mut maps,
    pub pid_: pid_t, /* Not all tools update this */
    /// @tid: thread ID number unique to a machine.
    pub tid: pid_t,
    /// @ppid: parent process of the process this thread belongs to.
    pub ppid: pid_t,
    pub cpu: c_int,
    pub guest_cpu: c_int, /* For QEMU thread */
    pub refcnt: refcount_t,
    /// @exited: Has the thread had an exit event. Such threads are usually
    /// removed from the machine's threads but some events/tools require
    /// access to dead threads.
    pub exited: bool,
    pub comm_set: bool,
    pub comm_len: c_int,
    pub namespaces_list: list_head,
    pub namespaces_lock: rw_semaphore,
    pub comm_list: list_head,
    pub comm_lock: rw_semaphore,
    pub db_id: u64,

    pub priv_: *mut c_void,
    pub ts: *mut thread_stack,
    pub nsinfo: *mut nsinfo,
    pub srccode_state: srccode_state,
    pub filter: bool,
    pub filter_entry_depth: c_int,
    /// @e_flags: The ELF EF_* associated with the thread. Valid if e_machine != EM_NONE.
    pub e_flags: uint16_t,
    /// @e_machine: The ELF EM_* associated with the thread. EM_NONE if not
    /// computed.
    pub e_machine: uint16_t,
    /// @e_is_big_endian: True if the ELF architecture of the thread is big endian.
    /// Valid if e_machine != EM_NONE.
    pub e_is_big_endian: bool,
    /* LBR call stack stitch */
    pub lbr_stitch_enable: bool,
    pub lbr_stitch: *mut lbr_stitch,
}

unsafe extern "C" {
    pub fn thread__new(pid: pid_t, tid: pid_t) -> *mut thread;
    pub fn thread__init_maps(thread: *mut thread, machine: *mut machine) -> c_int;
    pub fn thread__delete(thread: *mut thread);

    pub fn thread__set_priv_destructor(destructor: Option<unsafe extern "C" fn(priv_: *mut c_void)>);

    pub fn thread__get(thread: *mut thread) -> *mut thread;
    pub fn thread__put(thread: *mut thread);

    pub fn thread__namespaces(thread: *mut thread) -> *mut namespaces;
    pub fn thread__set_namespaces(
        thread: *mut thread,
        timestamp: u64,
        event: *mut perf_record_namespaces,
    ) -> c_int;

    pub fn __thread__set_comm(
        thread: *mut thread,
        comm: *const c_char,
        timestamp: u64,
        exec: bool,
    ) -> c_int;

    pub fn thread__set_comm_from_proc(thread: *mut thread) -> c_int;

    pub fn thread__comm_len(thread: *mut thread) -> c_int;
    pub fn thread__comm(thread: *mut thread) -> *mut comm;
    pub fn thread__exec_comm(thread: *mut thread) -> *mut comm;
    pub fn thread__comm_str(thread: *mut thread) -> *const c_char;
    pub fn thread__insert_map(thread: *mut thread, map: *mut map) -> c_int;
    pub fn thread__fork(
        thread: *mut thread,
        parent: *mut thread,
        timestamp: u64,
        do_maps_clone: bool,
    ) -> c_int;
    pub fn thread__fprintf(thread: *mut thread, fp: *mut FILE) -> size_t;

    pub fn thread__main_thread(machine: *mut machine, thread: *mut thread) -> *mut thread;

    pub fn thread__find_map(
        thread: *mut thread,
        cpumode: u8,
        addr: u64,
        al: *mut addr_location,
    ) -> *mut map;
    pub fn thread__find_map_fb(
        thread: *mut thread,
        cpumode: u8,
        addr: u64,
        al: *mut addr_location,
    ) -> *mut map;

    pub fn thread__find_symbol(
        thread: *mut thread,
        cpumode: u8,
        addr: u64,
        al: *mut addr_location,
    ) -> *mut symbol;
    pub fn thread__find_symbol_fb(
        thread: *mut thread,
        cpumode: u8,
        addr: u64,
        al: *mut addr_location,
    ) -> *mut symbol;

    pub fn thread__find_cpumode_addr_location(
        thread: *mut thread,
        addr: u64,
        symbols: bool,
        al: *mut addr_location,
    );

    pub fn thread__memcpy(
        thread: *mut thread,
        machine: *mut machine,
        buf: *mut c_void,
        ip: u64,
        len: c_int,
        is64bit: *mut bool,
    ) -> c_int;

    pub fn thread__e_machine_endian(
        thread: *mut thread,
        machine: *mut machine,
        e_flags: *mut uint32_t,
        is_big_endian: *mut bool,
    ) -> uint16_t;

    pub fn thread__free_stitch_list(thread: *mut thread);

    pub fn thread__resolve(thread: *mut thread, al: *mut addr_location, sample: *mut perf_sample);
}

#[inline]
pub unsafe fn __thread__zput(thread: *mut *mut thread) {
    unsafe {
        thread__put(*thread);
        *thread = ptr::null_mut();
    }
}

/* C macro: #define thread__zput(thread) __thread__zput(&thread) */
#[inline]
pub unsafe fn thread__zput(thread: *mut *mut thread) {
    unsafe { __thread__zput(thread) }
}

#[inline]
pub unsafe fn thread__set_comm(thread: *mut thread, comm: *const c_char, timestamp: u64) -> c_int {
    unsafe { __thread__set_comm(thread, comm, timestamp, false) }
}

#[inline]
pub unsafe fn thread__maps(thread: *mut thread) -> *mut maps {
    unsafe { (*thread).maps }
}

#[inline]
pub unsafe fn thread__set_maps(thread: *mut thread, maps: *mut maps) {
    unsafe {
        (*thread).maps = maps;
    }
}

#[inline]
pub unsafe fn thread__pid(thread: *const thread) -> pid_t {
    unsafe { (*thread).pid_ }
}

#[inline]
pub unsafe fn thread__set_pid(thread: *mut thread, pid_: pid_t) {
    unsafe {
        (*thread).pid_ = pid_;
    }
}

#[inline]
pub unsafe fn thread__tid(thread: *const thread) -> pid_t {
    unsafe { (*thread).tid }
}

#[inline]
pub unsafe fn thread__set_tid(thread: *mut thread, tid: pid_t) {
    unsafe {
        (*thread).tid = tid;
    }
}

#[inline]
pub unsafe fn thread__ppid(thread: *const thread) -> pid_t {
    unsafe { (*thread).ppid }
}

#[inline]
pub unsafe fn thread__set_ppid(thread: *mut thread, ppid: pid_t) {
    unsafe {
        (*thread).ppid = ppid;
    }
}

#[inline]
pub unsafe fn thread__cpu(thread: *const thread) -> c_int {
    unsafe { (*thread).cpu }
}

#[inline]
pub unsafe fn thread__set_cpu(thread: *mut thread, cpu: c_int) {
    unsafe {
        (*thread).cpu = cpu;
    }
}

#[inline]
pub unsafe fn thread__guest_cpu(thread: *const thread) -> c_int {
    unsafe { (*thread).guest_cpu }
}

#[inline]
pub unsafe fn thread__set_guest_cpu(thread: *mut thread, guest_cpu: c_int) {
    unsafe {
        (*thread).guest_cpu = guest_cpu;
    }
}

#[inline]
pub unsafe fn thread__refcnt(thread: *mut thread) -> *mut refcount_t {
    unsafe { &mut (*thread).refcnt }
}

#[inline]
pub unsafe fn thread__set_exited(thread: *mut thread, exited: bool) {
    unsafe {
        (*thread).exited = exited;
    }
}

#[inline]
pub unsafe fn thread__comm_set(thread: *const thread) -> bool {
    unsafe { (*thread).comm_set }
}

#[inline]
pub unsafe fn thread__set_comm_set(thread: *mut thread, set: bool) {
    unsafe {
        (*thread).comm_set = set;
    }
}

#[inline]
pub unsafe fn thread__var_comm_len(thread: *const thread) -> c_int {
    unsafe { (*thread).comm_len }
}

#[inline]
pub unsafe fn thread__set_comm_len(thread: *mut thread, len: c_int) {
    unsafe {
        (*thread).comm_len = len;
    }
}

#[inline]
pub unsafe fn thread__namespaces_list(thread: *mut thread) -> *mut list_head {
    unsafe { &mut (*thread).namespaces_list }
}

#[inline]
pub unsafe fn thread__namespaces_list_empty(thread: *const thread) -> c_int {
    unsafe { list_empty(&(*thread).namespaces_list) }
}

#[inline]
pub unsafe fn thread__namespaces_lock(thread: *mut thread) -> *mut rw_semaphore {
    unsafe { &mut (*thread).namespaces_lock }
}

#[inline]
pub unsafe fn thread__comm_lock(thread: *mut thread) -> *mut rw_semaphore {
    unsafe { &mut (*thread).comm_lock }
}

// C annotation preserved: SHARED_LOCKS_REQUIRED(thread__comm_lock(thread))
#[inline]
pub unsafe fn thread__comm_list(thread: *mut thread) -> *mut list_head {
    unsafe { &mut (*thread).comm_list }
}

#[inline]
pub unsafe fn thread__db_id(thread: *const thread) -> u64 {
    unsafe { (*thread).db_id }
}

#[inline]
pub unsafe fn thread__set_db_id(thread: *mut thread, db_id: u64) {
    unsafe {
        (*thread).db_id = db_id;
    }
}

#[inline]
pub unsafe fn thread__priv(thread: *mut thread) -> *mut c_void {
    unsafe { (*thread).priv_ }
}

#[inline]
pub unsafe fn thread__set_priv(thread: *mut thread, p: *mut c_void) {
    unsafe {
        (*thread).priv_ = p;
    }
}

#[inline]
pub unsafe fn thread__ts(thread: *mut thread) -> *mut thread_stack {
    unsafe { (*thread).ts }
}

#[inline]
pub unsafe fn thread__set_ts(thread: *mut thread, ts: *mut thread_stack) {
    unsafe {
        (*thread).ts = ts;
    }
}

#[inline]
pub unsafe fn thread__nsinfo(thread: *mut thread) -> *mut nsinfo {
    unsafe { (*thread).nsinfo }
}

#[inline]
pub unsafe fn thread__srccode_state(thread: *mut thread) -> *mut srccode_state {
    unsafe { &mut (*thread).srccode_state }
}

#[inline]
pub unsafe fn thread__filter(thread: *const thread) -> bool {
    unsafe { (*thread).filter }
}

#[inline]
pub unsafe fn thread__set_filter(thread: *mut thread, filter: bool) {
    unsafe {
        (*thread).filter = filter;
    }
}

#[inline]
pub unsafe fn thread__filter_entry_depth(thread: *const thread) -> c_int {
    unsafe { (*thread).filter_entry_depth }
}

#[inline]
pub unsafe fn thread__set_filter_entry_depth(thread: *mut thread, depth: c_int) {
    unsafe {
        (*thread).filter_entry_depth = depth;
    }
}

#[inline]
pub unsafe fn thread__e_machine(
    thread: *mut thread,
    machine: *mut machine,
    e_flags: *mut uint32_t,
) -> uint16_t {
    unsafe { thread__e_machine_endian(thread, machine, e_flags, ptr::null_mut()) }
}

#[inline]
pub unsafe fn thread__set_e_machine(thread: *mut thread, e_machine: uint16_t) {
    unsafe {
        (*thread).e_machine = e_machine;
    }
}

#[inline]
pub unsafe fn thread__e_flags(thread: *const thread) -> uint32_t {
    unsafe { (*thread).e_flags as uint32_t }
}

#[inline]
pub unsafe fn thread__set_e_flags(thread: *mut thread, e_flags: uint32_t) {
    unsafe {
        (*thread).e_flags = e_flags as uint16_t;
    }
}

#[inline]
pub unsafe fn thread__e_is_big_endian(thread: *const thread) -> bool {
    unsafe { (*thread).e_is_big_endian }
}

#[inline]
pub unsafe fn thread__set_e_is_big_endian(thread: *mut thread, is_big_endian: bool) {
    unsafe {
        (*thread).e_is_big_endian = is_big_endian;
    }
}

#[inline]
pub unsafe fn thread__lbr_stitch_enable(thread: *const thread) -> bool {
    unsafe { (*thread).lbr_stitch_enable }
}

#[inline]
pub unsafe fn thread__set_lbr_stitch_enable(thread: *mut thread, en: bool) {
    unsafe {
        (*thread).lbr_stitch_enable = en;
    }
}

#[inline]
pub unsafe fn thread__lbr_stitch(thread: *mut thread) -> *mut lbr_stitch {
    unsafe { (*thread).lbr_stitch }
}

#[inline]
pub unsafe fn thread__set_lbr_stitch(thread: *mut thread, lbrs: *mut lbr_stitch) {
    unsafe {
        (*thread).lbr_stitch = lbrs;
    }
}

#[inline]
pub unsafe fn thread__is_filtered(thread: *mut thread) -> bool {
    unsafe {
        if !symbol_conf.comm_list.is_null()
            && strlist__has_entry(symbol_conf.comm_list, thread__comm_str(thread)) == 0
        {
            return true;
        }

        if !symbol_conf.pid_list.is_null()
            && intlist__has_entry(symbol_conf.pid_list, thread__pid(thread)) == 0
        {
            return true;
        }

        if !symbol_conf.tid_list.is_null()
            && intlist__has_entry(symbol_conf.tid_list, thread__tid(thread)) == 0
        {
            return true;
        }

        false
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
