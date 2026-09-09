// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match FRAG parameters. */

/* (C) 2001-2002 Andras Kis-Szabo <kisza@sch.bme.hu>
 */
// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Dependencies supplied by the Linux kernel and netfilter headers.

// MODULE_LICENSE!("GPL");
// MODULE_DESCRIPTION!("Xtables: IPv6 fragment match");
// MODULE_AUTHOR!("Andras Kis-Szabo <kisza@sch.bme.hu>");

/* Returns 1 if the id is matched by the range, 0 otherwise */
#[inline]
unsafe fn id_match(min: u32, max: u32, id: u32, invert: bool) -> bool {
    ((id >= min && id <= max) as u8 ^ invert as u8) != 0
}

unsafe fn frag_mt6(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let mut frag: frag_hdr = core::mem::zeroed();
    let mut fh: *const frag_hdr;
    let fraginfo: *const ip6t_frag = (*par).matchinfo as *const ip6t_frag;
    let mut ptr: u32 = 0;
    let err: i32;

    err = ipv6_find_hdr(skb, &mut ptr, NEXTHDR_FRAGMENT, core::ptr::null_mut(), core::ptr::null_mut());
    if err < 0 {
        if err != -ENOENT {
            (*par).hotdrop = true;
        }
        return false;
    }

    fh = skb_header_pointer(
        skb,
        ptr,
        core::mem::size_of::<frag_hdr>(),
        &mut frag as *mut frag_hdr as *mut core::ffi::c_void,
    ) as *const frag_hdr;
    if fh.is_null() {
        (*par).hotdrop = true;
        return false;
    }

    id_match(
        (*fraginfo).ids[0],
        (*fraginfo).ids[1],
        ntohl((*fh).identification),
        ((*fraginfo).invflags & IP6T_FRAG_INV_IDS) != 0,
    ) && !(((*fraginfo).flags & IP6T_FRAG_RES) != 0
        && ((*fh).reserved != 0 || (ntohs((*fh).frag_off) & 0x6) != 0))
        && !(((*fraginfo).flags & IP6T_FRAG_FST) != 0
            && (ntohs((*fh).frag_off) & !0x7) != 0)
        && !(((*fraginfo).flags & IP6T_FRAG_MF) != 0
            && (ntohs((*fh).frag_off) & IP6_MF) == 0)
        && !(((*fraginfo).flags & IP6T_FRAG_NMF) != 0
            && (ntohs((*fh).frag_off) & IP6_MF) != 0)
}

unsafe fn frag_mt6_check(par: *const xt_mtchk_param) -> i32 {
    let fraginfo: *const ip6t_frag = (*par).matchinfo as *const ip6t_frag;

    if ((*fraginfo).invflags & !IP6T_FRAG_INV_MASK) != 0 {
        pr_info_ratelimited!("unknown flags %X\n", (*fraginfo).invflags);
        return -EINVAL;
    }
    0
}

static mut frag_mt6_reg: xt_match = xt_match {
    name: *b"frag\0",
    family: NFPROTO_IPV6,
    match_: Some(frag_mt6),
    matchsize: core::mem::size_of::<ip6t_frag>(),
    checkentry: Some(frag_mt6_check),
    me: THIS_MODULE,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn frag_mt6_init() -> i32 {
    xt_register_match(&mut frag_mt6_reg)
}

unsafe fn frag_mt6_exit() {
    xt_unregister_match(&mut frag_mt6_reg);
}

// module_init(frag_mt6_init);
// module_exit(frag_mt6_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
