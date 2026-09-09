// SPDX-License-Identifier: GPL-2.0
/*
 * Platform driver for CDX bus.
 *
 * Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
 */

// Linux kernel dependencies supplied by the surrounding translation.

#[repr(C)]
pub struct RpmsgDeviceId {
    pub name: *const core::ffi::c_char,
    pub driver_data: usize,
}

#[repr(C)]
pub struct RpmsgDevice;
#[repr(C)]
pub struct PlatformDevice;
#[repr(C)]
pub struct DeviceNode { pub phandle: u32 }
#[repr(C)]
pub struct Device;
#[repr(C)]
pub struct Rproc;
#[repr(C)]
pub struct RpmsgEndpoint;
#[repr(C)]
pub struct WorkStruct;
#[repr(C)]
pub struct CdxDword;
#[repr(C)]
pub struct CdxController { pub priv_: *mut CdxMcdi }
#[repr(C)]
pub struct CdxMcdi {
    pub ept: *mut RpmsgEndpoint,
    pub r5_rproc: *mut Rproc,
    pub rpdev: *mut RpmsgDevice,
    pub work: WorkStruct,
}
#[repr(C)]
pub struct RpmsgChannelInfo {
    pub name: [core::ffi::c_char; 32],
    pub src: u32,
    pub dst: u32,
}
#[repr(C)]
pub struct RpmsgDriver;

extern "C" {
    static mut cdx_rpmsg_id_table: [RpmsgDeviceId; 2];
    fn kzalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn memcpy(dst: *mut u8, src: *const core::ffi::c_void, n: usize);
    fn rpmsg_send(ept: *mut RpmsgEndpoint, data: *const u8, len: usize) -> i32;
    fn platform_get_drvdata(pdev: *mut PlatformDevice) -> *mut core::ffi::c_void;
    fn of_parse_phandle(node: *mut DeviceNode, name: *const core::ffi::c_char, index: i32) -> *mut DeviceNode;
    fn rproc_get_by_phandle(phandle: u32) -> *mut Rproc;
    fn rproc_boot(rp: *mut Rproc) -> i32;
    fn rproc_put(rp: *mut Rproc);
    fn of_node_put(node: *mut DeviceNode);
    fn rproc_detach(rp: *mut Rproc);
    fn dev_get_drvdata(dev: *mut Device) -> *mut core::ffi::c_void;
    fn cdx_mcdi_process_cmd(mcdi: *mut CdxMcdi, data: *mut CdxDword, len: i32);
    fn cdx_rpmsg_post_probe(controller: *mut CdxController);
    fn rpmsg_create_ept(rpdev: *mut RpmsgDevice, cb: unsafe extern "C" fn(*mut RpmsgDevice, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, u32) -> i32, priv_: *mut core::ffi::c_void, info: RpmsgChannelInfo) -> *mut RpmsgEndpoint;
    fn dev_set_drvdata(dev: *mut Device, data: *mut core::ffi::c_void);
    fn schedule_work(work: *mut WorkStruct);
    fn flush_work(work: *mut WorkStruct);
    fn cdx_rpmsg_pre_remove(controller: *mut CdxController);
    fn rpmsg_destroy_ept(ept: *mut RpmsgEndpoint);
    fn register_rpmsg_driver(driver: *mut RpmsgDriver) -> i32;
    fn unregister_rpmsg_driver(driver: *mut RpmsgDriver);
}

const GFP_KERNEL: u32 = 0;
const RPMSG_ADDR_ANY: u32 = 0xffff_ffff;
const MCDI_BUF_LEN: i32 = 4096;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const EPROBE_DEFER: i32 = 517;
const ENXIO: i32 = 6;

static mut CDX_RPMSG_ID_TABLE: [RpmsgDeviceId; 2] = [
    RpmsgDeviceId { name: b"mcdi_ipc\0".as_ptr() as *const _, driver_data: 0 },
    RpmsgDeviceId { name: core::ptr::null(), driver_data: 0 },
];

unsafe extern "C" fn cdx_attach_to_rproc(_pdev: *mut PlatformDevice) -> i32 { 0 }
unsafe extern "C" fn cdx_detach_to_r5(_pdev: *mut PlatformDevice) {}

unsafe extern "C" fn cdx_rpmsg_post_probe_work(work: *mut WorkStruct) {
    let cdx_mcdi = work as *mut CdxMcdi;
    let cdx_c = dev_get_drvdata((*cdx_mcdi).rpdev as *mut Device) as *mut CdxController;
    cdx_rpmsg_post_probe(cdx_c);
}

unsafe extern "C" fn cdx_rpmsg_probe(rpdev: *mut RpmsgDevice) -> i32 {
    let cdx_c = CDX_RPMSG_ID_TABLE[0].driver_data as *mut CdxController;
    let cdx_mcdi = (*cdx_c).priv_;
    let mut chinfo: RpmsgChannelInfo = core::mem::zeroed();
    chinfo.src = RPMSG_ADDR_ANY;
    chinfo.dst = 0;
    let ept = rpmsg_create_ept(rpdev, cdx_rpmsg_cb, core::ptr::null_mut(), chinfo);
    if ept.is_null() { return -EINVAL; }
    (*cdx_mcdi).ept = ept;
    (*cdx_mcdi).rpdev = rpdev;
    dev_set_drvdata(rpdev as *mut Device, cdx_c as *mut _);
    schedule_work(&mut (*cdx_mcdi).work);
    0
}

unsafe extern "C" fn cdx_rpmsg_remove(rpdev: *mut RpmsgDevice) {
    let cdx_c = dev_get_drvdata(rpdev as *mut Device) as *mut CdxController;
    let cdx_mcdi = (*cdx_c).priv_;
    flush_work(&mut (*cdx_mcdi).work);
    cdx_rpmsg_pre_remove(cdx_c);
    rpmsg_destroy_ept((*cdx_mcdi).ept);
    dev_set_drvdata(rpdev as *mut Device, core::ptr::null_mut());
}

pub unsafe extern "C" fn cdx_rpmsg_send(
    cdx_mcdi: *mut CdxMcdi, hdr: *const CdxDword, hdr_len: usize,
    sdu: *const CdxDword, sdu_len: usize,
) -> i32 {
    let send_buf = kzalloc(hdr_len + sdu_len, GFP_KERNEL);
    if send_buf.is_null() { return -ENOMEM; }
    memcpy(send_buf, hdr as *const core::ffi::c_void, hdr_len);
    memcpy(send_buf.add(hdr_len), sdu as *const core::ffi::c_void, sdu_len);
    let ret = rpmsg_send((*cdx_mcdi).ept, send_buf, hdr_len + sdu_len);
    kfree(send_buf);
    ret
}

pub unsafe extern "C" fn cdx_rpmsg_cb(
    rpdev: *mut RpmsgDevice, data: *mut core::ffi::c_void, len: i32,
    _priv: *mut core::ffi::c_void, _src: u32,
) -> i32 {
    let cdx_c = dev_get_drvdata(rpdev as *mut Device) as *mut CdxController;
    let cdx_mcdi = (*cdx_c).priv_;
    if len > MCDI_BUF_LEN { return -EINVAL; }
    cdx_mcdi_process_cmd(cdx_mcdi, data as *mut CdxDword, len);
    0
}

pub unsafe extern "C" fn cdx_setup_rpmsg(pdev: *mut PlatformDevice) -> i32 {
    let mut ret = cdx_attach_to_rproc(pdev);
    if ret != 0 { return ret; }
    let cdx_c = platform_get_drvdata(pdev) as *mut CdxController;
    let cdx_mcdi = (*cdx_c).priv_;
    CDX_RPMSG_ID_TABLE[0].driver_data = cdx_c as usize;
    (*cdx_mcdi).work = core::mem::zeroed();
    ret = 0;
    if ret != 0 { cdx_detach_to_r5(pdev); }
    ret
}

pub unsafe extern "C" fn cdx_destroy_rpmsg(pdev: *mut PlatformDevice) {
    cdx_detach_to_r5(pdev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
