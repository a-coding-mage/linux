// SPDX-License-Identifier: GPL-2.0-or-later
/*  Paravirtualization interfaces
    Copyright (C) 2006 Rusty Russell IBM Corporation

    2007 - x86_64 support added by Glauber de Oliveira Costa, Red Hat Inc
*/

// Dependencies are supplied by the surrounding kernel translation.

/* stub always returning 0. */
unsafe extern "C" {
    fn paravirt_ret0() -> usize;
}

pub unsafe extern "C" fn default_banner() {
    printk(KERN_INFO, c"Booting paravirtualized kernel on %s\n".as_ptr(), pv_info.name);
}

#[cfg(CONFIG_PARAVIRT_XXL)]
unsafe extern "C" {
    fn pv_native_save_fl() -> c_ulong;
    fn pv_native_irq_disable();
    fn pv_native_irq_enable();
    fn pv_native_read_cr2() -> c_ulong;
    fn _paravirt_ident_64(value: c_ulong) -> c_ulong;
}

#[cfg(CONFIG_PARAVIRT_XXL)]
unsafe extern "C" {
    fn native_safe_halt();
}

unsafe extern "C" fn pv_native_safe_halt() {
    native_safe_halt();
}

#[cfg(CONFIG_PARAVIRT_XXL)]
unsafe extern "C" fn pv_native_write_cr2(val: c_ulong) {
    native_write_cr2(val);
}

#[cfg(CONFIG_PARAVIRT_XXL)]
unsafe extern "C" fn pv_native_read_cr3() -> c_ulong {
    __native_read_cr3()
}

#[cfg(CONFIG_PARAVIRT_XXL)]
unsafe extern "C" fn pv_native_write_cr3(cr3: c_ulong) {
    native_write_cr3(cr3);
}

#[cfg(CONFIG_PARAVIRT_XXL)]
unsafe extern "C" fn pv_native_get_debugreg(regno: c_int) -> c_ulong {
    native_get_debugreg(regno)
}

#[cfg(CONFIG_PARAVIRT_XXL)]
unsafe extern "C" fn pv_native_set_debugreg(regno: c_int, val: c_ulong) {
    native_set_debugreg(regno, val);
}

pub static mut pv_info: pv_info = pv_info {
    name: c"bare hardware".as_ptr(),
    #[cfg(CONFIG_PARAVIRT_XXL)]
    extra_user_64bit_cs: __USER_CS,
    io_delay: true,
};

/* 64-bit pagetable entries */
#[cfg(CONFIG_PARAVIRT_XXL)]
const PTE_IDENT: _ = __PV_IS_CALLEE_SAVE(_paravirt_ident_64);

pub static mut pv_ops: paravirt_patch_template = paravirt_patch_template {
    #[cfg(CONFIG_PARAVIRT_XXL)]
    cpu: paravirt_cpu_ops {
        cpuid: native_cpuid,
        get_debugreg: pv_native_get_debugreg,
        set_debugreg: pv_native_set_debugreg,
        read_cr0: native_read_cr0,
        write_cr0: native_write_cr0,
        write_cr4: native_write_cr4,
        read_msr: native_read_msr,
        write_msr: native_write_msr,
        read_msr_safe: native_read_msr_safe,
        write_msr_safe: native_write_msr_safe,
        read_pmc: native_read_pmc,
        load_tr_desc: native_load_tr_desc,
        set_ldt: native_set_ldt,
        load_gdt: native_load_gdt,
        load_idt: native_load_idt,
        store_tr: native_store_tr,
        load_tls: native_load_tls,
        load_gs_index: native_load_gs_index,
        write_ldt_entry: native_write_ldt_entry,
        write_gdt_entry: native_write_gdt_entry,
        write_idt_entry: native_write_idt_entry,
        alloc_ldt: paravirt_nop,
        free_ldt: paravirt_nop,
        load_sp0: native_load_sp0,
        #[cfg(CONFIG_X86_IOPL_IOPERM)]
        invalidate_io_bitmap: native_tss_invalidate_io_bitmap,
        #[cfg(CONFIG_X86_IOPL_IOPERM)]
        update_io_bitmap: native_tss_update_io_bitmap,
        start_context_switch: paravirt_nop,
        end_context_switch: paravirt_nop,
    },
    #[cfg(CONFIG_PARAVIRT_XXL)]
    irq: paravirt_irq_ops {
        save_fl: __PV_IS_CALLEE_SAVE(pv_native_save_fl),
        irq_disable: __PV_IS_CALLEE_SAVE(pv_native_irq_disable),
        irq_enable: __PV_IS_CALLEE_SAVE(pv_native_irq_enable),
        safe_halt: pv_native_safe_halt,
        halt: native_halt,
    },
    #[cfg(not(CONFIG_PARAVIRT_XXL))]
    irq: paravirt_irq_ops { safe_halt: pv_native_safe_halt, halt: native_halt },
    mmu: paravirt_mmu_ops {
        flush_tlb_user: native_flush_tlb_local,
        flush_tlb_kernel: native_flush_tlb_global,
        flush_tlb_one_user: native_flush_tlb_one_user,
        flush_tlb_multi: native_flush_tlb_multi,
        exit_mmap: paravirt_nop,
        notify_page_enc_status_changed: paravirt_nop,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        read_cr2: __PV_IS_CALLEE_SAVE(pv_native_read_cr2),
        #[cfg(CONFIG_PARAVIRT_XXL)]
        write_cr2: pv_native_write_cr2,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        read_cr3: pv_native_read_cr3,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        write_cr3: pv_native_write_cr3,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        pgd_alloc: __paravirt_pgd_alloc,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        pgd_free: paravirt_nop,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        alloc_pte: paravirt_nop,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        alloc_pmd: paravirt_nop,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        alloc_pud: paravirt_nop,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        alloc_p4d: paravirt_nop,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        release_pte: paravirt_nop,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        release_pmd: paravirt_nop,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        release_pud: paravirt_nop,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        release_p4d: paravirt_nop,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        set_pte: native_set_pte,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        set_pmd: native_set_pmd,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        ptep_modify_prot_start: __ptep_modify_prot_start,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        ptep_modify_prot_commit: __ptep_modify_prot_commit,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        set_pud: native_set_pud,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        pmd_val: PTE_IDENT,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        make_pmd: PTE_IDENT,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        pud_val: PTE_IDENT,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        make_pud: PTE_IDENT,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        set_p4d: native_set_p4d,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        p4d_val: PTE_IDENT,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        make_p4d: PTE_IDENT,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        set_pgd: native_set_pgd,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        pte_val: PTE_IDENT,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        pgd_val: PTE_IDENT,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        make_pte: PTE_IDENT,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        make_pgd: PTE_IDENT,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        enter_mmap: paravirt_nop,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        lazy_mode_flush: paravirt_nop,
        #[cfg(CONFIG_PARAVIRT_XXL)]
        set_fixmap: native_set_fixmap,
    },
};

#[cfg(CONFIG_PARAVIRT_XXL)]
NOKPROBE_SYMBOL!(native_load_idt);

EXPORT_SYMBOL!(pv_ops);
EXPORT_SYMBOL_GPL!(pv_info);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
