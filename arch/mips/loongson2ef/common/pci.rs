// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007 Lemote, Inc. & Institute of Computing Technology
 * Author: Fuxin Zhang, zhangfx@lemote.com
 */

// Dependencies supplied by the Linux PCI, PCI, and Loongson headers.

static mut loongson_pci_mem_resource: resource = resource {
    name: "pci memory space",
    start: LOONGSON_PCI_MEM_START,
    end: LOONGSON_PCI_MEM_END,
    flags: IORESOURCE_MEM,
};

static mut loongson_pci_io_resource: resource = resource {
    name: "pci io space",
    start: 0x00000000u32, /* See loongson2ef_pcibios_init(). */
    end: IO_SPACE_LIMIT,
    flags: IORESOURCE_IO,
};

static mut loongson_pci_controller: pci_controller = pci_controller {
    pci_ops: &loongson_pci_ops,
    io_resource: &loongson_pci_io_resource,
    mem_resource: &loongson_pci_mem_resource,
    mem_offset: 0x00000000u32,
    io_offset: 0x00000000u32,
};

unsafe fn setup_pcimap() {
    /*
     * local to PCI mapping for CPU accessing PCI space
     * CPU address space [256M,448M] is window for accessing pci space
     * we set pcimap_lo[0,1,2] to map it to pci space[0M,64M], [320M,448M]
     *
     * pcimap: PCI_MAP2  PCI_Mem_Lo2 PCI_Mem_Lo1 PCI_Mem_Lo0
     *         [<2G]   [384M,448M] [320M,384M] [0M,64M]
     */
    LOONGSON_PCIMAP = LOONGSON_PCIMAP_PCIMAP_2
        | LOONGSON_PCIMAP_WIN(2, LOONGSON_PCILO2_BASE)
        | LOONGSON_PCIMAP_WIN(1, LOONGSON_PCILO1_BASE)
        | LOONGSON_PCIMAP_WIN(0, 0);

    /*
     * PCI-DMA to local mapping: [2G,2G+256M] -> [0M,256M]
     */
    LOONGSON_PCIBASE0 = 0x80000000u32; // base: 2G -> mmap: 0M
    /* size: 256M, burst transmission, pre-fetch enable, 64bit */
    LOONGSON_PCI_HIT0_SEL_L = 0xc000000cu32;
    LOONGSON_PCI_HIT0_SEL_H = 0xffffffffu32;
    LOONGSON_PCI_HIT1_SEL_L = 0x00000006u32; // set this BAR as invalid
    LOONGSON_PCI_HIT1_SEL_H = 0x00000000u32;
    LOONGSON_PCI_HIT2_SEL_L = 0x00000006u32; // set this BAR as invalid
    LOONGSON_PCI_HIT2_SEL_H = 0x00000000u32;

    /* avoid deadlock of PCI reading/writing lock operation */
    LOONGSON_PCI_ISR4C = 0xd2000001u32;

    /* can not change gnt to break pci transfer when device's gnt not
     * deassert for some broken device */
    LOONGSON_PXARB_CFG = 0x00fe0105u32;

    // Preserved build-time condition: CONFIG_CPU_SUPPORTS_ADDRWINCFG.
    #[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
    {
        /*
         * set cpu addr window2 to map CPU address space to PCI address space
         */
        LOONGSON_ADDRWIN_CPUTOPCI(
            ADDRWIN_WIN2,
            LOONGSON_CPU_MEM_SRC,
            LOONGSON_PCI_MEM_DST,
            MMAP_CPUTOPCI_SIZE,
        );
    }
}

unsafe fn loongson2ef_pcibios_init() {
    setup_pcimap();

    /*
     * ISA-mode only IDE controllers have a hard dependency on ISA IO ports.
     *
     * Claim them by setting PCI IO space to start at 0x00000000, and set
     * PCIBIOS_MIN_IO to prevent non-legacy PCI devices from touching
     * reserved regions.
     */
    PCIBIOS_MIN_IO = LOONGSON_PCI_IO_START;

    loongson_pci_controller.io_map_base = mips_io_port_base;
    register_pci_controller(&mut loongson_pci_controller);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
