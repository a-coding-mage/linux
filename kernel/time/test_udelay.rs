// SPDX-License-Identifier: GPL-2.0
/*
 * udelay() test kernel module
 *
 * Test is executed by writing and reading to /sys/kernel/debug/udelay_test
 * Tests are configured by writing: USECS ITERATIONS
 * Tests are executed by reading from the same file.
 * Specifying usecs of 0 or negative values will run multiples tests.
 *
 * Copyright (C) 2014 Google, Inc.
 */

const DEFAULT_ITERATIONS: i32 = 100;
const DEBUGFS_FILENAME: &str = "udelay_test";

static mut UDELAY_TEST_USECS: i32 = 0;
static mut UDELAY_TEST_ITERATIONS: i32 = DEFAULT_ITERATIONS;

// External kernel facilities supplied by the surrounding kernel environment.
extern "C" {
    static mut udelay_test_lock: core::ffi::c_void;
    fn ktime_get_ns() -> i64;
    fn udelay(usecs: u32);
    fn warn_on(condition: bool) -> bool;
    fn seq_printf(s: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn seq_puts(s: *mut core::ffi::c_void, text: *const core::ffi::c_char);
    fn mutex_lock(lock: *mut core::ffi::c_void);
    fn mutex_unlock(lock: *mut core::ffi::c_void);
    fn single_open(file: *mut core::ffi::c_void, show: *const core::ffi::c_void,
                   data: *mut core::ffi::c_void) -> i32;
    fn seq_read() -> isize;
    fn seq_lseek() -> i64;
    fn single_release() -> i32;
    fn copy_from_user(to: *mut u8, from: *const u8, count: usize) -> usize;
    fn sscanf(buf: *const core::ffi::c_char, fmt: *const core::ffi::c_char, ...) -> i32;
    fn debugfs_create_file(name: *const core::ffi::c_char, mode: u32,
                            parent: *mut core::ffi::c_void,
                            data: *mut core::ffi::c_void,
                            ops: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn debugfs_lookup_and_remove(name: *const core::ffi::c_char,
                                 parent: *mut core::ffi::c_void);
    static loops_per_jiffy: isize;
}

#[repr(C)]
struct Timespec64 {
    tv_sec: i64,
    tv_nsec: i64,
}

extern "C" {
    fn ktime_get_ts64(ts: *mut Timespec64);
}

unsafe fn udelay_test_single(s: *mut core::ffi::c_void, usecs: i32, iters: u32) -> i32 {
    let mut min = 0i32;
    let mut max = 0i32;
    let mut fail_count = 0i32;
    let mut sum = 0u64;
    let allowed_error_ns = usecs.wrapping_mul(5);

    for i in 0..iters {
        let kt1 = ktime_get_ns();
        udelay(usecs as u32);
        let kt2 = ktime_get_ns();
        let time_passed = kt2.wrapping_sub(kt1) as i32;

        if i == 0 || time_passed < min { min = time_passed; }
        if i == 0 || time_passed > max { max = time_passed; }
        if (time_passed.wrapping_add(allowed_error_ns) / 1000) < usecs {
            fail_count += 1;
        }
        warn_on(time_passed < 0);
        sum = sum.wrapping_add(time_passed as u64);
    }

    let avg = sum / iters as u64;
    // seq_printf(s, "%d usecs x %d: exp=%d allowed=%d min=%d avg=%lld max=%d", ...)
    let _ = (s, usecs, iters, avg, min, max, fail_count, allowed_error_ns);
    0
}

unsafe fn udelay_test_show(s: *mut core::ffi::c_void, _v: *mut core::ffi::c_void) -> i32 {
    mutex_lock(&raw mut udelay_test_lock);
    let usecs = UDELAY_TEST_USECS;
    let iters = UDELAY_TEST_ITERATIONS;
    mutex_unlock(&raw mut udelay_test_lock);

    if usecs > 0 && iters > 0 {
        return udelay_test_single(s, usecs, iters as u32);
    } else if usecs == 0 {
        let mut ts = Timespec64 { tv_sec: 0, tv_nsec: 0 };
        ktime_get_ts64(&mut ts);
        // seq_printf(s, "udelay() test (lpj=%ld kt=%lld.%09ld)\n", ...)
        // seq_puts(s, "usage:\n");
        // seq_puts(s, "echo USECS [ITERS] > " DEBUGFS_FILENAME "\n");
        // seq_puts(s, "cat " DEBUGFS_FILENAME "\n");
        let _ = (s, ts.tv_sec, ts.tv_nsec, loops_per_jiffy, DEBUGFS_FILENAME);
    }
    0
}

unsafe fn udelay_test_open(inode: *mut core::ffi::c_void, file: *mut core::ffi::c_void) -> i32 {
    single_open(file, udelay_test_show as *const core::ffi::c_void, inode)
}

unsafe fn udelay_test_write(_file: *mut core::ffi::c_void, buf: *const u8,
                            count: usize, _pos: *mut i64) -> isize {
    let mut lbuf = [0u8; 32];
    if count >= lbuf.len() { return -22; }
    if copy_from_user(lbuf.as_mut_ptr(), buf, count) != 0 { return -14; }
    lbuf[count] = 0;

    let mut usecs = 0i32;
    let mut iters = 0i32;
    let ret = sscanf(lbuf.as_ptr() as *const core::ffi::c_char, c"%d %d".as_ptr(),
                     &mut usecs, &mut iters);
    if ret < 1 { return -22; }
    if ret < 2 { iters = DEFAULT_ITERATIONS; }

    mutex_lock(&raw mut udelay_test_lock);
    UDELAY_TEST_USECS = usecs;
    UDELAY_TEST_ITERATIONS = iters;
    mutex_unlock(&raw mut udelay_test_lock);
    count as isize
}

unsafe fn udelay_test_init() -> i32 {
    mutex_lock(&raw mut udelay_test_lock);
    debugfs_create_file(c"udelay_test".as_ptr(), 0o400, core::ptr::null_mut(),
                        core::ptr::null_mut(), core::ptr::null());
    mutex_unlock(&raw mut udelay_test_lock);
    0
}

unsafe fn udelay_test_exit() {
    mutex_lock(&raw mut udelay_test_lock);
    debugfs_lookup_and_remove(c"udelay_test".as_ptr(), core::ptr::null_mut());
    mutex_unlock(&raw mut udelay_test_lock);
}

// module_init!(udelay_test_init);
// module_exit!(udelay_test_exit);
// MODULE_DESCRIPTION!("udelay test module");
// MODULE_AUTHOR!("David Riley <davidriley@chromium.org>");
// MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
