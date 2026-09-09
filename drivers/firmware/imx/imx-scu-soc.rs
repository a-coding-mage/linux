// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 NXP.
 */

// Dependencies supplied by the surrounding kernel translation.

static mut imx_sc_soc_ipc_handle: *mut imx_sc_ipc = core::ptr::null_mut();

#[repr(C, packed)]
pub struct imx_sc_msg_misc_get_soc_id {
    pub hdr: imx_sc_rpc_msg,
    pub data: imx_sc_msg_misc_get_soc_id_data,
}

#[repr(C)]
pub union imx_sc_msg_misc_get_soc_id_data {
    pub req: imx_sc_msg_misc_get_soc_id_req,
    pub resp: imx_sc_msg_misc_get_soc_id_resp,
}

#[repr(C, packed)]
pub struct imx_sc_msg_misc_get_soc_id_req {
    pub control: u32,
    pub resource: u16,
}

#[repr(C)]
pub struct imx_sc_msg_misc_get_soc_id_resp {
    pub id: u32,
}

#[repr(C, packed)]
pub struct imx_sc_msg_misc_get_soc_uid {
    pub hdr: imx_sc_rpc_msg,
    pub uid_low: u32,
    pub uid_high: u32,
}

extern "C" {
    fn imx_scu_call_rpc(ipc: *mut imx_sc_ipc, msg: *mut imx_sc_rpc_msg, have_lock: bool) -> i32;
    fn imx_scu_get_handle(ipc: *mut *mut imx_sc_ipc) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut soc_device_attribute;
    fn of_property_read_string(
        root: *mut device_node,
        name: *const core::ffi::c_char,
        value: *mut *const core::ffi::c_char,
    ) -> i32;
    fn devm_kasprintf(
        dev: *mut device,
        flags: u32,
        fmt: *const core::ffi::c_char,
        ...,
    ) -> *mut core::ffi::c_char;
    fn soc_device_register(attr: *mut soc_device_attribute) -> *mut soc_device;
    fn ptr_err(ptr: *mut soc_device) -> i32;
    fn is_err(ptr: *mut soc_device) -> bool;
    fn pr_err(fmt: *const core::ffi::c_char, ...);
}

unsafe fn imx_scu_soc_uid(soc_uid: *mut u64) -> i32 {
    let mut msg: imx_sc_msg_misc_get_soc_uid = core::mem::zeroed();
    let hdr: *mut imx_sc_rpc_msg = &mut msg.hdr;
    (*hdr).ver = IMX_SC_RPC_VERSION;
    (*hdr).svc = IMX_SC_RPC_SVC_MISC;
    (*hdr).func = IMX_SC_MISC_FUNC_UNIQUE_ID;
    (*hdr).size = 1;

    let ret = imx_scu_call_rpc(imx_sc_soc_ipc_handle, &mut msg.hdr, true);
    if ret != 0 {
        pr_err(c"%s: get soc uid failed, ret %d\n".as_ptr(), c"imx_scu_soc_uid".as_ptr(), ret);
        return ret;
    }

    *soc_uid = msg.uid_high as u64;
    *soc_uid <<= 32;
    *soc_uid |= msg.uid_low as u64;
    0
}

unsafe fn imx_scu_soc_id() -> i32 {
    let mut msg: imx_sc_msg_misc_get_soc_id = core::mem::zeroed();
    let hdr: *mut imx_sc_rpc_msg = &mut msg.hdr;
    (*hdr).ver = IMX_SC_RPC_VERSION;
    (*hdr).svc = IMX_SC_RPC_SVC_MISC;
    (*hdr).func = IMX_SC_MISC_FUNC_GET_CONTROL;
    (*hdr).size = 3;

    msg.data.req.control = IMX_SC_C_ID;
    msg.data.req.resource = IMX_SC_R_SYSTEM;

    let ret = imx_scu_call_rpc(imx_sc_soc_ipc_handle, &mut msg.hdr, true);
    if ret != 0 {
        pr_err(c"%s: get soc info failed, ret %d\n".as_ptr(), c"imx_scu_soc_id".as_ptr(), ret);
        return ret;
    }
    msg.data.resp.id as i32
}

unsafe fn imx_scu_soc_name(id: u32) -> *const core::ffi::c_char {
    match id {
        0x1 => c"i.MX8QM".as_ptr(),
        0x2 => c"i.MX8QXP".as_ptr(),
        0xe => c"i.MX8DXL".as_ptr(),
        _ => c"NULL".as_ptr(),
    }
}

pub unsafe fn imx_scu_soc_init(dev: *mut device) -> i32 {
    let mut soc_dev_attr: *mut soc_device_attribute;
    let soc_dev: *mut soc_device;
    let mut id: i32;
    let mut ret: i32;
    let mut uid: u64 = 0;
    let mut val: u32;

    ret = imx_scu_get_handle(&mut imx_sc_soc_ipc_handle);
    if ret != 0 { return ret; }

    soc_dev_attr = devm_kzalloc(dev, core::mem::size_of::<soc_device_attribute>(), GFP_KERNEL);
    if soc_dev_attr.is_null() { return -ENOMEM; }
    (*soc_dev_attr).family = c"Freescale i.MX".as_ptr();

    ret = of_property_read_string(of_root, c"model".as_ptr(), &mut (*soc_dev_attr).machine);
    if ret != 0 { return ret; }
    id = imx_scu_soc_id();
    if id < 0 { return -EINVAL; }
    ret = imx_scu_soc_uid(&mut uid);
    if ret < 0 { return -EINVAL; }

    val = (id as u32) & 0x1f;
    (*soc_dev_attr).soc_id = imx_scu_soc_name(val);
    val = ((id as u32) >> 5) & 0xf;
    val = (((val >> 2) + 1) << 4) | (val & 0x3);
    (*soc_dev_attr).revision = devm_kasprintf(dev, GFP_KERNEL, c"%d.%d".as_ptr(), (val >> 4) & 0xf, val & 0xf);
    if (*soc_dev_attr).revision.is_null() { return -ENOMEM; }
    (*soc_dev_attr).serial_number = devm_kasprintf(dev, GFP_KERNEL, c"%016llX".as_ptr(), uid);
    if (*soc_dev_attr).serial_number.is_null() { return -ENOMEM; }

    soc_dev = soc_device_register(soc_dev_attr);
    if is_err(soc_dev) { return ptr_err(soc_dev); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
