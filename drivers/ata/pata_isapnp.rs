// SPDX-License-Identifier: GPL-2.0-only

/*
 *   pata-isapnp.c - ISA PnP PATA controller driver.
 *   Copyright 2005/2006 Red Hat Inc, all rights reserved.
 *
 *   Based in part on ide-pnp.c by Andrey Panin <pazke@donpac.ru>
 */

// Linux kernel dependencies supplied by other translation units.

pub const DRV_NAME: &[u8] = b"pata_isapnp\0";
pub const DRV_VERSION: &[u8] = b"0.2.5\0";

static mut ISAPNP_SHT: scsi_host_template = scsi_host_template {
    // ATA_PIO_SHT(DRV_NAME)
};

static mut ISAPNP_PORT_OPS: ata_port_operations = ata_port_operations {
    inherits: unsafe { &ata_sff_port_ops },
    cable_detect: Some(ata_cable_40wire),
};

static mut ISAPNP_NOALT_PORT_OPS: ata_port_operations = ata_port_operations {
    inherits: unsafe { &ata_sff_port_ops },
    cable_detect: Some(ata_cable_40wire),
    /* No altstatus so we don't want to use the lost interrupt poll */
    lost_interrupt: ATA_OP_NULL,
};

/**
 *\tisapnp_init_one\t\t- attach an isapnp interface
 *\t@idev: PnP device
 *\t@dev_id: matching detect line
 *
 *\tRegister an ISA bus IDE interface. Such interfaces are PIO 0 and
 *\tnon shared IRQ.
 */
unsafe fn isapnp_init_one(
    idev: *mut pnp_dev,
    _dev_id: *const pnp_device_id,
) -> i32 {
    let mut host: *mut ata_host;
    let ap: *mut ata_port;
    let mut cmd_addr: *mut core::ffi::c_void;
    let mut ctl_addr: *mut core::ffi::c_void;
    let mut irq: i32 = 0;
    let mut handler: irq_handler_t = None;

    if pnp_port_valid(idev, 0) == 0 {
        return -ENODEV;
    }

    if pnp_irq_valid(idev, 0) != 0 {
        irq = pnp_irq(idev, 0);
        handler = Some(ata_sff_interrupt);
    }

    /* allocate host */
    host = ata_host_alloc(&mut (*idev).dev, 1);
    if host.is_null() {
        return -ENOMEM;
    }

    /* acquire resources and fill host */
    cmd_addr = devm_ioport_map(&mut (*idev).dev, pnp_port_start(idev, 0), 8);
    if cmd_addr.is_null() {
        return -ENOMEM;
    }

    ap = (*host).ports[0];

    (*ap).ops = &mut ISAPNP_NOALT_PORT_OPS;
    (*ap).pio_mask = ATA_PIO0;
    (*ap).flags |= ATA_FLAG_SLAVE_POSS;

    (*ap).ioaddr.cmd_addr = cmd_addr;

    if pnp_port_valid(idev, 1) != 0 {
        ctl_addr = devm_ioport_map(&mut (*idev).dev, pnp_port_start(idev, 1), 1);
        if ctl_addr.is_null() {
            return -ENOMEM;
        }

        (*ap).ioaddr.altstatus_addr = ctl_addr;
        (*ap).ioaddr.ctl_addr = ctl_addr;
        (*ap).ops = &mut ISAPNP_PORT_OPS;
    }

    ata_sff_std_ports(&mut (*ap).ioaddr);

    ata_port_desc(
        ap,
        b"cmd 0x%llx ctl 0x%llx\0".as_ptr(),
        pnp_port_start(idev, 0) as u64,
        pnp_port_start(idev, 1) as u64,
    );

    /* activate */
    ata_host_activate(host, irq, handler, 0, &ISAPNP_SHT)
}

/**
 *\tisapnp_remove_one\t- unplug an isapnp interface
 *\t@idev: PnP device
 *
 *\tRemove a previously configured PnP ATA port. Called only on module
 *\tunload events as the core does not currently deal with ISAPnP docking.
 */
unsafe fn isapnp_remove_one(idev: *mut pnp_dev) {
    let dev: *mut device = &mut (*idev).dev;
    let host: *mut ata_host = dev_get_drvdata(dev);

    ata_host_detach(host);
}

static mut ISAPNP_DEVICES: [pnp_device_id; 2] = [
    /* Generic ESDI/IDE/ATA compatible hard disk controller */
    pnp_device_id { id: *b"PNP0600\0" },
    pnp_device_id { id: [0; 8] },
];

// MODULE_DEVICE_TABLE(pnp, isapnp_devices);

static mut ISAPNP_DRIVER: pnp_driver = pnp_driver {
    name: DRV_NAME.as_ptr(),
    id_table: ISAPNP_DEVICES.as_ptr(),
    probe: Some(isapnp_init_one),
    remove: Some(isapnp_remove_one),
};

// module_pnp_driver(isapnp_driver);
// MODULE_AUTHOR("Alan Cox");
// MODULE_DESCRIPTION("low-level driver for ISA PnP ATA");
// MODULE_LICENSE("GPL");
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
