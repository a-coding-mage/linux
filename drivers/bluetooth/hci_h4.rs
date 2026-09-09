// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 *  Bluetooth HCI UART driver
 *
 *  Copyright (C) 2000-2001  Qualcomm Incorporated
 *  Copyright (C) 2002-2003  Maxim Krasnyansky <maxk@qualcomm.com>
 *  Copyright (C) 2004-2005  Marcel Holtmann <marcel@holtmann.org>
 */

// Linux kernel and Bluetooth declarations are supplied by other files.

#[repr(C)]
struct h4_struct {
    rx_skb: *mut sk_buff,
    txq: sk_buff_head,
}

/* Initialize protocol */
unsafe fn h4_open(hu: *mut hci_uart) -> i32 {
    let h4: *mut h4_struct = kzalloc_obj::<h4_struct>();

    bt_dbg!("hu %p", hu);

    if h4.is_null() {
        return -ENOMEM;
    }

    skb_queue_head_init(&mut (*h4).txq);

    (*hu).priv_ = h4 as *mut _;
    0
}

/* Flush protocol data */
unsafe fn h4_flush(hu: *mut hci_uart) -> i32 {
    let h4 = (*hu).priv_ as *mut h4_struct;

    bt_dbg!("hu %p", hu);

    skb_queue_purge(&mut (*h4).txq);

    0
}

/* Close protocol */
unsafe fn h4_close(hu: *mut hci_uart) -> i32 {
    let h4 = (*hu).priv_ as *mut h4_struct;

    bt_dbg!("hu %p", hu);

    skb_queue_purge(&mut (*h4).txq);
    kfree_skb((*h4).rx_skb);

    (*hu).priv_ = core::ptr::null_mut();
    kfree(h4 as *mut _);

    0
}

/* Enqueue frame for transmission (padding, crc, etc) */
unsafe fn h4_enqueue(hu: *mut hci_uart, skb: *mut sk_buff) -> i32 {
    let h4 = (*hu).priv_ as *mut h4_struct;

    bt_dbg!("hu %p skb %p", hu, skb);

    /* Prepend skb with frame type */
    let dst = skb_push(skb, 1);
    *dst = hci_skb_pkt_type(skb);
    skb_queue_tail(&mut (*h4).txq, skb);

    0
}

static h4_recv_pkts: [h4_recv_pkt; 4] = [
    h4_recv_pkt { type_: H4_RECV_ACL, recv: hci_recv_frame },
    h4_recv_pkt { type_: H4_RECV_SCO, recv: hci_recv_frame },
    h4_recv_pkt { type_: H4_RECV_EVENT, recv: hci_recv_frame },
    h4_recv_pkt { type_: H4_RECV_ISO, recv: hci_recv_frame },
];

/* Recv data */
unsafe fn h4_recv(hu: *mut hci_uart, data: *const core::ffi::c_void, mut count: i32) -> i32 {
    let h4 = (*hu).priv_ as *mut h4_struct;

    if h4.is_null() {
        return -ENODEV;
    }

    (*h4).rx_skb = h4_recv_buf(hu, (*h4).rx_skb, data as *const u8, count,
                               h4_recv_pkts.as_ptr(), h4_recv_pkts.len() as i32);
    if is_err((*h4).rx_skb) {
        let err = ptr_err((*h4).rx_skb);
        bt_dev_err!((*hu).hdev, "Frame reassembly failed (%d)", err);
        (*h4).rx_skb = core::ptr::null_mut();
        return err;
    }

    count
}

unsafe fn h4_dequeue(hu: *mut hci_uart) -> *mut sk_buff {
    let h4 = (*hu).priv_ as *mut h4_struct;
    skb_dequeue(&mut (*h4).txq)
}

static h4p: hci_uart_proto = hci_uart_proto {
    id: HCI_UART_H4,
    name: b"H4\0".as_ptr() as *const i8,
    open: h4_open,
    close: h4_close,
    recv: h4_recv,
    enqueue: h4_enqueue,
    dequeue: h4_dequeue,
    flush: h4_flush,
};

unsafe fn h4_init() -> i32 {
    hci_uart_register_proto(&h4p)
}

unsafe fn h4_deinit() -> i32 {
    hci_uart_unregister_proto(&h4p)
}

unsafe fn h4_recv_buf(
    hu: *mut hci_uart,
    mut skb: *mut sk_buff,
    mut buffer: *const u8,
    mut count: i32,
    pkts: *const h4_recv_pkt,
    pkts_count: i32,
) -> *mut sk_buff {
    let alignment: u8 = if (*hu).alignment != 0 { (*hu).alignment } else { 1 };
    let hdev = (*hu).hdev;

    /* Check for error from previous call */
    if is_err(skb) {
        skb = core::ptr::null_mut();
    }

    while count != 0 {
        let mut i: i32;
        let mut len: i32;

        /* remove padding bytes from buffer */
        while (*hu).padding != 0 && count > 0 {
            count -= 1;
            buffer = buffer.add(1);
            (*hu).padding -= 1;
        }
        if count == 0 {
            break;
        }

        if skb.is_null() {
            i = 0;
            while i < pkts_count {
                let pkt = &*pkts.add(i as usize);
                if *buffer != pkt.type_ {
                    i += 1;
                    continue;
                }

                skb = bt_skb_alloc(pkt.maxlen, GFP_ATOMIC);
                if skb.is_null() {
                    return err_ptr(-ENOMEM);
                }

                hci_skb_pkt_type(skb) = pkt.type_;
                hci_skb_expect(skb) = pkt.hlen;
                break;
            }

            /* Check for invalid packet type */
            if skb.is_null() {
                return err_ptr(-EILSEQ);
            }

            count -= 1;
            buffer = buffer.add(1);
        }

        len = core::cmp::min(hci_skb_expect(skb) - (*skb).len, count);
        skb_put_data(skb, buffer, len);

        count -= len;
        buffer = buffer.add(len as usize);

        /* Check for partial packet */
        if (*skb).len < hci_skb_expect(skb) {
            continue;
        }

        i = 0;
        while i < pkts_count {
            if hci_skb_pkt_type(skb) == (*pkts.add(i as usize)).type_ {
                break;
            }
            i += 1;
        }

        if i >= pkts_count {
            kfree_skb(skb);
            return err_ptr(-EILSEQ);
        }

        let pkt = &*pkts.add(i as usize);
        if (*skb).len == pkt.hlen {
            let dlen: u16;

            match pkt.lsize {
                0 => {
                    /* No variable data length */
                    dlen = 0;
                }
                1 => {
                    /* Single octet variable length */
                    dlen = *(*skb).data.add(pkt.loff as usize) as u16;
                    hci_skb_expect(skb) += dlen as i32;

                    if skb_tailroom(skb) < dlen as i32 {
                        kfree_skb(skb);
                        return err_ptr(-EMSGSIZE);
                    }
                }
                2 => {
                    /* Double octet variable length */
                    dlen = u16::from_le_bytes([
                        *(*skb).data.add(pkt.loff as usize),
                        *(*skb).data.add(pkt.loff as usize + 1),
                    ]);
                    hci_skb_expect(skb) += dlen as i32;

                    if skb_tailroom(skb) < dlen as i32 {
                        kfree_skb(skb);
                        return err_ptr(-EMSGSIZE);
                    }
                }
                _ => {
                    /* Unsupported variable length */
                    kfree_skb(skb);
                    return err_ptr(-EILSEQ);
                }
            }

            if dlen == 0 {
                (*hu).padding = ((*skb).len + 1) as u8 % alignment;
                (*hu).padding = (alignment - (*hu).padding) % alignment;

                /* No more data, complete frame */
                (pkt.recv)(hdev, skb);
                skb = core::ptr::null_mut();
            }
        } else {
            (*hu).padding = ((*skb).len + 1) as u8 % alignment;
            (*hu).padding = (alignment - (*hu).padding) % alignment;

            /* Complete frame */
            (pkt.recv)(hdev, skb);
            skb = core::ptr::null_mut();
        }
    }

    skb
}

// EXPORT_SYMBOL_GPL(h4_recv_buf);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
