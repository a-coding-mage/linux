// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Bluetooth HCI serdev driver lib
 *
 *  Copyright (C) 2017  Linaro, Ltd., Rob Herring <robh@kernel.org>
 *
 *  Based on hci_ldisc.c:
 *
 *  Copyright (C) 2000-2001  Qualcomm Incorporated
 *  Copyright (C) 2002-2003  Maxim Krasnyansky <maxk@qualcomm.com>
 *  Copyright (C) 2004-2005  Marcel Holtmann <marcel@holtmann.org>
 */

// Kernel dependencies supplied externally.

unsafe fn hci_uart_tx_complete(hu: *mut hci_uart, pkt_type: i32) {
    let hdev = (*hu).hdev;

    /* Update HCI stat counters */
    match pkt_type {
        HCI_COMMAND_PKT => (*hdev).stat.cmd_tx += 1,
        HCI_ACLDATA_PKT => (*hdev).stat.acl_tx += 1,
        HCI_SCODATA_PKT => (*hdev).stat.sco_tx += 1,
        _ => {}
    }
}

unsafe fn hci_uart_dequeue(hu: *mut hci_uart) -> *mut sk_buff {
    let mut skb = (*hu).tx_skb;

    if skb.is_null() {
        if test_bit(HCI_UART_PROTO_READY, &(*hu).flags) {
            skb = ((*(*hu).proto).dequeue)(hu);
        }
    } else {
        (*hu).tx_skb = core::ptr::null_mut();
    }

    skb
}

unsafe fn hci_uart_write_work(work: *mut work_struct) {
    let hu = container_of!(work, hci_uart, write_work);
    let serdev = (*hu).serdev;
    let hdev = (*hu).hdev;
    let mut skb: *mut sk_buff;

    /* REVISIT:
     * should we cope with bad skbs or ->write() returning an error value?
     */
    loop {
        clear_bit(HCI_UART_TX_WAKEUP, &mut (*hu).tx_state);

        while {
            skb = hci_uart_dequeue(hu);
            !skb.is_null()
        } {
            let len: i32;

            len = serdev_device_write_buf(serdev, (*skb).data, (*skb).len);
            (*hdev).stat.byte_tx += len as u64;

            skb_pull(skb, len);
            if (*skb).len != 0 {
                (*hu).tx_skb = skb;
                break;
            }

            hci_uart_tx_complete(hu, hci_skb_pkt_type(skb));
            kfree_skb(skb);
        }

        clear_bit(HCI_UART_SENDING, &mut (*hu).tx_state);
        if !test_bit(HCI_UART_TX_WAKEUP, &(*hu).tx_state) {
            break;
        }
    }
}

/* ------- Interface to HCI layer ------ */

/* Reset device */
unsafe fn hci_uart_flush(hdev: *mut hci_dev) -> i32 {
    let hu = hci_get_drvdata(hdev);

    BT_DBG!("hdev %p serdev %p", hdev, (*hu).serdev);

    if !(*hu).tx_skb.is_null() {
        kfree_skb((*hu).tx_skb);
        (*hu).tx_skb = core::ptr::null_mut();
    }

    /* Flush any pending characters in the driver and discipline. */
    serdev_device_write_flush((*hu).serdev);

    if test_bit(HCI_UART_PROTO_READY, &(*hu).flags) {
        ((*(*hu).proto).flush)(hu);
    }

    0
}

/* Initialize device */
unsafe fn hci_uart_open(hdev: *mut hci_dev) -> i32 {
    let hu = hci_get_drvdata(hdev);
    let err: i32;

    BT_DBG!("%s %p", (*hdev).name, hdev);

    /* When Quirk HCI_QUIRK_NON_PERSISTENT_SETUP is set by
     * driver, BT SoC is completely turned OFF during
     * BT OFF. Upon next BT ON UART port should be opened.
     */
    if !test_bit(HCI_UART_PROTO_READY, &(*hu).flags) {
        err = serdev_device_open((*hu).serdev);
        if err != 0 {
            return err;
        }
        set_bit(HCI_UART_PROTO_READY, &mut (*hu).flags);
    }

    /* Undo clearing this from hci_uart_close() */
    (*hdev).flush = Some(hci_uart_flush);

    0
}

/* Close device */
unsafe fn hci_uart_close(hdev: *mut hci_dev) -> i32 {
    let hu = hci_get_drvdata(hdev);

    BT_DBG!("hdev %p", hdev);

    if !test_bit(HCI_UART_PROTO_READY, &(*hu).flags) {
        return 0;
    }

    hci_uart_flush(hdev);
    (*hdev).flush = None;

    /* When QUIRK HCI_QUIRK_NON_PERSISTENT_SETUP is set by driver,
     * BT SOC is completely powered OFF during BT OFF, holding port
     * open may drain the battery.
     */
    if hci_test_quirk(hdev, HCI_QUIRK_NON_PERSISTENT_SETUP) {
        clear_bit(HCI_UART_PROTO_READY, &mut (*hu).flags);
        serdev_device_close((*hu).serdev);
    }

    0
}

/* Send frames from HCI layer */
unsafe fn hci_uart_send_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> i32 {
    let hu = hci_get_drvdata(hdev);

    BT_DBG!("%s: type %d len %d", (*hdev).name, hci_skb_pkt_type(skb), (*skb).len);

    ((*(*hu).proto).enqueue)(hu, skb);
    hci_uart_tx_wakeup(hu);
    0
}

unsafe fn hci_uart_setup(hdev: *mut hci_dev) -> i32 {
    let hu = hci_get_drvdata(hdev);
    let ver: *mut hci_rp_read_local_version;
    let skb: *mut sk_buff;
    let speed: u32;
    let err: i32;

    /* Init speed if any */
    if (*hu).init_speed != 0 {
        speed = (*hu).init_speed;
    } else if (*(*hu).proto).init_speed != 0 {
        speed = (*(*hu).proto).init_speed;
    } else {
        speed = 0;
    }

    if speed != 0 {
        serdev_device_set_baudrate((*hu).serdev, speed);
    }

    /* Operational speed if any */
    let speed = if (*hu).oper_speed != 0 {
        (*hu).oper_speed
    } else if (*(*hu).proto).oper_speed != 0 {
        (*(*hu).proto).oper_speed
    } else {
        0
    };

    if (*(*hu).proto).set_baudrate.is_some() && speed != 0 {
        err = ((*(*hu).proto).set_baudrate.unwrap())(hu, speed);
        if err != 0 {
            bt_dev_err!(hdev, "Failed to set baudrate");
        } else {
            serdev_device_set_baudrate((*hu).serdev, speed);
        }
    }

    if (*(*hu).proto).setup.is_some() {
        return ((*(*hu).proto).setup.unwrap())(hu);
    }

    if !test_bit(HCI_UART_VND_DETECT, &(*hu).hdev_flags) {
        return 0;
    }

    skb = __hci_cmd_sync(hdev, HCI_OP_READ_LOCAL_VERSION, 0, core::ptr::null_mut(), HCI_INIT_TIMEOUT);
    if IS_ERR(skb) {
        bt_dev_err!(hdev, "Reading local version info failed (%ld)", PTR_ERR(skb));
        return PTR_ERR(skb);
    }

    if (*skb).len != core::mem::size_of_val(&*ver) {
        bt_dev_err!(hdev, "Event length mismatch for version info");
    }

    kfree_skb(skb);
    0
}

/* Check if the device is wakeable */
unsafe fn hci_uart_wakeup(_hdev: *mut hci_dev) -> bool {
    /* HCI UART devices are assumed to be wakeable by default.
     * Implement wakeup callback to override this behavior.
     */
    true
}

/** hci_uart_write_wakeup - transmit buffer wakeup
 * @serdev: serial device
 *
 * This function is called by the serdev framework when it accepts
 * more data being sent.
 */
unsafe fn hci_uart_write_wakeup(serdev: *mut serdev_device) {
    let hu = serdev_device_get_drvdata(serdev);

    BT_DBG!("");

    if hu.is_null() || serdev != (*hu).serdev {
        WARN_ON!(1);
        return;
    }

    if test_bit(HCI_UART_PROTO_READY, &(*hu).flags) {
        hci_uart_tx_wakeup(hu);
    }
}

/** hci_uart_receive_buf - receive buffer wakeup
 * @serdev: serial device
 * @data:   pointer to received data
 * @count:  count of received data in bytes
 *
 * This function is called by the serdev framework when it received data
 * in the RX buffer.
 *
 * Return: number of processed bytes
 */
unsafe fn hci_uart_receive_buf(serdev: *mut serdev_device, data: *const u8, count: usize) -> usize {
    let hu = serdev_device_get_drvdata(serdev);

    if hu.is_null() || serdev != (*hu).serdev {
        WARN_ON!(1);
        return 0;
    }

    if !test_bit(HCI_UART_PROTO_READY, &(*hu).flags) {
        return 0;
    }

    /* It does not need a lock here as it is already protected by a mutex in
     * tty caller
     */
    ((*(*hu).proto).recv)(hu, data, count);

    if !(*hu).hdev.is_null() {
        (*(*hu).hdev).stat.byte_rx += count as u64;
    }

    count
}

static HCI_SERDEV_CLIENT_OPS: serdev_device_ops = serdev_device_ops {
    receive_buf: Some(hci_uart_receive_buf),
    write_wakeup: Some(hci_uart_write_wakeup),
};

unsafe fn hci_uart_register_device_priv(hu: *mut hci_uart, p: *const hci_uart_proto, sizeof_priv: i32) -> i32 {
    let mut err: i32;
    let hdev: *mut hci_dev;

    BT_DBG!("");
    serdev_device_set_client_ops((*hu).serdev, &HCI_SERDEV_CLIENT_OPS);

    if percpu_init_rwsem(&mut (*hu).proto_lock) != 0 {
        return -ENOMEM;
    }

    err = serdev_device_open((*hu).serdev);
    if err != 0 { goto_err_rwsem!(err); }
    err = ((*p).open)(hu);
    if err != 0 { goto_err_open!(err); }

    (*hu).proto = p;
    set_bit(HCI_UART_PROTO_READY, &mut (*hu).flags);

    /* Initialize and register HCI device */
    hdev = hci_alloc_dev_priv(sizeof_priv);
    if hdev.is_null() {
        BT_ERR!("Can't allocate HCI device");
        err = -ENOMEM;
        goto_err_alloc!(err);
    }

    (*hu).hdev = hdev;
    (*hdev).bus = HCI_UART;
    hci_set_drvdata(hdev, hu);
    INIT_WORK!(&mut (*hu).init_ready, hci_uart_init_work);
    INIT_WORK!(&mut (*hu).write_work, hci_uart_write_work);

    /* Only when vendor specific setup callback is provided, consider
     * the manufacturer information valid. This avoids filling in the
     * value for Ericsson when nothing is specified.
     */
    if (*(*hu).proto).setup.is_some() {
        (*hdev).manufacturer = (*(*hu).proto).manufacturer;
    }

    (*hdev).open = Some(hci_uart_open);
    (*hdev).close = Some(hci_uart_close);
    (*hdev).flush = Some(hci_uart_flush);
    (*hdev).send = Some(hci_uart_send_frame);
    (*hdev).setup = Some(hci_uart_setup);
    if (*hdev).wakeup.is_none() { (*hdev).wakeup = Some(hci_uart_wakeup); }
    SET_HCIDEV_DEV!(hdev, &mut (*(*hu).serdev).dev);

    if test_bit(HCI_UART_NO_SUSPEND_NOTIFIER, &(*hu).flags) { hci_set_quirk(hdev, HCI_QUIRK_NO_SUSPEND_NOTIFIER); }
    if test_bit(HCI_UART_RAW_DEVICE, &(*hu).hdev_flags) { hci_set_quirk(hdev, HCI_QUIRK_RAW_DEVICE); }
    if test_bit(HCI_UART_EXT_CONFIG, &(*hu).hdev_flags) { hci_set_quirk(hdev, HCI_QUIRK_EXTERNAL_CONFIG); }
    if test_bit(HCI_UART_INIT_PENDING, &(*hu).hdev_flags) { return 0; }

    if hci_register_dev(hdev) < 0 {
        BT_ERR!("Can't register HCI device");
        err = -ENODEV;
        hci_free_dev(hdev);
        clear_bit(HCI_UART_PROTO_READY, &mut (*hu).flags);
        ((*p).close)(hu);
        serdev_device_close((*hu).serdev);
        percpu_free_rwsem(&mut (*hu).proto_lock);
        return err;
    }

    set_bit(HCI_UART_REGISTERED, &mut (*hu).flags);
    0
}

unsafe fn hci_uart_unregister_device(hu: *mut hci_uart) {
    let hdev = (*hu).hdev;

    cancel_work_sync(&mut (*hu).init_ready);
    if test_bit(HCI_UART_REGISTERED, &(*hu).flags) { hci_unregister_dev(hdev); }
    hci_free_dev(hdev);
    cancel_work_sync(&mut (*hu).write_work);
    ((*(*hu).proto).close)(hu);

    if test_bit(HCI_UART_PROTO_READY, &(*hu).flags) {
        clear_bit(HCI_UART_PROTO_READY, &mut (*hu).flags);
        serdev_device_close((*hu).serdev);
    }
    percpu_free_rwsem(&mut (*hu).proto_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
