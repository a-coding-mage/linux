/*
 *  Copyright (C) 2004 Florian Schirmer <jolt@tuxbox.org>
 *  Copyright (C) 2007 Aurelien Jarno <aurelien@aurel32.net>
 *  Copyright (C) 2010-2012 Hauke Mehrtens <hauke@hauke-m.de>
 *
 *  This program is free software; you can redistribute  it and/or modify it
 *  under the terms of the GNU General Public License as published by the
 *  Free Software Foundation; either version 2 of the License, or (at your
 *  option) any later version.
 */

// Linux and architecture-specific dependencies are supplied by the surrounding kernel translation.

static mut BCM47XX_SYSTEM_TYPE: [core::ffi::c_char; 20] = *b"Broadcom BCM47XX\0\0\0\0\0";

pub unsafe fn get_system_type() -> *const core::ffi::c_char {
    BCM47XX_SYSTEM_TYPE.as_ptr()
}

pub unsafe fn bcm47xx_set_system_type(chip_id: u16) {
    let format = if chip_id > 0x9999 {
        "Broadcom BCM%d"
    } else {
        "Broadcom BCM%04X"
    };
    snprintf(BCM47XX_SYSTEM_TYPE.as_mut_ptr(), core::mem::size_of_val(&BCM47XX_SYSTEM_TYPE), format, chip_id);
}

static mut lowmem: usize = 0;

unsafe fn prom_init_mem() {
    let mut mem: usize;
    let max: usize;
    let off: usize;
    let c = &current_cpu_data;

    /* Figure out memory size by finding aliases.
     *
     * We should theoretically use the mapping from CFE using cfe_enummem().
     * However as the BCM47XX is mostly used on low-memory systems, we
     * want to reuse the memory used by CFE (around 4MB). That means cfe_*
     * functions stop to work at some point during the boot, we should only
     * call them at the beginning of the boot.
     *
     * BCM47XX uses 128MB for addressing the ram, if the system contains
     * less than that amount of ram it remaps the ram more often into the
     * available space.
     */

    /* Physical address, without mapping to any kernel segment */
    off = CPHYSADDR(prom_init as usize);

    /* Accessing memory after 128 MiB will cause an exception */
    max = 128usize << 20;

    mem = 1usize << 20;
    while mem < max {
        /* Loop condition may be not enough, off may be over 1 MiB */
        if off + mem >= max {
            mem = max;
            pr_debug!("Assume 128MB RAM\n");
            break;
        }
        if core::slice::from_raw_parts(prom_init as *const u8, 32)
            == core::slice::from_raw_parts((prom_init as *const u8).add(mem), 32)
        {
            break;
        }
        mem += 1usize << 20;
    }
    lowmem = mem;

    /* Ignoring the last page when ddr size is 128M. Cached
     * accesses to last page is causing the processor to prefetch
     * using address above 128M stepping out of the ddr address
     * space.
     */
    if c.cputype == CPU_74K && mem == (128usize << 20) {
        mem -= 0x1000;
    }
    memblock_add(0, mem);
}

/*
 * This is the first serial on the chip common core, it is at this position
 * for sb (ssb) and ai (bcma) bus.
 */
const BCM47XX_SERIAL_ADDR: usize = SSB_ENUM_BASE + SSB_CHIPCO_UART0_DATA;

pub unsafe fn prom_init() {
    /* Cache CBR addr before CPU/DMA setup */
    bmips_cbr_addr = BMIPS_GET_CBR();
    prom_init_mem();
    setup_8250_early_printk_port(CKSEG1ADDR(BCM47XX_SERIAL_ADDR), 0, 0);
}

// Preserved conditional intent: this section is compiled for BCM47XX BCMA with HIGHMEM.
#[cfg(all(feature = "CONFIG_BCM47XX_BCMA", feature = "CONFIG_HIGHMEM"))]
mod highmem {
    const EXTVBASE: usize = 0xc0000000;

    /* Stripped version of tlb_init, with the call to build_tlb_refill_handler
     * dropped. Calling it at this stage causes a hang.
     */
    unsafe fn early_tlb_init() {
        write_c0_pagemask(PM_DEFAULT_MASK);
        write_c0_wired(0);
        temp_tlb_entry = current_cpu_data.tlbsize - 1;
        local_flush_tlb_all();
    }

    pub unsafe fn bcm47xx_prom_highmem_init() {
        let mut off = prom_init as usize;
        let mut extmem: usize = 0;
        let mut highmem_region = false;

        if WARN_ON(bcm47xx_bus_type != BCM47XX_BUS_TYPE_BCMA) {
            return;
        }

        if bcm47xx_bus.bcma.bus.chipinfo.id == BCMA_CHIP_ID_BCM4706 {
            highmem_region = true;
        }

        if lowmem != 128usize << 20 || !highmem_region {
            return;
        }

        early_tlb_init();

        /* Add one temporary TLB entry to map SDRAM Region 2.
         *      Physical        Virtual
         *      0x80000000      0xc0000000      (1st: 256MB)
         *      0x90000000      0xd0000000      (2nd: 256MB)
         */
        add_temporary_entry(
            ENTRYLO(0x80000000),
            ENTRYLO(0x80000000 + (256usize << 20)),
            EXTVBASE,
            PM_256M,
        );

        off = EXTVBASE + __pa(off);
        extmem = 128usize << 20;
        while extmem < 512usize << 20 {
            if core::slice::from_raw_parts(prom_init as *const u8, 16)
                == core::slice::from_raw_parts((off + extmem) as *const u8, 16)
            {
                break;
            }
            extmem <<= 1;
        }
        extmem -= lowmem;

        early_tlb_init();

        if extmem == 0 {
            return;
        }

        pr_warn!("Found {} MiB of extra memory, but highmem is unsupported yet!\n", extmem >> 20);

        /* TODO: Register extra memory */
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
