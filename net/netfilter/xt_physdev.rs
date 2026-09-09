// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match the bridge port in and
 * out device for IP packets coming into contact with a bridge. */

/* (C) 2001-2003 Bart De Schuymer <bdschuym@pandora.be>
 */
// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt

// C dependencies supplied by the surrounding kernel translation unit.

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Bart De Schuymer <bdschuym@pandora.be>");
// MODULE_DESCRIPTION("Xtables: Bridge physical device match");
// MODULE_ALIAS("ipt_physdev");
// MODULE_ALIAS("ip6t_physdev");

unsafe fn physdev_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_physdev_info;
    let mut physdev: *const net_device;
    let mut ret: c_ulong;
    let indev: *const c_char;
    let outdev: *const c_char;

    /* Not a bridged IP packet or no info available yet:
     * LOCAL_OUT/mangle and LOCAL_OUT/nat don't know if
     * the destination device will be a bridge. */
    if !nf_bridge_info_exists(skb) {
        /* Return MATCH if the invert flags of the used options are on */
        if ((*info).bitmask & XT_PHYSDEV_OP_BRIDGED) != 0
            && ((*info).invert & XT_PHYSDEV_OP_BRIDGED) == 0
        {
            return false;
        }
        if ((*info).bitmask & XT_PHYSDEV_OP_ISIN) != 0
            && ((*info).invert & XT_PHYSDEV_OP_ISIN) == 0
        {
            return false;
        }
        if ((*info).bitmask & XT_PHYSDEV_OP_ISOUT) != 0
            && ((*info).invert & XT_PHYSDEV_OP_ISOUT) == 0
        {
            return false;
        }
        if ((*info).bitmask & XT_PHYSDEV_OP_IN) != 0
            && ((*info).invert & XT_PHYSDEV_OP_IN) == 0
        {
            return false;
        }
        if ((*info).bitmask & XT_PHYSDEV_OP_OUT) != 0
            && ((*info).invert & XT_PHYSDEV_OP_OUT) == 0
        {
            return false;
        }
        return true;
    }

    physdev = nf_bridge_get_physoutdev(skb);
    outdev = if !physdev.is_null() { (*physdev).name } else { core::ptr::null() };

    /* This only makes sense in the FORWARD and POSTROUTING chains */
    if ((*info).bitmask & XT_PHYSDEV_OP_BRIDGED) != 0
        && ((!outdev.is_null()) ^ (((*info).invert & XT_PHYSDEV_OP_BRIDGED) == 0))
    {
        return false;
    }

    physdev = nf_bridge_get_physindev(skb, xt_net(par));
    indev = if !physdev.is_null() { (*physdev).name } else { core::ptr::null() };

    if (((*info).bitmask & XT_PHYSDEV_OP_ISIN) != 0
        && ((!indev.is_null()) ^ (((*info).invert & XT_PHYSDEV_OP_ISIN) != 0)))
        || (((*info).bitmask & XT_PHYSDEV_OP_ISOUT) != 0
            && ((!outdev.is_null()) ^ (((*info).invert & XT_PHYSDEV_OP_ISOUT) != 0)))
    {
        return false;
    }

    if ((*info).bitmask & XT_PHYSDEV_OP_IN) != 0 {
        if !indev.is_null() {
            ret = ifname_compare_aligned(indev, (*info).physindev.as_ptr(), (*info).in_mask);
            if ((!ret != 0) ^ (((*info).invert & XT_PHYSDEV_OP_IN) == 0)) {
                return false;
            }
        }
    }

    if ((*info).bitmask & XT_PHYSDEV_OP_OUT) == 0 {
        return true;
    }
    if outdev.is_null() {
        return false;
    }

    ret = ifname_compare_aligned(outdev, (*info).physoutdev.as_ptr(), (*info).out_mask);
    ((!ret != 0) ^ (((*info).invert & XT_PHYSDEV_OP_OUT) == 0))
}

unsafe fn physdev_mt_check_hooks(par: *const xt_mtchk_param) -> c_int {
    let info = (*par).matchinfo as *const xt_physdev_info;

    if ((*info).bitmask & (XT_PHYSDEV_OP_OUT | XT_PHYSDEV_OP_ISOUT)) != 0
        && (((*info).bitmask & XT_PHYSDEV_OP_BRIDGED) == 0
            || ((*info).invert & XT_PHYSDEV_OP_BRIDGED) != 0)
        && ((*par).hook_mask & (1 << NF_INET_LOCAL_OUT)) != 0
    {
        pr_info_ratelimited!("--physdev-out and --physdev-is-out only supported in the FORWARD and POSTROUTING chains with bridged traffic\n");
        return -EINVAL;
    }
    0
}

unsafe fn physdev_mt_check(par: *const xt_mtchk_param) -> c_int {
    let info = (*par).matchinfo as *const xt_physdev_info;
    static mut brnf_probed: bool = false;

    if ((*info).bitmask & XT_PHYSDEV_OP_MASK) == 0
        || ((*info).bitmask & !XT_PHYSDEV_OP_MASK) != 0
    {
        return -EINVAL;
    }

    if ((*info).bitmask & XT_PHYSDEV_OP_IN) != 0 {
        if (*info).physindev[0] == 0 {
            return -EINVAL;
        }
        if strnlen((*info).physindev.as_ptr(), size_of_val(&(*info).physindev))
            >= size_of_val(&(*info).physindev)
        {
            return -ENAMETOOLONG;
        }
    }

    if ((*info).bitmask & XT_PHYSDEV_OP_OUT) != 0 {
        if (*info).physoutdev[0] == 0 {
            return -EINVAL;
        }
        if strnlen((*info).physoutdev.as_ptr(), size_of_val(&(*info).physoutdev))
            >= size_of_val(&(*info).physoutdev)
        {
            return -ENAMETOOLONG;
        }
    }

    if !brnf_probed {
        brnf_probed = true;
        request_module!("br_netfilter");
    }
    0
}

static mut physdev_mt_reg: [xt_match; 2] = [
    xt_match {
        name: *b"physdev\0",
        family: NFPROTO_IPV4,
        check_hooks: Some(physdev_mt_check_hooks),
        checkentry: Some(physdev_mt_check),
        match_: Some(physdev_mt),
        matchsize: size_of::<xt_physdev_info>(),
        me: THIS_MODULE,
    },
    xt_match {
        name: *b"physdev\0",
        family: NFPROTO_IPV6,
        check_hooks: Some(physdev_mt_check_hooks),
        checkentry: Some(physdev_mt_check),
        match_: Some(physdev_mt),
        matchsize: size_of::<xt_physdev_info>(),
        me: THIS_MODULE,
    },
];

unsafe fn physdev_mt_init() -> c_int {
    xt_register_matches(physdev_mt_reg.as_mut_ptr(), physdev_mt_reg.len())
}

unsafe fn physdev_mt_exit() {
    xt_unregister_matches(physdev_mt_reg.as_mut_ptr(), physdev_mt_reg.len());
}

// module_init(physdev_mt_init);
// module_exit(physdev_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
