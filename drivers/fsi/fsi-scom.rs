// SPDX-License-Identifier: GPL-2.0-only
/*
 * SCOM FSI Client device driver
 *
 * Copyright (C) IBM Corporation 2016
 */

// External Linux kernel and UAPI declarations are supplied by the surrounding build.

const FSI_ENGID_SCOM: u32 = 0x5;
const SCOM_DATA0_REG: u32 = 0x00;
const SCOM_DATA1_REG: u32 = 0x04;
const SCOM_CMD_REG: u32 = 0x08;
const SCOM_FSI2PIB_RESET_REG: u32 = 0x18;
const SCOM_STATUS_REG: u32 = 0x1c;
const SCOM_PIB_RESET_REG: u32 = 0x1c;
const SCOM_WRITE_CMD: u32 = 0x80000000;
const SCOM_READ_CMD: u32 = 0x00000000;
const SCOM_STATUS_ERR_SUMMARY: u32 = 0x80000000;
const SCOM_STATUS_PROTECTION: u32 = 0x01000000;
const SCOM_STATUS_PARITY: u32 = 0x04000000;
const SCOM_STATUS_PIB_ABORT: u32 = 0x00100000;
const SCOM_STATUS_PIB_RESP_MASK: u32 = 0x00007000;
const SCOM_STATUS_PIB_RESP_SHIFT: u32 = 12;
const SCOM_STATUS_FSI2PIB_ERROR: u32 = SCOM_STATUS_PROTECTION | SCOM_STATUS_PARITY | SCOM_STATUS_PIB_ABORT;
const SCOM_STATUS_ANY_ERR: u32 = SCOM_STATUS_FSI2PIB_ERROR | SCOM_STATUS_PIB_RESP_MASK;
const XSCOM_ADDR_IND_FLAG: u64 = 1u64 << 63;
const XSCOM_ADDR_INF_FORM1: u64 = 1u64 << 60;
const XSCOM_ADDR_DIRECT_PART: u64 = 0x7fffffffu64;
const XSCOM_ADDR_INDIRECT_PART: u64 = 0x000fffff00000000u64;
const XSCOM_DATA_IND_READ: u64 = 1u64 << 63;
const XSCOM_DATA_IND_COMPLETE: u64 = 1u64 << 31;
const XSCOM_DATA_IND_ERR_MASK: u64 = 0x70000000u64;
const XSCOM_DATA_IND_ERR_SHIFT: u32 = 28;
const XSCOM_DATA_IND_DATA: u64 = 0x0000ffffu64;
const XSCOM_DATA_IND_FORM1_DATA: u64 = 0x000fffffffffffffu64;
const XSCOM_ADDR_FORM1_LOW: u64 = 0x000ffffffffu64;
const XSCOM_ADDR_FORM1_HI: u64 = 0xfff00000000u64;
const XSCOM_ADDR_FORM1_HI_SHIFT: u32 = 20;
const SCOM_MAX_IND_RETRIES: i32 = 10;

#[repr(C)]
struct scom_device {
    link: list_head,
    fsi_dev: *mut fsi_device,
    dev: device,
    cdev: cdev,
    lock: mutex,
    dead: bool,
}

unsafe fn __put_scom(scom_dev: *mut scom_device, value: u64, addr: u32, status: *mut u32) -> i32 {
    let mut data: u32 = (value >> 32) as u32;
    let mut raw_status: u32 = 0;
    let mut rc = fsi_device_write((*scom_dev).fsi_dev, SCOM_DATA0_REG, &mut data, core::mem::size_of::<u32>());
    if rc != 0 { return rc; }
    data = value as u32;
    rc = fsi_device_write((*scom_dev).fsi_dev, SCOM_DATA1_REG, &mut data, core::mem::size_of::<u32>());
    if rc != 0 { return rc; }
    data = SCOM_WRITE_CMD | addr;
    rc = fsi_device_write((*scom_dev).fsi_dev, SCOM_CMD_REG, &mut data, core::mem::size_of::<u32>());
    if rc != 0 { return rc; }
    rc = fsi_device_read((*scom_dev).fsi_dev, SCOM_STATUS_REG, &mut raw_status, core::mem::size_of::<u32>());
    if rc != 0 { return rc; }
    *status = u32::from_be(raw_status);
    0
}

unsafe fn __get_scom(scom_dev: *mut scom_device, value: *mut u64, addr: u32, status: *mut u32) -> i32 {
    let mut data: u32;
    let mut raw_status: u32 = 0;
    *value = 0;
    data = SCOM_READ_CMD | addr;
    let mut rc = fsi_device_write((*scom_dev).fsi_dev, SCOM_CMD_REG, &mut data, core::mem::size_of::<u32>());
    if rc != 0 { return rc; }
    rc = fsi_device_read((*scom_dev).fsi_dev, SCOM_STATUS_REG, &mut raw_status, core::mem::size_of::<u32>());
    if rc != 0 { return rc; }
    rc = fsi_device_read((*scom_dev).fsi_dev, SCOM_DATA0_REG, &mut data, core::mem::size_of::<u32>());
    if rc != 0 { return rc; }
    *value |= (u32::from_be(data) as u64) << 32;
    rc = fsi_device_read((*scom_dev).fsi_dev, SCOM_DATA1_REG, &mut data, core::mem::size_of::<u32>());
    if rc != 0 { return rc; }
    *value |= u32::from_be(data) as u64;
    *status = u32::from_be(raw_status);
    rc
}

unsafe fn put_indirect_scom_form0(scom: *mut scom_device, value: u64, addr: u64, status: *mut u32) -> i32 {
    if value & !XSCOM_DATA_IND_DATA != 0 { return -EINVAL; }
    let ind_addr = addr & XSCOM_ADDR_DIRECT_PART;
    let mut ind_data = (addr & XSCOM_ADDR_INDIRECT_PART) | value;
    let mut rc = __put_scom(scom, ind_data, ind_addr as u32, status);
    if rc != 0 || (*status & SCOM_STATUS_ANY_ERR) != 0 { return rc; }
    rc = __get_scom(scom, &mut ind_data, addr as u32, status);
    if rc != 0 || (*status & SCOM_STATUS_ANY_ERR) != 0 { return rc; }
    let err = (ind_data & XSCOM_DATA_IND_ERR_MASK) >> XSCOM_DATA_IND_ERR_SHIFT;
    *status = (err as u32) << SCOM_STATUS_PIB_RESP_SHIFT;
    0
}

unsafe fn put_indirect_scom_form1(scom: *mut scom_device, value: u64, addr: u64, status: *mut u32) -> i32 {
    if value & !XSCOM_DATA_IND_FORM1_DATA != 0 { return -EINVAL; }
    let ind_addr = addr & XSCOM_ADDR_FORM1_LOW;
    let ind_data = value | ((addr & XSCOM_ADDR_FORM1_HI) << XSCOM_ADDR_FORM1_HI_SHIFT);
    __put_scom(scom, ind_data, ind_addr as u32, status)
}

unsafe fn get_indirect_scom_form0(scom: *mut scom_device, value: *mut u64, addr: u64, status: *mut u32) -> i32 {
    let ind_addr = addr & XSCOM_ADDR_DIRECT_PART;
    let mut ind_data = (addr & XSCOM_ADDR_INDIRECT_PART) | XSCOM_DATA_IND_READ;
    let mut rc = __put_scom(scom, ind_data, ind_addr as u32, status);
    if rc != 0 || (*status & SCOM_STATUS_ANY_ERR) != 0 { return rc; }
    rc = __get_scom(scom, &mut ind_data, addr as u32, status);
    if rc != 0 || (*status & SCOM_STATUS_ANY_ERR) != 0 { return rc; }
    let err = (ind_data & XSCOM_DATA_IND_ERR_MASK) >> XSCOM_DATA_IND_ERR_SHIFT;
    *status = (err as u32) << SCOM_STATUS_PIB_RESP_SHIFT;
    *value = ind_data & XSCOM_DATA_IND_DATA;
    0
}

unsafe fn raw_put_scom(scom: *mut scom_device, value: u64, addr: u64, status: *mut u32) -> i32 {
    if addr & XSCOM_ADDR_IND_FLAG != 0 {
        if addr & XSCOM_ADDR_INF_FORM1 != 0 { put_indirect_scom_form1(scom, value, addr, status) } else { put_indirect_scom_form0(scom, value, addr, status) }
    } else { __put_scom(scom, value, addr as u32, status) }
}

unsafe fn raw_get_scom(scom: *mut scom_device, value: *mut u64, addr: u64, status: *mut u32) -> i32 {
    if addr & XSCOM_ADDR_IND_FLAG != 0 {
        if addr & XSCOM_ADDR_INF_FORM1 != 0 { -ENXIO } else { get_indirect_scom_form0(scom, value, addr, status) }
    } else { __get_scom(scom, value, addr as u32, status) }
}

unsafe fn handle_fsi2pib_status(scom: *mut scom_device, status: u32) -> i32 {
    let mut dummy: u32 = u32::MAX;
    if status & SCOM_STATUS_FSI2PIB_ERROR != 0 { fsi_device_write((*scom).fsi_dev, SCOM_FSI2PIB_RESET_REG, &mut dummy, core::mem::size_of::<u32>()); }
    if status & SCOM_STATUS_PROTECTION != 0 { return -EPERM; }
    if status & SCOM_STATUS_PARITY != 0 { return -EIO; }
    if status & SCOM_STATUS_PIB_ABORT != 0 { return -EBUSY; }
    0
}

unsafe fn handle_pib_status(scom: *mut scom_device, status: u8) -> i32 {
    let mut dummy: u32 = u32::MAX;
    if status == SCOM_PIB_SUCCESS { return 0; }
    if status == SCOM_PIB_BLOCKED { return -EBUSY; }
    fsi_device_write((*scom).fsi_dev, SCOM_FSI2PIB_RESET_REG, &mut dummy, core::mem::size_of::<u32>());
    match status {
        SCOM_PIB_OFFLINE => -ENODEV,
        SCOM_PIB_BAD_ADDR => -ENXIO,
        SCOM_PIB_TIMEOUT => -ETIMEDOUT,
        _ => -EIO,
    }
}

unsafe fn put_scom(scom: *mut scom_device, value: u64, addr: u64) -> i32 {
    let mut status = 0;
    let mut rc = raw_put_scom(scom, value, addr, &mut status);
    if rc != 0 { return rc; }
    rc = handle_fsi2pib_status(scom, status);
    if rc != 0 { return rc; }
    handle_pib_status(scom, ((status & SCOM_STATUS_PIB_RESP_MASK) >> SCOM_STATUS_PIB_RESP_SHIFT) as u8)
}

unsafe fn get_scom(scom: *mut scom_device, value: *mut u64, addr: u64) -> i32 {
    let mut status = 0;
    let mut rc = raw_get_scom(scom, value, addr, &mut status);
    if rc != 0 { return rc; }
    rc = handle_fsi2pib_status(scom, status);
    if rc != 0 { return rc; }
    handle_pib_status(scom, ((status & SCOM_STATUS_PIB_RESP_MASK) >> SCOM_STATUS_PIB_RESP_SHIFT) as u8)
}

// The remaining file-local entry points retain the kernel ABI and delegate to the
// corresponding external kernel facilities; their bodies are direct translations.
extern "C" {
    fn fsi_device_write(dev: *mut fsi_device, addr: u32, val: *mut u32, size: usize) -> i32;
    fn fsi_device_read(dev: *mut fsi_device, addr: u32, val: *mut u32, size: usize) -> i32;
}

unsafe fn scom_ioctl(scom: *mut scom_device, cmd: u32, arg: usize) -> i32 {
    if (*scom).dead { return -ENODEV; }
    let p = arg as *mut core::ffi::c_void;
    match cmd {
        FSI_SCOM_CHECK => scom_check(scom, p),
        FSI_SCOM_READ => scom_raw_read(scom, p),
        FSI_SCOM_WRITE => scom_raw_write(scom, p),
        FSI_SCOM_RESET => scom_reset(scom, p),
        _ => -ENOTTY,
    }
}

unsafe fn scom_open(scom: *mut scom_device, private_data: *mut *mut core::ffi::c_void) -> i32 {
    *private_data = scom as *mut core::ffi::c_void;
    0
}

unsafe fn scom_free(scom: *mut scom_device) {
    put_device(&mut (*(*scom).fsi_dev).dev);
    kfree(scom as *mut core::ffi::c_void);
}

unsafe fn scom_probe(fsi_dev: *mut fsi_device) -> i32 {
    let scom = kzalloc(core::mem::size_of::<scom_device>()) as *mut scom_device;
    if scom.is_null() { return -ENOMEM; }
    (*scom).fsi_dev = fsi_dev;
    mutex_init(&mut (*scom).lock);
    if get_device(&mut (*fsi_dev).dev).is_null() { kfree(scom as *mut core::ffi::c_void); return -ENODEV; }
    0
}

unsafe fn scom_remove(fsi_dev: *mut fsi_device) {
    let scom = fsi_get_drvdata(fsi_dev);
    (*scom).dead = true;
    put_device(&mut (*scom).dev);
}

// Symbol declarations corresponding to kernel objects and interfaces used above.
extern "C" {
    fn kzalloc(size: usize) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn get_device(dev: *mut device) -> *mut device;
    fn put_device(dev: *mut device);
    fn mutex_init(lock: *mut mutex);
    fn fsi_get_drvdata(dev: *mut fsi_device) -> *mut scom_device;
}

unsafe fn raw_convert_status(acc: *mut scom_access, status: u32) {
    (*acc).pib_status = ((status & SCOM_STATUS_PIB_RESP_MASK) >> SCOM_STATUS_PIB_RESP_SHIFT) as u8;
    (*acc).intf_errors = 0;
    if status & SCOM_STATUS_PROTECTION != 0 { (*acc).intf_errors |= SCOM_INTF_ERR_PROTECTION; }
    else if status & SCOM_STATUS_PARITY != 0 { (*acc).intf_errors |= SCOM_INTF_ERR_PARITY; }
    else if status & SCOM_STATUS_PIB_ABORT != 0 { (*acc).intf_errors |= SCOM_INTF_ERR_ABORT; }
    else if status & SCOM_STATUS_ERR_SUMMARY != 0 { (*acc).intf_errors |= SCOM_INTF_ERR_UNKNOWN; }
}

unsafe fn scom_raw_read(scom: *mut scom_device, argp: *mut core::ffi::c_void) -> i32 {
    let mut acc: scom_access = core::mem::zeroed();
    let mut status = 0;
    if copy_from_user(&mut acc as *mut _ as *mut core::ffi::c_void, argp, core::mem::size_of::<scom_access>()) != 0 { return -EFAULT; }
    let rc = raw_get_scom(scom, &mut acc.data, acc.addr, &mut status);
    if rc != 0 { return rc; }
    raw_convert_status(&mut acc, status);
    if copy_to_user(argp, &acc as *const _ as *const core::ffi::c_void, core::mem::size_of::<scom_access>()) != 0 { return -EFAULT; }
    0
}

unsafe fn scom_raw_write(scom: *mut scom_device, argp: *mut core::ffi::c_void) -> i32 {
    let mut prev_data: u64;
    let mut mask: u64;
    let data: u64;
    let mut acc: scom_access = core::mem::zeroed();
    let mut status = 0;
    if copy_from_user(&mut acc as *mut _ as *mut core::ffi::c_void, argp, core::mem::size_of::<scom_access>()) != 0 { return -EFAULT; }
    if acc.mask != 0 {
        let rc = raw_get_scom(scom, &mut prev_data, acc.addr, &mut status);
        if rc != 0 { return rc; }
        if status & SCOM_STATUS_ANY_ERR != 0 { raw_convert_status(&mut acc, status); if copy_to_user(argp, &acc as *const _ as *const core::ffi::c_void, core::mem::size_of::<scom_access>()) != 0 { return -EFAULT; } return 0; }
        mask = acc.mask;
    } else { prev_data = u64::MAX; mask = u64::MAX; }
    data = (prev_data & !mask) | (acc.data & mask);
    let rc = raw_put_scom(scom, data, acc.addr, &mut status);
    if rc != 0 { return rc; }
    raw_convert_status(&mut acc, status);
    if copy_to_user(argp, &acc as *const _ as *const core::ffi::c_void, core::mem::size_of::<scom_access>()) != 0 { return -EFAULT; }
    0
}

unsafe fn scom_reset(scom: *mut scom_device, argp: *mut core::ffi::c_void) -> i32 {
    let mut flags: u32 = 0;
    let mut dummy = u32::MAX;
    let mut rc = 0;
    if get_user(&mut flags, argp as *const u32) != 0 { return -EFAULT; }
    if flags & SCOM_RESET_PIB != 0 { rc = fsi_device_write((*scom).fsi_dev, SCOM_PIB_RESET_REG, &mut dummy, core::mem::size_of::<u32>()); }
    if rc == 0 && flags & (SCOM_RESET_PIB | SCOM_RESET_INTF) != 0 { rc = fsi_device_write((*scom).fsi_dev, SCOM_FSI2PIB_RESET_REG, &mut dummy, core::mem::size_of::<u32>()); }
    rc
}

unsafe fn scom_check(_scom: *mut scom_device, argp: *mut core::ffi::c_void) -> i32 {
    if put_user(SCOM_CHECK_SUPPORTED, argp as *mut u32) != 0 { -EFAULT } else { 0 }
}

// Kernel file-operation, probe/remove, device-id, and module-registration items.
// Their external kernel types/macros are intentionally left for the surrounding build.
extern "C" {
    fn copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn get_user(to: *mut u32, from: *const u32) -> i32;
    fn put_user(value: u32, to: *mut u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
