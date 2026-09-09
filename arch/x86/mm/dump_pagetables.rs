// SPDX-License-Identifier: GPL-2.0-only
/* Debug helper to dump the current kernel pagetables of the system. */

// C dependencies are supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct pg_state {
    pub ptdump: ptdump_state,
    pub level: c_int,
    pub current_prot: pgprotval_t,
    pub effective_prot: pgprotval_t,
    pub prot_levels: [pgprotval_t; 5],
    pub start_address: c_ulong,
    pub marker: *const addr_marker,
    pub lines: c_ulong,
    pub to_dmesg: bool,
    pub check_wx: bool,
    pub wx_pages: c_ulong,
    pub seq: *mut seq_file,
}

#[repr(C)]
pub struct addr_marker {
    pub start_address: c_ulong,
    pub name: *const c_char,
    pub max_lines: c_ulong,
}

// Address marker enum and table are configuration-dependent in the C source.
// The following declarations preserve their native kernel names and layout.
#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub enum address_markers_idx {
    USER_SPACE_NR = 0,
    KERNEL_SPACE_NR,
    LOW_KERNEL_NR,
    VMALLOC_START_NR,
    VMEMMAP_START_NR,
    CPU_ENTRY_AREA_NR,
    HIGH_KERNEL_NR,
    MODULES_VADDR_NR,
    MODULES_END_NR,
    FIXADDR_START_NR,
    END_OF_SPACE_NR,
}

#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub static mut address_markers: [addr_marker; END_OF_SPACE_NR as usize + 1] = [
    addr_marker { start_address: 0, name: b"User Space\0".as_ptr() as *const c_char, max_lines: 0 },
    addr_marker { start_address: 1u64 << 63, name: b"Kernel Space\0".as_ptr() as *const c_char, max_lines: 0 },
    addr_marker { start_address: 0, name: b"Low Kernel Mapping\0".as_ptr() as *const c_char, max_lines: 0 },
    addr_marker { start_address: 0, name: b"vmalloc() Area\0".as_ptr() as *const c_char, max_lines: 0 },
    addr_marker { start_address: 0, name: b"Vmemmap\0".as_ptr() as *const c_char, max_lines: 0 },
    addr_marker { start_address: 0, name: b"CPU entry Area\0".as_ptr() as *const c_char, max_lines: 0 },
    addr_marker { start_address: __START_KERNEL_map, name: b"High Kernel Mapping\0".as_ptr() as *const c_char, max_lines: 0 },
    addr_marker { start_address: MODULES_VADDR, name: b"Modules\0".as_ptr() as *const c_char, max_lines: 0 },
    addr_marker { start_address: MODULES_END, name: b"End Modules\0".as_ptr() as *const c_char, max_lines: 0 },
    addr_marker { start_address: FIXADDR_START, name: b"Fixmap Area\0".as_ptr() as *const c_char, max_lines: 0 },
    addr_marker { start_address: c_ulong::MAX, name: core::ptr::null(), max_lines: 0 },
];

pub const PTE_LEVEL_MULT: c_ulong = PAGE_SIZE;
pub const PMD_LEVEL_MULT: c_ulong = PTRS_PER_PTE * PTE_LEVEL_MULT;
pub const PUD_LEVEL_MULT: c_ulong = PTRS_PER_PMD * PMD_LEVEL_MULT;
pub const P4D_LEVEL_MULT: c_ulong = PTRS_PER_PUD * PUD_LEVEL_MULT;
pub const PGD_LEVEL_MULT: c_ulong = PTRS_PER_P4D * P4D_LEVEL_MULT;

unsafe fn printk_prot(m: *mut seq_file, pr: pgprotval_t, level: c_int, dmsg: bool) {
    static LEVEL_NAME: [&[u8]; 5] = [*b"pgd\0", *b"p4d\0", *b"pud\0", *b"pmd\0", *b"pte\0"];
    if pr & _PAGE_PRESENT == 0 {
        pt_dump_cont_printf!(m, dmsg, "                              ");
    } else {
        pt_dump_cont_printf!(m, dmsg, if pr & _PAGE_USER != 0 { "USR " } else { "    " });
        pt_dump_cont_printf!(m, dmsg, if pr & _PAGE_RW != 0 { "RW " } else { "ro " });
        pt_dump_cont_printf!(m, dmsg, if pr & _PAGE_PWT != 0 { "PWT " } else { "    " });
        pt_dump_cont_printf!(m, dmsg, if pr & _PAGE_PCD != 0 { "PCD " } else { "    " });
        pt_dump_cont_printf!(m, dmsg, if level <= 3 && pr & _PAGE_PSE != 0 { "PSE " } else { "    " });
        pt_dump_cont_printf!(m, dmsg, if (level == 4 && pr & _PAGE_PAT != 0) || ((level == 3 || level == 2) && pr & _PAGE_PAT_LARGE != 0) { "PAT " } else { "    " });
        pt_dump_cont_printf!(m, dmsg, if pr & _PAGE_GLOBAL != 0 { "GLB " } else { "    " });
        pt_dump_cont_printf!(m, dmsg, if pr & _PAGE_NX != 0 { "NX " } else { "x  " });
    }
    pt_dump_cont_printf!(m, dmsg, "%s\n", LEVEL_NAME[level as usize].as_ptr());
}

unsafe fn note_wx(st: *mut pg_state, addr: c_ulong) {
    let npages = (addr - (*st).start_address) / PAGE_SIZE;
    (*st).wx_pages += npages;
    WARN_ONCE!(__supported_pte_mask & _PAGE_NX != 0, "x86/mm: Found insecure W+X mapping at address %pS\n", (*st).start_address as *const core::ffi::c_void);
}

unsafe fn effective_prot(pt_st: *mut ptdump_state, level: c_int, val: u64) {
    let st = container_of!(pt_st, pg_state, ptdump);
    let prot = val & PTE_FLAGS_MASK;
    let effective = if level > 0 {
        let higher = (*st).prot_levels[(level - 1) as usize];
        (higher & prot & (_PAGE_USER | _PAGE_RW)) | ((higher | prot) & _PAGE_NX)
    } else { prot };
    (*st).prot_levels[level as usize] = effective;
}

unsafe fn effective_prot_pte(st: *mut ptdump_state, pte: pte_t) { effective_prot(st, 4, pte_val(pte)); }
unsafe fn effective_prot_pmd(st: *mut ptdump_state, pmd: pmd_t) { effective_prot(st, 3, pmd_val(pmd)); }
unsafe fn effective_prot_pud(st: *mut ptdump_state, pud: pud_t) { effective_prot(st, 2, pud_val(pud)); }
unsafe fn effective_prot_p4d(st: *mut ptdump_state, p4d: p4d_t) { effective_prot(st, 1, p4d_val(p4d)); }
unsafe fn effective_prot_pgd(st: *mut ptdump_state, pgd: pgd_t) { effective_prot(st, 0, pgd_val(pgd)); }

unsafe fn note_page(pt_st: *mut ptdump_state, addr: c_ulong, level: c_int, val: u64) {
    let st = container_of!(pt_st, pg_state, ptdump);
    let new_prot = val & PTE_FLAGS_MASK;
    let new_eff = if val == 0 { 0 } else { (*st).prot_levels[level as usize] };
    if (*st).level == -1 {
        (*st).current_prot = new_prot; (*st).effective_prot = new_eff; (*st).level = level;
        (*st).marker = address_markers.as_ptr(); (*st).lines = 0;
        pt_dump_seq_printf!((*st).seq, (*st).to_dmesg, "---[ %s ]---\n", (*(*st).marker).name);
    } else if new_prot != (*st).current_prot || new_eff != (*st).effective_prot || level != (*st).level || addr >= (*st).marker.add(1).as_ref().unwrap().start_address {
        if (*st).check_wx && ((*st).effective_prot & _PAGE_RW != 0) && ((*st).effective_prot & _PAGE_NX == 0) { note_wx(st, addr); }
        (*st).start_address = addr; (*st).current_prot = new_prot; (*st).effective_prot = new_eff; (*st).level = level;
    }
}

unsafe fn note_page_pte(s: *mut ptdump_state, a: c_ulong, x: pte_t) { note_page(s, a, 4, pte_val(x)); }
unsafe fn note_page_pmd(s: *mut ptdump_state, a: c_ulong, x: pmd_t) { note_page(s, a, 3, pmd_val(x)); }
unsafe fn note_page_pud(s: *mut ptdump_state, a: c_ulong, x: pud_t) { note_page(s, a, 2, pud_val(x)); }
unsafe fn note_page_p4d(s: *mut ptdump_state, a: c_ulong, x: p4d_t) { note_page(s, a, 1, p4d_val(x)); }
unsafe fn note_page_pgd(s: *mut ptdump_state, a: c_ulong, x: pgd_t) { note_page(s, a, 0, pgd_val(x)); }
unsafe fn note_page_flush(s: *mut ptdump_state) { note_page(s, 0, -1, 0); }

pub unsafe fn ptdump_walk_pgd_level_core(m: *mut seq_file, mm: *mut mm_struct, pgd: *mut pgd_t, checkwx: bool, dmesg: bool) -> bool {
    let mut st = pg_state { ptdump: core::mem::zeroed(), level: -1, current_prot: 0, effective_prot: 0, prot_levels: [0; 5], start_address: 0, marker: core::ptr::null(), lines: 0, to_dmesg: dmesg, check_wx: checkwx, wx_pages: 0, seq: m };
    st.ptdump.note_page_pte = Some(note_page_pte); st.ptdump.note_page_pmd = Some(note_page_pmd); st.ptdump.note_page_pud = Some(note_page_pud); st.ptdump.note_page_p4d = Some(note_page_p4d); st.ptdump.note_page_pgd = Some(note_page_pgd); st.ptdump.note_page_flush = Some(note_page_flush);
    st.ptdump.effective_prot_pte = Some(effective_prot_pte); st.ptdump.effective_prot_pmd = Some(effective_prot_pmd); st.ptdump.effective_prot_pud = Some(effective_prot_pud); st.ptdump.effective_prot_p4d = Some(effective_prot_p4d); st.ptdump.effective_prot_pgd = Some(effective_prot_pgd);
    ptdump_walk_pgd(&mut st.ptdump, mm, pgd);
    if !checkwx { return true; }
    if st.wx_pages != 0 { pr_info!("x86/mm: Checked W+X mappings: FAILED, %lu W+X pages found.\n", st.wx_pages); false } else { pr_info!("x86/mm: Checked W+X mappings: passed, no W+X pages found.\n"); true }
}

pub unsafe fn ptdump_walk_pgd_level(m: *mut seq_file, mm: *mut mm_struct) { ptdump_walk_pgd_level_core(m, mm, (*mm).pgd, false, true); }
pub unsafe fn ptdump_walk_pgd_level_debugfs(m: *mut seq_file, mm: *mut mm_struct, user: bool) { ptdump_walk_pgd_level_core(m, mm, (*mm).pgd, false, false); }
pub unsafe fn ptdump_walk_user_pgd_level_checkwx() { }
pub unsafe fn ptdump_walk_pgd_level_checkwx() -> bool { if __supported_pte_mask & _PAGE_NX == 0 { true } else { ptdump_walk_pgd_level_core(core::ptr::null_mut(), &mut init_mm, INIT_PGD, true, false) } }

// __initcall(pt_dump_init) is retained as a kernel registration dependency.
pub unsafe fn pt_dump_init() -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
