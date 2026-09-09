// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of pata_cmd64x.c. */

const DRV_NAME: &str = "pata_cmd64x";
const DRV_VERSION: &str = "0.2.18";

const CFR: u8 = 0x50;
const CFR_INTR_CH0: u8 = 0x04;
const CNTRL: u8 = 0x51;
const CNTRL_CH0: u8 = 0x04;
const CNTRL_CH1: u8 = 0x08;
const CMDTIM: u8 = 0x52;
const ARTTIM0: u8 = 0x53;
const DRWTIM0: u8 = 0x54;
const ARTTIM1: u8 = 0x55;
const DRWTIM1: u8 = 0x56;
const ARTTIM23: u8 = 0x57;
const ARTTIM23_DIS_RA2: u8 = 0x04;
const ARTTIM23_DIS_RA3: u8 = 0x08;
const ARTTIM23_INTR_CH1: u8 = 0x10;
const DRWTIM2: u8 = 0x58;
const BRST: u8 = 0x59;
const DRWTIM3: u8 = 0x5b;
const BMIDECR0: u8 = 0x70;
const MRDMODE: u8 = 0x71;
const MRDMODE_INTR_CH0: u8 = 0x04;
const MRDMODE_INTR_CH1: u8 = 0x08;
const BMIDESR0: u8 = 0x72;
const UDIDETCR0: u8 = 0x73;
const DTPR0: u8 = 0x74;
const BMIDECR1: u8 = 0x78;
const BMIDECSR: u8 = 0x79;
const UDIDETCR1: u8 = 0x7b;
const DTPR1: u8 = 0x7c;

unsafe fn cmd648_cable_detect(ap: *mut ata_port) -> c_int {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut r: u8 = 0;
    pci_read_config_byte(pdev, BMIDECSR, &mut r);
    if r & (1 << (*ap).port_no) != 0 { ATA_CBL_PATA80 } else { ATA_CBL_PATA40 }
}

unsafe fn cmd64x_set_timing(ap: *mut ata_port, adev: *mut ata_device, mode: u8) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut t: ata_timing = core::mem::zeroed();
    const T: c_ulong = 1000000 / 33;
    let setup_data: [u8; 5] = [0x40, 0x40, 0x40, 0x80, 0x00];
    let arttim_port: [[u8; 2]; 2] = [[ARTTIM0, ARTTIM1], [ARTTIM23, ARTTIM23]];
    let drwtim_port: [[u8; 2]; 2] = [[DRWTIM0, DRWTIM1], [DRWTIM2, DRWTIM3]];
    let arttim = arttim_port[(*ap).port_no as usize][(*adev).devno as usize];
    let drwtim = drwtim_port[(*ap).port_no as usize][(*adev).devno as usize];
    if ata_timing_compute(adev, mode, &mut t, T, 0) < 0 {
        ata_dev_err(adev, concat!(DRV_NAME, ": mode computation failed.\n").as_ptr());
        return;
    }
    if (*ap).port_no != 0 {
        let pair = ata_dev_pair(adev);
        if !pair.is_null() {
            let mut tp: ata_timing = core::mem::zeroed();
            ata_timing_compute(pair, (*pair).pio_mode, &mut tp, T, 0);
            ata_timing_merge(&mut t, &mut tp, &mut t, ATA_TIMING_SETUP);
        }
    }
    ata_dev_dbg(adev, concat!(DRV_NAME, ": active %d recovery %d setup %d.\n").as_ptr(), t.active, t.recover, t.setup);
    if t.recover > 16 { t.active += t.recover - 16; t.recover = 16; }
    if t.active > 16 { t.active = 16; }
    if t.recover == 16 { t.recover = 0; } else if t.recover > 1 { t.recover -= 1; } else { t.recover = 15; }
    t.setup = if t.setup > 4 { 0xc0 } else { setup_data[t.setup as usize] };
    t.active &= 0x0f;
    let mut reg: u8 = 0;
    pci_read_config_byte(pdev, arttim, &mut reg);
    reg &= 0x3f;
    reg |= t.setup as u8;
    pci_write_config_byte(pdev, arttim, reg);
    pci_write_config_byte(pdev, drwtim, ((t.active << 4) | t.recover) as u8);
}

unsafe fn cmd64x_set_piomode(ap: *mut ata_port, adev: *mut ata_device) { cmd64x_set_timing(ap, adev, (*adev).pio_mode); }

unsafe fn cmd64x_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    const UDMA_DATA: [u8; 6] = [0x30, 0x20, 0x10, 0x20, 0x10, 0x00];
    let pdev = to_pci_dev((*(*ap).host).dev);
    let pci_u = UDIDETCR0 as c_int + 8 * (*ap).port_no as c_int;
    let pci_d = BMIDESR0 as c_int + 8 * (*ap).port_no as c_int;
    let shift = 2 * (*adev).devno;
    let mut reg_d = 0u8; let mut reg_u = 0u8;
    pci_read_config_byte(pdev, pci_d, &mut reg_d); pci_read_config_byte(pdev, pci_u, &mut reg_u);
    reg_d &= !(0x20 << (*adev).devno); reg_u &= !(0x30 << shift); reg_u &= !(0x05 << (*adev).devno);
    if (*adev).dma_mode >= XFER_UDMA_0 {
        reg_u |= UDMA_DATA[((*adev).dma_mode - XFER_UDMA_0) as usize] << shift;
        reg_u |= 1 << (*adev).devno; if (*adev).dma_mode > XFER_UDMA_2 { reg_u |= 4 << (*adev).devno; }
    } else { reg_u &= !(1 << (*adev).devno); cmd64x_set_timing(ap, adev, (*adev).dma_mode); }
    reg_d |= 0x20 << (*adev).devno;
    pci_write_config_byte(pdev, pci_u, reg_u); pci_write_config_byte(pdev, pci_d, reg_d);
}

unsafe fn cmd64x_sff_irq_check(ap: *mut ata_port) -> bool {
    let pdev = to_pci_dev((*(*ap).host).dev); let irq_mask = if (*ap).port_no != 0 { ARTTIM23_INTR_CH1 } else { CFR_INTR_CH0 }; let irq_reg = if (*ap).port_no != 0 { ARTTIM23 } else { CFR }; let mut irq_stat = 0u8;
    pci_read_config_byte(pdev, irq_reg, &mut irq_stat); irq_stat & irq_mask != 0
}
unsafe fn cmd64x_sff_irq_clear(ap: *mut ata_port) { let pdev = to_pci_dev((*(*ap).host).dev); let irq_reg = if (*ap).port_no != 0 { ARTTIM23 } else { CFR }; let mut irq_stat = 0u8; ata_bmdma_irq_clear(ap); pci_read_config_byte(pdev, irq_reg, &mut irq_stat); }
unsafe fn cmd648_sff_irq_check(ap: *mut ata_port) -> bool { let pdev = to_pci_dev((*(*ap).host).dev); let base = pci_resource_start(pdev, 4); let mask = if (*ap).port_no != 0 { MRDMODE_INTR_CH1 } else { MRDMODE_INTR_CH0 }; inb(base + 1) & mask != 0 }
unsafe fn cmd648_sff_irq_clear(ap: *mut ata_port) { let pdev = to_pci_dev((*(*ap).host).dev); let base = pci_resource_start(pdev, 4); let mask = if (*ap).port_no != 0 { MRDMODE_INTR_CH1 } else { MRDMODE_INTR_CH0 }; ata_bmdma_irq_clear(ap); let mut m = inb(base + 1); m &= !(MRDMODE_INTR_CH0 | MRDMODE_INTR_CH1); outb(m | mask, base + 1); }
unsafe fn cmd646r1_bmdma_stop(qc: *mut ata_queued_cmd) { ata_bmdma_stop(qc); }

static CMD64X_SHT: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);
static CMD64X_BASE_OPS: ata_port_operations = ata_port_operations { inherits: &ata_bmdma_port_ops, set_piomode: Some(cmd64x_set_piomode), set_dmamode: Some(cmd64x_set_dmamode), ..core::mem::zeroed() };
static CMD64X_PORT_OPS: ata_port_operations = ata_port_operations { inherits: &CMD64X_BASE_OPS, sff_irq_check: Some(cmd64x_sff_irq_check), sff_irq_clear: Some(cmd64x_sff_irq_clear), cable_detect: Some(ata_cable_40wire), ..core::mem::zeroed() };
static CMD646R1_PORT_OPS: ata_port_operations = ata_port_operations { inherits: &CMD64X_BASE_OPS, sff_irq_check: Some(cmd64x_sff_irq_check), sff_irq_clear: Some(cmd64x_sff_irq_clear), bmdma_stop: Some(cmd646r1_bmdma_stop), cable_detect: Some(ata_cable_40wire), ..core::mem::zeroed() };
static CMD646R3_PORT_OPS: ata_port_operations = ata_port_operations { inherits: &CMD64X_BASE_OPS, sff_irq_check: Some(cmd648_sff_irq_check), sff_irq_clear: Some(cmd648_sff_irq_clear), cable_detect: Some(ata_cable_40wire), ..core::mem::zeroed() };
static CMD648_PORT_OPS: ata_port_operations = ata_port_operations { inherits: &CMD64X_BASE_OPS, sff_irq_check: Some(cmd648_sff_irq_check), sff_irq_clear: Some(cmd648_sff_irq_clear), cable_detect: Some(cmd648_cable_detect), ..core::mem::zeroed() };

unsafe fn cmd64x_fixup(pdev: *mut pci_dev) { let mut m = 0u8; pci_write_config_byte(pdev, PCI_LATENCY_TIMER, 64); pci_read_config_byte(pdev, MRDMODE, &mut m); m &= !0x30; m |= 0x02; pci_write_config_byte(pdev, MRDMODE, m); /* CONFIG_PPC: pci_write_config_byte(pdev, UDIDETCR0, 0xF0); */ }

unsafe fn cmd64x_init_one(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    let cmd_info: [ata_port_info; 7] = [
        ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: 0, port_ops: &CMD64X_PORT_OPS },
        ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: 0, port_ops: &CMD64X_PORT_OPS },
        ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: 0, port_ops: &CMD646R3_PORT_OPS },
        ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: ATA_UDMA2, port_ops: &CMD646R3_PORT_OPS },
        ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: 0, port_ops: &CMD646R1_PORT_OPS },
        ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: ATA_UDMA4, port_ops: &CMD648_PORT_OPS },
        ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: ATA_UDMA5, port_ops: &CMD648_PORT_OPS },
    ];
    let mut ppi = [&cmd_info[(*id).driver_data as usize], &cmd_info[(*id).driver_data as usize], core::ptr::null()];
    let mut reg = 0u8; let bridge = (*(*pdev).bus).self_; let port_ok = !( !bridge.is_null() && (*bridge).vendor == PCI_VENDOR_ID_MOBILITY_ELECTRONICS ); let mut cntrl_ch0_ok = ((*id).driver_data != 0) as c_int;
    let rc = pcim_enable_device(pdev); if rc != 0 { return rc; }
    if (*id).driver_data == 0 { ata_pci_bmdma_clear_simplex(pdev); }
    if (*pdev).device == PCI_DEVICE_ID_CMD_646 { match (*pdev).revision { 3 | 4 => { ppi[0] = &cmd_info[2]; ppi[1] = &cmd_info[2]; }, 1 => { ppi[0] = &cmd_info[4]; ppi[1] = &cmd_info[4]; cntrl_ch0_ok = 0; }, 2 | 0 => cntrl_ch0_ok = 0, _ => { ppi[0] = &cmd_info[3]; ppi[1] = &cmd_info[3]; } } }
    cmd64x_fixup(pdev); pci_read_config_byte(pdev, CNTRL, &mut reg);
    if port_ok && cntrl_ch0_ok != 0 && reg & CNTRL_CH0 == 0 { ppi[0] = &ata_dummy_port_info; }
    if port_ok && reg & CNTRL_CH1 == 0 { ppi[1] = &ata_dummy_port_info; }
    ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &CMD64X_SHT, core::ptr::null_mut(), 0)
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn cmd64x_reinit_one(pdev: *mut pci_dev) -> c_int { let host = pci_get_drvdata(pdev); let rc = ata_pci_device_do_resume(pdev); if rc != 0 { return rc; } cmd64x_fixup(pdev); ata_host_resume(host); 0 }

static CMD64X: [pci_device_id; 5] = [
    pci_device_id { vendor: PCI_VENDOR_ID_CMD, device: PCI_DEVICE_ID_CMD_643, driver_data: 0 },
    pci_device_id { vendor: PCI_VENDOR_ID_CMD, device: PCI_DEVICE_ID_CMD_646, driver_data: 1 },
    pci_device_id { vendor: PCI_VENDOR_ID_CMD, device: PCI_DEVICE_ID_CMD_648, driver_data: 5 },
    pci_device_id { vendor: PCI_VENDOR_ID_CMD, device: PCI_DEVICE_ID_CMD_649, driver_data: 6 },
    pci_device_id::default(),
];

static CMD64X_PCI_DRIVER: pci_driver = pci_driver { name: DRV_NAME, id_table: CMD64X.as_ptr(), probe: Some(cmd64x_init_one), remove: Some(ata_pci_remove_one), /* CONFIG_PM_SLEEP: suspend = ata_pci_device_suspend, resume = cmd64x_reinit_one */ ..core::mem::zeroed() };

module_pci_driver!(CMD64X_PCI_DRIVER);
module_author!("Alan Cox");
module_description!("low-level driver for CMD64x series PATA controllers");
module_license!("GPL");
module_device_table!(pci, CMD64X);
module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
