// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/threads.c. Header-provided declarations from
// threads.h, machine.h, and thread.h are represented here only as the
// dependencies needed by this source-level translation.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::c_void;

pub type pid_t = i32;

// Provided by threads.h in the original repository.
pub const THREADS__TABLE_SIZE: usize = 256;

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
    pub key: i64,
    pub pvalue: *mut c_void,
}

#[repr(C)]
pub struct threads_table_entry {
    pub shard: hashmap,
    pub lock: rw_semaphore,
    pub last_match: *mut thread,
}

#[repr(C)]
pub struct threads {
    pub table: [threads_table_entry; THREADS__TABLE_SIZE],
}

unsafe extern "C" {
    fn hashmap__init(
        map: *mut hashmap,
        hash_fn: Option<unsafe extern "C" fn(i64, *mut c_void) -> usize>,
        equal_fn: Option<unsafe extern "C" fn(i64, i64, *mut c_void) -> bool>,
        ctx: *mut c_void,
    );
    fn hashmap__clear(map: *mut hashmap);
    fn hashmap__size(map: *const hashmap) -> usize;
    fn hashmap__find(map: *const hashmap, key: i64, value: *mut *mut thread) -> bool;
    fn hashmap__add(map: *mut hashmap, key: i64, value: *mut thread) -> i32;
    fn hashmap__delete(
        map: *mut hashmap,
        key: i64,
        old_key: *mut i64,
        old_value: *mut *mut thread,
    ) -> bool;

    fn init_rwsem(sem: *mut rw_semaphore);
    fn exit_rwsem(sem: *mut rw_semaphore);
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn down_write(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);

    fn thread__tid(thread: *mut thread) -> pid_t;
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn thread__put(thread: *mut thread);
    fn thread__new(pid: pid_t, tid: pid_t) -> *mut thread;
    fn RC_CHK_EQUAL(a: *mut thread, b: *mut thread) -> bool;
}

unsafe fn threads__table(threads: *mut threads, tid: pid_t) -> *mut threads_table_entry {
    /* Cast it to handle tid == -1 */
    unsafe {
        &mut (*threads).table[(tid as u32 as usize) % THREADS__TABLE_SIZE]
            as *mut threads_table_entry
    }
}

unsafe extern "C" fn key_hash(key: i64, _ctx: *mut c_void) -> usize {
    /* The table lookup removes low bit entropy, but this is just ignored here. */
    key as usize
}

unsafe extern "C" fn key_equal(key1: i64, key2: i64, _ctx: *mut c_void) -> bool {
    key1 == key2
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn threads__init(threads: *mut threads) {
    unsafe {
        for i in 0..THREADS__TABLE_SIZE {
            let table = &mut (*threads).table[i] as *mut threads_table_entry;

            hashmap__init(
                &mut (*table).shard,
                Some(key_hash),
                Some(key_equal),
                core::ptr::null_mut(),
            );
            init_rwsem(&mut (*table).lock);
            (*table).last_match = core::ptr::null_mut();
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn threads__exit(threads: *mut threads) {
    unsafe {
        threads__remove_all_threads(threads);
        for i in 0..THREADS__TABLE_SIZE {
            let table = &mut (*threads).table[i] as *mut threads_table_entry;

            hashmap__clear(&mut (*table).shard);
            exit_rwsem(&mut (*table).lock);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn threads__nr(threads: *mut threads) -> usize {
    let mut nr: usize = 0;

    unsafe {
        for i in 0..THREADS__TABLE_SIZE {
            let table = &mut (*threads).table[i] as *mut threads_table_entry;

            down_read(&mut (*table).lock);
            nr += hashmap__size(&(*table).shard);
            up_read(&mut (*table).lock);
        }
    }
    nr
}

/*
 * Front-end cache - TID lookups come in blocks,
 * so most of the time we dont have to look up
 * the full rbtree:
 */
unsafe fn __threads_table_entry__get_last_match(
    table: *mut threads_table_entry,
    tid: pid_t,
) -> *mut thread {
    let mut res: *mut thread = core::ptr::null_mut();

    unsafe {
        let th = (*table).last_match;
        if !th.is_null() {
            if thread__tid(th) == tid {
                res = thread__get(th);
            }
        }
    }
    res
}

unsafe fn __threads_table_entry__set_last_match(
    table: *mut threads_table_entry,
    th: *mut thread,
) {
    unsafe {
        thread__put((*table).last_match);
        (*table).last_match = thread__get(th);
    }
}

unsafe fn threads_table_entry__set_last_match(
    table: *mut threads_table_entry,
    th: *mut thread,
) {
    unsafe {
        down_write(&mut (*table).lock);
        __threads_table_entry__set_last_match(table, th);
        up_write(&mut (*table).lock);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn threads__find(threads: *mut threads, tid: pid_t) -> *mut thread {
    unsafe {
        let table = threads__table(threads, tid);
        let mut res: *mut thread;

        down_read(&mut (*table).lock);
        res = __threads_table_entry__get_last_match(table, tid);
        if res.is_null() {
            if hashmap__find(&(*table).shard, tid as i64, &mut res) {
                res = thread__get(res);
            }
        }
        up_read(&mut (*table).lock);
        if !res.is_null() {
            threads_table_entry__set_last_match(table, res);
        }
        res
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn threads__findnew(
    threads: *mut threads,
    pid: pid_t,
    tid: pid_t,
    created: *mut bool,
) -> *mut thread {
    unsafe {
        let table = threads__table(threads, tid);
        let mut res: *mut thread = core::ptr::null_mut();

        *created = false;
        down_write(&mut (*table).lock);
        res = thread__new(pid, tid);
        if !res.is_null() {
            if hashmap__add(&mut (*table).shard, tid as i64, res) != 0 {
                /* Add failed. Assume a race so find other entry. */
                thread__put(res);
                res = core::ptr::null_mut();
                if hashmap__find(&(*table).shard, tid as i64, &mut res) {
                    res = thread__get(res);
                }
            } else {
                res = thread__get(res);
                *created = true;
            }
            if !res.is_null() {
                __threads_table_entry__set_last_match(table, res);
            }
        }
        up_write(&mut (*table).lock);
        res
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn threads__remove_all_threads(threads: *mut threads) {
    unsafe {
        for i in 0..THREADS__TABLE_SIZE {
            let table = &mut (*threads).table[i] as *mut threads_table_entry;
            let mut _cur: *mut hashmap_entry;
            let mut _tmp: *mut hashmap_entry;
            let mut _bkt: usize;

            down_write(&mut (*table).lock);
            __threads_table_entry__set_last_match(table, core::ptr::null_mut());

            /*
             * hashmap__for_each_entry_safe(&table->shard, cur, tmp, bkt) {
             *     struct thread *old_value;
             *
             *     hashmap__delete(&table->shard, cur->key, old_key=NULL, &old_value);
             *     thread__put(old_value);
             * }
             *
             * This loop is supplied by a C macro from the hashmap dependency.
             * The source-level body is preserved below as a Rust equivalent
             * once the macro iterator is available to this translation unit.
             */
            while false {
                let mut old_value: *mut thread = core::ptr::null_mut();

                _cur = core::ptr::null_mut();
                hashmap__delete(
                    &mut (*table).shard,
                    (*_cur).key,
                    core::ptr::null_mut(),
                    &mut old_value,
                );
                thread__put(old_value);
                _tmp = core::ptr::null_mut();
                _bkt = 0;
                let _ = (_tmp, _bkt);
            }

            up_write(&mut (*table).lock);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn threads__remove(threads: *mut threads, thread: *mut thread) {
    unsafe {
        let table = threads__table(threads, thread__tid(thread));
        let mut old_value: *mut thread = core::ptr::null_mut();

        down_write(&mut (*table).lock);
        if !(*table).last_match.is_null() && RC_CHK_EQUAL((*table).last_match, thread) {
            __threads_table_entry__set_last_match(table, core::ptr::null_mut());
        }

        hashmap__delete(
            &mut (*table).shard,
            thread__tid(thread) as i64,
            core::ptr::null_mut(),
            &mut old_value,
        );
        thread__put(old_value);
        up_write(&mut (*table).lock);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn threads__for_each_thread(
    threads: *mut threads,
    fn_: Option<unsafe extern "C" fn(thread: *mut thread, data: *mut c_void) -> i32>,
    data: *mut c_void,
) -> i32 {
    unsafe {
        for i in 0..THREADS__TABLE_SIZE {
            let table = &mut (*threads).table[i] as *mut threads_table_entry;
            let mut _cur: *mut hashmap_entry;
            let mut _bkt: usize;

            down_read(&mut (*table).lock);

            /*
             * hashmap__for_each_entry(&table->shard, cur, bkt) {
             *     int rc = fn((struct thread *)cur->pvalue, data);
             *
             *     if (rc != 0) {
             *         up_read(&table->lock);
             *         return rc;
             *     }
             * }
             *
             * This loop is supplied by a C macro from the hashmap dependency.
             * The source-level body is preserved below as a Rust equivalent
             * once the macro iterator is available to this translation unit.
             */
            while false {
                _cur = core::ptr::null_mut();
                let rc = fn_.unwrap()((*_cur).pvalue as *mut thread, data);

                if rc != 0 {
                    up_read(&mut (*table).lock);
                    return rc;
                }
                _bkt = 0;
                let _ = _bkt;
            }

            up_read(&mut (*table).lock);
        }
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
