// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebt_arp
 *
 *	Authors:
 *	Bart De Schuymer <bdschuym@pandora.be>
 *	Tim Gardner <timg@tpi.com>
 *
 *  April, 2002
 *
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/if_arp.h, linux/if_ether.h, linux/module.h,
// linux/netfilter/x_tables.h, linux/netfilter_bridge/ebtables.h,
// linux/netfilter_bridge/ebt_arp.h

unsafe fn ebt_arp_mt(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let info = (*par).matchinfo as *const ebt_arp_info;
    let mut ah: *const arphdr;
    let mut _arph: arphdr = core::mem::zeroed();

    ah = skb_header_pointer(
        skb,
        0,
        core::mem::size_of::<arphdr>(),
        &mut _arph as *mut arphdr as *mut core::ffi::c_void,
    );
    if ah.is_null() {
        return false;
    }
    if ((*info).bitmask & EBT_ARP_OPCODE) != 0
        && NF_INVF(info, EBT_ARP_OPCODE, (*info).opcode != (*ah).ar_op)
    {
        return false;
    }
    if ((*info).bitmask & EBT_ARP_HTYPE) != 0
        && NF_INVF(info, EBT_ARP_HTYPE, (*info).htype != (*ah).ar_hrd)
    {
        return false;
    }
    if ((*info).bitmask & EBT_ARP_PTYPE) != 0
        && NF_INVF(info, EBT_ARP_PTYPE, (*info).ptype != (*ah).ar_pro)
    {
        return false;
    }

    if ((*info).bitmask & (EBT_ARP_SRC_IP | EBT_ARP_DST_IP | EBT_ARP_GRAT)) != 0 {
        let mut sap: *const __be32;
        let mut dap: *const __be32;
        let mut saddr: __be32 = 0;
        let mut daddr: __be32 = 0;

        if (*ah).ar_pln != core::mem::size_of::<__be32>() as _
            || (*ah).ar_pro != htons(ETH_P_IP)
        {
            return false;
        }
        sap = skb_header_pointer(
            skb,
            core::mem::size_of::<arphdr>() + (*ah).ar_hln as usize,
            core::mem::size_of::<__be32>(),
            &mut saddr as *mut __be32 as *mut core::ffi::c_void,
        );
        if sap.is_null() {
            return false;
        }
        dap = skb_header_pointer(
            skb,
            core::mem::size_of::<arphdr>() + 2 * (*ah).ar_hln as usize + core::mem::size_of::<__be32>(),
            core::mem::size_of::<__be32>(),
            &mut daddr as *mut __be32 as *mut core::ffi::c_void,
        );
        if dap.is_null() {
            return false;
        }
        if ((*info).bitmask & EBT_ARP_SRC_IP) != 0
            && NF_INVF(info, EBT_ARP_SRC_IP, (*info).saddr != (*sap & (*info).smsk))
        {
            return false;
        }
        if ((*info).bitmask & EBT_ARP_DST_IP) != 0
            && NF_INVF(info, EBT_ARP_DST_IP, (*info).daddr != (*dap & (*info).dmsk))
        {
            return false;
        }
        if ((*info).bitmask & EBT_ARP_GRAT) != 0
            && NF_INVF(info, EBT_ARP_GRAT, *dap != *sap)
        {
            return false;
        }
    }

    if ((*info).bitmask & (EBT_ARP_SRC_MAC | EBT_ARP_DST_MAC)) != 0 {
        let mut mp: *const u8;
        let mut _mac = [0u8; ETH_ALEN as usize];

        if (*ah).ar_hln != ETH_ALEN || (*ah).ar_hrd != htons(ARPHRD_ETHER) {
            return false;
        }
        if ((*info).bitmask & EBT_ARP_SRC_MAC) != 0 {
            mp = skb_header_pointer(
                skb,
                core::mem::size_of::<arphdr>(),
                core::mem::size_of_val(&_mac),
                _mac.as_mut_ptr() as *mut core::ffi::c_void,
            );
            if mp.is_null() {
                return false;
            }
            if NF_INVF(info, EBT_ARP_SRC_MAC, !ether_addr_equal_masked(mp, (*info).smaddr, (*info).smmsk)) {
                return false;
            }
        }
        if ((*info).bitmask & EBT_ARP_DST_MAC) != 0 {
            mp = skb_header_pointer(
                skb,
                core::mem::size_of::<arphdr>() + (*ah).ar_hln as usize + (*ah).ar_pln as usize,
                core::mem::size_of_val(&_mac),
                _mac.as_mut_ptr() as *mut core::ffi::c_void,
            );
            if mp.is_null() {
                return false;
            }
            if NF_INVF(info, EBT_ARP_DST_MAC, !ether_addr_equal_masked(mp, (*info).dmaddr, (*info).dmmsk)) {
                return false;
            }
        }
    }

    true
}

unsafe fn ebt_arp_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const ebt_arp_info;
    let e = (*par).entryinfo as *const ebt_entry;

    if (((*e).ethproto != htons(ETH_P_ARP)) && ((*e).ethproto != htons(ETH_P_RARP)))
        || ((*e).invflags & EBT_IPROTO) != 0
        || ((*info).bitmask & !EBT_ARP_MASK) != 0
        || ((*info).invflags & !EBT_ARP_MASK) != 0
    {
        return -EINVAL;
    }
    0
}

static mut ebt_arp_mt_reg: xt_match = xt_match {
    name: c"arp".as_ptr(),
    revision: 0,
    family: NFPROTO_BRIDGE,
    match_: Some(ebt_arp_mt),
    checkentry: Some(ebt_arp_mt_check),
    matchsize: core::mem::size_of::<ebt_arp_info>(),
    me: THIS_MODULE,
};

unsafe fn ebt_arp_init() -> i32 {
    xt_register_match(&raw mut ebt_arp_mt_reg)
}

unsafe fn ebt_arp_fini() {
    xt_unregister_match(&raw mut ebt_arp_mt_reg);
}

// module_init(ebt_arp_init);
// module_exit(ebt_arp_fini);
// MODULE_DESCRIPTION("Ebtables: ARP protocol packet match");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
