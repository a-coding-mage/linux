// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the corresponding Linux headers are intentionally
// left as external symbols/macros.

use core::ffi::c_void;

// `phys_addr_t` is the architecture's physical-address integer type.
pub type PhysAddr = u64;

extern "C" {
    fn __virt_to_phys_nodebug(x: *const c_void) -> PhysAddr;
    fn __pa_symbol_nodebug(x: usize) -> PhysAddr;
    static _text: u8;
    static _end: u8;
}

// MAX_DMA_ADDRESS, PAGE_OFFSET, KSEG2, KSEGX, IS_ENABLED(CONFIG_EVA), and
// IS_ENABLED(CONFIG_HIGHMEM) are supplied by the architecture headers.
#[inline]
unsafe fn __debug_virt_addr_valid(x: usize) -> bool {
    /*
     * MAX_DMA_ADDRESS is a virtual address that may not correspond to an
     * actual physical address. Enough code relies on
     * virt_to_phys(MAX_DMA_ADDRESS) that we just need to work around it
     * and always return true.
     */
    if x == MAX_DMA_ADDRESS {
        return true;
    }

    x >= PAGE_OFFSET
        && (KSEGX(x) < KSEG2
            || IS_ENABLED_CONFIG_EVA
            || !IS_ENABLED_CONFIG_HIGHMEM)
}

pub unsafe fn __virt_to_phys(x: *const c_void) -> PhysAddr {
    // WARN(!__debug_virt_addr_valid((unsigned long)x),
    //      "virt_to_phys used for non-linear address: %p (%pS)\\n", x, x);
    if !__debug_virt_addr_valid(x as usize) {
        // The Linux WARN side effect is provided by the surrounding kernel.
        WARN!(true, "virt_to_phys used for non-linear address: %p (%pS)\n", x, x);
    }

    __virt_to_phys_nodebug(x)
}

// EXPORT_SYMBOL(__virt_to_phys);

pub unsafe fn __phys_addr_symbol(x: usize) -> PhysAddr {
    /* This is bounds checking against the kernel image only.
     * __pa_symbol should only be used on kernel symbol addresses.
     */
    VIRTUAL_BUG_ON!(x < (&_text as *const u8) as usize || x > (&_end as *const u8) as usize);

    __pa_symbol_nodebug(x)
}

// EXPORT_SYMBOL(__phys_addr_symbol);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
