// SPDX-License-Identifier: GPL-2.0-or-later
/* Kernel module to match Segment Routing Header (SRH) parameters. */

/* Author:
 * Ahmed Abdelsalam <amsalam20@gmail.com>
 */

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Linux kernel and netfilter dependencies are supplied externally.

/* Test a struct->mt_invflags and a boolean for inequality */
#[inline]
unsafe fn nf_srh_invf<T>(ptr: *const T, flag: u32, boolean: bool) -> bool {
    boolean ^ ((*((ptr as *const ip6t_srh).cast())).mt_invflags & flag != 0)
}

unsafe fn srh_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let srhinfo = (*par).matchinfo as *const ip6t_srh;
    let mut srh: *mut ipv6_sr_hdr;
    let mut _srh = core::mem::MaybeUninit::<ipv6_sr_hdr>::uninit();
    let mut hdrlen: i32;
    let mut srhoff: i32 = 0;

    if ipv6_find_hdr(skb, &mut srhoff, IPPROTO_ROUTING, core::ptr::null_mut(), core::ptr::null_mut()) < 0 { return false; }
    srh = skb_header_pointer(skb, srhoff, core::mem::size_of::<ipv6_sr_hdr>(), _srh.as_mut_ptr().cast());
    if srh.is_null() { return false; }
    hdrlen = ipv6_optlen(srh);
    if (*skb).len - srhoff < hdrlen { return false; }
    if (*srh).type_ != IPV6_SRCRT_TYPE_4 || (*srh).segments_left > (*srh).first_segment { return false; }

    if (*srhinfo).mt_flags & IP6T_SRH_NEXTHDR != 0 && nf_srh_invf(srhinfo, IP6T_SRH_INV_NEXTHDR, (*srh).nexthdr == (*srhinfo).next_hdr) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_LEN_EQ != 0 && nf_srh_invf(srhinfo, IP6T_SRH_INV_LEN_EQ, (*srh).hdrlen == (*srhinfo).hdr_len) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_LEN_GT != 0 && nf_srh_invf(srhinfo, IP6T_SRH_INV_LEN_GT, (*srh).hdrlen > (*srhinfo).hdr_len) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_LEN_LT != 0 && nf_srh_invf(srhinfo, IP6T_SRH_INV_LEN_LT, (*srh).hdrlen < (*srhinfo).hdr_len) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_SEGS_EQ != 0 && nf_srh_invf(srhinfo, IP6T_SRH_INV_SEGS_EQ, (*srh).segments_left == (*srhinfo).segs_left) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_SEGS_GT != 0 && nf_srh_invf(srhinfo, IP6T_SRH_INV_SEGS_GT, (*srh).segments_left > (*srhinfo).segs_left) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_SEGS_LT != 0 && nf_srh_invf(srhinfo, IP6T_SRH_INV_SEGS_LT, (*srh).segments_left < (*srhinfo).segs_left) { return false; }

    /**
     * Last Entry matching
     * Last_Entry field was introduced in revision 6 of the SRH draft.
     * It was called First_Segment in the previous revision
     */
    if (*srhinfo).mt_flags & IP6T_SRH_LAST_EQ != 0 && nf_srh_invf(srhinfo, IP6T_SRH_INV_LAST_EQ, (*srh).first_segment == (*srhinfo).last_entry) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_LAST_GT != 0 && nf_srh_invf(srhinfo, IP6T_SRH_INV_LAST_GT, (*srh).first_segment > (*srhinfo).last_entry) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_LAST_LT != 0 && nf_srh_invf(srhinfo, IP6T_SRH_INV_LAST_LT, (*srh).first_segment < (*srhinfo).last_entry) { return false; }
    /**
     * Tag matchig
     * Tag field was introduced in revision 6 of the SRH draft.
     */
    if (*srhinfo).mt_flags & IP6T_SRH_TAG != 0 && nf_srh_invf(srhinfo, IP6T_SRH_INV_TAG, (*srh).tag == (*srhinfo).tag) { return false; }
    true
}

unsafe fn srh1_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let srhinfo = (*par).matchinfo as *const ip6t_srh1;
    let mut srhoff: i32 = 0;
    let mut srh: *mut ipv6_sr_hdr;
    let mut _srh = core::mem::MaybeUninit::<ipv6_sr_hdr>::uninit();
    let mut hdrlen: i32;
    if ipv6_find_hdr(skb, &mut srhoff, IPPROTO_ROUTING, core::ptr::null_mut(), core::ptr::null_mut()) < 0 { return false; }
    srh = skb_header_pointer(skb, srhoff, core::mem::size_of::<ipv6_sr_hdr>(), _srh.as_mut_ptr().cast());
    if srh.is_null() { return false; }
    hdrlen = ipv6_optlen(srh);
    if (*skb).len - srhoff < hdrlen || (*srh).type_ != IPV6_SRCRT_TYPE_4 || (*srh).segments_left > (*srh).first_segment { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_NEXTHDR != 0 && nf_srh_invf(srhinfo.cast(), IP6T_SRH_INV_NEXTHDR, (*srh).nexthdr == (*srhinfo).next_hdr) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_LEN_EQ != 0 && nf_srh_invf(srhinfo.cast(), IP6T_SRH_INV_LEN_EQ, (*srh).hdrlen == (*srhinfo).hdr_len) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_LEN_GT != 0 && nf_srh_invf(srhinfo.cast(), IP6T_SRH_INV_LEN_GT, (*srh).hdrlen > (*srhinfo).hdr_len) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_LEN_LT != 0 && nf_srh_invf(srhinfo.cast(), IP6T_SRH_INV_LEN_LT, (*srh).hdrlen < (*srhinfo).hdr_len) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_SEGS_EQ != 0 && nf_srh_invf(srhinfo.cast(), IP6T_SRH_INV_SEGS_EQ, (*srh).segments_left == (*srhinfo).segs_left) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_SEGS_GT != 0 && nf_srh_invf(srhinfo.cast(), IP6T_SRH_INV_SEGS_GT, (*srh).segments_left > (*srhinfo).segs_left) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_SEGS_LT != 0 && nf_srh_invf(srhinfo.cast(), IP6T_SRH_INV_SEGS_LT, (*srh).segments_left < (*srhinfo).segs_left) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_LAST_EQ != 0 && nf_srh_invf(srhinfo.cast(), IP6T_SRH_INV_LAST_EQ, (*srh).first_segment == (*srhinfo).last_entry) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_LAST_GT != 0 && nf_srh_invf(srhinfo.cast(), IP6T_SRH_INV_LAST_GT, (*srh).first_segment > (*srhinfo).last_entry) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_LAST_LT != 0 && nf_srh_invf(srhinfo.cast(), IP6T_SRH_INV_LAST_LT, (*srh).first_segment < (*srhinfo).last_entry) { return false; }
    if (*srhinfo).mt_flags & IP6T_SRH_TAG != 0 && nf_srh_invf(srhinfo.cast(), IP6T_SRH_INV_TAG, (*srh).tag == (*srhinfo).tag) { return false; }

    if (*srhinfo).mt_flags & IP6T_SRH_PSID != 0 {
        if (*srh).segments_left == (*srh).first_segment { return false; }
        let psidoff = srhoff + core::mem::size_of::<ipv6_sr_hdr>() as i32 + ((*srh).segments_left as i32 + 1) * core::mem::size_of::<in6_addr>() as i32;
        let mut psid = core::mem::MaybeUninit::<in6_addr>::uninit();
        let p = skb_header_pointer(skb, psidoff, core::mem::size_of::<in6_addr>(), psid.as_mut_ptr().cast());
        if p.is_null() || nf_srh_invf(srhinfo.cast(), IP6T_SRH_INV_PSID, ipv6_masked_addr_cmp(p, &(*srhinfo).psid_msk, &(*srhinfo).psid_addr)) { return false; }
    }
    if (*srhinfo).mt_flags & IP6T_SRH_NSID != 0 {
        if (*srh).segments_left == 0 { return false; }
        let nsidoff = srhoff + core::mem::size_of::<ipv6_sr_hdr>() as i32 + ((*srh).segments_left as i32 - 1) * core::mem::size_of::<in6_addr>() as i32;
        let mut nsid = core::mem::MaybeUninit::<in6_addr>::uninit();
        let p = skb_header_pointer(skb, nsidoff, core::mem::size_of::<in6_addr>(), nsid.as_mut_ptr().cast());
        if p.is_null() || nf_srh_invf(srhinfo.cast(), IP6T_SRH_INV_NSID, ipv6_masked_addr_cmp(p, &(*srhinfo).nsid_msk, &(*srhinfo).nsid_addr)) { return false; }
    }
    if (*srhinfo).mt_flags & IP6T_SRH_LSID != 0 {
        let mut lsid = core::mem::MaybeUninit::<in6_addr>::uninit();
        let p = skb_header_pointer(skb, srhoff + core::mem::size_of::<ipv6_sr_hdr>() as i32, core::mem::size_of::<in6_addr>(), lsid.as_mut_ptr().cast());
        if p.is_null() || nf_srh_invf(srhinfo.cast(), IP6T_SRH_INV_LSID, ipv6_masked_addr_cmp(p, &(*srhinfo).lsid_msk, &(*srhinfo).lsid_addr)) { return false; }
    }
    true
}

unsafe fn srh_mt6_check(par: *const xt_mtchk_param) -> i32 {
    let s = (*par).matchinfo as *const ip6t_srh;
    if (*s).mt_flags & !IP6T_SRH_MASK != 0 { pr_info_ratelimited!("unknown srh match flags  %X\n", (*s).mt_flags); return -EINVAL; }
    if (*s).mt_invflags & !IP6T_SRH_INV_MASK != 0 { pr_info_ratelimited!("unknown srh invflags %X\n", (*s).mt_invflags); return -EINVAL; }
    0
}

unsafe fn srh1_mt6_check(par: *const xt_mtchk_param) -> i32 {
    let s = (*par).matchinfo as *const ip6t_srh1;
    if (*s).mt_flags & !IP6T_SRH_MASK != 0 { pr_info_ratelimited!("unknown srh match flags  %X\n", (*s).mt_flags); return -EINVAL; }
    if (*s).mt_invflags & !IP6T_SRH_INV_MASK != 0 { pr_info_ratelimited!("unknown srh invflags %X\n", (*s).mt_invflags); return -EINVAL; }
    0
}

static mut srh_mt6_reg: [xt_match; 2] = [
    xt_match { name: "srh", revision: 0, family: NFPROTO_IPV6, r#match: Some(srh_mt6), matchsize: core::mem::size_of::<ip6t_srh>(), checkentry: Some(srh_mt6_check), me: THIS_MODULE },
    xt_match { name: "srh", revision: 1, family: NFPROTO_IPV6, r#match: Some(srh1_mt6), matchsize: core::mem::size_of::<ip6t_srh1>(), checkentry: Some(srh1_mt6_check), me: THIS_MODULE },
];

unsafe fn srh_mt6_init() -> i32 { xt_register_matches(srh_mt6_reg.as_mut_ptr(), srh_mt6_reg.len()) }
unsafe fn srh_mt6_exit() { xt_unregister_matches(srh_mt6_reg.as_mut_ptr(), srh_mt6_reg.len()); }

module_init!(srh_mt6_init);
module_exit!(srh_mt6_exit);
module_license!("GPL");
module_description!("Xtables: IPv6 Segment Routing Header match");
module_author!("Ahmed Abdelsalam <amsalam20@gmail.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
