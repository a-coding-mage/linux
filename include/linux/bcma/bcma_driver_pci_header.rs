/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/bcma/bcma_driver_pci.h. C includes and configuration
// guards are represented by comments; dependent types and symbols are external.

use core::ffi::c_void;

#[repr(C)]
pub struct pci_dev { _private: [u8; 0] }
pub struct bcma_drv_pci;
pub struct bcma_bus;
pub struct bcma_device;

pub const BCMA_CORE_PCI_CTL: u32 = 0x0000;
pub const BCMA_CORE_PCI_CTL_RST_OE: u32 = 0x00000001;
pub const BCMA_CORE_PCI_CTL_RST: u32 = 0x00000002;
pub const BCMA_CORE_PCI_CTL_CLK_OE: u32 = 0x00000004;
pub const BCMA_CORE_PCI_CTL_CLK: u32 = 0x00000008;
pub const BCMA_CORE_PCI_ARBCTL: u32 = 0x0010;
pub const BCMA_CORE_PCI_ARBCTL_INTERN: u32 = 0x00000001;
pub const BCMA_CORE_PCI_ARBCTL_EXTERN: u32 = 0x00000002;
pub const BCMA_CORE_PCI_ARBCTL_PARKID: u32 = 0x00000006;
pub const BCMA_CORE_PCI_ARBCTL_PARKID_LAST: u32 = 0x00000000;
pub const BCMA_CORE_PCI_ARBCTL_PARKID_4710: u32 = 0x00000002;
pub const BCMA_CORE_PCI_ARBCTL_PARKID_EXT0: u32 = 0x00000004;
pub const BCMA_CORE_PCI_ARBCTL_PARKID_EXT1: u32 = 0x00000006;
pub const BCMA_CORE_PCI_ISTAT: u32 = 0x0020;
pub const BCMA_CORE_PCI_ISTAT_INTA: u32 = 1;
pub const BCMA_CORE_PCI_ISTAT_INTB: u32 = 2;
pub const BCMA_CORE_PCI_ISTAT_SERR: u32 = 4;
pub const BCMA_CORE_PCI_ISTAT_PERR: u32 = 8;
pub const BCMA_CORE_PCI_ISTAT_PME: u32 = 0x10;
pub const BCMA_CORE_PCI_IMASK: u32 = 0x0024;
pub const BCMA_CORE_PCI_IMASK_INTA: u32 = 1;
pub const BCMA_CORE_PCI_IMASK_INTB: u32 = 2;
pub const BCMA_CORE_PCI_IMASK_SERR: u32 = 4;
pub const BCMA_CORE_PCI_IMASK_PERR: u32 = 8;
pub const BCMA_CORE_PCI_IMASK_PME: u32 = 0x10;
pub const BCMA_CORE_PCI_MBOX: u32 = 0x0028;
pub const BCMA_CORE_PCI_MBOX_F0_0: u32 = 0x100;
pub const BCMA_CORE_PCI_MBOX_F0_1: u32 = 0x200;
pub const BCMA_CORE_PCI_MBOX_F1_0: u32 = 0x400;
pub const BCMA_CORE_PCI_MBOX_F1_1: u32 = 0x800;
pub const BCMA_CORE_PCI_MBOX_F2_0: u32 = 0x1000;
pub const BCMA_CORE_PCI_MBOX_F2_1: u32 = 0x2000;
pub const BCMA_CORE_PCI_MBOX_F3_0: u32 = 0x4000;
pub const BCMA_CORE_PCI_MBOX_F3_1: u32 = 0x8000;
pub const BCMA_CORE_PCI_BCAST_ADDR: u32 = 0x50;
pub const BCMA_CORE_PCI_BCAST_ADDR_MASK: u32 = 0xff;
pub const BCMA_CORE_PCI_BCAST_DATA: u32 = 0x54;
pub const BCMA_CORE_PCI_GPIO_IN: u32 = 0x60;
pub const BCMA_CORE_PCI_GPIO_OUT: u32 = 0x64;
pub const BCMA_CORE_PCI_GPIO_ENABLE: u32 = 0x68;
pub const BCMA_CORE_PCI_GPIO_CTL: u32 = 0x6c;
pub const BCMA_CORE_PCI_SBTOPCI0: u32 = 0x100;
pub const BCMA_CORE_PCI_SBTOPCI0_MASK: u32 = 0xfc000000;
pub const BCMA_CORE_PCI_SBTOPCI1: u32 = 0x104;
pub const BCMA_CORE_PCI_SBTOPCI1_MASK: u32 = 0xfc000000;
pub const BCMA_CORE_PCI_SBTOPCI2: u32 = 0x108;
pub const BCMA_CORE_PCI_SBTOPCI2_MASK: u32 = 0xc0000000;
pub const BCMA_CORE_PCI_CONFIG_ADDR: u32 = 0x120;
pub const BCMA_CORE_PCI_CONFIG_DATA: u32 = 0x124;
pub const BCMA_CORE_PCI_MDIO_CONTROL: u32 = 0x128;
pub const BCMA_CORE_PCI_MDIOCTL_DIVISOR_MASK: u32 = 0x7f;
pub const BCMA_CORE_PCI_MDIOCTL_DIVISOR_VAL: u32 = 2;
pub const BCMA_CORE_PCI_MDIOCTL_PREAM_EN: u32 = 0x80;
pub const BCMA_CORE_PCI_MDIOCTL_ACCESS_DONE: u32 = 0x100;
pub const BCMA_CORE_PCI_MDIO_DATA: u32 = 0x12c;
pub const BCMA_CORE_PCI_MDIODATA_MASK: u32 = 0xffff;
pub const BCMA_CORE_PCI_MDIODATA_TA: u32 = 0x20000;
pub const BCMA_CORE_PCI_MDIODATA_REGADDR_SHF_OLD: u32 = 18;
pub const BCMA_CORE_PCI_MDIODATA_REGADDR_MASK_OLD: u32 = 0x3c0000;
pub const BCMA_CORE_PCI_MDIODATA_DEVADDR_SHF_OLD: u32 = 22;
pub const BCMA_CORE_PCI_MDIODATA_DEVADDR_MASK_OLD: u32 = 0xfc00000;
pub const BCMA_CORE_PCI_MDIODATA_REGADDR_SHF: u32 = 18;
pub const BCMA_CORE_PCI_MDIODATA_REGADDR_MASK: u32 = 0x7c0000;
pub const BCMA_CORE_PCI_MDIODATA_DEVADDR_SHF: u32 = 23;
pub const BCMA_CORE_PCI_MDIODATA_DEVADDR_MASK: u32 = 0xf800000;
pub const BCMA_CORE_PCI_MDIODATA_WRITE: u32 = 0x10000000;
pub const BCMA_CORE_PCI_MDIODATA_READ: u32 = 0x20000000;
pub const BCMA_CORE_PCI_MDIODATA_START: u32 = 0x40000000;
pub const BCMA_CORE_PCI_MDIODATA_DEV_ADDR: u32 = 0;
pub const BCMA_CORE_PCI_MDIODATA_BLK_ADDR: u32 = 0x1f;
pub const BCMA_CORE_PCI_MDIODATA_DEV_PLL: u32 = 0x1d;
pub const BCMA_CORE_PCI_MDIODATA_DEV_TX: u32 = 0x1e;
pub const BCMA_CORE_PCI_MDIODATA_DEV_RX: u32 = 0x1f;
pub const BCMA_CORE_PCI_PCIEIND_ADDR: u32 = 0x130;
pub const BCMA_CORE_PCI_PCIEIND_DATA: u32 = 0x134;
pub const BCMA_CORE_PCI_CLKREQENCTRL: u32 = 0x138;
pub const BCMA_CORE_PCI_PCICFG0: u32 = 0x400;
pub const BCMA_CORE_PCI_PCICFG1: u32 = 0x500;
pub const BCMA_CORE_PCI_PCICFG2: u32 = 0x600;
pub const BCMA_CORE_PCI_PCICFG3: u32 = 0x700;
pub const BCMA_CORE_PCI_SPROM_PI_OFFSET: u32 = 0;
pub const BCMA_CORE_PCI_SPROM_PI_MASK: u32 = 0xf000;
pub const BCMA_CORE_PCI_SPROM_PI_SHIFT: u32 = 12;
pub const BCMA_CORE_PCI_SPROM_MISC_CONFIG: u32 = 5;
pub const BCMA_CORE_PCI_SPROM_L23READY_EXIT_NOPERST: u32 = 0x8000;
pub const BCMA_CORE_PCI_SPROM_CLKREQ_OFFSET_REV5: u32 = 20;
pub const BCMA_CORE_PCI_SPROM_CLKREQ_ENB: u32 = 0x800;
#[inline] pub const fn BCMA_CORE_PCI_SPROM(wordoffset: u32) -> u32 { 0x800 + wordoffset * 2 }

pub const BCMA_CORE_PCI_SBTOPCI_MEM: u32 = 0;
pub const BCMA_CORE_PCI_SBTOPCI_IO: u32 = 1;
pub const BCMA_CORE_PCI_SBTOPCI_CFG0: u32 = 2;
pub const BCMA_CORE_PCI_SBTOPCI_CFG1: u32 = 3;
pub const BCMA_CORE_PCI_SBTOPCI_PREF: u32 = 4;
pub const BCMA_CORE_PCI_SBTOPCI_BURST: u32 = 8;
pub const BCMA_CORE_PCI_SBTOPCI_MRM: u32 = 0x20;
pub const BCMA_CORE_PCI_SBTOPCI_RC: u32 = 0x30;
pub const BCMA_CORE_PCI_SBTOPCI_RC_READ: u32 = 0;
pub const BCMA_CORE_PCI_SBTOPCI_RC_READL: u32 = 0x10;
pub const BCMA_CORE_PCI_SBTOPCI_RC_READM: u32 = 0x20;

pub const BCMA_CORE_PCI_PLP_MODEREG: u32 = 0x200;
pub const BCMA_CORE_PCI_PLP_STATUSREG: u32 = 0x204;
pub const BCMA_CORE_PCI_PLP_POLARITYINV_STAT: u32 = 0x10;
pub const BCMA_CORE_PCI_PLP_LTSSMCTRLREG: u32 = 0x208;
pub const BCMA_CORE_PCI_PLP_LTLINKNUMREG: u32 = 0x20c;
pub const BCMA_CORE_PCI_PLP_LTLANENUMREG: u32 = 0x210;
pub const BCMA_CORE_PCI_PLP_LTNFTSREG: u32 = 0x214;
pub const BCMA_CORE_PCI_PLP_ATTNREG: u32 = 0x218;
pub const BCMA_CORE_PCI_PLP_ATTNMASKREG: u32 = 0x21c;
pub const BCMA_CORE_PCI_PLP_RXERRCTR: u32 = 0x220;
pub const BCMA_CORE_PCI_PLP_RXFRMERRCTR: u32 = 0x224;
pub const BCMA_CORE_PCI_PLP_RXERRTHRESHREG: u32 = 0x228;
pub const BCMA_CORE_PCI_PLP_TESTCTRLREG: u32 = 0x22c;
pub const BCMA_CORE_PCI_PLP_SERDESCTRLOVRDREG: u32 = 0x230;
pub const BCMA_CORE_PCI_PLP_TIMINGOVRDREG: u32 = 0x234;
pub const BCMA_CORE_PCI_PLP_RXTXSMDIAGREG: u32 = 0x238;
pub const BCMA_CORE_PCI_PLP_LTSSMDIAGREG: u32 = 0x23c;
pub const BCMA_CORE_PCI_DLLP_LCREG: u32 = 0x100;
pub const BCMA_CORE_PCI_DLLP_LSREG: u32 = 0x104;
pub const BCMA_CORE_PCI_DLLP_LAREG: u32 = 0x108;
pub const BCMA_CORE_PCI_DLLP_LSREG_LINKUP: u32 = 1 << 16;
pub const BCMA_CORE_PCI_DLLP_LAMASKREG: u32 = 0x10c;
pub const BCMA_CORE_PCI_DLLP_NEXTTXSEQNUMREG: u32 = 0x110;
pub const BCMA_CORE_PCI_DLLP_ACKEDTXSEQNUMREG: u32 = 0x114;
pub const BCMA_CORE_PCI_DLLP_PURGEDTXSEQNUMREG: u32 = 0x118;
pub const BCMA_CORE_PCI_DLLP_RXSEQNUMREG: u32 = 0x11c;
pub const BCMA_CORE_PCI_DLLP_LRREG: u32 = 0x120;
pub const BCMA_CORE_PCI_DLLP_LACKTOREG: u32 = 0x124;
pub const BCMA_CORE_PCI_DLLP_PMTHRESHREG: u32 = 0x128;
pub const BCMA_CORE_PCI_ASPMTIMER_EXTEND: u32 = 0x01000000;
pub const BCMA_CORE_PCI_DLLP_RTRYWPREG: u32 = 0x12c;
pub const BCMA_CORE_PCI_DLLP_RTRYRPREG: u32 = 0x130;
pub const BCMA_CORE_PCI_DLLP_RTRYPPREG: u32 = 0x134;
pub const BCMA_CORE_PCI_DLLP_RTRRWREG: u32 = 0x138;
pub const BCMA_CORE_PCI_DLLP_ECTHRESHREG: u32 = 0x13c;
pub const BCMA_CORE_PCI_DLLP_TLPERRCTRREG: u32 = 0x140;
pub const BCMA_CORE_PCI_DLLP_ERRCTRREG: u32 = 0x144;
pub const BCMA_CORE_PCI_DLLP_NAKRXCTRREG: u32 = 0x148;
pub const BCMA_CORE_PCI_DLLP_TESTREG: u32 = 0x14c;
pub const BCMA_CORE_PCI_DLLP_PKTBIST: u32 = 0x150;
pub const BCMA_CORE_PCI_DLLP_PCIE11: u32 = 0x154;

pub const BCMA_CORE_PCI_SERDES_RX_CTRL: u32 = 1;
pub const BCMA_CORE_PCI_SERDES_RX_CTRL_FORCE: u32 = 0x80;
pub const BCMA_CORE_PCI_SERDES_RX_CTRL_POLARITY: u32 = 0x40;
pub const BCMA_CORE_PCI_SERDES_RX_TIMER1: u32 = 2;
pub const BCMA_CORE_PCI_SERDES_RX_CDR: u32 = 6;
pub const BCMA_CORE_PCI_SERDES_RX_CDRBW: u32 = 7;
pub const BCMA_CORE_PCI_SERDES_PLL_CTRL: u32 = 1;
pub const BCMA_CORE_PCI_PLL_CTRL_FREQDET_EN: u32 = 0x4000;
pub const BCMA_CORE_PCI_BFL_NOPCI: u32 = 0x400;
pub const BCMA_CORE_PCI_CFG_BUS_SHIFT: u32 = 24;
pub const BCMA_CORE_PCI_CFG_SLOT_SHIFT: u32 = 19;
pub const BCMA_CORE_PCI_CFG_FUN_SHIFT: u32 = 16;
pub const BCMA_CORE_PCI_CFG_OFF_SHIFT: u32 = 0;
pub const BCMA_CORE_PCI_CFG_BUS_MASK: u32 = 0xff;
pub const BCMA_CORE_PCI_CFG_SLOT_MASK: u32 = 0x1f;
pub const BCMA_CORE_PCI_CFG_FUN_MASK: u32 = 7;
pub const BCMA_CORE_PCI_CFG_OFF_MASK: u32 = 0xfff;
pub const BCMA_CORE_PCI_CFG_DEVCTRL: u32 = 0xd8;
// The source contains an incomplete preprocessor token: BCMA_CORE_PCI_.

pub const BCMA_CORE_PCI_MDIO_IEEE0: u32 = 0;
pub const BCMA_CORE_PCI_MDIO_IEEE1: u32 = 1;
pub const BCMA_CORE_PCI_MDIO_BLK0: u32 = 0x800;
pub const BCMA_CORE_PCI_MDIO_BLK1: u32 = 0x801;
pub const BCMA_CORE_PCI_MDIO_BLK1_MGMT0: u32 = 0x16;
pub const BCMA_CORE_PCI_MDIO_BLK1_MGMT1: u32 = 0x17;
pub const BCMA_CORE_PCI_MDIO_BLK1_MGMT2: u32 = 0x18;
pub const BCMA_CORE_PCI_MDIO_BLK1_MGMT3: u32 = 0x19;
pub const BCMA_CORE_PCI_MDIO_BLK1_MGMT4: u32 = 0x1a;
pub const BCMA_CORE_PCI_MDIO_BLK2: u32 = 0x802;
pub const BCMA_CORE_PCI_MDIO_BLK3: u32 = 0x803;
pub const BCMA_CORE_PCI_MDIO_BLK4: u32 = 0x804;
pub const BCMA_CORE_PCI_MDIO_TXPLL: u32 = 0x808;
pub const BCMA_CORE_PCI_MDIO_TXCTRL0: u32 = 0x820;
pub const BCMA_CORE_PCI_MDIO_SERDESID: u32 = 0x831;
pub const BCMA_CORE_PCI_MDIO_RXCTRL0: u32 = 0x840;
pub const BCMA_CORE_PCI_RC_RRS_VISIBILITY: u32 = 1;

// CONFIG_BCMA_DRIVER_PCI_HOSTMODE conditional structure.
#[repr(C)]
pub struct bcma_drv_pci_host {
    pub pdev: *mut bcma_drv_pci,
    pub host_cfg_addr: u32,
    pub cfgspace_lock: c_void,
    pub pci_controller: c_void,
    pub pci_ops: c_void,
    pub mem_resource: c_void,
    pub io_resource: c_void,
}

#[repr(C)]
pub struct bcma_drv_pci {
    pub core: *mut bcma_device,
    pub early_setup_done: u8,
    pub setup_done: u8,
    pub hostmode: u8,
    pub host_controller: *mut bcma_drv_pci_host,
}

extern "C" {
    pub fn bcma_read16(core: *mut bcma_device, offset: u32) -> u16;
    pub fn bcma_read32(core: *mut bcma_device, offset: u32) -> u32;
    pub fn bcma_write16(core: *mut bcma_device, offset: u32, val: u16);
    pub fn bcma_write32(core: *mut bcma_device, offset: u32, val: u32);
}

#[inline] pub unsafe fn pcicore_read16(pc: *mut bcma_drv_pci, offset: u32) -> u16 { bcma_read16((*pc).core, offset) }
#[inline] pub unsafe fn pcicore_read32(pc: *mut bcma_drv_pci, offset: u32) -> u32 { bcma_read32((*pc).core, offset) }
#[inline] pub unsafe fn pcicore_write16(pc: *mut bcma_drv_pci, offset: u32, val: u16) { bcma_write16((*pc).core, offset, val) }
#[inline] pub unsafe fn pcicore_write32(pc: *mut bcma_drv_pci, offset: u32, val: u32) { bcma_write32((*pc).core, offset, val) }

// CONFIG_BCMA_DRIVER_PCI / CONFIG_BCMA_DRIVER_PCI_HOSTMODE conditionals.
extern "C" {
    pub fn bcma_core_pci_power_save(bus: *mut bcma_bus, up: bool);
    pub fn bcma_core_pci_pcibios_map_irq(dev: *const pci_dev) -> i32;
    pub fn bcma_core_pci_plat_dev_init(dev: *mut pci_dev) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
