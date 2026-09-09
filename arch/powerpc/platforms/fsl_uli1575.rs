// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ULI M1575 setup code - specific to Freescale boards
 *
 * Copyright 2007 Freescale Semiconductor Inc.
 */

// Kernel and architecture dependencies are supplied by the surrounding translation unit.

const ULI_PIRQA: u8 = 0x08;
const ULI_PIRQB: u8 = 0x09;
const ULI_PIRQC: u8 = 0x0a;
const ULI_PIRQD: u8 = 0x0b;
const ULI_PIRQE: u8 = 0x0c;
const ULI_PIRQF: u8 = 0x0d;
const ULI_PIRQG: u8 = 0x0e;

const ULI_8259_NONE: u8 = 0x00;
const ULI_8259_IRQ1: u8 = 0x08;
const ULI_8259_IRQ3: u8 = 0x02;
const ULI_8259_IRQ4: u8 = 0x04;
const ULI_8259_IRQ5: u8 = 0x05;
const ULI_8259_IRQ6: u8 = 0x07;
const ULI_8259_IRQ7: u8 = 0x06;
const ULI_8259_IRQ9: u8 = 0x01;
const ULI_8259_IRQ10: u8 = 0x03;
const ULI_8259_IRQ11: u8 = 0x09;
const ULI_8259_IRQ12: u8 = 0x0b;
const ULI_8259_IRQ14: u8 = 0x0d;
const ULI_8259_IRQ15: u8 = 0x0f;

static mut uli_pirq_to_irq: [u8; 8] = [
    ULI_8259_IRQ9, ULI_8259_IRQ10, ULI_8259_IRQ11, ULI_8259_IRQ12,
    ULI_8259_IRQ5, ULI_8259_IRQ6, ULI_8259_IRQ7, ULI_8259_NONE,
];

#[inline]
unsafe fn is_quirk_valid() -> bool {
    machine_is(mpc86xx_hpcn) || machine_is(mpc8544_ds) || machine_is(p2020_ds) || machine_is(mpc8572_ds)
}

unsafe fn early_uli5249(dev: *mut pci_dev) {
    if !is_quirk_valid() { return; }
    pci_write_config_word(dev, PCI_COMMAND, PCI_COMMAND_IO | PCI_COMMAND_MEMORY | PCI_COMMAND_MASTER);
    let mut temp: u8 = 0;
    pci_read_config_byte(dev, 0x7c, &mut temp);
    pci_write_config_byte(dev, 0x7c, 0x80);
    pci_write_config_byte(dev, PCI_CLASS_PROG, 0x01);
    (*dev).class |= 0x1;
    pci_write_config_byte(dev, 0x7c, temp);
}

unsafe fn quirk_uli1575(dev: *mut pci_dev) {
    if !is_quirk_valid() { return; }
    for i in 0..4 {
        let val = uli_pirq_to_irq[i * 2] | (uli_pirq_to_irq[i * 2 + 1] << 4);
        pci_write_config_byte(dev, 0x48 + i, val);
    }
    pci_write_config_byte(dev, 0x86, ULI_PIRQD);
    pci_write_config_byte(dev, 0x87, ULI_PIRQA);
    pci_write_config_byte(dev, 0x88, ULI_PIRQB);
    pci_write_config_byte(dev, 0x89, ULI_PIRQF);
    pci_write_config_byte(dev, 0x8a, ULI_PIRQF);
    pci_write_config_byte(dev, 0x8b, ULI_PIRQF);
    pci_write_config_byte(dev, 0x8c, ULI_PIRQF);
    pci_write_config_byte(dev, 0x8d, ULI_PIRQE);
    pci_write_config_byte(dev, 0x8e, ULI_PIRQG);
    pci_write_config_byte(dev, 0x8f, ULI_PIRQG);
    pci_write_config_byte(dev, 0x74, ULI_8259_IRQ11);
    pci_write_config_byte(dev, 0x44, 0x30 | ULI_8259_IRQ14);
    pci_write_config_byte(dev, 0x75, ULI_8259_IRQ15);
}

unsafe fn quirk_final_uli1575(dev: *mut pci_dev) {
    if !is_quirk_valid() { return; }
    outb(0xfa, 0x4d0); outb(0x1e, 0x4d1);
    CMOS_WRITE(RTC_SET, RTC_CONTROL); CMOS_WRITE(RTC_24H, RTC_CONTROL);
    CMOS_WRITE(0, RTC_VALID);
    outb_p(0x7c, 0x72); outb_p(RTC_ALARM_DONT_CARE, 0x73);
    outb_p(0x7d, 0x72); outb_p(RTC_ALARM_DONT_CARE, 0x73);
}

unsafe fn quirk_uli5288(dev: *mut pci_dev) {
    if !is_quirk_valid() { return; }
    let mut c = 0u8; let mut d = 0u32;
    pci_read_config_byte(dev, 0x83, &mut c); pci_write_config_byte(dev, 0x83, c | 0x80);
    pci_read_config_dword(dev, PCI_CLASS_REVISION, &mut d);
    d = (d & 0xff) | (PCI_CLASS_STORAGE_SATA_AHCI << 8);
    pci_write_config_dword(dev, PCI_CLASS_REVISION, d); pci_write_config_byte(dev, 0x83, c);
    pci_read_config_byte(dev, 0x84, &mut c); pci_write_config_byte(dev, 0x84, c & !0x01);
}

unsafe fn quirk_uli5229(dev: *mut pci_dev) {
    if !is_quirk_valid() { return; }
    pci_write_config_word(dev, PCI_COMMAND, PCI_COMMAND_INTX_DISABLE | PCI_COMMAND_MASTER | PCI_COMMAND_IO);
    let mut temp = 0u16; pci_read_config_word(dev, 0x4a, &mut temp);
    pci_write_config_word(dev, 0x4a, temp | 0x1000);
}

unsafe fn quirk_final_uli5249(dev: *mut pci_dev) {
    let bus = (*dev).bus; let mut end: resource_size_t = 0; let mut dummy: *mut u8;
    for i in PCI_BRIDGE_RESOURCES..PCI_BRIDGE_RESOURCES + 3 {
        let flags = pci_resource_flags(dev, i);
        if (flags & (IORESOURCE_MEM | IORESOURCE_PREFETCH)) == IORESOURCE_MEM { end = pci_resource_end(dev, i); }
    }
    pci_bus_for_each_resource!(bus, res, i, {
        if !res.is_null() && (*res).flags & IORESOURCE_MEM != 0 {
            dummy = if (*res).end == end { ioremap((*res).start, 0x4) } else { ioremap((*res).end - 3, 0x4) };
            if !dummy.is_null() { in_8(dummy); iounmap(dummy); }
            break;
        }
    });
}

unsafe fn hpcd_quirk_uli1575(dev: *mut pci_dev) { if !machine_is(mpc86xx_hpcd) { return; } let mut v=0; pci_read_config_dword(dev,0x48,&mut v); pci_write_config_dword(dev,0x48,v|(1<<26)); pci_read_config_dword(dev,0x90,&mut v); pci_write_config_dword(dev,0x90,v|(1<<22)); }
unsafe fn hpcd_quirk_uli5288(dev: *mut pci_dev) { if !machine_is(mpc86xx_hpcd){return;} let mut c=0; pci_read_config_byte(dev,0x83,&mut c); pci_write_config_byte(dev,0x83,c|0x80); pci_write_config_byte(dev,PCI_CLASS_PROG,1); pci_write_config_byte(dev,PCI_CLASS_DEVICE,6); pci_read_config_byte(dev,0x83,&mut c); pci_write_config_byte(dev,0x83,c&0x7f); }
unsafe fn hpcd_quirk_uli5229(dev: *mut pci_dev) { if !machine_is(mpc86xx_hpcd){return;} let mut c=0; pci_read_config_byte(dev,0x4b,&mut c); pci_write_config_byte(dev,0x4b,c|0x10); }

unsafe fn hpcd_final_uli5288(dev: *mut pci_dev) {
    let hose = pci_bus_to_host((*dev).bus); let hosenode = if !hose.is_null() { (*hose).dn } else { core::ptr::null_mut() };
    if !machine_is(mpc86xx_hpcd) || hosenode.is_null() { return; }
    let mut oirq = of_phandle_args { np: hosenode, args: [0; 16], args_count: 0 };
    oirq.args[0]=2; oirq.args_count=1; let laddr=[((*hose).first_busno << 16) | (PCI_DEVFN(31,0)<<8),0,0];
    of_irq_parse_raw(laddr.as_ptr(), &mut oirq); (*dev).irq=irq_create_of_mapping(&oirq);
}

unsafe fn uli_exclude_device(hose: *mut pci_controller, bus: u8, devfn: u8) -> i32 {
    if (*hose).dn == fsl_pci_primary && bus == (*hose).first_busno + 2 && ((PCI_SLOT(devfn)==29 && PCI_FUNC(devfn)==1) || (PCI_SLOT(devfn)==29 && PCI_FUNC(devfn)==2)) { return PCIBIOS_DEVICE_NOT_FOUND; }
    PCIBIOS_SUCCESSFUL
}

unsafe fn uli_init() {
    let mut node = of_find_node_by_name(core::ptr::null_mut(), "uli1575");
    while { let parent = of_get_parent(node); if parent.is_null() { false } else { of_node_put(node); node=parent; if parent==fsl_pci_primary { ppc_md.pci_exclude_device=Some(uli_exclude_device); false } else { true } } } {}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
