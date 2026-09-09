// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * NET3: Fibre Channel device handling subroutines
 *
 * Vineet Abraham <vma@iol.unh.edu>
 * v 1.0 03/22/99
 */

// Linux kernel dependencies supplied by other translation units.

unsafe extern "C" {
    fn skb_push(skb: *mut sk_buff, len: usize) -> *mut core::ffi::c_void;
    fn alloc_netdev(
        sizeof_priv: i32,
        name: *const core::ffi::c_char,
        name_assign_type: u32,
        setup: Option<unsafe extern "C" fn(*mut net_device)>,
    ) -> *mut net_device;
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, count: usize);
    fn memset(dest: *mut core::ffi::c_void, value: i32, count: usize);
    fn htons(value: u16) -> u16;
}

#[repr(C)]
pub struct sk_buff;
#[repr(C)]
pub struct net_device;
#[repr(C)]
pub struct fch_hdr {
    pub daddr: [u8; FC_ALEN as usize],
    pub saddr: [u8; FC_ALEN as usize],
}
#[repr(C)]
pub struct fcllc {
    pub dsap: u8,
    pub ssap: u8,
    pub llc: u8,
    pub protid: [u8; 3],
    pub ethertype: u16,
}

#[repr(C)]
pub struct header_ops {
    pub create: Option<unsafe extern "C" fn(
        *mut sk_buff,
        *mut net_device,
        u16,
        *const core::ffi::c_void,
        *const core::ffi::c_void,
        u32,
    ) -> i32>,
}

extern "C" {
    static fc_header_ops: header_ops;
}

const ETH_P_IP: u16 = 0x0800;
const ETH_P_ARP: u16 = 0x0806;
const EXTENDED_SAP: u8 = 0xaa;
const UI_CMD: u8 = 0x03;
const ARPHRD_IEEE802: u16 = 6;
const FC_HLEN: u16 = 24;
const FC_ALEN: u16 = 6;
const IFF_BROADCAST: u32 = 0x2;
const NET_NAME_UNKNOWN: u32 = 0;

// Fields supplied by the Linux net_device definition.
#[repr(C)]
pub struct net_device_fields {
    pub header_ops: *const header_ops,
    pub type_: u16,
    pub hard_header_len: u16,
    pub mtu: u32,
    pub addr_len: u8,
    pub tx_queue_len: u32,
    pub flags: u32,
    pub broadcast: *mut u8,
    pub dev_addr: *const u8,
}

/* Put the headers on a Fibre Channel packet. */
unsafe extern "C" fn fc_header(
    skb: *mut sk_buff,
    dev: *mut net_device,
    type_: u16,
    daddr: *const core::ffi::c_void,
    saddr: *const core::ffi::c_void,
    _len: u32,
) -> i32 {
    let mut hdr_len: i32;
    let fch: *mut fch_hdr;

    /* Add the 802.2 SNAP header if IP as the IPv4 code calls
     * dev->hard_header directly. */
    if type_ == ETH_P_IP || type_ == ETH_P_ARP {
        hdr_len = (core::mem::size_of::<fch_hdr>() + core::mem::size_of::<fcllc>()) as i32;
        fch = skb_push(skb, hdr_len as usize) as *mut fch_hdr;
        let fcllc = fch.add(1) as *mut fcllc;
        (*fcllc).dsap = EXTENDED_SAP;
        (*fcllc).ssap = EXTENDED_SAP;
        (*fcllc).llc = UI_CMD;
        (*fcllc).protid[0] = 0x00;
        (*fcllc).protid[1] = 0x00;
        (*fcllc).protid[2] = 0x00;
        (*fcllc).ethertype = htons(type_);
    } else {
        hdr_len = core::mem::size_of::<fch_hdr>() as i32;
        fch = skb_push(skb, hdr_len as usize) as *mut fch_hdr;
    }

    if !saddr.is_null() {
        memcpy((*fch).saddr.as_mut_ptr() as *mut core::ffi::c_void, saddr, (*dev_fields(dev)).addr_len as usize);
    } else {
        memcpy((*fch).saddr.as_mut_ptr() as *mut core::ffi::c_void, (*dev_fields(dev)).dev_addr as *const core::ffi::c_void, (*dev_fields(dev)).addr_len as usize);
    }

    if !daddr.is_null() {
        memcpy((*fch).daddr.as_mut_ptr() as *mut core::ffi::c_void, daddr, (*dev_fields(dev)).addr_len as usize);
        return hdr_len;
    }
    -hdr_len
}

#[inline]
unsafe fn dev_fields(dev: *mut net_device) -> *mut net_device_fields {
    dev as *mut net_device_fields
}

unsafe extern "C" fn fc_setup(dev: *mut net_device) {
    let dev = dev_fields(dev);
    (*dev).header_ops = &fc_header_ops;
    (*dev).type_ = ARPHRD_IEEE802;
    (*dev).hard_header_len = FC_HLEN;
    (*dev).mtu = 2024;
    (*dev).addr_len = FC_ALEN as u8;
    (*dev).tx_queue_len = 100; /* Long queues on fc */
    (*dev).flags = IFF_BROADCAST;

    memset((*dev).broadcast as *mut core::ffi::c_void, 0xFF, FC_ALEN as usize);
}

/*
 * alloc_fcdev - Register fibre channel device
 * @sizeof_priv: Size of additional driver-private structure to be allocated
 * for this fibre channel device
 *
 * Fill in the fields of the device structure with fibre channel-generic values.
 *
 * Constructs a new net device, complete with a private data area of
 * size @sizeof_priv. A 32-byte (not bit) alignment is enforced for this
 * private data area.
 */
#[no_mangle]
pub unsafe extern "C" fn alloc_fcdev(sizeof_priv: i32) -> *mut net_device {
    alloc_netdev(sizeof_priv, b"fc%d\0".as_ptr() as *const core::ffi::c_char, NET_NAME_UNKNOWN, Some(fc_setup))
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
