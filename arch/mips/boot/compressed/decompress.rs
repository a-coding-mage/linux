// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2001 MontaVista Software Inc.
 * Author: Matt Porter <mporter@mvista.com>
 *
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Wu Zhangjin <wuzhangjin@gmail.com>
 */

// DISABLE_BRANCH_PROFILING
// __NO_FORTIFY
// C header dependencies are supplied by the surrounding build.

extern "C" {
    static __image_begin: u8;
    static __image_end: u8;
    static __appended_dtb: u8;

    fn puts(x: *const core::ffi::c_char);
    fn puthex(x: c_ulong);
    fn __decompress(
        input: *mut core::ffi::c_char,
        input_len: c_ulong,
        fill: c_ulong,
        flush: c_ulong,
        output: *mut core::ffi::c_void,
        output_len: c_ulong,
        error: c_ulong,
        error_fn: unsafe extern "C" fn(*mut core::ffi::c_char),
    );
    fn fdt_magic(fdt: *const core::ffi::c_void) -> u32;
    fn fdt_totalsize(fdt: *const core::ffi::c_void) -> u32;
}

type c_ulong = usize;

// These two variables specify the free mem region that can be used for
// temporary malloc area.
#[no_mangle]
pub static mut free_mem_ptr: c_ulong = 0;
#[no_mangle]
pub static mut free_mem_end_ptr: c_ulong = 0;

#[inline(always)]
unsafe fn c_puts(s: &'static [u8]) {
    puts(s.as_ptr() as *const core::ffi::c_char);
}

#[no_mangle]
pub unsafe extern "C" fn error(x: *mut core::ffi::c_char) {
    c_puts(b"\n\n\0");
    puts(x);
    c_puts(b"\n\n -- System halted\0");

    loop {
        core::hint::spin_loop();
    }
}

// activate the code for pre-boot environment
// STATIC static

// The algorithm-specific decompressor sources included by the C file are
// provided by the corresponding Rust build configuration.

#[no_mangle]
pub static __stack_chk_guard: c_ulong = 0x000a0dff;

#[no_mangle]
pub unsafe extern "C" fn __stack_chk_fail() {
    error(b"stack-protector: Kernel stack is corrupted\n\0".as_ptr() as *mut _);
}

// BOOT_HEAP_SIZE, VMLINUX_LOAD_ADDRESS_ULL, STRUCT_ALIGNMENT, FDT_MAGIC, and
// the linker symbols below are supplied by the target build.
extern "C" {
    static BOOT_HEAP_SIZE: c_ulong;
    static VMLINUX_LOAD_ADDRESS_ULL: c_ulong;
    static STRUCT_ALIGNMENT: c_ulong;
    static FDT_MAGIC: u32;
}

#[no_mangle]
pub unsafe extern "C" fn decompress_kernel(boot_heap_start: c_ulong) {
    let zimage_start: c_ulong = (&__image_begin as *const u8) as c_ulong;
    let zimage_size: c_ulong = (&__image_end as *const u8 as c_ulong).wrapping_sub(zimage_start);

    c_puts(b"zimage at:     \0");
    puthex(zimage_start);
    c_puts(b" \0");
    puthex(zimage_size.wrapping_add(zimage_start));
    c_puts(b"\n\0");

    // This area are prepared for mallocing when decompressing
    free_mem_ptr = boot_heap_start;
    free_mem_end_ptr = boot_heap_start.wrapping_add(BOOT_HEAP_SIZE);

    // Display standard Linux/MIPS boot prompt
    c_puts(b"Uncompressing Linux at load address \0");
    puthex(VMLINUX_LOAD_ADDRESS_ULL);
    c_puts(b"\n\0");

    // Decompress the kernel with according algorithm
    __decompress(
        zimage_start as *mut core::ffi::c_char,
        zimage_size,
        0,
        0,
        VMLINUX_LOAD_ADDRESS_ULL as *mut core::ffi::c_void,
        0,
        0,
        error,
    );

    if cfg!(feature = "CONFIG_MIPS_RAW_APPENDED_DTB")
        && fdt_magic(&__appended_dtb as *const u8 as *const core::ffi::c_void) == FDT_MAGIC
    {
        let dtb_size = fdt_totalsize(&__appended_dtb as *const u8 as *const core::ffi::c_void);
        // last four bytes is always image size in little endian
        let image_size_ptr = (&__image_end as *const u8).sub(4);
        let mut image_size = u32::from_le_bytes(*image_size_ptr.cast::<[u8; 4]>()) as c_ulong;

        // The device tree's address must be properly aligned
        image_size = (image_size.wrapping_add(STRUCT_ALIGNMENT - 1))
            & !(STRUCT_ALIGNMENT - 1);

        c_puts(b"Copy device tree to address  \0");
        puthex(VMLINUX_LOAD_ADDRESS_ULL.wrapping_add(image_size));
        c_puts(b"\n\0");

        // copy dtb to where the booted kernel will expect it
        core::ptr::copy_nonoverlapping(
            &__appended_dtb as *const u8,
            (VMLINUX_LOAD_ADDRESS_ULL.wrapping_add(image_size)) as *mut u8,
            dtb_size as usize,
        );
    }

    // FIXME: should we flush cache here?
    c_puts(b"Now, booting the kernel...\n\0");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
