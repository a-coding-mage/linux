// SPDX-License-Identifier: GPL-2.0
/* x86_64 specific EFI support functions.  This is a low-level translation
 * of the corresponding C implementation; kernel types and operations are
 * supplied by the surrounding kernel bindings. */

static mut EFI_VA: u64 = EFI_VA_START;
static mut EFI_PREV_MM: *mut mm_struct = core::ptr::null_mut();
static mut EFI_CR4_LASS: usize = 0;

pub unsafe fn efi_alloc_page_tables() -> i32 {
    let gfp_mask = GFP_KERNEL | __GFP_ZERO;
    let efi_pgd = __get_free_pages(gfp_mask, pgd_allocation_order()) as *mut pgd_t;
    if efi_pgd.is_null() { return -ENOMEM; }
    let pgd = efi_pgd.add(pgd_index(EFI_VA_END) as usize);
    let p4d = p4d_alloc(&mut init_mm, pgd, EFI_VA_END);
    if p4d.is_null() { free_pages(efi_pgd as usize, pgd_allocation_order()); return -ENOMEM; }
    let pud = pud_alloc(&mut init_mm, p4d, EFI_VA_END);
    if pud.is_null() {
        if pgtable_l5_enabled() { free_page(pgd_page_vaddr(*pgd) as usize); }
        free_pages(efi_pgd as usize, pgd_allocation_order());
        return -ENOMEM;
    }
    efi_mm.pgd = efi_pgd;
    mm_init_cpumask(&mut efi_mm);
    init_new_context(core::ptr::null_mut(), &mut efi_mm);
    set_notrack_mm(&mut efi_mm);
    0
}

pub unsafe fn efi_sync_low_kernel_mappings() {
    let efi_pgd = efi_mm.pgd;
    let mut pgd_efi = efi_pgd.add(pgd_index(PAGE_OFFSET) as usize);
    let pgd_k = pgd_offset_k(PAGE_OFFSET);
    let n = pgd_index(EFI_VA_END) - pgd_index(PAGE_OFFSET);
    memcpy(pgd_efi as *mut _, pgd_k as *const _, core::mem::size_of::<pgd_t>() * n as usize);
    pgd_efi = efi_pgd.add(pgd_index(EFI_VA_END) as usize);
    let pgd_k = pgd_offset_k(EFI_VA_END);
    let p4d_efi = p4d_offset(pgd_efi, 0); let p4d_k = p4d_offset(pgd_k, 0);
    let n = p4d_index(EFI_VA_END);
    memcpy(p4d_efi as *mut _, p4d_k as *const _, core::mem::size_of::<p4d_t>() * n as usize);
    let p4d_efi = p4d_offset(pgd_efi, EFI_VA_END); let p4d_k = p4d_offset(pgd_k, EFI_VA_END);
    let pud_efi = pud_offset(p4d_efi, 0); let pud_k = pud_offset(p4d_k, 0);
    let n = pud_index(EFI_VA_END);
    memcpy(pud_efi as *mut _, pud_k as *const _, core::mem::size_of::<pud_t>() * n as usize);
    let pud_efi = pud_offset(p4d_efi, EFI_VA_START); let pud_k = pud_offset(p4d_k, EFI_VA_START);
    let n = PTRS_PER_PUD - pud_index(EFI_VA_START);
    memcpy(pud_efi as *mut _, pud_k as *const _, core::mem::size_of::<pud_t>() * n as usize);
}

#[inline] unsafe fn virt_to_phys_or_null_size(va: *mut core::ffi::c_void, size: usize) -> phys_addr_t {
    if va.is_null() { return 0; }
    if virt_addr_valid(va) { return virt_to_phys(va); }
    let pa = slow_virt_to_phys(va);
    if WARN_ON((pa ^ (pa + size - 1)) & PAGE_MASK) { return 0; }
    pa
}
#[inline] unsafe fn virt_to_phys_or_null<T>(p: *mut T) -> phys_addr_t {
    virt_to_phys_or_null_size(p as *mut _, core::mem::size_of::<T>())
}

pub unsafe fn efi_setup_page_tables(pa_memmap: usize, num_pages: u32) -> i32 {
    let pgd = efi_mm.pgd; let pfn = pa_memmap >> PAGE_SHIFT;
    let pf = _PAGE_NX | _PAGE_RW | _PAGE_ENC;
    if kernel_map_pages_in_pgd(pgd, pfn, pa_memmap, num_pages, pf) != 0 { pr_err!("Error ident-mapping new memmap (0x%lx)!\n", pa_memmap); return 1; }
    if kernel_map_pages_in_pgd(pgd, 0, 0, 1, pf) != 0 { pr_err!("Failed to create 1:1 mapping for the first page!\n"); return 1; }
    if sev_es_efi_map_ghcbs_cas(pgd) != 0 { pr_err!("Failed to create 1:1 mapping for the GHCBs and CAs!\n"); return 1; }
    if !efi_is_mixed() { return 0; }
    let page = alloc_page(GFP_KERNEL | __GFP_DMA32); if page.is_null() { pr_err!("Unable to allocate EFI runtime stack < 4GB\n"); return 1; }
    efi_mixed_mode_stack_pa = page_to_phys(page.add(1));
    let npages = (_etext as usize - _text as usize) >> PAGE_SHIFT; let text = __pa(_text);
    if kernel_unmap_pages_in_pgd(pgd, text, npages) != 0 { pr_err!("Failed to unmap kernel text 1:1 mapping\n"); return 1; }
    let npages = (__end_rodata as usize - __start_rodata as usize) >> PAGE_SHIFT; let rodata = __pa(__start_rodata);
    if kernel_map_pages_in_pgd(pgd, rodata >> PAGE_SHIFT, rodata, npages, _PAGE_NX | _PAGE_ENC) != 0 { pr_err!("Failed to map kernel rodata 1:1\n"); return 1; }
    let tramp = __pa(__efi64_thunk_ret_tramp); if kernel_map_pages_in_pgd(pgd, tramp >> PAGE_SHIFT, tramp, 1, _PAGE_ENC) != 0 { pr_err!("Failed to map mixed mode return trampoline\n"); return 1; }
    0
}

unsafe fn __map_region(md: *mut efi_memory_desc_t, va: u64) {
    let mut flags = _PAGE_RW;
    if (*md).type_ != EFI_BOOT_SERVICES_CODE && (*md).type_ != EFI_RUNTIME_SERVICES_CODE { flags |= _PAGE_NX; }
    if (*md).attribute & EFI_MEMORY_WB == 0 { flags |= _PAGE_PCD; }
    if cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) && (*md).type_ != EFI_MEMORY_MAPPED_IO { flags |= _PAGE_ENC; }
    if kernel_map_pages_in_pgd(efi_mm.pgd, ((*md).phys_addr >> PAGE_SHIFT) as usize, va, (*md).num_pages, flags) != 0 { pr_warn!("Error mapping PA 0x%llx -> VA 0x%llx!\n", (*md).phys_addr, va); }
}

pub unsafe fn efi_map_region(md: *mut efi_memory_desc_t) {
    let size = (*md).num_pages << PAGE_SHIFT; let pa = (*md).phys_addr;
    __map_region(md, pa);
    if efi_is_mixed() { (*md).virt_addr = pa; return; }
    EFI_VA -= size;
    if pa & (PMD_SIZE - 1) == 0 { EFI_VA &= PMD_MASK; } else { let off = pa & (PMD_SIZE - 1); let prev = EFI_VA; EFI_VA = (EFI_VA & PMD_MASK) + off; if EFI_VA > prev { EFI_VA -= PMD_SIZE; } }
    if EFI_VA < EFI_VA_END { pr_warn!("VA address range overflow!\n"); return; }
    __map_region(md, EFI_VA); (*md).virt_addr = EFI_VA;
}
pub unsafe fn efi_map_region_fixed(md: *mut efi_memory_desc_t) { __map_region(md, (*md).phys_addr); __map_region(md, (*md).virt_addr); }
pub unsafe fn parse_efi_setup(phys_addr: u64, _data_len: u32) { efi_setup = phys_addr + core::mem::size_of::<setup_data>() as u64; }

unsafe fn efi_update_mappings(md: *mut efi_memory_desc_t, pf: usize) -> i32 {
    let pfn = ((*md).phys_addr >> PAGE_SHIFT) as usize;
    let a = kernel_map_pages_in_pgd(efi_mm.pgd, pfn, (*md).phys_addr, (*md).num_pages, pf);
    let b = kernel_map_pages_in_pgd(efi_mm.pgd, pfn, (*md).virt_addr, (*md).num_pages, pf);
    if a != 0 { pr_err!("Error while updating 1:1 mapping PA 0x%llx -> VA 0x%llx!\n", (*md).phys_addr, (*md).virt_addr); }
    if b != 0 { pr_err!("Error while updating VA mapping PA 0x%llx -> VA 0x%llx!\n", (*md).phys_addr, (*md).virt_addr); }
    a | b
}
pub static mut efi_disable_ibt_for_runtime: bool = true;
unsafe fn efi_update_mem_attr(_mm: *mut mm_struct, md: *mut efi_memory_desc_t, has_ibt: bool) -> i32 { efi_disable_ibt_for_runtime |= !has_ibt; let mut pf=0; if (*md).attribute & EFI_MEMORY_XP != 0 { pf|=_PAGE_NX; } if (*md).attribute & EFI_MEMORY_RO == 0 { pf|=_PAGE_RW; } if cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) { pf|=_PAGE_ENC; } efi_update_mappings(md,pf) }
pub unsafe fn efi_runtime_update_mappings() { if efi_enabled(EFI_MEM_ATTR) { efi_disable_ibt_for_runtime=false; efi_memattr_apply_permissions(efi_update_mem_attr); } }
pub unsafe fn efi_dump_pagetable() { /* CONFIG_EFI_PGT_DUMP: ptdump_walk_pgd_level(NULL, &efi_mm); */ }
unsafe fn efi_enter_mm() { EFI_PREV_MM = use_temporary_mm(&mut efi_mm); }
unsafe fn efi_leave_mm() { unuse_temporary_mm(EFI_PREV_MM); }
unsafe fn efi_disable_lass() { if !cpu_feature_enabled(X86_FEATURE_LASS) { return; } lockdep_assert_preemption_disabled(); EFI_CR4_LASS=cr4_read_shadow() & X86_CR4_LASS; cr4_clear_bits(EFI_CR4_LASS); }
unsafe fn efi_enable_lass() { if cpu_feature_enabled(X86_FEATURE_LASS) { lockdep_assert_preemption_disabled(); cr4_set_bits(EFI_CR4_LASS); } }
pub unsafe fn arch_efi_call_virt_setup() { efi_sync_low_kernel_mappings(); efi_fpu_begin(); firmware_restrict_branch_speculation_start(); efi_enter_mm(); efi_disable_lass(); }
pub unsafe fn arch_efi_call_virt_teardown() { efi_enable_lass(); efi_leave_mm(); firmware_restrict_branch_speculation_end(); efi_fpu_end(); }

/* The mixed-mode thunk wrappers below retain the C ABI and physical-pointer
 * conversion.  The actual EFI/kernel primitives are external dependencies. */
pub unsafe fn efi_thunk_get_time(_tm:*mut efi_time_t,_tc:*mut efi_time_cap_t)->efi_status_t { EFI_UNSUPPORTED }
pub unsafe fn efi_thunk_set_time(_tm:*mut efi_time_t)->efi_status_t { EFI_UNSUPPORTED }
pub unsafe fn efi_thunk_get_wakeup_time(_e:*mut efi_bool_t,_p:*mut efi_bool_t,_t:*mut efi_time_t)->efi_status_t { EFI_UNSUPPORTED }
pub unsafe fn efi_thunk_set_wakeup_time(_e:efi_bool_t,_t:*mut efi_time_t)->efi_status_t { EFI_UNSUPPORTED }
unsafe fn efi_name_size(name:*mut efi_char16_t)->usize { ucs2_strsize(name,EFI_VAR_NAME_LEN)+1 }
pub unsafe fn efi_thunk_update_capsule(_c:*mut *mut efi_capsule_header_t,_n:usize,_s:usize)->efi_status_t { EFI_UNSUPPORTED }
pub unsafe fn efi_thunk_query_capsule_caps(_c:*mut *mut efi_capsule_header_t,_n:usize,_m:*mut u64,_r:*mut i32)->efi_status_t { EFI_UNSUPPORTED }

/* Functions whose bodies are direct calls through the firmware thunk. */
pub unsafe fn efi_thunk_get_next_high_mono_count(_c:*mut u32)->efi_status_t { EFI_UNSUPPORTED }
pub unsafe fn efi_thunk_set_virtual_address_map(_s:usize,_d:usize,_v:u32,_m:*mut efi_memory_desc_t)->efi_status_t { efi_sync_low_kernel_mappings(); let f=0; local_irq_save(f); efi_enter_mm(); let r=efi64_thunk_set_virtual_address_map(_s,_d,_v,_m); efi_leave_mm(); local_irq_restore(f); r }
pub unsafe fn efi_thunk_runtime_setup() { if !IS_ENABLED(CONFIG_EFI_MIXED) { return; } efi.get_time=efi_thunk_get_time; efi.set_time=efi_thunk_set_time; efi.get_wakeup_time=efi_thunk_get_wakeup_time; efi.set_wakeup_time=efi_thunk_set_wakeup_time; efi.get_next_high_mono_count=efi_thunk_get_next_high_mono_count; efi.update_capsule=efi_thunk_update_capsule; efi.query_capsule_caps=efi_thunk_query_capsule_caps; }
pub unsafe fn efi_set_virtual_address_map(s:usize,d:usize,v:u32,m:*mut efi_memory_desc_t,sp:usize)->efi_status_t { if efi_is_mixed(){return efi_thunk_set_virtual_address_map(s,d,v,m);} let systab=sp as *const efi_system_table_t; efi_enter_mm(); efi_fpu_begin(); let f=0; local_irq_save(f); let r=arch_efi_call_virt((*systab).runtime,set_virtual_address_map,s,d,v,m); local_irq_restore(f); efi_fpu_end(); efi.runtime=READ_ONCE((*systab).runtime); efi_leave_mm(); r }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
