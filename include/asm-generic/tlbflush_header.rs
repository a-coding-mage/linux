/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This is a dummy tlbflush implementation that can be used on all
 * nommu architectures.
 * If you have an MMU, you need to write your own functions.
 *
 * The CONFIG_MMU build-time condition is preserved from the C header:
 * an architecture-specific implementation is required when enabled.
 * The symbols supplied by linux/bug.h and other headers are external
 * dependencies and are not implemented here.
 */

/// Opaque declaration supplied by the surrounding kernel translation.
#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn BUG() -> !;
}

pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    let _ = mm;
    BUG();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
