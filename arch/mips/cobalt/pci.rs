/*
 * Register PCI controller.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 1997, 2004, 05 by Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 2001, 2002, 2003 by Liam Davies (ldavies@agile.tv)
 *
 */

// Declarations supplied by the Linux and MIPS platform dependencies.
extern "C" {
    static mut gt64xxx_pci0_ops: pci_ops;
    fn register_pci_controller(controller: *mut pci_controller);
}

static mut cobalt_mem_resource: resource = resource {
    start: GT_DEF_PCI0_MEM0_BASE,
    end: GT_DEF_PCI0_MEM0_BASE + GT_DEF_PCI0_MEM0_SIZE - 1,
    name: b"PCI memory\0" as *const u8 as *const i8,
    flags: IORESOURCE_MEM,
};

static mut cobalt_io_resource: resource = resource {
    start: 0x1000,
    end: 0xffffff,
    name: b"PCI I/O\0" as *const u8 as *const i8,
    flags: IORESOURCE_IO,
};

static mut cobalt_pci_controller: pci_controller = pci_controller {
    pci_ops: unsafe { &mut gt64xxx_pci0_ops as *mut pci_ops },
    mem_resource: unsafe { &mut cobalt_mem_resource as *mut resource },
    io_resource: unsafe { &mut cobalt_io_resource as *mut resource },
    io_offset: 0u64.wrapping_sub(GT_DEF_PCI0_IO_BASE),
    // CKSEG1ADDR is a platform macro supplied by the MIPS dependencies.
    io_map_base: CKSEG1ADDR!(GT_DEF_PCI0_IO_BASE),
};

unsafe fn cobalt_pci_init() -> i32 {
    register_pci_controller(&mut cobalt_pci_controller);

    0
}

// __init and arch_initcall are build-time Linux annotations/macros.
arch_initcall!(cobalt_pci_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
