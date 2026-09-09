/*
 *  Copyright (C) 2008 Aurelien Jarno <aurelien@aurel32.net>
 *
 *  This program is free software; you can redistribute  it and/or modify it
 *  under the terms of  the GNU General  Public License as published by
 *  the Free Software Foundation;  either version 2 of the  License, or
 *  (at your option) any later version.
 */

// Linux dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct pci_dev {
    pub devfn: u32,
    pub irq: i32,
}

extern "C" {
    pub static bcm47xx_bus_type: i32;

    pub fn ssb_pcibios_plat_dev_init(dev: *mut pci_dev) -> i32;
    pub fn ssb_pcibios_map_irq(dev: *mut pci_dev, slot: u8, pin: u8) -> i32;
    pub fn bcma_core_pci_plat_dev_init(dev: *mut pci_dev) -> i32;
    pub fn bcma_core_pci_pcibios_map_irq(dev: *mut pci_dev) -> i32;
    pub fn pci_alert(dev: *mut pci_dev, format: *const u8, ...);
    pub fn pci_read_config_byte(dev: *mut pci_dev, where_: u32, value: *mut u8) -> i32;
}

pub const PCI_INTERRUPT_PIN: u32 = 0x3d;
pub const BCM47XX_BUS_TYPE_SSB: i32 = 0;
pub const BCM47XX_BUS_TYPE_BCMA: i32 = 1;

#[inline]
pub unsafe fn PCI_SLOT(devfn: u32) -> u8 {
    ((devfn >> 3) & 0x1f) as u8
}

pub unsafe fn pcibios_map_irq(_dev: *const pci_dev, _slot: u8, _pin: u8) -> i32 {
    0
}

// CONFIG_BCM47XX_SSB conditionally includes the following implementation.
#[cfg(CONFIG_BCM47XX_SSB)]
unsafe fn bcm47xx_pcibios_plat_dev_init_ssb(dev: *mut pci_dev) -> i32 {
    let mut res: i32;
    let mut slot: u8;
    let mut pin: u8 = 0;

    res = ssb_pcibios_plat_dev_init(dev);
    if res < 0 {
        pci_alert(dev, b"PCI: Failed to init device\n\0".as_ptr());
        return res;
    }

    pci_read_config_byte(dev, PCI_INTERRUPT_PIN, &mut pin);
    slot = PCI_SLOT((*dev).devfn);
    res = ssb_pcibios_map_irq(dev, slot, pin);

    /* IRQ-0 and IRQ-1 are software interrupts. */
    if res < 2 {
        pci_alert(dev, b"PCI: Failed to map IRQ of device\n\0".as_ptr());
        return res;
    }

    (*dev).irq = res;
    0
}

// CONFIG_BCM47XX_BCMA conditionally includes the following implementation.
#[cfg(CONFIG_BCM47XX_BCMA)]
unsafe fn bcm47xx_pcibios_plat_dev_init_bcma(dev: *mut pci_dev) -> i32 {
    let mut res: i32;

    res = bcma_core_pci_plat_dev_init(dev);
    if res < 0 {
        pci_alert(dev, b"PCI: Failed to init device\n\0".as_ptr());
        return res;
    }

    res = bcma_core_pci_pcibios_map_irq(dev);

    /* IRQ-0 and IRQ-1 are software interrupts. */
    if res < 2 {
        pci_alert(dev, b"PCI: Failed to map IRQ of device\n\0".as_ptr());
        return res;
    }

    (*dev).irq = res;
    0
}

pub unsafe fn pcibios_plat_dev_init(dev: *mut pci_dev) -> i32 {
    // CONFIG_BCM47XX_SSB
    #[cfg(CONFIG_BCM47XX_SSB)]
    if bcm47xx_bus_type == BCM47XX_BUS_TYPE_SSB {
        return bcm47xx_pcibios_plat_dev_init_ssb(dev);
    }

    // CONFIG_BCM47XX_BCMA
    #[cfg(CONFIG_BCM47XX_BCMA)]
    if bcm47xx_bus_type == BCM47XX_BUS_TYPE_BCMA {
        return bcm47xx_pcibios_plat_dev_init_bcma(dev);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
