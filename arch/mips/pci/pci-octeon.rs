/* Rust translation of pci-octeon.c. */

const OCTEON_PCI_IOSPACE_BASE: u64 = 0x80011a0400000000;
const OCTEON_PCI_IOSPACE_SIZE: u64 = 1u64 << 32;
const OCTEON_PCI_MEMSPACE_OFFSET: u64 = 0x00011b0000000000;

#[repr(C)]
pub union OcteonPciAddress {
    pub u64_: u64,
    pub s: OcteonPciAddressFields,
}
#[repr(C)]
pub struct OcteonPciAddressFields {
    pub upper: u64, pub reserved: u64, pub io: u64, pub did: u64,
    pub subdid: u64, pub reserved2: u64, pub endian_swap: u64,
    pub reserved3: u64, pub bus: u64, pub dev: u64, pub func: u64, pub reg: u64,
}

pub static mut octeon_bar1_pci_phys: u64 = 0;
pub static mut octeon_pcibios_map_irq: Option<unsafe extern "C" fn(*const pci_dev, u8, u8) -> i32> = None;
pub static mut octeon_dma_bar_type: octeon_dma_bar_type = OCTEON_DMA_BAR_TYPE_INVALID;

pub unsafe extern "C" fn pcibios_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    match octeon_pcibios_map_irq { Some(f) => f(dev, slot, pin), None => panic!("octeon_pcibios_map_irq not set.") }
}

pub unsafe extern "C" fn pcibios_plat_dev_init(dev: *mut pci_dev) -> i32 {
    let mut config: u16 = 0; let mut dconfig: u32 = 0; let mut pos: i32;
    pci_write_config_byte(dev, PCI_CACHE_LINE_SIZE, 64 / 4);
    pci_write_config_byte(dev, PCI_LATENCY_TIMER, 64);
    pci_read_config_word(dev, PCI_COMMAND, &mut config);
    config |= PCI_COMMAND_PARITY | PCI_COMMAND_SERR;
    pci_write_config_word(dev, PCI_COMMAND, config);
    if !(*dev).subordinate.is_null() {
        pci_write_config_byte(dev, PCI_SEC_LATENCY_TIMER, 64);
        pci_read_config_word(dev, PCI_BRIDGE_CONTROL, &mut config);
        config |= PCI_BRIDGE_CTL_PARITY | PCI_BRIDGE_CTL_SERR;
        pci_write_config_word(dev, PCI_BRIDGE_CONTROL, config);
    }
    config = PCI_EXP_DEVCTL_CERE | PCI_EXP_DEVCTL_NFERE | PCI_EXP_DEVCTL_FERE | PCI_EXP_DEVCTL_URRE;
    pcie_capability_set_word(dev, PCI_EXP_DEVCTL, config);
    pos = pci_find_ext_capability(dev, PCI_EXT_CAP_ID_ERR);
    if pos != 0 {
        pci_read_config_dword(dev, pos + PCI_ERR_UNCOR_STATUS, &mut dconfig);
        pci_write_config_dword(dev, pos + PCI_ERR_UNCOR_STATUS, dconfig);
        pci_write_config_dword(dev, pos + PCI_ERR_UNCOR_MASK, 0);
        pci_read_config_dword(dev, pos + PCI_ERR_COR_STATUS, &mut dconfig);
        pci_write_config_dword(dev, pos + PCI_ERR_COR_STATUS, dconfig);
        pci_write_config_dword(dev, pos + PCI_ERR_COR_MASK, 0);
        pci_read_config_dword(dev, pos + PCI_ERR_CAP, &mut dconfig);
        if dconfig & PCI_ERR_CAP_ECRC_GENC != 0 { dconfig |= PCI_ERR_CAP_ECRC_GENE; }
        if dconfig & PCI_ERR_CAP_ECRC_CHKC != 0 { dconfig |= PCI_ERR_CAP_ECRC_CHKE; }
        pci_write_config_dword(dev, pos + PCI_ERR_CAP, dconfig);
        pci_write_config_dword(dev, pos + PCI_ERR_ROOT_COMMAND, PCI_ERR_ROOT_CMD_COR_EN | PCI_ERR_ROOT_CMD_NONFATAL_EN | PCI_ERR_ROOT_CMD_FATAL_EN);
        pci_read_config_dword(dev, pos + PCI_ERR_ROOT_STATUS, &mut dconfig);
        pci_write_config_dword(dev, pos + PCI_ERR_ROOT_STATUS, dconfig);
    }
    0
}

pub unsafe extern "C" fn octeon_get_pci_interrupts() -> *const u8 {
    if of_machine_is_compatible(b"dlink,dsr-500n\0".as_ptr()) { return b"CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC\0".as_ptr(); }
    match octeon_bootinfo.board_type {
        CVMX_BOARD_TYPE_NAO38 => b"AAAAADABAAAAAAAAAAAAAAAAAAAAAAAA\0".as_ptr(),
        CVMX_BOARD_TYPE_EBH3100 | CVMX_BOARD_TYPE_CN3010_EVB_HS5 | CVMX_BOARD_TYPE_CN3005_EVB_HS5 => b"AAABAAAAAAAAAAAAAAAAAAAAAAAAAAAA\0".as_ptr(),
        CVMX_BOARD_TYPE_BBGW_REF => b"AABCD\0".as_ptr(),
        CVMX_BOARD_TYPE_CUST_DSR1000N => b"CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC\0".as_ptr(),
        _ => b"\0".as_ptr(),
    }
}

pub unsafe extern "C" fn octeon_pci_pcibios_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    let interrupts = octeon_get_pci_interrupts();
    let dev_num = ((*dev).devfn >> 3) as usize;
    let len = libc::strlen(interrupts);
    let irq_num = if dev_num < len { ((*interrupts.add(dev_num) as i32 - b'A' as i32 + pin as i32 - 1) & 3) + OCTEON_IRQ_PCI_INT0 } else { (((slot as i32 + pin as i32 - 3) & 3) + OCTEON_IRQ_PCI_INT0) };
    irq_num
}

/* The remaining controller operations retain the original register sequence and
 * depend on the kernel/Octeon declarations supplied by the surrounding tree. */
pub unsafe extern "C" fn octeon_pci_setup() -> i32 {
    if octeon_has_feature(OCTEON_FEATURE_PCIE) { return 0; }
    if !octeon_is_pci_host() { pr_notice!("Not in host mode, PCI Controller not initialized\n"); return 0; }
    octeon_pcibios_map_irq = Some(octeon_pci_pcibios_map_irq);
    set_io_port_base(OCTEON_PCI_IOSPACE_BASE as usize);
    ioport_resource.start = 0; ioport_resource.end = OCTEON_PCI_IOSPACE_SIZE - 1;
    octeon_pci_initialize();
    register_pci_controller(&mut octeon_pci_controller);
    octeon_pci_dma_init();
    0
}

unsafe fn octeon_read_config(bus: *mut pci_bus, devfn: u32, reg: i32, size: i32, val: *mut u32) -> i32 {
    let addr = octeon_config_address(bus, devfn, reg);
    match size { 4 => { *val = le32_to_cpu(cvmx_read64_uint32(addr)); PCIBIOS_SUCCESSFUL }, 2 => { *val = le16_to_cpu(cvmx_read64_uint16(addr)); PCIBIOS_SUCCESSFUL }, 1 => { *val = cvmx_read64_uint8(addr) as u32; PCIBIOS_SUCCESSFUL }, _ => PCIBIOS_FUNC_NOT_SUPPORTED }
}
unsafe fn octeon_write_config(bus: *mut pci_bus, devfn: u32, reg: i32, size: i32, val: u32) -> i32 {
    let addr = octeon_config_address(bus, devfn, reg);
    match size { 4 => cvmx_write64_uint32(addr, cpu_to_le32(val)), 2 => cvmx_write64_uint16(addr, cpu_to_le16(val)), 1 => cvmx_write64_uint8(addr, val as u8), _ => return PCIBIOS_FUNC_NOT_SUPPORTED }; PCIBIOS_SUCCESSFUL
}
unsafe fn octeon_config_address(bus: *mut pci_bus, devfn: u32, reg: i32) -> u64 {
    (2u64 << 62) | (1u64 << 57) | (3u64 << 52) | (1u64 << 49) | ((bus_number(*bus) as u64) << 16) | (((devfn >> 3) as u64) << 11) | (((devfn & 7) as u64) << 8) | reg as u64
}
unsafe fn octeon_pci_initialize() {
    cvmx_write_csr(CVMX_CIU_SOFT_PRST, 1); cvmx_read_csr(CVMX_CIU_SOFT_PRST); udelay(2000);
    cvmx_write_csr(CVMX_CIU_SOFT_PRST, 4); cvmx_read_csr(CVMX_CIU_SOFT_PRST); udelay(2000);
    octeon_npi_write32(CVMX_NPI_PCI_CTL_STATUS_2, 0); udelay(2000);
    octeon_npi_write32(CVMX_NPI_PCI_CFG01, 0x1f);
    octeon_npi_write32(CVMX_NPI_PCI_CFG16, 1); octeon_npi_write32(CVMX_NPI_PCI_CFG22, 0xff01);
    octeon_npi_write32(CVMX_NPI_PCI_CFG56, 0x3e8e07);
    octeon_npi_write32(CVMX_NPI_PCI_READ_CMD_6, 0x21); octeon_npi_write32(CVMX_NPI_PCI_READ_CMD_C, 0x31); octeon_npi_write32(CVMX_NPI_PCI_READ_CMD_E, 0x31);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
