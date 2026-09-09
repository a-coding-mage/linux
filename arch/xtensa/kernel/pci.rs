// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/xtensa/kernel/pci.c
 *
 * PCI bios-type initialisation for PCI machines
 *
 * Copyright (C) 2001-2005 Tensilica Inc.
 *
 * Based largely on work from Cort (ppc/kernel/pci.c)
 * IO functions copied from sparc.
 *
 * Chris Zankel <chris@zankel.net>
 */

use crate::{
    pci_align_resource, pci_name, pci_read_bridge_bases, pci_resource_start,
    pr_err, resource_size_t, vm_area_struct, PAGE_SHIFT, EINVAL, IORESOURCE_IO,
    IORESOURCE_MEM,
};
use crate::{pci_bus, pci_controller, pci_dev, resource};

/*
 * We need to avoid collisions with `mirrored' VGA ports
 * and other strange ISA hardware, so we always want the
 * addresses to be allocated in the 0x000-0x0ff region
 * modulo 0x400.
 *
 * Why? Because some silly external IO cards only decode
 * the low 10 bits of the IO address. The 0x00-0xff region
 * is reserved for motherboard devices that decode all 16
 * bits, so it's ok to allocate at, say, 0x2800-0x28ff,
 * but we want to try to avoid allocating at 0x2900-0x2bff
 * which might have be mirrored at 0x0100-0x03ff..
 */
pub unsafe fn pcibios_align_resource(
    data: *mut core::ffi::c_void,
    res: *const resource,
    empty_res: *const resource,
    size: resource_size_t,
    align: resource_size_t,
) -> resource_size_t {
    let dev = data as *mut pci_dev;
    let mut start = (*res).start;

    if (*res).flags & IORESOURCE_IO != 0 {
        if size > 0x100 {
            pr_err(
                "PCI: I/O Region %s/%d too large (%u bytes)\\n",
                pci_name(dev),
                (*dev).resource.offset_from(res),
                size,
            );
        }

        if start & 0x300 != 0 {
            start = (start.wrapping_add(0x3ff)) & !0x3ff;
        }
    } else if (*res).flags & IORESOURCE_MEM != 0 {
        start = pci_align_resource(dev, res, empty_res, size, align);
    }

    start
}

pub unsafe fn pcibios_fixup_bus(bus: *mut pci_bus) {
    if !(*bus).parent.is_null() {
        /* This is a subordinate bridge */
        pci_read_bridge_bases(bus);
    }
}

/*
 * Platform support for /proc/bus/pci/X/Y mmap()s.
 *  -- paulus.
 */

pub unsafe fn pci_iobar_pfn(
    pdev: *mut pci_dev,
    bar: i32,
    vma: *mut vm_area_struct,
) -> i32 {
    let pci_ctrl = (*pdev).sysdata as *mut pci_controller;
    let mut ioaddr = pci_resource_start(pdev, bar);

    if pci_ctrl.is_null() {
        return -EINVAL; /* should never happen */
    }

    /* Convert to an offset within this PCI controller */
    ioaddr = ioaddr.wrapping_sub((*pci_ctrl).io_space.base as usize);

    (*vma).vm_pgoff = (*vma).vm_pgoff.wrapping_add(
        (ioaddr.wrapping_add((*pci_ctrl).io_space.start)) >> PAGE_SHIFT,
    );
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
