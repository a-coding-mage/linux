// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/thread-maps-share.c.
// Original C dependencies: "tests.h", "machine.h", "thread.h", "debug.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machines {
    pub host: machine,
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn machines__init(machines: *mut machines) -> c_int;
    fn machines__exit(machines: *mut machines);
    fn machine__findnew_thread(machine: *mut machine, pid: c_int, tid: c_int) -> *mut thread;
    fn machine__find_thread(machine: *mut machine, pid: c_int, tid: c_int) -> *mut thread;
    fn machine__remove_thread(machine: *mut machine, thread: *mut thread);
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn maps__refcnt(maps: *mut maps) -> *mut refcount_t;
    fn maps__equal(a: *mut maps, b: *mut maps) -> bool;
    fn refcount_read(r: *const refcount_t) -> c_int;
}

unsafe fn TEST_ASSERT_VAL(msg: *const c_char, cond: bool) -> c_int {
    if !cond {
        eprintln!(
            "{}",
            unsafe { core::ffi::CStr::from_ptr(msg) }.to_string_lossy()
        );
        return -1;
    }
    0
}

unsafe fn TEST_ASSERT_EQUAL(msg: *const c_char, lhs: c_int, rhs: c_int) -> c_int {
    if lhs != rhs {
        eprintln!(
            "{}",
            unsafe { core::ffi::CStr::from_ptr(msg) }.to_string_lossy()
        );
        return -1;
    }
    0
}

unsafe extern "C" fn test__thread_maps_share(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut machines: machines = unsafe { core::mem::zeroed() };
    let machine: *mut machine;

    /* thread group */
    let leader: *mut thread;
    let t1: *mut thread;
    let t2: *mut thread;
    let t3: *mut thread;
    let maps: *mut maps;

    /* other process */
    let other: *mut thread;
    let other_leader: *mut thread;
    let other_maps: *mut maps;

    /*
     * This test create 2 processes abstractions (struct thread)
     * with several threads and checks they properly share and
     * maintain maps info (struct maps).
     *
     * thread group (pid: 0, tids: 0, 1, 2, 3)
     * other  group (pid: 4, tids: 4, 5)
     */

    if unsafe {
        TEST_ASSERT_VAL(
            c"failed to init machines".as_ptr(),
            machines__init(&mut machines) == 0,
        )
    } != 0
    {
        return -1;
    }
    machine = &mut machines.host;

    /* create process with 4 threads */
    leader = unsafe { machine__findnew_thread(machine, 0, 0) };
    t1 = unsafe { machine__findnew_thread(machine, 0, 1) };
    t2 = unsafe { machine__findnew_thread(machine, 0, 2) };
    t3 = unsafe { machine__findnew_thread(machine, 0, 3) };

    /* and create 1 separated process, without thread leader */
    other = unsafe { machine__findnew_thread(machine, 4, 5) };

    if unsafe {
        TEST_ASSERT_VAL(
            c"failed to create threads".as_ptr(),
            !leader.is_null() && !t1.is_null() && !t2.is_null() && !t3.is_null() && !other.is_null(),
        )
    } != 0
    {
        return -1;
    }

    maps = unsafe { thread__maps(leader) };
    if unsafe {
        TEST_ASSERT_EQUAL(
            c"wrong refcnt".as_ptr(),
            refcount_read(maps__refcnt(maps)),
            4,
        )
    } != 0
    {
        return -1;
    }

    /* test the maps pointer is shared */
    if unsafe {
        TEST_ASSERT_VAL(
            c"maps don't match".as_ptr(),
            maps__equal(maps, thread__maps(t1)),
        )
    } != 0
    {
        return -1;
    }
    if unsafe {
        TEST_ASSERT_VAL(
            c"maps don't match".as_ptr(),
            maps__equal(maps, thread__maps(t2)),
        )
    } != 0
    {
        return -1;
    }
    if unsafe {
        TEST_ASSERT_VAL(
            c"maps don't match".as_ptr(),
            maps__equal(maps, thread__maps(t3)),
        )
    } != 0
    {
        return -1;
    }

    /*
     * Verify the other leader was created by previous call.
     * It should have shared maps with no change in
     * refcnt.
     */
    other_leader = unsafe { machine__find_thread(machine, 4, 4) };
    if unsafe { TEST_ASSERT_VAL(c"failed to find other leader".as_ptr(), !other_leader.is_null()) }
        != 0
    {
        return -1;
    }

    /*
     * Ok, now that all the rbtree related operations were done,
     * lets remove all of them from there so that we can do the
     * refcounting tests.
     */
    unsafe { machine__remove_thread(machine, leader) };
    unsafe { machine__remove_thread(machine, t1) };
    unsafe { machine__remove_thread(machine, t2) };
    unsafe { machine__remove_thread(machine, t3) };
    unsafe { machine__remove_thread(machine, other) };
    unsafe { machine__remove_thread(machine, other_leader) };

    other_maps = unsafe { thread__maps(other) };
    if unsafe {
        TEST_ASSERT_EQUAL(
            c"wrong refcnt".as_ptr(),
            refcount_read(maps__refcnt(other_maps)),
            2,
        )
    } != 0
    {
        return -1;
    }

    if unsafe {
        TEST_ASSERT_VAL(
            c"maps don't match".as_ptr(),
            maps__equal(other_maps, thread__maps(other_leader)),
        )
    } != 0
    {
        return -1;
    }

    /* release thread group */
    unsafe { thread__put(t3) };
    if unsafe {
        TEST_ASSERT_EQUAL(
            c"wrong refcnt".as_ptr(),
            refcount_read(maps__refcnt(maps)),
            3,
        )
    } != 0
    {
        return -1;
    }

    unsafe { thread__put(t2) };
    if unsafe {
        TEST_ASSERT_EQUAL(
            c"wrong refcnt".as_ptr(),
            refcount_read(maps__refcnt(maps)),
            2,
        )
    } != 0
    {
        return -1;
    }

    unsafe { thread__put(t1) };
    if unsafe {
        TEST_ASSERT_EQUAL(
            c"wrong refcnt".as_ptr(),
            refcount_read(maps__refcnt(maps)),
            1,
        )
    } != 0
    {
        return -1;
    }

    unsafe { thread__put(leader) };

    /* release other group  */
    unsafe { thread__put(other_leader) };
    if unsafe {
        TEST_ASSERT_EQUAL(
            c"wrong refcnt".as_ptr(),
            refcount_read(maps__refcnt(other_maps)),
            1,
        )
    } != 0
    {
        return -1;
    }

    unsafe { thread__put(other) };

    unsafe { machines__exit(&mut machines) };
    0
}

unsafe extern "C" {
    fn thread__put(thread: *mut thread);
}

/*
 * DEFINE_SUITE("Share thread maps", thread_maps_share);
 *
 * The original C macro declares/registers this test suite using the local perf
 * test harness. Preserve that external registration intent for the Rust port.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
