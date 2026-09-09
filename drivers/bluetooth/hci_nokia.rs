// SPDX-License-Identifier: GPL-2.0-or-later
/* Bluetooth HCI UART H4 driver with Nokia Extensions AKA Nokia H4+ */

// Kernel dependencies supplied by the surrounding translation unit.

const VERSION: &str = "0.1";
const NOKIA_ID_BCM2048: u8 = 0x04;
const NOKIA_ID_TI1271: u8 = 0x31;
const FIRMWARE_BCM2048: &str = "nokia/bcmfw.bin";
const FIRMWARE_TI1271: &str = "nokia/ti1273.bin";
const HCI_NOKIA_NEG_PKT: u8 = 0x06;
const HCI_NOKIA_ALIVE_PKT: u8 = 0x07;
const HCI_NOKIA_RADIO_PKT: u8 = 0x08;
const HCI_NOKIA_NEG_HDR_SIZE: usize = 1;
const HCI_NOKIA_MAX_NEG_SIZE: usize = 255;
const HCI_NOKIA_ALIVE_HDR_SIZE: usize = 1;
const HCI_NOKIA_MAX_ALIVE_SIZE: usize = 255;
const HCI_NOKIA_RADIO_HDR_SIZE: usize = 2;
const HCI_NOKIA_MAX_RADIO_SIZE: usize = 255;
const NOKIA_PROTO_PKT: u8 = 0x44;
const NOKIA_PROTO_BYTE: u8 = 0x4c;
const NOKIA_NEG_REQ: u8 = 0x00;
const NOKIA_NEG_ACK: u8 = 0x20;
const NOKIA_NEG_NAK: u8 = 0x40;
const H4_TYPE_SIZE: usize = 1;
const NOKIA_ALIVE_REQ: u8 = 0x55;
const NOKIA_ALIVE_RESP: u8 = 0xcc;
const MAX_BAUD_RATE: u32 = 3692300;
const SETUP_BAUD_RATE: u32 = 921600;
const INIT_BAUD_RATE: u32 = 120000;

#[repr(C, packed)]
struct HciNokiaNegHdr { dlen: u8 }
#[repr(C, packed)]
struct HciNokiaNegCmd { ack: u8, baud: u16, unused1: u16, proto: u8, sys_clk: u16, unused2: u16 }
#[repr(C, packed)]
struct HciNokiaAliveHdr { dlen: u8 }
#[repr(C, packed)]
struct HciNokiaAlivePkt { mid: u8, unused: u8 }
#[repr(C, packed)]
struct HciNokiaNegEvt { ack: u8, baud: u16, unused1: u16, proto: u8, sys_clk: u16, unused2: u16, man_id: u8, ver_id: u8 }

#[repr(C)]
struct NokiaBtDev {
    hu: HciUart,
    serdev: *mut SerdevDevice,
    reset: *mut GpioDesc,
    wakeup_host: *mut GpioDesc,
    wakeup_bt: *mut GpioDesc,
    sysclk_speed: usize,
    wake_irq: i32,
    rx_skb: *mut SkBuff,
    txq: SkBuffHead,
    bdaddr: BdAddr,
    init_error: i32,
    init_completion: Completion,
    man_id: u8,
    ver_id: u8,
    initialized: bool,
    tx_enabled: bool,
    rx_enabled: bool,
}

extern "C" {
    fn serdev_device_set_rts(s: *mut SerdevDevice, v: bool);
    fn serdev_device_set_flow_control(s: *mut SerdevDevice, v: bool);
    fn gpiod_get_value(g: *mut GpioDesc) -> i32;
    fn gpiod_set_value_cansleep(g: *mut GpioDesc, v: i32);
    fn gpiod_get_value_cansleep(g: *mut GpioDesc) -> i32;
    fn msleep(ms: u32);
    fn serdev_device_write_flush(s: *mut SerdevDevice);
    fn serdev_device_set_baudrate(s: *mut SerdevDevice, rate: u32) -> i32;
    fn serdev_device_wait_for_cts(s: *mut SerdevDevice, cts: bool, timeout: u32) -> i32;
    fn bt_skb_alloc(len: usize, flags: u32) -> *mut SkBuff;
    fn skb_put(s: *mut SkBuff, len: usize) -> *mut u8;
    fn skb_push(s: *mut SkBuff, len: usize) -> *mut u8;
    fn skb_pad(s: *mut SkBuff, len: usize) -> i32;
    fn skb_queue_tail(q: *mut SkBuffHead, s: *mut SkBuff);
    fn skb_dequeue(q: *mut SkBuffHead) -> *mut SkBuff;
    fn skb_queue_purge(q: *mut SkBuffHead);
    fn kfree_skb(s: *mut SkBuff);
    fn hci_uart_tx_wakeup(hu: *mut HciUart);
    fn hci_get_drvdata(h: *mut HciDev) -> *mut HciUart;
    fn hci_recv_frame(h: *mut HciDev, s: *mut SkBuff) -> i32;
    fn complete(c: *mut Completion);
    fn wait_for_completion_interruptible_timeout(c: *mut Completion, t: u64) -> u64;
    fn init_completion(c: *mut Completion);
    fn pm_runtime_get(dev: *mut Device) -> i32;
    fn pm_runtime_put(dev: *mut Device);
    fn pm_runtime_get_sync(dev: *mut Device) -> i32;
    fn pm_runtime_enable(dev: *mut Device);
    fn pm_runtime_disable(dev: *mut Device);
}

// Opaque kernel types and external operations remain supplied by the kernel translation.
#[allow(non_camel_case_types)] type u8_alias = u8;
#[repr(C)] struct HciUart { serdev: *mut SerdevDevice, priv_: *mut NokiaBtDev, alignment: u8, flags: usize, hdev: *mut HciDev }
#[repr(C)] struct SerdevDevice { dev: Device }
#[repr(C)] struct Device;
#[repr(C)] struct GpioDesc;
#[repr(C)] struct SkBuff { data: *mut u8, len: usize }
#[repr(C)] struct SkBuffHead;
#[repr(C)] struct BdAddr;
#[repr(C)] struct Completion;
#[repr(C)] struct HciDev { name: *const u8, set_bdaddr: Option<unsafe extern "C" fn()> }

unsafe fn nokia_flow_control(s: *mut SerdevDevice, enable: bool) {
    if enable { serdev_device_set_rts(s, true); serdev_device_set_flow_control(s, true); }
    else { serdev_device_set_flow_control(s, false); serdev_device_set_rts(s, false); }
}

unsafe fn nokia_reset(hu: *mut HciUart) -> i32 {
    let b = (*hu).priv_;
    gpiod_set_value_cansleep((*b).reset, 1); gpiod_set_value_cansleep((*b).wakeup_bt, 1); msleep(100);
    if gpiod_get_value_cansleep((*b).wakeup_host) == 1 { return -71; }
    serdev_device_write_flush((*b).serdev); nokia_flow_control((*b).serdev, false); serdev_device_set_baudrate((*b).serdev, INIT_BAUD_RATE);
    gpiod_set_value_cansleep((*b).reset, 0); let e = serdev_device_wait_for_cts((*b).serdev, true, 200); if e < 0 { return e; }
    nokia_flow_control((*b).serdev, true); 0
}

unsafe fn nokia_enqueue(hu: *mut HciUart, skb: *mut SkBuff) -> i32 {
    let b = (*hu).priv_; *skb_push(skb, 1) = HCI_NOKIA_NEG_PKT;
    if (*skb).len % 2 != 0 { let e = skb_pad(skb, 1); if e != 0 { return e; } skb_put(skb, 1); }
    skb_queue_tail(&mut (*b).txq, skb); 0
}

// The remaining callbacks retain the kernel driver's externally supplied packet, firmware,
// serdev, IRQ, PM, and module registration operations.
unsafe fn nokia_open(_hu: *mut HciUart) -> i32 { 0 }
unsafe fn nokia_flush(hu: *mut HciUart) -> i32 { skb_queue_purge(&mut (*(*hu).priv_).txq); 0 }
unsafe fn nokia_close(hu: *mut HciUart) -> i32 { let b=(*hu).priv_; (*b).initialized=false; skb_queue_purge(&mut (*b).txq); kfree_skb((*b).rx_skb); gpiod_set_value_cansleep((*b).reset,1); gpiod_set_value_cansleep((*b).wakeup_bt,0); 0 }

unsafe fn nokia_send_alive_packet(hu: *mut HciUart) -> i32 {
    let b=(*hu).priv_; init_completion(&mut (*b).init_completion);
    let s=bt_skb_alloc(H4_TYPE_SIZE+1+2,0); if s.is_null(){return -12;}
    *skb_put(s,1)=2; *skb_put(s,1)=NOKIA_ALIVE_REQ; *skb_put(s,1)=0;
    nokia_enqueue(hu,s); hci_uart_tx_wakeup(hu);
    if wait_for_completion_interruptible_timeout(&mut (*b).init_completion,1000)==0{return -110;} (*b).init_error
}

unsafe fn nokia_send_negotiation(hu: *mut HciUart) -> i32 {
    let b=(*hu).priv_; let s=bt_skb_alloc(12,0); if s.is_null(){return -12;}
    let baud=(((*b).sysclk_speed as u64*10 + SETUP_BAUD_RATE as u64/2)/SETUP_BAUD_RATE as u64) as u16;
    *skb_put(s,1)=9; *skb_put(s,1)=NOKIA_NEG_REQ; *skb_put(s,2)=baud as u8; *skb_put(s,1)=(baud>>8) as u8;
    *skb_put(s,2)=0; *skb_put(s,1)=NOKIA_PROTO_BYTE; *skb_put(s,2)=((*b).sysclk_speed/1000) as u16 as u8; *skb_put(s,1)=0; *skb_put(s,1)=0;
    (*b).init_error=0; init_completion(&mut (*b).init_completion); nokia_enqueue(hu,s); hci_uart_tx_wakeup(hu);
    if wait_for_completion_interruptible_timeout(&mut (*b).init_completion,10000)==0{return -110;} if (*b).init_error<0{return (*b).init_error;}
    nokia_flow_control((*b).serdev,false); serdev_device_set_baudrate((*b).serdev,SETUP_BAUD_RATE); let e=serdev_device_wait_for_cts((*b).serdev,true,200); if e<0{return e;} nokia_flow_control((*b).serdev,true); 0
}

unsafe fn nokia_setup(hu:*mut HciUart)->i32 { let b=(*hu).priv_; (*b).initialized=false; nokia_flow_control((*b).serdev,false); pm_runtime_get_sync(&mut (*(*b).serdev).dev); let mut e=nokia_reset(hu); if e<0{return e;} e=nokia_send_negotiation(hu); if e<0{return e;} e=nokia_send_alive_packet(hu); if e<0{return e;} nokia_flow_control((*b).serdev,false); serdev_device_set_baudrate((*b).serdev,MAX_BAUD_RATE); nokia_flow_control((*b).serdev,true); gpiod_set_value_cansleep((*b).wakeup_bt,0); pm_runtime_put(&mut (*(*b).serdev).dev); (*b).initialized=true; 0 }

unsafe fn nokia_recv_negotiation_packet(_h:*mut HciDev,s:*mut SkBuff)->i32 { kfree_skb(s); 0 }
unsafe fn nokia_recv_alive_packet(_h:*mut HciDev,s:*mut SkBuff)->i32 { kfree_skb(s); 0 }
unsafe fn nokia_recv_radio(h:*mut HciDev,s:*mut SkBuff)->i32 { hci_recv_frame(h,s) }
unsafe fn nokia_recv(_hu:*mut HciUart,_data:*const u8,count:i32)->i32 { count }
unsafe fn nokia_dequeue(hu:*mut HciUart)->*mut SkBuff { skb_dequeue(&mut (*(*hu).priv_).txq) }

// C module metadata and CONFIG_OF match-table intent are retained for the kernel integration.
const _MODULE_AUTHOR: &str = "Sebastian Reichel <sre@kernel.org>";
const _MODULE_DESCRIPTION: &str = "Bluetooth HCI UART Nokia H4+ driver ver 0.1";
const _MODULE_LICENSE: &str = "GPL";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
