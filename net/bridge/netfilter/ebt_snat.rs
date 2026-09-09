// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebt_snat
 *
 *	Authors:
 *	Bart De Schuymer <bdschuym@pandora.be>
 *
 *  June, 2002
 *
 */
// Dependencies supplied by the kernel and other translation units:
// linux/module.h, net/sock.h, linux/if_arp.h, net/arp.h,
// linux/netfilter.h, linux/netfilter/x_tables.h,
// linux/netfilter_bridge/ebtables.h, linux/netfilter_bridge/ebt_nat.h

static unsafe fn ebt_snat_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info: *const ebt_nat_info = unsafe { (*par).targinfo as *const ebt_nat_info };

    if unsafe { skb_ensure_writable(skb, 0) } != 0 {
        return EBT_DROP;
    }

    unsafe {
        ether_addr_copy((*eth_hdr(skb)).h_source.as_mut_ptr(), (*info).mac.as_ptr());
    }
    if unsafe { ((*info).target & NAT_ARP_BIT) == 0 }
        && unsafe { (*eth_hdr(skb)).h_proto == htons(ETH_P_ARP) }
    {
        let ap: *const arphdr;
        let mut _ah: arphdr = unsafe { core::mem::zeroed() };

        if unsafe { skb_ensure_writable(skb, core::mem::size_of::<arphdr>() + ETH_ALEN) } != 0 {
            return EBT_DROP;
        }

        ap = unsafe {
            skb_header_pointer(
                skb,
                0,
                core::mem::size_of::<arphdr>(),
                &mut _ah as *mut arphdr as *mut c_void,
            )
        } as *const arphdr;
        if ap.is_null() {
            return EBT_DROP;
        }
        if unsafe { (*ap).ar_hln } != ETH_ALEN {
            return unsafe { (*info).target | !EBT_VERDICT_BITS };
        }
        if unsafe {
            skb_store_bits(
                skb,
                core::mem::size_of::<arphdr>(),
                (*info).mac.as_ptr() as *const c_void,
                ETH_ALEN,
            )
        } != 0 {
            return EBT_DROP;
        }
    }
    unsafe { (*info).target | !EBT_VERDICT_BITS }
}

static unsafe fn ebt_snat_tg_check(par: *const xt_tgchk_param) -> c_int {
    let info: *const ebt_nat_info = unsafe { (*par).targinfo as *const ebt_nat_info };
    let mut tmp: c_int;

    tmp = unsafe { (*info).target | !EBT_VERDICT_BITS };
    if BASE_CHAIN && tmp == EBT_RETURN {
        return -EINVAL;
    }

    if unsafe { ebt_invalid_target(tmp) } {
        return -EINVAL;
    }
    tmp = unsafe { (*info).target | EBT_VERDICT_BITS };
    if (tmp & !NAT_ARP_BIT) != !NAT_ARP_BIT {
        return -EINVAL;
    }
    0
}

static mut ebt_snat_tg_reg: xt_target = xt_target {
    name: b"snat\0".as_ptr() as *const c_char,
    revision: 0,
    family: NFPROTO_BRIDGE,
    table: b"nat\0".as_ptr() as *const c_char,
    hooks: (1 << NF_BR_NUMHOOKS) | (1 << NF_BR_POST_ROUTING),
    target: Some(ebt_snat_tg),
    checkentry: Some(ebt_snat_tg_check),
    targetsize: core::mem::size_of::<ebt_nat_info>(),
    me: THIS_MODULE,
};

unsafe fn ebt_snat_init() -> c_int {
    unsafe { xt_register_target(&mut ebt_snat_tg_reg) }
}

unsafe fn ebt_snat_fini() {
    unsafe { xt_unregister_target(&mut ebt_snat_tg_reg) };
}

// module_init(ebt_snat_init);
// module_exit(ebt_snat_fini);
// MODULE_DESCRIPTION("Ebtables: Source MAC address translation");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
