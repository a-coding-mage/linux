// SPDX-License-Identifier: GPL-2.0-only
/* AMD Memory Encryption Support */

// The C source overrides __pa/__va for identity mappings and disables
// paravirtual indirections and early paging feature detection.
// Required kernel and architecture headers are supplied by other modules.

const PGD_FLAGS: usize = _KERNPG_TABLE_NOENC;
const P4D_FLAGS: usize = _KERNPG_TABLE_NOENC;
const PUD_FLAGS: usize = _KERNPG_TABLE_NOENC;
const PMD_FLAGS: usize = _KERNPG_TABLE_NOENC;
const PMD_FLAGS_LARGE: usize = __PAGE_KERNEL_LARGE_EXEC & !_PAGE_GLOBAL;
const PMD_FLAGS_DEC: usize = PMD_FLAGS_LARGE;
const PMD_FLAGS_DEC_WP: usize = (PMD_FLAGS_DEC & !_PAGE_LARGE_CACHE_MASK) | (_PAGE_PAT_LARGE | _PAGE_PWT);
const PMD_FLAGS_ENC: usize = PMD_FLAGS_LARGE | _PAGE_ENC;
const PTE_FLAGS: usize = __PAGE_KERNEL_EXEC & !_PAGE_GLOBAL;
const PTE_FLAGS_DEC: usize = PTE_FLAGS;
const PTE_FLAGS_DEC_WP: usize = (PTE_FLAGS_DEC & !_PAGE_CACHE_MASK) | (_PAGE_PAT | _PAGE_PWT);
const PTE_FLAGS_ENC: usize = PTE_FLAGS | _PAGE_ENC;

#[repr(C)]
struct SmePopulatePgdData {
    pgtable_area: *mut core::ffi::c_void,
    pgd: *mut pgd_t,
    pmd_flags: pmdval_t,
    pte_flags: pteval_t,
    paddr: c_ulong,
    vaddr: c_ulong,
    vaddr_end: c_ulong,
}

// .init.scratch, sized for the intermediate copy buffer and page tables.
static mut SME_WORKAREA: [core::ffi::c_char; 2 * PMD_SIZE] = [0; 2 * PMD_SIZE];

unsafe fn sme_clear_pgd(ppd: *mut SmePopulatePgdData) {
    let pgd_start = (*ppd).vaddr & PGDIR_MASK;
    let pgd_end = (*ppd).vaddr_end & PGDIR_MASK;
    let pgd_size = (((pgd_end - pgd_start) / PGDIR_SIZE) + 1) * core::mem::size_of::<pgd_t>();
    let pgd_p = (*ppd).pgd.add(pgd_index((*ppd).vaddr));
    memset(pgd_p as *mut core::ffi::c_void, 0, pgd_size);
}

unsafe fn sme_prepare_pgd(ppd: *mut SmePopulatePgdData) -> *mut pud_t {
    let pgd = (*ppd).pgd.add(pgd_index((*ppd).vaddr));
    if pgd_none(*pgd) {
        let p4d = (*ppd).pgtable_area as *mut p4d_t;
        memset(p4d as *mut core::ffi::c_void, 0, core::mem::size_of::<p4d_t>() * PTRS_PER_P4D);
        (*ppd).pgtable_area = (*ppd).pgtable_area.add(core::mem::size_of::<p4d_t>() * PTRS_PER_P4D);
        set_pgd(pgd, __pgd(PGD_FLAGS | __pa(p4d)));
    }
    let p4d = p4d_offset(pgd, (*ppd).vaddr);
    if p4d_none(*p4d) {
        let pud = (*ppd).pgtable_area as *mut pud_t;
        memset(pud as *mut core::ffi::c_void, 0, core::mem::size_of::<pud_t>() * PTRS_PER_PUD);
        (*ppd).pgtable_area = (*ppd).pgtable_area.add(core::mem::size_of::<pud_t>() * PTRS_PER_PUD);
        set_p4d(p4d, __p4d(P4D_FLAGS | __pa(pud)));
    }
    let pud = pud_offset(p4d, (*ppd).vaddr);
    if pud_none(*pud) {
        let pmd = (*ppd).pgtable_area as *mut pmd_t;
        memset(pmd as *mut core::ffi::c_void, 0, core::mem::size_of::<pmd_t>() * PTRS_PER_PMD);
        (*ppd).pgtable_area = (*ppd).pgtable_area.add(core::mem::size_of::<pmd_t>() * PTRS_PER_PMD);
        set_pud(pud, __pud(PUD_FLAGS | __pa(pmd)));
    }
    if pud_leaf(*pud) { return core::ptr::null_mut(); }
    pud
}

unsafe fn sme_populate_pgd_large(ppd: *mut SmePopulatePgdData) {
    let pud = sme_prepare_pgd(ppd);
    if pud.is_null() { return; }
    let pmd = pmd_offset(pud, (*ppd).vaddr);
    if pmd_leaf(*pmd) { return; }
    set_pmd(pmd, __pmd((*ppd).paddr | (*ppd).pmd_flags));
}

unsafe fn sme_populate_pgd(ppd: *mut SmePopulatePgdData) {
    let pud = sme_prepare_pgd(ppd);
    if pud.is_null() { return; }
    let pmd = pmd_offset(pud, (*ppd).vaddr);
    if pmd_none(*pmd) {
        let pte = (*ppd).pgtable_area as *mut pte_t;
        memset(pte as *mut core::ffi::c_void, 0, core::mem::size_of::<pte_t>() * PTRS_PER_PTE);
        (*ppd).pgtable_area = (*ppd).pgtable_area.add(core::mem::size_of::<pte_t>() * PTRS_PER_PTE);
        set_pmd(pmd, __pmd(PMD_FLAGS | __pa(pte)));
    }
    if pmd_leaf(*pmd) { return; }
    let pte = pte_offset_kernel(pmd, (*ppd).vaddr);
    if pte_none(*pte) { set_pte(pte, __pte((*ppd).paddr | (*ppd).pte_flags)); }
}

unsafe fn __sme_map_range_pmd(ppd: *mut SmePopulatePgdData) {
    while (*ppd).vaddr < (*ppd).vaddr_end {
        sme_populate_pgd_large(ppd);
        (*ppd).vaddr += PMD_SIZE;
        (*ppd).paddr += PMD_SIZE;
    }
}

unsafe fn __sme_map_range_pte(ppd: *mut SmePopulatePgdData) {
    while (*ppd).vaddr < (*ppd).vaddr_end {
        sme_populate_pgd(ppd);
        (*ppd).vaddr += PAGE_SIZE;
        (*ppd).paddr += PAGE_SIZE;
    }
}

unsafe fn __sme_map_range(ppd: *mut SmePopulatePgdData, pmd_flags: pmdval_t, pte_flags: pteval_t) {
    (*ppd).pmd_flags = pmd_flags;
    (*ppd).pte_flags = pte_flags;
    let vaddr_end = (*ppd).vaddr_end;
    (*ppd).vaddr_end = ALIGN((*ppd).vaddr, PMD_SIZE);
    __sme_map_range_pte(ppd);
    (*ppd).vaddr_end = vaddr_end & PMD_MASK;
    __sme_map_range_pmd(ppd);
    (*ppd).vaddr_end = vaddr_end;
    __sme_map_range_pte(ppd);
}

unsafe fn sme_map_range_encrypted(ppd: *mut SmePopulatePgdData) { __sme_map_range(ppd, PMD_FLAGS_ENC, PTE_FLAGS_ENC); }
unsafe fn sme_map_range_decrypted(ppd: *mut SmePopulatePgdData) { __sme_map_range(ppd, PMD_FLAGS_DEC, PTE_FLAGS_DEC); }
unsafe fn sme_map_range_decrypted_wp(ppd: *mut SmePopulatePgdData) { __sme_map_range(ppd, PMD_FLAGS_DEC_WP, PTE_FLAGS_DEC_WP); }

unsafe fn sme_pgtable_calc(len: c_ulong) -> c_ulong {
    let mut entries = 0;
    let mut tables = 0;
    if PTRS_PER_P4D > 1 { entries += (DIV_ROUND_UP(len, PGDIR_SIZE) + 1) * core::mem::size_of::<p4d_t>() * PTRS_PER_P4D; }
    entries += (DIV_ROUND_UP(len, P4D_SIZE) + 1) * core::mem::size_of::<pud_t>() * PTRS_PER_PUD;
    entries += (DIV_ROUND_UP(len, PUD_SIZE) + 1) * core::mem::size_of::<pmd_t>() * PTRS_PER_PMD;
    entries += 2 * core::mem::size_of::<pte_t>() * PTRS_PER_PTE;
    if PTRS_PER_P4D > 1 { tables += DIV_ROUND_UP(entries, PGDIR_SIZE) * core::mem::size_of::<p4d_t>() * PTRS_PER_P4D; }
    tables += DIV_ROUND_UP(entries, P4D_SIZE) * core::mem::size_of::<pud_t>() * PTRS_PER_PUD;
    tables += DIV_ROUND_UP(entries, PUD_SIZE) * core::mem::size_of::<pmd_t>() * PTRS_PER_PMD;
    entries + tables
}

pub unsafe fn sme_encrypt_kernel(bp: *mut boot_params) {
    let (mut workarea_start, mut workarea_end, mut workarea_len);
    let (mut execute_start, mut execute_end, mut execute_len);
    let (mut kernel_start, mut kernel_end, mut kernel_len);
    let (mut initrd_start, mut initrd_end, mut initrd_len);
    let mut ppd: SmePopulatePgdData;
    let mut pgtable_area_len;
    let mut decrypted_base;
    if sme_get_me_mask() == 0 || (sev_status & MSR_AMD64_SEV_ENABLED) != 0 { return; }
    kernel_start = rip_rel_ptr(_text) as c_ulong;
    kernel_end = ALIGN(rip_rel_ptr(_end) as c_ulong, PMD_SIZE);
    kernel_len = kernel_end - kernel_start;
    initrd_start = 0; initrd_end = 0; initrd_len = 0;
    // CONFIG_BLK_DEV_INITRD controls the following source-level block.
    initrd_len = (*bp).hdr.ramdisk_size as c_ulong | ((*bp).ext_ramdisk_size as c_ulong) << 32;
    if initrd_len != 0 {
        initrd_start = (*bp).hdr.ramdisk_image as c_ulong | ((*bp).ext_ramdisk_image as c_ulong) << 32;
        initrd_end = PAGE_ALIGN(initrd_start + initrd_len);
        initrd_len = initrd_end - initrd_start;
    }
    execute_start = workarea_start = SME_WORKAREA.as_mut_ptr() as c_ulong;
    execute_end = execute_start + PAGE_SIZE * 2 + PMD_SIZE;
    execute_len = execute_end - execute_start;
    pgtable_area_len = core::mem::size_of::<pgd_t>() * PTRS_PER_PGD;
    pgtable_area_len += sme_pgtable_calc(execute_end - kernel_start) * 2;
    if initrd_len != 0 { pgtable_area_len += sme_pgtable_calc(initrd_len) * 2; }
    pgtable_area_len += sme_pgtable_calc(execute_len + pgtable_area_len);
    workarea_len = execute_len + pgtable_area_len;
    workarea_end = ALIGN(workarea_start + workarea_len, PMD_SIZE);
    ppd.pgtable_area = execute_end as *mut core::ffi::c_void;
    ppd.pgd = native_read_cr3_pa() as *mut pgd_t;
    ppd.paddr = workarea_start; ppd.vaddr = workarea_start; ppd.vaddr_end = workarea_end;
    sme_map_range_decrypted(&mut ppd);
    native_write_cr3(__native_read_cr3());
    ppd.pgd = ppd.pgtable_area as *mut pgd_t;
    memset(ppd.pgd as *mut core::ffi::c_void, 0, core::mem::size_of::<pgd_t>() * PTRS_PER_PGD);
    ppd.pgtable_area = ppd.pgtable_area.add(core::mem::size_of::<pgd_t>() * PTRS_PER_PGD);
    decrypted_base = (pgd_index(workarea_end) + 1) & (PTRS_PER_PGD - 1);
    if initrd_len != 0 { decrypted_base = core::cmp::max(decrypted_base, (pgd_index(initrd_end) + 1) & (PTRS_PER_PGD - 1)); }
    decrypted_base <<= PGDIR_SHIFT;
    ppd.paddr = kernel_start; ppd.vaddr = kernel_start; ppd.vaddr_end = kernel_end; sme_map_range_encrypted(&mut ppd);
    ppd.paddr = kernel_start; ppd.vaddr = kernel_start + decrypted_base; ppd.vaddr_end = kernel_end + decrypted_base; sme_map_range_decrypted_wp(&mut ppd);
    if initrd_len != 0 {
        ppd.paddr = initrd_start; ppd.vaddr = initrd_start; ppd.vaddr_end = initrd_end; sme_map_range_encrypted(&mut ppd);
        ppd.paddr = initrd_start; ppd.vaddr = initrd_start + decrypted_base; ppd.vaddr_end = initrd_end + decrypted_base; sme_map_range_decrypted_wp(&mut ppd);
    }
    ppd.paddr = workarea_start; ppd.vaddr = workarea_start; ppd.vaddr_end = workarea_end; sme_map_range_decrypted(&mut ppd);
    ppd.paddr = workarea_start; ppd.vaddr = workarea_start + decrypted_base; ppd.vaddr_end = workarea_end + decrypted_base; sme_map_range_decrypted(&mut ppd);
    sme_encrypt_execute(kernel_start, kernel_start + decrypted_base, kernel_len, workarea_start, ppd.pgd as c_ulong);
    if initrd_len != 0 { sme_encrypt_execute(initrd_start, initrd_start + decrypted_base, initrd_len, workarea_start, ppd.pgd as c_ulong); }
    ppd.vaddr = kernel_start + decrypted_base; ppd.vaddr_end = kernel_end + decrypted_base; sme_clear_pgd(&mut ppd);
    if initrd_len != 0 { ppd.vaddr = initrd_start + decrypted_base; ppd.vaddr_end = initrd_end + decrypted_base; sme_clear_pgd(&mut ppd); }
    ppd.vaddr = workarea_start + decrypted_base; ppd.vaddr_end = workarea_end + decrypted_base; sme_clear_pgd(&mut ppd);
    native_write_cr3(__native_read_cr3());
}

pub unsafe fn sme_enable(bp: *mut boot_params) {
    let (mut eax, mut ebx, mut ecx, mut edx): (u32, u32, u32, u32);
    let (mut feature_mask, mut me_mask): (c_ulong, c_ulong);
    let snp_en = snp_init(bp);
    eax = 0x80000000; ecx = 0; native_cpuid(&mut eax, &mut ebx, &mut ecx, &mut edx);
    if eax < 0x8000001f { return; }
    const AMD_SME_BIT: u32 = BIT(0);
    const AMD_SEV_BIT: u32 = BIT(1);
    eax = 0x8000001f; ecx = 0; native_cpuid(&mut eax, &mut ebx, &mut ecx, &mut edx);
    if (eax & (AMD_SEV_BIT | AMD_SME_BIT)) == 0 { return; }
    me_mask = 1 as c_ulong << (ebx & 0x3f);
    sev_snp_needs_sfw = (ebx & BIT(31)) == 0;
    sev_status = native_rdmsrq(MSR_AMD64_SEV); let msr = sev_status;
    feature_mask = if (msr & MSR_AMD64_SEV_ENABLED) != 0 { AMD_SEV_BIT as c_ulong } else { AMD_SME_BIT as c_ulong };
    if snp_en != ((msr & MSR_AMD64_SEV_SNP_ENABLED) != 0) { sev_es_terminate(SEV_TERM_SET_GEN, GHCB_SNP_UNSUPPORTED); }
    if feature_mask == AMD_SME_BIT as c_ulong {
        if ((*bp).hdr.xloadflags & XLF_MEM_ENCRYPTION) == 0 { return; }
        eax = 1; ecx = 0; native_cpuid(&mut eax, &mut ebx, &mut ecx, &mut edx);
        if (ecx & BIT(31)) != 0 { return; }
        let msr = native_rdmsrq(MSR_AMD64_SYSCFG);
        if (msr & MSR_AMD64_SYSCFG_MEM_ENCRYPT) == 0 { return; }
    }
    sme_me_mask = me_mask; physical_mask &= !me_mask; cc_vendor = CC_VENDOR_AMD; cc_set_mask(me_mask);
}

// Local startup-code version; user page tables are never operated on here.
#[cfg(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION)]
pub unsafe fn __pti_set_user_pgtbl(_pgdp: *mut pgd_t, pgd: pgd_t) -> pgd_t { pgd }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
