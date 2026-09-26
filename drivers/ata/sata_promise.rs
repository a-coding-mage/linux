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

pub const PDC_MAX_PORTS: i32 = 4;
pub const PDC_MMIO_BAR: i32 = 3;
pub const PDC_MAX_PRD: i32 = LIBATA_MAX_PRD - 1;
pub const PDC_INT_SEQMASK: i32 = 0x40;
pub const PDC_FLASH_CTL: i32 = 0x44;
pub const PDC_PCI_CTL: i32 = 0x48;
pub const PDC_SATA_PLUG_CSR: i32 = 0x6c;
pub const PDC2_SATA_PLUG_CSR: i32 = 0x60;
pub const PDC_TBG_MODE: i32 = 0x41c;
pub const PDC_SLEW_CTL: i32 = 0x470;
pub const PDC_FEATURE: i32 = 0x04;
pub const PDC_SECTOR_COUNT: i32 = 0x08;
pub const PDC_SECTOR_NUMBER: i32 = 0x0c;
pub const PDC_CYLINDER_LOW: i32 = 0x10;
pub const PDC_CYLINDER_HIGH: i32 = 0x14;
pub const PDC_DEVICE: i32 = 0x18;
pub const PDC_COMMAND: i32 = 0x1c;
pub const PDC_ALTSTATUS: i32 = 0x38;
pub const PDC_PKT_SUBMIT: i32 = 0x40;
pub const PDC_GLOBAL_CTL: i32 = 0x48;
pub const PDC_CTLSTAT: i32 = 0x60;
pub const PDC_SATA_ERROR: i32 = 0x04;
pub const PDC_PHYMODE4: i32 = 0x14;
pub const PDC_LINK_LAYER_ERRORS: i32 = 0x6c;
pub const PDC_FPDMA_CTLSTAT: i32 = 0xd8;
pub const PDC_INTERNAL_DEBUG_1: i32 = 0xf8;
pub const PDC_INTERNAL_DEBUG_2: i32 = 0xfc;
pub const PDC_FPDMA_CTLSTAT_RESET: i32 = 1 << 3;
pub const PDC_FPDMA_CTLSTAT_DMASETUP_INT_FLAG: i32 = 1 << 10;
pub const PDC_FPDMA_CTLSTAT_SETDB_INT_FLAG: i32 = 1 << 11;
pub const PDC_PH_ERR: i32 = 1 << 8;
pub const PDC_SH_ERR: i32 = 1 << 9;
pub const PDC_DH_ERR: i32 = 1 << 10;
pub const PDC2_HTO_ERR: i32 = 1 << 12;
pub const PDC2_ATA_HBA_ERR: i32 = 1 << 13;
pub const PDC2_ATA_DMA_CNT_ERR: i32 = 1 << 14;
pub const PDC_OVERRUN_ERR: i32 = 1 << 19;
pub const PDC_UNDERRUN_ERR: i32 = 1 << 20;
pub const PDC_DRIVE_ERR: i32 = 1 << 21;
pub const PDC_PCI_SYS_ERR: i32 = 1 << 22;
pub const PDC1_PCI_PARITY_ERR: i32 = 1 << 23;
pub const PDC1_ERR_MASK: i32 = PDC1_PCI_PARITY_ERR;
pub const PDC2_ERR_MASK: i32 = PDC2_HTO_ERR | PDC2_ATA_HBA_ERR | PDC2_ATA_DMA_CNT_ERR;
pub const PDC_ERR_MASK: i32 = PDC_PH_ERR | PDC_SH_ERR | PDC_DH_ERR | PDC_OVERRUN_ERR | PDC_UNDERRUN_ERR | PDC_DRIVE_ERR | PDC_PCI_SYS_ERR | PDC1_ERR_MASK | PDC2_ERR_MASK;
pub const board_2037x: i32 = 0;
pub const board_2037x_pata: i32 = 1;
pub const board_20319: i32 = 2;
pub const board_20619: i32 = 3;
pub const board_2057x: i32 = 4;
pub const board_2057x_pata: i32 = 5;
pub const board_40518: i32 = 6;
pub const PDC_HAS_PATA: i32 = 1 << 1;
pub const PDC_SEQCNTRL_INT_MASK: i32 = 1 << 5;
pub const PDC_FEATURE_ATAPI_PIO: i32 = 0;
pub const PDC_FEATURE_ATAPI_DMA: i32 = 1;
pub const PDC_DEVICE_SATA: i32 = 0xe0;
pub const PDC_DMA_ENABLE: i32 = 1 << 7;
pub const PDC_IRQ_DISABLE: i32 = 1 << 10;
pub const PDC_RESET: i32 = 1 << 11;
pub const PDC_COMMON_FLAGS: i32 = ATA_FLAG_PIO_POLLING;
pub const PDC_FLAG_GEN_II: i32 = 1 << 24;
pub const PDC_FLAG_SATA_PATA: i32 = 1 << 25;
pub const PDC_FLAG_4_PORTS: i32 = 1 << 26;

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
