// SPDX-License-Identifier: GPL-2.0
/* BlueZ - Bluetooth protocol stack for Linux
 * Copyright (C) 2015 Intel Corporation
 *
 * This is a source-level Rust translation of mgmt_util.c.  Kernel and
 * Bluetooth definitions referenced here are supplied by other modules.
 */

use core::mem::size_of;

// External kernel/Bluetooth declarations are provided by the surrounding tree.

unsafe fn create_monitor_ctrl_event(index: __le16, cookie: u32, opcode: u16,
                                    len: u16, buf: *mut core::ffi::c_void) -> *mut sk_buff {
    let skb = bt_skb_alloc(6 + len as usize, GFP_ATOMIC);
    if skb.is_null() { return core::ptr::null_mut(); }

    put_unaligned_le32(cookie, skb_put(skb, 4));
    put_unaligned_le16(opcode, skb_put(skb, 2));
    if !buf.is_null() { skb_put_data(skb, buf, len as usize); }
    __net_timestamp(skb);

    let hdr = skb_push(skb, HCI_MON_HDR_SIZE) as *mut hci_mon_hdr;
    (*hdr).opcode = cpu_to_le16(HCI_MON_CTRL_EVENT);
    (*hdr).index = index;
    (*hdr).len = cpu_to_le16((*skb).len.wrapping_sub(HCI_MON_HDR_SIZE));
    skb
}

pub unsafe fn mgmt_alloc_skb(hdev: *mut hci_dev, opcode: u16,
                             size: usize) -> *mut sk_buff {
    let skb = alloc_skb(size_of::<mgmt_hdr>() + size, GFP_KERNEL);
    if skb.is_null() { return skb; }
    skb_reserve(skb, size_of::<mgmt_hdr>());
    (*bt_cb(skb)).mgmt.hdev = hdev;
    (*bt_cb(skb)).mgmt.opcode = opcode;
    skb
}

pub unsafe fn mgmt_send_event_skb(channel: u16, skb: *mut sk_buff, flag: i32,
                                  skip_sk: *mut sock) -> i32 {
    if skb.is_null() { return -EINVAL; }
    let len = (*skb).len;
    let hdev = (*bt_cb(skb)).mgmt.hdev;
    __net_timestamp(skb);
    if channel == HCI_CHANNEL_CONTROL {
        hci_send_monitor_ctrl_event(hdev, (*bt_cb(skb)).mgmt.opcode,
            (*skb).data, (*skb).len, skb_get_ktime(skb), flag, skip_sk);
    }
    let hdr = skb_push(skb, size_of::<mgmt_hdr>()) as *mut mgmt_hdr;
    (*hdr).opcode = cpu_to_le16((*bt_cb(skb)).mgmt.opcode);
    (*hdr).index = if !hdev.is_null() { cpu_to_le16((*hdev).id) } else { cpu_to_le16(MGMT_INDEX_NONE) };
    (*hdr).len = cpu_to_le16(len as u16);
    hci_send_to_channel(channel, skb, flag, skip_sk);
    kfree_skb(skb);
    0
}

pub unsafe fn mgmt_send_event(event: u16, hdev: *mut hci_dev, channel: u16,
                              data: *mut core::ffi::c_void, data_len: u16,
                              flag: i32, skip_sk: *mut sock) -> i32 {
    let skb = mgmt_alloc_skb(hdev, event, data_len as usize);
    if skb.is_null() { return -ENOMEM; }
    if !data.is_null() { skb_put_data(skb, data, data_len as usize); }
    mgmt_send_event_skb(channel, skb, flag, skip_sk)
}

pub unsafe fn mgmt_cmd_status(sk: *mut sock, index: u16, cmd: u16, status: u8) -> i32 {
    let skb = alloc_skb(size_of::<mgmt_hdr>() + size_of::<mgmt_ev_cmd_status>(), GFP_KERNEL);
    if skb.is_null() { return -ENOMEM; }
    let hdr = skb_put(skb, size_of::<mgmt_hdr>()) as *mut mgmt_hdr;
    (*hdr).opcode = cpu_to_le16(MGMT_EV_CMD_STATUS); (*hdr).index = cpu_to_le16(index);
    (*hdr).len = cpu_to_le16(size_of::<mgmt_ev_cmd_status>() as u16);
    let ev = skb_put(skb, size_of::<mgmt_ev_cmd_status>()) as *mut mgmt_ev_cmd_status;
    (*ev).status = status; (*ev).opcode = cpu_to_le16(cmd);
    let mskb = create_monitor_ctrl_event((*hdr).index, hci_sock_get_cookie(sk), MGMT_EV_CMD_STATUS,
                                         size_of::<mgmt_ev_cmd_status>() as u16, ev as *mut _);
    if !mskb.is_null() { (*skb).tstamp = (*mskb).tstamp; } else { __net_timestamp(skb); }
    let err = sock_queue_rcv_skb(sk, skb); if err < 0 { kfree_skb(skb); }
    if !mskb.is_null() { hci_send_to_channel(HCI_CHANNEL_MONITOR, mskb, HCI_SOCK_TRUSTED, core::ptr::null_mut()); kfree_skb(mskb); }
    err
}

pub unsafe fn mgmt_cmd_complete(sk: *mut sock, index: u16, cmd: u16, status: u8,
                                rp: *mut core::ffi::c_void, rp_len: usize) -> i32 {
    let skb = alloc_skb(size_of::<mgmt_hdr>() + size_of::<mgmt_ev_cmd_complete>() + rp_len, GFP_KERNEL);
    if skb.is_null() { return -ENOMEM; }
    let hdr = skb_put(skb, size_of::<mgmt_hdr>()) as *mut mgmt_hdr;
    (*hdr).opcode = cpu_to_le16(MGMT_EV_CMD_COMPLETE); (*hdr).index = cpu_to_le16(index);
    (*hdr).len = cpu_to_le16((size_of::<mgmt_ev_cmd_complete>() + rp_len) as u16);
    let ev = skb_put(skb, size_of::<mgmt_ev_cmd_complete>() + rp_len) as *mut mgmt_ev_cmd_complete;
    (*ev).opcode = cpu_to_le16(cmd); (*ev).status = status;
    if !rp.is_null() { core::ptr::copy_nonoverlapping(rp as *const u8, (*ev).data.as_mut_ptr(), rp_len); }
    let mskb = create_monitor_ctrl_event((*hdr).index, hci_sock_get_cookie(sk), MGMT_EV_CMD_COMPLETE,
        (size_of::<mgmt_ev_cmd_complete>() + rp_len) as u16, ev as *mut _);
    if !mskb.is_null() { (*skb).tstamp = (*mskb).tstamp; } else { __net_timestamp(skb); }
    let err = sock_queue_rcv_skb(sk, skb); if err < 0 { kfree_skb(skb); }
    if !mskb.is_null() { hci_send_to_channel(HCI_CHANNEL_MONITOR, mskb, HCI_SOCK_TRUSTED, core::ptr::null_mut()); kfree_skb(mskb); }
    err
}

// The remaining list helpers retain Linux list traversal semantics.  The list
// and object layouts, locks, allocation, and socket primitives are external.
pub unsafe fn mgmt_pending_find(channel: u16, opcode: u16, hdev: *mut hci_dev) -> *mut mgmt_pending_cmd {
    mutex_lock(&mut (*hdev).mgmt_pending_lock);
    let mut cmd = list_first_entry(&(*hdev).mgmt_pending, mgmt_pending_cmd, list);
    while !cmd.is_null() {
        if hci_sock_get_channel((*cmd).sk) == channel && (*cmd).opcode == opcode { mutex_unlock(&mut (*hdev).mgmt_pending_lock); return cmd; }
        cmd = list_next_entry(cmd, list);
    }
    mutex_unlock(&mut (*hdev).mgmt_pending_lock); core::ptr::null_mut()
}

pub unsafe fn mgmt_pending_new(sk: *mut sock, opcode: u16, hdev: *mut hci_dev,
                               data: *mut core::ffi::c_void, len: u16) -> *mut mgmt_pending_cmd {
    let cmd = kzalloc_obj::<mgmt_pending_cmd>(); if cmd.is_null() { return cmd; }
    (*cmd).opcode = opcode; (*cmd).hdev = hdev; (*cmd).param = kmemdup(data, len as usize, GFP_KERNEL);
    if (*cmd).param.is_null() { kfree(cmd as *mut _); return core::ptr::null_mut(); }
    (*cmd).param_len = len; (*cmd).sk = sk; sock_hold(sk); cmd
}

pub unsafe fn mgmt_pending_add(sk: *mut sock, opcode: u16, hdev: *mut hci_dev,
                               data: *mut core::ffi::c_void, len: u16) -> *mut mgmt_pending_cmd {
    let cmd = mgmt_pending_new(sk, opcode, hdev, data, len); if cmd.is_null() { return cmd; }
    mutex_lock(&mut (*hdev).mgmt_pending_lock); list_add_tail(&mut (*cmd).list, &mut (*hdev).mgmt_pending); mutex_unlock(&mut (*hdev).mgmt_pending_lock); cmd
}

pub unsafe fn mgmt_pending_free(cmd: *mut mgmt_pending_cmd) { sock_put((*cmd).sk); kfree((*cmd).param); kfree(cmd as *mut _); }
pub unsafe fn mgmt_pending_remove(cmd: *mut mgmt_pending_cmd) { let hdev=(*cmd).hdev; mutex_lock(&mut (*hdev).mgmt_pending_lock); list_del(&mut (*cmd).list); mutex_unlock(&mut (*hdev).mgmt_pending_lock); mgmt_pending_free(cmd); }
pub unsafe fn __mgmt_pending_listed(hdev: *mut hci_dev, cmd: *mut mgmt_pending_cmd) -> bool { if cmd.is_null(){return false;} list_contains(&(*hdev).mgmt_pending, cmd) }
pub unsafe fn mgmt_pending_listed(hdev:*mut hci_dev,cmd:*mut mgmt_pending_cmd)->bool{mutex_lock(&mut(*hdev).mgmt_pending_lock);let r=__mgmt_pending_listed(hdev,cmd);mutex_unlock(&mut(*hdev).mgmt_pending_lock);r}
pub unsafe fn mgmt_pending_valid(hdev:*mut hci_dev,cmd:*mut mgmt_pending_cmd)->bool{if cmd.is_null(){return false;}mutex_lock(&mut(*hdev).mgmt_pending_lock);let r=__mgmt_pending_listed(hdev,cmd);if r{list_del(&mut(*cmd).list);}mutex_unlock(&mut(*hdev).mgmt_pending_lock);r}

pub unsafe fn mgmt_mesh_foreach(hdev:*mut hci_dev,cb:unsafe fn(*mut mgmt_mesh_tx,*mut core::ffi::c_void),data:*mut core::ffi::c_void,sk:*mut sock){let mut p=list_first_entry(&(*hdev).mesh_pending,mgmt_mesh_tx,list);while !p.is_null(){let n=list_next_entry(p,list);if sk.is_null()||(*p).sk==sk{cb(p,data);}p=n;}}
pub unsafe fn mgmt_mesh_next(hdev:*mut hci_dev,sk:*mut sock)->*mut mgmt_mesh_tx{if list_empty(&(*hdev).mesh_pending){return core::ptr::null_mut();}let mut p=list_first_entry(&(*hdev).mesh_pending,mgmt_mesh_tx,list);while !p.is_null(){if sk.is_null()||(*p).sk==sk{return p;}p=list_next_entry(p,list);}core::ptr::null_mut()}
pub unsafe fn mgmt_mesh_find(hdev:*mut hci_dev,handle:u8)->*mut mgmt_mesh_tx{if list_empty(&(*hdev).mesh_pending){return core::ptr::null_mut();}let mut p=list_first_entry(&(*hdev).mesh_pending,mgmt_mesh_tx,list);while !p.is_null(){if (*p).handle==handle{return p;}p=list_next_entry(p,list);}core::ptr::null_mut()}
pub unsafe fn mgmt_mesh_add(sk:*mut sock,hdev:*mut hci_dev,data:*mut core::ffi::c_void,len:u16)->*mut mgmt_mesh_tx{let p=kzalloc_obj::<mgmt_mesh_tx>();if p.is_null(){return p;}(*hdev).mesh_send_ref=(*hdev).mesh_send_ref.wrapping_add(1);if (*hdev).mesh_send_ref==0{(*hdev).mesh_send_ref=(*hdev).mesh_send_ref.wrapping_add(1);}(*p).handle=(*hdev).mesh_send_ref;(*p).index=(*hdev).id;core::ptr::copy_nonoverlapping(data as *const u8,(*p).param.as_mut_ptr(),len as usize);(*p).param_len=len;(*p).sk=sk;sock_hold(sk);list_add_tail(&mut(*p).list,&mut(*hdev).mesh_pending);p}
pub unsafe fn mgmt_mesh_remove(p:*mut mgmt_mesh_tx){list_del(&mut(*p).list);sock_put((*p).sk);kfree(p as *mut _);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
