// SPDX-License-Identifier: GPL-2.0-only
/*
 * Sample kfifo int type implementation
 *
 * Copyright (C) 2010 Stefani Seibold <stefani@seibold.net>
 */

// Linux kernel dependencies supplied by other files.

/*
 * This module shows how to create a int type fifo.
 */

/* fifo size in elements (ints) */
const FIFO_SIZE: usize = 32;

/* name of the proc entry */
const PROC_FIFO: &str = "int-fifo";

/* lock for procfs read access */
extern "C" {
    static mut read_access: crate::mutex;
    static mut write_access: crate::mutex;
    static mut test: crate::kfifo;
}

extern "C" {
    fn printk(format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn mutex_lock_interruptible(lock: *mut crate::mutex) -> core::ffi::c_int;
    fn mutex_unlock(lock: *mut crate::mutex);
    fn kfifo_put(fifo: *mut crate::kfifo, value: core::ffi::c_int) -> bool;
    fn kfifo_len(fifo: *const crate::kfifo) -> core::ffi::c_uint;
    fn kfifo_out(
        fifo: *mut crate::kfifo,
        buffer: *mut core::ffi::c_void,
        n: core::ffi::c_uint,
    ) -> core::ffi::c_uint;
    fn kfifo_in(
        fifo: *mut crate::kfifo,
        buffer: *const core::ffi::c_void,
        n: core::ffi::c_uint,
    ) -> core::ffi::c_uint;
    fn kfifo_skip(fifo: *mut crate::kfifo);
    fn kfifo_peek(fifo: *const crate::kfifo, value: *mut core::ffi::c_int) -> bool;
    fn kfifo_get(fifo: *mut crate::kfifo, value: *mut core::ffi::c_int) -> bool;
    fn kfifo_from_user(
        fifo: *mut crate::kfifo,
        buffer: *const core::ffi::c_char,
        count: usize,
        copied: *mut core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn kfifo_to_user(
        fifo: *mut crate::kfifo,
        buffer: *mut core::ffi::c_char,
        count: usize,
        copied: *mut core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn proc_create(
        name: *const core::ffi::c_char,
        mode: core::ffi::c_uint,
        parent: *mut crate::proc_dir_entry,
        ops: *const crate::proc_ops,
    ) -> *mut crate::proc_dir_entry;
    fn remove_proc_entry(name: *const core::ffi::c_char, parent: *mut crate::proc_dir_entry);
    fn noop_llseek(
        file: *mut crate::file,
        offset: *mut crate::loff_t,
        whence: core::ffi::c_int,
    ) -> crate::loff_t;
}

static EXPECTED_RESULT: [core::ffi::c_int; FIFO_SIZE] = [
    3, 4, 5, 6, 7, 8, 9, 0,
    1, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34,
    35, 36, 37, 38, 39, 40, 41, 42,
];

unsafe fn testfunc() -> core::ffi::c_int {
    let mut buf = [0 as core::ffi::c_int; 6];
    let mut i: core::ffi::c_int;
    let mut j: usize;
    let mut ret: core::ffi::c_uint;

    printk(b"int fifo test start\0".as_ptr() as *const _);

    i = 0;
    while i != 10 {
        kfifo_put(&raw mut test, i);
        i += 1;
    }

    printk(b"fifo len: %u\n\0".as_ptr() as *const _, kfifo_len(&raw const test));

    ret = kfifo_out(&raw mut test, buf.as_mut_ptr() as *mut _, 2);
    printk(b"ret: %d\n\0".as_ptr() as *const _, ret);
    ret = kfifo_in(&raw mut test, buf.as_ptr() as *const _, ret);
    printk(b"ret: %d\n\0".as_ptr() as *const _, ret);

    printk(b"skip 1st element\n\0".as_ptr() as *const _);
    kfifo_skip(&raw mut test);

    i = 20;
    while kfifo_put(&raw mut test, i) {
        i += 1;
    }

    printk(b"queue len: %u\n\0".as_ptr() as *const _, kfifo_len(&raw const test));

    if kfifo_peek(&raw const test, &raw mut i) {
        printk(b"%d\n\0".as_ptr() as *const _, i);
    }

    j = 0;
    while kfifo_get(&raw mut test, &raw mut i) {
        printk(b"item = %d\n\0".as_ptr() as *const _, i);
        if i != EXPECTED_RESULT[j] {
            printk(b"value mismatch: test failed\n\0".as_ptr() as *const _);
            return -5;
        }
        j += 1;
    }
    if j != EXPECTED_RESULT.len() {
        printk(b"size mismatch: test failed\n\0".as_ptr() as *const _);
        return -5;
    }
    printk(b"test passed\n\0".as_ptr() as *const _);
    0
}

unsafe fn fifo_write(
    file: *mut crate::file,
    buf: *const core::ffi::c_char,
    count: usize,
    ppos: *mut crate::loff_t,
) -> isize {
    let mut copied = 0;
    if mutex_lock_interruptible(&raw mut write_access) != 0 { return -512; }
    let ret = kfifo_from_user(&raw mut test, buf, count, &raw mut copied);
    mutex_unlock(&raw mut write_access);
    if ret != 0 { return ret as isize; }
    copied as isize
}

unsafe fn fifo_read(
    file: *mut crate::file,
    buf: *mut core::ffi::c_char,
    count: usize,
    ppos: *mut crate::loff_t,
) -> isize {
    let mut copied = 0;
    if mutex_lock_interruptible(&raw mut read_access) != 0 { return -512; }
    let ret = kfifo_to_user(&raw mut test, buf, count, &raw mut copied);
    mutex_unlock(&raw mut read_access);
    if ret != 0 { return ret as isize; }
    copied as isize
}

#[no_mangle]
pub unsafe extern "C" fn example_init() -> core::ffi::c_int {
    if testfunc() < 0 { return -5; }
    if proc_create(PROC_FIFO.as_ptr() as *const _, 0, core::ptr::null_mut(), core::ptr::null()) == core::ptr::null_mut() { return -12; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn example_exit() {
    remove_proc_entry(PROC_FIFO.as_ptr() as *const _, core::ptr::null_mut());
}

// module_init(example_init);
// module_exit(example_exit);
// MODULE_DESCRIPTION("Sample kfifo int type implementation");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Stefani Seibold <stefani@seibold.net>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
