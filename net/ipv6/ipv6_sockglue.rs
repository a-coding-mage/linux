// SPDX-License-Identifier: GPL-2.0-or-later
/* IPv6 BSD socket options, translated from the unchanged ipv6_sockglue.c.
 * Configured ABI/layout comes from the kernel headers; helper C contains only
 * declaration, macro and header-inline boundaries, never socket option policy. */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/ipv6_sockglue_generated.rs"));
}
use bindings::*;
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null_mut, read_volatile, write_volatile};
use kernel::ffi::{c_char, c_int};

#[no_mangle]
pub static mut ip6_ra_chain: *mut bindings::ip6_ra_chain = null_mut();

#[inline]
fn is_err<T>(p: *const T) -> bool { p as usize >= usize::MAX - 4094 }
#[inline]
unsafe fn copy_from<T>(dst: *mut T, src: sockptr_t, len: usize) -> i32 {
    rust_ip6_copy_from(dst.cast(), src, len)
}
#[inline]
unsafe fn copy_to<T>(dst: sockptr_t, src: *const T, len: usize) -> i32 {
    rust_ip6_copy_to(dst, src.cast(), len)
}
#[inline]
unsafe fn gf(p: *mut group_filter) -> *mut group_filter__bindgen_ty_1__bindgen_ty_2 { p.cast() }
#[inline]
unsafe fn family(p: *const __kernel_sockaddr_storage) -> u16 {
    (*p).__bindgen_anon_1.__bindgen_anon_1.ss_family
}
#[inline]
fn filter_size(n: u32) -> usize {
    (RUST_IP6_GROUP_FILTER_SIZE0 as usize).wrapping_add((n as usize).wrapping_mul(size_of::<__kernel_sockaddr_storage>()))
}
#[inline]
unsafe fn test_bit(sk: *mut sock, bit: u32) -> bool { rust_ip6_test_bit(sk, bit as i32) }
#[inline]
unsafe fn assign_bit(sk: *mut sock, bit: u32, val: bool) { rust_ip6_assign_bit(sk, bit as i32, val); }

#[no_mangle]
pub unsafe extern "C" fn ip6_ra_control(sk: *mut sock, sel: c_int) -> c_int {
    if rust_ip6_sk_type(sk) != RUST_IP6_SOCK_RAW || rust_ip6_inet_num(sk) != IPPROTO_RAW {
        return -(ENOPROTOOPT as i32);
    }
    let new_ra = if sel >= 0 { rust_ip6_kmalloc(size_of::<bindings::ip6_ra_chain>()).cast::<bindings::ip6_ra_chain>() } else { null_mut() };
    if sel >= 0 && new_ra.is_null() { return -(ENOMEM as i32); }
    rust_ip6_ra_lock();
    let mut rap = addr_of_mut!(ip6_ra_chain);
    let mut ra = *rap;
    while !ra.is_null() {
        if (*ra).sk == sk {
            if sel >= 0 {
                rust_ip6_ra_unlock();
                kfree(new_ra.cast());
                return -(EADDRINUSE as i32);
            }
            *rap = (*ra).next;
            rust_ip6_ra_unlock();
            rust_ip6_sock_put(sk);
            kfree(ra.cast());
            return 0;
        }
        rap = addr_of_mut!((*ra).next);
        ra = *rap;
    }
    if new_ra.is_null() {
        rust_ip6_ra_unlock();
        return -(ENOBUFS as i32);
    }
    (*new_ra).sk = sk;
    (*new_ra).sel = sel;
    (*new_ra).next = ra;
    *rap = new_ra;
    rust_ip6_sock_hold(sk);
    rust_ip6_ra_unlock();
    0
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_update_options(sk: *mut sock, opt: *mut ipv6_txoptions) -> *mut ipv6_txoptions {
    if test_bit(sk, INET_FLAGS_IS_ICSK) && !opt.is_null()
        && (1u32.wrapping_shl(rust_ip6_sk_state(sk)) & (TCPF_LISTEN | TCPF_CLOSE)) == 0
        && rust_ip6_inet_daddr(sk) != RUST_IP6_LOOPBACK4_IPV6 {
        let hdr_len = rust_ip6_psp_overhead(sk)
            .wrapping_add((*opt).opt_flen as u32)
            .wrapping_add((*opt).opt_nflen as u32);
        rust_ip6_set_ext_hdr_len(sk, hdr_len as u16);
        rust_ip6_sync_mss(sk, rust_ip6_pmtu_cookie(sk));
    }
    let old = rust_ip6_opt_xchg(sk, opt);
    rust_ip6_dst_reset(sk);
    old
}

unsafe fn copy_group_source(greqs: *mut group_source_req, optval: sockptr_t, optlen: i32) -> i32 {
    if rust_ip6_compat() {
        let mut gr32: compat_group_source_req = zeroed();
        if (optlen as usize) < size_of::<compat_group_source_req>() { return -(EINVAL as i32); }
        if copy_from(addr_of_mut!(gr32), optval, size_of::<compat_group_source_req>()) != 0 { return -(EFAULT as i32); }
        (*greqs).gsr_interface = gr32.gsr_interface;
        (*greqs).gsr_group = gr32.gsr_group;
        (*greqs).gsr_source = gr32.gsr_source;
    } else {
        if (optlen as usize) < size_of::<group_source_req>() { return -(EINVAL as i32); }
        if copy_from(greqs, optval, size_of::<group_source_req>()) != 0 { return -(EFAULT as i32); }
    }
    0
}
unsafe fn do_ipv6_mcast_group_source(sk: *mut sock, optname: u32, optval: sockptr_t, optlen: i32) -> i32 {
    let mut greqs: group_source_req = zeroed();
    let ret = copy_group_source(addr_of_mut!(greqs), optval, optlen);
    if ret != 0 { return ret; }
    if family(addr_of!(greqs.gsr_group)) as u32 != AF_INET6 || family(addr_of!(greqs.gsr_source)) as u32 != AF_INET6 { return -(EADDRNOTAVAIL as i32); }
    let (mode, add) = match optname {
        MCAST_BLOCK_SOURCE => (MCAST_EXCLUDE, 1),
        MCAST_UNBLOCK_SOURCE => (MCAST_EXCLUDE, 0),
        MCAST_JOIN_SOURCE_GROUP => {
            let psin = addr_of!(greqs.gsr_group).cast::<sockaddr_in6>();
            let ret = ipv6_sock_mc_join_ssm(sk, greqs.gsr_interface as i32, addr_of!((*psin).sin6_addr), MCAST_INCLUDE);
            if ret != 0 && ret != -(EADDRINUSE as i32) { return ret; }
            (MCAST_INCLUDE, 1)
        }
        _ => (MCAST_INCLUDE, 0),
    };
    ip6_mc_source(add, mode as i32, sk, addr_of_mut!(greqs))
}
unsafe fn ipv6_set_mcast_msfilter(sk: *mut sock, optval: sockptr_t, optlen: i32) -> i32 {
    if (optlen as usize) < filter_size(0) { return -(EINVAL as i32); }
    if optlen > rust_ip6_optmem_max(rust_ip6_net(sk)) { return -(ENOBUFS as i32); }
    let gsf = rust_ip6_memdup(optval, optlen as usize).cast::<group_filter>();
    if is_err(gsf) { return gsf as isize as i32; }
    let fields = gf(gsf);
    let n = (*fields).gf_numsrc;
    let ret = if n >= 0x1ffffff || n > sysctl_mld_max_msf as u32 { -(ENOBUFS as i32) }
        else if filter_size(n) > optlen as usize { -(EINVAL as i32) }
        else { ip6_mc_msfilter(sk, gsf, addr_of_mut!((*fields).gf_slist_flex).cast()) };
    kfree(gsf.cast());
    ret
}
unsafe fn compat_ipv6_set_mcast_msfilter(sk: *mut sock, optval: sockptr_t, optlen: i32) -> i32 {
    let size0 = RUST_IP6_COMPAT_GF_FLEX_OFFSET as i32;
    if optlen < size0 { return -(EINVAL as i32); }
    if optlen > rust_ip6_optmem_max(rust_ip6_net(sk)).wrapping_sub(4) { return -(ENOBUFS as i32); }
    let p = rust_ip6_kmalloc(optlen.wrapping_add(4) as usize);
    if p.is_null() { return -(ENOMEM as i32); }
    let gf32 = p.cast::<u8>().add(4).cast::<compat_group_filter__bindgen_ty_1__bindgen_ty_2>();
    let ret = if copy_from(gf32, optval, optlen as usize) != 0 { -(EFAULT as i32) } else {
        let n = (*gf32).gf_numsrc;
        // C's temporary n is signed here (unlike the native gf_numsrc).
        if n >= 0x1ffffff || (n as i32) > sysctl_mld_max_msf { -(ENOBUFS as i32) }
        else if (size0 as usize).wrapping_add(n as usize * size_of::<__kernel_sockaddr_storage>()) > optlen as usize { -(EINVAL as i32) }
        else {
            let mut native: group_filter = zeroed();
            let fields = gf(addr_of_mut!(native));
            (*fields).gf_interface = (*gf32).gf_interface;
            (*fields).gf_group = (*gf32).gf_group;
            (*fields).gf_fmode = (*gf32).gf_fmode;
            (*fields).gf_numsrc = (*gf32).gf_numsrc;
            ip6_mc_msfilter(sk, addr_of_mut!(native), addr_of_mut!((*gf32).gf_slist_flex).cast())
        }
    };
    kfree(p);
    ret
}
unsafe fn ipv6_mcast_join_leave(sk: *mut sock, optname: u32, optval: sockptr_t, optlen: i32) -> i32 {
    let mut greq: group_req = zeroed();
    if rust_ip6_compat() {
        let mut gr32: compat_group_req = zeroed();
        if (optlen as usize) < size_of::<compat_group_req>() { return -(EINVAL as i32); }
        if copy_from(addr_of_mut!(gr32), optval, size_of::<compat_group_req>()) != 0 { return -(EFAULT as i32); }
        greq.gr_interface = gr32.gr_interface;
        greq.gr_group = gr32.gr_group;
    } else {
        if (optlen as usize) < size_of::<group_req>() { return -(EINVAL as i32); }
        if copy_from(addr_of_mut!(greq), optval, size_of::<group_req>()) != 0 { return -(EFAULT as i32); }
    }
    if family(addr_of!(greq.gr_group)) as u32 != AF_INET6 { return -(EADDRNOTAVAIL as i32); }
    let psin = addr_of!(greq.gr_group).cast::<sockaddr_in6>();
    if optname == MCAST_JOIN_GROUP { ipv6_sock_mc_join(sk, greq.gr_interface as i32, addr_of!((*psin).sin6_addr)) }
    else { ipv6_sock_mc_drop(sk, greq.gr_interface as i32, addr_of!((*psin).sin6_addr)) }
}
unsafe fn release_options(sk: *mut sock, opt: *mut ipv6_txoptions) {
    if !opt.is_null() {
        rust_ip6_omem_sub(sk, (*opt).tot_len);
        rust_ip6_txopt_put(opt);
    }
}
unsafe fn ipv6_set_opt_hdr(sk: *mut sock, optname: u32, optval: sockptr_t, optlen: i32) -> i32 {
    if optname != IPV6_RTHDR && !sockopt_ns_capable(rust_ip6_user_ns(rust_ip6_net(sk)), CAP_NET_RAW as i32) { return -(EPERM as i32); }
    let mut new: *mut ipv6_opt_hdr = null_mut();
    if optlen > 0 {
        if rust_ip6_sockptr_null(optval) || (optlen as usize) < size_of::<ipv6_opt_hdr>() || optlen & 7 != 0 || optlen > 8 * 255 { return -(EINVAL as i32); }
        new = rust_ip6_memdup(optval, optlen as usize).cast();
        if is_err(new) { return new as isize as i32; }
        if (((*new).hdrlen as i32 + 1) << 3) > optlen {
            kfree(new.cast());
            return -(EINVAL as i32);
        }
    }
    let opt = ipv6_renew_options(sk, rust_ip6_opt_get_locked(sk), optname as i32, new);
    kfree(new.cast());
    if is_err(opt) { return opt as isize as i32; }
    if optname == IPV6_RTHDR && !opt.is_null() && !(*opt).srcrt.is_null() {
        let hdr = (*opt).srcrt;
        let valid = match (*hdr).type_ as u32 {
            #[cfg(any(CONFIG_IPV6_MIP6, CONFIG_IPV6_MIP6_MODULE))]
            IPV6_SRCRT_TYPE_2 => (*hdr).hdrlen == 2 && (*hdr).segments_left == 1,
            IPV6_SRCRT_TYPE_4 => seg6_validate_srh(hdr.cast(), optlen, false),
            _ => false,
        };
        if !valid { release_options(sk, opt); return -(EINVAL as i32); }
    }
    release_options(sk, ipv6_update_options(sk, opt));
    0
}

#[no_mangle]
pub unsafe extern "C" fn do_ipv6_setsockopt(sk: *mut sock, _level: c_int, optname: c_int, optval: sockptr_t, optlen: u32) -> c_int {
    let np = rust_ip6_np(sk);
    let net = rust_ip6_net(sk);
    let mut val: i32 = 0;
    if !rust_ip6_sockptr_null(optval) && optlen as usize >= size_of::<i32>() {
        if copy_from(addr_of_mut!(val), optval, size_of::<i32>()) != 0 { return -(EFAULT as i32); }
    }
    let valbool = val != 0;
    if rust_ip6_mroute_opt(optname) { return rust_ip6_mroute_set(sk, optname, optval, optlen); }
    match optname as u32 {
        IPV6_UNICAST_HOPS => {
            if optlen < 4 || val > 255 || val < -1 { return -(EINVAL as i32); }
            write_volatile(addr_of_mut!((*np).hop_limit), val as i16);
            return 0;
        }
        IPV6_MULTICAST_LOOP => {
            if optlen < 4 || val != valbool as i32 { return -(EINVAL as i32); }
            assign_bit(sk, INET_FLAGS_MC6_LOOP, valbool);
            return 0;
        }
        IPV6_MULTICAST_HOPS => {
            if rust_ip6_sk_type(sk) == RUST_IP6_SOCK_STREAM { return -(ENOPROTOOPT as i32); }
            if optlen < 4 || val > 255 || val < -1 { return -(EINVAL as i32); }
            write_volatile(addr_of_mut!((*np).mcast_hops), if val == -1 { IPV6_DEFAULT_MCASTHOPS as u8 } else { val as u8 });
            return 0;
        }
        IPV6_MTU => {
            if optlen < 4 || (val != 0 && val < IPV6_MIN_MTU as i32) { return -(EINVAL as i32); }
            write_volatile(addr_of_mut!((*np).frag_size), val as u32);
            return 0;
        }
        IPV6_MINHOPCOUNT => {
            if optlen < 4 || val < 0 || val > 255 { return -(EINVAL as i32); }
            if val != 0 { rust_ip6_min_hopcount_enable(); }
            write_volatile(addr_of_mut!((*np).min_hopcount), val as u8);
            return 0;
        }
        IPV6_RECVERR_RFC4884 => {
            if optlen < 4 || val < 0 || val > 1 { return -(EINVAL as i32); }
            assign_bit(sk, INET_FLAGS_RECVERR6_RFC4884, valbool);
            return 0;
        }
        IPV6_MULTICAST_ALL => {
            if optlen < 4 { return -(EINVAL as i32); }
            assign_bit(sk, INET_FLAGS_MC6_ALL, valbool);
            return 0;
        }
        IPV6_AUTOFLOWLABEL => {
            assign_bit(sk, INET_FLAGS_AUTOFLOWLABEL, valbool);
            assign_bit(sk, INET_FLAGS_AUTOFLOWLABEL_SET, true);
            return 0;
        }
        IPV6_DONTFRAG => { assign_bit(sk, INET_FLAGS_DONTFRAG, valbool); return 0; }
        IPV6_RECVERR => {
            if optlen < 4 { return -(EINVAL as i32); }
            assign_bit(sk, INET_FLAGS_RECVERR6, valbool);
            if val == 0 { rust_ip6_errqueue_purge(sk); }
            return 0;
        }
        IPV6_ROUTER_ALERT_ISOLATE => {
            if optlen < 4 { return -(EINVAL as i32); }
            assign_bit(sk, INET_FLAGS_RTALERT_ISOLATE, valbool);
            return 0;
        }
        IPV6_MTU_DISCOVER => {
            if optlen < 4 || val < IPV6_PMTUDISC_DONT as i32 || val > IPV6_PMTUDISC_OMIT as i32 { return -(EINVAL as i32); }
            write_volatile(addr_of_mut!((*np).pmtudisc), val as u8);
            return 0;
        }
        IPV6_FLOWINFO_SEND => {
            if optlen < 4 { return -(EINVAL as i32); }
            assign_bit(sk, INET_FLAGS_SNDFLOW, valbool);
            return 0;
        }
        IPV6_ADDR_PREFERENCES => {
            if optlen < 4 { return -(EINVAL as i32); }
            return rust_ip6_set_addr_preferences(sk, val);
        }
        IPV6_MULTICAST_IF => {
            if rust_ip6_sk_type(sk) == RUST_IP6_SOCK_STREAM { return -(ENOPROTOOPT as i32); }
            if optlen < 4 { return -(EINVAL as i32); }
            if val != 0 {
                rust_ip6_rcu_lock();
                let dev = dev_get_by_index_rcu(net, val);
                if dev.is_null() { rust_ip6_rcu_unlock(); return -(ENODEV as i32); }
                let midx = rust_ip6_l3master(dev);
                rust_ip6_rcu_unlock();
                let bound = rust_ip6_bound_dev(sk);
                if bound != 0 && bound != val && (midx == 0 || midx != bound) { return -(EINVAL as i32); }
            }
            write_volatile(addr_of_mut!((*np).mcast_oif), val);
            return 0;
        }
        IPV6_UNICAST_IF => {
            if optlen != 4 { return -(EINVAL as i32); }
            let ifindex = u32::from_be(val as u32) as i32;
            if ifindex == 0 { write_volatile(addr_of_mut!((*np).ucast_oif), 0); return 0; }
            let dev = dev_get_by_index(net, ifindex);
            if dev.is_null() { return -(EADDRNOTAVAIL as i32); }
            rust_ip6_dev_put(dev);
            if rust_ip6_bound_dev(sk) != 0 { return -(EINVAL as i32); }
            write_volatile(addr_of_mut!((*np).ucast_oif), ifindex);
            return 0;
        }
        _ => (),
    }
    rust_ip6_lock_sock(sk);
    let ret = if rust_ip6_sk_family(sk) != AF_INET6 { -(ENOPROTOOPT as i32) }
        else { setsockopt_locked(sk, np, net, optname as u32, optval, optlen, val) };
    rust_ip6_release_sock(sk);
    ret
}

unsafe fn setsockopt_locked(sk: *mut sock, np: *mut ipv6_pinfo, net: *mut net, optname: u32, optval: sockptr_t, optlen: u32, mut val: i32) -> i32 {
    let valbool = val != 0;
    macro_rules! rxopt {
        ($field:ident) => {{
            if optlen < 4 { return -(EINVAL as i32); }
            (*np).rxopt.bits.$field(valbool as u16);
            0
        }};
    }
    match optname {
        IPV6_ADDRFORM => {
            if optlen < 4 || val != PF_INET as i32 { return -(EINVAL as i32); }
            if rust_ip6_sk_type(sk) == RUST_IP6_SOCK_RAW { return -(ENOPROTOOPT as i32); }
            let protocol = rust_ip6_sk_protocol(sk);
            if protocol == IPPROTO_UDP {
                if rust_ip6_udp_pending(sk) == AF_INET6 as i32 { return -(EBUSY as i32); }
            } else if protocol == IPPROTO_TCP {
                if rust_ip6_prot(sk) != addr_of_mut!(tcpv6_prot) { return -(EBUSY as i32); }
            } else { return -(ENOPROTOOPT as i32); }
            if rust_ip6_sk_state(sk) != TCP_ESTABLISHED { return -(ENOTCONN as i32); }
            if rust_ip6_only(sk) || !rust_ip6_v4mapped(rust_ip6_daddr(sk)) { return -(EADDRNOTAVAIL as i32); }
            __ipv6_sock_mc_close(sk);
            __ipv6_sock_ac_close(sk);
            if protocol == IPPROTO_TCP {
                rust_ip6_prot_inuse_add(net, rust_ip6_prot(sk), -1);
                rust_ip6_prot_inuse_add(net, addr_of_mut!(tcp_prot), 1);
                rust_ip6_set_prot(sk, addr_of_mut!(tcp_prot));
                rust_ip6_set_af_ops(sk, addr_of!(ipv4_specific));
                rust_ip6_set_ops(sk, addr_of!(inet_stream_ops));
                rust_ip6_set_family(sk, PF_INET as u16);
                tcp_sync_mss(sk, rust_ip6_pmtu_cookie(sk));
            } else {
                rust_ip6_prot_inuse_add(net, rust_ip6_prot(sk), -1);
                rust_ip6_prot_inuse_add(net, addr_of_mut!(udp_prot), 1);
                rust_ip6_set_prot(sk, addr_of_mut!(udp_prot));
                rust_ip6_set_ops(sk, addr_of!(inet_dgram_ops));
                rust_ip6_set_family(sk, PF_INET as u16);
            }
            (*np).rxopt.all = 0;
            inet6_cleanup_sock(sk);
            rust_ip6_module_put();
            0
        }
        IPV6_V6ONLY => {
            if optlen < 4 || rust_ip6_inet_num(sk) != 0 { return -(EINVAL as i32); }
            rust_ip6_set_only(sk, valbool);
            0
        }
        IPV6_RECVPKTINFO => rxopt!(set_rxinfo),
        IPV6_2292PKTINFO => rxopt!(set_rxoinfo),
        IPV6_RECVHOPLIMIT => rxopt!(set_rxhlim),
        IPV6_2292HOPLIMIT => rxopt!(set_rxohlim),
        IPV6_RECVRTHDR => rxopt!(set_srcrt),
        IPV6_2292RTHDR => rxopt!(set_osrcrt),
        IPV6_RECVHOPOPTS => rxopt!(set_hopopts),
        IPV6_2292HOPOPTS => rxopt!(set_ohopopts),
        IPV6_RECVDSTOPTS => rxopt!(set_dstopts),
        IPV6_2292DSTOPTS => rxopt!(set_odstopts),
        IPV6_RECVTCLASS => rxopt!(set_rxtclass),
        IPV6_FLOWINFO => rxopt!(set_rxflow),
        IPV6_RECVPATHMTU => rxopt!(set_rxpmtu),
        IPV6_RECVORIGDSTADDR => rxopt!(set_rxorigdstaddr),
        IPV6_TCLASS => {
            if optlen < 4 || val < -1 || val > 255 { return -(EINVAL as i32); }
            if val == -1 { val = 0; }
            if rust_ip6_sk_type(sk) == RUST_IP6_SOCK_STREAM {
                val &= !(INET_ECN_MASK as i32);
                val |= (*np).tclass as i32 & INET_ECN_MASK as i32;
            }
            if (*np).tclass as i32 != val {
                (*np).tclass = val as u8;
                rust_ip6_dst_reset(sk);
            }
            0
        }
        IPV6_TRANSPARENT => {
            if valbool && !sockopt_ns_capable(rust_ip6_user_ns(net), CAP_NET_RAW as i32)
                && !sockopt_ns_capable(rust_ip6_user_ns(net), CAP_NET_ADMIN as i32) { return -(EPERM as i32); }
            if optlen < 4 { return -(EINVAL as i32); }
            assign_bit(sk, INET_FLAGS_TRANSPARENT, valbool);
            0
        }
        IPV6_FREEBIND => {
            if optlen < 4 { return -(EINVAL as i32); }
            assign_bit(sk, INET_FLAGS_FREEBIND, valbool);
            0
        }
        IPV6_HOPOPTS | IPV6_RTHDRDSTOPTS | IPV6_RTHDR | IPV6_DSTOPTS => ipv6_set_opt_hdr(sk, optname, optval, optlen as i32),
        IPV6_PKTINFO => {
            if (optlen as usize) < size_of::<in6_pktinfo>() || rust_ip6_sockptr_null(optval) { return -(EINVAL as i32); }
            let mut pkt: in6_pktinfo = zeroed();
            if copy_from(addr_of_mut!(pkt), optval, size_of::<in6_pktinfo>()) != 0 { return -(EFAULT as i32); }
            if !rust_ip6_dev_equal(sk, pkt.ipi6_ifindex) { return -(EINVAL as i32); }
            (*np).sticky_pktinfo.ipi6_ifindex = pkt.ipi6_ifindex;
            (*np).sticky_pktinfo.ipi6_addr = pkt.ipi6_addr;
            0
        }
        IPV6_2292PKTOPTIONS => {
            let mut opt: *mut ipv6_txoptions = null_mut();
            let mut fl6: flowi6 = zeroed();
            fl6.__fl_common.flowic_oif = rust_ip6_bound_dev(sk);
            fl6.__fl_common.flowic_mark = rust_ip6_mark(sk);
            if optlen != 0 {
                if optlen > 64 * 1024 { return -(EINVAL as i32); }
                opt = rust_ip6_sock_kmalloc(sk, size_of::<ipv6_txoptions>() + optlen as usize).cast();
                if opt.is_null() { return -(ENOBUFS as i32); }
                opt.write(zeroed());
                rust_ip6_refcount_set(addr_of_mut!((*opt).refcnt), 1);
                (*opt).tot_len = (size_of::<ipv6_txoptions>() + optlen as usize) as i32;
                if copy_from(opt.add(1), optval, optlen as usize) != 0 { release_options(sk, opt); return -(EFAULT as i32); }
                let mut msg: msghdr = zeroed();
                msg.msg_controllen = optlen as _;
                msg.set_msg_control_is_user(false);
                msg.__bindgen_anon_1.msg_control = opt.add(1).cast();
                let mut ipc6: ipcm6_cookie = zeroed();
                ipc6.opt = opt;
                let ret = ip6_datagram_send_ctl(net, sk, addr_of_mut!(msg), addr_of_mut!(fl6), addr_of_mut!(ipc6));
                if ret != 0 { release_options(sk, opt); return ret; }
            }
            release_options(sk, ipv6_update_options(sk, opt));
            0
        }
        IPV6_ADD_MEMBERSHIP | IPV6_DROP_MEMBERSHIP => {
            if (optlen as usize) < size_of::<ipv6_mreq>() { return -(EINVAL as i32); }
            if test_bit(sk, INET_FLAGS_IS_ICSK) { return -(EPROTO as i32); }
            let mut mreq: ipv6_mreq = zeroed();
            if copy_from(addr_of_mut!(mreq), optval, size_of::<ipv6_mreq>()) != 0 { return -(EFAULT as i32); }
            if optname == IPV6_ADD_MEMBERSHIP { ipv6_sock_mc_join(sk, mreq.ipv6mr_ifindex, addr_of!(mreq.ipv6mr_multiaddr)) }
            else { ipv6_sock_mc_drop(sk, mreq.ipv6mr_ifindex, addr_of!(mreq.ipv6mr_multiaddr)) }
        }
        IPV6_JOIN_ANYCAST | IPV6_LEAVE_ANYCAST => {
            if (optlen as usize) < size_of::<ipv6_mreq>() { return -(EINVAL as i32); }
            let mut mreq: ipv6_mreq = zeroed();
            if copy_from(addr_of_mut!(mreq), optval, size_of::<ipv6_mreq>()) != 0 { return -(EFAULT as i32); }
            if optname == IPV6_JOIN_ANYCAST { ipv6_sock_ac_join(sk, mreq.ipv6mr_ifindex, addr_of!(mreq.ipv6mr_multiaddr)) }
            else { ipv6_sock_ac_drop(sk, mreq.ipv6mr_ifindex, addr_of!(mreq.ipv6mr_multiaddr)) }
        }
        MCAST_JOIN_GROUP | MCAST_LEAVE_GROUP => ipv6_mcast_join_leave(sk, optname, optval, optlen as i32),
        MCAST_JOIN_SOURCE_GROUP | MCAST_LEAVE_SOURCE_GROUP | MCAST_BLOCK_SOURCE | MCAST_UNBLOCK_SOURCE => do_ipv6_mcast_group_source(sk, optname, optval, optlen as i32),
        MCAST_MSFILTER => {
            if rust_ip6_compat() { compat_ipv6_set_mcast_msfilter(sk, optval, optlen as i32) }
            else { ipv6_set_mcast_msfilter(sk, optval, optlen as i32) }
        }
        IPV6_ROUTER_ALERT => {
            if optlen < 4 { return -(EINVAL as i32); }
            let ret = ip6_ra_control(sk, val);
            if ret == 0 { assign_bit(sk, INET_FLAGS_RTALERT, valbool); }
            ret
        }
        IPV6_FLOWLABEL_MGR => ipv6_flowlabel_opt(sk, optval, optlen as i32),
        IPV6_IPSEC_POLICY | IPV6_XFRM_POLICY => {
            if !sockopt_ns_capable(rust_ip6_user_ns(net), CAP_NET_ADMIN as i32) { return -(EPERM as i32); }
            rust_ip6_xfrm_policy(sk, optname as i32, optval, optlen as i32)
        }
        IPV6_RECVFRAGSIZE => { (*np).rxopt.bits.set_recvfragsize(valbool as u16); 0 }
        _ => -(ENOPROTOOPT as i32),
    }
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_setsockopt(sk: *mut sock, level: c_int, optname: c_int, optval: sockptr_t, optlen: u32) -> c_int {
    if level == SOL_IP as i32 && rust_ip6_sk_type(sk) != RUST_IP6_SOCK_RAW { return ip_setsockopt(sk, level, optname, optval, optlen); }
    if level != SOL_IPV6 as i32 { return -(ENOPROTOOPT as i32); }
    #[allow(unused_mut)]
    let mut err = do_ipv6_setsockopt(sk, level, optname, optval, optlen);
    #[cfg(CONFIG_NETFILTER)]
    if err == -(ENOPROTOOPT as i32) && optname != IPV6_IPSEC_POLICY as i32 && optname != IPV6_XFRM_POLICY as i32 {
        err = nf_setsockopt(sk, PF_INET6 as u8, optname, optval, optlen);
    }
    err
}

unsafe fn ipv6_getsockopt_sticky(opt: *mut ipv6_txoptions, optname: u32, optval: sockptr_t, len: i32) -> i32 {
    if opt.is_null() { return 0; }
    let hdr = match optname {
        IPV6_HOPOPTS => (*opt).hopopt,
        IPV6_RTHDRDSTOPTS => (*opt).dst0opt,
        IPV6_RTHDR => (*opt).srcrt.cast(),
        IPV6_DSTOPTS => (*opt).dst1opt,
        _ => return -(EINVAL as i32),
    };
    if hdr.is_null() { return 0; }
    let len = (len as u32).min(((*hdr).hdrlen as u32 + 1) << 3) as i32;
    if copy_to(optval, hdr, len as usize) != 0 { return -(EFAULT as i32); }
    len
}
unsafe fn ipv6_get_msfilter(sk: *mut sock, optval: sockptr_t, optlen: sockptr_t, mut len: i32) -> i32 {
    let size0 = RUST_IP6_GF_FLEX_OFFSET as usize;
    let mut gsf: group_filter = zeroed();
    if len < size0 as i32 { return -(EINVAL as i32); }
    if copy_from(addr_of_mut!(gsf), optval, size0) != 0 { return -(EFAULT as i32); }
    let fields = gf(addr_of_mut!(gsf));
    if family(addr_of!((*fields).gf_group)) as u32 != AF_INET6 { return -(EADDRNOTAVAIL as i32); }
    let mut num = (*fields).gf_numsrc as i32;
    rust_ip6_lock_sock(sk);
    let mut err = ip6_mc_msfget(sk, addr_of_mut!(gsf), optval, size0);
    if err == 0 {
        if num as u32 > (*fields).gf_numsrc { num = (*fields).gf_numsrc as i32; }
        len = filter_size(num as u32) as i32;
        if copy_to(optlen, addr_of!(len), size_of::<i32>()) != 0 || copy_to(optval, addr_of!(gsf), size0) != 0 { err = -(EFAULT as i32); }
    }
    rust_ip6_release_sock(sk);
    err
}
unsafe fn compat_ipv6_get_msfilter(sk: *mut sock, optval: sockptr_t, optlen: sockptr_t, mut len: i32) -> i32 {
    let size0 = RUST_IP6_COMPAT_GF_FLEX_OFFSET as usize;
    if len < size0 as i32 { return -(EINVAL as i32); }
    // The outer compat union has alignment 1; use its generated header
    // layout directly so accessing fields never strengthens that alignment.
    let mut compat: compat_group_filter__bindgen_ty_1__bindgen_ty_2 = zeroed();
    if copy_from(addr_of_mut!(compat), optval, size0) != 0 { return -(EFAULT as i32); }
    let c = addr_of_mut!(compat);
    let mut native: group_filter = zeroed();
    let f = gf(addr_of_mut!(native));
    (*f).gf_interface = (*c).gf_interface;
    (*f).gf_fmode = (*c).gf_fmode;
    (*f).gf_numsrc = (*c).gf_numsrc;
    let mut num = (*f).gf_numsrc as i32;
    (*f).gf_group = (*c).gf_group;
    if family(addr_of!((*f).gf_group)) as u32 != AF_INET6 { return -(EADDRNOTAVAIL as i32); }
    rust_ip6_lock_sock(sk);
    let err = ip6_mc_msfget(sk, addr_of_mut!(native), optval, size0);
    rust_ip6_release_sock(sk);
    if err != 0 { return err; }
    if num as u32 > (*f).gf_numsrc { num = (*f).gf_numsrc as i32; }
    len = filter_size(num as u32).wrapping_sub(size_of::<group_filter>() - size_of::<compat_group_filter>()) as i32;
    if copy_to(optlen, addr_of!(len), size_of::<i32>()) != 0
        || rust_ip6_copy_to_offset(optval, RUST_IP6_COMPAT_GF_MODE_OFFSET as usize, addr_of!((*f).gf_fmode).cast(), size_of::<u32>()) != 0
        || rust_ip6_copy_to_offset(optval, RUST_IP6_COMPAT_GF_NUMSRC_OFFSET as usize, addr_of!((*f).gf_numsrc).cast(), size_of::<u32>()) != 0 { return -(EFAULT as i32); }
    0
}

unsafe fn recv_pktoptions(sk: *mut sock, np: *mut ipv6_pinfo, optval: sockptr_t, optlen: sockptr_t, mut len: i32) -> i32 {
    if rust_ip6_sk_type(sk) != RUST_IP6_SOCK_STREAM { return -(ENOPROTOOPT as i32); }
    let mut msg: msghdr = zeroed();
    if optval.is_kernel() {
        msg.set_msg_control_is_user(false);
        msg.__bindgen_anon_1.msg_control = optval.__bindgen_anon_1.kernel;
    } else {
        msg.set_msg_control_is_user(true);
        msg.__bindgen_anon_1.msg_control_user = optval.__bindgen_anon_1.user;
    }
    msg.msg_controllen = len as _;
    msg.msg_flags = 0;
    rust_ip6_lock_sock(sk);
    let skb = (*np).pktoptions;
    if !skb.is_null() { ip6_datagram_recv_ctl(sk, addr_of_mut!(msg), skb); }
    rust_ip6_release_sock(sk);
    if skb.is_null() {
        if (*np).rxopt.bits.rxinfo() != 0 {
            let oif = read_volatile(addr_of!((*np).mcast_oif));
            let mut src: in6_pktinfo = zeroed();
            src.ipi6_ifindex = if oif != 0 { oif } else { (*np).sticky_pktinfo.ipi6_ifindex };
            src.ipi6_addr = if oif != 0 { *rust_ip6_daddr(sk) } else { (*np).sticky_pktinfo.ipi6_addr };
            put_cmsg(addr_of_mut!(msg), SOL_IPV6 as i32, IPV6_PKTINFO as i32, size_of::<in6_pktinfo>() as i32, addr_of!(src).cast_mut().cast());
        }
        if (*np).rxopt.bits.rxhlim() != 0 {
            let hlim = read_volatile(addr_of!((*np).mcast_hops)) as i32;
            put_cmsg(addr_of_mut!(msg), SOL_IPV6 as i32, IPV6_HOPLIMIT as i32, size_of::<i32>() as i32, addr_of!(hlim).cast_mut().cast());
        }
        if (*np).rxopt.bits.rxtclass() != 0 {
            let tclass = ((u32::from_be((*np).rcv_flowinfo) >> 20) & 0xff) as i32;
            put_cmsg(addr_of_mut!(msg), SOL_IPV6 as i32, IPV6_TCLASS as i32, size_of::<i32>() as i32, addr_of!(tclass).cast_mut().cast());
        }
        if (*np).rxopt.bits.rxoinfo() != 0 {
            let oif = read_volatile(addr_of!((*np).mcast_oif));
            let mut src: in6_pktinfo = zeroed();
            src.ipi6_ifindex = if oif != 0 { oif } else { (*np).sticky_pktinfo.ipi6_ifindex };
            src.ipi6_addr = if oif != 0 { *rust_ip6_daddr(sk) } else { (*np).sticky_pktinfo.ipi6_addr };
            put_cmsg(addr_of_mut!(msg), SOL_IPV6 as i32, IPV6_2292PKTINFO as i32, size_of::<in6_pktinfo>() as i32, addr_of!(src).cast_mut().cast());
        }
        if (*np).rxopt.bits.rxohlim() != 0 {
            let hlim = read_volatile(addr_of!((*np).mcast_hops)) as i32;
            put_cmsg(addr_of_mut!(msg), SOL_IPV6 as i32, IPV6_2292HOPLIMIT as i32, size_of::<i32>() as i32, addr_of!(hlim).cast_mut().cast());
        }
        if (*np).rxopt.bits.rxflow() != 0 {
            let flowinfo = (*np).rcv_flowinfo;
            put_cmsg(addr_of_mut!(msg), SOL_IPV6 as i32, IPV6_FLOWINFO as i32, size_of::<u32>() as i32, addr_of!(flowinfo).cast_mut().cast());
        }
    }
    len = (len as usize).wrapping_sub(msg.msg_controllen as usize) as i32;
    copy_to(optlen, addr_of!(len), size_of::<i32>())
}

#[no_mangle]
pub unsafe extern "C" fn do_ipv6_getsockopt(sk: *mut sock, _level: c_int, optname: c_int, optval: sockptr_t, optlen: sockptr_t) -> c_int {
    let np = rust_ip6_np(sk);
    if rust_ip6_mroute_opt(optname) { return rust_ip6_mroute_get(sk, optname, optval, optlen); }
    let mut len: i32 = 0;
    if copy_from(addr_of_mut!(len), optlen, size_of::<i32>()) != 0 { return -(EFAULT as i32); }
    let val: i32 = match optname as u32 {
        IPV6_ADDRFORM => {
            if rust_ip6_sk_protocol(sk) != IPPROTO_UDP && rust_ip6_sk_protocol(sk) != IPPROTO_TCP { return -(ENOPROTOOPT as i32); }
            if rust_ip6_sk_state(sk) != TCP_ESTABLISHED { return -(ENOTCONN as i32); }
            rust_ip6_sk_family(sk) as i32
        }
        MCAST_MSFILTER => {
            return if rust_ip6_compat() { compat_ipv6_get_msfilter(sk, optval, optlen, len) } else { ipv6_get_msfilter(sk, optval, optlen, len) };
        }
        IPV6_2292PKTOPTIONS => return recv_pktoptions(sk, np, optval, optlen, len),
        IPV6_MTU => {
            let mut val = 0;
            rust_ip6_rcu_lock();
            let dst = rust_ip6_dst_get(sk);
            if !dst.is_null() { val = rust_ip6_dst_mtu(dst) as i32; }
            rust_ip6_rcu_unlock();
            if val == 0 { return -(ENOTCONN as i32); }
            val
        }
        IPV6_V6ONLY => rust_ip6_only(sk) as i32,
        IPV6_RECVPKTINFO => (*np).rxopt.bits.rxinfo() as i32,
        IPV6_2292PKTINFO => (*np).rxopt.bits.rxoinfo() as i32,
        IPV6_RECVHOPLIMIT => (*np).rxopt.bits.rxhlim() as i32,
        IPV6_2292HOPLIMIT => (*np).rxopt.bits.rxohlim() as i32,
        IPV6_RECVRTHDR => (*np).rxopt.bits.srcrt() as i32,
        IPV6_2292RTHDR => (*np).rxopt.bits.osrcrt() as i32,
        IPV6_HOPOPTS | IPV6_RTHDRDSTOPTS | IPV6_RTHDR | IPV6_DSTOPTS => {
            rust_ip6_lock_sock(sk);
            len = ipv6_getsockopt_sticky(rust_ip6_opt_get_locked(sk), optname as u32, optval, len);
            rust_ip6_release_sock(sk);
            if len < 0 { return len; }
            return copy_to(optlen, addr_of!(len), size_of::<i32>());
        }
        IPV6_RECVHOPOPTS => (*np).rxopt.bits.hopopts() as i32,
        IPV6_2292HOPOPTS => (*np).rxopt.bits.ohopopts() as i32,
        IPV6_RECVDSTOPTS => (*np).rxopt.bits.dstopts() as i32,
        IPV6_2292DSTOPTS => (*np).rxopt.bits.odstopts() as i32,
        IPV6_TCLASS => (*np).tclass as i32,
        IPV6_RECVTCLASS => (*np).rxopt.bits.rxtclass() as i32,
        IPV6_FLOWINFO => (*np).rxopt.bits.rxflow() as i32,
        IPV6_RECVPATHMTU => (*np).rxopt.bits.rxpmtu() as i32,
        IPV6_PATHMTU => {
            if (len as usize) < size_of::<ip6_mtuinfo>() { return -(EINVAL as i32); }
            len = size_of::<ip6_mtuinfo>() as i32;
            let mut mtuinfo: ip6_mtuinfo = zeroed();
            rust_ip6_rcu_lock();
            let dst = rust_ip6_dst_get(sk);
            if !dst.is_null() { mtuinfo.ip6m_mtu = rust_ip6_dst_mtu(dst); }
            rust_ip6_rcu_unlock();
            if mtuinfo.ip6m_mtu == 0 { return -(ENOTCONN as i32); }
            if copy_to(optlen, addr_of!(len), size_of::<i32>()) != 0 || copy_to(optval, addr_of!(mtuinfo), len as usize) != 0 { return -(EFAULT as i32); }
            return 0;
        }
        IPV6_TRANSPARENT => test_bit(sk, INET_FLAGS_TRANSPARENT) as i32,
        IPV6_FREEBIND => test_bit(sk, INET_FLAGS_FREEBIND) as i32,
        IPV6_RECVORIGDSTADDR => (*np).rxopt.bits.rxorigdstaddr() as i32,
        IPV6_UNICAST_HOPS | IPV6_MULTICAST_HOPS => {
            let mut val = if optname as u32 == IPV6_UNICAST_HOPS { read_volatile(addr_of!((*np).hop_limit)) as i32 } else { read_volatile(addr_of!((*np).mcast_hops)) as i32 };
            if val < 0 {
                rust_ip6_rcu_lock();
                let dst = rust_ip6_dst_get(sk);
                if !dst.is_null() { val = ip6_dst_hoplimit(dst); }
                rust_ip6_rcu_unlock();
            }
            if val < 0 { val = rust_ip6_default_hoplimit(rust_ip6_net(sk)); }
            val
        }
        IPV6_MULTICAST_LOOP => test_bit(sk, INET_FLAGS_MC6_LOOP) as i32,
        IPV6_MULTICAST_IF => read_volatile(addr_of!((*np).mcast_oif)),
        IPV6_MULTICAST_ALL => test_bit(sk, INET_FLAGS_MC6_ALL) as i32,
        IPV6_UNICAST_IF => (read_volatile(addr_of!((*np).ucast_oif)) as u32).to_be() as i32,
        IPV6_MTU_DISCOVER => read_volatile(addr_of!((*np).pmtudisc)) as i32,
        IPV6_RECVERR => test_bit(sk, INET_FLAGS_RECVERR6) as i32,
        IPV6_FLOWINFO_SEND => test_bit(sk, INET_FLAGS_SNDFLOW) as i32,
        IPV6_FLOWLABEL_MGR => {
            if (len as usize) < size_of::<in6_flowlabel_req>() { return -(EINVAL as i32); }
            let mut freq: in6_flowlabel_req = zeroed();
            if copy_from(addr_of_mut!(freq), optval, size_of::<in6_flowlabel_req>()) != 0 { return -(EFAULT as i32); }
            if freq.flr_action as u32 != IPV6_FL_A_GET { return -(EINVAL as i32); }
            len = size_of::<in6_flowlabel_req>() as i32;
            let flags = freq.flr_flags as i32;
            freq = zeroed();
            let ret = ipv6_flowlabel_opt_get(sk, addr_of_mut!(freq), flags);
            if ret < 0 { return ret; }
            if copy_to(optlen, addr_of!(len), size_of::<i32>()) != 0 || copy_to(optval, addr_of!(freq), len as usize) != 0 { return -(EFAULT as i32); }
            return 0;
        }
        IPV6_ADDR_PREFERENCES => {
            let srcprefs = read_volatile(addr_of!((*np).srcprefs)) as u32;
            let mut val = if srcprefs & IPV6_PREFER_SRC_TMP != 0 { IPV6_PREFER_SRC_TMP }
                else if srcprefs & IPV6_PREFER_SRC_PUBLIC != 0 { IPV6_PREFER_SRC_PUBLIC }
                else { IPV6_PREFER_SRC_PUBTMP_DEFAULT };
            val |= if srcprefs & IPV6_PREFER_SRC_COA != 0 { IPV6_PREFER_SRC_COA } else { IPV6_PREFER_SRC_HOME };
            val as i32
        }
        IPV6_MINHOPCOUNT => read_volatile(addr_of!((*np).min_hopcount)) as i32,
        IPV6_DONTFRAG => test_bit(sk, INET_FLAGS_DONTFRAG) as i32,
        IPV6_AUTOFLOWLABEL => rust_ip6_autoflowlabel(rust_ip6_net(sk), sk) as i32,
        IPV6_RECVFRAGSIZE => (*np).rxopt.bits.recvfragsize() as i32,
        IPV6_ROUTER_ALERT => test_bit(sk, INET_FLAGS_RTALERT) as i32,
        IPV6_ROUTER_ALERT_ISOLATE => test_bit(sk, INET_FLAGS_RTALERT_ISOLATE) as i32,
        IPV6_RECVERR_RFC4884 => test_bit(sk, INET_FLAGS_RECVERR6_RFC4884) as i32,
        _ => return -(ENOPROTOOPT as i32),
    };
    len = (size_of::<i32>() as u32).min(len as u32) as i32;
    if copy_to(optlen, addr_of!(len), size_of::<i32>()) != 0 || copy_to(optval, addr_of!(val), len as usize) != 0 { return -(EFAULT as i32); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_getsockopt(sk: *mut sock, level: c_int, optname: c_int, optval: *mut c_char, optlen: *mut c_int) -> c_int {
    if level == SOL_IP as i32 && rust_ip6_sk_type(sk) != RUST_IP6_SOCK_RAW { return ip_getsockopt(sk, level, optname, optval, optlen); }
    if level != SOL_IPV6 as i32 { return -(ENOPROTOOPT as i32); }
    #[allow(unused_mut)]
    let mut err = do_ipv6_getsockopt(sk, level, optname, rust_ip6_user_sockptr(optval.cast()), rust_ip6_user_sockptr(optlen.cast()));
    #[cfg(CONFIG_NETFILTER)]
    if err == -(ENOPROTOOPT as i32) && optname != IPV6_2292PKTOPTIONS as i32 {
        let mut len: i32 = 0;
        if rust_ip6_get_user_int(optlen, addr_of_mut!(len)) != 0 { return -(EFAULT as i32); }
        err = nf_getsockopt(sk, PF_INET6 as u8, optname, optval, addr_of_mut!(len));
        if err >= 0 { err = rust_ip6_put_user_int(optlen, len); }
    }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
