// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the corresponding kernel headers:
// linux/bug.h, linux/export.h, linux/types.h, linux/mmdebug.h, linux/mm.h,
// and asm/memory.h.

pub unsafe fn __virt_to_phys(x: ::kernel::ffi::c_ulong) -> phys_addr_t {
    WARN(
        !__is_lm_address(__tag_reset(x)),
        "virt_to_phys used for non-linear address: %p (%pS)\n",
        x as *mut ::kernel::ffi::c_void,
        x as *mut ::kernel::ffi::c_void,
    );

    __virt_to_phys_nodebug(x)
}

// EXPORT_SYMBOL(__virt_to_phys);

pub unsafe fn __phys_addr_symbol(x: ::kernel::ffi::c_ulong) -> phys_addr_t {
    /*
     * This is bounds checking against the kernel image only.
     * __pa_symbol should only be used on kernel symbol addresses.
     */
    VIRTUAL_BUG_ON(
        x < KERNEL_START as ::kernel::ffi::c_ulong
            || x > KERNEL_END as ::kernel::ffi::c_ulong,
    );
    __pa_symbol_nodebug(x)
}

// EXPORT_SYMBOL(__phys_addr_symbol);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
