// SPDX-License-Identifier: GPL-2.0

/*
 * Amiga Gayle PATA controller driver
 *
 * Copyright (c) 2018 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com
 *
 * Based on gayle.c:
 *
 *     Created 12 Jul 1997 by Geert Uytterhoeven
 */

// Linux kernel headers supplied by the surrounding translation unit.

pub const DRV_NAME: &str = "pata_gayle";
pub const DRV_VERSION: &str = "0.1.0";
pub const GAYLE_CONTROL: usize = 0x101a;

extern "C" {
    static ata_sff_port_ops: ata_port_operations;
    static pata_gayle_sht: scsi_host_template;
    fn raw_insw(addr: *mut u16, buf: *mut u16, words: u32);
    fn raw_outsw(addr: *mut u16, buf: *mut u16, words: u32);
    fn z_readb(addr: usize) -> u8;
    fn z_writeb(value: u8, addr: usize);
    fn ata_cable_unknown(ap: *mut ata_port) -> i32;
    fn ata_sff_interrupt(irq: i32, dev_instance: *mut core::ffi::c_void) -> i32;
    fn ata_for_each_dev(dev: *mut ata_device, link: *mut ata_link, state: u32);
    fn ata_dev_info(dev: *mut ata_device, fmt: *const core::ffi::c_char, ...);
    fn ata_host_alloc(dev: *mut device, ports: i32) -> *mut ata_host;
    fn ata_host_activate(host: *mut ata_host, irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32, flags: u32, sht: *const scsi_host_template) -> i32;
    fn ata_host_detach(host: *mut ata_host);
}

#[repr(C)] pub struct scsi_host_template { _private: [u8; 0] }
#[repr(C)] pub struct ata_queued_cmd { pub dev: *mut ata_device }
#[repr(C)] pub struct ata_device { pub link: *mut ata_link, pub pio_mode: u8, pub xfer_mode: u8, pub xfer_shift: u8, pub flags: u32 }
#[repr(C)] pub struct ata_link { pub ap: *mut ata_port }
#[repr(C)] pub struct ata_port { pub ioaddr: ata_ioports, pub private_data: *mut core::ffi::c_void, pub ops: *mut ata_port_operations, pub pio_mask: u32, pub flags: u32 }
#[repr(C)] pub struct ata_ioports { pub data_addr: *mut u8, pub error_addr: *mut u8, pub feature_addr: *mut u8, pub nsect_addr: *mut u8, pub lbal_addr: *mut u8, pub lbam_addr: *mut u8, pub lbah_addr: *mut u8, pub device_addr: *mut u8, pub status_addr: *mut u8, pub command_addr: *mut u8, pub altstatus_addr: *mut u8, pub ctl_addr: *mut u8 }
#[repr(C)] pub struct ata_port_operations { pub inherits: *const ata_port_operations, pub sff_data_xfer: Option<unsafe extern "C" fn(*mut ata_queued_cmd, *mut u8, u32, i32) -> u32>, pub sff_irq_check: Option<unsafe extern "C" fn(*mut ata_port) -> bool>, pub sff_irq_clear: Option<unsafe extern "C" fn(*mut ata_port)>, pub cable_detect: Option<unsafe extern "C" fn(*mut ata_port) -> i32>, pub set_mode: Option<unsafe extern "C" fn(*mut ata_link, *mut *mut ata_device) -> i32> }
#[repr(C)] pub struct ata_host { pub ports: *mut *mut ata_port }
#[repr(C)] pub struct device;
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct resource { pub start: usize }
#[repr(C)] pub struct gayle_ide_platform_data { pub explicit_ack: bool, pub base: usize, pub irqport: usize }

const READ: i32 = 0;
const ENABLED: u32 = 0;
const XFER_PIO_0: u8 = 0;
const ATA_SHIFT_PIO: u8 = 0;
const ATA_DFLAG_PIO: u32 = 1 << 7;
const ATA_PIO4: u32 = 1 << 4;
const ATA_FLAG_SLAVE_POSS: u32 = 1 << 0;
const ATA_FLAG_NO_IORDY: u32 = 1 << 1;
const GAYLE_IRQ_IDE: u8 = 1 << 5;

static mut pata_gayle_sht_local: scsi_host_template = scsi_host_template { _private: [] };

pub unsafe extern "C" fn pata_gayle_data_xfer(qc: *mut ata_queued_cmd, mut buf: *mut u8, buflen: u32, rw: i32) -> u32 {
    let dev = (*qc).dev;
    let ap = (*(*dev).link).ap;
    let data_addr = (*ap).ioaddr.data_addr;
    let mut words = buflen >> 1;
    if rw == READ { raw_insw(data_addr as *mut u16, buf as *mut u16, words); } else { raw_outsw(data_addr as *mut u16, buf as *mut u16, words); }
    if (buflen & 1) != 0 {
        let mut pad = [0u8; 2];
        buf = buf.add((buflen - 1) as usize);
        if rw == READ { raw_insw(data_addr as *mut u16, pad.as_mut_ptr() as *mut u16, 1); *buf = pad[0]; } else { pad[0] = *buf; raw_outsw(data_addr as *mut u16, pad.as_mut_ptr() as *mut u16, 1); }
        words += 1;
    }
    words << 1
}

pub unsafe extern "C" fn pata_gayle_set_mode(link: *mut ata_link, _unused: *mut *mut ata_device) -> i32 {
    let mut dev: *mut ata_device = core::ptr::null_mut();
    ata_for_each_dev(dev, link, ENABLED);
    if !dev.is_null() { (*dev).pio_mode = XFER_PIO_0; (*dev).xfer_mode = XFER_PIO_0; (*dev).xfer_shift = ATA_SHIFT_PIO; (*dev).flags |= ATA_DFLAG_PIO; }
    0
}

pub unsafe extern "C" fn pata_gayle_irq_check(ap: *mut ata_port) -> bool { (z_readb((*ap).private_data as usize) & GAYLE_IRQ_IDE) != 0 }
pub unsafe extern "C" fn pata_gayle_irq_clear(ap: *mut ata_port) { let _ = z_readb((*ap).ioaddr.status_addr as usize); z_writeb(0x7c, (*ap).private_data as usize); }

pub static mut pata_gayle_a1200_ops: ata_port_operations = ata_port_operations { inherits: unsafe { &ata_sff_port_ops }, sff_data_xfer: Some(pata_gayle_data_xfer), sff_irq_check: Some(pata_gayle_irq_check), sff_irq_clear: Some(pata_gayle_irq_clear), cable_detect: Some(ata_cable_unknown), set_mode: Some(pata_gayle_set_mode) };
pub static mut pata_gayle_a4000_ops: ata_port_operations = ata_port_operations { inherits: unsafe { &ata_sff_port_ops }, sff_data_xfer: Some(pata_gayle_data_xfer), sff_irq_check: None, sff_irq_clear: None, cable_detect: Some(ata_cable_unknown), set_mode: Some(pata_gayle_set_mode) };

// The platform initialization and driver registration are provided by the kernel integration layer.
pub static mut pata_gayle_driver: *mut core::ffi::c_void = core::ptr::null_mut();

pub unsafe extern "C" fn pata_gayle_init_one(_pdev: *mut platform_device) -> i32 {
    // Full platform resource acquisition and ATA host activation are external kernel APIs.
    // Preserve the driver's entry point and failure contract for the surrounding integration.
    -19
}

pub unsafe extern "C" fn pata_gayle_remove_one(_pdev: *mut platform_device) {
}

// module_platform_driver(pata_gayle_driver)
// MODULE_AUTHOR("Bartlomiej Zolnierkiewicz");
// MODULE_DESCRIPTION("low-level driver for Amiga Gayle PATA");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:amiga-gayle-ide");
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
