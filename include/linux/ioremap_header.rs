/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/kasan.h, asm/pgtable.h, and asm/vmalloc.h.

// Ioremap often, but not always uses the generic vmalloc area. E.g on
// Power ARCH, it could have different ioremap space.

// This branch corresponds to:
// defined(CONFIG_HAS_IOMEM) || defined(CONFIG_GENERIC_IOREMAP)
//
// When IOREMAP_START/IOREMAP_END are not supplied by the target build, the
// corresponding VMALLOC_START/VMALLOC_END values are used.
#[cfg(any(feature = "CONFIG_HAS_IOMEM", feature = "CONFIG_GENERIC_IOREMAP"))]
pub unsafe fn is_ioremap_addr(x: *const core::ffi::c_void) -> bool {
    let addr = kasan_reset_tag(x) as usize;

    addr >= IOREMAP_START && addr < IOREMAP_END
}

// This declaration is supplied by linux/kasan.h.
unsafe extern "C" {
    fn kasan_reset_tag(x: *const core::ffi::c_void) -> *const core::ffi::c_void;
}

// These names are supplied by the target kernel translation. IOREMAP_START
// and IOREMAP_END default to VMALLOC_START and VMALLOC_END when absent.
#[cfg(any(feature = "CONFIG_HAS_IOMEM", feature = "CONFIG_GENERIC_IOREMAP"))]
unsafe extern "C" {
    static IOREMAP_START: usize;
    static IOREMAP_END: usize;
}

#[cfg(not(any(feature = "CONFIG_HAS_IOMEM", feature = "CONFIG_GENERIC_IOREMAP")))]
pub unsafe fn is_ioremap_addr(_x: *const core::ffi::c_void) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
