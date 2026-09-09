// SPDX-License-Identifier: GPL-2.0

/*
 * Buddha, Catweasel and X-Surf PATA controller driver
 *
 * Copyright (c) 2018 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com
 *
 * Based on buddha.c:
 *
 *	Copyright (C) 1997, 2001 by Geert Uytterhoeven and others
 */

// C dependencies: linux/ata.h, linux/blkdev.h, linux/delay.h,
// linux/interrupt.h, linux/kernel.h, linux/libata.h, linux/mm.h,
// linux/module.h, linux/types.h, linux/zorro.h, scsi/scsi_cmnd.h,
// scsi/scsi_host.h, asm/amigahw.h, asm/amigaints.h, asm/setup.h.

const DRV_NAME: &str = "pata_buddha";
const DRV_VERSION: &str = "0.1.1";

const BUDDHA_BASE1: usize = 0x800;
const BUDDHA_BASE2: usize = 0xa00;
const BUDDHA_BASE3: usize = 0xc00;
const XSURF_BASE1: usize = 0xb000; // 2.5" interface
const XSURF_BASE2: usize = 0xd000; // 3.5" interface
const BUDDHA_CONTROL: usize = 0x11a;
const BUDDHA_IRQ: usize = 0xf00;
const XSURF_IRQ: usize = 0x7e;
const BUDDHA_IRQ_MR: usize = 0xfc0; // master interrupt enable

const BOARD_BUDDHA: u32 = 0;
const BOARD_CATWEASEL: u32 = 1;
const BOARD_XSURF: u32 = 2;

static mut BUDDHA_BASES: [u32; 3] = [BUDDHA_BASE1 as u32, BUDDHA_BASE2 as u32, BUDDHA_BASE3 as u32];
static mut XSURF_BASES: [u32; 2] = [XSURF_BASE1 as u32, XSURF_BASE2 as u32];

// FIXME: is this needed?
unsafe fn pata_buddha_data_xfer(
    qc: *mut ata_queued_cmd,
    mut buf: *mut u8,
    buflen: u32,
    rw: i32,
) -> u32 {
    let dev = (*qc).dev;
    let ap = (*(*dev).link).ap;
    let data_addr = (*ap).ioaddr.data_addr;
    let mut words = buflen >> 1;

    // Transfer multiple of 2 bytes
    if rw == READ {
        raw_insw(data_addr as *mut u16, buf as *mut u16, words);
    } else {
        raw_outsw(data_addr as *mut u16, buf as *mut u16, words);
    }

    // Transfer trailing byte, if any.
    if (buflen & 0x01) != 0 {
        let mut pad = [0u8; 2];

        // Point buf to the tail of buffer
        buf = buf.add((buflen - 1) as usize);

        if rw == READ {
            raw_insw(data_addr as *mut u16, pad.as_mut_ptr() as *mut u16, 1);
            *buf = pad[0];
        } else {
            pad[0] = *buf;
            raw_outsw(data_addr as *mut u16, pad.as_mut_ptr() as *mut u16, 1);
        }
        words += 1;
    }

    words << 1
}

/*
 * Provide our own set_mode() as we don't want to change anything that has
 * already been configured..
 */
unsafe fn pata_buddha_set_mode(link: *mut ata_link, _unused: *mut *mut ata_device) -> i32 {
    let mut dev: *mut ata_device = core::ptr::null_mut();
    ata_for_each_dev!(dev, link, ENABLED, {
        // We don't really care
        (*dev).pio_mode = XFER_PIO_0;
        (*dev).xfer_mode = XFER_PIO_0;
        (*dev).xfer_shift = ATA_SHIFT_PIO;
        (*dev).flags |= ATA_DFLAG_PIO;
        ata_dev_info!(dev, "configured for PIO\n");
    });
    0
}

unsafe fn pata_buddha_irq_check(ap: *mut ata_port) -> bool {
    let ch = z_readb((*ap).private_data as usize);
    (ch & 0x80) != 0
}

unsafe fn pata_xsurf_irq_clear(ap: *mut ata_port) {
    z_writeb(0, (*ap).private_data as usize);
}

// C static ata_port_operations tables. Field initializers retain the source layout.
static mut PATA_BUDDHA_OPS: ata_port_operations = ata_port_operations {
    inherits: &ata_sff_port_ops,
    sff_data_xfer: Some(pata_buddha_data_xfer),
    sff_irq_check: Some(pata_buddha_irq_check),
    cable_detect: Some(ata_cable_unknown),
    set_mode: Some(pata_buddha_set_mode),
    ..ata_port_operations::zeroed()
};

static mut PATA_XSURF_OPS: ata_port_operations = ata_port_operations {
    inherits: &ata_sff_port_ops,
    sff_data_xfer: Some(pata_buddha_data_xfer),
    sff_irq_check: Some(pata_buddha_irq_check),
    sff_irq_clear: Some(pata_xsurf_irq_clear),
    cable_detect: Some(ata_cable_unknown),
    set_mode: Some(pata_buddha_set_mode),
    ..ata_port_operations::zeroed()
};

unsafe fn pata_buddha_probe(z: *mut zorro_dev, ent: *const zorro_device_id) -> i32 {
    static BOARD_NAME: [&str; 3] = ["Buddha", "Catweasel", "X-Surf"];
    let mut host: *mut ata_host;
    let mut buddha_board: *mut core::ffi::c_void;
    let board = (*z).resource.start;
    let typ = (*ent).driver_data as u32;
    let nr_ports = if typ == BOARD_CATWEASEL { 3 } else { 2 };
    let mut old_drvdata: *mut core::ffi::c_void = core::ptr::null_mut();

    dev_info!(&(*z).dev, "{} IDE controller\n", BOARD_NAME[typ as usize]);

    if typ != BOARD_XSURF {
        if devm_request_mem_region(&(*z).dev, board + BUDDHA_BASE1, 0x800, DRV_NAME).is_null() {
            return -ENXIO;
        }
    } else {
        if devm_request_mem_region(&(*z).dev, board + XSURF_BASE1, 0x1000, DRV_NAME).is_null() {
            return -ENXIO;
        }
        let _ = devm_request_mem_region(&(*z).dev, board + XSURF_BASE2, 0x1000, DRV_NAME);
    }

    // Workaround for X-Surf: Save drvdata in case zorro8390 has set it
    if typ == BOARD_XSURF { old_drvdata = dev_get_drvdata(&(*z).dev); }

    // allocate host
    host = ata_host_alloc(&(*z).dev, nr_ports);
    if typ == BOARD_XSURF { dev_set_drvdata(&(*z).dev, old_drvdata); }
    if host.is_null() { return -ENXIO; }

    buddha_board = ZTWO_VADDR(board);

    // enable the board IRQ on Buddha/Catweasel
    if typ != BOARD_XSURF { z_writeb(0, buddha_board.add(BUDDHA_IRQ_MR)); }

    for i in 0..nr_ports {
        let ap = *(*host).ports.add(i as usize);
        let (base, ctl, irqport);
        if typ != BOARD_XSURF {
            (*ap).ops = &PATA_BUDDHA_OPS;
            base = buddha_board.add(BUDDHA_BASES[i as usize] as usize);
            ctl = BUDDHA_CONTROL;
            irqport = buddha_board.add(BUDDHA_IRQ + i as usize * 0x40);
        } else {
            (*ap).ops = &PATA_XSURF_OPS;
            base = buddha_board.add(XSURF_BASES[i as usize] as usize);
            // X-Surf has no CS1* (Control/AltStat)
            ctl = 0;
            irqport = buddha_board.add(XSURF_IRQ);
        }

        (*ap).pio_mask = ATA_PIO4;
        (*ap).flags |= ATA_FLAG_SLAVE_POSS | ATA_FLAG_NO_IORDY;
        (*ap).ioaddr.data_addr = base;
        (*ap).ioaddr.error_addr = base.add(2 + 1 * 4);
        (*ap).ioaddr.feature_addr = base.add(2 + 1 * 4);
        (*ap).ioaddr.nsect_addr = base.add(2 + 2 * 4);
        (*ap).ioaddr.lbal_addr = base.add(2 + 3 * 4);
        (*ap).ioaddr.lbam_addr = base.add(2 + 4 * 4);
        (*ap).ioaddr.lbah_addr = base.add(2 + 5 * 4);
        (*ap).ioaddr.device_addr = base.add(2 + 6 * 4);
        (*ap).ioaddr.status_addr = base.add(2 + 7 * 4);
        (*ap).ioaddr.command_addr = base.add(2 + 7 * 4);
        if ctl != 0 {
            (*ap).ioaddr.altstatus_addr = base.add(ctl);
            (*ap).ioaddr.ctl_addr = base.add(ctl);
        }
        (*ap).private_data = irqport;
        ata_port_desc!(ap, "cmd 0x{:lx} ctl 0x{:lx}", board,
            if ctl != 0 { board + BUDDHA_BASES[i as usize] as usize + ctl } else { 0 });
    }

    ata_host_activate(host, IRQ_AMIGA_PORTS, ata_sff_interrupt, IRQF_SHARED, &PATA_BUDDHA_SHT);
    0
}

unsafe fn pata_buddha_remove(z: *mut zorro_dev) {
    let host = dev_get_drvdata(&(*z).dev) as *mut ata_host;
    ata_host_detach(host);
}

static PATA_BUDDHA_SHT: scsi_host_template = ATA_PIO_SHT!(DRV_NAME);

// The following driver registration and module metadata correspond to the C macros.
static PATA_BUDDHA_ZORRO_TBL: [zorro_device_id; 3] = [
    zorro_device_id { id: ZORRO_PROD_INDIVIDUAL_COMPUTERS_BUDDHA, driver_data: BOARD_BUDDHA as usize },
    zorro_device_id { id: ZORRO_PROD_INDIVIDUAL_COMPUTERS_CATWEASEL, driver_data: BOARD_CATWEASEL as usize },
    zorro_device_id::default(),
];

static mut PATA_BUDDHA_DRIVER: zorro_driver = zorro_driver {
    name: DRV_NAME,
    id_table: PATA_BUDDHA_ZORRO_TBL.as_ptr(),
    probe: Some(pata_buddha_probe),
    remove: Some(pata_buddha_remove),
    ..zorro_driver::zeroed()
};

/*
 * We cannot have a modalias for X-Surf boards, as it competes with the
 * zorro8390 network driver. As a stopgap measure until we have proper
 * MFD support for this board, we manually attach to it late after Zorro
 * has enumerated its boards.
 */
unsafe fn pata_buddha_late_init() -> i32 {
    let mut z: *mut zorro_dev = core::ptr::null_mut();

    // Auto-bind to regular boards
    zorro_register_driver(&mut PATA_BUDDHA_DRIVER);

    // Manually bind to all X-Surf boards
    while {
        z = zorro_find_device(ZORRO_PROD_INDIVIDUAL_COMPUTERS_X_SURF, z);
        !z.is_null()
    } {
        let xsurf_ent = zorro_device_id {
            id: ZORRO_PROD_INDIVIDUAL_COMPUTERS_X_SURF,
            driver_data: BOARD_XSURF as usize,
        };
        pata_buddha_probe(z, &xsurf_ent);
    }
    0
}

// late_initcall(pata_buddha_late_init);
// MODULE_AUTHOR("Bartlomiej Zolnierkiewicz");
// MODULE_DESCRIPTION("low-level driver for Buddha/Catweasel/X-Surf PATA");
// MODULE_LICENSE("GPL v2");
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
