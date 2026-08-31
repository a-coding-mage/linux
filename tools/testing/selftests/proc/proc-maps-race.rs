// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022 Google LLC.
 * Author: Suren Baghdasaryan <surenb@google.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * Fork a child that concurrently modifies address space while the main
 * process is reading /proc/$PID/maps and /proc/$PID/smaps, verifying the
 * results. Address space modifications include:
 *     VMA splitting and merging
 *
 */

use libc::{
    c_char, c_int, c_ulong, c_ulonglong, c_void, pid_t, pthread_cond_t, pthread_condattr_t,
    pthread_mutex_t, pthread_mutexattr_t, size_t, ssize_t, timespec, MAP_ANONYMOUS, MAP_FAILED,
    MAP_FIXED, MAP_PRIVATE, MAP_SHARED, O_RDONLY, PROT_NONE, PROT_READ, PROT_WRITE, SEEK_SET,
};
use std::mem;
use std::ptr;

/* kselftest_harness.h supplies the original fixture/test harness macros. */
macro_rules! ASSERT_TRUE {
    ($cond:expr) => {
        assert!($cond)
    };
}

macro_rules! ASSERT_FALSE {
    ($cond:expr) => {
        assert!(!$cond)
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right)
    };
}

const LINE_MAX_SIZE: usize = 256;

/* linux/fs.h */
const PROCMAP_QUERY: c_ulong = 0xc0386611;

/* sys/mman.h */
const MREMAP_MAYMOVE: c_int = 1;
const MREMAP_FIXED: c_int = 2;
const MREMAP_DONTUNMAP: c_int = 4;

#[repr(C)]
struct procmap_query {
    size: u64,
    query_flags: u64,
    query_addr: u64,
    vma_start: u64,
    vma_end: u64,
    vma_flags: u64,
    vma_page_size: u64,
}

extern "C" {
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn lseek(fd: c_int, offset: libc::off_t, whence: c_int) -> libc::off_t;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut libc::FILE) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn atol(nptr: *const c_char) -> libc::c_long;
    fn sysconf(name: c_int) -> libc::c_long;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: libc::off_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn mremap(
        old_address: *mut c_void,
        old_size: size_t,
        new_size: size_t,
        flags: c_int,
        ...
    ) -> *mut c_void;
    fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> c_int;
    fn pthread_mutexattr_setpshared(attr: *mut pthread_mutexattr_t, pshared: c_int) -> c_int;
    fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attr: *const pthread_mutexattr_t) -> c_int;
    fn pthread_condattr_init(attr: *mut pthread_condattr_t) -> c_int;
    fn pthread_condattr_setpshared(attr: *mut pthread_condattr_t, pshared: c_int) -> c_int;
    fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *const pthread_condattr_t) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    fn fork() -> pid_t;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn clock_gettime(clk_id: libc::clockid_t, tp: *mut timespec) -> c_int;
    fn ksft_exit_fail() -> !;
}

extern "C" {
    static mut stdout: *mut libc::FILE;
}

#[repr(C)]
struct page_content {
    data: *mut c_char,
    size: ssize_t,
}

#[repr(C)]
struct line_content {
    text: [c_char; LINE_MAX_SIZE],
    start_addr: c_ulong,
    end_addr: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum test_state {
    INIT,
    CHILD_READY,
    PARENT_READY,
    SETUP_READY,
    SETUP_MODIFY_MAPS,
    SETUP_MAPS_MODIFIED,
    SETUP_RESTORE_MAPS,
    SETUP_MAPS_RESTORED,
    TEST_READY,
    TEST_DONE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum maps_file {
    MAPS,
    SMAPS,
}

type vma_modifier_op = unsafe extern "C" fn(*mut proc_maps_race) -> bool;
type vma_mod_result_check_op = unsafe extern "C" fn(
    *mut line_content,
    *mut line_content,
    *mut line_content,
    *mut line_content,
) -> bool;

#[repr(C)]
struct vma_modifier_info {
    vma_count: c_int,
    addr: *mut c_void,
    prot: c_int,
    next_addr: *mut c_void,
    vma_modify: Option<vma_modifier_op>,
    vma_restore: Option<vma_modifier_op>,
    vma_mod_check: Option<vma_mod_result_check_op>,
    sync_lock: pthread_mutex_t,
    sync_cond: pthread_cond_t,
    curr_state: test_state,
    exit: bool,
    child_mapped_addr: [*mut c_void; 0],
}

#[repr(C)]
struct proc_maps_race {
    mod_info: *mut vma_modifier_info,
    page1: page_content,
    page2: page_content,
    last_line: line_content,
    first_line: line_content,
    duration_sec: c_ulong,
    maps_file: maps_file,
    shared_mem_size: c_int,
    skip_pages: c_int,
    page_size: c_int,
    vma_count: c_int,
    verbose: bool,
    maps_fd: c_int,
    pid: pid_t,
}

#[repr(C)]
struct proc_maps_race_variant {
    maps_file: maps_file,
}

static PROC_MAPS_RACE_MAPS: proc_maps_race_variant = proc_maps_race_variant {
    maps_file: maps_file::MAPS,
};

static PROC_MAPS_RACE_SMAPS: proc_maps_race_variant = proc_maps_race_variant {
    maps_file: maps_file::SMAPS,
};

unsafe fn child_mapped_addr(mod_info: *mut vma_modifier_info) -> *mut *mut c_void {
    (*mod_info).child_mapped_addr.as_mut_ptr()
}

unsafe fn read_page(self_: *mut proc_maps_race, page: *mut page_content) -> bool {
    let bytes_read: ssize_t;

    bytes_read = read(
        (*self_).maps_fd,
        (*page).data as *mut c_void,
        (*self_).page_size as size_t,
    );
    if bytes_read <= 0 {
        return false;
    }

    /* Make sure data always ends with a newline character. */
    if *(*page).data.offset(bytes_read as isize - 1) != b'\n' as c_char {
        return false;
    }

    (*page).size = bytes_read;

    true
}

unsafe fn parse_vma_line(
    line_start: *mut c_char,
    line_end: *mut c_char,
    start: *mut c_ulong,
    end: *mut c_ulong,
) -> bool {
    let found: bool;

    *line_end = b'\0' as c_char; /* stop sscanf at the EOL */
    found = sscanf(line_start, b"%lx-%lx\0".as_ptr() as *const c_char, start, end) == 2;
    *line_end = b'\n' as c_char;

    found
}

unsafe fn locate_containing_page(
    self_: *mut proc_maps_race,
    addr: c_ulong,
    size: c_ulong,
) -> c_int {
    let mut start: c_ulong = 0;
    let mut end: c_ulong = 0;
    let mut page: c_int = 0;

    if lseek((*self_).maps_fd, 0, SEEK_SET) < 0 {
        return -1;
    }

    loop {
        let mut curr_pos: *mut c_char;
        let end_pos: *mut c_char;

        if !read_page(self_, &mut (*self_).page1) {
            return -1;
        }

        curr_pos = (*self_).page1.data;
        end_pos = (*self_).page1.data.offset((*self_).page1.size as isize);
        while curr_pos < end_pos {
            let line_end: *mut c_char;

            line_end = strchr(curr_pos, b'\n' as c_int);
            if line_end.is_null() {
                break;
            }

            if parse_vma_line(curr_pos, line_end, &mut start, &mut end)
                && start == addr
                && end == addr.wrapping_add(size)
            {
                return page;
            }

            curr_pos = line_end.offset(1);
        }
        page += 1;
    }
}

unsafe fn read_two_pages(self_: *mut proc_maps_race) -> bool {
    if lseek((*self_).maps_fd, 0, SEEK_SET) < 0 {
        return false;
    }

    for _i in 0..(*self_).skip_pages {
        if !read_page(self_, &mut (*self_).page1) {
            return false;
        }
    }

    read_page(self_, &mut (*self_).page1) && read_page(self_, &mut (*self_).page2)
}

unsafe fn copy_line(
    line_start: *const c_char,
    line_end: *const c_char,
    buf: *mut c_char,
    buf_size: size_t,
) {
    let len = std::cmp::min(line_end.offset_from(line_start) as size_t, buf_size - 1);

    strncpy(buf, line_start, len);
    *buf.add(len) = b'\0' as c_char;
}

unsafe fn copy_first_line(page: *mut page_content, first_line: *mut c_char, line_size: size_t) {
    copy_line(
        (*page).data,
        strchr((*page).data, b'\n' as c_int),
        first_line,
        line_size,
    );
}

unsafe fn copy_last_line(page: *mut page_content, last_line: *mut c_char, line_size: size_t) {
    /* Get the last line in the first page */
    let end = (*page).data.offset((*page).size as isize - 1);
    /* skip last newline */
    let mut pos = end.offset(-1);

    /* search previous newline */
    while *pos.offset(-1) != b'\n' as c_char {
        pos = pos.offset(-1);
    }

    copy_line(pos, end, last_line, line_size);
}

unsafe fn copy_first_entry(page: *mut page_content, first_line: *mut c_char, line_size: size_t) -> bool {
    let mut start_pos = (*page).data;

    while start_pos < (*page).data.offset((*page).size as isize) {
        let mut start_addr: c_ulong = 0;
        let mut end_addr: c_ulong = 0;
        let end_pos: *mut c_char;

        end_pos = strchr(start_pos, b'\n' as c_int);
        if end_pos.is_null() {
            break;
        }

        if parse_vma_line(start_pos, end_pos, &mut start_addr, &mut end_addr) {
            copy_line(start_pos, end_pos, first_line, line_size);
            return true;
        }

        start_pos = end_pos.offset(1);
    }

    false
}

unsafe fn copy_last_entry(page: *mut page_content, last_line: *mut c_char, line_size: size_t) -> bool {
    let mut end_pos = (*page).data.offset((*page).size as isize - 1);
    let mut start_pos: *mut c_char;

    while end_pos > (*page).data {
        let mut start_addr: c_ulong = 0;
        let mut end_addr: c_ulong = 0;

        /* skip last newline */
        start_pos = end_pos.offset(-1);
        /* search previous newline */
        while start_pos > (*page).data && *start_pos.offset(-1) != b'\n' as c_char {
            start_pos = start_pos.offset(-1);
        }
        if parse_vma_line(start_pos, end_pos, &mut start_addr, &mut end_addr) {
            copy_line(start_pos, end_pos, last_line, line_size);
            return true;
        }

        end_pos = start_pos.offset(-1);
    }

    false
}

/* Read the last line of the first page and the first line of the second page */
unsafe fn read_boundary_lines(
    self_: *mut proc_maps_race,
    last_line: *mut line_content,
    first_line: *mut line_content,
) -> bool {
    if !read_two_pages(self_) {
        return false;
    }

    if (*self_).maps_file == maps_file::MAPS {
        copy_last_line(&mut (*self_).page1, (*last_line).text.as_mut_ptr(), LINE_MAX_SIZE);
        copy_first_line(&mut (*self_).page2, (*first_line).text.as_mut_ptr(), LINE_MAX_SIZE);
    } else if (*self_).maps_file == maps_file::SMAPS {
        if !copy_last_entry(&mut (*self_).page1, (*last_line).text.as_mut_ptr(), LINE_MAX_SIZE)
            || !copy_first_entry(&mut (*self_).page2, (*first_line).text.as_mut_ptr(), LINE_MAX_SIZE)
        {
            return false;
        }
    } else {
        return false;
    }

    sscanf(
        (*last_line).text.as_ptr(),
        b"%lx-%lx\0".as_ptr() as *const c_char,
        &mut (*last_line).start_addr,
        &mut (*last_line).end_addr,
    ) == 2
        && sscanf(
            (*first_line).text.as_ptr(),
            b"%lx-%lx\0".as_ptr() as *const c_char,
            &mut (*first_line).start_addr,
            &mut (*first_line).end_addr,
        ) == 2
}

/* Thread synchronization routines */
unsafe fn wait_for_state(mod_info: *mut vma_modifier_info, state: test_state) {
    pthread_mutex_lock(&mut (*mod_info).sync_lock);
    while (*mod_info).curr_state != state {
        pthread_cond_wait(&mut (*mod_info).sync_cond, &mut (*mod_info).sync_lock);
    }
    pthread_mutex_unlock(&mut (*mod_info).sync_lock);
}

unsafe fn signal_state(mod_info: *mut vma_modifier_info, state: test_state) {
    pthread_mutex_lock(&mut (*mod_info).sync_lock);
    (*mod_info).curr_state = state;
    pthread_cond_signal(&mut (*mod_info).sync_cond);
    pthread_mutex_unlock(&mut (*mod_info).sync_lock);
}

unsafe fn stop_vma_modifier(mod_info: *mut vma_modifier_info) {
    wait_for_state(mod_info, test_state::SETUP_READY);
    (*mod_info).exit = true;
    signal_state(mod_info, test_state::SETUP_MODIFY_MAPS);
}

unsafe fn print_first_lines(text: *mut c_char, mut nr: c_int) {
    let mut end = text as *const c_char;

    while nr != 0 {
        end = strchr(end, b'\n' as c_int);
        if end.is_null() {
            break;
        }
        nr -= 1;
        end = end.offset(1);
    }

    if !end.is_null() {
        let offs = end.offset_from(text) as c_int;

        *text.offset(offs as isize) = b'\0' as c_char;
        printf(b"%s\0".as_ptr() as *const c_char, text);
        *text.offset(offs as isize) = b'\n' as c_char;
        printf(b"\n\0".as_ptr() as *const c_char);
    } else {
        printf(b"%s\0".as_ptr() as *const c_char, text);
    }
}

unsafe fn print_last_lines(text: *mut c_char, mut nr: c_int) {
    let mut start = text.add(strlen(text));

    nr += 1; /* to ignore the last newline */
    while nr != 0 {
        while start > text && *start != b'\n' as c_char {
            start = start.offset(-1);
        }
        nr -= 1;
        start = start.offset(-1);
    }
    printf(b"%s\0".as_ptr() as *const c_char, start);
}

unsafe fn print_boundaries(title: *const c_char, self_: *mut proc_maps_race) {
    if !(*self_).verbose {
        return;
    }

    printf(b"%s\0".as_ptr() as *const c_char, title);
    /* Print 3 boundary lines from each page */
    print_last_lines((*self_).page1.data, 3);
    printf(b"-----------------page boundary-----------------\n\0".as_ptr() as *const c_char);
    print_first_lines((*self_).page2.data, 3);
}

unsafe fn print_boundaries_on(
    condition: bool,
    title: *const c_char,
    self_: *mut proc_maps_race,
) -> bool {
    if (*self_).verbose && condition {
        print_boundaries(title, self_);
    }

    condition
}

unsafe fn report_test_start(name: *const c_char, verbose: bool) {
    if verbose {
        printf(b"==== %s ====\n\0".as_ptr() as *const c_char, name);
    }
}

static mut print_ts: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};

unsafe fn start_test_loop(ts: *mut timespec, verbose: bool) {
    if verbose {
        print_ts.tv_sec = (*ts).tv_sec;
    }
}

unsafe fn end_test_iteration(ts: *mut timespec, verbose: bool) {
    if !verbose {
        return;
    }

    /* Update every second */
    if print_ts.tv_sec == (*ts).tv_sec {
        return;
    }

    printf(b".\0".as_ptr() as *const c_char);
    fflush(stdout);
    print_ts.tv_sec = (*ts).tv_sec;
}

unsafe fn end_test_loop(verbose: bool) {
    if verbose {
        printf(b"\n\0".as_ptr() as *const c_char);
    }
}

unsafe fn capture_mod_pattern(
    self_: *mut proc_maps_race,
    mod_last_line: *mut line_content,
    mod_first_line: *mut line_content,
    restored_last_line: *mut line_content,
    restored_first_line: *mut line_content,
) -> bool {
    print_boundaries(b"Before modification\0".as_ptr() as *const c_char, self_);

    signal_state((*self_).mod_info, test_state::SETUP_MODIFY_MAPS);
    wait_for_state((*self_).mod_info, test_state::SETUP_MAPS_MODIFIED);

    /* Copy last line of the first page and first line of the last page */
    if !read_boundary_lines(self_, mod_last_line, mod_first_line) {
        return false;
    }

    print_boundaries(b"After modification\0".as_ptr() as *const c_char, self_);

    signal_state((*self_).mod_info, test_state::SETUP_RESTORE_MAPS);
    wait_for_state((*self_).mod_info, test_state::SETUP_MAPS_RESTORED);

    /* Copy last line of the first page and first line of the last page */
    if !read_boundary_lines(self_, restored_last_line, restored_first_line) {
        return false;
    }

    print_boundaries(b"After restore\0".as_ptr() as *const c_char, self_);

    if !((*(*self_).mod_info).vma_mod_check.unwrap())(
        mod_last_line,
        mod_first_line,
        restored_last_line,
        restored_first_line,
    ) {
        return false;
    }

    /*
     * The content of these lines after modify+resore should be the same
     * as the original.
     */
    strcmp((*restored_last_line).text.as_ptr(), (*self_).last_line.text.as_ptr()) == 0
        && strcmp(
            (*restored_first_line).text.as_ptr(),
            (*self_).first_line.text.as_ptr(),
        ) == 0
}

unsafe fn query_addr_at(
    maps_fd: c_int,
    addr: *mut c_void,
    vma_start: *mut c_ulong,
    vma_end: *mut c_ulong,
) -> bool {
    let mut q: procmap_query = mem::zeroed();

    q.size = mem::size_of::<procmap_query>() as u64;
    /* Find the VMA at the split address */
    q.query_addr = addr as c_ulonglong as u64;
    q.query_flags = 0;
    if ioctl(maps_fd, PROCMAP_QUERY, &mut q) != 0 {
        return false;
    }

    *vma_start = q.vma_start as c_ulong;
    *vma_end = q.vma_end as c_ulong;

    true
}

unsafe extern "C" fn split_vma(self_: *mut proc_maps_race) -> bool {
    /* PROT_NONE differs from both readable neighbors. */
    mmap(
        (*(*self_).mod_info).addr,
        (*self_).page_size as size_t,
        PROT_NONE,
        MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED,
        -1,
        0,
    ) != MAP_FAILED
}

unsafe extern "C" fn merge_vma(self_: *mut proc_maps_race) -> bool {
    mmap(
        (*(*self_).mod_info).addr,
        (*self_).page_size as size_t,
        (*(*self_).mod_info).prot,
        MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED,
        -1,
        0,
    ) != MAP_FAILED
}

unsafe extern "C" fn check_split_result(
    mod_last_line: *mut line_content,
    mod_first_line: *mut line_content,
    restored_last_line: *mut line_content,
    restored_first_line: *mut line_content,
) -> bool {
    /* Make sure vmas at the boundaries are changing */
    strcmp((*mod_last_line).text.as_ptr(), (*restored_last_line).text.as_ptr()) != 0
        && strcmp(
            (*mod_first_line).text.as_ptr(),
            (*restored_first_line).text.as_ptr(),
        ) != 0
}

unsafe extern "C" fn shrink_vma(self_: *mut proc_maps_race) -> bool {
    mremap(
        (*(*self_).mod_info).addr,
        ((*self_).page_size * 3) as size_t,
        (*self_).page_size as size_t,
        0,
    ) != MAP_FAILED
}

unsafe extern "C" fn expand_vma(self_: *mut proc_maps_race) -> bool {
    mremap(
        (*(*self_).mod_info).addr,
        (*self_).page_size as size_t,
        ((*self_).page_size * 3) as size_t,
        0,
    ) != MAP_FAILED
}

unsafe extern "C" fn check_shrink_result(
    mod_last_line: *mut line_content,
    mod_first_line: *mut line_content,
    restored_last_line: *mut line_content,
    restored_first_line: *mut line_content,
) -> bool {
    /* Make sure only the last vma of the first page is changing */
    strcmp((*mod_last_line).text.as_ptr(), (*restored_last_line).text.as_ptr()) != 0
        && strcmp(
            (*mod_first_line).text.as_ptr(),
            (*restored_first_line).text.as_ptr(),
        ) == 0
}

unsafe extern "C" fn remap_vma(self_: *mut proc_maps_race) -> bool {
    /*
     * Remap the last page of the next vma into the middle of the vma.
     * This splits the current vma and the first and middle parts (the
     * parts at lower addresses) become the last vma objserved in the
     * first page and the first vma observed in the last page.
     */
    mremap(
        ((*(*self_).mod_info).next_addr as *mut u8).add(((*self_).page_size * 2) as usize)
            as *mut c_void,
        (*self_).page_size as size_t,
        (*self_).page_size as size_t,
        MREMAP_FIXED | MREMAP_MAYMOVE | MREMAP_DONTUNMAP,
        ((*(*self_).mod_info).addr as *mut u8).add((*self_).page_size as usize) as *mut c_void,
    ) != MAP_FAILED
}

unsafe extern "C" fn patch_vma(self_: *mut proc_maps_race) -> bool {
    mprotect(
        ((*(*self_).mod_info).addr as *mut u8).add((*self_).page_size as usize) as *mut c_void,
        (*self_).page_size as size_t,
        (*(*self_).mod_info).prot,
    ) == 0
}

unsafe extern "C" fn check_remap_result(
    mod_last_line: *mut line_content,
    mod_first_line: *mut line_content,
    restored_last_line: *mut line_content,
    restored_first_line: *mut line_content,
) -> bool {
    /* Make sure vmas at the boundaries are changing */
    strcmp((*mod_last_line).text.as_ptr(), (*restored_last_line).text.as_ptr()) != 0
        && strcmp(
            (*mod_first_line).text.as_ptr(),
            (*restored_first_line).text.as_ptr(),
        ) != 0
}

unsafe fn proc_maps_race_setup(self_: *mut proc_maps_race, variant: *const proc_maps_race_variant) {
    let verbose = getenv(b"VERBOSE\0".as_ptr() as *const c_char);
    let duration = getenv(b"DURATION\0".as_ptr() as *const c_char);
    let mut mod_info: *mut vma_modifier_info;
    let mut mutex_attr: pthread_mutexattr_t = mem::zeroed();
    let mut cond_attr: pthread_condattr_t = mem::zeroed();
    let first_map_addr: c_ulong;
    let last_map_addr: c_ulong;
    let duration_sec: c_ulong;
    let mut fname: [c_char; 32] = [0; 32];

    (*self_).page_size = sysconf(libc::_SC_PAGESIZE) as c_int;
    (*self_).verbose = !verbose.is_null() && strncmp(verbose, b"1\0".as_ptr() as *const c_char, 1) == 0;
    (*self_).maps_file = (*variant).maps_file;
    duration_sec = if !duration.is_null() {
        atol(duration) as c_ulong
    } else {
        0
    };
    (*self_).duration_sec = if duration_sec != 0 { duration_sec } else { 5u64 as c_ulong };

    /*
     * Have to map enough vmas for /proc/pid/maps to contain more than one
     * page worth of vmas. Assume at least 32 bytes per line in maps output
     */
    (*self_).vma_count = (*self_).page_size / 32 + 1;
    (*self_).shared_mem_size =
        mem::size_of::<vma_modifier_info>() as c_int + (*self_).vma_count * mem::size_of::<*mut c_void>() as c_int;

    /* map shared memory for communication with the child process */
    (*self_).mod_info = mmap(
        ptr::null_mut(),
        (*self_).shared_mem_size as size_t,
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut vma_modifier_info;
    ASSERT_NE!((*self_).mod_info as *mut c_void, MAP_FAILED);
    mod_info = (*self_).mod_info;

    /* Initialize shared members */
    pthread_mutexattr_init(&mut mutex_attr);
    pthread_mutexattr_setpshared(&mut mutex_attr, libc::PTHREAD_PROCESS_SHARED);
    ASSERT_EQ!(pthread_mutex_init(&mut (*mod_info).sync_lock, &mutex_attr), 0);
    pthread_condattr_init(&mut cond_attr);
    pthread_condattr_setpshared(&mut cond_attr, libc::PTHREAD_PROCESS_SHARED);
    ASSERT_EQ!(pthread_cond_init(&mut (*mod_info).sync_cond, &cond_attr), 0);
    (*mod_info).vma_count = (*self_).vma_count;
    (*mod_info).curr_state = test_state::INIT;
    (*mod_info).exit = false;

    (*self_).pid = fork();
    if (*self_).pid == 0 {
        /* Child process modifying the address space */
        let mut prot = PROT_READ | PROT_WRITE;
        let mut i: c_int;

        i = 0;
        while i < (*mod_info).vma_count {
            *child_mapped_addr(mod_info).offset(i as isize) = mmap(
                ptr::null_mut(),
                ((*self_).page_size * 3) as size_t,
                prot,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            );
            ASSERT_NE!(*child_mapped_addr(mod_info).offset(i as isize), MAP_FAILED);
            /* change protection in adjacent maps to prevent merging */
            prot ^= PROT_WRITE;
            i += 1;
        }
        signal_state(mod_info, test_state::CHILD_READY);
        wait_for_state(mod_info, test_state::PARENT_READY);
        loop {
            signal_state(mod_info, test_state::SETUP_READY);
            wait_for_state(mod_info, test_state::SETUP_MODIFY_MAPS);
            if (*mod_info).exit {
                break;
            }

            ASSERT_TRUE!(((*mod_info).vma_modify.unwrap())(self_));
            signal_state(mod_info, test_state::SETUP_MAPS_MODIFIED);
            wait_for_state(mod_info, test_state::SETUP_RESTORE_MAPS);
            ASSERT_TRUE!(((*mod_info).vma_restore.unwrap())(self_));
            signal_state(mod_info, test_state::SETUP_MAPS_RESTORED);

            wait_for_state(mod_info, test_state::TEST_READY);
            while (*mod_info).curr_state != test_state::TEST_DONE {
                ASSERT_TRUE!(((*mod_info).vma_modify.unwrap())(self_));
                ASSERT_TRUE!(((*mod_info).vma_restore.unwrap())(self_));
            }
        }
        i = 0;
        while i < (*mod_info).vma_count {
            munmap(
                *child_mapped_addr(mod_info).offset(i as isize),
                ((*self_).page_size * 3) as size_t,
            );
            i += 1;
        }

        exit(0);
    }

    match (*self_).maps_file {
        maps_file::MAPS => {
            sprintf(
                fname.as_mut_ptr(),
                b"/proc/%d/maps\0".as_ptr() as *const c_char,
                (*self_).pid,
            );
        }
        maps_file::SMAPS => {
            sprintf(
                fname.as_mut_ptr(),
                b"/proc/%d/smaps\0".as_ptr() as *const c_char,
                (*self_).pid,
            );
        }
    }
    (*self_).maps_fd = open(fname.as_ptr(), O_RDONLY);
    ASSERT_NE!((*self_).maps_fd, -1);

    /* Wait for the child to map the VMAs */
    wait_for_state(mod_info, test_state::CHILD_READY);

    /* Read first two pages */
    (*self_).page1.data = malloc((*self_).page_size as size_t) as *mut c_char;
    ASSERT_NE!((*self_).page1.data, ptr::null_mut());
    (*self_).page2.data = malloc((*self_).page_size as size_t) as *mut c_char;
    ASSERT_NE!((*self_).page2.data, ptr::null_mut());

    first_map_addr = *child_mapped_addr(mod_info).offset(0) as c_ulong;
    last_map_addr = *child_mapped_addr(mod_info).offset(((*mod_info).vma_count - 1) as isize) as c_ulong;

    (*self_).skip_pages = locate_containing_page(
        self_,
        std::cmp::min(first_map_addr, last_map_addr),
        ((*self_).page_size * 3) as c_ulong,
    );
    ASSERT_NE!((*self_).skip_pages, -1);
    ASSERT_TRUE!(read_boundary_lines(
        self_,
        &mut (*self_).last_line,
        &mut (*self_).first_line
    ));

    /*
     * Find the addresses corresponding to the last line in the first page
     * and the first line in the last page.
     */
    (*mod_info).addr = ptr::null_mut();
    (*mod_info).next_addr = ptr::null_mut();
    for i in 0..(*mod_info).vma_count {
        if *child_mapped_addr(mod_info).offset(i as isize) == (*self_).last_line.start_addr as *mut c_void {
            (*mod_info).addr = *child_mapped_addr(mod_info).offset(i as isize);
            (*mod_info).prot = PROT_READ;
            /* Even VMAs have write permission */
            if (i % 2) == 0 {
                (*mod_info).prot |= PROT_WRITE;
            }
        } else if *child_mapped_addr(mod_info).offset(i as isize)
            == (*self_).first_line.start_addr as *mut c_void
        {
            (*mod_info).next_addr = *child_mapped_addr(mod_info).offset(i as isize);
        }

        if !(*mod_info).addr.is_null() && !(*mod_info).next_addr.is_null() {
            break;
        }
    }
    ASSERT_TRUE!(!(*mod_info).addr.is_null() && !(*mod_info).next_addr.is_null());

    signal_state(mod_info, test_state::PARENT_READY);
}

unsafe fn proc_maps_race_teardown(self_: *mut proc_maps_race) {
    let mut status: c_int = 0;

    stop_vma_modifier((*self_).mod_info);

    free((*self_).page2.data as *mut c_void);
    free((*self_).page1.data as *mut c_void);

    for i in 0..(*self_).vma_count {
        munmap(
            *child_mapped_addr((*self_).mod_info).offset(i as isize),
            (*self_).page_size as size_t,
        );
    }
    close((*self_).maps_fd);
    waitpid((*self_).pid, &mut status, 0);
    munmap((*self_).mod_info as *mut c_void, (*self_).shared_mem_size as size_t);
}

unsafe fn test_maps_tearing_from_split(self_: *mut proc_maps_race) {
    let mod_info = (*self_).mod_info;

    let mut split_last_line: line_content = mem::zeroed();
    let mut split_first_line: line_content = mem::zeroed();
    let mut restored_last_line: line_content = mem::zeroed();
    let mut restored_first_line: line_content = mem::zeroed();

    wait_for_state(mod_info, test_state::SETUP_READY);

    /* re-read the file to avoid using stale data from previous test */
    ASSERT_TRUE!(read_boundary_lines(
        self_,
        &mut (*self_).last_line,
        &mut (*self_).first_line
    ));

    (*mod_info).vma_modify = Some(split_vma);
    (*mod_info).vma_restore = Some(merge_vma);
    (*mod_info).vma_mod_check = Some(check_split_result);

    report_test_start(b"Tearing from split\0".as_ptr() as *const c_char, (*self_).verbose);
    ASSERT_TRUE!(capture_mod_pattern(
        self_,
        &mut split_last_line,
        &mut split_first_line,
        &mut restored_last_line,
        &mut restored_first_line
    ));

    /* Now start concurrent modifications for self->duration_sec */
    signal_state(mod_info, test_state::TEST_READY);

    let mut new_last_line: line_content = mem::zeroed();
    let mut new_first_line: line_content = mem::zeroed();
    let mut start_ts: timespec = mem::zeroed();
    let mut end_ts: timespec = mem::zeroed();

    clock_gettime(libc::CLOCK_MONOTONIC_COARSE, &mut start_ts);
    start_test_loop(&mut start_ts, (*self_).verbose);
    loop {
        let last_line_changed: bool;
        let first_line_changed: bool;
        let mut vma_start: c_ulong = 0;
        let mut vma_end: c_ulong = 0;

        ASSERT_TRUE!(read_boundary_lines(self_, &mut new_last_line, &mut new_first_line));

        /* Check if we read vmas after split */
        if strcmp(new_last_line.text.as_ptr(), split_last_line.text.as_ptr()) == 0 {
            /*
             * The vmas should be consistent with split results,
             * however if vma was concurrently restored after a
             * split, it can be reported twice (first the original
             * split one, then the same vma but extended after the
             * merge) because we found it as the next vma again.
             * In that case new first line will be the same as the
             * last restored line.
             */
            ASSERT_FALSE!(print_boundaries_on(
                strcmp(new_first_line.text.as_ptr(), split_first_line.text.as_ptr()) != 0
                    && strcmp(new_first_line.text.as_ptr(), restored_last_line.text.as_ptr()) != 0,
                b"Split result invalid\0".as_ptr() as *const c_char,
                self_
            ));
        } else {
            /* The vmas should be consistent with merge results */
            ASSERT_FALSE!(print_boundaries_on(
                strcmp(new_last_line.text.as_ptr(), restored_last_line.text.as_ptr()) != 0,
                b"Merge result invalid\0".as_ptr() as *const c_char,
                self_
            ));
            ASSERT_FALSE!(print_boundaries_on(
                strcmp(new_first_line.text.as_ptr(), restored_first_line.text.as_ptr()) != 0,
                b"Merge result invalid\0".as_ptr() as *const c_char,
                self_
            ));
        }
        /*
         * First and last lines should change in unison. If the last
         * line changed then the first line should change as well and
         * vice versa.
         */
        last_line_changed = strcmp(new_last_line.text.as_ptr(), (*self_).last_line.text.as_ptr()) != 0;
        first_line_changed = strcmp(new_first_line.text.as_ptr(), (*self_).first_line.text.as_ptr()) != 0;
        ASSERT_EQ!(last_line_changed, first_line_changed);
        if (*self_).maps_file == maps_file::MAPS {
            /* Check if PROCMAP_QUERY ioclt() finds the right VMA */
            ASSERT_TRUE!(query_addr_at(
                (*self_).maps_fd,
                ((*mod_info).addr as *mut u8).add((*self_).page_size as usize) as *mut c_void,
                &mut vma_start,
                &mut vma_end
            ));
            /*
             * The vma at the split address can be either the same as
             * original one (if read before the split) or the same as the
             * first line in the second page (if read after the split).
             */
            ASSERT_TRUE!(
                (vma_start == (*self_).last_line.start_addr && vma_end == (*self_).last_line.end_addr)
                    || (vma_start == split_first_line.start_addr && vma_end == split_first_line.end_addr)
            );
        }
        clock_gettime(libc::CLOCK_MONOTONIC_COARSE, &mut end_ts);
        end_test_iteration(&mut end_ts, (*self_).verbose);
        if !(end_ts.tv_sec - start_ts.tv_sec < (*self_).duration_sec as libc::time_t) {
            break;
        }
    }
    end_test_loop((*self_).verbose);

    /* Signal the modifyer thread to stop and wait until it exits */
    signal_state(mod_info, test_state::TEST_DONE);
}

unsafe fn test_maps_tearing_from_resize(self_: *mut proc_maps_race) {
    let mod_info = (*self_).mod_info;

    let mut shrunk_last_line: line_content = mem::zeroed();
    let mut shrunk_first_line: line_content = mem::zeroed();
    let mut restored_last_line: line_content = mem::zeroed();
    let mut restored_first_line: line_content = mem::zeroed();

    wait_for_state(mod_info, test_state::SETUP_READY);

    /* re-read the file to avoid using stale data from previous test */
    ASSERT_TRUE!(read_boundary_lines(
        self_,
        &mut (*self_).last_line,
        &mut (*self_).first_line
    ));

    (*mod_info).vma_modify = Some(shrink_vma);
    (*mod_info).vma_restore = Some(expand_vma);
    (*mod_info).vma_mod_check = Some(check_shrink_result);

    report_test_start(b"Tearing from resize\0".as_ptr() as *const c_char, (*self_).verbose);
    ASSERT_TRUE!(capture_mod_pattern(
        self_,
        &mut shrunk_last_line,
        &mut shrunk_first_line,
        &mut restored_last_line,
        &mut restored_first_line
    ));

    /* Now start concurrent modifications for self->duration_sec */
    signal_state(mod_info, test_state::TEST_READY);

    let mut new_last_line: line_content = mem::zeroed();
    let mut new_first_line: line_content = mem::zeroed();
    let mut start_ts: timespec = mem::zeroed();
    let mut end_ts: timespec = mem::zeroed();

    clock_gettime(libc::CLOCK_MONOTONIC_COARSE, &mut start_ts);
    start_test_loop(&mut start_ts, (*self_).verbose);
    loop {
        let mut vma_start: c_ulong = 0;
        let mut vma_end: c_ulong = 0;

        ASSERT_TRUE!(read_boundary_lines(self_, &mut new_last_line, &mut new_first_line));

        /* Check if we read vmas after shrinking it */
        if strcmp(new_last_line.text.as_ptr(), shrunk_last_line.text.as_ptr()) == 0 {
            /*
             * The vmas should be consistent with shrunk results,
             * however if the vma was concurrently restored, it
             * can be reported twice (first as shrunk one, then
             * as restored one) because we found it as the next vma
             * again. In that case new first line will be the same
             * as the last restored line.
             */
            ASSERT_FALSE!(print_boundaries_on(
                strcmp(new_first_line.text.as_ptr(), shrunk_first_line.text.as_ptr()) != 0
                    && strcmp(new_first_line.text.as_ptr(), restored_last_line.text.as_ptr()) != 0,
                b"Shrink result invalid\0".as_ptr() as *const c_char,
                self_
            ));
        } else {
            /* The vmas should be consistent with the original/resored state */
            ASSERT_FALSE!(print_boundaries_on(
                strcmp(new_last_line.text.as_ptr(), restored_last_line.text.as_ptr()) != 0,
                b"Expand result invalid\0".as_ptr() as *const c_char,
                self_
            ));
            ASSERT_FALSE!(print_boundaries_on(
                strcmp(new_first_line.text.as_ptr(), restored_first_line.text.as_ptr()) != 0,
                b"Expand result invalid\0".as_ptr() as *const c_char,
                self_
            ));
        }
        if (*self_).maps_file == maps_file::MAPS {
            /* Check if PROCMAP_QUERY ioclt() finds the right VMA */
            ASSERT_TRUE!(query_addr_at((*self_).maps_fd, (*mod_info).addr, &mut vma_start, &mut vma_end));
            /*
             * The vma should stay at the same address and have either the
             * original size of 3 pages or 1 page if read after shrinking.
             */
            ASSERT_TRUE!(
                vma_start == (*self_).last_line.start_addr
                    && (vma_end - vma_start == ((*self_).page_size * 3) as c_ulong
                        || vma_end - vma_start == (*self_).page_size as c_ulong)
            );
        }
        clock_gettime(libc::CLOCK_MONOTONIC_COARSE, &mut end_ts);
        end_test_iteration(&mut end_ts, (*self_).verbose);
        if !(end_ts.tv_sec - start_ts.tv_sec < (*self_).duration_sec as libc::time_t) {
            break;
        }
    }
    end_test_loop((*self_).verbose);

    /* Signal the modifyer thread to stop and wait until it exits */
    signal_state(mod_info, test_state::TEST_DONE);
}

unsafe fn test_maps_tearing_from_remap(self_: *mut proc_maps_race) {
    let mod_info = (*self_).mod_info;

    let mut remapped_last_line: line_content = mem::zeroed();
    let mut remapped_first_line: line_content = mem::zeroed();
    let mut restored_last_line: line_content = mem::zeroed();
    let mut restored_first_line: line_content = mem::zeroed();

    wait_for_state(mod_info, test_state::SETUP_READY);

    /* re-read the file to avoid using stale data from previous test */
    ASSERT_TRUE!(read_boundary_lines(
        self_,
        &mut (*self_).last_line,
        &mut (*self_).first_line
    ));

    (*mod_info).vma_modify = Some(remap_vma);
    (*mod_info).vma_restore = Some(patch_vma);
    (*mod_info).vma_mod_check = Some(check_remap_result);

    report_test_start(b"Tearing from remap\0".as_ptr() as *const c_char, (*self_).verbose);
    ASSERT_TRUE!(capture_mod_pattern(
        self_,
        &mut remapped_last_line,
        &mut remapped_first_line,
        &mut restored_last_line,
        &mut restored_first_line
    ));

    /* Now start concurrent modifications for self->duration_sec */
    signal_state(mod_info, test_state::TEST_READY);

    let mut new_last_line: line_content = mem::zeroed();
    let mut new_first_line: line_content = mem::zeroed();
    let mut start_ts: timespec = mem::zeroed();
    let mut end_ts: timespec = mem::zeroed();

    clock_gettime(libc::CLOCK_MONOTONIC_COARSE, &mut start_ts);
    start_test_loop(&mut start_ts, (*self_).verbose);
    loop {
        let mut vma_start: c_ulong = 0;
        let mut vma_end: c_ulong = 0;

        ASSERT_TRUE!(read_boundary_lines(self_, &mut new_last_line, &mut new_first_line));

        /* Check if we read vmas after remapping it */
        if strcmp(new_last_line.text.as_ptr(), remapped_last_line.text.as_ptr()) == 0 {
            /*
             * The vmas should be consistent with remap results,
             * however if the vma was concurrently restored, it
             * can be reported twice (first as split one, then
             * as restored one) because we found it as the next vma
             * again. In that case new first line will be the same
             * as the last restored line.
             */
            ASSERT_FALSE!(print_boundaries_on(
                strcmp(new_first_line.text.as_ptr(), remapped_first_line.text.as_ptr()) != 0
                    && strcmp(new_first_line.text.as_ptr(), restored_last_line.text.as_ptr()) != 0,
                b"Remap result invalid\0".as_ptr() as *const c_char,
                self_
            ));
        } else {
            /* The vmas should be consistent with the original/resored state */
            ASSERT_FALSE!(print_boundaries_on(
                strcmp(new_last_line.text.as_ptr(), restored_last_line.text.as_ptr()) != 0,
                b"Remap restore result invalid\0".as_ptr() as *const c_char,
                self_
            ));
            ASSERT_FALSE!(print_boundaries_on(
                strcmp(new_first_line.text.as_ptr(), restored_first_line.text.as_ptr()) != 0,
                b"Remap restore result invalid\0".as_ptr() as *const c_char,
                self_
            ));
        }
        if (*self_).maps_file == maps_file::MAPS {
            /* Check if PROCMAP_QUERY ioclt() finds the right VMA */
            ASSERT_TRUE!(query_addr_at(
                (*self_).maps_fd,
                ((*mod_info).addr as *mut u8).add((*self_).page_size as usize) as *mut c_void,
                &mut vma_start,
                &mut vma_end
            ));
            /*
             * The vma should either stay at the same address and have the
             * original size of 3 pages or we should find the remapped vma
             * at the remap destination address with size of 1 page.
             */
            ASSERT_TRUE!(
                (vma_start == (*self_).last_line.start_addr
                    && vma_end - vma_start == ((*self_).page_size * 3) as c_ulong)
                    || (vma_start == (*self_).last_line.start_addr + (*self_).page_size as c_ulong
                        && vma_end - vma_start == (*self_).page_size as c_ulong)
            );
        }
        clock_gettime(libc::CLOCK_MONOTONIC_COARSE, &mut end_ts);
        end_test_iteration(&mut end_ts, (*self_).verbose);
        if !(end_ts.tv_sec - start_ts.tv_sec < (*self_).duration_sec as libc::time_t) {
            break;
        }
    }
    end_test_loop((*self_).verbose);

    /* Signal the modifyer thread to stop and wait until it exits */
    signal_state(mod_info, test_state::TEST_DONE);
}

fn main() {
    unsafe {
        let variants = [&PROC_MAPS_RACE_MAPS, &PROC_MAPS_RACE_SMAPS];

        for variant in variants {
            let mut self_: proc_maps_race = mem::zeroed();

            proc_maps_race_setup(&mut self_, variant);
            test_maps_tearing_from_split(&mut self_);
            test_maps_tearing_from_resize(&mut self_);
            test_maps_tearing_from_remap(&mut self_);
            proc_maps_race_teardown(&mut self_);
        }
    }
}
