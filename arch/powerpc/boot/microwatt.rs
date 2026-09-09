// SPDX-License-Identifier: GPL-2.0-or-later

// Dependencies supplied by the surrounding firmware sources:
// stddef.h, stdio.h, types.h, io.h, and ops.h.

// BSS_STACK(8192);

unsafe extern "C" {
    static mut _end: u8;
    static mut _dtb_start: u8;

    fn simple_alloc_init(
        base: *mut core::ffi::c_void,
        heapsize: usize,
        align: usize,
        min_alloc: usize,
    );
    fn fdt_init(dtb: *mut core::ffi::c_void);
    fn serial_console_init();
}

pub unsafe fn platform_init(r3: usize, r4: usize, r5: usize) {
    let _ = (r3, r4, r5);
    let heapsize: usize = 16 * 1024 * 1024 - (&raw mut _end as usize);

    /*
     * Disable interrupts and turn off MSR_RI, since we'll
     * shortly be overwriting the interrupt vectors.
     */
    core::arch::asm!("mtmsrd {0},1", in(reg) 0usize);

    simple_alloc_init(&raw mut _end as *mut core::ffi::c_void, heapsize, 32, 64);
    fdt_init(&raw mut _dtb_start as *mut core::ffi::c_void);
    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
