// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the corresponding Linux headers:
// use linux::mmdebug::*;
// use linux::export::*;
// use linux::mm::*;
// use asm::page::*;

/// Convert a virtual address to a physical address.
pub unsafe fn __phys_addr(mut x: usize, is_31bit: bool) -> usize {
    VIRTUAL_BUG_ON!(is_vmalloc_or_module_addr(x as *mut core::ffi::c_void));
    x = __pa_nodebug(x);
    if is_31bit {
        VIRTUAL_BUG_ON!(x >> 31 != 0);
    }
    x
}

// EXPORT_SYMBOL(__phys_addr);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
