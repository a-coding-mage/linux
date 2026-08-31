/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/util/threads.h.
// C includes "hashmap.h" and "rwsem.h"; the types `hashmap` and
// `rw_semaphore` are expected to be supplied by those translated dependencies.

use core::ffi::{c_int, c_void};

pub const THREADS__TABLE_BITS: usize = 3;
pub const THREADS__TABLE_SIZE: usize = 1usize << THREADS__TABLE_BITS;

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct threads_table_entry {
    /* Key is tid, value is struct thread. */
    pub shard: hashmap,
    pub lock: rw_semaphore,
    pub last_match: *mut thread,
}

#[repr(C)]
pub struct threads {
    pub table: [threads_table_entry; THREADS__TABLE_SIZE],
}

unsafe extern "C" {
    pub fn threads__init(threads: *mut threads);
    pub fn threads__exit(threads: *mut threads);
    pub fn threads__nr(threads: *mut threads) -> usize;
    pub fn threads__find(threads: *mut threads, tid: pid_t) -> *mut thread;
    pub fn threads__findnew(
        threads: *mut threads,
        pid: pid_t,
        tid: pid_t,
        created: *mut bool,
    ) -> *mut thread;
    pub fn threads__remove_all_threads(threads: *mut threads);
    pub fn threads__remove(threads: *mut threads, thread: *mut thread);
    pub fn threads__for_each_thread(
        threads: *mut threads,
        fn_: Option<unsafe extern "C" fn(thread: *mut thread, data: *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;
}
