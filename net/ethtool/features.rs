// SPDX-License-Identifier: GPL-2.0-only

// External kernel and ethtool declarations are supplied by other translation units.

#[repr(C)]
pub struct features_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct features_reply_data {
    pub base: ethnl_reply_data,
    pub hw: [u32; ETHTOOL_DEV_FEATURE_WORDS],
    pub wanted: [u32; ETHTOOL_DEV_FEATURE_WORDS],
    pub active: [u32; ETHTOOL_DEV_FEATURE_WORDS],
    pub nochange: [u32; ETHTOOL_DEV_FEATURE_WORDS],
    pub all: [u32; ETHTOOL_DEV_FEATURE_WORDS],
}

pub static ethnl_features_get_policy: [nla_policy; ETHTOOL_A_FEATURES_HEADER + 1] =
    [nla_policy { type_: NLA_POLICY_NESTED_TYPE };
        ETHTOOL_A_FEATURES_HEADER + 1];

unsafe fn ethnl_features_to_bitmap32(dest: *mut u32, src: netdev_features_t) {
    let mut i: usize = 0;
    while i < ETHTOOL_DEV_FEATURE_WORDS {
        *dest.add(i) = (src >> (32 * i)) as u32;
        i += 1;
    }
}

unsafe fn features_prepare_data(
    _req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    _info: *const genl_info,
) -> i32 {
    let data = &mut *(reply_base as *mut features_reply_data);
    let dev = (*reply_base).dev;
    let all_features: netdev_features_t = (!0u64) >> (64 - NETDEV_FEATURE_COUNT);

    ethnl_features_to_bitmap32(data.hw.as_mut_ptr(), (*dev).hw_features);
    ethnl_features_to_bitmap32(data.wanted.as_mut_ptr(), (*dev).wanted_features);
    ethnl_features_to_bitmap32(data.active.as_mut_ptr(), (*dev).features);
    ethnl_features_to_bitmap32(data.nochange.as_mut_ptr(), NETIF_F_NEVER_CHANGE);
    ethnl_features_to_bitmap32(data.all.as_mut_ptr(), all_features);
    0
}

unsafe fn features_reply_size(
    req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> i32 {
    let data = &*(reply_base as *const features_reply_data);
    let compact = ((*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS) != 0;
    let mut len: u32 = 0;
    let mut ret: i32;

    ret = ethnl_bitset32_size(data.hw.as_ptr(), data.all.as_ptr(), NETDEV_FEATURE_COUNT,
                              netdev_features_strings, compact);
    if ret < 0 { return ret; }
    len = len.wrapping_add(ret as u32);
    ret = ethnl_bitset32_size(data.wanted.as_ptr(), core::ptr::null(), NETDEV_FEATURE_COUNT,
                              netdev_features_strings, compact);
    if ret < 0 { return ret; }
    len = len.wrapping_add(ret as u32);
    ret = ethnl_bitset32_size(data.active.as_ptr(), core::ptr::null(), NETDEV_FEATURE_COUNT,
                              netdev_features_strings, compact);
    if ret < 0 { return ret; }
    len = len.wrapping_add(ret as u32);
    ret = ethnl_bitset32_size(data.nochange.as_ptr(), core::ptr::null(), NETDEV_FEATURE_COUNT,
                              netdev_features_strings, compact);
    if ret < 0 { return ret; }
    len = len.wrapping_add(ret as u32);
    len as i32
}

unsafe fn features_fill_reply(
    skb: *mut sk_buff,
    req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> i32 {
    let data = &*(reply_base as *const features_reply_data);
    let compact = ((*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS) != 0;
    let mut ret = ethnl_put_bitset32(skb, ETHTOOL_A_FEATURES_HW, data.hw.as_ptr(), data.all.as_ptr(),
                                     NETDEV_FEATURE_COUNT, netdev_features_strings, compact);
    if ret < 0 { return ret; }
    ret = ethnl_put_bitset32(skb, ETHTOOL_A_FEATURES_WANTED, data.wanted.as_ptr(), core::ptr::null(),
                             NETDEV_FEATURE_COUNT, netdev_features_strings, compact);
    if ret < 0 { return ret; }
    ret = ethnl_put_bitset32(skb, ETHTOOL_A_FEATURES_ACTIVE, data.active.as_ptr(), core::ptr::null(),
                             NETDEV_FEATURE_COUNT, netdev_features_strings, compact);
    if ret < 0 { return ret; }
    ethnl_put_bitset32(skb, ETHTOOL_A_FEATURES_NOCHANGE, data.nochange.as_ptr(), core::ptr::null(),
                       NETDEV_FEATURE_COUNT, netdev_features_strings, compact)
}

pub static ethnl_features_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_FEATURES_GET,
    reply_cmd: ETHTOOL_MSG_FEATURES_GET_REPLY,
    hdr_attr: ETHTOOL_A_FEATURES_HEADER,
    req_info_size: core::mem::size_of::<features_req_info>(),
    reply_data_size: core::mem::size_of::<features_reply_data>(),
    prepare_data: Some(features_prepare_data),
    reply_size: Some(features_reply_size),
    fill_reply: Some(features_fill_reply),
};

pub static ethnl_features_set_policy: [nla_policy; ETHTOOL_A_FEATURES_WANTED + 1] =
    [nla_policy { type_: 0 }; ETHTOOL_A_FEATURES_WANTED + 1];

unsafe fn ethnl_features_to_bitmap(dest: *mut libc::c_ulong, val: netdev_features_t) {
    let words = BITS_TO_LONGS(NETDEV_FEATURE_COUNT);
    let mut i = 0;
    while i < words {
        *dest.add(i) = (val >> (i * BITS_PER_LONG)) as libc::c_ulong;
        i += 1;
    }
}

unsafe fn ethnl_bitmap_to_features(src: *const libc::c_ulong) -> netdev_features_t {
    let nft_bits = core::mem::size_of::<netdev_features_t>() * BITS_PER_BYTE;
    let words = BITS_TO_LONGS(NETDEV_FEATURE_COUNT);
    let mut ret: netdev_features_t = 0;
    let mut i = 0;
    while i < words {
        ret |= (*src.add(i) as netdev_features_t) << (i * BITS_PER_LONG);
        i += 1;
    }
    ret &= (!0 as netdev_features_t) >> (nft_bits - NETDEV_FEATURE_COUNT);
    ret
}

unsafe fn features_send_reply(
    dev: *mut net_device,
    info: *mut genl_info,
    wanted: *const libc::c_ulong,
    wanted_mask: *const libc::c_ulong,
    active: *const libc::c_ulong,
    active_mask: *const libc::c_ulong,
    compact: bool,
) -> i32 {
    let mut reply_payload: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut reply_len = ethnl_reply_header_size();
    let mut ret = ethnl_bitset_size(wanted, wanted_mask, NETDEV_FEATURE_COUNT,
                                    netdev_features_strings, compact);
    if ret < 0 { GENL_SET_ERR_MSG(info, "failed to send reply message"); return ret; }
    reply_len += ret;
    ret = ethnl_bitset_size(active, active_mask, NETDEV_FEATURE_COUNT,
                            netdev_features_strings, compact);
    if ret < 0 { GENL_SET_ERR_MSG(info, "failed to send reply message"); return ret; }
    reply_len += ret;

    let rskb = ethnl_reply_init(reply_len, dev, ETHTOOL_MSG_FEATURES_SET_REPLY,
                                ETHTOOL_A_FEATURES_HEADER, info, &mut reply_payload);
    if rskb.is_null() {
        GENL_SET_ERR_MSG(info, "failed to send reply message");
        return -ENOMEM;
    }
    ret = ethnl_put_bitset(rskb, ETHTOOL_A_FEATURES_WANTED, wanted, wanted_mask,
                           NETDEV_FEATURE_COUNT, netdev_features_strings, compact);
    if ret < 0 { nlmsg_free(rskb); WARN_ONCE(true, "calculated message payload length (%d) not sufficient\n", reply_len); GENL_SET_ERR_MSG(info, "failed to send reply message"); return ret; }
    ret = ethnl_put_bitset(rskb, ETHTOOL_A_FEATURES_ACTIVE, active, active_mask,
                           NETDEV_FEATURE_COUNT, netdev_features_strings, compact);
    if ret < 0 { nlmsg_free(rskb); WARN_ONCE(true, "calculated message payload length (%d) not sufficient\n", reply_len); GENL_SET_ERR_MSG(info, "failed to send reply message"); return ret; }
    genlmsg_end(rskb, reply_payload);
    genlmsg_reply(rskb, info)
}

// The remaining SET implementation retains C bitmap operations and kernel control flow.
pub unsafe fn ethnl_set_features(skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let mut wanted_diff_mask = [0 as libc::c_ulong; BITS_TO_LONGS(NETDEV_FEATURE_COUNT)];
    let mut active_diff_mask = [0 as libc::c_ulong; BITS_TO_LONGS(NETDEV_FEATURE_COUNT)];
    let mut old_active = [0 as libc::c_ulong; BITS_TO_LONGS(NETDEV_FEATURE_COUNT)];
    let mut old_wanted = [0 as libc::c_ulong; BITS_TO_LONGS(NETDEV_FEATURE_COUNT)];
    let mut new_active = [0 as libc::c_ulong; BITS_TO_LONGS(NETDEV_FEATURE_COUNT)];
    let mut new_wanted = [0 as libc::c_ulong; BITS_TO_LONGS(NETDEV_FEATURE_COUNT)];
    let mut req_wanted = [0 as libc::c_ulong; BITS_TO_LONGS(NETDEV_FEATURE_COUNT)];
    let mut req_mask = [0 as libc::c_ulong; BITS_TO_LONGS(NETDEV_FEATURE_COUNT)];
    let mut req_info: ethnl_req_info = core::mem::zeroed();
    let tb = (*info).attrs;
    if (*tb.add(ETHTOOL_A_FEATURES_WANTED)).is_null() { return -EINVAL; }
    let mut ret = ethnl_parse_header_dev_get(&mut req_info, *tb.add(ETHTOOL_A_FEATURES_HEADER),
                                             genl_info_net(info), (*info).extack, true);
    if ret < 0 { return ret; }
    let dev = req_info.dev;
    rtnl_lock();
    netdev_lock_ops(dev);
    ret = ethnl_ops_begin(dev);
    if ret < 0 { netdev_unlock_ops(dev); rtnl_unlock(); ethnl_parse_header_dev_put(&mut req_info); return ret; }
    ethnl_features_to_bitmap(old_active.as_mut_ptr(), (*dev).features);
    ethnl_features_to_bitmap(old_wanted.as_mut_ptr(), (*dev).wanted_features);
    ret = ethnl_parse_bitset(req_wanted.as_mut_ptr(), req_mask.as_mut_ptr(), NETDEV_FEATURE_COUNT,
                             *tb.add(ETHTOOL_A_FEATURES_WANTED), netdev_features_strings, (*info).extack);
    if ret >= 0 && (ethnl_bitmap_to_features(req_mask.as_ptr()) & !NETIF_F_ETHTOOL_BITS) != 0 {
        GENL_SET_ERR_MSG(info, "attempt to change non-ethtool features"); ret = -EINVAL;
    }
    if ret >= 0 {
        bitmap_and(req_wanted.as_mut_ptr(), req_wanted.as_ptr(), req_mask.as_ptr(), NETDEV_FEATURE_COUNT);
        bitmap_andnot(new_wanted.as_mut_ptr(), old_wanted.as_ptr(), req_mask.as_ptr(), NETDEV_FEATURE_COUNT);
        bitmap_or(req_wanted.as_mut_ptr(), new_wanted.as_ptr(), req_wanted.as_ptr(), NETDEV_FEATURE_COUNT);
        if !bitmap_equal(req_wanted.as_ptr(), old_wanted.as_ptr(), NETDEV_FEATURE_COUNT) {
            (*dev).wanted_features &= !(*dev).hw_features;
            (*dev).wanted_features |= ethnl_bitmap_to_features(req_wanted.as_ptr()) & (*dev).hw_features;
            __netdev_update_features(dev);
        }
        ethnl_features_to_bitmap(new_active.as_mut_ptr(), (*dev).features);
        let modified = !bitmap_equal(old_active.as_ptr(), new_active.as_ptr(), NETDEV_FEATURE_COUNT);
        if ((*req_info.flags & ETHTOOL_FLAG_OMIT_REPLY) == 0) {
            let compact = (req_info.flags & ETHTOOL_FLAG_COMPACT_BITSETS) != 0;
            bitmap_xor(wanted_diff_mask.as_mut_ptr(), req_wanted.as_ptr(), new_active.as_ptr(), NETDEV_FEATURE_COUNT);
            bitmap_xor(active_diff_mask.as_mut_ptr(), old_active.as_ptr(), new_active.as_ptr(), NETDEV_FEATURE_COUNT);
            bitmap_and(wanted_diff_mask.as_mut_ptr(), wanted_diff_mask.as_ptr(), req_mask.as_ptr(), NETDEV_FEATURE_COUNT);
            bitmap_and(req_wanted.as_mut_ptr(), req_wanted.as_ptr(), wanted_diff_mask.as_ptr(), NETDEV_FEATURE_COUNT);
            bitmap_and(new_active.as_mut_ptr(), new_active.as_ptr(), active_diff_mask.as_ptr(), NETDEV_FEATURE_COUNT);
            ret = features_send_reply(dev, info, req_wanted.as_ptr(), wanted_diff_mask.as_ptr(), new_active.as_ptr(), active_diff_mask.as_ptr(), compact);
        }
        if modified { netdev_features_change(dev); }
    }
    ethnl_ops_complete(dev);
    netdev_unlock_ops(dev);
    rtnl_unlock();
    ethnl_parse_header_dev_put(&mut req_info);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
