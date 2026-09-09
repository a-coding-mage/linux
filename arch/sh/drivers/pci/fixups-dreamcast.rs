// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/drivers/pci/fixups-dreamcast.c
 *
 * PCI fixups for the Sega Dreamcast
 *
 * Copyright (C) 2001, 2002  M. R. Brown
 * Copyright (C) 2002, 2003, 2006  Paul Mundt
 *
 * This file originally bore the message (with enclosed-$):
 *\tId: pci.c,v 1.3 2003/05/04 19:29:46 lethal Exp
 *\tDreamcast PCI: Supports SEGA Broadband Adaptor only.
 */

// Kernel and architecture declarations are supplied by the surrounding tree.

unsafe extern "C" {
    fn printk(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn pci_name(dev: *const pci_dev) -> *const core::ffi::c_char;
    fn pcibios_resource_to_bus(bus: *mut pci_bus, region: *mut pci_bus_region,
                               res: *const resource);
    fn dma_declare_coherent_memory(dev: *mut device, phys_addr: usize,
                                   dma_addr: u64, size: usize) -> core::ffi::c_int;
    fn resource_size(res: *const resource) -> usize;
    fn bug_on(condition: bool);
}

#[repr(C)]
struct pci_dev {
    sysdata: *mut core::ffi::c_void,
    device: u16,
    resource: [resource; 6],
    bus: *mut pci_bus,
    dev: device,
}

#[repr(C)]
struct pci_channel {
    resources: [resource; 1],
}

#[repr(C)]
struct resource {
    start: usize,
    end: usize,
    flags: u64,
}

#[repr(C)]
struct pci_bus_region {
    start: u64,
    end: u64,
}

#[repr(C)]
struct pci_bus;
#[repr(C)]
struct device;

const PCI_DEVICE_ID_SEGA_BBA: u16 = 0xabc;
const IORESOURCE_PCI_FIXED: u64 = 0x0000_0040;
const IORESOURCE_MEM: u64 = 0x0000_0200;
// GAPSPCI_DMA_BASE, GAPSPCI_DMA_SIZE, and GAPSPCI_IRQ are supplied by mach/pci.h.

unsafe fn gapspci_fixup_resources(dev: *mut pci_dev) {
    let p = (*dev).sysdata as *mut pci_channel;
    let mut res: resource = core::mem::zeroed();
    let mut region: pci_bus_region = core::mem::zeroed();

    printk(b"PCI: Fixing up device %s\n\0".as_ptr() as *const _, pci_name(dev));

    match (*dev).device {
        PCI_DEVICE_ID_SEGA_BBA => {
            /*
             * We also assume that dev->devfn == 0
             */
            (*dev).resource[1].start = (*p).resources[0].start + 0x100;
            (*dev).resource[1].end = (*dev).resource[1].start + 0x200 - 1;

            /*
             * This is not a normal BAR, prevent any attempts to move
             * the BAR, as this will result in a bus lock.
             */
            (*dev).resource[1].flags |= IORESOURCE_PCI_FIXED;

            /*
             * Redirect dma memory allocations to special memory window.
             *
             * If this GAPSPCI region were mapped by a BAR, the CPU
             * phys_addr_t would be pci_resource_start(), and the bus
             * address would be pci_bus_address(pci_resource_start()).
             * But apparently there's no BAR mapping it, so we just
             * "know" its CPU address is GAPSPCI_DMA_BASE.
             */
            res.start = GAPSPCI_DMA_BASE;
            res.end = GAPSPCI_DMA_BASE + GAPSPCI_DMA_SIZE - 1;
            res.flags = IORESOURCE_MEM;
            pcibios_resource_to_bus((*dev).bus, &mut region, &res);
            bug_on(dma_declare_coherent_memory(
                &mut (*dev).dev,
                res.start,
                region.start,
                resource_size(&res),
            ) != 0);
        }
        _ => {
            printk(b"PCI: Failed resource fixup\n\0".as_ptr() as *const _);
        }
    }
}

// DECLARE_PCI_FIXUP_HEADER(PCI_ANY_ID, PCI_ANY_ID, gapspci_fixup_resources);

unsafe fn pcibios_map_platform_irq(_dev: *const pci_dev, _slot: u8, _pin: u8) -> i32 {
    /*
     * The interrupt routing semantics here are quite trivial.
     *
     * We basically only support one interrupt, so we only bother
     * updating a device's interrupt line with this single shared
     * interrupt. Keeps routing quite simple, doesn't it?
     */
    GAPSPCI_IRQ
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
