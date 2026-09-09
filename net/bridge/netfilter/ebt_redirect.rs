// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebt_redirect
 *
 *	Authors:
 *	Bart De Schuymer <bdschuym@pandora.be>
 *
 *  April, 2002
 *
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn ebt_redirect_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info = (*par).targinfo as *const ebt_redirect_info;

    if skb_ensure_writable(skb, 0) != 0 {
        return EBT_DROP;
    }

    if xt_hooknum(par) != NF_BR_BROUTING {
        let dev: *const net_device;

        dev = netdev_master_upper_dev_get_rcu(xt_in(par));
        if dev.is_null() {
            return EBT_DROP;
        }

        ether_addr_copy((*eth_hdr(skb)).h_dest.as_mut_ptr(), (*dev).dev_addr.as_ptr());
    } else {
        ether_addr_copy(
            (*eth_hdr(skb)).h_dest.as_mut_ptr(),
            (*xt_in(par)).dev_addr.as_ptr(),
        );
    }

    (*skb).pkt_type = PACKET_HOST;
    (*info).target
}

unsafe fn ebt_redirect_tg_check(par: *const xt_tgchk_param) -> c_int {
    let info = (*par).targinfo as *const ebt_redirect_info;
    let hook_mask: c_uint;

    // BASE_CHAIN is a build-time kernel condition preserved from the source.
    if BASE_CHAIN && (*info).target == EBT_RETURN {
        return -EINVAL;
    }

    hook_mask = (*par).hook_mask & !(1 << NF_BR_NUMHOOKS);
    if ((strcmp((*par).table, b"nat\0".as_ptr() as *const c_char) != 0
        || hook_mask & !(1 << NF_BR_PRE_ROUTING) != 0)
        && (strcmp((*par).table, b"broute\0".as_ptr() as *const c_char) != 0
            || hook_mask & !(1 << NF_BR_BROUTING) != 0))
    {
        return -EINVAL;
    }
    if ebt_invalid_target((*info).target) != 0 {
        return -EINVAL;
    }
    0
}

static mut ebt_redirect_tg_reg: xt_target = xt_target {
    name: b"redirect\0".as_ptr() as *const c_char,
    revision: 0,
    family: NFPROTO_BRIDGE,
    hooks: (1 << NF_BR_NUMHOOKS) | (1 << NF_BR_PRE_ROUTING) | (1 << NF_BR_BROUTING),
    target: Some(ebt_redirect_tg),
    checkentry: Some(ebt_redirect_tg_check),
    targetsize: core::mem::size_of::<ebt_redirect_info>(),
    me: THIS_MODULE,
};

unsafe fn ebt_redirect_init() -> c_int {
    xt_register_target(&mut ebt_redirect_tg_reg)
}

unsafe fn ebt_redirect_fini() {
    xt_unregister_target(&mut ebt_redirect_tg_reg);
}

// module_init(ebt_redirect_init);
// module_exit(ebt_redirect_fini);
// MODULE_DESCRIPTION("Ebtables: Packet redirection to localhost");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
