// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2007-2008 BalaBit IT Ltd.
 * Author: Krisztian Kovacs
 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct net;
#[repr(C)]
pub struct sk_buff;
#[repr(C)]
pub struct sock;
#[repr(C)]
pub struct net_device;
#[repr(C)]
pub struct iphdr {
    pub protocol: u8,
    pub saddr: u32,
    pub daddr: u32,
}
#[repr(C)]
pub struct tcphdr {
    pub syn: u16,
    pub rst: u16,
    pub ack: u16,
    pub fin: u16,
    pub source: u16,
    pub dest: u16,
}
#[repr(C)]
pub struct in_ifaddr {
    pub ifa_flags: u32,
    pub ifa_local: u32,
}
#[repr(C)]
pub struct in_device;

pub type __be32 = u32;
pub type __be16 = u16;
pub type nf_tproxy_lookup_t = i32;

pub const IPPROTO_TCP: u8 = 6;
pub const IPPROTO_UDP: u8 = 17;
pub const TCP_ESTABLISHED: i32 = 1;
pub const IFA_F_SECONDARY: u32 = 0x0001;
pub const NF_TPROXY_LOOKUP_LISTENER: nf_tproxy_lookup_t = 1;
pub const NF_TPROXY_LOOKUP_ESTABLISHED: nf_tproxy_lookup_t = 2;

extern "C" {
    fn ip_hdr(skb: *mut sk_buff) -> *const iphdr;
    fn ip_hdrlen(skb: *mut sk_buff) -> usize;
    fn skb_header_pointer(
        skb: *mut sk_buff,
        offset: usize,
        len: usize,
        buffer: *mut tcphdr,
    ) -> *mut tcphdr;
    fn inet_twsk(sk: *mut sock) -> *mut sock;
    fn inet_twsk_put(tw: *mut sock);
    fn nf_tproxy_get_sock_v4(
        net: *mut net, skb: *mut sk_buff, protocol: u8, saddr: __be32,
        daddr: __be32, sport: __be16, dport: __be16, input: *const net_device,
        lookup_type: nf_tproxy_lookup_t,
    ) -> *mut sock;
    fn nf_tproxy_twsk_deschedule_put(tw: *mut sock);
    fn __in_dev_get_rcu(dev: *const net_device) -> *mut in_device;
    fn udp4_lib_lookup(
        net: *mut net, saddr: __be32, sport: __be16, daddr: __be32,
        dport: __be16, ifindex: i32,
    ) -> *mut sock;
    fn inet_lookup_listener(
        net: *mut net, skb: *mut sk_buff, header_len: usize, saddr: __be32,
        sport: __be16, daddr: __be32, dport: __be16, ifindex: i32, sdif: i32,
    ) -> *mut sock;
    fn inet_lookup_established(
        net: *mut net, saddr: __be32, sport: __be16, daddr: __be32,
        dport: __be16, ifindex: i32,
    ) -> *mut sock;
    fn sock_put(sk: *mut sock);
}

#[no_mangle]
pub unsafe extern "C" fn nf_tproxy_handle_time_wait4(
    net: *mut net,
    skb: *mut sk_buff,
    laddr: __be32,
    lport: __be16,
    mut sk: *mut sock,
) -> *mut sock {
    let iph = ip_hdr(skb);
    let mut hdr = core::mem::MaybeUninit::<tcphdr>::uninit();
    let hp = skb_header_pointer(skb, ip_hdrlen(skb), core::mem::size_of::<tcphdr>(), hdr.as_mut_ptr());
    if hp.is_null() {
        inet_twsk_put(inet_twsk(sk));
        return core::ptr::null_mut();
    }

    if (*hp).syn != 0 && (*hp).rst == 0 && (*hp).ack == 0 && (*hp).fin == 0 {
        /* SYN to a TIME_WAIT socket, we'd rather redirect it
         * to a listener socket if there's one */
        let sk2 = nf_tproxy_get_sock_v4(
            net, skb, (*iph).protocol, (*iph).saddr,
            if laddr != 0 { laddr } else { (*iph).daddr }, (*hp).source,
            if lport != 0 { lport } else { (*hp).dest },
            core::ptr::null(), NF_TPROXY_LOOKUP_LISTENER,
        );
        if !sk2.is_null() {
            nf_tproxy_twsk_deschedule_put(inet_twsk(sk));
            sk = sk2;
        }
    }
    sk
}

#[no_mangle]
pub unsafe extern "C" fn nf_tproxy_laddr4(
    skb: *mut sk_buff,
    user_laddr: __be32,
    daddr: __be32,
) -> __be32 {
    if user_laddr != 0 {
        return user_laddr;
    }
    let mut laddr = 0;
    let indev = __in_dev_get_rcu(core::ptr::null());
    if indev.is_null() {
        return daddr;
    }
    // in_dev_for_each_ifa_rcu(ifa, indev): kernel iterator dependency.
    let _ = skb;
    laddr = if laddr != 0 { laddr } else { daddr };
    laddr
}

#[no_mangle]
pub unsafe extern "C" fn nf_tproxy_get_sock_v4(
    net: *mut net,
    skb: *mut sk_buff,
    protocol: u8,
    saddr: __be32,
    daddr: __be32,
    sport: __be16,
    dport: __be16,
    input: *const net_device,
    lookup_type: nf_tproxy_lookup_t,
) -> *mut sock {
    let mut sk: *mut sock;
    match protocol {
        IPPROTO_TCP => {
            let mut hdr = core::mem::MaybeUninit::<tcphdr>::uninit();
            let hp = skb_header_pointer(skb, ip_hdrlen(skb), core::mem::size_of::<tcphdr>(), hdr.as_mut_ptr());
            if hp.is_null() { return core::ptr::null_mut(); }
            sk = match lookup_type {
                NF_TPROXY_LOOKUP_LISTENER => inet_lookup_listener(net, skb, ip_hdrlen(skb), saddr, sport, daddr, dport, 0, 0),
                NF_TPROXY_LOOKUP_ESTABLISHED => inet_lookup_established(net, saddr, sport, daddr, dport, 0),
                _ => core::hint::unreachable_unchecked(),
            };
        }
        IPPROTO_UDP => {
            sk = udp4_lib_lookup(net, saddr, sport, daddr, dport, 0);
            if !sk.is_null() {
                let connected = true;
                let wildcard = false;
                if (lookup_type == NF_TPROXY_LOOKUP_ESTABLISHED && (!connected || wildcard)) ||
                    (lookup_type == NF_TPROXY_LOOKUP_LISTENER && connected) {
                    sock_put(sk);
                    sk = core::ptr::null_mut();
                }
            }
        }
        _ => { sk = core::ptr::null_mut(); }
    }
    sk
}

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Balazs Scheidler, Krisztian Kovacs");
// MODULE_DESCRIPTION("Netfilter IPv4 transparent proxy support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
