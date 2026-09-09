// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2016, Rashmica Gupta, IBM Corp.
 *
 * This traverses the kernel pagetables and dumps the
 * information about the used sections of memory to
 * /sys/kernel/debug/kernel_pagetables.
 *
 * Derived from the arm64 implementation:
 * Copyright (c) 2014, The Linux Foundation, Laura Abbott.
 * (C) Copyright 2008 Intel Corporation, Arjan van de Ven.
 */

#[repr(C)]
struct PgState {
    ptdump: ptdump_state,
    seq: *mut seq_file,
    marker: *const addr_marker,
    start_address: c_ulong,
    start_pa: c_ulong,
    level: c_int,
    current_flags: u64,
    check_wx: bool,
    wx_pages: c_ulong,
}

#[repr(C)]
struct addr_marker {
    start_address: c_ulong,
    name: *const c_char,
}

static mut address_markers: [addr_marker; 1] = [addr_marker { start_address: 0, name: core::ptr::null() }];

static mut ptdump_range: [ptdump_range; 2] = [
    ptdump_range { start: TASK_SIZE_MAX, end: !0 as c_ulong },
    ptdump_range { start: 0, end: 0 },
];

unsafe fn pt_dump_seq_printf(m: *mut seq_file, fmt: *const c_char, args: ...) {
    if !m.is_null() {
        seq_printf(m, fmt, args);
    }
}

unsafe fn pt_dump_seq_putc(m: *mut seq_file, c: c_int) {
    if !m.is_null() {
        seq_putc(m, c);
    }
}

#[no_mangle]
pub unsafe extern "C" fn pt_dump_size(mut m: *mut seq_file, mut size: c_ulong) {
    static units: [c_char; 8] = *b" KMGTPE\0";
    let mut unit = units.as_ptr();
    while (size & 1023) == 0 && *unit.add(1) != 0 {
        size >>= 10;
        unit = unit.add(1);
    }
    pt_dump_seq_printf(m, b"%9lu%c \0".as_ptr() as *const c_char, size, *unit);
}

unsafe fn dump_flag_info(st: *mut PgState, mut flag: *const flag_info, pte: u64, num: c_int) {
    for _ in 0..num {
        let mut s: *const c_char = core::ptr::null();
        let mut val: u64;
        if (*flag).mask == 0 { flag = flag.add(1); continue; }
        if (*flag).is_val {
            val = pte & (*flag).val;
            if (*flag).shift != 0 { val >>= (*flag).shift; }
            pt_dump_seq_printf((*st).seq, b"  %s:%llx\0".as_ptr() as *const c_char, (*flag).set, val);
        } else {
            if (pte & (*flag).mask) == (*flag).val { s = (*flag).set; } else { s = (*flag).clear; }
            if !s.is_null() { pt_dump_seq_printf((*st).seq, b"  %s\0".as_ptr() as *const c_char, s); }
        }
        (*st).current_flags &= !(*flag).mask;
        flag = flag.add(1);
    }
    if (*st).current_flags != 0 {
        pt_dump_seq_printf((*st).seq, b"  unknown flags:%llx\0".as_ptr() as *const c_char, (*st).current_flags);
    }
}

unsafe fn dump_addr(st: *mut PgState, addr: c_ulong) {
    pt_dump_seq_printf((*st).seq, b"0x%016lx-0x%016lx \0".as_ptr() as *const c_char, (*st).start_address, addr - 1);
    pt_dump_seq_printf((*st).seq, b" 0x%016lx \0".as_ptr() as *const c_char, (*st).start_pa);
    pt_dump_size((*st).seq, addr - (*st).start_address);
    pt_dump_seq_printf((*st).seq, b"%s \0".as_ptr() as *const c_char, (*pg_level.add((*st).level as usize)).name);
}

unsafe fn note_prot_wx(st: *mut PgState, addr: c_ulong) {
    let pte = __pte((*st).current_flags);
    if !(*st).check_wx || !pte_write(pte) || !pte_exec(pte) { return; }
    WARN_ONCE(IS_ENABLED(CONFIG_DEBUG_WX), b"powerpc/mm: Found insecure W+X mapping at address %p/%pS\n\0".as_ptr() as *const c_char, (*st).start_address, (*st).start_address);
    (*st).wx_pages += (addr - (*st).start_address) / PAGE_SIZE;
}

unsafe fn note_page_update_state(st: *mut PgState, addr: c_ulong, level: c_int, val: u64) {
    let flag = if level >= 0 { val & (*pg_level.add(level as usize)).mask } else { 0 };
    (*st).level = level; (*st).current_flags = flag; (*st).start_address = addr; (*st).start_pa = val & PTE_RPN_MASK;
    while addr >= (*(*st).marker.add(1)).start_address {
        (*st).marker = (*st).marker.add(1);
        pt_dump_seq_printf((*st).seq, b"---[ %s ]---\n\0".as_ptr() as *const c_char, (*(*st).marker).name);
    }
}

unsafe fn note_page(pt_st: *mut ptdump_state, addr: c_ulong, level: c_int, val: u64) {
    let flag = if level >= 0 { val & (*pg_level.add(level as usize)).mask } else { 0 };
    let st = container_of!(pt_st, PgState, ptdump);
    if (*st).level == -1 {
        pt_dump_seq_printf((*st).seq, b"---[ %s ]---\n\0".as_ptr() as *const c_char, (*(*st).marker).name);
        note_page_update_state(st, addr, level, val);
    } else if flag != (*st).current_flags || level != (*st).level || addr >= (*(*st).marker.add(1)).start_address {
        if (*st).current_flags != 0 {
            note_prot_wx(st, addr); dump_addr(st, addr);
            if !(*pg_level.add((*st).level as usize)).flag.is_null() {
                dump_flag_info(st, (*pg_level.add((*st).level as usize)).flag, (*st).current_flags, (*pg_level.add((*st).level as usize)).num);
            }
            pt_dump_seq_putc((*st).seq, b'\n' as c_int);
        }
        note_page_update_state(st, addr, level, val);
    }
}

unsafe fn note_page_pte(s: *mut ptdump_state, a: c_ulong, p: pte_t) { note_page(s, a, 4, pte_val(p)); }
unsafe fn note_page_pmd(s: *mut ptdump_state, a: c_ulong, p: pmd_t) { note_page(s, a, 3, pmd_val(p)); }
unsafe fn note_page_pud(s: *mut ptdump_state, a: c_ulong, p: pud_t) { note_page(s, a, 2, pud_val(p)); }
unsafe fn note_page_p4d(s: *mut ptdump_state, a: c_ulong, p: p4d_t) { note_page(s, a, 1, p4d_val(p)); }
unsafe fn note_page_pgd(s: *mut ptdump_state, a: c_ulong, p: pgd_t) { note_page(s, a, 0, pgd_val(p)); }
unsafe fn note_page_flush(s: *mut ptdump_state) { note_page(s, 0, -1, pte_val(__pte(0))); }

// The remaining initialization, marker population, page-table walking, and kernel
// registration retain their C interfaces and configuration-dependent symbols.
unsafe extern "C" {
    fn ptdump_walk_pgd(state: *mut ptdump_state, mm: *mut mm_struct, v: *mut core::ffi::c_void);
    static mut pg_level: flag_info;
    static mut init_mm: mm_struct;
}

unsafe fn populate_markers() {
    let mut i = 0usize;
    address_markers[i].start_address = PAGE_OFFSET; i += 1;
    address_markers[i].start_address = VMALLOC_START; i += 1;
    address_markers[i].start_address = VMALLOC_END; i += 1;
    address_markers[i].start_address = IOREMAP_BASE; i += 1;
    address_markers[i].start_address = IOREMAP_END; i += 1;
}

unsafe fn build_pgtable_complete_mask() {
    let mut i = 0usize;
    while i < ARRAY_SIZE(pg_level) {
        if !pg_level[i].flag.is_null() {
            let mut j = 0;
            while j < pg_level[i].num { pg_level[i].mask |= (*pg_level[i].flag.add(j as usize)).mask; j += 1; }
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn ptdump_check_wx() -> bool {
    let mut st = PgState { ptdump: core::mem::zeroed(), seq: core::ptr::null_mut(), marker: core::ptr::null(), start_address: 0, start_pa: 0, level: -1, current_flags: 0, check_wx: true, wx_pages: 0 };
    st.ptdump.note_page_pte = Some(note_page_pte); st.ptdump.note_page_pmd = Some(note_page_pmd);
    st.ptdump.note_page_pud = Some(note_page_pud); st.ptdump.note_page_p4d = Some(note_page_p4d);
    st.ptdump.note_page_pgd = Some(note_page_pgd); st.ptdump.note_page_flush = Some(note_page_flush);
    st.ptdump.range = ptdump_range.as_mut_ptr();
    if IS_ENABLED(CONFIG_PPC_BOOK3S_64) && !mmu_has_feature(MMU_FTR_KERNEL_RO) { return true; }
    ptdump_walk_pgd(&mut st.ptdump, &mut init_mm, core::ptr::null_mut());
    if st.wx_pages != 0 { pr_warn(b"Checked W+X mappings: FAILED, %lu W+X pages found\n\0".as_ptr() as *const c_char, st.wx_pages); false }
    else { pr_info(b"Checked W+X mappings: passed, no W+X pages found\n\0".as_ptr() as *const c_char); true }
}

unsafe fn ptdump_init() -> c_int {
    ptdump_range[0].start = PAGE_OFFSET;
    ptdump_range[0].end = PAGE_OFFSET + PGDIR_SIZE * PTRS_PER_PGD;
    populate_markers(); build_pgtable_complete_mask();
    if IS_ENABLED(CONFIG_PTDUMP_DEBUGFS) { debugfs_create_file(b"kernel_page_tables\0".as_ptr() as *const c_char, 0o400, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null()); }
    0
}

// device_initcall(ptdump_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
