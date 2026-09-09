// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel/netlink translation.

#[repr(C)]
pub struct rings_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct rings_reply_data {
    pub base: ethnl_reply_data,
    pub ringparam: ethtool_ringparam,
    pub kernel_ringparam: kernel_ethtool_ringparam,
    pub supported_ring_params: u32,
}

#[inline]
unsafe fn rings_repdata(reply_base: *mut ethnl_reply_data) -> *mut rings_reply_data {
    (reply_base as *mut u8).sub(core::mem::offset_of!(rings_reply_data, base))
        as *mut rings_reply_data
}

unsafe extern "C" {
    pub static ethnl_header_policy: nla_policy;
    pub static ethnl_rings_get_policy: [nla_policy; ETHTOOL_A_RINGS_HEADER as usize + 1];
    pub static ethnl_rings_set_policy: [nla_policy; ETHTOOL_A_RINGS_HDS_THRESH as usize + 1];
}

#[no_mangle]
pub static mut ethnl_rings_get_policy_local: [nla_policy; ETHTOOL_A_RINGS_HEADER as usize + 1] =
    [nla_policy::default(); ETHTOOL_A_RINGS_HEADER as usize + 1];

unsafe fn rings_prepare_data(
    _req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    info: *const genl_info,
) -> i32 {
    let data = &mut *rings_repdata(reply_base);
    let dev = (*reply_base).dev;

    if (*(*dev).ethtool_ops).get_ringparam.is_none() {
        return -EOPNOTSUPP;
    }

    data.supported_ring_params = (*(*dev).ethtool_ops).supported_ring_params;
    let ret = ethnl_ops_begin(dev);
    if ret < 0 {
        return ret;
    }

    data.kernel_ringparam.tcp_data_split = (*dev).cfg.hds_config;
    data.kernel_ringparam.hds_thresh = (*dev).cfg.hds_thresh;

    ((*(*dev).ethtool_ops).get_ringparam.unwrap())(
        dev,
        &mut data.ringparam,
        &mut data.kernel_ringparam,
        (*info).extack,
    );
    ethnl_ops_complete(dev);
    0
}

unsafe fn rings_reply_size(
    _req_base: *const ethnl_req_info,
    _reply_base: *const ethnl_reply_data,
) -> i32 {
    nla_total_size(core::mem::size_of::<u32>()) as i32 * 15
        + nla_total_size(core::mem::size_of::<u8>()) as i32 * 3
}

unsafe fn rings_fill_reply(
    skb: *mut sk_buff,
    _req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> i32 {
    let data = &*rings_repdata(reply_base as *mut ethnl_reply_data);
    let kr = &data.kernel_ringparam;
    let ringparam = &data.ringparam;
    let supported_ring_params = data.supported_ring_params;

    WARN_ON(kr.tcp_data_split > ETHTOOL_TCP_DATA_SPLIT_ENABLED);

    if (ringparam.rx_max_pending != 0
        && (nla_put_u32(skb, ETHTOOL_A_RINGS_RX_MAX, ringparam.rx_max_pending) != 0
            || nla_put_u32(skb, ETHTOOL_A_RINGS_RX, ringparam.rx_pending) != 0))
        || (ringparam.rx_mini_max_pending != 0
            && (nla_put_u32(skb, ETHTOOL_A_RINGS_RX_MINI_MAX, ringparam.rx_mini_max_pending) != 0
                || nla_put_u32(skb, ETHTOOL_A_RINGS_RX_MINI, ringparam.rx_mini_pending) != 0))
        || (ringparam.rx_jumbo_max_pending != 0
            && (nla_put_u32(skb, ETHTOOL_A_RINGS_RX_JUMBO_MAX, ringparam.rx_jumbo_max_pending) != 0
                || nla_put_u32(skb, ETHTOOL_A_RINGS_RX_JUMBO, ringparam.rx_jumbo_pending) != 0))
        || (ringparam.tx_max_pending != 0
            && (nla_put_u32(skb, ETHTOOL_A_RINGS_TX_MAX, ringparam.tx_max_pending) != 0
                || nla_put_u32(skb, ETHTOOL_A_RINGS_TX, ringparam.tx_pending) != 0))
        || (kr.rx_buf_len != 0 && nla_put_u32(skb, ETHTOOL_A_RINGS_RX_BUF_LEN, kr.rx_buf_len) != 0)
        || (kr.tcp_data_split != 0
            && nla_put_u8(skb, ETHTOOL_A_RINGS_TCP_DATA_SPLIT, kr.tcp_data_split) != 0)
        || (kr.cqe_size != 0 && nla_put_u32(skb, ETHTOOL_A_RINGS_CQE_SIZE, kr.cqe_size) != 0)
        || nla_put_u8(skb, ETHTOOL_A_RINGS_TX_PUSH, (kr.tx_push != 0) as u8) != 0
        || nla_put_u8(skb, ETHTOOL_A_RINGS_RX_PUSH, (kr.rx_push != 0) as u8) != 0
        || (supported_ring_params & ETHTOOL_RING_USE_TX_PUSH_BUF_LEN != 0
            && (nla_put_u32(skb, ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN_MAX, kr.tx_push_buf_max_len) != 0
                || nla_put_u32(skb, ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN, kr.tx_push_buf_len) != 0))
        || (supported_ring_params & ETHTOOL_RING_USE_HDS_THRS != 0
            && (nla_put_u32(skb, ETHTOOL_A_RINGS_HDS_THRESH, kr.hds_thresh) != 0
                || nla_put_u32(skb, ETHTOOL_A_RINGS_HDS_THRESH_MAX, kr.hds_thresh_max) != 0))
    {
        return -EMSGSIZE;
    }
    0
}

// RINGS_SET

unsafe fn ethnl_set_rings_validate(req_info: *mut ethnl_req_info, info: *mut genl_info) -> i32 {
    let ops = (*(*req_info).dev).ethtool_ops;
    let tb = (*info).attrs;

    macro_rules! supported {
        ($attr:expr, $flag:expr, $msg:expr) => {
            if !(*tb.add($attr as usize)).is_null() && ((*ops).supported_ring_params & $flag) == 0 {
                NL_SET_ERR_MSG_ATTR((*info).extack, *tb.add($attr as usize), $msg);
                return -EOPNOTSUPP;
            }
        };
    }
    supported!(ETHTOOL_A_RINGS_RX_BUF_LEN, ETHTOOL_RING_USE_RX_BUF_LEN, "setting rx buf len not supported");
    supported!(ETHTOOL_A_RINGS_TCP_DATA_SPLIT, ETHTOOL_RING_USE_TCP_DATA_SPLIT, "setting TCP data split is not supported");
    supported!(ETHTOOL_A_RINGS_HDS_THRESH, ETHTOOL_RING_USE_HDS_THRS, "setting hds-thresh is not supported");
    supported!(ETHTOOL_A_RINGS_CQE_SIZE, ETHTOOL_RING_USE_CQE_SIZE, "setting cqe size not supported");
    supported!(ETHTOOL_A_RINGS_TX_PUSH, ETHTOOL_RING_USE_TX_PUSH, "setting tx push not supported");
    supported!(ETHTOOL_A_RINGS_RX_PUSH, ETHTOOL_RING_USE_RX_PUSH, "setting rx push not supported");
    supported!(ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN, ETHTOOL_RING_USE_TX_PUSH_BUF_LEN, "setting tx push buf len is not supported");

    if (*ops).get_ringparam.is_some() && (*ops).set_ringparam.is_some() { 1 } else { -EOPNOTSUPP }
}

unsafe fn ethnl_set_rings(req_info: *mut ethnl_req_info, info: *mut genl_info) -> i32 {
    let dev = (*req_info).dev;
    let tb = (*info).attrs;
    let mut kernel_ringparam = core::mem::zeroed::<kernel_ethtool_ringparam>();
    let mut ringparam = core::mem::zeroed::<ethtool_ringparam>();
    let mut modified = false;

    ethtool_ringparam_get_cfg(dev, &mut ringparam, &mut kernel_ringparam, (*info).extack);
    ethnl_update_u32(&mut ringparam.rx_pending, *tb.add(ETHTOOL_A_RINGS_RX as usize), &mut modified);
    ethnl_update_u32(&mut ringparam.rx_mini_pending, *tb.add(ETHTOOL_A_RINGS_RX_MINI as usize), &mut modified);
    ethnl_update_u32(&mut ringparam.rx_jumbo_pending, *tb.add(ETHTOOL_A_RINGS_RX_JUMBO as usize), &mut modified);
    ethnl_update_u32(&mut ringparam.tx_pending, *tb.add(ETHTOOL_A_RINGS_TX as usize), &mut modified);
    ethnl_update_u32(&mut kernel_ringparam.rx_buf_len, *tb.add(ETHTOOL_A_RINGS_RX_BUF_LEN as usize), &mut modified);
    ethnl_update_u8(&mut kernel_ringparam.tcp_data_split, *tb.add(ETHTOOL_A_RINGS_TCP_DATA_SPLIT as usize), &mut modified);
    ethnl_update_u32(&mut kernel_ringparam.cqe_size, *tb.add(ETHTOOL_A_RINGS_CQE_SIZE as usize), &mut modified);
    ethnl_update_u8(&mut kernel_ringparam.tx_push, *tb.add(ETHTOOL_A_RINGS_TX_PUSH as usize), &mut modified);
    ethnl_update_u8(&mut kernel_ringparam.rx_push, *tb.add(ETHTOOL_A_RINGS_RX_PUSH as usize), &mut modified);
    ethnl_update_u32(&mut kernel_ringparam.tx_push_buf_len, *tb.add(ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN as usize), &mut modified);
    ethnl_update_u32(&mut kernel_ringparam.hds_thresh, *tb.add(ETHTOOL_A_RINGS_HDS_THRESH as usize), &mut modified);
    if !modified { return 0; }

    if kernel_ringparam.tcp_data_split == ETHTOOL_TCP_DATA_SPLIT_ENABLED && dev_xdp_sb_prog_count(dev) {
        NL_SET_ERR_MSG_ATTR((*info).extack, *tb.add(ETHTOOL_A_RINGS_TCP_DATA_SPLIT as usize), "tcp-data-split can not be enabled with single buffer XDP");
        return -EINVAL;
    }
    if dev_get_min_mp_channel_count(dev) != 0 {
        if kernel_ringparam.tcp_data_split != ETHTOOL_TCP_DATA_SPLIT_ENABLED {
            NL_SET_ERR_MSG((*info).extack, "can't disable tcp-data-split while device has memory provider enabled");
            return -EINVAL;
        } else if kernel_ringparam.hds_thresh != 0 {
            NL_SET_ERR_MSG((*info).extack, "can't set non-zero hds_thresh while device is memory provider enabled");
            return -EINVAL;
        }
    }

    let err_attr = if ringparam.rx_pending > ringparam.rx_max_pending { *tb.add(ETHTOOL_A_RINGS_RX as usize) }
        else if ringparam.rx_mini_pending > ringparam.rx_mini_max_pending { *tb.add(ETHTOOL_A_RINGS_RX_MINI as usize) }
        else if ringparam.rx_jumbo_pending > ringparam.rx_jumbo_max_pending { *tb.add(ETHTOOL_A_RINGS_RX_JUMBO as usize) }
        else if ringparam.tx_pending > ringparam.tx_max_pending { *tb.add(ETHTOOL_A_RINGS_TX as usize) }
        else if kernel_ringparam.hds_thresh > kernel_ringparam.hds_thresh_max { *tb.add(ETHTOOL_A_RINGS_HDS_THRESH as usize) }
        else { core::ptr::null() };
    if !err_attr.is_null() { NL_SET_ERR_MSG_ATTR((*info).extack, err_attr, "requested ring size exceeds maximum"); return -EINVAL; }
    if kernel_ringparam.tx_push_buf_len > kernel_ringparam.tx_push_buf_max_len {
        NL_SET_ERR_MSG_ATTR_FMT((*info).extack, *tb.add(ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN as usize), "Requested TX push buffer exceeds the maximum of %u", kernel_ringparam.tx_push_buf_max_len);
        return -EINVAL;
    }
    (*dev).cfg_pending.hds_config = kernel_ringparam.tcp_data_split;
    (*dev).cfg_pending.hds_thresh = kernel_ringparam.hds_thresh;
    let ret = ((*(*dev).ethtool_ops).set_ringparam.unwrap())(dev, &mut ringparam, &mut kernel_ringparam, (*info).extack);
    if ret < 0 { ret } else { 1 }
}

#[no_mangle]
pub static ethnl_rings_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_RINGS_GET,
    reply_cmd: ETHTOOL_MSG_RINGS_GET_REPLY,
    hdr_attr: ETHTOOL_A_RINGS_HEADER,
    req_info_size: core::mem::size_of::<rings_req_info>(),
    reply_data_size: core::mem::size_of::<rings_reply_data>(),
    prepare_data: Some(rings_prepare_data),
    reply_size: Some(rings_reply_size),
    fill_reply: Some(rings_fill_reply),
    set_validate: Some(ethnl_set_rings_validate),
    set: Some(ethnl_set_rings),
    set_ntf_cmd: ETHTOOL_MSG_RINGS_NTF,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
