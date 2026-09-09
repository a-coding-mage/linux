/*
 * PCIMT specific code
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 97, 98, 2000, 03, 04 Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 2006,2007 Thomas Bogendoerfer (tsbogend@alpha.franken.de)
 */

// Linux and MIPS declarations supplied by the surrounding translation unit.

const PORT_3F8: u64 = 0x3f8;
const PORT_2F8: u64 = 0x2f8;

unsafe fn sni_pcimt_sc_init() {
    let scsiz: u32 = core::ptr::read_volatile(PCIMT_CACHECONF as *const u32) & 7;
    if scsiz == 0 {
        printk(c"Second level cache is deactivated.\n".as_ptr());
        return;
    }
    if scsiz >= 6 {
        printk(c"Invalid second level cache size configured, deactivating second level cache.\n".as_ptr());
        core::ptr::write_volatile(PCIMT_CACHECONF as *mut u32, 0);
        return;
    }
    let sc_size = 128u32 << scsiz;
    printk(c"%dkb second level cache detected, deactivating.\n".as_ptr(), sc_size);
    core::ptr::write_volatile(PCIMT_CACHECONF as *mut u32, 0);
}

/* A bit more gossip about the iron we're running on ... */
unsafe fn sni_pcimt_detect() {
    let mut boardtype = [0i8; 80];
    let csmsr = core::ptr::read_volatile(PCIMT_CSMSR as *const u8);
    let mut p = boardtype.as_mut_ptr();
    p = p.add(sprintf(p, c"%s PCI".as_ptr(), if csmsr & 0x80 != 0 { c"RM200".as_ptr() } else { c"RM300".as_ptr() }));
    if csmsr & 0x80 == 0 {
        p = p.add(sprintf(p, c", board revision %s".as_ptr(), if csmsr & 0x20 != 0 { c"D".as_ptr() } else { c"C".as_ptr() }));
    }
    let asic = if csmsr & 0x08 != 0 { csmsr & 0x80 } else { if csmsr & 0x80 != 0 { 0 } else { 1 } };
    p = p.add(sprintf(p, c", ASIC PCI Rev %s".as_ptr(), if asic != 0 { c"1.0".as_ptr() } else { c"1.1".as_ptr() }));
    printk(c"%s.\n".as_ptr(), boardtype.as_ptr());
}

static mut pcimt_data: [plat_serial8250_port; 3] = [
    plat_serial8250_port { iobase: 0x3f8, irq: 4, uartclk: 1843200, iotype: UPIO_PORT, flags: UPF_BOOT_AUTOCONF, ..unsafe { core::mem::zeroed() } },
    plat_serial8250_port { iobase: 0x2f8, irq: 3, uartclk: 1843200, iotype: UPIO_PORT, flags: UPF_BOOT_AUTOCONF, ..unsafe { core::mem::zeroed() } },
    unsafe { core::mem::zeroed() },
];

static mut pcimt_serial8250_device: platform_device = platform_device { name: c"serial8250".as_ptr(), id: PLAT8250_DEV_PLATFORM, dev: device { platform_data: unsafe { pcimt_data.as_ptr() as *mut _ }, ..unsafe { core::mem::zeroed() } }, ..unsafe { core::mem::zeroed() } };

static mut pcimt_cmos_rsrc: [resource; 2] = [
    resource { start: 0x70, end: 0x71, flags: IORESOURCE_IO, ..unsafe { core::mem::zeroed() } },
    resource { start: 8, end: 8, flags: IORESOURCE_IRQ, ..unsafe { core::mem::zeroed() } },
];
static mut pcimt_cmos_device: platform_device = platform_device { name: c"rtc_cmos".as_ptr(), num_resources: 2, resource: unsafe { pcimt_cmos_rsrc.as_mut_ptr() }, ..unsafe { core::mem::zeroed() } };

static mut sni_io_resource: resource = resource { start: 0x00000000, end: 0x03bfffff, name: c"PCIMT IO MEM".as_ptr(), flags: IORESOURCE_IO, ..unsafe { core::mem::zeroed() } };
static mut pcimt_io_resources: [resource; 6] = [
    resource { start: 0x00, end: 0x1f, name: c"dma1".as_ptr(), flags: IORESOURCE_BUSY, ..unsafe { core::mem::zeroed() } },
    resource { start: 0x40, end: 0x5f, name: c"timer".as_ptr(), flags: IORESOURCE_BUSY, ..unsafe { core::mem::zeroed() } },
    resource { start: 0x60, end: 0x6f, name: c"keyboard".as_ptr(), flags: IORESOURCE_BUSY, ..unsafe { core::mem::zeroed() } },
    resource { start: 0x80, end: 0x8f, name: c"dma page reg".as_ptr(), flags: IORESOURCE_BUSY, ..unsafe { core::mem::zeroed() } },
    resource { start: 0xc0, end: 0xdf, name: c"dma2".as_ptr(), flags: IORESOURCE_BUSY, ..unsafe { core::mem::zeroed() } },
    resource { start: 0xcfc, end: 0xcff, name: c"PCI config data".as_ptr(), flags: IORESOURCE_BUSY, ..unsafe { core::mem::zeroed() } },
];
static mut pcimt_mem_resources: [resource; 1] = [resource { /* this region should only be 4 bytes long, but is 16MB on RM300C */ start: 0x1a000000, end: 0x1affffff, name: c"PCI INT ACK".as_ptr(), flags: IORESOURCE_BUSY, ..unsafe { core::mem::zeroed() } }];
static mut sni_mem_resource: resource = resource { start: 0x18000000, end: 0x1fbfffff, name: c"PCIMT PCI MEM".as_ptr(), flags: IORESOURCE_MEM, ..unsafe { core::mem::zeroed() } };

unsafe fn sni_pcimt_resource_init() {
    for i in 0..pcimt_io_resources.len() { request_resource(&mut sni_io_resource, pcimt_io_resources.as_mut_ptr().add(i)); }
    for i in 0..pcimt_mem_resources.len() { request_resource(&mut sni_mem_resource, pcimt_mem_resources.as_mut_ptr().add(i)); }
}

unsafe fn enable_pcimt_irq(d: *mut irq_data) { let mask = 1u8 << ((*d).irq - PCIMT_IRQ_INT2); core::ptr::write_volatile(PCIMT_IRQSEL as *mut u8, core::ptr::read_volatile(PCIMT_IRQSEL as *const u8) | mask); }
unsafe fn disable_pcimt_irq(d: *mut irq_data) { let mask = !(1u8 << ((*d).irq - PCIMT_IRQ_INT2)); core::ptr::write_volatile(PCIMT_IRQSEL as *mut u8, core::ptr::read_volatile(PCIMT_IRQSEL as *const u8) & mask); }
static mut pcimt_irq_type: irq_chip = irq_chip { name: c"PCIMT".as_ptr(), irq_mask: Some(disable_pcimt_irq), irq_unmask: Some(enable_pcimt_irq), ..unsafe { core::mem::zeroed() } };

unsafe fn pcimt_hwint0() { panic!("Received int0 but no handler yet ..."); }
unsafe fn pcimt_hwint1() { let pend = core::ptr::read_volatile(PCIMT_CSITPEND as *const u8); if pend & IT_EISA != 0 { let irq = i8259_irq(); if irq < 0 { return; } do_IRQ(irq as u32); } if pend & IT_SCSI == 0 { let flags = read_c0_status(); clear_c0_status(ST0_IM); do_IRQ(PCIMT_IRQ_SCSI); write_c0_status(flags); } }
unsafe fn pcimt_hwint3() { let mut pend = core::ptr::read_volatile(PCIMT_CSITPEND as *const u8); pend = !(pend & (IT_INTA|IT_INTB|IT_INTC|IT_INTD)) & 0xff; clear_c0_status(IE_IRQ3); do_IRQ(PCIMT_IRQ_INT2 + ffs(pend) - 1); set_c0_status(IE_IRQ3); }
unsafe fn sni_pcimt_hwint() { let pending = read_c0_cause() & read_c0_status(); if pending & C_IRQ5 != 0 { do_IRQ(MIPS_CPU_IRQ_BASE + 7); } else if pending & C_IRQ4 != 0 { do_IRQ(MIPS_CPU_IRQ_BASE + 6); } else if pending & C_IRQ3 != 0 { pcimt_hwint3(); } else if pending & C_IRQ1 != 0 { pcimt_hwint1(); } else if pending & C_IRQ0 != 0 { pcimt_hwint0(); } }

pub unsafe fn sni_pcimt_irq_init() { core::ptr::write_volatile(PCIMT_IRQSEL as *mut u8, IT_ETH | IT_EISA); mips_cpu_irq_init(); for i in PCIMT_IRQ_INT2..=PCIMT_IRQ_SCSI { irq_set_chip_and_handler(i, &raw mut pcimt_irq_type, handle_level_irq); } sni_hwint = Some(sni_pcimt_hwint); change_c0_status(ST0_IM, IE_IRQ1|IE_IRQ3); }
pub unsafe fn sni_pcimt_init() { sni_pcimt_detect(); sni_pcimt_sc_init(); ioport_resource.end = sni_io_resource.end; sni_pcimt_resource_init(); }
unsafe fn snirm_pcimt_setup_devinit() -> i32 { match sni_brd_type { SNI_BRD_PCI_MTOWER | SNI_BRD_PCI_DESKTOP | SNI_BRD_PCI_MTOWER_CPLUS => { platform_device_register(&raw mut pcimt_serial8250_device); platform_device_register(&raw mut pcimt_cmos_device); }, _ => {} } 0 }

// device_initcall(snirm_pcimt_setup_devinit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
