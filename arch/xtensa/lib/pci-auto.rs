// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/xtensa/lib/pci-auto.c
 *
 * PCI autoconfiguration library
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 *
 * Chris Zankel <zankel@tensilica.com, cez@zankel.net>
 *
 * Based on work from Matt Porter <mporter@mvista.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

static mut pciauto_upper_iospc: i32 = 0;
static mut pciauto_upper_memspc: i32 = 0;

static mut pciauto_dev: pci_dev = pci_dev::default();
static mut pciauto_bus: pci_bus = pci_bus::default();

/* Helper functions */

/* Initialize the bars of a PCI device. */
unsafe fn pciauto_setup_bars(dev: *mut pci_dev, bar_limit: i32) {
    let mut bar_size: i32;
    let mut bar: i32 = PCI_BASE_ADDRESS_0;
    let mut bar_nr: i32 = 0;
    let mut upper_limit: *mut i32;
    let mut found_mem64 = 0;

    while bar <= bar_limit {
        /* Tickle the BAR and get the size */
        pci_write_config_dword(dev, bar, 0xffffffff);
        pci_read_config_dword(dev, bar, &mut bar_size);

        /* If BAR is not implemented go to the next BAR */
        if bar_size == 0 {
            bar += 4;
            bar_nr += 1;
            continue;
        }

        /* Check the BAR type and set our address mask */
        if bar_size & PCI_BASE_ADDRESS_SPACE_IO != 0 {
            bar_size &= PCI_BASE_ADDRESS_IO_MASK;
            upper_limit = &raw mut pciauto_upper_iospc;
            pr_debug!("PCI Autoconfig: BAR {}, I/O, ", bar_nr);
        } else {
            if (bar_size & PCI_BASE_ADDRESS_MEM_TYPE_MASK) == PCI_BASE_ADDRESS_MEM_TYPE_64 {
                found_mem64 = 1;
            }
            bar_size &= PCI_BASE_ADDRESS_MEM_MASK;
            upper_limit = &raw mut pciauto_upper_memspc;
            pr_debug!("PCI Autoconfig: BAR {}, Mem, ", bar_nr);
        }

        /* Allocate a base address (bar_size is negative!) */
        *upper_limit = (*upper_limit + bar_size) & bar_size;

        /* Write it out and update our limit */
        pci_write_config_dword(dev, bar, *upper_limit);

        /* If we are a 64-bit decoder, locate it in the lower 4GB. */
        if found_mem64 != 0 {
            bar += 4;
            pci_write_config_dword(dev, bar, 0x00000000);
        }

        pr_debug!("size=0x{:x}, address=0x{:x}\n", !bar_size + 1, *upper_limit);
        bar += 4;
        bar_nr += 1;
    }
}

/* Initialize the interrupt number. */
unsafe fn pciauto_setup_irq(pci_ctrl: *mut pci_controller, dev: *mut pci_dev, devfn: i32) {
    let mut pin: u8 = 0;
    let mut irq: i32 = 0;

    pci_read_config_byte(dev, PCI_INTERRUPT_PIN, &mut pin);
    if pin == 0 || pin > 4 { pin = 1; }
    if !(*pci_ctrl).map_irq.is_null() {
        irq = ((*pci_ctrl).map_irq)(dev, PCI_SLOT(devfn), pin);
    }
    if irq == -1 { irq = 0; }
    pr_debug!("PCI Autoconfig: Interrupt {}, pin {}\n", irq, pin);
    pci_write_config_byte(dev, PCI_INTERRUPT_LINE, irq as u8);
}

unsafe fn pciauto_prescan_setup_bridge(dev: *mut pci_dev, current_bus: i32, sub_bus: i32,
                                       iosave: *mut i32, memsave: *mut i32) {
    pci_write_config_byte(dev, PCI_PRIMARY_BUS, current_bus as u8);
    pci_write_config_byte(dev, PCI_SECONDARY_BUS, (sub_bus + 1) as u8);
    pci_write_config_byte(dev, PCI_SUBORDINATE_BUS, 0xff);
    pciauto_upper_memspc &= !(0x100000 - 1);
    *memsave = pciauto_upper_memspc;
    pciauto_upper_iospc &= !(0x1000 - 1);
    *iosave = pciauto_upper_iospc;
    pci_write_config_word(dev, PCI_MEMORY_LIMIT,
        ((pciauto_upper_memspc - 1) & 0xfff00000) >> 16);
    pci_write_config_byte(dev, PCI_IO_LIMIT,
        (((pciauto_upper_iospc - 1) & 0x0000f000) >> 8) as u8);
    pci_write_config_word(dev, PCI_IO_LIMIT_UPPER16,
        ((pciauto_upper_iospc - 1) & 0xffff0000) >> 16);
}

unsafe fn pciauto_postscan_setup_bridge(dev: *mut pci_dev, _current_bus: i32, sub_bus: i32,
                                        iosave: *mut i32, memsave: *mut i32) {
    let mut cmdstat: i32 = 0;
    pci_write_config_byte(dev, PCI_SUBORDINATE_BUS, sub_bus as u8);
    pciauto_upper_memspc &= !(0x100000 - 1);
    if *memsave == pciauto_upper_memspc { pciauto_upper_memspc -= 0x00100000; }
    pci_write_config_word(dev, PCI_MEMORY_BASE, pciauto_upper_memspc >> 16);
    pci_write_config_word(dev, PCI_PREF_MEMORY_LIMIT,
        ((pciauto_upper_memspc - 1) & 0xfff00000) >> 16);
    pciauto_upper_memspc -= 0x100000;
    pci_write_config_word(dev, PCI_PREF_MEMORY_BASE, pciauto_upper_memspc >> 16);
    pciauto_upper_iospc &= !(0x1000 - 1);
    if *iosave == pciauto_upper_iospc { pciauto_upper_iospc -= 0x1000; }
    pci_write_config_byte(dev, PCI_IO_BASE, ((pciauto_upper_iospc & 0x0000f000) >> 8) as u8);
    pci_write_config_word(dev, PCI_IO_BASE_UPPER16, pciauto_upper_iospc >> 16);
    pci_read_config_dword(dev, PCI_COMMAND, &mut cmdstat);
    pci_write_config_dword(dev, PCI_COMMAND, cmdstat | PCI_COMMAND_IO | PCI_COMMAND_MEMORY | PCI_COMMAND_MASTER);
}

pub unsafe fn pciauto_bus_scan(pci_ctrl: *mut pci_controller, current_bus: i32) -> i32 {
    let mut sub_bus = current_bus;
    let mut found_multi = false;
    (*pciauto_dev).bus = &raw mut pciauto_bus;
    (*pciauto_dev).sysdata = pci_ctrl;
    (*pciauto_bus).ops = (*pci_ctrl).ops;

    if current_bus == (*pci_ctrl).first_busno {
        pciauto_upper_iospc = (*pci_ctrl).io_resource.end + 1;
        pciauto_upper_memspc = (*pci_ctrl).mem_resources[0].end + 1;
    }

    let mut pci_devfn = 0;
    while pci_devfn < 0xff {
        if current_bus == (*pci_ctrl).first_busno && pci_devfn == 0 { pci_devfn += 1; continue; }
        if PCI_FUNC(pci_devfn) != 0 && !found_multi { pci_devfn += 1; continue; }
        (*pciauto_bus).number = current_bus;
        (*pciauto_dev).devfn = pci_devfn;
        let mut header_type = 0u8;
        if pci_read_config_byte(&raw mut pciauto_dev, PCI_HEADER_TYPE, &mut header_type) != 0 {
            pci_devfn += 1; continue;
        }
        if PCI_FUNC(pci_devfn) == 0 { found_multi = FIELD_GET(PCI_HEADER_TYPE_MFD, header_type) != 0; }
        let mut vid = 0u16;
        pci_read_config_word(&raw mut pciauto_dev, PCI_VENDOR_ID, &mut vid);
        if vid == 0xffff || vid == 0 { found_multi = false; pci_devfn += 1; continue; }
        let mut pci_class = 0i32;
        pci_read_config_dword(&raw mut pciauto_dev, PCI_CLASS_REVISION, &mut pci_class);
        if (pci_class >> 16) == PCI_CLASS_BRIDGE_PCI {
            let mut iosave = 0; let mut memsave = 0;
            pr_debug!("PCI Autoconfig: Found P2P bridge, device {}\n", PCI_SLOT(pci_devfn));
            pciauto_setup_bars(&raw mut pciauto_dev, PCI_BASE_ADDRESS_1);
            pciauto_prescan_setup_bridge(&raw mut pciauto_dev, current_bus, sub_bus, &mut iosave, &mut memsave);
            sub_bus = pciauto_bus_scan(pci_ctrl, sub_bus + 1);
            pciauto_postscan_setup_bridge(&raw mut pciauto_dev, current_bus, sub_bus, &mut iosave, &mut memsave);
            (*pciauto_bus).number = current_bus;
            pci_devfn += 1; continue;
        }
        let mut cmdstat = 0i32;
        pci_read_config_dword(&raw mut pciauto_dev, PCI_COMMAND, &mut cmdstat);
        pci_write_config_dword(&raw mut pciauto_dev, PCI_COMMAND, cmdstat | PCI_COMMAND_IO | PCI_COMMAND_MEMORY | PCI_COMMAND_MASTER);
        pci_write_config_byte(&raw mut pciauto_dev, PCI_LATENCY_TIMER, 0x80);
        pr_debug!("PCI Autoconfig: Found Bus {}, Device {}, Function {}\n", current_bus, PCI_SLOT(pci_devfn), PCI_FUNC(pci_devfn));
        pciauto_setup_bars(&raw mut pciauto_dev, PCI_BASE_ADDRESS_5);
        pciauto_setup_irq(pci_ctrl, &raw mut pciauto_dev, pci_devfn);
        pci_devfn += 1;
    }
    sub_bus
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
