// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018-2022 Intel Corporation
//
// Author: Keyon Jie <yang.jie@linux.intel.com>
//

use core::ffi::{c_char, c_int, c_uchar, c_void};
use core::ptr;

// C dependencies:
// #include <linux/unaligned.h>
// #include <linux/io-64-nonatomic-lo-hi.h>
// #include <linux/device.h>
// #include <sound/memalloc.h>
// #include <linux/module.h>
// #include "sof-utils.h"

pub type size_t = usize;
pub type u8 = u8;
pub type u32 = u32;

// PAGE_SIZE and PAGE_SHIFT are supplied by the kernel build. Their exact values
// are architecture-dependent in C, so they remain dependency constants here.
unsafe extern "C" {
    static PAGE_SIZE: size_t;
    static PAGE_SHIFT: c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub area: *mut c_void,
}

unsafe extern "C" {
    fn snd_sgbuf_aligned_pages(size: size_t) -> c_int;
    fn snd_sgbuf_get_addr(dmab: *mut snd_dma_buffer, offset: size_t) -> u64;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

unsafe fn put_unaligned_le32(value: u32, ptr: *mut u8) {
    ptr::write_unaligned(ptr as *mut u32, value.to_le());
}

/*
 * Generic buffer page table creation.
 * Take the each physical page address and drop the least significant unused
 * bits from each (based on PAGE_SIZE). Then pack valid page address bits
 * into compressed page table.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_create_page_table(
    dev: *mut device,
    dmab: *mut snd_dma_buffer,
    page_table: *mut c_uchar,
    size: size_t,
) -> c_int {
    let mut i: c_int;
    let pages: c_int;

    pages = unsafe { snd_sgbuf_aligned_pages(size) };

    unsafe {
        dev_dbg(
            dev,
            c"generating page table for %p size 0x%zx pages %d\n".as_ptr(),
            (*dmab).area,
            size,
            pages,
        );
    }

    i = 0;
    while i < pages {
        /*
         * The number of valid address bits for each page is 20.
         * idx determines the byte position within page_table
         * where the current page's address is stored
         * in the compressed page_table.
         * This can be calculated by multiplying the page number by 2.5.
         */
        let idx: u32 = ((5 * i) >> 1) as u32;
        let pfn: u32 = (unsafe {
            snd_sgbuf_get_addr(
                dmab,
                (i as size_t).wrapping_mul(unsafe { PAGE_SIZE }),
            ) >> unsafe { PAGE_SHIFT }
        }) as u32;
        let pg_table: *mut u8;

        pg_table = unsafe { page_table.add(idx as usize) as *mut u8 };

        /*
         * pagetable compression:
         * byte 0     byte 1     byte 2     byte 3     byte 4     byte 5
         * ___________pfn 0__________ __________pfn 1___________  _pfn 2...
         * .... ....  .... ....  .... ....  .... ....  .... ....  ....
         * It is created by:
         * 1. set current location to 0, PFN index i to 0
         * 2. put pfn[i] at current location in Little Endian byte order
         * 3. calculate an intermediate value as
         *    x = (pfn[i+1] << 4) | (pfn[i] & 0xf)
         * 4. put x at offset (current location + 2) in LE byte order
         * 5. increment current location by 5 bytes, increment i by 2
         * 6. continue to (2)
         */
        if (i & 1) != 0 {
            unsafe {
                put_unaligned_le32(
                    ((*pg_table.add(0) as u32) & 0xf) | pfn.wrapping_shl(4),
                    pg_table,
                );
            }
        } else {
            unsafe {
                put_unaligned_le32(pfn, pg_table);
            }
        }

        i += 1;
    }

    pages
}

// EXPORT_SYMBOL(snd_sof_create_page_table);
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF utils");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
