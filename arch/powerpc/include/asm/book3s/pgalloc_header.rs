/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/mm.h.
#[repr(C)]
pub struct mmu_gather {
    _private: [u8; 0],
}

extern "C" {
    pub fn tlb_remove_table(
        tlb: *mut mmu_gather,
        table: *mut core::ffi::c_void,
    );
}

// CONFIG_PPC64 selects <asm/book3s/64/pgalloc.h>; otherwise
// <asm/book3s/32/pgalloc.h> is selected. The selected declarations are
// supplied by the corresponding Rust translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
