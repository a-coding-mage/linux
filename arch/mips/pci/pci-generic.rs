// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 *
 * pcibios_align_resource taken from arch/arm/kernel/bios32.c.
 */

/* Dependency declarations supplied by the Linux PCI environment are external. */

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
    let mut start: resource_size_t = (*res).start;
    let host_bridge: *mut pci_host_bridge;

    if ((*res).flags & IORESOURCE_IO) != 0 && (start & 0x300) != 0 {
        start = (start.wrapping_add(0x3ff)) & !0x3ff;
    }

    host_bridge = pci_find_host_bridge((*dev).bus);

    if !(*host_bridge).align_resource.is_none() {
        return ((*host_bridge).align_resource.unwrap())(dev, res, start, size, align);
    }

    if ((*res).flags & IORESOURCE_MEM) != 0 {
        return pci_align_resource(dev, res, empty_res, size, align);
    }

    start
}

pub unsafe fn pcibios_fixup_bus(bus: *mut pci_bus) {
    pci_read_bridge_bases(bus);
}

/* Preserved from the C conditional: enabled only when pci_remap_iospace is defined. */
#[cfg(pci_remap_iospace)]
pub unsafe fn pci_remap_iospace(res: *const resource, phys_addr: phys_addr_t) -> i32 {
    let vaddr: usize;

    if (*res).start != 0 {
        WARN_ONCE!(true, "resource start address is not zero\n");
        return -ENODEV;
    }

    vaddr = ioremap(phys_addr, resource_size(res)) as usize;
    set_io_port_base(vaddr as _);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
