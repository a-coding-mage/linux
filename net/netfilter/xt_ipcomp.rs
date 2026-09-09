// SPDX-License-Identifier: GPL-2.0-or-later
/*  Kernel module to match IPComp parameters for IPv4 and IPv6
 *
 *  Copyright (C) 2013 WindRiver
 *
 *  Author:
 *  Fan Du <fan.du@windriver.com>
 *
 *  Based on:
 *  net/netfilter/xt_esp.c
 */

// C includes and build-time module configuration are supplied by the kernel
// environment and are intentionally not reproduced here.

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Fan Du <fan.du@windriver.com>");
// MODULE_DESCRIPTION("Xtables: IPv4/6 IPsec-IPComp SPI match");
// MODULE_ALIAS("ipt_ipcomp");
// MODULE_ALIAS("ip6t_ipcomp");

#[inline]
unsafe fn spi_match(min: u32, max: u32, spi: u32, invert: bool) -> bool {
    ((spi >= min) && (spi <= max)) ^ invert
}

unsafe fn comp_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let mut _comphdr: ip_comp_hdr = core::mem::zeroed();
    let chdr: *const ip_comp_hdr;
    let compinfo: *const xt_ipcomp = (*par).matchinfo as *const xt_ipcomp;

    /* Must not be a fragment. */
    if (*par).fragoff != 0 {
        return false;
    }

    chdr = skb_header_pointer(
        skb,
        (*par).thoff,
        core::mem::size_of::<ip_comp_hdr>(),
        &mut _comphdr as *mut ip_comp_hdr as *mut core::ffi::c_void,
    );
    if chdr.is_null() {
        /* We've been asked to examine this packet, and we
         * can't.  Hence, no choice but to drop.
         */
        (*par).hotdrop = true;
        return false;
    }

    spi_match(
        (*compinfo).spis[0],
        (*compinfo).spis[1],
        u16::from_be((*chdr).cpi) as u32,
        ((*compinfo).invflags & XT_IPCOMP_INV_SPI) != 0,
    )
}

unsafe fn comp_mt_check(par: *const xt_mtchk_param) -> i32 {
    let compinfo: *const xt_ipcomp = (*par).matchinfo as *const xt_ipcomp;

    /* Must specify no unknown invflags */
    if (*compinfo).invflags & !XT_IPCOMP_INV_MASK != 0 {
        // pr_info_ratelimited("unknown flags %X\n", (*compinfo).invflags);
        return -22; // -EINVAL
    }
    0
}

#[repr(C)]
static mut comp_mt_reg: [xt_match; 2] = [
    xt_match {
        name: "ipcomp", // .name
        family: NFPROTO_IPV4,
        match_fn: Some(comp_mt),
        matchsize: core::mem::size_of::<xt_ipcomp>(),
        proto: IPPROTO_COMP,
        checkentry: Some(comp_mt_check),
        me: THIS_MODULE,
    },
    xt_match {
        name: "ipcomp", // .name
        family: NFPROTO_IPV6,
        match_fn: Some(comp_mt),
        matchsize: core::mem::size_of::<xt_ipcomp>(),
        proto: IPPROTO_COMP,
        checkentry: Some(comp_mt_check),
        me: THIS_MODULE,
    },
];

unsafe fn comp_mt_init() -> i32 {
    xt_register_matches(comp_mt_reg.as_mut_ptr(), comp_mt_reg.len())
}

unsafe fn comp_mt_exit() {
    xt_unregister_matches(comp_mt_reg.as_mut_ptr(), comp_mt_reg.len());
}

// module_init(comp_mt_init);
// module_exit(comp_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
