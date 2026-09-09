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

#[repr(u32)]
enum TpmNscAddr { TPM_NSC_IRQ = 0x07, TPM_NSC_BASE0_HI = 0x60, TPM_NSC_BASE0_LO = 0x61, TPM_NSC_BASE1_HI = 0x62, TPM_NSC_BASE1_LO = 0x63 }
#[repr(u32)]
enum TpmNscIndex { NSC_LDN_INDEX = 0x07, NSC_SID_INDEX = 0x20, NSC_LDC_INDEX = 0x30, NSC_DIO_INDEX = 0x60, NSC_CIO_INDEX = 0x62, NSC_IRQ_INDEX = 0x70, NSC_ITS_INDEX = 0x71 }
#[repr(u32)]
enum TpmNscStatusLoc { NSC_STATUS = 0x01, NSC_COMMAND = 0x01, NSC_DATA = 0x00 }
#[repr(u32)]
enum TpmNscStatus { NSC_STATUS_OBF = 0x01, NSC_STATUS_IBF = 0x02, NSC_STATUS_F0 = 0x04, NSC_STATUS_A2 = 0x08, NSC_STATUS_RDY = 0x10, NSC_STATUS_IBR = 0x20 }
#[repr(u32)]
enum TpmNscCmdMode { NSC_COMMAND_NORMAL = 0x01, NSC_COMMAND_EOC = 0x03, NSC_COMMAND_CANCEL = 0x22 }

#[repr(C)]
struct TpmNscPriv { base: c_ulong }

unsafe fn wait_for_stat(chip: *mut tpm_chip, mask: u8, val: u8, data: *mut u8) -> c_int {
    let priv_ = dev_get_drvdata((*chip).dev);
    let mut stop: c_ulong;
    *data = inb((*priv_).base + NSC_STATUS as c_ulong);
    if (*data & mask) == val { return 0; }
    stop = jiffies + 10 * HZ;
    loop {
        msleep(TPM_TIMEOUT);
        *data = inb((*priv_).base + 1);
        if (*data & mask) == val { return 0; }
        if !time_before(jiffies, stop) { break; }
    }
    -EBUSY
}

unsafe fn nsc_wait_for_ready(chip: *mut tpm_chip) -> c_int {
    let priv_ = dev_get_drvdata((*chip).dev);
    let mut status = inb((*priv_).base + NSC_STATUS as c_ulong);
    if status & NSC_STATUS_OBF as u8 != 0 { status = inb((*priv_).base + NSC_DATA as c_ulong); }
    if status & NSC_STATUS_RDY as u8 != 0 { return 0; }
    let stop = jiffies + 100;
    loop {
        msleep(TPM_TIMEOUT);
        status = inb((*priv_).base + NSC_STATUS as c_ulong);
        if status & NSC_STATUS_OBF as u8 != 0 { status = inb((*priv_).base + NSC_DATA as c_ulong); }
        if status & NSC_STATUS_RDY as u8 != 0 { return 0; }
        if !time_before(jiffies, stop) { break; }
    }
    dev_info(&(*chip).dev, "wait for ready failed\n");
    -EBUSY
}

unsafe fn tpm_nsc_recv(chip: *mut tpm_chip, buf: *mut u8, count: usize) -> c_int {
    let priv_ = dev_get_drvdata((*chip).dev);
    let mut data: u8 = 0;
    if count < 6 { return -EIO; }
    if wait_for_stat(chip, NSC_STATUS_F0 as u8, NSC_STATUS_F0 as u8, &mut data) < 0 { dev_err(&(*chip).dev, "F0 timeout\n"); return -EIO; }
    data = inb((*priv_).base + NSC_DATA as c_ulong);
    if data != NSC_COMMAND_NORMAL as u8 { dev_err(&(*chip).dev, "not in normal mode (0x%x)\n", data); return -EIO; }
    let mut p = buf;
    while p < buf.add(count) {
        if wait_for_stat(chip, NSC_STATUS_OBF as u8, NSC_STATUS_OBF as u8, &mut data) < 0 { dev_err(&(*chip).dev, "OBF timeout (while reading data)\n"); return -EIO; }
        if data & NSC_STATUS_F0 as u8 != 0 { break; }
        *p = inb((*priv_).base + NSC_DATA as c_ulong); p = p.add(1);
    }
    if data & NSC_STATUS_F0 as u8 == 0 && wait_for_stat(chip, NSC_STATUS_F0 as u8, NSC_STATUS_F0 as u8, &mut data) < 0 { dev_err(&(*chip).dev, "F0 not set\n"); return -EIO; }
    data = inb((*priv_).base + NSC_DATA as c_ulong);
    if data != NSC_COMMAND_EOC as u8 { dev_err(&(*chip).dev, "expected end of command(0x%x)\n", data); return -EIO; }
    let size = u32::from_be(*(buf.add(2) as *const u32));
    if count < size as usize { return -EIO; }
    size as c_int
}

unsafe fn tpm_nsc_send(chip: *mut tpm_chip, buf: *mut u8, _bufsiz: usize, count: usize) -> c_int {
    let priv_ = dev_get_drvdata((*chip).dev); let mut data = 0u8;
    outb(NSC_COMMAND_CANCEL as u8, (*priv_).base + NSC_COMMAND as c_ulong);
    if nsc_wait_for_ready(chip) != 0 { return -EIO; }
    if wait_for_stat(chip, NSC_STATUS_IBF as u8, 0, &mut data) < 0 { dev_err(&(*chip).dev, "IBF timeout\n"); return -EIO; }
    outb(NSC_COMMAND_NORMAL as u8, (*priv_).base + NSC_COMMAND as c_ulong);
    if wait_for_stat(chip, NSC_STATUS_IBR as u8, NSC_STATUS_IBR as u8, &mut data) < 0 { dev_err(&(*chip).dev, "IBR timeout\n"); return -EIO; }
    for i in 0..count { if wait_for_stat(chip, NSC_STATUS_IBF as u8, 0, &mut data) < 0 { dev_err(&(*chip).dev, "IBF timeout (while writing data)\n"); return -EIO; } outb(*buf.add(i), (*priv_).base + NSC_DATA as c_ulong); }
    if wait_for_stat(chip, NSC_STATUS_IBF as u8, 0, &mut data) < 0 { dev_err(&(*chip).dev, "IBF timeout\n"); return -EIO; }
    outb(NSC_COMMAND_EOC as u8, (*priv_).base + NSC_COMMAND as c_ulong); 0
}

unsafe fn tpm_nsc_cancel(chip: *mut tpm_chip) { let priv_ = dev_get_drvdata((*chip).dev); outb(NSC_COMMAND_CANCEL as u8, (*priv_).base + NSC_COMMAND as c_ulong); }
unsafe fn tpm_nsc_status(chip: *mut tpm_chip) -> u8 { let priv_ = dev_get_drvdata((*chip).dev); inb((*priv_).base + NSC_STATUS as c_ulong) }
unsafe fn tpm_nsc_req_canceled(_chip: *mut tpm_chip, status: u8) -> bool { status == NSC_STATUS_RDY as u8 }

#[repr(C)]
struct TpmNsc { recv: unsafe fn(*mut tpm_chip,*mut u8,usize)->c_int, send: unsafe fn(*mut tpm_chip,*mut u8,usize,usize)->c_int, cancel: unsafe fn(*mut tpm_chip), status: unsafe fn(*mut tpm_chip)->u8, req_complete_mask: u8, req_complete_val: u8, req_canceled: unsafe fn(*mut tpm_chip,u8)->bool }
static TPM_NSC: TpmNsc = TpmNsc { recv: tpm_nsc_recv, send: tpm_nsc_send, cancel: tpm_nsc_cancel, status: tpm_nsc_status, req_complete_mask: NSC_STATUS_OBF as u8, req_complete_val: NSC_STATUS_OBF as u8, req_canceled: tpm_nsc_req_canceled };

static mut PDEV: *mut platform_device = core::ptr::null_mut();

extern "C" {
    static mut nsc_drv: platform_driver;
    static TPM_ADDR: c_int;
    static TPM_SUPERIO_ADDR: c_int;
    static HZ: c_ulong;
    static mut jiffies: c_ulong;
}

#[inline] unsafe fn tpm_read_index(base: c_int, index: c_int) -> c_int { outb(index as u8, base as c_ulong); (inb((base + 1) as c_ulong) & 0xff) as c_int }
#[inline] unsafe fn tpm_write_index(base: c_int, index: c_int, value: c_int) { outb(index as u8, base as c_ulong); outb((value & 0xff) as u8, (base + 1) as c_ulong); }

unsafe fn tpm_nsc_remove(dev: *mut device) {
    let chip = dev_get_drvdata(dev);
    let priv_ = dev_get_drvdata((*chip).dev);
    tpm_chip_unregister(chip);
    release_region((*priv_).base, 2);
}

unsafe fn init_nsc() -> c_int {
    let mut rc = 0; let mut nsc_addr_base = TPM_ADDR; let mut lo; let mut hi;
    if tpm_read_index(TPM_ADDR, NSC_SID_INDEX as c_int) != 0xef {
        nsc_addr_base = (tpm_read_index(TPM_SUPERIO_ADDR, 0x2c) << 8) | (tpm_read_index(TPM_SUPERIO_ADDR, 0x2b) & 0xfe);
        if tpm_read_index(nsc_addr_base, NSC_SID_INDEX as c_int) != 0xf6 { return -ENODEV; }
    }
    rc = platform_driver_register(&nsc_drv); if rc != 0 { return rc; }
    hi = tpm_read_index(nsc_addr_base, TPM_NSC_BASE0_HI as c_int); lo = tpm_read_index(nsc_addr_base, TPM_NSC_BASE0_LO as c_int);
    let base = ((hi << 8) | lo) as c_ulong;
    tpm_write_index(nsc_addr_base, NSC_LDC_INDEX as c_int, 1);
    PDEV = platform_device_alloc("tpm_nscl0", -1); if PDEV.is_null() { platform_driver_unregister(&nsc_drv); return -ENOMEM; }
    (*PDEV).num_resources = 0; (*PDEV).dev.driver = &nsc_drv.driver; (*PDEV).dev.release = Some(tpm_nsc_remove);
    rc = platform_device_add(PDEV); if rc < 0 { platform_device_put(PDEV); platform_driver_unregister(&nsc_drv); return rc; }
    let priv_ = devm_kzalloc(&mut (*PDEV).dev, core::mem::size_of::<TpmNscPriv>(), GFP_KERNEL); if priv_.is_null() { platform_device_del(PDEV); platform_device_put(PDEV); platform_driver_unregister(&nsc_drv); return -ENOMEM; }
    (*priv_).base = base;
    if request_region(base, 2, "tpm_nsc0").is_null() { platform_device_del(PDEV); platform_device_put(PDEV); platform_driver_unregister(&nsc_drv); return -EBUSY; }
    let chip = tpmm_chip_alloc(&mut (*PDEV).dev, &TPM_NSC); if IS_ERR(chip) { release_region(base, 2); platform_device_del(PDEV); platform_device_put(PDEV); platform_driver_unregister(&nsc_drv); return -ENODEV; }
    dev_set_drvdata(&mut (*chip).dev, priv_); rc = tpm_chip_register(chip); if rc != 0 { release_region(base, 2); platform_device_del(PDEV); platform_device_put(PDEV); platform_driver_unregister(&nsc_drv); return rc; }
    dev_dbg(&(*PDEV).dev, "NSC TPM detected\n");
    dev_info(&(*PDEV).dev, "NSC TPM revision %d\n", tpm_read_index(nsc_addr_base, 0x27) & 0x1f);
    0
}

unsafe fn cleanup_nsc() { if !PDEV.is_null() { tpm_nsc_remove(&mut (*PDEV).dev); platform_device_unregister(PDEV); } platform_driver_unregister(&nsc_drv); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
