// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * pci.c -- basic PCI support code
 *
 * (C) Copyright 2011, Greg Ungerer <gerg@uclinux.org>
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * From arch/i386/kernel/pci-i386.c:
 *
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
 * which might be mirrored at 0x0100-0x03ff..
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

    if (((*res).flags & IORESOURCE_IO) != 0) && ((start & 0x300) != 0) {
        start = (start.wrapping_add(0x3ff)) & !0x3ff;
    }

    if ((*res).flags & IORESOURCE_MEM) != 0 {
        return pci_align_resource(dev, res, empty_res, size, align);
    }

    start
}

/*
 * This is taken from the ARM code for this.
 */
pub unsafe fn pcibios_enable_device(dev: *mut pci_dev, mask: i32) -> i32 {
    let mut cmd: u16;
    // The C source leaves newcmd uninitialized; preserve that declaration and behavior.
    let mut newcmd: u16;
    let ret = pci_enable_resources(dev, mask);
    if ret < 0 {
        return ret;
    }

    /*
     * Bridges (eg, cardbus bridges) need to be fully enabled
     */
    if ((*dev).class >> 16) == PCI_BASE_CLASS_BRIDGE {
        pci_read_config_word(dev, PCI_COMMAND, &mut cmd);
        newcmd |= PCI_COMMAND_IO | PCI_COMMAND_MEMORY;
        if newcmd != cmd {
            pr_info!(
                "PCI: enabling bridge %s (0x%04x -> 0x%04x)\n",
                pci_name(dev),
                cmd,
                newcmd
            );
            pci_write_config_word(dev, PCI_COMMAND, newcmd);
        }
    }
    0
}

pub unsafe fn pcibios_fixup_bus(bus: *mut pci_bus) {
    let mut dev: *mut pci_dev;

    list_for_each_entry!(dev, &mut (*bus).devices, bus_list, {
        pci_write_config_byte(dev, PCI_CACHE_LINE_SIZE, 8);
        pci_write_config_byte(dev, PCI_LATENCY_TIMER, 32);
    });
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
