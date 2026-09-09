/* SPDX-License-Identifier: GPL-2.0 */
//! Rust translation of x86/include/asm/paravirt_types.h.
//!
//! The original header is enabled by CONFIG_PARAVIRT and depends on Linux x86
//! types and paravirtualization support supplied by other translation units.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

#[repr(C)]
pub struct thread_struct { _private: [u8; 0] }
#[repr(C)]
pub struct mm_struct { _private: [u8; 0] }
#[repr(C)]
pub struct task_struct { _private: [u8; 0] }
#[repr(C)]
pub struct cpumask { _private: [u8; 0] }
#[repr(C)]
pub struct flush_tlb_info { _private: [u8; 0] }
#[repr(C)]
pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct desc_ptr { _private: [u8; 0] }
#[repr(C)] pub struct desc_struct { _private: [u8; 0] }
#[repr(C)] pub struct gate_desc { _private: [u8; 0] }
#[repr(C)] pub struct pte_t { _private: [u8; 0] }
#[repr(C)] pub struct pmd_t { _private: [u8; 0] }
#[repr(C)] pub struct pud_t { _private: [u8; 0] }
#[repr(C)] pub struct p4d_t { _private: [u8; 0] }
#[repr(C)] pub struct pgd_t { _private: [u8; 0] }
#[repr(C)] pub struct pgprot_t { _private: [u8; 0] }
pub type phys_addr_t = u64;
pub type u32_ = u32;
pub type u64_ = u64;

// struct paravirt_callee_save is declared by asm/paravirt-base.h.
#[repr(C)]
pub struct paravirt_callee_save { pub func: *const core::ffi::c_void }

#[repr(C)]
pub struct pv_cpu_ops {
    pub get_debugreg: Option<unsafe extern "C" fn(i32) -> usize>,
    pub set_debugreg: Option<unsafe extern "C" fn(i32, usize)>,
    pub read_cr0: Option<unsafe extern "C" fn() -> usize>,
    pub write_cr0: Option<unsafe extern "C" fn(usize)>,
    pub write_cr4: Option<unsafe extern "C" fn(usize)>,
    pub load_tr_desc: Option<unsafe extern "C" fn()>,
    pub load_gdt: Option<unsafe extern "C" fn(*const desc_ptr)>,
    pub load_idt: Option<unsafe extern "C" fn(*const desc_ptr)>,
    pub set_ldt: Option<unsafe extern "C" fn(*const core::ffi::c_void, u32)>,
    pub store_tr: Option<unsafe extern "C" fn() -> usize>,
    pub load_tls: Option<unsafe extern "C" fn(*mut thread_struct, u32)>,
    pub load_gs_index: Option<unsafe extern "C" fn(u32)>,
    pub write_ldt_entry: Option<unsafe extern "C" fn(*mut desc_struct, i32, *const core::ffi::c_void)>,
    pub write_gdt_entry: Option<unsafe extern "C" fn(*mut desc_struct, i32, *const core::ffi::c_void, i32)>,
    pub write_idt_entry: Option<unsafe extern "C" fn(*mut gate_desc, i32, *const gate_desc)>,
    pub alloc_ldt: Option<unsafe extern "C" fn(*mut desc_struct, u32)>,
    pub free_ldt: Option<unsafe extern "C" fn(*mut desc_struct, u32)>,
    pub load_sp0: Option<unsafe extern "C" fn(usize)>,
    pub cpuid: Option<unsafe extern "C" fn(*mut u32, *mut u32, *mut u32, *mut u32)>,
    pub read_msr: Option<unsafe extern "C" fn(u32) -> u64>,
    pub write_msr: Option<unsafe extern "C" fn(u32, u64)>,
    pub read_msr_safe: Option<unsafe extern "C" fn(u32, *mut u64) -> i32>,
    pub write_msr_safe: Option<unsafe extern "C" fn(u32, u64) -> i32>,
    pub read_pmc: Option<unsafe extern "C" fn(i32) -> u64>,
    pub start_context_switch: Option<unsafe extern "C" fn(*mut task_struct)>,
    pub end_context_switch: Option<unsafe extern "C" fn(*mut task_struct)>,
}

#[repr(C)]
pub struct pv_irq_ops {
    pub save_fl: paravirt_callee_save,
    pub irq_disable: paravirt_callee_save,
    pub irq_enable: paravirt_callee_save,
    pub safe_halt: Option<unsafe extern "C" fn()>,
    pub halt: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct pv_mmu_ops {
    pub flush_tlb_user: Option<unsafe extern "C" fn()>,
    pub flush_tlb_kernel: Option<unsafe extern "C" fn()>,
    pub flush_tlb_one_user: Option<unsafe extern "C" fn(usize)>,
    pub flush_tlb_multi: Option<unsafe extern "C" fn(*const cpumask, *const flush_tlb_info)>,
    pub exit_mmap: Option<unsafe extern "C" fn(*mut mm_struct)>,
    pub notify_page_enc_status_changed: Option<unsafe extern "C" fn(usize, i32, bool)>,
    pub read_cr2: paravirt_callee_save,
    pub write_cr2: Option<unsafe extern "C" fn(usize)>,
    pub read_cr3: Option<unsafe extern "C" fn() -> usize>,
    pub write_cr3: Option<unsafe extern "C" fn(usize)>,
    pub enter_mmap: Option<unsafe extern "C" fn(*mut mm_struct)>,
    pub pgd_alloc: Option<unsafe extern "C" fn(*mut mm_struct) -> i32>,
    pub pgd_free: Option<unsafe extern "C" fn(*mut mm_struct, *mut pgd_t)>,
    pub alloc_pte: Option<unsafe extern "C" fn(*mut mm_struct, usize)>,
    pub alloc_pmd: Option<unsafe extern "C" fn(*mut mm_struct, usize)>,
    pub alloc_pud: Option<unsafe extern "C" fn(*mut mm_struct, usize)>,
    pub alloc_p4d: Option<unsafe extern "C" fn(*mut mm_struct, usize)>,
    pub release_pte: Option<unsafe extern "C" fn(usize)>,
    pub release_pmd: Option<unsafe extern "C" fn(usize)>,
    pub release_pud: Option<unsafe extern "C" fn(usize)>,
    pub release_p4d: Option<unsafe extern "C" fn(usize)>,
    pub set_pte: Option<unsafe extern "C" fn(*mut pte_t, pte_t)>,
    pub set_pmd: Option<unsafe extern "C" fn(*mut pmd_t, pmd_t)>,
    pub ptep_modify_prot_start: Option<unsafe extern "C" fn(*mut vm_area_struct, usize, *mut pte_t) -> pte_t>,
    pub ptep_modify_prot_commit: Option<unsafe extern "C" fn(*mut vm_area_struct, usize, *mut pte_t, pte_t)>,
    pub pte_val: paravirt_callee_save,
    pub make_pte: paravirt_callee_save,
    pub pgd_val: paravirt_callee_save,
    pub make_pgd: paravirt_callee_save,
    pub set_pud: Option<unsafe extern "C" fn(*mut pud_t, pud_t)>,
    pub pmd_val: paravirt_callee_save,
    pub make_pmd: paravirt_callee_save,
    pub pud_val: paravirt_callee_save,
    pub make_pud: paravirt_callee_save,
    pub set_p4d: Option<unsafe extern "C" fn(*mut p4d_t, p4d_t)>,
    pub p4d_val: paravirt_callee_save,
    pub make_p4d: paravirt_callee_save,
    pub set_pgd: Option<unsafe extern "C" fn(*mut pgd_t, pgd_t)>,
    pub lazy_mode_flush: Option<unsafe extern "C" fn()>,
    pub set_fixmap: Option<unsafe extern "C" fn(u32, phys_addr_t, pgprot_t)>,
}

#[repr(C)]
pub struct paravirt_patch_template { pub cpu: pv_cpu_ops, pub irq: pv_irq_ops, pub mmu: pv_mmu_ops }

extern "C" { pub static mut pv_ops: paravirt_patch_template; }

// The following GCC inline-assembly macros are retained as Rust macro names;
// their architecture-specific implementations are supplied by the asm layer.
macro_rules! PVOP_RETVAL { ($rettype:ty, $eax:expr) => { $eax as $rettype }; }
macro_rules! PVOP_CALL0 { ($rettype:ty, $array:expr, $op:ident) => { compile_error!("PVOP_CALL requires architecture inline assembly") }; }
macro_rules! PVOP_VCALL0 { ($array:expr, $op:ident) => { compile_error!("PVOP_VCALL requires architecture inline assembly") }; }
macro_rules! PVOP_CALLEE0 { ($rettype:ty, $array:expr, $op:ident) => { compile_error!("PVOP_CALLEE requires architecture inline assembly") }; }
macro_rules! PVOP_VCALLEE0 { ($array:expr, $op:ident) => { compile_error!("PVOP_VCALLEE requires architecture inline assembly") }; }

// ALT_NOT_XEN, PV_SAVE_ALL_CALLER_REGS, PV_RESTORE_ALL_CALLER_REGS,
// PV_THUNK_NAME, PV_CALLEE_SAVE_REGS_THUNK, PV_CALLEE_SAVE and
// __PV_IS_CALLEE_SAVE preserve the original assembly/thunk interfaces and
// require the kernel alternative-patching and assembler definitions.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
