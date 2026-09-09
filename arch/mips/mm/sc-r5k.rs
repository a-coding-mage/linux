// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 1997, 2001 Ralf Baechle (ralf@gnu.org),
 * derived from r4xx0.c by David S. Miller (davem@davemloft.net).
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/kernel.h, linux/sched.h, linux/mm.h,
// asm/mipsregs.h, asm/bcache.h, asm/cacheops.h, asm/page.h,
// asm/mmu_context.h, asm/r4kcache.h.

/* Secondary cache size in bytes, if present. */
static mut scache_size: ::core::ffi::c_ulong = 0;

const SC_LINE: ::core::ffi::c_ulong = 32;
const SC_PAGE: ::core::ffi::c_ulong = 128 * SC_LINE;

#[inline]
unsafe fn blast_r5000_scache() {
    let mut start: ::core::ffi::c_ulong = INDEX_BASE;
    let end: ::core::ffi::c_ulong = start.wrapping_add(scache_size);

    while start < end {
        cache_op(R5K_Page_Invalidate_S, start);
        start = start.wrapping_add(SC_PAGE);
    }
}

unsafe fn r5k_dma_cache_inv_sc(
    addr: ::core::ffi::c_ulong,
    size: ::core::ffi::c_ulong,
) {
    let end: ::core::ffi::c_ulong;
    let mut a: ::core::ffi::c_ulong;

    /* Catch bad driver code */
    BUG_ON(size == 0);

    if size >= scache_size {
        blast_r5000_scache();
        return;
    }

    /* On the R5000 secondary cache we cannot
     * invalidate less than a page at a time.
     * The secondary cache is physically indexed, write-through.
     */
    a = addr & !(SC_PAGE - 1);
    end = addr.wrapping_add(size).wrapping_sub(1) & !(SC_PAGE - 1);
    while a <= end {
        cache_op(R5K_Page_Invalidate_S, a);
        a = a.wrapping_add(SC_PAGE);
    }
}

unsafe fn r5k_sc_enable() {
    let mut flags: ::core::ffi::c_ulong = 0;

    local_irq_save(&mut flags);
    set_c0_config(R5K_CONF_SE);
    blast_r5000_scache();
    local_irq_restore(flags);
}

unsafe fn r5k_sc_disable() {
    let mut flags: ::core::ffi::c_ulong = 0;

    local_irq_save(&mut flags);
    blast_r5000_scache();
    clear_c0_config(R5K_CONF_SE);
    local_irq_restore(flags);
}

#[inline]
unsafe fn r5k_sc_probe() -> ::core::ffi::c_int {
    let config: ::core::ffi::c_ulong = read_c0_config();

    if config & CONF_SC != 0 {
        return 0;
    }

    scache_size = (512 * 1024) << ((config & R5K_CONF_SS) >> 20);

    printk(
        "R5000 SCACHE size %ldkB, linesize 32 bytes.\n",
        scache_size >> 10,
    );

    1
}

static mut r5k_sc_ops: struct bcache_ops = struct bcache_ops {
    bc_enable: Some(r5k_sc_enable),
    bc_disable: Some(r5k_sc_disable),
    bc_wback_inv: Some(r5k_dma_cache_inv_sc),
    bc_inv: Some(r5k_dma_cache_inv_sc),
};

pub unsafe fn r5k_sc_init() {
    if r5k_sc_probe() != 0 {
        r5k_sc_enable();
        bcops = &mut r5k_sc_ops;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
