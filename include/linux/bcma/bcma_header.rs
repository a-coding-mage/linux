/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/bcma/bcma.h. External Linux types and symbols are dependencies. */

#[repr(C)]
pub enum bcma_hosttype { BCMA_HOSTTYPE_PCI, BCMA_HOSTTYPE_SDIO, BCMA_HOSTTYPE_SOC }

#[repr(C)]
pub struct bcma_chipinfo { pub id: u16, pub rev: u8, pub pkg: u8 }
#[repr(C)]
pub struct bcma_boardinfo { pub vendor: u16, pub r#type: u16 }
#[repr(C)]
pub enum bcma_clkmode { BCMA_CLKMODE_FAST, BCMA_CLKMODE_DYNAMIC }

#[repr(C)]
pub struct bcma_host_ops {
    pub read8: Option<unsafe extern "C" fn(*mut bcma_device, u16) -> u8>,
    pub read16: Option<unsafe extern "C" fn(*mut bcma_device, u16) -> u16>,
    pub read32: Option<unsafe extern "C" fn(*mut bcma_device, u16) -> u32>,
    pub write8: Option<unsafe extern "C" fn(*mut bcma_device, u16, u8)>,
    pub write16: Option<unsafe extern "C" fn(*mut bcma_device, u16, u16)>,
    pub write32: Option<unsafe extern "C" fn(*mut bcma_device, u16, u32)>,
    #[cfg(feature = "CONFIG_BCMA_BLOCKIO")]
    pub block_read: Option<unsafe extern "C" fn(*mut bcma_device, *mut core::ffi::c_void, usize, u16, u8)>,
    #[cfg(feature = "CONFIG_BCMA_BLOCKIO")]
    pub block_write: Option<unsafe extern "C" fn(*mut bcma_device, *const core::ffi::c_void, usize, u16, u8)>,
    pub aread32: Option<unsafe extern "C" fn(*mut bcma_device, u16) -> u32>,
    pub awrite32: Option<unsafe extern "C" fn(*mut bcma_device, u16, u32)>,
}

pub const BCMA_MANUF_ARM: u32 = 0x43B;
pub const BCMA_MANUF_MIPS: u32 = 0x4A7;
pub const BCMA_MANUF_BCM: u32 = 0x4BF;
pub const BCMA_CL_SIM: u32 = 0x0;
pub const BCMA_CL_EROM: u32 = 0x1;
pub const BCMA_CL_CORESIGHT: u32 = 0x9;
pub const BCMA_CL_VERIF: u32 = 0xB;
pub const BCMA_CL_OPTIMO: u32 = 0xD;
pub const BCMA_CL_GEN: u32 = 0xE;
pub const BCMA_CL_PRIMECELL: u32 = 0xF;

pub const BCMA_CORE_OOB_ROUTER: u32 = 0x367;
pub const BCMA_CORE_4706_CHIPCOMMON: u32 = 0x500;
pub const BCMA_CORE_NS_PCIEG2: u32 = 0x501;
pub const BCMA_CORE_NS_DMA: u32 = 0x502;
pub const BCMA_CORE_NS_SDIO3: u32 = 0x503;
pub const BCMA_CORE_NS_USB20: u32 = 0x504;
pub const BCMA_CORE_NS_USB30: u32 = 0x505;
pub const BCMA_CORE_NS_A9JTAG: u32 = 0x506;
pub const BCMA_CORE_NS_DDR23: u32 = 0x507;
pub const BCMA_CORE_NS_ROM: u32 = 0x508;
pub const BCMA_CORE_NS_NAND: u32 = 0x509;
pub const BCMA_CORE_NS_QSPI: u32 = 0x50A;
pub const BCMA_CORE_NS_CHIPCOMMON_B: u32 = 0x50B;
pub const BCMA_CORE_4706_SOC_RAM: u32 = 0x50E;
pub const BCMA_CORE_ARMCA9: u32 = 0x510;
pub const BCMA_CORE_4706_MAC_GBIT: u32 = 0x52D;
pub const BCMA_CORE_AMEMC: u32 = 0x52E;
pub const BCMA_CORE_ALTA: u32 = 0x534;
pub const BCMA_CORE_4706_MAC_GBIT_COMMON: u32 = 0x5DC;
pub const BCMA_CORE_DDR23_PHY: u32 = 0x5DD;
pub const BCMA_CORE_INVALID: u32 = 0x700;
pub const BCMA_CORE_CHIPCOMMON: u32 = 0x800;
pub const BCMA_CORE_ILINE20: u32 = 0x801;
pub const BCMA_CORE_SRAM: u32 = 0x802;
pub const BCMA_CORE_SDRAM: u32 = 0x803;
pub const BCMA_CORE_PCI: u32 = 0x804;
pub const BCMA_CORE_MIPS: u32 = 0x805;
pub const BCMA_CORE_ETHERNET: u32 = 0x806;
pub const BCMA_CORE_V90: u32 = 0x807;
pub const BCMA_CORE_USB11_HOSTDEV: u32 = 0x808;
pub const BCMA_CORE_ADSL: u32 = 0x809;
pub const BCMA_CORE_ILINE100: u32 = 0x80A;
pub const BCMA_CORE_IPSEC: u32 = 0x80B;
pub const BCMA_CORE_UTOPIA: u32 = 0x80C;
pub const BCMA_CORE_PCMCIA: u32 = 0x80D;
pub const BCMA_CORE_INTERNAL_MEM: u32 = 0x80E;
pub const BCMA_CORE_MEMC_SDRAM: u32 = 0x80F;
pub const BCMA_CORE_OFDM: u32 = 0x810;
pub const BCMA_CORE_EXTIF: u32 = 0x811;
pub const BCMA_CORE_80211: u32 = 0x812;
pub const BCMA_CORE_PHY_A: u32 = 0x813;
pub const BCMA_CORE_PHY_B: u32 = 0x814;
pub const BCMA_CORE_PHY_G: u32 = 0x815;
pub const BCMA_CORE_MIPS_3302: u32 = 0x816;
pub const BCMA_CORE_USB11_HOST: u32 = 0x817;
pub const BCMA_CORE_USB11_DEV: u32 = 0x818;
pub const BCMA_CORE_USB20_HOST: u32 = 0x819;
pub const BCMA_CORE_USB20_DEV: u32 = 0x81A;
pub const BCMA_CORE_SDIO_HOST: u32 = 0x81B;
pub const BCMA_CORE_ROBOSWITCH: u32 = 0x81C;
pub const BCMA_CORE_PARA_ATA: u32 = 0x81D;
pub const BCMA_CORE_SATA_XORDMA: u32 = 0x81E;
pub const BCMA_CORE_ETHERNET_GBIT: u32 = 0x81F;
pub const BCMA_CORE_PCIE: u32 = 0x820;
pub const BCMA_CORE_PHY_N: u32 = 0x821;
pub const BCMA_CORE_SRAM_CTL: u32 = 0x822;
pub const BCMA_CORE_MINI_MACPHY: u32 = 0x823;
pub const BCMA_CORE_ARM_1176: u32 = 0x824;
pub const BCMA_CORE_ARM_7TDMI: u32 = 0x825;
pub const BCMA_CORE_PHY_LP: u32 = 0x826;
pub const BCMA_CORE_PMU: u32 = 0x827;
pub const BCMA_CORE_PHY_SSN: u32 = 0x828;
pub const BCMA_CORE_SDIO_DEV: u32 = 0x829;
pub const BCMA_CORE_ARM_CM3: u32 = 0x82A;
pub const BCMA_CORE_PHY_HT: u32 = 0x82B;
pub const BCMA_CORE_MIPS_74K: u32 = 0x82C;
pub const BCMA_CORE_MAC_GBIT: u32 = 0x82D;
pub const BCMA_CORE_DDR12_MEM_CTL: u32 = 0x82E;
pub const BCMA_CORE_PCIE_RC: u32 = 0x82F;
pub const BCMA_CORE_OCP_OCP_BRIDGE: u32 = 0x830;
pub const BCMA_CORE_SHARED_COMMON: u32 = 0x831;
pub const BCMA_CORE_OCP_AHB_BRIDGE: u32 = 0x832;
pub const BCMA_CORE_SPI_HOST: u32 = 0x833;
pub const BCMA_CORE_I2S: u32 = 0x834;
pub const BCMA_CORE_SDR_DDR1_MEM_CTL: u32 = 0x835;
pub const BCMA_CORE_SHIM: u32 = 0x837;
pub const BCMA_CORE_PHY_AC: u32 = 0x83B;
pub const BCMA_CORE_PCIE2: u32 = 0x83C;
pub const BCMA_CORE_USB30_DEV: u32 = 0x83D;
pub const BCMA_CORE_ARM_CR4: u32 = 0x83E;
pub const BCMA_CORE_GCI: u32 = 0x840;
pub const BCMA_CORE_CMEM: u32 = 0x846;
pub const BCMA_CORE_ARM_CA7: u32 = 0x847;
pub const BCMA_CORE_SYS_MEM: u32 = 0x849;
pub const BCMA_CORE_DEFAULT: u32 = 0xFFF;
pub const BCMA_MAX_NR_CORES: usize = 16;
pub const BCMA_CORE_SIZE: usize = 0x1000;

/* Chip, package, and board constants. */
pub const BCMA_CHIP_ID_BCM4313: u32 = 0x4313;
pub const BCMA_CHIP_ID_BCM43142: u32 = 43142;
pub const BCMA_CHIP_ID_BCM43131: u32 = 43131;
pub const BCMA_CHIP_ID_BCM43217: u32 = 43217;
pub const BCMA_CHIP_ID_BCM43222: u32 = 43222;
pub const BCMA_CHIP_ID_BCM43224: u32 = 43224;
pub const BCMA_PKG_ID_BCM43224_FAB_CSM: u32 = 0x8;
pub const BCMA_PKG_ID_BCM43224_FAB_SMIC: u32 = 0xa;
pub const BCMA_CHIP_ID_BCM43225: u32 = 43225;
pub const BCMA_CHIP_ID_BCM43227: u32 = 43227;
pub const BCMA_CHIP_ID_BCM43228: u32 = 43228;
pub const BCMA_CHIP_ID_BCM43421: u32 = 43421;
pub const BCMA_CHIP_ID_BCM43428: u32 = 43428;
pub const BCMA_CHIP_ID_BCM43431: u32 = 43431;
pub const BCMA_CHIP_ID_BCM43460: u32 = 43460;
pub const BCMA_CHIP_ID_BCM4331: u32 = 0x4331;
pub const BCMA_CHIP_ID_BCM6362: u32 = 0x6362;
pub const BCMA_CHIP_ID_BCM4360: u32 = 0x4360;
pub const BCMA_CHIP_ID_BCM4352: u32 = 0x4352;
pub const BCMA_CHIP_ID_BCM4706: u32 = 0x5300;
pub const BCMA_PKG_ID_BCM4706L: u32 = 1;
pub const BCMA_CHIP_ID_BCM4716: u32 = 0x4716;
pub const BCMA_PKG_ID_BCM4716: u32 = 8;
pub const BCMA_PKG_ID_BCM4717: u32 = 9;
pub const BCMA_PKG_ID_BCM4718: u32 = 10;
pub const BCMA_CHIP_ID_BCM47162: u32 = 47162;
pub const BCMA_CHIP_ID_BCM4748: u32 = 0x4748;
pub const BCMA_CHIP_ID_BCM4749: u32 = 0x4749;
pub const BCMA_CHIP_ID_BCM5356: u32 = 0x5356;
pub const BCMA_CHIP_ID_BCM5357: u32 = 0x5357;
pub const BCMA_PKG_ID_BCM5358: u32 = 9;
pub const BCMA_PKG_ID_BCM47186: u32 = 10;
pub const BCMA_PKG_ID_BCM5357: u32 = 11;
pub const BCMA_CHIP_ID_BCM53572: u32 = 53572;
pub const BCMA_PKG_ID_BCM47188: u32 = 9;
pub const BCMA_CHIP_ID_BCM4707: u32 = 53010;
pub const BCMA_PKG_ID_BCM4707: u32 = 1;
pub const BCMA_PKG_ID_BCM4708: u32 = 2;
pub const BCMA_PKG_ID_BCM4709: u32 = 0;
pub const BCMA_CHIP_ID_BCM47094: u32 = 53030;
pub const BCMA_CHIP_ID_BCM53018: u32 = 53018;
pub const BCMA_CHIP_ID_BCM53573: u32 = 53573;
pub const BCMA_PKG_ID_BCM53573: u32 = 0;
pub const BCMA_PKG_ID_BCM47189: u32 = 1;

pub const BCMA_BOARD_TYPE_BCM94313BU: u32 = 0x050F;
pub const BCMA_BOARD_TYPE_BCM94313HM: u32 = 0x0510;
pub const BCMA_BOARD_TYPE_BCM94313EPA: u32 = 0x0511;
pub const BCMA_BOARD_TYPE_BCM94313HMG: u32 = 0x051C;
pub const BCMA_BOARD_TYPE_BCM94716NR2: u32 = 0x04CD;
pub const BCMA_BOARD_TYPE_BCM943224X21: u32 = 0x056E;
pub const BCMA_BOARD_TYPE_BCM943224X21_FCC: u32 = 0x00D1;
pub const BCMA_BOARD_TYPE_BCM943224X21B: u32 = 0x00E9;
pub const BCMA_BOARD_TYPE_BCM943224M93: u32 = 0x008B;
pub const BCMA_BOARD_TYPE_BCM943224M93A: u32 = 0x0090;
pub const BCMA_BOARD_TYPE_BCM943224X16: u32 = 0x0093;
pub const BCMA_BOARD_TYPE_BCM94322X9: u32 = 0x008D;
pub const BCMA_BOARD_TYPE_BCM94322M35E: u32 = 0x008E;
pub const BCMA_BOARD_TYPE_BCM943228BU8: u32 = 0x0540;
pub const BCMA_BOARD_TYPE_BCM943228BU9: u32 = 0x0541;
pub const BCMA_BOARD_TYPE_BCM943228BU: u32 = 0x0542;
pub const BCMA_BOARD_TYPE_BCM943227HM4L: u32 = 0x0543;
pub const BCMA_BOARD_TYPE_BCM943227HMB: u32 = 0x0544;
pub const BCMA_BOARD_TYPE_BCM943228HM4L: u32 = 0x0545;
pub const BCMA_BOARD_TYPE_BCM943228SD: u32 = 0x0573;
pub const BCMA_BOARD_TYPE_BCM94331X19: u32 = 0x00D6;
pub const BCMA_BOARD_TYPE_BCM94331X28: u32 = 0x00E4;
pub const BCMA_BOARD_TYPE_BCM94331X28B: u32 = 0x010E;
pub const BCMA_BOARD_TYPE_BCM94331PCIEBT3AX: u32 = 0x00E4;
pub const BCMA_BOARD_TYPE_BCM94331X12_2G: u32 = 0x00EC;
pub const BCMA_BOARD_TYPE_BCM94331X12_5G: u32 = 0x00ED;
pub const BCMA_BOARD_TYPE_BCM94331X29B: u32 = 0x00EF;
pub const BCMA_BOARD_TYPE_BCM94331CSAX: u32 = 0x00EF;
pub const BCMA_BOARD_TYPE_BCM94331X19C: u32 = 0x00F5;
pub const BCMA_BOARD_TYPE_BCM94331X33: u32 = 0x00F4;
pub const BCMA_BOARD_TYPE_BCM94331BU: u32 = 0x0523;
pub const BCMA_BOARD_TYPE_BCM94331S9BU: u32 = 0x0524;
pub const BCMA_BOARD_TYPE_BCM94331MC: u32 = 0x0525;
pub const BCMA_BOARD_TYPE_BCM94331MCI: u32 = 0x0526;
pub const BCMA_BOARD_TYPE_BCM94331PCIEBT4: u32 = 0x0527;
pub const BCMA_BOARD_TYPE_BCM94331HM: u32 = 0x0574;
pub const BCMA_BOARD_TYPE_BCM94331PCIEDUAL: u32 = 0x059B;
pub const BCMA_BOARD_TYPE_BCM94331MCH5: u32 = 0x05A9;
pub const BCMA_BOARD_TYPE_BCM94331CS: u32 = 0x05C6;
pub const BCMA_BOARD_TYPE_BCM94331CD: u32 = 0x05DA;
pub const BCMA_BOARD_TYPE_BCM953572BU: u32 = 0x058D;
pub const BCMA_BOARD_TYPE_BCM953572NR2: u32 = 0x058E;
pub const BCMA_BOARD_TYPE_BCM947188NR2: u32 = 0x058F;
pub const BCMA_BOARD_TYPE_BCM953572SDRNR2: u32 = 0x0590;
pub const BCMA_BOARD_TYPE_BCM943142HM: u32 = 0x05E0;

#[repr(C)]
pub struct bcma_device {
    pub bus: *mut bcma_bus, pub id: bcma_device_id, pub dev: device, pub dma_dev: *mut device,
    pub irq: core::ffi::c_uint, pub dev_registered: bool, pub core_index: u8, pub core_unit: u8,
    pub addr: u32, pub addr_s: [u32; 8], pub wrap: u32, pub io_addr: *mut core::ffi::c_void,
    pub io_wrap: *mut core::ffi::c_void, pub drvdata: *mut core::ffi::c_void,
    pub list: list_head,
}
pub unsafe fn bcma_get_drvdata(core: *mut bcma_device) -> *mut core::ffi::c_void { (*core).drvdata }
pub unsafe fn bcma_set_drvdata(core: *mut bcma_device, drvdata: *mut core::ffi::c_void) { (*core).drvdata = drvdata; }

#[repr(C)]
pub struct bcma_driver {
    pub name: *const core::ffi::c_char, pub id_table: *const bcma_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut bcma_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut bcma_device)>,
    pub suspend: Option<unsafe extern "C" fn(*mut bcma_device) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut bcma_device) -> i32>,
    pub shutdown: Option<unsafe extern "C" fn(*mut bcma_device)>, pub drv: device_driver,
}
extern "C" { pub fn __bcma_driver_register(drv: *mut bcma_driver, owner: *mut module) -> i32; pub fn bcma_driver_unregister(drv: *mut bcma_driver); }
/* bcma_driver_register(drv) expands to __bcma_driver_register(drv, THIS_MODULE). */
/* module_bcma_driver is a module_driver wrapper and remains build-system dependent. */
extern "C" { pub fn bcma_arch_register_fallback_sprom(cb: Option<unsafe extern "C" fn(*mut bcma_bus, *mut ssb_sprom) -> i32>) -> i32; }

#[repr(C)]
pub struct bcma_bus {
    pub dev: *mut device, pub mmio: *mut core::ffi::c_void, pub ops: *const bcma_host_ops,
    pub hosttype: bcma_hosttype, pub host_is_pcie2: bool, pub host_pci: *mut pci_dev,
    pub chipinfo: bcma_chipinfo, pub boardinfo: bcma_boardinfo, pub mapped_core: *mut bcma_device,
    pub cores: list_head, pub nr_cores: u8, pub num: u8, pub drv_cc: bcma_drv_cc,
    pub drv_cc_b: bcma_drv_cc_b, pub drv_pci: [bcma_drv_pci; 2], pub drv_pcie2: bcma_drv_pcie2,
    pub drv_mips: bcma_drv_mips, pub drv_gmac_cmn: bcma_drv_gmac_cmn, pub sprom: ssb_sprom,
}

pub unsafe fn bcma_read8(core: *mut bcma_device, offset: u16) -> u32 { ((*(*core).bus).ops).as_ref().unwrap().read8.unwrap()(core, offset) as u32 }
pub unsafe fn bcma_read16(core: *mut bcma_device, offset: u16) -> u32 { ((*(*core).bus).ops).as_ref().unwrap().read16.unwrap()(core, offset) as u32 }
pub unsafe fn bcma_read32(core: *mut bcma_device, offset: u16) -> u32 { ((*(*core).bus).ops).as_ref().unwrap().read32.unwrap()(core, offset) }
pub unsafe fn bcma_write8(core: *mut bcma_device, offset: u16, value: u32) { ((*(*core).bus).ops).as_ref().unwrap().write8.unwrap()(core, offset, value as u8); }
pub unsafe fn bcma_write16(core: *mut bcma_device, offset: u16, value: u32) { ((*(*core).bus).ops).as_ref().unwrap().write16.unwrap()(core, offset, value as u16); }
pub unsafe fn bcma_write32(core: *mut bcma_device, offset: u16, value: u32) { ((*(*core).bus).ops).as_ref().unwrap().write32.unwrap()(core, offset, value); }
pub unsafe fn bcma_aread32(core: *mut bcma_device, offset: u16) -> u32 { ((*(*core).bus).ops).as_ref().unwrap().aread32.unwrap()(core, offset) }
pub unsafe fn bcma_awrite32(core: *mut bcma_device, offset: u16, value: u32) { ((*(*core).bus).ops).as_ref().unwrap().awrite32.unwrap()(core, offset, value); }
#[cfg(feature = "CONFIG_BCMA_BLOCKIO")]
pub unsafe fn bcma_block_read(core: *mut bcma_device, buffer: *mut core::ffi::c_void, count: usize, offset: u16, reg_width: u8) { ((*(*core).bus).ops).as_ref().unwrap().block_read.unwrap()(core, buffer, count, offset, reg_width); }
#[cfg(feature = "CONFIG_BCMA_BLOCKIO")]
pub unsafe fn bcma_block_write(core: *mut bcma_device, buffer: *const core::ffi::c_void, count: usize, offset: u16, reg_width: u8) { ((*(*core).bus).ops).as_ref().unwrap().block_write.unwrap()(core, buffer, count, offset, reg_width); }
pub unsafe fn bcma_mask32(cc: *mut bcma_device, offset: u16, mask: u32) { bcma_write32(cc, offset, bcma_read32(cc, offset) & mask); }
pub unsafe fn bcma_set32(cc: *mut bcma_device, offset: u16, set: u32) { bcma_write32(cc, offset, bcma_read32(cc, offset) | set); }
pub unsafe fn bcma_maskset32(cc: *mut bcma_device, offset: u16, mask: u32, set: u32) { bcma_write32(cc, offset, (bcma_read32(cc, offset) & mask) | set); }
pub unsafe fn bcma_mask16(cc: *mut bcma_device, offset: u16, mask: u16) { bcma_write16(cc, offset, bcma_read16(cc, offset) & mask as u32); }
pub unsafe fn bcma_set16(cc: *mut bcma_device, offset: u16, set: u16) { bcma_write16(cc, offset, bcma_read16(cc, offset) | set as u32); }
pub unsafe fn bcma_maskset16(cc: *mut bcma_device, offset: u16, mask: u16, set: u16) { bcma_write16(cc, offset, (bcma_read16(cc, offset) & mask as u32) | set as u32); }

extern "C" {
    pub fn bcma_find_core_unit(bus: *mut bcma_bus, coreid: u16, unit: u8) -> *mut bcma_device;
    pub fn bcma_core_is_enabled(core: *mut bcma_device) -> bool;
    pub fn bcma_core_disable(core: *mut bcma_device, flags: u32);
    pub fn bcma_core_enable(core: *mut bcma_device, flags: u32) -> i32;
    pub fn bcma_core_set_clockmode(core: *mut bcma_device, clkmode: bcma_clkmode);
    pub fn bcma_core_pll_ctl(core: *mut bcma_device, req: u32, status: u32, on: bool);
    pub fn bcma_chipco_pll_read(cc: *mut bcma_drv_cc, offset: u32) -> u32;
    pub fn bcma_core_dma_translation(core: *mut bcma_device) -> u32;
    pub fn bcma_core_irq(core: *mut bcma_device, num: i32) -> core::ffi::c_uint;
}
pub unsafe fn bcma_find_core(bus: *mut bcma_bus, coreid: u16) -> *mut bcma_device { bcma_find_core_unit(bus, coreid, 0) }
/* CONFIG_BCMA_HOST_PCI selects external implementations; otherwise these are no-op stubs. */
#[cfg(feature = "CONFIG_BCMA_HOST_PCI")]
extern "C" { pub fn bcma_host_pci_up(bus: *mut bcma_bus); pub fn bcma_host_pci_down(bus: *mut bcma_bus); pub fn bcma_host_pci_irq_ctl(bus: *mut bcma_bus, core: *mut bcma_device, enable: bool) -> i32; }
#[cfg(not(feature = "CONFIG_BCMA_HOST_PCI"))]
pub unsafe fn bcma_host_pci_up(_bus: *mut bcma_bus) {}
#[cfg(not(feature = "CONFIG_BCMA_HOST_PCI"))]
pub unsafe fn bcma_host_pci_down(_bus: *mut bcma_bus) {}
#[cfg(not(feature = "CONFIG_BCMA_HOST_PCI"))]
pub unsafe fn bcma_host_pci_irq_ctl(bus: *mut bcma_bus, _core: *mut bcma_device, _enable: bool) -> i32 {
    if (*bus).hosttype as u32 == BCMA_HOSTTYPE_PCI as u32 { -95 } else { 0 }
}
pub const BCMA_DMA_TRANSLATION_MASK: u32 = 0xC0000000;
pub const BCMA_DMA_TRANSLATION_NONE: u32 = 0x00000000;
pub const BCMA_DMA_TRANSLATION_DMA32_CMT: u32 = 0x40000000;
pub const BCMA_DMA_TRANSLATION_DMA64_CMT: u32 = 0x80000000;
extern "C" { pub static bcma_gpio_swnode: software_node; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
