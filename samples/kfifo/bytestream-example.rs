// SPDX-License-Identifier: GPL-2.0-only
/*
 * Sample kfifo byte stream implementation
 *
 * Copyright (C) 2010 Stefani Seibold <stefani@seibold.net>
 */

// External Linux kernel declarations supplied by the surrounding repository.

/*
 * This module shows how to create a byte stream fifo.
 */

/* fifo size in elements (bytes) */
const FIFO_SIZE: usize = 32;

/* name of the proc entry */
const PROC_FIFO: &[u8] = b"bytestream-fifo\0";

/* lock for procfs read access */
static mut READ_ACCESS: Mutex = Mutex::new();

/* lock for procfs write access */
static mut WRITE_ACCESS: Mutex = Mutex::new();

/*
 * define DYNAMIC in this example for a dynamically allocated fifo.
 *
 * Otherwise the fifo storage will be a part of the fifo structure.
 */
// #define DYNAMIC is disabled, as in the original source.

#[repr(C)]
pub struct Kfifo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct File {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ProcOps {
    pub proc_read: Option<unsafe extern "C" fn(*mut File, *mut u8, usize, *mut i64) -> isize>,
    pub proc_write: Option<unsafe extern "C" fn(*mut File, *const u8, usize, *mut i64) -> isize>,
    pub proc_lseek: Option<unsafe extern "C" fn(*mut File, i64, i32) -> i64>,
}

extern "C" {
    static mut test: Kfifo;
    fn printk(fmt: *const u8, ...);
    fn kfifo_in(fifo: *mut Kfifo, buf: *const u8, n: usize) -> usize;
    fn kfifo_put(fifo: *mut Kfifo, value: u8) -> bool;
    fn kfifo_len(fifo: *const Kfifo) -> u32;
    fn kfifo_out(fifo: *mut Kfifo, buf: *mut u8, n: usize) -> usize;
    fn kfifo_skip(fifo: *mut Kfifo);
    fn kfifo_peek(fifo: *const Kfifo, value: *mut u8) -> bool;
    fn kfifo_get(fifo: *mut Kfifo, value: *mut u8) -> bool;
    fn mutex_lock_interruptible(lock: *mut Mutex) -> i32;
    fn mutex_unlock(lock: *mut Mutex);
    fn kfifo_from_user(fifo: *mut Kfifo, buf: *const u8, count: usize, copied: *mut u32) -> i32;
    fn kfifo_to_user(fifo: *mut Kfifo, buf: *mut u8, count: usize, copied: *mut u32) -> i32;
    fn noop_llseek(file: *mut File, offset: i64, whence: i32) -> i64;
    fn proc_create(name: *const u8, mode: u32, parent: *mut (), ops: *const ProcOps) -> *mut ();
    fn remove_proc_entry(name: *const u8, parent: *mut ());
}

static EXPECTED_RESULT: [u8; FIFO_SIZE] = [
    3, 4, 5, 6, 7, 8, 9, 0,
    1, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34,
    35, 36, 37, 38, 39, 40, 41, 42,
];

unsafe extern "C" fn testfunc() -> i32 {
    let mut buf = [0u8; 6];
    let mut i: u8;
    let mut j: usize;
    let mut ret: u32;

    printk(b"byte stream fifo test start\n\0".as_ptr());

    kfifo_in(&mut test, b"hello".as_ptr(), 5);

    i = 0;
    while i != 10 {
        kfifo_put(&mut test, i);
        i = i.wrapping_add(1);
    }

    printk(b"fifo len: %u\n\0".as_ptr(), kfifo_len(&test));

    i = kfifo_out(&mut test, buf.as_mut_ptr(), 5) as u8;
    printk(b"buf: %.*s\n\0".as_ptr(), i, buf.as_ptr());

    ret = kfifo_out(&mut test, buf.as_mut_ptr(), 2) as u32;
    printk(b"ret: %d\n\0".as_ptr(), ret);
    ret = kfifo_in(&mut test, buf.as_ptr(), ret as usize) as u32;
    printk(b"ret: %d\n\0".as_ptr(), ret);

    printk(b"skip 1st element\n\0".as_ptr());
    kfifo_skip(&mut test);

    i = 20;
    while kfifo_put(&mut test, i) {
        i = i.wrapping_add(1);
    }

    printk(b"queue len: %u\n\0".as_ptr(), kfifo_len(&test));

    if kfifo_peek(&test, &mut i) {
        printk(b"%d\n\0".as_ptr(), i);
    }

    j = 0;
    while kfifo_get(&mut test, &mut i) {
        printk(b"item = %d\n\0".as_ptr(), i);
        if i != EXPECTED_RESULT[j] {
            printk(b"value mismatch: test failed\n\0".as_ptr());
            return -5; // -EIO
        }
        j += 1;
    }
    if j != EXPECTED_RESULT.len() {
        printk(b"size mismatch: test failed\n\0".as_ptr());
        return -5; // -EIO
    }
    printk(b"test passed\n\0".as_ptr());
    0
}

unsafe extern "C" fn fifo_write(_file: *mut File, buf: *const u8, count: usize, _ppos: *mut i64) -> isize {
    let mut copied = 0u32;
    let ret = mutex_lock_interruptible(&mut WRITE_ACCESS);
    if ret != 0 { return -512; } // -ERESTARTSYS
    let ret = kfifo_from_user(&mut test, buf, count, &mut copied);
    mutex_unlock(&mut WRITE_ACCESS);
    if ret != 0 { return ret as isize; }
    copied as isize
}

unsafe extern "C" fn fifo_read(_file: *mut File, buf: *mut u8, count: usize, _ppos: *mut i64) -> isize {
    let mut copied = 0u32;
    let ret = mutex_lock_interruptible(&mut READ_ACCESS);
    if ret != 0 { return -512; } // -ERESTARTSYS
    let ret = kfifo_to_user(&mut test, buf, count, &mut copied);
    mutex_unlock(&mut READ_ACCESS);
    if ret != 0 { return ret as isize; }
    copied as isize
}

static FIFO_PROC_OPS: ProcOps = ProcOps {
    proc_read: Some(fifo_read),
    proc_write: Some(fifo_write),
    proc_lseek: Some(noop_llseek),
};

unsafe extern "C" fn example_init() -> i32 {
    if testfunc() < 0 { return -5; } // -EIO
    if proc_create(PROC_FIFO.as_ptr(), 0, core::ptr::null_mut(), &FIFO_PROC_OPS).is_null() {
        return -12; // -ENOMEM
    }
    0
}

unsafe extern "C" fn example_exit() {
    remove_proc_entry(PROC_FIFO.as_ptr(), core::ptr::null_mut());
}

// module_init(example_init);
// module_exit(example_exit);
// MODULE_DESCRIPTION("Sample kfifo byte stream implementation");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Stefani Seibold <stefani@seibold.net>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
