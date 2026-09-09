// SPDX-License-Identifier: GPL-2.0
/*
 * sc-rm7k.c: RM7000 cache management functions.
 *
 * Copyright (C) 1997, 2001, 2003, 2004 Ralf Baechle (ralf@linux-mips.org)
 */

// DEBUG is undefined in the original source.
// Linux and MIPS headers supplying the referenced symbols are external dependencies.

/* Primary cache parameters. */
const SC_LSIZE: usize = 32;
const TC_PAGESIZE: usize = 32 * 128;

/* Secondary cache parameters. */
const SCACHE_SIZE: usize = 256 * 1024; /* Fixed to 256KiB on RM7000 */

/* Tertiary cache parameters */
const TC_LSIZE: usize = 32;

extern "C" {
    static mut icache_way_size: usize;
    static mut dcache_way_size: usize;
}

static mut tcache_size: usize = 0;
static mut rm7k_tcache_init: i32 = 0;

/*
 * Writeback and invalidate the primary cache dcache before DMA.
 * (XXX These need to be fixed ...)
 */
unsafe fn rm7k_sc_wback_inv(addr: usize, size: usize) {
    pr_debug("rm7k_sc_wback_inv[%08lx,%08lx]", addr, size);

    /* Catch bad driver code */
    BUG_ON(size == 0);

    blast_scache_range(addr, addr + size);

    if rm7k_tcache_init == 0 {
        return;
    }

    let mut a = addr & !(TC_PAGESIZE - 1);
    let end = (addr + size - 1) & !(TC_PAGESIZE - 1);
    loop {
        invalidate_tcache_page(a); /* Page_Invalidate_T */
        if a == end {
            break;
        }
        a += TC_PAGESIZE;
    }
}

unsafe fn rm7k_sc_inv(addr: usize, size: usize) {
    pr_debug("rm7k_sc_inv[%08lx,%08lx]", addr, size);

    /* Catch bad driver code */
    BUG_ON(size == 0);

    blast_inv_scache_range(addr, addr + size);

    if rm7k_tcache_init == 0 {
        return;
    }

    let mut a = addr & !(TC_PAGESIZE - 1);
    let end = (addr + size - 1) & !(TC_PAGESIZE - 1);
    loop {
        invalidate_tcache_page(a); /* Page_Invalidate_T */
        if a == end {
            break;
        }
        a += TC_PAGESIZE;
    }
}

unsafe fn blast_rm7k_tcache() {
    let mut start = CKSEG0ADDR(0);
    let end = start + tcache_size;

    write_c0_taglo(0);

    while start < end {
        cache_op(Page_Invalidate_T, start);
        start += TC_PAGESIZE;
    }
}

/* This function is executed in uncached address space. */
unsafe fn __rm7k_tc_enable() {
    set_c0_config(RM7K_CONF_TE);

    write_c0_taglo(0);
    write_c0_taghi(0);

    let mut i = 0usize;
    while i < tcache_size {
        cache_op(Index_Store_Tag_T, CKSEG0ADDR(i));
        i += TC_LSIZE;
    }
}

unsafe fn rm7k_tc_enable() {
    if read_c0_config() & RM7K_CONF_TE != 0 {
        return;
    }

    BUG_ON(tcache_size == 0);
    run_uncached(__rm7k_tc_enable);
}

/* This function is executed in uncached address space. */
unsafe fn __rm7k_sc_enable() {
    set_c0_config(RM7K_CONF_SE);

    write_c0_taglo(0);
    write_c0_taghi(0);

    let mut i = 0usize;
    while i < SCACHE_SIZE {
        cache_op(Index_Store_Tag_SD, CKSEG0ADDR(i));
        i += SC_LSIZE;
    }
}

unsafe fn rm7k_sc_enable() {
    if read_c0_config() & RM7K_CONF_SE != 0 {
        return;
    }

    pr_info("Enabling secondary cache...\n");
    run_uncached(__rm7k_sc_enable);

    if rm7k_tcache_init != 0 {
        rm7k_tc_enable();
    }
}

unsafe fn rm7k_tc_disable() {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    blast_rm7k_tcache();
    clear_c0_config(RM7K_CONF_TE);
    local_irq_restore(flags);
}

unsafe fn rm7k_sc_disable() {
    clear_c0_config(RM7K_CONF_SE);

    if rm7k_tcache_init != 0 {
        rm7k_tc_disable();
    }
}

static mut rm7k_sc_ops: bcache_ops = bcache_ops {
    bc_enable: rm7k_sc_enable,
    bc_disable: rm7k_sc_disable,
    bc_wback_inv: rm7k_sc_wback_inv,
    bc_inv: rm7k_sc_inv,
};

/*
 * This is a probing function like the one found in c-r4k.c, we look for the
 * wrap around point with different addresses.
 */
unsafe fn __probe_tcache() {
    let mut flags: usize = 0;
    let mut begin = (&_stext as *const _ as usize) & !((8 * 1024 * 1024) - 1);
    let end = begin + 8 * 1024 * 1024;

    local_irq_save(&mut flags);
    set_c0_config(RM7K_CONF_TE);

    /* Fill size-multiple lines with a valid tag */
    let mut pow2 = 256 * 1024;
    let mut addr = begin;
    while addr <= end {
        let p = addr as *const usize;
        core::ptr::read_volatile(p);
        addr = begin + pow2;
        pow2 <<= 1;
    }

    /* Load first line with a 0 tag, to check after */
    write_c0_taglo(0);
    write_c0_taghi(0);
    cache_op(Index_Store_Tag_T, begin);

    /* Look for the wrap-around */
    pow2 = 512 * 1024;
    addr = begin + 512 * 1024;
    while addr <= end {
        cache_op(Index_Load_Tag_T, addr);
        if read_c0_taglo() == 0 {
            break;
        }
        pow2 <<= 1;
        addr = begin + pow2;
    }

    addr -= begin;
    tcache_size = addr;
    clear_c0_config(RM7K_CONF_TE);
    local_irq_restore(flags);
}

pub unsafe fn rm7k_sc_init() {
    let c = &mut current_cpu_data;
    let config = read_c0_config();

    if config & RM7K_CONF_SC != 0 {
        return;
    }

    c.scache.linesz = SC_LSIZE;
    c.scache.ways = 4;
    c.scache.waybit = __ffs(SCACHE_SIZE / c.scache.ways);
    c.scache.waysize = SCACHE_SIZE / c.scache.ways;
    c.scache.sets = SCACHE_SIZE / (c.scache.linesz * c.scache.ways);
    printk!(KERN_INFO, "Secondary cache size {}K, linesize {} bytes.\n", SCACHE_SIZE >> 10, SC_LSIZE);

    if config & RM7K_CONF_SE == 0 {
        rm7k_sc_enable();
    }

    bcops = &mut rm7k_sc_ops;

    /* While we're at it let's deal with the tertiary cache. */
    rm7k_tcache_init = 0;
    tcache_size = 0;

    if config & RM7K_CONF_TC != 0 {
        return;
    }

    /* No efficient way to ask the hardware for the size of the tcache, so must probe for it. */
    run_uncached(__probe_tcache);
    rm7k_tc_enable();
    rm7k_tcache_init = 1;
    c.tcache.linesz = TC_LSIZE;
    c.tcache.ways = 1;
    pr_info("Tertiary cache size {}K.\n", tcache_size >> 10);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
