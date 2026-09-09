/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2000  Ani Joshi <ajoshi@unixbox.com>
 * Copyright (C) 2000, 2001  Ralf Baechle <ralf@gnu.org>
 * Copyright (C) 2005 Ilya A. Volynets-Evenbakh <ilya@total-knowledge.com>
 * swiped from i386, and cloned for MIPS by Geert, polished by Ralf.
 * IP32 changes by Ilya.
 * Copyright (C) 2010 Cavium Networks, Inc.
 */
// Dependencies supplied by the surrounding kernel translation are intentionally omitted.

#[cfg(feature = "CONFIG_PCI")]
#[repr(C)]
struct octeon_dma_map_ops {
    phys_to_dma: unsafe extern "C" fn(dev: *mut device, paddr: phys_addr_t) -> dma_addr_t,
    dma_to_phys: unsafe extern "C" fn(dev: *mut device, daddr: dma_addr_t) -> phys_addr_t,
}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn octeon_hole_phys_to_dma(mut paddr: phys_addr_t) -> dma_addr_t {
    if paddr >= CVMX_PCIE_BAR1_PHYS_BASE && paddr < CVMX_PCIE_BAR1_PHYS_BASE + CVMX_PCIE_BAR1_PHYS_SIZE {
        paddr - CVMX_PCIE_BAR1_PHYS_BASE + CVMX_PCIE_BAR1_RC_BASE
    } else { paddr }
}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn octeon_hole_dma_to_phys(mut daddr: dma_addr_t) -> phys_addr_t {
    if daddr >= CVMX_PCIE_BAR1_RC_BASE {
        daddr + CVMX_PCIE_BAR1_PHYS_BASE - CVMX_PCIE_BAR1_RC_BASE
    } else { daddr }
}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn octeon_gen1_phys_to_dma(_dev: *mut device, mut paddr: phys_addr_t) -> dma_addr_t {
    if paddr >= 0x410000000u64 && paddr < 0x420000000u64 { paddr -= 0x400000000u64; }
    octeon_hole_phys_to_dma(paddr)
}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn octeon_gen1_dma_to_phys(_dev: *mut device, mut daddr: dma_addr_t) -> phys_addr_t {
    daddr = octeon_hole_dma_to_phys(daddr);
    if daddr >= 0x10000000u64 && daddr < 0x20000000u64 { daddr += 0x400000000u64; }
    daddr
}

#[cfg(feature = "CONFIG_PCI")]
static octeon_gen1_ops: octeon_dma_map_ops = octeon_dma_map_ops { phys_to_dma: octeon_gen1_phys_to_dma, dma_to_phys: octeon_gen1_dma_to_phys };

#[cfg(feature = "CONFIG_PCI")]
unsafe fn octeon_gen2_phys_to_dma(_dev: *mut device, paddr: phys_addr_t) -> dma_addr_t { octeon_hole_phys_to_dma(paddr) }
#[cfg(feature = "CONFIG_PCI")]
unsafe fn octeon_gen2_dma_to_phys(_dev: *mut device, daddr: dma_addr_t) -> phys_addr_t { octeon_hole_dma_to_phys(daddr) }
#[cfg(feature = "CONFIG_PCI")]
static octeon_gen2_ops: octeon_dma_map_ops = octeon_dma_map_ops { phys_to_dma: octeon_gen2_phys_to_dma, dma_to_phys: octeon_gen2_dma_to_phys };

#[cfg(feature = "CONFIG_PCI")]
unsafe fn octeon_big_phys_to_dma(_dev: *mut device, mut paddr: phys_addr_t) -> dma_addr_t {
    if paddr >= 0x410000000u64 && paddr < 0x420000000u64 { paddr -= 0x400000000u64; }
    /* Anything in the BAR1 hole or above goes via BAR2 */
    if paddr >= 0xf0000000u64 { paddr = OCTEON_BAR2_PCI_ADDRESS + paddr; }
    paddr
}
#[cfg(feature = "CONFIG_PCI")]
unsafe fn octeon_big_dma_to_phys(_dev: *mut device, mut daddr: dma_addr_t) -> phys_addr_t {
    if daddr >= OCTEON_BAR2_PCI_ADDRESS { daddr -= OCTEON_BAR2_PCI_ADDRESS; }
    if daddr >= 0x10000000u64 && daddr < 0x20000000u64 { daddr += 0x400000000u64; }
    daddr
}
#[cfg(feature = "CONFIG_PCI")]
static octeon_big_ops: octeon_dma_map_ops = octeon_dma_map_ops { phys_to_dma: octeon_big_phys_to_dma, dma_to_phys: octeon_big_dma_to_phys };

#[cfg(feature = "CONFIG_PCI")]
unsafe fn octeon_small_phys_to_dma(_dev: *mut device, mut paddr: phys_addr_t) -> dma_addr_t {
    if paddr >= 0x410000000u64 && paddr < 0x420000000u64 { paddr -= 0x400000000u64; }
    /* Anything not in the BAR1 range goes via BAR2 */
    if paddr >= octeon_bar1_pci_phys && paddr < octeon_bar1_pci_phys + 0x8000000u64 { paddr - octeon_bar1_pci_phys } else { OCTEON_BAR2_PCI_ADDRESS + paddr }
}
#[cfg(feature = "CONFIG_PCI")]
unsafe fn octeon_small_dma_to_phys(_dev: *mut device, mut daddr: dma_addr_t) -> phys_addr_t {
    if daddr >= OCTEON_BAR2_PCI_ADDRESS { daddr -= OCTEON_BAR2_PCI_ADDRESS; } else { daddr += octeon_bar1_pci_phys; }
    if daddr >= 0x10000000u64 && daddr < 0x20000000u64 { daddr += 0x400000000u64; }
    daddr
}
#[cfg(feature = "CONFIG_PCI")]
static octeon_small_ops: octeon_dma_map_ops = octeon_dma_map_ops { phys_to_dma: octeon_small_phys_to_dma, dma_to_phys: octeon_small_dma_to_phys };

#[cfg(feature = "CONFIG_PCI")]
static mut octeon_pci_dma_ops: *const octeon_dma_map_ops = core::ptr::null();

#[cfg(feature = "CONFIG_PCI")]
unsafe fn octeon_pci_dma_init() {
    octeon_pci_dma_ops = match octeon_dma_bar_type {
        OCTEON_DMA_BAR_TYPE_PCIE => &octeon_gen1_ops,
        OCTEON_DMA_BAR_TYPE_PCIE2 => &octeon_gen2_ops,
        OCTEON_DMA_BAR_TYPE_BIG => &octeon_big_ops,
        OCTEON_DMA_BAR_TYPE_SMALL => &octeon_small_ops,
        _ => { BUG!(); core::ptr::null() }
    };
}

unsafe fn phys_to_dma(dev: *mut device, paddr: phys_addr_t) -> dma_addr_t {
    #[cfg(feature = "CONFIG_PCI")]
    if !dev.is_null() && dev_is_pci(dev) { return ((*octeon_pci_dma_ops).phys_to_dma)(dev, paddr); }
    paddr
}

unsafe fn dma_to_phys(dev: *mut device, daddr: dma_addr_t) -> phys_addr_t {
    #[cfg(feature = "CONFIG_PCI")]
    if !dev.is_null() && dev_is_pci(dev) { return ((*octeon_pci_dma_ops).dma_to_phys)(dev, daddr); }
    daddr
}

unsafe fn plat_swiotlb_setup() {
    let (mut start, mut end): (phys_addr_t, phys_addr_t);
    let mut max_addr: phys_addr_t = 0;
    let mut addr_size: phys_addr_t = 0;
    let mut i: u64;
    for_each_mem_range!(i, &mut start, &mut end) {
        /* These addresses map low for PCI. */
        if start > 0x410000000u64 && !OCTEON_IS_OCTEON2() { continue; }
        addr_size += end - start;
        if max_addr < end { max_addr = end; }
    }
    let mut swiotlbsize: usize = PAGE_SIZE;
    #[cfg(feature = "CONFIG_PCI")]
    {
        /* For OCTEON_DMA_BAR_TYPE_SMALL, size the iotlb at 1/4 memory size to a maximum of 64MB */
        if OCTEON_IS_MODEL(OCTEON_CN31XX) || OCTEON_IS_MODEL(OCTEON_CN38XX_PASS2) {
            swiotlbsize = (addr_size / 4) as usize;
            if swiotlbsize > 64 * (1usize << 20) { swiotlbsize = 64 * (1usize << 20); }
        } else if max_addr > 0xf0000000u64 { swiotlbsize = 64 * (1usize << 20); }
    }
    #[cfg(feature = "CONFIG_USB_OHCI_HCD_PLATFORM")]
    if OCTEON_IS_OCTEON2() && max_addr >= 0x100000000u64 { swiotlbsize = 64 * (1usize << 20); }
    swiotlb_adjust_size(swiotlbsize);
    swiotlb_init(true, SWIOTLB_VERBOSE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
