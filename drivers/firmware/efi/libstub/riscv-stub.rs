// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 Western Digital Corporation or its affiliates.
 */

// Dependencies supplied by the corresponding Linux EFI and architecture headers:
// linux/efi.h, asm/efi.h, asm/sections.h, linux/unaligned.h, and efistub.h.

extern "C" {
    static _start: u8;
    static _start_kernel: u8;
    static _edata: u8;
    static __init_text_end: u8;
    static _end: u8;

    fn efi_kaslr_relocate_kernel(
        image_addr: *mut c_ulong,
        reserve_addr: *mut c_ulong,
        reserve_size: *mut c_ulong,
        kernel_size: c_ulong,
        kernel_codesize: c_ulong,
        kernel_memsize: c_ulong,
        seed: c_ulong,
    ) -> efi_status_t;
    fn efi_kaslr_get_phys_seed(image_handle: efi_handle_t) -> c_ulong;
    fn efi_err(format: *const c_char, ...);
}

pub unsafe fn stext_offset() -> c_ulong {
    /*
     * When built as part of the kernel, the EFI stub cannot branch to the
     * kernel proper via the image header, as the PE/COFF header is
     * strictly not part of the in-memory presentation of the image, only
     * of the file representation. So instead, we need to jump to the
     * actual entrypoint in the .text region of the image.
     */
    (&_start_kernel as *const u8 as c_ulong).wrapping_sub(&_start as *const u8 as c_ulong)
}

pub unsafe fn handle_kernel_image(
    image_addr: *mut c_ulong,
    image_size: *mut c_ulong,
    reserve_addr: *mut c_ulong,
    reserve_size: *mut c_ulong,
    image: *mut efi_loaded_image_t,
    image_handle: efi_handle_t,
) -> efi_status_t {
    let kernel_size: c_ulong;
    let kernel_codesize: c_ulong;
    let kernel_memsize: c_ulong;
    let status: efi_status_t;

    kernel_size = (&_edata as *const u8 as c_ulong).wrapping_sub(&_start as *const u8 as c_ulong);
    kernel_codesize =
        (&__init_text_end as *const u8 as c_ulong).wrapping_sub(&_start as *const u8 as c_ulong);
    kernel_memsize = kernel_size.wrapping_add(
        (&_end as *const u8 as c_ulong).wrapping_sub(&_edata as *const u8 as c_ulong),
    );
    *image_addr = &_start as *const u8 as c_ulong;
    *image_size = kernel_memsize;
    *reserve_size = *image_size;

    status = efi_kaslr_relocate_kernel(
        image_addr,
        reserve_addr,
        reserve_size,
        kernel_size,
        kernel_codesize,
        kernel_memsize,
        efi_kaslr_get_phys_seed(image_handle),
    );
    if (status != EFI_SUCCESS) {
        efi_err(b"Failed to relocate kernel\n\0".as_ptr() as *const c_char);
        *image_size = 0;
    }

    status
}

pub unsafe fn efi_icache_sync(_start: c_ulong, _end: c_ulong) {
    core::arch::asm!("fence.i", options(nostack, preserves_flags));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
