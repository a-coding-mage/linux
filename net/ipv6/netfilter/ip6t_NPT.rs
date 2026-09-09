// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2011, 2012 Patrick McHardy <kaber@trash.net>
 */

// Dependencies supplied by the kernel networking and netfilter headers are
// intentionally referenced here rather than reimplemented.

unsafe fn ip6t_npt_checkentry(par: *const xt_tgchk_param) -> i32 {
    let npt = (*par).targinfo as *mut ip6t_npt_tginfo;
    let mut pfx: in6_addr = core::mem::zeroed();
    let mut src_sum: __wsum;
    let mut dst_sum: __wsum;

    if (*npt).src_pfx_len > 64 || (*npt).dst_pfx_len > 64 {
        return -EINVAL;
    }

    /* Ensure that LSB of prefix is zero */
    ipv6_addr_prefix(&mut pfx, &(*npt).src_pfx.in6, (*npt).src_pfx_len);
    if !ipv6_addr_equal(&pfx, &(*npt).src_pfx.in6) {
        return -EINVAL;
    }
    ipv6_addr_prefix(&mut pfx, &(*npt).dst_pfx.in6, (*npt).dst_pfx_len);
    if !ipv6_addr_equal(&pfx, &(*npt).dst_pfx.in6) {
        return -EINVAL;
    }

    src_sum = csum_partial(
        &(*npt).src_pfx.in6 as *const in6_addr as *const core::ffi::c_void,
        core::mem::size_of::<in6_addr>(),
        0,
    );
    dst_sum = csum_partial(
        &(*npt).dst_pfx.in6 as *const in6_addr as *const core::ffi::c_void,
        core::mem::size_of::<in6_addr>(),
        0,
    );

    (*npt).adjustment = !csum_fold(csum_sub(src_sum, dst_sum));
    0
}

unsafe fn ip6t_npt_map_pfx(npt: *const ip6t_npt_tginfo, addr: *mut in6_addr) -> bool {
    let pfx_len = core::cmp::max((*npt).src_pfx_len, (*npt).dst_pfx_len);
    let mut i = 0;
    let mut mask: __be32;
    let mut idx: usize;
    let mut sum: __sum16;

    while i < pfx_len {
        if pfx_len - i >= 32 {
            mask = 0;
        } else {
            mask = htonl((1u32 << (i - pfx_len + 32)) - 1);
        }
        idx = (i / 32) as usize;
        (*addr).s6_addr32[idx] &= mask;
        (*addr).s6_addr32[idx] |= !mask & (*npt).dst_pfx.in6.s6_addr32[idx];
        i += 32;
    }

    if pfx_len <= 48 {
        idx = 3;
    } else {
        idx = 4;
        while idx < (*addr).s6_addr16.len() {
            if (*addr).s6_addr16[idx] as __sum16 != CSUM_MANGLED_0 {
                break;
            }
            idx += 1;
        }
        if idx == (*addr).s6_addr16.len() {
            return false;
        }
    }

    sum = !csum_fold(csum_add(
        csum_unfold((*addr).s6_addr16[idx] as __sum16),
        csum_unfold((*npt).adjustment),
    ));
    if sum == CSUM_MANGLED_0 {
        sum = 0;
    }
    (*addr).s6_addr16[idx] = sum as _;
    true
}

unsafe fn icmpv6_bounced_ipv6hdr(
    skb: *mut sk_buff,
    bounced_hdr: *mut ipv6hdr,
) -> *mut ipv6hdr {
    if (*ipv6_hdr(skb)).nexthdr != IPPROTO_ICMPV6 {
        return core::ptr::null_mut();
    }
    if !icmpv6_is_err((*icmp6_hdr(skb)).icmp6_type) {
        return core::ptr::null_mut();
    }
    skb_header_pointer(
        skb,
        skb_transport_offset(skb) + core::mem::size_of::<icmp6hdr>(),
        core::mem::size_of::<ipv6hdr>(),
        bounced_hdr as *mut core::ffi::c_void,
    ) as *mut ipv6hdr
}

unsafe fn ip6t_snpt_tg(skb: *mut sk_buff, par: *const xt_action_param) -> u32 {
    let npt = (*par).targinfo as *const ip6t_npt_tginfo;
    let mut bounced_hdr_storage: ipv6hdr = core::mem::zeroed();
    let mut bounced_pfx: in6_addr = core::mem::zeroed();
    if !ip6t_npt_map_pfx(npt, &mut (*ipv6_hdr(skb)).saddr) {
        icmpv6_send(skb, ICMPV6_PARAMPROB, ICMPV6_HDR_FIELD, core::mem::offset_of!(ipv6hdr, saddr));
        return NF_DROP;
    }
    /* rewrite dst addr of bounced packet which was sent to dst range */
    let bounced_hdr = icmpv6_bounced_ipv6hdr(skb, &mut bounced_hdr_storage);
    if !bounced_hdr.is_null() {
        ipv6_addr_prefix(&mut bounced_pfx, &(*bounced_hdr).daddr, (*npt).src_pfx_len);
        if ipv6_addr_cmp(&bounced_pfx, &(*npt).src_pfx.in6) == 0 {
            ip6t_npt_map_pfx(npt, &mut (*bounced_hdr).daddr);
        }
    }
    XT_CONTINUE
}

unsafe fn ip6t_dnpt_tg(skb: *mut sk_buff, par: *const xt_action_param) -> u32 {
    let npt = (*par).targinfo as *const ip6t_npt_tginfo;
    let mut bounced_hdr_storage: ipv6hdr = core::mem::zeroed();
    let mut bounced_pfx: in6_addr = core::mem::zeroed();
    if !ip6t_npt_map_pfx(npt, &mut (*ipv6_hdr(skb)).daddr) {
        icmpv6_send(skb, ICMPV6_PARAMPROB, ICMPV6_HDR_FIELD, core::mem::offset_of!(ipv6hdr, daddr));
        return NF_DROP;
    }
    /* rewrite src addr of bounced packet which was sent from dst range */
    let bounced_hdr = icmpv6_bounced_ipv6hdr(skb, &mut bounced_hdr_storage);
    if !bounced_hdr.is_null() {
        ipv6_addr_prefix(&mut bounced_pfx, &(*bounced_hdr).saddr, (*npt).src_pfx_len);
        if ipv6_addr_cmp(&bounced_pfx, &(*npt).src_pfx.in6) == 0 {
            ip6t_npt_map_pfx(npt, &mut (*bounced_hdr).saddr);
        }
    }
    XT_CONTINUE
}

// The target registration table and module metadata mirror the C translation;
// kernel-provided target-registration types and constants are external.
static mut ip6t_npt_target_reg: [xt_target; 2] = [
    xt_target { name: c"SNPT", table: c"mangle", target: ip6t_snpt_tg, targetsize: core::mem::size_of::<ip6t_npt_tginfo>(), usersize: core::mem::offset_of!(ip6t_npt_tginfo, adjustment), checkentry: ip6t_npt_checkentry, family: NFPROTO_IPV6, hooks: (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_POST_ROUTING), me: THIS_MODULE },
    xt_target { name: c"DNPT", table: c"mangle", target: ip6t_dnpt_tg, targetsize: core::mem::size_of::<ip6t_npt_tginfo>(), usersize: core::mem::offset_of!(ip6t_npt_tginfo, adjustment), checkentry: ip6t_npt_checkentry, family: NFPROTO_IPV6, hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_OUT), me: THIS_MODULE },
];

unsafe fn ip6t_npt_init() -> i32 {
    xt_register_targets(ip6t_npt_target_reg.as_mut_ptr(), ip6t_npt_target_reg.len())
}

unsafe fn ip6t_npt_exit() {
    xt_unregister_targets(ip6t_npt_target_reg.as_mut_ptr(), ip6t_npt_target_reg.len());
}

// module_init(ip6t_npt_init);
// module_exit(ip6t_npt_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("IPv6-to-IPv6 Network Prefix Translation (RFC 6296)");
// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_ALIAS("ip6t_SNPT");
// MODULE_ALIAS("ip6t_DNPT");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
