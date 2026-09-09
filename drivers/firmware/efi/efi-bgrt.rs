// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2012 Intel Corporation
 * Author: Josh Triplett <josh@joshtriplett.org>
 *
 * Based on the bgrt driver:
 * Copyright 2012 Red Hat, Inc <mjg@redhat.com>
 * Author: Matthew Garrett
 */

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// C dependencies: linux/kernel.h, linux/init.h, linux/acpi.h, linux/efi.h,
// and linux/efi-bgrt.h.

#[repr(C)]
pub struct bmp_header {
    pub id: u16,
    pub size: u32,
}

pub static mut bgrt_tab: acpi_table_bgrt = unsafe { core::mem::zeroed() };
pub static mut bgrt_image_size: usize = 0;

extern "C" {
    static mut acpi_disabled: bool;

    fn efi_enabled(feature: u32) -> bool;
    fn efi_mem_type(address: u64) -> i32;
    fn early_memremap(address: u64, size: usize) -> *mut core::ffi::c_void;
    fn early_memunmap(address: *mut core::ffi::c_void, size: usize);
    fn efi_mem_reserve(address: u64, size: usize);
}

// These types, constants, and the pr_notice! macro are supplied by the
// corresponding kernel headers and build environment.
extern "C" {
    type acpi_table_header;
    type acpi_table_bgrt;
}

pub unsafe fn efi_bgrt_init(table: *mut acpi_table_header) {
    let mut image: *mut core::ffi::c_void;
    let mut bmp_header = bmp_header { id: 0, size: 0 };
    let bgrt: *mut acpi_table_bgrt = core::ptr::addr_of_mut!(bgrt_tab);
    let mem_type: i32;

    if acpi_disabled {
        return;
    }

    if !efi_enabled(EFI_MEMMAP) && !efi_enabled(EFI_PARAVIRT) {
        return;
    }

    if (*table).length < core::mem::size_of::<acpi_table_bgrt>() {
        pr_notice!(
            "Ignoring BGRT: invalid length {} (expected {})\n",
            (*table).length,
            core::mem::size_of::<acpi_table_bgrt>()
        );
        return;
    }
    *bgrt = *(table as *mut acpi_table_bgrt);
    /*
     * Only version 1 is defined but some older laptops (seen on Lenovo
     * Ivy Bridge models) have a correct version 1 BGRT table with the
     * version set to 0, so we accept version 0 and 1.
     */
    if (*bgrt).version > 1 {
        pr_notice!(
            "Ignoring BGRT: invalid version {} (expected 1)\n",
            (*bgrt).version
        );
        goto_out(bgrt);
        return;
    }
    if (*bgrt).image_type != 0 {
        pr_notice!(
            "Ignoring BGRT: invalid image type {} (expected 0)\n",
            (*bgrt).image_type
        );
        goto_out(bgrt);
        return;
    }
    if (*bgrt).image_address == 0 {
        pr_notice!("Ignoring BGRT: null image address\n");
        goto_out(bgrt);
        return;
    }

    mem_type = efi_mem_type((*bgrt).image_address);
    if mem_type != EFI_BOOT_SERVICES_DATA && mem_type != EFI_ACPI_RECLAIM_MEMORY {
        pr_notice!("Ignoring BGRT: invalid image address\n");
        goto_out(bgrt);
        return;
    }
    image = early_memremap((*bgrt).image_address, core::mem::size_of::<bmp_header>());
    if image.is_null() {
        pr_notice!("Ignoring BGRT: failed to map image header memory\n");
        goto_out(bgrt);
        return;
    }

    core::ptr::copy_nonoverlapping(
        image as *const bmp_header,
        &mut bmp_header,
        1,
    );
    early_memunmap(image, core::mem::size_of::<bmp_header>());
    if bmp_header.id != 0x4d42 {
        pr_notice!(
            "Ignoring BGRT: Incorrect BMP magic number 0x{:x} (expected 0x4d42)\n",
            bmp_header.id
        );
        goto_out(bgrt);
        return;
    }
    bgrt_image_size = bmp_header.size as usize;
    efi_mem_reserve((*bgrt).image_address, bgrt_image_size);
    return;
}

#[inline(always)]
unsafe fn goto_out(bgrt: *mut acpi_table_bgrt) {
    core::ptr::write_bytes(bgrt, 0, 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
