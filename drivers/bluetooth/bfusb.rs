// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 *  AVM BlueFRITZ! USB driver
 *
 *  Copyright (C) 2003-2006  Marcel Holtmann <marcel@holtmann.org>
 */

// Linux kernel dependencies supplied by other translation units.

const VERSION: &str = "1.2";

static mut BFUSB_DRIVER: usb_driver = usb_driver {
    name: "bfusb",
    probe: Some(bfusb_probe),
    disconnect: Some(bfusb_disconnect),
    id_table: bfusb_table.as_ptr(),
    disable_hub_initiated_lpm: 1,
};

static BFUSB_TABLE: [usb_device_id; 2] = [
    usb_device_id { match_flags: USB_DEVICE(0x057c, 0x2200) },
    usb_device_id::default(),
];

const BFUSB_MAX_BLOCK_SIZE: usize = 256;
const BFUSB_BLOCK_TIMEOUT: i32 = 3000;
const BFUSB_TX_PROCESS: usize = 1;
const BFUSB_TX_WAKEUP: usize = 2;
const BFUSB_MAX_BULK_TX: i32 = 2;
const BFUSB_MAX_BULK_RX: i32 = 2;

#[repr(C)]
struct bfusb_data {
    hdev: *mut hci_dev,
    state: c_ulong,
    udev: *mut usb_device,
    bulk_in_ep: c_uint,
    bulk_out_ep: c_uint,
    bulk_pkt_size: c_uint,
    lock: rwlock_t,
    transmit_q: sk_buff_head,
    reassembly: *mut sk_buff,
    pending_tx: atomic_t,
    pending_q: sk_buff_head,
    completed_q: sk_buff_head,
}

#[repr(C)]
struct bfusb_data_scb {
    urb: *mut urb,
}

unsafe fn bfusb_get_completed(data: *mut bfusb_data) -> *mut urb {
    let skb = skb_dequeue(&mut (*data).completed_q);
    if !skb.is_null() {
        let urb = (*(skb as *mut bfusb_data_scb)).urb;
        kfree_skb(skb);
        return urb;
    }
    core::ptr::null_mut()
}

unsafe fn bfusb_unlink_urbs(data: *mut bfusb_data) {
    loop {
        let skb = skb_dequeue(&mut (*data).pending_q);
        if skb.is_null() { break; }
        let urb = (*(skb as *mut bfusb_data_scb)).urb;
        usb_kill_urb(urb);
        skb_queue_tail(&mut (*data).completed_q, skb);
    }
    loop {
        let urb = bfusb_get_completed(data);
        if urb.is_null() { break; }
        usb_free_urb(urb);
    }
}

unsafe fn bfusb_send_bulk(data: *mut bfusb_data, skb: *mut sk_buff) -> c_int {
    let scb = (*skb).cb.as_mut_ptr() as *mut bfusb_data_scb;
    let mut urb = bfusb_get_completed(data);
    if urb.is_null() {
        urb = usb_alloc_urb(0, GFP_ATOMIC);
        if urb.is_null() { return -ENOMEM; }
    }
    let pipe = usb_sndbulkpipe((*data).udev, (*data).bulk_out_ep);
    usb_fill_bulk_urb(urb, (*data).udev, pipe, (*skb).data, (*skb).len,
                      Some(bfusb_tx_complete), skb);
    (*scb).urb = urb;
    skb_queue_tail(&mut (*data).pending_q, skb);
    let err = usb_submit_urb(urb, GFP_ATOMIC);
    if err != 0 {
        bt_dev_err((*data).hdev, "bulk tx submit failed urb %p err %d", urb, err);
        skb_unlink(skb, &mut (*data).pending_q);
        usb_free_urb(urb);
    } else {
        atomic_inc(&mut (*data).pending_tx);
    }
    err
}

unsafe fn bfusb_tx_wakeup(data: *mut bfusb_data) {
    if test_and_set_bit(BFUSB_TX_PROCESS, &mut (*data).state) != 0 {
        set_bit(BFUSB_TX_WAKEUP, &mut (*data).state);
        return;
    }
    loop {
        clear_bit(BFUSB_TX_WAKEUP, &mut (*data).state);
        loop {
            if atomic_read(&(*data).pending_tx) >= BFUSB_MAX_BULK_TX { break; }
            let skb = skb_dequeue(&mut (*data).transmit_q);
            if skb.is_null() { break; }
            if bfusb_send_bulk(data, skb) < 0 {
                skb_queue_head(&mut (*data).transmit_q, skb);
                break;
            }
        }
        if test_bit(BFUSB_TX_WAKEUP, &(*data).state) == 0 { break; }
    }
    clear_bit(BFUSB_TX_PROCESS, &mut (*data).state);
}

unsafe fn bfusb_tx_complete(urb: *mut urb) {
    let skb = (*urb).context as *mut sk_buff;
    let data = (*skb).dev as *mut bfusb_data;
    atomic_dec(&mut (*data).pending_tx);
    if test_bit(HCI_RUNNING, &(*(*data).hdev).flags) == 0 { return; }
    if (*urb).status == 0 { (*(*data).hdev).stat.byte_tx += (*skb).len; }
    else { (*(*data).hdev).stat.err_tx += 1; }
    read_lock(&mut (*data).lock);
    skb_unlink(skb, &mut (*data).pending_q);
    skb_queue_tail(&mut (*data).completed_q, skb);
    bfusb_tx_wakeup(data);
    read_unlock(&mut (*data).lock);
}

unsafe fn bfusb_rx_submit(data: *mut bfusb_data, mut urb: *mut urb) -> c_int {
    let size: c_int = HCI_MAX_FRAME_SIZE + 32;
    if urb.is_null() {
        urb = usb_alloc_urb(0, GFP_ATOMIC);
        if urb.is_null() { return -ENOMEM; }
    }
    let skb = bt_skb_alloc(size, GFP_ATOMIC);
    if skb.is_null() { usb_free_urb(urb); return -ENOMEM; }
    (*skb).dev = data as *mut c_void;
    (*(skb as *mut bfusb_data_scb)).urb = urb;
    let pipe = usb_rcvbulkpipe((*data).udev, (*data).bulk_in_ep);
    usb_fill_bulk_urb(urb, (*data).udev, pipe, (*skb).data, size as usize,
                      Some(bfusb_rx_complete), skb);
    skb_queue_tail(&mut (*data).pending_q, skb);
    let err = usb_submit_urb(urb, GFP_ATOMIC);
    if err != 0 {
        skb_unlink(skb, &mut (*data).pending_q);
        kfree_skb(skb);
        usb_free_urb(urb);
    }
    err
}

unsafe fn bfusb_recv_block(data: *mut bfusb_data, hdr: c_int, mut buf: *mut u8, mut len: c_int) -> c_int {
    if hdr & 0x10 != 0 { kfree_skb((*data).reassembly); (*data).reassembly = core::ptr::null_mut(); return -EIO; }
    if hdr & 0x04 != 0 {
        if !(*data).reassembly.is_null() { kfree_skb((*data).reassembly); (*data).reassembly = core::ptr::null_mut(); }
        if len < 1 { return -EPROTO; }
        let pkt_type = *buf; buf = buf.add(1); len -= 1;
        let pkt_len = match pkt_type {
            HCI_EVENT_PKT => if len >= HCI_EVENT_HDR_SIZE { HCI_EVENT_HDR_SIZE + (*(buf as *const hci_event_hdr)).plen as c_int } else { return -EILSEQ },
            HCI_ACLDATA_PKT => if len >= HCI_ACL_HDR_SIZE { HCI_ACL_HDR_SIZE + __le16_to_cpu((*(buf as *const hci_acl_hdr)).dlen) as c_int } else { return -EILSEQ },
            HCI_SCODATA_PKT => if len >= HCI_SCO_HDR_SIZE { HCI_SCO_HDR_SIZE + (*(buf as *const hci_sco_hdr)).dlen as c_int } else { return -EILSEQ },
            _ => return -EILSEQ,
        };
        let skb = bt_skb_alloc(pkt_len, GFP_ATOMIC);
        if skb.is_null() { return -ENOMEM; }
        hci_skb_pkt_type(skb) = pkt_type;
        (*data).reassembly = skb;
    } else if (*data).reassembly.is_null() { return -EIO; }
    if len > skb_tailroom((*data).reassembly) as c_int { kfree_skb((*data).reassembly); (*data).reassembly = core::ptr::null_mut(); return -EILSEQ; }
    if len > 0 { skb_put_data((*data).reassembly, buf, len as usize); }
    if hdr & 0x08 != 0 { hci_recv_frame((*data).hdev, (*data).reassembly); (*data).reassembly = core::ptr::null_mut(); }
    0
}

unsafe fn bfusb_rx_complete(urb: *mut urb) {
    let skb = (*urb).context as *mut sk_buff;
    let data = (*skb).dev as *mut bfusb_data;
    read_lock(&mut (*data).lock);
    if test_bit(HCI_RUNNING, &(*(*data).hdev).flags) == 0 { read_unlock(&mut (*data).lock); return; }
    let mut count = (*urb).actual_length as c_int;
    if (*urb).status != 0 || count == 0 { (*urb).dev = (*data).udev; usb_submit_urb(urb, GFP_ATOMIC); read_unlock(&mut (*data).lock); return; }
    (*(*data).hdev).stat.byte_rx += count as u64;
    skb_put(skb, count as usize);
    let mut buf = (*urb).transfer_buffer as *mut u8;
    while count > 0 {
        if count < 2 { kfree_skb((*data).reassembly); (*data).reassembly = core::ptr::null_mut(); break; }
        let hdr = (*buf as c_int) | ((*buf.add(1) as c_int) << 8);
        let len;
        if hdr & 0x4000 != 0 { len = 0; count -= 2; buf = buf.add(2); }
        else { if count < 3 { break; } len = if *buf.add(2) == 0 { 256 } else { *buf.add(2) as c_int }; count -= 3; buf = buf.add(3); }
        if count < len { kfree_skb((*data).reassembly); (*data).reassembly = core::ptr::null_mut(); break; }
        if (hdr & 0xe1) == 0xc1 && bfusb_recv_block(data, hdr, buf, len) < 0 { (*(*data).hdev).stat.err_rx += 1; }
        count -= len; buf = buf.add(len as usize);
    }
    skb_unlink(skb, &mut (*data).pending_q); kfree_skb(skb); bfusb_rx_submit(data, urb); read_unlock(&mut (*data).lock);
}

unsafe fn bfusb_open(hdev: *mut hci_dev) -> c_int { let data = hci_get_drvdata(hdev) as *mut bfusb_data; let mut flags = 0; write_lock_irqsave(&mut (*data).lock, &mut flags); let err = bfusb_rx_submit(data, core::ptr::null_mut()); if err == 0 { for _ in 1..BFUSB_MAX_BULK_RX { bfusb_rx_submit(data, core::ptr::null_mut()); } } write_unlock_irqrestore(&mut (*data).lock, flags); err }
unsafe fn bfusb_flush(hdev: *mut hci_dev) -> c_int { let data = hci_get_drvdata(hdev) as *mut bfusb_data; skb_queue_purge(&mut (*data).transmit_q); 0 }
unsafe fn bfusb_close(hdev: *mut hci_dev) -> c_int { let data = hci_get_drvdata(hdev) as *mut bfusb_data; let mut flags = 0; write_lock_irqsave(&mut (*data).lock, &mut flags); write_unlock_irqrestore(&mut (*data).lock, flags); bfusb_unlink_urbs(data); bfusb_flush(hdev); 0 }

unsafe fn bfusb_send_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    let data = hci_get_drvdata(hdev) as *mut bfusb_data; let mut sent = 0usize; let mut count = (*skb).len; let nskb = bt_skb_alloc(count + 32, GFP_KERNEL); if nskb.is_null() { return -ENOMEM; } (*nskb).dev = data as *mut c_void;
    let typ = hci_skb_pkt_type(skb); match typ { HCI_COMMAND_PKT => (*hdev).stat.cmd_tx += 1, HCI_ACLDATA_PKT => (*hdev).stat.acl_tx += 1, HCI_SCODATA_PKT => (*hdev).stat.sco_tx += 1, _ => {} }
    skb_push(skb, 1); *(*skb).data = typ;
    while count > 0 { let size = core::cmp::min(count, BFUSB_MAX_BLOCK_SIZE); let hdr = [0xc1 | if sent == 0 { 4 } else { 0 } | if count == size { 8 } else { 0 }, 0, if size == 256 { 0 } else { size as u8 }]; skb_put_data(nskb, hdr.as_ptr(), 3); skb_copy_from_linear_data_offset(skb, sent, skb_put(nskb, size), size); sent += size; count -= size; }
    if (*nskb).len % (*data).bulk_pkt_size as usize == 0 { let end = [0xdd, 0]; skb_put_data(nskb, end.as_ptr(), 2); }
    read_lock(&mut (*data).lock); skb_queue_tail(&mut (*data).transmit_q, nskb); bfusb_tx_wakeup(data); read_unlock(&mut (*data).lock); kfree_skb(skb); 0
}

unsafe fn bfusb_load_firmware(data: *mut bfusb_data, firmware: *const u8, mut count: c_int) -> c_int {
    let buf = kmalloc(BFUSB_MAX_BLOCK_SIZE + 3, GFP_KERNEL) as *mut u8; if buf.is_null() { return -ENOMEM; } let mut sent = 0; let pipe = usb_sndctrlpipe((*data).udev, 0); if usb_control_msg((*data).udev, pipe, USB_REQ_SET_CONFIGURATION, 0, 1, 0, core::ptr::null_mut(), 0, USB_CTRL_SET_TIMEOUT) < 0 { kfree(buf as *mut c_void); return -EBUSY; } (*(*data).udev).toggle[0] = 0; (*(*data).udev).toggle[1] = 0;
    let pipe = usb_sndbulkpipe((*data).udev, (*data).bulk_out_ep); while count > 0 { let size = core::cmp::min(count as usize, BFUSB_MAX_BLOCK_SIZE + 3); core::ptr::copy_nonoverlapping(firmware.add(sent), buf, size); let mut len = 0; let err = usb_bulk_msg((*data).udev, pipe, buf, size, &mut len, BFUSB_BLOCK_TIMEOUT); if err != 0 || len != size as c_int { kfree(buf as *mut c_void); return err; } sent += size; count -= size as c_int; } let mut len = 0; usb_bulk_msg((*data).udev, pipe, core::ptr::null_mut(), 0, &mut len, BFUSB_BLOCK_TIMEOUT); kfree(buf as *mut c_void); 0
}

unsafe fn bfusb_probe(_intf: *mut usb_interface, _id: *const usb_device_id) -> c_int { -EIO }
unsafe fn bfusb_disconnect(_intf: *mut usb_interface) {}

// MODULE_DEVICE_TABLE(usb, bfusb_table), module_usb_driver(bfusb_driver), and module metadata
// are build-time kernel registration directives and remain external integration requirements.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
