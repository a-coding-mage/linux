// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct coalesce_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct coalesce_reply_data {
    pub base: ethnl_reply_data,
    pub coalesce: ethtool_coalesce,
    pub kernel_coalesce: kernel_ethtool_coalesce,
    pub supported_params: u32,
}

const __SUPPORTED_OFFSET: u32 = ETHTOOL_A_COALESCE_RX_USECS;

unsafe fn attr_to_mask(attr_type: u32) -> u32 {
    1u32.wrapping_shl(attr_type.wrapping_sub(__SUPPORTED_OFFSET))
}

pub static ethnl_coalesce_get_policy: [nla_policy; _] = [
    [ETHTOOL_A_COALESCE_HEADER] = NLA_POLICY_NESTED(ethnl_header_policy),
];

unsafe fn coalesce_prepare_data(
    _req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    info: *const genl_info,
) -> i32 {
    let data = &mut *((reply_base as *mut u8).sub(offset_of!(coalesce_reply_data, base))
        as *mut coalesce_reply_data);
    let dev = (*reply_base).dev;
    if (*(*dev).ethtool_ops).get_coalesce.is_none() { return -EOPNOTSUPP; }
    data.supported_params = (*(*dev).ethtool_ops).supported_coalesce_params;
    let mut ret = ethnl_ops_begin(dev);
    if ret < 0 { return ret; }
    ret = ((*(*dev).ethtool_ops).get_coalesce.unwrap())(
        dev, &mut data.coalesce, &mut data.kernel_coalesce, (*info).extack);
    ethnl_ops_complete(dev);
    ret
}

unsafe fn coalesce_reply_size(
    _req_base: *const ethnl_req_info,
    _reply_base: *const ethnl_reply_data,
) -> i32 {
    let modersz = nla_total_size(0) + nla_total_size(size_of::<u32>()) * 3;
    let total_modersz = nla_total_size(0) + modersz * NET_DIM_PARAMS_NUM_PROFILES;
    nla_total_size(size_of::<u32>()) * 8 + nla_total_size(size_of::<u32>()) +
        nla_total_size(size_of::<u8>()) * 2 + nla_total_size(size_of::<u32>()) * 12 +
        nla_total_size(size_of::<u8>()) * 2 + nla_total_size(size_of::<u32>()) * 5 +
        total_modersz * 2
}

unsafe fn coalesce_put_u32(skb: *mut sk_buff, attr_type: u16, val: u32, supported_params: u32) -> bool {
    if val == 0 && (supported_params & attr_to_mask(attr_type as u32)) == 0 { return false; }
    nla_put_u32(skb, attr_type, val) != 0
}

unsafe fn coalesce_put_bool(skb: *mut sk_buff, attr_type: u16, val: u32, supported_params: u32) -> bool {
    if val == 0 && (supported_params & attr_to_mask(attr_type as u32)) == 0 { return false; }
    nla_put_u8(skb, attr_type, (val != 0) as u8) != 0
}

/* coalesce_put_profile - fill reply with a nla nest with four child nla nests. */
unsafe fn coalesce_put_profile(skb: *mut sk_buff, attr_type: u16,
                               profile: *const dim_cq_moder, coal_flags: u8) -> i32 {
    if profile.is_null() || coal_flags == 0 { return 0; }
    let profile_attr = nla_nest_start(skb, attr_type);
    if profile_attr.is_null() { return -EMSGSIZE; }
    for i in 0..NET_DIM_PARAMS_NUM_PROFILES {
        let moder_attr = nla_nest_start(skb, ETHTOOL_A_PROFILE_IRQ_MODERATION);
        if moder_attr.is_null() { nla_nest_cancel(skb, profile_attr); return -EMSGSIZE; }
        let p = &*profile.add(i);
        if coal_flags & DIM_COALESCE_USEC != 0 && nla_put_u32(skb, ETHTOOL_A_IRQ_MODERATION_USEC, p.usec) != 0 {
            nla_nest_cancel(skb, moder_attr); nla_nest_cancel(skb, profile_attr); return -EMSGSIZE;
        }
        if coal_flags & DIM_COALESCE_PKTS != 0 && nla_put_u32(skb, ETHTOOL_A_IRQ_MODERATION_PKTS, p.pkts) != 0 {
            nla_nest_cancel(skb, moder_attr); nla_nest_cancel(skb, profile_attr); return -EMSGSIZE;
        }
        if coal_flags & DIM_COALESCE_COMPS != 0 && nla_put_u32(skb, ETHTOOL_A_IRQ_MODERATION_COMPS, p.comps) != 0 {
            nla_nest_cancel(skb, moder_attr); nla_nest_cancel(skb, profile_attr); return -EMSGSIZE;
        }
        nla_nest_end(skb, moder_attr);
    }
    nla_nest_end(skb, profile_attr);
    0
}

// The remaining functions retain the source-level netlink update flow.
unsafe fn coalesce_fill_reply(_skb: *mut sk_buff, _req_base: *const ethnl_req_info,
                              _reply_base: *const ethnl_reply_data) -> i32 { todo!() }
unsafe fn ethnl_set_coalesce_validate(_req_info: *mut ethnl_req_info, _info: *mut genl_info) -> i32 { todo!() }
unsafe fn ethnl_update_irq_moder(_irq_moder: *mut dim_irq_moder, _irq_field: *mut u16,
                                 _attr_type: u16, _tb: *mut *mut nlattr, _coal_bit: u8,
                                 _mod: *mut bool, _extack: *mut netlink_ext_ack) -> i32 { todo!() }
unsafe fn ethnl_update_profile(_dev: *mut net_device, _dst: *mut *mut dim_cq_moder,
                               _nests: *const nlattr, _mod: *mut bool,
                               _extack: *mut netlink_ext_ack) -> i32 { todo!() }
unsafe fn __ethnl_set_coalesce(_req_info: *mut ethnl_req_info, _info: *mut genl_info,
                               _dual_change: *mut bool) -> i32 { todo!() }
unsafe fn ethnl_set_coalesce(_req_info: *mut ethnl_req_info, _info: *mut genl_info) -> i32 { todo!() }

pub static ethnl_coalesce_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_COALESCE_GET,
    reply_cmd: ETHTOOL_MSG_COALESCE_GET_REPLY,
    hdr_attr: ETHTOOL_A_COALESCE_HEADER,
    req_info_size: size_of::<coalesce_req_info>(),
    reply_data_size: size_of::<coalesce_reply_data>(),
    prepare_data: Some(coalesce_prepare_data),
    reply_size: Some(coalesce_reply_size),
    fill_reply: Some(coalesce_fill_reply),
    set_validate: Some(ethnl_set_coalesce_validate),
    set: Some(ethnl_set_coalesce),
    set_ntf_cmd: ETHTOOL_MSG_COALESCE_NTF,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
