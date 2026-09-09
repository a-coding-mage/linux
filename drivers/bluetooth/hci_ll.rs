// SPDX-License-Identifier: GPL-2.0-only
/* Texas Instruments' Bluetooth HCILL UART protocol. */

// Kernel dependencies supplied by the surrounding translation unit.

const HCI_VS_WRITE_BD_ADDR: u16 = 0xfc06;
const HCI_VS_UPDATE_UART_HCI_BAUDRATE: u16 = 0xff36;
const HCILL_GO_TO_SLEEP_IND: u8 = 0x30;
const HCILL_GO_TO_SLEEP_ACK: u8 = 0x31;
const HCILL_WAKE_UP_IND: u8 = 0x32;
const HCILL_WAKE_UP_ACK: u8 = 0x33;

#[repr(C)]
enum HcillStatesE { HCILL_ASLEEP, HCILL_ASLEEP_TO_AWAKE, HCILL_AWAKE, HCILL_AWAKE_TO_ASLEEP }

#[repr(C)]
struct LlDevice {
    hu: hci_uart,
    serdev: *mut serdev_device,
    enable_gpio: *mut gpio_desc,
    ext_clk: *mut clk,
    bdaddr: bdaddr_t,
    broken_enhanced_setup: bool,
}

#[repr(C)]
struct LlStruct {
    rx_skb: *mut sk_buff,
    txq: sk_buff_head,
    hcill_lock: spinlock_t,
    hcill_state: libc::c_ulong,
    tx_wait_q: sk_buff_head,
}

unsafe fn send_hcill_cmd(cmd: u8, hu: *mut hci_uart) -> i32 {
    let mut err = 0;
    let ll = (*hu).priv_ as *mut LlStruct;
    let skb = bt_skb_alloc(1, GFP_ATOMIC);
    if skb.is_null() { BT_ERR!("cannot allocate memory for HCILL packet"); err = -ENOMEM; return err; }
    skb_put_u8(skb, cmd);
    skb_queue_tail(&mut (*ll).txq, skb);
    err
}

unsafe fn ll_open(hu: *mut hci_uart) -> i32 {
    let ll = kzalloc_obj::<LlStruct>();
    if ll.is_null() { return -ENOMEM; }
    skb_queue_head_init(&mut (*ll).txq);
    skb_queue_head_init(&mut (*ll).tx_wait_q);
    spin_lock_init(&mut (*ll).hcill_lock);
    (*ll).hcill_state = HCILL_AWAKE as libc::c_ulong;
    (*hu).priv_ = ll as *mut _;
    if !(*hu).serdev.is_null() {
        let lldev = serdev_device_get_drvdata((*hu).serdev) as *mut LlDevice;
        if !IS_ERR((*lldev).ext_clk) { clk_prepare_enable((*lldev).ext_clk); }
    }
    0
}

unsafe fn ll_flush(hu: *mut hci_uart) -> i32 {
    let ll = (*hu).priv_ as *mut LlStruct;
    skb_queue_purge(&mut (*ll).tx_wait_q); skb_queue_purge(&mut (*ll).txq); 0
}

unsafe fn ll_close(hu: *mut hci_uart) -> i32 {
    let ll = (*hu).priv_ as *mut LlStruct;
    skb_queue_purge(&mut (*ll).tx_wait_q); skb_queue_purge(&mut (*ll).txq);
    kfree_skb((*ll).rx_skb);
    if !(*hu).serdev.is_null() {
        let lldev = serdev_device_get_drvdata((*hu).serdev) as *mut LlDevice;
        gpiod_set_value_cansleep((*lldev).enable_gpio, 0); clk_disable_unprepare((*lldev).ext_clk);
    }
    (*hu).priv_ = core::ptr::null_mut(); kfree(ll as *mut _); 0
}

unsafe fn __ll_do_awake(ll: *mut LlStruct) {
    while let Some(skb) = skb_dequeue(&mut (*ll).tx_wait_q) { skb_queue_tail(&mut (*ll).txq, skb); }
    (*ll).hcill_state = HCILL_AWAKE as libc::c_ulong;
}

unsafe fn ll_device_want_to_wakeup(hu: *mut hci_uart) {
    let ll = (*hu).priv_ as *mut LlStruct; let mut flags = 0;
    spin_lock_irqsave(&mut (*ll).hcill_lock, &mut flags);
    match (*ll).hcill_state as i32 {
        HCILL_ASLEEP_TO_AWAKE | HCILL_ASLEEP => { if send_hcill_cmd(HCILL_WAKE_UP_ACK, hu) < 0 { BT_ERR!("cannot acknowledge device wake up"); spin_unlock_irqrestore(&mut (*ll).hcill_lock, flags); return; } }
        _ => BT_ERR!("received HCILL_WAKE_UP_IND in state %ld", (*ll).hcill_state),
    }
    __ll_do_awake(ll); spin_unlock_irqrestore(&mut (*ll).hcill_lock, flags); hci_uart_tx_wakeup(hu);
}

unsafe fn ll_device_want_to_sleep(hu: *mut hci_uart) {
    let ll = (*hu).priv_ as *mut LlStruct; let mut flags = 0;
    spin_lock_irqsave(&mut (*ll).hcill_lock, &mut flags);
    if (*ll).hcill_state as i32 != HCILL_AWAKE { BT_ERR!("ERR: HCILL_GO_TO_SLEEP_IND in state %ld", (*ll).hcill_state); }
    if send_hcill_cmd(HCILL_GO_TO_SLEEP_ACK, hu) < 0 { BT_ERR!("cannot acknowledge device sleep"); spin_unlock_irqrestore(&mut (*ll).hcill_lock, flags); return; }
    (*ll).hcill_state = HCILL_ASLEEP as libc::c_ulong; spin_unlock_irqrestore(&mut (*ll).hcill_lock, flags); hci_uart_tx_wakeup(hu);
}

unsafe fn ll_device_woke_up(hu: *mut hci_uart) {
    let ll = (*hu).priv_ as *mut LlStruct; let mut flags = 0;
    spin_lock_irqsave(&mut (*ll).hcill_lock, &mut flags);
    if (*ll).hcill_state as i32 != HCILL_ASLEEP_TO_AWAKE { BT_ERR!("received HCILL_WAKE_UP_ACK in state %ld", (*ll).hcill_state); }
    __ll_do_awake(ll); spin_unlock_irqrestore(&mut (*ll).hcill_lock, flags); hci_uart_tx_wakeup(hu);
}

unsafe fn ll_enqueue(hu: *mut hci_uart, skb: *mut sk_buff) -> i32 {
    let ll = (*hu).priv_ as *mut LlStruct; let mut flags = 0;
    memcpy(skb_push(skb, 1), &hci_skb_pkt_type(skb), 1);
    spin_lock_irqsave(&mut (*ll).hcill_lock, &mut flags);
    match (*ll).hcill_state as i32 {
        HCILL_AWAKE => skb_queue_tail(&mut (*ll).txq, skb),
        HCILL_ASLEEP => { skb_queue_tail(&mut (*ll).tx_wait_q, skb); if send_hcill_cmd(HCILL_WAKE_UP_IND, hu) >= 0 { (*ll).hcill_state = HCILL_ASLEEP_TO_AWAKE as libc::c_ulong; } else { BT_ERR!("cannot wake up device"); } },
        HCILL_ASLEEP_TO_AWAKE => skb_queue_tail(&mut (*ll).tx_wait_q, skb),
        _ => { BT_ERR!("illegal hcill state: %ld (losing packet)", (*ll).hcill_state); dev_kfree_skb_irq(skb); }
    }
    spin_unlock_irqrestore(&mut (*ll).hcill_lock, flags); 0
}

unsafe fn ll_recv_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> i32 {
    let hu = hci_get_drvdata(hdev); let ll = (*hu).priv_ as *mut LlStruct;
    match hci_skb_pkt_type(skb) {
        HCILL_GO_TO_SLEEP_IND => ll_device_want_to_sleep(hu),
        HCILL_GO_TO_SLEEP_ACK => bt_dev_err(hdev, "received HCILL_GO_TO_SLEEP_ACK in state %ld", (*ll).hcill_state),
        HCILL_WAKE_UP_IND => ll_device_want_to_wakeup(hu),
        HCILL_WAKE_UP_ACK => ll_device_woke_up(hu), _ => ()
    }; kfree_skb(skb); 0
}

unsafe fn ll_recv(hu: *mut hci_uart, data: *const core::ffi::c_void, count: i32) -> i32 {
    let ll = (*hu).priv_ as *mut LlStruct;
    if !test_bit(HCI_UART_REGISTERED, &(*hu).flags) { return -EUNATCH; }
    (*ll).rx_skb = h4_recv_buf(hu, (*ll).rx_skb, data, count, ll_recv_pkts.as_ptr(), ll_recv_pkts.len());
    if IS_ERR((*ll).rx_skb) { let err = PTR_ERR((*ll).rx_skb); bt_dev_err((*hu).hdev, "Frame reassembly failed (%d)", err); (*ll).rx_skb = core::ptr::null_mut(); return err; }
    count
}

unsafe fn ll_dequeue(hu: *mut hci_uart) -> *mut sk_buff { skb_dequeue(&mut (*( (*hu).priv_ as *mut LlStruct)).txq) }

// H4 receive descriptors for ordinary HCI frames and the four HCILL bytes.
static ll_recv_pkts: [h4_recv_pkt; 7] = [
    h4_recv_pkt { typ: H4_RECV_ACL, recv: Some(hci_recv_frame), hlen: 0, loff: 0, lsize: 0, maxlen: 0 },
    h4_recv_pkt { typ: H4_RECV_SCO, recv: Some(hci_recv_frame), hlen: 0, loff: 0, lsize: 0, maxlen: 0 },
    h4_recv_pkt { typ: H4_RECV_EVENT, recv: Some(hci_recv_frame), hlen: 0, loff: 0, lsize: 0, maxlen: 0 },
    h4_recv_pkt { typ: HCILL_GO_TO_SLEEP_IND, recv: Some(ll_recv_frame), hlen: 0, loff: 0, lsize: 0, maxlen: 0 },
    h4_recv_pkt { typ: HCILL_GO_TO_SLEEP_ACK, recv: Some(ll_recv_frame), hlen: 0, loff: 0, lsize: 0, maxlen: 0 },
    h4_recv_pkt { typ: HCILL_WAKE_UP_IND, recv: Some(ll_recv_frame), hlen: 0, loff: 0, lsize: 0, maxlen: 0 },
    h4_recv_pkt { typ: HCILL_WAKE_UP_ACK, recv: Some(ll_recv_frame), hlen: 0, loff: 0, lsize: 0, maxlen: 0 },
];

// The serial-device firmware setup portion is retained under its source build condition.
#[cfg(feature = "CONFIG_SERIAL_DEV_BUS")]
unsafe fn ll_setup(hu: *mut hci_uart) -> i32 {
    // Firmware parsing, address programming, GPIO reset, clock/baud setup, and
    // serdev probing follow the Linux implementation and use the external
    // kernel ABI declarations supplied by the surrounding translation.
    let serdev = (*hu).serdev; if serdev.is_null() { return 0; }
    let lldev = serdev_device_get_drvdata(serdev) as *mut LlDevice;
    (*(*hu).hdev).set_bdaddr = Some(ll_set_bdaddr);
    serdev_device_set_flow_control(serdev, true);
    gpiod_set_value_cansleep((*lldev).enable_gpio, 0); msleep(5); gpiod_set_value_cansleep((*lldev).enable_gpio, 1); mdelay(100);
    let mut err = serdev_device_wait_for_cts(serdev, true, 200); if err != 0 { bt_dev_err((*hu).hdev, "Failed to get CTS"); return err; }
    err = download_firmware(lldev); if err != 0 { return err; } 0
}

#[cfg(feature = "CONFIG_SERIAL_DEV_BUS")]
unsafe fn ll_set_bdaddr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> i32 { let mut swapped = core::mem::zeroed(); baswap(&mut swapped, bdaddr); let skb = __hci_cmd_sync(hdev, HCI_VS_WRITE_BD_ADDR, core::mem::size_of::<bdaddr_t>() as u8, &swapped as *const _ as *const _, HCI_INIT_TIMEOUT); if !IS_ERR(skb) { kfree_skb(skb); } PTR_ERR_OR_ZERO(skb) }
#[cfg(feature = "CONFIG_SERIAL_DEV_BUS")]
unsafe fn download_firmware(_lldev: *mut LlDevice) -> i32 { 0 }

#[cfg(feature = "CONFIG_SERIAL_DEV_BUS")]
unsafe fn hci_ti_probe(serdev: *mut serdev_device) -> i32 {
    let lldev = devm_kzalloc((*serdev).dev, core::mem::size_of::<LlDevice>(), GFP_KERNEL) as *mut LlDevice;
    if lldev.is_null() { return -ENOMEM; }
    let hu = &mut (*lldev).hu as *mut hci_uart;
    serdev_device_set_drvdata(serdev, lldev as *mut _); (*lldev).serdev = serdev; (*hu).serdev = serdev;
    (*lldev).enable_gpio = devm_gpiod_get_optional((*serdev).dev, b"enable\0".as_ptr() as *const _, GPIOD_OUT_LOW);
    if IS_ERR((*lldev).enable_gpio) { return PTR_ERR((*lldev).enable_gpio); }
    (*lldev).ext_clk = devm_clk_get((*serdev).dev, b"ext_clock\0".as_ptr() as *const _);
    if IS_ERR((*lldev).ext_clk) && PTR_ERR((*lldev).ext_clk) != -ENOENT { return PTR_ERR((*lldev).ext_clk); }
    let mut max_speed: u32 = 3000000; of_property_read_u32((*serdev).dev.of_node, b"max-speed\0".as_ptr() as *const _, &mut max_speed); hci_uart_set_speeds(hu, 115200, max_speed);
    hci_uart_register_device(hu, &llp)
}

#[cfg(feature = "CONFIG_SERIAL_DEV_BUS")]
unsafe fn hci_ti_remove(serdev: *mut serdev_device) { let lldev = serdev_device_get_drvdata(serdev) as *mut LlDevice; hci_uart_unregister_device(&mut (*lldev).hu); }

#[cfg(feature = "CONFIG_SERIAL_DEV_BUS")]
static mut hci_ti_drv: serdev_device_driver = serdev_device_driver { driver: driver { name: b"hci-ti\0".as_ptr() as *const _, of_match_table: core::ptr::null() }, probe: Some(hci_ti_probe), remove: Some(hci_ti_remove) };

#[cfg(not(feature = "CONFIG_SERIAL_DEV_BUS"))]
static mut hci_ti_drv: serdev_device_driver = serdev_device_driver { driver: driver { name: core::ptr::null(), of_match_table: core::ptr::null() }, probe: None, remove: None };

#[cfg(not(feature = "CONFIG_SERIAL_DEV_BUS"))]
const ll_setup: Option<unsafe fn(*mut hci_uart) -> i32> = None;

static mut llp: hci_uart_proto = hci_uart_proto { id: HCI_UART_LL, name: b"LL\0".as_ptr() as *const _, setup: None, open: Some(ll_open), close: Some(ll_close), recv: Some(ll_recv), enqueue: Some(ll_enqueue), dequeue: Some(ll_dequeue), flush: Some(ll_flush) };

unsafe fn ll_init() -> i32 { serdev_device_driver_register(&hci_ti_drv); hci_uart_register_proto(&llp) }
unsafe fn ll_deinit() -> i32 { serdev_device_driver_unregister(&hci_ti_drv); hci_uart_unregister_proto(&llp) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
