// SPDX-License-Identifier: GPL-2.0
/*
 * Definitions and wrapper functions for kernel decompressor
 *
 * Copyright IBM Corp. 2010
 *
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation unit are
// intentionally left external.

/* gzip declarations */
const STATIC: &str = "static";

// C preprocessor configuration selects the heap size at build time.
#[cfg(feature = "CONFIG_KERNEL_BZIP2")]
const BOOT_HEAP_SIZE: usize = 0x400000;
#[cfg(all(not(feature = "CONFIG_KERNEL_BZIP2"), feature = "CONFIG_KERNEL_ZSTD"))]
const BOOT_HEAP_SIZE: usize = 0x30000;
#[cfg(all(not(feature = "CONFIG_KERNEL_BZIP2"), not(feature = "CONFIG_KERNEL_ZSTD")))]
const BOOT_HEAP_SIZE: usize = 0x10000;

extern "C" {
    static mut _end: u8;
    static mut bootdebug: bool;
    fn boot_rb_dump();
    fn boot_panic(format: *const u8, ...);
    static _compressed_start: u8;
    static _compressed_end: u8;
    static vmlinux: Vmlinux;
    fn __decompress(
        inbuf: *const u8,
        len: usize,
        fill: *mut core::ffi::c_void,
        flush: *mut core::ffi::c_void,
        output: *mut core::ffi::c_void,
        output_len: usize,
        error: *mut core::ffi::c_void,
        error_fn: unsafe extern "C" fn(*mut u8),
    );
}

#[repr(C)]
struct Vmlinux {
    image_size: usize,
}

static mut free_mem_ptr: usize = unsafe { &_end as *const u8 as usize };
static mut free_mem_end_ptr: usize = unsafe { &_end as *const u8 as usize } + BOOT_HEAP_SIZE;

// The CONFIG_KERNEL_* branches above correspond to the C source's included
// decompressor implementations. Their definitions are supplied externally.

unsafe extern "C" fn decompress_error(m: *mut u8) {
    if bootdebug {
        boot_rb_dump();
    }
    boot_panic(b"Decompression error: %s\0".as_ptr(), m);
}

#[inline]
unsafe fn align(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

pub unsafe extern "C" fn mem_safe_offset() -> usize {
    align(free_mem_end_ptr, PAGE_SIZE)
}

pub unsafe extern "C" fn deploy_kernel(output: *mut core::ffi::c_void) {
    __decompress(
        &_compressed_start,
        (&_compressed_end as *const u8 as usize) - (&_compressed_start as *const u8 as usize),
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        output,
        vmlinux.image_size,
        core::ptr::null_mut(),
        decompress_error,
    );
}

extern "C" {
    static PAGE_SIZE: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
