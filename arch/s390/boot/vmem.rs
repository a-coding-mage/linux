// SPDX-License-Identifier: GPL-2.0
// C dependencies and build-time configuration are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct ctlreg { pub val: ::core::ffi::c_ulong }

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum populate_mode {
    POPULATE_NONE,
    POPULATE_DIRECT,
    POPULATE_LOWCORE,
    POPULATE_ABS_LOWCORE,
    POPULATE_IDENTITY,
    POPULATE_KERNEL,
    // CONFIG_KASAN: these modes are last and grouped together.
    POPULATE_KASAN_MAP_SHADOW,
    POPULATE_KASAN_ZERO_SHADOW,
    POPULATE_KASAN_SHALLOW,
}

static mut s390_invalid_asce: ctlreg = ctlreg { val: 0 };

unsafe fn get_populate_mode_name(t: populate_mode) -> *const u8 {
    match t {
        populate_mode::POPULATE_NONE => b"NONE\0".as_ptr(),
        populate_mode::POPULATE_DIRECT => b"DIRECT\0".as_ptr(),
        populate_mode::POPULATE_LOWCORE => b"LOWCORE\0".as_ptr(),
        populate_mode::POPULATE_ABS_LOWCORE => b"ABS_LOWCORE\0".as_ptr(),
        populate_mode::POPULATE_IDENTITY => b"IDENTITY\0".as_ptr(),
        populate_mode::POPULATE_KERNEL => b"KERNEL\0".as_ptr(),
        populate_mode::POPULATE_KASAN_MAP_SHADOW => b"KASAN_MAP_SHADOW\0".as_ptr(),
        populate_mode::POPULATE_KASAN_ZERO_SHADOW => b"KASAN_ZERO_SHADOW\0".as_ptr(),
        populate_mode::POPULATE_KASAN_SHALLOW => b"KASAN_SHALLOW\0".as_ptr(),
    }
}

unsafe fn is_kasan_populate_mode(mode: populate_mode) -> bool {
    mode >= populate_mode::POPULATE_KASAN_MAP_SHADOW
}

unsafe fn pgtable_populate(addr: usize, end: usize, mode: populate_mode);

#[cfg(feature = "CONFIG_KASAN")]
static mut pte_z: pte_t = pte_t { val: 0 };

#[cfg(feature = "CONFIG_KASAN")]
unsafe fn kasan_populate(start: usize, end: usize, mode: populate_mode) {
    let sha_start = PAGE_ALIGN_DOWN(kasan_mem_to_shadow(start as *mut _ ) as usize);
    let sha_end = PAGE_ALIGN(kasan_mem_to_shadow(end as *mut _) as usize);
    boot_debug!("%-17s 0x%016lx-0x%016lx >> 0x%016lx-0x%016lx\n", get_populate_mode_name(mode), start, end, sha_start, sha_end);
    pgtable_populate(sha_start, sha_end, mode);
}

#[cfg(feature = "CONFIG_KASAN")]
unsafe fn kasan_populate_shadow(kernel_start: usize, kernel_end: usize) {
    let pmd_z = __pmd(__pa(kasan_early_shadow_pte) | _SEGMENT_ENTRY);
    let pud_z = __pud(__pa(kasan_early_shadow_pmd) | _REGION3_ENTRY);
    let p4d_z = __p4d(__pa(kasan_early_shadow_pud) | _REGION2_ENTRY);
    let mut memgap_start = 0usize;
    let (mut start, mut end): (usize, usize);
    let mut i: i32;
    pte_z = __pte(__pa(kasan_early_shadow_page) | pgprot_val(PAGE_KERNEL_RO));
    crst_table_init(kasan_early_shadow_p4d as *mut usize, p4d_val(p4d_z));
    crst_table_init(kasan_early_shadow_pud as *mut usize, pud_val(pud_z));
    crst_table_init(kasan_early_shadow_pmd as *mut usize, pmd_val(pmd_z));
    memset64(kasan_early_shadow_pte as *mut u64, pte_val(pte_z), PTRS_PER_PTE);
    __arch_set_page_dat(kasan_early_shadow_p4d, 1usize << CRST_ALLOC_ORDER);
    __arch_set_page_dat(kasan_early_shadow_pud, 1usize << CRST_ALLOC_ORDER);
    __arch_set_page_dat(kasan_early_shadow_pmd, 1usize << CRST_ALLOC_ORDER);
    __arch_set_page_dat(kasan_early_shadow_pte, 1);
    for_each_physmem_usable_range!(i, &mut start, &mut end) {
        kasan_populate(__identity_va(start) as usize, __identity_va(end) as usize, populate_mode::POPULATE_KASAN_MAP_SHADOW);
        if memgap_start != 0 && physmem_info.info_source == MEM_DETECT_DIAG260 {
            kasan_populate(__identity_va(memgap_start) as usize, __identity_va(start) as usize, populate_mode::POPULATE_KASAN_ZERO_SHADOW);
        }
        memgap_start = end;
    }
    kasan_populate(kernel_start + TEXT_OFFSET, kernel_end, populate_mode::POPULATE_KASAN_MAP_SHADOW);
    kasan_populate(0, __identity_va(0) as usize, populate_mode::POPULATE_KASAN_ZERO_SHADOW);
    kasan_populate(AMODE31_START, AMODE31_END, populate_mode::POPULATE_KASAN_ZERO_SHADOW);
    kasan_populate(VMALLOC_START, MODULES_END, populate_mode::POPULATE_KASAN_SHALLOW);
    kasan_populate(__identity_va(ident_map_size) as usize, VMALLOC_START, populate_mode::POPULATE_KASAN_ZERO_SHADOW);
    kasan_populate(kernel_end, _REGION1_SIZE, populate_mode::POPULATE_KASAN_ZERO_SHADOW);
}

#[cfg(not(feature = "CONFIG_KASAN"))]
unsafe fn kasan_populate_shadow(_: usize, _: usize) {}

unsafe fn kasan_pgd_populate_zero_shadow(_: *mut pgd_t, _: usize, _: usize, _: populate_mode) -> bool { false }
unsafe fn kasan_p4d_populate_zero_shadow(_: *mut p4d_t, _: usize, _: usize, _: populate_mode) -> bool { false }
unsafe fn kasan_pud_populate_zero_shadow(_: *mut pud_t, _: usize, _: usize, _: populate_mode) -> bool { false }
unsafe fn kasan_pmd_populate_zero_shadow(_: *mut pmd_t, _: usize, _: usize, _: populate_mode) -> bool { false }
unsafe fn kasan_pte_populate_zero_shadow(_: *mut pte_t, _: populate_mode) -> bool { false }

unsafe fn __virt_to_kpte(va: usize) -> *mut pte_t {
    pte_offset_kernel(pmd_offset(pud_offset(p4d_offset(pgd_offset_k(va), va), va), va), va)
}

unsafe fn boot_crst_alloc(val: usize) -> *mut usize {
    let size = PAGE_SIZE << CRST_ALLOC_ORDER;
    let table = physmem_alloc_or_die(RR_VMEM, size, size) as *mut usize;
    crst_table_init(table, val);
    __arch_set_page_dat(table as *mut _, 1usize << CRST_ALLOC_ORDER);
    table
}

unsafe fn boot_pte_alloc() -> *mut pte_t {
    let pte = physmem_alloc_or_die(RR_VMEM, PAGE_SIZE, PAGE_SIZE) as *mut pte_t;
    __arch_set_page_dat(pte as *mut _, 1);
    memset64(pte as *mut u64, _PAGE_INVALID, PTRS_PER_PTE);
    pte
}

unsafe fn resolve_pa_may_alloc(addr: usize, size: usize, mode: populate_mode) -> usize {
    match mode {
        populate_mode::POPULATE_NONE => INVALID_PHYS_ADDR,
        populate_mode::POPULATE_DIRECT => addr,
        populate_mode::POPULATE_LOWCORE => __lowcore_pa(addr),
        populate_mode::POPULATE_ABS_LOWCORE => __abs_lowcore_pa(addr),
        populate_mode::POPULATE_KERNEL => __kernel_pa(addr),
        populate_mode::POPULATE_IDENTITY => __identity_pa(addr),
        populate_mode::POPULATE_KASAN_MAP_SHADOW => {
            let p = physmem_alloc(RR_VMEM, size, size, size == PAGE_SIZE);
            if p != 0 { memset(p as *mut _, 0, size); p } else { INVALID_PHYS_ADDR }
        }
        _ => INVALID_PHYS_ADDR,
    }
}

unsafe fn large_page_mapping_allowed(mode: populate_mode) -> bool {
    matches!(mode, populate_mode::POPULATE_DIRECT | populate_mode::POPULATE_IDENTITY | populate_mode::POPULATE_KERNEL | populate_mode::POPULATE_KASAN_MAP_SHADOW)
}

unsafe fn try_get_large_pud_pa(_: *mut pud_t, addr: usize, end: usize, mode: populate_mode) -> usize {
    let size = end - addr;
    if !cpu_has_edat2() || !large_page_mapping_allowed(mode) || !IS_ALIGNED(addr, PUD_SIZE) || size < PUD_SIZE { return INVALID_PHYS_ADDR; }
    let pa = resolve_pa_may_alloc(addr, size, mode);
    if !IS_ALIGNED(pa, PUD_SIZE) { INVALID_PHYS_ADDR } else { pa }
}

unsafe fn try_get_large_pmd_pa(_: *mut pmd_t, addr: usize, end: usize, mode: populate_mode) -> usize {
    let size = end - addr;
    if !cpu_has_edat1() || !large_page_mapping_allowed(mode) || !IS_ALIGNED(addr, PMD_SIZE) || size < PMD_SIZE { return INVALID_PHYS_ADDR; }
    let pa = resolve_pa_may_alloc(addr, size, mode);
    if !IS_ALIGNED(pa, PMD_SIZE) { INVALID_PHYS_ADDR } else { pa }
}

unsafe fn pgtable_pte_populate(pmd: *mut pmd_t, mut addr: usize, end: usize, mode: populate_mode) {
    let mut pages = 0; let mut pte = pte_offset_kernel(pmd, addr);
    while addr < end { if pte_none(ptep_get(pte)) { if kasan_pte_populate_zero_shadow(pte, mode) { addr += PAGE_SIZE; pte = pte.add(1); continue; } let mut entry = __pte(resolve_pa_may_alloc(addr, PAGE_SIZE, mode)); entry = set_pte_bit(entry, PAGE_KERNEL); set_pte(pte, entry); pages += 1; } addr += PAGE_SIZE; pte = pte.add(1); }
    if mode == populate_mode::POPULATE_IDENTITY { update_page_count(PG_DIRECT_MAP_4K, pages); }
}

unsafe fn pgtable_pmd_populate(pud: *mut pud_t, mut addr: usize, end: usize, mode: populate_mode) {
    let mut pages = 0; let mut pmd = pmd_offset(pud, addr);
    while addr < end { let next = pmd_addr_end(addr, end); let entry = pmdp_get(pmd); if pmd_none(entry) { if kasan_pmd_populate_zero_shadow(pmd, addr, next, mode) { addr = next; pmd = pmd.add(1); continue; } let pa = try_get_large_pmd_pa(pmd, addr, next, mode); if pa != INVALID_PHYS_ADDR { let mut e = __pmd(pa); e = set_pmd_bit(e, SEGMENT_KERNEL); set_pmd(pmd, e); pages += 1; addr = next; pmd = pmd.add(1); continue; } let pte = boot_pte_alloc(); pmd_populate(&init_mm, pmd, pte); } else if pmd_leaf(entry) { addr = next; pmd = pmd.add(1); continue; } pgtable_pte_populate(pmd, addr, next, mode); addr = next; pmd = pmd.add(1); }
    if mode == populate_mode::POPULATE_IDENTITY { update_page_count(PG_DIRECT_MAP_1M, pages); }
}

unsafe fn pgtable_pud_populate(p4d: *mut p4d_t, mut addr: usize, end: usize, mode: populate_mode) {
    let mut pages = 0; let mut pud = pud_offset(p4d, addr);
    while addr < end { let next = pud_addr_end(addr, end); let entry = pudp_get(pud); if pud_none(entry) { if kasan_pud_populate_zero_shadow(pud, addr, next, mode) { addr = next; pud = pud.add(1); continue; } let pa = try_get_large_pud_pa(pud, addr, next, mode); if pa != INVALID_PHYS_ADDR { let mut e = __pud(pa); e = set_pud_bit(e, REGION3_KERNEL); set_pud(pud, e); pages += 1; addr = next; pud = pud.add(1); continue; } let pmd = boot_crst_alloc(_SEGMENT_ENTRY_EMPTY); pud_populate(&init_mm, pud, pmd); } else if pud_leaf(entry) { addr = next; pud = pud.add(1); continue; } pgtable_pmd_populate(pud, addr, next, mode); addr = next; pud = pud.add(1); }
    if mode == populate_mode::POPULATE_IDENTITY { update_page_count(PG_DIRECT_MAP_2G, pages); }
}

unsafe fn pgtable_p4d_populate(pgd: *mut pgd_t, mut addr: usize, end: usize, mode: populate_mode) {
    let mut p4d = p4d_offset(pgd, addr); while addr < end { let next = p4d_addr_end(addr, end); if p4d_none(p4dp_get(p4d)) { if kasan_p4d_populate_zero_shadow(p4d, addr, next, mode) { addr = next; p4d = p4d.add(1); continue; } let pud = boot_crst_alloc(_REGION3_ENTRY_EMPTY); p4d_populate(&init_mm, p4d, pud); } pgtable_pud_populate(p4d, addr, next, mode); addr = next; p4d = p4d.add(1); }
}

unsafe fn pgtable_populate(addr: usize, end: usize, mode: populate_mode) {
    if !is_kasan_populate_mode(mode) { boot_debug!("%-17s 0x%016lx-0x%016lx -> 0x%016lx-0x%016lx\n", get_populate_mode_name(mode), addr, end, resolve_pa_may_alloc(addr, 0, mode), resolve_pa_may_alloc(end - 1, 0, mode) + 1); }
    let mut addr = addr; let mut pgd = pgd_offset(&init_mm, addr);
    while addr < end { let next = pgd_addr_end(addr, end); if pgd_none(pgdp_get(pgd)) { if kasan_pgd_populate_zero_shadow(pgd, addr, next, mode) { addr = next; pgd = pgd.add(1); continue; } let p4d = boot_crst_alloc(_REGION2_ENTRY_EMPTY); pgd_populate(&init_mm, pgd, p4d); } if mode == populate_mode::POPULATE_KASAN_SHALLOW { addr = next; pgd = pgd.add(1); continue; } pgtable_p4d_populate(pgd, addr, next, mode); addr = next; pgd = pgd.add(1); }
}

pub unsafe fn setup_vmem(kernel_start: usize, kernel_end: usize, asce_limit: usize) {
    let mut lowcore_address = 0; let (mut start, mut end): (usize, usize); let mut i: i32;
    for_each_physmem_online_range!(i, &mut start, &mut end) { __arch_set_page_nodat(start as *mut _, (end - start) >> PAGE_SHIFT); }
    let init_mm_pgd = init_mm.pgd; init_mm.pgd = swapper_pg_dir as *mut pgd_t;
    let (asce_type, asce_bits) = if asce_limit == _REGION1_SIZE { (_REGION2_ENTRY_EMPTY, _ASCE_TYPE_REGION2 | _ASCE_TABLE_LENGTH) } else { (_REGION3_ENTRY_EMPTY, _ASCE_TYPE_REGION3 | _ASCE_TABLE_LENGTH) };
    s390_invalid_asce.val = invalid_pg_dir | _ASCE_TYPE_REGION3 | _ASCE_TABLE_LENGTH;
    crst_table_init(swapper_pg_dir as *mut usize, asce_type); crst_table_init(invalid_pg_dir as *mut usize, _REGION3_ENTRY_EMPTY);
    __arch_set_page_dat(swapper_pg_dir as *mut _, 1usize << CRST_ALLOC_ORDER); __arch_set_page_dat(invalid_pg_dir as *mut _, 1usize << CRST_ALLOC_ORDER);
    if machine_has_relocated_lowcore() { lowcore_address = LOWCORE_ALT_ADDRESS; }
    pgtable_populate(lowcore_address, lowcore_address + core::mem::size_of::<lowcore>(), populate_mode::POPULATE_LOWCORE);
    for_each_physmem_usable_range!(i, &mut start, &mut end) { if start == 0 { start = core::mem::size_of::<lowcore>(); } pgtable_populate(__identity_va(start) as usize, __identity_va(end) as usize, populate_mode::POPULATE_IDENTITY); }
    pgtable_populate(kernel_start + TEXT_OFFSET, kernel_end, populate_mode::POPULATE_KERNEL); pgtable_populate(AMODE31_START, AMODE31_END, populate_mode::POPULATE_DIRECT); pgtable_populate(__abs_lowcore, __abs_lowcore + core::mem::size_of::<lowcore>(), populate_mode::POPULATE_ABS_LOWCORE); pgtable_populate(__memcpy_real_area, __memcpy_real_area + PAGE_SIZE, populate_mode::POPULATE_NONE);
    memcpy_real_ptep = __identity_va(__virt_to_kpte(__memcpy_real_area)); kasan_populate_shadow(kernel_start, kernel_end); get_lowcore().kernel_asce.val = swapper_pg_dir | asce_bits; get_lowcore().user_asce = s390_invalid_asce; local_ctl_load(1, &get_lowcore().kernel_asce); local_ctl_load(7, &get_lowcore().user_asce); local_ctl_load(13, &get_lowcore().kernel_asce); init_mm.context.asce = get_lowcore().kernel_asce.val; init_mm.pgd = init_mm_pgd;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
