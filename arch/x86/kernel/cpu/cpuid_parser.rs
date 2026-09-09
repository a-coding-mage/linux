// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * CPUID parser; for populating the system's CPUID tables.
 */

/* Dependencies supplied by the surrounding kernel translation unit. */

/* Clear a single CPUID table entry */
unsafe fn cpuid_clear(e: *const cpuid_parse_entry, out: *const cpuid_output) {
    let mut regs = (*out).regs;

    for _i in 0..(*e).maxcnt {
        core::ptr::write_bytes(regs, 0, 1);
        regs = regs.add(1);
    }

    core::ptr::write_bytes((*out).info, 0, 1);
}

/*
 * Leaf read functions:
 */

/*
 * Default CPUID read function
 * Satisfies the requirements stated at 'struct cpuid_parse_entry'->read().
 */
unsafe fn cpuid_read_generic(e: *const cpuid_parse_entry, out: *const cpuid_output) {
    let mut regs = (*out).regs;

    for i in 0..(*e).maxcnt {
        cpuid_read_subleaf((*e).leaf, (*e).subleaf + i, regs);
        regs = regs.add(1);
        (*(*out).info).nr_entries += 1;
    }
}

/*
 * CPUID parser table:
 */

static cpuid_parse_entries: &[cpuid_parse_entry] = &CPUID_PARSE_ENTRIES;

/*
 * Leaf-independent parser code:
 */

unsafe fn cpuid_range_max_leaf(t: *const cpuid_table, range: u32) -> u32 {
    let l0 = __cpuid_table_subleaf(t, 0x0, 0);

    match range {
        CPUID_BASE_START => if !l0.is_null() { (*l0).max_std_leaf } else { 0 },
        _ => 0,
    }
}

unsafe fn __cpuid_reset_table(
    t: *mut cpuid_table,
    entries: *const cpuid_parse_entry,
    nr_entries: usize,
    start: u32,
    end: u32,
    fill: bool,
) {
    let mut entry = entries;
    let range = CPUID_RANGE(start);

    for _i in 0..nr_entries {
        let out = cpuid_output {
            regs: cpuid_table_regs_p(t, (*entry).regs_offs),
            info: cpuid_table_info_p(t, (*entry).info_offs),
        };

        if (*entry).leaf < start || (*entry).leaf > end {
            entry = entry.add(1);
            continue;
        }

        cpuid_clear(entry, &out);

        /*
         * Read the range's anchor leaf unconditionally so that the cached
         * maximum valid leaf value is available for the remaining entries.
         */
        if fill && ((*entry).leaf == range || (*entry).leaf <= cpuid_range_max_leaf(t, range)) {
            ((*entry).read)(entry, &out);
        }

        entry = entry.add(1);
    }
}

/*
 * Zero all cached CPUID entries within [@start-@end] range.  This is needed when
 * certain operations like MSR writes induce changes to the CPU's CPUID layout.
 */
unsafe fn __cpuid_zero_table(
    t: *mut cpuid_table,
    entries: *const cpuid_parse_entry,
    nr_entries: usize,
    start: u32,
    end: u32,
) {
    __cpuid_reset_table(t, entries, nr_entries, start, end, false);
}

unsafe fn __cpuid_fill_table(
    t: *mut cpuid_table,
    entries: *const cpuid_parse_entry,
    nr_entries: usize,
    start: u32,
    end: u32,
) {
    __cpuid_reset_table(t, entries, nr_entries, start, end, true);
}

unsafe fn cpuid_fill_table(
    t: *mut cpuid_table,
    entries: *const cpuid_parse_entry,
    nr_entries: usize,
) {
    let ranges = [(CPUID_BASE_START, CPUID_BASE_END)];

    for &(start, end) in &ranges {
        __cpuid_fill_table(t, entries, nr_entries, start, end);
    }
}

unsafe fn __cpuid_scan_cpu_full(c: *mut cpuinfo_x86) {
    let nr_entries = cpuid_parse_entries.len();
    let table = &mut (*c).cpuid;

    cpuid_fill_table(table, cpuid_parse_entries.as_ptr(), nr_entries);
}

unsafe fn __cpuid_scan_cpu_partial(c: *mut cpuinfo_x86, start_leaf: u32, end_leaf: u32) {
    let nr_entries = cpuid_parse_entries.len();
    let table = &mut (*c).cpuid;

    __cpuid_zero_table(table, cpuid_parse_entries.as_ptr(), nr_entries, start_leaf, end_leaf);
    __cpuid_fill_table(table, cpuid_parse_entries.as_ptr(), nr_entries, start_leaf, end_leaf);
}

/*
 * Call-site APIs:
 */

/**
 * cpuid_scan_cpu() - Populate current CPU's CPUID table
 * @c:    CPU capability structure associated with the current CPU
 *
 * Populate the CPUID table embedded within @c with parsed CPUID data.  All CPUID
 * instructions are invoked locally, so this must be called on the CPU associated
 * with @c.
 */
pub unsafe fn cpuid_scan_cpu(c: *mut cpuinfo_x86) {
    __cpuid_scan_cpu_full(c);
}

/**
 * cpuid_refresh_range() - Rescan a CPUID table's leaf range
 * @c:    CPU capability structure associated with the current CPU
 * @start:    Start of leaf range to be re-scanned
 * @end:    End of leaf range
 */
pub unsafe fn cpuid_refresh_range(c: *mut cpuinfo_x86, start: u32, end: u32) {
    if WARN_ON_ONCE(start > end) {
        return;
    }

    if WARN_ON_ONCE(CPUID_RANGE(start) != CPUID_RANGE(end)) {
        return;
    }

    __cpuid_scan_cpu_partial(c, start, end);
}

/**
 * cpuid_refresh_leaf() - Rescan a CPUID table's leaf
 * @c:    CPU capability structure associated with the current CPU
 * @leaf:    Leaf to be re-scanned
 */
pub unsafe fn cpuid_refresh_leaf(c: *mut cpuinfo_x86, leaf: u32) {
    cpuid_refresh_range(c, leaf, leaf);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
