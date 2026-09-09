// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation unit:
// `phys_addr_t`, `kernel_map`, `is_linear_mapping`, `is_kernel_mapping`,
// `__va_to_pa_nodebug`, `WARN!`, `VIRTUAL_BUG_ON!`, and `BUG_ON!`.

pub unsafe fn __virt_to_phys(x: ::core::ffi::c_ulong) -> phys_addr_t {
    /*
     * Boundary checking against the kernel linear mapping space.
     */
    WARN!(
        !is_linear_mapping(x) && !is_kernel_mapping(x),
        "virt_to_phys used for non-linear address: %p (%pS)\n",
        x as *const ::core::ffi::c_void,
        x as *const ::core::ffi::c_void,
    );

    __va_to_pa_nodebug(x)
}

// EXPORT_SYMBOL(__virt_to_phys);

pub unsafe fn __phys_addr_symbol(x: ::core::ffi::c_ulong) -> phys_addr_t {
    let kernel_start = kernel_map.virt_addr;
    let kernel_end = kernel_start + kernel_map.size;

    /*
     * Boundary checking against the kernel image mapping.
     * __pa_symbol should only be used on kernel symbol addresses.
     */
    VIRTUAL_BUG_ON!(x < kernel_start || x > kernel_end);

    __va_to_pa_nodebug(x)
}

// EXPORT_SYMBOL(__phys_addr_symbol);

pub unsafe fn linear_mapping_va_to_pa(x: ::core::ffi::c_ulong) -> phys_addr_t {
    BUG_ON!(!kernel_map.va_pa_offset);

    x - kernel_map.va_pa_offset
}

// EXPORT_SYMBOL(linear_mapping_va_to_pa);

pub unsafe fn linear_mapping_pa_to_va(x: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void {
    BUG_ON!(!kernel_map.va_pa_offset);

    (x + kernel_map.va_pa_offset) as *mut ::core::ffi::c_void
}

// EXPORT_SYMBOL(linear_mapping_pa_to_va);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
