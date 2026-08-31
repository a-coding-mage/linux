// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_endian.h>, <bpf/bpf_helpers.h>,
// <linux/ip.h>, <linux/if_ether.h>

const ETH_HLEN: u32 = 14;
const BPF_OK: i32 = 0;
const BPF_DROP: i32 = 2;
const BPF_F_INGRESS: u64 = 1;

#[repr(C)]
pub struct __sk_buff {
    pub data: u32,
    pub data_end: u32,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: u8,
    pub tos: u8,
    pub tot_len: u16,
    pub id: u16,
    pub frag_off: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: u16,
    pub saddr: u32,
    pub daddr: u32,
}

extern "C" {
    fn bpf_skb_change_head(skb: *mut __sk_buff, len: u32, flags: u64) -> i64;
    fn bpf_skb_store_bytes(
        skb: *mut __sk_buff,
        offset: u32,
        from: *const core::ffi::c_void,
        len: u32,
        flags: u64,
    ) -> i64;
    fn bpf_redirect(ifindex: u32, flags: u64) -> i64;
}

#[inline]
fn bpf_ntohl(x: u32) -> u32 {
    u32::from_be(x)
}

/* We don't care about whether the packet can be received by network stack.
 * Just care if the packet is sent to the correct device at correct direction
 * and not panic the kernel.
 */
unsafe fn prepend_dummy_mac(skb: *mut __sk_buff) -> i32 {
    let mac: [u8; 14] = [
        0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0xf, 0xe, 0xd, 0xc, 0xb, 0xa, 0x08, 0x00,
    ];

    if bpf_skb_change_head(skb, ETH_HLEN, 0) != 0 {
        return -1;
    }

    if bpf_skb_store_bytes(
        skb,
        0,
        mac.as_ptr() as *const core::ffi::c_void,
        core::mem::size_of_val(&mac) as u32,
        0,
    ) != 0
    {
        return -1;
    }

    0
}

/* Use the last byte of IP address to redirect the packet */
unsafe fn get_redirect_target(skb: *mut __sk_buff) -> i32 {
    let mut iph: *mut iphdr = core::ptr::null_mut();
    let start = (*skb).data as usize as *mut core::ffi::c_void;
    let end = (*skb).data_end as usize as *mut core::ffi::c_void;

    if (start as usize).wrapping_add(core::mem::size_of::<iphdr>()) > end as usize {
        return -1;
    }

    iph = start as *mut iphdr;
    (bpf_ntohl((*iph).daddr) & 0xff) as i32
}

#[no_mangle]
#[link_section = "redir_ingress"]
pub unsafe extern "C" fn test_lwt_redirect_in(skb: *mut __sk_buff) -> i32 {
    let target = get_redirect_target(skb);

    if target < 0 {
        return BPF_OK;
    }

    if prepend_dummy_mac(skb) != 0 {
        return BPF_DROP;
    }

    bpf_redirect(target as u32, BPF_F_INGRESS) as i32
}

#[no_mangle]
#[link_section = "redir_egress"]
pub unsafe extern "C" fn test_lwt_redirect_out(skb: *mut __sk_buff) -> i32 {
    let target = get_redirect_target(skb);

    if target < 0 {
        return BPF_OK;
    }

    if prepend_dummy_mac(skb) != 0 {
        return BPF_DROP;
    }

    bpf_redirect(target as u32, 0) as i32
}

#[no_mangle]
#[link_section = "redir_egress_nomac"]
pub unsafe extern "C" fn test_lwt_redirect_out_nomac(skb: *mut __sk_buff) -> i32 {
    let target = get_redirect_target(skb);

    if target < 0 {
        return BPF_OK;
    }

    bpf_redirect(target as u32, 0) as i32
}

#[no_mangle]
#[link_section = "redir_ingress_nomac"]
pub unsafe extern "C" fn test_lwt_redirect_in_nomac(skb: *mut __sk_buff) -> i32 {
    let target = get_redirect_target(skb);

    if target < 0 {
        return BPF_OK;
    }

    bpf_redirect(target as u32, BPF_F_INGRESS) as i32
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
