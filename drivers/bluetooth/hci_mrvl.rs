// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Bluetooth HCI UART driver for marvell devices
 *
 * Copyright (C) 2016 Marvell International Ltd.
 * Copyright (C) 2016 Intel Corporation
 */

// Kernel dependencies supplied externally.

const HCI_FW_REQ_PKT: u8 = 0xA5;
const HCI_CHIP_VER_PKT: u8 = 0xAA;
const MRVL_ACK: u8 = 0x5A;
const MRVL_NAK: u8 = 0xBF;
const MRVL_RAW_DATA: u8 = 0x1F;
const MRVL_SET_BAUDRATE: u16 = 0xFC09;

enum State {
    ChipVerPending,
    FwReqPending,
    FwLoaded,
}

#[repr(C)]
struct MrvlData {
    rx_skb: *mut sk_buff,
    txq: sk_buff_head,
    rawq: sk_buff_head,
    flags: c_ulong,
    tx_len: c_uint,
    id: u8,
    rev: u8,
}

#[repr(C)]
struct MrvlSerdev {
    hu: hci_uart,
}

#[repr(C, packed)]
struct HciMrvlPkt {
    lhs: __le16,
    rhs: __le16,
}

const HCI_MRVL_PKT_SIZE: usize = 4;

unsafe fn mrvl_open(hu: *mut hci_uart) -> c_int {
    let mrvl: *mut MrvlData;
    let ret: c_int;

    BT_DBG!("hu %p", hu);
    if !hci_uart_has_flow_control(hu) { return -EOPNOTSUPP; }
    mrvl = kzalloc_obj::<MrvlData>();
    if mrvl.is_null() { return -ENOMEM; }
    skb_queue_head_init(&mut (*mrvl).txq);
    skb_queue_head_init(&mut (*mrvl).rawq);
    set_bit(State::ChipVerPending as usize, &mut (*mrvl).flags);
    (*hu).priv_ = mrvl as *mut c_void;
    if !(*hu).serdev.is_null() {
        ret = serdev_device_open((*hu).serdev);
        if ret != 0 { kfree(mrvl as *mut c_void); return ret; }
    }
    0
}

unsafe fn mrvl_close(hu: *mut hci_uart) -> c_int {
    let mrvl = (*hu).priv_ as *mut MrvlData;
    BT_DBG!("hu %p", hu);
    if !(*hu).serdev.is_null() { serdev_device_close((*hu).serdev); }
    skb_queue_purge(&mut (*mrvl).txq);
    skb_queue_purge(&mut (*mrvl).rawq);
    kfree_skb((*mrvl).rx_skb);
    kfree(mrvl as *mut c_void);
    (*hu).priv_ = core::ptr::null_mut();
    0
}

unsafe fn mrvl_flush(hu: *mut hci_uart) -> c_int {
    let mrvl = (*hu).priv_ as *mut MrvlData;
    BT_DBG!("hu %p", hu);
    skb_queue_purge(&mut (*mrvl).txq);
    skb_queue_purge(&mut (*mrvl).rawq);
    0
}

unsafe fn mrvl_dequeue(hu: *mut hci_uart) -> *mut sk_buff {
    let mrvl = (*hu).priv_ as *mut MrvlData;
    let mut skb = skb_dequeue(&mut (*mrvl).txq);
    if skb.is_null() {
        skb = skb_dequeue(&mut (*mrvl).rawq);
    } else {
        memcpy(skb_push(skb, 1), &hci_skb_pkt_type(skb) as *const _ as *const c_void, 1);
    }
    skb
}

unsafe fn mrvl_enqueue(hu: *mut hci_uart, skb: *mut sk_buff) -> c_int {
    let mrvl = (*hu).priv_ as *mut MrvlData;
    skb_queue_tail(&mut (*mrvl).txq, skb);
    0
}

unsafe fn mrvl_send_ack(hu: *mut hci_uart, typ: c_uchar) {
    let mrvl = (*hu).priv_ as *mut MrvlData;
    let skb = bt_skb_alloc(0, GFP_ATOMIC);
    if skb.is_null() { bt_dev_err((*hu).hdev, "Unable to alloc ack/nak packet"); return; }
    hci_skb_pkt_type(skb) = typ;
    skb_queue_tail(&mut (*mrvl).txq, skb);
    hci_uart_tx_wakeup(hu);
}

unsafe fn mrvl_recv_fw_req(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    let pkt = (*skb).data as *mut HciMrvlPkt;
    let hu = hci_get_drvdata(hdev);
    let mrvl = (*hu).priv_ as *mut MrvlData;
    let mut ret = 0;
    if ((*pkt).lhs ^ (*pkt).rhs) != 0xffff { bt_dev_err(hdev, "Corrupted mrvl header"); mrvl_send_ack(hu, MRVL_NAK); ret = -EINVAL; }
    else {
        mrvl_send_ack(hu, MRVL_ACK);
        if !test_bit(State::FwReqPending as usize, &(*mrvl).flags) { bt_dev_err(hdev, "Received unexpected firmware request"); ret = -EINVAL; }
        else { (*mrvl).tx_len = le16_to_cpu((*pkt).lhs) as c_uint; clear_bit(State::FwReqPending as usize, &mut (*mrvl).flags); smp_mb__after_atomic(); wake_up_bit(&mut (*mrvl).flags, State::FwReqPending as usize); }
    }
    kfree_skb(skb); ret
}

unsafe fn mrvl_recv_chip_ver(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    let pkt = (*skb).data as *mut HciMrvlPkt;
    let hu = hci_get_drvdata(hdev); let mrvl = (*hu).priv_ as *mut MrvlData;
    let version = le16_to_cpu((*pkt).lhs); let mut ret = 0;
    if ((*pkt).lhs ^ (*pkt).rhs) != 0xffff { bt_dev_err(hdev, "Corrupted mrvl header"); mrvl_send_ack(hu, MRVL_NAK); ret = -EINVAL; }
    else { mrvl_send_ack(hu, MRVL_ACK); if !test_bit(State::ChipVerPending as usize, &(*mrvl).flags) { bt_dev_err(hdev, "Received unexpected chip version"); } else { (*mrvl).id = version as u8; (*mrvl).rev = (version >> 8) as u8; bt_dev_info(hdev, "Controller id = %x, rev = %x", (*mrvl).id, (*mrvl).rev); clear_bit(State::ChipVerPending as usize, &mut (*mrvl).flags); smp_mb__after_atomic(); wake_up_bit(&mut (*mrvl).flags, State::ChipVerPending as usize); } }
    kfree_skb(skb); ret
}

const HCI_RECV_CHIP_VER: h4_recv_pkt = h4_recv_pkt { typ: HCI_CHIP_VER_PKT, hlen: HCI_MRVL_PKT_SIZE, loff: 0, lsize: 0, maxlen: HCI_MRVL_PKT_SIZE, recv: Some(mrvl_recv_chip_ver) };
const HCI_RECV_FW_REQ: h4_recv_pkt = h4_recv_pkt { typ: HCI_FW_REQ_PKT, hlen: HCI_MRVL_PKT_SIZE, loff: 0, lsize: 0, maxlen: HCI_MRVL_PKT_SIZE, recv: Some(mrvl_recv_fw_req) };
static mrvl_recv_pkts: [h4_recv_pkt; 5] = [
    h4_recv_pkt { typ: H4_RECV_ACL, recv: Some(hci_recv_frame), ..H4_RECV_DEFAULT },
    h4_recv_pkt { typ: H4_RECV_SCO, recv: Some(hci_recv_frame), ..H4_RECV_DEFAULT },
    h4_recv_pkt { typ: H4_RECV_EVENT, recv: Some(hci_recv_frame), ..H4_RECV_DEFAULT },
    HCI_RECV_FW_REQ, HCI_RECV_CHIP_VER,
];

unsafe fn mrvl_recv(hu: *mut hci_uart, data: *const c_void, count: c_int) -> c_int {
    let mrvl = (*hu).priv_ as *mut MrvlData;
    if !test_bit(HCI_UART_REGISTERED, &(*hu).flags) { return -EUNATCH; }
    if !test_bit(State::FwReqPending as usize, &(*mrvl).flags) && !test_bit(State::FwLoaded as usize, &(*mrvl).flags) { return count; }
    (*mrvl).rx_skb = h4_recv_buf(hu, (*mrvl).rx_skb, data, count, mrvl_recv_pkts.as_ptr(), mrvl_recv_pkts.len());
    if IS_ERR((*mrvl).rx_skb) { let err = PTR_ERR((*mrvl).rx_skb); bt_dev_err((*hu).hdev, "Frame reassembly failed (%d)", err); (*mrvl).rx_skb = core::ptr::null_mut(); return err; }
    count
}

unsafe fn mrvl_load_firmware(hdev: *mut hci_dev, name: *const c_char) -> c_int {
    let hu = hci_get_drvdata(hdev); let mrvl = (*hu).priv_ as *mut MrvlData;
    let mut fw: *const firmware = core::ptr::null(); let err = request_firmware(&mut fw, name, &mut (*hdev).dev);
    if err < 0 { bt_dev_err(hdev, "Failed to load firmware file %s", name); return err; }
    let mut fw_ptr = (*fw).data; let fw_max = (*fw).data.add((*fw).size); bt_dev_info(hdev, "Loading %s", name);
    set_bit(State::FwReqPending as usize, &mut (*mrvl).flags);
    while fw_ptr <= fw_max {
        let wait = wait_on_bit_timeout(&mut (*mrvl).flags, State::FwReqPending as usize, TASK_INTERRUPTIBLE, msecs_to_jiffies(2000));
        if wait == 1 { bt_dev_err(hdev, "Firmware load interrupted"); release_firmware(fw); return -EINTR; }
        if wait != 0 { bt_dev_err(hdev, "Firmware request timeout"); release_firmware(fw); return -ETIMEDOUT; }
        bt_dev_dbg(hdev, "Firmware request, expecting %d bytes", (*mrvl).tx_len);
        if fw_ptr == fw_max { if (*mrvl).tx_len == 0 { bt_dev_info(hdev, "Firmware loading complete"); } else { bt_dev_err(hdev, "Firmware loading failure"); release_firmware(fw); return -EINVAL; } break; }
        if fw_ptr.add((*mrvl).tx_len as usize) > fw_max { (*mrvl).tx_len = fw_max.offset_from(fw_ptr) as c_uint; }
        let skb = bt_skb_alloc((*mrvl).tx_len as usize, GFP_KERNEL); if skb.is_null() { release_firmware(fw); return -ENOMEM; }
        bt_cb(skb).pkt_type = MRVL_RAW_DATA; skb_put_data(skb, fw_ptr as *const c_void, (*mrvl).tx_len as usize); fw_ptr = fw_ptr.add((*mrvl).tx_len as usize);
        set_bit(State::FwReqPending as usize, &mut (*mrvl).flags); skb_queue_tail(&mut (*mrvl).rawq, skb); hci_uart_tx_wakeup(hu);
    }
    release_firmware(fw); 0
}

unsafe fn mrvl_setup(hu: *mut hci_uart) -> c_int {
    let mrvl = (*hu).priv_ as *mut MrvlData; hci_uart_set_flow_control(hu, true);
    if mrvl_load_firmware((*hu).hdev, b"mrvl/helper_uart_3000000.bin\0".as_ptr() as *const c_char) != 0 { bt_dev_err((*hu).hdev, "Unable to download firmware helper"); return -EINVAL; }
    hci_uart_wait_until_sent(hu); if !(*hu).serdev.is_null() { serdev_device_set_baudrate((*hu).serdev, (*hu).oper_speed); } else { hci_uart_set_baudrate(hu, (*hu).oper_speed); }
    hci_uart_set_flow_control(hu, false); let err = mrvl_load_firmware((*hu).hdev, b"mrvl/uart8897_bt.bin\0".as_ptr() as *const c_char); if err != 0 { return err; }
    set_bit(State::FwLoaded as usize, &mut (*mrvl).flags); 0
}

unsafe fn mrvl_set_baudrate(hu: *mut hci_uart, speed: c_uint) -> c_int {
    let mut speed_le = cpu_to_le32(speed); let err = serdev_device_wait_for_cts((*hu).serdev, true, 10000); if err != 0 { return err; }
    set_bit(State::FwLoaded as usize, &mut (*((*hu).priv_ as *mut MrvlData)).flags);
    let err = __hci_cmd_sync_status((*hu).hdev, MRVL_SET_BAUDRATE, core::mem::size_of_val(&speed_le), &mut speed_le as *mut _ as *mut c_void, HCI_INIT_TIMEOUT); if err != 0 { return err; }
    serdev_device_set_baudrate((*hu).serdev, speed); set_bit(HCI_UART_VND_DETECT, &mut (*(*hu).hdev).dev_flags); 0
}

// Protocol objects, serdev callbacks, and module init/exit are exported through the
// surrounding kernel bindings with the same names and callback ordering as the C source.
unsafe fn mrvl_init() -> c_int { serdev_device_driver_register(&mut mrvl_serdev_driver); hci_uart_register_proto(&mrvl_proto_8897) }
unsafe fn mrvl_deinit() -> c_int { serdev_device_driver_unregister(&mut mrvl_serdev_driver); hci_uart_unregister_proto(&mrvl_proto_8897) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
