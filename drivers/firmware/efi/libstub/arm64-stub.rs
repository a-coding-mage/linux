// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013, 2014 Linaro Ltd;  <roy.franz@linaro.org>
 *
 * This file implements the EFI boot stub for the arm64 kernel.
 * Adapted from ARM version by Mark Salter <msalter@redhat.com>
 */

// Dependencies supplied by the surrounding EFI, architecture, memory,
// section, and EFI stub code are intentionally left as external symbols.

pub unsafe fn handle_kernel_image(
    image_addr: *mut ::core::ffi::c_ulong,
    image_size: *mut ::core::ffi::c_ulong,
    reserve_addr: *mut ::core::ffi::c_ulong,
    reserve_size: *mut ::core::ffi::c_ulong,
    image: *mut efi_loaded_image_t,
    image_handle: efi_handle_t,
) -> efi_status_t {
    let kernel_size: ::core::ffi::c_ulong;
    let kernel_codesize: ::core::ffi::c_ulong;
    let kernel_memsize: ::core::ffi::c_ulong;

    if (*image).image_base != _text {
        efi_err!("FIRMWARE BUG: efi_loaded_image_t::image_base has bogus value\n");
        (*image).image_base = _text;
    }

    if !(((
        _text as u64
    ) & ((SEGMENT_ALIGN as u64) - 1)) == 0) {
        efi_err!(
            "FIRMWARE BUG: kernel image not aligned on %dk boundary\n",
            SEGMENT_ALIGN >> 10
        );
    }

    kernel_size = _edata - _text;
    kernel_codesize = __inittext_end - _text;
    kernel_memsize = kernel_size + (_end - _edata);
    *reserve_size = kernel_memsize;
    *image_addr = _text as ::core::ffi::c_ulong;

    efi_kaslr_relocate_kernel(
        image_addr,
        reserve_addr,
        reserve_size,
        kernel_size,
        kernel_codesize,
        kernel_memsize,
        efi_kaslr_get_phys_seed(image_handle),
    )
}

extern "C" {
    fn primary_entry();
}

pub fn primary_entry_offset() -> ::core::ffi::c_ulong {
    /*
     * When built as part of the kernel, the EFI stub cannot branch to the
     * kernel proper via the image header, as the PE/COFF header is
     * strictly not part of the in-memory presentation of the image, only
     * of the file representation. So instead, we need to jump to the
     * actual entrypoint in the .text region of the image.
     */
    (primary_entry as usize - _text as usize) as ::core::ffi::c_ulong
}

pub unsafe fn efi_icache_sync(
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
) {
    caches_clean_inval_pou(start, end);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
