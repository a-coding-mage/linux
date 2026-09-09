// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004 IBM Corporation
 *
 * Authors:
 * Leendert van Doorn <leendert@watson.ibm.com>
 * Dave Safford <safford@watson.ibm.com>
 * Reiner Sailer <sailer@watson.ibm.com>
 * Kylene Hall <kjhall@us.ibm.com>
 *
 * Maintained by: <tpmdd-devel@lists.sourceforge.net>
 *
 * Device driver for TCG/TCPA TPM (trusted platform module).
 * Specifications at www.trustedcomputinggroup.org
 */

// Dependency declarations supplied by tpm.h and the kernel environment remain external.

#[repr(C)]
struct tpm_atmel_priv {
    region_size: i32,
    have_region: i32,
    base: c_ulong,
    iobase: *mut c_void,
}

#[inline]
unsafe fn atmel_getb(chip: *mut tpm_chip, offset: c_ulong) -> u8 {
    let priv_: *mut tpm_atmel_priv = atmel_get_priv(chip);
    inb((*priv_).base.wrapping_add(offset))
}

#[inline]
unsafe fn atmel_putb(val: u8, chip: *mut tpm_chip, offset: c_ulong) {
    let priv_: *mut tpm_atmel_priv = atmel_get_priv(chip);
    outb(val, (*priv_).base.wrapping_add(offset));
}

// atmel_request_region and atmel_release_region are aliases for the platform
// request_region and release_region functions supplied by the kernel.

// Atmel definitions
#[repr(i32)]
enum tpm_atmel_addr {
    TPM_ATMEL_BASE_ADDR_LO = 0x08,
    TPM_ATMEL_BASE_ADDR_HI = 0x09,
}

#[inline]
unsafe fn tpm_read_index(base: i32, index: i32) -> i32 {
    outb(index as u8, base as c_ulong);
    (inb((base + 1) as c_ulong) & 0xFF) as i32
}

/* Verify this is a 1.1 Atmel TPM */
unsafe fn atmel_verify_tpm11() -> i32 {
    /* verify that it is an Atmel part */
    if tpm_read_index(TPM_ADDR, 4) != b'A' as i32
        || tpm_read_index(TPM_ADDR, 5) != b'T' as i32
        || tpm_read_index(TPM_ADDR, 6) != b'M' as i32
        || tpm_read_index(TPM_ADDR, 7) != b'L' as i32
    {
        return 1;
    }

    /* query chip for its version number */
    if tpm_read_index(TPM_ADDR, 0x00) != 1 || tpm_read_index(TPM_ADDR, 0x01) != 1 {
        return 1;
    }

    /* This is an atmel supported part */
    0
}

/* Determine where to talk to device */
unsafe fn atmel_get_base_addr(base: *mut c_ulong, region_size: *mut i32) -> *mut c_void {
    let lo: i32;
    let hi: i32;

    if atmel_verify_tpm11() != 0 {
        return core::ptr::null_mut();
    }

    lo = tpm_read_index(TPM_ADDR, tpm_atmel_addr::TPM_ATMEL_BASE_ADDR_LO as i32);
    hi = tpm_read_index(TPM_ADDR, tpm_atmel_addr::TPM_ATMEL_BASE_ADDR_HI as i32);

    *base = ((hi << 8) | lo) as c_ulong;
    *region_size = 2;

    ioport_map(*base, *region_size as c_ulong)
}

/* write status bits */
const ATML_STATUS_ABORT: u8 = 0x01;
const ATML_STATUS_LASTBYTE: u8 = 0x04;
/* read status bits */
const ATML_STATUS_BUSY: u8 = 0x01;
const ATML_STATUS_DATA_AVAIL: u8 = 0x02;
const ATML_STATUS_REWRITE: u8 = 0x04;
const ATML_STATUS_READY: u8 = 0x08;

unsafe fn tpm_atml_recv(chip: *mut tpm_chip, mut buf: *mut u8, count: usize) -> i32 {
    let priv_: *mut tpm_atmel_priv = dev_get_drvdata((*chip).dev as *mut c_void);
    let hdr = buf;
    let mut status: u8;
    let mut size: u32;
    let mut i: usize;

    /* start reading header */
    if count < 6 {
        return -EIO;
    }

    i = 0;
    while i < 6 {
        status = ioread8((*priv_).iobase.add(1));
        if (status & ATML_STATUS_DATA_AVAIL) == 0 {
            dev_err(&(*chip).dev, "error reading header\0".as_ptr());
            return -EIO;
        }
        *buf = ioread8((*priv_).iobase);
        buf = buf.add(1);
        i += 1;
    }

    /* size of the data received */
    size = u32::from_be_bytes([*hdr.add(2), *hdr.add(3), *hdr.add(4), *hdr.add(5)]);

    if count < size as usize {
        dev_err(&(*chip).dev, "Recv size(%d) less than available space\0".as_ptr());
        while i < size as usize {
            status = ioread8((*priv_).iobase.add(1));
            if (status & ATML_STATUS_DATA_AVAIL) == 0 {
                dev_err(&(*chip).dev, "error reading data\0".as_ptr());
                return -EIO;
            }
            i += 1;
        }
        return -EIO;
    }

    /* read all the data available */
    while i < size as usize {
        status = ioread8((*priv_).iobase.add(1));
        if (status & ATML_STATUS_DATA_AVAIL) == 0 {
            dev_err(&(*chip).dev, "error reading data\0".as_ptr());
            return -EIO;
        }
        *buf = ioread8((*priv_).iobase);
        buf = buf.add(1);
        i += 1;
    }

    /* make sure data available is gone */
    status = ioread8((*priv_).iobase.add(1));
    if (status & ATML_STATUS_DATA_AVAIL) != 0 {
        dev_err(&(*chip).dev, "data available is stuck\0".as_ptr());
        return -EIO;
    }

    size as i32
}

unsafe fn tpm_atml_send(chip: *mut tpm_chip, buf: *mut u8, _bufsiz: usize, count: usize) -> i32 {
    let priv_: *mut tpm_atmel_priv = dev_get_drvdata((*chip).dev as *mut c_void);
    dev_dbg(&(*chip).dev, "tpm_atml_send:\n\0".as_ptr());
    for i in 0..count {
        dev_dbg(&(*chip).dev, "\0".as_ptr());
        iowrite8(*buf.add(i), (*priv_).iobase);
    }
    0
}

unsafe fn tpm_atml_cancel(chip: *mut tpm_chip) {
    let priv_: *mut tpm_atmel_priv = dev_get_drvdata((*chip).dev as *mut c_void);
    iowrite8(ATML_STATUS_ABORT, (*priv_).iobase.add(1));
}

unsafe fn tpm_atml_status(chip: *mut tpm_chip) -> u8 {
    let priv_: *mut tpm_atmel_priv = dev_get_drvdata((*chip).dev as *mut c_void);
    ioread8((*priv_).iobase.add(1))
}

unsafe fn tpm_atml_req_canceled(_chip: *mut tpm_chip, status: u8) -> bool {
    status == ATML_STATUS_READY
}

#[repr(C)]
struct tpm_class_ops {
    recv: unsafe fn(*mut tpm_chip, *mut u8, usize) -> i32,
    send: unsafe fn(*mut tpm_chip, *mut u8, usize, usize) -> i32,
    cancel: unsafe fn(*mut tpm_chip),
    status: unsafe fn(*mut tpm_chip) -> u8,
    req_complete_mask: u8,
    req_complete_val: u8,
    req_canceled: unsafe fn(*mut tpm_chip, u8) -> bool,
}

static mut tpm_atmel: tpm_class_ops = tpm_class_ops {
    recv: tpm_atml_recv,
    send: tpm_atml_send,
    cancel: tpm_atml_cancel,
    status: tpm_atml_status,
    req_complete_mask: ATML_STATUS_BUSY | ATML_STATUS_DATA_AVAIL,
    req_complete_val: ATML_STATUS_DATA_AVAIL,
    req_canceled: tpm_atml_req_canceled,
};

#[repr(C)]
struct platform_driver {
    driver: driver,
}

#[repr(C)]
struct driver {
    name: *const u8,
    pm: *const c_void,
}

static mut tpm_atml_pm: c_void = c_void {};

static mut atml_drv: platform_driver = platform_driver {
    driver: driver {
        name: "tpm_atmel\0".as_ptr(),
        pm: core::ptr::addr_of!(tpm_atml_pm),
    },
};

static mut pdev: *mut platform_device = core::ptr::null_mut();

unsafe fn atml_plat_remove() {
    let chip: *mut tpm_chip = dev_get_drvdata((*pdev).dev as *mut c_void);
    let priv_: *mut tpm_atmel_priv = dev_get_drvdata((*chip).dev as *mut c_void);
    tpm_chip_unregister(chip);
    if (*priv_).have_region != 0 {
        atmel_release_region((*priv_).base, (*priv_).region_size as c_ulong);
    }
    platform_device_unregister(pdev);
}

unsafe fn init_atmel() -> i32 {
    let mut rc = 0;
    let mut iobase: *mut c_void;
    let mut have_region: i32;
    let mut region_size: i32;
    let mut base: c_ulong = 0;
    let mut chip: *mut tpm_chip;
    let mut priv_: *mut tpm_atmel_priv;

    rc = platform_driver_register(&mut atml_drv);
    if rc != 0 { return rc; }
    iobase = atmel_get_base_addr(&mut base, &mut region_size);
    if iobase.is_null() { rc = -ENODEV; goto err_unreg_drv; }
    have_region = if atmel_request_region(base, region_size as c_ulong, "tpm_atmel0\0".as_ptr()).is_null() { 0 } else { 1 };
    pdev = platform_device_register_simple("tpm_atmel\0".as_ptr(), -1, core::ptr::null_mut(), 0);
    if IS_ERR(pdev) { rc = PTR_ERR(pdev); goto err_rel_reg; }
    priv_ = devm_kzalloc((*pdev).dev, core::mem::size_of::<tpm_atmel_priv>(), GFP_KERNEL);
    if priv_.is_null() { rc = -ENOMEM; goto err_unreg_dev; }
    (*priv_).iobase = iobase; (*priv_).base = base; (*priv_).have_region = have_region; (*priv_).region_size = region_size;
    chip = tpmm_chip_alloc(&mut (*pdev).dev, &tpm_atmel);
    if IS_ERR(chip) { rc = PTR_ERR(chip); goto err_unreg_dev; }
    dev_set_drvdata(&mut (*chip).dev, priv_ as *mut c_void);
    rc = tpm_chip_register(chip);
    if rc != 0 { goto err_unreg_dev; }
    return 0;
err_unreg_dev:
    platform_device_unregister(pdev);
err_rel_reg:
    if have_region != 0 { atmel_release_region(base, region_size as c_ulong); }
err_unreg_drv:
    platform_driver_unregister(&mut atml_drv);
    rc
}

unsafe fn cleanup_atmel() {
    platform_driver_unregister(&mut atml_drv);
    atml_plat_remove();
}

// module_init(init_atmel);
// module_exit(cleanup_atmel);
// MODULE_AUTHOR("Leendert van Doorn <leendert@watson.ibm.com>");
// MODULE_DESCRIPTION("TPM Driver");
// MODULE_VERSION("2.0");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
