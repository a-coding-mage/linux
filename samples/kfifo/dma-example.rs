// SPDX-License-Identifier: GPL-2.0-only
/*
 * Sample fifo dma implementation
 *
 * Copyright (C) 2010 Stefani Seibold <stefani@seibold.net>
 */

// Linux kernel headers supplied by the surrounding kernel build.

use core::ffi::{c_char, c_int, c_uint, c_void};

/* fifo size in elements (bytes) */
const FIFO_SIZE: c_uint = 32;

#[repr(C)]
pub struct Kfifo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Scatterlist {
    _page_link: usize,
    pub offset: c_uint,
    pub length: c_uint,
}

unsafe extern "C" {
    fn printk(fmt: *const c_char, ...);
    fn kfifo_alloc(fifo: *mut Kfifo, size: c_uint, gfp_mask: c_uint) -> c_int;
    fn kfifo_free(fifo: *mut Kfifo);
    fn kfifo_size(fifo: *const Kfifo) -> c_uint;
    fn kfifo_in(fifo: *mut Kfifo, buf: *const c_void, len: c_uint) -> c_uint;
    fn kfifo_put(fifo: *mut Kfifo, val: u32) -> c_int;
    fn kfifo_skip(fifo: *mut Kfifo);
    fn kfifo_len(fifo: *const Kfifo) -> c_uint;
    fn sg_init_table(sgl: *mut Scatterlist, nents: c_uint);
    fn kfifo_dma_in_prepare(
        fifo: *mut Kfifo,
        sgl: *mut Scatterlist,
        nents: c_uint,
        len: c_uint,
    ) -> c_uint;
    fn kfifo_dma_in_finish(fifo: *mut Kfifo, len: c_uint);
    fn kfifo_dma_out_prepare(
        fifo: *mut Kfifo,
        sgl: *mut Scatterlist,
        nents: c_uint,
        len: c_uint,
    ) -> c_uint;
    fn kfifo_dma_out_finish(fifo: *mut Kfifo, len: c_uint);
    fn sg_page(sg: *const Scatterlist) -> *mut c_void;
    fn sg_is_last(sg: *const Scatterlist) -> bool;
}

// These values and symbols are supplied by the kernel headers/build.
unsafe extern "C" {
    static GFP_KERNEL: c_uint;
    static ENOMEM: c_int;
    static EIO: c_int;
}

static mut FIFO: Kfifo = Kfifo { _private: [] };

unsafe fn example_init() -> c_int {
    let mut i: c_int;
    let mut ret: c_uint;
    let mut nents: c_uint;
    let mut sg: [Scatterlist; 10] = [
        Scatterlist { _page_link: 0, offset: 0, length: 0 },
        Scatterlist { _page_link: 0, offset: 0, length: 0 },
        Scatterlist { _page_link: 0, offset: 0, length: 0 },
        Scatterlist { _page_link: 0, offset: 0, length: 0 },
        Scatterlist { _page_link: 0, offset: 0, length: 0 },
        Scatterlist { _page_link: 0, offset: 0, length: 0 },
        Scatterlist { _page_link: 0, offset: 0, length: 0 },
        Scatterlist { _page_link: 0, offset: 0, length: 0 },
        Scatterlist { _page_link: 0, offset: 0, length: 0 },
        Scatterlist { _page_link: 0, offset: 0, length: 0 },
    ];

    printk(b"DMA fifo test start\0".as_ptr() as *const c_char);

    if kfifo_alloc(&raw mut FIFO, FIFO_SIZE, GFP_KERNEL) != 0 {
        printk(b"error kfifo_alloc\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }

    printk(b"queue size: %u\n\0".as_ptr() as *const c_char, kfifo_size(&raw const FIFO));

    kfifo_in(&raw mut FIFO, b"test".as_ptr() as *const c_void, 4);

    i = 0;
    while i != 9 {
        kfifo_put(&raw mut FIFO, i as u32);
        i += 1;
    }

    /* kick away first byte */
    kfifo_skip(&raw mut FIFO);

    printk(b"queue len: %u\n\0".as_ptr() as *const c_char, kfifo_len(&raw const FIFO));

    /* Configure the kfifo buffer to receive data from DMA input. */
    sg_init_table(sg.as_mut_ptr(), sg.len() as c_uint);
    nents = kfifo_dma_in_prepare(&raw mut FIFO, sg.as_mut_ptr(), sg.len() as c_uint, FIFO_SIZE);
    printk(b"DMA sgl entries: %d\n\0".as_ptr() as *const c_char, nents);
    if nents == 0 {
        /* fifo is full and no sgl was created */
        printk(b"error kfifo_dma_in_prepare\n\0".as_ptr() as *const c_char);
        return -EIO;
    }

    /* receive data */
    printk(b"scatterlist for receive:\n\0".as_ptr() as *const c_char);
    i = 0;
    while (i as c_uint) < nents {
        let entry = &sg[i as usize];
        printk(
            b"sg[%d] -> page %p offset 0x%.8x length 0x%.8x\n\0".as_ptr() as *const c_char,
            i,
            sg_page(entry),
            entry.offset,
            entry.length,
        );
        if sg_is_last(entry) {
            break;
        }
        i += 1;
    }

    /* put here your code to setup and exectute the dma operation */
    /* ... */

    /* example: zero bytes received */
    ret = 0;

    /* finish the dma operation and update the received data */
    kfifo_dma_in_finish(&raw mut FIFO, ret);

    /* Prepare to transmit data, example: 8 bytes */
    nents = kfifo_dma_out_prepare(&raw mut FIFO, sg.as_mut_ptr(), sg.len() as c_uint, 8);
    printk(b"DMA sgl entries: %d\n\0".as_ptr() as *const c_char, nents);
    if nents == 0 {
        /* no data was available and no sgl was created */
        printk(b"error kfifo_dma_out_prepare\n\0".as_ptr() as *const c_char);
        return -EIO;
    }

    printk(b"scatterlist for transmit:\n\0".as_ptr() as *const c_char);
    i = 0;
    while (i as c_uint) < nents {
        let entry = &sg[i as usize];
        printk(
            b"sg[%d] -> page %p offset 0x%.8x length 0x%.8x\n\0".as_ptr() as *const c_char,
            i,
            sg_page(entry),
            entry.offset,
            entry.length,
        );
        if sg_is_last(entry) {
            break;
        }
        i += 1;
    }

    /* put here your code to setup and exectute the dma operation */
    /* ... */

    /* example: 5 bytes transmitted */
    ret = 5;

    /* finish the dma operation and update the transmitted data */
    kfifo_dma_out_finish(&raw mut FIFO, ret);

    ret = kfifo_len(&raw const FIFO);
    printk(b"queue len: %u\n\0".as_ptr() as *const c_char, kfifo_len(&raw const FIFO));

    if ret != 7 {
        printk(b"size mismatch: test failed\0".as_ptr() as *const c_char);
        return -EIO;
    }
    printk(b"test passed\n\0".as_ptr() as *const c_char);

    0
}

unsafe fn example_exit() {
    kfifo_free(&raw mut FIFO);
}

// Equivalent kernel registration/metadata declarations:
// module_init(example_init);
// module_exit(example_exit);
// MODULE_DESCRIPTION("Sample fifo dma implementation");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Stefani Seibold <stefani@seibold.net>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
