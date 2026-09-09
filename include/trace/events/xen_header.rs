/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/xen.h.  The tracepoint machinery and the
// kernel types referenced below are supplied by the surrounding kernel crate.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;
pub type c_ulong = usize;

#[repr(C)]
pub struct multicall_entry;

// TRACE_DEFINE_SIZEOF(ulong), pteval_t, pmdval_t, pudval_t and p4dval_t are
// represented by the corresponding target-platform types in the dependencies.

#[repr(C)] pub struct xen_mc_batch_entry { pub flags: c_ulong }
#[repr(C)] pub struct xen_mc_issue_entry { pub flags: c_ulong, pub flush: bool }
#[repr(C)] pub struct xen_mc_entry_entry { pub op: u32, pub nargs: u32, pub args: [c_ulong; 6] }
#[repr(C)] pub struct xen_mc_entry_alloc_entry { pub args: usize }
#[repr(C)] pub struct xen_mc_callback_entry { pub fn_: xen_mc_callback_fn_t, pub data: *mut c_void }
#[repr(C)] pub struct xen_mc_flush_reason_entry { pub reason: xen_mc_flush_reason }
#[repr(C)] pub struct xen_mc_flush_entry { pub mcidx: u32, pub argidx: u32, pub cbidx: u32 }
#[repr(C)] pub struct xen_mc_extend_args_entry { pub op: u32, pub args: usize, pub res: xen_mc_extend_args }

#[repr(C)] pub struct xen_mmu_set_pte_entry { pub ptep: *mut pte_t, pub pteval: pteval_t }
#[repr(C)] pub struct xen_mmu_set_pmd_entry { pub pmdp: *mut pmd_t, pub pmdval: pmdval_t }
#[repr(C)] pub struct xen_mmu_set_pud_entry { pub pudp: *mut pud_t, pub pudval: pudval_t }
#[repr(C)] pub struct xen_mmu_set_p4d_entry { pub p4dp: *mut p4d_t, pub user_p4dp: *mut p4d_t, pub p4dval: p4dval_t }
#[repr(C)] pub struct xen_mmu_ptep_modify_prot_entry { pub mm: *mut mm_struct, pub addr: c_ulong, pub ptep: *mut pte_t, pub pteval: pteval_t }
#[repr(C)] pub struct xen_mmu_alloc_ptpage_entry { pub mm: *mut mm_struct, pub pfn: c_ulong, pub level: u32, pub pinned: bool }
#[repr(C)] pub struct xen_mmu_release_ptpage_entry { pub pfn: c_ulong, pub level: u32, pub pinned: bool }
#[repr(C)] pub struct xen_mmu_pgd_entry { pub mm: *mut mm_struct, pub pgd: *mut pgd_t }
#[repr(C)] pub struct xen_mmu_flush_tlb_one_user_entry { pub addr: c_ulong }
#[repr(C)] pub struct xen_mmu_flush_tlb_multi_entry { pub ncpus: u32, pub mm: *mut mm_struct, pub addr: c_ulong, pub end: c_ulong }
#[repr(C)] pub struct xen_mmu_write_cr3_entry { pub kernel: bool, pub cr3: c_ulong }

#[repr(C)] pub struct xen_cpu_write_ldt_entry_entry { pub dt: *mut desc_struct, pub entrynum: i32, pub desc: u64 }
#[repr(C)] pub struct xen_cpu_write_idt_entry_entry { pub dt: *mut gate_desc, pub entrynum: i32 }
#[repr(C)] pub struct xen_cpu_load_idt_entry { pub addr: c_ulong }
#[repr(C)] pub struct xen_cpu_write_gdt_entry_entry { pub desc: u64, pub dt: *mut desc_struct, pub entrynum: i32, pub type_: i32 }
#[repr(C)] pub struct xen_cpu_set_ldt_entry { pub addr: *const c_void, pub entries: u32 }

// The following declarations preserve the trace-event interfaces and their
// field assignment/printing semantics.  Implementations are provided by the
// kernel tracepoint layer.
extern "C" {
    pub fn xen_mc_batch(flags: c_ulong);
    pub fn xen_mc_issue(flush: bool, flags: c_ulong);
    pub fn xen_mc_entry(mc: *mut multicall_entry, nargs: u32);
    pub fn xen_mc_entry_alloc(args: usize);
    pub fn xen_mc_callback(fn_: xen_mc_callback_fn_t, data: *mut c_void);
    pub fn xen_mc_flush_reason(reason: xen_mc_flush_reason);
    pub fn xen_mc_flush(mcidx: u32, argidx: u32, cbidx: u32);
    pub fn xen_mc_extend_args(op: c_ulong, args: usize, res: xen_mc_extend_args);
    pub fn xen_mmu_set_pte(ptep: *mut pte_t, pteval: pte_t);
    pub fn xen_mmu_set_pmd(pmdp: *mut pmd_t, pmdval: pmd_t);
    pub fn xen_mmu_set_pud(pudp: *mut pud_t, pudval: pud_t);
    pub fn xen_mmu_set_p4d(p4dp: *mut p4d_t, user_p4dp: *mut p4d_t, p4dval: p4d_t);
    pub fn xen_mmu_ptep_modify_prot_start(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, pteval: pte_t);
    pub fn xen_mmu_ptep_modify_prot_commit(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, pteval: pte_t);
    pub fn xen_mmu_alloc_ptpage(mm: *mut mm_struct, pfn: c_ulong, level: u32, pinned: bool);
    pub fn xen_mmu_release_ptpage(pfn: c_ulong, level: u32, pinned: bool);
    pub fn xen_mmu_pgd_pin(mm: *mut mm_struct, pgd: *mut pgd_t);
    pub fn xen_mmu_pgd_unpin(mm: *mut mm_struct, pgd: *mut pgd_t);
    pub fn xen_mmu_flush_tlb_one_user(addr: c_ulong);
    pub fn xen_mmu_flush_tlb_multi(cpus: *const cpumask, mm: *mut mm_struct, addr: c_ulong, end: c_ulong);
    pub fn xen_mmu_write_cr3(kernel: bool, cr3: c_ulong);
    pub fn xen_cpu_write_ldt_entry(dt: *mut desc_struct, entrynum: i32, desc: u64);
    pub fn xen_cpu_write_idt_entry(dt: *mut gate_desc, entrynum: i32, ent: *const gate_desc);
    pub fn xen_cpu_load_idt(desc: *const desc_ptr);
    pub fn xen_cpu_write_gdt_entry(dt: *mut desc_struct, entrynum: i32, desc: *const c_void, type_: i32);
    pub fn xen_cpu_set_ldt(addr: *const c_void, entries: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
