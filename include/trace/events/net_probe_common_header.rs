/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _TRACE_NET_PROBE_COMMON_H / TRACE_HEADER_MULTI_READ

macro_rules! TP_STORE_ADDR_PORTS_V4 {
    ($entry:expr, $inet:expr, $sk:expr) => {{
        let mut v4 = unsafe { ($entry).saddr as *mut sockaddr_in };
        unsafe {
            (*v4).sin_family = AF_INET;
            (*v4).sin_port = ($inet).inet_sport;
            (*v4).sin_addr.s_addr = ($inet).inet_saddr;
            v4 = ($entry).daddr as *mut sockaddr_in;
            (*v4).sin_family = AF_INET;
            (*v4).sin_port = ($inet).inet_dport;
            (*v4).sin_addr.s_addr = ($inet).inet_daddr;
        }
    }};
}

// IS_ENABLED(CONFIG_IPV6) selects the IPv6-capable definition at build time.
#[cfg(feature = "CONFIG_IPV6")]
macro_rules! TP_STORE_ADDR_PORTS {
    ($entry:expr, $inet:expr, $sk:expr) => {{
        if unsafe { (*($sk)).sk_family } == AF_INET6 {
            let mut v6 = unsafe { ($entry).saddr as *mut sockaddr_in6 };
            unsafe {
                (*v6).sin6_family = AF_INET6;
                (*v6).sin6_port = ($inet).inet_sport;
                (*v6).sin6_addr = inet6_sk($sk).saddr;
                v6 = ($entry).daddr as *mut sockaddr_in6;
                (*v6).sin6_family = AF_INET6;
                (*v6).sin6_port = ($inet).inet_dport;
                (*v6).sin6_addr = (*($sk)).sk_v6_daddr;
            }
        } else {
            TP_STORE_ADDR_PORTS_V4!($entry, $inet, $sk);
        }
    }};
}

#[cfg(not(feature = "CONFIG_IPV6"))]
macro_rules! TP_STORE_ADDR_PORTS {
    ($entry:expr, $inet:expr, $sk:expr) => {{
        TP_STORE_ADDR_PORTS_V4!($entry, $inet, $sk);
    }};
}

macro_rules! TP_STORE_V4MAPPED {
    ($entry:expr, $saddr:expr, $daddr:expr) => {{
        let mut pin6: *mut in6_addr;
        unsafe {
            pin6 = ($entry).saddr_v6 as *mut in6_addr;
            ipv6_addr_set_v4mapped($saddr, pin6);
            pin6 = ($entry).daddr_v6 as *mut in6_addr;
            ipv6_addr_set_v4mapped($daddr, pin6);
        }
    }};
}

#[cfg(feature = "CONFIG_IPV6")]
macro_rules! TP_STORE_ADDRS {
    ($entry:expr, $saddr:expr, $daddr:expr, $saddr6:expr, $daddr6:expr) => {{
        if unsafe { (*sk).sk_family } == AF_INET6 {
            let mut pin6: *mut in6_addr;
            unsafe {
                pin6 = ($entry).saddr_v6 as *mut in6_addr;
                *pin6 = $saddr6;
                pin6 = ($entry).daddr_v6 as *mut in6_addr;
                *pin6 = $daddr6;
            }
        } else {
            TP_STORE_V4MAPPED!($entry, $saddr, $daddr);
        }
    }};
}

#[cfg(not(feature = "CONFIG_IPV6"))]
macro_rules! TP_STORE_ADDRS {
    ($entry:expr, $saddr:expr, $daddr:expr, $saddr6:expr, $daddr6:expr) => {{
        TP_STORE_V4MAPPED!($entry, $saddr, $daddr);
    }};
}

macro_rules! TP_STORE_ADDR_PORTS_SKB_V4 {
    ($skb:expr, $protoh:expr, $entry_saddr:expr, $entry_daddr:expr) => {{
        let mut v4 = $entry_saddr as *mut sockaddr_in;
        unsafe {
            (*v4).sin_family = AF_INET;
            (*v4).sin_port = ($protoh).source;
            (*v4).sin_addr.s_addr = ip_hdr($skb).saddr;
            v4 = $entry_daddr as *mut sockaddr_in;
            (*v4).sin_family = AF_INET;
            (*v4).sin_port = ($protoh).dest;
            (*v4).sin_addr.s_addr = ip_hdr($skb).daddr;
        }
    }};
}

#[cfg(feature = "CONFIG_IPV6")]
macro_rules! TP_STORE_ADDR_PORTS_SKB {
    ($skb:expr, $protoh:expr, $entry_saddr:expr, $entry_daddr:expr) => {{
        let iph = ip_hdr($skb);
        if iph.version == 6 {
            let mut v6 = $entry_saddr as *mut sockaddr_in6;
            unsafe {
                (*v6).sin6_family = AF_INET6;
                (*v6).sin6_port = ($protoh).source;
                (*v6).sin6_addr = ipv6_hdr($skb).saddr;
                v6 = $entry_daddr as *mut sockaddr_in6;
                (*v6).sin6_family = AF_INET6;
                (*v6).sin6_port = ($protoh).dest;
                (*v6).sin6_addr = ipv6_hdr($skb).daddr;
            }
        } else {
            TP_STORE_ADDR_PORTS_SKB_V4!($skb, $protoh, $entry_saddr, $entry_daddr);
        }
    }};
}

#[cfg(not(feature = "CONFIG_IPV6"))]
macro_rules! TP_STORE_ADDR_PORTS_SKB {
    ($skb:expr, $protoh:expr, $entry_saddr:expr, $entry_daddr:expr) => {{
        TP_STORE_ADDR_PORTS_SKB_V4!($skb, $protoh, $entry_saddr, $entry_daddr);
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
