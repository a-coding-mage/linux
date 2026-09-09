// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 *  Digianswer Bluetooth USB driver
 *
 *  Copyright (C) 2004-2007  Marcel Holtmann <marcel@holtmann.org>
 */

// Kernel dependencies supplied by the surrounding Rust translation.

const VERSION: &str = "0.11";

static bpa10x_table: [usb_device_id; 2] = [
    usb_device_id { match_flags: USB_DEVICE_ID_MATCH_DEVICE, idVendor: 0x08fd, idProduct: 0x0002 },
    usb_device_id { match_flags: 0, idVendor: 0, idProduct: 0 },
];

#[repr(C)]
struct bpa10x_data {
    hdev: *mut hci_dev,
    udev: *mut usb_device,
    tx_anchor: usb_anchor,
    rx_anchor: usb_anchor,
    rx_skb: [*mut sk_buff; 2],
    hu: hci_uart,
}

unsafe extern "C" fn bpa10x_tx_complete(urb: *mut urb) {
    let skb = (*urb).context as *mut sk_buff;
    let hdev = (*skb).dev as *mut hci_dev;

    BT_DBG!("%s urb %p status %d count %d", (*hdev).name, urb, (*urb).status, (*urb).actual_length);

    if !test_bit(HCI_RUNNING, &(*hdev).flags) {
        kfree((*urb).setup_packet);
        kfree_skb(skb);
        return;
    }

    if (*urb).status == 0 {
        (*hdev).stat.byte_tx += (*urb).transfer_buffer_length;
    } else {
        (*hdev).stat.err_tx += 1;
    }

    kfree((*urb).setup_packet);
    kfree_skb(skb);
}

const HCI_VENDOR_HDR_SIZE: usize = 5;

static bpa10x_recv_pkts: [h4_recv_pkt; 4] = [
    h4_recv_pkt { type_: H4_RECV_ACL, recv: Some(hci_recv_frame) },
    h4_recv_pkt { type_: H4_RECV_SCO, recv: Some(hci_recv_frame) },
    h4_recv_pkt { type_: H4_RECV_EVENT, recv: Some(hci_recv_frame) },
    h4_recv_pkt { type_: HCI_VENDOR_PKT, hlen: HCI_VENDOR_HDR_SIZE, loff: 3, lsize: 2, maxlen: HCI_MAX_FRAME_SIZE, recv: Some(hci_recv_diag) },
];

unsafe extern "C" fn bpa10x_rx_complete(urb: *mut urb) {
    let hdev = (*urb).context as *mut hci_dev;
    let data = hci_get_drvdata(hdev) as *mut bpa10x_data;
    let mut err: c_int;

    BT_DBG!("%s urb %p status %d count %d", (*hdev).name, urb, (*urb).status, (*urb).actual_length);
    if !test_bit(HCI_RUNNING, &(*hdev).flags) { return; }

    if (*urb).status == 0 {
        let idx = usb_pipebulk((*urb).pipe) as usize;
        (*data).rx_skb[idx] = h4_recv_buf(&mut (*data).hu, (*data).rx_skb[idx], (*urb).transfer_buffer,
                                           (*urb).actual_length, bpa10x_recv_pkts.as_ptr(), bpa10x_recv_pkts.len());
        if IS_ERR((*data).rx_skb[idx]) {
            bt_dev_err!(hdev, "corrupted event packet");
            (*hdev).stat.err_rx += 1;
            (*data).rx_skb[idx] = core::ptr::null_mut();
        }
    }
    usb_anchor_urb(urb, &mut (*data).rx_anchor);
    err = usb_submit_urb(urb, GFP_ATOMIC);
    if err < 0 { bt_dev_err!(hdev, "urb %p failed to resubmit (%d)", urb, -err); usb_unanchor_urb(urb); }
}

unsafe fn bpa10x_submit_intr_urb(hdev: *mut hci_dev) -> c_int {
    let data = hci_get_drvdata(hdev) as *mut bpa10x_data;
    let urb = usb_alloc_urb(0, GFP_KERNEL);
    if urb.is_null() { return -ENOMEM; }
    let buf = kmalloc(16, GFP_KERNEL);
    if buf.is_null() { usb_free_urb(urb); return -ENOMEM; }
    let pipe = usb_rcvintpipe((*data).udev, 0x81);
    usb_fill_int_urb(urb, (*data).udev, pipe, buf, 16, Some(bpa10x_rx_complete), hdev as *mut c_void, 1);
    (*urb).transfer_flags |= URB_FREE_BUFFER;
    usb_anchor_urb(urb, &mut (*data).rx_anchor);
    let err = usb_submit_urb(urb, GFP_KERNEL);
    if err < 0 { bt_dev_err!(hdev, "urb %p submission failed (%d)", urb, -err); usb_unanchor_urb(urb); }
    usb_free_urb(urb); err
}

unsafe fn bpa10x_submit_bulk_urb(hdev: *mut hci_dev) -> c_int {
    let data = hci_get_drvdata(hdev) as *mut bpa10x_data;
    let urb = usb_alloc_urb(0, GFP_KERNEL);
    if urb.is_null() { return -ENOMEM; }
    let buf = kmalloc(64, GFP_KERNEL);
    if buf.is_null() { usb_free_urb(urb); return -ENOMEM; }
    let pipe = usb_rcvbulkpipe((*data).udev, 0x82);
    usb_fill_bulk_urb(urb, (*data).udev, pipe, buf, 64, Some(bpa10x_rx_complete), hdev as *mut c_void);
    (*urb).transfer_flags |= URB_FREE_BUFFER;
    usb_anchor_urb(urb, &mut (*data).rx_anchor);
    let err = usb_submit_urb(urb, GFP_KERNEL);
    if err < 0 { bt_dev_err!(hdev, "urb %p submission failed (%d)", urb, -err); usb_unanchor_urb(urb); }
    usb_free_urb(urb); err
}

unsafe extern "C" fn bpa10x_open(hdev: *mut hci_dev) -> c_int {
    let data = hci_get_drvdata(hdev) as *mut bpa10x_data;
    let err = bpa10x_submit_intr_urb(hdev);
    if err < 0 { usb_kill_anchored_urbs(&mut (*data).rx_anchor); return err; }
    let err = bpa10x_submit_bulk_urb(hdev);
    if err < 0 { usb_kill_anchored_urbs(&mut (*data).rx_anchor); return err; }
    0
}

unsafe extern "C" fn bpa10x_close(hdev: *mut hci_dev) -> c_int {
    let data = hci_get_drvdata(hdev) as *mut bpa10x_data;
    usb_kill_anchored_urbs(&mut (*data).rx_anchor); 0
}

unsafe extern "C" fn bpa10x_flush(hdev: *mut hci_dev) -> c_int {
    let data = hci_get_drvdata(hdev) as *mut bpa10x_data;
    usb_kill_anchored_urbs(&mut (*data).tx_anchor); 0
}

unsafe extern "C" fn bpa10x_setup(hdev: *mut hci_dev) -> c_int {
    let req: [u8; 1] = [0x07];
    let skb = __hci_cmd_sync(hdev, 0xfc0e, req.len(), req.as_ptr(), HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { return PTR_ERR(skb); }
    if (*skb).len > 1 {
        let len = (*skb).len - 1;
        bt_dev_info!(hdev, "%.*s", len, ((*skb).data.add(1)) as *const c_char);
        hci_set_fw_info(hdev, "%.*s", len, (*skb).data.add(1));
    }
    kfree_skb(skb); 0
}

unsafe extern "C" fn bpa10x_send_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    let data = hci_get_drvdata(hdev) as *mut bpa10x_data;
    let urb = usb_alloc_urb(0, GFP_KERNEL);
    if urb.is_null() { return -ENOMEM; }
    (*skb).dev = hdev as *mut c_void;
    *(skb_push(skb, 1) as *mut u8) = hci_skb_pkt_type(skb);
    let pipe;
    match hci_skb_pkt_type(skb) {
        HCI_COMMAND_PKT => {
            let dr = kmalloc(core::mem::size_of::<usb_ctrlrequest>(), GFP_KERNEL) as *mut usb_ctrlrequest;
            if dr.is_null() { usb_free_urb(urb); return -ENOMEM; }
            (*dr).bRequestType = USB_TYPE_VENDOR; (*dr).bRequest = 0; (*dr).wIndex = 0; (*dr).wValue = 0;
            (*dr).wLength = __cpu_to_le16((*skb).len as u16);
            pipe = usb_sndctrlpipe((*data).udev, 0x00);
            usb_fill_control_urb(urb, (*data).udev, pipe, dr as *mut c_void, (*skb).data, (*skb).len, Some(bpa10x_tx_complete), skb as *mut c_void);
            (*hdev).stat.cmd_tx += 1;
        }
        HCI_ACLDATA_PKT | HCI_SCODATA_PKT => {
            pipe = usb_sndbulkpipe((*data).udev, 0x02);
            usb_fill_bulk_urb(urb, (*data).udev, pipe, (*skb).data, (*skb).len, Some(bpa10x_tx_complete), skb as *mut c_void);
            if hci_skb_pkt_type(skb) == HCI_ACLDATA_PKT { (*hdev).stat.acl_tx += 1; } else { (*hdev).stat.sco_tx += 1; }
        }
        _ => { usb_free_urb(urb); return -EILSEQ; }
    }
    usb_anchor_urb(urb, &mut (*data).tx_anchor);
    let err = usb_submit_urb(urb, GFP_KERNEL);
    if err < 0 { bt_dev_err!(hdev, "urb %p submission failed", urb); kfree((*urb).setup_packet); usb_unanchor_urb(urb); }
    usb_free_urb(urb); err
}

unsafe extern "C" fn bpa10x_set_diag(hdev: *mut hci_dev, enable: bool) -> c_int {
    let req = [0x00, enable as u8];
    if !test_bit(HCI_RUNNING, &(*hdev).flags) { return -ENETDOWN; }
    let skb = __hci_cmd_sync(hdev, 0xfc0e, req.len(), req.as_ptr(), HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { return PTR_ERR(skb); }
    kfree_skb(skb); 0
}

unsafe extern "C" fn bpa10x_probe(intf: *mut usb_interface, _id: *const usb_device_id) -> c_int {
    if (*(*intf).cur_altsetting).desc.bInterfaceNumber != 0 { return -ENODEV; }
    let data = devm_kzalloc(&mut (*intf).dev, core::mem::size_of::<bpa10x_data>(), GFP_KERNEL) as *mut bpa10x_data;
    if data.is_null() { return -ENOMEM; }
    (*data).udev = interface_to_usbdev(intf);
    init_usb_anchor(&mut (*data).tx_anchor); init_usb_anchor(&mut (*data).rx_anchor);
    let hdev = hci_alloc_dev();
    if hdev.is_null() { return -ENOMEM; }
    (*hdev).bus = HCI_USB; hci_set_drvdata(hdev, data as *mut c_void);
    (*data).hdev = hdev; (*data).hu.hdev = hdev; SET_HCIDEV_DEV(hdev, &mut (*intf).dev);
    (*hdev).open = Some(bpa10x_open); (*hdev).close = Some(bpa10x_close); (*hdev).flush = Some(bpa10x_flush);
    (*hdev).setup = Some(bpa10x_setup); (*hdev).send = Some(bpa10x_send_frame); (*hdev).set_diag = Some(bpa10x_set_diag);
    hci_set_quirk(hdev, HCI_QUIRK_RESET_ON_CLOSE);
    let err = hci_register_dev(hdev);
    if err < 0 { hci_free_dev(hdev); return err; }
    usb_set_intfdata(intf, data as *mut c_void); 0
}

unsafe extern "C" fn bpa10x_disconnect(intf: *mut usb_interface) {
    let data = usb_get_intfdata(intf) as *mut bpa10x_data;
    if data.is_null() { return; }
    usb_set_intfdata(intf, core::ptr::null_mut());
    hci_unregister_dev((*data).hdev); hci_free_dev((*data).hdev);
    kfree_skb((*data).rx_skb[0]); kfree_skb((*data).rx_skb[1]);
}

static bpa10x_driver: usb_driver = usb_driver {
    name: "bpa10x", probe: Some(bpa10x_probe), disconnect: Some(bpa10x_disconnect), id_table: bpa10x_table.as_ptr(),
    disable_hub_initiated_lpm: 1,
};

module_usb_driver!(bpa10x_driver);
MODULE_AUTHOR!("Marcel Holtmann <marcel@holtmann.org>");
MODULE_DESCRIPTION!(concat!("Digianswer Bluetooth USB driver ver ", VERSION));
MODULE_VERSION!(VERSION);
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
