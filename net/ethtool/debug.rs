// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding translation unit:
// bitset.h, common.h, and netlink.h

#[repr(C)]
pub struct debug_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct debug_reply_data {
    pub base: ethnl_reply_data,
    pub msg_mask: u32,
}

// DEBUG_REPDATA(__reply_base): `base` is the first member of debug_reply_data.
#[inline]
unsafe fn debug_repdata(reply_base: *mut ethnl_reply_data) -> *mut debug_reply_data {
    reply_base as *mut debug_reply_data
}

pub static ethnl_debug_get_policy: [nla_policy; ETHTOOL_A_DEBUG_HEADER as usize + 1] = {
    let mut policy = [nla_policy { type_: 0 }; ETHTOOL_A_DEBUG_HEADER as usize + 1];
    policy[ETHTOOL_A_DEBUG_HEADER as usize] = NLA_POLICY_NESTED(ethnl_header_policy);
    policy
};

unsafe fn debug_prepare_data(
    _req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    _info: *const genl_info,
) -> i32 {
    let data = &mut *debug_repdata(reply_base);
    let dev = (*reply_base).dev;

    if (*(*dev).ethtool_ops).get_msglevel.is_none() {
        return -EOPNOTSUPP;
    }

    let ret = ethnl_ops_begin(dev);
    if ret < 0 {
        return ret;
    }
    data.msg_mask = ((*(*dev).ethtool_ops).get_msglevel.unwrap())(dev);
    ethnl_ops_complete(dev);

    0
}

unsafe fn debug_reply_size(
    req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> i32 {
    let data = &*debug_repdata(reply_base as *mut ethnl_reply_data);
    let compact = ((*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS) != 0;

    ethnl_bitset32_size(
        &data.msg_mask,
        core::ptr::null(),
        NETIF_MSG_CLASS_COUNT,
        netif_msg_class_names,
        compact,
    )
}

unsafe fn debug_fill_reply(
    skb: *mut sk_buff,
    req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> i32 {
    let data = &*debug_repdata(reply_base as *mut ethnl_reply_data);
    let compact = ((*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS) != 0;

    ethnl_put_bitset32(
        skb,
        ETHTOOL_A_DEBUG_MSGMASK,
        &data.msg_mask,
        core::ptr::null(),
        NETIF_MSG_CLASS_COUNT,
        netif_msg_class_names,
        compact,
    )
}

/* DEBUG_SET */

pub static ethnl_debug_set_policy: [nla_policy; ETHTOOL_A_DEBUG_MSGMASK as usize + 1] = {
    let mut policy = [nla_policy { type_: 0 }; ETHTOOL_A_DEBUG_MSGMASK as usize + 1];
    policy[ETHTOOL_A_DEBUG_HEADER as usize] = NLA_POLICY_NESTED(ethnl_header_policy);
    policy[ETHTOOL_A_DEBUG_MSGMASK as usize] = nla_policy { type_: NLA_NESTED };
    policy
};

unsafe fn ethnl_set_debug_validate(
    req_info: *mut ethnl_req_info,
    _info: *mut genl_info,
) -> i32 {
    let ops = (*(*req_info).dev).ethtool_ops;

    if (*ops).get_msglevel.is_some() && (*ops).set_msglevel.is_some() {
        1
    } else {
        -EOPNOTSUPP
    }
}

unsafe fn ethnl_set_debug(
    req_info: *mut ethnl_req_info,
    info: *mut genl_info,
) -> i32 {
    let dev = (*req_info).dev;
    let tb = (*info).attrs;
    let mut mod_: bool = false;
    let mut msg_mask: u32;
    let ret: i32;

    msg_mask = ((*(*dev).ethtool_ops).get_msglevel.unwrap())(dev);
    ret = ethnl_update_bitset32(
        &mut msg_mask,
        NETIF_MSG_CLASS_COUNT,
        *tb.add(ETHTOOL_A_DEBUG_MSGMASK as usize),
        netif_msg_class_names,
        (*info).extack,
        &mut mod_,
    );
    if ret < 0 || !mod_ {
        return ret;
    }

    ((*(*dev).ethtool_ops).set_msglevel.unwrap())(dev, msg_mask);
    1
}

pub static ethnl_debug_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_DEBUG_GET,
    reply_cmd: ETHTOOL_MSG_DEBUG_GET_REPLY,
    hdr_attr: ETHTOOL_A_DEBUG_HEADER,
    req_info_size: core::mem::size_of::<debug_req_info>(),
    reply_data_size: core::mem::size_of::<debug_reply_data>(),
    prepare_data: Some(debug_prepare_data),
    reply_size: Some(debug_reply_size),
    fill_reply: Some(debug_fill_reply),
    set_validate: Some(ethnl_set_debug_validate),
    set: Some(ethnl_set_debug),
    set_ntf_cmd: ETHTOOL_MSG_DEBUG_NTF,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
