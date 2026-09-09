// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to the original Linux kernel includes are supplied externally.

static mut early_memtest_done: bool = false;
static mut early_memtest_bad_size: phys_addr_t = 0;

static mut patterns: [u64; 17] = [
    // The first entry has to be 0 to leave memtest with zeroed memory
    0,
    0xffffffffffffffffu64,
    0x5555555555555555u64,
    0xaaaaaaaaaaaaaaaau64,
    0x1111111111111111u64,
    0x2222222222222222u64,
    0x4444444444444444u64,
    0x8888888888888888u64,
    0x3333333333333333u64,
    0x6666666666666666u64,
    0x9999999999999999u64,
    0xccccccccccccccccu64,
    0x7777777777777777u64,
    0xbbbbbbbbbbbbbbbbu64,
    0xddddddddddddddddu64,
    0xeeeeeeeeeeeeeeeeu64,
    0x7a6c7258554e494cu64, // yeah ;-)
];

unsafe fn reserve_bad_mem(pattern: u64, start_bad: phys_addr_t, end_bad: phys_addr_t) {
    pr_info!("  %016llx bad mem addr %pa - %pa reserved\n", cpu_to_be64(pattern), &start_bad, &end_bad);
    memblock_reserve(start_bad, end_bad - start_bad);
    early_memtest_bad_size += end_bad - start_bad;
}

unsafe fn memtest(pattern: u64, start_phys: phys_addr_t, size: phys_addr_t) {
    let mut p: *mut u64;
    let start_phys_aligned = ALIGN(start_phys, core::mem::size_of::<u64>());
    let start = __va(start_phys_aligned) as *mut u64;
    let end = start.add(((size - (start_phys_aligned - start_phys)) /
        core::mem::size_of::<u64>() as phys_addr_t) as usize);
    let mut start_bad: phys_addr_t = 0;
    let mut last_bad: phys_addr_t = 0;
    let mut current_phys = start_phys_aligned;

    VM_WARN_ON_ONCE(size < start_phys_aligned - start_phys);

    p = start;
    while p < end {
        core::ptr::write_volatile(p, pattern);
        p = p.add(1);
    }

    p = start;
    while p < end {
        if core::ptr::read_volatile(p) == pattern {
            p = p.add(1);
            current_phys += core::mem::size_of::<u64>() as phys_addr_t;
            continue;
        }
        if current_phys == last_bad + core::mem::size_of::<u64>() as phys_addr_t {
            last_bad += core::mem::size_of::<u64>() as phys_addr_t;
            p = p.add(1);
            current_phys += core::mem::size_of::<u64>() as phys_addr_t;
            continue;
        }
        if start_bad != 0 {
            reserve_bad_mem(pattern, start_bad, last_bad + core::mem::size_of::<u64>() as phys_addr_t);
        }
        start_bad = current_phys;
        last_bad = current_phys;
        p = p.add(1);
        current_phys += core::mem::size_of::<u64>() as phys_addr_t;
    }
    if start_bad != 0 {
        reserve_bad_mem(pattern, start_bad, last_bad + core::mem::size_of::<u64>() as phys_addr_t);
    }

    early_memtest_done = true;
}

unsafe fn do_one_pass(pattern: u64, start: phys_addr_t, end: phys_addr_t) {
    let mut i: u64;
    let mut this_start: phys_addr_t;
    let mut this_end: phys_addr_t;

    for_each_free_mem_range!(i, NUMA_NO_NODE, MEMBLOCK_NONE, &mut this_start,
        &mut this_end, core::ptr::null_mut(), {
        this_start = clamp(this_start, start, end);
        this_end = clamp(this_end, start, end);
        if this_start < this_end {
            pr_info!("  %pa - %pa pattern %016llx\n",
                &this_start, &this_end, cpu_to_be64(pattern));
            memtest(pattern, this_start, this_end - this_start);
        }
    });
}

// default is disabled
static mut memtest_pattern: u32 = 0;

unsafe fn parse_memtest(arg: *mut i8) -> i32 {
    let mut ret = 0;
    if !arg.is_null() {
        ret = kstrtouint(arg, 0, &mut memtest_pattern);
    } else {
        memtest_pattern = patterns.len() as u32;
    }
    ret
}

early_param!("memtest", parse_memtest);

pub unsafe fn early_memtest(start: phys_addr_t, end: phys_addr_t) {
    let mut idx: usize = 0;
    if memtest_pattern == 0 {
        return;
    }
    pr_info!("early_memtest: # of tests: %u\n", memtest_pattern);
    let mut i = memtest_pattern.wrapping_sub(1);
    while i < u32::MAX {
        idx = (i as usize) % patterns.len();
        do_one_pass(patterns[idx], start, end);
        i = i.wrapping_sub(1);
    }
}

pub unsafe fn memtest_report_meminfo(m: *mut seq_file) {
    let mut early_memtest_bad_size_kb: usize;
    if !IS_ENABLED!(CONFIG_PROC_FS) {
        return;
    }
    if !early_memtest_done {
        return;
    }
    early_memtest_bad_size_kb = (early_memtest_bad_size >> 10) as usize;
    if early_memtest_bad_size != 0 && early_memtest_bad_size_kb == 0 {
        early_memtest_bad_size_kb = 1;
    }
    // When 0 is reported, it means there actually was a successful test
    seq_printf!(m, "EarlyMemtestBad:   %5lu kB\n", early_memtest_bad_size_kb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
