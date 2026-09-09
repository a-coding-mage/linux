// SPDX-License-Identifier: GPL-2.0

// Translated from ptdump.c. Kernel types, globals, constants, and helpers are
// supplied by the surrounding kernel dependencies.

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
#[inline]
unsafe fn note_kasan_page_table(walk: *mut mm_walk, addr: c_ulong) -> c_int {
    let st = (*walk).private as *mut ptdump_state;

    ((*st).note_page_pte)(st, addr, kasan_early_shadow_pte[0]);

    (*walk).action = ACTION_CONTINUE;

    0
}

unsafe fn ptdump_pgd_entry(
    pgd: *mut pgd_t,
    addr: c_ulong,
    _next: c_ulong,
    walk: *mut mm_walk,
) -> c_int {
    let st = (*walk).private as *mut ptdump_state;
    let val: pgd_t = pgdp_get(pgd);

    // Preserved from: CONFIG_PGTABLE_LEVELS > 4 && KASAN_GENERIC/SW_TAGS.
    #[cfg(all(
        any(CONFIG_PGTABLE_LEVELS_5, CONFIG_PGTABLE_LEVELS_GREATER_THAN_4),
        any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)
    ))]
    {
        if pgd_page(val) == virt_to_page(lm_alias(kasan_early_shadow_p4d)) {
            return note_kasan_page_table(walk, addr);
        }
    }

    if let Some(effective_prot_pgd) = (*st).effective_prot_pgd {
        effective_prot_pgd(st, val);
    }

    if pgd_leaf(val) {
        ((*st).note_page_pgd)(st, addr, val);
        (*walk).action = ACTION_CONTINUE;
    }

    0
}

unsafe fn ptdump_p4d_entry(
    p4d: *mut p4d_t,
    addr: c_ulong,
    _next: c_ulong,
    walk: *mut mm_walk,
) -> c_int {
    let st = (*walk).private as *mut ptdump_state;
    let val: p4d_t = p4dp_get(p4d);

    // Preserved from: CONFIG_PGTABLE_LEVELS > 3 && KASAN_GENERIC/SW_TAGS.
    #[cfg(all(
        any(CONFIG_PGTABLE_LEVELS_4, CONFIG_PGTABLE_LEVELS_GREATER_THAN_3),
        any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)
    ))]
    {
        if p4d_page(val) == virt_to_page(lm_alias(kasan_early_shadow_pud)) {
            return note_kasan_page_table(walk, addr);
        }
    }

    if let Some(effective_prot_p4d) = (*st).effective_prot_p4d {
        effective_prot_p4d(st, val);
    }

    if p4d_leaf(val) {
        ((*st).note_page_p4d)(st, addr, val);
        (*walk).action = ACTION_CONTINUE;
    }

    0
}

unsafe fn ptdump_pud_entry(
    pud: *mut pud_t,
    addr: c_ulong,
    _next: c_ulong,
    walk: *mut mm_walk,
) -> c_int {
    let st = (*walk).private as *mut ptdump_state;
    let val: pud_t = pudp_get(pud);

    // Preserved from: CONFIG_PGTABLE_LEVELS > 2 && KASAN_GENERIC/SW_TAGS.
    #[cfg(all(
        any(CONFIG_PGTABLE_LEVELS_3, CONFIG_PGTABLE_LEVELS_GREATER_THAN_2),
        any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)
    ))]
    {
        if pud_page(val) == virt_to_page(lm_alias(kasan_early_shadow_pmd)) {
            return note_kasan_page_table(walk, addr);
        }
    }

    if let Some(effective_prot_pud) = (*st).effective_prot_pud {
        effective_prot_pud(st, val);
    }

    if pud_leaf(val) {
        ((*st).note_page_pud)(st, addr, val);
        (*walk).action = ACTION_CONTINUE;
    }

    0
}

unsafe fn ptdump_pmd_entry(
    pmd: *mut pmd_t,
    addr: c_ulong,
    _next: c_ulong,
    walk: *mut mm_walk,
) -> c_int {
    let st = (*walk).private as *mut ptdump_state;
    let val: pmd_t = pmdp_get(pmd);

    #[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
    {
        if pmd_page(val) == virt_to_page(lm_alias(kasan_early_shadow_pte)) {
            return note_kasan_page_table(walk, addr);
        }
    }

    if let Some(effective_prot_pmd) = (*st).effective_prot_pmd {
        effective_prot_pmd(st, val);
    }
    if pmd_leaf(val) {
        ((*st).note_page_pmd)(st, addr, val);
        (*walk).action = ACTION_CONTINUE;
    }

    0
}

unsafe fn ptdump_pte_entry(
    pte: *mut pte_t,
    addr: c_ulong,
    _next: c_ulong,
    walk: *mut mm_walk,
) -> c_int {
    let st = (*walk).private as *mut ptdump_state;
    let val: pte_t = ptep_get(pte);

    if let Some(effective_prot_pte) = (*st).effective_prot_pte {
        effective_prot_pte(st, val);
    }

    ((*st).note_page_pte)(st, addr, val);

    0
}

unsafe fn ptdump_hole(
    addr: c_ulong,
    _next: c_ulong,
    depth: c_int,
    walk: *mut mm_walk,
) -> c_int {
    let st = (*walk).private as *mut ptdump_state;
    let pte_zero: pte_t = core::mem::zeroed();
    let pmd_zero: pmd_t = core::mem::zeroed();
    let pud_zero: pud_t = core::mem::zeroed();
    let p4d_zero: p4d_t = core::mem::zeroed();
    let pgd_zero: pgd_t = core::mem::zeroed();

    match depth {
        4 => ((*st).note_page_pte)(st, addr, pte_zero),
        3 => ((*st).note_page_pmd)(st, addr, pmd_zero),
        2 => ((*st).note_page_pud)(st, addr, pud_zero),
        1 => ((*st).note_page_p4d)(st, addr, p4d_zero),
        0 => ((*st).note_page_pgd)(st, addr, pgd_zero),
        _ => {}
    }
    0
}

static ptdump_ops: mm_walk_ops = mm_walk_ops {
    pgd_entry: Some(ptdump_pgd_entry),
    p4d_entry: Some(ptdump_p4d_entry),
    pud_entry: Some(ptdump_pud_entry),
    pmd_entry: Some(ptdump_pmd_entry),
    pte_entry: Some(ptdump_pte_entry),
    pte_hole: Some(ptdump_hole),
};

unsafe fn ptdump_walk_pgd(st: *mut ptdump_state, mm: *mut mm_struct, pgd: *mut pgd_t) {
    let mut range = (*st).range;

    get_online_mems();
    mmap_write_lock(mm);
    // To stabilise kernel page tables we must hold the init_mm lock too.
    if mm != &raw mut init_mm {
        mmap_write_lock_nested(&raw mut init_mm, SINGLE_DEPTH_NESTING);
    }

    while (*range).start != (*range).end {
        walk_page_range_debug(
            mm,
            (*range).start,
            (*range).end,
            &raw const ptdump_ops,
            pgd,
            st,
        );
        range = range.add(1);
    }

    if mm != &raw mut init_mm {
        mmap_write_unlock(&raw mut init_mm);
    }
    mmap_write_unlock(mm);
    put_online_mems();

    // Flush out the last page
    ((*st).note_page_flush)(st);
}

unsafe fn check_wx_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> c_int {
    if ptdump_check_wx() != 0 {
        seq_puts(m, c"SUCCESS\n".as_ptr());
    } else {
        seq_puts(m, c"FAILED\n".as_ptr());
    }

    0
}

// DEFINE_SHOW_ATTRIBUTE(check_wx);
// The surrounding kernel build supplies the generated check_wx_fops object.

unsafe fn ptdump_debugfs_init() -> c_int {
    debugfs_create_file(
        c"check_wx_pages".as_ptr(),
        0o400,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        &raw const check_wx_fops,
    );

    0
}

// device_initcall(ptdump_debugfs_init);
// The kernel initcall registration is supplied by the surrounding build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
