// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel/netlink implementation.

use core::ffi::c_void;

#[repr(C)]
pub struct plca_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct plca_reply_data {
    pub base: ethnl_reply_data,
    pub plca_cfg: phy_plca_cfg,
    pub plca_st: phy_plca_status,
}

#[repr(C)]
pub struct ethnl_req_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ethnl_reply_data {
    pub dev: *mut net_device,
}

#[repr(C)]
pub struct phy_plca_cfg {
    pub version: i32,
    pub enabled: i32,
    pub node_id: i32,
    pub node_cnt: i32,
    pub to_tmr: i32,
    pub burst_cnt: i32,
    pub burst_tmr: i32,
}

#[repr(C)]
pub struct phy_plca_status {
    pub pst: u8,
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct phy_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nlattr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct genl_info {
    pub attrs: *mut *mut nlattr,
    pub extack: *mut c_void,
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nla_policy {
    pub kind: u32,
    pub data: u32,
}

#[repr(C)]
pub struct ethtool_phy_ops {
    pub get_plca_cfg: Option<unsafe extern "C" fn(*mut phy_device, *mut phy_plca_cfg) -> i32>,
    pub set_plca_cfg: Option<unsafe extern "C" fn(*mut phy_device, *mut phy_plca_cfg, *mut c_void) -> i32>,
    pub get_plca_status: Option<unsafe extern "C" fn(*mut phy_device, *mut phy_plca_status) -> i32>,
}

#[repr(C)]
pub struct ethnl_request_ops {
    pub request_cmd: u32,
    pub reply_cmd: u32,
    pub hdr_attr: u32,
    pub req_info_size: usize,
    pub reply_data_size: usize,
    pub prepare_data: Option<unsafe extern "C" fn(*const ethnl_req_info, *mut ethnl_reply_data, *const genl_info) -> i32>,
    pub reply_size: Option<unsafe extern "C" fn(*const ethnl_req_info, *const ethnl_reply_data) -> i32>,
    pub fill_reply: Option<unsafe extern "C" fn(*mut sk_buff, *const ethnl_req_info, *const ethnl_reply_data) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut ethnl_req_info, *mut genl_info) -> i32>,
    pub set_ntf_cmd: u32,
}

extern "C" {
    pub static ethnl_plca_set_cfg_policy: [nla_policy; 7];
    pub static ethnl_header_policy_phy: nla_policy;
    pub static mut ethtool_phy_ops: *const ethtool_phy_ops;
    fn ethnl_req_get_phydev(req: *const ethnl_req_info, tb: *mut *mut nlattr, attr: u32, extack: *mut c_void) -> *mut phy_device;
    fn ethnl_ops_begin(dev: *mut net_device) -> i32;
    fn ethnl_ops_complete(dev: *mut net_device);
    fn nla_get_u8(attr: *const nlattr) -> u8;
    fn nla_get_u32(attr: *const nlattr) -> u32;
    fn nla_put_u8(skb: *mut sk_buff, attr: u32, value: u8) -> i32;
    fn nla_put_u16(skb: *mut sk_buff, attr: u32, value: u16) -> i32;
    fn nla_put_u32(skb: *mut sk_buff, attr: u32, value: u32) -> i32;
    fn nla_total_size(size: usize) -> i32;
}

// PLCA get configuration message

pub static ethnl_plca_get_cfg_policy: [nla_policy; 1] = [nla_policy { kind: 0, data: 0 }];

unsafe fn plca_update_sint(dst: *mut i32, tb: *mut *mut nlattr, attrid: u32, modified: *mut bool) {
    let attr = *tb.add(attrid as usize);
    if attr.is_null() {
        return;
    }
    match ethnl_plca_set_cfg_policy[attrid as usize].kind {
        1 => *dst = nla_get_u8(attr) as i32,
        5 => *dst = nla_get_u32(attr) as i32,
        _ => return,
    }
    *modified = true;
}

unsafe extern "C" fn plca_get_cfg_prepare_data(req_base: *const ethnl_req_info, reply_base: *mut ethnl_reply_data, info: *const genl_info) -> i32 {
    let data = reply_base as *mut plca_reply_data;
    let dev = (*reply_base).dev;
    let tb = (*info).attrs;
    let phydev = ethnl_req_get_phydev(req_base, tb, 1, (*info).extack);
    if phydev.is_null() { return -95; }
    let ops = ethtool_phy_ops;
    if ops.is_null() || (*ops).get_plca_cfg.is_none() { return -95; }
    let mut ret = ethnl_ops_begin(dev);
    if ret < 0 { return ret; }
    core::ptr::write_bytes(&mut (*data).plca_cfg, 0xff, 1);
    ret = ((*ops).get_plca_cfg.unwrap())(phydev, &mut (*data).plca_cfg);
    ethnl_ops_complete(dev);
    ret
}

unsafe extern "C" fn plca_get_cfg_reply_size(_: *const ethnl_req_info, _: *const ethnl_reply_data) -> i32 {
    nla_total_size(2) + nla_total_size(1) + 5 * nla_total_size(4)
}

unsafe extern "C" fn plca_get_cfg_fill_reply(skb: *mut sk_buff, _: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> i32 {
    let plca = &(*(reply_base as *const plca_reply_data)).plca_cfg;
    if (plca.version >= 0 && nla_put_u16(skb, 2, plca.version as u16) != 0) ||
       (plca.enabled >= 0 && nla_put_u8(skb, 3, (plca.enabled != 0) as u8) != 0) ||
       (plca.node_id >= 0 && nla_put_u32(skb, 4, plca.node_id as u32) != 0) ||
       (plca.node_cnt >= 0 && nla_put_u32(skb, 5, plca.node_cnt as u32) != 0) ||
       (plca.to_tmr >= 0 && nla_put_u32(skb, 6, plca.to_tmr as u32) != 0) ||
       (plca.burst_cnt >= 0 && nla_put_u32(skb, 7, plca.burst_cnt as u32) != 0) ||
       (plca.burst_tmr >= 0 && nla_put_u32(skb, 8, plca.burst_tmr as u32) != 0) { return -90; }
    0
}

pub static ethnl_plca_set_cfg_policy_local: [nla_policy; 7] = [nla_policy { kind: 0, data: 0 }; 7];

unsafe extern "C" fn ethnl_set_plca(req_info: *mut ethnl_req_info, info: *mut genl_info) -> i32 {
    let tb = (*info).attrs;
    let phydev = ethnl_req_get_phydev(req_info, tb, 1, (*info).extack);
    if phydev.is_null() { return -95; }
    let ops = ethtool_phy_ops;
    if ops.is_null() || (*ops).set_plca_cfg.is_none() { return -95; }
    let mut cfg = phy_plca_cfg { version: -1, enabled: -1, node_id: -1, node_cnt: -1, to_tmr: -1, burst_cnt: -1, burst_tmr: -1 };
    let mut modified = false;
    plca_update_sint(&mut cfg.enabled, tb, 3, &mut modified);
    plca_update_sint(&mut cfg.node_id, tb, 4, &mut modified);
    plca_update_sint(&mut cfg.node_cnt, tb, 5, &mut modified);
    plca_update_sint(&mut cfg.to_tmr, tb, 6, &mut modified);
    plca_update_sint(&mut cfg.burst_cnt, tb, 7, &mut modified);
    plca_update_sint(&mut cfg.burst_tmr, tb, 8, &mut modified);
    if !modified { return 0; }
    let ret = ((*ops).set_plca_cfg.unwrap())(phydev, &mut cfg, (*info).extack);
    if ret < 0 { ret } else { 1 }
}

pub static ethnl_plca_cfg_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: 1, reply_cmd: 2, hdr_attr: 1, req_info_size: core::mem::size_of::<plca_req_info>(), reply_data_size: core::mem::size_of::<plca_reply_data>(),
    prepare_data: Some(plca_get_cfg_prepare_data), reply_size: Some(plca_get_cfg_reply_size), fill_reply: Some(plca_get_cfg_fill_reply), set: Some(ethnl_set_plca), set_ntf_cmd: 3,
};

// PLCA get status message
pub static ethnl_plca_get_status_policy: [nla_policy; 1] = [nla_policy { kind: 0, data: 0 }];

unsafe extern "C" fn plca_get_status_prepare_data(req: *const ethnl_req_info, reply: *mut ethnl_reply_data, info: *const genl_info) -> i32 {
    let data = reply as *mut plca_reply_data;
    let dev = (*reply).dev;
    let phydev = ethnl_req_get_phydev(req, (*info).attrs, 1, (*info).extack);
    if phydev.is_null() { return -95; }
    let ops = ethtool_phy_ops;
    if ops.is_null() || (*ops).get_plca_status.is_none() { return -95; }
    let mut ret = ethnl_ops_begin(dev);
    if ret < 0 { return ret; }
    core::ptr::write_bytes(&mut (*data).plca_st, 0xff, 1);
    ret = ((*ops).get_plca_status.unwrap())(phydev, &mut (*data).plca_st);
    ethnl_ops_complete(dev);
    ret
}

unsafe extern "C" fn plca_get_status_reply_size(_: *const ethnl_req_info, _: *const ethnl_reply_data) -> i32 { nla_total_size(1) }

unsafe extern "C" fn plca_get_status_fill_reply(skb: *mut sk_buff, _: *const ethnl_req_info, reply: *const ethnl_reply_data) -> i32 {
    if nla_put_u8(skb, 9, ((*(reply as *const plca_reply_data)).plca_st.pst != 0) as u8) != 0 { return -90; }
    0
}

pub static ethnl_plca_status_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: 4, reply_cmd: 5, hdr_attr: 1, req_info_size: core::mem::size_of::<plca_req_info>(), reply_data_size: core::mem::size_of::<plca_reply_data>(),
    prepare_data: Some(plca_get_status_prepare_data), reply_size: Some(plca_get_status_reply_size), fill_reply: Some(plca_get_status_fill_reply), set: None, set_ntf_cmd: 0,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
