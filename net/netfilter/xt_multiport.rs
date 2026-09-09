// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match one of a list of TCP/UDP(-Lite)/SCTP/DCCP ports:
   ports are in the same place so we can treat them as equal. */

/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
 */
// C dependencies: linux/module.h, linux/types.h, linux/udp.h,
// linux/skbuff.h, linux/in.h, linux/netfilter/xt_multiport.h,
// linux/netfilter/x_tables.h, linux/netfilter_ipv4/ip_tables.h,
// linux/netfilter_ipv6/ip6_tables.h

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Netfilter Core Team <coreteam@netfilter.org>");
// MODULE_DESCRIPTION("Xtables: multiple port matching for TCP, UDP, UDP-Lite, SCTP and DCCP");
// MODULE_ALIAS("ipt_multiport");
// MODULE_ALIAS("ip6t_multiport");

/* Returns 1 if the port is matched by the test, 0 otherwise. */
unsafe fn ports_match_v1(
    minfo: *const xt_multiport_v1,
    src: u16,
    dst: u16,
) -> bool {
    let mut i: u32 = 0;
    let mut s: u16;
    let mut e: u16;

    while i < (*minfo).count as u32 {
        s = (*minfo).ports[i as usize];

        if (*minfo).pflags[i as usize] != 0 {
            /* range port matching */
            i = i.wrapping_add(1);
            e = (*minfo).ports[i as usize];

            match (*minfo).flags {
                XT_MULTIPORT_SOURCE => {
                    if src >= s && src <= e {
                        return true ^ ((*minfo).invert != 0);
                    }
                }
                XT_MULTIPORT_DESTINATION => {
                    if dst >= s && dst <= e {
                        return true ^ ((*minfo).invert != 0);
                    }
                }
                XT_MULTIPORT_EITHER => {
                    if (dst >= s && dst <= e) || (src >= s && src <= e) {
                        return true ^ ((*minfo).invert != 0);
                    }
                }
                _ => {}
            }
        } else {
            /* exact port matching */
            match (*minfo).flags {
                XT_MULTIPORT_SOURCE => {
                    if src == s {
                        return true ^ ((*minfo).invert != 0);
                    }
                }
                XT_MULTIPORT_DESTINATION => {
                    if dst == s {
                        return true ^ ((*minfo).invert != 0);
                    }
                }
                XT_MULTIPORT_EITHER => {
                    if src == s || dst == s {
                        return true ^ ((*minfo).invert != 0);
                    }
                }
                _ => {}
            }
        }
        i = i.wrapping_add(1);
    }

    (*minfo).invert != 0
}

unsafe fn multiport_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let mut _ports: [__be16; 2] = [0; 2];
    let multiinfo: *const xt_multiport_v1 = (*par).matchinfo as *const xt_multiport_v1;

    if (*par).fragoff != 0 {
        return false;
    }

    let pptr: *const __be16 = skb_header_pointer(
        skb,
        (*par).thoff,
        core::mem::size_of_val(&_ports),
        _ports.as_mut_ptr() as *mut core::ffi::c_void,
    );
    if pptr.is_null() {
        /* We've been asked to examine this packet, and we
         * can't.  Hence, no choice but to drop.
         */
        (*par).hotdrop = true;
        return false;
    }

    ports_match_v1(multiinfo, ntohs(*pptr), ntohs(*pptr.add(1)))
}

unsafe fn multiport_valid_ranges(multiinfo: *const xt_multiport_v1) -> bool {
    let mut i: u32 = 0;

    while i < (*multiinfo).count as u32 {
        if (*multiinfo).pflags[i as usize] == 0 {
            i = i.wrapping_add(1);
            continue;
        }

        i = i.wrapping_add(1);
        if i >= (*multiinfo).count as u32 {
            return false;
        }

        if (*multiinfo).pflags[i as usize] != 0 {
            return false;
        }

        if (*multiinfo).ports[(i - 1) as usize] > (*multiinfo).ports[i as usize] {
            return false;
        }
        i = i.wrapping_add(1);
    }

    true
}

unsafe fn check(
    proto: u16,
    ip_invflags: u8,
    match_flags: u8,
    count: u8,
) -> bool {
    /* Must specify supported protocol, no unknown flags or bad count */
    (proto == IPPROTO_TCP
        || proto == IPPROTO_UDP
        || proto == IPPROTO_UDPLITE
        || proto == IPPROTO_SCTP
        || proto == IPPROTO_DCCP)
        && (ip_invflags & XT_INV_PROTO) == 0
        && (match_flags == XT_MULTIPORT_SOURCE
            || match_flags == XT_MULTIPORT_DESTINATION
            || match_flags == XT_MULTIPORT_EITHER)
        && count <= XT_MULTI_PORTS
}

unsafe extern "C" fn multiport_mt_check(par: *const xt_mtchk_param) -> i32 {
    let ip: *const ipt_ip = (*par).entryinfo as *const ipt_ip;
    let multiinfo: *const xt_multiport_v1 = (*par).matchinfo as *const xt_multiport_v1;

    if !check((*ip).proto, (*ip).invflags, (*multiinfo).flags, (*multiinfo).count) {
        return -EINVAL;
    }

    if multiport_valid_ranges(multiinfo) { 0 } else { -EINVAL }
}

unsafe extern "C" fn multiport_mt6_check(par: *const xt_mtchk_param) -> i32 {
    let ip: *const ip6t_ip6 = (*par).entryinfo as *const ip6t_ip6;
    let multiinfo: *const xt_multiport_v1 = (*par).matchinfo as *const xt_multiport_v1;

    if !check((*ip).proto, (*ip).invflags, (*multiinfo).flags, (*multiinfo).count) {
        return -EINVAL;
    }

    if multiport_valid_ranges(multiinfo) { 0 } else { -EINVAL }
}

static mut multiport_mt_reg: [xt_match; 2] = [
    xt_match {
        name: b"multiport\0".as_ptr() as *const i8,
        family: NFPROTO_IPV4,
        revision: 1,
        checkentry: Some(multiport_mt_check),
        r#match: Some(multiport_mt),
        matchsize: core::mem::size_of::<xt_multiport_v1>(),
        me: THIS_MODULE,
    },
    xt_match {
        name: b"multiport\0".as_ptr() as *const i8,
        family: NFPROTO_IPV6,
        revision: 1,
        checkentry: Some(multiport_mt6_check),
        r#match: Some(multiport_mt),
        matchsize: core::mem::size_of::<xt_multiport_v1>(),
        me: THIS_MODULE,
    },
];

unsafe extern "C" fn multiport_mt_init() -> i32 {
    xt_register_matches(multiport_mt_reg.as_mut_ptr(), multiport_mt_reg.len())
}

unsafe extern "C" fn multiport_mt_exit() {
    xt_unregister_matches(multiport_mt_reg.as_mut_ptr(), multiport_mt_reg.len());
}

// module_init(multiport_mt_init);
// module_exit(multiport_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
