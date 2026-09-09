// SPDX-License-Identifier: GPL-2.0
/*
 * SGI IP30 miscellaneous setup bits.
 *
 * Copyright (C) 2004-2007 Stanislaw Skowronek <skylark@unaligned.org>
 *               2007 Joshua Kinard <linux@kumba.dev>
 *               2009 Johannes Dickgreber <tanzy@gmx.de>
 */

// Dependencies supplied by the surrounding kernel translation unit.

/* Structure of accessible HEART registers located in XKPHYS space. */
extern "C" {
    static mut heart_regs: *mut ip30_heart_regs = HEART_XKPHYS_BASE;
}

/*
 * ARCS will report up to the first 1GB of
 * memory if queried.  Anything beyond that
 * is marked as reserved.
 */
const IP30_MAX_PROM_MEMORY: usize = 0x40000000;

/*
 * Memory in the Octane starts at 512MB
 */
const IP30_MEMORY_BASE: usize = 0x20000000;

/*
 * If using ARCS to probe for memory, then
 * remaining memory will start at this offset.
 */
const IP30_REAL_MEMORY_START: usize = IP30_MEMORY_BASE + IP30_MAX_PROM_MEMORY;

#[inline]
const fn mem_shift(x: usize) -> usize {
    x >> 20
}

unsafe fn ip30_mem_init() {
    let mut total_mem: usize = 0;
    let mut i: i32 = 0;
    while i < HEART_MEMORY_BANKS {
        let memcfg: u32 = __raw_readl(&(*heart_regs).mem_cfg.l[i as usize]);
        if (memcfg & HEART_MEMCFG_VALID) == 0 {
            i += 1;
            continue;
        }

        let mut addr: usize = ((memcfg & HEART_MEMCFG_ADDR_MASK) as usize)
            << HEART_MEMCFG_UNIT_SHIFT;
        addr += IP30_MEMORY_BASE;
        let mut size: usize = ((memcfg & HEART_MEMCFG_SIZE_MASK) as usize)
            >> HEART_MEMCFG_SIZE_SHIFT;
        size += 1;
        size <<= HEART_MEMCFG_UNIT_SHIFT;

        total_mem = total_mem.wrapping_add(size);

        if addr >= IP30_REAL_MEMORY_START {
            memblock_phys_free(addr, size);
        } else if addr.wrapping_add(size) > IP30_REAL_MEMORY_START {
            memblock_phys_free(IP30_REAL_MEMORY_START, size - IP30_MAX_PROM_MEMORY);
        }
        i += 1;
    }
    pr_info!("Detected {}MB of physical memory.\n", mem_shift(total_mem));
}

/**
 * ip30_cpu_time_init - platform time initialization.
 */
unsafe fn ip30_cpu_time_init() {
    let cpu: i32 = smp_processor_id();
    let heart_compare: u64 = heart_read(&(*heart_regs).count)
        .wrapping_add(HEART_CYCLES_PER_SEC / 10);
    let start: u32 = read_c0_count();
    while (heart_read(&(*heart_regs).count).wrapping_sub(heart_compare) & 0x800000) != 0 {
        cpu_relax();
    }

    let end: u32 = read_c0_count();
    let time_diff: i32 = (end as i32).wrapping_sub(start as i32);
    mips_hpt_frequency = time_diff.wrapping_mul(10);
    pr_info!(
        "IP30: CPU{}: {} MHz CPU detected.\n",
        cpu,
        (mips_hpt_frequency * 2) / 1_000_000
    );
}

pub unsafe fn ip30_per_cpu_init() {
    /* Disable all interrupts. */
    clear_c0_status(ST0_IM);

    ip30_cpu_time_init();
    // CONFIG_SMP: install the IPI handler when symmetric multiprocessing is enabled.
    #[cfg(CONFIG_SMP)]
    ip30_install_ipi();

    enable_percpu_irq(IP30_HEART_L0_IRQ, IRQ_TYPE_NONE);
    enable_percpu_irq(IP30_HEART_L1_IRQ, IRQ_TYPE_NONE);
    enable_percpu_irq(IP30_HEART_L2_IRQ, IRQ_TYPE_NONE);
    enable_percpu_irq(IP30_HEART_ERR_IRQ, IRQ_TYPE_NONE);
}

/**
 * plat_mem_setup - despite the name, misc setup happens here.
 */
pub unsafe fn plat_mem_setup() {
    ip30_mem_init();

    /* XXX: Hard lock on /sbin/init if this flag isn't specified. */
    prom_flags |= PROM_FLAG_DONT_FREE_TEMP;

    // CONFIG_SMP: register SMP operations when symmetric multiprocessing is enabled.
    #[cfg(CONFIG_SMP)]
    register_smp_ops(&ip30_smp_ops);
    #[cfg(not(CONFIG_SMP))]
    ip30_per_cpu_init();

    ioport_resource.start = 0;
    ioport_resource.end = !0usize;
    set_io_port_base(IO_BASE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
