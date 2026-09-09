/* SPDX-License-Identifier: GPL-2.0 */

/* Translation of <linux/types.h> and the PCI core declarations. */
/* CONFIG_SSB_DRIVER_PCICORE selects the implementation branch at build time. */

#[repr(C)]
pub struct ssb_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ssb_pcicore {
    #[cfg(feature = "CONFIG_SSB_DRIVER_PCICORE")]
    pub dev: *mut ssb_device,
    #[cfg(feature = "CONFIG_SSB_DRIVER_PCICORE")]
    pub setup_done: u8,
    #[cfg(feature = "CONFIG_SSB_DRIVER_PCICORE")]
    pub hostmode: u8,
    #[cfg(feature = "CONFIG_SSB_DRIVER_PCICORE")]
    pub cardbusmode: u8,
}

/* PCI core registers. */
pub const SSB_PCICORE_CTL: u32 = 0x0000;
pub const SSB_PCICORE_CTL_RST_OE: u32 = 0x00000001;
pub const SSB_PCICORE_CTL_RST: u32 = 0x00000002;
pub const SSB_PCICORE_CTL_CLK_OE: u32 = 0x00000004;
pub const SSB_PCICORE_CTL_CLK: u32 = 0x00000008;
pub const SSB_PCICORE_ARBCTL: u32 = 0x0010;
pub const SSB_PCICORE_ARBCTL_INTERN: u32 = 0x00000001;
pub const SSB_PCICORE_ARBCTL_EXTERN: u32 = 0x00000002;
pub const SSB_PCICORE_ARBCTL_PARKID: u32 = 0x00000006;
pub const SSB_PCICORE_ARBCTL_PARKID_LAST: u32 = 0x00000000;
pub const SSB_PCICORE_ARBCTL_PARKID_4710: u32 = 0x00000002;
pub const SSB_PCICORE_ARBCTL_PARKID_EXT0: u32 = 0x00000004;
pub const SSB_PCICORE_ARBCTL_PARKID_EXT1: u32 = 0x00000006;
pub const SSB_PCICORE_ISTAT: u32 = 0x0020;
pub const SSB_PCICORE_ISTAT_INTA: u32 = 0x00000001;
pub const SSB_PCICORE_ISTAT_INTB: u32 = 0x00000002;
pub const SSB_PCICORE_ISTAT_SERR: u32 = 0x00000004;
pub const SSB_PCICORE_ISTAT_PERR: u32 = 0x00000008;
pub const SSB_PCICORE_ISTAT_PME: u32 = 0x00000010;
pub const SSB_PCICORE_IMASK: u32 = 0x0024;
pub const SSB_PCICORE_IMASK_INTA: u32 = 0x00000001;
pub const SSB_PCICORE_IMASK_INTB: u32 = 0x00000002;
pub const SSB_PCICORE_IMASK_SERR: u32 = 0x00000004;
pub const SSB_PCICORE_IMASK_PERR: u32 = 0x00000008;
pub const SSB_PCICORE_IMASK_PME: u32 = 0x00000010;
pub const SSB_PCICORE_MBOX: u32 = 0x0028;
pub const SSB_PCICORE_MBOX_F0_0: u32 = 0x00000100;
pub const SSB_PCICORE_MBOX_F0_1: u32 = 0x00000200;
pub const SSB_PCICORE_MBOX_F1_0: u32 = 0x00000400;
pub const SSB_PCICORE_MBOX_F1_1: u32 = 0x00000800;
pub const SSB_PCICORE_MBOX_F2_0: u32 = 0x00001000;
pub const SSB_PCICORE_MBOX_F2_1: u32 = 0x00002000;
pub const SSB_PCICORE_MBOX_F3_0: u32 = 0x00004000;
pub const SSB_PCICORE_MBOX_F3_1: u32 = 0x00008000;
pub const SSB_PCICORE_BCAST_ADDR: u32 = 0x0050;
pub const SSB_PCICORE_BCAST_ADDR_MASK: u32 = 0x000000FF;
pub const SSB_PCICORE_BCAST_DATA: u32 = 0x0054;
pub const SSB_PCICORE_GPIO_IN: u32 = 0x0060;
pub const SSB_PCICORE_GPIO_OUT: u32 = 0x0064;
pub const SSB_PCICORE_GPIO_ENABLE: u32 = 0x0068;
pub const SSB_PCICORE_GPIO_CTL: u32 = 0x006C;
pub const SSB_PCICORE_SBTOPCI0: u32 = 0x0100;
pub const SSB_PCICORE_SBTOPCI0_MASK: u32 = 0xFC000000;
pub const SSB_PCICORE_SBTOPCI1: u32 = 0x0104;
pub const SSB_PCICORE_SBTOPCI1_MASK: u32 = 0xFC000000;
pub const SSB_PCICORE_SBTOPCI2: u32 = 0x0108;
pub const SSB_PCICORE_SBTOPCI2_MASK: u32 = 0xC0000000;
pub const SSB_PCICORE_PCICFG0: u32 = 0x0400;
pub const SSB_PCICORE_PCICFG1: u32 = 0x0500;
pub const SSB_PCICORE_PCICFG2: u32 = 0x0600;
pub const SSB_PCICORE_PCICFG3: u32 = 0x0700;
#[inline]
pub const fn SSB_PCICORE_SPROM(wordoffset: u32) -> u32 { 0x0800 + wordoffset * 2 }

/* SBtoPCIx */
pub const SSB_PCICORE_SBTOPCI_MEM: u32 = 0x00000000;
pub const SSB_PCICORE_SBTOPCI_IO: u32 = 0x00000001;
pub const SSB_PCICORE_SBTOPCI_CFG0: u32 = 0x00000002;
pub const SSB_PCICORE_SBTOPCI_CFG1: u32 = 0x00000003;
pub const SSB_PCICORE_SBTOPCI_PREF: u32 = 0x00000004;
pub const SSB_PCICORE_SBTOPCI_BURST: u32 = 0x00000008;
pub const SSB_PCICORE_SBTOPCI_MRM: u32 = 0x00000020;
pub const SSB_PCICORE_SBTOPCI_RC: u32 = 0x00000030;
pub const SSB_PCICORE_SBTOPCI_RC_READ: u32 = 0x00000000;
pub const SSB_PCICORE_SBTOPCI_RC_READL: u32 = 0x00000010;
pub const SSB_PCICORE_SBTOPCI_RC_READM: u32 = 0x00000020;

/* PCIcore specific boardflags */
pub const SSB_PCICORE_BFL_NOPCI: u32 = 0x00000400;

#[cfg(feature = "CONFIG_SSB_DRIVER_PCICORE")]
extern "C" {
    pub fn ssb_pcicore_init(pc: *mut ssb_pcicore);
    pub fn ssb_pcicore_dev_irqvecs_enable(pc: *mut ssb_pcicore, dev: *mut ssb_device) -> i32;
    pub fn ssb_pcicore_plat_dev_init(d: *mut pci_dev) -> i32;
    pub fn ssb_pcicore_pcibios_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32;
}

#[cfg(not(feature = "CONFIG_SSB_DRIVER_PCICORE"))]
pub unsafe fn ssb_pcicore_init(_pc: *mut ssb_pcicore) {}

#[cfg(not(feature = "CONFIG_SSB_DRIVER_PCICORE"))]
pub unsafe fn ssb_pcicore_dev_irqvecs_enable(_pc: *mut ssb_pcicore, _dev: *mut ssb_device) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_SSB_DRIVER_PCICORE"))]
pub unsafe fn ssb_pcicore_plat_dev_init(_d: *mut pci_dev) -> i32 { -ENODEV }

#[cfg(not(feature = "CONFIG_SSB_DRIVER_PCICORE"))]
pub unsafe fn ssb_pcicore_pcibios_map_irq(_dev: *const pci_dev, _slot: u8, _pin: u8) -> i32 { -ENODEV }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
