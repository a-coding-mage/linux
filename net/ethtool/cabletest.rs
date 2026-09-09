// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel/ethtool translation.

/* 802.3 standard allows 100 meters for BaseT cables. However longer
 * cables might work, depending on the quality of the cables and the
 * PHY. So allow testing for up to 150 meters.
 */
const MAX_CABLE_LENGTH_CM: u32 = 150 * 100;

pub static ethnl_cable_test_act_policy: [nla_policy; 1] = [
    nla_policy_nested!(ETHTOOL_A_CABLE_TEST_HEADER, ethnl_header_policy_phy),
];

unsafe fn ethnl_cable_test_started(phydev: *mut phy_device, cmd: u8) -> c_int {
    let mut skb: *mut sk_buff;
    let mut err: c_int = -ENOMEM;
    let mut ehdr: *mut c_void;

    skb = genlmsg_new(NLMSG_GOOD, GFP_KERNEL);
    if skb.is_null() { goto!(out); }

    ehdr = ethnl_bcastmsg_put(skb, cmd);
    if ehdr.is_null() {
        err = -EMSGSIZE;
        goto!(out);
    }

    err = ethnl_fill_reply_header((*phydev).attached_dev,
                                  skb, ETHTOOL_A_CABLE_TEST_NTF_HEADER);
    if err != 0 { goto!(out); }

    err = nla_put_u8(skb, ETHTOOL_A_CABLE_TEST_NTF_STATUS,
                     ETHTOOL_A_CABLE_TEST_NTF_STATUS_STARTED);
    if err != 0 { goto!(out); }

    genlmsg_end(skb, ehdr);
    return ethnl_multicast(skb, (*phydev).attached_dev);

out:
    nlmsg_free(skb);
    phydev_err(phydev, "%s: Error %pe\n", __func__, ERR_PTR(err));
    err
}

pub unsafe fn ethnl_act_cable_test(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let mut req_info: ethnl_req_info = core::mem::zeroed();
    let ops: *const ethtool_phy_ops;
    let tb = (*info).attrs;
    let phydev: *mut phy_device;
    let dev: *mut net_device;
    let mut ret: c_int;

    ret = ethnl_parse_header_dev_get(&mut req_info, *tb.add(ETHTOOL_A_CABLE_TEST_HEADER),
                                     genl_info_net(info), (*info).extack, true);
    if ret < 0 { return ret; }
    dev = req_info.dev;
    netdev_lock_ops_compat(dev);
    phydev = ethnl_req_get_phydev(&mut req_info, tb, ETHTOOL_A_CABLE_TEST_HEADER,
                                  (*info).extack);
    if IS_ERR_OR_NULL!(phydev) { ret = -EOPNOTSUPP; goto!(out_unlock); }
    ops = ethtool_phy_ops;
    if ops.is_null() || (*ops).start_cable_test.is_none() { ret = -EOPNOTSUPP; goto!(out_unlock); }
    ret = ethnl_ops_begin(dev);
    if ret < 0 { goto!(out_unlock); }
    ret = ((*ops).start_cable_test.unwrap())(phydev, (*info).extack);
    ethnl_ops_complete(dev);
    if ret == 0 { ethnl_cable_test_started(phydev, ETHTOOL_MSG_CABLE_TEST_NTF); }
out_unlock:
    netdev_unlock_ops_compat(dev);
    ethnl_parse_header_dev_put(&mut req_info);
    ret
}

pub unsafe fn ethnl_cable_test_alloc(phydev: *mut phy_device, cmd: u8) -> c_int {
    let mut err: c_int = -ENOMEM;
    (*phydev).skb = genlmsg_new(SZ_16K, GFP_KERNEL);
    if (*phydev).skb.is_null() { goto!(out); }
    (*phydev).ehdr = ethnl_bcastmsg_put((*phydev).skb, cmd);
    if (*phydev).ehdr.is_null() { err = -EMSGSIZE; goto!(out); }
    err = ethnl_fill_reply_header((*phydev).attached_dev, (*phydev).skb,
                                  ETHTOOL_A_CABLE_TEST_NTF_HEADER);
    if err != 0 { goto!(out); }
    err = nla_put_u8((*phydev).skb, ETHTOOL_A_CABLE_TEST_NTF_STATUS,
                     ETHTOOL_A_CABLE_TEST_NTF_STATUS_COMPLETED);
    if err != 0 { goto!(out); }
    (*phydev).nest = nla_nest_start((*phydev).skb, ETHTOOL_A_CABLE_TEST_NTF_NEST);
    if (*phydev).nest.is_null() { err = -EMSGSIZE; goto!(out); }
    0
out:
    nlmsg_free((*phydev).skb);
    (*phydev).skb = core::ptr::null_mut();
    err
}

pub unsafe fn ethnl_cable_test_free(phydev: *mut phy_device) {
    nlmsg_free((*phydev).skb);
    (*phydev).skb = core::ptr::null_mut();
}

pub unsafe fn ethnl_cable_test_finished(phydev: *mut phy_device) {
    nla_nest_end((*phydev).skb, (*phydev).nest);
    genlmsg_end((*phydev).skb, (*phydev).ehdr);
    ethnl_multicast((*phydev).skb, (*phydev).attached_dev);
}

pub unsafe fn ethnl_cable_test_result_with_src(phydev: *mut phy_device, pair: u8, result: u8, src: u32) -> c_int {
    let nest = nla_nest_start((*phydev).skb, ETHTOOL_A_CABLE_NEST_RESULT);
    if nest.is_null() { return -EMSGSIZE; }
    if nla_put_u8((*phydev).skb, ETHTOOL_A_CABLE_RESULT_PAIR, pair) != 0 { goto_cancel!(nest); }
    if nla_put_u8((*phydev).skb, ETHTOOL_A_CABLE_RESULT_CODE, result) != 0 { goto_cancel!(nest); }
    if src != ETHTOOL_A_CABLE_INF_SRC_UNSPEC && nla_put_u32((*phydev).skb, ETHTOOL_A_CABLE_RESULT_SRC, src) != 0 { goto_cancel!(nest); }
    nla_nest_end((*phydev).skb, nest); 0
}

pub unsafe fn ethnl_cable_test_fault_length_with_src(phydev: *mut phy_device, pair: u8, cm: u32, src: u32) -> c_int {
    let nest = nla_nest_start((*phydev).skb, ETHTOOL_A_CABLE_NEST_FAULT_LENGTH);
    if nest.is_null() { return -EMSGSIZE; }
    if nla_put_u8((*phydev).skb, ETHTOOL_A_CABLE_FAULT_LENGTH_PAIR, pair) != 0 { goto_cancel!(nest); }
    if nla_put_u32((*phydev).skb, ETHTOOL_A_CABLE_FAULT_LENGTH_CM, cm) != 0 { goto_cancel!(nest); }
    if src != ETHTOOL_A_CABLE_INF_SRC_UNSPEC && nla_put_u32((*phydev).skb, ETHTOOL_A_CABLE_FAULT_LENGTH_SRC, src) != 0 { goto_cancel!(nest); }
    nla_nest_end((*phydev).skb, nest); 0
}

pub static cable_test_tdr_act_cfg_policy: [nla_policy; 4] = [
    nla_policy_type!(NLA_U32), nla_policy_type!(NLA_U32),
    nla_policy_type!(NLA_U32), nla_policy_type!(NLA_U8),
];

pub static ethnl_cable_test_tdr_act_policy: [nla_policy; 2] = [
    nla_policy_nested!(ETHTOOL_A_CABLE_TEST_TDR_HEADER, ethnl_header_policy_phy),
    nla_policy_type!(NLA_NESTED),
];

unsafe fn ethnl_act_cable_test_tdr_cfg(nest: *const nlattr, info: *mut genl_info, cfg: *mut phy_tdr_config) -> c_int {
    (*cfg).first = 100; (*cfg).step = 100; (*cfg).last = MAX_CABLE_LENGTH_CM; (*cfg).pair = PHY_PAIR_ALL;
    if nest.is_null() { return 0; }
    let mut tb: [*mut nlattr; 4] = [core::ptr::null_mut(); 4];
    let ret = nla_parse_nested(tb.as_mut_ptr(), cable_test_tdr_act_cfg_policy.len() - 1, nest, cable_test_tdr_act_cfg_policy.as_ptr(), (*info).extack);
    if ret < 0 { return ret; }
    if !tb[ETHTOOL_A_CABLE_TEST_TDR_CFG_FIRST].is_null() { (*cfg).first = nla_get_u32(tb[ETHTOOL_A_CABLE_TEST_TDR_CFG_FIRST]); }
    if !tb[ETHTOOL_A_CABLE_TEST_TDR_CFG_LAST].is_null() { (*cfg).last = nla_get_u32(tb[ETHTOOL_A_CABLE_TEST_TDR_CFG_LAST]); }
    if !tb[ETHTOOL_A_CABLE_TEST_TDR_CFG_STEP].is_null() { (*cfg).step = nla_get_u32(tb[ETHTOOL_A_CABLE_TEST_TDR_CFG_STEP]); }
    if !tb[ETHTOOL_A_CABLE_TEST_TDR_CFG_PAIR].is_null() { (*cfg).pair = nla_get_u8(tb[ETHTOOL_A_CABLE_TEST_TDR_CFG_PAIR]); if (*cfg).pair > ETHTOOL_A_CABLE_PAIR_D { NL_SET_ERR_MSG_ATTR!((*info).extack, tb[ETHTOOL_A_CABLE_TEST_TDR_CFG_PAIR], "invalid pair parameter"); return -EINVAL; } }
    if (*cfg).first > MAX_CABLE_LENGTH_CM { NL_SET_ERR_MSG_ATTR!((*info).extack, tb[ETHTOOL_A_CABLE_TEST_TDR_CFG_FIRST], "invalid first parameter"); return -EINVAL; }
    if (*cfg).last > MAX_CABLE_LENGTH_CM { NL_SET_ERR_MSG_ATTR!((*info).extack, tb[ETHTOOL_A_CABLE_TEST_TDR_CFG_LAST], "invalid last parameter"); return -EINVAL; }
    if (*cfg).first > (*cfg).last { NL_SET_ERR_MSG!((*info).extack, "invalid first/last parameter"); return -EINVAL; }
    if (*cfg).step == 0 { NL_SET_ERR_MSG_ATTR!((*info).extack, tb[ETHTOOL_A_CABLE_TEST_TDR_CFG_STEP], "invalid step parameter"); return -EINVAL; }
    if (*cfg).step > (*cfg).last - (*cfg).first { NL_SET_ERR_MSG_ATTR!((*info).extack, tb[ETHTOOL_A_CABLE_TEST_TDR_CFG_STEP], "step parameter too big"); return -EINVAL; }
    0
}

pub unsafe fn ethnl_act_cable_test_tdr(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let mut req_info: ethnl_req_info = core::mem::zeroed(); let tb = (*info).attrs; let mut cfg: phy_tdr_config = core::mem::zeroed(); let mut ret;
    ret = ethnl_parse_header_dev_get(&mut req_info, *tb.add(ETHTOOL_A_CABLE_TEST_TDR_HEADER), genl_info_net(info), (*info).extack, true); if ret < 0 { return ret; }
    let dev = req_info.dev;
    ret = ethnl_act_cable_test_tdr_cfg(*tb.add(ETHTOOL_A_CABLE_TEST_TDR_CFG), info, &mut cfg); if ret != 0 { goto!(out_dev_put); }
    netdev_lock_ops_compat(dev); let phydev = ethnl_req_get_phydev(&mut req_info, tb, ETHTOOL_A_CABLE_TEST_TDR_HEADER, (*info).extack);
    if IS_ERR_OR_NULL!(phydev) { ret = -EOPNOTSUPP; goto!(out_unlock); }
    let ops = ethtool_phy_ops; if ops.is_null() || (*ops).start_cable_test_tdr.is_none() { ret = -EOPNOTSUPP; goto!(out_unlock); }
    ret = ethnl_ops_begin(dev); if ret < 0 { goto!(out_unlock); }
    ret = ((*ops).start_cable_test_tdr.unwrap())(phydev, (*info).extack, &mut cfg); ethnl_ops_complete(dev);
    if ret == 0 { ethnl_cable_test_started(phydev, ETHTOOL_MSG_CABLE_TEST_TDR_NTF); }
out_unlock: netdev_unlock_ops_compat(dev);
out_dev_put: ethnl_parse_header_dev_put(&mut req_info); ret
}

pub unsafe fn ethnl_cable_test_amplitude(phydev: *mut phy_device, pair: u8, mV: i16) -> c_int {
    let nest = nla_nest_start((*phydev).skb, ETHTOOL_A_CABLE_TDR_NEST_AMPLITUDE); if nest.is_null() { return -EMSGSIZE; }
    if nla_put_u8((*phydev).skb, ETHTOOL_A_CABLE_AMPLITUDE_PAIR, pair) != 0 { goto_cancel!(nest); }
    if nla_put_u16((*phydev).skb, ETHTOOL_A_CABLE_AMPLITUDE_mV, mV as u16) != 0 { goto_cancel!(nest); }
    nla_nest_end((*phydev).skb, nest); 0
}

pub unsafe fn ethnl_cable_test_pulse(phydev: *mut phy_device, mV: u16) -> c_int {
    let nest = nla_nest_start((*phydev).skb, ETHTOOL_A_CABLE_TDR_NEST_PULSE); if nest.is_null() { return -EMSGSIZE; }
    if nla_put_u16((*phydev).skb, ETHTOOL_A_CABLE_PULSE_mV, mV) != 0 { goto_cancel!(nest); }
    nla_nest_end((*phydev).skb, nest); 0
}

pub unsafe fn ethnl_cable_test_step(phydev: *mut phy_device, first: u32, last: u32, step: u32) -> c_int {
    let nest = nla_nest_start((*phydev).skb, ETHTOOL_A_CABLE_TDR_NEST_STEP); if nest.is_null() { return -EMSGSIZE; }
    if nla_put_u32((*phydev).skb, ETHTOOL_A_CABLE_STEP_FIRST_DISTANCE, first) != 0 { goto_cancel!(nest); }
    if nla_put_u32((*phydev).skb, ETHTOOL_A_CABLE_STEP_LAST_DISTANCE, last) != 0 { goto_cancel!(nest); }
    if nla_put_u32((*phydev).skb, ETHTOOL_A_CABLE_STEP_STEP_DISTANCE, step) != 0 { goto_cancel!(nest); }
    nla_nest_end((*phydev).skb, nest); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
