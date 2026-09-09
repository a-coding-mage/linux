// SPDX-License-Identifier: GPL-2.0
/*
 * memcg_event_listener.c - Simple listener of memcg memory.events
 *
 * Copyright (c) 2023, SaluteDevices. All Rights Reserved.
 *
 * Author: Dmitry Rokosov <ddrokosov@salutedevices.com>
 */

use std::ffi::{c_char, c_int, c_long, c_void, CStr};
use std::mem::{size_of, zeroed};
use std::ptr;

const INOTIFY_BUFFER_SIZE: usize = 8192;
const PATH_MAX: usize = 4096;
const IN_MODIFY: u32 = 0x0000_0002;
const POLLIN: i16 = 0x0001;
const POLLERR: i16 = 0x0008;
const EAGAIN: i32 = 11;
const EBADF: i32 = 9;
const ERANGE: i32 = 34;
const EINVAL: i32 = 22;
const EIO: i32 = 5;
const EMSGSIZE: i32 = 90;
const EMFILE: i32 = 24;
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

#[repr(C)]
struct InotifyEvent {
    wd: c_int,
    mask: u32,
    cookie: u32,
    len: u32,
}

#[repr(C)]
struct PollFd { fd: c_int, events: i16, revents: i16 }

#[repr(C)]
struct MemcgCounters {
    low: c_long,
    high: c_long,
    max: c_long,
    oom: c_long,
    oom_kill: c_long,
    oom_group_kill: c_long,
}

#[repr(C)]
struct MemcgEvents {
    counters: MemcgCounters,
    path: [c_char; PATH_MAX],
    inotify_fd: c_int,
    inotify_wd: c_int,
}

extern "C" {
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(stream: *mut c_void) -> c_int;
    fn getline(line: *mut *mut c_char, len: *mut usize, stream: *mut c_void) -> isize;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn strtol(s: *const c_char, end: *mut *mut c_char, base: c_int) -> c_long;
    fn __errno_location() -> *mut c_int;
    fn warn(fmt: *const c_char, ...);
    fn warnx(fmt: *const c_char, ...);
    fn err(status: c_int, fmt: *const c_char, ... ) -> !;
    fn errx(status: c_int, fmt: *const c_char, ... ) -> !;
    fn printf(fmt: *const c_char, ... ) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ... ) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn poll(fds: *mut PollFd, nfds: usize, timeout: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn inotify_init() -> c_int;
    fn inotify_add_watch(fd: c_int, path: *const c_char, mask: u32) -> c_int;
    fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int;
}

unsafe fn print_memcg_counters(c: *const MemcgCounters) {
    printf(b"MEMCG events:\n\0".as_ptr() as _);
    printf(b"\tlow: %ld\n\0".as_ptr() as _, (*c).low);
    printf(b"\thigh: %ld\n\0".as_ptr() as _, (*c).high);
    printf(b"\tmax: %ld\n\0".as_ptr() as _, (*c).max);
    printf(b"\toom: %ld\n\0".as_ptr() as _, (*c).oom);
    printf(b"\toom_kill: %ld\n\0".as_ptr() as _, (*c).oom_kill);
    printf(b"\toom_group_kill: %ld\n\0".as_ptr() as _, (*c).oom_group_kill);
}

unsafe fn get_memcg_counter(line: *mut c_char, name: *const c_char, counter: *mut c_long) -> c_int {
    let mut len = strlen(name);
    if libc_memcmp(line as *const u8, name as *const u8, len) != 0 {
        warnx(b"Counter line %s has wrong name, %s is expected\0".as_ptr() as _, line, name);
        return -EINVAL;
    }
    len += 1;
    *__errno_location() = 0;
    let mut end = ptr::null_mut();
    let tmp = strtol(line.add(len), &mut end, 10);
    if ((tmp == c_long::MAX || tmp == c_long::MIN) && *__errno_location() == ERANGE)
        || (*__errno_location() != 0 && tmp == 0) {
        warnx(b"Failed to parse: %s\n\0".as_ptr() as _, line.add(len));
        return -ERANGE;
    }
    if end == line.add(len) {
        warnx(b"Not digits were found in line %s\n\0".as_ptr() as _, line.add(len));
        return -EINVAL;
    }
    if *end != 0 && !(*end == b'\n' as c_char && { end = end.add(1); *end == 0 }) {
        warnx(b"Further characters after number: %s\n\0".as_ptr() as _, end);
        return -EINVAL;
    }
    *counter = tmp;
    0
}

unsafe fn libc_memcmp(a: *const u8, b: *const u8, n: usize) -> c_int {
    extern "C" { fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int; }
    memcmp(a as _, b as _, n)
}

unsafe fn read_memcg_events(events: *mut MemcgEvents, show_diff: bool) -> c_int {
    let mode = b"re\0";
    let fp = fopen((*events).path.as_ptr(), mode.as_ptr() as _);
    if fp.is_null() { warn(b"Failed to open memcg events file %s\0".as_ptr() as _, (*events).path.as_ptr()); return -EBADF; }
    let names = [b"low\0", b"high\0", b"max\0", b"oom\0", b"oom_kill\0", b"oom_group_kill\0"];
    let fields: [*mut c_long; 6] = [
        &mut (*events).counters.low, &mut (*events).counters.high, &mut (*events).counters.max,
        &mut (*events).counters.oom, &mut (*events).counters.oom_kill, &mut (*events).counters.oom_group_kill];
    let mut old = [0 as c_long; 6];
    for i in 0..6 { old[i] = *fields[i]; }
    let mut line = ptr::null_mut(); let mut cap = 0usize; let mut ret = 0;
    for i in 0..6 {
        *__errno_location() = 0;
        if getline(&mut line, &mut cap, fp) == -1 { if *__errno_location() != 0 { warn(b"Failed to read line for counter %s\0".as_ptr() as _, names[i].as_ptr()); ret = -EIO; } break; }
        ret = get_memcg_counter(line, names[i].as_ptr() as _, fields[i]);
        if ret != 0 { warnx(b"Failed to get counter value from line %s\0".as_ptr() as _, line); break; }
    }
    if ret == 0 {
        let mut any = false;
        for i in 0..6 { if *fields[i] > old[i] { let diff = *fields[i] - old[i]; if show_diff { printf(b"*** %ld MEMCG %s event%s, change counter %ld => %ld\n\0".as_ptr() as _, diff, names[i].as_ptr(), if diff == 1 { b"\0".as_ptr() } else { b"s\0".as_ptr() }, old[i], *fields[i]); } old[i] += diff; any = true; } }
        if show_diff && !any { printf(b"*** No new untracked memcg events available\n\0".as_ptr() as _); }
    }
    free(line as _); fclose(fp); ret
}

unsafe fn process_memcg_events(events: *mut MemcgEvents, event: *const InotifyEvent) {
    if (*events).inotify_wd != (*event).wd { warnx(b"Unknown inotify event %d, should be %d\0".as_ptr() as _, (*event).wd, (*events).inotify_wd); return; }
    printf(b"Received event in %s:\n\0".as_ptr() as _, (*events).path.as_ptr());
    if (*event).mask & IN_MODIFY == 0 { warnx(b"No IN_MODIFY event, skip it\0".as_ptr() as _); return; }
    if read_memcg_events(events, true) != 0 { warnx(b"Can't read memcg events\0".as_ptr() as _); }
}

unsafe fn monitor_events(events: *mut MemcgEvents) -> ! {
    printf(b"Started monitoring memory events from '%s'...\n\0".as_ptr() as _, (*events).path.as_ptr());
    let mut fd = PollFd { fd: (*events).inotify_fd, events: POLLIN, revents: 0 };
    loop {
        let ret = poll(&mut fd, 1, -1);
        if ret < 0 && *__errno_location() != EAGAIN { err(EXIT_FAILURE, b"Can't poll memcg events (%d)\0".as_ptr() as _, ret); }
        if fd.revents & POLLERR != 0 { err(EXIT_FAILURE, b"Got POLLERR during monitor events\0".as_ptr() as _); }
        if fd.revents & POLLIN != 0 {
            let mut buffer = [0u8; INOTIFY_BUFFER_SIZE]; let length = read(fd.fd, buffer.as_mut_ptr() as _, INOTIFY_BUFFER_SIZE);
            if length <= 0 { continue; }
            let mut event = buffer.as_ptr() as *const InotifyEvent; let mut remaining = length as usize;
            while remaining >= size_of::<InotifyEvent>() { process_memcg_events(events, event); remaining -= size_of::<InotifyEvent>() + (*event).len as usize; event = (event as *const u8).add(size_of::<InotifyEvent>() + (*event).len as usize) as _; }
        }
    }
}

unsafe fn initialize_memcg_events(events: *mut MemcgEvents, cgroup: *const c_char) -> c_int {
    memset(events as _, 0, size_of::<MemcgEvents>());
    let ret = snprintf((*events).path.as_mut_ptr(), PATH_MAX, b"/sys/fs/cgroup/%s/memory.events\0".as_ptr() as _, cgroup);
    if ret >= PATH_MAX as c_int { warnx(b"Path to cgroup memory.events is too long\0".as_ptr() as _); return -EMSGSIZE; }
    if ret < 0 { warn(b"Can't generate cgroup event full name\0".as_ptr() as _); return ret; }
    let ret = read_memcg_events(events, false); if ret != 0 { warnx(b"Failed to read initial memcg events state (%d)\0".as_ptr() as _, ret); return ret; }
    (*events).inotify_fd = inotify_init(); if (*events).inotify_fd < 0 { warn(b"Failed to setup new inotify device\0".as_ptr() as _); return -EMFILE; }
    (*events).inotify_wd = inotify_add_watch((*events).inotify_fd, (*events).path.as_ptr(), IN_MODIFY); if (*events).inotify_wd < 0 { warn(b"Couldn't add monitor in dir %s\0".as_ptr() as _, (*events).path.as_ptr()); return -EIO; }
    printf(b"Initialized MEMCG events with counters:\n\0".as_ptr() as _); print_memcg_counters(&(*events).counters); 0
}

unsafe fn cleanup_memcg_events(events: *mut MemcgEvents) { inotify_rm_watch((*events).inotify_fd, (*events).inotify_wd); close((*events).inotify_fd); }

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 2 { eprintln!("Usage: {} <cgroup>", args[0].to_string_lossy()); std::process::exit(EXIT_FAILURE); }
    let cgroup = std::ffi::CString::new(args[1].as_os_str().as_encoded_bytes()).unwrap();
    unsafe {
        let mut events: MemcgEvents = zeroed();
        let ret = initialize_memcg_events(&mut events, cgroup.as_ptr());
        if ret != 0 { errx(EXIT_FAILURE, b"Can't initialize memcg events (%d)\0".as_ptr() as _, ret); }
        monitor_events(&mut events);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
