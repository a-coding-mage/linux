// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation.

/* PCI interrupt pins */
const PCIA: usize = 1;
const PCIB: usize = 2;
const PCIC: usize = 3;
const PCID: usize = 4;

/* This table is filled in by interrogating the PIIX4 chip */
static mut pci_irq: [i8; 5] = [0; 5];

static mut irq_tab: [[i8; 5]; 22] = [
	[0, 0, 0, 0, 0], /*  0: GT64120 PCI bridge */
	[0, 0, 0, 0, 0], /*  1: Unused */
	[0, 0, 0, 0, 0], /*  2: Unused */
	[0, 0, 0, 0, 0], /*  3: Unused */
	[0, 0, 0, 0, 0], /*  4: Unused */
	[0, 0, 0, 0, 0], /*  5: Unused */
	[0, 0, 0, 0, 0], /*  6: Unused */
	[0, 0, 0, 0, 0], /*  7: Unused */
	[0, 0, 0, 0, 0], /*  8: Unused */
	[0, 0, 0, 0, 0], /*  9: Unused */
	[0, 0, 0, 0, PCID as i8], /* 10: PIIX4 USB */
	[0, PCIB as i8, 0, 0, 0], /* 11: AMD 79C973 Ethernet */
	[0, PCIC as i8, 0, 0, 0], /* 12: Crystal 4281 Sound */
	[0, 0, 0, 0, 0], /* 13: Unused */
	[0, 0, 0, 0, 0], /* 14: Unused */
	[0, 0, 0, 0, 0], /* 15: Unused */
	[0, 0, 0, 0, 0], /* 16: Unused */
	[0, 0, 0, 0, 0], /* 17: Bonito/SOC-it PCI Bridge */
	[PCIA as i8, PCIB as i8, PCIC as i8, PCID as i8, 0], /* 18: PCI Slot 1 */
	[PCIB as i8, PCIC as i8, PCID as i8, PCIA as i8, 0], /* 19: PCI Slot 2 */
	[PCIC as i8, PCID as i8, PCIA as i8, PCIB as i8, 0], /* 20: PCI Slot 3 */
	[PCID as i8, PCIA as i8, PCIB as i8, PCIC as i8, 0], /* 21: PCI Slot 4 */
];

pub unsafe fn pcibios_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i8 {
	let virq = irq_tab[slot as usize][pin as usize];
	pci_irq[virq as usize]
}

/* Do platform specific device initialization at pci_enable_device() time */
pub unsafe fn pcibios_plat_dev_init(dev: *mut pci_dev) -> i32 {
	0
}

unsafe fn malta_piix_func3_base_fixup(dev: *mut pci_dev) {
	/* Set a sane PM I/O base address */
	pci_write_config_word(dev, PIIX4_FUNC3_PMBA, 0x1000);

	/* Enable access to the PM I/O region */
	pci_write_config_byte(dev, PIIX4_FUNC3_PMREGMISC, PIIX4_FUNC3_PMREGMISC_EN);
}

// DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82371AB_3,
//                         malta_piix_func3_base_fixup);

unsafe fn malta_piix_func0_fixup(pdev: *mut pci_dev) {
	let mut reg_val: u8;
	let mut reg_val32: u32;
	let mut reg_val16: u16;
	/* PIIX PIRQC[A:D] irq mappings */
	static piixirqmap: [i32; PIIX4_FUNC0_PIRQRC_IRQ_ROUTING_MAX as usize] =
		[0, 0, 0, 3, 4, 5, 6, 7, 0, 9, 10, 11, 12, 0, 14, 15];

	/* Interrogate PIIX4 to get PCI IRQ mapping */
	for i in 0..=3 {
		pci_read_config_byte(pdev, PIIX4_FUNC0_PIRQRC + i, &mut reg_val);
		if reg_val & PIIX4_FUNC0_PIRQRC_IRQ_ROUTING_DISABLE != 0 {
			pci_irq[PCIA + i as usize] = 0; /* Disabled */
		} else {
			pci_irq[PCIA + i as usize] = piixirqmap[
				(reg_val & PIIX4_FUNC0_PIRQRC_IRQ_ROUTING_MASK) as usize
			] as i8;
		}
	}

	/* Done by YAMON 2.00 onwards */
	if PCI_SLOT((*pdev).devfn) == 10 {
		/* Set top of main memory accessible by ISA or DMA devices to 16 Mb. */
		pci_read_config_byte(pdev, PIIX4_FUNC0_TOM, &mut reg_val);
		pci_write_config_byte(pdev, PIIX4_FUNC0_TOM,
			reg_val | PIIX4_FUNC0_TOM_TOP_OF_MEMORY_MASK);
	}

	/* Mux SERIRQ to its pin */
	pci_read_config_dword(pdev, PIIX4_FUNC0_GENCFG, &mut reg_val32);
	pci_write_config_dword(pdev, PIIX4_FUNC0_GENCFG,
		reg_val32 | PIIX4_FUNC0_GENCFG_SERIRQ);

	/* Enable SERIRQ */
	pci_read_config_byte(pdev, PIIX4_FUNC0_SERIRQC, &mut reg_val);
	reg_val |= PIIX4_FUNC0_SERIRQC_EN | PIIX4_FUNC0_SERIRQC_CONT;
	pci_write_config_byte(pdev, PIIX4_FUNC0_SERIRQC, reg_val);

	/* Enable response to special cycles */
	pci_read_config_word(pdev, PCI_COMMAND, &mut reg_val16);
	pci_write_config_word(pdev, PCI_COMMAND, reg_val16 | PCI_COMMAND_SPECIAL);
}

// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82371AB_0,
//                          malta_piix_func0_fixup);

unsafe fn malta_piix_func1_fixup(pdev: *mut pci_dev) {
	let mut reg_val: u8;

	/* Done by YAMON 2.02 onwards */
	if PCI_SLOT((*pdev).devfn) == 10 {
		/* IDE Decode enable. */
		pci_read_config_byte(pdev, PIIX4_FUNC1_IDETIM_PRIMARY_HI, &mut reg_val);
		pci_write_config_byte(pdev, PIIX4_FUNC1_IDETIM_PRIMARY_HI,
			reg_val | PIIX4_FUNC1_IDETIM_PRIMARY_HI_IDE_DECODE_EN);
		pci_read_config_byte(pdev, PIIX4_FUNC1_IDETIM_SECONDARY_HI, &mut reg_val);
		pci_write_config_byte(pdev, PIIX4_FUNC1_IDETIM_SECONDARY_HI,
			reg_val | PIIX4_FUNC1_IDETIM_SECONDARY_HI_IDE_DECODE_EN);
	}
}

// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82371AB,
//                          malta_piix_func1_fixup);

/* Enable PCI 2.1 compatibility in PIIX4 */
unsafe fn quirk_dlcsetup(dev: *mut pci_dev) {
	let mut odlc: u8 = 0;
	let ndlc: u8;

	let _ = pci_read_config_byte(dev, PIIX4_FUNC0_DLC, &mut odlc);
	/* Enable passive releases and delayed transaction */
	ndlc = odlc | PIIX4_FUNC0_DLC_USBPR_EN |
		PIIX4_FUNC0_DLC_PASSIVE_RELEASE_EN |
		PIIX4_FUNC0_DLC_DELAYED_TRANSACTION_EN;
	let _ = pci_write_config_byte(dev, PIIX4_FUNC0_DLC, ndlc);
}

// DECLARE_PCI_FIXUP_FINAL(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82371AB_0,
//                         quirk_dlcsetup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
