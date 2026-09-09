// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016, Linaro Ltd.
 * Copyright (c) 2015, Sony Mobile Communications Inc.
 */

// Linux kernel dependencies supplied by other translation units.

use core::ffi::c_void;

#[repr(C)]
pub struct btqcomsmd {
    hdev: *mut hci_dev,
    acl_channel: *mut rpmsg_endpoint,
    cmd_channel: *mut rpmsg_endpoint,
}

#[repr(C)]
pub struct hci_dev {
    pub stat: hci_dev_stats,
    pub bus: i32,
    pub open: Option<unsafe extern "C" fn(*mut hci_dev) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut hci_dev) -> i32>,
    pub send: Option<unsafe extern "C" fn(*mut hci_dev, *mut sk_buff) -> i32>,
    pub setup: Option<unsafe extern "C" fn(*mut hci_dev) -> i32>,
    pub set_bdaddr: Option<unsafe extern "C" fn(*mut hci_dev, *const bdaddr_t) -> i32>,
}

#[repr(C)]
pub struct hci_dev_stats {
    pub err_rx: u64,
    pub byte_rx: u64,
    pub err_tx: u64,
    pub acl_tx: u64,
    pub byte_tx: u64,
    pub cmd_tx: u64,
}

#[repr(C)]
pub struct sk_buff {
    pub data: *mut u8,
    pub len: usize,
}

#[repr(C)]
pub struct rpmsg_endpoint;
#[repr(C)]
pub struct rpmsg_device;
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct device {
    pub parent: *mut device,
}
#[repr(C)]
pub struct bdaddr_t;
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
}
#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

extern "C" {
    fn bt_skb_alloc(size: usize, priority: i32) -> *mut sk_buff;
    fn hci_skb_pkt_type(skb: *mut sk_buff) -> *mut u8;
    fn skb_put_data(skb: *mut sk_buff, data: *const c_void, len: usize);
    fn hci_recv_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> i32;
    fn hci_get_drvdata(hdev: *mut hci_dev) -> *mut btqcomsmd;
    fn rpmsg_send(ept: *mut rpmsg_endpoint, data: *const u8, len: usize) -> i32;
    fn kfree_skb(skb: *mut sk_buff);
    fn __hci_cmd_sync(hdev: *mut hci_dev, opcode: u16, plen: u8, param: *const c_void, timeout: u32) -> *mut sk_buff;
    fn hci_set_quirk(hdev: *mut hci_dev, quirk: u32);
    fn qca_set_bdaddr_rome(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> i32;
    fn usleep_range(min: u32, max: u32);
    fn devm_kzalloc(dev: *mut device, size: usize, priority: i32) -> *mut btqcomsmd;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn qcom_wcnss_open_channel(wcnss: *mut c_void, name: *const u8, callback: unsafe extern "C" fn(*mut rpmsg_device, *mut c_void, i32, *mut c_void, u32) -> i32, priv_: *mut btqcomsmd) -> *mut rpmsg_endpoint;
    fn rpmsg_destroy_ept(ept: *mut rpmsg_endpoint);
    fn hci_alloc_dev() -> *mut hci_dev;
    fn hci_set_drvdata(hdev: *mut hci_dev, data: *mut btqcomsmd);
    fn set_hcidev_dev(hdev: *mut hci_dev, dev: *mut device);
    fn hci_register_dev(hdev: *mut hci_dev) -> i32;
    fn hci_free_dev(hdev: *mut hci_dev);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut btqcomsmd);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut btqcomsmd;
    fn hci_unregister_dev(hdev: *mut hci_dev);
}

const GFP_ATOMIC: i32 = 0;
const HCI_ACLDATA_PKT: u32 = 2;
const HCI_EVENT_PKT: u32 = 4;
const HCI_COMMAND_PKT: u32 = 1;
const HCI_SMD: i32 = 3;
const HCI_OP_RESET: u16 = 0x0c03;
const HCI_INIT_TIMEOUT: u32 = 10_000;
const HCI_QUIRK_USE_BDADDR_PROPERTY: u32 = 0;
const ENOMEM: i32 = 12;
const EILSEQ: i32 = 84;

unsafe extern "C" fn btqcomsmd_recv(hdev: *mut hci_dev, r#type: u32, data: *const c_void, count: usize) -> i32 {
    let skb = bt_skb_alloc(count, GFP_ATOMIC);
    if skb.is_null() {
        (*hdev).stat.err_rx += 1;
        return -ENOMEM;
    }
    *hci_skb_pkt_type(skb) = r#type as u8;
    skb_put_data(skb, data, count);
    hci_recv_frame(hdev, skb)
}

unsafe extern "C" fn btqcomsmd_acl_callback(_rpdev: *mut rpmsg_device, data: *mut c_void, count: i32, priv_: *mut c_void, _addr: u32) -> i32 {
    let btq = priv_ as *mut btqcomsmd;
    (*(*btq).hdev).stat.byte_rx += count as u64;
    btqcomsmd_recv((*btq).hdev, HCI_ACLDATA_PKT, data, count as usize)
}

unsafe extern "C" fn btqcomsmd_cmd_callback(_rpdev: *mut rpmsg_device, data: *mut c_void, count: i32, priv_: *mut c_void, _addr: u32) -> i32 {
    let btq = priv_ as *mut btqcomsmd;
    (*(*btq).hdev).stat.byte_rx += count as u64;
    btqcomsmd_recv((*btq).hdev, HCI_EVENT_PKT, data, count as usize)
}

unsafe extern "C" fn btqcomsmd_send(hdev: *mut hci_dev, skb: *mut sk_buff) -> i32 {
    let btq = hci_get_drvdata(hdev);
    let mut ret;
    match *hci_skb_pkt_type(skb) as u32 {
        HCI_ACLDATA_PKT => {
            ret = rpmsg_send((*btq).acl_channel, (*skb).data, (*skb).len);
            if ret != 0 { (*hdev).stat.err_tx += 1; } else { (*hdev).stat.acl_tx += 1; (*hdev).stat.byte_tx += (*skb).len as u64; }
        }
        HCI_COMMAND_PKT => {
            ret = rpmsg_send((*btq).cmd_channel, (*skb).data, (*skb).len);
            if ret != 0 { (*hdev).stat.err_tx += 1; } else { (*hdev).stat.cmd_tx += 1; (*hdev).stat.byte_tx += (*skb).len as u64; }
        }
        _ => { ret = -EILSEQ; }
    }
    if ret == 0 { kfree_skb(skb); }
    ret
}

unsafe extern "C" fn btqcomsmd_open(_hdev: *mut hci_dev) -> i32 { 0 }
unsafe extern "C" fn btqcomsmd_close(_hdev: *mut hci_dev) -> i32 { 0 }

unsafe extern "C" fn btqcomsmd_setup(hdev: *mut hci_dev) -> i32 {
    let skb = __hci_cmd_sync(hdev, HCI_OP_RESET, 0, core::ptr::null(), HCI_INIT_TIMEOUT);
    if (skb as isize) < 0 { return skb as isize as i32; }
    kfree_skb(skb);
    hci_set_quirk(hdev, HCI_QUIRK_USE_BDADDR_PROPERTY);
    0
}

unsafe extern "C" fn btqcomsmd_set_bdaddr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> i32 {
    let ret = qca_set_bdaddr_rome(hdev, bdaddr);
    if ret != 0 { return ret; }
    usleep_range(1000, 10000);
    0
}

unsafe extern "C" fn btqcomsmd_probe(pdev: *mut platform_device) -> i32 {
    let btq = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<btqcomsmd>(), GFP_ATOMIC);
    if btq.is_null() { return -ENOMEM; }
    let wcnss = dev_get_drvdata((*pdev).dev.parent);
    (*btq).acl_channel = qcom_wcnss_open_channel(wcnss, b"APPS_RIVA_BT_ACL\0".as_ptr(), btqcomsmd_acl_callback, btq);
    if (*btq).acl_channel.is_null() { return -ENOMEM; }
    (*btq).cmd_channel = qcom_wcnss_open_channel(wcnss, b"APPS_RIVA_BT_CMD\0".as_ptr(), btqcomsmd_cmd_callback, btq);
    if (*btq).cmd_channel.is_null() { rpmsg_destroy_ept((*btq).acl_channel); return -ENOMEM; }
    let hdev = hci_alloc_dev();
    if hdev.is_null() { rpmsg_destroy_ept((*btq).cmd_channel); rpmsg_destroy_ept((*btq).acl_channel); return -ENOMEM; }
    hci_set_drvdata(hdev, btq); (*btq).hdev = hdev; set_hcidev_dev(hdev, &mut (*pdev).dev);
    (*hdev).bus = HCI_SMD; (*hdev).open = Some(btqcomsmd_open); (*hdev).close = Some(btqcomsmd_close); (*hdev).send = Some(btqcomsmd_send); (*hdev).setup = Some(btqcomsmd_setup); (*hdev).set_bdaddr = Some(btqcomsmd_set_bdaddr);
    let ret = hci_register_dev(hdev);
    if ret < 0 { hci_free_dev(hdev); rpmsg_destroy_ept((*btq).cmd_channel); rpmsg_destroy_ept((*btq).acl_channel); return ret; }
    platform_set_drvdata(pdev, btq);
    0
}

unsafe extern "C" fn btqcomsmd_remove(pdev: *mut platform_device) {
    let btq = platform_get_drvdata(pdev);
    hci_unregister_dev((*btq).hdev); hci_free_dev((*btq).hdev);
    rpmsg_destroy_ept((*btq).cmd_channel); rpmsg_destroy_ept((*btq).acl_channel);
}

static mut BTQCOMSMD_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"qcom,wcnss-bt\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static mut BTQCOMSMD_DRIVER: platform_driver = platform_driver {
    probe: Some(btqcomsmd_probe),
    remove: Some(btqcomsmd_remove),
};

// MODULE_DEVICE_TABLE(of, btqcomsmd_of_match);
// module_platform_driver(btqcomsmd_driver);
// MODULE_AUTHOR("Bjorn Andersson <bjorn.andersson@sonymobile.com>");
// MODULE_DESCRIPTION("Qualcomm SMD HCI driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
