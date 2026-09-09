// SPDX-License-Identifier: GPL-2.0

/*
 * Early init before relocation
 */

use core::ffi::c_void;

/*
 * We're called here very early in the boot.
 *
 * Note that the kernel may be running at an address which is different
 * from the address that it was linked at, so we must use RELOC/PTRRELOC
 * to access static data (including strings).  -- paulus
 */

unsafe extern "C" {
    static mut kernstart_virt_addr: usize;
    static mut __bss_start: u8;
    static mut __bss_stop: u8;

    fn reloc_offset() -> usize;
    fn PTRRELOC(addr: *mut c_void) -> *mut c_void;
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn identify_cpu(offset: usize, pvr: usize);
    fn mfspr(spr: usize) -> usize;
    fn apply_feature_fixups();
}

pub unsafe fn early_init(dt_ptr: usize) -> usize {
    let _ = dt_ptr;
    let offset = reloc_offset();

    let kva = *(PTRRELOC((&raw mut kernstart_virt_addr).cast::<c_void>())
        .cast::<usize>());

    /* First zero the BSS */
    if kva == KERNELBASE {
        let bss_start = PTRRELOC((&raw mut __bss_start).cast::<c_void>());
        let bss_size = (&raw mut __bss_stop as *mut u8 as usize)
            .wrapping_sub(&raw mut __bss_start as *mut u8 as usize);
        memset(bss_start, 0, bss_size);
    }

    /*
     * Identify the CPU type and fix up code sections
     * that depend on which cpu we have.
     */
    identify_cpu(offset, mfspr(SPRN_PVR));

    apply_feature_fixups();

    kva.wrapping_add(offset)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
