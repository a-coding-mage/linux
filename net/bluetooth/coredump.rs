// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 Google Corporation
 */

// Kernel dependencies supplied by the surrounding Bluetooth implementation.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum HciDevcoredumpPktType {
    HCI_DEVCOREDUMP_PKT_INIT,
    HCI_DEVCOREDUMP_PKT_SKB,
    HCI_DEVCOREDUMP_PKT_PATTERN,
    HCI_DEVCOREDUMP_PKT_COMPLETE,
    HCI_DEVCOREDUMP_PKT_ABORT,
}

#[repr(C)]
pub struct HciDevcoredumpSkbCb {
    pub pkt_type: u16,
}

#[repr(C, packed)]
pub struct HciDevcoredumpSkbPattern {
    pub pattern: u8,
    pub len: u32,
}

unsafe fn hci_dmp_cb(skb: *mut sk_buff) -> *mut HciDevcoredumpSkbCb {
    (*skb).cb.as_mut_ptr() as *mut HciDevcoredumpSkbCb
}

unsafe fn hci_devcd_update_hdr_state(buf: *mut i8, size: usize, state: i32) -> i32 {
    if buf.is_null() {
        return 0;
    }
    let len = scnprintf(buf, size, b"Bluetooth devcoredump\nState: %d\n\0".as_ptr() as *const i8, state);
    len + 1
}

unsafe fn hci_devcd_update_state(hdev: *mut hci_dev, state: i32) -> i32 {
    bt_dev_dbg(hdev, b"Updating devcoredump state from %s to %s.\0".as_ptr() as *const i8,
               hci_devcd_state_name((*hdev).dump.state), hci_devcd_state_name(state));
    (*hdev).dump.state = state;
    hci_devcd_update_hdr_state((*hdev).dump.head, (*hdev).dump.alloc_size, state)
}

unsafe fn hci_devcd_mkheader(hdev: *mut hci_dev, skb: *mut sk_buff) -> i32 {
    let mut hdr = [0i8; 80];
    let hdr_len = hci_devcd_update_hdr_state(hdr.as_mut_ptr(), hdr.len(), HCI_DEVCOREDUMP_IDLE);
    skb_put_data(skb, hdr.as_ptr() as *const _, hdr_len as usize);
    if let Some(dmp_hdr) = (*hdev).dump.dmp_hdr {
        dmp_hdr(hdev, skb);
    }
    skb_put_data(skb, HCI_DEVCD_HDR_END_MARKER.as_ptr() as *const _, strlen(HCI_DEVCD_HDR_END_MARKER));
    (*skb).len as i32
}

unsafe fn hci_devcd_notify(hdev: *mut hci_dev, state: i32) {
    if let Some(notify_change) = (*hdev).dump.notify_change {
        notify_change(hdev, state);
    }
}

pub unsafe fn hci_devcd_reset(hdev: *mut hci_dev) {
    (*hdev).dump.head = core::ptr::null_mut();
    (*hdev).dump.tail = core::ptr::null_mut();
    (*hdev).dump.alloc_size = 0;
    hci_devcd_update_state(hdev, HCI_DEVCOREDUMP_IDLE);
    cancel_delayed_work(&mut (*hdev).dump.dump_timeout);
    skb_queue_purge(&mut (*hdev).dump.dump_q);
}

unsafe fn hci_devcd_free(hdev: *mut hci_dev) {
    vfree((*hdev).dump.head as *mut _);
    hci_devcd_reset(hdev);
}

unsafe fn hci_devcd_alloc(hdev: *mut hci_dev, size: u32) -> i32 {
    (*hdev).dump.head = vmalloc(size as usize) as *mut i8;
    if (*hdev).dump.head.is_null() { return -ENOMEM; }
    (*hdev).dump.alloc_size = size as usize;
    (*hdev).dump.tail = (*hdev).dump.head;
    (*hdev).dump.end = (*hdev).dump.head.add(size as usize);
    hci_devcd_update_state(hdev, HCI_DEVCOREDUMP_IDLE);
    0
}

unsafe fn hci_devcd_copy(hdev: *mut hci_dev, buf: *const i8, size: u32) -> bool {
    if (*hdev).dump.tail.add(size as usize) > (*hdev).dump.end { return false; }
    memcpy((*hdev).dump.tail as *mut _, buf as *const _, size as usize);
    (*hdev).dump.tail = (*hdev).dump.tail.add(size as usize);
    true
}

unsafe fn hci_devcd_memset(hdev: *mut hci_dev, pattern: u8, len: u32) -> bool {
    if (*hdev).dump.tail.add(len as usize) > (*hdev).dump.end { return false; }
    memset((*hdev).dump.tail as *mut _, pattern as i32, len as usize);
    (*hdev).dump.tail = (*hdev).dump.tail.add(len as usize);
    true
}

unsafe fn hci_devcd_prepare(hdev: *mut hci_dev, dump_size: u32) -> i32 {
    let skb = alloc_skb(HCI_DEVCD_HDR_SIZE_MAX, GFP_ATOMIC);
    if skb.is_null() { return -ENOMEM; }
    let dump_hdr_size = hci_devcd_mkheader(hdev, skb);
    if hci_devcd_alloc(hdev, (dump_hdr_size as u32).wrapping_add(dump_size)) != 0 {
        kfree_skb(skb); return -ENOMEM;
    }
    if !hci_devcd_copy(hdev, (*skb).data, (*skb).len as u32) {
        bt_dev_err(hdev, b"Failed to insert header\0".as_ptr() as *const i8);
        hci_devcd_free(hdev); kfree_skb(skb); return -ENOMEM;
    }
    kfree_skb(skb); 0
}

unsafe fn hci_devcd_handle_pkt_init(hdev: *mut hci_dev, skb: *mut sk_buff) {
    if (*hdev).dump.state != HCI_DEVCOREDUMP_IDLE { bt_dev_dbg(hdev, b"Unexpected packet (%d) for state %s.\0".as_ptr() as *const i8, (*hci_dmp_cb(skb)).pkt_type, hci_devcd_state_name((*hdev).dump.state)); return; }
    if (*skb).len != core::mem::size_of::<u32>() { bt_dev_dbg(hdev, b"Invalid dump init pkt\0".as_ptr() as *const i8); return; }
    let dump_size = get_unaligned_le32(skb_pull_data(skb, 4));
    if dump_size == 0 { bt_dev_err(hdev, b"Zero size dump init pkt\0".as_ptr() as *const i8); return; }
    if hci_devcd_prepare(hdev, dump_size) != 0 { bt_dev_err(hdev, b"Failed to prepare for dump\0".as_ptr() as *const i8); return; }
    hci_devcd_update_state(hdev, HCI_DEVCOREDUMP_ACTIVE);
    queue_delayed_work((*hdev).workqueue, &mut (*hdev).dump.dump_timeout, (*hdev).dump.timeout);
}

unsafe fn hci_devcd_handle_pkt_skb(hdev: *mut hci_dev, skb: *mut sk_buff) {
    if (*hdev).dump.state != HCI_DEVCOREDUMP_ACTIVE { return; }
    if !hci_devcd_copy(hdev, (*skb).data, (*skb).len as u32) { bt_dev_dbg(hdev, b"Failed to insert skb\0".as_ptr() as *const i8); }
}

unsafe fn hci_devcd_handle_pkt_pattern(hdev: *mut hci_dev, skb: *mut sk_buff) {
    if (*hdev).dump.state != HCI_DEVCOREDUMP_ACTIVE { return; }
    if (*skb).len != core::mem::size_of::<HciDevcoredumpSkbPattern>() { bt_dev_dbg(hdev, b"Invalid pattern skb\0".as_ptr() as *const i8); return; }
    let pattern = skb_pull_data(skb, core::mem::size_of::<HciDevcoredumpSkbPattern>());
    let p = &*(pattern as *const HciDevcoredumpSkbPattern);
    if !hci_devcd_memset(hdev, p.pattern, p.len) { bt_dev_dbg(hdev, b"Failed to set pattern\0".as_ptr() as *const i8); }
}

unsafe fn hci_devcd_dump(hdev: *mut hci_dev) {
    let size = (*hdev).dump.tail.offset_from((*hdev).dump.head) as u32;
    let skb = bt_skb_alloc(size, GFP_ATOMIC);
    if !skb.is_null() { skb_put_data(skb, (*hdev).dump.head as *const _, size as usize); hci_recv_diag(hdev, skb); }
    dev_coredumpv(&mut (*hdev).dev, (*hdev).dump.head as *mut _, size as usize, GFP_KERNEL);
}

unsafe fn hci_devcd_handle_pkt_complete(hdev: *mut hci_dev, _skb: *mut sk_buff) {
    if (*hdev).dump.state != HCI_DEVCOREDUMP_ACTIVE { return; }
    hci_devcd_update_state(hdev, HCI_DEVCOREDUMP_DONE); hci_devcd_dump(hdev);
}

unsafe fn hci_devcd_handle_pkt_abort(hdev: *mut hci_dev, _skb: *mut sk_buff) {
    if (*hdev).dump.state != HCI_DEVCOREDUMP_ACTIVE { return; }
    hci_devcd_update_state(hdev, HCI_DEVCOREDUMP_ABORT); hci_devcd_dump(hdev);
}

pub unsafe fn hci_devcd_rx(work: *mut work_struct) {
    let hdev = container_of(work, core::ptr::addr_of_mut!((*((0 as *mut hci_dev))), dump.dump_rx));
    while let Some(skb) = skb_dequeue(&mut (*hdev).dump.dump_q).as_mut() {
        if (*hdev).dump.state == HCI_DEVCOREDUMP_TIMEOUT { kfree_skb(skb); return; }
        hci_dev_lock(hdev);
        match (*hci_dmp_cb(skb)).pkt_type as i32 {
            HCI_DEVCOREDUMP_PKT_INIT => hci_devcd_handle_pkt_init(hdev, skb),
            HCI_DEVCOREDUMP_PKT_SKB => hci_devcd_handle_pkt_skb(hdev, skb),
            HCI_DEVCOREDUMP_PKT_PATTERN => hci_devcd_handle_pkt_pattern(hdev, skb),
            HCI_DEVCOREDUMP_PKT_COMPLETE => hci_devcd_handle_pkt_complete(hdev, skb),
            HCI_DEVCOREDUMP_PKT_ABORT => hci_devcd_handle_pkt_abort(hdev, skb),
            _ => bt_dev_dbg(hdev, b"Unknown packet (%d) for state %s.\0".as_ptr() as *const i8, (*hci_dmp_cb(skb)).pkt_type, hci_devcd_state_name((*hdev).dump.state)),
        }
        hci_dev_unlock(hdev); kfree_skb(skb);
        hci_dev_lock(hdev);
        if (*hdev).dump.state == HCI_DEVCOREDUMP_DONE || (*hdev).dump.state == HCI_DEVCOREDUMP_ABORT { hci_devcd_reset(hdev); }
        hci_dev_unlock(hdev);
    }
}

pub unsafe fn hci_devcd_timeout(work: *mut work_struct) {
    let hdev = container_of(work, core::ptr::addr_of_mut!((*((0 as *mut hci_dev))), dump.dump_timeout.work));
    hci_devcd_notify(hdev, HCI_DEVCOREDUMP_TIMEOUT); hci_dev_lock(hdev); cancel_work(&mut (*hdev).dump.dump_rx);
    hci_devcd_update_state(hdev, HCI_DEVCOREDUMP_TIMEOUT); hci_devcd_dump(hdev); hci_devcd_reset(hdev); hci_dev_unlock(hdev);
}

pub unsafe fn hci_devcd_register(hdev: *mut hci_dev, coredump: coredump_t, dmp_hdr: dmp_hdr_t, notify_change: notify_change_t) -> i32 {
    if coredump.is_none() || dmp_hdr.is_none() { return -EINVAL; }
    hci_dev_lock(hdev); (*hdev).dump.coredump = coredump; (*hdev).dump.dmp_hdr = dmp_hdr; (*hdev).dump.notify_change = notify_change; (*hdev).dump.supported = true; (*hdev).dump.timeout = DEVCOREDUMP_TIMEOUT; hci_dev_unlock(hdev); 0
}

unsafe fn hci_devcd_enabled(hdev: *mut hci_dev) -> bool { (*hdev).dump.supported }

pub unsafe fn hci_devcd_init(hdev: *mut hci_dev, dump_size: u32) -> i32 {
    if !hci_devcd_enabled(hdev) { return -EOPNOTSUPP; }
    let skb = alloc_skb(core::mem::size_of::<u32>(), GFP_ATOMIC); if skb.is_null() { return -ENOMEM; }
    (*hci_dmp_cb(skb)).pkt_type = HCI_DEVCOREDUMP_PKT_INIT as u16; put_unaligned_le32(dump_size, skb_put(skb, 4)); skb_queue_tail(&mut (*hdev).dump.dump_q, skb); queue_work((*hdev).workqueue, &mut (*hdev).dump.dump_rx); 0
}

pub unsafe fn hci_devcd_append(hdev: *mut hci_dev, skb: *mut sk_buff) -> i32 {
    if skb.is_null() { return -ENOMEM; } if !hci_devcd_enabled(hdev) { kfree_skb(skb); return -EOPNOTSUPP; }
    (*hci_dmp_cb(skb)).pkt_type = HCI_DEVCOREDUMP_PKT_SKB as u16; skb_queue_tail(&mut (*hdev).dump.dump_q, skb); queue_work((*hdev).workqueue, &mut (*hdev).dump.dump_rx); 0
}

pub unsafe fn hci_devcd_append_pattern(hdev: *mut hci_dev, pattern: u8, len: u32) -> i32 {
    if !hci_devcd_enabled(hdev) { return -EOPNOTSUPP; }
    let skb = alloc_skb(core::mem::size_of::<HciDevcoredumpSkbPattern>(), GFP_ATOMIC); if skb.is_null() { return -ENOMEM; }
    let p = HciDevcoredumpSkbPattern { pattern, len }; (*hci_dmp_cb(skb)).pkt_type = HCI_DEVCOREDUMP_PKT_PATTERN as u16; skb_put_data(skb, &p as *const _ as *const _, core::mem::size_of_val(&p)); skb_queue_tail(&mut (*hdev).dump.dump_q, skb); queue_work((*hdev).workqueue, &mut (*hdev).dump.dump_rx); 0
}

pub unsafe fn hci_devcd_complete(hdev: *mut hci_dev) -> i32 { hci_devcd_signal(hdev, HCI_DEVCOREDUMP_PKT_COMPLETE) }
pub unsafe fn hci_devcd_abort(hdev: *mut hci_dev) -> i32 { hci_devcd_signal(hdev, HCI_DEVCOREDUMP_PKT_ABORT) }

unsafe fn hci_devcd_signal(hdev: *mut hci_dev, pkt: HciDevcoredumpPktType) -> i32 {
    if !hci_devcd_enabled(hdev) { return -EOPNOTSUPP; } let skb = alloc_skb(0, GFP_ATOMIC); if skb.is_null() { return -ENOMEM; }
    (*hci_dmp_cb(skb)).pkt_type = pkt as u16; skb_queue_tail(&mut (*hdev).dump.dump_q, skb); queue_work((*hdev).workqueue, &mut (*hdev).dump.dump_rx); 0
}

pub unsafe fn hci_devcd_state_name(state: i32) -> *const i8 {
    match state { HCI_DEVCOREDUMP_IDLE => b"IDLE\0".as_ptr() as *const i8, HCI_DEVCOREDUMP_ACTIVE => b"ACTIVE\0".as_ptr() as *const i8, HCI_DEVCOREDUMP_DONE => b"DONE\0".as_ptr() as *const i8, HCI_DEVCOREDUMP_ABORT => b"ABORT\0".as_ptr() as *const i8, HCI_DEVCOREDUMP_TIMEOUT => b"TIMEOUT\0".as_ptr() as *const i8, _ => b"Unknown\0".as_ptr() as *const i8 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
