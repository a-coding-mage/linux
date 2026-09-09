// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SNAP data link layer. Derived from 802.2
 *
 *     Alan Cox <alan@lxorguk.ukuu.org.uk>,
 *     from the 802.2 layer by Greg Page.
 *     Merged in additions from Greg Page's psnap.c.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_int;

const ETH_P_SNAP: u16 = 0x0004; // supplied by linux/if_ether.h in the C source
const GFP_ATOMIC: c_int = 0; // supplied by linux/slab.h
const EBUSY: c_int = 16;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct net_device;
#[repr(C)]
pub struct sk_buff;
#[repr(C)]
pub struct packet_type {
    pub type_: u16,
}
#[repr(C)]
pub struct llc_sap {
    pub laddr: llc_addr,
}
#[repr(C)]
pub struct llc_addr {
    pub lsap: u8,
}

#[repr(C)]
pub struct datalink_proto {
    pub node: list_head,
    pub type_: [u8; 5],
    pub rcvfunc: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device, *mut packet_type, *mut net_device) -> c_int>,
    pub header_length: c_int,
    pub request: Option<unsafe extern "C" fn(*mut datalink_proto, *mut sk_buff, *const u8) -> c_int>,
}

extern "C" {
    fn memcmp(a: *const u8, b: *const u8, n: usize) -> c_int;
    fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn printk(msg: *const u8);
    fn llc_sap_open(lsap: u8, rcv: unsafe extern "C" fn(*mut sk_buff, *mut net_device, *mut packet_type, *mut net_device) -> c_int) -> *mut llc_sap;
    fn llc_sap_put(sap: *mut llc_sap);
    fn llc_build_and_send_ui_pkt(sap: *mut llc_sap, skb: *mut sk_buff, dest: *const u8, lsap: u8);
    fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> bool;
    fn skb_pull_rcsum(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn skb_reset_transport_header(skb: *mut sk_buff);
    fn skb_push(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn kfree_skb(skb: *mut sk_buff);
    fn kmalloc_datalink_proto(_: c_int) -> *mut datalink_proto;
    fn synchronize_net();
    fn kfree(ptr: *mut datalink_proto);
}

static mut snap_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut snap_lock: c_int = 0;
static mut snap_sap: *mut llc_sap = core::ptr::null_mut();

unsafe fn find_snap_client(desc: *const u8) -> *mut datalink_proto {
    let mut p: *mut datalink_proto = core::ptr::null_mut();
    let mut proto: *mut datalink_proto = core::ptr::null_mut();
    while !p.is_null() {
        if memcmp((*p).type_.as_ptr(), desc, 5) == 0 {
            proto = p;
            break;
        }
        p = (*p).node.next as *mut datalink_proto;
    }
    proto
}

unsafe extern "C" fn snap_rcv(skb: *mut sk_buff, dev: *mut net_device, _pt: *mut packet_type, orig_dev: *mut net_device) -> c_int {
    let mut rc: c_int = 1;
    let proto: *mut datalink_proto;
    static mut snap_packet_type: packet_type = packet_type { type_: ETH_P_SNAP.to_be() };

    if !pskb_may_pull(skb, 5) { kfree_skb(skb); return rc; }
    proto = find_snap_client((*skb).data());
    if !proto.is_null() {
        skb_pull_rcsum(skb, 5);
        skb_reset_transport_header(skb);
        if let Some(f) = (*proto).rcvfunc { rc = f(skb, dev, &mut snap_packet_type, orig_dev); }
    }
    if proto.is_null() { kfree_skb(skb); }
    rc
}

unsafe extern "C" fn snap_request(dl: *mut datalink_proto, skb: *mut sk_buff, dest: *const u8) -> c_int {
    memcpy(skb_push(skb, 5), (*dl).type_.as_ptr(), 5);
    llc_build_and_send_ui_pkt(snap_sap, skb, dest, (*snap_sap).laddr.lsap);
    0
}

static snap_err_msg: &[u8] = b"SNAP - unable to register with 802.2\n\0";

unsafe extern "C" fn snap_init() -> c_int {
    snap_sap = llc_sap_open(0xAA, snap_rcv);
    if snap_sap.is_null() { printk(snap_err_msg.as_ptr()); return -EBUSY; }
    0
}

unsafe extern "C" fn snap_exit() { llc_sap_put(snap_sap); }

pub unsafe extern "C" fn register_snap_client(desc: *const u8, rcvfunc: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device, *mut packet_type, *mut net_device) -> c_int>) -> *mut datalink_proto {
    let mut proto: *mut datalink_proto = core::ptr::null_mut();
    if !find_snap_client(desc).is_null() { return proto; }
    proto = kmalloc_datalink_proto(GFP_ATOMIC);
    if !proto.is_null() {
        memcpy((*proto).type_.as_mut_ptr(), desc, 5);
        (*proto).rcvfunc = rcvfunc;
        (*proto).header_length = 5 + 3;
        (*proto).request = Some(snap_request);
        (*proto).node.next = snap_list.next;
        snap_list.next = &mut (*proto).node;
    }
    proto
}

pub unsafe extern "C" fn unregister_snap_client(proto: *mut datalink_proto) {
    (*(*proto).node.prev).next = (*proto).node.next;
    (*(*proto).node.next).prev = (*proto).node.prev;
    synchronize_net();
    kfree(proto);
}

// module_init(snap_init); module_exit(snap_exit);
// EXPORT_SYMBOL(register_snap_client); EXPORT_SYMBOL(unregister_snap_client);
// MODULE_DESCRIPTION("SNAP data link layer. Derived from 802.2");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
