// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) IBM Corporation 2023 */

// Dependencies supplied by the Linux kernel and the associated headers are
// intentionally left external to this translation unit.

const I2CR_INITIAL_PARITY: bool = true;
const I2CR_STATUS_CMD: u32 = 0x60002;
const I2CR_STATUS_ERR: u64 = 1u64 << 61;
const I2CR_ERROR_CMD: u32 = 0x60004;
const I2CR_LOG_CMD: u32 = 0x60008;

static I2CR_CFAM: [u8; 48] = [
    0xc0, 0x02, 0x0d, 0xa6, 0x80, 0x01, 0x10, 0x02,
    0x80, 0x01, 0x10, 0x02, 0x80, 0x01, 0x10, 0x02,
    0x80, 0x01, 0x80, 0x52, 0x80, 0x01, 0x10, 0x02,
    0x80, 0x01, 0x10, 0x02, 0x80, 0x01, 0x10, 0x02,
    0x80, 0x01, 0x10, 0x02, 0x80, 0x01, 0x22, 0x2d,
    0x00, 0x00, 0x00, 0x00, 0xde, 0xad, 0xc0, 0xde,
];

#[inline]
fn i2cr_check_parity32(mut v: u32, mut parity: bool) -> bool {
    for i in 0..32 {
        if v & (1u32 << i) != 0 { parity = !parity; }
    }
    parity
}

#[inline]
fn i2cr_check_parity64(v: u64) -> bool {
    let mut parity = I2CR_INITIAL_PARITY;
    for i in 0..64 {
        if v & (1u64 << i) != 0 { parity = !parity; }
    }
    parity
}

#[inline]
fn i2cr_get_command(mut address: u32, parity: bool) -> u32 {
    address <<= 1;
    if i2cr_check_parity32(address, parity) { address |= 1; }
    address
}

unsafe fn i2cr_transfer(client: *mut i2c_client, command: u32, data: *mut u64) -> i32 {
    let mut msgs: [i2c_msg; 2] = core::mem::zeroed();
    msgs[0].addr = (*client).addr;
    msgs[0].flags = 0;
    msgs[0].len = core::mem::size_of::<u32>() as u16;
    msgs[0].buf = &command as *const u32 as *mut u8;
    msgs[1].addr = (*client).addr;
    msgs[1].flags = I2C_M_RD;
    msgs[1].len = core::mem::size_of::<u64>() as u16;
    msgs[1].buf = data as *mut u8;
    let ret = i2c_transfer((*client).adapter, msgs.as_mut_ptr(), 2);
    if ret == 2 { return 0; }
    trace_i2cr_i2c_error(client, command, ret);
    if ret < 0 { return ret; }
    -EIO
}

unsafe fn i2cr_check_status(client: *mut i2c_client) -> i32 {
    let mut status = 0u64;
    let ret = i2cr_transfer(client, I2CR_STATUS_CMD, &mut status);
    if ret != 0 { return ret; }
    if status & I2CR_STATUS_ERR != 0 {
        let mut buf = [0u32; 3];
        let mut error = 0u64;
        let mut log = 0u64;
        i2cr_transfer(client, I2CR_ERROR_CMD, &mut error);
        i2cr_transfer(client, I2CR_LOG_CMD, &mut log);
        trace_i2cr_status_error(client, status, error, log);
        buf[0] = I2CR_STATUS_CMD;
        i2c_master_send(client, buf.as_ptr() as *const i8, core::mem::size_of_val(&buf) as i32);
        buf[0] = I2CR_ERROR_CMD;
        i2c_master_send(client, buf.as_ptr() as *const i8, core::mem::size_of_val(&buf) as i32);
        buf[0] = I2CR_LOG_CMD;
        i2c_master_send(client, buf.as_ptr() as *const i8, core::mem::size_of_val(&buf) as i32);
        dev_err(&(*client).dev, "status:%016llx error:%016llx log:%016llx\n", status, error, log);
        return -EREMOTEIO;
    }
    trace_i2cr_status(client, status);
    0
}

pub unsafe fn fsi_master_i2cr_read(i2cr: *mut fsi_master_i2cr, addr: u32, data: *mut u64) -> i32 {
    let command = i2cr_get_command(addr, I2CR_INITIAL_PARITY);
    mutex_lock(&mut (*i2cr).lock);
    let mut ret = i2cr_transfer((*i2cr).client, command, data);
    if ret == 0 { ret = i2cr_check_status((*i2cr).client); }
    if ret == 0 { trace_i2cr_read((*i2cr).client, command, data); }
    mutex_unlock(&mut (*i2cr).lock);
    ret
}

pub unsafe fn fsi_master_i2cr_write(i2cr: *mut fsi_master_i2cr, addr: u32, data: u64) -> i32 {
    let mut buf = [0u32; 3];
    buf[0] = i2cr_get_command(addr, i2cr_check_parity64(data));
    core::ptr::copy_nonoverlapping(&data as *const u64 as *const u8, buf[1..].as_mut_ptr() as *mut u8, 8);
    mutex_lock(&mut (*i2cr).lock);
    let mut ret = i2c_master_send((*i2cr).client, buf.as_ptr() as *const i8, core::mem::size_of_val(&buf) as i32);
    if ret == core::mem::size_of_val(&buf) as i32 {
        ret = i2cr_check_status((*i2cr).client);
        if ret == 0 { trace_i2cr_write((*i2cr).client, buf[0], data); }
    } else {
        trace_i2cr_i2c_error((*i2cr).client, buf[0], ret);
        if ret >= 0 { ret = -EIO; }
    }
    mutex_unlock(&mut (*i2cr).lock);
    ret
}

// The remaining driver registration and FSI callback definitions retain the
// kernel structure layout and helper calls supplied by the included headers.
// They are expressed below as direct Rust equivalents of the C definitions.
unsafe fn i2cr_read(master: *mut fsi_master, link: i32, id: u8, addr: u32, val: *mut core::ffi::c_void, size: usize) -> i32 {
    let i2cr = container_of!(master, fsi_master_i2cr, master);
    if link != 0 || id != 0 || addr & 0xffff0000 != 0 || !(size == 1 || size == 2 || size == 4) { return -EINVAL; }
    if addr < 0xc00 {
        let mut offset = addr as usize;
        if offset > I2CR_CFAM.len() - 4 { offset = (addr as usize & 3) + I2CR_CFAM.len() - 4; }
        core::ptr::copy_nonoverlapping(I2CR_CFAM.as_ptr().add(offset), val as *mut u8, size);
        return 0;
    }
    let mut data = 0u64;
    let ret = fsi_master_i2cr_read(i2cr, addr >> 2, &mut data);
    if ret != 0 { return ret; }
    for i in 0..size { *((val as *mut u8).add(i)) = *((&data as *const u64 as *const u8).add(7 - i)); }
    0
}

unsafe fn i2cr_write(master: *mut fsi_master, link: i32, id: u8, addr: u32, val: *const core::ffi::c_void, size: usize) -> i32 {
    let i2cr = container_of!(master, fsi_master_i2cr, master);
    if link != 0 || id != 0 || addr & 0xffff0000 != 0 || !(size == 1 || size == 2 || size == 4) { return -EINVAL; }
    if addr < 0xc00 { return 0; }
    let mut data = 0u64;
    for i in 0..size { *((&mut data as *mut u64 as *mut u8).add(7 - i)) = *((val as *const u8).add(i)); }
    fsi_master_i2cr_write(i2cr, addr >> 2, data)
}

unsafe fn i2cr_release(dev: *mut device) {
    let i2cr = to_fsi_master_i2cr(to_fsi_master(dev));
    of_node_put((*dev).of_node);
    kfree(i2cr);
}

unsafe fn i2cr_probe(client: *mut i2c_client) -> i32 {
    let i2cr = kzalloc_obj::<fsi_master_i2cr>();
    if i2cr.is_null() { return -ENOMEM; }
    (*i2cr).master.idx = (*(*client).adapter).nr;
    dev_set_name(&mut (*i2cr).master.dev, "i2cr%d", (*i2cr).master.idx);
    (*i2cr).master.dev.parent = &mut (*client).dev;
    (*i2cr).master.dev.of_node = of_node_get(dev_of_node(&(*client).dev));
    (*i2cr).master.dev.release = Some(i2cr_release);
    (*i2cr).master.n_links = 1;
    (*i2cr).master.read = Some(i2cr_read);
    (*i2cr).master.write = Some(i2cr_write);
    mutex_init(&mut (*i2cr).lock);
    (*i2cr).client = client;
    let ret = fsi_master_register(&mut (*i2cr).master);
    if ret != 0 { return ret; }
    i2c_set_clientdata(client, i2cr as *mut core::ffi::c_void);
    0
}

unsafe fn i2cr_remove(client: *mut i2c_client) {
    let i2cr = i2c_get_clientdata(client) as *mut fsi_master_i2cr;
    fsi_master_unregister(&mut (*i2cr).master);
}

static I2CR_IDS: [of_device_id; 2] = [
    of_device_id { compatible: "ibm,i2cr-fsi-master\0" },
    of_device_id { compatible: "\0" },
];

static mut I2CR_DRIVER: i2c_driver = i2c_driver {
    probe: Some(i2cr_probe), remove: Some(i2cr_remove),
    driver: driver { name: "fsi-master-i2cr\0", of_match_table: I2CR_IDS.as_ptr() },
};

// MODULE_DEVICE_TABLE(of, i2cr_ids)
// module_i2c_driver(i2cr_driver)
// MODULE_AUTHOR("Eddie James <eajames@linux.ibm.com>")
// MODULE_DESCRIPTION("IBM I2C Responder virtual FSI master driver")
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
