/*
 * Broadcom specific AMBA
 * PCI Core
 *
 * Copyright 2005, 2011, Broadcom Corporation
 * Copyright 2006, 2007, Michael Buesch <m@bues.ch>
 * Copyright 2011, 2012, Hauke Mehrtens <hauke@hauke-m.de>
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

/* Dependencies supplied by the surrounding BCMA implementation. */
extern "C" {
    fn pcicore_write32(pc: *mut bcma_drv_pci, address: u32, data: u32);
    fn pcicore_read32(pc: *mut bcma_drv_pci, address: u32) -> u32;
    fn pcicore_read16(pc: *mut bcma_drv_pci, address: u32) -> u16;
    fn pcicore_write16(pc: *mut bcma_drv_pci, address: u32, data: u16);
    fn udelay(usecs: u32);
    fn usleep_range(min: u32, max: u32);
    fn bcma_core_pci_is_in_hostmode(pc: *mut bcma_drv_pci) -> bool;
    fn bcma_core_pci_hostmode_init(pc: *mut bcma_drv_pci);
}

#[repr(C)]
pub struct bcma_drv_pci {
    pub core: *mut bcma_device,
    pub early_setup_done: bool,
    pub hostmode: bool,
    pub setup_done: bool,
}

#[repr(C)]
pub struct bcma_device {
    pub id: bcma_device_id,
    pub core_index: u32,
}

#[repr(C)]
pub struct bcma_device_id { pub rev: u8 }

#[repr(C)]
pub struct bcma_bus {
    pub hosttype: u32,
    pub drv_pci: [bcma_drv_pci; 1],
}

pub unsafe fn bcma_pcie_read(pc: *mut bcma_drv_pci, address: u32) -> u32 {
    pcicore_write32(pc, BCMA_CORE_PCI_PCIEIND_ADDR, address);
    pcicore_read32(pc, BCMA_CORE_PCI_PCIEIND_ADDR);
    pcicore_read32(pc, BCMA_CORE_PCI_PCIEIND_DATA)
}

unsafe fn bcma_pcie_write(pc: *mut bcma_drv_pci, address: u32, data: u32) {
    pcicore_write32(pc, BCMA_CORE_PCI_PCIEIND_ADDR, address);
    pcicore_read32(pc, BCMA_CORE_PCI_PCIEIND_ADDR);
    pcicore_write32(pc, BCMA_CORE_PCI_PCIEIND_DATA, data);
}

unsafe fn bcma_pcie_mdio_set_phy(pc: *mut bcma_drv_pci, phy: u16) {
    let mut v: u32 = BCMA_CORE_PCI_MDIODATA_START | BCMA_CORE_PCI_MDIODATA_WRITE |
        (BCMA_CORE_PCI_MDIODATA_DEV_ADDR << BCMA_CORE_PCI_MDIODATA_DEVADDR_SHF) |
        (BCMA_CORE_PCI_MDIODATA_BLK_ADDR << BCMA_CORE_PCI_MDIODATA_REGADDR_SHF) |
        BCMA_CORE_PCI_MDIODATA_TA | ((phy as u32) << 4);
    pcicore_write32(pc, BCMA_CORE_PCI_MDIO_DATA, v);
    udelay(10);
    for _i in 0..200 {
        v = pcicore_read32(pc, BCMA_CORE_PCI_MDIO_CONTROL);
        if v & BCMA_CORE_PCI_MDIOCTL_ACCESS_DONE != 0 { break; }
        usleep_range(1000, 2000);
    }
}

unsafe fn bcma_pcie_mdio_read(pc: *mut bcma_drv_pci, device: u16, address: u8) -> u16 {
    let mut max_retries: i32 = 10;
    let mut ret: u16 = 0;
    let mut v: u32;
    pcicore_write32(pc, BCMA_CORE_PCI_MDIO_CONTROL,
        BCMA_CORE_PCI_MDIOCTL_PREAM_EN | BCMA_CORE_PCI_MDIOCTL_DIVISOR_VAL);
    if (*(*pc).core).id.rev >= 10 {
        max_retries = 200;
        bcma_pcie_mdio_set_phy(pc, device);
        v = BCMA_CORE_PCI_MDIODATA_DEV_ADDR << BCMA_CORE_PCI_MDIODATA_DEVADDR_SHF;
        v |= (address as u32) << BCMA_CORE_PCI_MDIODATA_REGADDR_SHF;
    } else {
        v = (device as u32) << BCMA_CORE_PCI_MDIODATA_DEVADDR_SHF_OLD;
        v |= (address as u32) << BCMA_CORE_PCI_MDIODATA_REGADDR_SHF_OLD;
    }
    v |= BCMA_CORE_PCI_MDIODATA_START | BCMA_CORE_PCI_MDIODATA_READ | BCMA_CORE_PCI_MDIODATA_TA;
    pcicore_write32(pc, BCMA_CORE_PCI_MDIO_DATA, v);
    udelay(10);
    for _i in 0..max_retries {
        v = pcicore_read32(pc, BCMA_CORE_PCI_MDIO_CONTROL);
        if v & BCMA_CORE_PCI_MDIOCTL_ACCESS_DONE != 0 {
            udelay(10); ret = pcicore_read32(pc, BCMA_CORE_PCI_MDIO_DATA) as u16; break;
        }
        usleep_range(1000, 2000);
    }
    pcicore_write32(pc, BCMA_CORE_PCI_MDIO_CONTROL, 0); ret
}

unsafe fn bcma_pcie_mdio_write(pc: *mut bcma_drv_pci, device: u16, address: u8, data: u16) {
    let mut max_retries: i32 = 10;
    let mut v: u32;
    pcicore_write32(pc, BCMA_CORE_PCI_MDIO_CONTROL,
        BCMA_CORE_PCI_MDIOCTL_PREAM_EN | BCMA_CORE_PCI_MDIOCTL_DIVISOR_VAL);
    if (*(*pc).core).id.rev >= 10 {
        max_retries = 200; bcma_pcie_mdio_set_phy(pc, device);
        v = BCMA_CORE_PCI_MDIODATA_DEV_ADDR << BCMA_CORE_PCI_MDIODATA_DEVADDR_SHF;
        v |= (address as u32) << BCMA_CORE_PCI_MDIODATA_REGADDR_SHF;
    } else {
        v = (device as u32) << BCMA_CORE_PCI_MDIODATA_DEVADDR_SHF_OLD;
        v |= (address as u32) << BCMA_CORE_PCI_MDIODATA_REGADDR_SHF_OLD;
    }
    v |= BCMA_CORE_PCI_MDIODATA_START | BCMA_CORE_PCI_MDIODATA_WRITE |
         BCMA_CORE_PCI_MDIODATA_TA | data as u32;
    pcicore_write32(pc, BCMA_CORE_PCI_MDIO_DATA, v); udelay(10);
    for _i in 0..max_retries {
        v = pcicore_read32(pc, BCMA_CORE_PCI_MDIO_CONTROL);
        if v & BCMA_CORE_PCI_MDIOCTL_ACCESS_DONE != 0 { break; }
        usleep_range(1000, 2000);
    }
    pcicore_write32(pc, BCMA_CORE_PCI_MDIO_CONTROL, 0);
}

unsafe fn bcma_pcie_mdio_writeread(pc: *mut bcma_drv_pci, device: u16, address: u8, data: u16) -> u16 {
    bcma_pcie_mdio_write(pc, device, address, data); bcma_pcie_mdio_read(pc, device, address)
}

unsafe fn bcma_core_pci_fixcfg(pc: *mut bcma_drv_pci) {
    let core = (*pc).core;
    let regoff = BCMA_CORE_PCI_SPROM(BCMA_CORE_PCI_SPROM_PI_OFFSET);
    let core_index = (*core).core_index as u16;
    let mut val16 = pcicore_read16(pc, regoff);
    if ((val16 & BCMA_CORE_PCI_SPROM_PI_MASK) >> BCMA_CORE_PCI_SPROM_PI_SHIFT) != core_index {
        val16 = (core_index << BCMA_CORE_PCI_SPROM_PI_SHIFT) | (val16 & !BCMA_CORE_PCI_SPROM_PI_MASK);
        pcicore_write16(pc, regoff, val16);
    }
}

pub unsafe fn bcma_core_pci_early_init(pc: *mut bcma_drv_pci) {
    if (*pc).early_setup_done { return; }
    (*pc).hostmode = bcma_core_pci_is_in_hostmode(pc);
    if !(*pc).hostmode { bcma_core_pci_fixcfg(pc); }
    (*pc).early_setup_done = true;
}

unsafe fn bcma_pcicore_polarity_workaround(pc: *mut bcma_drv_pci) -> u8 {
    let tmp = bcma_pcie_read(pc, BCMA_CORE_PCI_PLP_STATUSREG);
    if tmp & BCMA_CORE_PCI_PLP_POLARITYINV_STAT != 0 { (BCMA_CORE_PCI_SERDES_RX_CTRL_FORCE | BCMA_CORE_PCI_SERDES_RX_CTRL_POLARITY) as u8 } else { BCMA_CORE_PCI_SERDES_RX_CTRL_FORCE as u8 }
}

unsafe fn bcma_pcicore_serdes_workaround(pc: *mut bcma_drv_pci) {
    bcma_pcie_mdio_write(pc, BCMA_CORE_PCI_MDIODATA_DEV_RX, BCMA_CORE_PCI_SERDES_RX_CTRL, bcma_pcicore_polarity_workaround(pc));
    let tmp = bcma_pcie_mdio_read(pc, BCMA_CORE_PCI_MDIODATA_DEV_PLL, BCMA_CORE_PCI_SERDES_PLL_CTRL);
    if tmp & BCMA_CORE_PCI_PLL_CTRL_FREQDET_EN != 0 { bcma_pcie_mdio_write(pc, BCMA_CORE_PCI_MDIODATA_DEV_PLL, BCMA_CORE_PCI_SERDES_PLL_CTRL, tmp & !BCMA_CORE_PCI_PLL_CTRL_FREQDET_EN); }
}

unsafe fn bcma_core_pci_config_fixup(pc: *mut bcma_drv_pci) {
    let regoff = BCMA_CORE_PCI_SPROM(BCMA_CORE_PCI_SPROM_MISC_CONFIG);
    let mut val16 = pcicore_read16(pc, regoff);
    if val16 & BCMA_CORE_PCI_SPROM_L23READY_EXIT_NOPERST == 0 { val16 |= BCMA_CORE_PCI_SPROM_L23READY_EXIT_NOPERST; pcicore_write16(pc, regoff, val16); }
}

unsafe fn bcma_core_pci_clientmode_init(pc: *mut bcma_drv_pci) { bcma_pcicore_serdes_workaround(pc); bcma_core_pci_config_fixup(pc); }

pub unsafe fn bcma_core_pci_init(pc: *mut bcma_drv_pci) {
    if (*pc).setup_done { return; }
    bcma_core_pci_early_init(pc);
    if (*pc).hostmode { bcma_core_pci_hostmode_init(pc); } else { bcma_core_pci_clientmode_init(pc); }
}

pub unsafe fn bcma_core_pci_power_save(bus: *mut bcma_bus, up: bool) {
    if (*bus).hosttype != BCMA_HOSTTYPE_PCI { return; }
    let pc = &mut (*bus).drv_pci[0] as *mut bcma_drv_pci;
    let rev = (*(*pc).core).id.rev;
    if rev >= 15 && rev <= 20 { bcma_pcie_mdio_writeread(pc, BCMA_CORE_PCI_MDIO_BLK1, BCMA_CORE_PCI_MDIO_BLK1_MGMT1, 0x7F64); bcma_pcie_mdio_writeread(pc, BCMA_CORE_PCI_MDIO_BLK1, BCMA_CORE_PCI_MDIO_BLK1_MGMT3, if up { 0x74 } else { 0x7C }); }
    else if rev >= 21 && rev <= 22 { bcma_pcie_mdio_writeread(pc, BCMA_CORE_PCI_MDIO_BLK1, BCMA_CORE_PCI_MDIO_BLK1_MGMT1, 0x7E65); bcma_pcie_mdio_writeread(pc, BCMA_CORE_PCI_MDIO_BLK1, BCMA_CORE_PCI_MDIO_BLK1_MGMT3, if up { 0x75 } else { 0x7D }); }
}

unsafe fn bcma_core_pci_extend_L1timer(pc: *mut bcma_drv_pci, extend: bool) {
    let mut w = bcma_pcie_read(pc, BCMA_CORE_PCI_DLLP_PMTHRESHREG);
    if extend { w |= BCMA_CORE_PCI_ASPMTIMER_EXTEND; } else { w &= !BCMA_CORE_PCI_ASPMTIMER_EXTEND; }
    bcma_pcie_write(pc, BCMA_CORE_PCI_DLLP_PMTHRESHREG, w); bcma_pcie_read(pc, BCMA_CORE_PCI_DLLP_PMTHRESHREG);
}

pub unsafe fn bcma_core_pci_up(pc: *mut bcma_drv_pci) { bcma_core_pci_extend_L1timer(pc, true); }
pub unsafe fn bcma_core_pci_down(pc: *mut bcma_drv_pci) { bcma_core_pci_extend_L1timer(pc, false); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
