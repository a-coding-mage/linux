// SPDX-License-Identifier: GPL-2.0
// Translated from C source: testing/selftests/cgroup/test_zswap.c
// C dependency intent: linux/limits.h, unistd.h, stdio.h, signal.h, errno.h,
// fcntl.h, sys/sysinfo.h, string.h, sys/wait.h, sys/mman.h, sys/random.h,
// kselftest.h, cgroup_util.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

static mut page_size: c_int = 0;

const PATH_MAX: usize = 4096;
const BUF_SIZE: c_int = 4096;
const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;
const R_OK: c_int = 4;
const F_OK: c_int = 0;
const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_SHARED: c_int = 0x01;
const MAP_ANONYMOUS: c_int = 0x20;
const SIGTERM: c_int = 15;
const EAGAIN: c_int = 11;
const _SC_PAGE_SIZE: c_int = 30;
const MADV_PAGEOUT: c_int = 21;

const PATH_ZSWAP: &[u8] = b"/sys/module/zswap\0";
const PATH_ZSWAP_ENABLED: &[u8] = b"/sys/module/zswap/parameters/enabled\0";
const PATH_ZSWAP_STORED_PAGES: &[u8] = b"/sys/kernel/debug/zswap/stored_pages\0";

const fn mb(x: usize) -> usize {
    x * 1024 * 1024
}

const fn gb(x: usize) -> usize {
    x * 1024 * 1024 * 1024
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct Sysinfo {
    uptime: c_long,
    loads: [usize; 3],
    totalram: usize,
    freeram: usize,
    sharedram: usize,
    bufferram: usize,
    totalswap: usize,
    freeswap: usize,
    procs: u16,
    pad: u16,
    totalhigh: usize,
    freehigh: usize,
    mem_unit: c_uint,
    _f: [c_char; 0],
}

unsafe extern "C" {
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn usleep(usec: c_uint) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn sysinfo(info: *mut Sysinfo) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn pause() -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn waitpid(pid: c_int, wstatus: *mut c_int, options: c_int) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn madvise(addr: *mut c_void, length: usize, advice: c_int) -> c_int;
    fn getrandom(buf: *mut c_void, buflen: usize, flags: c_uint) -> isize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn __errno_location() -> *mut c_int;

    fn cg_name(root: *const c_char, name: *const c_char) -> *mut c_char;
    fn cg_create(cgroup: *const c_char) -> c_int;
    fn cg_destroy(cgroup: *const c_char) -> c_int;
    fn cg_write(cgroup: *const c_char, control: *const c_char, value: *const c_char) -> c_int;
    fn cg_write_numeric(cgroup: *const c_char, control: *const c_char, value: usize) -> c_int;
    fn cg_read_long(cgroup: *const c_char, control: *const c_char) -> c_long;
    fn cg_read_key_long(cgroup: *const c_char, control: *const c_char, key: *const c_char) -> c_long;
    fn cg_read_strcmp(cgroup: *const c_char, control: *const c_char, expected: *const c_char) -> c_int;
    fn cg_read_strstr(cgroup: *const c_char, control: *const c_char, needle: *const c_char) -> c_int;
    fn cg_find_unified_root(root: *mut c_char, len: usize, mount: *mut c_void) -> c_int;
    fn cg_enter_current(cgroup: *const c_char) -> c_int;
    fn cg_run(
        cgroup: *const c_char,
        fn_: unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn cg_run_nowait(
        cgroup: *const c_char,
        fn_: unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn read_text(path: *const c_char, buf: *mut c_char, size: usize) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_finished();
    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_exit_skip(format: *const c_char, ...) -> !;
    fn ksft_exit_fail_msg(format: *const c_char, ...) -> !;
    fn ksft_test_result_pass(format: *const c_char, ...);
    fn ksft_test_result_skip(format: *const c_char, ...);
    fn ksft_test_result_fail(format: *const c_char, ...);
}

unsafe fn map_failed() -> *mut c_void {
    (-1isize) as *mut c_void
}

unsafe fn read_int(path: *const c_char, value: *mut usize) -> c_int {
    let mut ret: c_int = 0;
    let file = fopen(path, c"r".as_ptr());

    if file.is_null() {
        return -1;
    }
    if fscanf(file, c"%ld".as_ptr(), value) != 1 {
        ret = -1;
    }
    fclose(file);
    ret
}

unsafe fn set_min_free_kb(value: usize) -> c_int {
    let file = fopen(c"/proc/sys/vm/min_free_kbytes".as_ptr(), c"w".as_ptr());

    if file.is_null() {
        return -1;
    }
    let ret = fprintf(file, c"%ld\n".as_ptr(), value);
    fclose(file);
    ret
}

unsafe fn read_min_free_kb(value: *mut usize) -> c_int {
    read_int(c"/proc/sys/vm/min_free_kbytes".as_ptr(), value)
}

unsafe fn get_zswap_stored_pages(value: *mut usize) -> c_int {
    read_int(PATH_ZSWAP_STORED_PAGES.as_ptr() as *const c_char, value)
}

unsafe fn get_cg_wb_count(cg: *const c_char) -> c_long {
    cg_read_key_long(cg, c"memory.stat".as_ptr(), c"zswpwb".as_ptr())
}

unsafe fn get_zswpout(cgroup: *const c_char) -> c_long {
    cg_read_key_long(cgroup, c"memory.stat".as_ptr(), c"zswpout ".as_ptr())
}

unsafe extern "C" fn allocate_and_read_bytes(_cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let size = arg as usize;
    let mem = malloc(size) as *mut c_char;
    let mut ret: c_int = 0;

    if mem.is_null() {
        return -1;
    }
    let mut i: c_int = 0;
    while (i as usize) < size {
        *mem.add(i as usize) = b'a' as c_char;
        i += page_size;
    }

    /* Go through the allocated memory to (z)swap in and out pages */
    i = 0;
    while (i as usize) < size {
        if *mem.add(i as usize) != b'a' as c_char {
            ret = -1;
        }
        i += page_size;
    }

    free(mem as *mut c_void);
    ret
}

unsafe extern "C" fn allocate_bytes(_cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let size = arg as usize;
    let mem = malloc(size) as *mut c_char;

    if mem.is_null() {
        return -1;
    }
    let mut i: c_int = 0;
    while (i as usize) < size {
        *mem.add(i as usize) = b'a' as c_char;
        i += page_size;
    }
    free(mem as *mut c_void);
    0
}

unsafe fn setup_test_group_1M(root: *const c_char, name: *const c_char) -> *mut c_char {
    let group_name = cg_name(root, name);

    if group_name.is_null() {
        return ptr::null_mut();
    }
    if cg_create(group_name) != 0 {
        goto_fail(group_name);
        return ptr::null_mut();
    }
    if cg_write(group_name, c"memory.max".as_ptr(), c"1M".as_ptr()) != 0 {
        cg_destroy(group_name);
        goto_fail(group_name);
        return ptr::null_mut();
    }
    group_name
}

unsafe fn goto_fail(group_name: *mut c_char) {
    free(group_name as *mut c_void);
}

/*
 * Writeback is asynchronous; poll until at least one writeback has
 * been recorded for @cg, or until @timeout_ms has elapsed.
 */
unsafe fn wait_for_writeback(cg: *const c_char, timeout_ms: c_int) -> c_long {
    let mut elapsed: c_long = 0;
    while elapsed < timeout_ms as c_long {
        let count = get_cg_wb_count(cg);

        if count < 0 {
            return -1;
        }
        if count > 0 {
            return count;
        }

        usleep(100000);
        elapsed += 100;
    }

    0
}

/*
 * Sanity test to check that pages are written into zswap.
 */
unsafe fn test_zswap_usage(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let test_group = cg_name(root, c"no_shrink_test".as_ptr());

    if test_group.is_null() {
        return ret;
    }
    if cg_create(test_group) != 0 {
        goto_out_destroy_free(test_group);
        return ret;
    }
    if cg_write(test_group, c"memory.max".as_ptr(), c"1M".as_ptr()) != 0 {
        goto_out_destroy_free(test_group);
        return ret;
    }

    let zswpout_before = get_zswpout(test_group);
    if zswpout_before < 0 {
        ksft_print_msg(c"Failed to get zswpout\n".as_ptr());
        goto_out_destroy_free(test_group);
        return ret;
    }

    /* Allocate more than memory.max to push memory into zswap */
    if cg_run(test_group, allocate_bytes, mb(4) as *mut c_void) != 0 {
        goto_out_destroy_free(test_group);
        return ret;
    }

    /* Verify that pages come into zswap */
    let zswpout_after = get_zswpout(test_group);
    if zswpout_after <= zswpout_before {
        ksft_print_msg(c"zswpout does not increase after test program\n".as_ptr());
        goto_out_destroy_free(test_group);
        return ret;
    }
    ret = KSFT_PASS;

    goto_out_destroy_free(test_group);
    ret
}

unsafe fn goto_out_destroy_free(test_group: *mut c_char) {
    cg_destroy(test_group);
    free(test_group as *mut c_void);
}

/*
 * Check that when memory.zswap.max = 0, no pages can go to the zswap pool for
 * the cgroup.
 */
unsafe fn test_swapin_nozswap(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut mem_max_buf = [0 as c_char; 32];
    let allocation_size = page_size as usize * 512;
    let min_swap: c_long = (allocation_size / 4) as c_long;

    snprintf(
        mem_max_buf.as_mut_ptr(),
        mem_max_buf.len(),
        c"%zu".as_ptr(),
        allocation_size * 3 / 4,
    );

    let test_group = cg_name(root, c"no_zswap_test".as_ptr());
    if test_group.is_null() {
        return ret;
    }
    if cg_create(test_group) != 0 {
        goto_out_destroy_free(test_group);
        return ret;
    }
    if cg_write(test_group, c"memory.max".as_ptr(), mem_max_buf.as_ptr()) != 0 {
        goto_out_destroy_free(test_group);
        return ret;
    }
    if cg_write(test_group, c"memory.zswap.max".as_ptr(), c"0".as_ptr()) != 0 {
        goto_out_destroy_free(test_group);
        return ret;
    }

    /* Allocate and read more than memory.max to trigger swapin */
    if cg_run(test_group, allocate_and_read_bytes, allocation_size as *mut c_void) != 0 {
        goto_out_destroy_free(test_group);
        return ret;
    }

    /* Verify that pages are swapped out, but no zswap happened */
    let swap_peak = cg_read_long(test_group, c"memory.swap.peak".as_ptr());
    if swap_peak < 0 {
        ksft_print_msg(c"failed to get cgroup's swap_peak\n".as_ptr());
        goto_out_destroy_free(test_group);
        return ret;
    }

    if swap_peak < min_swap {
        ksft_print_msg(
            c"at least %ldKB of memory should be swapped out\n".as_ptr(),
            min_swap / 1024,
        );
        goto_out_destroy_free(test_group);
        return ret;
    }

    let zswpout = get_zswpout(test_group);
    if zswpout < 0 {
        ksft_print_msg(c"failed to get zswpout\n".as_ptr());
        goto_out_destroy_free(test_group);
        return ret;
    }

    if zswpout > 0 {
        ksft_print_msg(c"zswapout > 0 when memory.zswap.max = 0\n".as_ptr());
        goto_out_destroy_free(test_group);
        return ret;
    }

    ret = KSFT_PASS;

    goto_out_destroy_free(test_group);
    ret
}

/* Simple test to verify the (z)swapin code paths */
unsafe fn test_zswapin(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let test_group = cg_name(root, c"zswapin_test".as_ptr());
    if test_group.is_null() {
        return ret;
    }
    if cg_create(test_group) != 0
        || cg_write(test_group, c"memory.max".as_ptr(), c"8M".as_ptr()) != 0
        || cg_write(test_group, c"memory.zswap.max".as_ptr(), c"max".as_ptr()) != 0
    {
        goto_out_destroy_free(test_group);
        return ret;
    }

    /* Allocate and read more than memory.max to trigger (z)swap in */
    if cg_run(test_group, allocate_and_read_bytes, mb(32) as *mut c_void) != 0 {
        goto_out_destroy_free(test_group);
        return ret;
    }

    let zswpin = cg_read_key_long(test_group, c"memory.stat".as_ptr(), c"zswpin ".as_ptr());
    if zswpin < 0 {
        ksft_print_msg(c"failed to get zswpin\n".as_ptr());
        goto_out_destroy_free(test_group);
        return ret;
    }

    if zswpin < (mb(24) / page_size as usize) as c_long {
        ksft_print_msg(c"at least 24MB should be brought back from zswap\n".as_ptr());
        goto_out_destroy_free(test_group);
        return ret;
    }

    ret = KSFT_PASS;
    goto_out_destroy_free(test_group);
    ret
}

/*
 * Attempt writeback with the following steps:
 * 1. Allocate memory.
 * 2. Reclaim memory equal to the amount that was allocated in step 1.
      This will move it into zswap.
 * 3. Save current zswap usage.
 * 4. Move the memory allocated in step 1 back in from zswap.
 * 5. Set zswap.max to 1/4 of the amount that was recorded in step 3.
 * 6. Attempt to reclaim memory equal to the amount that was allocated,
      this will either trigger writeback if it's enabled, or reclamation
      will fail if writeback is disabled as there isn't enough zswap space.
 */
unsafe extern "C" fn attempt_writeback(cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let memsize = page_size as usize * 1024;
    let mut buf = vec![0 as c_char; page_size as usize];
    let wb_enabled = *(arg as *mut bool);
    let mut ret: c_int = -1;
    let mem = malloc(memsize) as *mut c_char;

    if mem.is_null() {
        return ret;
    }

    /*
     * Fill half of each page with increasing data, and keep other
     * half empty, this will result in data that is still compressible
     * and ends up in zswap, with material zswap usage.
     */
    let mut i: c_int = 0;
    while i < page_size {
        buf[i as usize] = if i < page_size / 2 { i as c_char } else { 0 };
        i += 1;
    }

    i = 0;
    while (i as usize) < memsize {
        memcpy(mem.add(i as usize) as *mut c_void, buf.as_ptr() as *const c_void, page_size as usize);
        i += page_size;
    }

    /* Try and reclaim allocated memory */
    if cg_write_numeric(cgroup, c"memory.reclaim".as_ptr(), memsize) != 0 {
        ksft_print_msg(c"Failed to reclaim all of the requested memory\n".as_ptr());
        free(mem as *mut c_void);
        return ret;
    }

    let zswap_usage = cg_read_long(cgroup, c"memory.zswap.current".as_ptr());

    /* zswpin */
    i = 0;
    while (i as usize) < memsize {
        if memcmp(mem.add(i as usize) as *const c_void, buf.as_ptr() as *const c_void, page_size as usize) != 0 {
            ksft_print_msg(c"invalid memory\n".as_ptr());
            free(mem as *mut c_void);
            return ret;
        }
        i += page_size;
    }

    if cg_write_numeric(cgroup, c"memory.zswap.max".as_ptr(), (zswap_usage / 4) as usize) != 0 {
        free(mem as *mut c_void);
        return ret;
    }

    /*
     * If writeback is enabled, trying to reclaim memory now will trigger a
     * writeback as zswap.max is 1/4 of what was needed when reclaim ran the first time.
     * If writeback is disabled, memory reclaim will fail as zswap is limited and
     * it can't writeback to swap.
     */
    ret = cg_write_numeric(cgroup, c"memory.reclaim".as_ptr(), memsize);
    if !wb_enabled {
        ret = if ret == -EAGAIN { 0 } else { -1 };
    }

    free(mem as *mut c_void);
    ret
}

unsafe fn test_zswap_writeback_one(cgroup: *const c_char, wb: bool) -> c_int {
    let zswpwb_before = get_cg_wb_count(cgroup);
    if zswpwb_before != 0 {
        ksft_print_msg(c"zswpwb_before = %ld instead of 0\n".as_ptr(), zswpwb_before);
        return -1;
    }

    let mut wb_arg = wb;
    if cg_run(cgroup, attempt_writeback, (&mut wb_arg as *mut bool).cast()) != 0 {
        return -1;
    }

    /* Verify that zswap writeback occurred only if writeback was enabled */
    let zswpwb_after = if wb {
        wait_for_writeback(cgroup, 5000)
    } else {
        get_cg_wb_count(cgroup)
    };
    if zswpwb_after < 0 {
        return -1;
    }

    if wb != (zswpwb_after != 0) {
        ksft_print_msg(
            c"zswpwb_after is %ld while wb is %s\n".as_ptr(),
            zswpwb_after,
            if wb { c"enabled".as_ptr() } else { c"disabled".as_ptr() },
        );
        return -1;
    }

    0
}

/* Test to verify the zswap writeback path */
unsafe fn test_zswap_writeback(root: *const c_char, wb: bool) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut test_group_child: *mut c_char = ptr::null_mut();

    if cg_read_strcmp(root, c"memory.zswap.writeback".as_ptr(), c"1".as_ptr()) != 0 {
        return KSFT_SKIP;
    }

    let test_group = cg_name(root, c"zswap_writeback_test".as_ptr());
    if test_group.is_null() {
        return ret;
    }
    if cg_create(test_group) != 0
        || cg_write(test_group, c"memory.zswap.writeback".as_ptr(), if wb { c"1".as_ptr() } else { c"0".as_ptr() }) != 0
    {
        goto_writeback_out(test_group, test_group_child);
        return ret;
    }

    if test_zswap_writeback_one(test_group, wb) != 0 {
        goto_writeback_out(test_group, test_group_child);
        return ret;
    }

    /* Reset memory.zswap.max to max (modified by attempt_writeback), and
     * set up child cgroup, whose memory.zswap.writeback is hardcoded to 1.
     * Thus, the parent's setting shall be what's in effect. */
    if cg_write(test_group, c"memory.zswap.max".as_ptr(), c"max".as_ptr()) != 0
        || cg_write(test_group, c"cgroup.subtree_control".as_ptr(), c"+memory".as_ptr()) != 0
    {
        goto_writeback_out(test_group, test_group_child);
        return ret;
    }

    test_group_child = cg_name(test_group, c"zswap_writeback_test_child".as_ptr());
    if test_group_child.is_null()
        || cg_create(test_group_child) != 0
        || cg_write(test_group_child, c"memory.zswap.writeback".as_ptr(), c"1".as_ptr()) != 0
    {
        goto_writeback_out(test_group, test_group_child);
        return ret;
    }

    if test_zswap_writeback_one(test_group_child, wb) != 0 {
        goto_writeback_out(test_group, test_group_child);
        return ret;
    }

    ret = KSFT_PASS;
    goto_writeback_out(test_group, test_group_child);
    ret
}

unsafe fn goto_writeback_out(test_group: *mut c_char, test_group_child: *mut c_char) {
    if !test_group_child.is_null() {
        cg_destroy(test_group_child);
        free(test_group_child as *mut c_void);
    }
    cg_destroy(test_group);
    free(test_group as *mut c_void);
}

unsafe fn test_zswap_writeback_enabled(root: *const c_char) -> c_int {
    test_zswap_writeback(root, true)
}

unsafe fn test_zswap_writeback_disabled(root: *const c_char) -> c_int {
    test_zswap_writeback(root, false)
}

/*
 * When trying to store a memcg page in zswap, if the memcg hits its memory
 * limit in zswap, writeback should affect only the zswapped pages of that
 * memcg.
 */
unsafe fn test_no_invasive_cgroup_shrink(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let allocation_size = page_size as usize * 1024;
    let nr_pages: c_uint = (allocation_size / page_size as usize) as c_uint;
    let mut zswap_max_buf = [0 as c_char; 32];
    let mut mem_max_buf = [0 as c_char; 32];
    let mut zw_allocation: *mut c_char = ptr::null_mut();
    let mut wb_allocation: *mut c_char = ptr::null_mut();
    let mut zw_group: *mut c_char = ptr::null_mut();

    snprintf(zswap_max_buf.as_mut_ptr(), zswap_max_buf.len(), c"%d".as_ptr(), page_size);
    snprintf(mem_max_buf.as_mut_ptr(), mem_max_buf.len(), c"%zu".as_ptr(), allocation_size / 2);

    let wb_group = setup_test_group_1M(root, c"per_memcg_wb_test1".as_ptr());
    if wb_group.is_null() {
        return KSFT_FAIL;
    }
    if cg_write(wb_group, c"memory.zswap.max".as_ptr(), zswap_max_buf.as_ptr()) != 0
        || cg_write(wb_group, c"memory.max".as_ptr(), mem_max_buf.as_ptr()) != 0
    {
        goto_no_invasive_out(root, zw_group, wb_group, zw_allocation, wb_allocation);
        return ret;
    }

    zw_group = setup_test_group_1M(root, c"per_memcg_wb_test2".as_ptr());
    if zw_group.is_null()
        || cg_write(zw_group, c"memory.max".as_ptr(), mem_max_buf.as_ptr()) != 0
    {
        goto_no_invasive_out(root, zw_group, wb_group, zw_allocation, wb_allocation);
        return ret;
    }

    /* Push some zw_group memory into zswap (simple data, easy to compress) */
    if cg_enter_current(zw_group) != 0 {
        goto_no_invasive_out(root, zw_group, wb_group, zw_allocation, wb_allocation);
        return ret;
    }
    zw_allocation = malloc(allocation_size) as *mut c_char;
    let mut i: c_uint = 0;
    while i < nr_pages {
        let off: c_uint = ((i as c_ulong) * page_size as c_ulong) as c_uint;
        memset(zw_allocation.add(off as usize) as *mut c_void, 0, page_size as usize);
        memset(zw_allocation.add(off as usize) as *mut c_void, b'a' as c_int, (page_size / 4) as usize);
        i += 1;
    }
    if cg_read_key_long(zw_group, c"memory.stat".as_ptr(), c"zswapped".as_ptr()) < 1 {
        goto_no_invasive_out(root, zw_group, wb_group, zw_allocation, wb_allocation);
        return ret;
    }

    /* Push wb_group memory into zswap with hard-to-compress data to trigger wb */
    if cg_enter_current(wb_group) != 0 {
        goto_no_invasive_out(root, zw_group, wb_group, zw_allocation, wb_allocation);
        return ret;
    }
    wb_allocation = malloc(allocation_size) as *mut c_char;
    if wb_allocation.is_null() {
        goto_no_invasive_out(root, zw_group, wb_group, zw_allocation, wb_allocation);
        return ret;
    }
    i = 0;
    while i < nr_pages {
        let off: c_uint = ((i as c_ulong) * page_size as c_ulong) as c_uint;
        memset(wb_allocation.add(off as usize) as *mut c_void, 0, page_size as usize);
        getrandom(wb_allocation.add(off as usize) as *mut c_void, (page_size / 4) as usize, 0);
        i += 1;
    }

    /* Verify that only zswapped memory from gwb_group has been written back */
    if wait_for_writeback(wb_group, 5000) > 0 && get_cg_wb_count(zw_group) == 0 {
        ret = KSFT_PASS;
    }
    goto_no_invasive_out(root, zw_group, wb_group, zw_allocation, wb_allocation);
    ret
}

type c_ulong = usize;

unsafe fn goto_no_invasive_out(
    root: *const c_char,
    zw_group: *mut c_char,
    wb_group: *mut c_char,
    zw_allocation: *mut c_char,
    wb_allocation: *mut c_char,
) {
    cg_enter_current(root);
    if !zw_group.is_null() {
        cg_destroy(zw_group);
        free(zw_group as *mut c_void);
    }
    if !wb_group.is_null() {
        cg_destroy(wb_group);
        free(wb_group as *mut c_void);
    }
    if !zw_allocation.is_null() {
        free(zw_allocation as *mut c_void);
    }
    if !wb_allocation.is_null() {
        free(wb_allocation as *mut c_void);
    }
}

#[repr(C)]
struct no_kmem_bypass_child_args {
    target_alloc_bytes: usize,
    child_allocated: usize,
}

unsafe extern "C" fn no_kmem_bypass_child(_cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let values = arg as *mut no_kmem_bypass_child_args;
    let allocation = malloc((*values).target_alloc_bytes);

    if allocation.is_null() {
        (*values).child_allocated = true as usize;
        return -1;
    }
    let mut i: c_long = 0;
    while (i as usize) < (*values).target_alloc_bytes {
        *((allocation as *mut c_char).add(i as usize)) = b'a' as c_char;
        i += page_size as c_long;
    }
    (*values).child_allocated = true as usize;
    pause();
    free(allocation);
    0
}

/*
 * When pages owned by a memcg are pushed to zswap by kswapd, they should be
 * charged to that cgroup. This wasn't the case before commit
 * cd08d80ecdac("mm: correctly charge compressed memory to its memcg").
 *
 * The test first allocates memory in a memcg, then raises min_free_kbytes to
 * a very high value so that the allocation falls below low wm, then makes
 * another allocation to trigger kswapd that should push the memcg-owned pages
 * to zswap and verifies that the zswap pages are correctly charged.
 *
 * To be run on a VM with at most 4G of memory.
 */
unsafe fn test_no_kmem_bypass(root: *const c_char) -> c_int {
    let mut min_free_kb_original: usize = 0;
    let mut wait_child_iteration: c_int = 0;
    let mut sys_info: Sysinfo = core::mem::zeroed();
    let mut ret = KSFT_FAIL;
    let mut child_status: c_int = 0;
    let mut test_group: *mut c_char = ptr::null_mut();

    /* Read sys info and compute test values accordingly */
    if sysinfo(&mut sys_info) != 0 {
        return KSFT_FAIL;
    }
    if sys_info.totalram > gb(4) {
        ksft_print_msg(
            c"requires less than 4GB total ram, sys_info.totalram: %.1fGB\n".as_ptr(),
            sys_info.totalram as f64 / gb(1) as f64,
        );
        return KSFT_SKIP;
    }
    if access(PATH_ZSWAP_STORED_PAGES.as_ptr() as *const c_char, R_OK) != 0 {
        ksft_print_msg(c"debugfs not mounted at /sys/kernel/debug\n".as_ptr());
        return KSFT_SKIP;
    }
    let values = mmap(
        ptr::null_mut(),
        size_of::<no_kmem_bypass_child_args>(),
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut no_kmem_bypass_child_args;
    if values as *mut c_void == map_failed() {
        return KSFT_FAIL;
    }
    if read_min_free_kb(&mut min_free_kb_original) != 0 {
        return KSFT_FAIL;
    }
    let min_free_kb_high = sys_info.totalram / 2000;
    let min_free_kb_low = sys_info.totalram / 500000;
    (*values).target_alloc_bytes =
        (sys_info.totalram - min_free_kb_high * 1000) + sys_info.totalram * 5 / 100;
    let stored_pages_threshold: c_long = (sys_info.totalram / 5 / page_size as usize) as c_long;
    let trigger_allocation_size = sys_info.totalram / 20;

    /* Set up test memcg */
    test_group = cg_name(root, c"kmem_bypass_test".as_ptr());
    if test_group.is_null() {
        set_min_free_kb(min_free_kb_original);
        cg_destroy(test_group);
        free(test_group as *mut c_void);
        return ret;
    }

    /* Spawn memcg child and wait for it to allocate */
    set_min_free_kb(min_free_kb_low);
    if cg_create(test_group) != 0 {
        set_min_free_kb(min_free_kb_original);
        cg_destroy(test_group);
        free(test_group as *mut c_void);
        return ret;
    }
    (*values).child_allocated = false as usize;
    let child_pid = cg_run_nowait(test_group, no_kmem_bypass_child, values as *mut c_void);
    if child_pid < 0 {
        set_min_free_kb(min_free_kb_original);
        cg_destroy(test_group);
        free(test_group as *mut c_void);
        return ret;
    }
    while (*values).child_allocated == 0 && {
        let old = wait_child_iteration;
        wait_child_iteration += 1;
        old < 10000
    } {
        usleep(1000);
    }

    /* Try to wakeup kswapd and let it push child memory to zswap */
    set_min_free_kb(min_free_kb_high);
    let mut outer_i: c_int = 0;
    while outer_i < 20 {
        let mut stored_pages: usize = 0;
        let trigger_allocation = malloc(trigger_allocation_size) as *mut c_char;

        if trigger_allocation.is_null() {
            break;
        }
        let mut i: c_int = 0;
        while (i as usize) < trigger_allocation_size {
            *trigger_allocation.add(i as usize) = b'b' as c_char;
            i += page_size;
        }
        usleep(100000);
        free(trigger_allocation as *mut c_void);
        if get_zswap_stored_pages(&mut stored_pages) != 0 {
            break;
        }
        if (stored_pages as c_long) < 0 {
            break;
        }
        /* If memory was pushed to zswap, verify it belongs to memcg */
        if (stored_pages as c_long) > stored_pages_threshold {
            let zswapped = cg_read_key_long(test_group, c"memory.stat".as_ptr(), c"zswapped ".as_ptr()) as c_int;
            let delta = (stored_pages * page_size as usize) as c_int - zswapped;
            let result_ok = delta < (stored_pages * page_size as usize / 4) as c_int;

            ret = if result_ok { KSFT_PASS } else { KSFT_FAIL };
            break;
        }
        outer_i += 1;
    }

    kill(child_pid, SIGTERM);
    waitpid(child_pid, &mut child_status, 0);
    set_min_free_kb(min_free_kb_original);
    cg_destroy(test_group);
    free(test_group as *mut c_void);
    ret
}

#[repr(C)]
struct incomp_child_args {
    size: usize,
    pipefd: [c_int; 2],
    madvise_ret: c_int,
    madvise_errno: c_int,
}

unsafe extern "C" fn allocate_random_and_wait(_cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let values = arg as *mut incomp_child_args;
    let size = (*values).size;

    close((*values).pipefd[0]);

    let mem = mmap(
        ptr::null_mut(),
        size,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut c_char;
    if mem as *mut c_void == map_failed() {
        return -1;
    }

    /* Fill with random data from /dev/urandom - incompressible */
    let fd = open(c"/dev/urandom".as_ptr(), O_RDONLY);
    if fd < 0 {
        munmap(mem as *mut c_void, size);
        return -1;
    }

    let mut i: usize = 0;
    while i < size {
        let n = read(fd, mem.add(i) as *mut c_void, size - i);
        if n <= 0 {
            break;
        }
        i += n as usize;
    }
    close(fd);

    /* Touch all pages to ensure they're faulted in */
    i = 0;
    while i < size {
        *mem.add(i) = *mem.add(i);
        i += page_size as usize;
    }

    /* Use MADV_PAGEOUT to push pages into zswap */
    (*values).madvise_ret = madvise(mem as *mut c_void, size, MADV_PAGEOUT);
    (*values).madvise_errno = *__errno_location();

    /* Notify parent that allocation and pageout are done */
    write((*values).pipefd[1], c"x".as_ptr() as *const c_void, 1);
    close((*values).pipefd[1]);

    /* Keep memory alive for parent to check stats */
    pause();
    munmap(mem as *mut c_void, size);
    0
}

unsafe fn get_zswap_incomp(cgroup: *const c_char) -> c_long {
    cg_read_key_long(cgroup, c"memory.stat".as_ptr(), c"zswap_incomp ".as_ptr())
}

/*
 * Test that incompressible pages (random data) are tracked by zswap_incomp.
 *
 * The child process allocates random data within memory.max, then uses
 * MADV_PAGEOUT to push pages into zswap. The parent waits on a pipe for
 * the child to finish, then checks the zswap_incomp stat before the child
 * exits (zswap_incomp is a gauge that decreases on free).
 */
unsafe fn test_zswap_incompressible(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut child_status: c_int = 0;
    let mut buf: c_char = 0;

    let values = mmap(
        ptr::null_mut(),
        size_of::<incomp_child_args>(),
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut incomp_child_args;
    if values as *mut c_void == map_failed() {
        return KSFT_FAIL;
    }

    if pipe((*values).pipefd.as_mut_ptr()) != 0 {
        munmap(values as *mut c_void, size_of::<incomp_child_args>());
        return KSFT_FAIL;
    }

    let test_group = cg_name(root, c"zswap_incompressible_test".as_ptr());
    if test_group.is_null()
        || cg_create(test_group) != 0
        || cg_write(test_group, c"memory.max".as_ptr(), c"32M".as_ptr()) != 0
    {
        goto_incomp_out(test_group, values);
        return ret;
    }

    (*values).size = mb(4);
    let child_pid = cg_run_nowait(test_group, allocate_random_and_wait, values as *mut c_void);
    if child_pid < 0 {
        goto_incomp_out(test_group, values);
        return ret;
    }

    close((*values).pipefd[1]);

    /* Wait for child to finish allocating and pageout */
    read((*values).pipefd[0], (&mut buf as *mut c_char).cast(), 1);
    close((*values).pipefd[0]);

    let zswap_incomp = get_zswap_incomp(test_group);
    if zswap_incomp <= 0 {
        let zswpout = get_zswpout(test_group);
        let zswapped = cg_read_key_long(test_group, c"memory.stat".as_ptr(), c"zswapped ".as_ptr());
        let zswap_b = cg_read_key_long(test_group, c"memory.stat".as_ptr(), c"zswap ".as_ptr());

        ksft_print_msg(c"zswap_incomp not increased: %ld\n".as_ptr(), zswap_incomp);
        ksft_print_msg(
            c"debug: zswpout=%ld zswapped=%ld zswap_b=%ld\n".as_ptr(),
            zswpout,
            zswapped,
            zswap_b,
        );
        ksft_print_msg(
            c"debug: madvise ret=%d errno=%d\n".as_ptr(),
            (*values).madvise_ret,
            (*values).madvise_errno,
        );
        kill(child_pid, SIGTERM);
        waitpid(child_pid, &mut child_status, 0);
        goto_incomp_out(test_group, values);
        return ret;
    }

    ret = KSFT_PASS;

    kill(child_pid, SIGTERM);
    waitpid(child_pid, &mut child_status, 0);
    goto_incomp_out(test_group, values);
    ret
}

unsafe fn goto_incomp_out(test_group: *mut c_char, values: *mut incomp_child_args) {
    cg_destroy(test_group);
    free(test_group as *mut c_void);
    munmap(values as *mut c_void, size_of::<incomp_child_args>());
}

struct zswap_test {
    fn_: unsafe fn(*const c_char) -> c_int,
    name: *const c_char,
}

static mut tests: [zswap_test; 8] = [
    zswap_test { fn_: test_zswap_usage, name: c"test_zswap_usage".as_ptr() },
    zswap_test { fn_: test_swapin_nozswap, name: c"test_swapin_nozswap".as_ptr() },
    zswap_test { fn_: test_zswapin, name: c"test_zswapin".as_ptr() },
    zswap_test { fn_: test_zswap_writeback_enabled, name: c"test_zswap_writeback_enabled".as_ptr() },
    zswap_test { fn_: test_zswap_writeback_disabled, name: c"test_zswap_writeback_disabled".as_ptr() },
    zswap_test { fn_: test_no_kmem_bypass, name: c"test_no_kmem_bypass".as_ptr() },
    zswap_test { fn_: test_no_invasive_cgroup_shrink, name: c"test_no_invasive_cgroup_shrink".as_ptr() },
    zswap_test { fn_: test_zswap_incompressible, name: c"test_zswap_incompressible".as_ptr() },
];

unsafe fn check_zswap_enabled() {
    let mut value = [0 as c_char; 2];

    if access(PATH_ZSWAP.as_ptr() as *const c_char, F_OK) != 0 {
        ksft_exit_skip(c"zswap isn't configured\n".as_ptr());
    }

    if read_text(
        PATH_ZSWAP_ENABLED.as_ptr() as *const c_char,
        value.as_mut_ptr(),
        value.len(),
    ) <= 0
    {
        ksft_exit_fail_msg(c"Failed to read /sys/module/zswap/parameters/enabled\n".as_ptr());
    }

    if value[0] == b'N' as c_char {
        ksft_exit_skip(
            c"zswap is disabled (hint: echo 1 > /sys/module/zswap/parameters/enabled)\n".as_ptr(),
        );
    }
}

unsafe fn real_main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut root = [0 as c_char; PATH_MAX];

    page_size = sysconf(_SC_PAGE_SIZE) as c_int;
    if page_size <= 0 {
        page_size = BUF_SIZE;
    }

    ksft_print_header();
    ksft_set_plan(tests.len() as c_uint);
    if cg_find_unified_root(root.as_mut_ptr(), root.len(), ptr::null_mut()) != 0 {
        ksft_exit_skip(c"cgroup v2 isn't mounted\n".as_ptr());
    }

    check_zswap_enabled();

    /*
     * Check that memory controller is available:
     * memory is listed in cgroup.controllers
     */
    if cg_read_strstr(root.as_ptr(), c"cgroup.controllers".as_ptr(), c"memory".as_ptr()) != 0 {
        ksft_exit_skip(c"memory controller isn't available\n".as_ptr());
    }

    if cg_read_strstr(root.as_ptr(), c"cgroup.subtree_control".as_ptr(), c"memory".as_ptr()) != 0 {
        if cg_write(root.as_ptr(), c"cgroup.subtree_control".as_ptr(), c"+memory".as_ptr()) != 0 {
            ksft_exit_skip(c"Failed to set memory controller\n".as_ptr());
        }
    }

    let mut i = 0usize;
    while i < tests.len() {
        match (tests[i].fn_)(root.as_ptr()) {
            KSFT_PASS => ksft_test_result_pass(c"%s\n".as_ptr(), tests[i].name),
            KSFT_SKIP => ksft_test_result_skip(c"%s\n".as_ptr(), tests[i].name),
            _ => ksft_test_result_fail(c"%s\n".as_ptr(), tests[i].name),
        }
        i += 1;
    }

    ksft_finished();
    0
}

fn main() {
    unsafe {
        real_main(0, ptr::null_mut());
    }
}
