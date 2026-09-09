// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Atheros Communication Bluetooth HCIATH3K UART protocol
 *
 *  HCIATH3K (HCI Atheros AR300x Protocol) is a Atheros Communication's
 *  power management protocol extension to H4 to support AR300x Bluetooth Chip.
 *
 *  Copyright (c) 2009-2010 Atheros Communications Inc.
 *
 *  Acknowledgements:
 *  This file is based on hci_h4.c, which was written
 *  by Maxim Krasnyansky and Marcel Holtmann.
 */

// Linux kernel and Bluetooth definitions are supplied by external dependencies.

#[repr(C)]
struct ath_struct {
    hu: *mut hci_uart,
    cur_sleep: c_uint,
    rx_skb: *mut sk_buff,
    txq: sk_buff_head,
    ctxtsw: work_struct,
}

const OP_WRITE_TAG: u8 = 0x01;
const INDEX_BDADDR: u8 = 0x01;

#[repr(C, packed)]
struct ath_vendor_cmd {
    opcode: u8,
    index: __le16,
    len: u8,
    data: [u8; 251],
}

unsafe fn ath_wakeup_ar3k(tty: *mut tty_struct) -> c_int {
    let mut status = (*(*tty).driver).ops.tiocmget(tty);

    if status & TIOCM_CTS != 0 {
        return status;
    }

    /* Clear RTS first */
    (*(*tty).driver).ops.tiocmget(tty);
    (*(*tty).driver).ops.tiocmset(tty, 0x00, TIOCM_RTS);
    msleep(20);

    /* Set RTS, wake up board */
    (*(*tty).driver).ops.tiocmget(tty);
    (*(*tty).driver).ops.tiocmset(tty, TIOCM_RTS, 0x00);
    msleep(20);

    status = (*(*tty).driver).ops.tiocmget(tty);
    status
}

unsafe fn ath_hci_uart_work(work: *mut work_struct) {
    let status: c_int;
    let ath: *mut ath_struct;
    let hu: *mut hci_uart;
    let tty: *mut tty_struct;

    ath = container_of!(work, ath_struct, ctxtsw);
    hu = (*ath).hu;
    tty = (*hu).tty;

    /* verify and wake up controller */
    if (*ath).cur_sleep != 0 {
        status = ath_wakeup_ar3k(tty);
        if status & TIOCM_CTS == 0 {
            return;
        }
    }

    /* Ready to send Data */
    clear_bit(HCI_UART_SENDING, &mut (*hu).tx_state);
    hci_uart_tx_wakeup(hu);
}

unsafe fn ath_open(hu: *mut hci_uart) -> c_int {
    let ath: *mut ath_struct;

    BT_DBG!("hu %p", hu);

    if !hci_uart_has_flow_control(hu) {
        return -EOPNOTSUPP;
    }

    ath = kzalloc_obj::<ath_struct>();
    if ath.is_null() {
        return -ENOMEM;
    }

    skb_queue_head_init(&mut (*ath).txq);
    (*hu).priv_ = ath as *mut c_void;
    (*ath).hu = hu;
    INIT_WORK!(&mut (*ath).ctxtsw, ath_hci_uart_work);
    0
}

unsafe fn ath_close(hu: *mut hci_uart) -> c_int {
    let ath = (*hu).priv_ as *mut ath_struct;

    BT_DBG!("hu %p", hu);
    skb_queue_purge(&mut (*ath).txq);
    kfree_skb((*ath).rx_skb);
    cancel_work_sync(&mut (*ath).ctxtsw);
    (*hu).priv_ = core::ptr::null_mut();
    kfree(ath as *mut c_void);
    0
}

unsafe fn ath_flush(hu: *mut hci_uart) -> c_int {
    let ath = (*hu).priv_ as *mut ath_struct;

    BT_DBG!("hu %p", hu);
    skb_queue_purge(&mut (*ath).txq);
    0
}

unsafe fn ath_vendor_cmd(
    hdev: *mut hci_dev,
    opcode: u8,
    index: u16,
    data: *const c_void,
    dlen: usize,
) -> c_int {
    let skb: *mut sk_buff;
    let mut cmd: ath_vendor_cmd = core::mem::zeroed();

    if dlen > core::mem::size_of_val(&cmd.data) {
        return -EINVAL;
    }
    cmd.opcode = opcode;
    cmd.index = cpu_to_le16(index);
    cmd.len = dlen as u8;
    memcpy(cmd.data.as_mut_ptr() as *mut c_void, data, dlen);

    skb = __hci_cmd_sync(hdev, 0xfc0b, dlen + 4, &cmd as *const _ as *const c_void, HCI_INIT_TIMEOUT);
    if IS_ERR!(skb) {
        return PTR_ERR!(skb);
    }
    kfree_skb(skb);
    0
}

unsafe fn ath_set_bdaddr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> c_int {
    ath_vendor_cmd(hdev, OP_WRITE_TAG, INDEX_BDADDR, bdaddr as *const c_void, core::mem::size_of::<bdaddr_t>())
}

unsafe fn ath_setup(hu: *mut hci_uart) -> c_int {
    BT_DBG!("hu %p", hu);
    (*(*hu).hdev).set_bdaddr = Some(ath_set_bdaddr);
    0
}

static ath_recv_pkts: [h4_recv_pkt; 3] = [
    h4_recv_pkt { typ: H4_RECV_ACL, recv: Some(hci_recv_frame) },
    h4_recv_pkt { typ: H4_RECV_SCO, recv: Some(hci_recv_frame) },
    h4_recv_pkt { typ: H4_RECV_EVENT, recv: Some(hci_recv_frame) },
];

unsafe fn ath_recv(hu: *mut hci_uart, data: *const c_void, count: c_int) -> c_int {
    let ath = (*hu).priv_ as *mut ath_struct;
    if ath.is_null() {
        return -ENODEV;
    }
    (*ath).rx_skb = h4_recv_buf(hu, (*ath).rx_skb, data, count, ath_recv_pkts.as_ptr(), ath_recv_pkts.len());
    if IS_ERR!((*ath).rx_skb) {
        let err = PTR_ERR!((*ath).rx_skb);
        bt_dev_err!((*hu).hdev, "Frame reassembly failed (%d)", err);
        (*ath).rx_skb = core::ptr::null_mut();
        return err;
    }
    count
}

const HCI_OP_ATH_SLEEP: u16 = 0xFC04;

unsafe fn ath_enqueue(hu: *mut hci_uart, skb: *mut sk_buff) -> c_int {
    let ath = (*hu).priv_ as *mut ath_struct;
    if hci_skb_pkt_type(skb) == HCI_SCODATA_PKT {
        kfree_skb(skb);
        return 0;
    }
    /* Update power management enable flag with parameters of
     * HCI sleep enable vendor specific HCI command.
     */
    if hci_skb_pkt_type(skb) == HCI_COMMAND_PKT {
        let hdr = (*skb).data as *const hci_command_hdr;
        if __le16_to_cpu((*hdr).opcode) == HCI_OP_ATH_SLEEP {
            (*ath).cur_sleep = *((*skb).data.add(HCI_COMMAND_HDR_SIZE) as *const u8) as c_uint;
        }
    }
    BT_DBG!("hu %p skb %p", hu, skb);
    let pkt_type = hci_skb_pkt_type(skb);
    memcpy(skb_push(skb, 1) as *mut c_void, &pkt_type as *const _ as *const c_void, 1);
    skb_queue_tail(&mut (*ath).txq, skb);
    set_bit(HCI_UART_SENDING, &mut (*hu).tx_state);
    schedule_work(&mut (*ath).ctxtsw);
    0
}

unsafe fn ath_dequeue(hu: *mut hci_uart) -> *mut sk_buff {
    let ath = (*hu).priv_ as *mut ath_struct;
    skb_dequeue(&mut (*ath).txq)
}

static athp: hci_uart_proto = hci_uart_proto {
    id: HCI_UART_ATH3K,
    name: b"ATH3K\0".as_ptr() as *const c_char,
    manufacturer: 69,
    open: Some(ath_open),
    close: Some(ath_close),
    flush: Some(ath_flush),
    setup: Some(ath_setup),
    recv: Some(ath_recv),
    enqueue: Some(ath_enqueue),
    dequeue: Some(ath_dequeue),
};

pub unsafe fn ath_init() -> c_int {
    hci_uart_register_proto(&athp)
}

pub unsafe fn ath_deinit() -> c_int {
    hci_uart_unregister_proto(&athp)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
