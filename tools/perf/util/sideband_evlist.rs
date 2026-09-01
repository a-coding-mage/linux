// SPDX-License-Identifier: GPL-2.0-only

// Translated from perf/util/sideband_evlist.c.
// C includes referenced external perf, linux, poll, pthread, sched, and limits
// declarations supplied elsewhere in the original build.

use core::ffi::{c_int, c_uint, c_void};
use core::ptr;

const CLONE_FS: c_int = 0x0000_0200;
const POLLERR: c_int = 0x0008;
const POLLHUP: c_int = 0x0010;
const UINT_MAX: c_uint = c_uint::MAX;

type pthread_t = usize;
type evsel__sb_cb_t = Option<unsafe extern "C" fn(event: *mut perf_event, data: *mut c_void)>;

#[repr(C)]
pub struct perf_event_attr {
    pub sample_id_all: c_uint,
    pub watermark: c_uint,
    pub wakeup_watermark: c_uint,
}

#[repr(C)]
pub struct perf_evsel {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel_side_band {
    pub cb: evsel__sb_cb_t,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct evsel {
    pub core: perf_evsel,
    pub side_band: evsel_side_band,
}

#[repr(C)]
pub struct evlist_core {
    pub nr_mmaps: c_int,
    pub user_requested_cpus: *mut c_void,
    pub threads: *mut c_void,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct target {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mmap_core {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mmap {
    pub core: mmap_core,
}

#[repr(C)]
pub union perf_event {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn pr_warning(fmt: *const i8, ...);
    fn unshare(flags: c_int) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn evsel__new_idx(attr: *mut perf_event_attr, idx: c_int) -> *mut evsel;
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__add(evlist: *mut evlist, evsel: *mut evsel);
    fn evlist__sb_thread_done(evlist: *mut evlist) -> bool;
    fn evlist__poll(evlist: *mut evlist, timeout: c_int) -> c_int;
    fn evlist__filter_pollfd(evlist: *mut evlist, revents: c_int);
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__mmap(evlist: *mut evlist) -> *mut mmap;
    fn evlist__event2evsel(evlist: *mut evlist, event: *mut perf_event) -> *mut evsel;
    fn evlist__set_cb_first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__set_cb_next(evlist: *mut evlist, evsel: *mut evsel) -> *mut evsel;
    fn evlist__start_sb_thread_first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__start_sb_thread_next(evlist: *mut evlist, evsel: *mut evsel) -> *mut evsel;
    fn evlist__create_maps(evlist: *mut evlist, target: *mut target) -> c_int;
    fn perf_can_sample_identifier() -> bool;
    fn evsel__set_sample_id(counter: *mut evsel, can_sample_identifier: bool);
    fn evlist__set_id_pos(evlist: *mut evlist);
    fn evsel__open(counter: *mut evsel, cpus: *mut c_void, threads: *mut c_void) -> c_int;
    fn evlist__do_mmap(evlist: *mut evlist, pages: c_uint) -> c_int;
    fn evsel__enable(counter: *mut evsel) -> c_int;
    fn evlist__set_sb_thread_done(evlist: *mut evlist, done: c_int);
    fn evlist__sb_thread_th(evlist: *mut evlist) -> *mut pthread_t;
    fn evlist__put(evlist: *mut evlist);

    fn perf_mmap__read_init(map: *mut mmap_core) -> c_int;
    fn perf_mmap__read_event(map: *mut mmap_core) -> *mut perf_event;
    fn perf_mmap__consume(map: *mut mmap_core);
    fn perf_mmap__read_done(map: *mut mmap_core);
}

pub unsafe extern "C" fn evlist__add_sb_event(
    evlist: *mut evlist,
    attr: *mut perf_event_attr,
    cb: evsel__sb_cb_t,
    data: *mut c_void,
) -> c_int {
    let evsel: *mut evsel;

    if unsafe { (*attr).sample_id_all } == 0 {
        unsafe {
            pr_warning(c"enabling sample_id_all for all side band events\n".as_ptr());
            (*attr).sample_id_all = 1;
        }
    }

    evsel = unsafe { evsel__new_idx(attr, evlist__nr_entries(evlist)) };
    if evsel.is_null() {
        return -1;
    }

    unsafe {
        (*evsel).side_band.cb = cb;
        (*evsel).side_band.data = data;
        evlist__add(evlist, evsel);
    }
    0
}

unsafe extern "C" fn perf_evlist__poll_thread(arg: *mut c_void) -> *mut c_void {
    let evlist = arg as *mut evlist;
    let mut draining = false;
    let mut i: c_int;
    let done = 0;
    /*
     * In order to read symbols from other namespaces perf to needs to call
     * setns(2).  This isn't permitted if the struct_fs has multiple users.
     * unshare(2) the fs so that we may continue to setns into namespaces
     * that we're observing when, for instance, reading the build-ids at
     * the end of a 'perf record' session.
     */
    unsafe {
        unshare(CLONE_FS);
    }

    while done == 0 {
        let mut got_data = false;

        if unsafe { evlist__sb_thread_done(evlist) } {
            draining = true;
        }

        if !draining {
            unsafe {
                evlist__poll(evlist, 1000);
            }
        }

        /*
         * When a thread of the monitored target exits, its per-cpu
         * ring-buffer fd is closed and starts returning POLLHUP. Such
         * dead fds are never requested for POLLIN, but poll() reports
         * POLLHUP/POLLERR unconditionally, so leaving them in the
         * pollfd array makes the following evlist__poll() return
         * immediately forever, spinning this thread at 100% CPU.
         *
         * Filter them out here, mirroring what the 'perf record' main
         * loop does after fdarray__poll().
         */
        unsafe {
            evlist__filter_pollfd(evlist, POLLERR | POLLHUP);
        }

        i = 0;
        while i < unsafe { (*evlist__core(evlist)).nr_mmaps } {
            let map = unsafe { evlist__mmap(evlist).offset(i as isize) };
            let mut event: *mut perf_event;

            if unsafe { perf_mmap__read_init(&mut (*map).core) } != 0 {
                i += 1;
                continue;
            }
            loop {
                event = unsafe { perf_mmap__read_event(&mut (*map).core) };
                if event.is_null() {
                    break;
                }
                let evsel = unsafe { evlist__event2evsel(evlist, event) };

                if !evsel.is_null() && unsafe { (*evsel).side_band.cb.is_some() } {
                    unsafe {
                        ((*evsel).side_band.cb.unwrap())(event, (*evsel).side_band.data);
                    }
                } else {
                    unsafe {
                        pr_warning(
                            c"cannot locate proper evsel for the side band event\n".as_ptr(),
                        );
                    }
                }

                unsafe {
                    perf_mmap__consume(&mut (*map).core);
                }
                got_data = true;
            }
            unsafe {
                perf_mmap__read_done(&mut (*map).core);
            }
            i += 1;
        }

        if draining && !got_data {
            break;
        }
    }
    ptr::null_mut()
}

pub unsafe extern "C" fn evlist__set_cb(
    evlist: *mut evlist,
    cb: evsel__sb_cb_t,
    data: *mut c_void,
) {
    let mut evsel: *mut evsel;

    // Original C uses evlist__for_each_entry(evlist, evsel).
    evsel = unsafe { evlist__set_cb_first(evlist) };
    while !evsel.is_null() {
        unsafe {
            (*evsel).core.attr.sample_id_all = 1;
            (*evsel).core.attr.watermark = 1;
            (*evsel).core.attr.wakeup_watermark = 1;
            (*evsel).side_band.cb = cb;
            (*evsel).side_band.data = data;
        }
        evsel = unsafe { evlist__set_cb_next(evlist, evsel) };
    }
}

pub unsafe extern "C" fn evlist__start_sb_thread(
    evlist: *mut evlist,
    target: *mut target,
) -> c_int {
    let mut counter: *mut evsel;

    if evlist.is_null() {
        return 0;
    }

    if unsafe { evlist__create_maps(evlist, target) } != 0 {
        unsafe {
            evlist__put(evlist);
        }
        return -1;
    }

    if unsafe { evlist__nr_entries(evlist) } > 1 {
        let can_sample_identifier = unsafe { perf_can_sample_identifier() };

        // Original C uses evlist__for_each_entry(evlist, counter).
        counter = unsafe { evlist__start_sb_thread_first(evlist) };
        while !counter.is_null() {
            unsafe {
                evsel__set_sample_id(counter, can_sample_identifier);
            }
            counter = unsafe { evlist__start_sb_thread_next(evlist, counter) };
        }

        unsafe {
            evlist__set_id_pos(evlist);
        }
    }

    // Original C uses evlist__for_each_entry(evlist, counter).
    counter = unsafe { evlist__start_sb_thread_first(evlist) };
    while !counter.is_null() {
        if unsafe {
            evsel__open(
                counter,
                (*evlist__core(evlist)).user_requested_cpus,
                (*evlist__core(evlist)).threads,
            )
        } < 0
        {
            unsafe {
                evlist__put(evlist);
            }
            return -1;
        }
        counter = unsafe { evlist__start_sb_thread_next(evlist, counter) };
    }

    if unsafe { evlist__do_mmap(evlist, UINT_MAX) } != 0 {
        unsafe {
            evlist__put(evlist);
        }
        return -1;
    }

    // Original C uses evlist__for_each_entry(evlist, counter).
    counter = unsafe { evlist__start_sb_thread_first(evlist) };
    while !counter.is_null() {
        if unsafe { evsel__enable(counter) } != 0 {
            unsafe {
                evlist__put(evlist);
            }
            return -1;
        }
        counter = unsafe { evlist__start_sb_thread_next(evlist, counter) };
    }

    unsafe {
        evlist__set_sb_thread_done(evlist, 0);
    }
    if unsafe {
        pthread_create(
            evlist__sb_thread_th(evlist),
            ptr::null(),
            perf_evlist__poll_thread,
            evlist as *mut c_void,
        )
    } != 0
    {
        unsafe {
            evlist__put(evlist);
        }
        return -1;
    }

    0
}

pub unsafe extern "C" fn evlist__stop_sb_thread(evlist: *mut evlist) {
    if evlist.is_null() {
        return;
    }
    unsafe {
        evlist__set_sb_thread_done(evlist, 1);
        pthread_join(*evlist__sb_thread_th(evlist), ptr::null_mut());
        evlist__put(evlist);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
