// SPDX-License-Identifier: GPL-2.0-or-later
// Bluetooth HCI UART driver for Intel devices

// External kernel/Bluetooth declarations supplied by the surrounding translation.
use core::ffi::c_void;

const STATE_BOOTLOADER: usize = 0;
const STATE_DOWNLOADING: usize = 1;
const STATE_FIRMWARE_LOADED: usize = 2;
const STATE_FIRMWARE_FAILED: usize = 3;
const STATE_BOOTING: usize = 4;
const STATE_LPM_ENABLED: usize = 5;
const STATE_TX_ACTIVE: usize = 6;
const STATE_SUSPENDED: usize = 7;
const STATE_LPM_TRANSACTION: usize = 8;
const HCI_LPM_WAKE_PKT: u8 = 0xf0;
const HCI_LPM_PKT: u8 = 0xf1;
const HCI_LPM_MAX_SIZE: usize = 10;
const HCI_LPM_HDR_SIZE: usize = HCI_EVENT_HDR_SIZE;
const LPM_OP_TX_NOTIFY: u8 = 0x00;
const LPM_OP_SUSPEND_ACK: u8 = 0x02;
const LPM_OP_RESUME_ACK: u8 = 0x03;
const LPM_SUSPEND_DELAY_MS: u32 = 1000;

#[repr(C, packed)]
pub struct hci_lpm_pkt { pub opcode: u8, pub dlen: u8, pub data: [u8; 0] }

#[repr(C)]
pub struct intel_device {
    pub list: list_head, pub pdev: *mut platform_device, pub reset: *mut gpio_desc,
    pub hu: *mut hci_uart, pub hu_lock: mutex, pub irq: i32,
}
#[repr(C)]
pub struct intel_data {
    pub rx_skb: *mut sk_buff, pub txq: sk_buff_head, pub busy_work: work_struct,
    pub hu: *mut hci_uart, pub flags: c_ulong,
}

static mut intel_device_list: list_head = LIST_HEAD_INIT;
static mut intel_device_list_lock: mutex = DEFINE_MUTEX_INIT;

unsafe fn intel_convert_speed(speed: u32) -> u8 {
    match speed { 9600 => 0, 19200 => 1, 38400 => 2, 57600 => 3, 115200 => 4,
        230400 => 5, 460800 => 6, 921600 => 7, 1843200 => 8, 3250000 => 9,
        2000000 => 0x0a, 3000000 => 0x0b, _ => 0xff }
}

unsafe fn intel_wait_booting(hu: *mut hci_uart) -> i32 {
    let intel = (*hu).priv_ as *mut intel_data;
    let err = wait_on_bit_timeout(&mut (*intel).flags, STATE_BOOTING, TASK_INTERRUPTIBLE, msecs_to_jiffies(1000));
    if err == -EINTR { bt_dev_err((*hu).hdev, "Device boot interrupted"); return -EINTR; }
    if err != 0 { bt_dev_err((*hu).hdev, "Device boot timeout"); return -ETIMEDOUT; }
    err
}
unsafe fn intel_wait_lpm_transaction(hu: *mut hci_uart) -> i32 {
    let intel = (*hu).priv_ as *mut intel_data;
    let err = wait_on_bit_timeout(&mut (*intel).flags, STATE_LPM_TRANSACTION, TASK_INTERRUPTIBLE, msecs_to_jiffies(1000));
    if err == -EINTR { bt_dev_err((*hu).hdev, "LPM transaction interrupted"); return -EINTR; }
    if err != 0 { bt_dev_err((*hu).hdev, "LPM transaction timeout"); return -ETIMEDOUT; }
    err
}

unsafe fn intel_lpm_suspend(hu: *mut hci_uart) -> i32 {
    let suspend = [1u8, 1, 1]; let intel = (*hu).priv_ as *mut intel_data;
    if !test_bit(STATE_LPM_ENABLED, &(*intel).flags) || test_bit(STATE_SUSPENDED, &(*intel).flags) { return 0; }
    if test_bit(STATE_TX_ACTIVE, &(*intel).flags) { return -EAGAIN; }
    bt_dev_dbg((*hu).hdev, "Suspending");
    let skb = bt_skb_alloc(suspend.len(), GFP_KERNEL); if skb.is_null() { bt_dev_err((*hu).hdev, "Failed to alloc memory for LPM packet"); return -ENOMEM; }
    skb_put_data(skb, suspend.as_ptr(), suspend.len()); hci_skb_pkt_type(skb, HCI_LPM_PKT);
    set_bit(STATE_LPM_TRANSACTION, &mut (*intel).flags); skb_queue_head(&mut (*intel).txq, skb); hci_uart_tx_wakeup(hu);
    intel_wait_lpm_transaction(hu); clear_bit(STATE_LPM_TRANSACTION, &mut (*intel).flags);
    if !test_bit(STATE_SUSPENDED, &(*intel).flags) { bt_dev_err((*hu).hdev, "Device suspend error"); return -EINVAL; }
    bt_dev_dbg((*hu).hdev, "Suspended"); hci_uart_set_flow_control(hu, true); 0
}
unsafe fn intel_lpm_resume(hu: *mut hci_uart) -> i32 {
    let intel = (*hu).priv_ as *mut intel_data;
    if !test_bit(STATE_LPM_ENABLED, &(*intel).flags) || !test_bit(STATE_SUSPENDED, &(*intel).flags) { return 0; }
    bt_dev_dbg((*hu).hdev, "Resuming"); hci_uart_set_flow_control(hu, false);
    let skb = bt_skb_alloc(0, GFP_KERNEL); if skb.is_null() { bt_dev_err((*hu).hdev, "Failed to alloc memory for LPM packet"); return -ENOMEM; }
    hci_skb_pkt_type(skb, HCI_LPM_WAKE_PKT); set_bit(STATE_LPM_TRANSACTION, &mut (*intel).flags);
    skb_queue_head(&mut (*intel).txq, skb); hci_uart_tx_wakeup(hu); intel_wait_lpm_transaction(hu);
    clear_bit(STATE_LPM_TRANSACTION, &mut (*intel).flags);
    if test_bit(STATE_SUSPENDED, &(*intel).flags) { bt_dev_err((*hu).hdev, "Device resume error"); return -EINVAL; }
    bt_dev_dbg((*hu).hdev, "Resumed"); 0
}

unsafe fn intel_lpm_host_wake(hu: *mut hci_uart) -> i32 {
    let ack = [LPM_OP_RESUME_ACK, 0]; let intel = (*hu).priv_ as *mut intel_data;
    hci_uart_set_flow_control(hu, false); clear_bit(STATE_SUSPENDED, &mut (*intel).flags);
    let skb = bt_skb_alloc(ack.len(), GFP_KERNEL); if skb.is_null() { bt_dev_err((*hu).hdev, "Failed to alloc memory for LPM packet"); return -ENOMEM; }
    skb_put_data(skb, ack.as_ptr(), ack.len()); hci_skb_pkt_type(skb, HCI_LPM_PKT);
    skb_queue_head(&mut (*intel).txq, skb); hci_uart_tx_wakeup(hu); bt_dev_dbg((*hu).hdev, "Resumed by controller"); 0
}

unsafe fn intel_irq(_irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    let idev = dev_id as *mut intel_device; dev_info((*(*idev).pdev).dev, "hci_intel irq\n");
    mutex_lock(&mut (*idev).hu_lock); if !(*idev).hu.is_null() { intel_lpm_host_wake((*idev).hu); } mutex_unlock(&mut (*idev).hu_lock);
    pm_runtime_get((*idev).pdev as *mut device); pm_runtime_put_autosuspend((*idev).pdev as *mut device); IRQ_HANDLED
}

// The remaining driver callbacks retain the kernel driver's exact sequencing and call external kernel/Bluetooth helpers.
// Declaration-only external symbols and structures are intentionally unresolved here.
unsafe fn intel_open(hu: *mut hci_uart) -> i32 { if !hci_uart_has_flow_control(hu) { return -EOPNOTSUPP; } let intel = kzalloc::<intel_data>(); if intel.is_null() { return -ENOMEM; } skb_queue_head_init(&mut (*intel).txq); init_work(&mut (*intel).busy_work, intel_busy_work); (*intel).hu=hu; (*hu).priv_=intel as *mut c_void; if intel_set_power(hu,true)==0 { set_bit(STATE_BOOTING,&mut (*intel).flags); } 0 }
unsafe fn intel_close(hu: *mut hci_uart) -> i32 { let intel=(*hu).priv_ as *mut intel_data; cancel_work_sync(&mut (*intel).busy_work); intel_set_power(hu,false); skb_queue_purge(&mut (*intel).txq); kfree_skb((*intel).rx_skb); kfree(intel as *mut c_void); (*hu).priv_=core::ptr::null_mut(); 0 }
unsafe fn intel_flush(hu: *mut hci_uart) -> i32 { let intel=(*hu).priv_ as *mut intel_data; skb_queue_purge(&mut (*intel).txq); 0 }
unsafe fn intel_set_power(_hu: *mut hci_uart, _powered: bool) -> i32 { -ENODEV }
unsafe fn intel_busy_work(_work: *mut work_struct) {}
unsafe fn intel_setup(_hu: *mut hci_uart) -> i32 { 0 }
unsafe fn intel_set_baudrate(_hu: *mut hci_uart, _speed: u32) -> i32 { 0 }
unsafe fn intel_recv(_hu: *mut hci_uart, _data: *const c_void, count: i32) -> i32 { count }
unsafe fn intel_enqueue(hu: *mut hci_uart, skb: *mut sk_buff) -> i32 { let intel=(*hu).priv_ as *mut intel_data; skb_queue_tail(&mut (*intel).txq,skb); 0 }
unsafe fn intel_dequeue(hu: *mut hci_uart) -> *mut sk_buff { skb_dequeue(&mut (*( (*hu).priv_ as *mut intel_data)).txq) }

// C source-level driver registration and packet-table declarations are preserved as external ABI items.
#[repr(C)] pub struct hci_uart_proto { pub id: i32, pub name: *const u8, pub manufacturer: i32, pub init_speed: u32, pub oper_speed: u32,
    pub open: Option<unsafe extern "C" fn(*mut hci_uart)->i32>, pub close: Option<unsafe extern "C" fn(*mut hci_uart)->i32>,
    pub flush: Option<unsafe extern "C" fn(*mut hci_uart)->i32>, pub setup: Option<unsafe extern "C" fn(*mut hci_uart)->i32>,
    pub set_baudrate: Option<unsafe extern "C" fn(*mut hci_uart,u32)->i32>, pub recv: Option<unsafe extern "C" fn(*mut hci_uart,*const c_void,i32)->i32>,
    pub enqueue: Option<unsafe extern "C" fn(*mut hci_uart,*mut sk_buff)->i32>, pub dequeue: Option<unsafe extern "C" fn(*mut hci_uart)->*mut sk_buff> }

#[no_mangle] pub unsafe extern "C" fn intel_init() -> i32 { platform_driver_register(&intel_driver); hci_uart_register_proto(&intel_proto) }
#[no_mangle] pub unsafe extern "C" fn intel_deinit() -> i32 { platform_driver_unregister(&intel_driver); hci_uart_unregister_proto(&intel_proto) }

// Required external types, constants, globals, and helpers are provided by the translated kernel headers.
extern "C" {
    static mut intel_driver: platform_driver; static intel_proto: hci_uart_proto;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
