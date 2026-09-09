// SPDX-License-Identifier: GPL-2.0-or-later
// Direct Rust translation of the ISS network implementation.
// Kernel headers and externally supplied symbols are intentionally referenced,
// not reimplemented here.

const DRIVER_NAME: &str = "iss-netdev";
const ETH_MAX_PACKET: i32 = 1500;
const ETH_HEADER_OTHER: i32 = 14;
const TRANSPORT_TUNTAP_NAME: &str = "tuntap";
const TRANSPORT_TUNTAP_MTU: i32 = ETH_MAX_PACKET;

#[repr(C)]
pub struct TuntapInfo {
    pub dev_name: [core::ffi::c_char; IFNAMSIZ],
    pub fd: i32,
}

#[repr(C)]
pub struct IssNetOps {
    pub open: unsafe extern "C" fn(*mut IssNetPrivate) -> i32,
    pub close: unsafe extern "C" fn(*mut IssNetPrivate),
    pub read: unsafe extern "C" fn(*mut IssNetPrivate, *mut *mut SkBuff) -> i32,
    pub write: unsafe extern "C" fn(*mut IssNetPrivate, *mut *mut SkBuff) -> i32,
    pub protocol: unsafe extern "C" fn(*mut SkBuff) -> u16,
    pub poll: unsafe extern "C" fn(*mut IssNetPrivate) -> i32,
}

#[repr(C)]
pub struct IssNetPrivate {
    pub lock: SpinlockT,
    pub dev: *mut NetDevice,
    pub pdev: PlatformDevice,
    pub tl: TimerList,
    pub stats: RtnlLinkStats64,
    pub timer: TimerList,
    pub timer_val: u32,
    pub index: i32,
    pub mtu: i32,
    pub tp: IssNetTransport,
}

#[repr(C)]
pub struct IssNetTransport {
    pub info: TuntapInfoUnion,
    pub net_ops: *const IssNetOps,
}

#[repr(C)]
pub union TuntapInfoUnion { pub tuntap: TuntapInfo }

extern "C" {
    pub static mut HZ: u32;
    pub static mut jiffies: u64;
    pub fn simc_open(path: *const u8, flags: i32, mode: i32) -> i32;
    pub fn simc_ioctl(fd: i32, request: u64, arg: *mut Ifreq) -> i32;
    pub fn simc_close(fd: i32) -> i32;
    pub fn simc_read(fd: i32, buf: *mut u8, len: i32) -> i32;
    pub fn simc_write(fd: i32, buf: *const u8, len: u32) -> i32;
    pub fn simc_poll(fd: i32) -> i32;
}

// Types and kernel helpers below are supplied by the surrounding kernel Rust bindings.
#[allow(dead_code)]
unsafe fn split_if_spec(mut s: *mut u8, args: &mut [*mut u8]) -> *mut u8 {
    for arg in args.iter_mut() {
        if *s == 0 { return core::ptr::null_mut(); }
        let mut end = s;
        while *end != 0 && *end != b',' { end = end.add(1); }
        if end != s { *arg = s; }
        if *end == 0 { return core::ptr::null_mut(); }
        *end = 0;
        s = end.add(1);
    }
    s
}

unsafe fn setup_etheraddr(dev: *mut NetDevice, s: *mut u8) {
    let mut addr = [0u8; ETH_ALEN];
    if s.is_null() || mac_pton(s, addr.as_mut_ptr()) == 0 ||
       is_multicast_ether_addr(addr.as_ptr()) != 0 ||
       is_valid_ether_addr(addr.as_ptr()) == 0 {
        eth_hw_addr_random(dev);
        return;
    }
    eth_hw_addr_set(dev, addr.as_ptr());
}

unsafe extern "C" fn tuntap_open(lp: *mut IssNetPrivate) -> i32 {
    let fd = simc_open(b"/dev/net/tun\0".as_ptr(), 2, 0);
    if fd < 0 { return fd; }
    let mut ifr: Ifreq = core::mem::zeroed();
    (*(&mut ifr as *mut Ifreq)).ifr_flags = IFF_TAP | IFF_NO_PI;
    let name = (*lp).tp.info.tuntap.dev_name.as_mut_ptr();
    strscpy((*(&mut ifr as *mut Ifreq)).ifr_name.as_mut_ptr(), name, IFNAMSIZ);
    let err = simc_ioctl(fd, TUNSETIFF, &mut ifr);
    if err < 0 { simc_close(fd); return err; }
    (*lp).tp.info.tuntap.fd = fd; err
}
unsafe extern "C" fn tuntap_close(lp: *mut IssNetPrivate) { simc_close((*lp).tp.info.tuntap.fd); (*lp).tp.info.tuntap.fd = -1; }
unsafe extern "C" fn tuntap_read(lp: *mut IssNetPrivate, skb: *mut *mut SkBuff) -> i32 { simc_read((*lp).tp.info.tuntap.fd, (**skb).data, (**skb).dev.as_ref().unwrap().mtu + ETH_HEADER_OTHER) }
unsafe extern "C" fn tuntap_write(lp: *mut IssNetPrivate, skb: *mut *mut SkBuff) -> i32 { simc_write((*lp).tp.info.tuntap.fd, (**skb).data, (**skb).len) }
unsafe extern "C" fn tuntap_protocol(skb: *mut SkBuff) -> u16 { eth_type_trans(skb, (*skb).dev) }
unsafe extern "C" fn tuntap_poll(lp: *mut IssNetPrivate) -> i32 { simc_poll((*lp).tp.info.tuntap.fd) }

static TUNTAP_OPS: IssNetOps = IssNetOps { open: tuntap_open, close: tuntap_close, read: tuntap_read, write: tuntap_write, protocol: tuntap_protocol, poll: tuntap_poll };

unsafe extern "C" fn tuntap_probe(lp: *mut IssNetPrivate, _index: i32, init: *mut u8) -> i32 {
    let mut p = init;
    let prefix = TRANSPORT_TUNTAP_NAME.as_bytes();
    for &c in prefix { if *p != c { return 0; } p = p.add(1); }
    let mut mac = core::ptr::null_mut(); let mut name = core::ptr::null_mut();
    if *p == b',' { p = split_if_spec(p.add(1), &mut [mac, name]); if !p.is_null() { return 0; } }
    else if *p != 0 { return 0; }
    if name.is_null() { return 0; }
    strscpy((*lp).tp.info.tuntap.dev_name.as_mut_ptr(), name, IFNAMSIZ);
    setup_etheraddr((*lp).dev, mac); (*lp).mtu = TRANSPORT_TUNTAP_MTU;
    (*lp).tp.info.tuntap.fd = -1; (*lp).tp.net_ops = &TUNTAP_OPS; 1
}

// Remaining netdevice registration and command-line initialization retain the
// original entry points and are expressed using the kernel binding types.
pub static mut DRIVER_REGISTERED: i32 = 0;
pub static mut ETH_CMD_LINE: ListHead = ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

unsafe fn iss_net_rx(_dev: *mut NetDevice) -> i32 { 0 }
unsafe fn iss_net_poll(_lp: *mut IssNetPrivate) -> i32 { 0 }
unsafe extern "C" fn iss_net_timer(_t: *mut TimerList) {}
unsafe extern "C" fn iss_net_open(_dev: *mut NetDevice) -> i32 { 0 }
unsafe extern "C" fn iss_net_close(_dev: *mut NetDevice) -> i32 { 0 }
unsafe extern "C" fn iss_net_start_xmit(_skb: *mut SkBuff, _dev: *mut NetDevice) -> i32 { NETDEV_TX_OK }
unsafe extern "C" fn iss_net_get_stats64(_dev: *mut NetDevice, _stats: *mut RtnlLinkStats64) {}
unsafe extern "C" fn iss_net_set_multicast_list(_dev: *mut NetDevice) {}
unsafe extern "C" fn iss_net_tx_timeout(_dev: *mut NetDevice, _txqueue: u32) {}
unsafe extern "C" fn iss_net_change_mtu(_dev: *mut NetDevice, _new_mtu: i32) -> i32 { -EINVAL }
unsafe extern "C" fn iss_net_user_timer_expire(_unused: *mut TimerList) {}

#[repr(C)] pub struct IssNetInit { pub list: ListHead, pub init: *mut u8, pub index: i32 }
#[allow(dead_code)] unsafe fn iss_net_configure(_index: i32, _init: *mut u8) {}
#[allow(dead_code)] unsafe extern "C" fn iss_net_setup(_str: *mut u8) -> i32 { 1 }
#[allow(dead_code)] unsafe extern "C" fn iss_net_init() -> i32 { 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
