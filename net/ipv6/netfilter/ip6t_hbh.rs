// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match Hop-by-Hop and Destination parameters. */

/* (C) 2001-2002 Andras Kis-Szabo <kisza@sch.bme.hu>
 */
// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Kernel and networking dependencies are supplied externally.

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Xtables: IPv6 Hop-By-Hop and Destination Header match");
// MODULE_AUTHOR("Andras Kis-Szabo <kisza@sch.bme.hu>");
// MODULE_ALIAS("ip6t_dst");

/*
 *  (Type & 0xC0) >> 6
 *	0	-> ignorable
 *	1	-> must drop the packet
 *	2	-> send ICMP PARM PROB regardless and drop packet
 *	3	-> Send ICMP if not a multicast address and drop packet
 *  (Type & 0x20) >> 5
 *	0	-> invariant
 *	1	-> can change the routing
 *  (Type & 0x1F) Type
 *	0	-> Pad1 (only 1 byte!)
 *	1	-> PadN LENGTH info (total length = length + 2)
 *	C0 | 2	-> JUMBO 4 x x x x ( xxxx > 64k )
 *	5	-> RTALERT 2 x x
 */

static mut hbh_mt6_reg: [xt_match; 2] = [
    xt_match {
        name: b"hbh\0".as_ptr() as *const i8,
        family: NFPROTO_IPV6,
        match_: Some(hbh_mt6),
        matchsize: core::mem::size_of::<ip6t_opts>(),
        checkentry: Some(hbh_mt6_check),
        me: THIS_MODULE,
    },
    xt_match {
        name: b"dst\0".as_ptr() as *const i8,
        family: NFPROTO_IPV6,
        match_: Some(hbh_mt6),
        matchsize: core::mem::size_of::<ip6t_opts>(),
        checkentry: Some(hbh_mt6_check),
        me: THIS_MODULE,
    },
];

unsafe fn hbh_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let mut optsh = ipv6_opt_hdr::default();
    let mut ptr: u32 = 0;
    let mut hdrlen: u32 = 0;
    let mut ret = false;
    let mut opttype: u8 = 0;
    let mut optlen_byte: u8 = 0;
    let mut temp: u32;
    let mut optlen: u32;
    let optinfo = (*par).matchinfo as *const ip6t_opts;
    let err = ipv6_find_hdr(skb, &mut ptr, if (*par).match_ == hbh_mt6_reg.as_ptr() { NEXTHDR_HOP } else { NEXTHDR_DEST }, core::ptr::null_mut(), core::ptr::null_mut());
    if err < 0 {
        if err != -ENOENT { (*par).hotdrop = true; }
        return false;
    }
    let oh = skb_header_pointer(skb, ptr, core::mem::size_of::<ipv6_opt_hdr>(), &mut optsh as *mut _ as *mut core::ffi::c_void);
    if oh.is_null() { (*par).hotdrop = true; return false; }
    hdrlen = ipv6_optlen(oh);
    if (*skb).len - ptr < hdrlen { (*par).hotdrop = true; return false; }
    ret = ((*optinfo).flags & IP6T_OPTS_LEN == 0) || (((*optinfo).hdrlen == hdrlen) ^ (((*optinfo).invflags & IP6T_OPTS_INV_LEN) != 0));
    ptr += 2; hdrlen -= 2;
    if (*optinfo).flags & IP6T_OPTS_OPTS == 0 { return ret; }
    temp = 0;
    while temp < (*optinfo).optsnr {
        if hdrlen < 1 { break; }
        let tp = skb_header_pointer(skb, ptr, core::mem::size_of::<u8>(), &mut opttype as *mut _ as *mut core::ffi::c_void);
        if tp.is_null() { break; }
        if opttype != (((*optinfo).opts[temp as usize] & 0xFF00) >> 8) as u8 { return false; }
        if opttype != 0 {
            if hdrlen < 2 { break; }
            let lp = skb_header_pointer(skb, ptr + 1, core::mem::size_of::<u8>(), &mut optlen_byte as *mut _ as *mut core::ffi::c_void);
            if lp.is_null() { break; }
            let spec_len = (*optinfo).opts[temp as usize] & 0x00FF;
            if spec_len != 0x00FF && spec_len != optlen_byte as u16 { return false; }
            optlen = optlen_byte as u32 + 2;
        } else { optlen = 1; }
        if (ptr > (*skb).len - optlen || hdrlen < optlen) && temp < (*optinfo).optsnr - 1 { break; }
        ptr += optlen; hdrlen -= optlen; temp += 1;
    }
    if temp == (*optinfo).optsnr { ret } else { false }
}

unsafe fn hbh_mt6_check(par: *const xt_mtchk_param) -> i32 {
    let optsinfo = (*par).matchinfo as *const ip6t_opts;
    if (*optsinfo).invflags & !IP6T_OPTS_INV_MASK != 0 { pr_info_ratelimited!("unknown flags %X\n", (*optsinfo).invflags); return -EINVAL; }
    if (*optsinfo).optsnr > IP6T_OPTS_OPTSNR { pr_info_ratelimited!("too many supported opts specified\n"); return -EINVAL; }
    if (*optsinfo).flags & IP6T_OPTS_NSTRICT != 0 { pr_info_ratelimited!("Not strict - not implemented\n"); return -EINVAL; }
    0
}

unsafe extern "C" fn hbh_mt6_init() -> i32 { xt_register_matches(hbh_mt6_reg.as_mut_ptr(), hbh_mt6_reg.len()) }
unsafe extern "C" fn hbh_mt6_exit() { xt_unregister_matches(hbh_mt6_reg.as_mut_ptr(), hbh_mt6_reg.len()); }

// module_init(hbh_mt6_init);
// module_exit(hbh_mt6_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
