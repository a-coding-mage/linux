// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Google LLC
 * Author: Quentin Perret <qperret@google.com>
 */

// C dependencies are supplied by the surrounding kernel translation unit.

pub static mut hyp_nr_cpus: libc::c_ulong = 0;

// #define hyp_percpu_size ((unsigned long)__per_cpu_end - (unsigned long)__per_cpu_start)

static mut vmemmap_base: *mut libc::c_void = core::ptr::null_mut();
static mut vm_table_base: *mut libc::c_void = core::ptr::null_mut();
static mut hyp_pgt_base: *mut libc::c_void = core::ptr::null_mut();
static mut host_s2_pgt_base: *mut libc::c_void = core::ptr::null_mut();
static mut selftest_base: *mut libc::c_void = core::ptr::null_mut();
static mut ffa_proxy_pages: *mut libc::c_void = core::ptr::null_mut();
static mut pkvm_pgtable_mm_ops: kvm_pgtable_mm_ops = kvm_pgtable_mm_ops {};
static mut hpool: hyp_pool = hyp_pool {};

unsafe fn divide_memory_pool(virt: *mut libc::c_void, size: libc::c_ulong) -> libc::c_int {
    let mut nr_pages: libc::c_ulong;

    hyp_early_alloc_init(virt, size);

    nr_pages = pkvm_selftest_pages();
    selftest_base = hyp_early_alloc_contig(nr_pages);
    if nr_pages != 0 && selftest_base.is_null() { return -ENOMEM; }

    nr_pages = hyp_vmemmap_pages(core::mem::size_of::<hyp_page>() as libc::c_ulong);
    vmemmap_base = hyp_early_alloc_contig(nr_pages);
    if vmemmap_base.is_null() { return -ENOMEM; }

    nr_pages = hyp_vm_table_pages();
    vm_table_base = hyp_early_alloc_contig(nr_pages);
    if vm_table_base.is_null() { return -ENOMEM; }

    nr_pages = hyp_s1_pgtable_pages();
    hyp_pgt_base = hyp_early_alloc_contig(nr_pages);
    if hyp_pgt_base.is_null() { return -ENOMEM; }

    nr_pages = host_s2_pgtable_pages();
    host_s2_pgt_base = hyp_early_alloc_contig(nr_pages);
    if host_s2_pgt_base.is_null() { return -ENOMEM; }

    nr_pages = hyp_ffa_proxy_pages();
    ffa_proxy_pages = hyp_early_alloc_contig(nr_pages);
    if ffa_proxy_pages.is_null() { return -ENOMEM; }

    0
}

unsafe fn pkvm_create_host_sve_mappings() -> libc::c_int {
    if !system_supports_sve() { return 0; }

    for i in 0..hyp_nr_cpus {
        let host_data = per_cpu_ptr(&raw mut kvm_host_data, i);
        let sve_regs = (*host_data).sve_regs;
        let start = kern_hyp_va(sve_regs);
        let end = (start as usize + PAGE_ALIGN(pkvm_host_sve_state_size()) as usize) as *mut libc::c_void;
        let ret = pkvm_create_mappings(start, end, PAGE_HYP);
        if ret != 0 { return ret; }
    }
    0
}

unsafe fn recreate_hyp_mappings(phys: phys_addr_t, size: libc::c_ulong,
                                per_cpu_base: *mut libc::c_ulong,
                                hyp_va_bits: u32) -> libc::c_int {
    let virt = hyp_phys_to_virt(phys);
    let pgt_size = hyp_s1_pgtable_pages() << PAGE_SHIFT;
    hyp_early_alloc_init(hyp_pgt_base, pgt_size);
    let mut ret = kvm_pgtable_hyp_init(&raw mut pkvm_pgtable, hyp_va_bits, &raw const hyp_early_alloc_mm_ops);
    if ret != 0 { return ret; }
    ret = hyp_create_idmap(hyp_va_bits); if ret != 0 { return ret; }
    ret = hyp_map_vectors(); if ret != 0 { return ret; }
    ret = hyp_back_vmemmap(hyp_virt_to_phys(vmemmap_base)); if ret != 0 { return ret; }
    ret = pkvm_create_mappings(__hyp_text_start, __hyp_text_end, PAGE_HYP_EXEC); if ret != 0 { return ret; }
    ret = pkvm_create_mappings(__hyp_data_start, __hyp_data_end, PAGE_HYP); if ret != 0 { return ret; }
    ret = pkvm_create_mappings(__hyp_rodata_start, __hyp_rodata_end, PAGE_HYP_RO); if ret != 0 { return ret; }
    ret = pkvm_create_mappings(__hyp_bss_start, __hyp_bss_end, PAGE_HYP); if ret != 0 { return ret; }
    ret = pkvm_create_mappings(virt, (virt as usize + size as usize) as *mut libc::c_void, PAGE_HYP); if ret != 0 { return ret; }
    for i in 0..hyp_nr_cpus {
        let params = per_cpu_ptr(&raw mut kvm_init_params, i);
        let start = kern_hyp_va(*per_cpu_base.add(i as usize)) as *mut libc::c_void;
        let end = (start as usize + PAGE_ALIGN(hyp_percpu_size()) as usize) as *mut libc::c_void;
        ret = pkvm_create_mappings(start, end, PAGE_HYP); if ret != 0 { return ret; }
        ret = pkvm_create_stack((*params).stack_pa, &raw mut (*params).stack_hyp_va); if ret != 0 { return ret; }
    }
    pkvm_create_host_sve_mappings()
}

unsafe fn hyp_percpu_size() -> libc::c_ulong {
    (__per_cpu_end as usize - __per_cpu_start as usize) as libc::c_ulong
}

unsafe fn update_nvhe_init_params() {
    for i in 0..hyp_nr_cpus {
        let params = per_cpu_ptr(&raw mut kvm_init_params, i);
        (*params).pgd_pa = __hyp_pa(pkvm_pgtable.pgd);
        dcache_clean_inval_poc(params as libc::c_ulong, params as libc::c_ulong + core::mem::size_of::<kvm_nvhe_init_params>() as libc::c_ulong);
    }
}

unsafe fn hyp_zalloc_hyp_page(_arg: *mut libc::c_void) -> *mut libc::c_void { hyp_alloc_pages(&raw mut hpool, 0) }
unsafe fn hpool_get_page(addr: *mut libc::c_void) { hyp_get_page(&raw mut hpool, addr); }
unsafe fn hpool_put_page(addr: *mut libc::c_void) { hyp_put_page(&raw mut hpool, addr); }

unsafe fn fix_host_ownership_walker(ctx: *const kvm_pgtable_visit_ctx, _visit: kvm_pgtable_walk_flags) -> libc::c_int {
    if !kvm_pte_valid((*ctx).old) { return 0; }
    if (*ctx).level != KVM_PGTABLE_LAST_LEVEL { return -EINVAL; }
    let phys = kvm_pte_to_phys((*ctx).old);
    if !addr_is_memory(phys) { return -EINVAL; }
    let page = hyp_phys_to_page(phys);
    let prot = kvm_pgtable_hyp_pte_prot((*ctx).old);
    match pkvm_getstate(prot) {
        PKVM_PAGE_OWNED => { set_hyp_state(page, PKVM_PAGE_OWNED); if prot == PAGE_HYP_EXEC { set_host_state(page, PKVM_NOPAGE); host_stage2_idmap_locked(phys, PAGE_SIZE, KVM_PGTABLE_PROT_R) } else { host_stage2_set_owner_locked(phys, PAGE_SIZE, PKVM_ID_HYP) } },
        PKVM_PAGE_SHARED_OWNED => { set_hyp_state(page, PKVM_PAGE_SHARED_OWNED); set_host_state(page, PKVM_PAGE_SHARED_BORROWED); 0 },
        PKVM_PAGE_SHARED_BORROWED => { set_hyp_state(page, PKVM_PAGE_SHARED_BORROWED); set_host_state(page, PKVM_PAGE_SHARED_OWNED); 0 },
        _ => -EINVAL,
    }
}

unsafe fn fix_hyp_pgtable_refcnt_walker(ctx: *const kvm_pgtable_visit_ctx, _visit: kvm_pgtable_walk_flags) -> libc::c_int {
    if kvm_pte_valid((*ctx).old) { ((*ctx).mm_ops).get_page.unwrap()((*ctx).ptep); }
    0
}

unsafe fn fix_host_ownership() -> libc::c_int {
    let walker = kvm_pgtable_walker { cb: Some(fix_host_ownership_walker), flags: KVM_PGTABLE_WALK_LEAF, ..core::mem::zeroed() };
    for i in 0..hyp_memblock_nr { let reg = &hyp_memory[i as usize]; let ret = kvm_pgtable_walk(&raw mut pkvm_pgtable, hyp_phys_to_virt(reg.base) as u64, reg.size, &raw const walker); if ret != 0 { return ret; } }
    0
}

unsafe fn fix_hyp_pgtable_refcnt() -> libc::c_int {
    let walker = kvm_pgtable_walker { cb: Some(fix_hyp_pgtable_refcnt_walker), flags: KVM_PGTABLE_WALK_LEAF | KVM_PGTABLE_WALK_TABLE_POST, arg: pkvm_pgtable.mm_ops, ..core::mem::zeroed() };
    kvm_pgtable_walk(&raw mut pkvm_pgtable, 0, BIT(pkvm_pgtable.ia_bits), &raw const walker)
}

pub unsafe fn __pkvm_init_finalise() -> ! {
    let host_ctxt = host_data_ptr(host_ctxt);
    let pfn = hyp_virt_to_pfn(hyp_pgt_base);
    let nr_pages = hyp_s1_pgtable_pages();
    let reserved_pages = hyp_early_alloc_nr_used_pages();
    let mut ret = hyp_pool_init(&raw mut hpool, pfn, nr_pages, reserved_pages);
    if ret == 0 { ret = kvm_host_prepare_stage2(host_s2_pgt_base); }
    if ret == 0 {
        pkvm_pgtable_mm_ops = kvm_pgtable_mm_ops {
            zalloc_page: Some(hyp_zalloc_hyp_page), phys_to_virt: Some(hyp_phys_to_virt),
            virt_to_phys: Some(hyp_virt_to_phys), get_page: Some(hpool_get_page),
            put_page: Some(hpool_put_page), page_count: Some(hyp_page_count),
        };
        pkvm_pgtable.mm_ops = &raw mut pkvm_pgtable_mm_ops;
        ret = fix_hyp_pgtable_refcnt();
    }
    if ret == 0 { ret = hyp_create_fixmap(); }
    if ret == 0 { ret = fix_host_ownership(); }
    if ret == 0 { ret = hyp_ffa_init(ffa_proxy_pages); }
    if ret == 0 { pkvm_hyp_vm_table_init(vm_table_base); pkvm_ownership_selftest(selftest_base); }
    cpu_reg(host_ctxt, 1) = ret;
    __host_enter(host_ctxt);
}

pub unsafe fn __pkvm_init(phys: phys_addr_t, size: libc::c_ulong,
                          per_cpu_base: *mut libc::c_ulong, hyp_va_bits: u32) -> libc::c_int {
    BUG_ON(kvm_check_pvm_sysreg_table());
    if !PAGE_ALIGNED(phys) || !PAGE_ALIGNED(size) { return -EINVAL; }
    hyp_spin_lock_init(&raw mut pkvm_pgd_lock);
    let virt = hyp_phys_to_virt(phys);
    let mut ret = divide_memory_pool(virt, size);
    if ret != 0 { return ret; }
    ret = recreate_hyp_mappings(phys, size, per_cpu_base, hyp_va_bits);
    if ret != 0 { return ret; }
    update_nvhe_init_params();
    let params = this_cpu_ptr(&raw mut kvm_init_params);
    let fn_ptr = __pkvm_init_switch_pgd as unsafe extern "C" fn(_, _, _) -> _;
    fn_ptr((*params).pgd_pa, (*params).stack_hyp_va, __pkvm_init_finalise);
    core::hint::unreachable_unchecked()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
