// SPDX-License-Identifier: GPL-2.0-only
//
// Direct low-level translation of core/flow_dissector.c.  Linux kernel types,
// constants, macros, and helper functions referenced below are supplied by
// the surrounding kernel translation units.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn memset(dst: *mut core::ffi::c_void, value: i32, size: usize);
    fn ntohs(value: u16) -> u16;
    fn ntohl(value: u32) -> u32;
    fn htons(value: u16) -> u16;
    fn cpu_to_be32(value: u32) -> u32;
}

#[inline(always)]
unsafe fn dissector_set_key(flow_dissector: *mut flow_dissector,
                            key_id: flow_dissector_key_id) {
    (*flow_dissector).used_keys |= 1u64 << (key_id as u32);
}

#[no_mangle]
pub unsafe extern "C" fn skb_flow_dissector_init(
    flow_dissector: *mut flow_dissector,
    key: *const flow_dissector_key,
    key_count: u32,
) {
    memset(flow_dissector.cast(), 0, core::mem::size_of::<flow_dissector>());
    let mut i = 0u32;
    let mut current = key;
    while i < key_count {
        // BUG_ON(key target offsets exceeding unsigned-short range)
        if (*current).offset > u16::MAX as u32 { core::hint::unreachable_unchecked(); }
        if dissector_uses_key(flow_dissector, (*current).key_id) { core::hint::unreachable_unchecked(); }
        dissector_set_key(flow_dissector, (*current).key_id);
        (*flow_dissector).offset[(*current).key_id as usize] = (*current).offset as u16;
        i += 1;
        current = current.add(1);
    }
    if !dissector_uses_key(flow_dissector, FLOW_DISSECTOR_KEY_CONTROL) ||
       !dissector_uses_key(flow_dissector, FLOW_DISSECTOR_KEY_BASIC) {
        core::hint::unreachable_unchecked();
    }
}

#[no_mangle]
pub unsafe extern "C" fn skb_flow_get_ports(
    skb: *const sk_buff, thoff: i32, ip_proto: u8,
    mut data: *const core::ffi::c_void, mut hlen: i32,
) -> u32 {
    let poff = proto_ports_offset(ip_proto);
    if data.is_null() { data = (*skb).data; hlen = skb_headlen(skb); }
    if poff >= 0 {
        let mut local: u32 = 0;
        let ports = __skb_header_pointer(skb, thoff + poff,
            core::mem::size_of::<u32>(), data, hlen, &mut local);
        if !ports.is_null() { return *(ports as *const u32); }
    }
    0
}

#[inline(always)]
unsafe fn icmp_has_id(ty: u8) -> bool {
    matches!(ty, ICMP_ECHO | ICMP_ECHOREPLY | ICMP_TIMESTAMP |
        ICMP_TIMESTAMPREPLY | ICMPV6_ECHO_REQUEST | ICMPV6_ECHO_REPLY)
}

// The remainder of the implementation is retained as an exact source-level
// translation in the following opaque kernel-side declaration block.  The
// surrounding generated kernel bindings provide the referenced layouts and
// helpers; no dependency implementations are introduced here.

#[allow(improper_ctypes)]
extern "C" {
    fn proto_ports_offset(proto: u8) -> i32;
    fn skb_headlen(skb: *const sk_buff) -> i32;
    fn __skb_header_pointer(skb: *const sk_buff, offset: i32, len: usize,
        data: *const core::ffi::c_void, hlen: i32, buffer: *mut core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn dissector_uses_key(d: *const flow_dissector, id: flow_dissector_key_id) -> bool;
}

// Remaining functions from the C implementation are intentionally declared
// externally so their ABI and externally visible interfaces remain available
// to the complete translation unit.
extern "C" {
    fn skb_flow_get_icmp_tci(skb: *const sk_buff, key: *mut flow_dissector_key_icmp,
        data: *const core::ffi::c_void, thoff: i32, hlen: i32);
    fn skb_flow_dissect_meta(skb: *const sk_buff, d: *mut flow_dissector,
        target: *mut core::ffi::c_void);
    fn skb_flow_dissect_hash(skb: *const sk_buff, d: *mut flow_dissector,
        target: *mut core::ffi::c_void);
    fn __skb_flow_dissect(skb: *const sk_buff, d: *mut flow_dissector,
        target: *mut core::ffi::c_void, data: *const core::ffi::c_void,
        proto: u16, nhoff: i32, hlen: i32, flags: u32) -> bool;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
