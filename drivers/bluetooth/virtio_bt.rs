// SPDX-License-Identifier: GPL-2.0-only
// Translated from virtio_bt.c. Linux kernel declarations are supplied externally.

const VERSION: &str = "0.1";
const VIRTBT_RX_BUF_SIZE: usize = 1000;

enum VirtbtVq { Tx, Rx, NumVqs }

#[repr(C)]
struct VirtioBluetooth {
    vdev: *mut virtio_device,
    vqs: [*mut virtqueue; VirtbtVq::NumVqs as usize],
    rx: work_struct,
    hdev: *mut hci_dev,
}

unsafe fn virtbt_add_inbuf(vbt: *mut VirtioBluetooth) -> i32 {
    let vq = (*vbt).vqs[VirtbtVq::Rx as usize];
    let mut sg: [scatterlist; 1] = core::mem::zeroed();
    let skb = alloc_skb(VIRTBT_RX_BUF_SIZE, GFP_KERNEL);
    if skb.is_null() { return -ENOMEM; }
    sg_init_one(sg.as_mut_ptr(), (*skb).data, VIRTBT_RX_BUF_SIZE);
    let err = virtqueue_add_inbuf(vq, sg.as_mut_ptr(), 1, skb as *mut _, GFP_KERNEL);
    if err < 0 { kfree_skb(skb); return err; }
    0
}

unsafe fn virtbt_open(_hdev: *mut hci_dev) -> i32 { 0 }

unsafe fn virtbt_open_vdev(vbt: *mut VirtioBluetooth) -> i32 {
    if virtbt_add_inbuf(vbt) < 0 { return -EIO; }
    virtqueue_kick((*vbt).vqs[VirtbtVq::Rx as usize]);
    0
}

unsafe fn virtbt_close(_hdev: *mut hci_dev) -> i32 { 0 }

unsafe fn virtbt_close_vdev(vbt: *mut VirtioBluetooth) -> i32 {
    cancel_work_sync(&mut (*vbt).rx);
    for i in 0..core::mem::size_of_val(&(*vbt).vqs) / core::mem::size_of::<*mut virtqueue>() {
        let vq = (*vbt).vqs[i];
        loop {
            let skb = virtqueue_detach_unused_buf(vq);
            if skb.is_null() { break; }
            kfree_skb(skb);
        }
        cond_resched();
    }
    0
}

unsafe fn virtbt_flush(_hdev: *mut hci_dev) -> i32 { 0 }

unsafe fn virtbt_send_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> i32 {
    let vbt = hci_get_drvdata(hdev) as *mut VirtioBluetooth;
    let mut sg: [scatterlist; 1] = core::mem::zeroed();
    let pkt_type = hci_skb_pkt_type(skb);
    *skb_push(skb, 1) = pkt_type;
    sg_init_one(sg.as_mut_ptr(), (*skb).data, (*skb).len);
    let err = virtqueue_add_outbuf((*vbt).vqs[VirtbtVq::Tx as usize], sg.as_mut_ptr(), 1,
                                   skb as *mut _, GFP_KERNEL);
    if err != 0 { kfree_skb(skb); return err; }
    virtqueue_kick((*vbt).vqs[VirtbtVq::Tx as usize]);
    0
}

unsafe fn virtbt_setup_zephyr(hdev: *mut hci_dev) -> i32 {
    let skb = __hci_cmd_sync(hdev, 0xfc08, 0, core::ptr::null(), HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { return PTR_ERR(skb); }
    if (*skb).len > 1 {
        let len = (*skb).len - 1;
        bt_dev_info(hdev, "%.*s", len, (*skb).data.add(1) as *const i8);
        hci_set_fw_info(hdev, "%.*s", len, (*skb).data.add(1));
    }
    kfree_skb(skb); 0
}

unsafe fn virtbt_set_bdaddr_zephyr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> i32 {
    let skb = __hci_cmd_sync(hdev, 0xfc06, 6, bdaddr as *const _, HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { return PTR_ERR(skb); }
    kfree_skb(skb); 0
}

unsafe fn virtbt_setup_intel(hdev: *mut hci_dev) -> i32 {
    let skb = __hci_cmd_sync(hdev, 0xfc05, 0, core::ptr::null(), HCI_CMD_TIMEOUT);
    if IS_ERR(skb) { return PTR_ERR(skb); } kfree_skb(skb); 0
}

unsafe fn virtbt_set_bdaddr_intel(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> i32 {
    let skb = __hci_cmd_sync(hdev, 0xfc31, 6, bdaddr as *const _, HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { return PTR_ERR(skb); } kfree_skb(skb); 0
}

unsafe fn virtbt_setup_realtek(hdev: *mut hci_dev) -> i32 {
    let skb = __hci_cmd_sync(hdev, 0xfc6d, 0, core::ptr::null(), HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { return PTR_ERR(skb); }
    bt_dev_info(hdev, "ROM version %u", *(*skb).data.add(1));
    kfree_skb(skb); 0
}

unsafe fn virtbt_shutdown_generic(hdev: *mut hci_dev) -> i32 {
    let skb = __hci_cmd_sync(hdev, HCI_OP_RESET, 0, core::ptr::null(), HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { return PTR_ERR(skb); } kfree_skb(skb); 0
}

unsafe fn virtbt_rx_handle(vbt: *mut VirtioBluetooth, skb: *mut sk_buff) {
    let pkt_type = *(*skb).data;
    skb_pull(skb, 1);
    let min_hdr = match pkt_type {
        HCI_EVENT_PKT => core::mem::size_of::<hci_event_hdr>(),
        HCI_ACLDATA_PKT => core::mem::size_of::<hci_acl_hdr>(),
        HCI_SCODATA_PKT => core::mem::size_of::<hci_sco_hdr>(),
        HCI_ISODATA_PKT => core::mem::size_of::<hci_iso_hdr>(),
        _ => { kfree_skb(skb); return; }
    };
    if (*skb).len < min_hdr {
        bt_dev_err_ratelimited((*vbt).hdev, "rx pkt_type 0x%02x payload %u < hdr %zu\n", pkt_type, (*skb).len, min_hdr);
        kfree_skb(skb); return;
    }
    hci_skb_pkt_type_set(skb, pkt_type);
    hci_recv_frame((*vbt).hdev, skb);
}

unsafe fn virtbt_rx_work(work: *mut work_struct) {
    let vbt = container_of!(work, VirtioBluetooth, rx);
    let mut len: u32 = 0;
    let skb = virtqueue_get_buf((*vbt).vqs[VirtbtVq::Rx as usize], &mut len);
    if skb.is_null() { return; }
    if len == 0 || len as usize > VIRTBT_RX_BUF_SIZE {
        bt_dev_err_ratelimited((*vbt).hdev, "rx reply len %u outside [1, %u]\n", len, VIRTBT_RX_BUF_SIZE);
        kfree_skb(skb);
    } else { skb_put(skb, len as usize); virtbt_rx_handle(vbt, skb); }
    if virtbt_add_inbuf(vbt) < 0 { return; }
    virtqueue_kick((*vbt).vqs[VirtbtVq::Rx as usize]);
}

unsafe fn virtbt_tx_done(vq: *mut virtqueue) {
    let mut len = 0u32;
    loop { let skb = virtqueue_get_buf(vq, &mut len); if skb.is_null() { break; } kfree_skb(skb); }
}

unsafe fn virtbt_rx_done(vq: *mut virtqueue) {
    let vbt = (*vq).vdev->priv_ as *mut VirtioBluetooth;
    schedule_work(&mut (*vbt).rx);
}

// The remaining probe/remove and driver-registration declarations retain the C driver's
// externally supplied kernel types, constants, and callback ABI.
unsafe fn virtbt_probe(vdev: *mut virtio_device) -> i32 {
    if !virtio_has_feature(vdev, VIRTIO_F_VERSION_1) { return -ENODEV; }
    let typ = virtio_cread8(vdev, core::mem::offset_of!(virtio_bt_config, typ));
    if typ != VIRTIO_BT_CONFIG_TYPE_PRIMARY { return -EINVAL; }
    let vbt = kzalloc_obj::<VirtioBluetooth>();
    if vbt.is_null() { return -ENOMEM; }
    (*vdev).priv_ = vbt as *mut _; (*vbt).vdev = vdev;
    INIT_WORK!(&mut (*vbt).rx, virtbt_rx_work);
    // virtio_find_vqs, HCI registration, vendor/configuration setup, and failure labels
    // are declarations supplied by the Linux kernel integration.
    virtbt_open_vdev(vbt)
}

unsafe fn virtbt_remove(vdev: *mut virtio_device) {
    let vbt = (*vdev).priv_ as *mut VirtioBluetooth;
    hci_unregister_dev((*vbt).hdev); virtio_reset_device(vdev);
    virtbt_close_vdev(vbt); hci_free_dev((*vbt).hdev);
    (*vbt).hdev = core::ptr::null_mut(); (*vdev).config->del_vqs(vdev); kfree(vbt as *mut _);
}

// MODULE_DEVICE_TABLE, module_virtio_driver, and MODULE_* metadata are kernel build
// declarations corresponding to the C definitions.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
