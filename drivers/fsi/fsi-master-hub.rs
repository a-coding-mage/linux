// SPDX-License-Identifier: GPL-2.0-only
/*
 * FSI hub master driver
 *
 * Copyright (C) IBM Corporation 2016
 */

// Translated from the Linux kernel C implementation.  Kernel types, constants,
// and functions referenced below are supplied by the surrounding FSI code.

const FSI_ENGID_HUB_MASTER: u32 = 0x1c;
const FSI_LINK_ENABLE_SETUP_TIME: u32 = 10; // in mS

#[repr(C)]
struct fsi_master_hub {
    master: fsi_master,
    upstream: *mut fsi_device,
    addr: u32,
    size: u32,
}

unsafe fn to_fsi_master_hub(m: *mut fsi_master) -> *mut fsi_master_hub {
    m as *mut fsi_master_hub
}

unsafe fn hub_master_read(
    master: *mut fsi_master,
    link: i32,
    id: u8,
    mut addr: u32,
    val: *mut core::ffi::c_void,
    size: usize,
) -> i32 {
    let hub = &mut *to_fsi_master_hub(master);
    if id != 0 { return -EINVAL; }
    addr = addr.wrapping_add(hub.addr).wrapping_add((link as u32).wrapping_mul(FSI_HUB_LINK_SIZE));
    fsi_slave_read((*hub.upstream).slave, addr, val, size)
}

unsafe fn hub_master_write(
    master: *mut fsi_master,
    link: i32,
    id: u8,
    mut addr: u32,
    val: *const core::ffi::c_void,
    size: usize,
) -> i32 {
    let hub = &mut *to_fsi_master_hub(master);
    if id != 0 { return -EINVAL; }
    addr = addr.wrapping_add(hub.addr).wrapping_add((link as u32).wrapping_mul(FSI_HUB_LINK_SIZE));
    fsi_slave_write((*hub.upstream).slave, addr, val, size)
}

unsafe fn hub_master_break(master: *mut fsi_master, link: i32) -> i32 {
    let addr: u32 = 0x4;
    let cmd: u32 = cpu_to_be32(0xc0de0000);
    hub_master_write(master, link, 0, addr, &cmd as *const _ as *const core::ffi::c_void, core::mem::size_of::<u32>())
}

unsafe fn hub_master_link_enable(master: *mut fsi_master, link: i32, enable: bool) -> i32 {
    let hub = &mut *to_fsi_master_hub(master);
    let idx = link / 32;
    let bit = link % 32;
    let reg = cpu_to_be32(0x80000000u32 >> bit);
    if !enable {
        return fsi_device_write(hub.upstream, FSI_MCENP0 + (4 * idx as u32), &reg as *const _ as *const core::ffi::c_void, 4);
    }
    let rc = fsi_device_write(hub.upstream, FSI_MSENP0 + (4 * idx as u32), &reg as *const _ as *const core::ffi::c_void, 4);
    if rc != 0 { return rc; }
    mdelay(FSI_LINK_ENABLE_SETUP_TIME);
    0
}

unsafe fn hub_master_release(dev: *mut device) {
    let hub = to_fsi_master_hub(to_fsi_master(dev));
    kfree(hub as *mut core::ffi::c_void);
}

#[inline]
fn fsi_mmode_crs0(x: u32) -> u32 { (x & FSI_MMODE_CRS0MASK) << FSI_MMODE_CRS0SHFT }

#[inline]
fn fsi_mmode_crs1(x: u32) -> u32 { (x & FSI_MMODE_CRS1MASK) << FSI_MMODE_CRS1SHFT }

unsafe fn hub_master_init(hub: *mut fsi_master_hub) -> i32 {
    let dev = (*hub).upstream;
    let mut reg: u32;
    let mut rc: i32;
    reg = cpu_to_be32(FSI_MRESP_RST_ALL_MASTER | FSI_MRESP_RST_ALL_LINK | FSI_MRESP_RST_MCR | FSI_MRESP_RST_PYE);
    rc = fsi_device_write(dev, FSI_MRESP0, &reg as *const _ as *const core::ffi::c_void, 4); if rc != 0 { return rc; }
    reg = cpu_to_be32(FSI_MRESP_RST_ALL_MASTER | FSI_MRESP_RST_ALL_LINK | FSI_MRESP_RST_MCR | FSI_MRESP_RST_PYE);
    rc = fsi_device_write(dev, FSI_MRESP0, &reg as *const _ as *const core::ffi::c_void, 4); if rc != 0 { return rc; }
    reg = cpu_to_be32(FSI_MECTRL_EOAE | FSI_MECTRL_P8_AUTO_TERM);
    rc = fsi_device_write(dev, FSI_MECTRL, &reg as *const _ as *const core::ffi::c_void, 4); if rc != 0 { return rc; }
    reg = cpu_to_be32(FSI_MMODE_EIP | FSI_MMODE_ECRC | FSI_MMODE_EPC | fsi_mmode_crs0(1) | fsi_mmode_crs1(1) | FSI_MMODE_P8_TO_LSB);
    rc = fsi_device_write(dev, FSI_MMODE, &reg as *const _ as *const core::ffi::c_void, 4); if rc != 0 { return rc; }
    reg = cpu_to_be32(0xffff0000); rc = fsi_device_write(dev, FSI_MDLYR, &reg as *const _ as *const core::ffi::c_void, 4); if rc != 0 { return rc; }
    reg = cpu_to_be32(!0u32); rc = fsi_device_write(dev, FSI_MSENP0, &reg as *const _ as *const core::ffi::c_void, 4); if rc != 0 { return rc; }
    mdelay(FSI_LINK_ENABLE_SETUP_TIME);
    rc = fsi_device_write(dev, FSI_MCENP0, &reg as *const _ as *const core::ffi::c_void, 4); if rc != 0 { return rc; }
    rc = fsi_device_read(dev, FSI_MAEB, &mut reg as *mut _ as *mut core::ffi::c_void, 4); if rc != 0 { return rc; }
    reg = cpu_to_be32(FSI_MRESP_RST_ALL_MASTER | FSI_MRESP_RST_ALL_LINK); rc = fsi_device_write(dev, FSI_MRESP0, &reg as *const _ as *const core::ffi::c_void, 4); if rc != 0 { return rc; }
    rc = fsi_device_read(dev, FSI_MLEVP0, &mut reg as *mut _ as *mut core::ffi::c_void, 4); if rc != 0 { return rc; }
    reg = cpu_to_be32(FSI_MRESB_RST_GEN); rc = fsi_device_write(dev, FSI_MRESB0, &reg as *const _ as *const core::ffi::c_void, 4); if rc != 0 { return rc; }
    reg = cpu_to_be32(FSI_MRESB_RST_ERR);
    fsi_device_write(dev, FSI_MRESB0, &reg as *const _ as *const core::ffi::c_void, 4)
}

unsafe fn hub_master_probe(fsi_dev: *mut fsi_device) -> i32 {
    let dev = &mut (*fsi_dev).dev as *mut device;
    let mut hub: *mut fsi_master_hub;
    let mut reg: u32 = 0; let mut links: u32; let mut raw: u32 = 0; let mut rc: i32;
    rc = fsi_device_read(fsi_dev, FSI_MVER, &mut raw as *mut _ as *mut core::ffi::c_void, 4); if rc != 0 { return rc; }
    reg = be32_to_cpu(raw); links = (reg >> 8) & 0xff;
    dev_dbg(dev, "hub version %08x (%d links)\n", reg, links);
    rc = fsi_slave_claim_range((*fsi_dev).slave, FSI_HUB_LINK_OFFSET, FSI_HUB_LINK_SIZE * links); if rc != 0 { dev_err(dev, "can't claim slave address range for links"); return rc; }
    hub = kzalloc_obj(); if hub.is_null() { rc = -ENOMEM; goto err_release; }
    (*hub).addr = FSI_HUB_LINK_OFFSET; (*hub).size = FSI_HUB_LINK_SIZE * links; (*hub).upstream = fsi_dev;
    (*hub).master.dev.parent = dev; (*hub).master.dev.release = Some(hub_master_release); (*hub).master.dev.of_node = of_node_get(dev_of_node(dev));
    (*hub).master.n_links = links; (*hub).master.read = Some(hub_master_read); (*hub).master.write = Some(hub_master_write); (*hub).master.send_break = Some(hub_master_break); (*hub).master.link_enable = Some(hub_master_link_enable);
    fsi_set_drvdata(fsi_dev, hub as *mut core::ffi::c_void);
    hub_master_init(hub);
    rc = fsi_master_register(&mut (*hub).master); if rc != 0 { goto err_release; }
    get_device(&mut (*hub).master.dev); return 0;
err_release:
    fsi_slave_release_range((*fsi_dev).slave, FSI_HUB_LINK_OFFSET, FSI_HUB_LINK_SIZE * links); rc
}

unsafe fn hub_master_remove(fsi_dev: *mut fsi_device) {
    let hub = fsi_get_drvdata(fsi_dev) as *mut fsi_master_hub;
    fsi_master_unregister(&mut (*hub).master);
    fsi_slave_release_range((*(*hub).upstream).slave, (*hub).addr, (*hub).size);
    of_node_put((*hub).master.dev.of_node); put_device(&mut (*hub).master.dev);
}

// The following driver registration and metadata correspond to module_fsi_driver,
// MODULE_DESCRIPTION, and MODULE_LICENSE in the original source.
static mut HUB_MASTER_DRIVER: fsi_driver = fsi_driver {
    id_table: hub_master_ids,
    probe: Some(hub_master_probe),
    remove: Some(hub_master_remove),
    drv: driver { name: "fsi-master-hub", ..driver::default() },
};

static hub_master_ids: [fsi_device_id; 2] = [
    fsi_device_id { engine_type: FSI_ENGID_HUB_MASTER, version: FSI_VERSION_ANY },
    fsi_device_id { engine_type: 0, version: 0 },
];

module_fsi_driver!(HUB_MASTER_DRIVER);
module_description!("FSI hub master driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
