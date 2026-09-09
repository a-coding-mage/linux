// SPDX-License-Identifier: GPL-2.0-only
//
// ethtool interface for Ethernet PSE (Power Sourcing Equipment)
// and PD (Powered Device)
//
// Copyright (c) 2022 Pengutronix, Oleksij Rempel <kernel@pengutronix.de>
//
// External kernel/netlink declarations are supplied by the surrounding crate.

#[repr(C)]
pub struct pse_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct pse_reply_data {
    pub base: ethnl_reply_data,
    pub status: ethtool_pse_control_status,
}

// PSE_GET

pub static mut ethnl_pse_get_policy: [nla_policy; ETHTOOL_A_PSE_HEADER as usize + 1] = [nla_policy::default(); ETHTOOL_A_PSE_HEADER as usize + 1];

unsafe fn pse_get_pse_attributes(
    phydev: *mut phy_device,
    extack: *mut netlink_ext_ack,
    data: *mut pse_reply_data,
) -> i32 {
    if phydev.is_null() {
        NL_SET_ERR_MSG(extack, "No PHY found");
        return -EOPNOTSUPP;
    }
    if (*phydev).psec.is_null() {
        NL_SET_ERR_MSG(extack, "No PSE is attached");
        return -EOPNOTSUPP;
    }
    core::ptr::write_bytes(core::ptr::addr_of_mut!((*data).status), 0, 1);
    pse_ethtool_get_status((*phydev).psec, extack, core::ptr::addr_of_mut!((*data).status))
}

unsafe fn pse_prepare_data(
    req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    info: *const genl_info,
) -> i32 {
    let data = reply_base as *mut pse_reply_data;
    let dev = (*reply_base).dev;
    let tb = (*info).attrs;
    let phydev = ethnl_req_get_phydev(req_base, tb, ETHTOOL_A_PSE_HEADER, (*info).extack);
    if IS_ERR(phydev) { return PTR_ERR(phydev); }
    let mut ret = ethnl_ops_begin(dev);
    if ret < 0 { return ret; }
    ret = pse_get_pse_attributes(phydev, (*info).extack, data);
    ethnl_ops_complete(dev);
    ret
}

unsafe fn pse_reply_size(_req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> i32 {
    let data = reply_base as *const pse_reply_data;
    let st = &(*data).status;
    let mut len = 0;
    if st.pw_d_id != 0 { len += nla_total_size(core::mem::size_of::<u32>()); }
    if st.podl_admin_state > 0 { len += nla_total_size(core::mem::size_of::<u32>()); }
    if st.podl_pw_status > 0 { len += nla_total_size(core::mem::size_of::<u32>()); }
    if st.c33_admin_state > 0 { len += nla_total_size(core::mem::size_of::<u32>()); }
    if st.c33_pw_status > 0 { len += nla_total_size(core::mem::size_of::<u32>()); }
    if st.c33_pw_class > 0 { len += nla_total_size(core::mem::size_of::<u32>()); }
    if st.c33_actual_pw > 0 { len += nla_total_size(core::mem::size_of::<u32>()); }
    if st.c33_ext_state_info.c33_pse_ext_state > 0 {
        len += nla_total_size(core::mem::size_of::<u32>());
        if st.c33_ext_state_info.__c33_pse_ext_substate > 0 { len += nla_total_size(core::mem::size_of::<u32>()); }
    }
    if st.c33_avail_pw_limit > 0 { len += nla_total_size(core::mem::size_of::<u32>()); }
    if st.c33_pw_limit_nb_ranges > 0 {
        len += st.c33_pw_limit_nb_ranges * (nla_total_size(0) + nla_total_size(core::mem::size_of::<u32>()) * 2);
    }
    if st.prio_max != 0 { len += nla_total_size(core::mem::size_of::<u32>()) * 2; }
    len
}

unsafe fn pse_put_pw_limit_ranges(skb: *mut sk_buff, st: *const ethtool_pse_control_status) -> i32 {
    let mut ranges = (*st).c33_pw_limit_ranges;
    for _i in 0..(*st).c33_pw_limit_nb_ranges {
        let nest = nla_nest_start(skb, ETHTOOL_A_C33_PSE_PW_LIMIT_RANGES);
        if nest.is_null() { return -EMSGSIZE; }
        if nla_put_u32(skb, ETHTOOL_A_C33_PSE_PW_LIMIT_MIN, (*ranges).min) != 0 ||
           nla_put_u32(skb, ETHTOOL_A_C33_PSE_PW_LIMIT_MAX, (*ranges).max) != 0 {
            nla_nest_cancel(skb, nest); return -EMSGSIZE;
        }
        nla_nest_end(skb, nest);
        ranges = ranges.add(1);
    }
    0
}

unsafe fn pse_fill_reply(skb: *mut sk_buff, _req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> i32 {
    let st = &(*(reply_base as *const pse_reply_data)).status;
    macro_rules! put { ($cond:expr, $attr:expr, $value:expr) => { if $cond && nla_put_u32(skb, $attr, $value) != 0 { return -EMSGSIZE; } }; }
    put!(st.pw_d_id != 0, ETHTOOL_A_PSE_PW_D_ID, st.pw_d_id);
    put!(st.podl_admin_state > 0, ETHTOOL_A_PODL_PSE_ADMIN_STATE, st.podl_admin_state);
    put!(st.podl_pw_status > 0, ETHTOOL_A_PODL_PSE_PW_D_STATUS, st.podl_pw_status);
    put!(st.c33_admin_state > 0, ETHTOOL_A_C33_PSE_ADMIN_STATE, st.c33_admin_state);
    put!(st.c33_pw_status > 0, ETHTOOL_A_C33_PSE_PW_D_STATUS, st.c33_pw_status);
    put!(st.c33_pw_class > 0, ETHTOOL_A_C33_PSE_PW_CLASS, st.c33_pw_class);
    put!(st.c33_actual_pw > 0, ETHTOOL_A_C33_PSE_ACTUAL_PW, st.c33_actual_pw);
    if st.c33_ext_state_info.c33_pse_ext_state > 0 {
        put!(true, ETHTOOL_A_C33_PSE_EXT_STATE, st.c33_ext_state_info.c33_pse_ext_state);
        put!(st.c33_ext_state_info.__c33_pse_ext_substate > 0, ETHTOOL_A_C33_PSE_EXT_SUBSTATE, st.c33_ext_state_info.__c33_pse_ext_substate);
    }
    put!(st.c33_avail_pw_limit > 0, ETHTOOL_A_C33_PSE_AVAIL_PW_LIMIT, st.c33_avail_pw_limit);
    if st.c33_pw_limit_nb_ranges > 0 && pse_put_pw_limit_ranges(skb, st) != 0 { return -EMSGSIZE; }
    if st.prio_max != 0 {
        put!(true, ETHTOOL_A_PSE_PRIO_MAX, st.prio_max);
        put!(true, ETHTOOL_A_PSE_PRIO, st.prio);
    }
    0
}

unsafe fn pse_cleanup_data(reply_base: *mut ethnl_reply_data) {
    let data = reply_base as *const pse_reply_data;
    kfree((*data).status.c33_pw_limit_ranges as *mut core::ffi::c_void);
}

// PSE_SET
pub static mut ethnl_pse_set_policy: [nla_policy; ETHTOOL_A_PSE_MAX as usize + 1] = [nla_policy::default(); ETHTOOL_A_PSE_MAX as usize + 1];

unsafe fn ethnl_set_pse_validate(phydev: *mut phy_device, info: *mut genl_info) -> i32 {
    let tb = (*info).attrs;
    if IS_ERR_OR_NULL(phydev) { NL_SET_ERR_MSG((*info).extack, "No PHY is attached"); return -EOPNOTSUPP; }
    if (*phydev).psec.is_null() { NL_SET_ERR_MSG((*info).extack, "No PSE is attached"); return -EOPNOTSUPP; }
    if !tb[ETHTOOL_A_PODL_PSE_ADMIN_CONTROL as usize].is_null() && !pse_has_podl((*phydev).psec) { return -EOPNOTSUPP; }
    if !tb[ETHTOOL_A_C33_PSE_ADMIN_CONTROL as usize].is_null() && !pse_has_c33((*phydev).psec) { return -EOPNOTSUPP; }
    0
}

unsafe fn ethnl_set_pse(req_info: *mut ethnl_req_info, info: *mut genl_info) -> i32 {
    let phydev = ethnl_req_get_phydev(req_info, (*info).attrs, ETHTOOL_A_PSE_HEADER, (*info).extack);
    let mut ret = ethnl_set_pse_validate(phydev, info);
    if ret != 0 { return ret; }
    let tb = (*info).attrs;
    if !tb[ETHTOOL_A_PSE_PRIO as usize].is_null() { ret = pse_ethtool_set_prio((*phydev).psec, (*info).extack, nla_get_u32(tb[ETHTOOL_A_PSE_PRIO as usize])); if ret != 0 { return ret; } }
    if !tb[ETHTOOL_A_C33_PSE_AVAIL_PW_LIMIT as usize].is_null() { ret = pse_ethtool_set_pw_limit((*phydev).psec, (*info).extack, nla_get_u32(tb[ETHTOOL_A_C33_PSE_AVAIL_PW_LIMIT as usize])); if ret != 0 { return ret; } }
    if !tb[ETHTOOL_A_PODL_PSE_ADMIN_CONTROL as usize].is_null() || !tb[ETHTOOL_A_C33_PSE_ADMIN_CONTROL as usize].is_null() {
        let mut config: pse_control_config = core::mem::zeroed();
        if !tb[ETHTOOL_A_PODL_PSE_ADMIN_CONTROL as usize].is_null() { config.podl_admin_control = nla_get_u32(tb[ETHTOOL_A_PODL_PSE_ADMIN_CONTROL as usize]); }
        if !tb[ETHTOOL_A_C33_PSE_ADMIN_CONTROL as usize].is_null() { config.c33_admin_control = nla_get_u32(tb[ETHTOOL_A_C33_PSE_ADMIN_CONTROL as usize]); }
        ret = pse_ethtool_set_config((*phydev).psec, (*info).extack, &mut config);
        if ret != 0 { return ret; }
    }
    ret
}

pub unsafe fn ethnl_pse_send_ntf(netdev: *mut net_device, notifs: core::ffi::c_ulong) {
    ASSERT_RTNL();
    if netdev.is_null() || notifs == 0 { return; }
    let reply_len = ethnl_reply_header_size() + nla_total_size(core::mem::size_of::<u32>());
    let skb = genlmsg_new(reply_len, GFP_KERNEL);
    if skb.is_null() { return; }
    let reply_payload = ethnl_bcastmsg_put(skb, ETHTOOL_MSG_PSE_NTF);
    if reply_payload.is_null() { nlmsg_free(skb); return; }
    if ethnl_fill_reply_header(skb, netdev, ETHTOOL_A_PSE_NTF_HEADER) < 0 || nla_put_uint(skb, ETHTOOL_A_PSE_NTF_EVENTS, notifs) != 0 { nlmsg_free(skb); return; }
    genlmsg_end(skb, reply_payload);
    ethnl_multicast(skb, netdev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
