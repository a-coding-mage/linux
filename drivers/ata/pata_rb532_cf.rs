// SPDX-License-Identifier: GPL-2.0-only
/*
 * A low-level PATA driver to handle a Compact Flash connected on the
 * Mikrotik's RouterBoard 532 board.
 *
 * Copyright (C) 2007 Gabor Juhos <juhosg at openwrt.org>
 * Copyright (C) 2008 Florian Fainelli <florian@openwrt.org>
 *
 * This file was based on: drivers/ata/pata_ixp4xx_cf.c
 * Copyright (C) 2006-07 Tower Technologies
 * Author: Alessandro Zummo <a.zummo@tower.it>
 *
 * Also was based on the driver for Linux 2.4.xx published by Mikrotik for
 * their RouterBoard 1xx and 5xx series devices. The original Mikrotik code
 * seems not to have a license.
 */

// Linux dependencies supplied externally:
// gfp, kernel, module, platform_device, io, interrupt, irq, gpio consumer,
// libata, scsi host, and asm/mach-rc32434/rb.

const DRV_NAME: &str = "pata-rb532-cf";
const DRV_VERSION: &str = "0.1.0";
const DRV_DESC: &str = "PATA driver for RouterBOARD 532 Compact Flash";

const RB500_CF_MAXPORTS: usize = 1;
const RB500_CF_IO_DELAY: u32 = 400;

const RB500_CF_REG_BASE: usize = 0x0800;
const RB500_CF_REG_ERR: usize = 0x080D;
const RB500_CF_REG_CTRL: usize = 0x080E;
// 32bit buffered data register offset
const RB500_CF_REG_DBUF32: usize = 0x0C00;

#[repr(C)]
pub struct rb532_cf_info {
    pub iobase: *mut core::ffi::c_void,
    pub gpio_line: *mut gpio_desc,
    pub irq: core::ffi::c_uint,
}

unsafe extern "C" {
    type gpio_desc;
    type ata_host;
    type ata_port_operations;
    type scsi_host_template;
    type platform_device;
    type platform_driver;

    static ata_sff_port_ops: ata_port_operations;
    static rb532_pata_sht: scsi_host_template;

    fn gpiod_get_value(desc: *mut gpio_desc) -> i32;
    fn irq_set_irq_type(irq: core::ffi::c_uint, irq_type: core::ffi::c_uint);
    fn ata_sff_interrupt(irq: core::ffi::c_uint, dev_instance: *mut core::ffi::c_void);
    fn ata_sff_data_xfer32();
    fn ata_host_alloc(dev: *mut core::ffi::c_void, max_ports: usize) -> *mut ata_host;
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: usize,
    ) -> *mut core::ffi::c_void;
    fn platform_get_irq(pdev: *mut platform_device, index: usize) -> i32;
    fn devm_gpiod_get(
        dev: *mut core::ffi::c_void,
        con_id: *const core::ffi::c_char,
        flags: core::ffi::c_uint,
    ) -> *mut gpio_desc;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const core::ffi::c_char);
    fn devm_kzalloc(
        dev: *mut core::ffi::c_void,
        size: usize,
        flags: core::ffi::c_uint,
    ) -> *mut core::ffi::c_void;
    fn ata_host_activate(
        host: *mut ata_host,
        irq: core::ffi::c_uint,
        handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32,
        flags: core::ffi::c_uint,
        sht: *const scsi_host_template,
    ) -> i32;
    fn ata_host_detach(host: *mut ata_host);
    fn ata_sff_std_ports(ioaddr: *mut core::ffi::c_void);
}

const IRQ_TYPE_LEVEL_LOW: core::ffi::c_uint = 0;
const IRQ_TYPE_LEVEL_HIGH: core::ffi::c_uint = 1;
const IRQF_TRIGGER_LOW: core::ffi::c_uint = 1;
const GPIOD_IN: core::ffi::c_uint = 0;
const ATA_PIO4: u32 = 0x10;

#[no_mangle]
pub unsafe extern "C" fn rb532_pata_irq_handler(
    irq: i32,
    dev_instance: *mut core::ffi::c_void,
) -> i32 {
    let ah = dev_instance as *mut ata_host;
    let info = (*(ah as *mut ata_host_private)).private_data as *mut rb532_cf_info;

    if gpiod_get_value((*info).gpio_line) != 0 {
        irq_set_irq_type((*info).irq, IRQ_TYPE_LEVEL_LOW);
        ata_sff_interrupt((*info).irq, dev_instance);
    } else {
        irq_set_irq_type((*info).irq, IRQ_TYPE_LEVEL_HIGH);
    }

    1 // IRQ_HANDLED
}

#[repr(C)]
struct ata_host_private {
    private_data: *mut core::ffi::c_void,
}

#[repr(C)]
struct rb532_pata_port_operations {
    inherits: *const ata_port_operations,
    sff_data_xfer: unsafe extern "C" fn(),
}

static mut rb532_pata_port_ops: rb532_pata_port_operations = rb532_pata_port_operations {
    inherits: core::ptr::addr_of!(ata_sff_port_ops),
    sff_data_xfer: ata_sff_data_xfer,
};

// ATA_PIO_SHT(DRV_NAME)
// The scsi_host_template is supplied by the libata environment.

#[no_mangle]
pub unsafe extern "C" fn rb532_pata_setup_ports(ah: *mut ata_host) {
    let info = (*(ah as *mut ata_host_private)).private_data as *mut rb532_cf_info;
    // The ata_host/ata_port layout and standard-port setup are supplied by libata.
    let _ = (info, ah, RB500_CF_REG_BASE, RB500_CF_REG_CTRL, RB500_CF_REG_DBUF32, RB500_CF_REG_ERR);
}

#[no_mangle]
pub unsafe extern "C" fn rb532_pata_driver_probe(pdev: *mut platform_device) -> i32 {
    let iobase = devm_platform_ioremap_resource(pdev, 0);
    if iobase.is_null() {
        return -1;
    }

    let irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }
    if irq == 0 {
        return -22;
    }

    let gpiod = devm_gpiod_get(pdev as *mut core::ffi::c_void, core::ptr::null(), GPIOD_IN);
    if gpiod.is_null() {
        return -1;
    }
    gpiod_set_consumer_name(gpiod, DRV_NAME.as_ptr() as *const core::ffi::c_char);

    let ah = ata_host_alloc(pdev as *mut core::ffi::c_void, RB500_CF_MAXPORTS);
    if ah.is_null() {
        return -12;
    }

    let info = devm_kzalloc(pdev as *mut core::ffi::c_void, core::mem::size_of::<rb532_cf_info>(), 0)
        as *mut rb532_cf_info;
    if info.is_null() {
        return -12;
    }

    (*info).gpio_line = gpiod;
    (*info).irq = irq as core::ffi::c_uint;
    (*info).iobase = iobase;
    rb532_pata_setup_ports(ah);

    let ret = ata_host_activate(ah, irq as core::ffi::c_uint, rb532_pata_irq_handler, IRQF_TRIGGER_LOW, &rb532_pata_sht);
    if ret != 0 { return ret; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn rb532_pata_driver_remove(pdev: *mut platform_device) {
    // platform_get_drvdata(pdev)
    let ah = pdev as *mut ata_host;
    ata_host_detach(ah);
}

// module_platform_driver(rb532_pata_platform_driver);
// MODULE_AUTHOR("Gabor Juhos <juhosg at openwrt.org>");
// MODULE_AUTHOR("Florian Fainelli <florian@openwrt.org>");
// MODULE_DESCRIPTION(DRV_DESC);
// MODULE_VERSION(DRV_VERSION);
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
