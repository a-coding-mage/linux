// SPDX-License-Identifier: GPL-2.0
// Translated from the C implementation; declarations supplied by the kernel
// are referenced externally below.

use core::ffi::c_ulong;

extern "C" {
    fn save_counter();
    fn sync_counter();
    fn csr_read32(reg: u32) -> u32;
    fn csr_write32(value: u32, reg: u32);
    fn csr_read(reg: u32) -> c_ulong;
    fn csr_write(value: c_ulong, reg: u32);
    fn is_fpu_owner() -> bool;
    fn save_fp(task: *mut core::ffi::c_void);
    fn restore_fp(task: *mut core::ffi::c_void);
    static mut current: *mut core::ffi::c_void;
    fn enable_pci_wakeup();
    fn local_flush_tlb_all();
    fn __pa(address: *const core::ffi::c_void) -> c_ulong;
    fn swsusp_asm_suspend() -> i32;
    fn swsusp_asm_resume() -> i32;
    static __nosave_begin: u8;
    static __nosave_end: u8;
}

// These constants are supplied by the architecture headers:
// LOONGARCH_CSR_CRMD, LOONGARCH_CSR_PRMD, LOONGARCH_CSR_EUEN,
// LOONGARCH_CSR_ECFG, and PERCPU_BASE_KS.

#[repr(C)]
pub struct pt_regs {
    _opaque: [u8; 0],
}

static mut saved_crmd: u32 = 0;
static mut saved_prmd: u32 = 0;
static mut saved_euen: u32 = 0;
static mut saved_ecfg: u32 = 0;
static mut saved_pcpu_base: c_ulong = 0;
static mut saved_regs: pt_regs = pt_regs { _opaque: [] };

pub unsafe fn save_processor_state() {
    save_counter();
    saved_crmd = csr_read32(LOONGARCH_CSR_CRMD);
    saved_prmd = csr_read32(LOONGARCH_CSR_PRMD);
    saved_euen = csr_read32(LOONGARCH_CSR_EUEN);
    saved_ecfg = csr_read32(LOONGARCH_CSR_ECFG);
    saved_pcpu_base = csr_read(PERCPU_BASE_KS);

    if is_fpu_owner() {
        save_fp(current);
    }
}

pub unsafe fn restore_processor_state() {
    sync_counter();
    csr_write32(saved_crmd, LOONGARCH_CSR_CRMD);
    csr_write32(saved_prmd, LOONGARCH_CSR_PRMD);
    csr_write32(saved_euen, LOONGARCH_CSR_EUEN);
    csr_write32(saved_ecfg, LOONGARCH_CSR_ECFG);
    csr_write(saved_pcpu_base, PERCPU_BASE_KS);

    if is_fpu_owner() {
        restore_fp(current);
    }
}

pub unsafe fn pfn_is_nosave(pfn: c_ulong) -> i32 {
    let nosave_begin_pfn = __pa((&__nosave_begin as *const u8).cast()) >> 12;
    let nosave_end_pfn = (__pa((&__nosave_end as *const u8).cast()) + 4095) >> 12;

    if (pfn >= nosave_begin_pfn) && (pfn < nosave_end_pfn) {
        1
    } else {
        0
    }
}

pub unsafe fn swsusp_arch_suspend() -> i32 {
    enable_pci_wakeup();
    swsusp_asm_suspend()
}

pub unsafe fn swsusp_arch_resume() -> i32 {
    // Avoid TLB mismatch during and after kernel resume
    local_flush_tlb_all();
    swsusp_asm_resume()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
