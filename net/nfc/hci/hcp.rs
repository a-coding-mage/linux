// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2012  Intel Corporation. All rights reserved.
 */

// pr_fmt(fmt) "hci: %s: " fmt, __func__
// C dependencies supplied by the surrounding kernel/NFC implementation.

/*
 * Payload is the HCP message data only. Instruction will be prepended.
 * Guarantees that cb will be called upon completion or timeout delay
 * counted from the moment the cmd is sent to the transport.
 */
pub unsafe fn nfc_hci_hcp_message_tx(
    hdev: *mut nfc_hci_dev,
    pipe: u8,
    type_: u8,
    instruction: u8,
    payload: *const u8,
    payload_len: usize,
    cb: data_exchange_cb_t,
    cb_context: *mut core::ffi::c_void,
    completion_delay: c_ulong,
) -> i32 {
    let ndev = (*hdev).ndev;
    let mut cmd: *mut hci_msg;
    let mut ptr = payload;
    let mut hci_len: i32;
    let mut err: i32;
    let mut firstfrag = true;

    cmd = kzalloc_obj::<hci_msg>();
    if cmd.is_null() {
        return -ENOMEM;
    }

    INIT_LIST_HEAD(&mut (*cmd).msg_l);
    skb_queue_head_init(&mut (*cmd).msg_frags);
    (*cmd).wait_response = type_ == NFC_HCI_HCP_COMMAND;
    (*cmd).cb = cb;
    (*cmd).cb_context = cb_context;
    (*cmd).completion_delay = completion_delay;

    hci_len = (payload_len + 1) as i32;
    while hci_len > 0 {
        let skb: *mut sk_buff;
        let skb_len: i32;
        let data_link_len: i32;
        let packet: *mut hcp_packet;

        if (NFC_HCI_HCP_PACKET_HEADER_LEN as i32) + hci_len
            <= (*hdev).max_data_link_payload
        {
            data_link_len = hci_len;
        } else {
            data_link_len = (*hdev).max_data_link_payload
                - NFC_HCI_HCP_PACKET_HEADER_LEN as i32;
        }

        skb_len = (*ndev).tx_headroom
            + NFC_HCI_HCP_PACKET_HEADER_LEN as i32
            + data_link_len
            + (*ndev).tx_tailroom;
        hci_len -= data_link_len;

        skb = alloc_skb(skb_len as usize, GFP_KERNEL);
        if skb.is_null() {
            err = -ENOMEM;
            goto out_skb_err;
        }
        skb_reserve(skb, (*ndev).tx_headroom as usize);

        skb_put(
            skb,
            (NFC_HCI_HCP_PACKET_HEADER_LEN as i32 + data_link_len) as usize,
        );

        // Only the last fragment will have the cb bit set to 1
        packet = (*skb).data as *mut hcp_packet;
        (*packet).header = pipe;
        if firstfrag {
            firstfrag = false;
            (*packet).message.header = HCP_HEADER(type_, instruction);
        } else {
            (*packet).message.header = *ptr;
            ptr = ptr.add(1);
        }
        if !ptr.is_null() {
            memcpy(
                (*packet).message.data.as_mut_ptr() as *mut core::ffi::c_void,
                ptr as *const core::ffi::c_void,
                (data_link_len - 1) as usize,
            );
            ptr = ptr.add((data_link_len - 1) as usize);
        }

        // This is the last fragment, set the cb bit
        if hci_len == 0 {
            (*packet).header |= !NFC_HCI_FRAGMENT;
        }

        skb_queue_tail(&mut (*cmd).msg_frags, skb);
    }

    mutex_lock(&mut (*hdev).msg_tx_mutex);

    if (*hdev).shutting_down {
        err = -ESHUTDOWN;
        mutex_unlock(&mut (*hdev).msg_tx_mutex);
        goto out_skb_err;
    }

    list_add_tail(&mut (*cmd).msg_l, &mut (*hdev).msg_tx_queue);
    mutex_unlock(&mut (*hdev).msg_tx_mutex);

    schedule_work(&mut (*hdev).msg_tx_work);

    return 0;

out_skb_err:
    skb_queue_purge(&mut (*cmd).msg_frags);
    kfree(cmd);

    err
}

/*
 * Receive hcp message for pipe, with type and cmd.
 * skb contains optional message data only.
 */
pub unsafe fn nfc_hci_hcp_message_rx(
    hdev: *mut nfc_hci_dev,
    pipe: u8,
    type_: u8,
    instruction: u8,
    skb: *mut sk_buff,
) {
    match type_ {
        NFC_HCI_HCP_RESPONSE => {
            nfc_hci_resp_received(hdev, instruction, skb);
        }
        NFC_HCI_HCP_COMMAND => {
            nfc_hci_cmd_received(hdev, pipe, instruction, skb);
        }
        NFC_HCI_HCP_EVENT => {
            nfc_hci_event_received(hdev, pipe, instruction, skb);
        }
        _ => {
            pr_err!("UNKNOWN MSG Type %d, instruction=%d\n", type_, instruction);
            kfree_skb(skb);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
