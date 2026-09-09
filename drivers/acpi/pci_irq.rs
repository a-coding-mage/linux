// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  pci_irq.c - ACPI PCI Interrupt Routing ($Revision: 11 $)
 *
 *  Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
 *  Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 *  Copyright (C) 2002       Dominik Brodowski <devel@brodo.de>
 *  (c) Copyright 2008 Hewlett-Packard Development Company, L.P.
 *	Bjorn Helgaas <bjorn.helgaas@hp.com>
 */

// C includes and preprocessor configuration are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct acpi_prt_entry {
    pub id: acpi_pci_id,
    pub pin: u8,
    pub link: acpi_handle,
    pub index: u32, // GSI, or link _CRS index
}

#[inline]
unsafe fn pin_name(pin: i32) -> u8 { b'A'.wrapping_add((pin - 1) as u8) }

// DMI quirk tables are represented using the native kernel types supplied elsewhere.
static medion_md9580: [dmi_system_id; 2] = [
    dmi_system_id { ident: "Medion MD9580-F laptop", matches: [DMI_MATCH!(DMI_SYS_VENDOR, "MEDIONNB"), DMI_MATCH!(DMI_PRODUCT_NAME, "A555")] },
    dmi_system_id::default(),
];
static dell_optiplex: [dmi_system_id; 2] = [
    dmi_system_id { ident: "Dell Optiplex GX1", matches: [DMI_MATCH!(DMI_SYS_VENDOR, "Dell Computer Corporation"), DMI_MATCH!(DMI_PRODUCT_NAME, "OptiPlex GX1 600S+")] },
    dmi_system_id::default(),
];
static hp_t5710: [dmi_system_id; 2] = [
    dmi_system_id { ident: "HP t5710", matches: [DMI_MATCH!(DMI_SYS_VENDOR, "Hewlett-Packard"), DMI_MATCH!(DMI_PRODUCT_NAME, "hp t5000 series"), DMI_MATCH!(DMI_BOARD_NAME, "098Ch")] },
    dmi_system_id::default(),
];

#[repr(C)]
struct prt_quirk {
    system: *const dmi_system_id,
    segment: u32,
    bus: u32,
    device: u32,
    pin: u8,
    source: *const u8,
    actual_source: *const u8,
}

const fn pci_intx_pin(c: u8) -> u8 { c - b'A' + 1 }

static prt_quirks: [prt_quirk; 3] = [
    prt_quirk { system: medion_md9580.as_ptr(), segment: 0, bus: 0, device: 9, pin: pci_intx_pin(b'A'), source: c"\\_SB_.PCI0.ISA_.LNKA".as_ptr(), actual_source: c"\\_SB_.PCI0.ISA_.LNKB".as_ptr() },
    prt_quirk { system: dell_optiplex.as_ptr(), segment: 0, bus: 0, device: 0xd, pin: pci_intx_pin(b'A'), source: c"\\_SB_.LNKB".as_ptr(), actual_source: c"\\_SB_.LNKA".as_ptr() },
    prt_quirk { system: hp_t5710.as_ptr(), segment: 0, bus: 0, device: 1, pin: pci_intx_pin(b'A'), source: c"\\_SB_.PCI0.LNK1".as_ptr(), actual_source: c"\\_SB_.PCI0.LNK3".as_ptr() },
];

unsafe fn do_prt_fixups(entry: *mut acpi_prt_entry, prt: *mut acpi_pci_routing_table) {
    for quirk in &prt_quirks {
        if dmi_check_system(quirk.system) && (*entry).id.segment == quirk.segment as _ && (*entry).id.bus == quirk.bus as _ && (*entry).id.device == quirk.device as _ && (*entry).pin == quirk.pin && !strcmp((*prt).source, quirk.source) && strlen((*prt).source) >= strlen(quirk.actual_source) {
            pr_warn!("Firmware reports %04x:%02x:%02x PCI INT %c connected to %s; changing to %s\n", (*entry).id.segment, (*entry).id.bus, (*entry).id.device, pin_name((*entry).pin as i32), (*prt).source, quirk.actual_source);
            strcpy((*prt).source, quirk.actual_source);
        }
    }
}

unsafe fn acpi_pci_irq_check_entry(handle: acpi_handle, dev: *mut pci_dev, pin: i32, prt: *mut acpi_pci_routing_table, entry_ptr: *mut *mut acpi_prt_entry) -> i32 {
    let segment = pci_domain_nr((*dev).bus);
    let bus = (*(*dev).bus).number;
    let device = if pci_ari_enabled((*dev).bus) { 0 } else { PCI_SLOT((*dev).devfn) };
    if (((*prt).address >> 16) & 0xffff) != device || (*prt).pin as i32 + 1 != pin { return -ENODEV; }
    let entry = kzalloc_obj::<acpi_prt_entry>();
    if entry.is_null() { return -ENOMEM; }
    (*entry).id.segment = segment;
    (*entry).id.bus = bus;
    (*entry).id.device = ((*prt).address >> 16) & 0xffff;
    (*entry).pin = (*prt).pin + 1;
    do_prt_fixups(entry, prt);
    (*entry).index = (*prt).source_index;
    if (*prt).source[0] != 0 { acpi_get_handle(handle, (*prt).source, &mut (*entry).link); }
    pr_debug!("%04x:%02x:%02x[%c] -> %s[%u]\n", (*entry).id.segment, (*entry).id.bus, (*entry).id.device, pin_name((*entry).pin as i32), (*prt).source, (*entry).index);
    *entry_ptr = entry;
    0
}

unsafe fn acpi_pci_irq_find_prt_entry(dev: *mut pci_dev, pin: i32, entry_ptr: *mut *mut acpi_prt_entry) -> i32 {
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };
    let mut handle: acpi_handle = core::ptr::null_mut();
    if !(*(*dev).bus).bridge.is_null() { handle = ACPI_HANDLE((*(*dev).bus).bridge); }
    if handle.is_null() { return -ENODEV; }
    let status = acpi_get_irq_routing_table(handle, &mut buffer);
    if ACPI_FAILURE(status) { kfree(buffer.pointer); return -ENODEV; }
    let mut entry = buffer.pointer as *mut acpi_pci_routing_table;
    while !entry.is_null() && (*entry).length > 0 {
        if acpi_pci_irq_check_entry(handle, dev, pin, entry, entry_ptr) == 0 { break; }
        entry = ((entry as usize).wrapping_add((*entry).length as usize)) as *mut acpi_pci_routing_table;
    }
    kfree(buffer.pointer);
    0
}

#[cfg(CONFIG_X86_IO_APIC)]
unsafe fn bridge_has_boot_interrupt_variant(mut bus: *mut pci_bus) -> i32 {
    while !bus.is_null() {
        if (*bus).self_.is_null() { return 0; }
        if (*(*bus).self_).irq_reroute_variant != 0 { return (*(*bus).self_).irq_reroute_variant; }
        bus = (*bus).parent;
    }
    0
}

#[cfg(CONFIG_X86_IO_APIC)]
unsafe fn acpi_reroute_boot_interrupt(dev: *mut pci_dev, entry: *mut acpi_prt_entry) -> i32 {
    if noioapicquirk != 0 || noioapicreroute != 0 { return 0; }
    match bridge_has_boot_interrupt_variant((*dev).bus) {
        0 => 0,
        INTEL_IRQ_REROUTE_VARIANT => { dev_info!(&(*dev).dev, "PCI IRQ {} -> rerouted to legacy IRQ {}\n", (*entry).index, ((*entry).index % 4) + 16); (*entry).index = ((*entry).index % 4) + 16; 1 },
        _ => { dev_warn!(&(*dev).dev, "Cannot reroute IRQ {} to legacy IRQ: unknown mapping\n", (*entry).index); -1 }
    }
}

pub unsafe fn acpi_pci_irq_lookup(dev: *mut pci_dev, mut pin: i32) -> *mut acpi_prt_entry {
    let mut entry: *mut acpi_prt_entry = core::ptr::null_mut();
    let bridge_pin: u8;
    let orig_pin = pin;
    let mut ret = acpi_pci_irq_find_prt_entry(dev, pin, &mut entry);
    if ret == 0 && !entry.is_null() {
        #[cfg(CONFIG_X86_IO_APIC)] { acpi_reroute_boot_interrupt(dev, entry); }
        dev_dbg!(&(*dev).dev, "Found [{}] _PRT entry\n", pin_name(pin));
        return entry;
    }
    let mut bridge = (*(*dev).bus).self_;
    while !bridge.is_null() {
        pin = pci_swizzle_interrupt_pin(dev, pin);
        if ((*bridge).class >> 8) == PCI_CLASS_BRIDGE_CARDBUS {
            bridge_pin = (*bridge).pin;
            if bridge_pin == 0 { dev_dbg!(&(*bridge).dev, "No interrupt pin configured\n"); return core::ptr::null_mut(); }
            pin = bridge_pin as i32;
        }
        ret = acpi_pci_irq_find_prt_entry(bridge, pin, &mut entry);
        if ret == 0 && !entry.is_null() { dev_dbg!(&(*dev).dev, "Derived GSI INT {} from {}\n", pin_name(orig_pin), pci_name(bridge)); return entry; }
        dev = bridge;
        bridge = (*(*dev).bus).self_;
    }
    dev_warn!(&(*dev).dev, "can't derive routing for PCI INT {}\n", pin_name(orig_pin));
    core::ptr::null_mut()
}

#[cfg(any(CONFIG_ISA, CONFIG_EISA))]
unsafe fn acpi_isa_register_gsi(dev: *mut pci_dev) -> i32 {
    let mut dev_gsi = 0u32;
    if (*dev).irq > 0 && (*dev).irq <= 0xf && acpi_isa_irq_available((*dev).irq) && acpi_isa_irq_to_gsi((*dev).irq, &mut dev_gsi) == 0 {
        dev_warn!(&(*dev).dev, "PCI INT {}: no GSI - using ISA IRQ {}\n", pin_name((*dev).pin as i32), (*dev).irq);
        acpi_register_gsi(&mut (*dev).dev, dev_gsi, ACPI_LEVEL_SENSITIVE, ACPI_ACTIVE_LOW); return 0;
    }
    -EINVAL
}
#[cfg(not(any(CONFIG_ISA, CONFIG_EISA)))]
unsafe fn acpi_isa_register_gsi(_dev: *mut pci_dev) -> i32 { -ENODEV }

unsafe fn acpi_pci_irq_valid(dev: *mut pci_dev, pin: u8) -> bool {
    #[cfg(CONFIG_X86)] if (*dev).irq == 0xff { (*dev).irq = IRQ_NOTCONNECTED; dev_warn!(&(*dev).dev, "PCI INT {}: not connected\n", pin_name(pin as i32)); return false; }
    true
}

pub unsafe fn acpi_pci_irq_enable(dev: *mut pci_dev) -> i32 {
    let pin = (*dev).pin;
    if pin == 0 { dev_dbg!(&(*dev).dev, "No interrupt pin configured\n"); return 0; }
    if (*dev).irq_managed && (*dev).irq > 0 { return 0; }
    let entry = acpi_pci_irq_lookup(dev, pin as i32);
    if entry.is_null() && ((*dev).class >> 8) == PCI_CLASS_STORAGE_IDE && ((*dev).class & 0x05) == 0 { return 0; }
    let mut triggering = ACPI_LEVEL_SENSITIVE;
    let mut polarity = if acpi_irq_model == ACPI_IRQ_MODEL_GIC || acpi_irq_model == ACPI_IRQ_MODEL_LPIC { ACPI_ACTIVE_HIGH } else { ACPI_ACTIVE_LOW };
    let mut gsi = 0u32;
    let mut link: *mut i8 = core::ptr::null_mut();
    let mut rc = -ENODEV;
    if !entry.is_null() { if !(*entry).link.is_null() { rc = acpi_pci_link_allocate_irq((*entry).link, (*entry).index, &mut triggering, &mut polarity, &mut link, &mut gsi); } else { gsi = (*entry).index; rc = 0; } }
    if rc < 0 { if !acpi_pci_irq_valid(dev, pin) { kfree(entry as *mut core::ffi::c_void); return 0; } acpi_isa_register_gsi(dev); kfree(entry as *mut core::ffi::c_void); return 0; }
    rc = acpi_register_gsi(&mut (*dev).dev, gsi, triggering, polarity);
    if rc < 0 { dev_warn!(&(*dev).dev, "PCI INT {}: failed to register GSI\n", pin_name(pin as i32)); kfree(entry as *mut core::ffi::c_void); return rc; }
    (*dev).irq = rc; (*dev).irq_managed = true;
    kfree(entry as *mut core::ffi::c_void); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
