// SPDX-License-Identifier: GPL-2.0

/*
 * Atari Falcon PATA controller driver
 *
 * Copyright (c) 2016 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com
 *
 * Based on falconide.c:
 *
 *     Created 12 Jul 1997 by Geert Uytterhoeven
 */

// Linux kernel dependencies supplied externally:
// linux/kernel.h, linux/module.h, linux/init.h, linux/blkdev.h,
// linux/delay.h, scsi/scsi_host.h, scsi/scsi_cmnd.h, linux/ata.h,
// linux/libata.h, linux/mm.h, linux/interrupt.h, linux/platform_device.h,
// asm/setup.h, asm/atarihw.h, asm/atariints.h, asm/atari_stdma.h

const DRV_NAME: &str = "pata_falcon";
const DRV_VERSION: &str = "0.1.0";

static mut pata_falcon_swap_mask: i32 = 0;

// module_param_named(data_swab, pata_falcon_swap_mask, int, 0444);
// MODULE_PARM_DESC(data_swab, "Data byte swap enable/disable bitmap (0x1==drive1, 0x2==drive2, 0x4==drive3, 0x8==drive4, default==0)");

static pata_falcon_sht: scsi_host_template = scsi_host_template {
    // ATA_PIO_SHT(DRV_NAME)
};

unsafe fn pata_falcon_data_xfer(
    qc: *mut ata_queued_cmd,
    mut buf: *mut u8,
    buflen: u32,
    rw: i32,
) -> u32 {
    let dev = (*qc).dev;
    let ap = (*(*dev).link).ap;
    let data_addr = (*ap).ioaddr.data_addr;
    let mut words = buflen >> 1;
    let cmd = (*qc).scsicmd;
    let mut swap = true;

    if (*dev).class == ATA_DEV_ATA && !cmd.is_null()
        && !blk_rq_is_passthrough(scsi_cmd_to_rq(cmd))
    {
        swap = ((*ap).private_data as usize & (1usize << (*dev).devno)) != 0;
    }

    /* Transfer multiple of 2 bytes */
    if rw == READ {
        if swap {
            raw_insw_swapw(data_addr, buf as *mut u16, words);
        } else {
            raw_insw(data_addr, buf as *mut u16, words);
        }
    } else if swap {
        raw_outsw_swapw(data_addr, buf as *mut u16, words);
    } else {
        raw_outsw(data_addr, buf as *mut u16, words);
    }

    /* Transfer trailing byte, if any. */
    if (buflen & 0x01) != 0 {
        let mut pad = [0u8; 2];

        /* Point buf to the tail of buffer */
        buf = buf.add(buflen as usize - 1);

        if rw == READ {
            if swap {
                raw_insw_swapw(data_addr, pad.as_mut_ptr() as *mut u16, 1);
            } else {
                raw_insw(data_addr, pad.as_mut_ptr() as *mut u16, 1);
            }
            *buf = pad[0];
        } else {
            pad[0] = *buf;
            if swap {
                raw_outsw_swapw(data_addr, pad.as_mut_ptr() as *mut u16, 1);
            } else {
                raw_outsw(data_addr, pad.as_mut_ptr() as *mut u16, 1);
            }
        }
        words += 1;
    }

    words << 1
}

/*
 * Provide our own set_mode() as we don't want to change anything that has
 * already been configured..
 */
unsafe fn pata_falcon_set_mode(link: *mut ata_link, _unused: *mut *mut ata_device) -> i32 {
    let mut dev: *mut ata_device = core::ptr::null_mut();

    ata_for_each_dev!(dev, link, ENABLED, {
        /* We don't really care */
        (*dev).pio_mode = XFER_PIO_0;
        (*dev).xfer_mode = XFER_PIO_0;
        (*dev).xfer_shift = ATA_SHIFT_PIO;
        (*dev).flags |= ATA_DFLAG_PIO;
        ata_dev_info!(dev, "configured for PIO\n");
    });
    0
}

static mut pata_falcon_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_sff_port_ops,
    sff_data_xfer: Some(pata_falcon_data_xfer),
    cable_detect: Some(ata_cable_unknown),
    set_mode: Some(pata_falcon_set_mode),
};

unsafe fn pata_falcon_init_one(pdev: *mut platform_device) -> i32 {
    let mut base_mem_res: *mut resource;
    let mut ctl_mem_res: *mut resource;
    let mut base_res: *mut resource;
    let mut ctl_res: *mut resource;
    let mut irq_res: *mut resource;
    let mut host: *mut ata_host;
    let ap: *mut ata_port;
    let mut base: *mut core::ffi::c_void;
    let mut ctl_base: *mut core::ffi::c_void;
    let mut mask_shift: i32 = 0; /* Q40 & Falcon default */
    let mut irq: i32 = 0;
    let mut io_offset: usize = 1;
    let mut reg_shift: u32 = 2; /* Falcon defaults */

    dev_info!(&(*pdev).dev, "Atari Falcon and Q40/Q60 PATA controller\n");

    base_res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if !base_res.is_null()
        && devm_request_region(&(*pdev).dev, (*base_res).start,
                               resource_size(base_res), DRV_NAME).is_null()
    {
        dev_err!(&(*pdev).dev, "resources busy\n");
        return -EBUSY;
    }

    ctl_res = platform_get_resource(pdev, IORESOURCE_IO, 1);
    if !ctl_res.is_null()
        && devm_request_region(&(*pdev).dev, (*ctl_res).start,
                               resource_size(ctl_res), DRV_NAME).is_null()
    {
        dev_err!(&(*pdev).dev, "resources busy\n");
        return -EBUSY;
    }

    base_mem_res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if base_mem_res.is_null() { return -ENODEV; }
    if devm_request_mem_region(&(*pdev).dev, (*base_mem_res).start,
                               resource_size(base_mem_res), DRV_NAME).is_null()
    {
        dev_err!(&(*pdev).dev, "resources busy\n");
        return -EBUSY;
    }

    ctl_mem_res = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    if ctl_mem_res.is_null() { return -ENODEV; }

    /* allocate host */
    host = ata_host_alloc(&(*pdev).dev, 1);
    if host.is_null() { return -ENOMEM; }
    ap = (*host).ports[0];

    (*ap).ops = &pata_falcon_ops;
    (*ap).pio_mask = ATA_PIO4;
    (*ap).flags |= ATA_FLAG_SLAVE_POSS | ATA_FLAG_NO_IORDY;

    /* N.B. this assumes data_addr will be used for word-sized I/O only */
    (*ap).ioaddr.data_addr = (*base_mem_res).start as *mut core::ffi::c_void;

    if !base_res.is_null() { /* only Q40 has IO resources */
        io_offset = 0x10000;
        reg_shift = 0;
        base = (*base_res).start as *mut core::ffi::c_void;
        ctl_base = (*ctl_res).start as *mut core::ffi::c_void;
    } else {
        base = (*base_mem_res).start as *mut core::ffi::c_void;
        ctl_base = (*ctl_mem_res).start as *mut core::ffi::c_void;
    }

    (*ap).ioaddr.error_addr = base.add(io_offset + (1usize << reg_shift));
    (*ap).ioaddr.feature_addr = base.add(io_offset + (1usize << reg_shift));
    (*ap).ioaddr.nsect_addr = base.add(io_offset + (2usize << reg_shift));
    (*ap).ioaddr.lbal_addr = base.add(io_offset + (3usize << reg_shift));
    (*ap).ioaddr.lbam_addr = base.add(io_offset + (4usize << reg_shift));
    (*ap).ioaddr.lbah_addr = base.add(io_offset + (5usize << reg_shift));
    (*ap).ioaddr.device_addr = base.add(io_offset + (6usize << reg_shift));
    (*ap).ioaddr.status_addr = base.add(io_offset + (7usize << reg_shift));
    (*ap).ioaddr.command_addr = base.add(io_offset + (7usize << reg_shift));
    (*ap).ioaddr.altstatus_addr = ctl_base.add(io_offset);
    (*ap).ioaddr.ctl_addr = ctl_base.add(io_offset);

    ata_port_desc!(ap, "cmd %px ctl %px data %px", base, ctl_base, (*ap).ioaddr.data_addr);

    if (*pdev).id > 0 { mask_shift = 2; }
    (*ap).private_data = (pata_falcon_swap_mask >> mask_shift) as usize as *mut core::ffi::c_void;

    irq_res = platform_get_resource(pdev, IORESOURCE_IRQ, 0);
    if !irq_res.is_null() && (*irq_res).start > 0 {
        irq = (*irq_res).start;
    } else {
        (*ap).flags |= ATA_FLAG_PIO_POLLING;
        ata_port_desc!(ap, "no IRQ, using PIO polling");
    }

    /* activate */
    ata_host_activate(host, irq, if irq != 0 { Some(ata_sff_interrupt) } else { None },
                      IRQF_SHARED, &pata_falcon_sht)
}

unsafe fn pata_falcon_remove_one(pdev: *mut platform_device) {
    let host = platform_get_drvdata(pdev) as *mut ata_host;
    ata_host_detach(host);
}

static mut pata_falcon_driver: platform_driver = platform_driver {
    probe: Some(pata_falcon_init_one),
    remove: Some(pata_falcon_remove_one),
    driver: driver { name: "atari-falcon-ide" },
};

// module_platform_driver!(pata_falcon_driver);
// MODULE_AUTHOR("Bartlomiej Zolnierkiewicz");
// MODULE_DESCRIPTION("low-level driver for Atari Falcon PATA");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:atari-falcon-ide");
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
