// SPDX-License-Identifier: GPL-2.0-only

// Declarations supplied by bitset.h, common.h, and netlink.h remain external
// dependencies of this translation.

#[repr(C)]
pub struct privflags_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct privflags_reply_data {
    pub base: ethnl_reply_data,
    pub priv_flag_names: *const [u8; ETH_GSTRING_LEN],
    pub n_priv_flags: ::core::ffi::c_uint,
    pub priv_flags: u32,
}

#[allow(non_camel_case_types)]
pub type u8 = ::core::ffi::c_uchar;

extern "C" {
    pub static ethnl_privflags_get_policy: [nla_policy; ETHTOOL_A_PRIVFLAGS_HEADER + 1];
    pub static ethnl_privflags_set_policy: [nla_policy; ETHTOOL_A_PRIVFLAGS_FLAGS + 1];
}

unsafe fn privflags_repdata(reply_base: *const ethnl_reply_data) -> *mut privflags_reply_data {
    reply_base as *mut privflags_reply_data
}

pub unsafe fn ethnl_get_priv_flags_info(
    dev: *mut net_device,
    count: *mut ::core::ffi::c_uint,
    names: *mut *const [u8; ETH_GSTRING_LEN],
) -> ::core::ffi::c_int {
    let ops = (*dev).ethtool_ops;
    let mut nflags = ((*ops).get_sset_count)(dev, ETH_SS_PRIV_FLAGS);
    if nflags < 0 {
        return nflags;
    }

    if !names.is_null() {
        *names = kcalloc(nflags as usize, ETH_GSTRING_LEN, GFP_KERNEL)
            as *const [u8; ETH_GSTRING_LEN];
        if (*names).is_null() {
            return -ENOMEM;
        }
        ((*ops).get_strings)(dev, ETH_SS_PRIV_FLAGS, *names as *mut u8);
    }

    // We can pass more than 32 private flags to userspace via netlink but
    // we cannot get more with ethtool_ops::get_priv_flags(). Note that we
    // must not adjust nflags before allocating the space for flag names
    // as the buffer must be large enough for all flags.
    if WARN_ONCE(
        nflags > 32,
        "device %s reports more than 32 private flags (%d)\n",
        netdev_name(dev),
        nflags,
    ) {
        nflags = 32;
    }
    *count = nflags as ::core::ffi::c_uint;
    0
}

pub unsafe fn privflags_prepare_data(
    _req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    _info: *const genl_info,
) -> ::core::ffi::c_int {
    let data = privflags_repdata(reply_base);
    let dev = (*reply_base).dev;
    let mut names: *const [u8; ETH_GSTRING_LEN] = core::ptr::null();
    let ops = (*dev).ethtool_ops;
    let mut nflags: ::core::ffi::c_uint = 0;
    if (*ops).get_priv_flags.is_none()
        || (*ops).get_sset_count.is_none()
        || (*ops).get_strings.is_none()
    {
        return -EOPNOTSUPP;
    }
    let mut ret = ethnl_ops_begin(dev);
    if ret < 0 {
        return ret;
    }
    ret = ethnl_get_priv_flags_info(dev, &mut nflags, &mut names);
    if ret < 0 {
        ethnl_ops_complete(dev);
        return ret;
    }
    (*data).priv_flags = ((*ops).get_priv_flags.unwrap())(dev);
    (*data).priv_flag_names = names;
    (*data).n_priv_flags = nflags;
    ethnl_ops_complete(dev);
    ret
}

pub unsafe fn privflags_reply_size(
    req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> ::core::ffi::c_int {
    let data = privflags_repdata(reply_base);
    let compact = (*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS != 0;
    let all_flags = !0u32 >> (32 - (*data).n_priv_flags);
    ethnl_bitset32_size(
        &(*data).priv_flags, &all_flags, (*data).n_priv_flags,
        (*data).priv_flag_names, compact,
    )
}

pub unsafe fn privflags_fill_reply(
    skb: *mut sk_buff,
    req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> ::core::ffi::c_int {
    let data = privflags_repdata(reply_base);
    let compact = (*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS != 0;
    let all_flags = !0u32 >> (32 - (*data).n_priv_flags);
    ethnl_put_bitset32(
        skb, ETHTOOL_A_PRIVFLAGS_FLAGS, &(*data).priv_flags, &all_flags,
        (*data).n_priv_flags, (*data).priv_flag_names, compact,
    )
}

pub unsafe fn privflags_cleanup_data(reply_data: *mut ethnl_reply_data) {
    let data = privflags_repdata(reply_data);
    kfree((*data).priv_flag_names as *mut ::core::ffi::c_void);
}

pub unsafe fn ethnl_set_privflags_validate(
    req_info: *mut ethnl_req_info,
    info: *mut genl_info,
) -> ::core::ffi::c_int {
    let ops = (*(*req_info).dev).ethtool_ops;
    if (*info).attrs[ETHTOOL_A_PRIVFLAGS_FLAGS].is_null() {
        return -EINVAL;
    }
    if (*ops).get_priv_flags.is_none()
        || (*ops).set_priv_flags.is_none()
        || (*ops).get_sset_count.is_none()
        || (*ops).get_strings.is_none()
    {
        return -EOPNOTSUPP;
    }
    1
}

pub unsafe fn ethnl_set_privflags(
    req_info: *mut ethnl_req_info,
    info: *mut genl_info,
) -> ::core::ffi::c_int {
    let mut names: *const [u8; ETH_GSTRING_LEN] = core::ptr::null();
    let dev = (*req_info).dev;
    let tb = (*info).attrs;
    let mut nflags: ::core::ffi::c_uint = 0;
    let mut modified = false;
    let mut compact = false;
    let mut flags: u32;
    let mut ret = ethnl_bitset_is_compact(tb[ETHTOOL_A_PRIVFLAGS_FLAGS], &mut compact);
    if ret < 0 { return ret; }
    ret = ethnl_get_priv_flags_info(dev, &mut nflags, if compact { core::ptr::null_mut() } else { &mut names });
    if ret < 0 { return ret; }
    flags = ((*(*dev).ethtool_ops).get_priv_flags.unwrap())(dev);
    ret = ethnl_update_bitset32(&mut flags, nflags, tb[ETHTOOL_A_PRIVFLAGS_FLAGS], names, (*info).extack, &mut modified);
    if ret < 0 || !modified { kfree(names as *mut ::core::ffi::c_void); return ret; }
    ret = ((*(*dev).ethtool_ops).set_priv_flags.unwrap())(dev, flags);
    if ret >= 0 { ret = 1; }
    kfree(names as *mut ::core::ffi::c_void);
    ret
}

pub static ethnl_privflags_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_PRIVFLAGS_GET,
    reply_cmd: ETHTOOL_MSG_PRIVFLAGS_GET_REPLY,
    hdr_attr: ETHTOOL_A_PRIVFLAGS_HEADER,
    req_info_size: core::mem::size_of::<privflags_req_info>(),
    reply_data_size: core::mem::size_of::<privflags_reply_data>(),
    prepare_data: Some(privflags_prepare_data), reply_size: Some(privflags_reply_size),
    fill_reply: Some(privflags_fill_reply), cleanup_data: Some(privflags_cleanup_data),
    set_validate: Some(ethnl_set_privflags_validate), set: Some(ethnl_set_privflags),
    set_ntf_cmd: ETHTOOL_MSG_PRIVFLAGS_NTF,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
