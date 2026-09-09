// SPDX-License-Identifier: GPL-2.0-only
/*
 * TTL modification target for IP tables
 * (C) 2000,2005 by Harald Welte <laforge@netfilter.org>
 *
 * Hop Limit modification target for ip6tables
 * Maciej Soltysiak <solt@dns.toxicfilms.tv>
 */

// Dependencies supplied by the Linux kernel and netfilter headers.

#[allow(non_camel_case_types)]
unsafe fn ttl_tg(skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let mut iph: *mut iphdr;
    let info: *const ipt_TTL_info = unsafe { (*par).targinfo as *const ipt_TTL_info };
    let mut new_ttl: c_int;

    if unsafe { skb_ensure_writable(skb, core::mem::size_of::<iphdr>()) } != 0 {
        return NF_DROP;
    }

    iph = unsafe { ip_hdr(skb) };

    new_ttl = match unsafe { (*info).mode } {
        IPT_TTL_SET => unsafe { (*info).ttl as c_int },
        IPT_TTL_INC => {
            let mut value = unsafe { (*iph).ttl as c_int + (*info).ttl as c_int };
            if value > 255 { value = 255; }
            value
        }
        IPT_TTL_DEC => {
            let mut value = unsafe { (*iph).ttl as c_int - (*info).ttl as c_int };
            if value < 0 { value = 0; }
            value
        }
        _ => unsafe { (*iph).ttl as c_int },
    };

    if new_ttl != unsafe { (*iph).ttl as c_int } {
        unsafe {
            csum_replace2(
                &mut (*iph).check,
                htons(((*iph).ttl as c_int) << 8),
                htons(new_ttl << 8),
            );
            (*iph).ttl = new_ttl as u8;
        }
    }

    XT_CONTINUE
}

unsafe fn hl_tg6(skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let ip6h: *mut ipv6hdr;
    let info: *const ip6t_HL_info = unsafe { (*par).targinfo as *const ip6t_HL_info };
    let mut new_hl: c_int;

    if unsafe { skb_ensure_writable(skb, core::mem::size_of::<ipv6hdr>()) } != 0 {
        return NF_DROP;
    }

    ip6h = unsafe { ipv6_hdr(skb) };

    new_hl = match unsafe { (*info).mode } {
        IP6T_HL_SET => unsafe { (*info).hop_limit as c_int },
        IP6T_HL_INC => {
            let mut value = unsafe { (*ip6h).hop_limit as c_int + (*info).hop_limit as c_int };
            if value > 255 { value = 255; }
            value
        }
        IP6T_HL_DEC => {
            let mut value = unsafe { (*ip6h).hop_limit as c_int - (*info).hop_limit as c_int };
            if value < 0 { value = 0; }
            value
        }
        _ => unsafe { (*ip6h).hop_limit as c_int },
    };

    unsafe { (*ip6h).hop_limit = new_hl as u8; }
    XT_CONTINUE
}

unsafe fn ttl_tg_check(par: *const xt_tgchk_param) -> c_int {
    let info: *const ipt_TTL_info = unsafe { (*par).targinfo as *const ipt_TTL_info };
    if unsafe { (*info).mode } > IPT_TTL_MAXMODE { return -EINVAL; }
    if unsafe { (*info).mode } != IPT_TTL_SET && unsafe { (*info).ttl } == 0 { return -EINVAL; }
    0
}

unsafe fn hl_tg6_check(par: *const xt_tgchk_param) -> c_int {
    let info: *const ip6t_HL_info = unsafe { (*par).targinfo as *const ip6t_HL_info };
    if unsafe { (*info).mode } > IP6T_HL_MAXMODE { return -EINVAL; }
    if unsafe { (*info).mode } != IP6T_HL_SET && unsafe { (*info).hop_limit } == 0 { return -EINVAL; }
    0
}

static mut hl_tg_reg: [xt_target; 2] = [
    xt_target {
        name: *b"TTL\0",
        revision: 0,
        family: NFPROTO_IPV4,
        target: Some(ttl_tg),
        targetsize: core::mem::size_of::<ipt_TTL_info>(),
        table: *b"mangle\0",
        checkentry: Some(ttl_tg_check),
        me: THIS_MODULE,
    },
    xt_target {
        name: *b"HL\0",
        revision: 0,
        family: NFPROTO_IPV6,
        target: Some(hl_tg6),
        targetsize: core::mem::size_of::<ip6t_HL_info>(),
        table: *b"mangle\0",
        checkentry: Some(hl_tg6_check),
        me: THIS_MODULE,
    },
];

unsafe fn hl_tg_init() -> c_int {
    xt_register_targets(hl_tg_reg.as_mut_ptr(), hl_tg_reg.len())
}

unsafe fn hl_tg_exit() {
    xt_unregister_targets(hl_tg_reg.as_mut_ptr(), hl_tg_reg.len());
}

// module_init(hl_tg_init);
// module_exit(hl_tg_exit);
// MODULE_ALIAS("ipt_TTL");
// MODULE_ALIAS("ip6t_HL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
