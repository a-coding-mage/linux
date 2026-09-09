// SPDX-License-Identifier: GPL-2.0
//
// Dependencies corresponding to the original Linux kernel headers are supplied
// by other translation units.

use core::ffi::c_void;

extern "C" {
    static mut high_memory: *mut c_void;

    fn __virt_to_phys_nodebug(x: usize) -> phys_addr_t;
    fn __pa_symbol_nodebug(x: usize) -> phys_addr_t;
}

// External kernel type and constants/macros are supplied by the translated
// dependencies: phys_addr_t, PAGE_OFFSET, MAX_DMA_ADDRESS, KERNEL_START, and
// KERNEL_END.

#[inline]
unsafe fn __virt_addr_valid(x: usize) -> bool {
    /*
     * high_memory does not get immediately defined, and there
     * are early callers of __pa() against PAGE_OFFSET
     */
    if high_memory.is_null() && x >= PAGE_OFFSET {
        return true;
    }

    if !high_memory.is_null() && x >= PAGE_OFFSET && x < high_memory as usize {
        return true;
    }

    /*
     * MAX_DMA_ADDRESS is a virtual address that may not correspond to an
     * actual physical address. Enough code relies on __pa(MAX_DMA_ADDRESS)
     * that we just need to work around it and always return true.
     */
    if x == MAX_DMA_ADDRESS {
        return true;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn __virt_to_phys(x: usize) -> phys_addr_t {
    WARN!(
        !__virt_addr_valid(x),
        "virt_to_phys used for non-linear address: %px (%pS)\n",
        x as *mut c_void,
        x as *mut c_void
    );

    __virt_to_phys_nodebug(x)
}
// EXPORT_SYMBOL(__virt_to_phys);

#[no_mangle]
pub unsafe extern "C" fn __phys_addr_symbol(x: usize) -> phys_addr_t {
    /* This is bounds checking against the kernel image only.
     * __pa_symbol should only be used on kernel symbol addresses.
     */
    VIRTUAL_BUG_ON!(
        x < KERNEL_START as usize || x > KERNEL_END as usize
    );

    __pa_symbol_nodebug(x)
}
// EXPORT_SYMBOL(__phys_addr_symbol);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
