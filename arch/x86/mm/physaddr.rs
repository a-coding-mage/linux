// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation unit.

#[cfg(feature = "CONFIG_X86_64")]
extern "C" {
    static __START_KERNEL_map: usize;
    static phys_base: usize;
    static PAGE_OFFSET: usize;
    static KERNEL_IMAGE_SIZE: usize;
    static PAGE_SHIFT: u32;

    fn phys_addr_valid(x: usize) -> bool;
    fn pfn_valid(x: usize) -> bool;
}

#[cfg(not(feature = "CONFIG_X86_64"))]
extern "C" {
    static PAGE_OFFSET: usize;
    static FIXADDR_START: usize;
    static PAGE_SHIFT: u32;
    static __vmalloc_start_set: bool;
    static max_low_pfn: usize;

    fn is_vmalloc_addr(x: *const core::ffi::c_void) -> bool;
    fn slow_virt_to_phys(x: *const core::ffi::c_void) -> usize;
}

#[cfg(all(feature = "CONFIG_X86_64", feature = "CONFIG_DEBUG_VIRTUAL"))]
#[no_mangle]
pub unsafe extern "C" fn __phys_addr(mut x: usize) -> usize {
    let mut y = x.wrapping_sub(__START_KERNEL_map);

    /* use the carry flag to determine if x was < __START_KERNEL_map */
    if x > y {
        x = y.wrapping_add(phys_base);

        // VIRTUAL_BUG_ON(y >= KERNEL_IMAGE_SIZE);
        debug_assert!(y < KERNEL_IMAGE_SIZE);
    } else {
        x = y.wrapping_add(__START_KERNEL_map.wrapping_sub(PAGE_OFFSET));

        /* carry flag will be set if starting x was >= PAGE_OFFSET */
        // VIRTUAL_BUG_ON((x > y) || !phys_addr_valid(x));
        debug_assert!((x <= y) && phys_addr_valid(x));
    }

    x
}

#[cfg(feature = "CONFIG_X86_64")]
#[no_mangle]
pub unsafe extern "C" fn __virt_addr_valid(mut x: usize) -> bool {
    let y = x.wrapping_sub(__START_KERNEL_map);

    /* use the carry flag to determine if x was < __START_KERNEL_map */
    if x > y {
        x = y.wrapping_add(phys_base);

        if y >= KERNEL_IMAGE_SIZE {
            return false;
        }
    } else {
        x = y.wrapping_add(__START_KERNEL_map.wrapping_sub(PAGE_OFFSET));

        /* carry flag will be set if starting x was >= PAGE_OFFSET */
        if (x > y) || !phys_addr_valid(x) {
            return false;
        }
    }

    pfn_valid(x >> PAGE_SHIFT)
}

#[cfg(all(not(feature = "CONFIG_X86_64"), feature = "CONFIG_DEBUG_VIRTUAL"))]
#[no_mangle]
pub unsafe extern "C" fn __phys_addr(mut x: usize) -> usize {
    let phys_addr = x.wrapping_sub(PAGE_OFFSET);
    /* VMALLOC_* aren't constants  */
    // VIRTUAL_BUG_ON(x < PAGE_OFFSET);
    debug_assert!(x >= PAGE_OFFSET);
    // VIRTUAL_BUG_ON(__vmalloc_start_set && is_vmalloc_addr((void *) x));
    debug_assert!(!__vmalloc_start_set || !is_vmalloc_addr(x as *const core::ffi::c_void));
    /* max_low_pfn is set early, but not _that_ early */
    if max_low_pfn != 0 {
        // VIRTUAL_BUG_ON((phys_addr >> PAGE_SHIFT) > max_low_pfn);
        debug_assert!((phys_addr >> PAGE_SHIFT) <= max_low_pfn);
        // BUG_ON(slow_virt_to_phys((void *)x) != phys_addr);
        assert!(slow_virt_to_phys(x as *const core::ffi::c_void) == phys_addr);
    }
    phys_addr
}

#[cfg(not(feature = "CONFIG_X86_64"))]
#[no_mangle]
pub unsafe extern "C" fn __virt_addr_valid(x: usize) -> bool {
    if x < PAGE_OFFSET {
        return false;
    }
    if __vmalloc_start_set && is_vmalloc_addr(x as *const core::ffi::c_void) {
        return false;
    }
    if x >= FIXADDR_START {
        return false;
    }
    pfn_valid((x.wrapping_sub(PAGE_OFFSET)) >> PAGE_SHIFT)
}

// EXPORT_SYMBOL(__phys_addr);
// EXPORT_SYMBOL(__virt_addr_valid);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
