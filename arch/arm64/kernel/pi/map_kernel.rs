// SPDX-License-Identifier: GPL-2.0-only
// Copyright 2023 Google LLC
// Author: Ard Biesheuvel <ardb@google.com>

// Kernel dependencies supplied by the surrounding tree are intentionally not
// reimplemented here.

extern "C" {
    static __eh_frame_start: u8;
    static __eh_frame_end: u8;

    fn idmap_cpu_replace_ttbr1(pgdir: phys_addr_t);
}

unsafe fn map_segment(
    pg_dir: *mut pgd_t,
    pgd: *mut phys_addr_t,
    va_offset: u64,
    start: *mut core::ffi::c_void,
    end: *mut core::ffi::c_void,
    prot: pgprot_t,
    may_use_cont: bool,
    root_level: i32,
) {
    map_range(
        pgd,
        ((start as u64).wrapping_add(va_offset)) & !PAGE_OFFSET,
        ((end as u64).wrapping_add(va_offset)) & !PAGE_OFFSET,
        start as u64,
        prot,
        root_level,
        pg_dir as *mut pte_t,
        may_use_cont,
        0,
    );
}

unsafe fn unmap_segment(
    pg_dir: *mut pgd_t,
    va_offset: u64,
    start: *mut core::ffi::c_void,
    end: *mut core::ffi::c_void,
    root_level: i32,
) {
    map_segment(
        pg_dir,
        core::ptr::null_mut(),
        va_offset,
        start,
        end,
        __pgprot(0),
        false,
        root_level,
    );
}

unsafe fn map_kernel(kaslr_offset: u64, va_offset: u64, root_level: i32) {
    let mut enable_scs = IS_ENABLED(CONFIG_UNWIND_PATCH_PAC_INTO_SCS);
    let mut twopass = IS_ENABLED(CONFIG_RELOCATABLE);
    let mut pgdp: phys_addr_t = (init_pg_dir as phys_addr_t).wrapping_add(PAGE_SIZE);
    let mut text_prot: pgprot_t = PAGE_KERNEL_ROX;
    let data_prot: pgprot_t = PAGE_KERNEL;
    let prot: pgprot_t;

    if arm64_test_sw_feature_override(ARM64_SW_FEATURE_OVERRIDE_RODATA_OFF) {
        text_prot = PAGE_KERNEL_EXEC;
    }

    if IS_ENABLED(CONFIG_ARM64_PTR_AUTH_KERNEL) && cpu_has_pac() {
        enable_scs = false;
    }

    if IS_ENABLED(CONFIG_ARM64_BTI_KERNEL) && cpu_has_bti() {
        enable_scs = false;
        text_prot = __pgprot_modify(text_prot, PTE_GP, PTE_GP);
    }

    twopass |= enable_scs;
    prot = if twopass { data_prot } else { text_prot };

    map_segment(init_pg_dir, &mut pgdp, va_offset, _text, _stext, data_prot, false, root_level);
    map_segment(init_pg_dir, &mut pgdp, va_offset, _stext, _etext, prot, !twopass, root_level);
    map_segment(init_pg_dir, &mut pgdp, va_offset, __start_rodata, __inittext_begin, data_prot, false, root_level);
    map_segment(init_pg_dir, &mut pgdp, va_offset, __inittext_begin, __inittext_end, prot, false, root_level);
    map_segment(init_pg_dir, &mut pgdp, va_offset, __initdata_begin, __initdata_end, data_prot, false, root_level);
    map_segment(init_pg_dir, &mut pgdp, va_offset, _data, _end, data_prot, true, root_level);
    dsb(ishst);

    idmap_cpu_replace_ttbr1(init_pg_dir as phys_addr_t);

    if twopass {
        if IS_ENABLED(CONFIG_RELOCATABLE) {
            relocate_kernel(kaslr_offset);
        }

        if enable_scs {
            scs_patch(
                (&__eh_frame_start as *const u8).wrapping_add(va_offset as usize),
                (&__eh_frame_end as *const u8).offset_from(&__eh_frame_start) as usize,
                false,
            );
            core::arch::asm!("ic ialluis");
            dynamic_scs_is_enabled = true;
        }

        unmap_segment(init_pg_dir, va_offset, _stext, _etext, root_level);
        dsb(ishst);
        isb();
        __tlbi(vmalle1);
        isb();

        map_segment(init_pg_dir, core::ptr::null_mut(), va_offset, _stext, _etext, text_prot, true, root_level);
        map_segment(init_pg_dir, core::ptr::null_mut(), va_offset, __inittext_begin, __inittext_end, text_prot, false, root_level);
    }

    memcpy((swapper_pg_dir as *mut u8).wrapping_add(va_offset as usize) as *mut core::ffi::c_void, init_pg_dir as *const core::ffi::c_void, PAGE_SIZE);
    dsb(ishst);
    idmap_cpu_replace_ttbr1(swapper_pg_dir as phys_addr_t);
}

unsafe fn set_ttbr0_for_lpa2(ttbr: phys_addr_t) {
    let sctlr = read_sysreg(sctlr_el1);
    let mut tcr = read_sysreg(tcr_el1) | TCR_EL1_DS;
    let mmfr0 = read_sysreg(id_aa64mmfr0_el1);
    let parange = cpuid_feature_extract_unsigned_field(mmfr0, ID_AA64MMFR0_EL1_PARANGE_SHIFT);

    tcr &= !TCR_EL1_IPS_MASK;
    tcr |= parange << TCR_EL1_IPS_SHIFT;

    core::arch::asm!(
        "msr sctlr_el1, {sctlr_off}; isb; msr ttbr0_el1, {ttbr}; msr tcr_el1, {tcr}; isb; tlbi vmalle1; dsb nsh; isb; msr sctlr_el1, {sctlr}; isb",
        sctlr_off = in(reg) (sctlr & !SCTLR_ELx_M), ttbr = in(reg) ttbr, tcr = in(reg) tcr, sctlr = in(reg) sctlr,
    );
}

unsafe fn remap_idmap_for_lpa2() {
    let mask: ptval_t = PTE_SHARED;
    create_init_idmap(init_pg_dir, mask);
    dsb(ishst);
    set_ttbr0_for_lpa2(init_pg_dir as phys_addr_t);
    memset(init_idmap_pg_dir as *mut core::ffi::c_void, 0, (init_idmap_pg_end as usize) - (init_idmap_pg_dir as usize));
    create_init_idmap(init_idmap_pg_dir, mask);
    dsb(ishst);
    set_ttbr0_for_lpa2(init_idmap_pg_dir as phys_addr_t);
    memset(init_pg_dir as *mut core::ffi::c_void, 0, (init_pg_end as usize) - (init_pg_dir as usize));
}

unsafe fn map_fdt(fdt: phys_addr_t) -> *mut core::ffi::c_void {
    static mut ptes: [u8; INIT_IDMAP_FDT_SIZE] = [0; INIT_IDMAP_FDT_SIZE];
    let efdt = fdt.wrapping_add(MAX_FDT_SIZE);
    let mut ptep = ptes.as_mut_ptr() as phys_addr_t;
    map_range(&mut ptep, fdt, if _text as u64 > fdt { core::cmp::min(_text as u64, efdt) } else { efdt }, fdt, PAGE_KERNEL, IDMAP_ROOT_LEVEL, init_idmap_pg_dir as *mut pte_t, false, 0);
    dsb(ishst);
    fdt as *mut core::ffi::c_void
}

unsafe fn ng_mappings_allowed() -> bool {
    let cavium_erratum_27456_cpus = [
        MIDR_RANGE(MIDR_THUNDERX, 0, 0, 1, 1),
        MIDR_REV(MIDR_THUNDERX_81XX, 0, 0),
        midr_range { model: 0, rv_min: 0, rv_max: 0 },
    ];
    for r in cavium_erratum_27456_cpus.iter() {
        if r.model == 0 { break; }
        if midr_is_cpu_model_range(read_cpuid_id(), r.model, r.rv_min, r.rv_max) { return false; }
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn early_map_kernel(boot_status: u64, fdt: phys_addr_t) {
    static chosen_str: &[u8] = b"/chosen\0";
    let pa_base = _text as u64;
    let mut kaslr_offset = pa_base % MIN_KIMG_ALIGN;
    let mut root_level = 4 - CONFIG_PGTABLE_LEVELS;
    let mut va_bits = VA_BITS;
    let fdt_mapped = map_fdt(fdt);
    memset(__bss_start as *mut core::ffi::c_void, 0, (init_pg_end as usize) - (__bss_start as usize));
    let chosen = fdt_path_offset(fdt_mapped, chosen_str.as_ptr());
    init_feature_override(boot_status, fdt_mapped, chosen);
    if IS_ENABLED(CONFIG_ARM64_64K_PAGES) && !cpu_has_lva() { va_bits = VA_BITS_MIN; }
    else if IS_ENABLED(CONFIG_ARM64_LPA2) && !cpu_has_lpa2() { va_bits = VA_BITS_MIN; root_level += 1; }
    if va_bits > VA_BITS_MIN { sysreg_clear_set(tcr_el1, TCR_EL1_T1SZ_MASK, TCR_T1SZ(va_bits)); }
    if IS_ENABLED(CONFIG_RANDOMIZE_BASE) {
        let kaslr_seed = kaslr_early_init(fdt_mapped, chosen);
        if kaslr_seed != 0 && kaslr_requires_kpti() { arm64_use_ng_mappings = ng_mappings_allowed(); }
        kaslr_offset |= kaslr_seed & !(MIN_KIMG_ALIGN - 1);
    }
    if IS_ENABLED(CONFIG_ARM64_LPA2) && va_bits > VA_BITS_MIN { remap_idmap_for_lpa2(); }
    map_kernel(KIMAGE_VADDR + kaslr_offset, KIMAGE_VADDR + kaslr_offset - pa_base, root_level);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
