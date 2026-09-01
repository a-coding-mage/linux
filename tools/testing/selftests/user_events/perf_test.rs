// SPDX-License-Identifier: GPL-2.0
/*
 * User Events Perf Events Test Program
 *
 * Copyright (c) 2021 Beau Belgrave <beaub@linux.microsoft.com>
 */

// C includes translated as external dependencies:
// errno.h, linux/user_events.h, linux/perf_event.h, stdio.h, stdlib.h,
// fcntl.h, sys/ioctl.h, sys/stat.h, unistd.h, asm/unistd.h,
// kselftest_harness.h, user_events_selftests.h.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::{size_of, size_of_val};
use core::ptr;

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;
type __u32 = u32;
type __u64 = u64;

const data_file: *const c_char = b"/sys/kernel/tracing/user_events_data\0".as_ptr() as *const c_char;
const id_file: *const c_char =
    b"/sys/kernel/tracing/events/user_events/__test_event/id\0".as_ptr() as *const c_char;
const fmt_file: *const c_char =
    b"/sys/kernel/tracing/events/user_events/__test_event/format\0".as_ptr() as *const c_char;

#[repr(C)]
struct event {
    index: __u32,
    field1: __u32,
    field2: __u32,
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

extern "C" {
    static mut errno: c_int;

    fn syscall(num: c_long, ...) -> c_long;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getc(stream: *mut FILE) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn printf(format: *const c_char, ...) -> c_int;
    fn test_harness_run(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

type c_uint = u32;

unsafe fn perf_event_open(
    pe: *mut perf_event_attr,
    pid: pid_t,
    cpu: c_int,
    group_fd: c_int,
    flags: c_ulong,
) -> c_long {
    syscall(__NR_perf_event_open as c_long, pe, pid, cpu, group_fd, flags)
}

unsafe fn get_id() -> c_int {
    let fp = fopen(id_file, b"r\0".as_ptr() as *const c_char);
    let mut ret: c_int;
    let mut id: c_int = 0;

    if fp.is_null() {
        return -1;
    }

    ret = fscanf(fp, b"%d\0".as_ptr() as *const c_char, &mut id);
    fclose(fp);

    if ret != 1 {
        return -1;
    }

    id
}

unsafe fn get_offset() -> c_int {
    let fp = fopen(fmt_file, b"r\0".as_ptr() as *const c_char);
    let mut ret: c_int;
    let mut c: c_int;
    let mut last: c_int = 0;
    let mut offset: c_int = 0;

    if fp.is_null() {
        return -1;
    }

    /* Read until empty line */
    loop {
        c = getc(fp);

        if c == EOF {
            break;
        }

        if last == '\n' as c_int && c == '\n' as c_int {
            break;
        }

        last = c;
    }

    ret = fscanf(
        fp,
        b"\tfield:u32 field1;\toffset:%d;\0".as_ptr() as *const c_char,
        &mut offset,
    );
    fclose(fp);

    if ret != 1 {
        return -1;
    }

    offset
}

unsafe fn clear(check: *mut c_int) -> c_int {
    let mut unreg: user_unreg = core::mem::zeroed();
    let mut i: c_int;
    let mut ret: c_int = 0;

    unreg.size = size_of::<user_unreg>() as __u32;
    unreg.disable_bit = 31;
    unreg.disable_addr = check as __u64;

    let fd = open(data_file, O_RDWR);

    if fd == -1 {
        return -1;
    }

    if ioctl(fd, DIAG_IOCSUNREG, &mut unreg) == -1 {
        if errno != ENOENT {
            return -1;
        }
    }

    /*
     * Deleting the event drops its last reference, but the unregister
     * above defers that put (and the freeing of the enabler) past an RCU
     * grace period. The delete can therefore transiently fail with -EBUSY
     * until that reference is dropped. Retry for up to ~10 seconds so the
     * event is actually gone before the next test registers the same name.
     */
    i = 0;
    while i < 10000 {
        ret = ioctl(
            fd,
            DIAG_IOCSDEL,
            b"__test_event\0".as_ptr() as *const c_char,
        );

        if ret == 0 || errno == ENOENT {
            ret = 0;
            break;
        }

        if errno != EBUSY {
            close(fd);
            return -1;
        }

        usleep(1000);
        i += 1;
    }

    close(fd);

    ret
}

fixture!(user {
    data_fd: c_int,
    check: c_int,
    umount: bool,
});

fixture_setup!(user, |self_: *mut user| unsafe {
    USER_EVENT_FIXTURE_SETUP!(return, (*self_).umount);

    (*self_).data_fd = open(data_file, O_RDWR);
    ASSERT_NE!(-1, (*self_).data_fd);
});

fixture_teardown!(user, |self_: *mut user| unsafe {
    USER_EVENT_FIXTURE_TEARDOWN!((*self_).umount);

    close((*self_).data_fd);

    if clear(&mut (*self_).check) != 0 {
        printf(b"WARNING: Clear didn't work!\n\0".as_ptr() as *const c_char);
    }
});

test_f!(user, perf_write, |self_: *mut user| unsafe {
    let mut pe: perf_event_attr = core::mem::zeroed();
    let mut reg: user_reg = core::mem::zeroed();
    let mut event: event = core::mem::zeroed();
    let mut perf_page: *mut perf_event_mmap_page;
    let page_size: c_int = sysconf(_SC_PAGESIZE) as c_int;
    let mut id: c_int;
    let mut fd: c_int;
    let mut offset: c_int;
    let mut val: *mut __u32;

    reg.size = size_of::<user_reg>() as __u32;
    reg.name_args = b"__test_event u32 field1; u32 field2\0".as_ptr() as __u64;
    reg.enable_bit = 31;
    reg.enable_addr = &mut (*self_).check as *mut c_int as __u64;
    reg.enable_size = size_of_val(&(*self_).check) as __u32;

    /* Register should work */
    ASSERT_EQ!(0, ioctl((*self_).data_fd, DIAG_IOCSREG, &mut reg));
    ASSERT_EQ!(0, reg.write_index);
    ASSERT_EQ!(0, (*self_).check);

    /* Id should be there */
    id = get_id();
    ASSERT_NE!(-1, id);
    offset = get_offset();
    ASSERT_NE!(-1, offset);

    pe.type_ = PERF_TYPE_TRACEPOINT;
    pe.size = size_of::<perf_event_attr>() as __u32;
    pe.config = id as __u64;
    pe.sample_type = PERF_SAMPLE_RAW as __u64;
    pe.sample_period = 1;
    pe.wakeup_events = 1;

    /* Tracepoint attach should work */
    fd = perf_event_open(&mut pe, 0, -1, -1, 0) as c_int;
    ASSERT_NE!(-1, fd);

    perf_page = mmap(
        ptr::null_mut(),
        (page_size * 2) as size_t,
        PROT_READ,
        MAP_SHARED,
        fd,
        0,
    ) as *mut perf_event_mmap_page;
    ASSERT_NE!(MAP_FAILED, perf_page as *mut c_void);

    /* Status should be updated */
    ASSERT_EQ!(1 << reg.enable_bit, (*self_).check);

    event.index = reg.write_index;
    event.field1 = 0xc001;
    event.field2 = 0xc01a;

    /* Ensure write shows up at correct offset */
    ASSERT_NE!(
        -1,
        write(
            (*self_).data_fd,
            &event as *const event as *const c_void,
            size_of::<event>(),
        )
    );
    val = (perf_page as *mut c_char).add((*perf_page).data_offset as usize) as *mut __u32;
    ASSERT_EQ!(PERF_RECORD_SAMPLE, *val);
    /* Skip over header and size, move to offset */
    val = val.add(3);
    val = (val as *mut c_char).add(offset as usize) as *mut __u32;
    /* Ensure correct */
    ASSERT_EQ!(event.field1, *val);
    val = val.add(1);
    ASSERT_EQ!(event.field2, *val);
    val = val.add(1);

    munmap(perf_page as *mut c_void, (page_size * 2) as size_t);
    close(fd);

    /* Status should be updated */
    ASSERT_EQ!(0, (*self_).check);
});

test_f!(user, perf_empty_events, |self_: *mut user| unsafe {
    let mut pe: perf_event_attr = core::mem::zeroed();
    let mut reg: user_reg = core::mem::zeroed();
    let mut perf_page: *mut perf_event_mmap_page;
    let page_size: c_int = sysconf(_SC_PAGESIZE) as c_int;
    let mut id: c_int;
    let mut fd: c_int;
    let mut val: *mut __u32;

    reg.size = size_of::<user_reg>() as __u32;
    reg.name_args = b"__test_event\0".as_ptr() as __u64;
    reg.enable_bit = 31;
    reg.enable_addr = &mut (*self_).check as *mut c_int as __u64;
    reg.enable_size = size_of_val(&(*self_).check) as __u32;

    /* Register should work */
    ASSERT_EQ!(0, ioctl((*self_).data_fd, DIAG_IOCSREG, &mut reg));
    ASSERT_EQ!(0, reg.write_index);
    ASSERT_EQ!(0, (*self_).check);

    /* Id should be there */
    id = get_id();
    ASSERT_NE!(-1, id);

    pe.type_ = PERF_TYPE_TRACEPOINT;
    pe.size = size_of::<perf_event_attr>() as __u32;
    pe.config = id as __u64;
    pe.sample_type = PERF_SAMPLE_RAW as __u64;
    pe.sample_period = 1;
    pe.wakeup_events = 1;

    /* Tracepoint attach should work */
    fd = perf_event_open(&mut pe, 0, -1, -1, 0) as c_int;
    ASSERT_NE!(-1, fd);

    perf_page = mmap(
        ptr::null_mut(),
        (page_size * 2) as size_t,
        PROT_READ,
        MAP_SHARED,
        fd,
        0,
    ) as *mut perf_event_mmap_page;
    ASSERT_NE!(MAP_FAILED, perf_page as *mut c_void);

    /* Status should be updated */
    ASSERT_EQ!(1 << reg.enable_bit, (*self_).check);

    /* Ensure write shows up at correct offset */
    ASSERT_NE!(
        -1,
        write(
            (*self_).data_fd,
            &reg.write_index as *const __u32 as *const c_void,
            size_of_val(&reg.write_index),
        )
    );
    val = (perf_page as *mut c_char).add((*perf_page).data_offset as usize) as *mut __u32;
    ASSERT_EQ!(PERF_RECORD_SAMPLE, *val);

    munmap(perf_page as *mut c_void, (page_size * 2) as size_t);
    close(fd);

    /* Status should be updated */
    ASSERT_EQ!(0, (*self_).check);
});

fn main() -> c_int {
    let mut argv: Vec<*mut c_char> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
        .collect();
    let argc = argv.len() as c_int;
    argv.push(ptr::null_mut());
    unsafe { test_harness_run(argc, argv.as_mut_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
