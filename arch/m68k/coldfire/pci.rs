/*
 * pci.c -- PCI bus support for ColdFire processors
 *
 * (C) Copyright 2012, Greg Ungerer <gerg@uclinux.org>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/types.h, linux/module.h, linux/init.h, linux/kernel.h,
// linux/interrupt.h, linux/irq.h, linux/io.h, linux/pci.h, linux/delay.h,
// asm/coldfire.h, asm/mcfsim.h, asm/m54xxpci.h

static mut rootbus: *mut pci_bus = core::ptr::null_mut();
static mut iospace: c_ulong = 0;

/*
 * We need to be careful probing on bus 0 (directly connected to host
 * bridge). We should only access the well defined possible devices in
 * use, ignore aliases and the like.
 */
static mut mcf_host_slot2sid: [u8; 32] = [
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 1, 2, 0, 3, 4, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
];

static mut mcf_host_irq: [u8; 5] = [0, 69, 69, 71, 71];

/* Configuration space access functions, through the IO mapping window. */
unsafe fn mcf_mk_pcicar(bus: c_int, devfn: c_uint, where_: c_int) -> c_ulong {
	((bus << PCICAR_BUSN) | ((devfn as c_int) << PCICAR_DEVFNN)
		| (where_ & 0xfc)) as c_ulong
}

unsafe fn mcf_pci_readconfig(
	bus: *mut pci_bus, devfn: c_uint, where_: c_int, size: c_int, value: *mut u32,
) -> c_int {
	let mut addr: c_ulong;
	*value = 0xffff_ffff;
	if (*bus).number == 0 && mcf_host_slot2sid[PCI_SLOT(devfn) as usize] == 0 {
		return PCIBIOS_SUCCESSFUL;
	}
	addr = mcf_mk_pcicar((*bus).number, devfn, where_);
	mcf_write32(PCICAR_E as u32 | addr as u32, PCICAR);
	mcf_read32(PCICAR);
	addr = iospace + (where_ as c_ulong & 0x3);
	match size {
		1 => *value = mcf_read8(addr) as u32,
		2 => *value = le16_to_cpu(mcf_read16(addr)) as u32,
		_ => *value = le32_to_cpu(mcf_read32(addr)),
	}
	mcf_write32(0, PCICAR);
	mcf_read32(PCICAR);
	PCIBIOS_SUCCESSFUL
}

unsafe fn mcf_pci_writeconfig(
	bus: *mut pci_bus, devfn: c_uint, where_: c_int, size: c_int, value: u32,
) -> c_int {
	let mut addr: c_ulong;
	if (*bus).number == 0 && mcf_host_slot2sid[PCI_SLOT(devfn) as usize] == 0 {
		return PCIBIOS_SUCCESSFUL;
	}
	addr = mcf_mk_pcicar((*bus).number, devfn, where_);
	mcf_write32(PCICAR_E as u32 | addr as u32, PCICAR);
	mcf_read32(PCICAR);
	addr = iospace + (where_ as c_ulong & 0x3);
	match size {
		1 => mcf_write8(value as u8, addr),
		2 => mcf_write16(cpu_to_le16(value) as u16, addr),
		_ => mcf_write32(cpu_to_le32(value), addr),
	}
	mcf_write32(0, PCICAR);
	mcf_read32(PCICAR);
	PCIBIOS_SUCCESSFUL
}

static mut mcf_pci_ops: pci_ops = pci_ops {
	read: Some(mcf_pci_readconfig),
	write: Some(mcf_pci_writeconfig),
};

static mut mcf_pci_mem: resource = resource {
	name: b"PCI Memory space\0".as_ptr() as *const c_char,
	start: PCI_MEM_PA,
	end: PCI_MEM_PA + PCI_MEM_SIZE - 1,
	flags: IORESOURCE_MEM,
};
static mut mcf_pci_io: resource = resource {
	name: b"PCI IO space\0".as_ptr() as *const c_char,
	start: 0x400,
	end: 0x10000 - 1,
	flags: IORESOURCE_IO,
};
static mut busn_resource: resource = resource {
	name: b"PCI busn\0".as_ptr() as *const c_char,
	start: 0,
	end: 255,
	flags: IORESOURCE_BUS,
};

unsafe fn mcf_pci_map_irq(_dev: *const pci_dev, slot: u8, _pin: u8) -> c_int {
	let sid = mcf_host_slot2sid[slot as usize];
	if sid != 0 { mcf_host_irq[sid as usize] as c_int } else { 0 }
}

unsafe fn mcf_pci_init() -> c_int {
	let bridge = pci_alloc_host_bridge(0);
	if bridge.is_null() { return -ENOMEM; }
	pr_info!("ColdFire: PCI bus initialization...\n");
	mcf_write32(PCIGSCR_RESET, PCIGSCR);
	mcf_write32(0, PCITCR);
	request_resource(&mut iomem_resource, &mut mcf_pci_mem);
	request_resource(&mut iomem_resource, &mut mcf_pci_io);
	mcf_write32(PACR_INTMPRI | PACR_INTMINTE | PACR_EXTMPRI(0x1f) | PACR_EXTMINTE(0x1f), PACR);
	mcf_write16(0x3ff, MCFGPIO_PAR_PCIBG);
	mcf_write16(0x3ff, MCFGPIO_PAR_PCIBR);
	mcf_write32(PCI_COMMAND_MEMORY | PCI_COMMAND_MASTER | PCI_COMMAND_INVALIDATE, PCISCR);
	mcf_write32(PCICR1_LT(32) | PCICR1_CL(8), PCICR1);
	mcf_write32(0, PCICR2);
	mcf_write32(WXBTAR(PCI_MEM_PA, PCI_MEM_BA, PCI_MEM_SIZE), PCIIW0BTAR);
	mcf_write32(WXBTAR(PCI_IO_PA, PCI_IO_BA, PCI_IO_SIZE), PCIIW1BTAR);
	mcf_write32(PCIIWCR_W0_MEM | PCIIWCR_W0_E | PCIIWCR_W1_IO | PCIIWCR_W1_E, PCIIWCR);
	mcf_write32(CONFIG_RAMBASE, PCIBAR1);
	mcf_write32(CONFIG_RAMBASE | PCITBATR1_E, PCITBATR1);
	iospace = ioremap(PCI_IO_PA, PCI_IO_SIZE) as c_ulong;
	if iospace == 0 { pci_free_host_bridge(bridge); return -ENODEV; }
	pr_info!("Coldfire: PCI IO/config window mapped to 0x%x\n", iospace as u32);
	mcf_write32(0, PCIGSCR);
	set_current_state(TASK_UNINTERRUPTIBLE);
	schedule_timeout(msecs_to_jiffies(200));
	pci_add_resource(&mut (*bridge).windows, &mut ioport_resource);
	pci_add_resource(&mut (*bridge).windows, &mut iomem_resource);
	pci_add_resource(&mut (*bridge).windows, &mut busn_resource);
	(*bridge).dev.parent = core::ptr::null_mut();
	(*bridge).sysdata = core::ptr::null_mut();
	(*bridge).busnr = 0;
	(*bridge).ops = &mut mcf_pci_ops;
	(*bridge).swizzle_irq = Some(pci_common_swizzle);
	(*bridge).map_irq = Some(mcf_pci_map_irq);
	let ret = pci_scan_root_bus_bridge(bridge);
	if ret != 0 { pci_free_host_bridge(bridge); return ret; }
	rootbus = (*bridge).bus;
	(*rootbus).resource[0] = &mut mcf_pci_io;
	(*rootbus).resource[1] = &mut mcf_pci_mem;
	pci_bus_size_bridges(rootbus);
	pci_bus_assign_resources(rootbus);
	pci_bus_add_devices(rootbus);
	0
}

subsys_initcall!(mcf_pci_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
