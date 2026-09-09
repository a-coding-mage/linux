/*
 * Generic platform device PATA driver
 *
 * Copyright (C) 2006 - 2007  Paul Mundt
 *
 * Based on pata_pcmcia:
 *
 *   Copyright 2005-2006 Red Hat Inc, all rights reserved.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Kernel dependencies supplied by other translation units.
use crate::*;

const DRV_NAME: &str = "pata_platform";
const DRV_VERSION: &str = "1.2";

static mut pio_mask: i32 = 1;

/*
 * Provide our own set_mode() as we don't want to change anything that has
 * already been configured..
 */
unsafe fn pata_platform_set_mode(
    link: *mut ata_link,
    _unused: *mut *mut ata_device,
) -> i32 {
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

static pata_platform_sht: scsi_host_template = ATA_PIO_SHT!(DRV_NAME);

unsafe fn pata_platform_setup_port(ioaddr: *mut ata_ioports, shift: u32) {
    /* Fixup the port shift for platforms that need it */
    (*ioaddr).data_addr = (*ioaddr).cmd_addr + ((ATA_REG_DATA as usize) << shift);
    (*ioaddr).error_addr = (*ioaddr).cmd_addr + ((ATA_REG_ERR as usize) << shift);
    (*ioaddr).feature_addr = (*ioaddr).cmd_addr + ((ATA_REG_FEATURE as usize) << shift);
    (*ioaddr).nsect_addr = (*ioaddr).cmd_addr + ((ATA_REG_NSECT as usize) << shift);
    (*ioaddr).lbal_addr = (*ioaddr).cmd_addr + ((ATA_REG_LBAL as usize) << shift);
    (*ioaddr).lbam_addr = (*ioaddr).cmd_addr + ((ATA_REG_LBAM as usize) << shift);
    (*ioaddr).lbah_addr = (*ioaddr).cmd_addr + ((ATA_REG_LBAH as usize) << shift);
    (*ioaddr).device_addr = (*ioaddr).cmd_addr + ((ATA_REG_DEVICE as usize) << shift);
    (*ioaddr).status_addr = (*ioaddr).cmd_addr + ((ATA_REG_STATUS as usize) << shift);
    (*ioaddr).command_addr = (*ioaddr).cmd_addr + ((ATA_REG_CMD as usize) << shift);
}

/**
 * __pata_platform_probe - attach a platform interface
 * @dev: device
 * @io_res: Resource representing I/O base
 * @ctl_res: Resource representing CTL base
 * @irq_res: Resource representing IRQ and its flags
 * @ioport_shift: I/O port shift
 * @__pio_mask: PIO mask
 * @sht: scsi_host_template to use when registering
 * @use16bit: Flag to indicate 16-bit IO instead of 32-bit
 */
pub unsafe extern "C" fn __pata_platform_probe(
    dev: *mut device,
    io_res: *mut resource,
    ctl_res: *mut resource,
    irq_res: *mut resource,
    ioport_shift: u32,
    __pio_mask: i32,
    sht: *const scsi_host_template,
    use16bit: bool,
) -> i32 {
    let mut host: *mut ata_host;
    let ap: *mut ata_port;
    let mmio: bool;
    let mut irq: i32 = 0;
    let mut irq_flags: i32 = 0;

    /* Check for MMIO */
    mmio = (*io_res).flags == IORESOURCE_MEM && (*ctl_res).flags == IORESOURCE_MEM;

    /* And the IRQ */
    if !irq_res.is_null() && (*irq_res).start > 0 {
        irq = (*irq_res).start as i32;
        irq_flags = (((*irq_res).flags & IRQF_TRIGGER_MASK) | IRQF_SHARED) as i32;
    }

    /* Now that that's out of the way, wire up the port.. */
    host = ata_host_alloc(dev, 1);
    if host.is_null() { return -ENOMEM; }
    ap = (*host).ports[0];

    (*ap).ops = devm_kzalloc(dev, core::mem::size_of::<ata_port_operations>(), GFP_KERNEL)
        as *mut ata_port_operations;
    if (*ap).ops.is_null() { return -ENOMEM; }
    (*(*ap).ops).inherits = &ata_sff_port_ops;
    (*(*ap).ops).cable_detect = Some(ata_cable_unknown);
    (*(*ap).ops).set_mode = Some(pata_platform_set_mode);
    (*(*ap).ops).sff_data_xfer = if use16bit { Some(ata_sff_data_xfer) } else { Some(ata_sff_data_xfer32) };
    (*ap).pio_mask = __pio_mask;
    (*ap).flags |= ATA_FLAG_SLAVE_POSS;

    /* Use polling mode if there's no IRQ */
    if irq == 0 {
        (*ap).flags |= ATA_FLAG_PIO_POLLING;
        ata_port_desc!(ap, "no IRQ, using PIO polling");
    }

    /* Handle the MMIO case */
    if mmio {
        (*ap).ioaddr.cmd_addr = devm_ioremap(dev, (*io_res).start, resource_size(io_res));
        (*ap).ioaddr.ctl_addr = devm_ioremap(dev, (*ctl_res).start, resource_size(ctl_res));
    } else {
        (*ap).ioaddr.cmd_addr = devm_ioport_map(dev, (*io_res).start, resource_size(io_res));
        (*ap).ioaddr.ctl_addr = devm_ioport_map(dev, (*ctl_res).start, resource_size(ctl_res));
    }
    if (*ap).ioaddr.cmd_addr.is_null() || (*ap).ioaddr.ctl_addr.is_null() {
        dev_err!(dev, "failed to map IO/CTL base\n");
        return -ENOMEM;
    }

    (*ap).ioaddr.altstatus_addr = (*ap).ioaddr.ctl_addr;
    pata_platform_setup_port(&mut (*ap).ioaddr, ioport_shift);
    ata_port_desc!(ap, "%s cmd 0x%llx ctl 0x%llx", if mmio { "mmio" } else { "ioport" },
        (*io_res).start as u64, (*ctl_res).start as u64);

    /* activate */
    ata_host_activate(host, irq, if irq != 0 { Some(ata_sff_interrupt) } else { None }, irq_flags, sht)
}

unsafe fn pata_platform_probe(pdev: *mut platform_device) -> i32 {
    let pp_info = dev_get_platdata(&mut (*pdev).dev) as *mut pata_platform_info;
    if (*pdev).num_resources != 3 && (*pdev).num_resources != 2 {
        dev_err!(&mut (*pdev).dev, "invalid number of resources\n");
        return -EINVAL;
    }
    let io_res = platform_get_mem_or_io(pdev, 0);
    if io_res.is_null() { return -EINVAL; }
    let ctl_res = platform_get_mem_or_io(pdev, 1);
    if ctl_res.is_null() { return -EINVAL; }
    let irq_res = platform_get_resource(pdev, IORESOURCE_IRQ, 0);
    __pata_platform_probe(&mut (*pdev).dev, io_res, ctl_res, irq_res,
        if pp_info.is_null() { 0 } else { (*pp_info).ioport_shift },
        pio_mask, &pata_platform_sht, false)
}

static mut pata_platform_driver: platform_driver = platform_driver {
    probe: Some(pata_platform_probe),
    remove: Some(ata_platform_remove_one),
    driver: driver { name: DRV_NAME },
};

// module_platform_driver!(pata_platform_driver);
// EXPORT_SYMBOL_GPL!(__pata_platform_probe);
// MODULE_AUTHOR!("Paul Mundt");
// MODULE_DESCRIPTION!("low-level driver for platform device ATA");
// MODULE_LICENSE!("GPL");
// MODULE_VERSION!(DRV_VERSION);
// MODULE_ALIAS!(concat!("platform:", DRV_NAME));

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
