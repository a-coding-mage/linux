// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2006  Ralf Baechle <ralf@linux-mips.org>
 */
// C dependencies:
//   #include <linux/dma-direct.h>
//   #include <asm/ip32/crime.h>

/*
 * Few notes.
 * 1. CPU sees memory as two chunks: 0-256M@0x0, and the rest @0x40000000+256M
 * 2. PCI sees memory as one big chunk @0x0 (or we could use 0x40000000 for
 *    native-endian)
 * 3. All other devices see memory as one big chunk at 0x40000000
 * 4. Non-PCI devices will pass NULL as struct device*
 *
 * Thus we translate differently, depending on device.
 */

const RAM_OFFSET_MASK: usize = 0x3fffffffusize;

pub unsafe fn phys_to_dma(dev: *mut device, paddr: phys_addr_t) -> dma_addr_t {
    let mut dma_addr: dma_addr_t = (paddr as usize & RAM_OFFSET_MASK) as dma_addr_t;

    if dev.is_null() {
        dma_addr = dma_addr.wrapping_add(CRIME_HI_MEM_BASE as dma_addr_t);
    }
    dma_addr
}

pub unsafe fn dma_to_phys(dev: *mut device, dma_addr: dma_addr_t) -> phys_addr_t {
    let mut paddr: phys_addr_t = (dma_addr as usize & RAM_OFFSET_MASK) as phys_addr_t;

    if dma_addr >= (256usize * 1024usize * 1024usize) as dma_addr_t {
        paddr = paddr.wrapping_add(CRIME_HI_MEM_BASE as phys_addr_t);
    }
    paddr
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
