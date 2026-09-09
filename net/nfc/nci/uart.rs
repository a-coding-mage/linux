// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015, Marvell International Ltd.
 *
 * Inspired (hugely) by HCI LDISC implementation in Bluetooth.
 *
 *  Copyright (C) 2000-2001  Qualcomm Incorporated
 *  Copyright (C) 2002-2003  Maxim Krasnyansky <maxk@qualcomm.com>
 *  Copyright (C) 2004-2005  Marcel Holtmann <marcel@holtmann.org>
 */

// Linux kernel dependencies are supplied by the surrounding Rust kernel bindings.

/* TX states */
const NCI_UART_SENDING: usize = 1;
const NCI_UART_TX_WAKEUP: usize = 2;

static mut nci_uart_drivers: [*mut nci_uart; NCI_UART_DRIVER_MAX] =
    [core::ptr::null_mut(); NCI_UART_DRIVER_MAX];

unsafe fn nci_uart_dequeue(nu: *mut nci_uart) -> *mut sk_buff {
    let mut skb = (*nu).tx_skb;
    if skb.is_null() {
        skb = skb_dequeue(&mut (*nu).tx_q);
    } else {
        (*nu).tx_skb = core::ptr::null_mut();
    }
    skb
}

unsafe fn nci_uart_queue_empty(nu: *mut nci_uart) -> i32 {
    if !(*nu).tx_skb.is_null() { return 0; }
    skb_queue_empty(&(*nu).tx_q) as i32
}

unsafe fn nci_uart_tx_wakeup(nu: *mut nci_uart) -> i32 {
    if test_and_set_bit(NCI_UART_SENDING, &mut (*nu).tx_state) != 0 {
        set_bit(NCI_UART_TX_WAKEUP, &mut (*nu).tx_state);
        return 0;
    }
    schedule_work(&mut (*nu).write_work);
    0
}

unsafe extern "C" fn nci_uart_write_work(work: *mut work_struct) {
    let nu = container_of!(work, nci_uart, write_work);
    let tty = (*nu).tty;
    let mut skb: *mut sk_buff;

    'restart: loop {
        clear_bit(NCI_UART_TX_WAKEUP, &mut (*nu).tx_state);
        if let Some(tx_start) = (*nu).ops.tx_start { tx_start(nu); }

        loop {
            skb = nci_uart_dequeue(nu);
            if skb.is_null() { break; }
            set_bit(TTY_DO_WRITE_WAKEUP, &mut (*tty).flags);
            let len = ((*(*tty).ops).write)(tty, (*skb).data, (*skb).len);
            skb_pull(skb, len);
            if (*skb).len != 0 {
                (*nu).tx_skb = skb;
                break;
            }
            kfree_skb(skb);
        }

        if test_bit(NCI_UART_TX_WAKEUP, &(*nu).tx_state) != 0 { continue 'restart; }
        if let Some(tx_done) = (*nu).ops.tx_done {
            if nci_uart_queue_empty(nu) != 0 { tx_done(nu); }
        }
        clear_bit(NCI_UART_SENDING, &mut (*nu).tx_state);
        break;
    }
}

unsafe fn nci_uart_set_driver(tty: *mut tty_struct, driver: c_uint) -> i32 {
    if driver >= NCI_UART_DRIVER_MAX as c_uint { return -EINVAL; }
    let source = nci_uart_drivers[driver as usize];
    if source.is_null() { return -ENOENT; }
    let nu = kzalloc_obj::<nci_uart>();
    if nu.is_null() { return -ENOMEM; }
    core::ptr::copy_nonoverlapping(source, nu, 1);
    (*nu).tty = tty;
    skb_queue_head_init(&mut (*nu).tx_q);
    INIT_WORK(&mut (*nu).write_work, nci_uart_write_work);
    spin_lock_init(&mut (*nu).rx_lock);
    let ret = ((*nu).ops.open)(nu);
    if ret != 0 { kfree(nu); return ret; }
    if !try_module_get((*nu).owner) {
        ((*nu).ops.close)(nu); kfree(nu); return -ENOENT;
    }
    (*tty).disc_data = nu;
    0
}

/* ------ LDISC part ------ */

unsafe extern "C" fn nci_uart_tty_open(tty: *mut tty_struct) -> i32 {
    if (*(*tty).ops).write.is_none() { return -EOPNOTSUPP; }
    (*tty).disc_data = core::ptr::null_mut();
    (*tty).receive_room = 65536;
    tty_driver_flush_buffer(tty);
    0
}

unsafe extern "C" fn nci_uart_tty_close(tty: *mut tty_struct) {
    let nu = (*tty).disc_data;
    (*tty).disc_data = core::ptr::null_mut();
    if nu.is_null() { return; }
    kfree_skb((*nu).tx_skb); kfree_skb((*nu).rx_skb);
    skb_queue_purge(&mut (*nu).tx_q);
    ((*nu).ops.close)(nu); (*nu).tty = core::ptr::null_mut();
    module_put((*nu).owner); cancel_work_sync(&mut (*nu).write_work); kfree(nu);
}

unsafe extern "C" fn nci_uart_tty_wakeup(tty: *mut tty_struct) {
    let nu = (*tty).disc_data;
    if nu.is_null() { return; }
    clear_bit(TTY_DO_WRITE_WAKEUP, &mut (*tty).flags);
    if tty != (*nu).tty { return; }
    nci_uart_tx_wakeup(nu);
}

unsafe fn nci_uart_default_recv_buf(nu: *mut nci_uart, mut data: *const u8, mut count: i32) -> i32 {
    if (*nu).ndev.is_null() {
        nfc_err((*(*nu).tty).dev, "receive data from tty but no NCI dev is attached yet, drop buffer\n");
        return 0;
    }
    while count > 0 {
        if (*nu).rx_skb.is_null() {
            (*nu).rx_packet_len = -1;
            (*nu).rx_skb = nci_skb_alloc((*nu).ndev, NCI_MAX_PACKET_SIZE, GFP_ATOMIC);
            if (*nu).rx_skb.is_null() { return -ENOMEM; }
        }
        if (*(*nu).rx_skb).len < NCI_CTRL_HDR_SIZE {
            skb_put_u8((*nu).rx_skb, *data); data = data.add(1); count -= 1; continue;
        }
        if (*nu).rx_packet_len < 0 {
            (*nu).rx_packet_len = NCI_CTRL_HDR_SIZE + nci_plen((*(*nu).rx_skb).data);
        }
        let mut chunk_len = (*nu).rx_packet_len - (*(*nu).rx_skb).len as i32;
        if count < chunk_len { chunk_len = count; }
        skb_put_data((*nu).rx_skb, data, chunk_len as usize);
        data = data.add(chunk_len as usize); count -= chunk_len;
        if (*nu).rx_packet_len == (*(*nu).rx_skb).len as i32 {
            if ((*nu).ops.recv)(nu, (*nu).rx_skb) != 0 { nfc_err((*(*nu).tty).dev, "corrupted RX packet\n"); }
            (*nu).rx_skb = core::ptr::null_mut();
        }
    }
    0
}

unsafe extern "C" fn nci_uart_tty_receive(tty: *mut tty_struct, data: *const u8, _flags: *const u8, count: usize) {
    let nu = (*tty).disc_data;
    if nu.is_null() || tty != (*nu).tty { return; }
    spin_lock(&mut (*nu).rx_lock); nci_uart_default_recv_buf(nu, data, count as i32); spin_unlock(&mut (*nu).rx_lock);
    tty_unthrottle(tty);
}

unsafe extern "C" fn nci_uart_tty_ioctl(tty: *mut tty_struct, cmd: c_uint, arg: c_ulong) -> i32 {
    let nu = (*tty).disc_data;
    match cmd {
        NCIUARTSETDRIVER => if nu.is_null() { nci_uart_set_driver(tty, arg as c_uint) } else { -EBUSY },
        _ => n_tty_ioctl_helper(tty, cmd, arg),
    }
}

unsafe extern "C" fn nci_uart_tty_read(_tty: *mut tty_struct, _file: *mut file, _buf: *mut u8, _nr: usize, _cookie: *mut *mut c_void, _offset: c_ulong) -> isize { 0 }
unsafe extern "C" fn nci_uart_tty_write(_tty: *mut tty_struct, _file: *mut file, _data: *const u8, _count: usize) -> isize { 0 }

unsafe fn nci_uart_send(nu: *mut nci_uart, skb: *mut sk_buff) -> i32 {
    skb_queue_tail(&mut (*nu).tx_q, skb); nci_uart_tx_wakeup(nu); 0
}

#[no_mangle]
pub unsafe extern "C" fn nci_uart_register(nu: *mut nci_uart) -> i32 {
    if nu.is_null() || (*nu).ops.open.is_none() || (*nu).ops.recv.is_none() || (*nu).ops.close.is_none() { return -EINVAL; }
    (*nu).ops.send = Some(nci_uart_send);
    if !nci_uart_drivers[(*nu).driver as usize].is_null() { pr_err!("driver {} is already registered\n", (*nu).driver); return -EBUSY; }
    nci_uart_drivers[(*nu).driver as usize] = nu;
    pr_info!("NCI uart driver '{}' registered\n", (*nu).name, (*nu).driver); 0
}

#[no_mangle]
pub unsafe extern "C" fn nci_uart_unregister(nu: *mut nci_uart) {
    pr_info!("NCI uart driver '{}' unregistered\n", (*nu).name, (*nu).driver);
    nci_uart_drivers[(*nu).driver as usize] = core::ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn nci_uart_set_config(nu: *mut nci_uart, baudrate: i32, flow_ctrl: i32) {
    if (*nu).tty.is_null() { return; }
    down_read(&(*(*nu).tty).termios_rwsem); let mut new_termios = (*(*nu).tty).termios; up_read(&(*(*nu).tty).termios_rwsem);
    tty_termios_encode_baud_rate(&mut new_termios, baudrate, baudrate);
    if flow_ctrl != 0 { new_termios.c_cflag |= CRTSCTS; } else { new_termios.c_cflag &= !CRTSCTS; }
    tty_set_termios((*nu).tty, &new_termios);
}

static nci_uart_ldisc: tty_ldisc_ops = tty_ldisc_ops {
    owner: THIS_MODULE, num: N_NCI, name: "n_nci", open: Some(nci_uart_tty_open), close: Some(nci_uart_tty_close),
    read: Some(nci_uart_tty_read), write: Some(nci_uart_tty_write), receive_buf: Some(nci_uart_tty_receive),
    write_wakeup: Some(nci_uart_tty_wakeup), ioctl: Some(nci_uart_tty_ioctl), compat_ioctl: Some(nci_uart_tty_ioctl),
};

unsafe extern "C" fn nci_uart_init() -> i32 { tty_register_ldisc(&nci_uart_ldisc) }
unsafe extern "C" fn nci_uart_exit() { tty_unregister_ldisc(&nci_uart_ldisc); }

module_init!(nci_uart_init);
module_exit!(nci_uart_exit);
module_author!("Marvell International Ltd.");
module_description!("NFC NCI UART driver");
module_license!("GPL");
module_alias_ldisc!(N_NCI);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
