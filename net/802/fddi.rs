// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * INET An implementation of the TCP/IP protocol suite for Linux.
 * FDDI-type device handling.
 *
 * This is a direct Rust translation of fddi.c.  Linux kernel types,
 * constants, and functions referenced below are supplied by other files.
 */

use core::ffi::c_void;

extern "C" {
    fn skb_push(skb: *mut sk_buff, len: usize) -> *mut c_void;
    fn skb_pull(skb: *mut sk_buff, len: usize) -> *mut c_void;
    fn skb_reset_mac_header(skb: *mut sk_buff);
    fn htons(value: u16) -> u16;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, value: i32, n: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32;
    fn alloc_netdev(
        sizeof_priv: i32,
        name: *const u8,
        name_type: u32,
        setup: unsafe extern "C" fn(*mut net_device),
    ) -> *mut net_device;
}

#[repr(C)]
pub struct sk_buff {
    pub data: *mut u8,
    pub len: usize,
    pub dev: *mut net_device,
    pub pkt_type: i32,
}

#[repr(C)]
pub struct net_device {
    pub header_ops: *const header_ops,
    pub type_: u16,
    pub hard_header_len: u16,
    pub mtu: u32,
    pub min_mtu: u32,
    pub max_mtu: u32,
    pub addr_len: u8,
    pub tx_queue_len: u32,
    pub flags: u32,
    pub broadcast: [u8; 8],
    pub dev_addr: [u8; 8],
}

#[repr(C)]
pub struct header_ops {
    pub create: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device, u16, *const c_void, *const c_void, u32) -> i32>,
}

#[repr(C)]
pub struct fddi_llc_snap {
    pub dsap: u8,
    pub ssap: u8,
    pub ctrl: u8,
    pub oui: [u8; 3],
    pub ethertype: u16,
}

#[repr(C)]
pub union fddi_hdr_union {
    pub llc_snap: fddi_llc_snap,
    pub llc_8022_1: fddi_llc_8022_1,
}

#[repr(C)]
pub struct fddi_llc_8022_1 {
    pub dsap: u8,
}

#[repr(C)]
pub struct fddihdr {
    pub fc: u8,
    pub hdr: fddi_hdr_union,
    pub daddr: [u8; 6],
    pub saddr: [u8; 6],
}

unsafe extern "C" fn fddi_header(
    skb: *mut sk_buff,
    dev: *mut net_device,
    type_: u16,
    daddr: *const c_void,
    saddr: *const c_void,
    _len: u32,
) -> i32 {
    let mut hl: i32 = FDDI_K_SNAP_HLEN as i32;
    if type_ != ETH_P_IP && type_ != ETH_P_IPV6 && type_ != ETH_P_ARP {
        hl = FDDI_K_8022_HLEN as i32 - 3;
    }
    let fddi = skb_push(skb, hl as usize) as *mut fddihdr;
    (*fddi).fc = FDDI_FC_K_ASYNC_LLC_DEF;
    if type_ == ETH_P_IP || type_ == ETH_P_IPV6 || type_ == ETH_P_ARP {
        let snap = &mut (*fddi).hdr.llc_snap;
        snap.dsap = FDDI_EXTENDED_SAP;
        snap.ssap = FDDI_EXTENDED_SAP;
        snap.ctrl = FDDI_UI_CMD;
        snap.oui = [0, 0, 0];
        snap.ethertype = htons(type_);
    }
    let addr_len = (*dev).addr_len as usize;
    if !saddr.is_null() {
        memcpy((*fddi).saddr.as_mut_ptr() as *mut c_void, saddr, addr_len);
    } else {
        memcpy((*fddi).saddr.as_mut_ptr() as *mut c_void, (*dev).dev_addr.as_ptr() as *const c_void, addr_len);
    }
    if !daddr.is_null() {
        memcpy((*fddi).daddr.as_mut_ptr() as *mut c_void, daddr, addr_len);
        return hl;
    }
    -hl
}

#[no_mangle]
pub unsafe extern "C" fn fddi_type_trans(skb: *mut sk_buff, dev: *mut net_device) -> u16 {
    let fddi = (*skb).data as *mut fddihdr;
    skb_reset_mac_header(skb);
    (*skb).dev = dev;
    if (*skb).len < FDDI_K_8022_HLEN as usize { return htons(0); }
    let dsap = (*fddi).hdr.llc_8022_1.dsap;
    let type_;
    if dsap == 0xe0 {
        skb_pull(skb, (FDDI_K_8022_HLEN - 3) as usize);
        type_ = htons(ETH_P_802_2);
    } else {
        if (*skb).len < FDDI_K_SNAP_HLEN as usize { return htons(0); }
        skb_pull(skb, FDDI_K_SNAP_HLEN as usize);
        type_ = (*fddi).hdr.llc_snap.ethertype;
    }
    if (*fddi).daddr[0] & 1 != 0 {
        if memcmp((*fddi).daddr.as_ptr() as *const c_void, (*dev).broadcast.as_ptr() as *const c_void, FDDI_K_ALEN as usize) == 0 {
            (*skb).pkt_type = PACKET_BROADCAST;
        } else { (*skb).pkt_type = PACKET_MULTICAST; }
    } else if (*dev).flags & IFF_PROMISC != 0 && memcmp((*fddi).daddr.as_ptr() as *const c_void, (*dev).dev_addr.as_ptr() as *const c_void, FDDI_K_ALEN as usize) != 0 {
        (*skb).pkt_type = PACKET_OTHERHOST;
    }
    type_
}

static FDDI_HEADER_OPS: header_ops = header_ops { create: Some(fddi_header) };

unsafe extern "C" fn fddi_setup(dev: *mut net_device) {
    (*dev).header_ops = &FDDI_HEADER_OPS;
    (*dev).type_ = ARPHRD_FDDI;
    (*dev).hard_header_len = (FDDI_K_SNAP_HLEN + 3) as u16;
    (*dev).mtu = FDDI_K_SNAP_DLEN;
    (*dev).min_mtu = FDDI_K_SNAP_HLEN;
    (*dev).max_mtu = FDDI_K_SNAP_DLEN;
    (*dev).addr_len = FDDI_K_ALEN as u8;
    (*dev).tx_queue_len = 100;
    (*dev).flags = IFF_BROADCAST | IFF_MULTICAST;
    memset((*dev).broadcast.as_mut_ptr() as *mut c_void, 0xff, FDDI_K_ALEN as usize);
}

#[no_mangle]
pub unsafe extern "C" fn alloc_fddidev(sizeof_priv: i32) -> *mut net_device {
    alloc_netdev(sizeof_priv, b"fddi%d\0".as_ptr(), NET_NAME_UNKNOWN, fddi_setup)
}

// MODULE_DESCRIPTION("Core routines for FDDI network devices");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
