/* SPDX-License-Identifier: GPL-2.0 */
/* fault_32.c - visible as they are called from assembler */

// `asmlinkage` is a C calling-convention annotation; the declarations below
// use the platform C ABI.
extern "C" {
    pub fn do_sparc_fault(
        regs: *mut pt_regs,
        text_fault: core::ffi::c_int,
        write: core::ffi::c_int,
        address: core::ffi::c_ulong,
    );
}

extern "C" {
    pub fn window_overflow_fault();
    pub fn window_underflow_fault(sp: core::ffi::c_ulong);
    pub fn window_ret_fault(regs: *mut pt_regs);
}

/* srmmu.c */
extern "C" {
    pub static mut srmmu_name: *mut core::ffi::c_char;
    pub static mut viking_mxcc_present: core::ffi::c_int;
    pub static mut flush_page_for_dma_global: core::ffi::c_int;

    pub static mut poke_srmmu: Option<unsafe extern "C" fn()>;
}

// `__init` is a C kernel initialization annotation.
extern "C" {
    pub fn srmmu_paging_init();
}

/* iommu.c */
extern "C" {
    pub fn ld_mmu_iommu();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
