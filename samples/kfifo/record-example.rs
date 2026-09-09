// SPDX-License-Identifier: GPL-2.0-only
/*
 * Sample dynamic sized record fifo implementation
 *
 * Copyright (C) 2010 Stefani Seibold <stefani@seibold.net>
 */

// Linux kernel dependencies supplied by other files.

/*
 * This module shows how to create a variable sized record fifo.
 */

/* fifo size in elements (bytes) */
const FIFO_SIZE: usize = 128;

/* name of the proc entry */
const PROC_FIFO: &[u8] = b"record-fifo\0";

/* lock for procfs read access */
static mut READ_ACCESS: Mutex = DEFINE_MUTEX!();

/* lock for procfs write access */
static mut WRITE_ACCESS: Mutex = DEFINE_MUTEX!();

/*
 * define DYNAMIC in this example for a dynamically allocated fifo.
 *
 * Otherwise the fifo storage will be a part of the fifo structure.
 */
// #if 0
// #define DYNAMIC
// #endif

/*
 * struct kfifo_rec_ptr_1 and STRUCT_KFIFO_REC_1 can handle records of a
 * length between 0 and 255 bytes.
 *
 * struct kfifo_rec_ptr_2 and STRUCT_KFIFO_REC_2 can handle records of a
 * length between 0 and 65535 bytes.
 */

// The DYNAMIC configuration is disabled in the source example.
type Mytest = STRUCT_KFIFO_REC_1!(FIFO_SIZE);
static mut TEST: Mytest = INIT_KFIFO!();

static EXPECTED_RESULT: [&[u8]; 10] = [
    b"a", b"bb", b"ccc", b"dddd", b"eeeee", b"ffffff", b"ggggggg", b"hhhhhhhh",
    b"iiiiiiiii", b"jjjjjjjjjj",
];

unsafe fn testfunc() -> i32 {
    let mut buf = [0i8; 100];
    let mut i: u32;
    let mut ret: u32;
    let hello = [b'h', b'e', b'l', b'l', b'o', 0];

    printk!(KERN_INFO, "record fifo test start\n");

    kfifo_in!(&mut TEST, hello.as_ptr(), core::mem::size_of_val(&hello));

    /* show the size of the next record in the fifo */
    printk!(KERN_INFO, "fifo peek len: %u\n", kfifo_peek_len!(&TEST));

    /* put in variable length data */
    i = 0;
    while i < 10 {
        core::ptr::write_bytes(buf.as_mut_ptr().add(0), (b'a' + i as u8) as i8, (i + 1) as usize);
        kfifo_in!(&mut TEST, buf.as_ptr(), (i + 1) as usize);
        i += 1;
    }

    /* skip first element of the fifo */
    printk!(KERN_INFO, "skip 1st element\n");
    kfifo_skip!(&mut TEST);

    printk!(KERN_INFO, "fifo len: %u\n", kfifo_len!(&TEST));

    /* show the first record without removing from the fifo */
    ret = kfifo_out_peek!(&TEST, buf.as_mut_ptr(), buf.len());
    if ret != 0 {
        printk!(KERN_INFO, "%.*s\n", ret, buf.as_ptr());
    }

    /* check the correctness of all values in the fifo */
    i = 0;
    while !kfifo_is_empty!(&TEST) {
        ret = kfifo_out!(&mut TEST, buf.as_mut_ptr(), buf.len());
        buf[ret as usize] = 0;
        printk!(KERN_INFO, "item = %.*s\n", ret, buf.as_ptr());
        if strcmp!(buf.as_ptr(), EXPECTED_RESULT[i as usize].as_ptr()) != 0 {
            printk!(KERN_WARNING, "value mismatch: test failed\n");
            return -EIO;
        }
        i += 1;
    }
    if i != EXPECTED_RESULT.len() as u32 {
        printk!(KERN_WARNING, "size mismatch: test failed\n");
        return -EIO;
    }
    printk!(KERN_INFO, "test passed\n");
    0
}

unsafe fn fifo_write(file: *mut file, buf: *const u8, count: usize, ppos: *mut loff_t) -> isize {
    let mut copied: u32 = 0;
    if mutex_lock_interruptible!(&mut WRITE_ACCESS) != 0 { return -ERESTARTSYS as isize; }
    let ret = kfifo_from_user!(&mut TEST, buf, count, &mut copied);
    mutex_unlock!(&mut WRITE_ACCESS);
    if ret != 0 { return ret as isize; }
    copied as isize
}

unsafe fn fifo_read(file: *mut file, buf: *mut u8, count: usize, ppos: *mut loff_t) -> isize {
    let mut copied: u32 = 0;
    if mutex_lock_interruptible!(&mut READ_ACCESS) != 0 { return -ERESTARTSYS as isize; }
    let ret = kfifo_to_user!(&mut TEST, buf, count, &mut copied);
    mutex_unlock!(&mut READ_ACCESS);
    if ret != 0 { return ret as isize; }
    copied as isize
}

static FIFO_PROC_OPS: proc_ops = proc_ops {
    proc_read: Some(fifo_read),
    proc_write: Some(fifo_write),
    proc_lseek: Some(noop_llseek),
};

unsafe fn example_init() -> i32 {
    INIT_KFIFO!(&mut TEST);
    if testfunc() < 0 { return -EIO; }
    if proc_create!(PROC_FIFO.as_ptr(), 0, core::ptr::null_mut(), &FIFO_PROC_OPS).is_null() {
        return -ENOMEM;
    }
    0
}

unsafe fn example_exit() {
    remove_proc_entry!(PROC_FIFO.as_ptr(), core::ptr::null_mut());
}

module_init!(example_init);
module_exit!(example_exit);
MODULE_DESCRIPTION!("Sample dynamic sized record fifo implementation");
MODULE_LICENSE!("GPL");
MODULE_AUTHOR!("Stefani Seibold <stefani@seibold.net>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
