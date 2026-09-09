/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the Linux trace event `icmp_send` in icmp.h.
// The declarations referenced by this header (`sk_buff`, `iphdr`, `udphdr`,
// `ip_hdr`, `udp_hdr`, `skb_tail_pointer`, `udp_get_len_short`, and `ntohs`)
// are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct IcmpSendEntry {
    pub skbaddr: *const core::ffi::c_void,
    pub type_: core::ffi::c_int,
    pub code: core::ffi::c_int,
    pub saddr: [u8; 4],
    pub daddr: [u8; 4],
    pub sport: u16,
    pub dport: u16,
    pub ulen: u16,
}

/// Equivalent of the `TP_fast_assign` block of `TRACE_EVENT(icmp_send, ...)`.
///
/// # Safety
/// This has the same pointer and packet-boundary requirements as the original
/// C tracepoint assignment block.
pub unsafe fn icmp_send_fast_assign(
    entry: *mut IcmpSendEntry,
    skb: *const sk_buff,
    type_: core::ffi::c_int,
    code: core::ffi::c_int,
) {
    let iph: *mut iphdr = ip_hdr(skb);
    let uh: *mut udphdr = udp_hdr(skb);
    let proto_4: u8 = (*iph).protocol;
    let mut p32: *mut u32;

    (*entry).skbaddr = skb.cast();
    (*entry).type_ = type_;
    (*entry).code = code;

    if proto_4 != IPPROTO_UDP
        || (uh.cast::<u8>() as usize) < ((*skb).head as usize)
        || (uh.cast::<u8>() as usize).wrapping_add(core::mem::size_of::<udphdr>())
            > (skb_tail_pointer(skb) as usize)
    {
        (*entry).sport = 0;
        (*entry).dport = 0;
        (*entry).ulen = 0;
    } else {
        (*entry).sport = ntohs((*uh).source);
        (*entry).dport = ntohs((*uh).dest);
        (*entry).ulen = udp_get_len_short(uh);
    }

    p32 = (*entry).saddr.as_mut_ptr().cast();
    *p32 = (*iph).saddr;

    p32 = (*entry).daddr.as_mut_ptr().cast();
    *p32 = (*iph).daddr;
}

// TP_printk format from the source tracepoint:
// "icmp_send: type=%d, code=%d. From %pI4:%u to %pI4:%u ulen=%d skbaddr=%p"

// External kernel declarations used by this header.
extern "C" {
    pub type sk_buff;
    pub type iphdr;
    pub type udphdr;

    fn ip_hdr(skb: *const sk_buff) -> *mut iphdr;
    fn udp_hdr(skb: *const sk_buff) -> *mut udphdr;
    fn skb_tail_pointer(skb: *const sk_buff) -> *const core::ffi::c_void;
    fn udp_get_len_short(uh: *const udphdr) -> u16;
    fn ntohs(value: u16) -> u16;
}

const IPPROTO_UDP: u8 = 17;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
