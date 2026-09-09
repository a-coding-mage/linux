// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2008 Lemote Technology
 * Copyright (C) 2004 ICT CAS
 * Author: Li xiaoyu, lixy@ict.ac.cn
 *
 * Copyright (C) 2007 Lemote, Inc.
 * Author: Fuxin Zhang, zhangfx@lemote.com
 */

// Dependencies supplied by the Linux PCI, Loongson, and CS5536 headers.

/* PCI interrupt pins
 *
 * These should not be changed, or you should consider loongson2f interrupt
 * register and your pci card dispatch
 */
const PCIA: u8 = 4;
const PCIB: u8 = 5;
const PCIC: u8 = 6;
const PCID: u8 = 7;

/* all the pci device has the PCIA pin, check the datasheet. */
static IRQ_TAB: [[i8; 5]; 17] = [
    /* INTA INTB INTC INTD */
    [0, 0, 0, 0, 0], /* 11: Unused */
    [0, 0, 0, 0, 0], /* 12: Unused */
    [0, 0, 0, 0, 0], /* 13: Unused */
    [0, 0, 0, 0, 0], /* 14: Unused */
    [0, 0, 0, 0, 0], /* 15: Unused */
    [0, 0, 0, 0, 0], /* 16: Unused */
    [0, PCIA as i8, 0, 0, 0], /* 17: RTL8110-0 */
    [0, PCIB as i8, 0, 0, 0], /* 18: RTL8110-1 */
    [0, PCIC as i8, 0, 0, 0], /* 19: SiI3114 */
    [0, PCID as i8, 0, 0, 0], /* 20: 3-ports nec usb */
    [0, PCIA as i8, PCIB as i8, PCIC as i8, PCID as i8], /* 21: PCI-SLOT */
    [0, 0, 0, 0, 0], /* 22: Unused */
    [0, 0, 0, 0, 0], /* 23: Unused */
    [0, 0, 0, 0, 0], /* 24: Unused */
    [0, 0, 0, 0, 0], /* 25: Unused */
    [0, 0, 0, 0, 0], /* 26: Unused */
    [0, 0, 0, 0, 0], /* 27: Unused */
];

pub unsafe fn pcibios_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    let virq: i32;

    if (PCI_SLOT((*dev).devfn) != PCI_IDSEL_CS5536) && (PCI_SLOT((*dev).devfn) < 32) {
        virq = IRQ_TAB[slot as usize][pin as usize] as i32;
        printk(KERN_INFO, "slot: %d, pin: %d, irq: %d\n", slot, pin,
               virq + LOONGSON_IRQ_BASE);
        if virq != 0 {
            return LOONGSON_IRQ_BASE + virq;
        } else {
            return 0;
        }
    } else if PCI_SLOT((*dev).devfn) == PCI_IDSEL_CS5536 {
        match PCI_FUNC((*dev).devfn) {
            2 => {
                pci_write_config_byte(dev, PCI_INTERRUPT_LINE, CS5536_IDE_INTR);
                return CS5536_IDE_INTR;
            }
            3 => {
                pci_write_config_byte(dev, PCI_INTERRUPT_LINE, CS5536_ACC_INTR);
                return CS5536_ACC_INTR;
            }
            4 | 5 | 6 | 7 => {
                pci_write_config_byte(dev, PCI_INTERRUPT_LINE, CS5536_USB_INTR);
                return CS5536_USB_INTR;
            }
            _ => return (*dev).irq,
        }
    } else {
        printk(KERN_INFO, "strange PCI slot number.\n");
        return 0;
    }
}

/* Do platform specific device initialization at pci_enable_device() time */
pub unsafe fn pcibios_plat_dev_init(_dev: *mut pci_dev) -> i32 {
    0
}

/* CS5536 SPEC. fixup */
unsafe fn loongson_cs5536_isa_fixup(pdev: *mut pci_dev) {
    /* the uart1 and uart2 interrupt in PIC is enabled as default */
    pci_write_config_dword(pdev, PCI_UART1_INT_REG, 1);
    pci_write_config_dword(pdev, PCI_UART2_INT_REG, 1);
}

unsafe fn loongson_cs5536_ide_fixup(pdev: *mut pci_dev) {
    /* setting the mutex pin as IDE function */
    pci_write_config_dword(pdev, PCI_IDE_CFG_REG, CS5536_IDE_FLASH_SIGNATURE);
}

unsafe fn loongson_cs5536_acc_fixup(pdev: *mut pci_dev) {
    /* enable the AUDIO interrupt in PIC  */
    pci_write_config_dword(pdev, PCI_ACC_INT_REG, 1);
    pci_write_config_byte(pdev, PCI_LATENCY_TIMER, 0xc0);
}

unsafe fn loongson_cs5536_ohci_fixup(pdev: *mut pci_dev) {
    /* enable the OHCI interrupt in PIC */
    /* THE OHCI, EHCI, UDC, OTG are shared with interrupt in PIC */
    pci_write_config_dword(pdev, PCI_OHCI_INT_REG, 1);
}

unsafe fn loongson_cs5536_ehci_fixup(pdev: *mut pci_dev) {
    let mut hi: u32 = 0;
    let mut lo: u32 = 0;

    /* Serial short detect enable */
    _rdmsr(USB_MSR_REG(USB_CONFIG), &mut hi, &mut lo);
    _wrmsr(USB_MSR_REG(USB_CONFIG), (1 << 1) | (1 << 3), lo);

    /* setting the USB2.0 micro frame length */
    pci_write_config_dword(pdev, PCI_EHCI_FLADJ_REG, 0x2000);
}

unsafe fn loongson_nec_fixup(pdev: *mut pci_dev) {
    let mut val: u32 = 0;
    pci_read_config_dword(pdev, 0xe0, &mut val);
    /* Only 2 port be used */
    pci_write_config_dword(pdev, 0xe0, (val & !3) | 0x2);
}

// DECLARE_PCI_FIXUP_HEADER registrations:
// AMD CS5536 ISA -> loongson_cs5536_isa_fixup
// AMD CS5536 OHC -> loongson_cs5536_ohci_fixup
// AMD CS5536 EHC -> loongson_cs5536_ehci_fixup
// AMD CS5536 AUDIO -> loongson_cs5536_acc_fixup
// AMD CS5536 IDE -> loongson_cs5536_ide_fixup
// NEC USB -> loongson_nec_fixup

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
