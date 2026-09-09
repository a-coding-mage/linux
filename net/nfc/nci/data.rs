// SPDX-License-Identifier: GPL-2.0-only
/*
 *  The NFC Controller Interface is the communication protocol between an
 *  NFC Controller (NFCC) and a Device Host (DH).
 *
 *  Copyright (C) 2011 Texas Instruments, Inc.
 *  Copyright (C) 2014 Marvell International Ltd.
 *
 *  Written by Ilan Elias <ilane@ti.com>
 */

// C dependencies and build-time symbols are supplied by the surrounding NFC
// implementation.

pub unsafe fn nci_data_exchange_complete(
    ndev: *mut nci_dev,
    skb: *mut sk_buff,
    conn_id: __u8,
    err: c_int,
) {
    let conn_info = nci_get_conn_info_by_conn_id(ndev, conn_id);
    if conn_info.is_null() {
        kfree_skb(skb);
        clear_bit(NCI_DATA_EXCHANGE, &mut (*ndev).flags);
        return;
    }

    let cb = (*conn_info).data_exchange_cb;
    let cb_context = (*conn_info).data_exchange_cb_context;

    pr_debug!("len {}, err {}\n", if !skb.is_null() { (*skb).len } else { 0 }, err);
    timer_delete_sync(&mut (*ndev).data_timer);
    clear_bit(NCI_DATA_EXCHANGE_TO, &mut (*ndev).flags);

    if !test_and_clear_bit(NCI_DATA_EXCHANGE, &mut (*ndev).flags) {
        kfree_skb(skb);
        return;
    }

    if let Some(cb) = cb {
        cb(cb_context, skb, err);
    } else if !skb.is_null() {
        pr_err!("no rx callback, dropping rx data...\n");
        kfree_skb(skb);
    }
}

/* ----------------- NCI TX Data ----------------- */

unsafe fn nci_push_data_hdr(
    _ndev: *mut nci_dev,
    conn_id: __u8,
    skb: *mut sk_buff,
    pbf: __u8,
) {
    let plen = (*skb).len;
    let hdr = skb_push(skb, NCI_DATA_HDR_SIZE) as *mut nci_data_hdr;
    (*hdr).conn_id = conn_id;
    (*hdr).rfu = 0;
    (*hdr).plen = plen;
    nci_mt_set(hdr as *mut __u8, NCI_MT_DATA_PKT);
    nci_pbf_set(hdr as *mut __u8, pbf);
}

pub unsafe fn nci_conn_max_data_pkt_payload_size(ndev: *mut nci_dev, conn_id: __u8) -> c_int {
    let conn_info = nci_get_conn_info_by_conn_id(ndev, conn_id);
    if conn_info.is_null() {
        return -EPROTO;
    }
    (*conn_info).max_pkt_payload_len
}

unsafe fn nci_queue_tx_data_frags(
    ndev: *mut nci_dev,
    conn_id: __u8,
    skb: *mut sk_buff,
) -> c_int {
    let mut total_len = (*skb).len;
    let mut data = (*skb).data;
    let mut flags: c_ulong = 0;
    let mut frags_q = sk_buff_head::default();
    let mut rc = 0;

    pr_debug!("conn_id 0x{:x}, total_len {}\n", conn_id, total_len);
    let conn_info = nci_get_conn_info_by_conn_id(ndev, conn_id);
    if conn_info.is_null() {
        return -EPROTO;
    }
    __skb_queue_head_init(&mut frags_q);

    while total_len != 0 {
        let frag_len = core::cmp::min(total_len, (*conn_info).max_pkt_payload_len);
        let skb_frag = nci_skb_alloc(ndev, NCI_DATA_HDR_SIZE + frag_len, GFP_ATOMIC);
        if skb_frag.is_null() {
            rc = -ENOMEM;
            break;
        }
        skb_reserve(skb_frag, NCI_DATA_HDR_SIZE);
        skb_put_data(skb_frag, data, frag_len);
        nci_push_data_hdr(ndev, conn_id, skb_frag,
            if total_len == frag_len { NCI_PBF_LAST } else { NCI_PBF_CONT });
        __skb_queue_tail(&mut frags_q, skb_frag);
        data = data.add(frag_len);
        total_len -= frag_len;
        pr_debug!("frag_len {}, remaining total_len {}\n", frag_len, total_len);
    }

    if rc != 0 {
        while let Some(skb_frag) = __skb_dequeue(&mut frags_q) { kfree_skb(skb_frag); }
        return rc;
    }
    spin_lock_irqsave(&mut (*ndev).tx_q.lock, &mut flags);
    while let Some(skb_frag) = __skb_dequeue(&mut frags_q) {
        __skb_queue_tail(&mut (*ndev).tx_q, skb_frag);
    }
    spin_unlock_irqrestore(&mut (*ndev).tx_q.lock, flags);
    kfree_skb(skb);
    rc
}

pub unsafe fn nci_send_data(ndev: *mut nci_dev, conn_id: __u8, skb: *mut sk_buff) -> c_int {
    let conn_info = nci_get_conn_info_by_conn_id(ndev, conn_id);
    if conn_info.is_null() { kfree_skb(skb); return -EPROTO; }
    pr_debug!("conn_id 0x{:x}, plen {}\n", conn_id, (*skb).len);
    let rc;
    if (*skb).len <= (*conn_info).max_pkt_payload_len {
        nci_push_data_hdr(ndev, conn_id, skb, NCI_PBF_LAST);
        skb_queue_tail(&mut (*ndev).tx_q, skb);
        rc = 0;
    } else {
        rc = nci_queue_tx_data_frags(ndev, conn_id, skb);
        if rc != 0 { pr_err!("failed to fragment tx data packet\n"); return rc; }
    }
    (*ndev).cur_conn_id = conn_id;
    queue_work((*ndev).tx_wq, &mut (*ndev).tx_work);
    rc
}

/* ----------------- NCI RX Data ----------------- */

unsafe fn nci_add_rx_data_frag(ndev: *mut nci_dev, mut skb: *mut sk_buff,
                               pbf: __u8, conn_id: __u8, status: __u8) {
    let mut err = 0;
    if status != 0 { err = status as c_int; }
    else {
        if !(*ndev).rx_data_reassembly.is_null() {
            let reassembly_len = (*(*ndev).rx_data_reassembly).len;
            if skb_cow_head(skb, reassembly_len) != 0 {
                pr_err!("error adding room for accumulated rx data\n");
                kfree_skb(skb); skb = core::ptr::null_mut();
                kfree_skb((*ndev).rx_data_reassembly);
                (*ndev).rx_data_reassembly = core::ptr::null_mut();
                err = -ENOMEM;
            } else {
                memcpy(skb_push(skb, reassembly_len), (*(*ndev).rx_data_reassembly).data, reassembly_len);
                kfree_skb((*ndev).rx_data_reassembly);
                (*ndev).rx_data_reassembly = core::ptr::null_mut();
            }
        }
        if err == 0 && pbf == NCI_PBF_CONT {
            (*ndev).rx_data_reassembly = skb;
            return;
        }
    }
    if (*(*ndev).nfc_dev).rf_mode == NFC_RF_TARGET {
        err = nfc_tm_data_received((*ndev).nfc_dev, skb);
        if err != 0 { pr_err!("unable to handle received data\n"); }
    } else { nci_data_exchange_complete(ndev, skb, conn_id, err); }
}

pub unsafe fn nci_rx_data_packet(ndev: *mut nci_dev, skb: *mut sk_buff) {
    let pbf = nci_pbf((*skb).data);
    let mut status = 0;
    let conn_id = nci_conn_id((*skb).data);
    pr_debug!("len {}\n", (*skb).len);
    pr_debug!("NCI RX: MT=data, PBF={}, conn_id={}, plen={}\n", pbf, conn_id, nci_plen((*skb).data));
    if nci_get_conn_info_by_conn_id(ndev, conn_id).is_null() { kfree_skb(skb); return; }
    skb_pull(skb, NCI_DATA_HDR_SIZE);
    let proto = (*ndev).target_active_prot;
    if proto == NFC_PROTO_MIFARE || proto == NFC_PROTO_JEWEL ||
       proto == NFC_PROTO_FELICA || proto == NFC_PROTO_ISO15693 {
        pr_debug!("frame I/F => remove the status byte\n");
        status = *(*skb).data.add((*skb).len - 1);
        skb_trim(skb, (*skb).len - 1);
    }
    nci_add_rx_data_frag(ndev, skb, pbf, conn_id, nci_to_errno(status));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
