// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebt_dnat
 *
 *	Authors:
 *	Bart De Schuymer <bdschuym@pandora.be>
 *
 *  June, 2002
 *
 */

// External kernel declarations supplied by the surrounding translation unit.

unsafe fn ebt_dnat_tg(skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let info = (*par).targinfo as *const ebt_nat_info;

    if skb_ensure_writable(skb, 0) != 0 {
        return EBT_DROP;
    }

    ether_addr_copy((*eth_hdr(skb)).h_dest.as_mut_ptr(), (*info).mac.as_ptr());

    if is_multicast_ether_addr((*info).mac.as_ptr()) {
        if is_broadcast_ether_addr((*info).mac.as_ptr()) {
            (*skb).pkt_type = PACKET_BROADCAST;
        } else {
            (*skb).pkt_type = PACKET_MULTICAST;
        }
    } else {
        let dev: *const net_device;

        match xt_hooknum(par) {
            NF_BR_BROUTING => {
                dev = xt_in(par);
            }
            NF_BR_PRE_ROUTING => {
                dev = netdev_master_upper_dev_get_rcu(xt_in(par));
                if dev.is_null() { /* bridge port removed? */
                    return EBT_DROP;
                }
            }
            _ => {
                dev = core::ptr::null();
            }
        }

        if dev.is_null() { /* NF_BR_LOCAL_OUT */
            return (*info).target;
        }

        if ether_addr_equal((*info).mac.as_ptr(), (*dev).dev_addr.as_ptr()) {
            (*skb).pkt_type = PACKET_HOST;
        } else {
            (*skb).pkt_type = PACKET_OTHERHOST;
        }
    }

    (*info).target
}

unsafe fn ebt_dnat_tg_check(par: *const xt_tgchk_param) -> c_int {
    let info = (*par).targinfo as *const ebt_nat_info;
    let hook_mask: c_uint;

    if BASE_CHAIN != 0 && (*info).target == EBT_RETURN {
        return -EINVAL;
    }

    hook_mask = (*par).hook_mask & !(1 << NF_BR_NUMHOOKS);
    if ((strcmp((*par).table, b"nat\0".as_ptr()) != 0
        || (hook_mask & !((1 << NF_BR_PRE_ROUTING) | (1 << NF_BR_LOCAL_OUT))) != 0)
        && (strcmp((*par).table, b"broute\0".as_ptr()) != 0
            || (hook_mask & !(1 << NF_BR_BROUTING)) != 0))
    {
        return -EINVAL;
    }
    if ebt_invalid_target((*info).target) != 0 {
        return -EINVAL;
    }
    0
}

static mut ebt_dnat_tg_reg: xt_target = xt_target {
    name: b"dnat\0".as_ptr(),
    revision: 0,
    family: NFPROTO_BRIDGE,
    hooks: (1 << NF_BR_NUMHOOKS) | (1 << NF_BR_PRE_ROUTING)
        | (1 << NF_BR_LOCAL_OUT) | (1 << NF_BR_BROUTING),
    target: Some(ebt_dnat_tg),
    checkentry: Some(ebt_dnat_tg_check),
    targetsize: core::mem::size_of::<ebt_nat_info>(),
    me: THIS_MODULE,
};

unsafe fn ebt_dnat_init() -> c_int {
    xt_register_target(&raw mut ebt_dnat_tg_reg)
}

unsafe fn ebt_dnat_fini() {
    xt_unregister_target(&raw mut ebt_dnat_tg_reg);
}

module_init!(ebt_dnat_init);
module_exit!(ebt_dnat_fini);
module_description!("Ebtables: Destination MAC address translation");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
