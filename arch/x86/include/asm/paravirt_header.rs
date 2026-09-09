/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of asm/paravirt.h.  Configuration and included definitions
 * are supplied by the surrounding kernel translation. */

#[allow(unused_variables, dead_code)]
unsafe extern "C" {
    pub fn native_flush_tlb_local();
    pub fn native_flush_tlb_global();
    pub fn native_flush_tlb_one_user(addr: ::core::ffi::c_ulong);
    pub fn native_flush_tlb_multi(cpumask: *const cpumask, info: *const flush_tlb_info);
}

/* The PVOP_* names below denote the corresponding paravirtual operation
 * macros from the translated paravirt support. */
#[inline] pub unsafe fn __flush_tlb_local() { PVOP_VCALL0!(pv_ops, mmu.flush_tlb_user); }
#[inline] pub unsafe fn __flush_tlb_global() { PVOP_VCALL0!(pv_ops, mmu.flush_tlb_kernel); }
#[inline] pub unsafe fn __flush_tlb_one_user(addr: ::core::ffi::c_ulong) { PVOP_VCALL1!(pv_ops, mmu.flush_tlb_one_user, addr); }
#[inline] pub unsafe fn __flush_tlb_multi(cpumask: *const cpumask, info: *const flush_tlb_info) { PVOP_VCALL2!(pv_ops, mmu.flush_tlb_multi, cpumask, info); }
#[inline] pub unsafe fn paravirt_arch_exit_mmap(mm: *mut mm_struct) { PVOP_VCALL1!(pv_ops, mmu.exit_mmap, mm); }
#[inline] pub unsafe fn notify_page_enc_status_changed(pfn: ::core::ffi::c_ulong, npages: i32, enc: bool) { PVOP_VCALL3!(pv_ops, mmu.notify_page_enc_status_changed, pfn, npages, enc); }
#[inline] pub unsafe fn arch_safe_halt() { PVOP_VCALL0!(pv_ops, irq.safe_halt); }
#[inline] pub unsafe fn halt() { PVOP_VCALL0!(pv_ops, irq.halt); }

#[inline] pub unsafe fn load_sp0(sp0: ::core::ffi::c_ulong) { PVOP_VCALL1!(pv_ops, cpu.load_sp0, sp0); }
#[inline] pub unsafe fn __cpuid(eax: *mut u32, ebx: *mut u32, ecx: *mut u32, edx: *mut u32) { PVOP_VCALL4!(pv_ops, cpu.cpuid, eax, ebx, ecx, edx); }
#[inline] pub unsafe fn paravirt_get_debugreg(reg: i32) -> ::core::ffi::c_ulong { PVOP_CALL1!(::core::ffi::c_ulong, pv_ops, cpu.get_debugreg, reg) }
#[inline] pub unsafe fn get_debugreg(var: &mut ::core::ffi::c_ulong, reg: i32) { *var = paravirt_get_debugreg(reg); }
#[inline] pub unsafe fn set_debugreg(val: ::core::ffi::c_ulong, reg: i32) { PVOP_VCALL2!(pv_ops, cpu.set_debugreg, reg, val); }
#[inline] pub unsafe fn read_cr0() -> ::core::ffi::c_ulong { PVOP_CALL0!(::core::ffi::c_ulong, pv_ops, cpu.read_cr0) }
#[inline] pub unsafe fn write_cr0(x: ::core::ffi::c_ulong) { PVOP_VCALL1!(pv_ops, cpu.write_cr0, x); }
#[inline] pub unsafe fn read_cr2() -> ::core::ffi::c_ulong { PVOP_ALT_CALLEE0!(::core::ffi::c_ulong, pv_ops, mmu.read_cr2, "mov %%cr2, %%rax", ALT_NOT_XEN) }
#[inline] pub unsafe fn write_cr2(x: ::core::ffi::c_ulong) { PVOP_VCALL1!(pv_ops, mmu.write_cr2, x); }
#[inline] pub unsafe fn __read_cr3() -> ::core::ffi::c_ulong { PVOP_ALT_CALL0!(::core::ffi::c_ulong, pv_ops, mmu.read_cr3, "mov %%cr3, %%rax", ALT_NOT_XEN) }
#[inline] pub unsafe fn write_cr3(x: ::core::ffi::c_ulong) { PVOP_ALT_VCALL1!(pv_ops, mmu.write_cr3, x, "mov %%rdi, %%cr3", ALT_NOT_XEN); }
#[inline] pub unsafe fn __write_cr4(x: ::core::ffi::c_ulong) { PVOP_VCALL1!(pv_ops, cpu.write_cr4, x); }
#[inline] pub unsafe fn paravirt_read_msr(msr: u32) -> u64 { PVOP_CALL1!(u64, pv_ops, cpu.read_msr, msr) }
#[inline] pub unsafe fn paravirt_write_msr(msr: u32, val: u64) { PVOP_VCALL2!(pv_ops, cpu.write_msr, msr, val); }
#[inline] pub unsafe fn paravirt_read_msr_safe(msr: u32, val: *mut u64) -> i32 { PVOP_CALL2!(i32, pv_ops, cpu.read_msr_safe, msr, val) }
#[inline] pub unsafe fn paravirt_write_msr_safe(msr: u32, val: u64) -> i32 { PVOP_CALL2!(i32, pv_ops, cpu.write_msr_safe, msr, val) }
#[inline] pub unsafe fn rdmsr(msr: u32, val1: &mut u32, val2: &mut u32) { let l = paravirt_read_msr(msr); *val1 = l as u32; *val2 = (l >> 32) as u32; }
#[inline] pub unsafe fn wrmsr(msr: u32, low: u32, high: u32) { paravirt_write_msr(msr, (high as u64) << 32 | low as u64); }
#[inline] pub unsafe fn rdmsrq(msr: u32, val: &mut u64) { *val = paravirt_read_msr(msr); }
#[inline] pub unsafe fn wrmsrq(msr: u32, val: u64) { paravirt_write_msr(msr, val); }
#[inline] pub unsafe fn wrmsrq_safe(msr: u32, val: u64) -> i32 { paravirt_write_msr_safe(msr, val) }
#[inline] pub unsafe fn rdmsr_safe(msr: u32, a: *mut u32, b: *mut u32) -> i32 { let mut l = 0; let e = paravirt_read_msr_safe(msr, &mut l); *a = l as u32; *b = (l >> 32) as u32; e }
#[inline] pub unsafe fn rdmsrq_safe(msr: u32, p: *mut u64) -> i32 { paravirt_read_msr_safe(msr, p) }
#[inline] pub unsafe fn rdpmc(counter: i32) -> u64 { PVOP_CALL1!(u64, pv_ops, cpu.read_pmc, counter) }

/* Remaining operations retain their C ABI-facing names and operation order. */
#[inline] pub unsafe fn paravirt_alloc_ldt(ldt: *mut desc_struct, entries: u32) { PVOP_VCALL2!(pv_ops, cpu.alloc_ldt, ldt, entries); }
#[inline] pub unsafe fn paravirt_free_ldt(ldt: *mut desc_struct, entries: u32) { PVOP_VCALL2!(pv_ops, cpu.free_ldt, ldt, entries); }
#[inline] pub unsafe fn load_TR_desc() { PVOP_VCALL0!(pv_ops, cpu.load_tr_desc); }
#[inline] pub unsafe fn load_gdt(dtr: *const desc_ptr) { PVOP_VCALL1!(pv_ops, cpu.load_gdt, dtr); }
#[inline] pub unsafe fn load_idt(dtr: *const desc_ptr) { PVOP_VCALL1!(pv_ops, cpu.load_idt, dtr); }
#[inline] pub unsafe fn set_ldt(addr: *const core::ffi::c_void, entries: u32) { PVOP_VCALL2!(pv_ops, cpu.set_ldt, addr, entries); }
#[inline] pub unsafe fn paravirt_store_tr() -> ::core::ffi::c_ulong { PVOP_CALL0!(::core::ffi::c_ulong, pv_ops, cpu.store_tr) }
#[inline] pub unsafe fn store_tr(tr: &mut ::core::ffi::c_ulong) { *tr = paravirt_store_tr(); }
#[inline] pub unsafe fn load_TLS(t: *mut thread_struct, cpu: u32) { PVOP_VCALL2!(pv_ops, cpu.load_tls, t, cpu); }
#[inline] pub unsafe fn load_gs_index(gs: u32) { PVOP_VCALL1!(pv_ops, cpu.load_gs_index, gs); }

#[cfg(feature = "x86_iopl_ioperm")]
#[inline] pub unsafe fn tss_invalidate_io_bitmap() { PVOP_VCALL0!(pv_ops, cpu.invalidate_io_bitmap); }
#[cfg(feature = "x86_iopl_ioperm")]
#[inline] pub unsafe fn tss_update_io_bitmap() { PVOP_VCALL0!(pv_ops, cpu.update_io_bitmap); }

#[inline] pub unsafe fn paravirt_enter_mmap(next: *mut mm_struct) { PVOP_VCALL1!(pv_ops, mmu.enter_mmap, next); }
#[inline] pub unsafe fn paravirt_pgd_alloc(mm: *mut mm_struct) -> i32 { PVOP_CALL1!(i32, pv_ops, mmu.pgd_alloc, mm) }
#[inline] pub unsafe fn paravirt_pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t) { PVOP_VCALL2!(pv_ops, mmu.pgd_free, mm, pgd); }
#[inline] pub unsafe fn paravirt_alloc_pte(mm: *mut mm_struct, pfn: ::core::ffi::c_ulong) { PVOP_VCALL2!(pv_ops, mmu.alloc_pte, mm, pfn); }
#[inline] pub unsafe fn paravirt_release_pte(pfn: ::core::ffi::c_ulong) { PVOP_VCALL1!(pv_ops, mmu.release_pte, pfn); }
#[inline] pub unsafe fn paravirt_alloc_pmd(mm: *mut mm_struct, pfn: ::core::ffi::c_ulong) { PVOP_VCALL2!(pv_ops, mmu.alloc_pmd, mm, pfn); }
#[inline] pub unsafe fn paravirt_release_pmd(pfn: ::core::ffi::c_ulong) { PVOP_VCALL1!(pv_ops, mmu.release_pmd, pfn); }
#[inline] pub unsafe fn paravirt_alloc_pud(mm: *mut mm_struct, pfn: ::core::ffi::c_ulong) { PVOP_VCALL2!(pv_ops, mmu.alloc_pud, mm, pfn); }
#[inline] pub unsafe fn paravirt_release_pud(pfn: ::core::ffi::c_ulong) { PVOP_VCALL1!(pv_ops, mmu.release_pud, pfn); }
#[inline] pub unsafe fn paravirt_alloc_p4d(mm: *mut mm_struct, pfn: ::core::ffi::c_ulong) { PVOP_VCALL2!(pv_ops, mmu.alloc_p4d, mm, pfn); }
#[inline] pub unsafe fn paravirt_release_p4d(pfn: ::core::ffi::c_ulong) { PVOP_VCALL1!(pv_ops, mmu.release_p4d, pfn); }

#[inline] pub unsafe fn arch_start_context_switch(prev: *mut task_struct) { PVOP_VCALL1!(pv_ops, cpu.start_context_switch, prev); }
#[inline] pub unsafe fn arch_end_context_switch(next: *mut task_struct) { PVOP_VCALL1!(pv_ops, cpu.end_context_switch, next); }
#[inline] pub unsafe fn arch_enter_lazy_mmu_mode() {}
#[inline] pub unsafe fn arch_flush_lazy_mmu_mode() { PVOP_VCALL0!(pv_ops, mmu.lazy_mode_flush); }
#[inline] pub unsafe fn arch_leave_lazy_mmu_mode() { arch_flush_lazy_mmu_mode(); }
#[inline] pub unsafe fn arch_local_save_flags() -> ::core::ffi::c_ulong { PVOP_ALT_CALLEE0!(::core::ffi::c_ulong, pv_ops, irq.save_fl, "pushf; pop %%rax", ALT_NOT_XEN) }
#[inline] pub unsafe fn arch_local_irq_disable() { PVOP_ALT_VCALLEE0!(pv_ops, irq.irq_disable, "cli", ALT_NOT_XEN); }
#[inline] pub unsafe fn arch_local_irq_enable() { PVOP_ALT_VCALLEE0!(pv_ops, irq.irq_enable, "sti", ALT_NOT_XEN); }
#[inline] pub unsafe fn arch_local_irq_save() -> ::core::ffi::c_ulong { let f = arch_local_save_flags(); arch_local_irq_disable(); f }

#[inline] pub unsafe fn __pte(val: pteval_t) -> pte_t { pte_t { pte: PVOP_ALT_CALLEE1!(pteval_t, pv_ops, mmu.make_pte, val, "mov %%rdi, %%rax", ALT_NOT_XEN) } }
#[inline] pub unsafe fn pte_val(pte: pte_t) -> pteval_t { PVOP_ALT_CALLEE1!(pteval_t, pv_ops, mmu.pte_val, pte.pte, "mov %%rdi, %%rax", ALT_NOT_XEN) }
#[inline] pub unsafe fn __pgd(val: pgdval_t) -> pgd_t { pgd_t { pgd: PVOP_ALT_CALLEE1!(pgdval_t, pv_ops, mmu.make_pgd, val, "mov %%rdi, %%rax", ALT_NOT_XEN) } }
#[inline] pub unsafe fn pgd_val(pgd: pgd_t) -> pgdval_t { PVOP_ALT_CALLEE1!(pgdval_t, pv_ops, mmu.pgd_val, pgd.pgd, "mov %%rdi, %%rax", ALT_NOT_XEN) }
#[inline] pub unsafe fn pmd_clear(pmdp: *mut pmd_t) { set_pmd(pmdp, native_make_pmd(0)); }
#[inline] pub unsafe fn pte_clear(_mm: *mut mm_struct, _addr: ::core::ffi::c_ulong, ptep: *mut pte_t) { set_pte(ptep, native_make_pte(0)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
