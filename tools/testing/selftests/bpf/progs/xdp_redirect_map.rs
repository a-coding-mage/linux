// SPDX-License-Identifier: GPL-2.0

// Translated from C source using external Linux/BPF definitions:
// linux/if_ether.h, linux/bpf.h, bpf/bpf_helpers.h, bpf/bpf_endian.h

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

const BPF_MAP_TYPE_DEVMAP: u32 = 14;
const BPF_MAP_TYPE_ARRAY: u32 = 2;
const ETH_ALEN: usize = 6;
const ETH_P_IP: u16 = 0x0800;
const XDP_DROP: c_int = 1;
const XDP_PASS: c_int = 2;

#[repr(C)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
    pub data_meta: u32,
    pub ingress_ifindex: u32,
    pub rx_queue_index: u32,
    pub egress_ifindex: u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; ETH_ALEN],
    pub h_source: [u8; ETH_ALEN],
    pub h_proto: u16,
}

type __u32 = u32;
type __u64 = u64;
type __be64 = u64;

#[repr(C)]
pub struct tx_port_map {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut tx_port: tx_port_map = tx_port_map {
    type_: BPF_MAP_TYPE_DEVMAP,
    max_entries: 8,
    key_size: size_of::<c_int>() as u32,
    value_size: size_of::<c_int>() as u32,
};

unsafe extern "C" {
    fn bpf_redirect_map(map: *mut c_void, key: __u32, flags: __u64) -> c_int;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_map_update_elem(
        map: *mut c_void,
        key: *const c_void,
        value: *const c_void,
        flags: __u64,
    ) -> c_int;
    fn bpf_printk(fmt: *const c_char, ...) -> c_int;
}

#[inline]
fn bpf_htons(x: u16) -> u16 {
    x.to_be()
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "xdp")]
pub unsafe extern "C" fn xdp_redirect_map_0(xdp: *mut xdp_md) -> c_int {
    let _ = xdp;
    unsafe { bpf_redirect_map(&raw mut tx_port as *mut c_void, 0, 0) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "xdp")]
pub unsafe extern "C" fn xdp_redirect_map_1(xdp: *mut xdp_md) -> c_int {
    let _ = xdp;
    unsafe { bpf_redirect_map(&raw mut tx_port as *mut c_void, 1, 0) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "xdp")]
pub unsafe extern "C" fn xdp_redirect_map_2(xdp: *mut xdp_md) -> c_int {
    let _ = xdp;
    unsafe { bpf_redirect_map(&raw mut tx_port as *mut c_void, 2, 0) }
}

#[repr(C)]
pub struct rxcnt_map {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut rxcnt: rxcnt_map = rxcnt_map {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 3,
    key_size: size_of::<__u32>() as u32,
    value_size: size_of::<__u64>() as u32,
};

unsafe fn xdp_count(xdp: *mut xdp_md, key: __u32) -> c_int {
    let data_end = unsafe { (*xdp).data_end as usize as *mut c_void };
    let data = unsafe { (*xdp).data as usize as *mut c_void };
    let eth = data as *mut ethhdr;
    let count: *mut __u64;

    if unsafe { (data as *mut u8).add(size_of::<ethhdr>()) } > data_end as *mut u8 {
        return XDP_DROP;
    }

    if unsafe { bpf_htons((*eth).h_proto) as c_int == ETH_P_IP as c_int } {
        /* We only count IPv4 packets */
        count = unsafe {
            bpf_map_lookup_elem(
                &raw mut rxcnt as *mut c_void,
                &key as *const __u32 as *const c_void,
            ) as *mut __u64
        };
        if !count.is_null() {
            unsafe {
                *count = (*count).wrapping_add(1);
            }
        }
    }

    XDP_PASS
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "xdp")]
pub unsafe extern "C" fn xdp_count_0(xdp: *mut xdp_md) -> c_int {
    unsafe { xdp_count(xdp, 0) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "xdp")]
pub unsafe extern "C" fn xdp_count_1(xdp: *mut xdp_md) -> c_int {
    unsafe { xdp_count(xdp, 1) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "xdp")]
pub unsafe extern "C" fn xdp_count_2(xdp: *mut xdp_md) -> c_int {
    unsafe { xdp_count(xdp, 2) }
}

#[repr(C)]
pub struct rx_mac_map {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut rx_mac: rx_mac_map = rx_mac_map {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 2,
    key_size: size_of::<__u32>() as u32,
    value_size: size_of::<__be64>() as u32,
};

unsafe fn store_mac(xdp: *mut xdp_md, id: __u32) -> c_int {
    let data_end = unsafe { (*xdp).data_end as usize as *mut c_void };
    let data = unsafe { (*xdp).data as usize as *mut c_void };
    let eth = data as *mut ethhdr;
    let key: __u32 = id;
    let mut mac: __be64 = 0;

    if unsafe { (data as *mut u8).add(size_of::<ethhdr>()) } > data_end as *mut u8 {
        return XDP_DROP;
    }

    /* Only store IPv4 MAC to avoid being polluted by IPv6 packets */
    if unsafe { (*eth).h_proto == bpf_htons(ETH_P_IP) } {
        unsafe {
            ptr::copy_nonoverlapping(
                (*eth).h_source.as_ptr(),
                &mut mac as *mut __be64 as *mut u8,
                ETH_ALEN,
            );
            bpf_map_update_elem(
                &raw mut rx_mac as *mut c_void,
                &key as *const __u32 as *const c_void,
                &mac as *const __be64 as *const c_void,
                0,
            );
            bpf_printk(
                c"%s - %x".as_ptr(),
                c"store_mac".as_ptr(),
                mac,
            );
        }
    }

    XDP_PASS
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "xdp")]
pub unsafe extern "C" fn store_mac_1(xdp: *mut xdp_md) -> c_int {
    unsafe { store_mac(xdp, 0) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "xdp")]
pub unsafe extern "C" fn store_mac_2(xdp: *mut xdp_md) -> c_int {
    unsafe { store_mac(xdp, 1) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
