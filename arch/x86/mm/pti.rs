// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright(c) 2017 Intel Corporation. All rights reserved.
 *
 * This code is based in part on work published here:
 * https://github.com/IAIK/KAISER
 *
 * Major changes to the original code by: Dave Hansen <dave.hansen@intel.com>
 * Mostly rewritten by Thomas Gleixner <tglx@kernel.org> and Andy Lutomirsky <luto@amacapital.net>
 */

// C headers and kernel assembly headers are supplied by other translation units.

#[cfg(target_pointer_width = "64")]
const PTI_LEVEL_KERNEL_IMAGE: pti_clone_level = pti_clone_level::PTI_CLONE_PMD;
#[cfg(not(target_pointer_width = "64"))]
const PTI_LEVEL_KERNEL_IMAGE: pti_clone_level = pti_clone_level::PTI_CLONE_PTE;

const __GFP_NOTRACK: usize = 0;

unsafe fn pti_print_if_insecure(reason: *const core::ffi::c_char) {
    if boot_cpu_has_bug(X86_BUG_CPU_MELTDOWN) { pr_info("%s\n", reason); }
}

unsafe fn pti_print_if_secure(reason: *const core::ffi::c_char) {
    if !boot_cpu_has_bug(X86_BUG_CPU_MELTDOWN) { pr_info("%s\n", reason); }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum pti_mode_t { PTI_AUTO = 0, PTI_FORCE_OFF, PTI_FORCE_ON }
static mut pti_mode: pti_mode_t = pti_mode_t::PTI_AUTO;

pub unsafe fn pti_check_boottime_disable() {
    if hypervisor_is_type(X86_HYPER_XEN_PV) { pti_mode = pti_mode_t::PTI_FORCE_OFF; pti_print_if_insecure(c"disabled on XEN PV.".as_ptr()); return; }
    if pti_mode == pti_mode_t::PTI_AUTO && !cpu_attack_vector_mitigated(CPU_MITIGATE_USER_KERNEL) { pti_mode = pti_mode_t::PTI_FORCE_OFF; }
    if pti_mode == pti_mode_t::PTI_FORCE_OFF { pti_print_if_insecure(c"disabled on command line.".as_ptr()); return; }
    if pti_mode == pti_mode_t::PTI_FORCE_ON { pti_print_if_secure(c"force enabled on command line.".as_ptr()); }
    if pti_mode == pti_mode_t::PTI_AUTO && !boot_cpu_has_bug(X86_BUG_CPU_MELTDOWN) { return; }
    setup_force_cpu_cap(X86_FEATURE_PTI);
    if cpu_feature_enabled(X86_FEATURE_INVLPGB) { pr_debug("PTI enabled, disabling INVLPGB\n"); setup_clear_cpu_cap(X86_FEATURE_INVLPGB); }
    if cpu_feature_enabled(X86_FEATURE_FRED) { pr_debug("PTI enabled, disabling FRED\n"); setup_clear_cpu_cap(X86_FEATURE_FRED); }
}

unsafe fn pti_parse_cmdline(arg: *mut core::ffi::c_char) -> i32 {
    if strcmp(arg, c"off".as_ptr()) == 0 { pti_mode = pti_mode_t::PTI_FORCE_OFF; }
    else if strcmp(arg, c"on".as_ptr()) == 0 { pti_mode = pti_mode_t::PTI_FORCE_ON; }
    else if strcmp(arg, c"auto".as_ptr()) == 0 { pti_mode = pti_mode_t::PTI_AUTO; }
    else { return -EINVAL; }
    0
}
// early_param("pti", pti_parse_cmdline);

unsafe fn pti_parse_cmdline_nopti(_arg: *mut core::ffi::c_char) -> i32 { pti_mode = pti_mode_t::PTI_FORCE_OFF; 0 }
// early_param("nopti", pti_parse_cmdline_nopti);

pub unsafe fn __pti_set_user_pgtbl(pgdp: *mut pgd_t, mut pgd: pgd_t) -> pgd_t {
    if !pgdp_maps_userspace(pgdp) || (pgd.pgd & _PAGE_NOPTISHADOW) != 0 { return pgd; }
    kernel_to_user_pgdp(pgdp).write().pgd = pgd.pgd;
    if (pgd.pgd & (_PAGE_USER | _PAGE_PRESENT)) == (_PAGE_USER | _PAGE_PRESENT) && (__supported_pte_mask & _PAGE_NX) != 0 { pgd.pgd |= _PAGE_NX; }
    pgd
}

unsafe fn pti_user_pagetable_walk_p4d(address: usize) -> *mut p4d_t {
    let pgd = kernel_to_user_pgdp(pgd_offset_k(address));
    let gfp = GFP_KERNEL | __GFP_NOTRACK | __GFP_ZERO;
    if address < PAGE_OFFSET { WARN_ONCE(true, "attempt to walk user address\n"); return core::ptr::null_mut(); }
    if pgd.read().pgd == 0 {
        let page = __get_free_page(gfp); if WARN_ON_ONCE(page == 0) { return core::ptr::null_mut(); }
        set_pgd(pgd, __pgd(_KERNPG_TABLE | __pa(page)));
    }
    BUILD_BUG_ON(pgd_leaf(pgd.read())); p4d_offset(pgd, address)
}

unsafe fn pti_user_pagetable_walk_pmd(address: usize) -> *mut pmd_t {
    let gfp = GFP_KERNEL | __GFP_NOTRACK | __GFP_ZERO;
    let p4d = pti_user_pagetable_walk_p4d(address); if p4d.is_null() { return core::ptr::null_mut(); }
    BUILD_BUG_ON(p4d_leaf(p4d.read()));
    if p4d.read().p4d == 0 { let page = __get_free_page(gfp); if WARN_ON_ONCE(page == 0) { return core::ptr::null_mut(); } set_p4d(p4d, __p4d(_KERNPG_TABLE | __pa(page))); }
    let pud = pud_offset(p4d, address);
    if pud_leaf(pud.read()) { WARN_ON(true); return core::ptr::null_mut(); }
    if pud.read().pud == 0 { let page = __get_free_page(gfp); if WARN_ON_ONCE(page == 0) { return core::ptr::null_mut(); } set_pud(pud, __pud(_KERNPG_TABLE | __pa(page))); }
    pmd_offset(pud, address)
}

unsafe fn pti_user_pagetable_walk_pte(address: usize, late_text: bool) -> *mut pte_t {
    let gfp = GFP_KERNEL | __GFP_NOTRACK | __GFP_ZERO;
    let pmd = pti_user_pagetable_walk_pmd(address); if pmd.is_null() { return core::ptr::null_mut(); }
    if pmd_leaf(pmd.read()) { if late_text { set_pmd(pmd, __pmd(0)); } else { WARN_ON_ONCE(true); return core::ptr::null_mut(); } }
    if pmd.read().pmd == 0 { let page = __get_free_page(gfp); if page == 0 { return core::ptr::null_mut(); } set_pmd(pmd, __pmd(_KERNPG_TABLE | __pa(page))); }
    let pte = pte_offset_kernel(pmd, address);
    if (pte_flags(pte.read()) & _PAGE_USER) != 0 { WARN_ONCE(true, "attempt to walk to user pte\n"); return core::ptr::null_mut(); }
    pte
}

#[cfg(CONFIG_X86_VSYSCALL_EMULATION)]
unsafe fn pti_setup_vsyscall() { let mut level = 0; let pte = lookup_address(VSYSCALL_ADDR, &mut level); if pte.is_null() || WARN_ON(level != PG_LEVEL_4K) || pte_none(pte.read()) { return; } let target = pti_user_pagetable_walk_pte(VSYSCALL_ADDR, false); if WARN_ON(target.is_null()) { return; } target.write(pte.read()); set_vsyscall_pgtable_user_bits(kernel_to_user_pgdp(swapper_pg_dir)); }
#[cfg(not(CONFIG_X86_VSYSCALL_EMULATION))]
unsafe fn pti_setup_vsyscall() {}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum pti_clone_level { PTI_CLONE_PMD, PTI_CLONE_PTE }

unsafe fn pti_clone_pgtable(start: usize, end: usize, level: pti_clone_level, late_text: bool) {
    let mut addr = start;
    while addr < end {
        if addr < start { break; }
        let pgd = pgd_offset_k(addr); if WARN_ON(pgd_none(pgd.read())) { return; }
        let p4d = p4d_offset(pgd, addr); if WARN_ON(p4d_none(p4d.read())) { return; }
        let pud = pud_offset(p4d, addr);
        if pud_none(pud.read()) { WARN_ON_ONCE((addr & !PUD_MASK) != 0); addr = round_up(addr.wrapping_add(1), PUD_SIZE); continue; }
        let pmd = pmd_offset(pud, addr);
        if pmd_none(pmd.read()) { WARN_ON_ONCE((addr & !PMD_MASK) != 0); addr = round_up(addr.wrapping_add(1), PMD_SIZE); continue; }
        if pmd_leaf(pmd.read()) || level == pti_clone_level::PTI_CLONE_PMD {
            let target = pti_user_pagetable_walk_pmd(addr); if WARN_ON(target.is_null()) { return; }
            if WARN_ON((pmd_flags(pmd.read()) & _PAGE_PRESENT) == 0) { return; }
            if boot_cpu_has(X86_FEATURE_PGE) { pmd.write(pmd_set_flags(pmd.read(), _PAGE_GLOBAL)); }
            target.write(pmd.read()); addr = round_up(addr.wrapping_add(1), PMD_SIZE);
        } else if level == pti_clone_level::PTI_CLONE_PTE {
            let pte = pte_offset_kernel(pmd, addr);
            if pte_none(pte.read()) { addr = round_up(addr.wrapping_add(1), PAGE_SIZE); continue; }
            if WARN_ON((pte_flags(pte.read()) & _PAGE_PRESENT) == 0) { return; }
            let target = pti_user_pagetable_walk_pte(addr, late_text); if WARN_ON(target.is_null()) { return; }
            if boot_cpu_has(X86_FEATURE_PGE) { pte.write(pte_set_flags(pte.read(), _PAGE_GLOBAL)); }
            target.write(pte.read()); addr = round_up(addr.wrapping_add(1), PAGE_SIZE);
        } else { BUG!(); }
    }
}

#[cfg(target_pointer_width = "64")]
unsafe fn pti_clone_p4d(addr: usize) { let user = pti_user_pagetable_walk_p4d(addr); if user.is_null() { return; } let pgd = pgd_offset_k(addr); user.write(p4d_offset(pgd, addr).read()); }

#[cfg(target_pointer_width = "64")]
unsafe fn pti_clone_user_shared() {
    pti_clone_p4d(CPU_ENTRY_AREA_BASE);
    for_each_possible_cpu(|cpu| { let va = &per_cpu(cpu_tss_rw, cpu) as *const _ as usize; let pa = per_cpu_ptr_to_phys(va as *mut core::ffi::c_void); let target = pti_user_pagetable_walk_pte(va, false); if WARN_ON(target.is_null()) { return; } target.write(pfn_pte(pa >> PAGE_SHIFT, PAGE_KERNEL)); });
}
#[cfg(not(target_pointer_width = "64"))]
unsafe fn pti_clone_user_shared() { let start = CPU_ENTRY_AREA_BASE; pti_clone_pgtable(start, start + PAGE_SIZE * CPU_ENTRY_AREA_PAGES, pti_clone_level::PTI_CLONE_PMD, false); }

unsafe fn pti_setup_espfix64() {
    #[cfg(CONFIG_X86_ESPFIX64)] pti_clone_p4d(ESPFIX_BASE_ADDR);
}
unsafe fn pti_clone_entry_text(late: bool) { pti_clone_pgtable(__entry_text_start as usize, __entry_text_end as usize, PTI_LEVEL_KERNEL_IMAGE, late); }

unsafe fn pti_kernel_image_global_ok() -> bool {
    if cpu_feature_enabled(X86_FEATURE_PCID) || pti_mode != pti_mode_t::PTI_AUTO || boot_cpu_has(X86_FEATURE_K8) { return false; }
    if IS_ENABLED(CONFIG_RANDSTRUCT) { return false; }
    true
}

unsafe fn pti_clone_kernel_text() {
    let start = PFN_ALIGN(_text); let end_clone = __end_rodata_aligned as usize; let end_global = PFN_ALIGN(_etext as usize);
    if !pti_kernel_image_global_ok() { return; }
    pr_debug("mapping partial kernel image into user address space\n");
    pti_clone_pgtable(start, end_clone, PTI_LEVEL_KERNEL_IMAGE, false);
    set_memory_global(start, (end_global - start) >> PAGE_SHIFT);
}
unsafe fn pti_set_kernel_image_nonglobal() { let start = PFN_ALIGN(_text); let end = ALIGN(_end as usize, PMD_SIZE); set_memory_nonglobal(start, (end - start) >> PAGE_SHIFT); }

pub unsafe fn pti_init() {
    if !boot_cpu_has(X86_FEATURE_PTI) { return; }
    pr_info("enabled\n");
    pti_clone_user_shared(); pti_set_kernel_image_nonglobal(); pti_clone_entry_text(false); pti_setup_espfix64(); pti_setup_vsyscall();
}

pub unsafe fn pti_finalize() {
    if !boot_cpu_has(X86_FEATURE_PTI) { return; }
    pti_clone_entry_text(true); pti_clone_kernel_text(); debug_checkwx_user();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
