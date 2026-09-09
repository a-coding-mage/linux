// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2021 NXP
 */
// Dependencies are supplied by the surrounding ethtool/kernel translation.

#[repr(C)]
pub struct phc_vclocks_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct phc_vclocks_reply_data {
    pub base: ethnl_reply_data,
    pub num: ::core::ffi::c_int,
    pub index: *mut ::core::ffi::c_int,
}

// Equivalent of PHC_VCLOCKS_REPDATA: container_of(__reply_base, ... , base).
#[inline]
unsafe fn phc_vclocks_repdata<'a>(reply_base: *const ethnl_reply_data) -> &'a phc_vclocks_reply_data {
    &*((reply_base as *const u8).sub(core::mem::offset_of!(phc_vclocks_reply_data, base))
        as *const phc_vclocks_reply_data)
}

pub static ethnl_phc_vclocks_get_policy: [nla_policy; ETHTOOL_A_PHC_VCLOCKS_HEADER + 1] = [
    // [ETHTOOL_A_PHC_VCLOCKS_HEADER] = NLA_POLICY_NESTED(ethnl_header_policy)
];

unsafe fn phc_vclocks_prepare_data(
    _req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    _info: *const genl_info,
) -> ::core::ffi::c_int {
    let data = phc_vclocks_repdata(reply_base);
    let dev = (*reply_base).dev;
    let mut ret: ::core::ffi::c_int;

    ret = ethnl_ops_begin(dev);
    if ret < 0 {
        return ret;
    }
    (*data).num = ethtool_get_phc_vclocks(dev, &mut (*data).index);
    ethnl_ops_complete(dev);

    ret
}

unsafe fn phc_vclocks_reply_size(
    _req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> ::core::ffi::c_int {
    let data = phc_vclocks_repdata(reply_base);
    let mut len: ::core::ffi::c_int = 0;

    if (*data).num > 0 {
        len += nla_total_size(core::mem::size_of::<u32>());
        len += nla_total_size(core::mem::size_of::<i32>() * (*data).num as usize);
    }

    len
}

unsafe fn phc_vclocks_fill_reply(
    skb: *mut sk_buff,
    _req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> ::core::ffi::c_int {
    let data = phc_vclocks_repdata(reply_base);

    if (*data).num <= 0 {
        return 0;
    }

    if nla_put_u32(skb, ETHTOOL_A_PHC_VCLOCKS_NUM, (*data).num as u32) != 0
        || nla_put(
            skb,
            ETHTOOL_A_PHC_VCLOCKS_INDEX,
            core::mem::size_of::<i32>() * (*data).num as usize,
            (*data).index as *const ::core::ffi::c_void,
        ) != 0
    {
        return -EMSGSIZE;
    }

    0
}

unsafe fn phc_vclocks_cleanup_data(reply_base: *mut ethnl_reply_data) {
    let data = phc_vclocks_repdata(reply_base);
    kfree((*data).index as *mut ::core::ffi::c_void);
}

pub static ethnl_phc_vclocks_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_PHC_VCLOCKS_GET,
    reply_cmd: ETHTOOL_MSG_PHC_VCLOCKS_GET_REPLY,
    hdr_attr: ETHTOOL_A_PHC_VCLOCKS_HEADER,
    req_info_size: core::mem::size_of::<phc_vclocks_req_info>(),
    reply_data_size: core::mem::size_of::<phc_vclocks_reply_data>(),
    prepare_data: Some(phc_vclocks_prepare_data),
    reply_size: Some(phc_vclocks_reply_size),
    fill_reply: Some(phc_vclocks_fill_reply),
    cleanup_data: Some(phc_vclocks_cleanup_data),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
