// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 1999, 2000, 2004, 2005 MIPS Technologies, Inc.
 * All rights reserved.
 * Authors: Carsten Langgaard <carstenl@mips.com>
 *          Maciej W. Rozycki <macro@mips.com>
 *
 * Copyright (C) 2004 by Ralf Baechle (ralf@linux-mips.org)
 *
 * MIPS boards specific PCI support.
 */

// Linux and architecture headers provide the following types, constants, and functions.

static mut bonito64_mem_resource: resource = resource {
    name: cstr!("Bonito PCI MEM"), flags: IORESOURCE_MEM,
};
static mut bonito64_io_resource: resource = resource {
    name: cstr!("Bonito PCI I/O"), start: 0x00000000, end: 0x000fffff,
    flags: IORESOURCE_IO,
};
static mut gt64120_mem_resource: resource = resource {
    name: cstr!("GT-64120 PCI MEM"), flags: IORESOURCE_MEM,
};
static mut gt64120_io_resource: resource = resource {
    name: cstr!("GT-64120 PCI I/O"), flags: IORESOURCE_IO,
};
static mut msc_mem_resource: resource = resource {
    name: cstr!("MSC PCI MEM"), flags: IORESOURCE_MEM,
};
static mut msc_io_resource: resource = resource {
    name: cstr!("MSC PCI I/O"), flags: IORESOURCE_IO,
};

extern "C" {
    static mut bonito64_pci_ops: pci_ops;
    static mut gt64xxx_pci0_ops: pci_ops;
    static mut msc_pci_ops: pci_ops;
}

static mut bonito64_controller: pci_controller = pci_controller {
    pci_ops: unsafe { &mut bonito64_pci_ops },
    io_resource: unsafe { &mut bonito64_io_resource },
    mem_resource: unsafe { &mut bonito64_mem_resource },
    io_offset: 0x00000000,
};
static mut gt64120_controller: pci_controller = pci_controller {
    pci_ops: unsafe { &mut gt64xxx_pci0_ops },
    io_resource: unsafe { &mut gt64120_io_resource },
    mem_resource: unsafe { &mut gt64120_mem_resource },
};
static mut msc_controller: pci_controller = pci_controller {
    pci_ops: unsafe { &mut msc_pci_ops },
    io_resource: unsafe { &mut msc_io_resource },
    mem_resource: unsafe { &mut msc_mem_resource },
};

pub unsafe extern "C" fn mips_pcibios_init() {
    let mut controller: *mut pci_controller;
    let (mut start, mut end, mut map, mut start1, mut end1, mut map1, mut map2, mut map3, mut mask): (resource_size_t, resource_size_t, resource_size_t, resource_size_t, resource_size_t, resource_size_t, resource_size_t, resource_size_t, resource_size_t);

    match mips_revision_sconid {
        MIPS_REVISION_SCON_GT64120 => {
            GT_WRITE(GT_PCI0_CFGADDR_OFS,
                (0 << GT_PCI0_CFGADDR_BUSNUM_SHF) |
                (0 << GT_PCI0_CFGADDR_DEVNUM_SHF) |
                (0 << GT_PCI0_CFGADDR_FUNCTNUM_SHF) |
                ((0x20 / 4) << GT_PCI0_CFGADDR_REGNUM_SHF) |
                GT_PCI0_CFGADDR_CONFIGEN_BIT);
            GT_WRITE(GT_PCI0_CFGDATA_OFS, CPHYSADDR(MIPS_GT_BASE));

            start = GT_READ(GT_PCI0M0LD_OFS);
            end = GT_READ(GT_PCI0M0HD_OFS);
            map = GT_READ(GT_PCI0M0REMAP_OFS);
            end = (end & GT_PCI_HD_MSK) | (start & !GT_PCI_HD_MSK);
            start1 = GT_READ(GT_PCI0M1LD_OFS);
            end1 = GT_READ(GT_PCI0M1HD_OFS);
            map1 = GT_READ(GT_PCI0M1REMAP_OFS);
            end1 = (end1 & GT_PCI_HD_MSK) | (start1 & !GT_PCI_HD_MSK);
            if end1 - start1 > end - start { start = start1; end = end1; map = map1; }
            mask = !(start ^ end);
            BUG_ON((start & GT_PCI_HD_MSK) != (map & GT_PCI_HD_MSK) && mask != !((mask & (!mask + 1)) - 1));
            gt64120_mem_resource.start = start;
            gt64120_mem_resource.end = end;
            gt64120_controller.mem_offset = (start & mask) - (map & mask);
            gt64120_mem_resource.start <<= GT_PCI_DCRM_SHF;
            gt64120_mem_resource.end <<= GT_PCI_DCRM_SHF;
            gt64120_mem_resource.end |= (1 << GT_PCI_DCRM_SHF) - 1;
            gt64120_controller.mem_offset <<= GT_PCI_DCRM_SHF;

            start = GT_READ(GT_PCI0IOLD_OFS);
            end = GT_READ(GT_PCI0IOHD_OFS);
            map = GT_READ(GT_PCI0IOREMAP_OFS);
            end = (end & GT_PCI_HD_MSK) | (start & !GT_PCI_HD_MSK);
            mask = !(start ^ end);
            BUG_ON((start & GT_PCI_HD_MSK) != (map & GT_PCI_HD_MSK) && mask != !((mask & (!mask + 1)) - 1));
            gt64120_io_resource.start = map & mask;
            gt64120_io_resource.end = (map & mask) | !mask;
            gt64120_controller.io_offset = 0;
            gt64120_io_resource.start <<= GT_PCI_DCRM_SHF;
            gt64120_io_resource.end <<= GT_PCI_DCRM_SHF;
            gt64120_io_resource.end |= (1 << GT_PCI_DCRM_SHF) - 1;
            controller = &mut gt64120_controller;
        }
        MIPS_REVISION_SCON_BONITO => {
            map = BONITO_PCIMAP;
            map1 = (BONITO_PCIMAP & BONITO_PCIMAP_PCIMAP_LO0) >> BONITO_PCIMAP_PCIMAP_LO0_SHIFT;
            map2 = (BONITO_PCIMAP & BONITO_PCIMAP_PCIMAP_LO1) >> BONITO_PCIMAP_PCIMAP_LO1_SHIFT;
            map3 = (BONITO_PCIMAP & BONITO_PCIMAP_PCIMAP_LO2) >> BONITO_PCIMAP_PCIMAP_LO2_SHIFT;
            map = map1; start = BONITO_PCILO0_BASE; end = 1;
            if map3 == map2 + 1 { map = map2; start = BONITO_PCILO1_BASE; end += 1; }
            if map2 == map1 + 1 { map = map1; start = BONITO_PCILO0_BASE; end += 1; }
            bonito64_mem_resource.start = start;
            bonito64_mem_resource.end = start + BONITO_PCIMAP_WINBASE(end) - 1;
            bonito64_controller.mem_offset = start - BONITO_PCIMAP_WINBASE(map);
            controller = &mut bonito64_controller;
        }
        MIPS_REVISION_SCON_SOCIT | MIPS_REVISION_SCON_ROCIT |
        MIPS_REVISION_SCON_SOCITSC | MIPS_REVISION_SCON_SOCITSCP => {
            MSC_READ(MSC01_PCI_SC2PMBASL, start);
            MSC_READ(MSC01_PCI_SC2PMMSKL, mask);
            MSC_READ(MSC01_PCI_SC2PMMAPL, map);
            msc_mem_resource.start = start & mask;
            msc_mem_resource.end = (start & mask) | !mask;
            msc_controller.mem_offset = (start & mask) - (map & mask);
            if mips_cps_numiocu(0) != 0 { write_gcr_reg0_base(start); write_gcr_reg0_mask(mask | CM_GCR_REGn_MASK_CMTGT_IOCU0); }
            MSC_READ(MSC01_PCI_SC2PIOBASL, start);
            MSC_READ(MSC01_PCI_SC2PIOMSKL, mask);
            MSC_READ(MSC01_PCI_SC2PIOMAPL, map);
            msc_io_resource.start = map & mask;
            msc_io_resource.end = (map & mask) | !mask;
            msc_controller.io_offset = 0;
            ioport_resource.end = !mask;
            if mips_cps_numiocu(0) != 0 { write_gcr_reg1_base(start); write_gcr_reg1_mask(mask | CM_GCR_REGn_MASK_CMTGT_IOCU0); }
            start = start & mask; end = start | !mask;
            if (start >= msc_mem_resource.start && start <= msc_mem_resource.end) || (end >= msc_mem_resource.start && end <= msc_mem_resource.end) {
                start = max(start, msc_mem_resource.start); end = min(end, msc_mem_resource.end);
                if start - msc_mem_resource.start >= msc_mem_resource.end - end { msc_mem_resource.end = start - 1; } else { msc_mem_resource.start = end + 1; }
            }
            controller = &mut msc_controller;
        }
        _ => return,
    }
    PCIBIOS_MIN_IO = 0x1000;
    iomem_resource.end &= 0xfffffffff;
    ioport_resource.end = (*(*controller).io_resource).end;
    (*controller).io_map_base = mips_io_port_base;
    register_pci_controller(controller);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
