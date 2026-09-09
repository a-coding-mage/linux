// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   pata_pcmcia.c - PCMCIA PATA controller driver.
 *   Copyright 2005-2006 Red Hat Inc, all rights reserved.
 *   PCMCIA ident update Copyright 2006 Marcin Juszkiewicz
 *
 *   Heavily based upon ide-cs.c
 */

// Linux kernel, ATA, SCSI, and PCMCIA dependencies are supplied externally.

const DRV_NAME: &str = "pata_pcmcia";
const DRV_VERSION: &str = "0.3.5";

unsafe fn pcmcia_set_mode(link: *mut ata_link, r_failed_dev: *mut *mut ata_device) -> i32 {
    let master = unsafe { &mut (*link).device[0] } as *mut ata_device;
    let slave = unsafe { &mut (*link).device[1] } as *mut ata_device;
    if !unsafe { ata_dev_enabled(master) } || !unsafe { ata_dev_enabled(slave) } {
        return unsafe { ata_set_mode(link, r_failed_dev) };
    }
    if unsafe {
        memcmp(
            (*master).id.as_ptr().add(ATA_ID_FW_REV),
            (*slave).id.as_ptr().add(ATA_ID_FW_REV),
            ATA_ID_FW_REV_LEN + ATA_ID_PROD_LEN,
        ) == 0
    } {
        // Suspicious match, but could be two cards from the same vendor - check serial.
        if unsafe {
            memcmp(
                (*master).id.as_ptr().add(ATA_ID_SERNO),
                (*slave).id.as_ptr().add(ATA_ID_SERNO),
                ATA_ID_SERNO_LEN,
            ) == 0 && ((*master).id[ATA_ID_SERNO] >> 8) != 0
        } {
            unsafe { ata_dev_warn(slave, "is a ghost device, ignoring\n") };
            unsafe { ata_dev_disable(slave) };
        }
    }
    unsafe { ata_set_mode(link, r_failed_dev) }
}

unsafe fn pcmcia_set_mode_8bit(_link: *mut ata_link, _r_failed_dev: *mut *mut ata_device) -> i32 {
    0
}

unsafe fn ata_data_xfer_8bit(
    qc: *mut ata_queued_cmd,
    buf: *mut u8,
    buflen: u32,
    rw: i32,
) -> u32 {
    let ap = unsafe { (*(*qc).dev).link.as_ref().unwrap().ap };
    if rw == READ {
        unsafe { ioread8_rep((*ap).ioaddr.data_addr, buf, buflen) };
    } else {
        unsafe { iowrite8_rep((*ap).ioaddr.data_addr, buf, buflen) };
    }
    buflen
}

unsafe fn pcmcia_8bit_drain_fifo(qc: *mut ata_queued_cmd) {
    if qc.is_null() || unsafe { (*qc).dma_dir == DMA_TO_DEVICE } {
        return;
    }
    let ap = unsafe { (*qc).ap };
    let mut count: i32 = 0;
    while unsafe { ((*(*ap).ops).sff_check_status)(ap) & ATA_DRQ != 0 } && count < 65536 {
        unsafe { ioread8((*ap).ioaddr.data_addr) };
        count += 1;
    }
    if count != 0 {
        unsafe { ata_port_warn(ap, "drained %d bytes to clear DRQ\n", count) };
    }
}

static pcmcia_sht: scsi_host_template = ATA_PIO_SHT!(DRV_NAME);

static mut pcmcia_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_sff_port_ops,
    sff_data_xfer: Some(ata_sff_data_xfer32),
    cable_detect: Some(ata_cable_40wire),
    set_mode: Some(pcmcia_set_mode),
};

static mut pcmcia_8bit_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_sff_port_ops,
    sff_data_xfer: Some(ata_data_xfer_8bit),
    cable_detect: Some(ata_cable_40wire),
    set_mode: Some(pcmcia_set_mode_8bit),
    sff_drain_fifo: Some(pcmcia_8bit_drain_fifo),
};

unsafe fn pcmcia_check_one_config(pdev: *mut pcmcia_device, priv_data: *mut core::ffi::c_void) -> i32 {
    let is_kme = priv_data as *mut i32;
    if unsafe { ((*pdev).resource[0]).flags & IO_DATA_PATH_WIDTH != IO_DATA_PATH_WIDTH_8 } {
        unsafe { (*pdev).resource[0].flags &= !IO_DATA_PATH_WIDTH; (*pdev).resource[0].flags |= IO_DATA_PATH_WIDTH_AUTO; }
    }
    unsafe { (*pdev).resource[1].flags &= !IO_DATA_PATH_WIDTH; (*pdev).resource[1].flags |= IO_DATA_PATH_WIDTH_8; }
    if unsafe { (*pdev).resource[1].end } != 0 {
        unsafe { (*pdev).resource[0].end = 8; (*pdev).resource[1].end = if *is_kme != 0 { 2 } else { 1 }; }
    } else if unsafe { (*pdev).resource[0].end } < 16 {
        return -ENODEV;
    }
    unsafe { pcmcia_request_io(pdev) }
}

unsafe fn pcmcia_init_one(pdev: *mut pcmcia_device) -> i32 {
    let mut is_kme = 0;
    let mut ret = -ENOMEM;
    let mut n_ports = 1;
    let mut ops: *mut ata_port_operations = &raw mut pcmcia_port_ops;
    unsafe { (*pdev).config_flags |= CONF_ENABLE_IRQ | CONF_AUTO_SET_IO | CONF_AUTO_SET_VPP | CONF_AUTO_CHECK_VCC; }
    is_kme = (unsafe { (*pdev).manf_id } == MANFID_KME) && unsafe { (*pdev).card_id == PRODID_KME_KXLC005_A || (*pdev).card_id == PRODID_KME_KXLC005_B } as i32;
    if unsafe { pcmcia_loop_config(pdev, Some(pcmcia_check_one_config), &mut is_kme as *mut _ as *mut _) } != 0 {
        unsafe { (*pdev).config_flags &= !CONF_AUTO_CHECK_VCC; }
        if unsafe { pcmcia_loop_config(pdev, Some(pcmcia_check_one_config), &mut is_kme as *mut _ as *mut _) } != 0 { goto_failed(pdev, ret); return ret; }
    }
    let io_base = unsafe { (*pdev).resource[0].start };
    let ctl_base = if unsafe { (*pdev).resource[1].end } != 0 { unsafe { (*pdev).resource[1].start } } else { io_base + 0x0e };
    if unsafe { (*pdev).irq } == 0 { goto_failed(pdev, ret); return ret; }
    ret = unsafe { pcmcia_enable_device(pdev) }; if ret != 0 { goto_failed(pdev, ret); return ret; }
    let io_addr = unsafe { devm_ioport_map(&mut (*pdev).dev, io_base, 8) };
    let ctl_addr = unsafe { devm_ioport_map(&mut (*pdev).dev, ctl_base, 1) };
    if io_addr.is_null() || ctl_addr.is_null() { goto_failed(pdev, -ENOMEM); return -ENOMEM; }
    unsafe { iowrite8(0x02, ctl_addr); if is_kme != 0 { iowrite8(0x81, ctl_addr.add(1)); } }
    if unsafe { resource_size((*pdev).resource[0]) } >= 0x20 { n_ports = 2; }
    if unsafe { (*pdev).manf_id == 0x0097 && (*pdev).card_id == 0x1620 } { ops = &raw mut pcmcia_8bit_port_ops; }
    let host = unsafe { ata_host_alloc(&mut (*pdev).dev, n_ports) }; if host.is_null() { goto_failed(pdev, -ENOMEM); return -ENOMEM; }
    for p in 0..n_ports { unsafe { let ap = (*host).ports[p]; (*ap).ops = ops; (*ap).pio_mask = ATA_PIO0; (*ap).flags |= ATA_FLAG_SLAVE_POSS; (*ap).ioaddr.cmd_addr = io_addr.add(0x10 * p); (*ap).ioaddr.altstatus_addr = ctl_addr.add(0x10 * p); (*ap).ioaddr.ctl_addr = ctl_addr.add(0x10 * p); ata_sff_std_ports(&mut (*ap).ioaddr); ata_port_desc(ap, "cmd 0x%lx ctl 0x%lx", io_base, ctl_base); } }
    ret = unsafe { ata_host_activate(host, (*pdev).irq, Some(ata_sff_interrupt), IRQF_SHARED, &pcmcia_sht) };
    if ret != 0 { goto_failed(pdev, ret); return ret; }
    unsafe { (*pdev).priv_ = host; }
    0
}

unsafe fn goto_failed(pdev: *mut pcmcia_device, _ret: i32) { pcmcia_disable_device(pdev); }

unsafe fn pcmcia_remove_one(pdev: *mut pcmcia_device) {
    let host = unsafe { (*pdev).priv_ };
    if !host.is_null() { unsafe { ata_host_detach(host); } }
    unsafe { pcmcia_disable_device(pdev); }
}

static pcmcia_devices: [pcmcia_device_id; 59] = [
    PCMCIA_DEVICE_FUNC_ID!(4),
    PCMCIA_DEVICE_MANF_CARD!(0x0000, 0x0000), PCMCIA_DEVICE_MANF_CARD!(0x0007, 0x0000),
    PCMCIA_DEVICE_MANF_CARD!(0x000a, 0x0000), PCMCIA_DEVICE_MANF_CARD!(0x001c, 0x0001),
    PCMCIA_DEVICE_MANF_CARD!(0x0032, 0x0704), PCMCIA_DEVICE_MANF_CARD!(0x0032, 0x2904),
    PCMCIA_DEVICE_MANF_CARD!(0x0045, 0x0401), PCMCIA_DEVICE_MANF_CARD!(0x004f, 0x0000),
    PCMCIA_DEVICE_MANF_CARD!(0x0097, 0x1620), PCMCIA_DEVICE_MANF_CARD!(0x0098, 0x0000),
    PCMCIA_DEVICE_MANF_CARD!(0x00a4, 0x002d), PCMCIA_DEVICE_MANF_CARD!(0x00ce, 0x0000),
    PCMCIA_DEVICE_MANF_CARD!(0x00f1, 0x0101), PCMCIA_DEVICE_MANF_CARD!(0x0319, 0x0000),
    PCMCIA_DEVICE_MANF_CARD!(0x2080, 0x0001), PCMCIA_DEVICE_MANF_CARD!(0x4e01, 0x0100),
    PCMCIA_DEVICE_MANF_CARD!(0x4e01, 0x0200),
    PCMCIA_DEVICE_PROD_ID123!("Caravelle", "PSC-IDE ", "PSC000", 0x8c36137c, 0xd0693ab8, 0x2768a9f0),
    PCMCIA_DEVICE_PROD_ID123!("CDROM", "IDE", "MCD-601p", 0x1b9179ca, 0xede88951, 0x0d902f74),
    PCMCIA_DEVICE_PROD_ID123!("PCMCIA", "IDE CARD", "F1", 0x281f1c5d, 0x1907960c, 0xf7fde8b9),
    PCMCIA_DEVICE_PROD_ID12!("ARGOSY", "CD-ROM", 0x78f308dc, 0x66536591),
    PCMCIA_DEVICE_PROD_ID12!("ARGOSY", "PnPIDE", 0x78f308dc, 0x0c694728),
    PCMCIA_DEVICE_PROD_ID12!("CNF   ", "CD-ROM", 0x46d7db81, 0x66536591),
    PCMCIA_DEVICE_PROD_ID12!("CNF CD-M", "CD-ROM", 0x7d93b852, 0x66536591),
    PCMCIA_DEVICE_PROD_ID12!("Creative Technology Ltd.", "PCMCIA CD-ROM Interface Card", 0xff8c8a45, 0xfe8020c4),
    PCMCIA_DEVICE_PROD_ID12!("Digital Equipment Corporation.", "Digital Mobile Media CD-ROM", 0x17692a66, 0xef1dcbde),
    PCMCIA_DEVICE_PROD_ID12!("EXP", "CD+GAME", 0x6f58c983, 0x63c13aaf),
    PCMCIA_DEVICE_PROD_ID12!("EXP   ", "CD-ROM", 0x0a5c52fd, 0x66536591),
    PCMCIA_DEVICE_PROD_ID12!("EXP   ", "PnPIDE", 0x0a5c52fd, 0x0c694728),
    PCMCIA_DEVICE_PROD_ID12!("FREECOM", "PCCARD-IDE", 0x5714cbf7, 0x48e0ab8e),
    PCMCIA_DEVICE_PROD_ID12!("HITACHI", "FLASH", 0xf4f43949, 0x9eb86aae),
    PCMCIA_DEVICE_PROD_ID12!("HITACHI", "microdrive", 0xf4f43949, 0xa6d76178),
    PCMCIA_DEVICE_PROD_ID12!("Hyperstone", "Model1", 0x3d5b9ef5, 0xca6ab420),
    PCMCIA_DEVICE_PROD_ID12!("IBM", "microdrive", 0xb569a6e5, 0xa6d76178),
    PCMCIA_DEVICE_PROD_ID12!("IBM", "IBM17JSSFP20", 0xb569a6e5, 0xf2508753),
    PCMCIA_DEVICE_PROD_ID12!("KINGSTON", "CF CARD 1GB", 0x2e6d1829, 0x55d5bffb),
    PCMCIA_DEVICE_PROD_ID12!("KINGSTON", "CF CARD 4GB", 0x2e6d1829, 0x531e7d10),
    PCMCIA_DEVICE_PROD_ID12!("KINGSTON", "CF8GB", 0x2e6d1829, 0xacbe682e),
    PCMCIA_DEVICE_PROD_ID12!("IO DATA", "CBIDE2      ", 0x547e66dc, 0x8671043b),
    PCMCIA_DEVICE_PROD_ID12!("IO DATA", "PCIDE", 0x547e66dc, 0x5c5ab149),
    PCMCIA_DEVICE_PROD_ID12!("IO DATA", "PCIDEII", 0x547e66dc, 0xb3662674),
    PCMCIA_DEVICE_PROD_ID12!("LOOKMEET", "CBIDE2      ", 0xe37be2b5, 0x8671043b),
    PCMCIA_DEVICE_PROD_ID12!("M-Systems", "CF300", 0x7ed2ad87, 0x7e9e78ee),
    PCMCIA_DEVICE_PROD_ID12!("M-Systems", "CF500", 0x7ed2ad87, 0x7a13045c),
    PCMCIA_DEVICE_PROD_ID2!("NinjaATA-", 0xebe0bd79), PCMCIA_DEVICE_PROD_ID12!("PCMCIA", "CD-ROM", 0x281f1c5d, 0x66536591),
    PCMCIA_DEVICE_PROD_ID12!("PCMCIA", "PnPIDE", 0x281f1c5d, 0x0c694728), PCMCIA_DEVICE_PROD_ID2!("PCMCIA ATA/ATAPI Adapter", 0x888d7b73),
    PCMCIA_DEVICE_PROD_ID12!("SHUTTLE TECHNOLOGY LTD.", "PCCARD-IDE/ATAPI Adapter", 0x4a3f0ba0, 0x322560e1),
    PCMCIA_DEVICE_PROD_ID12!("SEAGATE", "ST1", 0x87c1b330, 0xe1f30883), PCMCIA_DEVICE_PROD_ID12!("SAMSUNG", "04/05/06", 0x43d74cb4, 0x6a22777d),
    PCMCIA_DEVICE_PROD_ID12!("SMI VENDOR", "SMI PRODUCT", 0x30896c92, 0x703cc5f6), PCMCIA_DEVICE_PROD_ID12!("TOSHIBA", "MK2001MPL", 0xb4585a1a, 0x3489e003),
    PCMCIA_DEVICE_PROD_ID1!("TRANSCEND    512M   ", 0xd0909443), PCMCIA_DEVICE_PROD_ID12!("TRANSCEND", "TS1GCF45", 0x709b1bf1, 0xf68b6f32),
    PCMCIA_DEVICE_PROD_ID12!("TRANSCEND", "TS1GCF80", 0x709b1bf1, 0x2a54d4b1), PCMCIA_DEVICE_PROD_ID12!("TRANSCEND", "TS2GCF120", 0x709b1bf1, 0x969aa4f2),
    PCMCIA_DEVICE_PROD_ID12!("TRANSCEND", "TS4GCF120", 0x709b1bf1, 0xf54a91c8), PCMCIA_DEVICE_PROD_ID12!("TRANSCEND", "TS4GCF133", 0x709b1bf1, 0x7558f133),
    PCMCIA_DEVICE_PROD_ID12!("TRANSCEND", "TS8GCF133", 0x709b1bf1, 0xb2f89b47), PCMCIA_DEVICE_PROD_ID12!("WIT", "IDE16", 0x244e5994, 0x3e232852),
    PCMCIA_DEVICE_PROD_ID12!("WEIDA", "TWTTI", 0xcc7cf69c, 0x212bb918), PCMCIA_DEVICE_PROD_ID1!("STI Flash", 0xe4a13209),
    PCMCIA_DEVICE_PROD_ID12!("STI", "Flash 5.0", 0xbf2df18d, 0x8cb57a0e), PCMCIA_MFC_DEVICE_PROD_ID12!(1, "SanDisk", "ConnectPlus", 0x7a954bd9, 0x74be00c6),
    PCMCIA_DEVICE_PROD_ID2!("Flash Card", 0x5a362506), PCMCIA_DEVICE_NULL!(),
];
static mut pcmcia_driver: pcmcia_driver = pcmcia_driver { owner: THIS_MODULE, name: DRV_NAME, id_table: &pcmcia_devices, probe: Some(pcmcia_init_one), remove: Some(pcmcia_remove_one) };
module_pcmcia_driver!(pcmcia_driver);
MODULE_AUTHOR!("Alan Cox");
MODULE_DESCRIPTION!("low-level driver for PCMCIA ATA");
MODULE_LICENSE!("GPL");
MODULE_VERSION!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
