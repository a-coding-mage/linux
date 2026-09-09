// SPDX-License-Identifier: GPL-2.0-only
// Dependency intent from the original Linux includes is preserved through the
// external kernel/netfilter types, constants, and functions referenced below.

#[allow(non_camel_case_types)]
type u_int8_t = u8;
#[allow(non_camel_case_types)]
type u_int32_t = u32;

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Kiran Kumar Immidi");
// MODULE_DESCRIPTION("Xtables: SCTP protocol packet match");
// MODULE_ALIAS("ipt_sctp");
// MODULE_ALIAS("ip6t_sctp");

#[inline]
unsafe fn sccheck(cond: bool, option: u32, flag: u32, invflag: u32) -> bool {
    (!(flag & option != 0)) || (((invflag & option != 0) ^ cond))
}

unsafe fn match_flags(
    flag_info: *const xt_sctp_flag_info,
    flag_count: i32,
    chunktype: u_int8_t,
    chunkflags: u_int8_t,
) -> bool {
    let mut i = 0;
    while i < flag_count {
        let info = &*flag_info.add(i as usize);
        if info.chunktype == chunktype {
            return (chunkflags & info.flag_mask) == info.flag;
        }
        i += 1;
    }
    true
}

#[inline]
unsafe fn match_packet(
    skb: *const sk_buff,
    mut offset: u32,
    info: *const xt_sctp_info,
    hotdrop: *mut bool,
) -> bool {
    let mut chunkmapcopy: [u_int32_t; 256 / core::mem::size_of::<u_int32_t>()] = [0; 256 / core::mem::size_of::<u_int32_t>()];
    let mut _sch: sctp_chunkhdr = core::mem::zeroed();
    let chunk_match_type = (*info).chunk_match_type;
    let flag_info = (*info).flag_info;
    let flag_count = (*info).flag_count;

    if chunk_match_type == SCTP_CHUNK_MATCH_ALL {
        SCTP_CHUNKMAP_COPY(chunkmapcopy.as_mut_ptr(), (*info).chunkmap);
    }

    loop {
        let sch = skb_header_pointer(skb, offset, core::mem::size_of::<sctp_chunkhdr>() as i32, &mut _sch as *mut _ as *mut core::ffi::c_void);
        if sch.is_null() || (*sch).length == 0 {
            *hotdrop = true;
            return false;
        }
        offset += SCTP_PAD4(ntohs((*sch).length));

        if SCTP_CHUNKMAP_IS_SET((*info).chunkmap, (*sch).type_) {
            match chunk_match_type {
                SCTP_CHUNK_MATCH_ANY => {
                    if match_flags(flag_info, flag_count, (*sch).type_, (*sch).flags) { return true; }
                }
                SCTP_CHUNK_MATCH_ALL => {
                    if match_flags(flag_info, flag_count, (*sch).type_, (*sch).flags) {
                        SCTP_CHUNKMAP_CLEAR(chunkmapcopy.as_mut_ptr(), (*sch).type_);
                    }
                }
                SCTP_CHUNK_MATCH_ONLY => {
                    if !match_flags(flag_info, flag_count, (*sch).type_, (*sch).flags) { return false; }
                }
                _ => {}
            }
        } else if chunk_match_type == SCTP_CHUNK_MATCH_ONLY {
            return false;
        }
        if offset >= (*skb).len { break; }
    }

    match chunk_match_type {
        SCTP_CHUNK_MATCH_ALL => SCTP_CHUNKMAP_IS_CLEAR(chunkmapcopy.as_ptr()),
        SCTP_CHUNK_MATCH_ANY => false,
        SCTP_CHUNK_MATCH_ONLY => true,
        _ => false,
    }
}

unsafe fn sctp_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_sctp_info;
    let mut _sh: sctphdr = core::mem::zeroed();
    if (*par).fragoff != 0 { return false; }
    let sh = skb_header_pointer(skb, (*par).thoff, core::mem::size_of::<sctphdr>() as i32, &mut _sh as *mut _ as *mut core::ffi::c_void);
    if sh.is_null() { (*par).hotdrop = true; return false; }
    sccheck(ntohs((*sh).source) >= (*info).spts[0] && ntohs((*sh).source) <= (*info).spts[1], XT_SCTP_SRC_PORTS, (*info).flags, (*info).invflags)
        && sccheck(ntohs((*sh).dest) >= (*info).dpts[0] && ntohs((*sh).dest) <= (*info).dpts[1], XT_SCTP_DEST_PORTS, (*info).flags, (*info).invflags)
        && sccheck(match_packet(skb, (*par).thoff + core::mem::size_of::<sctphdr>() as u32, info, &mut (*par).hotdrop), XT_SCTP_CHUNK_TYPES, (*info).flags, (*info).invflags)
}

unsafe fn sctp_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_sctp_info;
    if (*info).flag_count > core::mem::size_of_val(&(*info).flag_info) / core::mem::size_of::<xt_sctp_flag_info>() { return -EINVAL; }
    if (*info).flags & !XT_SCTP_VALID_FLAGS != 0 || (*info).invflags & !XT_SCTP_VALID_FLAGS != 0 || (*info).invflags & !(*info).flags != 0 { return -EINVAL; }
    if (*info).flags & XT_SCTP_CHUNK_TYPES == 0 { return 0; }
    if (*info).chunk_match_type & (SCTP_CHUNK_MATCH_ALL | SCTP_CHUNK_MATCH_ANY | SCTP_CHUNK_MATCH_ONLY) != 0 { return 0; }
    -EINVAL
}

static mut sctp_mt_reg: [xt_match; 2] = [
    xt_match { name: "sctp", family: NFPROTO_IPV4, checkentry: Some(sctp_mt_check), r#match: Some(sctp_mt), matchsize: core::mem::size_of::<xt_sctp_info>(), proto: IPPROTO_SCTP, me: THIS_MODULE },
    xt_match { name: "sctp", family: NFPROTO_IPV6, checkentry: Some(sctp_mt_check), r#match: Some(sctp_mt), matchsize: core::mem::size_of::<xt_sctp_info>(), proto: IPPROTO_SCTP, me: THIS_MODULE },
];

unsafe fn sctp_mt_init() -> i32 { xt_register_matches(sctp_mt_reg.as_mut_ptr(), sctp_mt_reg.len()) }
unsafe fn sctp_mt_exit() { xt_unregister_matches(sctp_mt_reg.as_mut_ptr(), sctp_mt_reg.len()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
