/*
 * Broadcom specific AMBA
 * Bus scanning
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

#[repr(C)]
struct BcmaDeviceIdName {
    id: u16,
    name: *const core::ffi::c_char,
}

static BCMA_ARM_DEVICE_NAMES: &[BcmaDeviceIdName] = &[
    BcmaDeviceIdName { id: BCMA_CORE_4706_MAC_GBIT_COMMON, name: c"BCM4706 GBit MAC Common".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_ARM_1176, name: c"ARM 1176".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_ARM_7TDMI, name: c"ARM 7TDMI".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_ARM_CM3, name: c"ARM CM3".as_ptr() },
];

static BCMA_BCM_DEVICE_NAMES: &[BcmaDeviceIdName] = &[
    BcmaDeviceIdName { id: BCMA_CORE_OOB_ROUTER, name: c"OOB Router".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_4706_CHIPCOMMON, name: c"BCM4706 ChipCommon".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_4706_SOC_RAM, name: c"BCM4706 SOC RAM".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_4706_MAC_GBIT, name: c"BCM4706 GBit MAC".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_NS_PCIEG2, name: c"PCIe Gen 2".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_NS_DMA, name: c"DMA".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_NS_SDIO3, name: c"SDIO3".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_NS_USB20, name: c"USB 2.0".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_NS_USB30, name: c"USB 3.0".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_NS_A9JTAG, name: c"ARM Cortex A9 JTAG".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_NS_DDR23, name: c"Denali DDR2/DDR3 memory controller".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_NS_ROM, name: c"ROM".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_NS_NAND, name: c"NAND flash controller".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_NS_QSPI, name: c"SPI flash controller".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_NS_CHIPCOMMON_B, name: c"Chipcommon B".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_ARMCA9, name: c"ARM Cortex A9 core (ihost)".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_AMEMC, name: c"AMEMC (DDR)".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_ALTA, name: c"ALTA (I2S)".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_INVALID, name: c"Invalid".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_CHIPCOMMON, name: c"ChipCommon".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_ILINE20, name: c"ILine 20".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_SRAM, name: c"SRAM".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_SDRAM, name: c"SDRAM".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_PCI, name: c"PCI".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_ETHERNET, name: c"Fast Ethernet".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_V90, name: c"V90".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_USB11_HOSTDEV, name: c"USB 1.1 Hostdev".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_ADSL, name: c"ADSL".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_ILINE100, name: c"ILine 100".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_IPSEC, name: c"IPSEC".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_UTOPIA, name: c"UTOPIA".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_PCMCIA, name: c"PCMCIA".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_INTERNAL_MEM, name: c"Internal Memory".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_MEMC_SDRAM, name: c"MEMC SDRAM".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_OFDM, name: c"OFDM".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_EXTIF, name: c"EXTIF".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_80211, name: c"IEEE 802.11".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_PHY_A, name: c"PHY A".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_PHY_B, name: c"PHY B".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_PHY_G, name: c"PHY G".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_USB11_HOST, name: c"USB 1.1 Host".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_USB11_DEV, name: c"USB 1.1 Device".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_USB20_HOST, name: c"USB 2.0 Host".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_USB20_DEV, name: c"USB 2.0 Device".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_SDIO_HOST, name: c"SDIO Host".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_ROBOSWITCH, name: c"Roboswitch".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_PARA_ATA, name: c"PATA".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_SATA_XORDMA, name: c"SATA XOR-DMA".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_ETHERNET_GBIT, name: c"GBit Ethernet".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_PCIE, name: c"PCIe".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_PHY_N, name: c"PHY N".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_SRAM_CTL, name: c"SRAM Controller".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_MINI_MACPHY, name: c"Mini MACPHY".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_PHY_LP, name: c"PHY LP".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_PMU, name: c"PMU".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_PHY_SSN, name: c"PHY SSN".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_SDIO_DEV, name: c"SDIO Device".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_PHY_HT, name: c"PHY HT".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_MAC_GBIT, name: c"GBit MAC".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_DDR12_MEM_CTL, name: c"DDR1/DDR2 Memory Controller".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_PCIE_RC, name: c"PCIe Root Complex".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_OCP_OCP_BRIDGE, name: c"OCP to OCP Bridge".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_SHARED_COMMON, name: c"Common Shared".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_OCP_AHB_BRIDGE, name: c"OCP to AHB Bridge".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_SPI_HOST, name: c"SPI Host".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_I2S, name: c"I2S".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_SDR_DDR1_MEM_CTL, name: c"SDR/DDR1 Memory Controller".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_SHIM, name: c"SHIM".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_PCIE2, name: c"PCIe Gen2".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_ARM_CR4, name: c"ARM CR4".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_GCI, name: c"GCI".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_CMEM, name: c"CNDS DDR2/3 memory controller".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_ARM_CA7, name: c"ARM CA7".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_DEFAULT, name: c"Default".as_ptr() },
];

static BCMA_MIPS_DEVICE_NAMES: &[BcmaDeviceIdName] = &[
    BcmaDeviceIdName { id: BCMA_CORE_MIPS, name: c"MIPS".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_MIPS_3302, name: c"MIPS 3302".as_ptr() },
    BcmaDeviceIdName { id: BCMA_CORE_MIPS_74K, name: c"MIPS 74K".as_ptr() },
];

unsafe fn bcma_device_name(id: *const bcma_device_id) -> *const core::ffi::c_char {
    let names: &[BcmaDeviceIdName] = match (*id).manuf {
        BCMA_MANUF_ARM => BCMA_ARM_DEVICE_NAMES,
        BCMA_MANUF_BCM => BCMA_BCM_DEVICE_NAMES,
        BCMA_MANUF_MIPS => BCMA_MIPS_DEVICE_NAMES,
        _ => return c"UNKNOWN".as_ptr(),
    };
    for name in names {
        if name.id == (*id).id { return name.name; }
    }
    c"UNKNOWN".as_ptr()
}

unsafe fn bcma_scan_read32(bus: *mut bcma_bus, offset: u16) -> u32 {
    readl((*bus).mmio.add(offset as usize))
}

unsafe fn bcma_scan_switch_core(bus: *mut bcma_bus, addr: u32) {
    if (*bus).hosttype == BCMA_HOSTTYPE_PCI {
        pci_write_config_dword((*bus).host_pci, BCMA_PCI_BAR0_WIN, addr);
    }
}

unsafe fn bcma_erom_get_ent(_bus: *mut bcma_bus, eromptr: *mut *mut u32) -> u32 {
    let ent = readl(*eromptr);
    *eromptr = (*eromptr).add(1);
    ent
}

unsafe fn bcma_erom_push_ent(_bus: *mut bcma_bus, eromptr: *mut *mut u32) { *eromptr = (*eromptr).sub(1); }

unsafe fn bcma_erom_get_ci(bus: *mut bcma_bus, eromptr: *mut *mut u32) -> i32 {
    let ent = bcma_erom_get_ent(bus, eromptr);
    if ent & SCAN_ER_VALID == 0 || ent & SCAN_ER_TAG != SCAN_ER_TAG_CI { return -ENOENT; }
    ent as i32
}

unsafe fn bcma_erom_is_end(bus: *mut bcma_bus, eromptr: *mut *mut u32) -> bool {
    let ent = bcma_erom_get_ent(bus, eromptr); bcma_erom_push_ent(bus, eromptr); ent == (SCAN_ER_TAG_END | SCAN_ER_VALID)
}

unsafe fn bcma_erom_is_bridge(bus: *mut bcma_bus, eromptr: *mut *mut u32) -> bool {
    let ent = bcma_erom_get_ent(bus, eromptr); bcma_erom_push_ent(bus, eromptr);
    ent & SCAN_ER_VALID != 0 && ent & SCAN_ER_TAGX == SCAN_ER_TAG_ADDR && ent & SCAN_ADDR_TYPE == SCAN_ADDR_TYPE_BRIDGE
}

unsafe fn bcma_erom_skip_component(bus: *mut bcma_bus, eromptr: *mut *mut u32) {
    loop { let ent = bcma_erom_get_ent(bus, eromptr); if (ent & SCAN_ER_VALID != 0 && ent & SCAN_ER_TAG == SCAN_ER_TAG_CI) || ent == (SCAN_ER_TAG_END | SCAN_ER_VALID) { break; } }
    bcma_erom_push_ent(bus, eromptr);
}

unsafe fn bcma_erom_get_mst_port(bus: *mut bcma_bus, eromptr: *mut *mut u32) -> i32 {
    let ent = bcma_erom_get_ent(bus, eromptr); if ent & SCAN_ER_VALID == 0 || ent & SCAN_ER_TAG != SCAN_ER_TAG_MP { return -ENOENT; } ent as i32
}

unsafe fn bcma_erom_get_addr_desc(bus: *mut bcma_bus, eromptr: *mut *mut u32, typ: u32, port: u8) -> u32 {
    let ent = bcma_erom_get_ent(bus, eromptr);
    if ent & SCAN_ER_VALID == 0 || ent & SCAN_ER_TAGX != SCAN_ER_TAG_ADDR || ent & SCAN_ADDR_TYPE != typ || ((ent & SCAN_ADDR_PORT) >> SCAN_ADDR_PORT_SHIFT) != port as u32 { bcma_erom_push_ent(bus, eromptr); return (-EINVAL) as u32; }
    let addrl = ent & SCAN_ADDR_ADDR;
    if ent & SCAN_ADDR_AG32 != 0 { bcma_erom_get_ent(bus, eromptr); }
    if ent & SCAN_ADDR_SZ == SCAN_ADDR_SZ_SZD { let size = bcma_erom_get_ent(bus, eromptr); if size & SCAN_SIZE_SG32 != 0 { bcma_erom_get_ent(bus, eromptr); } }
    addrl
}

// The remaining declarations and scan routine retain the kernel list and allocation APIs.
unsafe fn bcma_find_core_by_index(bus: *mut bcma_bus, index: u16) -> *mut bcma_device { list_for_each_entry_core(bus, index) }
unsafe fn bcma_find_core_reverse(bus: *mut bcma_bus, coreid: u16) -> *mut bcma_device { list_for_each_entry_reverse_core(bus, coreid) }

unsafe fn bcma_get_next_core(bus: *mut bcma_bus, eromptr: *mut *mut u32, match_id: *const bcma_device_id, core_num: i32, core: *mut bcma_device) -> i32 {
    let cia = bcma_erom_get_ci(bus, eromptr); if cia < 0 { bcma_erom_push_ent(bus, eromptr); if bcma_erom_is_end(bus, eromptr) { return -ESPIPE; } return -EILSEQ; }
    let cib = bcma_erom_get_ci(bus, eromptr); if cib < 0 { return -EILSEQ; }
    (*core).id.class = ((cia as u32 & SCAN_CIA_CLASS) >> SCAN_CIA_CLASS_SHIFT) as _;
    (*core).id.id = ((cia as u32 & SCAN_CIA_ID) >> SCAN_CIA_ID_SHIFT) as _;
    (*core).id.manuf = ((cia as u32 & SCAN_CIA_MANUF) >> SCAN_CIA_MANUF_SHIFT) as _;
    let ports = [((cib as u32 & SCAN_CIB_NMP) >> SCAN_CIB_NMP_SHIFT) as u8, ((cib as u32 & SCAN_CIB_NSP) >> SCAN_CIB_NSP_SHIFT) as u8];
    let wrappers = [((cib as u32 & SCAN_CIB_NMW) >> SCAN_CIB_NMW_SHIFT) as u8, ((cib as u32 & SCAN_CIB_NSW) >> SCAN_CIB_NSW_SHIFT) as u8];
    (*core).id.rev = ((cib as u32 & SCAN_CIB_REV) >> SCAN_CIB_REV_SHIFT) as _;
    if ((*core).id.manuf == BCMA_MANUF_ARM && (*core).id.id == 0xFFF) || ports[1] == 0 { bcma_erom_skip_component(bus, eromptr); return -ENXIO; }
    if wrappers[0] + wrappers[1] == 0 { match (*core).id.id { BCMA_CORE_4706_MAC_GBIT_COMMON | BCMA_CORE_NS_CHIPCOMMON_B | BCMA_CORE_PMU | BCMA_CORE_GCI => {}, _ => { bcma_erom_skip_component(bus, eromptr); return -ENXIO; } } }
    if bcma_erom_is_bridge(bus, eromptr) { bcma_erom_skip_component(bus, eromptr); return -ENXIO; }
    if !bcma_find_core_by_index(bus, core_num as u16).is_null() { bcma_erom_skip_component(bus, eromptr); return -ENODEV; }
    if !match_id.is_null() && (((*match_id).manuf != BCMA_ANY_MANUF && (*match_id).manuf != (*core).id.manuf) || ((*match_id).id != BCMA_ANY_ID && (*match_id).id != (*core).id.id) || ((*match_id).rev != BCMA_ANY_REV && (*match_id).rev != (*core).id.rev) || ((*match_id).class != BCMA_ANY_CLASS && (*match_id).class != (*core).id.class)) { bcma_erom_skip_component(bus, eromptr); return -ENODEV; }
    for _ in 0..ports[0] { if bcma_erom_get_mst_port(bus, eromptr) < 0 { return -EILSEQ; } }
    let tmp = bcma_erom_get_addr_desc(bus, eromptr, SCAN_ADDR_TYPE_SLAVE, 0); if tmp == 0 || tmp >= (-MAX_ERRNO) as u32 { let bridge = bcma_erom_get_addr_desc(bus, eromptr, SCAN_ADDR_TYPE_BRIDGE, 0); if bridge == 0 || bridge >= (-MAX_ERRNO) as u32 { return -EILSEQ; } bcma_info(bus, c"Bridge found\n".as_ptr()); return -ENXIO; } (*core).addr = tmp;
    let mut k = 0usize;
    for i in 0..ports[1] { for _ in 0..u8::MAX { let tmp = bcma_erom_get_addr_desc(bus, eromptr, SCAN_ADDR_TYPE_SLAVE, i); if tmp >= (-MAX_ERRNO) as u32 { break; } if k < (*core).addr_s.len() { (*core).addr_s[k] = tmp; k += 1; } } }
    for i in 0..wrappers[0] { for j in 0..u8::MAX { let tmp = bcma_erom_get_addr_desc(bus, eromptr, SCAN_ADDR_TYPE_MWRAP, i); if tmp >= (-MAX_ERRNO) as u32 { break; } if i == 0 && j == 0 { (*core).wrap = tmp; } } }
    for i in 0..wrappers[1] { let hack = if ports[1] == 1 { 0 } else { 1 }; for j in 0..u8::MAX { let tmp = bcma_erom_get_addr_desc(bus, eromptr, SCAN_ADDR_TYPE_SWRAP, i + hack); if tmp >= (-MAX_ERRNO) as u32 { break; } if wrappers[0] == 0 && i == 0 && j == 0 { (*core).wrap = tmp; } } }
    if (*bus).hosttype == BCMA_HOSTTYPE_SOC { (*core).io_addr = ioremap((*core).addr, BCMA_CORE_SIZE); if (*core).io_addr.is_null() { return -ENOMEM; } if (*core).wrap != 0 { (*core).io_wrap = ioremap((*core).wrap, BCMA_CORE_SIZE); if (*core).io_wrap.is_null() { iounmap((*core).io_addr); return -ENOMEM; } } }
    0
}

pub unsafe fn bcma_detect_chip(bus: *mut bcma_bus) { let mut chip_id = [0i8; 8]; bcma_scan_switch_core(bus, BCMA_ADDR_BASE); let tmp = bcma_scan_read32(bus, BCMA_CC_ID); (*bus).chipinfo.id = ((tmp & BCMA_CC_ID_ID) >> BCMA_CC_ID_ID_SHIFT) as _; (*bus).chipinfo.rev = ((tmp & BCMA_CC_ID_REV) >> BCMA_CC_ID_REV_SHIFT) as _; (*bus).chipinfo.pkg = ((tmp & BCMA_CC_ID_PKG) >> BCMA_CC_ID_PKG_SHIFT) as _; snprintf(chip_id.as_mut_ptr(), chip_id.len(), if (*bus).chipinfo.id > 0x9999 { c"%d".as_ptr() } else { c"0x%04X".as_ptr() }, (*bus).chipinfo.id); bcma_info(bus, c"Found chip with id %s, rev 0x%02X and package 0x%02X\n".as_ptr(), chip_id.as_ptr(), (*bus).chipinfo.rev, (*bus).chipinfo.pkg); }

pub unsafe fn bcma_bus_scan(bus: *mut bcma_bus) -> i32 { if (*bus).nr_cores != 0 { return 0; } let erombase = bcma_scan_read32(bus, BCMA_CC_EROM); let mut eromptr = if (*bus).hosttype == BCMA_HOSTTYPE_SOC { ioremap(erombase, BCMA_CORE_SIZE) as *mut u32 } else { (*bus).mmio }; if eromptr.is_null() { return -ENOMEM; } let eromend = eromptr.add(BCMA_CORE_SIZE as usize / core::mem::size_of::<u32>()); bcma_scan_switch_core(bus, erombase); let mut core_num = 0; while eromptr < eromend { let core = kzalloc_obj(); if core.is_null() { if (*bus).hosttype == BCMA_HOSTTYPE_SOC { iounmap(eromptr); } return -ENOMEM; } (*core).bus = bus; let err = bcma_get_next_core(bus, &mut eromptr, core::ptr::null(), core_num, core); if err < 0 { kfree(core); if err == -ENODEV { core_num += 1; continue; } if err == -ENXIO { continue; } if err == -ESPIPE { break; } if (*bus).hosttype == BCMA_HOSTTYPE_SOC { iounmap(eromptr); } return err; } (*core).core_index = core_num as _; core_num += 1; (*bus).nr_cores += 1; bcma_prepare_core(bus, core); list_add_tail_core(bus, core); } if (*bus).hosttype == BCMA_HOSTTYPE_SOC { iounmap(eromptr); } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
