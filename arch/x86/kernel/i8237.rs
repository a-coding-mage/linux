// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * 8237A DMA controller suspend functions.
 *
 * Written by Pierre Ossman, 2005.
 */

// Dependency declarations corresponding to the C headers:
// linux/dmi.h, linux/init.h, linux/syscore_ops.h, asm/dma.h, asm/x86_init.h

extern "C" {
    fn claim_dma_lock() -> ::core::ffi::c_ulong;
    fn dma_outb(value: u8, port: u32);
    fn set_dma_addr(channel: i32, address: u32);
    fn set_dma_count(channel: i32, count: u32);
    fn enable_dma(channel: i32);
    fn release_dma_lock(flags: ::core::ffi::c_ulong);
    fn dma_inb(port: u32) -> u8;
    fn x86_pnpbios_disabled() -> bool;
    fn dmi_get_bios_year() -> i32;
    fn register_syscore(core: *const syscore);
}

extern "C" {
    static DMA1_RESET_REG: u32;
    static DMA2_RESET_REG: u32;
    static DMA_PAGE_0: u32;
}

#[repr(C)]
struct syscore_ops {
    resume: Option<unsafe extern "C" fn(data: *mut ::core::ffi::c_void)>,
}

#[repr(C)]
struct syscore {
    ops: *const syscore_ops,
}

const ENODEV: i32 = 19;

unsafe extern "C" fn i8237A_resume(_data: *mut ::core::ffi::c_void) {
    let flags: ::core::ffi::c_ulong;
    let mut i: i32;

    flags = claim_dma_lock();

    dma_outb(0, DMA1_RESET_REG);
    dma_outb(0, DMA2_RESET_REG);

    i = 0;
    while i < 8 {
        set_dma_addr(i, 0x000000);
        /* DMA count is a bit weird so this is not 0 */
        set_dma_count(i, 1);
        i += 1;
    }

    /* Enable cascade DMA or channel 0-3 won't work */
    enable_dma(4);

    release_dma_lock(flags);
}

static i8237_syscore_ops: syscore_ops = syscore_ops {
    resume: Some(i8237A_resume),
};

static mut i8237_syscore: syscore = syscore {
    ops: &i8237_syscore_ops,
};

unsafe extern "C" fn i8237A_init_ops() -> i32 {
    /*
     * From SKL PCH onwards, the legacy DMA device is removed in which the
     * I/O ports (81h-83h, 87h, 89h-8Bh, 8Fh) related to it are removed
     * as well. All removed ports must return 0xff for a inb() request.
     *
     * Note: DMA_PAGE_2 (port 0x81) should not be checked for detecting
     * the presence of DMA device since it may be used by BIOS to decode
     * LPC traffic for POST codes. Original LPC only decodes one byte of
     * port 0x80 but some BIOS may choose to enhance PCH LPC port 0x8x
     * decoding.
     */
    if dma_inb(DMA_PAGE_0) == 0xFF {
        return -ENODEV;
    }

    /*
     * It is not required to load this driver as newer SoC may not
     * support 8237 DMA or bus mastering from LPC. Platform firmware
     * must announce the support for such legacy devices via
     * ACPI_FADT_LEGACY_DEVICES field in FADT table.
     */
    if x86_pnpbios_disabled() && dmi_get_bios_year() >= 2017 {
        return -ENODEV;
    }

    register_syscore(&i8237_syscore);
    0
}

// device_initcall(i8237A_init_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
