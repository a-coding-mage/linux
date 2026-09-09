// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Generic Bluetooth SDIO driver
 *
 * Copyright (C) 2007 Cambridge Silicon Radio Ltd.
 * Copyright (C) 2007 Marcel Holtmann <marcel@holtmann.org>
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const VERSION: &str = "0.1";

static BTSDIO_TABLE: [sdio_device_id; 3] = [
    sdio_device_id { class: SDIO_CLASS_BT_A },
    sdio_device_id { class: SDIO_CLASS_BT_B },
    sdio_device_id { class: 0 },
];

#[repr(C)]
struct btsdio_data {
    hdev: *mut hci_dev,
    func: *mut sdio_func,
    work: work_struct,
    txq: sk_buff_head,
}

const REG_RDAT: u32 = 0x00;
const REG_TDAT: u32 = 0x00;
const REG_PC_RRT: u32 = 0x10;
const REG_PC_WRT: u32 = 0x11;
const REG_RTC_STAT: u32 = 0x12;
const REG_RTC_SET: u32 = 0x12;
const REG_INTRD: u32 = 0x13;
const REG_CL_INTRD: u32 = 0x13;
const REG_EN_INTRD: u32 = 0x14;
const REG_MD_STAT: u32 = 0x20;
const REG_MD_SET: u32 = 0x20;

unsafe fn btsdio_tx_packet(data: *mut btsdio_data, skb: *mut sk_buff) -> i32 {
    let mut err: i32;
    BT_DBG!("%s", (*(*data).hdev).name);

    skb_push(skb, 4);
    (*skb).data[0] = ((*skb).len & 0x0000ff) as u8;
    (*skb).data[1] = (((*skb).len & 0x00ff00) >> 8) as u8;
    (*skb).data[2] = (((*skb).len & 0xff0000) >> 16) as u8;
    (*skb).data[3] = hci_skb_pkt_type(skb);

    err = sdio_writesb((*data).func, REG_TDAT, (*skb).data, (*skb).len);
    if err < 0 {
        skb_pull(skb, 4);
        sdio_writeb((*data).func, 0x01, REG_PC_WRT, core::ptr::null_mut());
        return err;
    }
    (*(*data).hdev).stat.byte_tx += (*skb).len;
    kfree_skb(skb);
    0
}

unsafe extern "C" fn btsdio_work(work: *mut work_struct) {
    let data = container_of!(work, btsdio_data, work);
    let mut skb: *mut sk_buff;
    let mut err: i32;
    BT_DBG!("%s", (*(*data).hdev).name);
    sdio_claim_host((*data).func);
    loop {
        skb = skb_dequeue(&mut (*data).txq);
        if skb.is_null() { break; }
        err = btsdio_tx_packet(data, skb);
        if err < 0 {
            (*(*data).hdev).stat.err_tx += 1;
            skb_queue_head(&mut (*data).txq, skb);
            break;
        }
    }
    sdio_release_host((*data).func);
}

unsafe fn btsdio_rx_packet(data: *mut btsdio_data) -> i32 {
    let mut hdr = [0u8; 4];
    let skb: *mut sk_buff;
    let mut err: i32;
    let len: usize;
    BT_DBG!("%s", (*(*data).hdev).name);
    err = sdio_readsb((*data).func, hdr.as_mut_ptr(), REG_RDAT, 4);
    if err < 0 { return err; }
    len = (hdr[0] as usize) | ((hdr[1] as usize) << 8) | ((hdr[2] as usize) << 16);
    if len < 4 || len > 65543 { return -EILSEQ; }
    skb = bt_skb_alloc(len - 4, GFP_KERNEL);
    if skb.is_null() { return -ENOMEM; }
    skb_put(skb, len - 4);
    err = sdio_readsb((*data).func, (*skb).data, REG_RDAT, len - 4);
    if err < 0 { kfree_skb(skb); return err; }
    (*(*data).hdev).stat.byte_rx += len;
    match hdr[3] {
        HCI_EVENT_PKT | HCI_ACLDATA_PKT | HCI_SCODATA_PKT | HCI_ISODATA_PKT => {
            hci_skb_pkt_type_set(skb, hdr[3]);
            err = hci_recv_frame((*data).hdev, skb);
            if err < 0 { return err; }
        }
        _ => { kfree_skb(skb); return -EINVAL; }
    }
    sdio_writeb((*data).func, 0x00, REG_PC_RRT, core::ptr::null_mut());
    0
}

unsafe extern "C" fn btsdio_interrupt(func: *mut sdio_func) {
    let data = sdio_get_drvdata(func);
    BT_DBG!("%s", (*(*data).hdev).name);
    let intrd = sdio_readb(func, REG_INTRD, core::ptr::null_mut());
    if intrd & 0x01 != 0 {
        sdio_writeb(func, 0x01, REG_CL_INTRD, core::ptr::null_mut());
        if btsdio_rx_packet(data) < 0 {
            (*(*data).hdev).stat.err_rx += 1;
            sdio_writeb((*data).func, 0x01, REG_PC_RRT, core::ptr::null_mut());
        }
    }
}

unsafe extern "C" fn btsdio_open(hdev: *mut hci_dev) -> i32 {
    let data = hci_get_drvdata(hdev);
    BT_DBG!("%s", (*hdev).name);
    sdio_claim_host((*data).func);
    let mut err = sdio_enable_func((*data).func);
    if err < 0 { sdio_release_host((*data).func); return err; }
    err = sdio_claim_irq((*data).func, Some(btsdio_interrupt));
    if err < 0 { sdio_disable_func((*data).func); sdio_release_host((*data).func); return err; }
    if (*(*data).func).class == SDIO_CLASS_BT_B { sdio_writeb((*data).func, 0x00, REG_MD_SET, core::ptr::null_mut()); }
    sdio_writeb((*data).func, 0x01, REG_EN_INTRD, core::ptr::null_mut());
    sdio_release_host((*data).func);
    err
}

unsafe extern "C" fn btsdio_close(hdev: *mut hci_dev) -> i32 {
    let data = hci_get_drvdata(hdev);
    BT_DBG!("%s", (*hdev).name);
    sdio_claim_host((*data).func);
    sdio_writeb((*data).func, 0x00, REG_EN_INTRD, core::ptr::null_mut());
    sdio_release_irq((*data).func);
    sdio_disable_func((*data).func);
    sdio_release_host((*data).func);
    0
}

unsafe extern "C" fn btsdio_flush(hdev: *mut hci_dev) -> i32 {
    let data = hci_get_drvdata(hdev);
    BT_DBG!("%s", (*hdev).name);
    skb_queue_purge(&mut (*data).txq);
    0
}

unsafe extern "C" fn btsdio_send_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> i32 {
    let data = hci_get_drvdata(hdev);
    BT_DBG!("%s", (*hdev).name);
    match hci_skb_pkt_type(skb) {
        HCI_COMMAND_PKT => (*hdev).stat.cmd_tx += 1,
        HCI_ACLDATA_PKT => (*hdev).stat.acl_tx += 1,
        HCI_SCODATA_PKT => (*hdev).stat.sco_tx += 1,
        _ => return -EILSEQ,
    }
    skb_queue_tail(&mut (*data).txq, skb);
    schedule_work(&mut (*data).work);
    0
}

unsafe extern "C" fn btsdio_probe(func: *mut sdio_func, id: *const sdio_device_id) -> i32 {
    let mut data: *mut btsdio_data;
    let hdev: *mut hci_dev;
    let mut tuple = (*func).tuples;
    while !tuple.is_null() { BT_DBG!("code 0x%x size %d", (*tuple).code, (*tuple).size); tuple = (*tuple).next; }
    if (*func).vendor == SDIO_VENDOR_ID_BROADCOM && !mmc_card_is_removable((*(*func).card).host) {
        match (*func).device {
            SDIO_DEVICE_ID_BROADCOM_43341 | SDIO_DEVICE_ID_BROADCOM_43430 |
            SDIO_DEVICE_ID_BROADCOM_4345 | SDIO_DEVICE_ID_BROADCOM_43455 |
            SDIO_DEVICE_ID_BROADCOM_4356 | SDIO_DEVICE_ID_BROADCOM_CYPRESS_4373 => return -ENODEV,
            _ => {}
        }
    }
    data = devm_kzalloc(&mut (*func).dev, core::mem::size_of::<btsdio_data>(), GFP_KERNEL) as *mut btsdio_data;
    if data.is_null() { return -ENOMEM; }
    (*data).func = func;
    INIT_WORK(&mut (*data).work, Some(btsdio_work));
    skb_queue_head_init(&mut (*data).txq);
    hdev = hci_alloc_dev();
    if hdev.is_null() { return -ENOMEM; }
    (*hdev).bus = HCI_SDIO;
    hci_set_drvdata(hdev, data);
    (*data).hdev = hdev;
    SET_HCIDEV_DEV(hdev, &mut (*func).dev);
    (*hdev).open = Some(btsdio_open); (*hdev).close = Some(btsdio_close);
    (*hdev).flush = Some(btsdio_flush); (*hdev).send = Some(btsdio_send_frame);
    if (*func).vendor == 0x0104 && (*func).device == 0x00c5 { hci_set_quirk(hdev, HCI_QUIRK_RESET_ON_CLOSE); }
    let err = hci_register_dev(hdev);
    if err < 0 { hci_free_dev(hdev); return err; }
    sdio_set_drvdata(func, data);
    0
}

unsafe extern "C" fn btsdio_remove(func: *mut sdio_func) {
    let data = sdio_get_drvdata(func);
    BT_DBG!("func %p", func);
    if data.is_null() { return; }
    cancel_work_sync(&mut (*data).work);
    let hdev = (*data).hdev;
    sdio_set_drvdata(func, core::ptr::null_mut());
    hci_unregister_dev(hdev);
    hci_free_dev(hdev);
}

// module_sdio_driver(btsdio_driver);
// MODULE_AUTHOR("Marcel Holtmann <marcel@holtmann.org>");
// MODULE_DESCRIPTION("Generic Bluetooth SDIO driver ver " VERSION);
// MODULE_VERSION(VERSION);
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
