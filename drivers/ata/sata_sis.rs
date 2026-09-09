// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  sata_sis.c - Silicon Integrated Systems SATA
 *
 *  Maintained by:  Uwe Koziolek
 *  Copyright 2004 Uwe Koziolek
 *
 *  Rust translation of the original Linux libata driver.
 */

// External Linux kernel, libata, PCI, SCSI, and sis.h symbols are supplied by
// the surrounding translation unit/dependencies.

const DRV_NAME: &str = "sata_sis";
const DRV_VERSION: &str = "1.0";

const SIS_180: u32 = 0;
const SIS_SCR_PCI_BAR: u32 = 5;
const SIS_GENCTL: u32 = 0x54;
const SIS_SCR_BASE: u32 = 0xc0;
const SIS180_SATA1_OFS: u32 = 0x10;
const SIS182_SATA1_OFS: u32 = 0x20;
const SIS_PMR: u32 = 0x90;
const SIS_PMR_COMBINED: u8 = 0x30;
const SIS_FLAG_CFGSCR: u32 = 1u32 << 30;
const GENCTL_IOMAPPED_SCR: u32 = 1u32 << 26;

extern "C" {
    fn sis_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32;
    fn sis_scr_read(link: *mut ata_link, sc_reg: u32, val: *mut u32) -> i32;
    fn sis_scr_write(link: *mut ata_link, sc_reg: u32, val: u32) -> i32;
}

static SIS_PCI_TBL: [pci_device_id; 7] = [
    PCI_VDEVICE!(SI, 0x0180, SIS_180),
    PCI_VDEVICE!(SI, 0x0181, SIS_180),
    PCI_VDEVICE!(SI, 0x0182, SIS_180),
    PCI_VDEVICE!(SI, 0x0183, SIS_180),
    PCI_VDEVICE!(SI, 0x1182, SIS_180),
    PCI_VDEVICE!(SI, 0x1183, SIS_180),
    pci_device_id::default(),
];

static mut SIS_PCI_DRIVER: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: SIS_PCI_TBL.as_ptr(),
    probe: Some(sis_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)]
    suspend: Some(ata_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)]
    resume: Some(ata_pci_device_resume),
};

static SIS_SHT: scsi_host_template = scsi_host_template {
    // ATA_BMDMA_SHT(DRV_NAME)
    ..ATA_BMDMA_SHT!(DRV_NAME)
};

static mut SIS_OPS: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    scr_read: Some(sis_scr_read),
    scr_write: Some(sis_scr_write),
};

static SIS_PORT_INFO: ata_port_info = ata_port_info {
    flags: ATA_FLAG_SATA,
    pio_mask: ATA_PIO4,
    mwdma_mask: ATA_MWDMA2,
    udma_mask: ATA_UDMA6,
    port_ops: &SIS_OPS,
};

// MODULE_AUTHOR, MODULE_DESCRIPTION, MODULE_LICENSE, MODULE_DEVICE_TABLE, and
// MODULE_VERSION are kernel module metadata declarations.
module_author!("Uwe Koziolek");
module_description!("low-level driver for Silicon Integrated Systems SATA");
module_license!("GPL");
module_device_table!(pci, SIS_PCI_TBL);
module_version!(DRV_VERSION);

unsafe fn get_scr_cfg_addr(link: *mut ata_link, sc_reg: u32) -> u32 {
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut addr = SIS_SCR_BASE + (4 * sc_reg);
    let mut pmr: u8 = 0;

    if (*ap).port_no != 0 {
        match (*pdev).device {
            0x0180 | 0x0181 => {
                pci_read_config_byte(pdev, SIS_PMR, &mut pmr);
                if (pmr & SIS_PMR_COMBINED) == 0 {
                    addr += SIS180_SATA1_OFS;
                }
            }
            0x0182 | 0x0183 | 0x1182 => addr += SIS182_SATA1_OFS,
            _ => {}
        }
    }
    if (*link).pmp != 0 {
        addr += 0x10;
    }
    addr
}

unsafe fn sis_scr_cfg_read(link: *mut ata_link, sc_reg: u32, val: *mut u32) -> u32 {
    let pdev = to_pci_dev((*(*link).ap).host.dev);
    let cfg_addr = get_scr_cfg_addr(link, sc_reg);
    if sc_reg == SCR_ERROR {
        return (-EINVAL) as u32;
    }
    pci_read_config_dword(pdev, cfg_addr, val);
    0
}

unsafe fn sis_scr_cfg_write(link: *mut ata_link, sc_reg: u32, val: u32) -> i32 {
    let pdev = to_pci_dev((*(*link).ap).host.dev);
    let cfg_addr = get_scr_cfg_addr(link, sc_reg);
    pci_write_config_dword(pdev, cfg_addr, val);
    0
}

unsafe extern "C" fn sis_scr_read(link: *mut ata_link, sc_reg: u32, val: *mut u32) -> i32 {
    let ap = (*link).ap;
    let base = (*ap).ioaddr.scr_addr.add(((*link).pmp * 0x10) as usize);
    if sc_reg > SCR_CONTROL {
        return -EINVAL;
    }
    if ((*ap).flags & SIS_FLAG_CFGSCR) != 0 {
        return sis_scr_cfg_read(link, sc_reg, val) as i32;
    }
    *val = ioread32(base.add((sc_reg * 4) as usize));
    0
}

unsafe extern "C" fn sis_scr_write(link: *mut ata_link, sc_reg: u32, val: u32) -> i32 {
    let ap = (*link).ap;
    let base = (*ap).ioaddr.scr_addr.add(((*link).pmp * 0x10) as usize);
    if sc_reg > SCR_CONTROL {
        return -EINVAL;
    }
    if ((*ap).flags & SIS_FLAG_CFGSCR) != 0 {
        return sis_scr_cfg_write(link, sc_reg, val);
    }
    iowrite32(val, base.add((sc_reg * 4) as usize));
    0
}

unsafe extern "C" fn sis_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    let mut pi = SIS_PORT_INFO;
    let mut ppi: [*const ata_port_info; 2] = [&pi, &pi];
    let mut host: *mut ata_host = core::ptr::null_mut();
    let (mut genctl, mut val): (u32, u32) = (0, 0);
    let mut pmr: u8 = 0;
    let mut port2_start: u8 = 0x20;
    let mut rc: i32;

    ata_print_version_once(&mut (*pdev).dev, DRV_VERSION);
    rc = pcim_enable_device(pdev);
    if rc != 0 { return rc; }

    pci_read_config_dword(pdev, SIS_GENCTL, &mut genctl);
    if (genctl & GENCTL_IOMAPPED_SCR) == 0 { pi.flags |= SIS_FLAG_CFGSCR; }
    if (pi.flags & SIS_FLAG_CFGSCR) == 0 &&
       (pci_resource_start(pdev, SIS_SCR_PCI_BAR) == 0 ||
        pci_resource_len(pdev, SIS_SCR_PCI_BAR) < 128) {
        genctl &= !GENCTL_IOMAPPED_SCR;
        pci_write_config_dword(pdev, SIS_GENCTL, genctl);
        pi.flags |= SIS_FLAG_CFGSCR;
    }

    pci_read_config_byte(pdev, SIS_PMR, &mut pmr);
    match (*ent).device {
        0x0180 | 0x0181 => {
            match pmr & 0x30 {
                0x10 => ppi[1] = &sis_info133_for_sata,
                0x30 => ppi[0] = &sis_info133_for_sata,
                _ => {}
            }
            if (pmr & SIS_PMR_COMBINED) == 0 {
                dev_info!(&(*pdev).dev, "Detected SiS 180/181/964 chipset in SATA mode\n");
                port2_start = 64;
            } else {
                dev_info!(&(*pdev).dev, "Detected SiS 180/181 chipset in combined mode\n");
                port2_start = 0;
                pi.flags |= ATA_FLAG_SLAVE_POSS;
            }
        }
        0x0182 | 0x0183 => {
            pci_read_config_dword(pdev, 0x6C, &mut val);
            if (val & (1u32 << 31)) != 0 {
                dev_info!(&(*pdev).dev, "Detected SiS 182/965 chipset\n");
                pi.flags |= ATA_FLAG_SLAVE_POSS;
            } else { dev_info!(&(*pdev).dev, "Detected SiS 182/965L chipset\n"); }
        }
        0x1182 => {
            dev_info!(&(*pdev).dev, "Detected SiS 1182/966/680 SATA controller\n");
            pi.flags |= ATA_FLAG_SLAVE_POSS;
        }
        0x1183 => {
            dev_info!(&(*pdev).dev, "Detected SiS 1183/966/966L/968/680 controller in PATA mode\n");
            ppi = [&sis_info133_for_sata, &sis_info133_for_sata];
        }
        _ => {}
    }

    rc = ata_pci_bmdma_prepare_host(pdev, ppi.as_ptr(), &mut host);
    if rc != 0 { return rc; }
    for i in 0..2 {
        let ap = (*host).ports[i];
        if ((*ap).flags & ATA_FLAG_SATA) != 0 && ((*ap).flags & ATA_FLAG_SLAVE_POSS) != 0 {
            rc = ata_slave_link_init(ap);
            if rc != 0 { return rc; }
        }
    }
    if (pi.flags & SIS_FLAG_CFGSCR) == 0 {
        rc = pcim_iomap_regions(pdev, 1u32 << SIS_SCR_PCI_BAR, DRV_NAME);
        if rc != 0 { return rc; }
        let mmio = (*host).iomap[SIS_SCR_PCI_BAR as usize];
        (*(*host).ports[0]).ioaddr.scr_addr = mmio;
        (*(*host).ports[1]).ioaddr.scr_addr = mmio.add(port2_start as usize);
    }
    pci_set_master(pdev);
    pcim_intx(pdev, 1);
    ata_host_activate(host, (*pdev).irq, ata_bmdma_interrupt, IRQF_SHARED, &SIS_SHT)
}

module_pci_driver!(SIS_PCI_DRIVER);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
