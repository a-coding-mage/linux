/*
 * atari_nfeth.c - ARAnyM ethernet card driver for GNU/Linux
 *
 * Copyright (c) 2005 Milan Jurik, Petr Stehlik of ARAnyM dev team
 *
 * Based on ARAnyM driver for FreeMiNT written by Standa Opichal
 *
 * This software may be used and distributed according to the terms of
 * the GNU General Public License (GPL), incorporated herein by reference.
 */

/* Linux kernel and architecture dependencies are supplied by the surrounding crate. */

pub const DRV_VERSION: &str = "0.3";
pub const DRV_RELDATE: &str = "10/12/2005";

pub const GET_VERSION: i32 = 0;
pub const XIF_INTLEVEL: i32 = 1;
pub const XIF_IRQ: i32 = 2;
pub const XIF_START: i32 = 3;
pub const XIF_STOP: i32 = 4;
pub const XIF_READLENGTH: i32 = 5;
pub const XIF_READBLOCK: i32 = 6;
pub const XIF_WRITEBLOCK: i32 = 7;
pub const XIF_GET_MAC: i32 = 8;
pub const XIF_GET_IPHOST: i32 = 9;
pub const XIF_GET_IPATARI: i32 = 10;
pub const XIF_GET_NETMASK: i32 = 11;

pub const MAX_UNIT: usize = 8;

#[allow(dead_code)]
pub static VERSION: &[u8] = b"nfeth.c:v0.3 10/12/2005 S.Opichal, M.Jurik, P.Stehlik\n http://aranym.org/\n\0";

static mut NF_ETHER_ID: i64 = 0;
static mut NF_ETHER_IRQ: i32 = 0;

#[repr(C)]
pub struct NfethPrivate {
    pub eth_x: i32,
}

extern "C" {
    static mut nfeth_dev: [*mut NetDevice; MAX_UNIT];
}

/* External kernel types and functions correspond to the included Linux headers. */
#[repr(C)]
pub struct NetDevice;
#[repr(C)]
pub struct SkBuff;
pub type Irqreturn = i32;

extern "C" {
    fn nf_call(id: i64, ...) -> i64;
    fn netdev_priv(dev: *mut NetDevice) -> *mut NfethPrivate;
    fn netif_start_queue(dev: *mut NetDevice);
    fn netif_stop_queue(dev: *mut NetDevice);
    fn netif_wake_queue(dev: *mut NetDevice);
    fn dev_alloc_skb(len: u32) -> *mut SkBuff;
    fn skb_reserve(skb: *mut SkBuff, len: u32);
    fn skb_put(skb: *mut SkBuff, len: u32) -> *mut u8;
    fn virt_to_phys(ptr: *const core::ffi::c_void) -> u64;
    fn eth_type_trans(skb: *mut SkBuff, dev: *mut NetDevice) -> u16;
    fn netif_rx(skb: *mut SkBuff);
    fn dev_kfree_skb(skb: *mut SkBuff);
    fn memset(dst: *mut core::ffi::c_void, value: i32, len: usize) -> *mut core::ffi::c_void;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void;
    fn alloc_etherdev(size: usize) -> *mut NetDevice;
    fn eth_hw_addr_set(dev: *mut NetDevice, addr: *const u8);
    fn register_netdev(dev: *mut NetDevice) -> i32;
    fn free_netdev(dev: *mut NetDevice);
    fn unregister_netdev(dev: *mut NetDevice);
    fn nf_get_id(name: *const u8) -> i64;
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> Irqreturn, flags: u32, name: *const u8, dev: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> Irqreturn) -> i32;
    fn free_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> Irqreturn);
}

#[repr(C)]
pub struct NetDeviceStats {
    pub rx_errors: u64, pub rx_dropped: u64, pub rx_packets: u64, pub rx_bytes: u64,
    pub tx_packets: u64, pub tx_bytes: u64, pub tx_errors: u64,
}

const ETH_ZLEN: usize = 60;

unsafe fn nfeth_open(dev: *mut NetDevice) -> i32 {
    let priv_ = netdev_priv(dev);
    let res = nf_call(NF_ETHER_ID + XIF_START as i64, (*priv_).eth_x);
    let _ = res;
    netif_start_queue(dev);
    0
}

unsafe fn nfeth_stop(dev: *mut NetDevice) -> i32 {
    let priv_ = netdev_priv(dev);
    netif_stop_queue(dev);
    nf_call(NF_ETHER_ID + XIF_STOP as i64, (*priv_).eth_x);
    0
}

unsafe fn recv_packet(dev: *mut NetDevice) {
    let priv_ = netdev_priv(dev);
    let pktlen = nf_call(NF_ETHER_ID + XIF_READLENGTH as i64, (*priv_).eth_x) as u16;
    if pktlen == 0 { return; }
    let skb = dev_alloc_skb(pktlen as u32 + 2);
    if skb.is_null() { return; }
    skb_reserve(skb, 2);
    skb_put(skb, pktlen as u32);
    nf_call(NF_ETHER_ID + XIF_READBLOCK as i64, (*priv_).eth_x, skb as i64, pktlen as i64);
    let _ = eth_type_trans(skb, dev);
    netif_rx(skb);
}

unsafe extern "C" fn nfeth_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> Irqreturn {
    let mask = nf_call(NF_ETHER_ID + XIF_IRQ as i64, 0);
    let mut i = 0usize;
    let mut m = 1i64;
    while i < MAX_UNIT {
        if (mask & m) != 0 && !nfeth_dev[i].is_null() {
            recv_packet(nfeth_dev[i]);
            nf_call(NF_ETHER_ID + XIF_IRQ as i64, m);
        }
        m <<= 1; i += 1;
    }
    1
}

unsafe fn nfeth_xmit(skb: *mut SkBuff, dev: *mut NetDevice) -> i32 {
    let priv_ = netdev_priv(dev);
    let _ = priv_;
    let mut shortpkt = [0i8; ETH_ZLEN];
    let data = shortpkt.as_mut_ptr();
    nf_call(NF_ETHER_ID + XIF_WRITEBLOCK as i64, (*priv_).eth_x, data as i64, ETH_ZLEN as i64);
    dev_kfree_skb(skb);
    0
}

unsafe fn nfeth_tx_timeout(dev: *mut NetDevice, _txqueue: u32) {
    netif_wake_queue(dev);
}

unsafe fn nfeth_probe(unit: i32) -> *mut NetDevice {
    let mut mac = [0i8; 6];
    let mut host_ip = [0i8; 32];
    let mut local_ip = [0i8; 32];
    if nf_call(NF_ETHER_ID + XIF_GET_MAC as i64, unit, mac.as_mut_ptr() as i64, 6) == 0 { return core::ptr::null_mut(); }
    let dev = alloc_etherdev(core::mem::size_of::<NfethPrivate>());
    if dev.is_null() { return core::ptr::null_mut(); }
    eth_hw_addr_set(dev, mac.as_ptr() as *const u8);
    (*netdev_priv(dev)).eth_x = unit;
    if register_netdev(dev) != 0 { free_netdev(dev); return core::ptr::null_mut(); }
    nf_call(NF_ETHER_ID + XIF_GET_IPHOST as i64, unit, host_ip.as_mut_ptr() as i64, 32);
    nf_call(NF_ETHER_ID + XIF_GET_IPATARI as i64, unit, local_ip.as_mut_ptr() as i64, 32);
    dev
}

pub unsafe fn nfeth_init() -> i32 {
    NF_ETHER_ID = nf_get_id(b"ETHERNET\0".as_ptr());
    if NF_ETHER_ID == 0 { return -19; }
    NF_ETHER_IRQ = nf_call(NF_ETHER_ID + XIF_INTLEVEL as i64) as i32;
    let error = request_irq(NF_ETHER_IRQ, nfeth_interrupt, 0x80, b"eth emu\0".as_ptr(), nfeth_interrupt);
    if error != 0 { return error; }
    for i in 0..MAX_UNIT { nfeth_dev[i] = nfeth_probe(i as i32); }
    0
}

pub unsafe fn nfeth_cleanup() {
    for i in 0..MAX_UNIT {
        if !nfeth_dev[i].is_null() { unregister_netdev(nfeth_dev[i]); free_netdev(nfeth_dev[i]); }
    }
    free_irq(NF_ETHER_IRQ, nfeth_interrupt);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
