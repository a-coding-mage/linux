// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match AH parameters. */
/* (C) 1999-2000 Yon Uriarte <yon@astaro.de>
 */

// Dependency intent from the original source:
// linux/in.h, linux/module.h, linux/skbuff.h, linux/ip.h,
// linux/netfilter_ipv4/ipt_ah.h, linux/netfilter/x_tables.h

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Yon Uriarte <yon@astaro.de>");
// MODULE_DESCRIPTION("Xtables: IPv4 IPsec-AH SPI match");

/* Returns 1 if the spi is matched by the range, 0 otherwise */
#[inline]
unsafe fn spi_match(min: u32, max: u32, spi: u32, invert: bool) -> bool {
    ((spi >= min && spi <= max) as u8 ^ invert as u8) != 0
}

unsafe fn ah_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let mut _ahdr: ip_auth_hdr = core::mem::zeroed();
    let ah: *const ip_auth_hdr;
    let ahinfo: *const ipt_ah = (*par).matchinfo as *const ipt_ah;

    /* Must not be a fragment. */
    if (*par).fragoff != 0 {
        return false;
    }

    ah = skb_header_pointer(
        skb,
        (*par).thoff,
        core::mem::size_of::<ip_auth_hdr>(),
        &mut _ahdr as *mut ip_auth_hdr as *mut core::ffi::c_void,
    ) as *const ip_auth_hdr;
    if ah.is_null() {
        /* We've been asked to examine this packet, and we
         * can't.  Hence, no choice but to drop.
         */
        (*par).hotdrop = true;
        return false;
    }

    spi_match(
        (*ahinfo).spis[0],
        (*ahinfo).spis[1],
        u32::from_be((*ah).spi),
        ((*ahinfo).invflags & IPT_AH_INV_SPI) != 0,
    )
}

unsafe fn ah_mt_check(par: *const xt_mtchk_param) -> i32 {
    let ahinfo: *const ipt_ah = (*par).matchinfo as *const ipt_ah;

    /* Must specify no unknown invflags */
    if (*ahinfo).invflags & !IPT_AH_INV_MASK != 0 {
        pr_info_ratelimited!("unknown flags %X\n", (*ahinfo).invflags);
        return -22; // -EINVAL
    }
    0
}

static mut ah_mt_reg: xt_match = xt_match {
    name: "ah",
    family: NFPROTO_IPV4,
    match_fn: Some(ah_mt),
    matchsize: core::mem::size_of::<ipt_ah>(),
    proto: IPPROTO_AH,
    checkentry: Some(ah_mt_check),
    me: THIS_MODULE,
};

unsafe fn ah_mt_init() -> i32 {
    xt_register_match(&mut ah_mt_reg)
}

unsafe fn ah_mt_exit() {
    xt_unregister_match(&mut ah_mt_reg);
}

// Original module_init(ah_mt_init);
// Original module_exit(ah_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
