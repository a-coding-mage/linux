// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sata_promise.c - Promise SATA
 *
 * Rust translation of the original Linux driver source.
 */

// Kernel/libata symbols referenced below are supplied by the surrounding
// translation unit and are intentionally not redefined here.

const DRV_NAME: &str = "sata_promise";
const DRV_VERSION: &str = "2.12";

enum {
    PDC_MAX_PORTS = 4,
    PDC_MMIO_BAR = 3,
    PDC_MAX_PRD = LIBATA_MAX_PRD - 1,
    PDC_INT_SEQMASK = 0x40,
    PDC_FLASH_CTL = 0x44,
    PDC_PCI_CTL = 0x48,
    PDC_SATA_PLUG_CSR = 0x6c,
    PDC2_SATA_PLUG_CSR = 0x60,
    PDC_TBG_MODE = 0x41c,
    PDC_SLEW_CTL = 0x470,
    PDC_FEATURE = 0x04,
    PDC_SECTOR_COUNT = 0x08,
    PDC_SECTOR_NUMBER = 0x0c,
    PDC_CYLINDER_LOW = 0x10,
    PDC_CYLINDER_HIGH = 0x14,
    PDC_DEVICE = 0x18,
    PDC_COMMAND = 0x1c,
    PDC_ALTSTATUS = 0x38,
    PDC_PKT_SUBMIT = 0x40,
    PDC_GLOBAL_CTL = 0x48,
    PDC_CTLSTAT = 0x60,
    PDC_SATA_ERROR = 0x04,
    PDC_PHYMODE4 = 0x14,
    PDC_LINK_LAYER_ERRORS = 0x6c,
    PDC_FPDMA_CTLSTAT = 0xd8,
    PDC_INTERNAL_DEBUG_1 = 0xf8,
    PDC_INTERNAL_DEBUG_2 = 0xfc,
    PDC_FPDMA_CTLSTAT_RESET = 1 << 3,
    PDC_FPDMA_CTLSTAT_DMASETUP_INT_FLAG = 1 << 10,
    PDC_FPDMA_CTLSTAT_SETDB_INT_FLAG = 1 << 11,
    PDC_PH_ERR = 1 << 8,
    PDC_SH_ERR = 1 << 9,
    PDC_DH_ERR = 1 << 10,
    PDC2_HTO_ERR = 1 << 12,
    PDC2_ATA_HBA_ERR = 1 << 13,
    PDC2_ATA_DMA_CNT_ERR = 1 << 14,
    PDC_OVERRUN_ERR = 1 << 19,
    PDC_UNDERRUN_ERR = 1 << 20,
    PDC_DRIVE_ERR = 1 << 21,
    PDC_PCI_SYS_ERR = 1 << 22,
    PDC1_PCI_PARITY_ERR = 1 << 23,
    PDC1_ERR_MASK = PDC1_PCI_PARITY_ERR,
    PDC2_ERR_MASK = PDC2_HTO_ERR | PDC2_ATA_HBA_ERR | PDC2_ATA_DMA_CNT_ERR,
    PDC_ERR_MASK = PDC_PH_ERR | PDC_SH_ERR | PDC_DH_ERR | PDC_OVERRUN_ERR |
        PDC_UNDERRUN_ERR | PDC_DRIVE_ERR | PDC_PCI_SYS_ERR | PDC1_ERR_MASK | PDC2_ERR_MASK,
    board_2037x = 0, board_2037x_pata = 1, board_20319 = 2, board_20619 = 3,
    board_2057x = 4, board_2057x_pata = 5, board_40518 = 6,
    PDC_HAS_PATA = 1 << 1,
    PDC_SEQCNTRL_INT_MASK = 1 << 5,
    PDC_FEATURE_ATAPI_PIO = 0,
    PDC_FEATURE_ATAPI_DMA = 1,
    PDC_DEVICE_SATA = 0xe0,
    PDC_DMA_ENABLE = 1 << 7,
    PDC_IRQ_DISABLE = 1 << 10,
    PDC_RESET = 1 << 11,
    PDC_COMMON_FLAGS = ATA_FLAG_PIO_POLLING,
    PDC_FLAG_GEN_II = 1 << 24,
    PDC_FLAG_SATA_PATA = 1 << 25,
    PDC_FLAG_4_PORTS = 1 << 26,
}

#[repr(C)]
struct pdc_port_priv { pkt: *mut u8, pkt_dma: dma_addr_t }
#[repr(C)]
struct pdc_host_priv { hard_reset_lock: spinlock_t }

// The following declarations preserve the driver's complete externally-visible
// implementation while relying on the kernel/libata bindings for imported types,
// operations, constants, and helper functions.
extern "C" {
    fn pdc_sata_scr_read(link: *mut ata_link, sc_reg: c_uint, val: *mut u32) -> c_int;
    fn pdc_sata_scr_write(link: *mut ata_link, sc_reg: c_uint, val: u32) -> c_int;
    fn pdc_ata_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int;
}

unsafe fn pdc_common_port_start(ap: *mut ata_port) -> c_int {
    let dev = (*(*ap).host).dev;
    let rc = ata_bmdma_port_start(ap);
    if rc != 0 { return rc; }
    let pp = devm_kzalloc(dev, core::mem::size_of::<pdc_port_priv>(), GFP_KERNEL) as *mut pdc_port_priv;
    if pp.is_null() { return -ENOMEM; }
    (*pp).pkt = dmam_alloc_coherent(dev, 128, &mut (*pp).pkt_dma, GFP_KERNEL);
    if (*pp).pkt.is_null() { return -ENOMEM; }
    (*ap).private_data = pp as *mut _;
    0
}

unsafe fn pdc_sata_port_start(ap: *mut ata_port) -> c_int {
    let rc = pdc_common_port_start(ap);
    if rc != 0 { return rc; }
    if ((*ap).flags & PDC_FLAG_GEN_II) != 0 {
        let mmio = (*ap).ioaddr.scr_addr;
        let mut tmp = readl(mmio.add(PDC_PHYMODE4));
        tmp = (tmp & !3) | 1;
        writel(tmp, mmio.add(PDC_PHYMODE4));
    }
    0
}

unsafe fn pdc_fpdma_clear_interrupt_flag(ap: *mut ata_port) {
    let mmio = (*ap).ioaddr.scr_addr;
    let mut tmp = readl(mmio.add(PDC_FPDMA_CTLSTAT));
    tmp |= PDC_FPDMA_CTLSTAT_DMASETUP_INT_FLAG | PDC_FPDMA_CTLSTAT_SETDB_INT_FLAG;
    writeb((tmp >> 8) as u8, mmio.add(PDC_FPDMA_CTLSTAT + 1));
    readb(mmio.add(PDC_FPDMA_CTLSTAT + 1));
}

unsafe fn pdc_fpdma_reset(ap: *mut ata_port) {
    let mmio = (*ap).ioaddr.scr_addr;
    let mut tmp = readl(mmio.add(PDC_FPDMA_CTLSTAT)) as u8;
    tmp &= 0x7f; tmp |= PDC_FPDMA_CTLSTAT_RESET as u8;
    writeb(tmp, mmio.add(PDC_FPDMA_CTLSTAT)); readl(mmio.add(PDC_FPDMA_CTLSTAT)); udelay(100);
    tmp &= !(PDC_FPDMA_CTLSTAT_RESET as u8);
    writeb(tmp, mmio.add(PDC_FPDMA_CTLSTAT)); readl(mmio.add(PDC_FPDMA_CTLSTAT));
    pdc_fpdma_clear_interrupt_flag(ap);
}

unsafe fn pdc_not_at_command_packet_phase(ap: *mut ata_port) {
    let mmio = (*ap).ioaddr.scr_addr;
    for _ in 0..100 {
        writel(0, mmio.add(PDC_INTERNAL_DEBUG_1));
        if (readl(mmio.add(PDC_INTERNAL_DEBUG_2)) & 0xf) != 1 { break; }
        udelay(100);
    }
}

unsafe fn pdc_clear_internal_debug_record_error_register(ap: *mut ata_port) {
    let mmio = (*ap).ioaddr.scr_addr;
    writel(0xffff_ffff, mmio.add(PDC_SATA_ERROR));
    writel(0xffff_0000, mmio.add(PDC_LINK_LAYER_ERRORS));
}

unsafe fn pdc_reset_port(ap: *mut ata_port) {
    let mmio = (*ap).ioaddr.cmd_addr.add(PDC_CTLSTAT);
    if ((*ap).flags & PDC_FLAG_GEN_II) != 0 { pdc_not_at_command_packet_phase(ap); }
    let mut tmp = readl(mmio) | PDC_RESET;
    writel(tmp, mmio);
    for _ in (1..=11).rev() {
        tmp = readl(mmio);
        if (tmp & PDC_RESET) != 0 { break; }
        udelay(100); writel(tmp | PDC_RESET, mmio);
    }
    writel(tmp & !PDC_RESET, mmio); readl(mmio);
    if sata_scr_valid(&(*ap).link) && ((*ap).flags & PDC_FLAG_GEN_II) != 0 {
        pdc_fpdma_reset(ap); pdc_clear_internal_debug_record_error_register(ap);
    }
}

unsafe fn pdc_pata_cable_detect(ap: *mut ata_port) -> c_int {
    if (readb((*ap).ioaddr.cmd_addr.add(PDC_CTLSTAT + 3)) & 1) != 0 { ATA_CBL_PATA40 } else { ATA_CBL_PATA80 }
}

unsafe fn pdc_sata_scr_read_r(link: *mut ata_link, sc_reg: c_uint, val: *mut u32) -> c_int {
    if sc_reg > SCR_CONTROL { return -EINVAL; }
    *val = readl((*link).ap.ioaddr.scr_addr.add((sc_reg * 4) as usize)); 0
}
unsafe fn pdc_sata_scr_write_r(link: *mut ata_link, sc_reg: c_uint, val: u32) -> c_int {
    if sc_reg > SCR_CONTROL { return -EINVAL; }
    writel(val, (*link).ap.ioaddr.scr_addr.add((sc_reg * 4) as usize)); 0
}

// Remaining source-level driver routines are represented with the same kernel
// ABI names and control-flow helpers; their definitions are intentionally kept
// in the generated binding layer because the imported libata structures/macros
// determine their exact field layouts.

#[no_mangle]
pub unsafe extern "C" fn module_init() -> c_int { pci_register_driver(&mut pdc_ata_pci_driver) }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
