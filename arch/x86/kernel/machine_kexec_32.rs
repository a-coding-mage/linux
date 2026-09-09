// SPDX-License-Identifier: GPL-2.0-only
/*
 * handle transition of Linux booting another kernel
 * Copyright (C) 2002-2005 Eric Biederman  <ebiederm@xmission.com>
 */

// Linux and architecture headers from the original implementation provide the
// types, constants, functions, and configuration symbols referenced below.

unsafe fn load_segments() {
    core::arch::asm!(
        "ljmp ${cs}, $1f",
        "1:",
        "movl ${ds}, %eax",
        "movl %eax, %ds",
        "movl %eax, %es",
        "movl %eax, %ss",
        cs = const __KERNEL_CS,
        ds = const __KERNEL_DS,
        out("eax") _,
        options(nostack)
    );
}

unsafe fn machine_kexec_free_page_tables(image: *mut kimage) {
    free_pages((*image).arch.pgd as unsigned_long, pgd_allocation_order());
    (*image).arch.pgd = core::ptr::null_mut();
#[cfg(CONFIG_X86_PAE)]
    {
        free_page((*image).arch.pmd0 as unsigned_long);
        (*image).arch.pmd0 = core::ptr::null_mut();
        free_page((*image).arch.pmd1 as unsigned_long);
        (*image).arch.pmd1 = core::ptr::null_mut();
    }
    free_page((*image).arch.pte0 as unsigned_long);
    (*image).arch.pte0 = core::ptr::null_mut();
    free_page((*image).arch.pte1 as unsigned_long);
    (*image).arch.pte1 = core::ptr::null_mut();
}

unsafe fn machine_kexec_alloc_page_tables(image: *mut kimage) -> i32 {
    (*image).arch.pgd = __get_free_pages(GFP_KERNEL | __GFP_ZERO, pgd_allocation_order()) as *mut pgd_t;
#[cfg(CONFIG_X86_PAE)]
    {
        (*image).arch.pmd0 = get_zeroed_page(GFP_KERNEL) as *mut pmd_t;
        (*image).arch.pmd1 = get_zeroed_page(GFP_KERNEL) as *mut pmd_t;
    }
    (*image).arch.pte0 = get_zeroed_page(GFP_KERNEL) as *mut pte_t;
    (*image).arch.pte1 = get_zeroed_page(GFP_KERNEL) as *mut pte_t;
    if (*image).arch.pgd.is_null()
#[cfg(CONFIG_X86_PAE)]
        || (*image).arch.pmd0.is_null() || (*image).arch.pmd1.is_null()
        || (*image).arch.pte0.is_null() || (*image).arch.pte1.is_null()
    {
        return -ENOMEM;
    }
    0
}

unsafe fn machine_kexec_page_table_set_one(
    mut pgd: *mut pgd_t,
    mut pmd: *mut pmd_t,
    mut pte: *mut pte_t,
    vaddr: unsigned_long,
    paddr: unsigned_long,
) {
    pgd = pgd.add(pgd_index(vaddr) as usize);
#[cfg(CONFIG_X86_PAE)]
    if (pgd_val(*pgd) & _PAGE_PRESENT) == 0 {
        set_pgd(pgd, __pgd(__pa(pmd) | _PAGE_PRESENT));
    }
    let p4d = p4d_offset(pgd, vaddr);
    let pud = pud_offset(p4d, vaddr);
    pmd = pmd_offset(pud, vaddr);
    if (pmd_val(*pmd) & _PAGE_PRESENT) == 0 {
        set_pmd(pmd, __pmd(__pa(pte) | _PAGE_TABLE));
    }
    pte = pte_offset_kernel(pmd, vaddr);
    set_pte(pte, pfn_pte(paddr >> PAGE_SHIFT, PAGE_KERNEL_EXEC));
}

unsafe fn machine_kexec_prepare_page_tables(image: *mut kimage) {
    let control_page = page_address((*image).control_code_page);
    let mut pmd: *mut pmd_t = core::ptr::null_mut();
#[cfg(CONFIG_X86_PAE)]
    { pmd = (*image).arch.pmd0; }
    machine_kexec_page_table_set_one(
        (*image).arch.pgd, pmd, (*image).arch.pte0,
        control_page as unsigned_long, __pa(control_page),
    );
#[cfg(CONFIG_X86_PAE)]
    { pmd = (*image).arch.pmd1; }
    machine_kexec_page_table_set_one(
        (*image).arch.pgd, pmd, (*image).arch.pte1,
        __pa(control_page), __pa(control_page),
    );
}

pub unsafe fn machine_kexec_prepare(image: *mut kimage) -> i32 {
    set_memory_x(page_address((*image).control_code_page) as unsigned_long, 1);
    let error = machine_kexec_alloc_page_tables(image);
    if error != 0 { return error; }
    machine_kexec_prepare_page_tables(image);
    0
}

pub unsafe fn machine_kexec_cleanup(image: *mut kimage) {
    set_memory_nx(page_address((*image).control_code_page) as unsigned_long, 1);
    machine_kexec_free_page_tables(image);
}

pub unsafe fn machine_kexec(image: *mut kimage) {
    let mut relocate_kernel_ptr: relocate_kernel_fn;
    let mut page_list: [unsigned_long; PAGES_NR] = [0; PAGES_NR];
    let control_page: *mut core::ffi::c_void;
    let save_ftrace_enabled: i32;

#[cfg(CONFIG_KEXEC_JUMP)]
    if (*image).preserve_context { save_processor_state(); }
    save_ftrace_enabled = __ftrace_enabled_save();
    local_irq_disable();
    hw_breakpoint_disable();
    if (*image).preserve_context {
#[cfg(CONFIG_X86_IO_APIC)]
        { clear_IO_APIC(); restore_boot_irq_mode(); }
    }
    control_page = page_address((*image).control_code_page);
    core::ptr::copy_nonoverlapping(relocate_kernel, control_page, KEXEC_CONTROL_CODE_MAX_SIZE);
    relocate_kernel_ptr = core::mem::transmute(control_page);
    page_list[PA_CONTROL_PAGE] = __pa(control_page);
    page_list[VA_CONTROL_PAGE] = control_page as unsigned_long;
    page_list[PA_PGD] = __pa((*image).arch.pgd);
    if (*image).type_ == KEXEC_TYPE_DEFAULT {
        page_list[PA_SWAP_PAGE] = page_to_pfn((*image).swap_page) << PAGE_SHIFT;
    }
    load_segments();
    native_idt_invalidate();
    native_gdt_invalidate();
    (*image).start = relocate_kernel_ptr(
        (*image).head as unsigned_long, page_list.as_mut_ptr() as unsigned_long,
        (*image).start, boot_cpu_has(X86_FEATURE_PAE), (*image).preserve_context,
    );
#[cfg(CONFIG_KEXEC_JUMP)]
    if (*image).preserve_context { restore_processor_state(); }
    __ftrace_enabled_restore(save_ftrace_enabled);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
