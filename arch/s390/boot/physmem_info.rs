// SPDX-License-Identifier: GPL-2.0
// C dependencies and build-time annotations are supplied by the surrounding kernel.

const ENTRIES_EXTENDED_MAX: usize = 256 * (1020 / 2) * core::mem::size_of::<physmem_range>();

#[no_mangle]
pub static mut physmem_info: physmem_info = physmem_info::default();
static mut physmem_alloc_ranges: u32 = 0;
static mut physmem_alloc_pos: usize = 0;

unsafe fn __get_physmem_range_ptr(n: u32) -> *mut physmem_range {
    if n < MEM_INLINED_ENTRIES {
        return &mut physmem_info.online[n as usize] as *mut physmem_range;
    }
    if physmem_info.online_extended.is_null() {
        physmem_info.online_extended = physmem_alloc_range(
            RR_MEM_DETECT_EXT, ENTRIES_EXTENDED_MAX as usize, core::mem::size_of::<usize>(), 0,
            physmem_alloc_pos, true,
        ) as *mut physmem_range;
    }
    physmem_info.online_extended.add((n - MEM_INLINED_ENTRIES) as usize)
}

pub unsafe fn add_physmem_online_range(start: u64, end: u64) {
    let mut range: *mut physmem_range;
    if physmem_info.range_count != 0 {
        range = __get_physmem_range_ptr(physmem_info.range_count - 1);
        if (*range).end == start {
            (*range).end = end;
            return;
        }
    }
    range = __get_physmem_range_ptr(physmem_info.range_count);
    (*range).start = start;
    (*range).end = end;
    physmem_info.range_count += 1;
}

unsafe fn __diag260(rx1: usize, rx2: usize) -> i32 {
    let mut ry: usize = 0x10;
    let mut exception = 1;
    let mut cc: i32 = 0;
    // Inline s390 DIAG 0x260, exception table, and condition-code transformation.
    core::arch::asm!("diag {0}, {1}, 0x260", in("r2") rx1, in("r4") rx2,
                     inout("r1") ry, lateout("r0") cc, options(nostack));
    exception = 0;
    cc = if exception != 0 { -1 } else { cc };
    if cc == 0 { ry as i32 } else { -1 }
}

unsafe fn diag260() -> i32 {
    let mut storage_extents = [StorageExtent { start: 0, end: 0 }; 8];
    let rc = __diag260(storage_extents.as_mut_ptr() as usize, core::mem::size_of_val(&storage_extents));
    if rc == -1 { return -1; }
    for i in 0..core::cmp::min(rc as usize, storage_extents.len()) {
        add_physmem_online_range(storage_extents[i].start as u64, storage_extents[i].end as u64 + 1);
    }
    0
}

const DIAG500_SC_STOR_LIMIT: usize = 4;

unsafe fn diag500_storage_limit(max_physmem_end: *mut usize) -> i32 {
    let storage_limit: usize;
    // Inline s390 DIAG 0x500 storage-limit query and exception-table handling.
    core::arch::asm!("lghi %r1, 4\n lghi %r2, 0\n diag %r2, %r4, 0x500\n lgr {0}, %r2",
                     out(reg) storage_limit, options(nostack));
    if storage_limit == 0 { return -EINVAL; }
    *max_physmem_end = storage_limit + 1;
    0
}

unsafe fn tprot(addr: usize) -> i32 {
    let mut exception = 1;
    let mut cc: i32 = 0;
    // Inline s390 TPROT instruction and exception-table handling.
    core::arch::asm!("tprot 0({0}), 0", in(reg) addr, lateout("r0") cc, options(nostack));
    exception = 0;
    if exception != 0 { -EFAULT } else { cc }
}

unsafe fn search_mem_end() -> usize {
    let mut range: usize = 1 << (MAX_PHYSMEM_BITS - 20);
    let mut offset = 0usize;
    while range > 1 {
        range >>= 1;
        let pivot = offset + range;
        if tprot(pivot << 20) == 0 { offset = pivot; }
    }
    (offset + 1) << 20
}

pub unsafe fn detect_max_physmem_end() -> usize {
    let mut max_physmem_end = 0usize;
    if diag500_storage_limit(&mut max_physmem_end) == 0 {
        physmem_info.info_source = MEM_DETECT_DIAG500_STOR_LIMIT;
    } else if sclp_early_get_memsize(&mut max_physmem_end) == 0 {
        physmem_info.info_source = MEM_DETECT_SCLP_READ_INFO;
    } else {
        max_physmem_end = search_mem_end();
        physmem_info.info_source = MEM_DETECT_BIN_SEARCH;
    }
    boot_debug!("Max physical memory: 0x%016lx (info source: %s)\n", max_physmem_end, get_physmem_info_source());
    max_physmem_end
}

pub unsafe fn detect_physmem_online_ranges(max_physmem_end: usize) {
    let mut start: usize;
    let mut end: usize;
    let mut i: i32;
    if sclp_early_read_storage_info() == 0 {
        physmem_info.info_source = MEM_DETECT_SCLP_STOR_INFO;
    } else if physmem_info.info_source == MEM_DETECT_DIAG500_STOR_LIMIT {
        let mut online_end = 0usize;
        if sclp_early_get_memsize(&mut online_end) == 0 {
            physmem_info.info_source = MEM_DETECT_SCLP_READ_INFO;
            add_physmem_online_range(0, online_end as u64);
        }
    } else if diag260() == 0 {
        physmem_info.info_source = MEM_DETECT_DIAG260;
    } else if max_physmem_end != 0 { add_physmem_online_range(0, max_physmem_end as u64); }
    boot_debug!("Online memory ranges (info source: %s):\n", get_physmem_info_source());
    for_each_physmem_online_range!(i, &mut start, &mut end) {
        boot_debug!(" online [%d]:   0x%016lx-0x%016lx\n", i, start, end);
    }
}

pub unsafe fn physmem_set_usable_limit(limit: usize) {
    physmem_info.usable = limit;
    physmem_alloc_pos = limit;
    boot_debug!("Usable memory limit: 0x%016lx\n", limit);
}

unsafe fn die_oom(size: usize, align: usize, min: usize, max: usize) {
    let mut start; let mut end; let mut total_mem = 0usize; let mut total_reserved_mem = 0usize;
    boot_emerg!("Linux version %s\n", kernel_version);
    if !is_prot_virt_guest() && early_command_line[0] != 0 { boot_emerg!("Kernel command line: %s\n", early_command_line); }
    boot_emerg!("Out of memory allocating %lu bytes 0x%lx aligned in range %lx:%lx\n", size, align, min, max);
    boot_emerg!("Reserved memory ranges:\n");
    for_each_physmem_reserved_range!(t, range, &mut start, &mut end) {
        boot_emerg!("%016lx %016lx %s\n", start, end, get_rr_type_name(t)); total_reserved_mem += end - start;
    }
    boot_emerg!("Usable online memory ranges (info source: %s [%d]):\n", get_physmem_info_source(), physmem_info.info_source);
    for_each_physmem_usable_range!(i, &mut start, &mut end) { boot_emerg!("%016lx %016lx\n", start, end); total_mem += end - start; }
    boot_emerg!("Usable online memory total: %lu Reserved: %lu Free: %lu\n", total_mem, total_reserved_mem, if total_mem > total_reserved_mem { total_mem - total_reserved_mem } else { 0 });
    boot_panic!("Oom\n");
}

unsafe fn _physmem_reserve(typ: reserved_range_type, addr: usize, size: usize) { physmem_info.reserved[typ as usize].start = addr; physmem_info.reserved[typ as usize].end = addr + size; }
pub unsafe fn physmem_reserve(typ: reserved_range_type, addr: usize, size: usize) { _physmem_reserve(typ, addr, size); boot_debug!("%-14s 0x%016lx-0x%016lx %s\n", "Reserve:", addr, addr + size, get_rr_type_name(typ)); }
pub unsafe fn physmem_free(typ: reserved_range_type) { let r = &mut physmem_info.reserved[typ as usize]; boot_debug!("%-14s 0x%016lx-0x%016lx %s\n", "Free:", r.start, r.end, get_rr_type_name(typ)); r.start = 0; r.end = 0; }

unsafe fn __physmem_alloc_intersects(addr: usize, size: usize, intersection_start: *mut usize) -> bool {
    for t in 0..RR_MAX { let (mut a, mut s) = (0usize, 0usize); if get_physmem_reserved(t, &mut a, &mut s) && intersects(addr, size, a, s) { *intersection_start = a; return true; } }
    ipl_report_certs_intersects(addr, size, intersection_start)
}

unsafe fn __physmem_alloc_range(size: usize, mut align: usize, min: usize, max: usize, from_ranges: u32, ranges_left: *mut u32, die_on_oom: bool) -> usize {
    let mut nranges = if from_ranges != 0 { from_ranges } else { physmem_info.range_count };
    let (mut range_start, mut range_end, mut intersection_start); let mut pos = max;
    align = core::cmp::max(align, 8);
    while nranges != 0 { __get_physmem_range(nranges - 1, &mut range_start, &mut range_end, false); pos = core::cmp::min(range_end, pos); if round_up(min, align) + size > pos { break; } let addr = round_down(pos - size, align); if range_start > addr { nranges -= 1; continue; } if __physmem_alloc_intersects(addr, size, &mut intersection_start) { pos = intersection_start; continue; } if !ranges_left.is_null() { *ranges_left = nranges; } return addr; }
    if die_on_oom { die_oom(size, align, min, max); } 0
}

pub unsafe fn physmem_alloc_range(typ: reserved_range_type, size: usize, align: usize, min: usize, max: usize, die_on_oom: bool) -> usize { let max = core::cmp::min(max, physmem_alloc_pos); let addr = __physmem_alloc_range(size, align, min, max, 0, core::ptr::null_mut(), die_on_oom); if addr != 0 { _physmem_reserve(typ, addr, size); } boot_debug!("%-14s 0x%016lx-0x%016lx %s\n", "Alloc range:", addr, addr + size, get_rr_type_name(typ)); addr }

pub unsafe fn physmem_alloc(typ: reserved_range_type, size: usize, align: usize, die_on_oom: bool) -> usize { let range = &mut physmem_info.reserved[typ as usize]; let mut new_range: *mut reserved_range = core::ptr::null_mut(); let mut ranges_left = 0; let mut addr = __physmem_alloc_range(size, align, 0, physmem_alloc_pos, physmem_alloc_ranges, &mut ranges_left, die_on_oom); if addr == 0 { return 0; } if range.start != addr + size { if range.end != 0 { addr = __physmem_alloc_range(core::mem::size_of::<reserved_range>(), 0, 0, physmem_alloc_pos, physmem_alloc_ranges, &mut ranges_left, true); new_range = addr as *mut reserved_range; addr = __physmem_alloc_range(size, align, 0, addr, ranges_left, &mut ranges_left, die_on_oom); if addr == 0 { return 0; } *new_range = *range; range.chain = new_range; } range.end = addr + size; } if typ != RR_VMEM { boot_debug!("%-14s 0x%016lx-0x%016lx %-20s align 0x%lx split %d\n", "Alloc topdown:", addr, addr + size, get_rr_type_name(typ), align, !new_range.is_null()); } range.start = addr; physmem_alloc_pos = addr; physmem_alloc_ranges = ranges_left; addr }
pub unsafe fn physmem_alloc_or_die(typ: reserved_range_type, size: usize, align: usize) -> usize { physmem_alloc(typ, size, align, true) }
pub unsafe fn get_physmem_alloc_pos() -> usize { physmem_alloc_pos }
pub unsafe fn dump_physmem_reserved() { boot_debug!("Reserved memory ranges:\n"); for_each_physmem_reserved_range!(t, range, &mut start, &mut end) { if end != 0 { boot_debug!("%-14s 0x%016lx-0x%016lx @%012lx chain %012lx\n", get_rr_type_name(t), start, end, range as usize, (*range).chain as usize); } } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
