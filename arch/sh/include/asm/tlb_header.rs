/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header guard: __ASM_SH_TLB_H
 *
 * The original declarations are available only when not assembling, and the
 * following MMU declarations are available only with CONFIG_MMU enabled.
 */

/* CONFIG_CPU_SH4 selects external implementations of these functions. */
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_CPU_SH4"))]
unsafe extern "C" {
    pub fn tlb_wire_entry(vma: *mut vm_area_struct, addr: core::ffi::c_ulong, pte: pte_t);
    pub fn tlb_unwire_entry();
}

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_CPU_SH4")))]
pub unsafe fn tlb_wire_entry(
    _vma: *mut vm_area_struct,
    _addr: core::ffi::c_ulong,
    _pte: pte_t,
) {
    BUG();
}

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_CPU_SH4")))]
pub unsafe fn tlb_unwire_entry() {
    BUG();
}

#[cfg(feature = "CONFIG_MMU")]
unsafe extern "C" {
    pub fn handle_tlbmiss(
        regs: *mut pt_regs,
        error_code: core::ffi::c_ulong,
        address: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
